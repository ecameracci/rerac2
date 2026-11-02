//! The Lava Gun's particles (crate::moby::classes::lava_gun): its glowing sparks (type 0x19, `0x336148` /
//! `0x3362d0`) and its drops (type 0x6d, `0x3424d0` / `0x3425e8`).
//!
//! **A spark** (`0x336148(size, pos, vel, flip)`): its colour from `(r, g, b, a)` = (1, 1, 1, 0.6) (`0x31f6c8`: each
//! × 255; stored negated with `flip`), size `size` (negative: 5000..30000), far 4 (`+0x09` 0x44), additive, 80 ticks,
//! frame `*def[0x19]`. Each tick (`0x3362d0`): its size down 100; moving, the line to its next place (flags 2, the
//! world) — met, it stops there and its size doubles; else it moves and falls `+0x2c` (0: 0.0063) a tick; gone with
//! its alpha out, its ticks out, its size below 0 or its next place under 0.01 on an axis; else its green fades 0.03
//! and its red 0.01 a tick (flipped: the other way round), its blue 0.05, and it turns back a step.
//!
//! **A drop** (`0x3424d0(size, pos, vel, alpha)`): ALPHA `alpha` (0x48 / 0x44 in turn), far 4, a random roll, frame
//! `*def[0x6d]`, 5..50 ticks (unread), size `size`·525000, its alpha 0x40..0x80 (`+0x30`), its colour `0x103080`
//! darkened by 0..0.5. Each tick (`0x3425e8`): its alpha down one (out: gone), its roll on a step, it moves and grows
//! 1575, its velocity ×0.99 and, while its red is under 0x20 (smoke), 0.0015 up a tick, else 0.00075 down; its colour
//! 10% darker.
//!
//! Not ported: the gravity frames' bends (`0x380e08`) and their up (`0x326448`, `0x37ecc0`: world up here).

use super::{rec, Particles};
use crate::rng::Rng;

/// Type 0x19: the Lava Gun's sparks.
pub const SPARK: u8 = 0x19;
/// Type 0x6d: its drops.
pub const DROP: u8 = 0x6d;

/// `0x31f6c8(r, g, b, a)`: a colour word from four 0..1 floats (each × 255, truncated).
fn pack(r: f32, g: f32, b: f32, a: f32) -> u32 {
    let k = |x: f32| (x * 255.0) as i32 as u32;
    k(r) & 0xff | (k(g) & 0xff) << 8 | (k(b) & 0xff) << 16 | k(a) << 24
}

/// `0x336148(size, pos, vel, flip)` (module docs).
pub fn spark(sys: &mut Particles, rng: &mut Rng, size: f32, pos: [f32; 3], vel: [f32; 3], flip: bool) -> Option<usize> {
    let i = sys.create_part(SPARK)?;
    let def = sys.def_first(SPARK);
    let s = if size < 0.0 { rng.randf(5000.0, 30000.0) } else { size };
    let r = &mut sys.pool.recs[i];
    rec::set_v3(r, 0x10, pos);
    rec::set_u32(r, 4, pack(1.0, 1.0, 1.0, 0.6));
    let c = if flip { -1.0 } else { 1.0 };
    for o in [0x38, 0x30, 0x34] { rec::set_ff(r, o, c); }
    r[9] = 4 + 0x40;
    r[3] = 0x48;
    r[1] = 0;
    rec::set_ff(r, 0xc, s);
    r[8] = 0;
    r[2] = def;
    rec::set_ff(r, 0x3c, 0.6);
    rec::set_i16(r, 0xa, 0x50);
    rec::set_v3(r, 0x20, vel);
    Some(i)
}

