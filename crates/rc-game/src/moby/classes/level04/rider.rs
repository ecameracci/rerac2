//! Barlow's desert rider **0x0ff8** (L04 `0x44f2c8`, 49 placed; its senses `0x451a80`, tail `0x4527c8`), on the
//! framework ([`crate::moby::ai`]): health block `+0x20`, the colour flash `+0x50`, reaction `+0x60` (a hurt goes to
//! state 0x19, sub 4), the wander block `+0x110`, movement `+0x140`, target `+0x230` (its moby `+0x240`). Some ride
//! the beasts 0x08ba (crate::moby::classes::level04::beast; the game's own message for a missing one: "There is a
//! rider who is supposed to be on a beast").
//!
//! Its words: the target areas `+0x250` / `+0x254` and what they found `+0x258` / `+0x25c`, the near and far paths
//! `+0x260` / `+0x264` and ranges `+0x268` / `+0x26c` (15 / 20) of the range test (`+0x274`, [`range_test`],
//! `0x34cb40`), timers `+0x270` / `+0x272`, alarmed `+0x276`; riding `+0x27c` on the beast `+0x280` (kept at
//! `+0x284`), the saddle part 0x0ff9 `+0x288`, the turn speed `+0x28c`, the seat target 0x0911 `+0x290`; the wander
//! timer `+0x294` and range `+0x298`, its path in `+0x29c` (−1 none), the attack flag `+0x2a0` (the damage intake's
//! flag), the charge `+0x2a4`, its yaw offset `+0x2a8` (degrees, then radians), the weapon part 0x09ac `+0x2ac`, the
//! alarm wait `+0x2b0`, `+0x2b4` a nimbler kind, its fence `+0x2b8`, its damage `+0x2bc`; its path network `+0x340`
//! (16 records, crate::path_network, the count `+0x540`, from the path `+0x544`, used when `+0x548` ≠ 0, the records
//! it and its goal are in `+0x550` / `+0x554`, the walk speed `+0x558`, stuck ticks `+0x55c`); `+0x560` sure-footed,
//! the chat `+0x564` (1 / 2 / 3) with the moby `+0x568` (kept at `+0x56c`), its patrol path `+0x570` and the alarms
//! counted `+0x574`, the path to go to `+0x578`.
//!
//! **The set-up** (state 0, in [`senses`]): on the ground; sure-footed it is hidden without collision; with the
//! network it builds it (`0x34cc90` = crate::path_network::build, its openings kept); its yaw offset (none: ±15..25°;
//! nimbler ±5..30°) in radians; mirrored at random (mode 0x8000 and the once bit 2, `0x34d778`); the reaction (state
//! 0x19 sub 4, sequences 0x15 / 0x10 / 0x12 / 0x13, speeds 0.2 / 0.125, gravities 1/180 / 7/720 / 1/240, keys, the
//! hurt share 0.18), movement (radius / height 0.75, steps 0.4 (nimbler 0.6; sure-footed 0.7, slope 75°), slope 55°,
//! gravity 1/720, the turn before walking 85°, stop 1.5, its fence `+0x2b8`), the calm limits ([`CALM`]), its damage
//! the health block's byte `+0x34`. On foot: idle (4, sub 0, sequence 10), or chatting with its moby (5: sequence
//! 0x1c, 0x1e from frame 5, 0x1e); its weapon 0x09ac made on its joint 3. Riding (sub 5): on the beast (deleted
//! without one), its place and angles, no collision, the beast's `+0x2ac` itself; the seat 0x0911 made 0.75 above it
//! (not drawn), the saddle 0x0ff9 on its joint 1. Then the wander's home its place (radius 1, its range `+0x298`, 10
//! from Ratchet), the wander timer 120..700.
//!
//! **Each tick** ([`senses`]): its shadow within 24 of the camera (range 0x16); `+0x2e` = 2 (the health block's byte,
//! set by nothing here): its parts and itself deleted; its weapon on joint 3; at 1 health or less the weapon drops (a
//! smoke burst, `0x342c98(0.5, 3)`); stuck 16 ticks in sub 2: a new yaw offset and the swing (0x11); the frame's
//! damage (flag `+0x2a0`) — any: the playback speed 1, with the attack flag eight pairs of sparks at joint 3
//! ([`sparks`], `0x452900`), a kill its sound 0x18, the flash 0xd2 red, its group's riders alarmed (`0x452878`), a
//! kill or the flag's kind gives the charge to its group's first member without it (two alive or more); the range
//! test and the two target searches (`0x34dc98` = crate::moby::ai::find_target, height 10); off sub 4 it falls 0.313
//! a tick to the ground; under z 40 its bolts, its parts gone, deleted.
//!
//! **Subs** (`+0x95`): 0 calm (the calm limits): the near area seeing Ratchet (or sure-footed with him in the near
//! range) within 2 in height, once the alarm wait is out: with its path the run along it (0x16, sub 2), the charge
//! (0x11, sub 2, sequence 0x1f), else attacking (a third the swing 0x11, else the walk 0x13, sub 3); alarmed
//! (`+0x276`): no patrol path sub 1 (the sequence 0x18), once alarmed the look (6), more the patrol (7, the run
//! limits); then him in range within 2 in height as above. 1 alert (the run limits, wander range 6): seeing him (either
//! search) the charge (0x15, sub 2) or the swing (0x11, sub 3), or with its path the run along it from its nearest
//! point (0x16, sub 2). 2 fighting / 3 attacking: losing him (sub 2: unless sure-footed) one of four idles and 0xc
//! (sub 1); in sub 3 the charge first. 4 the knock (the framework's flight, facing the target with health): dead, its
//! bolts and the fade (crate::moby::classes::death_fade); over, back as sub 0 would, or the lunge 0x17 after a hit of
//! the weapon kind 10, or the patrol, or the stand 0xc (sub 1, `+0x270` 580..600). 5 riding: on the beast with its
//! parts; the beast's `+0x27c` cleared (by the beast) or a hit on the seat (flags 0x10000, its damage off the health):
//! the saddle drops (smoke), the seat goes, off the beast (2, or 3 sequence 9; sub 6). 6 on foot again: its weapon
//! made, the attack flag, upright.
//!
//! **States**: 2 / 3 thrown off (sequence end: dead → its bolts and fade; else collision back, mode | 0x5000, 0xc sub
//! 1, the timer 480..600, sequence 0xb), on the ground from frame 10, between frames 1.5 and 17 a step (2: 0.05 to its
//! left, 3: 0.15 back) on walkable ground (`0x33d6e0` = crate::moby::ai::step_walk 0.5 / 0.55 / 0.5 / 30°). 4 the
//! idle wander (range over 0.2): sequence 0xd walking (speed 0.05 eased by 1/1200, 0 for the timer's last 20 ticks)
//! with the wander (`0x345c78` = crate::moby::classes::leaper::wander), 10 standing; the timer 120..700 again. 5
//! the chat: its moby's facing turns its head (`+0x328`, ±90°, the joint aims are not ported); the sequences 0x1a..
//! 0x1e passed between the two (`+0x564` 1 ↔ 2; 3 picks a 1 of its group). 6 the look (the attack flag; at its end
//! 7, sequence 0xd). 7 / 0xf along the patrol path / the path to go (`0x350cc0` = crate::moby::ai::follow_path; 7's
//! end: 9; 0xf's: 0x10). 9 at its end (the calm limits): two of its group alive or more, a chat (0xb, then 5). 0xb
//! facing its moby (5° → 5). 0xc standing, turning to the target (`0x33d5a8` = crate::moby::ai::turn_to 0.001 /
//! 0.01 / 0.1, range over 0.2), at its sequence's end the walk home 0xd (home the path to go's first point). 0xd the
//! walk limits toward home; within 2: 0xe (stands), or 0x10 with the path. 0x10 at its sequence's end along the path
//! from its nearer end (0xf). 0x11 the swing: facing the target, at its end the attack flag and within 12 the strafe
//! 0x12 (sequence 0x14) else the walk 0x13 (sub 3); with the charge 0x15 (sub 2). 0x12 the strafe: turning to it;
//! over 12 a tenth of the time 0x13; within 2.5 the lunge 0x17; a third of its sequence ends a new swing (0x11, sub
//! 3). 0x13 the walk: toward the target (`0x34fbb0` = crate::moby::ai::move_toward, its yaw offset; with the network
//! [`net_walk`], `0x452500`, there: 0x12); within 8 the strafe. 0x14 the close walk (the walk limits): within 2.5
//! facing the target within 20° the lunge; within 2.5 facing it; over 9 a quarter of the time the charge 0x15. 0x15
//! the charge: within 5 with over 1 health the close walk (0x14, sequence 0x16, the attack flag); within 2.5 the
//! lunge / facing; else toward it (with the network [`net_walk`]). 0x16 the run along its path (stop 1.5, accel
//! 1/240; within 2.5 of the target the lunge, accel 1/1200, its path gone): at the end the charge or the attack. 0x17
//! the lunge (sequence 0xe: frames 10..15 two hit spheres of 0.4 at 0.5 and 1.5 ahead of joint 2, its damage, flags
//! 0x4000001; under frame 10 at 0.65 speed): at its end 0x18 (sequence 0x1f), out of sub 2 the strafe. 0x18 at its
//! end the charge.
//!
//! **The tail** (`0x4527c8`): the flash, the timers `+0x270` / `+0x272` / `+0x26` down, its ticks; a height change
//! under 0.005 this tick undone.
//!
//! Not ported: the joint aims (`0x346548` / `0x349928` on `+0x2c0`: the head's look); the byte `0x139646` its first
//! alarm sets; 0x0015's lead hand-off (it compares the same distance twice, so it never happens); the creates' out of
//! slots (the port's always succeed).

