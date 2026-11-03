//! Level 6's brawler **0x0bba** (L06 `0x41f460`, 40 placed; its senses `0x420490`, engage `0x421080`, wander point
//! `0x4211e0`, walk `0x420828`, tail `0x420ae0`, muzzle sparks `0x420da0`) on the framework ([`crate::moby::ai`]):
//! health `+0x20`, the colour flash `+0x50`, reaction `+0x60` (a hurt goes to sub 3), target `+0x200`; and the switch
//! **0x0fdc** (`0x4594e8`) whose hit sends its groups' brawlers wandering, smashed (`0x459750`) by the hit or by a
//! brawler's charge. It throws the arc shot 0xfe5 (crate::moby::classes::level06::arc_shot).
//!
//! Its words: the two target areas `+0x220` / `+0x224` and what they found `+0x228` / `+0x22c` within the ranges
//! `+0x240` / `+0x244` (height 10, `0x36bbc0` = crate::moby::ai::find_target); its polygon and network start path
//! `+0x230`, `+0x234` / `+0x238` an area pair copied once it sees Ratchet (`+0x5c0` ≠ 0), its run path `+0x23c` (−1
//! none); timers `+0x24a` / `+0x24c` (s16) and `+0x25c` (the engage's wait, nothing sets it); alarmed `+0x258` (set by
//! its group's brawlers and glowbugs); its path network `+0x260` (16 records, crate::path_network, the count `+0x480`,
//! the records it and its goal are in `+0x488` / `+0x48c`); home `+0x460` (`w` its yaw); the walk's speed `+0x470` and
//! turn speed `+0x474`, the run's point `+0x478`, its yaw offset `+0x47c` (±10..25°); `+0x490` (s16) it starts on its
//! run path, `+0x492` (s16) it starts sitting, `+0x494` the punches left before it gives up (sub 4); the shot slot
//! `+0x498`, the ground spring `+0x49c`; the run flag `+0x5a0` (from `+0x490`), the wander yaw `+0x5a4` and goal
//! `+0x5b0`, the walk's stuck ticks `+0x5a8`, its damage `+0x5ac` (the health block's byte `+0x34`); `+0x5c4` ≠ 0 the
//! charge follows the ground (and smashes no switch), `+0x5c8` ≠ 0 its sound 0xb once on first sight.
//!
//! The L06 engine functions with the port's: `0x35b8c8` set_state, `0x35b5e8` the sequence, `0x3682a0` take_damage,
//! `0x368fe8` flight, `0x35c418` = crate::moby::ai::step_walk, `0x36b048` = crate::path_network::steer_offset,
//! `0x35f460` / `0x35efa8` / `0x35ed48` / `0x35ee08` = L00 `0x31f230` / `0x31ed78` / `0x31eb18` / `0x31ebd8`
//! (crate::hero::physics::turn_ease / ease / approach / spring), `0x35d038` = crate::moby::ai::ground_z, `0x35d398`
//! = crate::moby::ai::attack_sphere, `0x31f0c0` the sphere query (crate::collision::coll_sphere_m; the moby it met
//! at `0x1bfc98`), `0x35d180` = crate::level::LevelData::in_polygon, `0x37b450` = crate::particles::twinkle,
//! `0x362678` = L00 `0x321c00` (crate::camera::rot), `0x367818` = crate::moby::shards::throw_chunks, `0x42aa28` the
//! fade (crate::moby::classes::death_fade).
//!
//! Not ported: the head's look at its target (`0x365050` / `0x3674d8` on `+0x4a0` / `+0x520`, the joint aims); the
//! switch's smaller smash (13 pieces) when the frame's loads `0x1b234c` / `0x1b2350` are 0.9 or more (the port's
//! frame is never that loaded: 40).

use crate::hero::physics::{add, approach, atan2, dist, dist2, ease, rot_dist, scale, set_len, spring, sub, turn_ease, V3};
use crate::moby::classes::props::{seq, wrapped};
use crate::moby::{ai, countdown, Ctx, MobyWorld};

pub const CLASS: i16 = 0x0bba;
pub const SWITCH_CLASS: i16 = 0x0fdc;

const FLASH: usize = 0x50;
const T_POINT: usize = 0x200;
const AREA_A: usize = 0x220;
const AREA_B: usize = 0x224;
const SEEN_A: usize = 0x228;
const SEEN_B: usize = 0x22c;
const POLY: usize = 0x230;
const AREA_C: usize = 0x234;
const AREA_D: usize = 0x238;
const RUN_PATH: usize = 0x23c;
const RANGE_A: usize = 0x240;
const RANGE_B: usize = 0x244;
const T24A: usize = 0x24a;
const T24C: usize = 0x24c;
const ALARMED: usize = 0x258;
const WAIT: usize = 0x25c;
const NODES: usize = 0x260;
const NODES_MAX: usize = 16;
const HOME: usize = 0x460;
const SPEED: usize = 0x470;
const TURN_V: usize = 0x474;
const RUN_AT: usize = 0x478;
const OFFSET: usize = 0x47c;
const N: usize = 0x480;
const AT: usize = 0x488;
const GOAL: usize = 0x48c;
const RUN_START: usize = 0x490;
const SITS: usize = 0x492;
const PUNCHES: usize = 0x494;
const SLOT: usize = 0x498;
const SPRING_V: usize = 0x49c;
const RUNS: usize = 0x5a0;
const W_YAW: usize = 0x5a4;
const STUCK: usize = 0x5a8;
const DAMAGE: usize = 0x5ac;
const W_GOAL: usize = 0x5b0;
const COPY_AREAS: usize = 0x5c0;
const GROUNDED: usize = 0x5c4;
const CALL: usize = 0x5c8;
const PVARS: usize = 0x5d0;
const DEG: f32 = 0.017_453_294;
/// 25°: facing its target.
const FACING: f32 = 0.436_332_35;

