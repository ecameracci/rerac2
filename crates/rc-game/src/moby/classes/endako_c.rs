//! Endako (level 3)'s enemies, set C (L03 addresses; the engine functions they call are named by their L03
//! addresses): the laser sentry 0x0ffd ([`update_0ffd`], below), the flame turret 0x0ffc on its rail
//! ([`update_0ffc`]) and the floating mine 0x0b23 ([`update_0b23`]).
//!
//! The energy field **0x0944** is [`update_0944`] / [`draw_0944`].
//!
//! Not ported (the set's other classes):
//! * **0x10f6** (`0x422a78`): the wall the remote crane's bomb box breaks, crate::moby::classes::endako_crane.
//! * **0x0972** (`0x3cb898`): the cage of the robot 0x0964, crate::moby::classes::endako_d::update_0972.
//!
//! **The laser sentry 0x0ffd** ([`update_0ffd`], `0x41f9b0`, 28 placed; its senses `0x4201b8`, beam `0x420850`, sparks
//! `0x420ae8`, draw `0x420df8`, tail `0x420668`), on the framework ([`crate::moby::ai`]): health `+0x20`, the flash
//! `+0x50`, reaction `+0x60`, movement `+0x110`; its path `+0x280` (−1 none), the cuboid it comes in at `+0x284`, a
//! turret `+0x288` (≠ 0), waiting to be hooked `+0x28c` (≠ 0), the view cuboid `+0x294` (−1 none), its beam's end
//! shown to the spills `+0x298` (≠ 0); the beam's history `+0x2a0` (four of {start, end}, the newest at `+0x334`,
//! `+0x336` of them), its length `+0x33c`, its brightness `+0x330`, its looped sound `+0x340`.
//!
//! * **The set-up** (state 0, in [`senses`]): a turret: 3 health, its beam 4.5 long, waiting for Clank (4, sub 1).
//!   On a path: at its first point heading for the second (movement options | 0x210: no turn, the fall kill), its beam
//!   6 long, patrolling (3, sub 0) — or, waiting to be hooked, hidden without collision and its beam 0 long (9, sub
//!   4). Neither: deleted. Then the reaction (`+0x48` 20, `+0x4a` 17, hurt state 8, sub 3, `+0x30` / `+0x34` 0.5,
//!   `+0x3c` 0.4, the kill height 1 under it), the movement (radius 0.5, height 0.25, steps 0.4, slope 45°, turn left
//!   30°, stop 0.05, walk 4/s² up to 4/s), the scale × 1.25.
//! * **Each tick** ([`senses`]): a hit with flags 0x10000 from another 0x0ffd less than 2 above it is let go; the
//!   frame's damage — a kill: a blast at 0.75 up, its pieces thrown along its knock (a fifteenth of the reaction's
//!   velocity), the remote-control kill count ([`crane_kill`]), its sound 3, its bolts, the loop stopped, deleted; its
//!   shadow within 24 of the camera (range 0x16).
//! * **Subs** (`+0x95`): 0 with a view cuboid — drawn only while the camera is in it (out: hidden, its loop stopped,
//!   and nothing more this tick). 3 the knock: dead or tipped past 0.1 across, the blast, its pieces thrown away from
//!   Ratchet at 0.1 a tick, its sound 3 and bolts, deleted; over, back to its watch (4, sub 1) or its patrol (3, sub 0).
//! * **States**: 2 into the arena: its sound 4 once, toward 6 along its cuboid's x, then 3. 3 the patrol: its beam
//!   grows to 6 (0.05 a tick), it spins 225°/s and rolls point to point round its path (hooked: it stops); its beam
//!   from 6 ticks in. 4 the watch: Clank (`0x18c0d4` = 1) wakes it (5). 5 it turns toward Ratchet weaving ±5° (6° a
//!   tick), its beam from 6 ticks in; Ratchet again, 4. 8 knocked (nothing). 9 waiting hidden: hooked, it teleports in
//!   at its cuboid (crate::moby::classes::teleport; a hit sphere 0.5 of 2 where it waited) and goes into the arena (2).
//!   6 carried by the remote crane (crate::moby::classes::endako_crane; set by [`carry`], `0x4206e0`, sub 2): under its
//!   hook (`0x313be0(crane, 2)`: 0.75 lower, 0.5 down its own up row), tipping to 90° (6° a tick), its beam growing to
//!   10 (1/15 a tick), flags 0x10001 in sub 2. 7 dropped ([`drop`], `0x420720`): falling at 1/240 a tick² from the
//!   crane's velocity, its beam on; a sphere of 0.5 at 0.5 up its row meeting anything: the blast, its pieces,
//!   deleted. [`arm`] (`0x420768`) is the crane terminal's: it brings a waiting one in.
//! * **The beam** (`0x420850`, every odd frame): from (1.0875, 0, 0.99375) in its frame to (length, 0, −0.0125), the
//!   line (`0x2df4f8`) to the first thing it meets, which takes the hit (its damage `+0x34`, flags 1 or 0x10001 in sub
//!   2, no knock); a crate (classes 500..520) also a hit of flags 0x10000; its end shown to the spills (crate::moby::
//!   classes::endako_a::update_spill) when `+0x298`; with three ends kept the sparks between this end and the last
//!   ([`sparks`]); this start and end kept.
//! * **The draw** (`0x420df8`, [`draw_beam`]): the line again (its hit again, as the original's draw makes it), two
//!   quads of FX 0xb (additive) along it, 0.25 and 0.02 to either side across the view, red 0x40 / 0x80; the sweep
//!   from each kept beam to the next and on to this one in flat FX 0xb (its middle), blue to red over the sweep at
//!   alpha 0x40; its looped sound 5.
//! * **The tail** (`0x420668`): the flash `+0x50`, the timers `+0x320` / `+0x322` down, its ticks.
//!
//! Not ported: the class record's `+0x46` = 5 the set-up writes; `0x327aa8(2, m, +0x200)` (its `+0x270` 2 under the
//! flag `0x1a7a4f`, else 1: a block nothing here reads); the moby's `+0x7f` byte state 9 clears. The beam's second line query is made in the
//! update, where it registers its draw (the port's draws have no collision), and its sound is started there too.

use crate::hero::physics::{add, add_rot, atan2, cross, dist, dist2, len2, scale, set_len, sub, unit, V3};
use crate::hud::lerp_rgba;
use crate::moby::{ai, Ctx, MobyWorld};
use super::endako_a::to_world;

/// The laser sentry.
pub const SENTRY: i16 = 0x0ffd;

const HEALTH: usize = 0x20;
const FLASH: usize = 0x50;
const REACT: usize = 0x60;
const MV: usize = 0x110;
const PATH: usize = 0x280;
const CUBOID: usize = 0x284;
const TURRET: usize = 0x288;
const ASLEEP: usize = 0x28c;
const SAID: usize = 0x290;
const VIEW: usize = 0x294;
const SHOW_END: usize = 0x298;
const PHASE: usize = 0x29c;
const HIST: usize = 0x2a0;
const POINT: usize = 0x32c;
const BRIGHT: usize = 0x330;
const HEAD: usize = 0x334;
const COUNT: usize = 0x336;
const CRANE_LINK: usize = 0x338;
const LEN: usize = 0x33c;
const LOOP: usize = 0x340;

/// `$gp+0x2bd0`: the beam's start in its frame.
const START: V3 = [f32::from_bits(0x3f8b_3333), 0.0, f32::from_bits(0x3f7e_6667)];
/// `$gp+0x2be8`: the beam's end's height in its frame.
const END_Z: f32 = f32::from_bits(0xbc4c_cccc);
/// `$gp+0x2bf0`: where the blast goes in its frame.
const BLAST_AT: V3 = [0.0, 0.0, 0.75];
/// `$gp+0x2bc0..0x2bcc`: walk accel (/s²), walk most (/s), the spin (°/s), the scale.
const WALK: (f32, f32) = (4.0, 4.0);
const SPIN: f32 = 225.0;
const SCALE: f32 = 1.25;
/// `$gp+0x2c80` / `+0x2c58`: the beam's two quads (colour, half-width).
const LAYERS: [(u32, f32); 2] = [(0x4000_0040, 0.25), (0x8000_0080, f32::from_bits(0x3ca3_d70a))];
/// `$gp+0x2c48` / `+0x2c4c` / `+0x2c50`: the sweep's colours and their step.
const SWEEP: (u32, u32, f32) = (0x4010_0000, 0x4000_0060, f32::from_bits(0x3e4c_cccd));
/// The beam's FX texture (`$gp+0x2c38`, both quads).
const FX: usize = 0xb;

