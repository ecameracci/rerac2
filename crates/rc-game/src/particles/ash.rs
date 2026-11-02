//! Type 0x66: ash (`0x3412e8` / `0x341438`; the Zodiac's kills, crate::moby::classes::zodiac): dark flakes drifting
//! down and swaying across the view.
//!
//! **A flake** ([`ash`], `0x3412e8(at, vel, life)`): ALPHA 0x44, near 2 / far 4 (`+0x09` 0x42), frame `*def[0x66]`, size
//! 0.03..0.175 × 210000, a random roll, its life (`+0x0a`), `at` and `vel` (`+0x20`), its fall 4.6..8.6 / 3600 a tick
//! (`+0x2c`, the velocity's fourth word), twice its size (`+0x38`, unread), a roll step 0..3 (`+0x36`), half its life
//! (`+0x34`) and its sway's phase (`+0x3c`, `0x31bb00`).
//!
//! **Each tick** ([`update`], `0x341438`): its velocity × 0.9; above the ground (`0x31ce20(0.5)`, none: 0) by more
//! than 0.25 it falls its fall and rolls its step; it moves; every tenth frame (`0x1b1948 % 10`) while above, its sway's
//! phase on 1 and `0.0075·sin` of it along the view matrix's first row (`0x1b8c00`: the camera's −left.x, −up.x,
//! forward.x, `0x2e4818`) added to its velocity; its colour `0x050505`, its alpha 0x9f, in the second half of its life
//! `128 · left / half`; its life out: gone.

use super::{dec_life, rec, Particles};
use crate::rng::Rng;

/// Type 0x66.
pub const ASH: u8 = 0x66;

/// `0x3412e8(at, vel, life)` (module docs).
pub fn ash(sys: &mut Particles, rng: &mut Rng, at: [f32; 3], vel: [f32; 3], life: i32) -> Option<usize> {
    let i = sys.create_part(ASH)?;
    let def = sys.def_first(ASH);
    let size = rng.randf(f32::from_bits(0x3cf5_c28f), f32::from_bits(0x3e33_3333)) * 210_000.0;
    let roll = rng.randi(0xff) as u8;
    let fall = rng.randf(f32::from_bits(0x4093_3333), f32::from_bits(0x4109_999a)) * 0.000_277_777_78;
    let step = rng.rand_range(0, 3) as i16;
    let phase = rng.rand_angle();
    let r = &mut sys.pool.recs[i];
    r[9] = 2 + 0x40;
    r[3] = 0x44;
    r[1] = 0;
    r[2] = def;
    rec::set_ff(r, 0xc, size);
    r[8] = roll;
    rec::set_i16(r, 0xa, life as i16);
    rec::set_v3(r, 0x10, at);
    rec::set_v3(r, 0x20, vel);
    rec::set_ff(r, 0x2c, fall);
    rec::set_ff(r, 0x38, size + size);
    rec::set_i16(r, 0x36, step);
    rec::set_i16(r, 0x34, (life / 2) as i16);
    rec::set_ff(r, 0x3c, phase);
    Some(i)
}

/// `0x341438` (module docs). `coll`: the level's collision (the ground); `frame`: `0x1b1948`; `view_x`: the view
/// matrix's first row (`0x1b8c00`).
pub fn update(sys: &mut Particles, i: usize, coll: Option<&rc_formats::collision::Collision>, frame: i32, view_x: [f32; 3]) {
    let r = &mut sys.pool.recs[i];
    let mut v = rec::v3(r, 0x20);
    for c in v.iter_mut() { *c *= 0.9; }
    let p = rec::v3(r, 0x10);
    // `0x31ce20(0.5, p, 0)`: the ground under it, 0 when none.
    let ground = coll.and_then(|c| crate::collision::coll_line(c, [p[0], p[1], p[2] + 0.5], [p[0], p[1], 0.01], crate::collision::QueryFlags(2))).map_or(0.0, |o| o.point[2]);
    let above = ground + 0.25 < p[2];
    if above {
        v[2] -= rec::ff(r, 0x2c);
        r[8] = r[8].wrapping_add(rec::i16(r, 0x36) as u8);
    }
    rec::set_v3(r, 0x10, std::array::from_fn(|c| p[c] + v[c]));
    if frame % 10 == 0 && above {
        let phase = crate::hero::physics::add_rot(rec::ff(r, 0x3c), 1.0);
        rec::set_ff(r, 0x3c, phase);
        let k = phase.sin() * 0.0075;
        for c in 0..3 { v[c] += view_x[c] * k; }
    }
    rec::set_v3(r, 0x20, v);
    let (left, half) = (rec::i16(r, 0xa), rec::i16(r, 0x34));
    let c = if left < half { (((left as f32 / half as f32) * 128.0) as i32 as u32) << 24 | 0x05_0505 } else { 0x9f05_0505 };
    rec::set_u32(r, 4, c);
    if dec_life(r) { sys.kill_part(i); }
}
