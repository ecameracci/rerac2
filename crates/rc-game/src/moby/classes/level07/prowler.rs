//! Level 7's prowler **0x0d29** (L07 `0x3e7520`, 37 placed; its set-up and checks `0x3e7cf8`, its tail `0x3e8140`,
//! its death `0x3e8580`): 0x0b0b's body (its two spinning parts 0x0aa2 at its joints 0 and 2, `+0x330` / `+0x334`,
//! and its three glows, crate::fx's `0x3dc220`) on the framework ([`crate::moby::ai`]) with its own mind: health
//! `+0x20` (its damage byte `+0x34`, `+0x2e` = 2 a kill asked by another), movement `+0x50`, reaction `+0x140` (hurts
//! into state 8, sub-state 3), target `+0x210` (found `+0x2d4`).
//!
//! Its words: `+0x2b0` its area (its first path its fence), `+0x2b4` its sight's range, `+0x2b8` a moby group it
//! waits on (−1 none), `+0x2bc` 0 hidden at the start; `+0x2d0` alarmed (a hurt alarms its group), `+0x2dc` its
//! parts' spin, `+0x2f0` home, `+0x300` the stroll's goal, `+0x344` / `+0x348` timers, `+0x350` the level's chunk it
//! lives in (1 above the first variant plane: it updates only while that chunk is in place).
//!
//! **Sub-states**: 0 calm — seeing Ratchet, the chase (state 6, sub 2); alarmed, idle (3, sub 1) and `+0x344`
//! `rand_range(480, 600)` if under 300. 1 alert — seeing him, the chase. 2 hunting — on its first tick the movement
//! limits (turn 20/3600 a tick², 6/60 most; walk 720″, 1440·1/3438); each tick the follow camera pulled out to 6 and
//! its pivot to 3 (`0x422648` / `0x422690`, rate 0.003); losing him (not attacking) the wander (4, sub 1), `+0x344`
//! `rand_range(580, 600)`. 3 the hurt's flight: dead → [`die`]; over, the attack (7, sub 2).
//!
//! **States**: 1 idle (sequence 2) to its end → 2 a stroll (sequence 8) to a point `randf(0, 5)` from home a random
//! way, done there, stuck past 40 ticks or after 181 → 1. 3 idle (one of the sequences 6 / 7 / 12 / 13) to its end → 4
//! a wander (sequence 8) to a point `randf(0, 3)` from home, facing its target and not turning to walk, as long → 3.
//! 6 the chase (sequence 0xb) toward the target; within 2.5 across → 7. 7 the swipe (sequence 0xc from frame 5): facing
//! the target until frame 19, frames 19..22 spheres of 1/3 at its two parts (damage `+0x34`, flags 1); at its end on
//! again while within 3 and facing within 20°, else the chase. 9 waiting while its group `+0x2b8` has a member alive
//! and its health is full → 10 (sequence 1) to its end → the chase.
//!
//! The engine functions: `0x31e3d0` = crate::moby::ai::take_damage, `0x31f118` = crate::moby::ai::flight, `0x321198`
//! = crate::moby::ai::find_target, `0x322d80` = crate::moby::ai::move_step (`0x322b60` toward a point, `0x322bb8` with
//! the options 0x10, `0x322c90` = crate::moby::ai::move_toward), `0x323f08` = crate::moby::ai::senses, `0x323dc8` /
//! `0x323e28` its facing eased at the movement limits, `0x312bb8` = crate::moby::classes::props::group_alive,
//! `0x313428` = crate::moby::ai::attack_sphere, `0x313e30` = crate::shadow_volumes::probe_near, `0x3eb618` /
//! `0x3eb6f8` / `0x3eb7d8` = crate::moby::classes::enemies' parts (L00 `0x3dbed0` / `0x3dbfb0` / `0x3dc090`),
//! `0x3eb848` → `0x3eb968` = crate::fx's `0x3dc220`, `0x3e3a50` = crate::moby::blast::burst, `0x310fd0` =
//! crate::moby::bolts::drop, `0x31dd30` = crate::moby::shards::break_into.
//!
//! Not ported: its life in the level's chunk 1 (`0x3e8328`: riding the moving moby `0x1bf5d8` it lands on, `+0x338`
//! / `+0x33c` / `+0x310` / `+0x320`, the carry `0x189f70` added to its target, its fall height cleared on a hurt; the
//! idles and the blast wait on `+0x338` too), the hidden start's way out (state 0xb: no class here sets it), its parts'
//! sparks in the hunt (`0x3eb368`, one in six ticks; as 0x0b0b's), the big-head cheat's scale (`0x31dba0`, `+0x230`).