/// The level's globals the set's classes share.
#[derive(Clone, Debug, Default)]
pub struct Globals {
    /// `$gp+0x1550` (`0x1b0540`): the end of the last beam that shows it (`+0x298`); the spills it reaches go.
    pub beam_end: V3,
    /// `$gp−0x5fac` (`0x1a9044`): the enemies killed while Ratchet works the remote crane (state 0x31).
    pub crane_kills: i32,
    /// `0x18b650`: the moby Ratchet steers in state 0x31 (crate::moby::classes::endako_crane's terminal sets it).
    pub steered: Option<usize>,
    /// The beams to draw this frame: the moby, its start and the end its draw's line found.
    pub beams: Vec<(usize, V3, V3)>,
}

/// The kill count's step in 0x0ffd's and 0x0ff3's deaths (`0x4201b8`, `0x41e138`): with Ratchet on the remote crane
/// (state 0x31) one more, and past 9 skill point 0x1c (crate::skill_points).
pub fn crane_kill(w: &mut MobyWorld, ctx: &mut Ctx) {
    if ctx.hero.state != 0x31 { return; }
    w.endako.crane_kills += 1;
    if 9 < w.endako.crane_kills { ctx.game.award_skill_point(crate::skill_points::point::CRANE); }
}

/// `0x31ff20(target, step, &v)`: `v` toward `target` by at most `step`.
fn ease(v: f32, target: f32, step: f32) -> f32 { v + (target - v).clamp(-step, step) }

fn yaw_to(w: &MobyWorld, id: usize, p: V3) -> f32 { let q = w.mobys[id].pos; atan2(p[0] - q[0], p[1] - q[1]) }

/// `0x32e2d0(m, goal)`: the frame's move toward `goal`, turned toward it.
fn go(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, goal: V3) -> i32 {
    let y = yaw_to(w, id, goal);
    ai::move_step(w, id, ctx, y, goal)
}

/// Its looped sound stopped (`0x361ac8`, `+0x340` = −1).
fn stop_loop(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let h = w.mobys[id].pv_i(LOOP);
    if h != -1 {
        ctx.host.stop_sound(h);
        w.mobys[id].set_pv_i(LOOP, -1);
    }
}

/// The cuboid's place and angles, and its x axis.
fn cuboid(ctx: &Ctx, i: i32) -> Option<(V3, V3, V3)> {
    let c = usize::try_from(i).ok().and_then(|i| ctx.level.cuboids.get(i))?;
    let (p, r) = ctx.level.cuboid_place(i)?;
    Some((p, r, [c.matrix[0][0], c.matrix[0][1], c.matrix[0][2]]))
}

/// The blast (`0x403f10`: size 0.5, sprites 0x19 / 0x14 / 0x14, 0x14 puffs, 0x2d streaks, its light) at 0.75 up.
fn blast(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let at = add(to_world(BLAST_AT, &m.rows), m.pos);
    crate::moby::blast::burst(w, ctx, at, &crate::moby::blast::Blast { size: 0.5, bursts: [0x19, 0x14, 0x14], puffs: 0x14, streaks: 0x2d, light: true, shake: false });
}

/// `0x4201b8` (module docs). False when it went.
fn senses(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    if w.mobys[id].state == 0 {
        stop_loop(w, id, ctx);
        let m = &w.mobys[id];
        if m.pv_i(TURRET) != 0 {
            let m = &mut w.mobys[id];
            m.set_state(4, 1);
            ctx.host.set_anim(&w.mobys[id], 0, 0, 0);
            let m = &mut w.mobys[id];
            m.set_pv_f(HEALTH, 3.0);
            m.set_pv_s(HEALTH + 4, 3);
            m.set_pv_f(LEN, 4.5);
        } else if let Some(first) = usize::try_from(m.pv_i(PATH)).ok().and_then(|p| ctx.level.paths.get(p)).and_then(|p| p.first()).copied() {
            let m = &mut w.mobys[id];
            m.pos = [first[0], first[1], first[2]];
            m.set_pv_s(POINT, 1);
            m.set_pv_i(MV + 0x38, m.pv_i(MV + 0x38) | 0x210);
            if m.pv_i(ASLEEP) == 0 {
                m.set_state(3, 0);
                ctx.host.set_anim(&w.mobys[id], 0, 0, 0);
                w.mobys[id].set_pv_f(LEN, 6.0);
            } else {
                m.set_state(9, 4);
                ctx.host.set_anim(&w.mobys[id], 0, 0, 0);
                let m = &mut w.mobys[id];
                m.set_pv_f(LEN, 0.0);
                m.has_collision = false;
                m.mode = (m.mode & 0xefff) | 0x41;
            }
        } else {
            w.delete(id);
            return false;
        }
        let class_scale = w.class(SENTRY).map_or(1.0, |c| c.scale);
        let m = &mut w.mobys[id];
        m.set_pv_b(0x48, 20);
        m.set_pv_b(0x4a, 17.6_f32 as u8);
        m.set_pv_b(REACT + 0x10, 8);
        m.set_pv_i(REACT + 0x48, 3);
        m.set_pv_f(REACT + 0x30, 0.5);
        m.set_pv_f(REACT + 0x34, 0.5);
        m.set_pv_f(REACT + 0x3c, f32::from_bits(0x3ecc_cccd));
        m.set_pv_f(REACT + 0x40, m.pos[2] - 1.0);
        m.set_pv_f(0x30, 0.75);
        m.set_pv_f(MV + 0x34, f32::from_bits(0x3d4c_cccd));
        m.set_pv_f(MV + 4, 0.25);
        m.set_pv_f(MV, 0.5);
        m.set_pv_f(MV + 0xc, f32::from_bits(0x3ecc_cccd));
        m.set_pv_f(MV + 0x10, f32::from_bits(0x3f49_0fdc));
        m.set_pv_f(MV + 0x30, f32::from_bits(0x3f06_0a93));
        m.set_pv_f(BRIGHT, 1.0);
        m.set_pv_f(MV + 0x20, WALK.0 * 0.000_277_777_78);
        m.set_pv_f(MV + 0x28, WALK.1 * 0.016_666_668);
        m.set_pv_f(MV + 8, f32::from_bits(0x3ecc_cccd));
        m.set_pv_f(MV + 0x24, WALK.0 * 0.000_277_777_78);
        m.scale = class_scale * SCALE;
        m.set_pv_i(SAID, 0);
    }
    // `0x31f0f0(m, 0x10000, 0)`: another sentry's beam from less than 2 above does not count.
    if let Some(h) = w.hit_message(id) {
        let z = w.mobys[id].pos[2];
        let from_below = h.tmpl.source.and_then(|s| w.mobys.get(s)).is_some_and(|s| s.o_class == SENTRY && s.pos[2] < z + 2.0);
        if h.tmpl.flags & 0x10000 != 0 && from_below { w.clear_hit(id); }
    }
    let kind = ai::take_damage(w, id, ctx, false);
    if kind != 0 && w.mobys[id].pv_f(HEALTH) <= 0.0 {
        blast(w, id, ctx);
        let vel = set_len(w.mobys[id].pv_v(REACT + 0x80), f32::from_bits(0x3d88_8889));
        crate::moby::shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 6.0), 0x1f, vel, 0xff);
        crane_kill(w, ctx);
        ctx.host.play_sound(&w.mobys[id], 3, 0);
        crate::moby::bolts::drop(w, id, 0, 3, ctx);
        stop_loop(w, id, ctx);
        w.delete(id);
        return false;
    }
    crate::shadow_volumes::probe_near(w, id, ctx, 24.0, 0x16);
    true
}

