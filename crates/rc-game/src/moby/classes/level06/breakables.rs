//! Level 6's breakables (L06 addresses; the engine functions named by their L06 addresses with the L00 ones the port
//! knows them by). Each waits in state 1 for a hit with the flags 0x4010000 (`0x35df18`, L00 `0x31dce8`) while
//! `+0x68`[0] = 0 and breaks in state 2; the hit record is let go every tick of state 1.
//!
//! * **0x12c8** ([`update_12c8`], `0x3c73b0`, 57 placed): 18 debris chunks, the level's bolts, its class sound 0.
//! * **0x1183** ([`update_1183`], `0x4637e0`, 39 placed): a lamp with two glows at its head; the hit throws its first
//!   piece along the hit, the break the rest, the explosion's embers and chunks out of the head.
//! * **0x11b1** ([`update_11b1`], `0x464c00`, 22 placed): a lamp with two glows; its break gives every piece, a blast
//!   and chunks.
//! * **0x0fe8** ([`update_0fe8`], `0x45d350`, 13 placed): bursts into a ring of chunks.
//! * **0x0fb5** ([`update_0fb5`], `0x458520`, 11 placed): spins with a looped sound; the last of its group gives 15
//!   times the bolts.
//! * **0x0fea** ([`update_0fea`], `0x45d708`, 6 placed): explodes into its pieces.
//! * **0x0912** ([`update_0912`], `0x403990`, 6 placed): breaks into its pieces.
//! * **0x0faa** ([`update_0faa`], `0x457890`, 4 placed): explodes, its pieces thrown one by one.
//!
//! The engine functions: `0x36c408` = L00 `0x32b580` ([`shards::throw_all`]), `0x367818` = L00 `0x327648`
//! ([`shards::throw_chunks`]), `0x367558` = L00 `0x327388` ([`bolts::level_drop`]), `0x380418` = L00 `0x33f550`
//! (crate::particles::glow_def), `0x360fb0` = L00 `0x320538` ([`blast::explode`]), `0x43b580` = L03 `0x403f10` (the
//! level's blast wrapper, [`blast::burst`]), `0x367668` = L00 `0x327498` ([`shards::break_into`]), `0x31b598` = L00
//! `0x2dbdb0` ([`shards::spawn_shard`] / [`shards::spawn_ball`]), `0x3a1c98` the class sound, `0x3662f8` a point in
//! its frame ([`frame_point`]).

use crate::hero::physics::{add, scale, sub, V3};
use crate::moby::blast::{self, Blast, Explosion};
use crate::moby::{bolts, shards, Ctx, MobyWorld};

/// The hit flags the breakables answer (`0x35df18(m, 0x4010000, 0)`).
const BREAK_FLAGS: u32 = 0x0401_0000;

/// `0x32e8f0(out, v, m + 0xc0)`: `v` through the moby's rows.
fn through(r: &[V3; 3], v: V3) -> V3 { std::array::from_fn(|k| v[0] * r[0][k] + v[1] * r[1][k] + v[2] * r[2][k]) }

/// `0x3662f8(x, y, z, m, out)`: a point in the moby's frame (its rows, unscaled) plus its place.
fn frame_point(w: &MobyWorld, id: usize, v: V3) -> V3 {
    let m = &w.mobys[id];
    add(through(&m.rows, v), m.pos)
}

/// State 1: a hit with [`BREAK_FLAGS`] while `+0x68`[0] = 0 (the record, when it breaks it); the record let go.
fn hit(w: &mut MobyWorld, id: usize) -> Option<crate::moby::hits::HitRecord> {
    let h = w.hit_message(id).filter(|h| h.tmpl.flags & BREAK_FLAGS != 0 && w.mobys[id].pv_i(0) == 0);
    w.clear_hit(id);
    h
}

/// Chunks out of the four corners `(±d, ±d, 0.11667)` (through its rows, `0x32e8f0`) of a head at `at`: `0x367818(reach,
/// 5, s₀, s₁, m, at, 3, pointed, v, colour, 0)` from each.
fn corner_chunks(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, at: V3, d: f32, reach: f32, s: (f32, f32), colour: u32) {
    for i in 0..4 {
        let (x, y) = ((i / 2 * 2 - 1) as f32, (i % 2 * 2 - 1) as f32);
        let v = through(&w.mobys[id].rows, [x * d, y * d, f32::from_bits(0x3dee_eef0)]);
        shards::throw_chunks(w, ctx, reach, 5.0, s, id, at, 3, true, v, colour, None);
    }
}

