//! Endako's arena locks and their props (L03 class updates; no names in any reference, so they go by o_class).
//!
//! * 0x09b2 (`0x3d0dd0`, 12): the arena locks ([`update_09b2`]): set off by their trigger, Ratchet in a cuboid or a
//!   group all dead, they hook the barriers' group, then send waves of enemy groups (hooked: the enemies teleport in)
//!   until each is dead, hook the last groups and are done for good (the uid).
//! * 0x09b1 (`0x3d06c8`, 6): the cans six locks wait on: knocked, they tip over spilling 0x09d4 / 0x09d7 that 0x0ff3
//!   eats ([`update_09b1`], [`update_spill`]); a hard hit bursts them.
//! * 0x09a3 / 0x09a4 / 0x09a6 (`0x3cd840`, 6 / 5 / 13; level 21's 0x1345 is the same code): the barriers the locks
//!   hook ([`update_barrier`]): hooked they play their rise and a hidden copy of their collision climbs; let go, back.
//! * 0x0985 (`0x3cc9a0`, 5): the transporter pads to a cuboid; the one back at the start is hidden until the far one
//!   is used ([`update_0985`]).
//! * 0x0d40 (`0x4063d0`, 15): no update (`jr ra`).
//! * 0x02f6 / 0x0323 (`0x3a55d0`, the Swingshot's nodes and hook nodes on twelve levels): shown, or hidden until a
//!   mission is done or a moby is gone, then teleported in ([`update_swing_node`]).
//! * 0x09b6 (`0x3d1138`, 4): floating platforms rocking under Ratchet ([`update_09b6`], crate::moby::floater).
//! * 0x0979 (`0x3cbf40`, 8): fans turning at a random speed while drawn ([`update_0979`]).
//! * 0x0994 (`0x3cd378`, 12): turning at a random speed, a looped sound when `+0x68`[0] ([`update_0994`]).
//! * 0x09ff (`0x3d2708`, 5): shot by 0x0b1b's or hooked, they burst into twelve pieces ([`update_09ff`]).
//! * 0x0995 (`0x3cd460`, 8): flaps that tip over, carrying a moby, once that moby is hooked ([`update_0995`]).
//!
//! The small enemy 0x0ff3 the waves bring is crate::moby::classes::endako_a_enemies.
//!
//! Not ported: the Swingshot nodes' camera hint (`0x43c2d8` / `0x43c3c8`: a node ahead within 22 eases the camera's
//! look toward it, `0x1b9be0..`; the port's cameras have no such hint), 0x09ff's 50 particles of `0x33b0e0` (L00
//! `0x339b50`: a type the port lacks).

use crate::hero::physics::{add, add_rot, atan2, dist2, set_len, turn_ease, V3};
use crate::moby::{Ctx, MobyWorld};

/// `0x31dd00(group, 1)` (L02 `0x320b50`): every member of the group hooked.
fn hook(w: &mut MobyWorld, group: i32) { super::maktar::hook_group(w, group, true); }

/// `0x31dc20(group, −1)`: the group's members alive.
fn alive(w: &MobyWorld, group: i32) -> usize { super::props::group_alive(w, group) }

/// The uid done before (`0x21eec4[uid]`, or its save count).
fn uid_done(ctx: &Ctx, uid: i16) -> bool {
    ctx.game.uid_dead.get(uid.max(0) as usize).copied().unwrap_or(0) != 0 || ctx.game.uid_count(uid) != 0
}

/// 0x09b2 (`0x3d0dd0`): an arena lock. Its pvars: `[0]` a trigger moby (−1 none; set off in its states 2..4,
/// `0x3d0dc0`), `[4]` a cuboid Ratchet sets it off in, `[8]` a group whose death sets it off, `[0xc]` the barriers'
/// group (hooked at the start), `[0x10]` the group hooked at the second wave, `[0x14..0x20]` the waves' groups,
/// `[0x24..0x30]` the groups hooked at the end (each with its wave's), `[0x40]` the waves done. It counts its own ticks
/// (`+0x96`, every tick).
/// * **0**: done before (the uid dead or counted): `[0xc]` and `[0x10]` hooked, 5; else 1.
/// * **1**: set off: 2.
/// * **2**: after 31 ticks, 3, hooking `[0xc]` (no wave done) or `[0x10]`.
/// * **3**: after 31 ticks: no wave done, the waves hooked (their enemies come), 4; else the end groups hooked, the uid
///   gone (the mission + 2, `0x21e254` / `0x21eec4`), 5.
/// * **4**: every wave dead: one more done, 2.
pub fn update_09b2(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let p: [i32; 17] = std::array::from_fn(|k| w.mobys[id].pv_i(4 * k));
    let pv = |k: usize| p[k];
    let (state, ticks, uid) = (w.mobys[id].state, w.mobys[id].state_ticks as u16, w.mobys[id].uid);
    match state {
        0 => {
            if uid_done(ctx, uid) {
                let (a, b) = (pv(3), pv(4));
                hook(w, a);
                hook(w, b);
                w.mobys[id].set_state(5, -1);
            } else {
                w.mobys[id].set_state(1, -1);
            }
        }
        1 => {
            let trigger = usize::try_from(pv(0)).ok().and_then(|t| w.mobys.get(t)).is_some_and(|t| (2..5).contains(&t.state));
            let set_off = trigger || (pv(1) != -1 && ctx.level.in_cuboid(pv(1), ctx.hero.pos)) || (pv(2) != -1 && alive(w, pv(2)) == 0);
            if set_off { w.mobys[id].set_state(2, -1); }
        }
        2 => {
            if 0x1f <= ticks {
                w.mobys[id].set_state(3, -1);
                let g = if pv(0x10) == 0 { pv(3) } else { pv(4) };
                hook(w, g);
            }
        }
        3 => {
            if 0x1f <= ticks {
                if pv(0x10) == 0 {
                    for k in 5..9 {
                        if pv(k) != -1 { hook(w, pv(k)); }
                    }
                    w.mobys[id].set_state(4, -1);
                } else {
                    for k in 0..4 {
                        if pv(5 + k) != -1 { hook(w, pv(9 + k)); }
                    }
                    let (uid, mission) = (w.mobys[id].uid, w.mobys[id].mission);
                    ctx.game.uid_gone(uid, mission);
                    w.mobys[id].set_state(5, -1);
                }
            }
        }
        4 => {
            if (5..9).all(|k| pv(k) == -1 || alive(w, pv(k)) == 0) {
                let n = pv(0x10);
                w.mobys[id].set_pv_i(0x40, n + 1);
                w.mobys[id].set_state(2, -1);
            }
        }
        _ => {}
    }
    let m = &mut w.mobys[id];
    m.state_ticks = m.state_ticks.wrapping_add(1);
}