/// 0x0ffd (module docs).
pub fn update_0ffd(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if !senses(w, id, ctx) { return; }
    match w.mobys[id].sub_state {
        0 => {
            let view = w.mobys[id].pv_i(VIEW);
            if view != -1 {
                if !ctx.level.in_cuboid(view, ctx.host.camera_pos()) {
                    stop_loop(w, id, ctx);
                    let m = &mut w.mobys[id];
                    m.drawn = 0;
                    m.mode |= 1;
                    return;
                }
                let m = &mut w.mobys[id];
                m.drawn = 1;
                m.mode &= !1;
            }
        }
        3 => {
            let r = ai::flight(w, id, ctx);
            if w.mobys[id].first() { stop_loop(w, id, ctx); }
            if r & 1 != 0 || 0.1 < w.mobys[id].rot[1].abs() {
                blast(w, id, ctx);
                let d = sub(w.mobys[id].pos, ctx.hero.pos);
                let vel = set_len([d[0], d[1], 0.0], f32::from_bits(0x3dcc_ccce));
                crate::moby::shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 6.0), 0x1f, vel, 0xff);
                ctx.host.play_sound(&w.mobys[id], 3, 0);
                crate::moby::bolts::drop(w, id, 0, 3, ctx);
                w.delete(id);
                return;
            }
            if r & 2 != 0 {
                let turret = w.mobys[id].pv_i(TURRET) != 0;
                if turret { w.mobys[id].set_state(4, 1); } else { w.mobys[id].set_state(3, 0); }
                super::props::set_seq(w, id, ctx, 0, 0, 10);
            }
        }
        _ => {}
    }
    match w.mobys[id].state {
        2 => {
            if w.mobys[id].pv_i(SAID) == 0 {
                ctx.host.play_sound(&w.mobys[id], 4, 0);
                w.mobys[id].set_pv_i(SAID, 1);
            }
            // `0x2f0300((6, 0, 0, 1), cuboid)`.
            if let Some((p, _, x)) = cuboid(ctx, w.mobys[id].pv_i(CUBOID)) {
                let g = add(p, scale(x, 6.0));
                if go(w, id, ctx, g) & 0x80 != 0 { w.mobys[id].set_state(3, -1); }
            }
        }
        3 => {
            if w.mobys[id].first() {
                w.mobys[id].set_pv_s(HEAD, 0);
                w.mobys[id].set_pv_s(COUNT, 0);
            } else if w.mobys[id].once & 4 != 0 {
                let m = &mut w.mobys[id];
                m.once &= !4;
                m.set_pv_f(MV + 0x28, 0.0);
                m.set_pv_f(MV + 0x20, f32::from_bits(0x3b36_0b61));
                m.set_pv_f(MV + 0x24, f32::from_bits(0x3b36_0b61));
            }
            let m = &mut w.mobys[id];
            m.set_pv_f(LEN, ease(m.pv_f(LEN), 6.0, f32::from_bits(0x3d4c_ccce)));
            beam(w, id, ctx);
            let m = &mut w.mobys[id];
            m.rot[2] = add_rot(m.rot[2], SPIN * f32::from_bits(0x3998_825c));
            let k = m.pv_s(POINT) as i32;
            let pts = usize::try_from(m.pv_i(PATH)).ok().and_then(|p| ctx.level.paths.get(p)).filter(|p| !p.is_empty());
            if let Some((n, q)) = pts.map(|p| (p.len() as i32, p[k.rem_euclid(p.len() as i32) as usize])) {
                if go(w, id, ctx, [q[0], q[1], q[2]]) & 0x80 != 0 {
                    w.mobys[id].set_pv_s(POINT, ((k + 1) % n) as i16);
                }
            }
            if 6 <= w.mobys[id].state_ticks as u16 { draw_prep(w, id, ctx); }
        }
        4 => {
            if ctx.hero.mode == 1 { w.mobys[id].set_state(5, -1); }
        }
        5 => {
            let m = &mut w.mobys[id];
            let phase = add_rot(m.pv_f(PHASE), f32::from_bits(0x3dd6_7752));
            m.set_pv_f(PHASE, phase);
            let to = yaw_to(w, id, ctx.hero.pos);
            let yaw = add_rot(to, phase.sin() * f32::from_bits(0x3db2_b8c4));
            let pos = w.mobys[id].pos;
            ai::move_step(w, id, ctx, yaw, pos);
            beam(w, id, ctx);
            if 6 <= w.mobys[id].state_ticks as u16 { draw_prep(w, id, ctx); }
            if ctx.hero.mode != 1 { w.mobys[id].set_state(4, -1); }
        }
        6 => {
            // `0x313be0(+0x338, 2)`: under the crane's hook, 0.75 lower and 0.5 down its own up row.
            if let Some(c) = ai::link(&w.mobys[id], CRANE_LINK) {
                let j = super::endako_crane::hook_point(w, c, ctx);
                let m = &mut w.mobys[id];
                let off = to_world([0.0, 0.0, 0.5], &m.rows);
                m.pos = sub([j[0], j[1], j[2] - 0.75], off);
            }
            let m = &mut w.mobys[id];
            m.rot[1] = ease(m.rot[1], f32::from_bits(0x3fc9_0fdb), f32::from_bits(0x3dd6_7752));
            m.set_pv_f(LEN, ease(m.pv_f(LEN), 10.0, f32::from_bits(0x3d88_8889)));
            beam(w, id, ctx);
            draw_prep(w, id, ctx);
        }
        7 => {
            let m = &mut w.mobys[id];
            let mut v = m.pv_v(MV + 0xa0);
            v[2] -= f32::from_bits(0x3b88_8889);
            m.set_pv_v(MV + 0xa0, v);
            m.pos = add(m.pos, v);
            let at = add(to_world([0.0, 0.0, 0.5], &m.rows), m.pos);
            beam(w, id, ctx);
            draw_prep(w, id, ctx);
            // `0x2e0558(0.5, at, 0, m)`: meeting anything, it blows up.
            if ctx.coll.and_then(|c| crate::collision::coll_sphere_m(c, ctx.scene, at, 0.5, crate::collision::QueryFlags(0), Some(id))).is_some() {
                blast(w, id, ctx);
                crate::moby::shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 6.0), 0x1f, [0.0; 3], 0xff);
                // The port's delete leaves a looped sound playing.
                stop_loop(w, id, ctx);
                w.delete(id);
                return;
            }
        }
        9 => {
            if w.mobys[id].first() { stop_loop(w, id, ctx); }
            if w.mobys[id].once & 4 != 0 {
                w.mobys[id].once &= !4;
                let solid = w.class(SENTRY).is_some_and(|c| c.has_collision);
                let m = &mut w.mobys[id];
                m.set_state(2, 0);
                m.mode = (m.mode & 0xffbe) | 0x1000;
                m.has_collision = solid;
                let at = m.pos;
                ai::attack_sphere(w, id, ctx, 0.5, 2.0, 1.0, at, 0x10001, 0, 1, 0);
                if let Some((p, r, _)) = cuboid(ctx, w.mobys[id].pv_i(CUBOID)) {
                    super::teleport::start(w, id, p, r, &[]);
                }
            }
            let m = &mut w.mobys[id];
            m.has_collision = false;
            m.mode = (m.mode & 0xefff) | 0x41;
        }
        _ => {}
    }
    tail(w, id);
}

/// `0x420768(m)`: the crane terminal's arming (crate::moby::classes::endako_crane). A sentry with a cuboid to come in at,
/// waiting hidden (9) and not yet hooked: its hit let go, its knock `+0x98` 0, its walk back (4/s² up to 4/s), its
/// beam 6 long, its health full, its class's collision, Ratchet's light (`0x321328`), hooked — true. Else false.
pub fn arm(w: &mut MobyWorld, id: usize, ctx: &Ctx) -> bool {
    let m = &w.mobys[id];
    if m.pv_i(CUBOID) == -1 || m.state != 9 || m.once & 4 != 0 { return false; }
    w.clear_hit(id);
    let solid = w.class(SENTRY).is_some_and(|c| c.has_collision);
    let light = ctx.hero_moby.and_then(|h| w.mobys.get(h)).map(|h| (h.light, h.ambient));
    let m = &mut w.mobys[id];
    m.set_pv_s(REACT + 0x38, 0);
    m.set_pv_f(MV + 0x28, WALK.1 * 0.016_666_668);
    m.set_pv_f(LEN, 6.0);
    m.set_pv_f(HEALTH, m.pv_s(HEALTH + 4) as f32);
    m.set_pv_f(MV + 0x20, WALK.0 * 0.000_277_777_78);
    m.set_pv_f(MV + 0x24, WALK.0 * 0.000_277_777_78);
    m.has_collision = solid;
    if let Some((l, a)) = light { (m.light, m.ambient) = (l, a); }
    m.once |= 4;
    w.changed = true;
    true
}

/// `0x4206e0(m, crane)`: hung on the crane's hook (6, sub 2), the crane kept (`+0x338`).
pub fn carry(w: &mut MobyWorld, id: usize, crane: usize) {
    let m = &mut w.mobys[id];
    m.set_state(6, 2);
    ai::set_link(m, CRANE_LINK, Some(crane));
}

/// `0x420720(m, vel)`: let go with the crane's velocity (7, sub 2; its movement's velocity `+0x1b0`).
pub fn drop(w: &mut MobyWorld, id: usize, vel: V3) {
    let m = &mut w.mobys[id];
    m.set_state(7, 2);
    m.set_pv_v(MV + 0xa0, vel);
}

/// `0x420668` (module docs).
fn tail(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    ai::flash(m, FLASH);
    for o in [0x320, 0x322] { count_s(m, o); }
    if m.state_ticks != -1 { m.state_ticks = m.state_ticks.wrapping_add(1); }
}

/// `0x2efa80(&s)`: a 16-bit timer down (crate::moby::countdown); true when it is (or runs) out.
fn count_s(m: &mut crate::moby::Moby, o: usize) -> bool {
    let mut t = m.pv_s(o) as i32;
    let out = crate::moby::countdown(&mut t);
    m.set_pv_s(o, t as i16);
    out
}