fn st(w: &mut MobyWorld, id: usize, s: u8, sub: i32) { w.mobys[id].set_state(s, sub); }
fn anim(w: &MobyWorld, id: usize, ctx: &mut Ctx, s: u8, frame: u8, blend: u8) {
    if seq(w, id, ctx) != s { ctx.host.set_anim(&w.mobys[id], s, frame, blend); }
}
/// The reaction's keys (`+0x48` 20, `+0x4a` 8 or 24).
fn keys(w: &mut MobyWorld, id: usize, b: u8) {
    let m = &mut w.mobys[id];
    m.set_pv_b(0x48, 20);
    m.set_pv_b(0x4a, b);
}
fn steady(w: &MobyWorld, id: usize, ctx: &Ctx) -> bool { ctx.host.anim(&w.mobys[id]).is_some_and(|a| a.seq_a == a.seq_b) }
fn seeing(w: &MobyWorld, id: usize, off: usize) -> bool { w.mobys[id].pv_i(off) == 1 }
fn tick_s(w: &mut MobyWorld, id: usize, off: usize) -> bool {
    let mut t = w.mobys[id].pv_s(off) as i32;
    let out = countdown(&mut t);
    w.mobys[id].set_pv_s(off, t as i16);
    out
}
fn tick_i(w: &mut MobyWorld, id: usize, off: usize) -> bool {
    let mut t = w.mobys[id].pv_i(off);
    let out = countdown(&mut t);
    w.mobys[id].set_pv_i(off, t);
    out
}
fn yaw_to(w: &MobyWorld, id: usize, p: V3) -> f32 {
    let q = w.mobys[id].pos;
    atan2(p[0] - q[0], p[1] - q[1])
}
fn faces(w: &MobyWorld, id: usize, p: V3) -> bool { rot_dist(w.mobys[id].rot[2], yaw_to(w, id, p)) < FACING }

/// A wander: a random yaw, its goal home, `+0x24c` 270, state 4 (sequence 6) — the part the switch also does.
fn wander(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let y = ctx.game.rng.rand_angle();
    let m = &mut w.mobys[id];
    m.set_pv_f(W_YAW, y);
    let h = m.pv_v(HOME);
    m.set_pv_v(W_GOAL, h);
    let hw = m.pv_f(HOME + 0xc);
    m.set_pv_f(W_GOAL + 0xc, hw);
    m.set_pv_s(T24C, 0x10e);
}

/// `0x4211e0`: outside its polygon its goal is home (`w` 1); else a point within 7 across of it (`0x35bc10`: a random
/// way, `(1 − f²)·7` long), kept (`w` 1) when inside, else its own place (`w` 0).
fn wander_point(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    let poly = w.mobys[id].pv_i(POLY);
    let pos = w.mobys[id].pos;
    if !ctx.level.in_polygon(poly, pos) {
        let m = &mut w.mobys[id];
        let h = m.pv_v(HOME);
        m.set_pv_v(W_GOAL, h);
        m.set_pv_f(W_GOAL + 0xc, 1.0);
        return true;
    }
    let rng = &mut ctx.game.rng;
    let a = rng.rand_angle();
    let f = ((rng.rand() >> 16) & 0xfff) as f32 * 0.000_244_140_62;
    let d = (1.0 - f * f) * 7.0;
    let g = add([a.cos() * d, a.sin() * d, 0.0], pos);
    let inside = ctx.level.in_polygon(poly, g);
    let m = &mut w.mobys[id];
    m.set_pv_v(W_GOAL, if inside { g } else { pos });
    m.set_pv_f(W_GOAL + 0xc, if inside { 1.0 } else { 0.0 });
    inside
}