/// 0x12c8 (`0x3c73b0`). **0**: 1. **1**: the hit ([`hit`]) → 2. **2**: 18 debris chunks at its place (`0x367818`:
/// reach 1.5, 3 a second, 0.75..1.5 of their size, pointed, falling (0, 0, −0.05) a tick, colour 0x80404040), the
/// level's bolts (`0x367558`, at its place), its class sound 0 (`0x3a1c98`); deleted.
pub fn update_12c8(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => w.mobys[id].set_state(1, -1),
        1 => if hit(w, id).is_some() { w.mobys[id].set_state(2, -1) },
        2 => {
            let pos = w.mobys[id].pos;
            shards::throw_chunks(w, ctx, 1.5, 3.0, (0.75, 1.5), id, pos, 0x12, true, [0.0, 0.0, f32::from_bits(0xbd4c_ccce)], 0x8040_4040, None);
            bolts::level_drop(w, id, 0.0, 1, ctx);
            ctx.host.play_sound(&w.mobys[id], 0, 0);
            w.delete(id);
        }
        _ => {}
    }
}

/// `$gp+0x2bc0` / `+0x2bc8`: 0x1183's pieces' gravity (10..20) and spin (90..360).
const LAMP_GRAVITY: (f32, f32) = (10.0, 20.0);
const LAMP_SPIN: (f32, f32) = (90.0, 360.0);
/// `$gp+0x2bd0`: 0x1183's pieces' velocity ranges (x / y 1..3 of either sign, z 3..9).
const LAMP_H: [f32; 8] = [1.0, 1.0, 3.0, 0.0, 3.0, 3.0, 9.0, 0.0];

/// 0x1183 (`0x4637e0`), its head `21.5 × its scale` up its frame. **0**: two glows at the head (`0x380418`, offset
/// unturned, far 0x60, lasting): 430500 of 0x50801420 (the def's frame 0) and 73500 of 0x40801420 pulled 0.75 (frame
/// 1); 1. **1**: a hit ([`hit`]): its piece 0 thrown along the hit (the hit's direction with z 0.75, × 6..8, ±(3, 3, 0)
/// about it, unturned; flags 9: no bolts; every piece pointed, none skipped, no sound), 2. **2**: pieces 1..5
/// (`$gp+0x2bd0`, flags 7: x / y of either sign, no bolts here), its class sound 0; the level's bolts a third of the
/// head's height up; at the head the explosion (`0x360fb0`: near 9, size 0.35, 8 embers, 3 billows, 3 flares, flags
/// 0x810001) and, out of the head's four corners (±1/60), three chunks each (reach 0, 0.45..0.6 of their size, colour
/// 0x7fff8080); deleted.
pub fn update_1183(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let head = w.mobys[id].scale * 21.5;
    match w.mobys[id].state {
        0 => {
            let m = &w.mobys[id];
            let pos = m.pos;
            if let Some(sys) = ctx.parts.as_deref_mut() {
                crate::particles::glow_def(sys, &mut ctx.game.rng, 430_500.0, 0.0, [0.0, 0.0, head], id, pos, 0x60, false, 0x5080_1420, -1, 0);
                crate::particles::glow_def(sys, &mut ctx.game.rng, 73_500.0, 0.75, [0.0, 0.0, head], id, pos, 0x60, false, 0x4080_1420, -1, 1);
            }
            w.mobys[id].set_state(1, -1);
        }
        1 => {
            if let Some(h) = hit(w, id) {
                let d = [h.tmpl.dir[0], h.tmpl.dir[1], 0.75];
                let v = scale(d, ctx.game.rng.randf(6.0, 8.0));
                let (lo, hi) = (sub(v, [3.0, 3.0, 0.0]), add(v, [3.0, 3.0, 0.0]));
                let hr = [lo[0], lo[1], lo[2], 0.0, hi[0], hi[1], hi[2], 0.0];
                shards::throw_all(w, id, ctx, hr, LAMP_SPIN, LAMP_GRAVITY, 0xffff, -1, -1, 9, 1);
                w.mobys[id].set_state(2, -1);
            }
        }
        2 => {
            shards::throw_all(w, id, ctx, LAMP_H, LAMP_SPIN, LAMP_GRAVITY, 0xffff, 0, -1, 7, 0x3e);
            bolts::level_drop(w, id, head * 0.333_333_34, 1, ctx);
            let at = frame_point(w, id, [0.0, 0.0, head]);
            let e = Explosion { near: 9.0, size: 0.35, embers: 8, billows: 3, flares: 3, sound: -1, lod: -1, flags: 0x81_0001, ..Default::default() };
            blast::explode(w, ctx, Some(id), Some(at), &e);
            corner_chunks(w, id, ctx, at, 0.016_666_668, 0.0, (0.45, 0.6), 0x7fff_8080);
            w.delete(id);
        }
        _ => {}
    }
}