use crate::hero::physics::{add, approach, atan2, dist2, ease, euler_rows, rot_dist, set_len, sub_rot, turn_ease, V3};
use crate::moby::{ai, Ctx, MobyWorld};
use super::super::props::{group_alive, set_seq, wrapped};
use super::super::leaper::seq_a;

/// The class.
pub const CLASS: i16 = 0x0ff8;
/// The saddle on its joint 1 while it rides, the seat target above it, its weapon on joint 3.
const SADDLE_CLASS: i16 = 0x0ff9;
const SEAT_CLASS: i16 = 0x0911;
const WEAPON_CLASS: i16 = 0x09ac;

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
const PATH_NEAR: usize = 0x260;
const PATH_FAR: usize = 0x264;
const RANGE_NEAR: usize = 0x268;
const RANGE_FAR: usize = 0x26c;
const T270: usize = 0x270;
const T272: usize = 0x272;
const RANGE: usize = 0x274;
const ALARMED: usize = 0x276;
const RIDES: usize = 0x27c;
const BEAST: usize = 0x280;
const BEAST_LINK: usize = 0x284;
const SADDLE: usize = 0x288;
const TURN_V: usize = 0x28c;
const SEAT: usize = 0x290;
const W_TIMER: usize = 0x294;
const W_RANGE: usize = 0x298;
const APATH: usize = 0x29c;
const ATTACK: usize = 0x2a0;
const CHARGE: usize = 0x2a4;
const OFFSET: usize = 0x2a8;
const WEAPON: usize = 0x2ac;
const WAIT: usize = 0x2b0;
const NIMBLE: usize = 0x2b4;
const FENCE: usize = 0x2b8;
const DAMAGE: usize = 0x2bc;
const NODES: usize = 0x340;
const NODES_MAX: usize = 16;
const N: usize = 0x540;
const START: usize = 0x544;
const NET: usize = 0x548;
const AT: usize = 0x550;
const GOAL: usize = 0x554;
const SPEED: usize = 0x558;
const STUCK: usize = 0x55c;
const SURE: usize = 0x560;
const CHAT: usize = 0x564;
const PARTNER_I: usize = 0x568;
const PARTNER: usize = 0x56c;
const PATROL: usize = 0x570;
const ALARMS: usize = 0x574;
const GO_PATH: usize = 0x578;
const PVARS: usize = 0x580;

/// The framework's limits (`0x350c50(walk accel, walk most, turn accel, turn most, m)`): calm 6/s² 3/s 240″ 6°, the
/// run 14/s² 7/s, the run and turn 700″ 15°, the walk 2/s² 1/s, the path 780″ 17°.
const CALM: [u32; 4] = [0x3ada_740e, 0x3d4c_ccce, 0x3a98_825c, 0x3dd6_7752];
const RUN: [u32; 4] = [0x3b7e_dcbb, 0x3dee_eef0, 0x3a98_825c, 0x3dd6_7752];
const RUN_TURN: [u32; 4] = [0x3b7e_dcbb, 0x3dee_eef0, 0x3b5e_68c6, 0x3e86_0a93];
const WALK: [u32; 4] = [0x3a11_a2b4, 0x3c88_8889, 0x3a98_825c, 0x3dd6_7752];
const PATH: [u32; 4] = [0x3ada_740e, 0x3d4c_ccce, 0x3b77_d3d5, 0x3e97_e9da];

const DEG: f32 = 0.017_453_294;

fn limits(w: &mut MobyWorld, id: usize, l: [u32; 4]) {
    let [a, b, c, d] = l.map(f32::from_bits);
    ai::senses(&mut w.mobys[id], a, b, c, d);
}

/// `0x34d750`: the first tick since the last sub-state change.
fn first_sub(w: &mut MobyWorld, id: usize) -> bool {
    let m = &mut w.mobys[id];
    if m.once & 2 != 0 { return false; }
    m.once |= 2;
    true
}

fn anim(w: &MobyWorld, id: usize, ctx: &mut Ctx, s: u8, frame: u8, blend: i32) { set_seq(w, id, ctx, s, frame, blend.clamp(0, 255) as u8); }
fn st(w: &mut MobyWorld, id: usize, state: u8, sub: i32) { w.mobys[id].set_state(state, sub); }
fn rand_blend(ctx: &mut Ctx, a: i32, b: i32) -> i32 { ctx.game.rng.rand_range(a, b) }
/// [`anim`] with its blend drawn from `lo..=hi` only when the sequence is to change.
fn anim_r(w: &MobyWorld, id: usize, ctx: &mut Ctx, s: u8, frame: u8, lo: i32, hi: i32) {
    if super::super::props::seq(w, id, ctx) != s {
        let b = rand_blend(ctx, lo, hi);
        ctx.host.set_anim(&w.mobys[id], s, frame, b.clamp(0, 255) as u8);
    }
}

/// The target moby's place (Ratchet's for the hero's moby).
fn tpos(w: &MobyWorld, id: usize, ctx: &Ctx) -> V3 {
    match ai::link(&w.mobys[id], T_MOBY) {
        Some(t) if Some(t) != ctx.hero_moby => w.mobys.get(t).map_or(ctx.hero.pos, |m| m.pos),
        _ => ctx.hero.pos,
    }
}

fn yaw_to(w: &MobyWorld, id: usize, p: V3) -> f32 { let q = w.mobys[id].pos; atan2(p[0] - q[0], p[1] - q[1]) }
fn tdist(w: &MobyWorld, id: usize, ctx: &Ctx) -> f32 { dist2(w.mobys[id].pos, tpos(w, id, ctx)) }
fn seen(w: &MobyWorld, id: usize) -> bool { w.mobys[id].pv_i(SEEN_NEAR) == 1 || w.mobys[id].pv_i(SEEN_FAR) == 1 }

/// `0x34fca0(yaw to the target, m, pos)`: the frame's move in place, facing the target.
fn face_target(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let y = yaw_to(w, id, tpos(w, id, ctx));
    let p = w.mobys[id].pos;
    ai::move_step(w, id, ctx, y, p);
}