/// The beam's line (`0x2df4f8(start, end, 0, m, hit)`): its start, its end (where it met something) and what it met;
/// the thing it met takes the hit (its damage `+0x34`, flags 1, or 0x10001 in sub 2; `0x31e560`).
fn line(w: &mut MobyWorld, id: usize, ctx: &Ctx) -> (V3, V3, Option<crate::collision::CollOutput>) {
    let m = &w.mobys[id];
    let a = add(to_world(START, &m.rows), m.pos);
    let b = add(to_world([m.pv_f(LEN), 0.0, END_Z], &m.rows), m.pos);
    let flags = if m.sub_state == 2 { 0x10001 } else { 1 };
    let t = crate::moby::hits::HitTemplate::new(m.pv_b(HEALTH + 0x14) as f32, Some(id), flags, [0.0; 4]);
    let hit = ctx.coll.and_then(|c| crate::collision::coll_line_m(c, ctx.scene, a, b, crate::collision::QueryFlags(0), Some(id)));
    match hit {
        Some(o) => {
            // The line's delivery (L03's kernel wants mode 0x4000, as the port's other line callers: not ported).
            if let Some(t_id) = o.moby { w.deliver_hit_any(t_id, &t, [o.point[0], o.point[1], o.point[2], 1.0]); }
            (a, o.point, Some(o))
        }
        None => (a, b, None),
    }
}

/// `0x420850` (module docs).
fn beam(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if ctx.game.frame & 1 == 0 { return; }
    let m = &mut w.mobys[id];
    let head = (m.pv_s(HEAD) as i32 + 1).rem_euclid(4);
    m.set_pv_s(HEAD, head as i16);
    if m.pv_s(COUNT) < 3 { m.set_pv_s(COUNT, m.pv_s(COUNT) + 1); }
    let (a, b, hit) = line(w, id, ctx);
    let mut floor = false;
    if let Some(o) = hit {
        // `0x2f04d0(0x2eff70(normal), normal.z)`: the face's normal more than 80° up.
        floor = 1.396_263_6 < atan2(len2(o.normal), o.normal[2]);
        if let Some(t_id) = o.moby {
            if (500..0x209).contains(&w.mobys[t_id].o_class) {
                let t = crate::moby::hits::HitTemplate::new(w.mobys[id].pv_b(HEALTH + 0x14) as f32, Some(id), 0x10000, [0.0; 4]);
                // `0x31e780`: no mode test.
                w.deliver_hit_any(t_id, &t, [0.0, 0.0, 0.0, 1.0]);
            }
        }
    }
    let m = &w.mobys[id];
    if 3 <= m.pv_s(COUNT) {
        let prev = m.pv_v(HIST + ((head + 3) & 3) as usize * 0x20 + 0x10);
        let state = m.state;
        sparks(ctx, b, prev, floor, state);
    }
    if w.mobys[id].pv_i(SHOW_END) != 0 { w.endako.beam_end = b; }
    let m = &mut w.mobys[id];
    m.set_pv_v(HIST + head as usize * 0x20, a);
    m.set_pv_v(HIST + head as usize * 0x20 + 0x10, b);
}

/// `0x420ae8(m, end, last, floor)`: when this end is within 1 of the last and the stretch between is in view
/// (`0x2f1080`), four sparks along it (`0x33b0e0`: 0.05..0.2 × 210000 growing 500, blue 0x4080 at alpha 0x7f, 10..30
/// ticks, a random way at 0..1/60 a tick and up 1..2/60, spinning ±1..2); on a floor (its normal more than 80° up)
/// four puffs of dark smoke (`0x333f48`, one of two at random in state 5) a quarter of the way apart, 0.02 up (0x40
/// black → 0x10, 5 / 30 / 30 ticks, 0.5 → 0.25 across).
fn sparks(ctx: &mut Ctx, end: V3, last: V3, floor: bool, state: u8) {
    let d = dist(end, last);
    if 1.0 < d || !super::space::in_view(ctx, end, d) { return; }
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let rng = &mut ctx.game.rng;
    for _ in 0..4 {
        let life = rng.rand_range(10, 30) as i16;
        let spin = rng.randf_signed(1.0, 2.0) as i32;
        let size = rng.randf(0.05, 0.2) * 210_000.0;
        let mut vel = rng.rand_vec(0.0, 0.016_666_668);
        vel[2] += rng.randf(1.0, 2.0) * 0.016_666_668;
        let t = rng.randf(0.0, 1.0);
        let at = add(end, scale(sub(last, end), t));
        crate::particles::spark(sys, rng, size, 500.0, 1.0, 0.0, 1.0, at, vel, life, 0x7f, 0x00_4080, spin);
    }
    if !floor { return; }
    let n = if state == 5 { rng.randi(2) } else { 4 };
    let step = 1.0 / 4.0;
    for i in 0..n {
        let t = rng.randf(0.0, step) + i as f32 * step;
        let mut at = add(end, scale(sub(last, end), t));
        at[2] += f32::from_bits(0x3ca3_d70a);
        if let Some(p) = crate::particles::staged(sys, rng, at, [0.0, 0.0, 0.0, 0.5], [0.0, 0.0, 0.0, 0.25], 0x4000_0000, 0x1000_0000, 5, 30, 30, -1) {
            sys.pool.recs[p][1] = 1;
        }
    }
}

/// `0x2e8310(0x420df8, m)` and the draw's own work that needs the world: its line (and hit) again, kept for
/// [`draw_beam`]; its looped sound 5 (`+0x340`) started, or again when it has stopped.
fn draw_prep(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (a, b, _) = line(w, id, ctx);
    let beams = &mut w.endako.beams;
    beams.retain(|e| e.0 != id);
    beams.push((id, a, b));
    let h = w.mobys[id].pv_i(LOOP);
    if h == -1 || !ctx.host.sound_playing(h) {
        let h = ctx.host.play_sound(&w.mobys[id], 5, 4);
        w.mobys[id].set_pv_i(LOOP, h);
    }
    w.register_draw(id);
}

/// `0x420df8` (module docs): the beam's quads, then the sweep. `frame` the game's (`0x1b2548`).
pub fn draw_beam(w: &MobyWorld, id: usize, frame: i32, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let Some(&(_, a, b)) = w.endako.beams.iter().find(|e| e.0 == id) else { return };
    let m = &w.mobys[id];
    let side = unit(cross(sub(cam, a), sub(b, a)));
    let st = [[0.5, 0.0], [0.5, 1.0], [0.5, 0.0], [0.5, 1.0]];
    let bright = m.pv_f(BRIGHT);
    for (c, half) in LAYERS {
        let rgba = lerp_rgba(bright, c & 0xff00_0000, c);
        let s = scale(side, half);
        out.push(crate::fx::FxQuad { fx: FX, additive: true, corners: [sub(a, s), add(a, s), sub(b, s), add(b, s)], st, rgba: [rgba; 4] });
    }
    // The sweep: flat colour (the texture's middle), oldest kept beam first.
    let mid = [[0.5, 0.5]; 4];
    let col = |f: f32| lerp_rgba(f, SWEEP.0, SWEEP.1);
    let mut f = if frame & 1 != 0 { 0.0 } else { 0.1 };
    let (mut c0, mut c3) = (col(f), col(f + SWEEP.2));
    let kept = |k: i32| -> (V3, V3) { let o = HIST + k as usize * 0x20; (m.pv_v(o), m.pv_v(o + 0x10)) };
    let count = m.pv_s(COUNT) as i32;
    let mut k = (m.pv_s(HEAD) as i32 - (count - 4)).rem_euclid(4);
    for _ in 0..count.max(0) {
        let (p0, p1) = kept(k);
        k = (k + 1).rem_euclid(4);
        let (q0, q1) = kept(k);
        out.push(crate::fx::FxQuad { fx: FX, additive: true, corners: [p0, p1, q0, q1], st: mid, rgba: [c0, c0, c3, c3] });
        f += SWEEP.2;
        c0 = col(f);
        c3 = col(f + SWEEP.2);
    }
    let (p0, p1) = kept(k);
    out.push(crate::fx::FxQuad { fx: FX, additive: true, corners: [p0, p1, a, b], st: mid, rgba: [c0, c0, c3, c3] });
}

/// The flame turret on its rail.
pub const FLAMER: i16 = 0x0ffc;

