//! Barlow's beast **0x08ba** (L04 `0x3e38e8`, 19 placed; its senses `0x3e60b0`, tail `0x3e66a0`, the fire's draw
//! `0x3e67c8`), on the framework ([`crate::moby::ai`]): health block `+0x20` (the most `+0x24`), the flash `+0x50`,
//! reaction `+0x60` (a hurt goes to state 0x16, sub 3), the wander block `+0x110`, movement `+0x140`, target
//! `+0x230` (its moby `+0x240`). A desert rider 0x0ff8 may ride it (crate::moby::classes::level04::rider; its moby at
//! `+0x2ac`): then its every sequence change is the rider's too ([`anim2`], `0x3e3760`), and the rider's saddle
//! 0x0ff9 (its joint 0) is the flamethrower of state 0x12.
//!
//! Its words: the target areas `+0x250` / `+0x254` and what they found `+0x258` / `+0x25c` (ranges `+0x298` /
//! `+0x29c`, height 10); the flame's start `+0x260`, end `+0x270`, way `+0x280` (also the young one's goal) and
//! length `+0x2c8`, its light `+0x2e0`; timers `+0x2a0` / `+0x2a2`, alarmed `+0x2a6`; the flame ticks `+0x2b0`, the
//! turn speed `+0x2b4`, the nozzle's glow size `+0x2b8`, the wander timer `+0x2bc` and range `+0x2c0`; a path to run
//! `+0x2c4` (−1 none), the patrol path `+0x2cc` (walked back and forth, `+0x308` its way), the flames since a run
//! `+0x2d0`, a home cuboid `+0x2d4`, the charge kind `+0x2d8` (≠ 0: the trample 0xf), its territory `+0x2dc` (a path
//! polygon, −1 none), its bite / trample damages `+0x2e4` / `+0x2e8`; a young one's four cuboids `+0x2f0..+0x2fc`
//! (`+0x2f0` = −1: grown up), the one it is at `+0x306`, its wait `+0x300`, its hit guard `+0x307`.
//!
//! **The set-up** (state 0, [`senses`]): on the ground; the reaction (state 0x16 sub 3, sequences 0xb / 0xb / 0xd /
//! 0xe, speeds 0.1 / 1/15, gravities 1/240 / 1/120 / 1/240, no keys), movement (radius 1, height 0.25, steps 0.5,
//! slope 85°, the turn before walking 30°, gravity 1/180), the walk limits (8/s², 4/s, 240″, 6°), the wander's home its
//! place (radius 1.25, range `+0x2c0`, 10 from Ratchet, timer 120..700), its damages the health block's byte `+0x34`;
//! idle (1, sub 0). Grown up: sequence 4 from a random frame at 0.5..1.25 speed. Young: 0.4 of its class's size, at
//! its first cuboid (`+0x280` too), sequence 1 from a random frame, its wait 1200..3000, 5; its radius 0.4.
//!
//! **Each tick** ([`senses`]): its shadow within 24 of the camera (range 0x16); `+0x2e` = 2 (nothing here sets it):
//! its rider thrown (sequence 9, its `+0x27c` cleared) and it is deleted. A young one: a hit (flags 0x10000) while
//! not hurting nor guarded flees it 6.5 across a line through (604.7, 812.5) ([`flee_point`], `0x414008`) and does
//! 0.3 of its most health, else the record is dropped; its guard down. The frame's damage — any: the flames' count 0,
//! the light freed, the spin's (kind 5) knock 0.8395 up; its rider hurt (2 / 3: sequence 8) or thrown (4 / 5: 9, off
//! it); its group's beasts alarmed (`0x3e6748`); calm (sub 0) it wanders off (8, sub 1, 0x2a0 580..600) or walks its
//! patrol (9). A young one's health back to the most. The two target searches.
//!
//! **Subs** (`+0x95`): 0 calm (grown up; the walk limits; range under 0.2 the stand; with a patrol and no run path
//! off along it, 2): seeing Ratchet (the near area; not in state 0x6f): its run path (0xd, the run limits 60/s² 30/s,
//! accel 1/180, sub 2), else the spot (10, sub 2); alarmed: wandering (8, sub 1; the timer at least 480..600; the
//! patrol: 9); him in its territory: the run. 1 wary (12/s², 6/s): not seeing him with its timer out (and `+0x30c`
//! clear) idle (1, sub 0); seeing him, the spot (10, sub 2). 2 roused (12/s², 6/s): him in its territory, or seen,
//! or in 0xd / 0x10 / 0xc: nothing; in 0xf the stop (0x10, sequence 8 from 1); else back: its patrol (2, sub 1) or
//! wandering (8, sub 1, sequence 3; home its cuboid `+0x2d4`, range at least 4; 0x2a0 1200..1300). 3 the knock (the
//! framework's flight): dead, its bolts and the fade; over, grown up: seeing him the charge (0xb, sub 2) else
//! wandering (8, sub 1; 0x2a0 580..600; the patrol: sequence 4); young: the flight (7, sub 0, its guard 120 ticks).
//!
//! **States**: 1 / 8 the wander (range over 0.2): the walk (sequence 5 / 6) at its speed (1/15 a tick, eased down
//! for the timer's last 20 ticks) with `0x345a58` = crate::moby::ai::wander(0.35, 0.25), the timer out: idle (1 / 0;
//! 3 / 4); idle to its end, the walk again (timer 120..700); a rider without its sequence 3 on a still one: 4. 2 / 3
//! along its patrol either way (`0x350cc0` = crate::moby::ai::follow_path), at its end the turn 4 (sequence 4) and the
//! way back from its next point. 4 the turn's end: 0x15 (sequence 0xf). 0x15 along the patrol at 480/s² 4/s 0.55°
//! 40° while sequence 0xf plays, then 2 / 3. 5 / 6 / 7 the young one: waiting (1200..3000), walking to another of its
//! cuboids (`0x3e37e8`; 1.6/s², 1.6/s) or fleeing ([`stride`], 2.4/s², 2.4/s, its fall at most 30 gravities) until
//! within 1 (or stuck 61 ticks). 9 the patrol's start at its sequence's end (5 → 2). 10 the spot: turning to him
//! (`0x33d5a8`, 0.001 / 0.01 / 0.1); no rider: the charge (0xb) or the trample (0xf, sequence 7); a rider: the aim
//! (0x14). 0xb the charge ([`stride`]): no rider: within 2.7 facing him within 20° the bite (0x11, sequence 0xc);
//! over 2.7 out of the walk sequences 5 / 0x10 sequence 6; a rider: within 20 facing him within 10° the fire's start
//! (0xe). 0xc the stomp's end (accel 1/1200): the charge, or with a rider the fire's start. 0xd along its run path
//! (60/s², 30/s): its end, the aim (0x14). 0xe the fire's start: sequence 9 plays, the nozzle glows ([`nozzle_glow`]);
//! from tick 46 the fire (0x12, sequence 10). 0xf the trample (24/s², 12/s, accel 1/144): toward him, turning to him;
//! within 3.5 the stop (0x10, sequence 8 from 1); a hit at joint 1 (1.1: the trample damage, flags 1, its knock
//! (cos, sin, 2)·2.5; and four times it, flags 0x10000). 0x10 the stop (15/s², 2/s) still toward him with a hit at
//! joint 1 (1.3, flags 0x4000001); at its end the stomp's end (0xc, sequence 2), or within 3 of him the back-off
//! (0x17, sequence 0x11). 0x11 the bite (frames 10..15 at joint 1: 1.0, the bite damage, flags 0x4000001): at its
//! end again facing him within 2.7, else the charge. 0x12 the fire ([`fire`]). 0x13 the turn away (sequence 3): at
//! its end within 25 with a rider the aim, else the charge; within 6 the fire's start. 0x14 the aim: turning to the
//! target point until within 5°, the fire's start. 0x17 the back-off: 5.5 behind it (4/s², 2/s, no turn) for 121
//! ticks, then the charge or the fire's start.
//!
//! **The tail** (`0x3e66a0`): the flash, the timers `+0x2a0` / `+0x2a2` down, its ticks; under z 40 its bolts and gone.
//!
//! Not ported: `0x1c9c40` = 1's rule (hidden in a scene, shown and solid out of it: nothing here sets it); the
//! kind's low half of the fire's hit word (the stack word the template leaves); `+0x30c` is read as the pvar. As in
//! the game, the light word `+0x2e0` starts as the pvar data has it (0 on the placed beasts), so a beast's first hurt
//! frees point-light slot 0; the walk's step past frame 20 (the table's end) is 0.