/// `0x33d5a8(0.001, 0.01, 0.1, m, target, +0x28c)`: the facing springs toward the target.
fn turn_target(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let y = yaw_to(w, id, tpos(w, id, ctx));
    ai::turn_to(&mut w.mobys[id], y, 0.001, 0.01, 0.1, TURN_V);
}

/// The level path `i`'s points.
fn path(ctx: &Ctx, i: i32) -> Option<Vec<V3>> {
    usize::try_from(i).ok().and_then(|p| ctx.level.paths.get(p)).filter(|p| !p.is_empty()).map(|p| p.iter().map(|q| [q[0], q[1], q[2]]).collect())
}

/// `0x350dc8(m, path, last, first)`: a walk along the path from point `first` to `last` (−1: its last point).
fn path_start(w: &mut MobyWorld, id: usize, ctx: &Ctx, i: i32, last: i32, first: i32) {
    let Some(p) = path(ctx, i) else { return };
    let m = &mut w.mobys[id];
    let o = ai::mv(m);
    m.set_pv_i(o + 0xc4, i + 1);
    m.set_pv_s(o + 0xca, if last == -1 { p.len() as i16 - 1 } else { last as i16 });
    m.set_pv_s(o + 0xc8, first as i16);
}

/// `0x34cb40(near, far, m, &who, near path, far path, 0)`: Ratchet (the moby kept at `who`) in the near range (2; in
/// the near path's polygon when it has one) or the far one (1), outside play neither.
fn range_test(w: &mut MobyWorld, id: usize, ctx: &Ctx) -> i16 {
    let m = &mut w.mobys[id];
    ai::set_link(m, 0x278, ctx.hero_moby);
    if ctx.game.scenes.mode != crate::scene::PLAY { return 0; }
    let hero = ctx.hero.pos;
    let d = dist2(m.pos, hero);
    let (np, fp) = (m.pv_i(PATH_NEAR), m.pv_i(PATH_FAR));
    if if np == -1 { d < m.pv_f(RANGE_NEAR) } else { ctx.level.in_polygon(np, hero) } { return 2; }
    if if fp == -1 { d < m.pv_f(RANGE_FAR) } else { ctx.level.in_polygon(fp, hero) } { return 1; }
    0
}

/// A part (`0x332670`): its update distance, draw distance, light and ambient its owner's, drawn, rebuilt; at its
/// owner's `joint` (crate::moby::classes::enemies::place_part).
fn make_part(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, class: i16, joint: usize, solid: bool) -> usize {
    let (ud, dd, light, amb) = { let m = &w.mobys[id]; (m.update_dist, m.draw_dist, m.light, m.ambient) };
    let p = w.create(class);
    let pm = &mut w.mobys[p];
    pm.update_dist = ud;
    pm.draw_dist = dd;
    pm.drawn = 1;
    pm.light = light;
    pm.ambient = amb;
    w.rebuild(p, &*ctx.host);
    if !solid { w.mobys[p].has_collision = false; }
    super::super::enemies::place_part(w, id, p, joint, ctx);
    p
}

fn part(w: &MobyWorld, id: usize, off: usize) -> Option<usize> { ai::link(&w.mobys[id], off).filter(|&p| w.mobys.get(p).is_some_and(|m| m.state < 0x80)) }

/// The part at `off` deleted (`0x3329d8`), with a smoke burst at it first (`0x342c98(0.5, 3, m, part, −1)`).
fn drop_part(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, off: usize, smoke: bool) {
    if let Some(p) = part(w, id, off) {
        if smoke {
            let at = w.mobys[p].pos;
            crate::moby::blast::smoke_burst(w, ctx, 0.5, 3.0, Some(id), at, -1);
        }
        w.delete(p);
    }
    ai::set_link(&mut w.mobys[id], off, None);
}

/// The beast's rider word (`+0x2ac`).
fn unseat(w: &mut MobyWorld, id: usize) {
    if let Some(b) = ai::link(&w.mobys[id], BEAST_LINK) { ai::set_link(&mut w.mobys[b], 0x2ac, None); }
}

/// Its bolts (`0x33c0c0(m, 0, 3)`) and the fade (`0x4254e8(m, 0, 0)`).
fn die(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    crate::moby::bolts::drop(w, id, 0, 3, ctx);
    super::super::death_fade::start(w, id, ctx, 0.0, None);
}

fn members(w: &MobyWorld, id: usize) -> Vec<usize> {
    usize::try_from(w.mobys[id].group).ok().and_then(|g| w.groups.get(g)).cloned().unwrap_or_default()
}

/// `0x452878`: its group's live riders alarmed (`+0x276`), each counting the alarm (`+0x574`).
fn alarm(w: &mut MobyWorld, id: usize) {
    for i in members(w, id) {
        let m = &mut w.mobys[i];
        if m.state < 0x80 && m.o_class == CLASS {
            m.set_pv_s(ALARMED, 1);
            m.set_pv_i(ALARMS, m.pv_i(ALARMS) + 1);
        }
    }
}

/// `0x452900(at, n)`: `n` pairs of twinkles 0.3 toward the camera from `at` (which jitters ±0.15 a pair), thrown
/// 0.05..3 a second across (the camera's way turned ±60°) and 1..5 up, 0.5..2 in size: pink `0x7f207f7f` and a smaller
/// white `0x7f7f7f7f`, 60 ticks, spinning opposite ways.
fn sparks(ctx: &mut Ctx, mut at: V3, n: i32) {
    let cam = ctx.host.camera_pos();
    let mut d = crate::hero::physics::sub(cam, at);
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
        let v = [d[0] * c - d[1] * s, d[0] * s + d[1] * c, 0.0];
        let k = rng.randf(0.05, 3.0) * 0.016_666_668;
        let mut v = set_len(v, k);
        v[2] = rng.randf(1.0, 5.0) * 0.016_666_668;
        let p = add(at, off);
        for k in 0..3 { at[k] += rng.randf_signed(0.0, 0.15); }
        if let Some(i) = crate::particles::twinkle(sys, rng, f * 0.1, f, 0.002, p, 0x3c, 0x7f20_7f7f, 0, spin, v, 0) { sys.pool.recs[i][9] = 0x74; }
        if let Some(i) = crate::particles::twinkle(sys, rng, f * 0.07, f * 0.7, 0.002, p, 0x3c, 0x7f7f_7f7f, 1, -spin, v, 0) { sys.pool.recs[i][9] = 0x74; }
    }
}