use crate::hero::physics::{add, atan2, dist, dist2, rot_dist, set_len, turn_ease, V3};
use crate::moby::classes::enemies;
use crate::moby::classes::props::{group_alive, seq, set_seq, wrapped};
use crate::moby::{ai, Ctx, MobyWorld};

pub const CLASS: i16 = 0x0d29;
const MV: usize = 0x50;
const TARGET: usize = 0x210;
const SEEN: usize = 0x2d4;
const ALARM: usize = 0x2d0;
const HOME: usize = 0x2f0;
const GOAL: usize = 0x300;
const PARTS: usize = 0x330;
const TIMER: usize = 0x344;
/// `$gp+0x1aa8` (`0x1b0a98`): its reach.
const REACH: f32 = 2.5;
/// `0x1b0ab8`: its idles.
const IDLES: [u8; 4] = [6, 7, 12, 13];

/// `0x320c50`: the first tick since the last sub-state change.
fn first_sub(m: &mut crate::moby::Moby) -> bool {
    if m.once & 2 != 0 { return false; }
    m.once |= 2;
    true
}

/// `0x323dc8(yaw, m)`: its facing eased toward `yaw` at the movement block's turn limits.
fn face(w: &mut MobyWorld, id: usize, yaw: f32) {
    let m = &mut w.mobys[id];
    let (a, b, c) = (m.pv_f(MV + 0x14), m.pv_f(MV + 0x18), m.pv_f(MV + 0x1c));
    let (mut r, mut v) = (m.rot[2], m.pv_f(MV + 0xb4));
    turn_ease(yaw, a, b, c, &mut r, &mut v, 0);
    m.rot[2] = r;
    m.set_pv_f(MV + 0xb4, v);
}

fn yaw_to(w: &MobyWorld, id: usize, p: V3) -> f32 {
    let q = w.mobys[id].pos;
    atan2(p[0] - q[0], p[1] - q[1])
}

/// `0x3e8280`: its group alarmed (`+0x2d0` of every member).
fn alarm(w: &mut MobyWorld, id: usize) {
    let Some(members) = usize::try_from(w.mobys[id].group).ok().and_then(|g| w.groups.get(g)).cloned() else { return };
    for i in members {
        if let Some(m) = w.mobys.get_mut(i) { m.set_pv_i(ALARM, 1); }
    }
}

