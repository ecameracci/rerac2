//! Level 7's gunner **0x0d2a** (L07 `0x3ebc40`, 34 placed; its set-up and checks `0x3ec600`, its tail `0x3eca40`,
//! its death `0x3ecdf8`) and its fireball **0x0b7f** (made by `0x3cbba0`, its update `0x3cb618`, its draw
//! `0x3cbcc8`), on the framework ([`crate::moby::ai`]): health `+0x20` (its damage byte `+0x34`), movement `+0x50`
//! (options |= 0x201, gravity 1/180), reaction `+0x140` (hurts into state 6, sub-state 3), target `+0x210` (found
//! `+0x2dc`).
//!
//! Its words: `+0x2b0` its area (its first path its fence), `+0x2b4` its sight's range, `+0x2b8` a moby group it
//! waits on (−1 none), `+0x2bc` 0 hidden at the start, `+0x2c0` / `+0x2c4` the cuboids it jumps from and to (−1
//! none), `+0x2d0` alarmed (a hurt alarms its group), `+0x2d4` / `+0x2d8` timers, `+0x2e0` home, `+0x320` the chunk it
//! lives in (as crate::moby::classes::level07::prowler's `+0x350`), `+0x324` the strafe's heading.
//!
//! **Sub-states**: 0 calm — seeing Ratchet, the strafe (state 5, sub 2); alarmed, sub 1 (state 2) and `+0x2d4`
//! `rand_range(480, 600)` if under 300. 1 alert — seeing him, the strafe; its timer out, calm (state 1). 2 fighting —
//! the follow camera pulled out to 6 and its pivot to 3 each tick (as the prowler's); losing him, alert (state 2,
//! `+0x2d4` `rand_range(580, 600)`). 3 the hurt's flight: dead → [`die`]; over, the strafe or alert.
//!
//! **States**: 1 idle (sequence 2), 2 alert (sequence 5). 3 the shot (sequence 0x14): at frame 17 a fireball from its
//! joint 4 at 20 a second toward Ratchet's aim point (or the target's), kept within 20° of its facing; at its end
//! one time in two facing him within 20° → 4 (sequence 0xe, then 3 again), else → 5. 5 the strafe (sequence 7): a
//! heading 45°..100° to a random side of him, walking it without turning while facing the target, stuck past 30
//! ticks or after 121 → 3. 7 waiting while its group `+0x2b8` has a member alive and its health is full → 8
//! (sequence 1) to its end → 9 with both cuboids (else the shot, sub 2). 9 to the jump (sequence 10): toward the
//! middle of its cuboid `+0x2c0` across (its local x 0.5); inside it, the jump (0.1 a tick across, gravity 1/180) to
//! the same place in `+0x2c4` at a random x → 10 the jump (sequence 4, crate::moby::ai::jump_step 1 / 8 / 12.5): its
//! fall height 3 under it; landed facing the target within 10° the shot, else the strafe (sub 2).
//!
//! **The fireball** ([`update_0b7f`]): spinning (`+0x18` by `+0x1c` a tick, its rows from (½, ¼, 1)·the spin), it
//! moves `+0x00` a tick; the step's line (flags 0) or a sphere of 0.3 at it meeting anything but its maker hits there
//! (a sphere of 0.3 from its maker, damage `+0x14`, flags 0x10001) and it blows up the next tick (the general
//! explosion: 10 embers, 16 flares, 9 debris, its light 15; deleted). In flight, three sprites trail it each tick
//! (5 ticks, 0x808080; 1..1.5 → 0.2..0.4, 0.1..0.3 and 0.2..0.4 times 210000 by its size, alphas 0x4f / 0x5f / 0x7f,
//! the last two jittered 0.15), and its draw two glows (effect texture 0xb, additive): 45 times its scale, 0x103080 at
//! alpha 0x5f..0x6f, and 22.5 times, 0x3f3f3f at 0x6f..0x7f. Its size `+0x20` grows 0.025 a tick to 1, its scale
//! `randf(0.01, 0.0125)` of it.
//!
//! Not ported: as the prowler's, its life in the level's chunk 1 (`0x3ecb78`; the fireball's carry by its maker's
//! platform `+0x24`, `0x318370` / `0x318120` / `0x318290`), the hidden start's way out (state 0xb), the big-head
//! cheat's scale (`0x31dba0`); the template the fireball's line test carries (`0x2d4778`'s last argument: the sphere's
//! hit stands for it).