/// `0x420828(yaw, speed, near, offset, m, goal)`: farther than `near` across, along its network (`0x36b048`, its yaw
/// offset, 0.5) turning (0.00378 / 0.00378 / 0.297) and, within 75° of the way, its speed eased toward the distance
/// (`speed` a second, its 1/1200 the step); else turning to `yaw` and braking — stopped and within 5° of it, done.
/// The step along its facing (`0x35c418` = crate::moby::ai::step_walk 0.9 / 1 / 0.5 / 45°); 20 stuck ticks, done.
#[allow(clippy::too_many_arguments)]
fn walk(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, yaw: f32, speed: f32, near: f32, off: f32, goal: V3) -> bool {
    let mut done = false;
    let m = &w.mobys[id];
    let pos = m.pos;
    let l = dist2(goal, pos);
    let step = speed * 0.000_833_333_35;
    let nodes = crate::path_network::read_nodes(&m.pvars, NODES, (m.pv_i(N).max(0) as usize).min(NODES_MAX));
    let m = &mut w.mobys[id];
    let (mut a, mut v, mut s) = (m.rot[2], m.pv_f(TURN_V), m.pv_f(SPEED));
    let (ta, tb) = (f32::from_bits(0x3b77_d3d5), f32::from_bits(0x3e97_e9da));
    if near < l {
        let (mut y, mut at, mut to) = (0.0, m.pv_i(AT), m.pv_i(GOAL));
        crate::path_network::steer_offset(ctx.level, &nodes, off, 0.5, pos, goal, &mut y, &mut at, &mut to);
        m.set_pv_i(AT, at);
        m.set_pv_i(GOAL, to);
        turn_ease(y, ta, ta, tb, &mut a, &mut v, 0);
        if rot_dist(a, y) < 1.308_997 {
            let mut x = 0.0;
            ease(l, step, step, speed * 0.016_666_668, &mut x, &mut s);
        } else {
            s = 0.0;
        }
    } else {
        turn_ease(yaw, ta, ta, tb, &mut a, &mut v, 0);
        approach(0.0, step, &mut s);
        if s == 0.0 && rot_dist(a, yaw) < 0.087_266_475 {
            v = 0.0;
            done = true;
        }
    }
    m.rot[2] = a;
    m.set_pv_f(TURN_V, v);
    m.set_pv_f(SPEED, s);
    let mut target = add(set_len(m.rows[0], s), pos);
    let mut p = pos;
    let ok = ai::step_walk(ctx, id, f32::from_bits(0x3f66_6666), 1.0, 0.5, f32::from_bits(0x3f49_0fdc), &mut p, &mut target, 0);
    let m = &mut w.mobys[id];
    m.pos = p;
    w.changed = true;
    if !ok { m.set_pv_i(STUCK, m.pv_i(STUCK) + 1); }
    if 0x13 < m.pv_i(STUCK) {
        m.set_pv_i(STUCK, 0);
        done = true;
    }
    done
}

/// `0x420d20`: its group's live brawlers alarmed (`+0x258`).
fn alarm(w: &mut MobyWorld, id: usize) {
    let members = usize::try_from(w.mobys[id].group).ok().and_then(|g| w.groups.get(g)).cloned().unwrap_or_default();
    for k in members {
        let Some(m) = w.mobys.get_mut(k).filter(|m| m.state < 0x80 && m.o_class == CLASS) else { continue };
        m.set_pv_i(ALARMED, 1);
    }
}

/// `0x420da0(at, n)` (life 20) and `0x45ae40(at, n)` (life 60): `n` pairs of twinkles 0.3 toward the camera from `at`
/// (which jitters ±0.15 a pair), thrown 0.05..3 a second across (the camera's way turned ±60°) and 1..5 up, 0.5..2 in
/// size: red `0x7f801010` and a smaller white `0x7f7f7f7f`, spinning opposite ways.
pub(super) fn sparks(ctx: &mut Ctx, at: &mut V3, n: i32, life: i16) {
    let cam = ctx.host.camera_pos();
    let mut d = sub(cam, *at);
    let off = set_len(d, 0.3);
    d[2] = 0.0;
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let rng = &mut ctx.game.rng;
    for _ in 0..n {
        let f = rng.randf(0.5, 2.0);
        let r = rng.randi(2);
        let spin = if r == 0 { -1 } else { r } as i8;
        let a = rng.randf(-1.047_197_7, 1.047_197_7);
        let v = crate::camera::rot(d, a, [0.0, 0.0, 1.0]);
        let k = rng.randf(0.05, 3.0) * 0.016_666_668;
        let mut v = set_len(v, k);
        v[2] = rng.randf(1.0, 5.0) * 0.016_666_668;
        let p = add(*at, off);
        for k in 0..3 { at[k] += rng.randf_signed(0.0, 0.15); }
        if let Some(i) = crate::particles::twinkle(sys, rng, f * 0.1, f, 0.002, p, life, 0x7f80_1010, 0, spin, v, 0) { sys.pool.recs[i][9] = 0x74; }
        if let Some(i) = crate::particles::twinkle(sys, rng, f * 0.07, f * 0.7, 0.002, p, life, 0x7f7f_7f7f, 1, -spin, v, 0) { sys.pool.recs[i][9] = 0x74; }
    }
}