/// The barriers' climb: 3.5 (× their scale over the class's) over 60 ticks.
const CLIMB: f32 = 0.058_333_337;

/// `0x3cda80(m, _, vel, ticks)`: a copy of the barrier (`0x32be48`: updated and drawn at any distance, its parent, place,
/// angles and light), hidden, with its class's collision, moving `vel` a tick for `ticks` ticks (state 5). The place
/// the callers pass is not read.
fn climber(w: &mut MobyWorld, id: usize, vel: V3, ticks: i32) {
    let src = &w.mobys[id];
    let (class, pos, rot, light, ambient) = (src.o_class, src.pos, src.rot, src.light, src.ambient);
    let coll = w.class(class).is_some_and(|c| c.has_collision);
    let c = w.create(class);
    let m = &mut w.mobys[c];
    (m.update_dist, m.draw_dist, m.parent, m.pos, m.rot, m.light, m.ambient) = (0xff, 0xff, Some(id), pos, rot, light, ambient);
    m.rows = crate::hero::physics::euler_rows(rot);
    m.mode &= !7;
    m.drawn = 0;
    m.mode |= 1;
    m.has_collision = coll;
    m.set_pv_v(0, vel);
    m.set_pv_i(0x10, ticks);
    m.set_state(5, -1);
    w.changed = true;
}

/// 0x09a3 / 0x09a4 / 0x09a6 (`0x3cd840`): a barrier.
/// * **0**: 1.
/// * **1**: hooked: 2, its sequence 1 from frame 0, no collision, a climber (0, 0, [`CLIMB`]·k) for 60 ticks.
/// * **2**: at its sequence's end, 3 and sequence 2 (blend 1).
/// * **3**: no longer hooked: 4, sequence 3, a climber again, its collision back.
/// * **4**: at its sequence's end, 1 and sequence 0 (blend 1).
/// * **5** (a climber): moves its velocity a tick (`+0x68`[0..0xc]) until its ticks (`[0x10]`) run out, then goes.
pub fn update_barrier(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use super::props::{seq, wrapped};
    let k = {
        let m = &w.mobys[id];
        w.class(m.o_class).map_or(1.0, |c| m.scale / c.scale)
    };
    match w.mobys[id].state {
        0 => w.mobys[id].set_state(1, -1),
        1 => {
            if w.mobys[id].once & 4 == 0 { return; }
            w.mobys[id].set_state(2, -1);
            ctx.host.set_anim(&w.mobys[id], 1, 0, 0);
            w.mobys[id].has_collision = false;
            w.changed = true;
            climber(w, id, [0.0, 0.0, k * CLIMB], 0x3c);
        }
        2 => {
            if !wrapped(w, id, ctx) { return; }
            w.mobys[id].set_state(3, -1);
            if seq(w, id, ctx) != 2 { ctx.host.set_anim(&w.mobys[id], 2, 0, 1); }
        }
        3 => {
            if w.mobys[id].once & 4 != 0 { return; }
            w.mobys[id].set_state(4, -1);
            ctx.host.set_anim(&w.mobys[id], 3, 0, 0);
            climber(w, id, [0.0, 0.0, k * CLIMB], 0x3c);
            let coll = w.class(w.mobys[id].o_class).is_some_and(|c| c.has_collision);
            w.mobys[id].has_collision = coll;
            w.changed = true;
        }
        4 => {
            if !wrapped(w, id, ctx) { return; }
            w.mobys[id].set_state(1, -1);
            if seq(w, id, ctx) != 0 { ctx.host.set_anim(&w.mobys[id], 0, 0, 1); }
        }
        5 => {
            let m = &mut w.mobys[id];
            let v = m.pv_v(0);
            m.pos = add(m.pos, v);
            let mut t = m.pv_i(0x10);
            let out = crate::moby::countdown(&mut t);
            m.set_pv_i(0x10, t);
            w.changed = true;
            if out { w.delete(id); }
        }
        _ => {}
    }
}

/// 0x0d40 (`0x4063d0`): `jr ra`.
pub fn update_0d40(_w: &mut MobyWorld, _id: usize, _ctx: &mut Ctx) {}

/// The class of the hook nodes.
pub const HOOK_NODE: i16 = 0x0323;

