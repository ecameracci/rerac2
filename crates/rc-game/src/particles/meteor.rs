//! The Meteor Gun's particles (crate::moby::classes::meteor): the burst's streaks (type 0x6a, `0x341890` / `0x341a18`)
//! and embers (type 0x6b, `0x341bc8` / `0x341dc0`), the burning's smoke (type 0x6c, `0x342028` / `0x342230`). Each lives
//! two phases: its life (phase 0) then 20 ticks of fading (phase 1: its alpha the start's × (1 − t²)), `t` the share of
//! the phase's ticks gone; in phase 0 its colour runs from `0x103080` to its grey (`0x2f0000`) at its own alpha.
//!
//! **A streak** (`0x341890(speed, at, way, ALPHA, life)`): far 4 (`+0x09` 0x44), a random roll turning 1..4 a tick, frame
//! `*def[0x6a]`, a grey of 0x0f..0x1f, its alpha 0x5f..0x7f, its size 0.0625..0.125 → 0.5..0.6 (× 210000) over phase 0.
//! It moves along `way` at `speed` × `1 − (1 − t)²` in phase 0 (`0x31b918`), at `speed` in phase 1.
//!
//! **An ember** (`0x341bc8(far, at, way, ALPHA, life)`): as a streak, a grey of 0x0f..0x11 (`0x31b9e0(0xf, 0xb)`: `rand() % −3`), its size
//! 0.125..0.15 (× 105000) → 0.25..0.5 (× 210000); its speed 0.9..1.1 / 12 → 0..0.0125 (bytes in 128ths) over phase 0,
//! the end's in phase 1; it moves along `way` until it has gone `far`, then stays.
//!
//! **A smoke puff** (`0x342028(drift, at, up, across, ALPHA, life)`): a random roll turning 1..4 a tick, a grey of 0..0xb
//! (its nibble in each channel), its alpha 0x70..0x7f, its size 0.15..0.25 → 0.5..1 (× 128 · 1640.625), its rise 0.9..1.1
//! / 12 → 0..0.0125 over phase 0 (the end's in phase 1); it rises along `up` at that and drifts along `across` by `drift`
//! × t² (phase 1: `drift`).
//!
//! Not ported: the gravity frames' bends (`0x380e08`).

use super::{rec, Particles};
use crate::hud::lerp_rgba;
use crate::rng::Rng;

/// Type 0x6a: the burst's streaks.
pub const STREAK: u8 = 0x6a;
/// Type 0x6b: the burst's embers.
pub const EMBER: u8 = 0x6b;
/// Type 0x6c: the burning's smoke.
pub const SMOKE: u8 = 0x6c;
/// The colour phase 0 starts from.
const RGB: u32 = 0x10_3080;

fn add(a: [f32; 3], b: [f32; 3]) -> [f32; 3] { [a[0] + b[0], a[1] + b[1], a[2] + b[2]] }
fn scale(a: [f32; 3], k: f32) -> [f32; 3] { [a[0] * k, a[1] * k, a[2] * k] }
/// `0x2effb8` stored as a byte.
fn byte(x: f32) -> u8 { x as i32 as u8 }

/// The records' common start: place, ALPHA, far 4, a random roll, the frame, the life.
fn start(sys: &mut Particles, rng: &mut Rng, ty: u8, at: [f32; 3], alpha: u8, life: i16) -> Option<usize> {
    let i = sys.create_part(ty)?;
    let def = sys.def_first(ty);
    let roll = rng.randi(0xff) as u8;
    let r = &mut sys.pool.recs[i];
    rec::set_v3(r, 0x10, at);
    r[3] = alpha;
    r[1] = 0;
    r[9] = 4 + 0x40;
    r[8] = roll;
    r[2] = def;
    rec::set_i16(r, 0xa, life);
    Some(i)
}

/// `0x341890` (module docs).
pub fn streak(sys: &mut Particles, rng: &mut Rng, speed: f32, at: [f32; 3], way: [f32; 3], alpha: u8, life: i16) -> Option<usize> {
    let i = start(sys, rng, STREAK, at, alpha, life)?;
    let grey = rng.rand_range(0xf, 0x1f) as u32 * 0x1_0101;
    let turn = rng.rand_range(1, 4) as u8;
    let a = rng.rand_range(0x5f, 0x7f) as u8;
    let s0 = rng.randf(0.0625, 0.125) * 210_000.0;
    let s1 = rng.randf(0.5, f32::from_bits(0x3f19_999a)) * 210_000.0;
    let r = &mut sys.pool.recs[i];
    rec::set_v3(r, 0x20, way);
    rec::set_u32(r, 0x2c, grey);
    r[0x31] = life as u8;
    r[0x33] = 0;
    r[0x30] = turn;
    r[0x32] = a;
    rec::set_ff(r, 0x34, s0);
    rec::set_ff(r, 0x3c, speed);
    rec::set_ff(r, 0x38, s1);
    rec::set_u32(r, 4, (a as u32) << 24 | RGB);
    Some(i)
}