/// `0x420490`. **The set-up**: its network from its polygon path (`0x36abb8` = crate::path_network::build), home
/// its place (`w` its yaw), the reaction (sequences 0x10 / 0xe / 0xf, speeds 0.2 / 0.125, gravities 1/180 / 1/410 /
/// 1/240, keys, hurts 7 / 0xc / 3 / 5 / 10 / 0x11, the share 0.75), its damage, the yaw offset, `+0x24c` 0..95, its
/// run flag from `+0x490`; idle (1, sub 0, sequence 1; sitting sequence 3). **Each tick**: its shadow within 24 of the
/// camera; a hurt (a hit of kind 10 sets `+0xd7`; a knock-down kind 5 spins it 48°): its group alarmed, the hit let
/// go; the two searches; under z 48 its bolts, a smoke burst, gone (true).
fn senses(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    if w.mobys[id].state == 0 {
        let m = &mut w.mobys[id];
        if m.pvars.len() < PVARS { m.pvars.resize(PVARS, 0); }
        let mut nodes = crate::path_network::read_nodes(&m.pvars, NODES, NODES_MAX);
        let n = crate::path_network::build(ctx.level, m.pv_i(POLY), &mut nodes);
        crate::path_network::write_nodes(&mut m.pvars, NODES, &nodes[..n]);
        m.set_pv_i(N, n as i32);
        crate::path_network::open(&mut ctx.game.path_openings, ctx.level, &nodes[..n]);
        let (p, y) = (m.pos, m.rot[2]);
        m.set_pv_v(HOME, p);
        m.set_pv_f(HOME + 0xc, y);
        m.set_pv_b(0xd4, 0);
        m.set_pv_s(0x26, 0);
        for (o, b) in [(0x72, 0x10), (0x76, 0xe), (0x77, 0xf), (0xcc, 7), (0xcd, 0xc), (0xce, 3), (0xcf, 5), (0xd2, 10), (0xd3, 0x11), (0x29, 1), (0x70, 0xf), (0x71, 0xc), (0x28, 1)] { m.set_pv_b(o, b); }
        for (o, v) in [(0x7c, 0x3e4c_ccceu32), (0x80, 0x3e00_0000), (0x84, 0x3bb6_0b61), (0x88, 0x3c1f_49f5), (0x94, 0x3f00_0000), (0x9c, 0x3ecc_cccd), (0x30, 0x3f40_0000), (0x90, 0x3f00_0000), (0x8c, 0x3b88_8889)] { m.set_pv_f(o, f32::from_bits(v)); }
        let d = m.pv_b(0x34) as f32;
        m.set_pv_f(DAMAGE, d);
        m.set_pv_i(0xa8, 3);
        let off = ctx.game.rng.randf_signed(10.0, 25.0);
        let t = ctx.game.rng.rand_range(0, 0x5f);
        let m = &mut w.mobys[id];
        m.set_pv_i(RUN_AT, 0);
        m.set_pv_f(OFFSET, off * DEG);
        m.set_pv_i(SLOT, -1);
        let r = m.pv_s(RUN_START) as i32;
        m.set_pv_i(RUNS, r);
        m.set_pv_s(T24C, t as i16);
        m.set_pv_v(W_GOAL, [0.0; 3]);
        m.set_pv_f(W_GOAL + 0xc, 0.0);
        st(w, id, 1, 0);
        anim(w, id, ctx, 1, 0, 0);
        keys(w, id, 8);
        if w.mobys[id].pv_s(SITS) != 0 {
            anim(w, id, ctx, 3, 0, 0);
            keys(w, id, 24);
        }
    }
    if w.mobys[id].drawn != 0 { crate::shadow_volumes::probe_near(w, id, ctx, 24.0, 0x16); }
    let k = ai::take_damage(w, id, ctx, false);
    if k != 0 {
        let m = &mut w.mobys[id];
        if m.pv_b(0x37) == 10 { m.set_pv_b(0xd7, 1); }
        if k == 5 {
            m.set_pv_f(0xf0, 0.0);
            m.set_pv_f(0xf4, f32::from_bits(0x3f56_7752));
        }
        alarm(w, id);
        w.clear_hit(id);
    }
    for (range, area, seen) in [(RANGE_A, AREA_A, SEEN_A), (RANGE_B, AREA_B, SEEN_B)] {
        let (r, a) = (w.mobys[id].pv_f(range), w.mobys[id].pv_i(area));
        let s = ai::find_target(w, id, ctx, r, 10.0, a, None, false);
        w.mobys[id].set_pv_i(seen, s as i32);
    }
    if w.mobys[id].pos[2] < 48.0 {
        crate::moby::bolts::drop(w, id, 0, 3, ctx);
        let pos = w.mobys[id].pos;
        crate::moby::blast::smoke_burst(w, ctx, 1.0, 13.0, Some(id), pos, -1);
        w.delete(id);
        return true;
    }
    false
}

