//! Type 0x16: a smoke puff that grows and rises (`0x335a90` / `0x331048`, a VU0 routine read from its listing: R&C1's
//! type 22, the same record and steps; the Sheepinator's sheep popping, crate::moby::classes::sheep).
//!
//! **Made** ([`smoke`], `0x335a90(size, at, vel, c1, c2, life)`): ALPHA 0x44, near 4 / far 0x70 (`+0x09` 0x74), from
//! `size`·0.2 growing to `size` over its life (`+0x3c`), its roll the low byte of a raw `rand()`, frame `*def[0x16]`, its
//! life (`+0x0a`, `+0x38`), the colours `c1` → `c2` (`+0x30` / `+0x34`), `vel` (`+0x20`) and its rise a tick
//! (`+0x2c`, 0x39b4968d: 1.24 / 3600).
//!
//! **Each tick** ([`update`]): its life down, out gone; it moves, rolls one step, its velocity rises; its colour runs
//! from `c1` to `c2` with the life (`t / life`, both alphas × `min(t, 8) / 8`); it grows.

use super::{rec, Particles};
use crate::rng::Rng;

/// Type 0x16.
pub const SMOKE: u8 = 0x16;

/// `0x335a90` (module docs).
pub fn smoke(sys: &mut Particles, rng: &mut Rng, size: f32, at: [f32; 3], vel: [f32; 3], c1: u32, c2: u32, life: i32) -> Option<usize> {
    let i = sys.create_part(SMOKE)?;
    let def = sys.def_first(SMOKE);
    let roll = rng.rand() as u8;
    let r = &mut sys.pool.recs[i];
    rec::set_u32(r, 4, c1);
    r[1] = 0;
    r[3] = 0x44;
    r[9] = 4 + 0x70;
    let s0 = size * 0.2;
    rec::set_ff(r, 0xc, s0);
    r[8] = roll;
    rec::set_i16(r, 0xa, life as i16);
    r[2] = def;
    rec::set_i16(r, 0x38, life as i16);
    rec::set_u32(r, 0x30, c1);
    rec::set_u32(r, 0x34, c2);
    rec::set_ff(r, 0x3c, (size - s0) / life as f32);
    rec::set_v3(r, 0x20, vel);
    rec::set_v3(r, 0x10, at);
    rec::set_ff(r, 0x2c, f32::from_bits(0x39b4_968d));
    Some(i)
}

/// `0x331048` (module docs).
pub fn update(sys: &mut Particles, i: usize) {
    let r = &mut sys.pool.recs[i];
    let t = rec::i16(r, 0xa) as i32 - 1;
    if t <= 0 {
        sys.kill_part(i);
        return;
    }
    rec::set_i16(r, 0xa, t as i16);
    let n = rec::i16(r, 0x38) as f32;
    let (p, v) = (rec::v3(r, 0x10), rec::v3(r, 0x20));
    rec::set_v3(r, 0x10, [p[0] + v[0], p[1] + v[1], p[2] + v[2]]);
    r[8] = r[8].wrapping_add(1);
    rec::set_ff(r, 0x28, v[2] + rec::ff(r, 0x2c));
    let q = t as f32 / n;
    let fade = t.min(8) as f32 / 8.0;
    let (c1, c2) = (rec::u32(r, 0x30), rec::u32(r, 0x34));
    let mut out = 0u32;
    for k in 0..4 {
        let (mut a, mut b) = ((c1 >> (8 * k) & 0xff) as f32, (c2 >> (8 * k) & 0xff) as f32);
        if k == 3 {
            a *= fade;
            b *= fade;
        }
        let v = b * (1.0 - q) + a * q;
        out |= (v as i32 as u32 & 0xff) << (8 * k);
    }
    rec::set_ff(r, 0xc, rec::ff(r, 0xc) + rec::ff(r, 0x3c));
    rec::set_u32(r, 4, out);
}