/// `0x3e7cf8` (module docs). Returns whether it is to go.
fn checks(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    if w.mobys[id].state == 0 {
        let rng = &mut ctx.game.rng;
        let mirror = rng.randi(2) != 0;
        let weave = if rng.randi(2) != 0 { -15.0 } else { 15.0 } * 0.017_453_294;
        let m = &mut w.mobys[id];
        m.set_pv_b(0x48, 15);
        m.set_pv_b(0x4a, 9);
        let home = m.pos;
        m.set_pv_v(HOME, home);
        if mirror { m.mode |= crate::moby::mode::MIRROR } else { m.mode &= !crate::moby::mode::MIRROR }
        m.set_pv_b(0x150, 8);
        m.set_pv_i(0x188, 3);
        m.set_pv_f(0x17c, 0.15);
        m.set_pv_f(0x170, 1.0);
        m.set_pv_f(0x174, 1.0);
        m.set_pv_f(0x180, m.pos[2] - 3.0);
        m.set_pv_f(MV + 4, 0.3);
        m.set_pv_f(MV, 1.0);
        m.set_pv_f(MV + 0x10, std::f32::consts::PI);
        m.set_pv_f(MV + 0xdc, weave);
        m.set_pv_s(MV + 0xe2, 30);
        m.set_pv_b(0x2e, 1);
        m.set_pv_s(MV + 0xe4, 90);
        enemies::make_parts(w, id, id, PARTS, ctx);
        let m = &mut w.mobys[id];
        if ctx.level.variant_planes.first().is_some_and(|(o, _)| o[2] < m.pos[2]) {
            m.set_pv_i(0x350, 1);
            m.update_dist = 0xff;
        }
        m.pos[2] += 0.5;
        let (a, b) = (m.pos, [m.pos[0], m.pos[1], f32::from_bits(0x3c23_d70a)]);
        let a = [a[0], a[1], a[2] + 0.5];
        if let Some(hit) = ctx.coll.and_then(|c| crate::collision::coll_line_m(c, ctx.scene, a, b, crate::collision::QueryFlags(2), Some(id))) {
            w.mobys[id].pos = hit.point;
        }
        let area = w.mobys[id].pv_i(0x2b0);
        if area != -1 {
            let path = usize::try_from(area).ok().and_then(|a| ctx.level.areas.get(a)).and_then(|a| a.lists[0].first().copied()).unwrap_or(-1);
            w.mobys[id].set_pv_i(MV + 0x3c, path);
        }
        let m = &mut w.mobys[id];
        let (state, sub, s) = if m.pv_i(0x2bc) == 0 {
            m.has_collision = false;
            m.mode = (m.mode & 0xefff) | 0x41;
            (0xb, 4, 0)
        } else if m.pv_i(0x2b8) == -1 {
            (1, 0, 2)
        } else {
            (9, 4, 0)
        };
        m.set_state(state, sub);
        set_seq(w, id, ctx, s, 0, 0);
        w.changed = true;
    }
    crate::shadow_volumes::probe_near(w, id, ctx, 24.0, 0x16);
    // A hit from a moby within 32 with the world between them is dropped.
    if let Some(h) = w.hit_message(id).filter(|h| h.tmpl.flags & 0x10000 != 0) {
        if let Some(s) = h.tmpl.source.and_then(|s| w.mobys.get(s)).filter(|s| dist(s.pos, w.mobys[id].pos) < 32.0) {
            let (p, q) = (s.pos, w.mobys[id].pos);
            let (a, b) = ([p[0], p[1], p[2] + 0.5], [q[0], q[1], q[2] + 0.5]);
            if ctx.coll.and_then(|c| crate::collision::coll_line_m(c, ctx.scene, a, b, crate::collision::QueryFlags(2), Some(id))).is_some() { w.clear_hit(id); }
        }
    }
    if ai::take_damage(w, id, ctx, false) != 0 {
        alarm(w, id);
        if w.mobys[id].pv_f(0x20) <= 0.0 {
            die(w, id, ctx);
            return true;
        }
    }
    if w.mobys[id].pv_b(0x2e) == 2 {
        enemies::delete_parts(w, id, PARTS);
        return true;
    }
    let (range, area) = (w.mobys[id].pv_f(0x2b4), w.mobys[id].pv_i(0x2b0));
    let seen = ai::find_target(w, id, ctx, range, 0.0, area, None, false);
    w.mobys[id].set_pv_i(SEEN, seen as i32);
    false
}

/// `0x3e8580`: no collision and hidden, its parts gone, its bolts (3), a blast 0.75 up its frame (0.6: bursts 0x19 /
/// 0x14 / 0x14, 20 puffs, 45 streaks, the flash), its sound 6 and its pieces (mask 0x3f8, 3..6 / 3..6, along its knock
/// at 0.05).
fn die(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    m.has_collision = false;
    m.mode = (m.mode & 0xefff) | 0x41;
    enemies::delete_parts(w, id, PARTS);
    crate::moby::bolts::drop(w, id, 0, 3, ctx);
    let m = &w.mobys[id];
    let at = add(m.pos, crate::hero::physics::scale(m.rows[2], 0.75));
    let b = crate::moby::blast::Blast { size: f32::from_bits(0x3f19_999a), bursts: [0x19, 0x14, 0x14], puffs: 0x14, streaks: 0x2d, light: true, shake: false };
    crate::moby::blast::burst(w, ctx, at, &b);
    ctx.host.play_sound(&w.mobys[id], 6, 0);
    let v = set_len(w.mobys[id].pv_v(0x1c0), f32::from_bits(0x3d4c_ccce));
    crate::moby::shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 6.0), 0x3f8, v, 0xff);
}