use crate::hero::physics::{add, approach, atan2, cross, dist2, rot_dist, scale, set_len, sub, V3};
use crate::moby::{ai, Ctx, MobyWorld};
use crate::moby::hits::HitTemplate;
use crate::point_lights::PointLight;
use super::super::props::{seq, set_seq, wrapped};
use super::super::leaper::seq_a;

/// The class.
pub const CLASS: i16 = 0x08ba;

const HEALTH: usize = 0x20;
const FLASH: usize = 0x50;
const REACT: usize = 0x60;
const WANDER: usize = 0x110;
const MV: usize = 0x140;
const TARGET: usize = 0x230;
const T_MOBY: usize = 0x240;
const AREA_NEAR: usize = 0x250;
const AREA_FAR: usize = 0x254;
const SEEN_NEAR: usize = 0x258;
const SEEN_FAR: usize = 0x25c;
const START: usize = 0x260;
const END: usize = 0x270;
const WAY: usize = 0x280;
const RANGE_NEAR: usize = 0x298;
const RANGE_FAR: usize = 0x29c;
const T2A0: usize = 0x2a0;
const T2A2: usize = 0x2a2;
const ALARMED: usize = 0x2a6;
const RIDER: usize = 0x2ac;
const FLAMING: usize = 0x2b0;
const TURN_V: usize = 0x2b4;
const GLOW: usize = 0x2b8;
const W_TIMER: usize = 0x2bc;
const W_RANGE: usize = 0x2c0;
const RUN_PATH: usize = 0x2c4;
const LEN: usize = 0x2c8;
const PATROL: usize = 0x2cc;
const FLAMES: usize = 0x2d0;
const HOME_C: usize = 0x2d4;
const TRAMPLE: usize = 0x2d8;
const TERR: usize = 0x2dc;
const LIGHT: usize = 0x2e0;
const BITE: usize = 0x2e4;
const STOMP: usize = 0x2e8;
const CUBOIDS: usize = 0x2f0;
const WAIT: usize = 0x300;
const AT_C: usize = 0x306;
const GUARD: usize = 0x307;
const BACK: usize = 0x308;
const PVARS: usize = 0x310;

/// The framework's limits (`0x350c50`).
const WALK: [u32; 4] = [0x3b11_a2b4, 0x3d88_8889, 0x3a98_825c, 0x3dd6_7752];
const TROT: [u32; 4] = [0x3b5a_740e, 0x3dcc_ccce, 0x3a98_825c, 0x3dd6_7752];
const RUN: [u32; 4] = [0x3c88_8889, 0x3f00_0000, 0x3a98_825c, 0x3dd6_7752];
const YOUNG_WALK: [u32; 4] = [0x39e9_0453, 0x3cda_740f, 0x3a98_825c, 0x3dd6_7752];
const YOUNG_FLEE: [u32; 4] = [0x3a2e_c33f, 0x3d23_d70b, 0x3a98_825c, 0x3dd6_7752];
const TRAMPLE_L: [u32; 4] = [0x3bda_740e, 0x3e4c_ccce, 0x3a98_825c, 0x3dd6_7752];
const STOP: [u32; 4] = [0x3b88_8889, 0x3d08_8889, 0x3a98_825c, 0x3dd6_7752];
const PATROL_L: [u32; 4] = [0x3e08_8889, 0x3d88_8889, 0x3c1e_dd20, 0x3f32_b8c4];
const BACK_L: [u32; 4] = [0x3a91_a2b4, 0x3d08_8889, 0x3a98_825c, 0x3dd6_7752];

/// `0x2af730`: the walk's step a frame of its sequence (frames 0..20).
const STRIDE: [f32; 21] = [0.075, 0.075, 0.1, 0.1, 0.075, 0.05, 0.075, 0.075, 0.075, 0.075, 0.075, 0.1, 0.1, 0.1, 0.075, 0.03, 0.03, 0.03, 0.03, 0.03, 0.05];
/// The hits' knock (`0x45afdf66`: no knock-back limit).
const NO_LIMIT: f32 = 5_627.987;
/// The fire's FX layers (`0x3e6880` / `0x3e6a90`: texture, half width, colour), additive (ALPHA 0x48).
const FIRE: [(usize, f32, u32); 3] = [(7, 0.1, 0x2f00_00ff), (0x13, 0.05, 0x2fff_ffff), (0x10, 0.05, 0x2fff_ffff)];

fn limits(w: &mut MobyWorld, id: usize, l: [u32; 4]) {
    let [a, b, c, d] = l.map(f32::from_bits);
    ai::senses(&mut w.mobys[id], a, b, c, d);
}

fn first_sub(w: &mut MobyWorld, id: usize) -> bool {
    let m = &mut w.mobys[id];
    if m.once & 2 != 0 { return false; }
    m.once |= 2;
    true
}

fn st(w: &mut MobyWorld, id: usize, state: u8, sub: i32) { w.mobys[id].set_state(state, sub); }
fn rider(w: &MobyWorld, id: usize) -> Option<usize> { ai::link(&w.mobys[id], RIDER).filter(|&r| w.mobys.get(r).is_some_and(|m| m.state < 0x80)) }

/// `0x3e3760(m, s, rider's, frame, blend)`: its sequence `s` unless it plays; with a rider both always, the rider's
/// `rs`.
fn anim2(w: &MobyWorld, id: usize, ctx: &mut Ctx, s: u8, rs: u8, frame: u8, blend: u8) {
    match rider(w, id) {
        None => set_seq(w, id, ctx, s, frame, blend),
        Some(r) => {
            ctx.host.set_anim(&w.mobys[id], s, frame, blend);
            ctx.host.set_anim(&w.mobys[r], rs, frame, blend);
        }
    }
}

fn tpos(w: &MobyWorld, id: usize, ctx: &Ctx) -> V3 {
    match ai::link(&w.mobys[id], T_MOBY) {
        Some(t) if Some(t) != ctx.hero_moby => w.mobys.get(t).map_or(ctx.hero.pos, |m| m.pos),
        _ => ctx.hero.pos,
    }
}
fn yaw_to(w: &MobyWorld, id: usize, p: V3) -> f32 { let q = w.mobys[id].pos; atan2(p[0] - q[0], p[1] - q[1]) }
fn tdist(w: &MobyWorld, id: usize, ctx: &Ctx) -> f32 { dist2(w.mobys[id].pos, tpos(w, id, ctx)) }
/// Its facing's difference from the way to the target point (`+0x230`).
fn off_aim(w: &MobyWorld, id: usize) -> f32 { let tp = w.mobys[id].pv_v(TARGET); rot_dist(w.mobys[id].rot[2], yaw_to(w, id, tp)) }
fn seen(w: &MobyWorld, id: usize) -> bool { w.mobys[id].pv_i(SEEN_NEAR) == 1 || w.mobys[id].pv_i(SEEN_FAR) == 1 }
fn young(w: &MobyWorld, id: usize) -> bool { w.mobys[id].pv_i(CUBOIDS) != -1 }