/// `0x451a80` (module docs). True when it is to be deleted.
fn senses(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    if w.mobys[id].state == 0 {
        let m = &mut w.mobys[id];
        if m.pvars.len() < PVARS { m.pvars.resize(PVARS, 0); }
        m.set_pv_i(0x18, MV as i32);
        let z = ai::ground_z(ctx, 0.5, w.mobys[id].pos, 0);
        let m = &mut w.mobys[id];
        m.pos[2] = z;
        if m.pv_i(SURE) != 0 {
            m.has_collision = false;
            m.mode |= 0x41;
        }
        if m.pv_i(NET) != 0 {
            let mut nodes = crate::path_network::read_nodes(&m.pvars, NODES, NODES_MAX);
            let n = crate::path_network::build(ctx.level, m.pv_i(START), &mut nodes);
            crate::path_network::write_nodes(&mut m.pvars, NODES, &nodes[..n]);
            m.set_pv_i(N, n as i32);
            crate::path_network::open(&mut ctx.game.path_openings, ctx.level, &nodes[..n]);
        }
        let mut off = w.mobys[id].pv_f(OFFSET);
        if off == 0.0 { off = ctx.game.rng.randf_signed(15.0, 25.0); }
        w.mobys[id].set_pv_f(OFFSET, off * DEG);
        if w.mobys[id].pv_i(NIMBLE) != 0 {
            let o = ctx.game.rng.randf_signed(5.0, 30.0);
            w.mobys[id].set_pv_f(OFFSET, o * DEG);
        }
        if ctx.game.rng.randi(2) == 1 {
            let m = &mut w.mobys[id];
            m.mode |= 0x8000;
            m.once |= 4;
        }
        let m = &mut w.mobys[id];
        m.set_pv_b(0x48, 20);
        m.set_pv_b(0x4a, 14);
        m.set_pv_b(REACT + 0x10, 0x19);
        m.set_pv_i(REACT + 0x48, 4);
        m.set_pv_b(REACT + 0x11, 0x15);
        m.set_pv_b(REACT + 0x12, 0x10);
        m.set_pv_b(REACT + 0x16, 0x12);
        m.set_pv_b(REACT + 0x17, 0x13);
        m.set_pv_f(REACT + 0x1c, f32::from_bits(0x3e4c_ccce));
        m.set_pv_f(REACT + 0x20, 0.125);
        m.set_pv_f(REACT + 0x24, f32::from_bits(0x3bb6_0b61));
        m.set_pv_f(REACT + 0x28, f32::from_bits(0x3c1f_49f5));
        m.set_pv_f(REACT + 0x2c, f32::from_bits(0x3b88_8889));
        for (k, v) in [(0x6c, 5u8), (0x6d, 10), (0x6e, 8), (0x6f, 0xf), (0x72, 6), (0x73, 0xb)] { m.set_pv_b(REACT + k, v); }
        m.set_pv_f(REACT + 0xac, 1.0);
        m.set_pv_f(REACT + 0x58, f32::from_bits(0x3e38_51ec));
        m.set_pv_i(MV + 0x48, 0x2c0);
        m.set_pv_f(MV + 0xc, f32::from_bits(0x3ecc_cccd));
        m.set_pv_f(REACT + 0x30, 0.75);
        m.set_pv_f(REACT + 0x34, 0.75);
        m.set_pv_f(REACT + 0x3c, f32::from_bits(0x3ecc_cccd));
        m.set_pv_f(MV, 0.75);
        m.set_pv_f(MV + 4, 0.75);
        m.set_pv_f(MV + 8, f32::from_bits(0x3ecc_cccd));
        if m.pv_i(NIMBLE) != 0 {
            m.set_pv_f(MV + 0xc, f32::from_bits(0x3f19_999a));
            m.set_pv_f(MV + 8, f32::from_bits(0x3f19_999a));
        }
        if m.pv_i(FENCE) != -1 { m.set_pv_i(MV + 0x3c, m.pv_i(FENCE)); }
        m.set_pv_f(MV + 0x10, f32::from_bits(0x3f75_be0d));
        m.set_pv_f(MV + 0x30, f32::from_bits(0x3fbd_e450));
        m.set_pv_f(MV + 0x2c, f32::from_bits(0x3ab6_0b61));
        m.set_pv_f(MV + 0x34, 1.5);
        limits(w, id, CALM);
        let m = &mut w.mobys[id];
        if m.pv_i(SURE) != 0 {
            m.set_pv_f(MV + 0xc, f32::from_bits(0x3f33_3333));
            m.set_pv_f(MV + 0x10, f32::from_bits(0x3fa7_8d37));
            m.set_pv_f(MV + 8, f32::from_bits(0x3f33_3333));
        }
        m.set_pv_b(0x28, 3);
        m.set_pv_b(0x29, 1);
        m.set_pv_f(0x30, 0.75);
        m.set_pv_f(DAMAGE, m.pv_b(0x34) as f32);
        m.set_pv_b(0x2e, 1);
        if m.pv_i(RIDES) == 0 {
            st(w, id, 4, 0);
            anim(w, id, ctx, 10, 0, 0);
            let chat = w.mobys[id].pv_i(CHAT);
            if (1..=3).contains(&chat) {
                let p = w.mobys[id].pv_i(PARTNER_I);
                ai::set_link(&mut w.mobys[id], PARTNER, usize::try_from(p).ok());
                st(w, id, 5, 0);
                match chat { 1 => anim(w, id, ctx, 0x1c, 0, 0), 2 => anim(w, id, ctx, 0x1e, 5, 0), _ => anim(w, id, ctx, 0x1e, 0, 0) }
            }
            let g = make_part(w, id, ctx, WEAPON_CLASS, 3, false);
            ai::set_link(&mut w.mobys[id], WEAPON, Some(g));
        } else {
            st(w, id, 1, 5);
            let Some(b) = usize::try_from(w.mobys[id].pv_i(BEAST)).ok().filter(|&b| b < w.mobys.len()) else {
                w.mobys[id].set_pv_i(RIDES, 0);
                st(w, id, 4, 0);
                return true;
            };
            let (bp, br) = (w.mobys[b].pos, w.mobys[b].rot);
            let m = &mut w.mobys[id];
            ai::set_link(m, BEAST_LINK, Some(b));
            m.pos = bp;
            m.rot = br;
            m.has_collision = false;
            m.mode &= 0xafff;
            ai::set_link(&mut w.mobys[b], 0x2ac, Some(id));
            let ud = w.mobys[id].update_dist;
            let s = w.create(SEAT_CLASS);
            w.mobys[s].update_dist = ud;
            w.mobys[s].drawn = 0;
            w.rebuild(s, &*ctx.host);
            w.mobys[s].pos = add(bp, [0.0, 0.0, 0.75]);
            ai::set_link(&mut w.mobys[id], SEAT, Some(s));
            let p = make_part(w, id, ctx, SADDLE_CLASS, 1, true);
            let m = &mut w.mobys[id];
            ai::set_link(m, SADDLE, Some(p));
            m.set_pv_i(ATTACK, 0);
        }
        let m = &mut w.mobys[id];
        let pos = m.pos;
        m.set_pv_v(WANDER, pos);
        m.set_pv_f(WANDER + 0x10, 1.0);
        m.set_pv_f(WANDER + 0x1c, m.pv_f(W_RANGE));
        m.set_pv_f(WANDER + 0x20, 10.0);
        let t = ctx.game.rng.rand_range(0x78, 700);
        w.mobys[id].set_pv_i(W_TIMER, t);
    }
    crate::shadow_volumes::probe_near(w, id, ctx, 24.0, 0x16);
    if w.mobys[id].pv_b(0x2e) == 2 {
        drop_part(w, id, ctx, WEAPON, false);
        drop_part(w, id, ctx, SADDLE, false);
        drop_part(w, id, ctx, SEAT, false);
        return true;
    }
    if let Some(g) = part(w, id, WEAPON) { super::super::enemies::place_part(w, id, g, 3, ctx); }
    if w.mobys[id].pv_f(HEALTH) <= 1.0 {
        drop_part(w, id, ctx, WEAPON, true);
        w.mobys[id].set_pv_i(ATTACK, 0);
    }
    if 0x10 <= w.mobys[id].pv_i(MV + 0x80) && w.mobys[id].sub_state == 2 {
        let o = ctx.game.rng.randf_signed(15.0, 25.0);
        let m = &mut w.mobys[id];
        m.set_pv_i(MV + 0x80, 0);
        m.set_pv_f(OFFSET, o * DEG);
        anim(w, id, ctx, 0xb, 0, 10);
        st(w, id, 0x11, -1);
    }
    let flag = w.mobys[id].pv_i(ATTACK) != 0;
    let kind = ai::take_damage(w, id, ctx, flag);
    if kind != 0 {
        if ctx.host.anim(&w.mobys[id]).is_some_and(|a| a.speed != 1.0) { ctx.host.set_anim_speed(&w.mobys[id], 1.0); }
        if w.mobys[id].pv_i(ATTACK) != 0 {
            if let Some(at) = ctx.host.joint(&w.mobys[id], 3) { sparks(ctx, at, 8); }
        }
        if kind == 4 { ctx.host.play_sound(&w.mobys[id], 0x18, 0); }
        let m = &mut w.mobys[id];
        m.set_pv_i(ATTACK, 0);
        m.set_pv_b(FLASH + 7, 0xd2);
        ai::flash_start(m, FLASH);
        alarm(w, id);
        if matches!(kind, 4 | 5) {
            let g = w.mobys[id].group as i32;
            if 0 <= g && 1 < group_alive(w, g) {
                if let Some(i) = members(w, id).into_iter().find(|&i| w.mobys[i].pv_i(CHARGE) == 0) { w.mobys[i].set_pv_i(CHARGE, 1); }
            }
        }
    }
    let r = range_test(w, id, ctx);
    w.mobys[id].set_pv_s(RANGE, r);
    // `0x34dc98(range, 10, m, area, 0, 0)` twice; the port's searches keep Ratchet as the target either way, so the
    // game's restore of a target other than him after the second is moot.
    let (rn, an) = (w.mobys[id].pv_f(RANGE_NEAR), w.mobys[id].pv_i(AREA_NEAR));
    let a = ai::find_target(w, id, ctx, rn, 10.0, an, None, false);
    w.mobys[id].set_pv_i(SEEN_NEAR, a as i32);
    let (rf, af) = (w.mobys[id].pv_f(RANGE_FAR), w.mobys[id].pv_i(AREA_FAR));
    let b = ai::find_target(w, id, ctx, rf, 10.0, af, None, false);
    w.mobys[id].set_pv_i(SEEN_FAR, b as i32);
    if w.mobys[id].sub_state != 4 {
        let g = ai::ground_z(ctx, 0.5, w.mobys[id].pos, 0);
        let m = &mut w.mobys[id];
        m.pos[2] = (m.pos[2] - f32::from_bits(0x3ea0_6d3a)).max(g);
    }
    if w.mobys[id].pos[2] < 40.0 {
        crate::moby::bolts::drop(w, id, 0, 3, ctx);
        drop_part(w, id, ctx, SADDLE, false);
        drop_part(w, id, ctx, WEAPON, true);
        return true;
    }
    false
}

