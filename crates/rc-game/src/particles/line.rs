//! Type 0x13: a line (record kind 3, a ribbon from its head `+0x10` to its tail `+0x20`) moving with its velocity
//! (`0x3352c0` / `0x3353d8`; the Minirocket Tube's muzzle burst, crate::moby::classes::minirocket).
//!
//! **A line** ([`line`], `0x3352c0(at, vel)`): ALPHA 0x48 (additive), near 2 / far 2 (`+0x09` 0x22), frame
//! `*def[0x13]`, both colours `0x784040xx` (blue 0x40 + 6·`randi(6)`), its head `at`, its tail two moves behind
//! (`at − 2·vel`), its widths 0.09 and ×0.5 (`+0x1c`, `+0x2c`), its fade 6 a tick (`+0x3c`). Each tick
//! ([`update`], `0x3353d8`): both ends move by the velocity; the first colour's word goes down by the fade (truncated)
//! in its top byte and both colours take it; below 0 (the alpha run out) it is gone.
//!
//! Not ported: the gravity frames' bends (`0x380e08`).

use super::{rec, Particles};
use crate::rng::Rng;

/// Type 0x13.
pub const LINE: u8 = 0x13;

/// `0x3352c0(at, vel)` (module docs).
pub fn line(sys: &mut Particles, rng: &mut Rng, at: [f32; 3], vel: [f32; 3]) -> Option<usize> {
    let i = sys.create_part(LINE)?;
    let def = sys.def_first(LINE);
    let c = (rng.randi(6) * 6 + 0x40) as u32 | 0x7840_4000;
    let r = &mut sys.pool.recs[i];
    rec::set_u32(r, 0xc, c);
    rec::set_u32(r, 4, c);
    r[9] = 2 + 0x20;
    r[3] = 0x48;
    r[1] = 3;
    r[2] = def;
    rec::set_v3(r, 0x10, at);
    rec::set_v3(r, 0x30, vel);
    rec::set_v3(r, 0x20, std::array::from_fn(|k| at[k] - vel[k] - vel[k]));
    rec::set_ff(r, 0x3c, 6.0);
    rec::set_ff(r, 0x1c, 0.09);
    rec::set_ff(r, 0x2c, 0.5);
    Some(i)
}

/// `0x3353d8` (module docs).
pub fn update(sys: &mut Particles, i: usize) {
    let r = &mut sys.pool.recs[i];
    let v = rec::v3(r, 0x30);
    let (a, b) = (rec::v3(r, 0x10), rec::v3(r, 0x20));
    rec::set_v3(r, 0x10, std::array::from_fn(|k| a[k] + v[k]));
    rec::set_v3(r, 0x20, std::array::from_fn(|k| b[k] + v[k]));
    let fade = rec::ff(r, 0x3c) as i32;
    let c = (rec::u32(r, 4) as i32).wrapping_sub(fade.wrapping_mul(0x100_0000));
    rec::set_u32(r, 4, c as u32);
    rec::set_u32(r, 0xc, c as u32);
    if c < 0 { sys.kill_part(i); }
}
