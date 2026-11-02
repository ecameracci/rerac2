//! Particle type 0x1c, the dust at Ratchet's feet (L00 spawner `0x336888`, update `0x336b40` at `0x1fe6c0[0x1c]`):
//! a pale puff that brightens to 0.12 and fades again, turning as it drifts. crate::hero::posture arms it (the skid,
//! the landings) and [`crate::hero::posture::dust_pass`] puffs it at the feet; and type 0x46, his footprints on snow.

use super::{rec, Particles};
use crate::hero::physics::{add, V3};
use crate::rng::Rng;

/// Type 0x1c.
pub const PUFF: u8 = 0x1c;

/// `0x336888(size, at, rows)`: a puff at `at` (±0.03 across, through `rows` when given, 0.05 down) of a random pale
/// colour, alpha factor `randf(0, 0.0224)`, size `randf(10080, 19950)`, 252 ticks at most; drifting `±size` across
/// and 0.0025 down a tick (without rows), else `size` along `rows`' first row and 0.0025 down.
pub fn puff(sys: &mut Particles, rng: &mut Rng, size: f32, at: V3, rows: Option<[V3; 3]>) -> Option<usize> {
    let i = sys.create_part(PUFF)?;
    let k = f32::from_bits(0x3cf5_c28f);
    let off = [rng.randf(-k, k), rng.randf(-k, k), 0.0];
    let through = |v: V3, r: &[V3; 3]| -> V3 { std::array::from_fn(|j| v[0] * r[0][j] + v[1] * r[1][j] + v[2] * r[2][j]) };
    let mut pos = add(at, rows.as_ref().map_or(off, |r| through(off, r)));
    pos[2] -= 0.05;
    let a = match rng.rand() as i32 % 0x60 { 0 => 1, n => n };
    let b = match (rng.rand() as i32 % a) % 0x40 { 0 => 1, n => n };
    let c = (rng.rand() as i32 % b) % 0x30 + 0x6f;
    let frame = sys.def_first(PUFF);
    let fade = rng.randf(0.0, f32::from_bits(0x3cb7_0e1f)) + 1e-5;
    let big = rng.randf(f32::from_bits(0x461d_8000), f32::from_bits(0x469b_dc00));
    let down = f32::from_bits(0xbb23_d70a);
    let vel = match rows {
        None => [rng.randf(-size, size), rng.randf(-size, size), down],
        Some(r) => through([size, 0.0, down], &r),
    };
    let r = &mut sys.pool.recs[i];
    rec::set_v3(r, 0x10, pos);
    rec::set_u32(r, 4, (a + 0x8f) as u32 | ((b + 0x8f) as u32) << 8 | (c as u32) << 16);
    r[9] = 0x44;
    r[3] = 0x48;
    r[1] = 0;
    r[8] = 0;
    r[2] = frame;
    rec::set_ff(r, 0x24, fade);
    rec::set_ff(r, 0xc, big);
    rec::set_ff(r, 0x28, 0.0);
    rec::set_i16(r, 0xa, 0xfc);
    rec::set_ff(r, 0x2c, 1.0);
    rec::set_v3(r, 0x30, vel);
    Some(i)
}

/// `0x336b40`: the alpha factor `+0x24` moves by its way `+0x2c` · 0.007 (fading below 0.03: · 0.0014 and growing
/// 5460 more); at 0.0244 or less, or its ticks out, it is gone; at 0.12 it turns to fade. It grows 5460 a tick, its
/// alpha the factor · 255, its turn `+0x28` 0.002 a tick (the roll byte), and it drifts.
pub fn update(sys: &mut Particles, i: usize) {
    let r = &mut sys.pool.recs[i];
    let (way, a) = (rec::ff(r, 0x2c), rec::ff(r, 0x24));
    if 0.0 < way || 0.03 < a {
        rec::set_ff(r, 0x24, a + way * 0.007);
    } else {
        rec::set_ff(r, 0x24, a + way * 0.001_400_000_1);
        rec::set_ff(r, 0xc, rec::ff(r, 0xc) + 5460.0);
    }
    let mut n = rec::i16(r, 0xa);
    let out = rec::ff(r, 0x24) <= 0.0244 || n == 0 || { n -= 1; n == 0 };
    rec::set_i16(r, 0xa, n);
    if out {
        sys.kill_part(i);
        return;
    }
    if 0.12 <= rec::ff(r, 0x24) {
        let w = rec::ff(r, 0x2c);
        rec::set_ff(r, 0x2c, -w);
        rec::set_ff(r, 0x24, rec::ff(r, 0x24) - w * 0.007);
    }
    rec::set_ff(r, 0xc, rec::ff(r, 0xc) + 5460.0);
    let alpha = (rec::ff(r, 0x24) * 255.0) as i32 as u32;
    rec::set_u32(r, 4, rec::u32(r, 4) & 0xff_ffff | alpha << 24);
    let mut t = rec::ff(r, 0x28) + 0.002;
    if 1.0 < t { t -= 1.0; }
    rec::set_ff(r, 0x28, t);
    r[8] = (t * 255.0) as i32 as u8;
    let p = add(rec::v3(r, 0x10), rec::v3(r, 0x30));
    rec::set_v3(r, 0x10, p);
}

/// Type 0x46, the footprint (L00 spawner `0x33cd98`, update `0x33ce70` at `0x1fe6c0[0x46]`).
pub const PRINT: u8 = 0x46;

/// `0x33cd98(yaw, at, left)`: a grey print (`0x5a404040`) at `at`, the second frame of its type for the left foot,
/// its roll the yaw (`yaw · 256 / 2π − 0x20`), size 105000, 120 ticks before it fades.
pub fn footprint(sys: &mut Particles, yaw: f32, at: V3, left: bool) -> Option<usize> {
    let i = sys.create_part(PRINT)?;
    let frame = sys.def_frame(PRINT, left as usize);
    let r = &mut sys.pool.recs[i];
    rec::set_v3(r, 0x10, at);
    rec::set_u32(r, 4, 0x5a40_4040);
    r[9] = 0x21;
    r[3] = 0x44;
    r[1] = 1;
    r[2] = frame;
    r[8] = ((yaw * 40.743_664) as i32 as u8).wrapping_sub(0x20);
    rec::set_ff(r, 0xc, f32::from_bits(0x47cd_1400));
    rec::set_i16(r, 0xa, 0x78);
    Some(i)
}

/// `0x33ce70`: its ticks run down; at the last one its alpha drops a step every other tick, gone at 0.
pub fn update_print(sys: &mut Particles, i: usize) {
    let r = &mut sys.pool.recs[i];
    let n = rec::i16(r, 0xa);
    rec::set_i16(r, 0xa, n.wrapping_sub(1));
    if n == 1 {
        rec::set_i16(r, 0xa, 2);
        rec::set_u32(r, 4, rec::u32(r, 4).wrapping_sub(0x0100_0000));
    }
    if rec::u32(r, 4) & 0xff00_0000 == 0 { sys.kill_part(i); }
}