/// `$gp+0x2c70` / `+0x2c78` / `+0x2c80`: 0x11b1's pieces' gravity, spin and velocity ranges (x / y −3..3, z 4..7).
const LAMP2_GRAVITY: (f32, f32) = (10.0, 20.0);
const LAMP2_SPIN: (f32, f32) = (90.0, 360.0);
const LAMP2_H: [f32; 8] = [-3.0, -3.0, 4.0, 0.0, 3.0, 3.0, 7.0, 0.0];
/// `$gp+0x2ca0`: where 0x11b1's blast goes in its frame.
const LAMP2_BLAST: V3 = [0.0, 0.0, 3.3];

/// 0x11b1 (`0x464c00`). **0**: two glows (`0x380418`, far its draw distance, lasting), both `525000 × r` pulled 0.75
/// at `(0, 0, 3.75 × r)` (`r` its scale over its class's): 0x300033ff (the def's frame 1) and 0x280033ff (frame 0);
/// its glow word `+0x78` 0xffffc0c0; 1, and on into 1's test the same tick. **1**: the hit ([`hit`]) → 2. **2**:
/// every piece (`$gp+0x2c80`, flags 6: x / y of either sign; the level's bolts; its class sound 0), the blast
/// (`0x43b580`: size 1, sprites 5 / 5 / 5, 0x14 puffs, 5 streaks, its light) at (0, 0, 3.3) in its frame, and out of
/// that point's four corners (±0.05) three chunks each (reach 0.2, 1.45..2.2 of their size, colour 0x7f40ffff);
/// deleted.
pub fn update_11b1(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let m = &w.mobys[id];
            let class_scale = w.class(m.o_class).map_or(1.0, |c| c.scale);
            let r = m.scale / class_scale;
            let (pos, far) = (m.pos, m.draw_dist);
            if let Some(sys) = ctx.parts.as_deref_mut() {
                crate::particles::glow_def(sys, &mut ctx.game.rng, r * 525_000.0, 0.75, [0.0, 0.0, r * 3.75], id, pos, far, false, 0x3000_33ff, -1, 1);
                crate::particles::glow_def(sys, &mut ctx.game.rng, r * 525_000.0, 0.75, [0.0, 0.0, r * 3.75], id, pos, far, false, 0x2800_33ff, -1, 0);
            }
            let m = &mut w.mobys[id];
            m.glow = 0xffff_c0c0;
            m.set_state(1, -1);
            if hit(w, id).is_some() { w.mobys[id].set_state(2, -1); }
        }
        1 => if hit(w, id).is_some() { w.mobys[id].set_state(2, -1) },
        2 => {
            shards::throw_all(w, id, ctx, LAMP2_H, LAMP2_SPIN, LAMP2_GRAVITY, 0xffff, 0, -1, 6, 0xff);
            let at = frame_point(w, id, LAMP2_BLAST);
            blast::burst(w, ctx, at, &Blast { size: 1.0, bursts: [5, 5, 5], puffs: 0x14, streaks: 5, light: true, shake: false });
            corner_chunks(w, id, ctx, at, 0.050_000_004, 0.2, (1.45, 2.2), 0x7f40_ffff);
            w.delete(id);
        }
        _ => {}
    }
}

/// A hit with `flags` whose source is not a 0x0fe5 (`+0x20`'s class), the record let go either way.
fn hit_not_0fe5(w: &mut MobyWorld, id: usize, flags: u32) -> bool {
    let h = w.hit_message(id).filter(|h| h.tmpl.flags & flags != 0);
    w.clear_hit(id);
    h.is_some_and(|h| !h.tmpl.source.and_then(|s| w.mobys.get(s)).is_some_and(|s| s.o_class == 0x0fe5))
}