fn tick_s(w: &mut MobyWorld, id: usize, off: usize) -> bool {
    let m = &mut w.mobys[id];
    let mut t = m.pv_s(off) as i32;
    let r = crate::moby::countdown(&mut t);
    m.set_pv_s(off, t as i16);
    r
}

fn path(ctx: &Ctx, i: i32) -> Option<Vec<V3>> {
    usize::try_from(i).ok().and_then(|p| ctx.level.paths.get(p)).filter(|p| !p.is_empty()).map(|p| p.iter().map(|q| [q[0], q[1], q[2]]).collect())
}

/// `0x350dc8(m, path, last, first)`.
fn path_start(w: &mut MobyWorld, id: usize, ctx: &Ctx, i: i32, last: i32, first: i32) {
    let Some(p) = path(ctx, i) else { return };
    let m = &mut w.mobys[id];
    let o = ai::mv(m);
    m.set_pv_i(o + 0xc4, i + 1);
    m.set_pv_s(o + 0xca, if last == -1 { p.len() as i16 - 1 } else { last as i16 });
    m.set_pv_s(o + 0xc8, first as i16);
}

/// `0x34fa80(m, p)`: the frame's move toward `p`.
fn go(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, p: V3) -> i32 { let y = yaw_to(w, id, p); ai::move_step(w, id, ctx, y, p) }

fn face_target(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let y = yaw_to(w, id, tpos(w, id, ctx));
    let p = w.mobys[id].pos;
    ai::move_step(w, id, ctx, y, p);
}

fn turn_target(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let y = yaw_to(w, id, tpos(w, id, ctx));
    ai::turn_to(&mut w.mobys[id], y, 0.001, 0.01, 0.1, TURN_V);
}

fn in_territory(w: &MobyWorld, id: usize, ctx: &Ctx) -> bool {
    let t = w.mobys[id].pv_i(TERR);
    t != -1 && ctx.level.in_polygon(t, w.mobys[id].pv_v(TARGET))
}

/// `0x3e6748`: its group's live beasts alarmed (`+0x2a6`).
fn alarm(w: &mut MobyWorld, id: usize) {
    let members = usize::try_from(w.mobys[id].group).ok().and_then(|g| w.groups.get(g)).cloned().unwrap_or_default();
    for i in members {
        let m = &mut w.mobys[i];
        if m.state < 0x80 && m.o_class == CLASS { m.set_pv_s(ALARMED, 1); }
    }
}

/// The rider thrown (`0x3e6700`): its sequence 9, its `+0x27c` cleared, no rider.
fn throw_rider(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if let Some(r) = rider(w, id) {
        ctx.host.set_anim(&w.mobys[r], 9, 0, 3);
        w.mobys[r].set_pv_i(0x27c, 0);
    }
    ai::set_link(&mut w.mobys[id], RIDER, None);
}

fn free_light(w: &mut MobyWorld, id: usize) {
    let l = w.mobys[id].pv_i(LIGHT);
    if 0 <= l {
        w.point_lights.free(l as usize);
        w.mobys[id].set_pv_i(LIGHT, -1);
    }
}

/// The flame's light at `p` (one up): red, radius 5.
fn light_at(w: &mut MobyWorld, id: usize, p: V3) {
    let l = w.mobys[id].pv_i(LIGHT);
    if 0 <= l { w.point_lights.set(l as usize, PointLight { color: [1.0, 0.0, 0.0], intensity: 0.0, pos: add(p, [0.0, 0.0, 1.0]), radius: 5.0 }); }
}

/// `0x414008(m, out)`: 6.5 across the line through (604.7, 812.5) along (0.864, 0.503), on its side.
fn flee_point(p: V3) -> V3 {
    let f = [0.864_432, 0.502_75, 0.0];
    let u = set_len([p[0] - 604.7, p[1] - 812.5, 0.0], 1.0);
    let side = if 0.0 < cross(u, f)[2] { 6.5 } else { -6.5 };
    add(scale([-f[1], f[0], 0.0], side), p)
}

/// `0x3e37e8`: another of its four cuboids at random (`+0x306`), its place the goal (`+0x280`).
fn pick_cuboid(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let at = m.pv_b(AT_C) as usize;
    let list: Vec<usize> = (0..4).filter(|&k| k != at && m.pv_i(CUBOIDS + 4 * k) != -1).take(3).collect();
    if list.is_empty() { return; }
    let k = list[ctx.game.rng.rand_range(0, list.len() as i32 - 1) as usize];
    w.mobys[id].set_pv_b(AT_C, k as u8);
    let c = w.mobys[id].pv_i(CUBOIDS + 4 * k);
    if let Some((p, _)) = ctx.level.cuboid_place(c) { w.mobys[id].set_pv_v(WAY, p); }
}

/// `0x3e6fc0(m, goal)`: the walk its sequence drives — settled (not blending), its step for the frame ([`STRIDE`],
/// between frames) along its facing, the move in place toward `goal`, on the ground; at frames 5 and 0xf (feet 3 /
/// 2) ten dust puffs half ahead of the foot, 0.2 lower (`0x359048`: 0.3..1 in size, drifting 0.005..0.05 a tick, grey
/// `0x406080`, 60 ticks). Blending: the framework's move toward `goal`.
fn stride(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, goal: V3) {
    if seq_a(w, id, ctx) != seq(w, id, ctx) {
        go(w, id, ctx, goal);
        return;
    }
    let t = ctx.host.frame_time(&w.mobys[id]);
    let k = t as i32;
    let at = |i: i32| STRIDE.get(i.max(0) as usize).copied().unwrap_or(0.0);
    let step = at(k) + (at(k + 1) - at(k)) * (t - k as f32);
    let m = &mut w.mobys[id];
    m.pos = add(m.pos, scale(m.rows[0], step));
    let y = yaw_to(w, id, goal);
    let p = w.mobys[id].pos;
    ai::move_step(w, id, ctx, y, p);
    let z = ai::ground_z(ctx, 0.5, w.mobys[id].pos, 0);
    w.mobys[id].pos[2] = z;
    let foot = match k { 5 => 3, 0xf => 2, _ => return };
    let Some(j) = ctx.host.joint(&w.mobys[id], foot) else { return };
    let mut p = add(j, scale(w.mobys[id].rows[0], 0.5));
    p[2] -= 0.2;
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let rng = &mut ctx.game.rng;
    for _ in 0..10 {
        let v = rng.rand_vec(f32::from_bits(0x3ba3_d70b), f32::from_bits(0x3d4c_ccce));
        let spin = rng.randf_signed(1.0, 3.0) as i32;
        let size = rng.randf(0.3, 1.0);
        let q = [p[0] + rng.randf_signed(0.0, 0.3), p[1] + rng.randf_signed(0.0, 0.3), p[2] + rng.randf_signed(0.0, 0.3)];
        if let Some(i) = crate::particles::puff(sys, rng, 0.0, 1.0, f32::from_bits(0x3f81_47ae), size * 210000.0, q, spin, v, 0x40_6080, 0) {
            use crate::particles::rec;
            let r = &mut sys.pool.recs[i];
            rec::set_i16(r, 0xa, 0x3c);
            r[3] = 0x44;
            r[9] = 0x44;
            rec::set_ff(r, 0x3c, 5.0);
            rec::set_u32(r, 0x24, 3);
            r[0x2a] = 0x20;
            r[0x2b] = 0x3c;
        }
    }
}