mod flamer {
    pub const HEALTH: usize = 0x20;
    pub const REACT: usize = 0x50;
    pub const MV: usize = 0x100;
    pub const TARGET: usize = 0x1f0;
    pub const RAIL: usize = 0x210;
    pub const AREA: usize = 0x214;
    pub const REACH: usize = 0x218;
    pub const CALM: usize = 0x330;
    pub const WAIT: usize = 0x332;
    pub const ALARM: usize = 0x336;
    pub const FLAME: usize = 0x33c;
    pub const ALONG: usize = 0x340;
    pub const GROW: usize = 0x348;
    pub const GIVE_UP: usize = 0x34c;
    pub const LOOP: usize = 0x350;
    /// `$gp+0x2b80..0x2b94`: walk most (/s), the accels (/s²), the nozzle angles' bases, the flame's speed (/s).
    pub const WALK: (f32, f32, f32) = (6.0, 10.0, 10.0);
    pub const NOZZLE: (f32, f32) = (1.0, 1.0);
    /// `$gp+0x2ba0`: where its death's smoke goes in its frame; `$gp+0x2bb0`: its lean.
    pub const SMOKE_AT: [f32; 3] = [0.0, 0.0, 1.5];
    pub const LEAN: f32 = -3.0;
    /// The turn (`0x3206d8`'s accel, brake, most).
    pub const TURN: (f32, f32, f32) = (f32::from_bits(0x3b64_c38a), f32::from_bits(0x3b64_c38a), f32::from_bits(0x3e56_7752));
}

/// `0x41f918`: hurt — calm for 240 ticks no more, and it and its group's living 0x0ffc alarmed (`+0x336`).
fn flamer_alarm(w: &mut MobyWorld, id: usize) {
    use flamer::*;
    w.mobys[id].set_pv_s(CALM, 0xf0);
    w.mobys[id].set_pv_s(ALARM, 1);
    let members = usize::try_from(w.mobys[id].group).ok().and_then(|g| w.groups.get(g)).cloned().unwrap_or_default();
    for i in members {
        let x = &mut w.mobys[i];
        if x.state < 0x80 && x.o_class == FLAMER { x.set_pv_s(ALARM, 1); }
    }
}

/// `0x41f1a8`: the set-up (state 0: the reaction — `+0x48` 20, `+0x4a` 14, hurt state 7 sub 3, `+0x30` / `+0x34`
/// 0.5, `+0x3c` 0.4, the kill height 2 under it; the movement — options | 0x15 (no ledge test, no gravity, no turn),
/// radius 0.5, height 0.25, steps 0.75, slope 45°, turn left 30°, stop 0.05; health `+0x28` 3; no rail: deleted; else
/// calm (1, sub 0)), then each tick its shadow within 48 (range 0x2d), its walk (10/s², at most 6/s), the frame's
/// damage (any: [`flamer_alarm`]) and its target (Ratchet within 16 in its area `+0x214`). False when it went.
fn flamer_senses(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    use flamer::*;
    if w.mobys[id].state == 0 {
        let m = &mut w.mobys[id];
        m.set_pv_b(0x48, 20);
        m.set_pv_b(0x4a, 14.4_f32 as u8);
        m.set_pv_b(REACT + 0x10, 7);
        m.set_pv_f(REACT + 0x3c, f32::from_bits(0x3ecc_cccd));
        m.set_pv_i(REACT + 0x48, 3);
        m.set_pv_f(REACT + 0x30, 0.5);
        m.set_pv_f(REACT + 0x34, 0.5);
        m.set_pv_f(REACT + 0x40, m.pos[2] - 2.0);
        m.set_pv_i(MV + 0x38, m.pv_i(MV + 0x38) | 0x15);
        m.set_pv_f(MV + 0x34, f32::from_bits(0x3d4c_cccd));
        m.set_pv_f(MV + 4, 0.25);
        m.set_pv_f(MV, 0.5);
        m.set_pv_f(MV + 0xc, 0.75);
        m.set_pv_f(MV + 0x10, f32::from_bits(0x3f49_0fdc));
        m.set_pv_f(MV + 0x30, f32::from_bits(0x3f06_0a93));
        m.set_pv_b(HEALTH + 8, 3);
        m.set_pv_f(MV + 8, 0.75);
        m.set_pv_i(0x1d8, -1);
        m.set_pv_i(LOOP, -1);
        if m.pv_i(RAIL) == -1 {
            w.delete(id);
            return false;
        }
        m.set_state(1, 0);
        super::props::set_seq(w, id, ctx, 0, 0, 0);
    }
    crate::shadow_volumes::probe_near(w, id, ctx, 48.0, 0x2d);
    let m = &mut w.mobys[id];
    m.set_pv_f(MV + 0x20, WALK.1 * 0.000_277_777_78);
    m.set_pv_f(MV + 0x24, WALK.2 * 0.000_277_777_78);
    m.set_pv_f(MV + 0x28, WALK.0 * 0.016_666_668);
    if ai::take_damage(w, id, ctx, false) != 0 { flamer_alarm(w, id); }
    let area = w.mobys[id].pv_i(AREA);
    ai::find_target(w, id, ctx, 16.0, 0.0, area, None, false);
    true
}

/// Its looped sound 0 (`+0x350`) stopped.
fn flamer_quiet(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let h = w.mobys[id].pv_i(flamer::LOOP);
    if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
    w.mobys[id].set_pv_i(flamer::LOOP, -1);
}

/// `0x41f3b8`: along its rail (the path `+0x210`'s first two points) at `+0x340` of the way, kept level with Ratchet
/// across it (at most `+0x218` aside); with him within 6 across of that place, `+0x340` runs (1/60 a tick) to the end
/// he is not at. Its looped sound 0 while more than 0.5 from it; the frame's move there.
fn flamer_rail(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use crate::hero::physics::dot;
    use flamer::*;
    let m = &w.mobys[id];
    let Some(pts) = usize::try_from(m.pv_i(RAIL)).ok().and_then(|p| ctx.level.paths.get(p)).filter(|p| 2 <= p.len()) else { return };
    let (p0, p1) = ([pts[0][0], pts[0][1], pts[0][2]], [pts[1][0], pts[1][1], pts[1][2]]);
    let d = sub(p1, p0);
    let dir = [d[0], d[1], 0.0];
    // `0x2eff28(side, dir, up)` = up × dir.
    let side = unit(cross([0.0, 0.0, 1.0], dir));
    let dir = unit(dir);
    let target = m.pv_v(TARGET);
    let t = sub(target, p0);
    let reach = m.pv_f(REACH);
    let off = dot([t[0], t[1], 0.0], side).clamp(-reach, reach);
    let on = add(add(p0, scale(d, m.pv_f(ALONG))), scale(side, off));
    let t = sub(target, on);
    let ahead = dot([t[0], t[1], 0.0], dir);
    if dist2(target, on) < 6.0 {
        let to = if ahead < 0.0 { 1.0 } else { 0.0 };
        let m = &mut w.mobys[id];
        m.set_pv_f(ALONG, ease(m.pv_f(ALONG), to, f32::from_bits(0x3c88_8889)));
    }
    if 0.5 <= dist(w.mobys[id].pos, on) {
        let h = w.mobys[id].pv_i(LOOP);
        if h == -1 || !ctx.host.sound_playing(h) {
            let h = ctx.host.play_sound(&w.mobys[id], 0, 4);
            w.mobys[id].set_pv_i(LOOP, h);
        }
    } else {
        flamer_quiet(w, id, ctx);
    }
    go(w, id, ctx, on);
    w.mobys[id].set_pv_i(0x1a8, 0);
}

/// `0x3206d8(yaw to its target, …, rot.z, +0x1b4)`: its turn toward the target.
fn flamer_turn(w: &mut MobyWorld, id: usize) {
    use flamer::*;
    let m = &mut w.mobys[id];
    let t = m.pv_v(TARGET);
    let yaw = atan2(t[0] - m.pos[0], t[1] - m.pos[1]);
    let (mut r, mut v) = (m.rot[2], m.pv_f(MV + 0xb4));
    crate::hero::physics::turn_ease(yaw, TURN.0, TURN.1, TURN.2, &mut r, &mut v, 0);
    m.rot[2] = r;
    m.set_pv_f(MV + 0xb4, v);
}

