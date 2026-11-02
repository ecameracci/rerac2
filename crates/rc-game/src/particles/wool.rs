//! Type 0x63: wool (`0x340ce8` / `0x340e38`; the Sheepinator's sheep popping, crate::moby::classes::sheep): tufts
//! that fly out, slow, fall and sway across the view, and scatter from Ratchet running through them.
//!
//! **A tuft** ([`wool`], `0x340ce8(at, vel, life)`): ALPHA 0x44, near 2 / far 4 (`+0x09` 0x42), frame `*def[0x63]`, size
//! 0.05..0.2 × 210000, a random roll, its life (`+0x0a`), `at` and `vel` (`+0x20`), its fall 0.6..2.6 / 3600 a tick
//! (`+0x2c`), twice its size (`+0x38`, unread), a roll step 0..3 (`+0x36`), half its life (`+0x34`) and its sway's
//! phase (`+0x3c`, `0x31bb00`).
//!
//! **Each tick** ([`update`], `0x340e38`): (`0x380e08`, the gravity frames' bend, not ported) its velocity × 0.9 and
//! falling its fall; it moves; within 1.05 of Ratchet's feet while he moves (`0x189f90`, his speed) it is pushed away
//! from them by `0.2·speed + 0.15·randf(0.1, 2)/60` and rolls `step · speed / 2`; every tenth frame (`0x1b1948 % 10`) its
//! sway's phase on 1 and `0.0075·sin` of it along the view matrix's first row (`0x1b8c00`) added to its velocity; its
//! colour by half its life: under 0xa0 ticks `0x909090` at alpha 0x40 (in the second half `64 · left / half`), else
//! `0x353535` at 0x9f (`128 · left / half`); it rolls its step; its life out: gone.

use super::{dec_life, rec, Particles};
use crate::hero::physics::{add, dist, set_len, sub};
use crate::rng::Rng;

/// Type 0x63.
pub const WOOL: u8 = 0x63;

/// `0x340ce8(at, vel, life)` (module docs).
pub fn wool(sys: &mut Particles, rng: &mut Rng, at: [f32; 3], vel: [f32; 3], life: i32) -> Option<usize> {
    let i = sys.create_part(WOOL)?;
    let def = sys.def_first(WOOL);
    let size = rng.randf(f32::from_bits(0x3d4c_cccd), f32::from_bits(0x3e4c_cccd)) * 210_000.0;
    let roll = rng.randi(0xff) as u8;
    let fall = rng.randf(f32::from_bits(0x3f19_999a), f32::from_bits(0x4026_6666)) * 0.000_277_777_78;
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

/// `0x340e38` (module docs). `hero`: Ratchet's feet (`0x189ea0`) and his speed (`0x189f90`); `frame`: `0x1b1948`;
/// `view_x`: the view matrix's first row.
pub fn update(sys: &mut Particles, i: usize, rng: &mut Rng, hero: ([f32; 3], f32), frame: i32, view_x: [f32; 3]) {
    let r = &mut sys.pool.recs[i];
    let mut v = rec::v3(r, 0x20);
    for c in v.iter_mut() { *c *= 0.9; }
    v[2] -= rec::ff(r, 0x2c);
    let mut p = add(rec::v3(r, 0x10), v);
    let (feet, speed) = hero;
    if dist(p, feet) < 1.05 && speed != 0.0 {
        let k = rng.randf(0.1, 2.0);
        let push = set_len(sub(p, feet), speed * 0.2 + k * 0.016_666_668 * 0.15);
        r[8] = r[8].wrapping_add((rec::i16(r, 0x36) as f32 * speed * 0.5) as i32 as u8);
        p = add(p, push);
    }
    rec::set_v3(r, 0x10, p);
    if frame % 10 == 0 {
        let phase = crate::hero::physics::add_rot(rec::ff(r, 0x3c), 1.0);
        rec::set_ff(r, 0x3c, phase);
        let k = phase.sin() * 0.0075;
        for c in 0..3 { v[c] += view_x[c] * k; }
    }
    rec::set_v3(r, 0x20, v);
    let (left, half) = (rec::i16(r, 0xa), rec::i16(r, 0x34));
    let f = left as f32 / half as f32;
    let c = if half < 0xa0 {
        if left < half { ((f * 64.0) as i32 as u32) << 24 | 0x90_9090 } else { 0x4090_9090 }
    } else if left < half {
        ((f * 128.0) as i32 as u32) << 24 | 0x35_3535
    } else {
        0x9f35_3535
    };
    rec::set_u32(r, 4, c);
    r[8] = r[8].wrapping_add(rec::i16(r, 0x36) as u8);
    if dec_life(r) { sys.kill_part(i); }
}