/// The nozzle: the rider's saddle's joint 0, 0.1 ahead along its facing, 0.05 lower.
fn nozzle(w: &MobyWorld, id: usize, ctx: &Ctx) -> Option<V3> {
    let r = rider(w, id)?;
    let s = ai::link(&w.mobys[r], 0x288).filter(|&s| w.mobys.get(s).is_some_and(|m| m.state < 0x80))?;
    let j = ctx.host.joint(&w.mobys[s], 0)?;
    let y = w.mobys[id].rot[2];
    Some([j[0] + y.cos() * 0.1, j[1] + y.sin() * 0.1, j[2] - 0.05])
}

/// The nozzle's glow (states 0xe and 0x12): `size` × 0.2 × 0.2..0.9, two twinkles of two ticks (orange `0x804020ff`
/// and a smaller white `0x7f6f6fff`) spinning opposite ways.
fn nozzle_glow(w: &MobyWorld, id: usize, ctx: &mut Ctx, size: f32) {
    let Some(p) = nozzle(w, id, ctx) else { return };
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let rng = &mut ctx.game.rng;
    let f = rng.randf(0.2, f32::from_bits(0x3f66_6666)) * 0.2 * size;
    let r = rng.randi(2);
    let spin = if r == 0 { -1 } else { r } as i8;
    crate::particles::twinkle(sys, rng, f * 0.3, f, 0.0, p, 2, 0x8040_20ff, 0, spin, [0.0; 3], 0);
    crate::particles::twinkle(sys, rng, f * 0.04, f * 0.4, 0.0, p, 2, 0x7f6f_6fff, 1, -spin, [0.0; 3], 0);
}

/// `0x3e6ce0(at, n)`: as the rider's sparks (crate::moby::classes::level04::rider's `0x452900`) in `0x80ca00cb`.
fn scorch(ctx: &mut Ctx, mut at: V3, n: i32) {
    let cam = ctx.host.camera_pos();
    let mut d = sub(cam, at);
    let off = set_len(d, 0.3);
    d[2] = 0.0;
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let rng = &mut ctx.game.rng;
    for _ in 0..n {
        let f = rng.randf(0.5, 2.0);
        let r = rng.randi(2);
        let spin = if r == 0 { -1 } else { r } as i8;
        let a = rng.randf(-1.047_197_7, 1.047_197_7);
        let (s, c) = a.sin_cos();
        let mut v = set_len([d[0] * c - d[1] * s, d[0] * s + d[1] * c, 0.0], rng.randf(0.05, 3.0) * 0.016_666_668);
        v[2] = rng.randf(1.0, 5.0) * 0.016_666_668;
        let p = add(at, off);
        for k in 0..3 { at[k] += rng.randf_signed(0.0, 0.15); }
        if let Some(i) = crate::particles::twinkle(sys, rng, f * 0.1, f, 0.002, p, 0x3c, 0x80ca_00cb, 0, spin, v, 0) { sys.pool.recs[i][9] = 0x74; }
        if let Some(i) = crate::particles::twinkle(sys, rng, f * 0.07, f * 0.7, 0.002, p, 0x3c, 0x7f7f_7f7f, 1, -spin, v, 0) { sys.pool.recs[i][9] = 0x74; }
    }
}