/// The attack's start (sub 0's and the knock's): the run along its path (0x16, sub 2), the charge (0x11, sub 2), or
/// the swing (0x11) / walk (0x13) in sub 3 — `third` picks a third of the time the swing (sub 0) or always (else).
fn attack(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, third: bool) {
    let m = &w.mobys[id];
    if m.pv_i(APATH) != -1 {
        anim(w, id, ctx, 0xc, 0, 10);
        st(w, id, 0x16, 2);
    } else if m.pv_i(CHARGE) == 1 {
        anim_r(w, id, ctx, 0x1f, 0, 10, 20);
        st(w, id, 0x11, 2);
    } else {
        w.mobys[id].set_pv_i(ATTACK, 1);
        if !third || ctx.game.rng.randi(3) == 1 {
            anim(w, id, ctx, 0xb, 0, 10);
            st(w, id, 0x11, 3);
        } else {
            anim(w, id, ctx, 0xc, 0, 10);
            st(w, id, 0x13, 3);
        }
    }
}

/// Sub 0 (module docs).
fn calm(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if first_sub(w, id) { limits(w, id, CALM); }
    let m = &w.mobys[id];
    let sees = m.pv_i(SEEN_NEAR) == 1 || (m.pv_i(SURE) != 0 && m.pv_s(RANGE) == 2);
    if sees && (tpos(w, id, ctx)[2] - w.mobys[id].pos[2]).abs() < 2.0 {
        let mut t = w.mobys[id].pv_i(WAIT);
        let out = crate::moby::countdown(&mut t);
        w.mobys[id].set_pv_i(WAIT, t);
        if out {
            attack(w, id, ctx, true);
            return;
        }
    }
    if w.mobys[id].pv_s(ALARMED) == 0 { return; }
    let (patrol, alarms) = (w.mobys[id].pv_i(PATROL), w.mobys[id].pv_i(ALARMS));
    if patrol == -1 || alarms < 1 {
        w.mobys[id].sub_state = 1;
        anim(w, id, ctx, 0x18, 0, 0x14);
    } else if alarms == 1 {
        anim_r(w, id, ctx, 0x18, 0, 5, 15);
        st(w, id, 6, -1);
    } else {
        anim_r(w, id, ctx, 0xc, 0, 5, 15);
        st(w, id, 7, -1);
        limits(w, id, RUN_TURN);
    }
    let r = w.mobys[id].pv_s(RANGE);
    if (1..=2).contains(&r) && (tpos(w, id, ctx)[2] - w.mobys[id].pos[2]).abs() < 2.0 { attack(w, id, ctx, false); }
    w.mobys[id].set_pv_s(ALARMED, 0);
}

/// Sub 1 (module docs).
fn alert(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if first_sub(w, id) {
        limits(w, id, RUN);
        let m = &mut w.mobys[id];
        m.set_pv_f(W_RANGE, 6.0);
        m.set_pv_f(WANDER + 0x1c, 6.0);
    }
    if !seen(w, id) { return; }
    let m = &mut w.mobys[id];
    m.set_pv_i(MV + 0x38, m.pv_i(MV + 0x38) & !0x10);
    let ap = m.pv_i(APATH);
    if ap == -1 {
        if m.pv_i(CHARGE) == 1 {
            anim(w, id, ctx, 0xc, 0, 10);
            st(w, id, 0x15, 2);
        } else {
            m.set_pv_i(ATTACK, 1);
            anim(w, id, ctx, 0xb, 0, 10);
            st(w, id, 0x11, 3);
        }
    } else {
        anim(w, id, ctx, 0xc, 0, 10);
        st(w, id, 0x16, 2);
        let pos = w.mobys[id].pos;
        let mut best = (100.0, 0);
        for (k, p) in path(ctx, ap).unwrap_or_default().iter().enumerate() {
            let d = dist2(pos, *p);
            if d < best.0 { best = (d, k as i32); }
        }
        path_start(w, id, ctx, ap, -1, best.1);
    }
}

/// Subs 2 and 3: losing him, one of four idles (each a sequence and its blend range) and the stand (0xc, sub 1).
fn lost(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, idles: [(u8, i32, i32); 4]) {
    let (s, lo, hi) = idles[ctx.game.rng.randi(4) as usize];
    if super::super::props::seq(w, id, ctx) != s {
        let b = if lo == hi { lo } else { rand_blend(ctx, lo, hi) };
        ctx.host.set_anim(&w.mobys[id], s, 0, b as u8);
    }
    st(w, id, 0xc, 1);
}

/// The beast's ride (sub 5).
fn riding(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if let Some(p) = part(w, id, SADDLE) { super::super::enemies::place_part(w, id, p, 1, ctx); }
    if let Some(b) = ai::link(&w.mobys[id], BEAST_LINK) {
        let (bp, br) = (w.mobys[b].pos, w.mobys[b].rot);
        let m = &mut w.mobys[id];
        m.pos = bp;
        m.rot = br;
        if let Some(s) = part(w, id, SEAT) {
            let sm = &mut w.mobys[s];
            sm.pos = add(bp, [0.0, 0.0, 0.65]);
            sm.rot = br;
        }
    }
    if w.mobys[id].pv_i(RIDES) == 0 {
        drop_part(w, id, ctx, SADDLE, true);
        drop_part(w, id, ctx, SEAT, false);
        unseat(w, id);
        st(w, id, 2, 6);
        return;
    }
    // `0x44f278`: a hit on the seat (flags 0x10000) takes its damage off the health.
    let hit = part(w, id, SEAT).and_then(|s| w.hit_message(s)).filter(|h| h.tmpl.flags & 0x10000 != 0);
    let Some(h) = hit else { return };
    let m = &mut w.mobys[id];
    m.set_pv_f(HEALTH, m.pv_f(HEALTH) - h.tmpl.damage);
    drop_part(w, id, ctx, SADDLE, true);
    drop_part(w, id, ctx, SEAT, false);
    unseat(w, id);
    if super::super::props::seq(w, id, ctx) != 9 { ctx.host.set_anim(&w.mobys[id], 9, 0, 0); }
    st(w, id, 3, 6);
}