/// The stroll's or wander's goal: `randf(0, r)` from home a random way.
fn goal(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, r: f32) {
    let r = ctx.game.rng.randf(0.0, r);
    let a = ctx.game.rng.rand_angle();
    let home = w.mobys[id].pv_v(HOME);
    w.mobys[id].set_pv_v(GOAL, [a.cos() * r + home[0], a.sin() * r + home[1], home[2]]);
}

/// The stroll's and wander's end: at the goal (flags 0x80 alone), stuck past 40 ticks, or after 181.
fn done(w: &MobyWorld, id: usize, flags: i32) -> bool {
    let m = &w.mobys[id];
    flags == 0x80 || (m.pv_i(MV + 0x80) != 0 && 0x28 < m.state_ticks) || 0xb4 < m.state_ticks
}

/// 0x0d29 (module docs).
pub fn update_0d29(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    // `0x1b1d88` (the frame's count) past 1: only in the chunk it lives in.
    if 1 < ctx.game.frame && ctx.game.variant != w.mobys[id].pv_i(0x350) { return; }
    if checks(w, id, ctx) {
        w.delete(id);
        return;
    }
    let rand_timer = |ctx: &mut Ctx, lo: i32| ctx.game.rng.rand_range(lo, 600);
    match w.mobys[id].sub_state {
        0 => {
            let m = &mut w.mobys[id];
            if m.pv_i(SEEN) == 1 {
                m.set_state(6, 2);
            } else if m.pv_i(ALARM) != 0 {
                m.set_state(3, 1);
                m.set_pv_i(ALARM, 0);
                if w.mobys[id].pv_i(TIMER) < 300 {
                    let t = rand_timer(ctx, 0x1e0);
                    w.mobys[id].set_pv_i(TIMER, t);
                }
            }
        }
        1 => {
            if w.mobys[id].pv_i(SEEN) == 1 { w.mobys[id].set_state(6, 2); }
        }
        2 => {
            ctx.game.cam_tweaks.follow_dist = Some((6.0, f32::from_bits(0x3b44_9ba6)));
            ctx.game.cam_tweaks.follow_pivot = Some((3.0, f32::from_bits(0x3b44_9ba6)));
            if first_sub(&mut w.mobys[id]) {
                ai::senses(&mut w.mobys[id], 20.0 * 0.000_277_777_78, 6.0 * 0.016_666_668, 720.0 * 4.848_137_5e-6, 1440.0 * 0.000_290_888_25);
            }
            let m = &w.mobys[id];
            if m.pv_i(SEEN) != 1 && m.state != 7 {
                w.mobys[id].set_state(4, 1);
                let t = rand_timer(ctx, 0x244);
                w.mobys[id].set_pv_i(TIMER, t);
            }
        }
        3 => {
            let res = ai::flight(w, id, ctx);
            if res & 1 != 0 {
                die(w, id, ctx);
                w.delete(id);
                return;
            }
            if res & 2 != 0 {
                w.mobys[id].set_state(7, 2);
                let t = rand_timer(ctx, 0x244);
                w.mobys[id].set_pv_i(TIMER, t);
            }
        }
        _ => {}
    }
    states(w, id, ctx);
    tail(w, id, ctx);
}