/// `0x421080`: on its run path the run (8), without one once `+0x25c` is out the charge (7, sequence 0x13); sitting,
/// once it is out it stands (`+0x492` 0) into the charge (7, sequence 0x11 or 0x13); else once it is out the
/// approach (9, sequence 0x12) — each sub 2.
fn engage(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let s = if w.mobys[id].pv_i(RUNS) == 1 {
        if w.mobys[id].pv_i(RUN_PATH) != -1 {
            st(w, id, 8, 2);
            0x12
        } else {
            if !tick_i(w, id, WAIT) { return; }
            st(w, id, 7, 2);
            0x13
        }
    } else if w.mobys[id].pv_s(SITS) != 0 {
        if !tick_i(w, id, WAIT) { return; }
        w.mobys[id].set_pv_s(SITS, 0);
        st(w, id, 7, 2);
        if ctx.game.rng.randi(2) == 0 { 0x11 } else { 0x13 }
    } else {
        if !tick_i(w, id, WAIT) { return; }
        st(w, id, 9, 2);
        0x12
    };
    anim(w, id, ctx, s, 0, 0x14);
    keys(w, id, 24);
}

/// 0x0bba (`0x41f460`). **Subs**: 0 calm: seeing Ratchet in its first area (its sound 0xb the first time) it engages
/// ([`engage`]; with `+0x5c0` the area pair copied over); alarmed, it engages. 1 alert: seeing him in either area it
/// engages; `+0x24a` out, calm; the alarm cleared. 2 engaged: seeing him in neither, at its sequence's end it gives up
/// (its run path's last point, two punches left, 0xe sub 4, `+0x24a` 580..600). 3 the knock (the framework's flight):
/// dead, its bolts and the fade; over, seeing him the approach (9, facing him within 25° the throw 0xb), else alert
/// wandering (5 sub 1, `+0x24a` 580..600). 4 giving up: no punches left, a wander (5 sub 1).
///
/// **States**: 1 idle (sequence 1; `+0x24c` out, not sitting: sequence 2 and `+0x24c` 900..1200). 4 the wander walk
/// (sequence 6, [`walk`] at 4 a second, near 0.25, its yaw offset, to the wander goal; there or `+0x24c` out: 5,
/// sequence 7). 5 at its end a new wander point ([`wander_point`]) and 4. 6 at its end the approach. 7 the charge: on
/// its sequence's frames 4..22, 0.16 a tick along its facing (with `+0x5c4` falling 1/6 a tick to the ground); without
/// it a 0.5 sphere 0.6 up 1 ahead smashing a switch 0x0fdc it meets; at its end the approach (the run flag cleared). 8
/// the run along its path (8 a second, each point within 1.5); at its end the run flag cleared, and facing him within
/// 25° home its place and the wind-up 10 (sequence 0x15). 9 the approach (a new yaw offset; [`walk`] at 8, near 16):
/// there facing him, 10. 10 the wind-up turning to him: at its end within 3 the punch 0xc (sequence 0xd from frame 10),
/// else the throw 0xb (sequence 9). 0xb the throw: at frame 2 a shot from its joint 0 at its target 10 a second (a free
/// slot of six: muzzle sparks, crate::moby::classes::level06::arc_shot::fire, its damage the byte `+0x35`); at its end
/// 0xd (sequence 10). 0xc the punch: frames 17..25 a sphere between its joint 0 and 2 above its feet (¾ of half the
/// way, its damage, flags 1); at its end 0xd (sequence 0xb). 0xd at its end giving up a punch less (0xe) or the
/// approach; within 4 facing him within 25°, the wind-up. 0xe the wind-up.
pub fn update_0bba(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if senses(w, id, ctx) { return; }
    match w.mobys[id].sub_state {
        0 => {
            if seeing(w, id, SEEN_A) {
                if w.mobys[id].pv_i(CALL) != 0 {
                    w.mobys[id].set_pv_i(CALL, 0);
                    ctx.host.play_sound(&w.mobys[id], 0xb, 0);
                }
                engage(w, id, ctx);
                let m = &mut w.mobys[id];
                if m.pv_i(COPY_AREAS) != 0 {
                    let (c, b) = (m.pv_i(AREA_C), m.pv_i(AREA_B));
                    m.set_pv_i(AREA_D, c);
                    m.set_pv_i(AREA_A, b);
                }
            } else if w.mobys[id].pv_i(ALARMED) != 0 {
                engage(w, id, ctx);
            }
        }
        1 => {
            if seeing(w, id, SEEN_A) || seeing(w, id, SEEN_B) {
                engage(w, id, ctx);
            } else if w.mobys[id].pv_s(T24A) == 0 {
                w.mobys[id].sub_state = 0;
            }
            w.mobys[id].set_pv_i(ALARMED, 0);
        }
        2 => {
            if !seeing(w, id, SEEN_A) && !seeing(w, id, SEEN_B) && wrapped(w, id, ctx) {
                let path = w.mobys[id].pv_i(RUN_PATH);
                if path != -1 {
                    let n = usize::try_from(path).ok().and_then(|p| ctx.level.paths.get(p)).map_or(0, |p| p.len() as i32);
                    w.mobys[id].set_pv_i(RUN_AT, n - 1);
                }
                w.mobys[id].set_pv_f(PUNCHES, 2.0);
                st(w, id, 0xe, 4);
                let t = ctx.game.rng.rand_range(0x244, 600);
                w.mobys[id].set_pv_s(T24A, t as i16);
            }
        }
        3 => {
            let r = ai::flight(w, id, ctx);
            if r & 1 != 0 {
                crate::moby::bolts::drop(w, id, 0, 3, ctx);
                super::super::death_fade::start(w, id, ctx, 0.0, None);
                return;
            }
            if r & 2 != 0 {
                if seeing(w, id, SEEN_A) || seeing(w, id, SEEN_B) {
                    let t = w.mobys[id].pv_v(T_POINT);
                    if !faces(w, id, t) {
                        st(w, id, 9, 2);
                        anim(w, id, ctx, 0x12, 0, 10);
                        keys(w, id, 24);
                    } else {
                        st(w, id, 0xb, 2);
                        anim(w, id, ctx, 9, 0, 2);
                        keys(w, id, 8);
                    }
                } else {
                    let t = ctx.game.rng.rand_range(0x244, 600);
                    w.mobys[id].set_pv_s(T24A, t as i16);
                    st(w, id, 5, 1);
                    anim(w, id, ctx, 0x12, 0, 0x14);
                    keys(w, id, 24);
                }
            }
        }
        4 => {
            if w.mobys[id].pv_f(PUNCHES) <= 0.0 {
                wander(w, id, ctx);
                st(w, id, 4, -1);
                anim(w, id, ctx, 6, 0, 10);
                keys(w, id, 24);
                st(w, id, 5, 1);
            }
        }
        _ => {}
    }
    states(w, id, ctx);
    tail(w, id, ctx);
}