/// `0x452500(yaw, speed, near, offset, m, goal)`: the walk through its network toward `goal`: farther than `near`
/// across, the heading through the network (crate::path_network::steer_offset, 0.4 narrowing) eased to (`0x340e18`:
/// 0.4°, 0.2°, 12°) and, within 30° of it, the speed eased toward the distance (`speed`/1200, at most `speed`/60);
/// nearer, `yaw` eased to (0.4°, 0.2, 12°) and the speed down by `speed`/1200 — stopped and within 5° of `yaw`, done.
/// The step along its facing on walkable ground (`0x33d6e0(0.75, 0.75, 0.4, 30°)`); 30 steps that fail, done.
fn net_walk(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, yaw: f32, speed: f32, near: f32, goal: V3) -> bool {
    let mut done = false;
    let m = &w.mobys[id];
    let pos = m.pos;
    let l = dist2(goal, pos);
    let step = speed * 0.000_833_333_35;
    let off = m.pv_f(OFFSET);
    let nodes = crate::path_network::read_nodes(&m.pvars, NODES, (m.pv_i(N).max(0) as usize).min(NODES_MAX));
    let m = &mut w.mobys[id];
    let (mut a, mut v, mut s) = (m.rot[2], m.pv_f(TURN_V), m.pv_f(SPEED));
    if near < l {
        let (mut y, mut at, mut to) = (0.0, m.pv_i(AT), m.pv_i(GOAL));
        crate::path_network::steer_offset(ctx.level, &nodes, off, f32::from_bits(0x3ecc_cccd), pos, goal, &mut y, &mut at, &mut to);
        m.set_pv_i(AT, at);
        m.set_pv_i(GOAL, to);
        turn_ease(y, f32::from_bits(0x3be4_c38a), f32::from_bits(0x3b64_c38a), f32::from_bits(0x3e56_7752), &mut a, &mut v, 0);
        if rot_dist(a, y) < 0.523_598_85 {
            let mut x = 0.0;
            ease(l, step, step, speed * 0.016_666_668, &mut x, &mut s);
        } else {
            s = 0.0;
        }
    } else {
        turn_ease(yaw, f32::from_bits(0x3be4_c38a), 0.2, f32::from_bits(0x3e56_7752), &mut a, &mut v, 0);
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
    let ok = ai::step_walk(ctx, id, 0.75, 0.75, f32::from_bits(0x3ecc_cccd), f32::from_bits(0x3f06_0a93), &mut p, &mut target, 0);
    let m = &mut w.mobys[id];
    m.pos = p;
    w.changed = true;
    if !ok { m.set_pv_i(STUCK, m.pv_i(STUCK) + 1); }
    if 0x1d < m.pv_i(STUCK) {
        m.set_pv_i(STUCK, 0);
        done = true;
    }
    done
}

/// The move toward the target point (`+0x230`): with the network [`net_walk`] (7, 0.25) — done: the strafe
/// (0x12) — else the framework's move with its yaw offset.
fn chase(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    let goal = w.mobys[id].pv_v(TARGET);
    if w.mobys[id].pv_i(NET) == 0 {
        let off = w.mobys[id].pv_f(OFFSET);
        ai::move_toward(w, id, ctx, off, goal);
        false
    } else {
        let y = w.mobys[id].rot[2];
        net_walk(w, id, ctx, y, 7.0, 0.25, goal)
    }
}

/// The lunge (0x17, sequence 0xe from frame 0 blended `blend`) after the run limits.
fn lunge(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, blend: i32, sub: i32) {
    anim(w, id, ctx, 0xe, 0, blend);
    limits(w, id, RUN);
    st(w, id, 0x17, sub);
}

/// 0x0ff8 (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let z0 = w.mobys[id].pos[2];
    if senses(w, id, ctx) {
        w.delete(id);
        return;
    }
    match w.mobys[id].sub_state {
        0 => calm(w, id, ctx),
        1 => alert(w, id, ctx),
        2 => {
            if first_sub(w, id) { limits(w, id, RUN); }
            if !seen(w, id) && w.mobys[id].pv_i(SURE) == 0 { lost(w, id, ctx, [(0x19, 10, 0x14), (0x11, 0x14, 0x1e), (0x1e, 0x14, 0x1e), (0x18, 0x14, 0x1e)]); }
        }
        3 => {
            if first_sub(w, id) { limits(w, id, RUN); }
            if w.mobys[id].pv_i(CHARGE) == 1 {
                anim(w, id, ctx, 0xc, 0, 10);
                st(w, id, 0x15, 2);
            }
            if !seen(w, id) { lost(w, id, ctx, [(0xb, 10, 10), (0x11, 0x14, 0x14), (0x1e, 0x14, 0x14), (0x18, 0x14, 0x14)]); }
        }
        4 => {
            if 0.0 < w.mobys[id].pv_f(HEALTH) { face_target(w, id, ctx); }
            let r = ai::flight(w, id, ctx);
            if r & 1 != 0 {
                die(w, id, ctx);
                return;
            }
            if r & 2 != 0 {
                w.mobys[id].set_pv_s(HEALTH + 6, 0);
                if seen(w, id) && (tpos(w, id, ctx)[2] - w.mobys[id].pos[2]).abs() < 2.0 {
                    anim(w, id, ctx, 0xc, 0, 10);
                    st(w, id, 0x15, 2);
                    if w.mobys[id].pv_b(0x37) == 10 {
                        anim(w, id, ctx, 0xe, 9, 4);
                        limits(w, id, RUN);
                        st(w, id, 0x17, 2);
                    }
                } else if w.mobys[id].pv_i(PATROL) != -1 {
                    anim_r(w, id, ctx, 0xc, 0, 5, 15);
                    st(w, id, 7, 0);
                    limits(w, id, RUN_TURN);
                } else {
                    anim(w, id, ctx, 10, 0, 10);
                    st(w, id, 0xc, 1);
                    let t = rand_blend(ctx, 0x244, 600);
                    w.mobys[id].set_pv_s(T270, t as i16);
                }
            }
        }
        5 => riding(w, id, ctx),
        6 => {
            if first_sub(w, id) {
                let g = make_part(w, id, ctx, WEAPON_CLASS, 3, false);
                let m = &mut w.mobys[id];
                ai::set_link(m, WEAPON, Some(g));
                m.set_pv_i(ATTACK, 1);
                m.rot[0] = 0.0;
                m.rot[1] = 0.0;
                m.rows = euler_rows(m.rot);
            }
        }
        _ => {}
    }
    if w.mobys[id].state >= 0x80 { return; }
    states(w, id, ctx);
    if w.mobys[id].state >= 0x80 { return; }
    tail(w, id);
    let m = &mut w.mobys[id];
    if (m.pos[2] - z0).abs() < 0.005 { m.pos[2] = z0; }
}

