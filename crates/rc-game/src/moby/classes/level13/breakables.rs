//! Level 13's breakables (L13 addresses; the engine functions named by their L13 addresses with the L00 ones the
//! port knows them by). Each waits in state 1 for a hit with the flags 0x4010000 (`0x327f60`, L00 `0x31dce8`) while
//! `+0x68`[0] = 0 and breaks in state 2; the hit record is let go every tick of state 1.
//!
//! * **0x130b** ([`update_130b`], `0x38db60`, 37 placed): every piece, the level's bolts.
//! * **0x130a** ([`update_130a`], `0x38d9c0`, 28 placed): a glow at its head; every piece, the bolts and a blast at
//!   the head.
//! * **0x130f** ([`update_130f`], `0x38de68`, 13 placed): every piece, the bolts and a blast.
//! * **0x130d** ([`update_130d`], `0x38dc58`, 11 placed): a glow; its class sound, four times the bolts, a blast and
//!   its four pieces thrown four times, a quarter turn apart.
//!
//! The engine functions: `0x3345e0` = L00 `0x32b580` ([`shards::throw_all`]), `0x330570` = L00 `0x327388`
//! ([`bolts::level_drop`]), `0x330680` = L00 `0x327498` ([`shards::break_into`]), `0x348fc0` = L00 `0x33f550`
//! (crate::particles::glow_def), `0x3fa180` = L00 `0x3f8348` ([`blast::burst`]), `0x32f248` a point in its frame
//! ([`frame_point`]), `0x36a110` the class sound, `0x2fa528` add_rot, `0x2f9d80` the rows from the angles.

use crate::hero::physics::{add, add_rot, euler_rows, V3};
use crate::moby::blast::{self, Blast};
use crate::moby::{bolts, shards, Ctx, MobyWorld};

/// The hit flags the breakables answer (`0x327f60(m, 0x4010000, 0)`).
const BREAK_FLAGS: u32 = 0x0401_0000;

/// State 1: a hit with [`BREAK_FLAGS`] while `+0x68`[0] = 0 breaks it; the record let go.
fn hit(w: &mut MobyWorld, id: usize) -> bool {
    let h = w.hit_message(id).is_some_and(|h| h.tmpl.flags & BREAK_FLAGS != 0) && w.mobys[id].pv_i(0) == 0;
    w.clear_hit(id);
    h
}

/// `0x32f248(x, y, z, m, out)`: a point in the moby's frame (its rows, unscaled) plus its place.
fn frame_point(w: &MobyWorld, id: usize, v: V3) -> V3 {
    let m = &w.mobys[id];
    add(std::array::from_fn(|k| v[0] * m.rows[0][k] + v[1] * m.rows[1][k] + v[2] * m.rows[2][k]), m.pos)
}

/// The blast the breakables give (`0x3fa180(size, 20, at, 0, 0x19, 0x14, 0x14, 0x14, 0x2d, 1, 1, 1, 0, 0)`): sprites
/// 0x19 / 0x14 / 0x14, 0x14 puffs, 0x2d streaks, its flash ball, no shake.
fn blast_at(w: &mut MobyWorld, ctx: &mut Ctx, at: V3, size: f32) {
    blast::burst(w, ctx, at, &Blast { size, bursts: [0x19, 0x14, 0x14], puffs: 0x14, streaks: 0x2d, light: true, shake: false });
}

/// `$gp−0x430` / `−0x438` / `−0x440`: 0x130b's pieces' velocity ranges (x / y −3..3, z 3..7), spin and gravity.
const H_130B: [f32; 8] = [-3.0, -3.0, 3.0, 0.0, 3.0, 3.0, 7.0, 0.0];
/// `$gp−0x470`: 0x130a's (z 3..9).
const H_130A: [f32; 8] = [-3.0, -3.0, 3.0, 0.0, 3.0, 3.0, 9.0, 0.0];
/// `$gp−0x3f0`: 0x130f's (as 0x130b's).
const H_130F: [f32; 8] = [-3.0, -3.0, 3.0, 0.0, 3.0, 3.0, 7.0, 0.0];
/// Every one's spin (90..360) and gravity (10..20).
const SPIN: (f32, f32) = (90.0, 360.0);
const GRAVITY: (f32, f32) = (10.0, 20.0);

/// The break of 0x130b / 0x130a / 0x130f: every piece pointed (`0x3345e0(m, h, spin, gravity, 0xffff, 0, −1, 0,
/// 0xffff, 0xff, 0)`: its class sound 0, the level's bolts once).
fn throw(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, h: [f32; 8]) {
    shards::throw_all(w, id, ctx, h, SPIN, GRAVITY, 0xffff, 0, -1, 0, 0xffff);
}