/// `0x3e60b0` (module docs). True when it is to be deleted.
fn senses(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    if w.mobys[id].state == 0 {
        let z = ai::ground_z(ctx, 0.5, w.mobys[id].pos, 0);
        let m = &mut w.mobys[id];
        if m.pvars.len() < PVARS { m.pvars.resize(PVARS, 0); }
        m.set_pv_i(0x18, MV as i32);
        m.pos[2] = z;
        m.set_pv_i(FLAMES, 0);
        m.set_pv_b(0x48, 20);
        m.set_pv_i(REACT + 0x48, 3);
        m.set_pv_f(REACT + 0x30, 0.5);
        m.set_pv_f(REACT + 0x34, 0.5);
        m.set_pv_f(REACT + 0x3c, f32::from_bits(0x3ecc_cccd));
        m.set_pv_b(REACT + 0x77, 1);
        m.set_pv_f(MV + 8, 0.5);
        m.set_pv_f(MV + 0xc, 0.5);
        for k in [0x73, 0x6c, 0x6d, 0x6e, 0x6f, 0x72] { m.set_pv_b(REACT + k, 0xff); }
        m.set_pv_b(0x4a, 11);
        m.set_pv_b(REACT + 0x10, 0x16);
        m.set_pv_b(REACT + 0x12, 0xb);
        m.set_pv_b(REACT + 0x16, 0xd);
        m.set_pv_b(REACT + 0x17, 0xe);
        m.set_pv_f(REACT + 0x1c, f32::from_bits(0x3dcc_ccce));
        m.set_pv_f(REACT + 0x28, f32::from_bits(0x3c08_8889));
        m.set_pv_f(REACT + 0x2c, f32::from_bits(0x3b88_8889));
        m.set_pv_f(MV, 1.0);
        m.set_pv_f(MV + 4, 0.25);
        m.set_pv_f(MV + 0x10, f32::from_bits(0x3fbd_e450));
        m.set_pv_f(MV + 0x30, f32::from_bits(0x3f06_0a93));
        m.set_pv_f(MV + 0x2c, f32::from_bits(0x3bb6_0b61));
        m.set_pv_b(REACT + 0x11, 0xb);
        m.set_pv_f(REACT + 0x20, f32::from_bits(0x3d88_8889));
        m.set_pv_f(REACT + 0x24, f32::from_bits(0x3b88_8889));
        limits(w, id, WALK);
        let m = &mut w.mobys[id];
        let pos = m.pos;
        m.set_pv_v(WANDER, pos);
        m.set_pv_f(WANDER + 0x10, 1.25);
        m.set_pv_f(WANDER + 0x1c, m.pv_f(W_RANGE));
        m.set_pv_f(WANDER + 0x20, 10.0);
        let t = ctx.game.rng.rand_range(0x78, 700);
        let m = &mut w.mobys[id];
        m.set_pv_b(0x28, 3);
        m.set_pv_b(0x2e, 1);
        m.set_pv_b(0x29, 1);
        m.set_pv_i(W_TIMER, t);
        let d = m.pv_b(0x34) as f32;
        m.set_pv_f(BITE, d);
        m.set_pv_f(STOMP, d);
        m.set_pv_f(0x30, 0.75);
        st(w, id, 1, 0);
        if !young(w, id) {
            let frames = ctx.host.seq_frames(&w.mobys[id], 4).unwrap_or(0) as i32;
            let f = ctx.game.rng.randi(frames);
            anim2(w, id, ctx, 4, 1, f as u8, 0);
            let s = ctx.game.rng.randf(0.5, 1.25);
            ctx.host.set_anim_speed(&w.mobys[id], s);
        } else {
            let k = w.class(CLASS).map_or(1.0, |c| c.scale);
            let c0 = w.mobys[id].pv_i(CUBOIDS);
            let m = &mut w.mobys[id];
            m.scale = k * 0.4;
            m.set_pv_b(AT_C, 0);
            if let Some((p, _)) = ctx.level.cuboid_place(c0) {
                m.pos = p;
                m.set_pv_v(WAY, p);
            }
            let frames = ctx.host.seq_frames(&w.mobys[id], 1).unwrap_or(0) as i32;
            let f = ctx.game.rng.randi(frames);
            anim2(w, id, ctx, 1, 1, f as u8, 10);
            let t = ctx.game.rng.rand_range(0x4b0, 3000);
            let m = &mut w.mobys[id];
            m.set_pv_s(WAIT, t as i16);
            st(w, id, 5, 0);
            let m = &mut w.mobys[id];
            let r = m.pv_f(MV) * 0.4;
            m.set_pv_b(GUARD, 0);
            m.set_pv_f(MV, r);
            m.set_pv_f(MV + 4, r + 0.01);
        }
    }
    crate::shadow_volumes::probe_near(w, id, ctx, 24.0, 0x16);
    if w.mobys[id].pv_b(0x2e) == 2 {
        throw_rider(w, id, ctx);
        return true;
    }
    if young(w, id) {
        let hit = w.hit_message(id).filter(|h| h.tmpl.flags & 0x10000 != 0);
        if w.mobys[id].state == 0x16 || w.mobys[id].pv_b(GUARD) != 0 {
            w.clear_hit(id);
        } else if hit.is_some() {
            let p = flee_point(w.mobys[id].pos);
            let most = w.mobys[id].pv_s(HEALTH + 4) as f32 * 0.3;
            w.mobys[id].set_pv_v(WAY, p);
            if let Ok(slot) = usize::try_from(w.mobys[id].hit_slot) { w.hits.records[slot].tmpl.damage = most; }
        }
        let g = w.mobys[id].pv_b(GUARD);
        if g != 0 { w.mobys[id].set_pv_b(GUARD, g - 1); }
    }
    let kind = ai::take_damage(w, id, ctx, false);
    if kind != 0 {
        w.mobys[id].set_pv_i(FLAMES, 0);
        free_light(w, id);
        if kind == 5 {
            let m = &mut w.mobys[id];
            m.set_pv_f(REACT + 0x90, 0.0);
            m.set_pv_f(REACT + 0x94, f32::from_bits(0x3f56_7752));
        }
        if let Some(r) = rider(w, id) {
            match kind {
                2 | 3 => ctx.host.set_anim(&w.mobys[r], 8, 0, 3),
                4 | 5 => {
                    ctx.host.set_anim(&w.mobys[r], 9, 0, 3);
                    w.mobys[r].set_pv_i(0x27c, 0);
                }
                _ => {}
            }
        }
        alarm(w, id);
        if w.mobys[id].sub_state == 0 {
            if w.mobys[id].pv_i(PATROL) == -1 {
                st(w, id, 8, 1);
                let t = ctx.game.rng.rand_range(0x244, 600);
                w.mobys[id].set_pv_s(T2A0, t as i16);
                anim2(w, id, ctx, 6, 5, 0, 10);
            } else {
                st(w, id, 9, 1);
                anim2(w, id, ctx, 4, 3, 0, 0xc);
            }
        }
    }
    if young(w, id) {
        let m = &mut w.mobys[id];
        m.set_pv_f(HEALTH, m.pv_s(HEALTH + 4) as f32);
    }
    let (rn, an) = (w.mobys[id].pv_f(RANGE_NEAR), w.mobys[id].pv_i(AREA_NEAR));
    let a = ai::find_target(w, id, ctx, rn, 10.0, an, None, false);
    w.mobys[id].set_pv_i(SEEN_NEAR, a as i32);
    let (rf, af) = (w.mobys[id].pv_f(RANGE_FAR), w.mobys[id].pv_i(AREA_FAR));
    let b = ai::find_target(w, id, ctx, rf, 10.0, af, None, false);
    w.mobys[id].set_pv_i(SEEN_FAR, b as i32);
    false
}

/// The spot (10, sub 2): without a rider its sequence 0 first (`0x3e3760(m, 0, 2, 0, 10)`).
fn spot(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if rider(w, id).is_none() { anim2(w, id, ctx, 0, 2, 0, 10); }
    st(w, id, 10, 2);
}

/// The run along its path (0xd, sub 2).
fn run(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    anim2(w, id, ctx, 6, 5, 0, 10);
    limits(w, id, RUN);
    w.mobys[id].set_pv_f(MV + 0x24, f32::from_bits(0x3bb6_0b61));
    st(w, id, 0xd, 2);
}

/// The charge (0xb) or with a rider the fire's start (0xe), after the limits.
fn charge_or_fire(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].set_pv_f(MV + 0x24, f32::from_bits(0x3a5a_740e));
    if rider(w, id).is_none() {
        limits(w, id, TROT);
        anim2(w, id, ctx, 6, 5, 0, 10);
        st(w, id, 0xb, -1);
    } else {
        limits(w, id, WALK);
        anim2(w, id, ctx, 9, 6, 0, 10);
        st(w, id, 0xe, -1);
    }
}

/// 0x08ba (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if senses(w, id, ctx) {
        w.delete(id);
        return;
    }
    match w.mobys[id].sub_state {
        0 => if !young(w, id) {
            if first_sub(w, id) {
                limits(w, id, WALK);
                if w.mobys[id].pv_f(W_RANGE) < 0.2 { anim2(w, id, ctx, 4, 3, 0, 0xc); }
                if w.mobys[id].pv_i(PATROL) != -1 && w.mobys[id].pv_i(RUN_PATH) == -1 {
                    anim2(w, id, ctx, 5, 4, 0, 0xc);
                    st(w, id, 2, -1);
                }
            }
            let mut check_alarm = true;
            if w.mobys[id].pv_i(SEEN_NEAR) == 1 && ctx.hero.state != 0x6f {
                check_alarm = false;
                if w.mobys[id].pv_i(RUN_PATH) != -1 { run(w, id, ctx) } else { spot(w, id, ctx) }
            }
            if check_alarm {
                if w.mobys[id].pv_s(ALARMED) == 0 {
                    if in_territory(w, id, ctx) { run(w, id, ctx); }
                } else {
                    st(w, id, 8, 1);
                    w.mobys[id].set_pv_s(ALARMED, 0);
                    if w.mobys[id].pv_s(T2A0) < 300 {
                        let t = ctx.game.rng.rand_range(0x1e0, 600);
                        w.mobys[id].set_pv_s(T2A0, t as i16);
                    }
                    if w.mobys[id].pv_i(PATROL) != -1 {
                        st(w, id, 8, 1);
                        anim2(w, id, ctx, 4, 3, 0, 0xc);
                    }
                }
            }
        },
        1 => {
            if first_sub(w, id) { limits(w, id, TROT); }
            if seen(w, id) {
                spot(w, id, ctx);
            } else if w.mobys[id].pv_s(T2A0) == 0 && w.mobys[id].pv_i(0x30c) == 0 {
                st(w, id, 1, 0);
                anim2(w, id, ctx, 5, 4, 0, 0xc);
            }
        }
        2 => {
            if first_sub(w, id) { limits(w, id, TROT); }
            let state = w.mobys[id].state;
            if in_territory(w, id, ctx) || seen(w, id) || matches!(state, 0xd | 0x10 | 0xc) {
            } else if state == 0xf {
                anim2(w, id, ctx, 8, 0, 1, 3);
                st(w, id, 0x10, -1);
            } else if w.mobys[id].pv_i(PATROL) != -1 && w.mobys[id].pv_i(RUN_PATH) == -1 {
                anim2(w, id, ctx, 5, 4, 0, 0xc);
                st(w, id, 2, 1);
            } else {
                w.mobys[id].set_pv_i(FLAMES, 0);
                anim2(w, id, ctx, 3, 1, 0, 0xc);
                st(w, id, 8, 1);
                let hc = w.mobys[id].pv_i(HOME_C);
                if hc != -1 {
                    if let Some((p, _)) = ctx.level.cuboid_place(hc) {
                        let m = &mut w.mobys[id];
                        m.set_pv_v(WANDER, p);
                        if m.pv_f(WANDER + 0x1c) == 0.0 { m.set_pv_f(WANDER + 0x1c, 4.0); }
                    }
                }
                let t = ctx.game.rng.rand_range(0x4b0, 0x514);
                w.mobys[id].set_pv_s(T2A0, t as i16);
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
                if !young(w, id) {
                    if seen(w, id) {
                        anim2(w, id, ctx, 6, 5, 0, 10);
                        st(w, id, 0xb, 2);
                    } else {
                        st(w, id, 8, 1);
                        anim2(w, id, ctx, 1, 1, 0, 10);
                        let t = ctx.game.rng.rand_range(0x244, 600);
                        w.mobys[id].set_pv_s(T2A0, t as i16);
                        if w.mobys[id].pv_i(PATROL) != -1 {
                            st(w, id, 8, 1);
                            anim2(w, id, ctx, 4, 3, 0, 0xc);
                        }
                    }
                } else {
                    anim2(w, id, ctx, 6, 5, 0, 10);
                    st(w, id, 7, 0);
                    w.mobys[id].set_pv_b(GUARD, 0x78);
                }
            }
        }
        _ => {}
    }
    if w.mobys[id].state >= 0x80 { return; }
    states(w, id, ctx);
    if w.mobys[id].state >= 0x80 { return; }
    let m = &mut w.mobys[id];
    ai::flash(m, FLASH);
    tick_s(w, id, T2A0);
    tick_s(w, id, T2A2);
    let m = &mut w.mobys[id];
    if m.state_ticks != -1 { m.state_ticks += 1; }
    if m.pos[2] < 40.0 {
        crate::moby::bolts::drop(w, id, 0, 3, ctx);
        w.delete(id);
    }
}