/// 0x0ffc (`0x41e918`, 5 placed; senses `0x41f1a8`, rail `0x41f3b8`, tail `0x41f680`): a flame turret sliding along
/// a rail, on the framework: health `+0x20` (its damage `+0x34`), reaction `+0x50`, movement `+0x100`, target `+0x1f0`.
/// * **Subs** (`+0x95`): 0 calm — its wait `+0x332` `rand_range(20, 60)`; seeing Ratchet or alarmed, at its end it
///   rises (3, sub 2; sequence 1). 1 wary — its calm `+0x330` `rand_range(120, 150)`; seeing him or alarmed: the
///   flame (5, sub 2; sequence 4); else, its calm out, more than 24 from him across: idle (1, sub 0; sequence 0), or
///   watching (4, sub 2; sequence 3). 2 alert — `+0x34c` `rand_range(400, 500)`; losing him, in state 5 once that is
///   out: back down (2, sub 1; sequence 2). 3 the knock: dead (or its death sequence 8 at frame 8) — smoke (0.5, the
///   light 13) at 1.5 up, its sound 6, the loop stopped, its bolts, deleted; over, the flame (5, sub 2).
/// * **States** (turning toward its target at 0.2°/tick², at most 12°/tick): 3 → 4 (sequence 3). 4 on its rail; 61
///   ticks in, the flame (5, sequence 4). 5 the flame: from 4 long it grows to 16 (`randf(3.5, 4.5)`/60 a tick); the
///   nozzle's angles from its length; at frame 1 of the sequence its joint 2, at 11 joint 3: a hit sphere 0.666 there
///   and one of 1.5 (push 3) at (2.5, 0, 0.5) in its frame (damage `+0x34`, flags 1); at 12 long, 6 (sequence 5). 6
///   on its rail until the sequence wraps: 5 (sequence 4).
/// * **The tail**: the timers down; its lean (across its velocity × −3); two flame puffs at joint 4 each tick
///   (crate::particles::puff: 0.1 jitter, 105000, growing 0.9..1, rising 0.02 a tick, 0xdf204080; the first five ticks
///   long with its alpha stage 2, the second's stage set over the first's: the original's slip).
///
/// Not ported: the flame shot `0x3d2f00` (a moby of class 0x0a02 thrown along its aim at 24/s, `$gp+0x2b94`; Endako
/// does not load the class), the death's
/// debris (`0x327c38`: classes 0x0a0d..0x0a11, through `0x328278`, a body the port has not), the nozzle's joint
/// manipulators (`0x325588` on `+0x220` / `+0x2a0`, joints 0 and 1: the angles `+0x280` / `+0x288` / `+0x300` /
/// `+0x308` are kept), `0x327aa8(2.5, m, +0x360, 5)`.
pub fn update_0ffc(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use flamer::*;
    use super::props::set_seq;
    if !flamer_senses(w, id, ctx) { return; }
    let found = |w: &MobyWorld| w.mobys[id].pv_i(TARGET + 0x14) == 1;
    let first_sub = |w: &mut MobyWorld| { let m = &mut w.mobys[id]; if m.once & 2 != 0 { false } else { m.once |= 2; true } };
    match w.mobys[id].sub_state {
        0 => {
            if first_sub(w) {
                let t = ctx.game.rng.rand_range(0x14, 0x3c);
                w.mobys[id].set_pv_s(WAIT, t as i16);
            }
            if (found(w) || w.mobys[id].pv_s(ALARM) != 0) && count_s(&mut w.mobys[id], WAIT) {
                w.mobys[id].set_state(3, 2);
                set_seq(w, id, ctx, 1, 0, 0x14);
                w.mobys[id].set_pv_s(ALARM, 0);
            }
        }
        1 => {
            if first_sub(w) {
                let t = ctx.game.rng.rand_range(0x78, 0x96);
                w.mobys[id].set_pv_s(CALM, t as i16);
            }
            if found(w) || w.mobys[id].pv_s(ALARM) != 0 {
                w.mobys[id].set_state(5, 2);
                set_seq(w, id, ctx, 4, 0, 0x14);
                w.mobys[id].set_pv_s(ALARM, 0);
            } else if w.mobys[id].pv_s(CALM) == 0 {
                if 24.0 < dist2(w.mobys[id].pos, ctx.hero.pos) {
                    w.mobys[id].set_state(1, 0);
                    set_seq(w, id, ctx, 0, 0, 0x14);
                } else {
                    w.mobys[id].set_state(4, 2);
                    set_seq(w, id, ctx, 3, 0, 0x14);
                }
            }
        }
        2 => {
            if first_sub(w) {
                let t = ctx.game.rng.rand_range(400, 500);
                w.mobys[id].set_pv_i(GIVE_UP, t);
            }
            if !found(w) && w.mobys[id].state == 5 && w.mobys[id].pv_i(GIVE_UP) == 0 {
                w.mobys[id].set_state(2, 1);
                set_seq(w, id, ctx, 2, 0, 0x14);
            }
        }
        3 => {
            let r = ai::flight(w, id, ctx);
            let dying = super::props::seq(w, id, ctx) == 8 && ai::reached(w, id, ctx, 8.0);
            if r & 1 != 0 || dying {
                let m = &w.mobys[id];
                let at = add(to_world(SMOKE_AT, &m.rows), m.pos);
                crate::moby::blast::smoke_burst(w, ctx, 0.5, 13.0, Some(id), at, -1);
                ctx.host.play_sound(&w.mobys[id], 6, 0);
                flamer_quiet(w, id, ctx);
                crate::moby::bolts::drop(w, id, 0, 3, ctx);
                w.delete(id);
                return;
            }
            if r & 2 != 0 {
                w.mobys[id].set_state(5, 2);
                set_seq(w, id, ctx, 4, 0, 0x14);
            }
        }
        _ => {}
    }
    match w.mobys[id].state {
        3 => {
            flamer_turn(w, id);
            w.mobys[id].set_state(4, -1);
            set_seq(w, id, ctx, 3, 0, 0x14);
        }
        4 => {
            flamer_turn(w, id);
            flamer_rail(w, id, ctx);
            if 0x3d <= w.mobys[id].state_ticks as u16 {
                w.mobys[id].set_state(5, -1);
                set_seq(w, id, ctx, 4, 0, 0x14);
            }
        }
        5 => {
            if w.mobys[id].first() {
                let k = ctx.game.rng.randf(3.5, 4.5);
                let m = &mut w.mobys[id];
                m.set_pv_f(FLAME, 4.0);
                m.set_pv_s(0x356, 0);
                m.set_pv_s(0x354, 0);
                m.set_pv_f(GROW, k * 0.016_666_668);
            }
            flamer_turn(w, id);
            flamer_rail(w, id, ctx);
            let m = &mut w.mobys[id];
            let len = m.pv_f(FLAME);
            let a = atan2(len, NOZZLE.0);
            let b = atan2(len, NOZZLE.1);
            m.set_pv_f(0x280, a);
            m.set_pv_f(0x308, b);
            m.set_pv_f(0x288, -b);
            m.set_pv_f(0x300, -a);
            m.set_pv_f(FLAME, ease(len, 16.0, m.pv_f(GROW)));
            let joint = if ai::reached(w, id, ctx, 1.0) { Some(2) } else if ai::reached(w, id, ctx, 11.0) { Some(3) } else { None };
            if let Some(at) = joint.and_then(|j| ctx.host.joint(&w.mobys[id], j)) {
                // The shot's aim (`0x3265b8`): (length, `randf_signed(0, 0.5)`, 0) in its frame; the shot itself is
                // not ported, its random draw is.
                let _ = ctx.game.rng.randf_signed(0.0, 0.5);
                let dmg = w.mobys[id].pv_b(HEALTH + 0x14) as f32;
                ai::attack_sphere(w, id, ctx, f32::from_bits(0x3f2a_7efa), dmg, 1.0, at, 1, 0, 1, 0);
                let m = &w.mobys[id];
                let at2 = add(to_world([2.5, 0.0, 0.5], &m.rows), m.pos);
                ai::attack_sphere(w, id, ctx, 1.5, dmg, 3.0, at2, 1, 0, 1, 0);
            }
            if super::props::wrapped(w, id, ctx) {
                w.mobys[id].set_pv_s(0x356, 0);
                w.mobys[id].set_pv_s(0x354, 0);
            }
            if 12.0 <= w.mobys[id].pv_f(FLAME) {
                w.mobys[id].set_state(6, -1);
                set_seq(w, id, ctx, 5, 0, 0x14);
            }
        }
        6 => {
            flamer_turn(w, id);
            flamer_rail(w, id, ctx);
            if super::props::wrapped(w, id, ctx) {
                w.mobys[id].set_state(5, -1);
                set_seq(w, id, ctx, 4, 0, 0x14);
            }
        }
        _ => {}
    }
    flamer_tail(w, id, ctx);
}

