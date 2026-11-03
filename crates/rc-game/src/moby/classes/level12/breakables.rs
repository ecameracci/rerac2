//! Level 12's breakables (L12 addresses; the engine functions named by their L12 addresses with the L00 ones the
//! port knows them by). Each waits in state 1 for a hit with the flags 0x4010000 (`0x321700`, L00 `0x31dce8`) while
//! `+0x68`[0] = 0 and breaks in state 2; the hit record is let go every tick of state 1.
//!
//! * **0x110c** ([`update_110c`], `0x421ca0`, 71 placed) and **0x1307** ([`update_1307`], `0x3892f8`, 20 placed):
//!   every piece, the bolts, then their stub (0x110d / 0x1308) left in their place.
//! * **0x1114** ([`update_1114`], `0x422578`, 26 placed): its pieces thrown two ways, the bolts, a blast.
//! * **0x0852** / **0x0884** ([`update_0852`], `0x3bd440`, 4 and 4 placed): animated from a random frame; 0x0852
//!   breaks into the pieces of 0x12a1 with a blast and chunks, 0x0884 never does.
//!
//! The engine functions: `0x32f888` = L00 `0x32b580` ([`shards::throw_all`]), `0x32b088` = L00 `0x327898`
//! (crate::moby::classes::props::spawn_in_place), `0x3fb0a0` = L00 `0x3f8348` ([`blast::burst`]), `0x32aaf0` = L00
//! `0x327648` ([`shards::throw_chunks`]), `0x328e08` a point in its frame, scaled ([`scaled_point`]), `0x365ed8` = L00
//! `0x3605f0` the class sound, `0x31ef20` = L00 `0x31b518` the sequence at a frame, `0x31f2d8` = L00 `0x31b8d0` the
//! state, `0x3150e0` the delete.

use crate::hero::physics::{add, scale, V3};
use crate::moby::blast::{self, Blast};
use crate::moby::{shards, Ctx, MobyWorld};

/// The hit flags the breakables answer (`0x321700(m, 0x4010000, 0)`).
const BREAK_FLAGS: u32 = 0x0401_0000;

/// State 1: a hit with [`BREAK_FLAGS`] while `+0x68`[0] = 0 (then its class sound 0 and state 2); the record let go.
fn wait_hit(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.hit_message(id).is_some_and(|h| h.tmpl.flags & BREAK_FLAGS != 0) && w.mobys[id].pv_i(0) == 0 {
        w.mobys[id].set_state(2, -1);
        ctx.host.play_sound(&w.mobys[id], 0, 0);
    }
    w.clear_hit(id);
}

/// `0x328e08(x, y, z, m, out)`: `v` through the moby's rows, times its scale, plus its place.
fn scaled_point(w: &MobyWorld, id: usize, v: V3) -> V3 {
    let m = &w.mobys[id];
    let r = &m.rows;
    add(scale(std::array::from_fn(|k| v[0] * r[0][k] + v[1] * r[1][k] + v[2] * r[2][k]), m.scale), m.pos)
}

/// `$gp+0x2eb0` / `+0x2eb8` / `+0x2ec0` (0x110c) and `$gp−0x400` / `−0x3f8` / `−0x3f0` (0x1307), the same values:
/// the pieces' gravity (10..13), spin (90..360) and velocity ranges (x / y −3..3, z 5..10).
const STUB_GRAVITY: (f32, f32) = (10.0, 13.0);
const STUB_SPIN: (f32, f32) = (90.0, 360.0);
const STUB_H: [f32; 8] = [-3.0, -3.0, 5.0, 0.0, 3.0, 3.0, 10.0, 0.0];

/// 0x110c / 0x1307's break: every piece (flags 0: the level's bolts; no sound), the stub `stub` in its place
/// (`0x32b088(m, stub, 0)`), deleted.
fn break_to_stub(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, stub: i16) {
    match w.mobys[id].state {
        0 => w.mobys[id].set_state(1, -1),
        1 => wait_hit(w, id, ctx),
        2 => {
            shards::throw_all(w, id, ctx, STUB_H, STUB_SPIN, STUB_GRAVITY, 0xffff, 0, -1, 0, 0xffff);
            super::super::props::spawn_in_place(w, id, stub, 0, &*ctx.host);
            w.delete(id);
        }
        _ => {}
    }
}

/// 0x110c (`0x421ca0`, 71 placed). **0**: 1. **1**: the hit ([`wait_hit`]) → 2. **2**: every piece, its stub 0x110d;
/// deleted ([`break_to_stub`]).
pub fn update_110c(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) { break_to_stub(w, id, ctx, 0x110d) }

/// 0x1307 (`0x3892f8`, 20 placed): 0x110c's code ([`update_110c`]) with its stub 0x1308.
pub fn update_1307(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) { break_to_stub(w, id, ctx, 0x1308) }