/// 0x02f6 / 0x0323 (`0x3a55d0`): a Swingshot node (pvars as crate::hero::swing reads them; `[0x30]` a mission to wait
/// for, `[0x38]` the hook node's turn speed, `[0x3c]` a moby to wait for).
/// * **0**: a target (mode 0x1000); with nothing to wait for (`[0x30]` −1, and `[0x3c]` −1 or gone) its colour 0xc0
///   (`0x313300`), 1; else updated at any distance, hidden (mode 0x41), 2.
/// * **1**: a hook node turns to face Ratchet (`0x31f230`: 0.00175 a tick², at most 0.21 a tick) and, every other frame
///   while drawn, sheds a glint from a side (±1 along its left row) drifting across it over 30 ticks (crate::particles::glint
///   `0x33b5e0`: 1.25, 0x60404080).
/// * **2**: the mission's byte set, or the moby gone: shown, teleported in (crate::moby::classes::teleport), `[0x30]`
///   −1, back to 0 (the state set directly).
pub fn update_swing_node(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let gone = |w: &MobyWorld, i: i32| usize::try_from(i).ok().and_then(|i| w.mobys.get(i)).is_none_or(|o| o.state == 0xfe || o.state == 0xfd);
    match w.mobys[id].state {
        0 => {
            w.mobys[id].mode |= 0x1000;
            let (mission, link) = (w.mobys[id].pv_i(0x30), w.mobys[id].pv_i(0x3c));
            if mission == -1 && (link == -1 || gone(w, link)) {
                let m = &mut w.mobys[id];
                m.ambient = [0xc0; 3];
                m.state = 1;
            } else {
                let m = &mut w.mobys[id];
                m.update_dist = 0xff;
                m.state = 2;
                m.mode |= 0x1041;
            }
        }
        1 => {
            let m = &w.mobys[id];
            if m.o_class != HOOK_NODE { return; }
            let hero = ctx.hero.pos;
            let t = atan2(hero[0] - m.pos[0], hero[1] - m.pos[1]);
            let m = &mut w.mobys[id];
            let (mut a, mut v) = (m.rot[2], m.pv_f(0x38));
            turn_ease(t, 0.001_745_329_3, 0.001_745_329_3, 0.209_439_51, &mut a, &mut v, 0);
            m.rot[2] = a;
            m.set_pv_f(0x38, v);
            if ctx.game.frame & 1 != 0 && m.drawn != 0 {
                let side = if ctx.game.frame & 4 != 0 { -1.0 } else { 1.0 };
                let off = set_len(m.rows[1], side);
                let vel = set_len(off, -2.0 / 30.0);
                let at = add(off, m.pos);
                let rot = ctx.game.rng.randi(0xff) as u8;
                let hero = ctx.hero.pos;
                if let Some(sys) = ctx.parts.as_deref_mut() { crate::particles::glint(sys, 1.25, at, vel, 0x6040_4080, 30, rot, false, hero); }
            }
        }
        2 => {
            let m = &w.mobys[id];
            let level = ctx.game.level.max(0) as usize;
            let mission = m.pv_i(0x30);
            let done = ctx.game.missions.get(level * 0x10 + mission.max(0) as usize).is_some_and(|&b| b != 0);
            if !done && (m.pv_i(0x3c) == -1 || !gone(w, m.pv_i(0x3c))) { return; }
            let m = &mut w.mobys[id];
            m.mode &= 0xffbe;
            let (pos, rot) = (m.pos, m.rot);
            super::teleport::start(w, id, pos, rot, &[]);
            let m = &mut w.mobys[id];
            m.set_pv_i(0x30, -1);
            m.state = 0;
        }
        _ => {}
    }
}

/// 0x09b6 (`0x3d1138`): `+0x68`[0x5c] |= 1, the floating platforms' rocking at `+0x60` with the default tunables
/// (`0x32d438`, the table `0x1aa320`), 1; then it rocks (`0x32d750`).
pub fn update_09b6(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use crate::moby::floater;
    const BLOCK: usize = 0x60;
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            let f = m.pv_i(0x5c);
            m.set_pv_i(0x5c, f | 1);
            floater::init(w, id, ctx, BLOCK, &floater::Params::default());
            w.mobys[id].set_state(1, -1);
        }
        1 => floater::step(w, id, ctx, BLOCK),
        _ => {}
    }
}

/// 0x0979 (`0x3cbf40`), while drawn: at its start its timer byte 5..10 (+15 one time in two), 1; then it turns
/// 18°/s × (timer − 15) about z.
pub fn update_0979(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].drawn == 0 { return; }
    match w.mobys[id].state {
        0 => {
            let mut t = ctx.game.rng.rand_range(5, 10) as u8;
            if ctx.game.rng.randi(2) != 0 { t += 0xf; }
            let m = &mut w.mobys[id];
            m.timer = t;
            m.set_state(1, -1);
        }
        1 => {
            let m = &mut w.mobys[id];
            m.rot[2] = add_rot(m.rot[2], 18.0 * 0.000_290_888_25 * (m.timer as i32 - 0xf) as f32);
        }
        _ => {}
    }
}

/// 0x0994 (`0x3cd378`): at its start 1, its turn `[4]` = 60..90°/s at random (`0x31ba98`: either way), no sound
/// (`[8]` −1); then with `[0]` its class sound 0 looped (again whenever it stops) and the turn about z.
pub fn update_0994(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            w.mobys[id].set_state(1, -1);
            let s = ctx.game.rng.randf_signed(60.0, 90.0);
            let m = &mut w.mobys[id];
            m.set_pv_i(8, -1);
            m.set_pv_f(4, s * 0.000_290_888_25);
        }
        1 => {
            if w.mobys[id].pv_i(0) != 0 {
                let h = w.mobys[id].pv_i(8);
                if !(h != -1 && ctx.host.sound_playing(h)) {
                    let h = ctx.host.play_sound(&w.mobys[id], 0, 4);
                    w.mobys[id].set_pv_i(8, h);
                }
            }
            let m = &mut w.mobys[id];
            m.rot[2] = add_rot(m.rot[2], m.pv_f(4));
        }
        _ => {}
    }
}