/// `0x3362d0` (module docs). `coll`: the level's collision.
pub fn update_spark(sys: &mut Particles, i: usize, coll: Option<&rc_formats::collision::Collision>) {
    let r = &mut sys.pool.recs[i];
    let (mut cr, mut cg, mut cb) = (rec::ff(r, 0x30), rec::ff(r, 0x34), rec::ff(r, 0x38));
    let flip = cr < 0.0 || cg < 0.0 || cb < 0.0;
    if flip { (cr, cg, cb) = (-cr, -cg, -cb); }
    rec::set_ff(r, 0xc, rec::ff(r, 0xc) - 100.0);
    let pos = rec::v3(r, 0x10);
    let vel = rec::v3(r, 0x20);
    let next = [pos[0] + vel[0], pos[1] + vel[1], pos[2] + vel[2]];
    if vel != [0.0; 3] {
        match coll.and_then(|c| crate::collision::coll_line(c, pos, next, crate::collision::QueryFlags(2))) {
            None => {
                rec::set_v3(r, 0x10, next);
                let f = rec::ff(r, 0x2c);
                let f = if f == 0.0 { -0.0063 } else { f };
                rec::set_ff(r, 0x28, rec::ff(r, 0x28) + f);
            }
            Some(o) => {
                rec::set_v3(r, 0x10, o.point);
                rec::set_v3(r, 0x20, [0.0; 3]);
                rec::set_ff(r, 0xc, rec::ff(r, 0xc) * 2.0);
            }
        }
    }
    let mut life = rec::i16(r, 0xa) as i32;
    let out = rec::ff(r, 0x3c) <= 0.0 || crate::moby::countdown(&mut life);
    rec::set_i16(r, 0xa, life as i16);
    if out || rec::ff(r, 0xc) < 0.0 || next.iter().any(|&c| c < 0.01) {
        sys.kill_part(i);
        return;
    }
    if flip {
        cg = (cg - 0.01).max(0.0);
        cr = (cr - 0.03001).max(0.0);
    } else {
        cr = (cr - 0.01).max(0.0);
        cg = (cg - 0.03001).max(0.0);
    }
    cb = (cb - 0.05).max(0.0);
    if rec::ff(r, 0x3c) < 0.0 { rec::set_ff(r, 0x3c, 0.0); }
    rec::set_u32(r, 4, pack(cr, cg, cb, rec::ff(r, 0x3c)));
    let k = if flip { -1.0 } else { 1.0 };
    rec::set_ff(r, 0x30, cr * k);
    rec::set_ff(r, 0x34, cg * k);
    rec::set_ff(r, 0x38, cb * k);
    r[8] = r[8].wrapping_sub(1);
}

/// `0x3424d0(size, pos, vel, alpha)` (module docs).
pub fn drop(sys: &mut Particles, rng: &mut Rng, size: f32, pos: [f32; 3], vel: [f32; 3], alpha: u8) -> Option<usize> {
    let i = sys.create_part(DROP)?;
    let def = sys.def_first(DROP);
    let rot = rng.randi(0xff) as u8;
    let life = rng.rand_range(5, 0x32) as i16;
    let a = rng.rand_range(0x40, 0x80);
    let t = rng.randf(0.0, 0.5);
    let r = &mut sys.pool.recs[i];
    rec::set_v3(r, 0x10, pos);
    r[3] = alpha;
    r[1] = 0;
    r[9] = 4 + 0x40;
    r[8] = rot;
    r[2] = def;
    rec::set_i16(r, 0xa, life);
    rec::set_v3(r, 0x20, vel);
    rec::set_ff(r, 0xc, size * 525_000.0);
    rec::set_u32(r, 0x30, a as u32);
    rec::set_u32(r, 4, crate::hud::lerp_rgba(t, 0x10_3080, 0) | (a as u32) << 24);
    Some(i)
}

/// `0x3425e8` (module docs).
pub fn update_drop(sys: &mut Particles, i: usize) {
    let r = &mut sys.pool.recs[i];
    let a = (rec::u32(r, 0x30) as i32 - 1).max(0);
    rec::set_u32(r, 0x30, a as u32);
    r[8] = r[8].wrapping_add(1);
    let (p, v) = (rec::v3(r, 0x10), rec::v3(r, 0x20));
    rec::set_v3(r, 0x10, [p[0] + v[0], p[1] + v[1], p[2] + v[2]]);
    rec::set_ff(r, 0xc, rec::ff(r, 0xc) + 1575.0);
    let mut v = v.map(|x| x * f32::from_bits(0x3f7d_70a4));
    v[2] += if r[4] < 0x20 { f32::from_bits(0x3ac4_9ba6) } else { f32::from_bits(0xba44_9ba6) };
    rec::set_v3(r, 0x20, v);
    let c = crate::hud::lerp_rgba(f32::from_bits(0x3dcc_cccd), rec::u32(r, 4) & 0xff_ffff, 0);
    rec::set_u32(r, 4, c | (a as u32) << 24);
    if a == 0 { sys.kill_part(i); }
}