/// `$gp+0x2f10` / `+0x2f18`: 0x1114's pieces' gravity (10..20) and spin (90..360); `$gp+0x2f20` / `0x1b1f30`: the
/// velocity ranges of its piece 0 (x 2.5..10, y 5..6.5, z 4.5..10) and of pieces 1..8 (y −6.5..4.5).
const POST_GRAVITY: (f32, f32) = (10.0, 20.0);
const POST_SPIN: (f32, f32) = (90.0, 360.0);
const POST_H0: [f32; 8] = [2.5, 5.0, 4.5, 0.0, 10.0, 6.5, 10.0, 0.0];
const POST_H1: [f32; 8] = [2.5, -6.5, 4.5, 0.0, 10.0, 4.5, 10.0, 0.0];

/// 0x1114 (`0x422578`, 26 placed). **0**: 1. **1**: the hit ([`wait_hit`]) → 2. **2**: its piece 0 (`POST_H0`, flags
/// 0: the level's bolts) and pieces 1..8 (`POST_H1`, flags 1: no bolts), no sound, none skipped; at (0, 0, 15) in its
/// scaled frame ([`scaled_point`]) the blast (`0x3fb0a0`: size 20 × its scale, sprites 0x19 / 0x14 / 0x14, 0x14
/// puffs, 0x2d streaks, its flash ball, no shake); deleted.
pub fn update_1114(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => w.mobys[id].set_state(1, -1),
        1 => wait_hit(w, id, ctx),
        2 => {
            shards::throw_all(w, id, ctx, POST_H0, POST_SPIN, POST_GRAVITY, 0xffff, 0, -1, 0, 1);
            shards::throw_all(w, id, ctx, POST_H1, POST_SPIN, POST_GRAVITY, 0xffff, 0, -1, 1, 0x1fe);
            let at = scaled_point(w, id, [0.0, 0.0, 15.0]);
            let size = w.mobys[id].scale * 20.0;
            blast::burst(w, ctx, at, &Blast { size, bursts: [0x19, 0x14, 0x14], puffs: 0x14, streaks: 0x2d, light: true, shake: false });
            w.delete(id);
        }
        _ => {}
    }
}

/// `$gp+0xa90` / `+0xa98` / `+0xaa0`: 0x12a1's pieces' gravity (10..20), spin (90..360) and velocity ranges (x / y
/// 4..10 of either sign, z 8..12).
const CAN_GRAVITY: (f32, f32) = (10.0, 20.0);
const CAN_SPIN: (f32, f32) = (90.0, 360.0);
const CAN_H: [f32; 8] = [4.0, 4.0, 8.0, 0.0, 10.0, 10.0, 12.0, 0.0];

/// 0x0852 / 0x0884 (`0x3bd440`). **0**: its sequence 0 from a random frame of it (`0x31ef20`); 1. **1**: 0x0852
/// only: a hit with [`BREAK_FLAGS`] (no `+0x68` test): its class sound 0, 2; the record let go. **2**: a 0x12a1 in
/// its place (`0x32b088`) thrown into all its pieces (flags 0x16: x / y of either sign, the level's bolts and nine
/// times them again; no sound); the blast at its place (`0x3fb0a0`: size 1, sprites 0x19 / 0x14 / 0x14, 0x14 puffs,
/// 0x2d streaks, its flash ball, the shake); 0x14 chunks (`0x32aaf0`: reach 6, 3 a second, 1.5..7 of their size, balls,
/// (0, 0, 0.2) a tick, colour 0x80804040); the 0x12a1 and it deleted.
pub fn update_0852(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let n = ctx.host.seq_frames(&w.mobys[id], 0).unwrap_or(0) as i32;
            let f = ctx.game.rng.randi(n) as u8;
            ctx.host.set_anim(&w.mobys[id], 0, f, 0);
            w.mobys[id].set_state(1, -1);
        }
        1 => if w.mobys[id].o_class == 0x0852 {
            if w.hit_message(id).is_some_and(|h| h.tmpl.flags & BREAK_FLAGS != 0) {
                ctx.host.play_sound(&w.mobys[id], 0, 0);
                w.mobys[id].set_state(2, -1);
            }
            w.clear_hit(id);
        },
        2 => {
            let c = super::super::props::spawn_in_place(w, id, 0x12a1, 0, &*ctx.host);
            shards::throw_all(w, c, ctx, CAN_H, CAN_SPIN, CAN_GRAVITY, 0xffff, 0, -1, 0x16, 0xffff);
            let pos = w.mobys[id].pos;
            blast::burst(w, ctx, pos, &Blast { size: 1.0, bursts: [0x19, 0x14, 0x14], puffs: 0x14, streaks: 0x2d, light: true, shake: true });
            shards::throw_chunks(w, ctx, 6.0, 3.0, (1.5, 7.0), id, pos, 0x14, false, [0.0, 0.0, f32::from_bits(0x3e4c_ccce)], 0x8080_4040, None);
            w.delete(c);
            w.delete(id);
        }
        _ => {}
    }
}