/// `0x41f680` (crate docs of [`update_0ffc`]).
fn flamer_tail(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use flamer::*;
    let m = &mut w.mobys[id];
    let mut t = m.pv_i(GIVE_UP);
    crate::moby::countdown(&mut t);
    m.set_pv_i(GIVE_UP, t);
    count_s(m, CALM);
    count_s(m, WAIT);
    if m.state_ticks != -1 { m.state_ticks = m.state_ticks.wrapping_add(1); }
    // `0x3266c8(m, out, +0x1a0, 0)`: its velocity in its frame (the rows' transpose); the lean from its y.
    let v = m.pv_v(MV + 0xa0);
    let across = m.rows[1][0] * v[0] + m.rows[1][1] * v[1] + m.rows[1][2] * v[2];
    m.rot[0] = across * LEAN;
    let rng = &mut ctx.game.rng;
    let mut spin = rng.randi(0x10);
    if rng.randi(2) != 0 { spin = -spin; }
    let Some(mut at) = ctx.host.joint(&w.mobys[id], 4) else { return };
    at[2] += 0.1;
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let rng = &mut ctx.game.rng;
    let vel = [0.0, 0.0, f32::from_bits(0xbca3_d70a)];
    let rgba = 0xdf20_4080;
    let a = crate::particles::puff(sys, rng, 0.1, 1.0, f32::from_bits(0x3f66_6666), 105_000.0, at, spin, vel, rgba, 0);
    if let Some(a) = a {
        let r = &mut sys.pool.recs[a];
        crate::particles::rec::set_i16(r, 0xa, 5);
        r[9] = 4 + 0x70;
        crate::particles::rec::set_u32(r, 0x24, 2);
        r[0x2a] = 0xdf;
        r[0x2b] = r[0xa];
    }
    let b = crate::particles::puff(sys, rng, 0.1, 1.0, f32::from_bits(0x3f66_6666), 105_000.0, at, spin, vel, rgba, 0);
    if let Some(b) = b {
        // The original writes the life and the frame count to the first puff again here.
        if let Some(a) = a {
            let r = &mut sys.pool.recs[a];
            crate::particles::rec::set_i16(r, 0xa, 5);
            r[9] = 4 + 0x70;
        }
        let first_life = a.map_or(0, |a| sys.pool.recs[a][0xa]);
        let r = &mut sys.pool.recs[b];
        crate::particles::rec::set_u32(r, 0x24, 2);
        r[0x2a] = 0xdf;
        r[0x2b] = first_life;
    }
}

/// The floating mine.
pub const MINE: i16 = 0x0b23;

/// 0x0b23 (`0x3e3298`, 19 placed): a mine floating where it was placed. Its pvars: `+0x00` home, `+0x10` / `+0x14` its
/// two phases and `+0x18` / `+0x1c` their speeds, `+0x20` the knock, `+0x30` ≠ 0 shot proof, `+0x34` its turn speed,
/// `+0x38` / `+0x3c` its turn's accel and most.
/// * **0**: home here; its pose held (sequence 0, speed 0); its phases random and their speeds `randf_signed(20, 40)`
///   and `randf_signed(40, 60)` °/s; its turn 0.1° / 6° a tick; 1.
/// * **1** (while drawn): the phases on; tipping 0.1 × their sines; at home plus 0.75 along the second row of the
///   phases' angles (`0x2f0678`); a hit with flags 0x4010000 (`+0x30` = 0): its direction the knock, 2; within √200 of
///   Ratchet across, it turns side-on to him.
/// * **2** (`rand_range(20, 30)` ticks): its tips four times, the knock at 1/6 a tick, one tick in two a dark puff
///   (`0x2ee160`), turning 6° a tick; then 3.
/// * **3**: its sound 0; a stand-in of class 0x113d in its place throws its pieces (±2.5 across, 5..10 up,
///   90..360°/s, gravity 10..20, sound 1, flags 6) and goes; the explosion (near 9, 30 × its scale, 10 embers, 5
///   billows, 15 flares, flags 0x810001); deleted.
///
/// Not ported: the pose's blend from sequence 0 to 1 as Ratchet is below it (`+0x44` = 1 − his angle down / 90°, its
/// frames refreshed by `0x312af8`: the port's animation has no held two-sequence blend), so it holds sequence 0.
pub fn update_0b23(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let pos = w.mobys[id].pos;
            w.mobys[id].set_pv_v(0, pos);
            ctx.host.set_anim(&w.mobys[id], 0, 0, 0);
            ctx.host.set_anim_speed(&w.mobys[id], 0.0);
            let g = &mut ctx.game.rng;
            let (a, b) = (g.rand_angle(), g.rand_angle());
            let s0 = g.randf_signed(20.0, 40.0);
            let s1 = g.randf_signed(40.0, 60.0);
            let m = &mut w.mobys[id];
            m.set_pv_f(0x10, a);
            m.set_pv_f(0x14, b);
            m.set_pv_f(0x18, s0 * 0.000_290_888_25);
            m.set_pv_f(0x3c, f32::from_bits(0x3dd6_7752));
            m.set_pv_f(0x38, f32::from_bits(0x3ae4_c38a));
            m.set_pv_f(0x1c, s1 * 0.000_290_888_25);
            m.set_state(1, -1);
        }
        1 => {
            if w.mobys[id].drawn == 0 { return; }
            let m = &mut w.mobys[id];
            let (a, b) = (add_rot(m.pv_f(0x10), m.pv_f(0x18)), add_rot(m.pv_f(0x14), m.pv_f(0x1c)));
            m.set_pv_f(0x10, a);
            m.set_pv_f(0x14, b);
            m.rot[0] = 0.1 * a.sin();
            m.rot[1] = 0.1 * b.sin();
            let row = crate::moby::attach::rows_of([a, 0.0, b])[1];
            m.pos = add(scale(row, 0.75), m.pv_v(0));
            if m.pv_i(0x30) == 0 {
                if let Some(h) = w.hit_message(id).filter(|h| h.tmpl.flags & 0x0401_0000 != 0) {
                    let d = h.tmpl.dir;
                    w.mobys[id].set_pv_v(0x20, [d[0], d[1], d[2]]);
                    w.mobys[id].set_pv_f(0x2c, d[3]);
                    w.mobys[id].set_state(2, -1);
                }
            }
            let (pos, hero) = (w.mobys[id].pos, ctx.hero.pos);
            let d = sub(hero, pos);
            // `0x2f0048`: the squared distance across.
            if d[0] * d[0] + d[1] * d[1] < 200.0 {
                let m = &mut w.mobys[id];
                let yaw = add_rot(atan2(d[0], d[1]), f32::from_bits(0x3fc9_0fdb));
                let (mut r, mut v) = (m.rot[2], m.pv_f(0x34));
                crate::hero::physics::turn_ease(yaw, m.pv_f(0x38), m.pv_f(0x38), m.pv_f(0x3c), &mut r, &mut v, 0);
                m.rot[2] = r;
                m.set_pv_f(0x34, v);
            }
        }
        2 => {
            if w.mobys[id].first() {
                let t = ctx.game.rng.rand_range(0x14, 0x1e);
                let m = &mut w.mobys[id];
                m.state_ticks = t as i16;
                m.rot[0] *= 4.0;
                m.rot[1] *= 4.0;
                m.set_pv_v(0x20, set_len(m.pv_v(0x20), f32::from_bits(0x3e2a_aaab)));
            }
            if ctx.game.rng.randi(2) == 0 {
                let pos = w.mobys[id].pos;
                dark_puffs(ctx, f32::from_bits(0x3bda_740f), f32::from_bits(0x3f66_6666), f32::from_bits(0x3f82_8f5c), pos, [0.0, 0.0, f32::from_bits(0x3d88_8889)], 1, 0x4020_2020, true);
            }
            let m = &mut w.mobys[id];
            m.rot[2] = add_rot(m.rot[2], f32::from_bits(0x3e56_7752));
            m.pos = add(m.pos, m.pv_v(0x20));
            let t = m.state_ticks;
            m.state_ticks = t.wrapping_sub(1);
            if t == 0 { m.set_state(3, -1); }
        }
        3 => {
            ctx.host.play_sound(&w.mobys[id], 0, 0);
            if w.class(0x113d).is_some() {
                let alpha = w.mobys[id].alpha;
                let c = super::props::spawn_in_place(w, id, 0x113d, 0, &*ctx.host);
                w.mobys[c].alpha = alpha;
                crate::moby::shards::throw_all(w, c, ctx, [-2.5, -2.5, 5.0, 0.0, 2.5, 2.5, 10.0, 0.0], (90.0, 360.0), (10.0, 20.0), 0xffff, 1, -1, 6, 0xffff);
                w.delete(c);
            }
            let size = w.mobys[id].scale * 30.0;
            let e = crate::moby::blast::Explosion { near: 9.0, size, embers: 10, billows: 5, flares: 0xf, sound: -1, lod: -1, flags: 0x81_0001, ..Default::default() };
            crate::moby::blast::explode(w, ctx, Some(id), None, &e);
            w.delete(id);
        }
        _ => {}
    }
}

/// `0x2ee160(spread, k, grow, pos, vel, n, rgba, flat)`: `n` puffs (crate::particles::puff, 0.5·k jitter, growing
/// 1..`grow`, 70000..150000 × k, moving `vel` plus up to `spread` a random way, `rgba`), 120 ticks long with their
/// alpha stage 2 at `rgba` dimmed by `randf(0.2, 1)`; `flat`: not additive (ALPHA 0x44).
#[allow(clippy::too_many_arguments)]
pub fn dark_puffs(ctx: &mut Ctx, spread: f32, k: f32, grow: f32, pos: V3, vel: V3, n: i32, rgba: u32, flat: bool) {
    for _ in 0..n {
        let rng = &mut ctx.game.rng;
        let size = k * rng.randf(70_000.0, 150_000.0);
        let t = rng.randf(0.2, 1.0);
        let dim = lerp_rgba(t, 0, rgba);
        let v = add(rng.rand_vec(0.0, spread), vel);
        let Some(sys) = ctx.parts.as_deref_mut() else { continue };
        if let Some(p) = crate::particles::puff(sys, &mut ctx.game.rng, k * 0.5, 1.0, grow, size, pos, 0, v, rgba, 0) {
            let r = &mut sys.pool.recs[p];
            crate::particles::rec::set_i16(r, 0xa, 0x78);
            crate::particles::rec::set_u32(r, 0x24, 2);
            r[0x2a] = (dim >> 24) as u8;
            r[0x2b] = r[0xa];
            if flat { r[3] = 0x44; }
        }
    }
}