/// The second switch, on the state (module docs).
fn states(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let state = w.mobys[id].state;
    match state {
        2 | 3 => {
            if wrapped(w, id, ctx) {
                if w.mobys[id].pv_f(HEALTH) <= 0.0 {
                    die(w, id, ctx);
                    return;
                }
                let solid = w.class(CLASS).is_some_and(|c| c.has_collision);
                let m = &mut w.mobys[id];
                m.set_pv_i(MV + 0x38, m.pv_i(MV + 0x38) & !0x10);
                m.mode |= 0x5000;
                m.has_collision = solid;
                st(w, id, 0xc, 1);
                if w.mobys[id].pv_s(T270) < 300 {
                    let t = rand_blend(ctx, 0x1e0, 600);
                    w.mobys[id].set_pv_s(T270, t as i16);
                }
                anim(w, id, ctx, 0xb, 0, 10);
            }
            let t = ctx.host.frame_time(&w.mobys[id]);
            if 10.0 <= t {
                let z = ai::ground_z(ctx, 0.5, w.mobys[id].pos, 0);
                w.mobys[id].pos[2] = z;
            }
            if !(1.5..=17.0).contains(&t) { return; }
            let m = &mut w.mobys[id];
            m.set_pv_i(MV + 0x38, m.pv_i(MV + 0x38) | 0x10);
            let yaw = m.rot[2];
            let (a, k) = if state == 2 { (yaw + 1.570_796_5, 0.05) } else { (yaw + 3.141_593, 0.15) };
            let pos = m.pos;
            let mut goal = add(pos, [a.cos() * k, a.sin() * k, 0.0]);
            let mut p = pos;
            ai::step_walk(ctx, id, 0.5, f32::from_bits(0x3f0c_cccd), 0.5, f32::from_bits(0x3f06_0a93), &mut p, &mut goal, 0);
            w.mobys[id].pos = p;
            w.changed = true;
        }
        4 => {
            if w.mobys[id].pv_f(W_RANGE) <= 0.2 { return; }
            let mut t = w.mobys[id].pv_i(W_TIMER);
            crate::moby::countdown(&mut t);
            w.mobys[id].set_pv_i(W_TIMER, t);
            let s = super::super::props::seq(w, id, ctx);
            if s != 0xd && !(s == 10 && t == 0) { anim(w, id, ctx, 0xd, 0, 0xe); }
            let to = if w.mobys[id].pv_i(W_TIMER) < 0x15 { 0.0 } else { 0.05 };
            let m = &mut w.mobys[id];
            let mut v = m.pv_f(WANDER + 0x14);
            approach(to, f32::from_bits(0x3a5a_740e), &mut v);
            m.set_pv_f(WANDER + 0x14, v);
            let s = super::super::props::seq(w, id, ctx);
            if s == 0xd {
                if w.mobys[id].pv_i(W_TIMER) == 0 { ctx.host.set_anim(&w.mobys[id], 10, 0, 0xc); }
                super::super::leaper::wander(w, id, ctx, WANDER);
            } else if s == 10 && wrapped(w, id, ctx) {
                let t = rand_blend(ctx, 0x78, 700);
                w.mobys[id].set_pv_i(W_TIMER, t);
                anim(w, id, ctx, 0xd, 0, 0xc);
            }
        }
        5 => chat(w, id, ctx),
        6 => {
            w.mobys[id].set_pv_i(ATTACK, 1);
            if wrapped(w, id, ctx) {
                st(w, id, 7, -1);
                anim(w, id, ctx, 0xd, 0, 0x1e);
                w.mobys[id].set_pv_i(ATTACK, 0);
            }
        }
        7 => {
            let p = w.mobys[id].pv_i(PATROL);
            if ai::follow_path(w, id, ctx, p) {
                st(w, id, 9, -1);
                anim(w, id, ctx, 0x18, 0, 0x14);
            }
        }
        9 => {
            if wrapped(w, id, ctx) {
                limits(w, id, CALM);
                let g = w.mobys[id].group as i32;
                if 0 <= g && 1 < group_alive(w, g) {
                    // `0x33e6d0`: its group's first live member, or the next when that is itself.
                    let live: Vec<usize> = members(w, id).into_iter().filter(|&i| w.mobys[i].state < 0x80).collect();
                    let other = match live.first() { Some(&f) if f == id => live.get(1).copied(), f => f.copied() };
                    ai::set_link(&mut w.mobys[id], PARTNER, other);
                    st(w, id, 0xb, -1);
                    anim(w, id, ctx, 0x1c, 0, 0x1e);
                    w.mobys[id].set_pv_i(CHAT, 1);
                    if other.is_some_and(|o| seq_a(w, o, ctx) == 0x1c) {
                        anim(w, id, ctx, 0x1e, 0, 0x1e);
                        w.mobys[id].set_pv_i(CHAT, 2);
                    }
                }
            }
        }
        0xb => {
            let Some(o) = ai::link(&w.mobys[id], PARTNER) else { return };
            let y = yaw_to(w, id, w.mobys[o].pos);
            let p = w.mobys[id].pos;
            ai::move_step(w, id, ctx, y, p);
            if rot_dist(w.mobys[id].rot[2], y) < 0.087_266_475 { st(w, id, 5, -1); }
        }
        0xc => {
            if 0.2 < w.mobys[id].pv_f(W_RANGE) {
                turn_target(w, id, ctx);
                if wrapped(w, id, ctx) {
                    anim(w, id, ctx, 0x16, 0, 0x14);
                    st(w, id, 0xd, -1);
                    let m = &mut w.mobys[id];
                    m.set_pv_i(MV + 0x38, m.pv_i(MV + 0x38) | 0x10);
                    let gp = m.pv_i(GO_PATH);
                    if let Some(p) = path(ctx, gp) { w.mobys[id].set_pv_v(WANDER, p[0]); }
                }
            }
        }
        0xd => {
            turn_target(w, id, ctx);
            limits(w, id, WALK);
            let home = w.mobys[id].pv_v(WANDER);
            let y = yaw_to(w, id, home);
            ai::move_step(w, id, ctx, y, home);
            if dist2(w.mobys[id].pos, home) < 2.0 {
                let m = &mut w.mobys[id];
                m.set_pv_i(MV + 0x38, m.pv_i(MV + 0x38) & !0x10);
                st(w, id, 0xe, -1);
                anim(w, id, ctx, 0x18, 0, 0x14);
                if w.mobys[id].pv_i(GO_PATH) != -1 { st(w, id, 0x10, -1); }
            }
        }
        0xf => {
            limits(w, id, PATH);
            let p = w.mobys[id].pv_i(GO_PATH);
            if ai::follow_path(w, id, ctx, p) {
                st(w, id, 0x10, -1);
                anim(w, id, ctx, 0x18, 0, 0x14);
            }
        }
        0x10 => {
            if !wrapped(w, id, ctx) { return; }
            let gp = w.mobys[id].pv_i(GO_PATH);
            let Some(p) = path(ctx, gp) else { return };
            let pos = w.mobys[id].pos;
            let last = p.len() as i32 - 1;
            if dist2(pos, p[p.len() - 1]) < dist2(pos, p[0]) { path_start(w, id, ctx, gp, 0, last) } else { path_start(w, id, ctx, gp, last, 0) }
            st(w, id, 0xf, -1);
            anim(w, id, ctx, 0xd, 0, 0x14);
        }
        0x11 => {
            w.mobys[id].set_pv_i(ATTACK, 0);
            face_target(w, id, ctx);
            if !wrapped(w, id, ctx) { return; }
            w.mobys[id].set_pv_i(ATTACK, 1);
            if tdist(w, id, ctx) < 12.0 {
                anim(w, id, ctx, 0x14, 0, 10);
                st(w, id, 0x12, -1);
            } else {
                anim(w, id, ctx, 0xc, 0, 10);
                st(w, id, 0x13, 3);
            }
            if w.mobys[id].pv_i(CHARGE) != 0 {
                anim(w, id, ctx, 0xc, 0, 10);
                st(w, id, 0x15, 2);
            }
        }
        0x12 => {
            turn_target(w, id, ctx);
            if 12.0 < tdist(w, id, ctx) && ctx.game.rng.randi(10) == 1 {
                anim(w, id, ctx, 0xc, 0, 10);
                st(w, id, 0x13, -1);
            }
            if tdist(w, id, ctx) < 2.5 {
                anim(w, id, ctx, 0xe, 0, 10);
                st(w, id, 0x17, -1);
            }
            if ctx.game.rng.randi(3) == 1 && wrapped(w, id, ctx) {
                anim(w, id, ctx, 0x19, 0, 10);
                if ctx.game.rng.randi(3) == 1 { anim(w, id, ctx, 0x18, 0, 10); }
                st(w, id, 0x11, 3);
            }
        }
        0x13 => {
            let done = chase(w, id, ctx);
            if done || tdist(w, id, ctx) < 8.0 {
                anim(w, id, ctx, 0x14, 0, 10);
                st(w, id, 0x12, -1);
            }
        }
        0x14 => {
            let d = tdist(w, id, ctx);
            let tp = w.mobys[id].pv_v(TARGET);
            if d < 2.5 && rot_dist(w.mobys[id].rot[2], yaw_to(w, id, tp)) < 0.349_065_9 {
                lunge(w, id, ctx, 6, 2);
            } else if d < 2.5 {
                face_target(w, id, ctx);
            } else if d <= 9.0 || ctx.game.rng.randi(4) != 1 {
                limits(w, id, WALK);
                let off = w.mobys[id].pv_f(OFFSET);
                ai::move_toward(w, id, ctx, off, tp);
            } else {
                limits(w, id, RUN);
                anim(w, id, ctx, 0xc, 0, 10);
                st(w, id, 0x15, -1);
            }
        }
        0x15 => {
            let d = tdist(w, id, ctx);
            let tp = w.mobys[id].pv_v(TARGET);
            if d < 5.0 && 1.0 < w.mobys[id].pv_f(HEALTH) {
                w.mobys[id].set_pv_i(ATTACK, 1);
                limits(w, id, WALK);
                st(w, id, 0x14, -1);
                anim(w, id, ctx, 0x16, 0, 10);
            } else if d < 2.5 && rot_dist(w.mobys[id].rot[2], yaw_to(w, id, tp)) < 0.349_065_9 {
                lunge(w, id, ctx, 10, 2);
            } else if d < 2.5 {
                face_target(w, id, ctx);
            } else if w.mobys[id].pv_i(NET) != 0 {
                let y = w.mobys[id].rot[2];
                net_walk(w, id, ctx, y, 7.0, 0.25, tp);
            } else {
                let off = w.mobys[id].pv_f(OFFSET);
                ai::move_toward(w, id, ctx, off, tp);
            }
        }
        0x16 => {
            let o = MV;
            w.mobys[id].set_pv_f(o + 0x34, 1.5);
            if tdist(w, id, ctx) < 2.5 {
                anim(w, id, ctx, 0xe, 0, 2);
                st(w, id, 0x17, -1);
                let m = &mut w.mobys[id];
                m.set_pv_i(APATH, -1);
                m.set_pv_f(o + 0x24, f32::from_bits(0x3a5a_740e));
            } else {
                w.mobys[id].set_pv_f(o + 0x24, f32::from_bits(0x3b88_8889));
                let ap = w.mobys[id].pv_i(APATH);
                if ai::follow_path(w, id, ctx, ap) {
                    if w.mobys[id].pv_i(CHARGE) == 1 {
                        anim(w, id, ctx, 0xc, 0, 10);
                        st(w, id, 0x15, 2);
                    } else {
                        w.mobys[id].set_pv_i(ATTACK, 1);
                        if ctx.game.rng.randi(3) == 1 {
                            anim(w, id, ctx, 0xb, 0, 10);
                            st(w, id, 0x11, 3);
                        } else {
                            anim(w, id, ctx, 0xc, 0, 10);
                            st(w, id, 0x13, 3);
                        }
                    }
                    w.mobys[id].set_pv_i(APATH, -1);
                }
            }
        }
        0x17 => {
            w.mobys[id].set_pv_i(ATTACK, 0);
            let t = ctx.host.frame_time(&w.mobys[id]);
            let a = seq_a(w, id, ctx);
            if a == 0xe && (10.0..=15.0).contains(&t) {
                if let Some(j) = ctx.host.joint(&w.mobys[id], 2) {
                    let yaw = w.mobys[id].rot[2];
                    let dmg = w.mobys[id].pv_f(DAMAGE);
                    let mut p = add(j, [yaw.cos() * 0.5, yaw.sin() * 0.5, 0.0]);
                    ai::attack_sphere(w, id, ctx, f32::from_bits(0x3ecc_cccd), dmg, 1.0, p, 0x0400_0001, 0, 1, 0);
                    p = add(p, [yaw.cos(), yaw.sin(), 0.0]);
                    ai::attack_sphere(w, id, ctx, f32::from_bits(0x3ecc_cccd), dmg, 1.0, p, 0x0400_0001, 0, 1, 0);
                }
            }
            let speed = if a == 0xe && t < 10.0 { f32::from_bits(0x3f26_6666) } else { 1.0 };
            ctx.host.set_anim_speed(&w.mobys[id], speed);
            if !wrapped(w, id, ctx) { return; }
            anim(w, id, ctx, 0x1f, 0, 10);
            st(w, id, 0x18, -1);
            if w.mobys[id].sub_state != 2 {
                st(w, id, 0x12, -1);
                anim(w, id, ctx, 0x14, 0, 10);
            }
        }
        0x18 => {
            if wrapped(w, id, ctx) {
                anim(w, id, ctx, 0xc, 0, 10);
                st(w, id, 0x15, -1);
            }
        }
        _ => {}
    }
}