/// 0x09ff (`0x3d2708`): `+0x68`[0x5c] |= 8, 1; then a hit (flags 0x10000) by a 0x0b1b or a hook makes it burst
/// ([`burst_09ff`]), its class sound 0, and go; any other hit is dropped.
pub fn update_09ff(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            let f = m.pv_i(0x5c);
            m.set_pv_i(0x5c, f | 8);
            m.set_state(1, -1);
        }
        1 => {
            let hit = w.hit_message(id).filter(|h| h.tmpl.flags & 0x10000 != 0);
            let by = hit.and_then(|h| h.tmpl.source).and_then(|s| w.mobys.get(s)).is_some_and(|s| s.o_class == 0x0b1b);
            if w.mobys[id].once & 4 == 0 && !by {
                w.clear_hit(id);
                return;
            }
            burst_09ff(w, id, ctx);
            ctx.host.play_sound(&w.mobys[id], 0, 0);
            w.delete(id);
        }
        _ => {}
    }
}

/// `0x3d27e8`: twelve pieces (0..11, `0x2dbdb0` mode 2, crate::moby::shards): each at a random heading 0..6 a second
/// across and 5..20 up, plus 6 a second back along its facing; spin 30..60, gravity 15..25; their floor loss 0.5 and
/// least speed 4 a second.
fn burst_09ff(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (o_class, pos, rot, rows) = { let m = &w.mobys[id]; (m.o_class, m.pos, m.rot, m.rows) };
    let back = set_len(rows[0], -(6.0 * 0.016_666_668));
    for k in 0..12u8 {
        let rng = &mut ctx.game.rng;
        let a = rng.rand_angle();
        let s = rng.randf(0.0, 6.0) * 0.016_666_668;
        let z = rng.randf(5.0, 20.0) * 0.016_666_668;
        let vel = add([a.cos() * s, a.sin() * s, z], back);
        let spin = rng.randf(30.0, 60.0);
        let gravity = rng.randf(15.0, 25.0);
        if let Some(p) = crate::moby::shards::spawn_shard(w, ctx, o_class, Some(id), pos, rot, Some(rows), spin, gravity, k, vel, 0xff) {
            let m = &mut w.mobys[p];
            m.set_pv_f(0x38, 0.5);
            m.set_pv_f(0x3c, 4.0 * 0.016_666_668);
        }
    }
}

/// `0x2f02d8(out, l, rows)` (L00 `0x2ef318`): a local vector into the world by the rows.
pub(crate) fn to_world(l: V3, r: &[V3; 3]) -> V3 { std::array::from_fn(|k| l[0] * r[0][k] + l[1] * r[1][k] + l[2] * r[2][k]) }

/// The spills' classes (`0x3d06c8`: one in two each).
const SPILLS: [i16; 2] = [0x09d4, 0x09d7];
/// `$gp+0x1440..`: where a spill leaves the tipped can, in its frame.
const MOUTH: [V3; 4] = [
    [-0.362_000_47, -0.26, 1.241],
    [-0.362_000_47, 0.26, 1.241],
    [0.362_000_47, -0.26, 1.241],
    [0.362_000_47, 0.26, 1.241],
];

/// `0x2dcc78(gravity, m, 0, −1, vel, spin, sound)` (L00 `0x2dbff8` mode 0): the moby itself made a tumbling body (the
/// shards' block in its pvars, crate::moby::shards): no collision, its rows kept, `vel`, the spin quaternion `spin`,
/// floor loss 0.45, least 1/30, gravity / 3600, 180 ticks; its pivot and radius its bounding sphere in its frame.
fn become_body(w: &mut MobyWorld, id: usize, gravity: f32, vel: V3, spin: [f32; 4], sound: u8) {
    let n = w.class(w.mobys[id].o_class).map_or(0, |c| c.coll_points.len());
    let m = &mut w.mobys[id];
    if m.pvars.len() < 0x80 { m.pvars.resize(0x80, 0); }
    m.has_collision = false;
    m.mode |= crate::moby::mode::KEEP_ROWS;
    m.set_pv_v(0, vel);
    m.set_pv_q(0x10, spin);
    m.set_pv_f(0x38, f32::from_bits(0x3ee6_6666));
    m.set_pv_f(0x3c, f32::from_bits(0x3d08_8889));
    m.set_pv_f(0x40, 0.5);
    m.set_pv_f(0x44, f32::from_bits(0x3c56_7752));
    m.set_pv_f(0x48, gravity * 0.000_277_777_78);
    m.set_pv_f(0x4c, f32::from_bits(0x3ae4_c38a));
    m.set_pv_s(0x34, 0xb4);
    m.pvars[0x5e] = sound;
    m.pvars[0x5d] = 0;
    m.pvars[0x58] = n as u8;
    m.pvars[0x5c] = 0;
    m.set_pv_i(0x30, 0);
    let s = m.bsphere;
    let d = [s[0] * 0.000_976_562_5 - m.pos[0], s[1] * 0.000_976_562_5 - m.pos[1], s[2] * 0.000_976_562_5 - m.pos[2]];
    let r = m.rows;
    let local = [0, 1, 2].map(|k| d[0] * r[k][0] + d[1] * r[k][1] + d[2] * r[k][2]);
    m.set_pv_v(0x20, local);
    m.set_pv_f(0x2c, s[3] * 0.000_976_562_5);
}

/// `0x2dca30(spin, 20, m, 0, class, −1, at, angles, vel, 0xff)` and the spill's update ([`update_spill`]).
fn spill(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, at: V3, vel: V3) {
    let class = SPILLS[(ctx.game.rng.randi(2) == 0) as usize];
    spill_of(w, id, ctx, class, at, vel);
}