/// `0x341bc8` (module docs).
pub fn ember(sys: &mut Particles, rng: &mut Rng, far: f32, at: [f32; 3], way: [f32; 3], alpha: u8, life: i32) -> Option<usize> {
    let i = start(sys, rng, EMBER, at, alpha, life as i16)?;
    let grey = rng.rand_range(0xf, 0xb) as u32 * 0x1_0101;
    let turn = rng.rand_range(1, 4) as u8;
    let a = rng.rand_range(0x5f, 0x7f) as u8;
    let s0 = rng.randf(0.125, f32::from_bits(0x3e19_999a)) * 105_000.0;
    let s1 = rng.randf(0.25, 0.5) * 210_000.0;
    let v0 = byte(rng.randf(f32::from_bits(0x3f66_6666), f32::from_bits(0x3f8c_cccd)) * 10.666_667);
    let v1 = byte(rng.randf(0.0, f32::from_bits(0x3c4c_cccd)) * 128.0);
    let r = &mut sys.pool.recs[i];
    rec::set_v3(r, 0x20, way);
    rec::set_u32(r, 0x2c, grey);
    r[0x31] = life as u8;
    r[0x33] = 0;
    r[0x30] = turn;
    r[0x32] = a;
    rec::set_ff(r, 0x38, s0);
    rec::set_ff(r, 0x3c, s1);
    r[0x34] = byte(far * 128.0);
    r[0x35] = v0;
    r[0x36] = v1;
    r[0x37] = 0;
    rec::set_u32(r, 4, (a as u32) << 24 | RGB);
    Some(i)
}

/// `0x342028` (module docs).
#[allow(clippy::too_many_arguments)]
pub fn smoke(sys: &mut Particles, rng: &mut Rng, drift: f32, at: [f32; 3], up: [f32; 3], across: [f32; 3], alpha: u8, life: i32) -> Option<usize> {
    let i = start(sys, rng, SMOKE, at, alpha, life as i16)?;
    let grey = rng.rand_range(0, 0xb) as u8;
    let a = rng.rand_range(0, 0xf) as u32;
    let turn = rng.rand_range(1, 4) as u8;
    let s0 = byte(rng.randf(f32::from_bits(0x3e19_999a), 0.25) * 128.0);
    let s1 = byte(rng.randf(0.5, 1.0) * 128.0);
    let v0 = byte(rng.randf(f32::from_bits(0x3f66_6666), f32::from_bits(0x3f8c_cccd)) * 10.666_667);
    let v1 = byte(rng.randf(0.0, f32::from_bits(0x3c4c_cccd)) * 128.0);
    let r = &mut sys.pool.recs[i];
    rec::set_v3(r, 0x20, up);
    rec::set_v3(r, 0x30, across);
    r[0x2c] = grey | (a as u8) << 4;
    r[0x2e] = life as u8;
    r[0x2d] = turn << 4;
    r[0x3e] = s0;
    r[0x3f] = s1;
    r[0x3c] = v0;
    r[0x3d] = v1;
    r[0x2f] = byte(drift * 128.0);
    rec::set_u32(r, 4, (a + 0x70) << 24 | RGB);
    Some(i)
}

/// The life down one: phase 0 out goes to phase 1 for 20 ticks (`ticks` the phase's length byte, `phase` its flag
/// byte and bit); phase 1 out: gone.
fn age(sys: &mut Particles, i: usize, ticks: usize, phase: usize, bit: u8) {
    let r = &mut sys.pool.recs[i];
    let mut n = rec::i16(r, 0xa) as i32;
    if !crate::moby::countdown(&mut n) {
        rec::set_i16(r, 0xa, n as i16);
        return;
    }
    if r[phase] & bit == 0 {
        r[ticks] = 0x14;
        r[phase] |= bit;
        rec::set_i16(r, 0xa, 0x14);
    } else {
        rec::set_i16(r, 0xa, n as i16);
        sys.kill_part(i);
    }
}