fn states(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let next = match w.mobys[id].state {
        1 => {
            if w.mobys[id].first() { set_seq(w, id, ctx, 2, 0, 0x14); }
            if !wrapped(w, id, ctx) { return; }
            2
        }
        2 => {
            if w.mobys[id].first() {
                set_seq(w, id, ctx, 8, 0, 0x14);
                goal(w, id, ctx, 5.0);
            }
            let g = w.mobys[id].pv_v(GOAL);
            let yaw = yaw_to(w, id, g);
            let flags = ai::move_step(w, id, ctx, yaw, g);
            if !done(w, id, flags) { return; }
            1
        }
        3 => {
            if w.mobys[id].first() {
                let r = IDLES[ctx.game.rng.randi(4) as usize];
                if seq(w, id, ctx) != r {
                    let s = IDLES[ctx.game.rng.randi(4) as usize];
                    ctx.host.set_anim(&w.mobys[id], s, 0, 0x14);
                }
            }
            if !wrapped(w, id, ctx) { return; }
            4
        }
        4 => {
            if w.mobys[id].first() {
                set_seq(w, id, ctx, 8, 0, 0x14);
                goal(w, id, ctx, 3.0);
            }
            let t = w.mobys[id].pv_v(TARGET);
            let yaw = yaw_to(w, id, t);
            face(w, id, yaw);
            let g = w.mobys[id].pv_v(GOAL);
            let yaw = yaw_to(w, id, g);
            let o = MV + 0x38;
            let keep = w.mobys[id].pv_i(o);
            w.mobys[id].set_pv_i(o, 0x10);
            let flags = ai::move_step(w, id, ctx, yaw, g);
            w.mobys[id].set_pv_i(o, keep);
            if !done(w, id, flags) { return; }
            3
        }
        6 => {
            if w.mobys[id].first() { set_seq(w, id, ctx, 0xb, 0, 0x14); }
            let t = w.mobys[id].pv_v(TARGET);
            ai::move_toward(w, id, ctx, 0.0, t);
            if REACH <= dist2(w.mobys[id].pos, w.mobys[id].pv_v(TARGET)) { return; }
            7
        }
        7 => {
            if w.mobys[id].first() && seq(w, id, ctx) != 0xc { ctx.host.set_anim(&w.mobys[id], 0xc, 5, 10); }
            let f = ctx.host.frame_time(&w.mobys[id]);
            let t = w.mobys[id].pv_v(TARGET);
            if f < 19.0 {
                let yaw = yaw_to(w, id, t);
                face(w, id, yaw);
            } else if f <= 22.0 {
                let damage = w.mobys[id].pv_b(0x34) as f32;
                for p in enemies::parts(w, id, PARTS).into_iter().flatten() {
                    let at = w.mobys[p].pos;
                    ai::attack_sphere(w, id, ctx, f32::from_bits(0x3eaa_7efa), damage, 1.0, at, 1, 0, 1, 0);
                }
            }
            if !wrapped(w, id, ctx) { return; }
            let m = &w.mobys[id];
            if dist2(m.pos, t) <= REACH + 0.5 && rot_dist(m.rot[2], yaw_to(w, id, t)) <= 0.349_065_9 { return; }
            6
        }
        9 => {
            let m = &w.mobys[id];
            if group_alive(w, m.pv_i(0x2b8)) != 0 && m.pv_s(0x24) as f32 <= m.pv_f(0x20) { return; }
            10
        }
        10 => {
            if w.mobys[id].first() { set_seq(w, id, ctx, 1, 0, 2); }
            if wrapped(w, id, ctx) { w.mobys[id].set_state(6, -1); }
            return;
        }
        _ => return,
    };
    w.mobys[id].set_state(next, -1);
}

/// `0x3e8140`: its timers, its state's ticks, its glows (crate::fx), its parts spun (12° a tick hunting, else 2/3°).
fn tail(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    for o in [TIMER, 0x348] {
        let mut t = m.pv_i(o);
        crate::moby::countdown(&mut t);
        m.set_pv_i(o, t);
    }
    if m.state_ticks != -1 { m.state_ticks = m.state_ticks.wrapping_add(1); }
    w.register_draw(id);
    let m = &mut w.mobys[id];
    let step = if m.sub_state == 2 { f32::from_bits(0x3e56_7752) } else { f32::from_bits(0x3c3e_a2f4) };
    let a = crate::hero::physics::add_rot(m.pv_f(0x2dc), step);
    m.set_pv_f(0x2dc, a);
    enemies::spin_parts(w, id, id, PARTS, ctx, a);
    w.changed = true;
}