fn spill_of(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, class: i16, at: V3, vel: V3) -> bool {
    let rng = &mut ctx.game.rng;
    let rot = [rng.rand_angle(), rng.rand_angle(), rng.rand_angle()];
    let spin = rng.randf_signed(45.0, 180.0);
    match crate::moby::shards::spawn_whole(w, ctx, class, Some(id), at, rot, spin * 0.000_290_888_25, 20.0, vel, 0xff) {
        Some(s) => {
            w.mobys[s].update = Some(update_spill);
            true
        }
        None => false,
    }
}

/// 0x09b1 (`0x3d06c8`, 6): a can the arena locks (0x09b2) wait on. Every tick a hit's record without flags 0x10000
/// is dropped.
/// * **0**: 1.
/// * **1**: hit (0x10000), or Ratchet in state 0x10 pushing against it: its help text (`+0x68`[0x84] + 0x2de8, 180
///   ticks) the first time (the uid not dead, its count 0), its uid counted, its sound 0; it tips over (made a body,
///   gravity 15: 4/60 across along the hit's way or Ratchet's facing, 6/60 up, turning 150°/s about the way × up), its
///   collision back, 2.
/// * **2**: tipped between 0.1 and 0.9 (its up row on up), one tick in four (by its index): a spill out of its mouth
///   (`$gp+0x1440`, one of four) flung 0..2 across and 4..8 up a second in its frame, its sound 1; at rest, 3.
/// * any but 4: a hit of 3 or more bursts it — hidden, no collision, not a target, its pieces (3..5 across, 5..10 up,
///   mask 0x1ff) and twenty spills from 0.5 up (0..0.5 out, 0..2 across, 5..12 up a second), 4.
///
/// Not ported: the tick its spills come on is the game's moby address `>> 8 & 3`; the index stands for it.
pub fn update_09b1(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let hit = w.hit_message(id);
    let hit = match hit {
        Some(h) if h.tmpl.flags & 0x10000 != 0 => Some(h),
        Some(_) => {
            w.clear_hit(id);
            None
        }
        None => None,
    };
    match w.mobys[id].state {
        0 => w.mobys[id].set_state(1, -1),
        1 => {
            let pushed = ctx.hero.state == 0x10 && ctx.hero.push_moby == Some(id);
            if hit.is_some() || pushed {
                let (uid, msg) = (w.mobys[id].uid, w.mobys[id].pv_i(0x84));
                if 0 <= msg && ctx.game.uid_dead.get(uid.max(0) as usize).copied().unwrap_or(0) == 0 && ctx.game.uid_count(uid) == 0 {
                    ctx.game.messages.push(crate::msgbox::Request::Text { text: msg + 0x2de8, ticks: Some(0xb4) });
                }
                ctx.game.count_uid(uid);
                ctx.host.play_sound(&w.mobys[id], 0, 0);
                let up = [0.0, 0.0, 1.0];
                let way = match hit {
                    Some(h) => [h.tmpl.dir[0], h.tmpl.dir[1], h.tmpl.dir[2]],
                    None => ctx.hero.rows[0],
                };
                let mut vel = crate::hero::physics::set_len2(way, 4.0 * 0.016_666_668);
                vel[2] = 6.0 * 0.016_666_668;
                let axis = crate::hero::physics::cross(vel, up);
                let s = (150.0 * 0.000_290_888_25f32).sin() * 0.5;
                let a = set_len(axis, s);
                become_body(w, id, 15.0, vel, [a[0], a[1], a[2], (1.0 - s * s).sqrt()], 0xff);
                let coll = w.class(w.mobys[id].o_class).is_some_and(|c| c.has_collision);
                w.mobys[id].has_collision = coll;
                w.mobys[id].set_state(2, -1);
                w.changed = true;
            }
        }
        2 => {
            let m = &w.mobys[id];
            let t = m.rows[2][2];
            if ctx.game.frame & 3 == (id & 3) as i32 && t < 0.9 && 0.1 < t {
                let k = ctx.game.rng.randi(4) as usize;
                let (rows, pos) = (m.rows, m.pos);
                let at = add(to_world(MOUTH[k], &rows), pos);
                let class = SPILLS[(ctx.game.rng.randi(2) == 0) as usize];
                let rng = &mut ctx.game.rng;
                let v = [rng.randf_signed(0.0, 2.0), rng.randf_signed(0.0, 2.0), rng.randf(4.0, 8.0)];
                let v = to_world(set_len(v, crate::hero::physics::len(v) * 0.016_666_668), &rows);
                if spill_of(w, id, ctx, class, at, v) { ctx.host.play_sound(&w.mobys[id], 1, 0); }
            }
            if crate::moby::shards::step(w, id, ctx) { w.mobys[id].set_state(3, -1); }
        }
        4 => return,
        _ => {}
    }
    if hit.is_some_and(|h| 3.0 <= h.tmpl.damage) {
        let m = &mut w.mobys[id];
        m.set_state(4, -1);
        m.has_collision = false;
        m.mode = (m.mode & 0xefff) | 0x41;
        m.rot = crate::moby::attach::angles_of(&m.rows);
        crate::moby::shards::break_into(w, id, ctx, (3.0, 5.0), (5.0, 10.0), 0x1ff, [0.0; 3], 0xff);
        let m = &w.mobys[id];
        let base = add(to_world([0.0, 0.0, 0.5], &m.rows), m.pos);
        for _ in 0..20 {
            let rng = &mut ctx.game.rng;
            let at = add(rng.rand_vec(0.0, 0.5), base);
            let v = [rng.randf_signed(0.0, 2.0), rng.randf_signed(0.0, 2.0), rng.randf(5.0, 12.0)];
            let v = set_len(v, crate::hero::physics::len(v) * 0.016_666_668);
            spill(w, id, ctx, at, v);
        }
        w.changed = true;
    }
}