/// The wind-up (10, sequence 0x15).
fn wind_up(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    st(w, id, 10, -1);
    anim(w, id, ctx, 0x15, 0, 0x10);
    keys(w, id, 8);
}

fn states(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let t = w.mobys[id].pv_v(T_POINT);
    match w.mobys[id].state {
        1 => {
            let out = tick_s(w, id, T24C);
            let sits = w.mobys[id].pv_s(SITS) != 0;
            if out && !sits {
                anim(w, id, ctx, 2, 0, 0x14);
                let t = ctx.game.rng.rand_range(900, 0x4b0);
                w.mobys[id].set_pv_s(T24C, t as i16);
                keys(w, id, 24);
            } else if wrapped(w, id, ctx) && !sits {
                anim(w, id, ctx, 1, 0, 5);
                keys(w, id, 8);
            }
        }
        4 => {
            let (y, off, g) = (w.mobys[id].pv_f(W_YAW), w.mobys[id].pv_f(OFFSET), w.mobys[id].pv_v(W_GOAL));
            if walk(w, id, ctx, y, 4.0, 0.25, off, g) || tick_s(w, id, T24C) {
                st(w, id, 5, -1);
                anim(w, id, ctx, 7, 0, 10);
                keys(w, id, 8);
            }
        }
        5 => {
            if wrapped(w, id, ctx) {
                let y = ctx.game.rng.rand_angle();
                w.mobys[id].set_pv_f(W_YAW, y);
                wander_point(w, id, ctx);
                w.mobys[id].set_pv_s(T24C, 0x10e);
                st(w, id, 4, -1);
                anim(w, id, ctx, 6, 0, 10);
                keys(w, id, 24);
            }
        }
        6 => {
            if wrapped(w, id, ctx) {
                st(w, id, 9, 2);
                anim(w, id, ctx, 0x12, 0, 0x14);
                keys(w, id, 24);
            }
        }
        7 => {
            if wrapped(w, id, ctx) {
                w.mobys[id].state = 9;
                anim(w, id, ctx, 0x12, 0, 0x14);
                keys(w, id, 24);
                w.mobys[id].set_pv_i(RUNS, 0);
                return;
            }
            charge(w, id, ctx);
        }
        8 => {
            let path = w.mobys[id].pv_i(RUN_PATH);
            let Some(pts) = usize::try_from(path).ok().and_then(|p| ctx.level.paths.get(p)).cloned() else {
                st(w, id, 9, -1);
                return;
            };
            let n = pts.len() as i32;
            let k = w.mobys[id].pv_i(RUN_AT);
            let pt = |k: i32| usize::try_from(k).ok().and_then(|k| pts.get(k)).map_or([0.0; 3], |p| [p[0], p[1], p[2]]);
            let y = yaw_to(w, id, t);
            walk(w, id, ctx, y, 8.0, 0.25, 0.0, pt(k));
            if dist2(w.mobys[id].pos, pt(k)) < 1.5 { w.mobys[id].set_pv_i(RUN_AT, k + 1); }
            if w.mobys[id].pv_i(RUN_AT) < n { return; }
            let m = &mut w.mobys[id];
            m.set_pv_i(RUNS, 0);
            m.set_pv_i(RUN_AT, n - 1);
            if !faces(w, id, t) { return; }
            // Home its place (its `w` the place's, which the port does not keep).
            let p = w.mobys[id].pos;
            w.mobys[id].set_pv_v(HOME, p);
            wind_up(w, id, ctx);
        }
        9 => {
            if (w.mobys[id].state_ticks as u16) < 2 {
                let o = ctx.game.rng.randf_signed(10.0, 25.0);
                w.mobys[id].set_pv_f(OFFSET, o * DEG);
            }
            let (y, off) = (yaw_to(w, id, t), w.mobys[id].pv_f(OFFSET));
            if walk(w, id, ctx, y, 8.0, 16.0, off, t) && faces(w, id, t) { wind_up(w, id, ctx); }
        }
        10 => {
            let (y, p) = (yaw_to(w, id, t), w.mobys[id].pos);
            walk(w, id, ctx, y, 6.0, 6.0, 0.0, p);
            if !wrapped(w, id, ctx) { return; }
            if dist2(w.mobys[id].pos, t) < 3.0 {
                st(w, id, 0xc, -1);
                anim(w, id, ctx, 0xd, 10, 0x14);
                keys(w, id, 24);
            } else {
                st(w, id, 0xb, -1);
                anim(w, id, ctx, 9, 0, 2);
                keys(w, id, 8);
            }
        }
        0xb => {
            if wrapped(w, id, ctx) {
                st(w, id, 0xd, -1);
                if seq(w, id, ctx) != 10 {
                    let b = ctx.game.rng.rand_range(10, 0x1e) as u8;
                    ctx.host.set_anim(&w.mobys[id], 10, 0, b);
                }
                keys(w, id, 8);
                return;
            }
            if ai::reached(w, id, ctx, 2.0) { throw(w, id, ctx, t); }
        }
        0xc => {
            let f = ctx.host.frame_time(&w.mobys[id]);
            if steady(w, id, ctx) && (17.0..=25.0).contains(&f) {
                w.mobys[id].set_pv_b(0xd7, 0);
                let Some(j) = ctx.host.joint(&w.mobys[id], 0) else { return };
                let p = w.mobys[id].pos;
                let q = [p[0], p[1], p[2] + 2.0];
                let mid = add(j, scale(sub(q, j), 0.5));
                let r = dist(mid, q) * 0.75;
                let d = w.mobys[id].pv_f(DAMAGE);
                ai::attack_sphere(w, id, ctx, r, d, 1.0, mid, 1, 0, 1, 0);
                return;
            }
            if wrapped(w, id, ctx) {
                st(w, id, 0xd, -1);
                if seq(w, id, ctx) != 0xb {
                    let b = ctx.game.rng.rand_range(10, 0x1e) as u8;
                    ctx.host.set_anim(&w.mobys[id], 0xb, 0, b);
                }
                keys(w, id, 24);
            }
        }
        0xd => {
            if !wrapped(w, id, ctx) { return; }
            if w.mobys[id].sub_state == 4 {
                let p = w.mobys[id].pv_f(PUNCHES);
                w.mobys[id].set_pv_f(PUNCHES, p - 1.0);
                st(w, id, 0xe, -1);
            } else {
                st(w, id, 9, -1);
                anim(w, id, ctx, 0x12, 0, 10);
                keys(w, id, 24);
            }
            if dist2(w.mobys[id].pos, t) < 16.0 && faces(w, id, t) { wind_up(w, id, ctx); }
        }
        0xe => wind_up(w, id, ctx),
        _ => {}
    }
}

