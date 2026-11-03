//! Level 11's birds 0x1105 (`0x471da8`, 38 placed; L11 addresses): they peck about their place, waddle off when
//! Ratchet comes near and fly away when he fires.

use super::super::props::{bob, seq, set_seq, wrapped};
use crate::hero::physics::{add, add_rot, atan2, dist, dist2, V3};
use crate::moby::{ai, Ctx, MobyWorld};

/// The movement block (`+0x20`; its pointer word `+0x18`).
const MV: usize = 0x20;
/// `+0x110` its place, `+0x120` the point it walks to, `+0x130` its first state, `+0x134` the way off (°, from its
/// facing), `+0x138` its speed, `+0x13c` the turn's speed, `+0x140` its path (−1 none), `+0x144` the point on it,
/// `+0x148` / `+0x14c` the bob's phase and offset, `+0x150` it does not fly.
const HOME: usize = 0x110;
const GOAL: usize = 0x120;
const FIRST: usize = 0x130;
const WAY: usize = 0x134;
const SPEED: usize = 0x138;
const TURN_V: usize = 0x13c;
const PATH: usize = 0x140;
const POINT: usize = 0x144;
const PHASE: usize = 0x148;
const NO_FLY: usize = 0x150;
/// `$gp+0x39d8` / `+0x39dc`: the bob's height and speed (°/s).
const BOB: (f32, f32) = (0.25, 300.0);
/// `0x403de450`: the pitch of its way off (170°: back along the way, a little up).
const PITCH: f32 = f32::from_bits(0x403d_e450);
const DEG: f32 = 0.017_453_294;

/// `0x337298(r, yaw, pitch, out)`.
fn polar(r: f32, yaw: f32, pitch: f32) -> V3 { [yaw.cos() * r * pitch.cos(), yaw.sin() * r * pitch.cos(), pitch.sin() * r] }

/// `0x32df40(r, out)`: a point in the disc of radius `r` about 0 (an angle, `(1 − f²)·r` out), z 0.
fn disc(ctx: &mut Ctx, r: f32) -> V3 {
    let a = ctx.game.rng.rand_angle();
    let f = ((ctx.game.rng.rand() >> 16) & 0xfff) as f32 * 0.000_244_140_62;
    let r = (1.0 - f * f) * r;
    [a.cos() * r, a.sin() * r, 0.0]
}

/// A new point to walk to, within 4 of its place.
fn new_goal(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let d = disc(ctx, 4.0);
    let m = &mut w.mobys[id];
    let g = add(m.pv_v(HOME), d);
    m.set_pv_v(GOAL, g);
}

/// The sequence the animation blends from (`+0x42`).
fn seq_from(w: &MobyWorld, id: usize, ctx: &Ctx) -> u8 { ctx.host.anim(&w.mobys[id]).map_or(0xff, |a| a.seq_a) }