/// 0x0fe8 (`0x45d350`, 13 placed). **0**: mode | 0x800, 1. **Each tick**: a hit with [`BREAK_FLAGS`] not from a
/// 0x0fe5: its chunks ([`ring_chunks`]: 14 at the full frame rate, 3 while either frame load `$gp+0x335c` /
/// `0x1b2350` is 0.9 or more, which the port never is), its class sound 0, deleted.
pub fn update_0fe8(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 0 {
        let m = &mut w.mobys[id];
        m.state = 1;
        m.mode |= 0x800;
    }
    if hit_not_0fe5(w, id, BREAK_FLAGS) {
        ring_chunks(w, id, ctx, (0.0, f32::from_bits(0x4050_d4fe)), f32::from_bits(0x3ffc_ac08), f32::from_bits(0x3dee_eef0), 0xe);
        ctx.host.play_sound(&w.mobys[id], 0, 0);
        w.delete(id);
    }
}

/// `0x45d448(z₀, z₁, r, speed, m, n)`: `n` chunks, each at a random heading `r` out from its place and `z₀..z₁` up,
/// moving out along (that heading, `0..1` up) at `0.75..1.25 × speed` a tick (`0x32e710`), one chunk there
/// (`0x367818`: reach 0, no speed of its own, 1..5.5 of its size, pointed, colour 0x50808040).
fn ring_chunks(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, z: (f32, f32), r: f32, speed: f32, n: i32) {
    use std::f32::consts::PI;
    for _ in 0..n {
        let rng = &mut ctx.game.rng;
        let a = rng.randf(-PI, PI);
        let mut p = [r * a.cos(), r * a.sin(), rng.randf(0.0, 1.0)];
        let v = crate::hero::physics::set_len(p, rng.randf(speed * 0.75, speed * 1.25));
        p[2] = rng.randf(z.0, z.1);
        let at = add(w.mobys[id].pos, p);
        shards::throw_chunks(w, ctx, 0.0, 0.0, (1.0, 5.5), id, at, 1, true, v, 0x5080_8040, None);
    }
}

/// 0x0fb5 (`0x458520`, 11 placed): its pvars `[0]` unbreakable, `[4]` its spin (°/s), `[8]` its group, `[0xc]` its
/// looped sound. **0**: 1. **1**: its class sound 1 looped (`0x3a1c98(1, 4)`) while not playing; its roll `+0xf8`
/// turned by the spin (`0x32f438`); a hit ([`hit`]): its class sound 0, the loop stopped (`0x3a1998`), 2. **2**: the
/// last of its group alive (`0x35caf8(group, −1)` = 1, crate::moby::classes::props::group_alive) gives 15 times the
/// level's bolts; its pieces 0..4 thrown (`0x367668`: 3..6 a second across and up, plus (−0.1167, 0, 0) through its
/// rows); deleted.
pub fn update_0fb5(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => w.mobys[id].set_state(1, -1),
        1 => {
            let h = w.mobys[id].pv_i(0xc);
            if !(h != -1 && ctx.host.sound_playing(h)) {
                let h = ctx.host.play_sound(&w.mobys[id], 1, 4);
                w.mobys[id].set_pv_i(0xc, h);
            }
            let m = &mut w.mobys[id];
            m.rot[2] = crate::hero::physics::add_rot(m.rot[2], m.pv_f(4) * 0.000_290_888_25);
            if hit(w, id).is_some() {
                ctx.host.play_sound(&w.mobys[id], 0, 0);
                w.mobys[id].set_state(2, -1);
                let h = w.mobys[id].pv_i(0xc);
                if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
                w.mobys[id].set_pv_i(0xc, -1);
            }
        }
        2 => {
            if super::super::props::group_alive(w, w.mobys[id].pv_i(8)) == 1 { bolts::level_drop(w, id, 0.0, 0xf, ctx); }
            let v = through(&w.mobys[id].rows, [f32::from_bits(0xbdee_eef0), 0.0, 0.0]);
            shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 6.0), 0x1f, v, 0xff);
            w.delete(id);
        }
        _ => {}
    }
}