use crate::hero::physics::{add, add_rot, atan2, dot, euler_rows, rot_dist, scale, set_len, sub, turn_ease, V3};
use crate::moby::classes::props::{group_alive, seq, set_seq, wrapped};
use crate::moby::{ai, Ctx, MobyWorld};

pub const CLASS: i16 = 0x0d2a;
pub const SHOT: i16 = 0x0b7f;
const MV: usize = 0x50;
const TARGET: usize = 0x210;
const SEEN: usize = 0x2dc;
const ALARM: usize = 0x2d0;
const TIMER: usize = 0x2d4;
const HEADING: usize = 0x324;
/// `$gp+0x1b60` / `$gp+0x1b64` (`0x1b0b50`): the follow camera's distance and pivot height while it fights.
const CAM: (f32, f32) = (6.0, 3.0);
/// `0x3b449ba6`: their rate.
const CAM_RATE: f32 = f32::from_bits(0x3b44_9ba6);
/// `$gp+0x1480` (`0x1b0470`): the fireball's trail colour.
const TRAIL_RGB: u32 = 0x0080_8080;
/// `0x3bb60b61`: 1/180, the gravity of its jump and of its block.
const GRAVITY: f32 = f32::from_bits(0x3bb6_0b61);

fn ask_camera(ctx: &mut Ctx) {
    ctx.game.cam_tweaks.follow_dist = Some((CAM.0, CAM_RATE));
    ctx.game.cam_tweaks.follow_pivot = Some((CAM.1, CAM_RATE));
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

/// `0x3ecad0`: its group alarmed (`+0x2d0` of every member).
fn alarm(w: &mut MobyWorld, id: usize) {
    let Some(members) = usize::try_from(w.mobys[id].group).ok().and_then(|g| w.groups.get(g)).cloned() else { return };
    for i in members {
        if let Some(m) = w.mobys.get_mut(i) { m.set_pv_i(ALARM, 1); }
    }
}

/// A point in cuboid `i`'s frame to the world (`0x2e53d8` by its rows, plus its place), and back.
fn cuboid_world(ctx: &Ctx, i: i32, l: V3) -> Option<V3> {
    let c = usize::try_from(i).ok().and_then(|i| ctx.level.cuboids.get(i))?;
    let m = &c.matrix;
    Some(std::array::from_fn(|k| l[0] * m[0][k] + l[1] * m[1][k] + l[2] * m[2][k] + m[3][k]))
}
fn cuboid_local(ctx: &Ctx, i: i32, p: V3) -> Option<V3> {
    let c = usize::try_from(i).ok().and_then(|i| ctx.level.cuboids.get(i))?;
    let d = [p[0] - c.matrix[3][0], p[1] - c.matrix[3][1], p[2] - c.matrix[3][2]];
    let m = &c.inverse;
    Some(std::array::from_fn(|k| d[0] * m[0][k] + d[1] * m[1][k] + d[2] * m[2][k]))
}

/// `0x3ec600` (module docs). Returns whether it is to go.
fn checks(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    if w.mobys[id].state == 0 {
        let rng = &mut ctx.game.rng;
        let mirror = rng.randi(2) != 0;
        let weave = if rng.randi(2) != 0 { -15.0 } else { 15.0 } * 0.017_453_294;
        let m = &mut w.mobys[id];
        m.set_pv_b(0x48, 15);
        m.set_pv_b(0x4a, 9);
        if mirror { m.mode |= crate::moby::mode::MIRROR } else { m.mode &= !crate::moby::mode::MIRROR }
        m.set_pv_b(0x150, 6);
        m.set_pv_i(0x188, 3);
        m.set_pv_f(0x17c, 0.15);
        m.set_pv_f(0x170, 1.0);
        m.set_pv_f(0x174, 1.0);
        m.set_pv_f(0x180, m.pos[2] - 3.0);
        m.set_pv_i(MV + 0x38, m.pv_i(MV + 0x38) | 0x201);
        m.set_pv_f(MV + 4, 0.3);
        m.set_pv_f(MV, 1.0);
        m.set_pv_f(MV + 0x10, std::f32::consts::PI);
        m.set_pv_f(MV + 0xdc, weave);
        m.set_pv_s(MV + 0xe2, 30);
        m.set_pv_f(MV + 0x2c, GRAVITY);
        m.set_pv_s(MV + 0xe4, 90);
        let home = m.pos;
        m.set_pv_v(0x2e0, home);
        if ctx.level.variant_planes.first().is_some_and(|(o, _)| o[2] < m.pos[2]) {
            m.update_dist = 0xff;
            m.set_pv_i(0x320, 1);
        }
        m.pos[2] += 0.5;
        let a = [m.pos[0], m.pos[1], m.pos[2] + 0.5];
        let b = [m.pos[0], m.pos[1], f32::from_bits(0x3c23_d70a)];
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
            (7, 4, 0)
        };
        m.set_state(state, sub);
        set_seq(w, id, ctx, s, 0, 0);
        w.changed = true;
    }
    crate::shadow_volumes::probe_near(w, id, ctx, 24.0, 0x16);
    // A hit from its kind's fireballs is dropped, and one from a moby within 32 with the world between them.
    if let Some(h) = w.hit_message(id).filter(|h| h.tmpl.flags & 0x10000 != 0) {
        if let Some(s) = h.tmpl.source.and_then(|s| w.mobys.get(s)) {
            let (p, q) = (s.pos, w.mobys[id].pos);
            let drop = s.o_class == SHOT || (crate::hero::physics::dist(p, q) < 32.0 && {
                let (a, b) = ([p[0], p[1], p[2] + 0.5], [q[0], q[1], q[2] + 0.5]);
                ctx.coll.and_then(|c| crate::collision::coll_line_m(c, ctx.scene, a, b, crate::collision::QueryFlags(2), Some(id))).is_some()
            });
            if drop { w.clear_hit(id); }
        }
    }
    if ai::take_damage(w, id, ctx, false) != 0 {
        alarm(w, id);
        if w.mobys[id].pv_f(0x20) <= 0.0 {
            die(w, id, ctx);
            return true;
        }
    }
    let (range, area) = (w.mobys[id].pv_f(0x2b4), w.mobys[id].pv_i(0x2b0));
    let seen = ai::find_target(w, id, ctx, range, 0.0, area, None, false);
    w.mobys[id].set_pv_i(SEEN, seen as i32);
    false
}