/// 0x1105 (module docs). Not while the arena's chunk is in (`0x1caec0`). Its movement block: radius 0.4, height 0.4,
/// steps 0.6, slope 85°, gravity 1/720, the turn 85° before it walks, stop 0.25 (`0x340930(0.000555, 1/60, 0.00116,
/// 6°)`, crate::moby::ai::senses).
/// * **0**: its scale × 1.5, its place kept, a goal ([`new_goal`]), its first state (`+0x130`), update distance 0xff,
///   its sequence 0 from a random frame (`randi(8)`, blend `randi(18)`); on a path (and not first in 8): its scale × 1.5
///   again, sequence 4 (frame `randi(3)`, blend 10..17), **7**.
/// * **2** (sitting): Ratchet within 9 → sequence 3, **4**. Else Ratchet within 64 across pressing ○ with an item in
///   hand other than 10 and 0xd, and it flies (`+0x150` 0): its way off 170..190°, sequence 4, speed 1/12, **6**.
/// * **4** (startled): its sequence over → its way off, sequence 4, speed 1/30, **5**. **5** (waddling off): speed up by
///   1/120 to 7/60 a tick, the move by its velocity ([`ai::move_by`], `0x33fa98`; held back, it turns about); past 64
///   across from Ratchet, deleted; it bobs. **6** (flying off): as fast as 5/12, straight on (no collision); past 64,
///   deleted.
/// * **7** (on its path): 7/60 a tick along its facing pitched to the point, its facing turned toward it
///   (`0x32e468(…, 0.0001, 0.001, 0.02)`); within 0.5 across the next (held at the last); sequence 4; it bobs.
/// * **8** (wandering): sequence 2, the move toward its goal ([`ai::move_step`]); at it, stuck or after 240 ticks a new
///   goal, and one time in four (`randi(4)` = 1) sequence 1 and **9** (pecking). Ratchet within 10 → sequence 2, **4**.
///   **9**: its sequence over → **8**; Ratchet within 10 → **4**.
///
/// Each tick its ticks in the state count (`+0x96`); its sequence 4 plays at 2.5 (`+0x48`), any other but 0 at 1. A
/// hit (flags 0x10000, `0x330218`): the birds' kill count (`$gp−0x334`) up, deleted. Not ported: the 13 feathers it
/// leaves (class 0x1ac, `0x398140`: no port), its head's aim (`0x3395b0(2.5, m, +0x160, 0, 1)`, L00 `0x327308`).
pub fn update_1105(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if ctx.game.variant != 0 { return; }
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.set_pv_i(0x18, MV as i32);
            m.scale *= 1.5;
            let p = m.pos;
            m.set_pv_v(HOME, p);
            for (o, v) in [(0x24, 0x3ecc_cccd), (0x54, 0x3e80_0000), (0x2c, 0x3f19_999a), (0x50, 0x3fbd_e450), (0x4c, 0x3ab6_0b61), (0x20, 0x3ecc_cccd), (0x28, 0x3f19_999a), (0x30, 0x3fbd_e450)] {
                m.set_pv_i(o, v);
            }
            ai::senses(m, f32::from_bits(0x3a11_a2b4), f32::from_bits(0x3c88_8889), f32::from_bits(0x3a98_825c), f32::from_bits(0x3dd6_7752));
            new_goal(w, id, ctx);
            let m = &mut w.mobys[id];
            let first = m.pv_i(FIRST);
            m.set_state(first as u8, -1);
            m.update_dist = 0xff;
            let frame = ctx.game.rng.randi(8) as u8;
            let blend = ctx.game.rng.randi(0x12) as u8;
            ctx.host.set_anim(&w.mobys[id], 0, frame, blend);
            let m = &mut w.mobys[id];
            m.set_pv_i(POINT, 0);
            if m.pv_i(PATH) != -1 && first != 8 {
                m.scale *= 1.5;
                if seq(w, id, ctx) != 4 {
                    let frame = ctx.game.rng.randi(3) as u8;
                    let blend = ctx.game.rng.randi(8) as u8 + 10;
                    ctx.host.set_anim(&w.mobys[id], 4, frame, blend);
                }
                w.mobys[id].set_state(7, -1);
            }
        }
        2 => {
            if dist(w.mobys[id].pos, ctx.hero.pos) < 9.0 {
                set_seq(w, id, ctx, 3, 0, 10);
                w.mobys[id].set_state(4, -1);
            } else {
                let hand = ctx.hero.items.item;
                let fire = hand != 10 && hand != 0xd && ctx.hero.items.pressed & crate::pad::button::CIRCLE != 0;
                if fire && dist2(w.mobys[id].pos, ctx.hero.pos) < 64.0 && w.mobys[id].pv_i(NO_FLY) == 0 {
                    off(w, id, ctx, 1.0 / 12.0, 6);
                }
            }
        }
        4 => {
            if wrapped(w, id, ctx) { off(w, id, ctx, 1.0 / 30.0, 5); }
        }
        5 => {
            let m = &mut w.mobys[id];
            let s = (m.pv_f(SPEED) + 1.0 / 120.0).min(f32::from_bits(0x3dee_eef0));
            m.set_pv_f(SPEED, s);
            let from = m.pos;
            let v = polar(s, add_rot(m.rot[2], m.pv_f(WAY) * DEG), PITCH);
            ai::move_by(w, id, ctx, v);
            let m = &mut w.mobys[id];
            if dist2(m.pos, from) < m.pv_f(SPEED) {
                let yaw = add_rot(m.rot[2], f32::from_bits(0x4049_0fdc));
                ai::turn_to(m, yaw, f32::from_bits(0x3951_b717), f32::from_bits(0x3b03_126f), f32::from_bits(0x3d23_d70a), TURN_V);
            }
            if 64.0 < dist2(w.mobys[id].pos, ctx.hero.pos) { return w.delete(id); }
            bob(&mut w.mobys[id], BOB.0, BOB.1 * 0.000_290_888_25, -1.0, PHASE, PHASE + 4);
        }
        6 => {
            let m = &mut w.mobys[id];
            let s = (m.pv_f(SPEED) + 1.0 / 120.0).min(f32::from_bits(0x3ed5_5556));
            m.set_pv_f(SPEED, s);
            let v = polar(s, add_rot(m.rot[2], m.pv_f(WAY) * DEG), PITCH);
            m.pos = add(m.pos, v);
            if 64.0 < dist2(m.pos, ctx.hero.pos) { return w.delete(id); }
        }
        7 => {
            let path = usize::try_from(w.mobys[id].pv_i(PATH)).ok().and_then(|p| ctx.level.paths.get(p)).cloned().unwrap_or_default();
            if !path.is_empty() {
                let k = (w.mobys[id].pv_i(POINT).max(0) as usize).min(path.len() - 1);
                let p = [path[k][0], path[k][1], path[k][2]];
                let m = &mut w.mobys[id];
                let pitch = atan2(dist2(m.pos, p), p[2] - m.pos[2]);
                let v = polar(f32::from_bits(0x3dee_eef0), m.rot[2], pitch);
                let yaw = atan2(p[0] - m.pos[0], p[1] - m.pos[1]);
                ai::turn_to(m, yaw, f32::from_bits(0x38d1_b717), f32::from_bits(0x3a83_126f), f32::from_bits(0x3ca3_d70a), TURN_V);
                m.pos = add(m.pos, v);
                set_seq(w, id, ctx, 4, 0, 10);
                let m = &mut w.mobys[id];
                if dist2(m.pos, p) < 0.5 {
                    let n = (m.pv_i(POINT) + 1).min(path.len() as i32 - 1);
                    m.set_pv_i(POINT, n);
                }
            }
            bob(&mut w.mobys[id], BOB.0, BOB.1 * 0.000_290_888_25, -1.0, PHASE, PHASE + 4);
        }
        8 => {
            set_seq(w, id, ctx, 2, 0, 10);
            let goal = w.mobys[id].pv_v(GOAL);
            let arrived = dist2(w.mobys[id].pos, goal) < 0.5;
            let stuck = w.mobys[id].pv_i(MV + 0x80) != 0;
            if arrived || stuck || 0xef < w.mobys[id].state_ticks as u16 {
                let m = &mut w.mobys[id];
                m.state_ticks = 0;
                m.set_pv_i(MV + 0x80, 0);
                new_goal(w, id, ctx);
                if ctx.game.rng.randi(4) == 1 {
                    set_seq(w, id, ctx, 1, 0, 10);
                    w.mobys[id].set_state(9, -1);
                }
            } else {
                let p = w.mobys[id].pos;
                let yaw = atan2(goal[0] - p[0], goal[1] - p[1]);
                ai::move_step(w, id, ctx, yaw, goal);
            }
            near(w, id, ctx);
        }
        9 => {
            if wrapped(w, id, ctx) {
                w.mobys[id].set_state(8, -1);
                set_seq(w, id, ctx, 2, 0, 10);
            }
            near(w, id, ctx);
        }
        _ => {}
    }
    let m = &mut w.mobys[id];
    if m.state_ticks != -1 { m.state_ticks = m.state_ticks.wrapping_add(1); }
    match seq_from(w, id, ctx) {
        4 => ctx.host.set_anim_speed(&w.mobys[id], 2.5),
        0 => {}
        _ => ctx.host.set_anim_speed(&w.mobys[id], 1.0),
    }
    if w.hit_message(id).is_some_and(|h| h.tmpl.flags & 0x10000 != 0) {
        w.level11.bird_kills += 1;
        w.delete(id);
    }
}

/// Ratchet within 10: sequence 2, **4**.
fn near(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if 10.0 <= dist(w.mobys[id].pos, ctx.hero.pos) { return; }
    set_seq(w, id, ctx, 2, 0, 10);
    w.mobys[id].set_state(4, -1);
}

/// Off: its way 170..190° (`0x32dd08(0xaa, 0xbe)`), sequence 4, its speed, the state.
fn off(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, speed: f32, state: u8) {
    let a = ctx.game.rng.rand_range(0xaa, 0xbe) as f32;
    w.mobys[id].set_pv_f(WAY, a);
    set_seq(w, id, ctx, 4, 0, 10);
    let m = &mut w.mobys[id];
    m.set_pv_f(SPEED, speed);
    m.set_state(state, -1);
}