/// 0x0fea (`0x45d708`, 6 placed): a hit with the flags 0x4010001 not from a 0x0fe5 breaks it: its class sound 0, the
/// explosion at its place (`0x360fb0`: flashes 4 and 2 past 9, size 0, light 15, 10 embers, 3 billows, 9 flares, the
/// shake, one debris piece, flags 0x810001), its piece 6 as a ball (`0x31b598` mode 1: `1/30..1/15` a tick a random
/// way, upward, spin ±20..45°/s, gravity 15, its bounce sound 1) and pieces 0..5 pointed (`0.1..0.2` a tick, upward,
/// spin ±20..45, gravity 15..25), its class sound 1, the level's bolts; deleted. The record let go every tick. The
/// ball's update `0x45d968` is crate::moby::shards::update's.
pub fn update_0fea(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if !hit_not_0fe5(w, id, 0x0401_0001) { return; }
    ctx.host.play_sound(&w.mobys[id], 0, 0);
    let e = Explosion { flash: 4.0, flash2: 2.0, near: 9.0, size: 0.0, light: 15.0, embers: 10, billows: 3, flares: 9, sound: -1, shake: true, debris: 1, lod: -1, flags: 0x81_0001, ..Default::default() };
    blast::explode(w, ctx, Some(id), None, &e);
    let (o_class, pos, rot) = { let m = &w.mobys[id]; (m.o_class, m.pos, m.rot) };
    let rng = &mut ctx.game.rng;
    let mut v = rng.rand_vec(f32::from_bits(0x3d08_8889), f32::from_bits(0x3d88_8889));
    v[2] = v[2].abs();
    let spin = rng.randf_signed(20.0, 45.0);
    shards::spawn_ball(w, ctx, o_class, Some(id), pos, rot, spin, 15.0, 6, v, 1);
    for k in 0..6 {
        let rng = &mut ctx.game.rng;
        let mut v = rng.rand_vec(f32::from_bits(0x3dcc_ccce), f32::from_bits(0x3e4c_ccce));
        v[2] = v[2].abs();
        let spin = rng.randf_signed(20.0, 45.0);
        let gravity = rng.randf(15.0, 25.0);
        shards::spawn_shard(w, ctx, o_class, Some(id), pos, rot, None, spin, gravity, k, v, 0xff);
    }
    ctx.host.play_sound(&w.mobys[id], 1, 0);
    bolts::level_drop(w, id, 0.0, 1, ctx);
    w.delete(id);
}

/// `$gp+0xfe0` / `+0xfe8` / `+0xff0`: 0x0912's pieces' gravity, spin and velocity ranges (x / y 1..3, z 4..9).
const POT_GRAVITY: (f32, f32) = (10.0, 20.0);
const POT_SPIN: (f32, f32) = (90.0, 360.0);
const POT_H: [f32; 8] = [1.0, 1.0, 4.0, 0.0, 3.0, 3.0, 9.0, 0.0];

/// 0x0912 (`0x403990`, 6 placed). **0**: mirrored (mode 0x8000) with `+0x68`[4] ≠ 0; 1. **1**: the hit ([`hit`]) →
/// 2. **2**: every piece (`$gp+0xff0`, flags 6: x / y of either sign; the level's bolts; its class sound 0); deleted.
///
/// Not ported: `0x403780`'s ten mobys of class 0x11b7 (`0x465358`: thrown out of it at 8..14 a second), a class the
/// level does not load (no update, no model), so they would only fill moby slots.
pub fn update_0912(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            if m.pv_i(4) != 0 { m.mode |= crate::moby::mode::MIRROR; }
            m.set_state(1, -1);
        }
        1 => if hit(w, id).is_some() { w.mobys[id].set_state(2, -1) },
        2 => {
            shards::throw_all(w, id, ctx, POT_H, POT_SPIN, POT_GRAVITY, 0xffff, 0, -1, 6, 0xffff);
            w.delete(id);
        }
        _ => {}
    }
}

/// 0x0faa (`0x457890`, 4 placed): a hit with [`BREAK_FLAGS`] not from a 0x0fe5 breaks it: its class sound 0, the
/// explosion of 0x0fea ([`update_0fea`]), then its pieces 0 and 2..6 each thrown on its own (`0x367668`: 3..6 a
/// second across and up, plus `1/15..0.1` a tick a random way, upward × 1.5); deleted. The record let go every tick.
pub fn update_0faa(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if !hit_not_0fe5(w, id, BREAK_FLAGS) { return; }
    ctx.host.play_sound(&w.mobys[id], 0, 0);
    let e = Explosion { flash: 4.0, flash2: 2.0, near: 9.0, size: 0.0, light: 15.0, embers: 10, billows: 3, flares: 9, sound: -1, shake: true, debris: 1, lod: -1, flags: 0x81_0001, ..Default::default() };
    blast::explode(w, ctx, Some(id), None, &e);
    for mask in [1, 4, 8, 0x10, 0x20, 0x40] {
        let mut v = ctx.game.rng.rand_vec(f32::from_bits(0x3d88_8889), f32::from_bits(0x3dcc_ccce));
        v[2] = v[2].abs() * 1.5;
        shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 6.0), mask, v, 0xff);
    }
    w.delete(id);
}