/// `0x341a18` (module docs).
pub fn update_streak(sys: &mut Particles, i: usize) {
    let r = &mut sys.pool.recs[i];
    let way = rec::v3(r, 0x20);
    let t = 1.0 - rec::i16(r, 0xa) as f32 / r[0x31] as f32;
    let a = r[0x32];
    let step = if r[0x33] == 0 {
        rec::set_u32(r, 4, lerp_rgba(t, RGB, rec::u32(r, 0x2c)) | (a as u32) << 24);
        let (s0, s1) = (rec::ff(r, 0x34), rec::ff(r, 0x38));
        rec::set_ff(r, 0xc, s0 + (s1 - s0) * t);
        scale(way, rec::ff(r, 0x3c) * (1.0 - (1.0 - t) * (1.0 - t)))
    } else {
        let alpha = (a as f32 - a as f32 * (t * t)) as i32 as u32;
        rec::set_u32(r, 4, rec::u32(r, 0x2c) | alpha << 24);
        scale(way, rec::ff(r, 0x3c))
    };
    rec::set_v3(r, 0x10, add(rec::v3(r, 0x10), step));
    r[8] = r[8].wrapping_add(r[0x30]);
    age(sys, i, 0x31, 0x33, 1);
}

/// `0x341dc0` (module docs).
pub fn update_ember(sys: &mut Particles, i: usize) {
    let r = &mut sys.pool.recs[i];
    let way = rec::v3(r, 0x20);
    let v0 = r[0x35] as f32 * 0.007_812_5;
    let mut speed = r[0x36] as f32 * 0.007_812_5;
    let far = r[0x34] as f32 * 0.007_812_5;
    let gone = r[0x37] as f32;
    let t = 1.0 - rec::i16(r, 0xa) as f32 / r[0x31] as f32;
    let a = r[0x32];
    if r[0x33] & 1 == 0 {
        rec::set_u32(r, 4, lerp_rgba(t, RGB, rec::u32(r, 0x2c)) | (a as u32) << 24);
        speed = v0 + (speed - v0) * t;
        let (s0, s1) = (rec::ff(r, 0x38), rec::ff(r, 0x3c));
        rec::set_ff(r, 0xc, s0 + (s1 - s0) * t);
    } else {
        let alpha = (a as f32 - a as f32 * (t * t)) as i32 as u32;
        rec::set_u32(r, 4, rec::u32(r, 0x2c) | alpha << 24);
    }
    if r[0x33] & 2 == 0 {
        let p = add(rec::v3(r, 0x10), scale(way, speed));
        let total = gone * 0.007_812_5 + speed;
        if total <= far {
            rec::set_v3(r, 0x10, p);
            r[0x37] = byte(total * 128.0);
        } else {
            r[0x33] |= 2;
            rec::set_v3(r, 0x10, add(scale(way, far - total), p));
        }
    }
    r[8] = r[8].wrapping_add(r[0x30]);
    age(sys, i, 0x31, 0x33, 1);
}

/// `0x342230` (module docs).
pub fn update_smoke(sys: &mut Particles, i: usize) {
    let r = &mut sys.pool.recs[i];
    let (up, across) = (rec::v3(r, 0x20), rec::v3(r, 0x30));
    let flags = r[0x2d];
    let g = (r[0x2c] & 0xf) as u32;
    let grey = g | g << 16 | g << 8;
    let a0 = r[0x2c] >> 4 | 0x70;
    let (s0, s1) = (r[0x3e] as f32 * 1640.625, r[0x3f] as f32 * 1640.625);
    let v0 = r[0x3c] as f32 * 0.007_812_5;
    let mut rise = r[0x3d] as f32 * 0.007_812_5;
    let drift = r[0x2f] as f32;
    let t = 1.0 - rec::i16(r, 0xa) as f32 / r[0x2e] as f32;
    let side = if flags & 0xf == 0 {
        rec::set_u32(r, 4, lerp_rgba(t, RGB, grey) | (a0 as u32) << 24);
        rise = v0 + (rise - v0) * t;
        rec::set_ff(r, 0xc, s0 + (s1 - s0) * t);
        scale(across, drift * 0.007_812_5 * (t * t))
    } else {
        let alpha = (a0 as f32 - a0 as f32 * (t * t)) as i32 as u32;
        rec::set_u32(r, 4, grey | alpha << 24);
        scale(across, drift * 0.007_812_5)
    };
    let l = (up[0] * up[0] + up[1] * up[1] + up[2] * up[2]).sqrt();
    let lift = if 0.0 < l { scale(up, rise / l) } else { up };
    let p = add(rec::v3(r, 0x10), lift);
    rec::set_v3(r, 0x10, add(side, p));
    r[8] = r[8].wrapping_add(flags >> 4);
    age(sys, i, 0x2e, 0x2d, 1);
}