/// The wander's states 1 and 8: `walk` the walking sequence (and the rider's), `idles` the two idles (and the
/// rider's), `step` the speed's ease.
fn wander(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, walk: (u8, u8), idles: [(u8, u8); 2], step: f32, ease_only: bool) {
    let mut t = w.mobys[id].pv_i(W_TIMER);
    crate::moby::countdown(&mut t);
    w.mobys[id].set_pv_i(W_TIMER, t);
    let m = &mut w.mobys[id];
    let mut v = m.pv_f(WANDER + 0x14);
    if t < 0x15 {
        approach(0.0, step, &mut v);
    } else if ease_only {
        approach(f32::from_bits(0x3d88_8889), step, &mut v);
    } else {
        v = f32::from_bits(0x3d88_8889);
    }
    m.set_pv_f(WANDER + 0x14, v);
    let s = seq(w, id, ctx);
    if s == walk.0 {
        if t == 0 {
            anim2(w, id, ctx, idles[0].0, idles[0].1, 0, 0xc);
            if ctx.game.rng.randi(2) == 1 { anim2(w, id, ctx, idles[1].0, idles[1].1, 0, 0xc); }
        }
        ai::wander(w, id, ctx, f32::from_bits(0x3eb3_3333), 0.25, WANDER);
    } else if (s == idles[0].0 || s == idles[1].0) && wrapped(w, id, ctx) {
        let t = ctx.game.rng.rand_range(0x78, 700);
        w.mobys[id].set_pv_i(W_TIMER, t);
        anim2(w, id, ctx, walk.0, walk.1, 0, 0xc);
    }
}