/// State 7's move (module docs).
fn charge(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let f = ctx.host.frame_time(&w.mobys[id]);
    let on = steady(w, id, ctx) && (4.0..=22.0).contains(&f);
    let grounded = w.mobys[id].pv_i(GROUNDED) != 0;
    if on {
        let m = &mut w.mobys[id];
        let y = m.rot[2];
        m.pos[0] += y.cos() * 0.16;
        m.pos[1] += y.sin() * 0.16;
        if grounded {
            let g = ai::ground_z(ctx, 0.5, w.mobys[id].pos, 0);
            let m = &mut w.mobys[id];
            m.pos[2] -= 0.166_666_67;
            if m.pos[2] < g { m.pos[2] = g; }
        }
        w.changed = true;
    }
    if grounded { return; }
    let m = &w.mobys[id];
    let y = m.rot[2];
    let at = [m.pos[0] + y.cos(), m.pos[1] + y.sin(), m.pos[2] + 0.6];
    let met = ctx.coll.and_then(|c| crate::collision::coll_sphere_m(c, ctx.scene, at, 0.5, crate::collision::QueryFlags(0), Some(id))).and_then(|o| o.moby);
    if let Some(s) = met.filter(|&s| w.mobys.get(s).is_some_and(|o| o.o_class == SWITCH_CLASS)) { smash(w, s, ctx); }
}