/// The spills 0x09d4 / 0x09d7 (`0x3d1908`, the update 0x09b1 gives them).
/// * **0**: tumbling (the shards' body); at rest 1, mode |= 0x2000.
/// * **1**: one tick in sixteen (by its index), a 0x0ff3 or 0x0ffd within 0.2 takes it (3); 600 ticks in and out of
///   view, 2.
/// * **2** / **3**: it fades 2 / 8 an alpha a tick and goes.
///
/// Not ported: taken by the beam of 0x0ffd (its end `$gp+0x1550` within 0.3; that class is not ported).
pub fn update_spill(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            if crate::moby::shards::step(w, id, ctx) {
                w.mobys[id].set_state(1, -1);
                w.mobys[id].mode |= 0x2000;
            }
        }
        1 => {
            if ctx.game.frame & 0xf == (id & 0xf) as i32 {
                let pos = w.mobys[id].pos;
                let near = ctx.scene.map_or(Vec::new(), |sc| crate::collision::coll_sphere_mobys(sc, pos, 0.2, crate::collision::QueryFlags(0), Some(id)));
                if near.iter().any(|&o| w.mobys.get(o).is_some_and(|o| o.o_class == 0x0ff3 || o.o_class == 0x0ffd)) { w.mobys[id].set_state(3, -1); }
            }
            let m = &mut w.mobys[id];
            let t = m.state_ticks;
            m.state_ticks = t.wrapping_add(1);
            if 600 < t as u16 && m.drawn == 0 { m.set_state(2, -1); }
        }
        2 | 3 => {
            let m = &mut w.mobys[id];
            let step = if m.state == 2 { 2 } else { 8 };
            if m.alpha < step { w.delete(id); } else { m.alpha -= step; }
        }
        _ => {}
    }
}

/// `0x1395c4`: the flag byte the far pads set (`+0x68`[0xc]) and the hidden pads wait for (`[8]`).
const PAD_FLAG: usize = 0x1395c4 - 0x1395b8;