/// `0x3ecdf8`: as the prowler's death — its bolts (3), the blast 0.75 up its frame, its sound 9, its pieces (mask
/// 0x7f8, along its knock at 0.05).
fn die(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    m.has_collision = false;
    m.mode = (m.mode & 0xefff) | 0x41;
    crate::moby::bolts::drop(w, id, 0, 3, ctx);
    let m = &w.mobys[id];
    let at = add(m.pos, scale(m.rows[2], 0.75));
    let b = crate::moby::blast::Blast { size: f32::from_bits(0x3f19_999a), bursts: [0x19, 0x14, 0x14], puffs: 0x14, streaks: 0x2d, light: true, shake: false };
    crate::moby::blast::burst(w, ctx, at, &b);
    ctx.host.play_sound(&w.mobys[id], 9, 0);
    let v = set_len(w.mobys[id].pv_v(0x1c0), f32::from_bits(0x3d4c_ccce));
    crate::moby::shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 6.0), 0x7f8, v, 0xff);
}

/// 0x0d2a (module docs).
pub fn update_0d2a(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    // `0x1b1d88` (the frame's count) past 1: only in the chunk it lives in.
    if 1 < ctx.game.frame && ctx.game.variant != w.mobys[id].pv_i(0x320) { return; }
    if checks(w, id, ctx) {
        w.delete(id);
        return;
    }
    let alert = |w: &mut MobyWorld, ctx: &mut Ctx| {
        w.mobys[id].set_state(2, 1);
        let t = ctx.game.rng.rand_range(0x244, 600);
        w.mobys[id].set_pv_i(TIMER, t);
    };
    match w.mobys[id].sub_state {
        0 => {
            let m = &mut w.mobys[id];
            if m.pv_i(SEEN) == 1 {
                m.set_state(5, 2);
            } else if m.pv_i(ALARM) != 0 {
                m.set_state(2, 1);
                m.set_pv_i(ALARM, 0);
                if m.pv_i(TIMER) < 300 {
                    let t = ctx.game.rng.rand_range(0x1e0, 600);
                    w.mobys[id].set_pv_i(TIMER, t);
                }
            }
        }
        1 => {
            let m = &mut w.mobys[id];
            if m.pv_i(SEEN) == 1 { m.set_state(5, 2) } else if m.pv_i(TIMER) == 0 { m.set_state(1, 0) }
        }
        2 => {
            ask_camera(ctx);
            if w.mobys[id].pv_i(SEEN) != 1 { alert(w, ctx); }
        }
        3 => {
            let res = ai::flight(w, id, ctx);
            if res & 1 != 0 {
                die(w, id, ctx);
                w.delete(id);
                return;
            }
            if res & 2 != 0 {
                if w.mobys[id].pv_i(SEEN) == 1 { w.mobys[id].set_state(5, 2) } else { alert(w, ctx) }
            }
        }
        _ => {}
    }
    states(w, id, ctx);
    // `0x3eca40`: its timers and its state's ticks.
    let m = &mut w.mobys[id];
    for o in [TIMER, 0x2d8] {
        let mut t = m.pv_i(o);
        crate::moby::countdown(&mut t);
        m.set_pv_i(o, t);
    }
    if m.state_ticks != -1 { m.state_ticks = m.state_ticks.wrapping_add(1); }
}