/// 0x130b (`0x38db60`). **0**: 1 (`0x325bd0`). **1**: the hit ([`hit`]) → 2. **2**: its pieces ([`throw`],
/// `$gp−0x430`); deleted.
pub fn update_130b(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => w.mobys[id].set_state(1, -1),
        1 => if hit(w, id) { w.mobys[id].set_state(2, -1) },
        2 => {
            throw(w, id, ctx, H_130B);
            w.delete(id);
        }
        _ => {}
    }
}

/// `$gp−0x450`: 0x130a's head in its frame.
const HEAD_130A: V3 = [0.0, 0.0, f32::from_bits(0x4074_49ba)];

/// 0x130a (`0x38d9c0`). **0**: a glow at its head (`0x348fc0`: 210000 of 0x50204040 pulled 0.75 at (0, 0, 3.817)
/// unturned, the def's frame 1, lasting, far its draw distance), 1. **1**: the hit ([`hit`]) → 2. **2**: its pieces
/// ([`throw`], `$gp−0x470`), the blast of size 0.3 ([`blast_at`]) at its head in its frame; deleted.
pub fn update_130a(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let m = &w.mobys[id];
            let (pos, far) = (m.pos, m.draw_dist);
            if let Some(sys) = ctx.parts.as_deref_mut() {
                crate::particles::glow_def(sys, &mut ctx.game.rng, 210_000.0, 0.75, HEAD_130A, id, pos, far, false, 0x5020_4040, -1, 1);
            }
            w.mobys[id].set_state(1, -1);
        }
        1 => if hit(w, id) { w.mobys[id].set_state(2, -1) },
        2 => {
            throw(w, id, ctx, H_130A);
            let at = frame_point(w, id, HEAD_130A);
            blast_at(w, ctx, at, 0.3);
            w.delete(id);
        }
        _ => {}
    }
}

/// 0x130f (`0x38de68`). **0**: 1. **1**: the hit ([`hit`]) → 2. **2**: its pieces ([`throw`], `$gp−0x3f0`), the
/// blast of size 0.3 ([`blast_at`]) at its place; deleted.
pub fn update_130f(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => w.mobys[id].set_state(1, -1),
        1 => if hit(w, id) { w.mobys[id].set_state(2, -1) },
        2 => {
            throw(w, id, ctx, H_130F);
            let pos = w.mobys[id].pos;
            blast_at(w, ctx, pos, 0.3);
            w.delete(id);
        }
        _ => {}
    }
}

/// 0x130d (`0x38dc58`). **0**: a glow (`0x348fc0`: 420000 of 0x40204080 pulled 1 at (0, 0, 2) unturned, the def's
/// frame 1, lasting, far its draw distance), 1. **1**: the hit ([`hit`]) → 2. **2**: its class sound 0 (`0x36a110`),
/// four times the level's bolts (`0x330570(0, m, 4)`), the blast of size 0.5 ([`blast_at`]) at its place; then four
/// times its yaw turned a quarter (`0x2fa528`), its rows rebuilt (`0x2f9d80`) and its pieces 0..3 thrown
/// (`0x330680(3, 6, 3, 6, m, 0xf, 0, 0xff)`: 3..6 a second across and up); deleted.
pub fn update_130d(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let m = &w.mobys[id];
            let (pos, far) = (m.pos, m.draw_dist);
            if let Some(sys) = ctx.parts.as_deref_mut() {
                crate::particles::glow_def(sys, &mut ctx.game.rng, 420_000.0, 1.0, [0.0, 0.0, 2.0], id, pos, far, false, 0x4020_4080, -1, 1);
            }
            w.mobys[id].set_state(1, -1);
        }
        1 => if hit(w, id) { w.mobys[id].set_state(2, -1) },
        2 => {
            ctx.host.play_sound(&w.mobys[id], 0, 0);
            bolts::level_drop(w, id, 0.0, 4, ctx);
            let pos = w.mobys[id].pos;
            blast_at(w, ctx, pos, 0.5);
            for _ in 0..4 {
                let m = &mut w.mobys[id];
                m.rot[2] = add_rot(m.rot[2], std::f32::consts::FRAC_PI_2);
                m.rows = euler_rows(m.rot);
                shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 6.0), 0xf, [0.0; 3], 0xff);
            }
            w.delete(id);
        }
        _ => {}
    }
}