/// 0x0985 (`0x3cc9a0`, 5): the transporter pads. Its pvars: `[0]` the cuboid it sends Ratchet to, `[8]` hidden until
/// [`PAD_FLAG`], `[0xc]` setting it on arrival, `[0x10]` a placed camera view put back on (−1 none), `[0x18]` the
/// swirl's fade-in, `[0x1c]` the send's phase, `[0x20]` its colour (its ambient), `[0x24..]` / `[0x34..]` its rows'
/// growth (velocities, scales), `[0x30]` the swirl's phase.
/// * **0**: its colour kept; waiting for the flag, hidden without collision and updated at any distance (5); else 1.
/// * **1**: Ratchet on it (his ground moby, grounded, within 2 across) gets its prompt (owner 8, text 0x12fb); △
///   with the prompt his: its sound 0, he walks to its middle facing a quarter turn left of it (`0x2d4440`), a 2.5
///   sphere pushes things off it (100, 0x10000), 2. Hooked off it: hidden (5).
/// * **2**: he is there (state 100): he stands still (0x6f), 3; after 360 ticks, idle again (`0x2c8e30(0)`), 1.
/// * **3**: the phase eases to 90° (π/2 by 6°/s); at tick 48 Ratchet goes to the cuboid's ground, facing its angles
///   in state 0 (`0x2b9e68`), the view `[0x10]` back on (`0x42b9f0`), 4.
/// * **4**: the flag set with `[0xc]` (its first tick); the phase eases back to 0, then its update distance back, 1.
/// * **5**: shown (6) once the flag is set (`[8]`), or, without `[8]`, once no longer hooked: its rows from 0.02,
///   a target with its collision, its sound 1 (6's first tick).
/// * **6**: each row grows (`0x31ed78`: 6/3600, 6/3600, at most 3/60) to 1, the next once the last is past half;
///   its colour from 0x80ff8000 to its own by the last; all grown, 1.
/// * States 0..4: the swirl's phase turns 100°/s, its fade-in up by 1/30 to 1, and one tick in two a glint rises
///   from a point 0.85 out (0.1 up; 0.3..0.2 big, 0x80808000..0x80804000, 2..3 up over 60..180 ticks).
///
/// Not ported: Ratchet's stretch while sent (`0x18be40..` / `0x18c320`: x and y `1 − sin`, z `2 − cos` of the phase),
/// the hero byte `0x18c0d7` held while it sends, the camera's reset on arrival (`0x2bf080`'s last flag: `0x2dade0`), his stop when walking off it past 180 ticks (`0x189f10`), the
/// prompt's other conditions (`0x18b05a`, `0x18c2ec`, his group 0x14), the swirl's draw (`0x3cd248`, the first draw
/// list).
pub fn update_0985(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use crate::hero::physics::{approach, ease};
    let flag = |ctx: &Ctx| ctx.game.flags.get(PAD_FLAG).copied().unwrap_or(0) != 0;
    let hide = |m: &mut crate::moby::Moby| {
        m.has_collision = false;
        m.mode = (m.mode & 0xefff) | 0x41;
    };
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            let a = m.ambient;
            m.set_pv_i(0x20, (0x8000_0000u32 | (a[2] as u32) << 16 | (a[1] as u32) << 8 | a[0] as u32) as i32);
            if m.pv_i(8) != 0 && !flag(ctx) {
                hide(m);
                m.set_state(5, -1);
                m.update_dist = 0xff;
            } else {
                m.set_state(1, -1);
            }
        }
        1 => {
            let m = &w.mobys[id];
            let on = ctx.hero.ground_moby == Some(id) && ctx.hero.grounded_ticks != 0 && dist2(m.pos, ctx.hero.pos) < 2.0;
            if !on {
                if w.mobys[id].once & 4 != 0 {
                    let m = &mut w.mobys[id];
                    m.set_state(5, -1);
                    hide(m);
                }
            } else {
                ctx.game.prompt.request(8, 0x12fb, 1);
                if ctx.game.prompt.owner == 8 && ctx.hero.items.pressed & crate::pad::button::TRIANGLE != 0 {
                    ctx.host.play_sound(&w.mobys[id], 0, 0);
                    let (yaw, at) = (add_rot(w.mobys[id].rot[2], std::f32::consts::FRAC_PI_2), w.mobys[id].pos);
                    ctx.hero.script_walk_to(yaw, at, false);
                    crate::moby::ai::attack_sphere(w, id, ctx, 2.5, 100.0, 1.0, at, 0x10000, 0, 1, 0);
                    w.mobys[id].set_state(2, -1);
                }
            }
        }
        2 => {
            if ctx.hero.state == 100 {
                w.mobys[id].set_state(3, -1);
                ctx.hero.request_state(0x6f, true);
            } else {
                let m = &mut w.mobys[id];
                let t = m.state_ticks as u16;
                m.state_ticks = m.state_ticks.wrapping_add(1);
                if 0xb4 < t && 0x169 <= t + 1 {
                    m.set_state(1, -1);
                    match ctx.hero.mode {
                        0 => ctx.hero.request_state(0, true),
                        1 => ctx.hero.request_state(0x3f, true),
                        _ => {}
                    }
                }
            }
        }
        3 => {
            let m = &mut w.mobys[id];
            if m.first() {
                m.set_pv_f(0x1c, 0.0);
                m.update_dist = 0xff;
            }
            m.state_ticks = m.state_ticks.wrapping_add(1);
            let mut ph = m.pv_f(0x1c);
            approach(f32::from_bits(0x3fc9_0fdc), 0.104_719_76, &mut ph);
            m.set_pv_f(0x1c, ph);
            if m.state_ticks == 0x30 {
                let cuboid = m.pv_i(0);
                if let Some((p, r)) = ctx.level.cuboid_place(cuboid) {
                    let z = crate::moby::ai::ground_z(ctx, 0.5, p, 0);
                    // `0x2bf080(p, r, 0, 1)`: placed, in state 0.
                    ctx.hero.teleport([p[0], p[1], z], r);
                    ctx.hero.request_state(0, true);
                }
                let view = w.mobys[id].pv_i(0x10);
                if view != -1 { ctx.game.cameras.reset_placed(view); }
                w.mobys[id].set_state(4, -1);
            }
        }
        4 => {
            let m = &mut w.mobys[id];
            if m.first() && m.pv_i(0xc) != 0 {
                if let Some(f) = ctx.game.flags.get_mut(PAD_FLAG) { *f = 1; }
            }
            let mut ph = m.pv_f(0x1c);
            approach(0.0, 0.104_719_76, &mut ph);
            m.set_pv_f(0x1c, ph);
            if ph <= 0.0 {
                m.update_dist = m.draw_dist as u8;
                m.set_state(1, -1);
            }
        }
        5 => {
            let m = &w.mobys[id];
            let show = if m.pv_i(8) == 0 { m.once & 4 == 0 } else { flag(ctx) };
            if show {
                let solid = w.class(w.mobys[id].o_class).is_some_and(|c| c.has_collision);
                let m = &mut w.mobys[id];
                m.set_state(6, -1);
                m.mode |= crate::moby::mode::KEEP_ROWS;
                for k in 0..3 {
                    m.set_pv_f(0x18, 0.0);
                    m.set_pv_f(0x34 + 4 * k, 0.02);
                    m.rows[k] = set_len(m.rows[k], 0.02);
                }
                m.mode = (m.mode & 0xffbe) | 0x1000;
                m.has_collision = solid;
                w.changed = true;
            }
        }
        6 => {
            if w.mobys[id].first() { ctx.host.play_sound(&w.mobys[id], 1, 0); }
            let (accel, most) = (6.0 * 0.000_277_777_78, 3.0 * 0.016_666_668);
            let m = &mut w.mobys[id];
            for k in 0..3 {
                let s = m.pv_f(0x34 + 4 * k);
                if s < 1.0 && (k == 0 || 0.5 < m.pv_f(0x34 + 4 * (k - 1))) {
                    let (mut x, mut v) = (s, m.pv_f(0x24 + 4 * k));
                    ease(1.0, accel, accel, most, &mut x, &mut v);
                    x = x.min(1.0);
                    m.set_pv_f(0x34 + 4 * k, x);
                    m.set_pv_f(0x24 + 4 * k, v);
                    m.rows[k] = set_len(m.rows[k], x);
                }
            }
            if (0..3).all(|k| 1.0 <= m.pv_f(0x34 + 4 * k)) { m.set_state(1, -1); }
            let c = crate::hud::lerp_rgba(m.pv_f(0x3c), 0x80ff_8000, m.pv_i(0x20) as u32);
            m.ambient = [c as u8, (c >> 8) as u8, (c >> 16) as u8];
        }
        _ => {}
    }
    if w.mobys[id].state < 5 { swirl(w, id, ctx); }
}