/// The second switch, on the state (module docs).
fn states(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        1 => {
            if 0.2 < w.mobys[id].pv_f(W_RANGE) {
                wander(w, id, ctx, (5, 4), [(1, 1), (0, 0)], f32::from_bits(0x3a91_a2b4), false);
            } else if let Some(r) = rider(w, id) {
                if seq(w, r, ctx) != 3 { anim2(w, id, ctx, 4, 3, 0, 0xc); }
            }
        }
        2 | 3 => {
            let back = w.mobys[id].state == 2;
            let p = w.mobys[id].pv_i(PATROL);
            if ai::follow_path(w, id, ctx, p) {
                st(w, id, 4, -1);
                anim2(w, id, ctx, 4, 3, 0, 10);
                if back { path_start(w, id, ctx, p, 0, 1) } else { path_start(w, id, ctx, p, 1, 0) }
                w.mobys[id].set_pv_i(BACK, back as i32);
            }
        }
        4 => if wrapped(w, id, ctx) {
            anim2(w, id, ctx, 0xf, 0x17, 0, 7);
            st(w, id, 0x15, -1);
        },
        5 => if tick_s(w, id, WAIT) {
            anim2(w, id, ctx, 5, 4, 0, 10);
            pick_cuboid(w, id, ctx);
            st(w, id, 6, 0);
        },
        6 | 7 => {
            let flee = w.mobys[id].state == 7;
            let goal = w.mobys[id].pv_v(WAY);
            if flee {
                limits(w, id, YOUNG_FLEE);
                stride(w, id, ctx, goal);
                let m = &mut w.mobys[id];
                let most = m.pv_f(MV + 0x2c) * 30.0;
                if most < m.pv_f(MV + 0xa8) { m.set_pv_f(MV + 0xa8, most); }
            } else {
                limits(w, id, YOUNG_WALK);
                go(w, id, ctx, goal);
            }
            let d = dist2(w.mobys[id].pos, goal);
            if d < 1.0 || (flee && 0x3d <= w.mobys[id].pv_i(MV + 0x80)) {
                anim2(w, id, ctx, 1, 1, 0, 10);
                let t = ctx.game.rng.rand_range(0x4b0, 3000);
                w.mobys[id].set_pv_s(WAIT, t as i16);
                st(w, id, 5, -1);
            }
        }
        8 => wander(w, id, ctx, (6, 5), [(3, 2), (4, 3)], f32::from_bits(0x3afe_dcbb), true),
        9 => if wrapped(w, id, ctx) {
            anim2(w, id, ctx, 5, 4, 0, 0xc);
            st(w, id, 2, -1);
        },
        10 => {
            turn_target(w, id, ctx);
            if rider(w, id).is_none() {
                if w.mobys[id].pv_i(TRAMPLE) == 0 {
                    st(w, id, 0xb, -1);
                    anim2(w, id, ctx, 6, 5, 0, 10);
                } else {
                    st(w, id, 0xf, -1);
                    anim2(w, id, ctx, 7, 5, 0, 10);
                }
            } else {
                st(w, id, 0x14, -1);
                anim2(w, id, ctx, 0x10, 4, 0, 10);
            }
        }
        0xb => {
            let tp = w.mobys[id].pv_v(TARGET);
            stride(w, id, ctx, tp);
            let m = &mut w.mobys[id];
            m.set_pv_i(FLAMING, m.pv_i(FLAMING) + 1);
            if rider(w, id).is_none() {
                if tdist(w, id, ctx) < 2.7 && off_aim(w, id) < 0.349_065_9 {
                    anim2(w, id, ctx, 0xc, 0, 0, 5);
                    st(w, id, 0x11, -1);
                }
                let a = seq_a(w, id, ctx);
                if 2.7 < tdist(w, id, ctx) && (a == 5 || a == 0x10) {
                    anim2(w, id, ctx, 6, 5, 0, 4);
                    limits(w, id, TROT);
                }
            } else if tdist(w, id, ctx) < 20.0 && off_aim(w, id) < 0.174_532_95 {
                anim2(w, id, ctx, 9, 6, 0, 10);
                st(w, id, 0xe, -1);
            }
        }
        0xc => if wrapped(w, id, ctx) { charge_or_fire(w, id, ctx) },
        0xd => {
            limits(w, id, TROT);
            let p = w.mobys[id].pv_i(RUN_PATH);
            if ai::follow_path(w, id, ctx, p) {
                st(w, id, 0x14, -1);
                anim2(w, id, ctx, 0x10, 4, 0, 10);
                limits(w, id, TROT);
                let m = &mut w.mobys[id];
                m.set_pv_i(TERR, -1);
                m.set_pv_f(MV + 0x24, f32::from_bits(0x3a5a_740e));
                m.set_pv_i(RUN_PATH, -1);
            }
        }
        0xe => {
            if rider(w, id).is_some() && seq_a(w, id, ctx) == 9 {
                let t = ctx.host.frame_time(&w.mobys[id]);
                nozzle_glow(w, id, ctx, t);
                w.mobys[id].set_pv_f(GLOW, t);
            }
            if 0x2e <= w.mobys[id].state_ticks as u16 {
                anim2(w, id, ctx, 10, 7, 0, 10);
                st(w, id, 0x12, -1);
            }
        }
        0xf => {
            w.mobys[id].set_pv_f(MV + 0x24, f32::from_bits(0x3be3_8e39));
            limits(w, id, TRAMPLE_L);
            let tp = w.mobys[id].pv_v(TARGET);
            go(w, id, ctx, tp);
            turn_target(w, id, ctx);
            let m = &mut w.mobys[id];
            m.set_pv_i(FLAMING, m.pv_i(FLAMING) + 1);
            if tdist(w, id, ctx) < 3.5 {
                anim2(w, id, ctx, 8, 0, 1, 3);
                ctx.host.set_anim_speed(&w.mobys[id], 1.0);
                st(w, id, 0x10, -1);
            }
            let dmg = w.mobys[id].pv_f(STOMP);
            trample_hit(w, id, ctx, 1.1, dmg, 1);
            if let Some(j) = ctx.host.joint(&w.mobys[id], 1) { ai::attack_sphere(w, id, ctx, 1.1, dmg * 4.0, 0.0, j, 0x10000, 0, 1, 0); }
        }
        0x10 => {
            limits(w, id, STOP);
            let tp = w.mobys[id].pv_v(TARGET);
            go(w, id, ctx, tp);
            let m = &mut w.mobys[id];
            m.set_pv_i(FLAMING, m.pv_i(FLAMING) + 1);
            if wrapped(w, id, ctx) {
                anim2(w, id, ctx, 2, 2, 0, 10);
                st(w, id, 0xc, -1);
                if dist2(w.mobys[id].pos, ctx.hero.pos) < 3.0 {
                    limits(w, id, WALK);
                    set_seq(w, id, ctx, 0x11, 0, 10);
                    st(w, id, 0x17, -1);
                }
            }
            let dmg = w.mobys[id].pv_f(STOMP);
            trample_hit(w, id, ctx, f32::from_bits(0x3fa6_6666), dmg, 0x0400_0001);
        }
        0x11 => {
            let t = ctx.host.frame_time(&w.mobys[id]);
            if seq_a(w, id, ctx) == 0xc && (10.0..=15.0).contains(&t) {
                let d = w.mobys[id].pv_f(BITE);
                ai::attack_joint(w, id, ctx, 1.0, d, 1.0, 1, 0x0400_0001, 0, 1, 0);
            }
            if wrapped(w, id, ctx) {
                if tdist(w, id, ctx) < 2.7 && off_aim(w, id) < 0.349_065_9 {
                    anim2(w, id, ctx, 0xc, 0, 0, 10);
                    st(w, id, 0x11, -1);
                } else {
                    st(w, id, 0xb, -1);
                    anim2(w, id, ctx, 6, 5, 0, 10);
                    limits(w, id, TROT);
                }
            }
        }
        0x12 => fire(w, id, ctx),
        0x13 => {
            if wrapped(w, id, ctx) {
                if 25.0 <= tdist(w, id, ctx) || rider(w, id).is_none() {
                    st(w, id, 0xb, -1);
                    anim2(w, id, ctx, 6, 5, 0, 10);
                } else {
                    st(w, id, 0x14, -1);
                    anim2(w, id, ctx, 5, 4, 0, 10);
                }
            } else if tdist(w, id, ctx) < 6.0 {
                anim2(w, id, ctx, 9, 6, 0, 10);
                st(w, id, 0xe, -1);
            }
        }
        0x14 => {
            if 0.087_266_475 < off_aim(w, id) {
                face_target(w, id, ctx);
            } else {
                anim2(w, id, ctx, 9, 6, 0, 10);
                st(w, id, 0xe, -1);
            }
        }
        0x15 => {
            if !wrapped(w, id, ctx) {
                if seq(w, id, ctx) == 0xf {
                    limits(w, id, PATROL_L);
                    let p = w.mobys[id].pv_i(PATROL);
                    ai::follow_path(w, id, ctx, p);
                }
            } else {
                anim2(w, id, ctx, 5, 4, 0, 0);
                let s = if w.mobys[id].pv_i(BACK) == 0 { 2 } else { 3 };
                st(w, id, s, -1);
            }
        }
        0x17 => {
            limits(w, id, BACK_L);
            let m = &w.mobys[id];
            let goal = add(scale(m.rows[0], -5.5), m.pos);
            let y = yaw_to(w, id, goal);
            let o = MV + 0x38;
            let kept = w.mobys[id].pv_i(o);
            w.mobys[id].set_pv_i(o, 0x10);
            ai::move_step(w, id, ctx, y, goal);
            w.mobys[id].set_pv_i(o, kept);
            if 0x79 <= w.mobys[id].state_ticks as u16 { charge_or_fire(w, id, ctx); }
        }
        _ => {}
    }
}

/// The trample's and the stop's hit at joint 1 (`0x2e14b0(r, p, 0, m, t)`): `damage`, `flags`, its knock (cos, sin,
/// 2)·2.5 without a limit, its kind its class's `<< 16 | 0x100`.
fn trample_hit(w: &mut MobyWorld, id: usize, ctx: &Ctx, r: f32, damage: f32, flags: u32) {
    let Some(j) = ctx.host.joint(&w.mobys[id], 1) else { return };
    let m = &w.mobys[id];
    let y = m.rot[2];
    let mut t = HitTemplate::new(damage, Some(id), flags, [y.cos() * 2.5, y.sin() * 2.5, 5.0, NO_LIMIT]);
    t.kind = (m.o_class as u16 as u32) << 16 | 0x100;
    w.sphere_hit(ctx.scene, j, r, 0, Some(id), Some(&t));
}