fn states(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let next = match w.mobys[id].state {
        1 => {
            if w.mobys[id].first() { set_seq(w, id, ctx, 2, 0, 0x14); }
            return;
        }
        2 => {
            if w.mobys[id].first() { set_seq(w, id, ctx, 5, 0, 0x14); }
            return;
        }
        3 => {
            if w.mobys[id].first() { set_seq(w, id, ctx, 0x14, 0, 0x14); }
            if ai::reached(w, id, ctx, 17.0) {
                fire(w, id, ctx);
                return;
            }
            if !wrapped(w, id, ctx) { return; }
            let facing = rot_dist(w.mobys[id].rot[2], yaw_to(w, id, ctx.hero.pos)) <= f32::from_bits(0x3eb2_b8c3);
            if ctx.game.rng.randi(2) == 0 && facing { 4 } else { 5 }
        }
        4 => {
            if w.mobys[id].first() { set_seq(w, id, ctx, 0xe, 0, 0x14); }
            if !wrapped(w, id, ctx) { return; }
            3
        }
        5 => {
            if w.mobys[id].first() {
                set_seq(w, id, ctx, 7, 0, 0xf);
                let off = ctx.game.rng.randf_signed(45.0, 100.0) * 0.017_453_294;
                let h = add_rot(yaw_to(w, id, ctx.hero.pos), off);
                w.mobys[id].set_pv_f(HEADING, h);
            }
            let yaw = w.mobys[id].pv_f(HEADING);
            let o = MV + 0x38;
            let keep = w.mobys[id].pv_i(o);
            w.mobys[id].set_pv_i(o, keep | 0x10);
            ai::move_step(w, id, ctx, yaw, [0.0; 3]);
            w.mobys[id].set_pv_i(o, keep);
            let t = w.mobys[id].pv_v(TARGET);
            let yaw = yaw_to(w, id, t);
            face(w, id, yaw);
            let m = &w.mobys[id];
            if m.pv_i(MV + 0x80) < 0x1f && m.state_ticks < 0x79 { return; }
            3
        }
        7 => {
            let m = &w.mobys[id];
            if group_alive(w, m.pv_i(0x2b8)) != 0 && m.pv_s(0x24) as f32 <= m.pv_f(0x20) { return; }
            8
        }
        8 => {
            if w.mobys[id].first() { set_seq(w, id, ctx, 1, 0, 2); }
            if !wrapped(w, id, ctx) { return; }
            let m = &mut w.mobys[id];
            if m.pv_i(0x2c0) == -1 || m.pv_i(0x2c4) == -1 {
                m.set_state(3, 2);
                return;
            }
            9
        }
        9 => {
            if w.mobys[id].first() { set_seq(w, id, ctx, 10, 0, 2); }
            ask_camera(ctx);
            let (from, to, pos) = (w.mobys[id].pv_i(0x2c0), w.mobys[id].pv_i(0x2c4), w.mobys[id].pos);
            let Some(l) = cuboid_local(ctx, from, pos) else { return };
            let Some(goal) = cuboid_world(ctx, from, [0.5, l[1], l[2]]) else { return };
            let yaw = yaw_to(w, id, goal);
            ai::move_step(w, id, ctx, yaw, goal);
            let pos = w.mobys[id].pos;
            if !ctx.level.in_cuboid(from, pos) { return; }
            let Some(l) = cuboid_local(ctx, from, pos) else { return };
            let x = ctx.game.rng.randf(0.0, 1.0);
            let Some(land) = cuboid_world(ctx, to, [x, l[1], l[2]]) else { return };
            ai::jump_start(&mut w.mobys[id], f32::from_bits(0x3dcc_ccce), GRAVITY, land);
            10
        }
        10 => {
            if w.mobys[id].first() { set_seq(w, id, ctx, 4, 0, 2); }
            ai::jump_step(w, id, ctx, 1.0, 8.0, 12.5);
            ask_camera(ctx);
            let m = &mut w.mobys[id];
            m.set_pv_f(0x180, m.pos[2] - 3.0);
            if !wrapped(w, id, ctx) { return; }
            let t = w.mobys[id].pv_v(TARGET);
            let s = if rot_dist(w.mobys[id].rot[2], yaw_to(w, id, t)) < 0.174_532_94 { 3 } else { 5 };
            w.mobys[id].set_state(s, 2);
            return;
        }
        _ => return,
    };
    w.mobys[id].set_state(next, -1);
}

