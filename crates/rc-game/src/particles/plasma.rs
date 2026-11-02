//! The upgraded Mini-Turret's plasma shots' smoke (type 0x6e, `0x342740` / `0x342988`; crate::moby::classes::turret).
//!
//! **A puff** (`0x342740(size, shot, at, alpha mode, life lo, life hi)`): ALPHA `alpha mode`, far 4 (`+0x09` 0x44), a
//! random roll, frame `*def[0x6e]`, `lo..hi` ticks, size `size`·630000, at `at` jittered ±0.015 on each axis, rising
//! 0.01..0.015 a tick along the up, its alpha 0x50..200 (`+0x30`), a grey of 0x60..0x80 (`+0x34`). Each tick
//! (`0x342988`): by `t` its life's share left, its alpha the start's · `t` and its colour from black toward its grey by
//! `t / 2`; its roll on a step; it grows 0.004..0.012 · 210000; it moves, its velocity ×0.99; its life out: gone.
//!
//! Not ported: the gravity frames' bends (`0x380e08` / `0x380ec0`).

use super::{rec, Particles};
use crate::rng::Rng;

/// Type 0x6e.
pub const PUFF: u8 = 0x6e;

/// `0x342740` (module docs).
#[allow(clippy::too_many_arguments)]
pub fn puff(sys: &mut Particles, rng: &mut Rng, size: f32, at: [f32; 3], up: [f32; 3], alpha_mode: u8, lo: i32, hi: i32) -> Option<usize> {
    let i = sys.create_part(PUFF)?;
    let def = sys.def_first(PUFF);
    let roll = rng.randi(0xff) as u8;
    let life = rng.rand_range(lo, hi) as i16;
    let p: [f32; 3] = std::array::from_fn(|k| at[k] + rng.randf(-0.015, 0.015));
    let rise = rng.randf(0.01, 0.015);
    let alpha = rng.rand_range(0x50, 200) as u32;
    let grey = rng.rand_range(0x60, 0x80) as u32 * 0x1_0101;
    let r = &mut sys.pool.recs[i];
    r[3] = alpha_mode;
    r[1] = 0;
    r[9] = 4 + 0x40;
    r[8] = roll;
    r[2] = def;
    rec::set_i16(r, 0xa, life);
    rec::set_u32(r, 0x38, life as i32 as u32);
    rec::set_ff(r, 0xc, size * 630_000.0);
    rec::set_v3(r, 0x10, p);
    rec::set_v3(r, 0x20, up.map(|x| x * rise));
    rec::set_u32(r, 0x30, alpha);
    rec::set_u32(r, 0x34, grey);
    rec::set_u32(r, 4, grey | alpha << 24);
    Some(i)
}

/// `0x342988` (module docs).
pub fn update(sys: &mut Particles, i: usize, rng: &mut Rng) {
    let grow = rng.randf(f32::from_bits(0x3b83_126f), f32::from_bits(0x3c44_9ba6));
    let r = &mut sys.pool.recs[i];
    let t = rec::i16(r, 0xa) as f32 / rec::u32(r, 0x38) as i32 as f32;
    let alpha = (rec::u32(r, 0x30) as i32 as f32 * t) as i32 as u32;
    let colour = crate::hud::lerp_rgba(t * 0.5, 0, rec::u32(r, 0x34));
    r[8] = r[8].wrapping_add(1);
    rec::set_ff(r, 0xc, rec::ff(r, 0xc) + grow * 210_000.0);
    let (p, v) = (rec::v3(r, 0x10), rec::v3(r, 0x20));
    rec::set_v3(r, 0x10, [p[0] + v[0], p[1] + v[1], p[2] + v[2]]);
    rec::set_v3(r, 0x20, v.map(|x| x * f32::from_bits(0x3f7d_70a4)));
    rec::set_u32(r, 4, colour | alpha << 24);
    let mut n = rec::i16(r, 0xa) as i32;
    let out = crate::moby::countdown(&mut n);
    rec::set_i16(r, 0xa, n as i16);
    if out { sys.kill_part(i); }
}