/// The flame's step (`0x2c8` one longer along its way): its end, the line to it (`0x2ff080`, the flame's damage
/// `+0x35`, flags 0x4000001, its way the knock) to the first thing it meets, which takes the hit and ends it there —
/// the first time a scorch of 30 sparks ([`scorch`]); then the end's fire twinkles (`0xff000080`, `0x7f6f6fff`,
/// `0xff4080ff`: 25 / 5 / 20 ticks), its draw, the light at the end.
fn flame_step(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> (f32, i8) {
    let m = &mut w.mobys[id];
    let l = m.pv_f(LEN) + 1.0;
    m.set_pv_f(LEN, l);
    let v = set_len(m.pv_v(WAY), l);
    let dmg = m.pv_b(0x35) as f32;
    let mut t = HitTemplate::new(dmg, Some(id), 0x0400_0001, [v[0], v[1], v[2], 0.0]);
    t.kind = (m.o_class as u16 as u32) << 16;
    let start = m.pv_v(START);
    let mut end = add(start, v);
    if let Some(o) = w.coll_line(ctx, start, end, 0, Some(id)) {
        end = o.point;
        if let Some(target) = o.moby { w.deliver_hit_any(target, &t, [o.point[0], o.point[1], o.point[2], 1.0]); }
        if w.mobys[id].once & 4 == 0 {
            scorch(ctx, o.point, 0x1e);
            w.mobys[id].once |= 4;
        }
    }
    w.mobys[id].set_pv_v(END, end);
    let rng = &mut ctx.game.rng;
    let f = rng.randf(1.0, 6.0);
    let spin = rng.rand_range(0x19, 0x23) as i8;
    let at = [end[0], end[1], end[2] - 0.1];
    if let Some(sys) = ctx.parts.as_deref_mut() {
        crate::particles::twinkle(sys, rng, f * 0.3, f, 0.0, at, 0x19, 0xff00_0080, 0, spin, [0.0; 3], 0);
        crate::particles::twinkle(sys, rng, f * 0.04, f * 0.4, 0.0, at, 5, 0x7f6f_6fff, 1, -spin, [0.0; 3], 0);
        crate::particles::twinkle(sys, rng, f * 0.4, f * 0.2, 0.0, at, 0x14, 0xff40_80ff, 1, -spin, [0.0; 3], 0);
    }
    (f, spin)
}

/// State 0x12, the fire (`0x3e38e8`'s case 0x12). Before its sequence's end: at frame 3 with a rider the flame
/// starts at the nozzle, its way to the target point (or, facing more than 45° off it, 25 straight ahead), the light
/// taken (`0x31e468`: red, radius 5); from frame 3 to 9 of sequence 10 it grows ([`flame_step`], and two twinkles at
/// the nozzle, `0x80ca00cb` / `0x7f6f6fff`, 10 ticks), from 9 to 15 its tail follows (1.667 a tick) while it is more
/// than 0.5 long, then (or past 15) the light goes; out of the growth the nozzle glows. At its end: within 25 with a
/// rider and two flames or fewer since the run, the aim (0x14) or the fire's start (within 5°); else within 10, no
/// rider or fewer than three flames the aim, else the turn away (0x13, the count 0); no rider: the charge; else one
/// flame more.
fn fire(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if !wrapped(w, id, ctx) {
        if ai::reached(w, id, ctx, 3.0) {
            if let Some(p) = nozzle(w, id, ctx) {
                let m = &w.mobys[id];
                let to = if off_aim(w, id) < 0.785_398_24 { m.pv_v(TARGET) } else { let y = m.rot[2]; add([y.cos() * 25.0, y.sin() * 25.0, 0.0], m.pos) };
                let m = &mut w.mobys[id];
                m.set_pv_v(START, p);
                m.set_pv_v(WAY, sub(to, p));
                m.set_pv_f(LEN, 0.0);
                m.once &= !4;
                let l = w.point_lights.alloc(PointLight { color: [1.0, 0.0, 0.0], intensity: 0.0, pos: add(p, [0.0, 0.0, 1.0]), radius: 5.0 }).map_or(-1, |s| s as i32);
                w.mobys[id].set_pv_i(LIGHT, l);
            }
        }
    } else {
        let d = tdist(w, id, ctx);
        let has = rider(w, id).is_some();
        if d < 25.0 && has && w.mobys[id].pv_i(FLAMES) <= 2 {
            if 0.087_266_475 < off_aim(w, id) {
                st(w, id, 0x14, -1);
                anim2(w, id, ctx, 0x10, 4, 0, 10);
            } else {
                anim2(w, id, ctx, 9, 6, 0, 10);
                st(w, id, 0xe, -1);
            }
        } else if d <= 10.0 || !has || w.mobys[id].pv_i(FLAMES) < 3 {
            st(w, id, 0x14, -1);
            anim2(w, id, ctx, 0x10, 4, 0, 10);
        } else {
            anim2(w, id, ctx, 3, 2, 0, 10);
            st(w, id, 0x13, -1);
            w.mobys[id].set_pv_i(FLAMES, 0);
        }
        if !has {
            st(w, id, 0xb, -1);
            anim2(w, id, ctx, 6, 5, 0, 10);
            return;
        }
        let m = &mut w.mobys[id];
        m.set_pv_i(FLAMES, m.pv_i(FLAMES) + 1);
    }
    let t = ctx.host.frame_time(&w.mobys[id]);
    let a = seq_a(w, id, ctx);
    if (t < 3.0 || a != 10) && rider(w, id).is_some() {
        let g = w.mobys[id].pv_f(GLOW);
        nozzle_glow(w, id, ctx, g);
        w.mobys[id].set_pv_f(GLOW, g + 1.0);
    }
    if (3.0..9.0).contains(&t) && a == 10 {
        let (f, spin) = flame_step(w, id, ctx);
        let start = w.mobys[id].pv_v(START);
        if let Some(sys) = ctx.parts.as_deref_mut() {
            let rng = &mut ctx.game.rng;
            crate::particles::twinkle(sys, rng, f * 0.3, f, 0.0, start, 10, 0x80ca_00cb, 0, -spin, [0.0; 3], 0);
            crate::particles::twinkle(sys, rng, f * 0.04, f * 0.4, 0.0, start, 10, 0x7f6f_6fff, 1, spin, [0.0; 3], 0);
        }
        w.register_draw(id);
        let e = w.mobys[id].pv_v(END);
        light_at(w, id, e);
    }
    if (9.0..15.0).contains(&t) && a == 10 {
        let m = &w.mobys[id];
        if 0.5 < dist2(m.pv_v(START), m.pv_v(END)) {
            flame_step(w, id, ctx);
            let m = &mut w.mobys[id];
            let s = add(m.pv_v(START), set_len(m.pv_v(WAY), f32::from_bits(0x3fd5_5556)));
            m.set_pv_v(START, s);
            w.register_draw(id);
            let e = w.mobys[id].pv_v(END);
            light_at(w, id, e);
            return;
        }
    } else if t <= 15.0 || a != 10 {
        return;
    }
    free_light(w, id);
}

/// `0x3e67c8` (with a rider): the flame from `+0x260` to `+0x270` in its three layers ([`FIRE`]), each a quad across
/// the view at both ends.
pub fn draw(w: &MobyWorld, id: usize, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    if rider(w, id).is_none() { return; }
    let m = &w.mobys[id];
    let (a, b) = (m.pv_v(START), m.pv_v(END));
    let d = sub(a, b);
    let st = [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]];
    for (fx, half, rgba) in FIRE {
        let sa = set_len(cross(sub(a, cam), d), half);
        let sb = set_len(cross(sub(b, cam), d), half);
        out.push(crate::fx::FxQuad { fx, additive: true, corners: [sub(a, sa), add(a, sa), sub(b, sb), add(b, sb)], st, rgba: [rgba; 4] });
    }
}