/// The chat (state 5, module docs).
fn chat(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let Some(o) = ai::link(&w.mobys[id], PARTNER).filter(|&o| o < w.mobys.len()) else { return };
    if w.mobys[id].pv_i(CHAT) == 3 {
        let y = yaw_to(w, id, w.mobys[o].pos);
        let mut d = sub_rot(y, w.mobys[id].rot[2]);
        if w.mobys[id].once & 4 != 0 { d = -d; }
        let h = std::f32::consts::FRAC_PI_2;
        w.mobys[id].set_pv_f(0x328, d.clamp(-h, h));
    }
    if w.mobys[id].pv_i(CHAT) == 1 && seq_a(w, id, ctx) == 0x1c && ctx.game.rng.randi(200) == 1 {
        anim(w, id, ctx, 0x1b, 0, 0x1e);
        if ctx.game.rng.randi(3) == 1 { anim(w, id, ctx, 0x1a, 0, 0x1e); }
    }
    let mut a = seq_a(w, id, ctx);
    if (0x1a..=0x1b).contains(&a) && wrapped(w, id, ctx) {
        anim(w, id, ctx, 0x1d, 0, 0x1e);
        if w.mobys[o].o_class == CLASS {
            w.mobys[o].set_pv_i(CHAT, 1);
            w.mobys[id].set_pv_i(CHAT, 2);
        }
        a = seq_a(w, id, ctx);
    }
    if a == 0x1d && wrapped(w, id, ctx) {
        anim(w, id, ctx, 0x1e, 0, 0x1e);
        if w.mobys[id].pv_i(CHAT) == 1 { anim(w, id, ctx, 0x1c, 0, 0x1e); }
        if w.mobys[id].pv_i(CHAT) == 3 {
            if let Some(p) = members(w, id).into_iter().find(|&i| w.mobys[i].state < 0x80 && w.mobys[i].o_class == CLASS && w.mobys[i].pv_i(CHAT) == 1) {
                ai::set_link(&mut w.mobys[id], PARTNER, Some(p));
            }
        }
    }
    let Some(o) = ai::link(&w.mobys[id], PARTNER).filter(|&o| o < w.mobys.len()) else { return };
    if w.mobys[id].pv_i(CHAT) == 1 && seq_a(w, id, ctx) == 0x1e { anim(w, id, ctx, 0x1d, 0, 0x14); }
    if w.mobys[id].pv_i(CHAT) == 3 && seq_a(w, o, ctx) == 0x1d { anim(w, id, ctx, 0x1d, 0, 0x23); }
}

/// `0x4527c8` (module docs); its ticks.
fn tail(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    ai::flash(m, FLASH);
    for off in [T270, T272, HEALTH + 6] {
        let mut t = m.pv_s(off) as i32;
        crate::moby::countdown(&mut t);
        m.set_pv_s(off, t as i16);
    }
    if m.state_ticks != -1 { m.state_ticks += 1; }
}