/// The shot at frame 17: from its joint 4 toward Ratchet's aim point (the target block's point when it has no moby,
/// else that moby's middle), kept within 20° of its facing (`0x317cc8`), at 1/3 a tick.
fn fire(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let Some(muzzle) = ctx.host.joint(&w.mobys[id], 4) else { return };
    let m = &w.mobys[id];
    let t = match ai::link(m, TARGET + 0x10) {
        Some(t) if Some(t) == ctx.hero_moby => ctx.hero.aim_point,
        None => m.pv_v(TARGET),
        Some(t) => w.mobys.get(t).map_or(m.pv_v(TARGET), |o| [o.pos[0], o.pos[1], o.pos[2] + o.bsphere[3] * 0.000_976_562_5]),
    };
    let d = sub(t, muzzle);
    let fwd = m.rows[0];
    let u = set_len(d, 1.0);
    let cone = f32::from_bits(0x3eb2_b8c3);
    let v = if dot(set_len(fwd, 1.0), u) < cone.cos() { toward(fwd, u, cone) } else { d };
    let v = set_len(v, f32::from_bits(0x3eaa_aaab));
    let damage = w.mobys[id].pv_b(0x34) as f32;
    shot(w, id, ctx, damage, v, muzzle);
}

/// `0x317cc8(−a, &out, from, to)`: `from` turned `a` toward `to` (about their cross).
fn toward(from: V3, to: V3, a: f32) -> V3 {
    let f = set_len(from, 1.0);
    let n = crate::hero::physics::cross(f, to);
    let l = crate::hero::physics::len(n);
    if l < 1e-5 { return from; }
    let n = scale(n, 1.0 / l);
    let side = crate::hero::physics::cross(n, f);
    scale(add(scale(f, a.cos()), scale(side, a.sin())), crate::hero::physics::len(from))
}

/// `0x3cbba0(damage, maker, vel, at, 0, platform)`: a fireball 0x0b7f at `at` moving `vel` a tick, its maker's light,
/// a random spin of 12°..36° a tick, alpha 0x5d, its rows kept.
fn shot(w: &mut MobyWorld, maker: usize, ctx: &mut Ctx, damage: f32, vel: V3, at: V3) {
    if w.class(SHOT).is_none() { return; }
    let (light, ambient) = (w.mobys[maker].light, w.mobys[maker].ambient);
    let k = w.create(SHOT);
    let rng = &mut ctx.game.rng;
    let (a, spin, s) = (rng.rand_angle(), rng.randf(f32::from_bits(0x3e56_7752), f32::from_bits(0x3f20_d97d)), rng.randf(f32::from_bits(0x3c23_d70a), f32::from_bits(0x3c4c_cccd)));
    let m = &mut w.mobys[k];
    m.update_dist = 0xff;
    m.draw_dist = 0xff;
    m.drawn = 1;
    m.state = 0;
    m.light = light;
    m.ambient = ambient;
    m.pos = at;
    m.set_pv_v(0, vel);
    ai::set_link(m, 0x10, Some(maker));
    m.set_pv_f(0x14, damage);
    m.set_pv_f(0x18, a);
    m.set_pv_f(0x1c, spin);
    m.set_pv_f(0x20, 1.0);
    m.alpha = 0x5d;
    m.mode |= crate::moby::mode::KEEP_ROWS;
    m.update = Some(update_0b7f);
    m.mode &= !crate::moby::mode::NO_UPDATE;
    m.scale = s;
    w.changed = true;
}