/// `0x3cc9a0`'s tail for states 0..4 (module docs of [`update_0985`]).
fn swirl(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    let ph = m.pv_f(0x30);
    m.set_pv_f(0x30, add_rot(ph, 100.0 * 0.000_290_888_25));
    if m.pv_f(0x18) < 1.0 {
        let mut f = m.pv_f(0x18);
        crate::hero::physics::approach(1.0, f32::from_bits(0x3d08_8889), &mut f);
        m.set_pv_f(0x18, f);
    }
    let pos = m.pos;
    let rng = &mut ctx.game.rng;
    if rng.randi(2) == 0 { return; }
    let a = rng.rand_angle();
    let at = [a.cos() * 0.85 + pos[0], a.sin() * 0.85 + pos[1], pos[2] + 0.1];
    let size = rng.randf(0.3, 0.2);
    let t = rng.randf(0.0, 1.0);
    let colour = crate::hud::lerp_rgba(t, 0x8080_8000, 0x8080_4000);
    let up = rng.randf(2.0, 3.0);
    let life = rng.rand_range(0x3c, 0xb4);
    let vel = [0.0, 0.0, up / life as f32];
    let rot = rng.randi(0xff) as u8;
    let hero = ctx.hero.pos;
    if let Some(sys) = ctx.parts.as_deref_mut() { crate::particles::glint(sys, size, at, vel, colour, life as u16, rot, false, hero); }
}

/// `0x3fd9259b`: how far 0x0995 tips (97.2°); `$gp+0x140c`: the other way's stop (0.15 π); `$gp+0x1408`: the fall's
/// gain divisor.
const TIP: f32 = 1.696_460_1;
const TIP_BACK: f32 = 0.15;
const TIP_GAIN: f32 = 800.0;

/// 0x0995 (`0x3cd460`, 8): flaps that tip over when the moby they carry (`+0x68`[0x70], −1 none) is hooked.
/// * **0**: with a moby, 2, keeping where it sits in its frame (`[0x60]`), no sound (`[0x7c]` −1), no bounces
///   (`[0x78]`); else 1.
/// * **2**: the moby hooked: its class sound 0 looped (unless still playing), its group's living members put in state
///   4 (`0x31dd70`), it 3.
/// * **3**: it tips about y: the speed `[0x74]` gains its angle / 800 + 0.0000970 a tick; past [`TIP`] it stops there
///   (the first five times its loop stopped and its sound 1), bouncing back at half the speed until it is under
///   0.000582, then 5. The moby rides it.
/// * **4**: the same toward 0.15 π, without the sound or the moby.
pub fn update_0995(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let other = usize::try_from(w.mobys[id].pv_i(0x70)).ok().filter(|&o| o < w.mobys.len());
    match w.mobys[id].state {
        0 => {
            let Some(o) = other else { return w.mobys[id].set_state(1, -1) };
            let op = w.mobys[o].pos;
            let m = &mut w.mobys[id];
            m.set_state(2, -1);
            let d = [op[0] - m.pos[0], op[1] - m.pos[1], op[2] - m.pos[2]];
            m.set_pv_i(0x7c, -1);
            m.set_pv_i(0x78, 0);
            let r = m.rows;
            m.set_pv_v(0x60, [0, 1, 2].map(|k| d[0] * r[k][0] + d[1] * r[k][1] + d[2] * r[k][2]));
        }
        2 => {
            let Some(o) = other else { return };
            if w.mobys[o].once & 4 == 0 { return; }
            let h = w.mobys[id].pv_i(0x7c);
            if !(h != -1 && ctx.host.sound_playing(h)) {
                let h = ctx.host.play_sound(&w.mobys[id], 0, 4);
                w.mobys[id].set_pv_i(0x7c, h);
            }
            if let Some(g) = usize::try_from(w.mobys[id].group).ok().and_then(|g| w.groups.get(g)).cloned() {
                for i in g {
                    if let Some(x) = w.mobys.get_mut(i) { if x.state < 0x80 { x.state = 4; } }
                }
            }
            w.mobys[id].set_state(3, -1);
        }
        3 => {
            let m = &mut w.mobys[id];
            let v = m.pv_f(0x74) + m.rot[1] / TIP_GAIN + 9.696_275e-5;
            m.set_pv_f(0x74, v);
            m.rot[1] += v;
            if TIP < m.rot[1] {
                if m.pv_i(0x78) < 5 {
                    let h = m.pv_i(0x7c);
                    if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
                    let m = &mut w.mobys[id];
                    m.set_pv_i(0x7c, -1);
                    ctx.host.play_sound(&w.mobys[id], 1, 0);
                    let m = &mut w.mobys[id];
                    m.set_pv_i(0x78, m.pv_i(0x78) + 1);
                }
                let m = &mut w.mobys[id];
                m.rot[1] = TIP;
                if 0.000_581_776_5 < m.pv_f(0x74) {
                    m.set_pv_f(0x74, m.pv_f(0x74) * -0.5);
                } else {
                    m.set_pv_f(0x74, 0.0);
                    m.set_state(5, -1);
                }
            }
            if let Some(o) = other {
                let m = &w.mobys[id];
                let at = add(to_world(m.pv_v(0x60), &m.rows), m.pos);
                w.mobys[o].pos = at;
                w.changed = true;
            }
        }
        4 => {
            let m = &mut w.mobys[id];
            let v = m.pv_f(0x74) + m.rot[1] / TIP_GAIN + 9.696_275e-5;
            m.set_pv_f(0x74, v);
            m.rot[1] += v;
            let stop = TIP_BACK * std::f32::consts::PI;
            if m.rot[1] <= stop { return; }
            m.rot[1] = stop;
            if m.pv_f(0x74) <= 0.000_581_776_5 {
                m.set_pv_f(0x74, 0.0);
                m.set_state(5, -1);
            } else {
                m.set_pv_f(0x74, m.pv_f(0x74) * -0.5);
            }
        }
        _ => {}
    }
}