/// 0x0944 (`0x3c75f8`, five placed; no Ghidra function): an energy field. Its pvars: `+0x00` its size, `+0x04` /
/// `+0x10` two phases, `+0x08` / `+0x0c` two turns, `+0x14` its strength, `+0x18` its looped sound. **0**: its model
/// hidden (mode | 1, `+0x31` 0), its strength 1, 1 (its sphere's radius `+0x2c` the class's `+0x24` × its size: the
/// port's rebuild keeps its own). **1**: the phases turn 240° and 720° a second, the turns 87° and −120° (`$gp+0x1020..`
/// on L03); its draw ([`draw_0944`]) on the frame's first list (`0x2e8310`); hooked (`0x32bf38`), its strength fades by
/// 1/30 a tick, and at 0 it goes off without collision (`+0x98` 0), 2. Its class sound 0 loops while in 1 and stops
/// otherwise.
pub fn update_0944(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let k = 0.000_290_888_25;
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.set_state(1, -1);
            m.mode |= 1;
            m.drawn = 0;
            m.set_pv_f(0x14, 1.0);
        }
        1 => {
            let m = &mut w.mobys[id];
            m.set_pv_f(4, add_rot(m.pv_f(4), 240.0 * k));
            m.set_pv_f(0x10, add_rot(m.pv_f(0x10), 720.0 * k));
            m.set_pv_f(8, add_rot(m.pv_f(8), 87.0 * k));
            m.set_pv_f(0xc, add_rot(m.pv_f(0xc), -120.0 * k));
            w.register_draw_first(id);
            if w.mobys[id].once & 4 != 0 {
                let m = &mut w.mobys[id];
                let mut f = m.pv_f(0x14);
                crate::hero::physics::approach(0.0, f32::from_bits(0x3d08_8889), &mut f);
                m.set_pv_f(0x14, f);
                if f <= 0.0 {
                    m.set_state(2, -1);
                    m.has_collision = false;
                }
            }
        }
        _ => {}
    }
    let h = w.mobys[id].pv_i(0x18);
    if w.mobys[id].state == 1 {
        if !(h != -1 && ctx.host.sound_playing(h)) {
            let h = ctx.host.play_sound(&w.mobys[id], 0, 4);
            w.mobys[id].set_pv_i(0x18, h);
        }
    } else {
        if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
        w.mobys[id].set_pv_i(0x18, -1);
    }
}

/// `0x3c7810`: 0x0944's field, in its frame (its rows × its scale × its size, at its place), additive (ALPHA
/// `Cs·As + Cd`), every colour's alpha × its strength. The quads go round its x axis, a pair of corners stepping on
/// alternately (`0x2ec4f0` each): **1** (FX 5) a disc from 1 to 4.5 out, 32 quads of π/16, the colours between
/// `0x60800000` / `0x60806000` and `0x60806010` / `0x60800010` by the first phase; **2** (FX 5) two arcs 4.5 out and
/// `0.5 + 0.1·p` thick from 30° up, 16 quads of −π/48 each, the second mirrored below, between `0x60804040` /
/// `0x60808080` and `0x60800000` by the second phase; **3** and **4** (FX 0xf) half rings 2.8..3.25 and 2.25..2.7 out
/// turned by the two turns, 20 quads of π/20 ($gp+0x1030..0x110c on L03).
pub fn draw_0944(w: &MobyWorld, id: usize, out: &mut Vec<crate::fx::FxQuad>) {
    use crate::hero::physics::euler_rows;
    let m = &w.mobys[id];
    let size = m.pv_f(0);
    let s = m.scale * if size != 1.0 { size } else { 1.0 };
    let (rows, pos) = (m.rows.map(|r| scale(r, s)), m.pos);
    let world = |v: V3| add(pos, add(add(scale(rows[0], v[0]), scale(rows[1], v[1])), scale(rows[2], v[2])));
    let turn = |v: V3, r: &[V3; 3]| -> V3 { std::array::from_fn(|j| v[0] * r[0][j] + v[1] * r[1][j] + v[2] * r[2][j]) };
    let mul = |a: &[V3; 3], b: &[V3; 3]| -> [V3; 3] { std::array::from_fn(|i| turn(a[i], b)) };
    let fade = m.pv_f(0x14);
    let faded = |c: u32| lerp_rgba(fade, c & 0xff_ffff, c);
    let p1 = m.pv_f(4).sin() * 0.5 + 0.5;
    let p2 = m.pv_f(0x10).sin() * 0.5 + 0.5;
    let emit = |out: &mut Vec<crate::fx::FxQuad>, fx: usize, v: &[V3; 4], st: &[[f32; 2]; 4], rgba: &[u32; 4]| {
        out.push(crate::fx::FxQuad { fx, additive: true, corners: v.map(|p| world(p)), st: *st, rgba: *rgba });
    };
    // A ring: the second pair turned by `a`, then `n` quads, a pair turned by 2a before each (the first pair first
    // unless `second`).
    let ring = |out: &mut Vec<crate::fx::FxQuad>, v: &mut [V3; 4], a: f32, n: usize, second: bool, fx: usize, st: &[[f32; 2]; 4], rgba: &[u32; 4]| {
        let r = euler_rows([a, 0.0, 0.0]);
        v[2] = turn(v[2], &r);
        v[3] = turn(v[3], &r);
        let r2 = mul(&r, &r);
        for i in 0..n {
            let k = if (i & 1 == 0) != second { 0 } else { 2 };
            v[k] = turn(v[k], &r2);
            v[k + 1] = turn(v[k + 1], &r2);
            emit(out, fx, v, st, rgba);
        }
    };
    // 1: the disc.
    let st1 = [[0.5, 0.5], [0.5, 1.5], [0.5, 0.5], [0.5, 1.5]];
    let c1: [u32; 4] = std::array::from_fn(|k| faded(if k & 1 == 0 { lerp_rgba(p1, 0x6080_0000, 0x6080_6000) } else { lerp_rgba(p1, 0x6080_6010, 0x6080_0010) }));
    let (outer, inner) = ([0.0, 0.0, 4.5], [0.0, 0.0, 1.0]);
    let mut v = [outer, inner, outer, inner];
    ring(out, &mut v, 0.196_349_55, 32, false, 5, &st1, &c1);
    // 2: the arcs.
    let st2 = [[0.5, 0.5], [0.5, 1.0], [0.5, 0.5], [0.5, 1.0]];
    let c2: [u32; 4] = std::array::from_fn(|k| faded(if k & 1 == 0 { lerp_rgba(p2, 0x6080_4040, 0x6080_8080) } else { lerp_rgba(p2, 0x6080_0000, 0x6080_0000) }));
    let up = euler_rows([0.523_598_8, 0.0, 0.0]);
    let thin = [0.0, 0.0, 4.5 - (p2 * 0.1 + 0.5)];
    let mut v = [turn(outer, &up), turn(thin, &up), turn(outer, &up), turn(thin, &up)];
    ring(out, &mut v, -0.065_449_85, 16, false, 5, &st2, &c2);
    for p in v.iter_mut() { p[2] = -p[2]; }
    let r2 = mul(&euler_rows([-0.065_449_85, 0.0, 0.0]), &euler_rows([-0.065_449_85, 0.0, 0.0]));
    for i in 0..16 {
        let k = if i & 1 == 0 { 2 } else { 0 };
        v[k] = turn(v[k], &r2);
        v[k + 1] = turn(v[k + 1], &r2);
        emit(out, 5, &v, &st2, &c2);
    }
    // 3 and 4: the half rings.
    let st3 = [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]];
    let c3 = [0x6080_0000, 0x6080_0000, 0x6080_8000, 0x6080_8000].map(faded);
    for (turned, (a, b)) in [(m.pv_f(8), (3.25, 2.8)), (m.pv_f(0xc), (2.7, 2.25))] {
        let r = euler_rows([turned, 0.0, 0.0]);
        let mut v = [a, b, a, b].map(|z| turn([0.0, 0.0, z], &r));
        ring(out, &mut v, 0.157_079_64, 20, false, 0xf, &st3, &c3);
    }
}