/// State 0xb at frame 2 (module docs): the first free slot of six (`0x1b3760`), muzzle sparks (10, 20 ticks) at its
/// joint 0, the shot from where they leave it.
fn throw(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, t: V3) {
    let Some(mut j) = ctx.host.joint(&w.mobys[id], 0) else { return };
    let v = set_len(sub(t, j), f32::from_bits(0x3e2a_aaab));
    let slots = &w.level06.shot_slots;
    let slot = if slots[0] == 0 { Some(0) } else { (1..6).find(|&k| slots[k] == 0) };
    w.mobys[id].set_pv_i(SLOT, slot.map_or(-1, |s| s as i32));
    if let Some(s) = slot {
        sparks(ctx, &mut j, 10, 0x14);
        let d = w.mobys[id].pv_b(0x35) as f32;
        super::arc_shot::fire(w, ctx, d, id, v, j, s);
        w.mobys[id].set_pv_i(SLOT, -1);
    }
    w.mobys[id].set_pv_b(0xd7, 0);
}

/// `0x420ae0`: the flash, `+0x24a` down, its ticks; within 10 of the ground under it (`0x35d038`), its height sprung
/// toward it (1/1200, 0.01, 0.18) and not below it.
fn tail(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    ai::flash(&mut w.mobys[id], FLASH);
    tick_s(w, id, T24A);
    let m = &mut w.mobys[id];
    if m.state_ticks != -1 { m.state_ticks += 1; }
    let g = ai::ground_z(ctx, 0.5, w.mobys[id].pos, 0);
    let m = &mut w.mobys[id];
    if (m.pos[2] - g).abs() < 10.0 {
        let (mut z, mut v) = (m.pos[2], m.pv_f(SPRING_V));
        spring(g, f32::from_bits(0x3a5a_740f), f32::from_bits(0x3c23_d70b), f32::from_bits(0x3e38_51ec), &mut z, &mut v);
        m.pos[2] = z.max(g);
        m.set_pv_f(SPRING_V, v);
    }
}

/// 0x0fdc (`0x4594e8`): set up (1, mode | 0x800); a hit (`0x35df18(m, 0x10000, 0)`) not from an arc shot 0xfe5: the
/// live brawlers of its group `[0]` with health and of its group `[1]` sent wandering (the wander's yaw and goal home,
/// `+0x24c` 270, sequence 6, sub 1, state 5; the first group's also stand and leave their run), then smashed
/// ([`smash`]).
pub fn update_0fdc(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 0 {
        let m = &mut w.mobys[id];
        m.state = 1;
        m.mode |= 0x800;
    }
    let hit = w.hit_message(id).filter(|h| h.tmpl.flags & 0x10000 != 0);
    let by_shot = hit.and_then(|h| h.tmpl.source).and_then(|s| w.mobys.get(s)).is_some_and(|s| s.o_class == super::arc_shot::CLASS);
    if hit.is_none() || by_shot {
        w.clear_hit(id);
        return;
    }
    for (k, first) in [(0usize, true), (4, false)] {
        let g = w.mobys[id].pv_i(k);
        if g == 0xff { break; }
        let members = usize::try_from(g).ok().and_then(|g| w.groups.get(g)).cloned().unwrap_or_default();
        for b in members {
            let Some(m) = w.mobys.get(b).filter(|m| m.state < 0x80 && m.o_class == CLASS) else { continue };
            if first && m.pv_f(0x20) <= 0.0 { continue; }
            wander(w, b, ctx);
            ctx.host.set_anim(&w.mobys[b], 6, 0, 10);
            let m = &mut w.mobys[b];
            if first {
                m.set_pv_s(SITS, 0);
                m.set_pv_i(RUNS, 0);
            }
            m.sub_state = 1;
            m.state = 5;
        }
        if !first { w.clear_hit(id); }
    }
    smash(w, id, ctx);
}

/// `0x459750`: 40 pieces (`0x367818`: none about, 3 a second, 1..5.5 of their size, pointed, colour `0x50808040`)
/// at random points of its frame (across ±8.045, out |0.1243 across|·1.745, up 0..3.99) moving (0.0041 across,
/// −0.133, 3..7 a second up) through its rows, its class sound 0 with each; gone.
fn smash(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let r = w.mobys[id].rows;
    let through = |v: V3| -> V3 { std::array::from_fn(|k| v[0] * r[0][k] + v[1] * r[1][k] + v[2] * r[2][k]) };
    for _ in 0..0x28 {
        let rng = &mut ctx.game.rng;
        let x = rng.randf(f32::from_bits(0xc100_b852), f32::from_bits(0x4100_b852));
        let y = (x * 0.124_300_81).abs() * 1.745;
        let z = rng.randf(0.0, f32::from_bits(0x407f_5c29));
        let vz = rng.randf(3.0, 7.0) * 0.016_666_668;
        let at = add(through([x, y, z]), w.mobys[id].pos);
        let v = through([x * 0.004_143_360_5, f32::from_bits(0xbe08_8889), vz]);
        crate::moby::shards::throw_chunks(w, ctx, 0.0, 3.0, (1.0, 5.5), id, at, 1, true, v, 0x5080_8040, None);
        ctx.host.play_sound(&w.mobys[id], 0, 0);
    }
    w.delete(id);
}