/// 0x0b7f (`0x3cb618`, module docs).
pub fn update_0b7f(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 1 {
        let e = crate::moby::blast::Explosion { near: 9.0, size: 1.0, light: 15.0, embers: 10, flares: 0x10, sound: -1, debris: 9, lod: -1, ..Default::default() };
        let at = w.mobys[id].pos;
        crate::moby::blast::explode(w, ctx, Some(id), Some(at), &e);
        w.delete(id);
        return;
    }
    if w.mobys[id].state == 0 {
        let m = &mut w.mobys[id];
        let p = add_rot(m.pv_f(0x18), m.pv_f(0x1c));
        m.set_pv_f(0x18, p);
        m.rows = euler_rows([p * 0.5, p * 0.25, p]);
        let (from, vel) = (m.pos, m.pv_v(0));
        let to = add(from, vel);
        let maker = ai::link(m, 0x10).filter(|&o| w.mobys.get(o).is_some_and(|x| x.o_class == CLASS && x.state < 0x80));
        let met = ctx.coll.and_then(|c| {
            crate::collision::coll_line_m(c, ctx.scene, from, to, crate::collision::QueryFlags(0), Some(id))
                .or_else(|| crate::collision::coll_sphere_m(c, ctx.scene, from, f32::from_bits(0x3e99_999a), crate::collision::QueryFlags(0), Some(id)))
        });
        if met.is_some_and(|o| maker.is_none() || o.moby != maker) {
            let damage = w.mobys[id].pv_f(0x14);
            ai::attack_sphere(w, maker.unwrap_or(id), ctx, f32::from_bits(0x3e99_999a), damage, 0.0, from, 0x1_0001, 0, 1, 0);
            w.mobys[id].state = 1;
            w.changed = true;
            return;
        }
        w.mobys[id].pos = to;
        let k = w.mobys[id].pv_f(0x20);
        let back = scale(vel, -0.5);
        if let Some(sys) = ctx.parts.as_deref_mut() {
            let rng = &mut ctx.game.rng;
            let big = k * 210_000.0;
            let mut p = to;
            for (spin, alpha, lo, hi, jitter) in [(3u8, 0x4f, 0.2, 0.4, false), (7, 0x5f, 0.1, 0.3, true), (0xc, 0x7f, 0.2, 0.4, true)] {
                if jitter { for c in &mut p { *c += rng.randf_signed(0.0, 0.15); } }
                let s0 = rng.randf(1.0, 1.5) * big;
                let s1 = rng.randf(lo, hi) * big;
                crate::particles::sprite(sys, rng, s0, s1, 0.0, p, back, 5, TRAIL_RGB, spin, alpha, 0, 0);
            }
        }
    }
    let m = &mut w.mobys[id];
    let k = m.pv_f(0x20);
    if k < 1.0 { m.set_pv_f(0x20, k + 0.025); }
    let s = ctx.game.rng.randf(f32::from_bits(0x3c23_d70a), f32::from_bits(0x3c4c_cccd));
    let m = &mut w.mobys[id];
    m.scale = s * m.pv_f(0x20);
    w.changed = true;
    w.register_draw(id);
}

/// `0x3cbcc8` (module docs).
pub fn draw_0b7f(w: &MobyWorld, id: usize, rng: &mut crate::rng::Rng, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let m = &w.mobys[id];
    if 1 < m.state { return; }
    let size = m.scale * m.pv_f(0x20);
    let a = rng.rand_range(0x5f, 0x6f) as u32;
    let roll = rng.rand_angle();
    crate::fx::sprite(out, size * 45.0, 1.0, roll, m.pos, cam, 0xb, 0x0010_3080 | a << 24, true);
    let a = rng.rand_range(0x6f, 0x7f) as u32;
    let roll = rng.rand_angle();
    crate::fx::sprite(out, size * 22.5, 1.0, roll, m.pos, cam, 0xb, 0x003f_3f3f | a << 24, true);
}
