//! Level 12's trooper 0x0f31 (`0x416580`, 50 placed; its set-up `0x414a00`, senses `0x415600`): legs that ride the
//! level's paths carrying a turret 0x114b (`0x425478`, only its shadow) with two guns 0x0d66 on it
//! (crate::moby::classes::level12::debris, thrown off as it breaks), on the framework ([`crate::moby::ai`]): health
//! `+0x20` (its damage byte `+0x34`, `+0x2e` = 2 a kill asked by another), reaction `+0x50`, movement `+0x100`
//! (its rail speed `+0x128`, path points `+0x1c8` / `+0x1ca`), target `+0x1f0` (its moby `+0x200`, found `+0x204`).
//!
//! Its words: the turret's aim block `+0x210` (list 3), its own break pieces `+0x290`; paths `+0x2b0` (the way in,
//! −1 none), `+0x2b4` (patrol), `+0x2b8` (the second), `+0x2bc` (after the way in), `+0x2c0` (where a replacement
//! comes in); `+0x2cc` the most of its group shooting at once, `+0x2d0` / `+0x2d4` its target areas (`+0x2d8` the
//! one searched), `+0x2dc` the path it is on; timers `+0x2e0` (alert), `+0x2e2`, `+0x2e4` (between shots), `+0x2e6`
//! (between bursts), `+0x2e8` (calm), `+0x300`; `+0x2ea` alarmed, `+0x2eb` shooting (counted in its group's shared
//! word `+0x304`), `+0x2ec` the turret's heading, the guns `+0x2f0` / `+0x2f4` and turret `+0x2f8` (links), the
//! turret's turn speed `+0x2fc`; `+0x302` 1 a reserve (hidden until one of its group dies), 2 taking over;
//! `+0x303` taking over a stand; `+0x308` a second group alarmed with it; `+0x30c` / `+0x30d` the shot's height
//! picks, `+0x30e` patrol after the way in, `+0x30f` its turret's head lost; `+0x310` the reaction's `+0x40`;
//! `+0x314` the area that shows it (−1 none), `+0x31c` show (1) / hide (−1) asked, `+0x31d` hidden, `+0x31e` its
//! update distance, `+0x31f` a reserve's first tick; the guns' aim blocks `+0x320` / `+0x3a0` (lists 7 / 8).
//!
//! The L12 engine functions with the port's: `0x32b628` take_damage, `0x32c370` flight, `0x32f040` find_target,
//! `0x331d30` senses, `0x331ee8` / `0x331d98` the path walk's start and step (crate::moby::ai::follow_path, its
//! words `+0x1c4..`), `0x330c28` / `0x330dc0` move_step / hold, `0x322c48` turn_ease, `0x327a50` the joint aim
//! (crate::moby::classes::level12::head's), `0x328018` = L00 `0x3253f0` (a part at a joint), `0x32eb50` = L00
//! `0x32a848` (crate::level::LevelData::in_area), `0x2efc98` the teleport-in (crate::moby::classes::teleport),
//! `0x32dad0` = L00 `0x32a2e0` the break pieces ([`part_break`]; `0x2df358` = L00 `0x2dbdb0`, crate::moby::shards),
//! `0x337df0` = L00 `0x332c10` (crate::particles::trail), `0x3a7fb8` the shot (crate::moby::classes::level12::shot),
//! `0x323f50` = L00 `0x320538` (crate::moby::blast::explode), `0x31e7c8` = L00 `0x31adc0` (crate::moby::bolts).
//!
//! Not ported: the level's floor gate (`0x1ca300` / `0x1ca30c` / `0x1ca328` / `0x1ca338`: updates kept to one floor by
//! z, set by no class the port has), the colour put back when it is shown (`0x316d58` with `0x1b3d18..`) and the
//! colour moby `+0x4a0`, the path lengths written into the points' `w` (`0x347e98`: read by nothing here), the break
//! pieces' own update (`0x3a1e00`), the big-head cheat (`0x32a7b0`, `$gp+0x2818` 2.5) and the debug prints.

use crate::hero::physics::{add, add_rot, atan2, dist, dist2, dot, len2, scale, set_len, sub, sub_rot, turn_ease, V3};
use crate::moby::classes::props::{seq, wrapped};
use crate::moby::{ai, countdown, mode, Ctx, MobyWorld};

const H: usize = 0x20;
const REACT: usize = 0x50;
const MV: usize = 0x100;
const SPEED: usize = MV + 0x28;
const AT: usize = MV + 0xc8;
const LAST: usize = MV + 0xca;
const T_POINT: usize = 0x1f0;
const T_MOBY: usize = 0x200;
const FOUND: usize = 0x204;
const AIM_TURRET: usize = 0x210;
const PARTS: usize = 0x290;
const P_IN: usize = 0x2b0;
const P_PATROL: usize = 0x2b4;
const P_SECOND: usize = 0x2b8;
const P_AFTER: usize = 0x2bc;
const P_RESPAWN: usize = 0x2c0;
const MOST: usize = 0x2cc;
const AREA_A: usize = 0x2d0;
const AREA_B: usize = 0x2d4;
const AREA: usize = 0x2d8;
const PATH: usize = 0x2dc;
const T_ALERT: usize = 0x2e0;
const T_2E2: usize = 0x2e2;
const T_SHOT: usize = 0x2e4;
const T_BURST: usize = 0x2e6;
const T_CALM: usize = 0x2e8;
const ALARMED: usize = 0x2ea;
const SHOOTING: usize = 0x2eb;
const HEADING: usize = 0x2ec;
const GUNS: [usize; 2] = [0x2f0, 0x2f4];
const TURRET: usize = 0x2f8;
const TURRET_V: usize = 0x2fc;
const T300: usize = 0x300;
const FLAGS: usize = 0x302;
const STAND: usize = 0x303;
const SHARED: usize = 0x304;
const GROUP2: usize = 0x308;
const PICK_C: usize = 0x30c;
const PICK_D: usize = 0x30d;
const PATROL_AFTER: usize = 0x30e;
const HEADLESS: usize = 0x30f;
const SHOW_AREA: usize = 0x314;
const SHOW: usize = 0x31c;
const HIDDEN: usize = 0x31d;
const UPD: usize = 0x31e;
const FIRST: usize = 0x31f;
const AIM_GUNS: [usize; 2] = [0x320, 0x3a0];
const PVARS: usize = 0x500;

pub const CLASS: i16 = 0x0f31;
pub const TURRET_CLASS: i16 = 0x114b;
pub const GUN_CLASS: i16 = 0x0d66;
/// `$gp+0x2808`: the turret's break thresholds (by piece, of 255 health).
const THRESHOLDS: [u8; 10] = [0xe9, 0xd4, 0xaa, 0x7f, 1, 1, 0x55, 1, 1, 1];

fn link(w: &MobyWorld, id: usize, off: usize, class: i16) -> Option<usize> {
    ai::link(&w.mobys[id], off).filter(|&k| w.mobys.get(k).is_some_and(|m| m.o_class == class && m.state < 0xfd))
}
fn turret(w: &MobyWorld, id: usize) -> Option<usize> { link(w, id, TURRET, TURRET_CLASS) }
fn gun(w: &MobyWorld, id: usize, k: usize) -> Option<usize> { link(w, id, GUNS[k], GUN_CLASS) }
fn st(w: &mut MobyWorld, id: usize, s: u8, sub: i32) { w.mobys[id].set_state(s, sub); }
fn rr(ctx: &mut Ctx, a: i32, b: i32) -> i32 { ctx.game.rng.rand_range(a, b) }
fn seq_a(w: &MobyWorld, id: usize, ctx: &Ctx) -> u8 { ctx.host.anim(&w.mobys[id]).map_or(0xff, |a| a.seq_a) }
fn t_pos(w: &MobyWorld, id: usize, ctx: &Ctx) -> V3 {
    ai::link(&w.mobys[id], T_MOBY).and_then(|t| w.mobys.get(t)).map_or(ctx.hero.pos, |t| t.pos)
}
/// `0x32eaf8`: the first tick since the last sub-state change.
fn first_sub(w: &mut MobyWorld, id: usize) -> bool {
    let m = &mut w.mobys[id];
    if m.once & 2 != 0 { return false; }
    m.once |= 2;
    true
}
/// `0x323e58`: an angle wrapped into −π..π.
fn wrap(a: f32) -> f32 { ((a + std::f32::consts::PI) * 0.159_154_94).fract().rem_euclid(1.0) * std::f32::consts::TAU - std::f32::consts::PI }
fn found(w: &MobyWorld, id: usize) -> bool { w.mobys[id].pv_i(FOUND) == 1 }
fn tick_s(w: &mut MobyWorld, id: usize, off: usize) -> bool {
    let mut t = w.mobys[id].pv_s(off) as i32;
    let out = countdown(&mut t);
    w.mobys[id].set_pv_s(off, t as i16);
    out
}
fn path_pts<'a>(ctx: &'a Ctx, path: i32) -> Option<&'a Vec<[f32; 4]>> { usize::try_from(path).ok().and_then(|p| ctx.level.paths.get(p)).filter(|p| !p.is_empty()) }
fn pt(p: &[[f32; 4]], i: i32) -> V3 { let q = p[(i.max(0) as usize).min(p.len() - 1)]; [q[0], q[1], q[2]] }

/// `0x4142a8(m, seq, frame, blend)`: the limits (sequence 0xc: 10 / 2, else 16 / 6; the turn 1440 / 720), the brake
/// doubled; the reaction's sequences and speeds for the falls 0xd..0xf and the jump 0x10; the sequence on the legs and
/// the turret when they have another.
fn anim(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, s: u8, frame: u8, blend: i32) {
    let (a, b) = if s == 0xc { (10.0, 2.0) } else { (16.0, 6.0) };
    let m = &mut w.mobys[id];
    ai::senses(m, a * 0.000_277_777_78, b * 0.016_666_668, 1440.0 * 4.848_137_5e-6, 720.0 * 2.908_882_5e-4);
    m.set_pv_f(MV + 0x18, m.pv_f(MV + 0x18) * 2.0);
    if (0xd..=0xf).contains(&s) {
        for (k, v) in [(0x66, 0xf), (0xc2, 7), (0xc3, 0x13), (0xc6, 1), (0x61, 0xf), (0x62, 0xf), (0xbc, 7), (0xbd, 0x13), (0xbe, 7), (0xbf, 0x13)] { m.set_pv_b(k, v); }
        for k in [0x6c, 0x70, 0x74, 0x78, 0x7c] { m.set_pv_f(k, 0.0); }
    } else if s == 0x10 {
        m.set_pv_f(0x6c, f32::from_bits(0x3ebb_bbbc));
        m.set_pv_f(0x70, f32::from_bits(0x3dee_eef0));
        m.set_pv_f(0x78, f32::from_bits(0x3c08_8889));
        m.set_pv_f(0x7c, f32::from_bits(0x3b88_8889));
        m.set_pv_f(0x74, f32::from_bits(0x3b88_8889));
        for (k, v) in [(0x61, 0x12), (0x66, 0x14), (0xbc, 5), (0xbd, 9), (0xbe, 4), (0xbf, 7), (0xc2, 10), (0xc3, 0x11), (0x62, 0x11), (0xc6, 0)] { m.set_pv_b(k, v); }
    }
    let b = blend.clamp(0, 255) as u8;
    if seq(w, id, ctx) != s { ctx.host.set_anim(&w.mobys[id], s, frame, b); }
    if let Some(t) = ai::link(&w.mobys[id], TURRET) {
        if w.mobys.get(t).is_some() && seq(w, t, ctx) != s { ctx.host.set_anim(&w.mobys[t], s, frame, b); }
    }
}
fn anim_r(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, s: u8) { let b = rr(ctx, 7, 0xc); anim(w, id, ctx, s, 0, b); }

/// `0x414178`: out of its group's shooters (the shared count down, when it was in).
fn release(w: &mut MobyWorld, id: usize) {
    let m = &w.mobys[id];
    if m.group >= 0 && m.pv_b(SHOOTING) != 0 {
        let off = m.pv_i(SHARED);
        let mut v = w.shared_i(off);
        countdown(&mut v);
        w.set_shared_i(off, v);
    }
    w.mobys[id].set_pv_b(SHOOTING, 0);
}
/// `0x4141c0`: one of its group's shooters.
fn take_slot(w: &mut MobyWorld, id: usize) {
    let m = &w.mobys[id];
    if m.group >= 0 && m.pv_b(SHOOTING) == 0 {
        let off = m.pv_i(SHARED);
        let v = w.shared_i(off) + 1;
        w.set_shared_i(off, v);
        w.mobys[id].set_pv_b(SHOOTING, 1);
    }
}

/// `0x414970(m, path, last)`: onto `path` (−1: off any, its point −1) from its nearest point (`0x347dd0`), to `last`
/// (−1 its end); the walk's words (`0x331ee8`).
fn set_path(w: &mut MobyWorld, id: usize, ctx: &Ctx, path: i32, last: i32) {
    let m = &mut w.mobys[id];
    m.set_pv_i(PATH, path);
    if path == -1 { m.set_pv_s(AT, -1); return; }
    let Some(p) = path_pts(ctx, path) else { m.set_pv_s(AT, 0); return };
    let pos = m.pos;
    let near = (0..p.len()).fold((0usize, 1e11f32), |(bi, bd), i| { let d = dist(pt(p, i as i32), pos).abs(); if bd <= d { (bi, bd) } else { (i, d) } }).0;
    m.set_pv_i(MV + 0xc4, path + 1);
    m.set_pv_s(LAST, if last == -1 { p.len() as i16 - 1 } else { last as i16 });
    m.set_pv_s(AT, near as i16);
}

/// `0x347fa0(d, m, path, &pos, i, dir)`: its place moved along the path: onto the line from point `i` toward the
/// next one in `dir` (the ends wrapping round), `d` past its foot on it; past that segment, onto the next. Returns
/// the point it is now from.
fn slide(w: &mut MobyWorld, id: usize, p: &[[f32; 4]], d: f32, i: i32, dir: i32) -> i32 {
    let n = p.len() as i32;
    let next = |i: i32| if i == n - 1 { if dir < 1 { n - 2 } else { 0 } } else if i == 0 { if dir < 1 { n - 1 } else { 1 } } else { i + dir };
    let mut i = i;
    let j = next(i);
    let seg = sub(pt(p, j), pt(p, i));
    let l = crate::hero::physics::len(seg);
    let mut u = set_len(seg, 1.0);
    let mut t = dot(sub(w.mobys[id].pos, pt(p, i)), u) + d;
    if l < t {
        let k = next(j);
        u = set_len(sub(pt(p, k), pt(p, j)), 1.0);
        t -= l;
        i = j;
    }
    w.mobys[id].pos = add(pt(p, i), scale(u, t));
    i
}

/// `0x414848(m, path)`: along the path at its rail speed toward its last point; facing its next point (at the end the
/// target's point), eased (`0x322c48` with its limits). True at the last point (or off any path).
fn rail(w: &mut MobyWorld, id: usize, ctx: &Ctx, path: i32) -> bool {
    let Some(p) = path_pts(ctx, path).cloned() else { w.mobys[id].set_pv_s(AT, -1); return true };
    let m = &w.mobys[id];
    let (at, last) = (m.pv_s(AT) as i32, m.pv_s(LAST) as i32);
    let dir = if at < last { 1 } else { -1 };
    let speed = m.pv_f(SPEED);
    let i = slide(w, id, &p, speed, at, dir);
    let m = &mut w.mobys[id];
    m.set_pv_s(AT, i as i16);
    let done = i == last;
    let to = if i + 1 <= p.len() as i32 - 1 { pt(&p, i + 1) } else { m.pv_v(T_POINT) };
    let yaw = atan2(to[0] - m.pos[0], to[1] - m.pos[1]);
    let (a, b, c) = (m.pv_f(MV + 0x14), m.pv_f(MV + 0x18), m.pv_f(MV + 0x1c));
    let (mut r, mut v) = (m.rot[2], m.pv_f(MV + 0xb4));
    turn_ease(yaw, a, b, c, &mut r, &mut v, 0);
    m.rot[2] = r;
    m.set_pv_f(MV + 0xb4, v);
    done
}

/// `0x4162c8`: its walk turned back: toward its path's end from the start (true), else toward the start.
fn reverse(w: &mut MobyWorld, id: usize, ctx: &Ctx) -> bool {
    let m = &mut w.mobys[id];
    if m.pv_s(LAST) == 0 {
        let n = path_pts(ctx, m.pv_i(PATH)).map_or(-1, |p| p.len() as i32 - 1);
        m.set_pv_s(LAST, n as i16);
        true
    } else {
        m.set_pv_s(LAST, 0);
        false
    }
}

/// `0x415f10(m, aim, from_end, low)`: a clear shot — its head lost, always; else the line from 1.76 above it (`low`:
/// 1.0; `from_end`: above its path's last point) to 1.18 above its target (flags 0) meets nothing or the target; with
/// `aim`, a 0.5 sphere where the line stopped (flags 1) meeting the world or a moby not of the kind 5.
fn can_shoot(w: &MobyWorld, id: usize, ctx: &Ctx, aim: bool, from_end: bool, low: bool) -> bool {
    let m = &w.mobys[id];
    if m.pv_b(HEADLESS) != 0 { return true; }
    let mut from = m.pos;
    if from_end && m.pv_i(PATH) != -1 && m.pv_s(LAST) != -1 {
        if let Some(p) = path_pts(ctx, m.pv_i(PATH)) { from = pt(p, m.pv_s(LAST) as i32); }
    }
    from[2] += if low { 1.0 } else { 1.76 };
    let t = ai::link(m, T_MOBY);
    let mut to = t_pos(w, id, ctx);
    to[2] += 1.18;
    let Some(c) = ctx.coll else { return true };
    // The live mobys: its own turret's collision is off for its update.
    let live = |k: usize| w.mobys.get(k).is_some_and(|m| m.has_collision && m.state < 0xfd);
    let line = crate::collision::coll_line_live(c, ctx.scene, from, to, crate::collision::QueryFlags(0), Some(id), &live);
    let mut clear = line.as_ref().is_none_or(|o| o.moby.is_some() && o.moby == t);
    if aim && !clear {
        let at = line.as_ref().map_or(to, |o| o.point);
        if let Some(o) = crate::collision::coll_sphere_live(c, ctx.scene, at, 0.5, crate::collision::QueryFlags(1), Some(id), &live) {
            let kind = o.moby.and_then(|k| w.mobys.get(k)).and_then(|k| w.class(k.o_class)).map(|c| c.kind);
            if kind != Some(5) { clear = true; }
        }
    }
    clear
}

/// `0x4160b8`: a shot when the timer between them is out: from the gun of the half of ten ticks it is in (its piece
/// 4 / 5 still on the turret), at 10 a second toward the target's point (far from it and not low: at most 2 a second
/// up or down), its damage byte.
fn fire(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let far = 3.0 <= dist2(w.mobys[id].pos, t_pos(w, id, ctx));
    let Some(t) = turret(w, id) else { return };
    if !tick_s(w, id, T_SHOT) { return; }
    let k = ((w.mobys[id].state_ticks as u16 as i32) / 10 & 1) as usize;
    if (w.mobys[t].bangles >> (4 + k)) & 1 == 0 {
        w.mobys[id].set_pv_s(T_SHOT, 10);
        return;
    }
    let low = w.mobys[id].pv_b(PICK_D) != 0;
    if !can_shoot(w, id, ctx, true, false, low) { return; }
    let Some(g) = gun(w, id, k) else { return };
    w.mobys[id].set_pv_s(T_SHOT, 10);
    let Some(p) = ctx.host.joint(&w.mobys[g], 0) else { return };
    let speed = 10.0 * 0.016_666_668;
    let mut v = set_len(sub(w.mobys[id].pv_v(T_POINT), p), speed);
    if far && !low { v[2] = v[2].clamp(-0.033_333_335, 0.033_333_335); }
    let v = set_len(v, speed);
    let dmg = w.mobys[id].pv_b(0x34) as f32;
    super::shot::fire(w, ctx, dmg, id, v, p);
}

/// `0x415300` / `0x415380`: every live trooper of its group and of its second group alarmed (`+0x2ea`).
fn alarm(w: &mut MobyWorld, id: usize) {
    for g in [w.mobys[id].group as i32, w.mobys[id].pv_i(GROUP2)] {
        let Some(list) = usize::try_from(g).ok().and_then(|g| w.groups.get(g)).cloned() else { continue };
        for k in list {
            if let Some(m) = w.mobys.get_mut(k).filter(|m| m.state < 0x80 && m.o_class == CLASS) { m.set_pv_b(ALARMED, 1); }
        }
    }
}

/// `0x415138`: its replacement — the reserve (`+0x302` & 1) of its group nearest the first point of its
/// replacement path takes its paths and areas, alarmed, the alert timer 1000, onto that path, taking over (2).
fn hand_off(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let m = &w.mobys[id];
    let Some(start) = path_pts(ctx, m.pv_i(P_RESPAWN)).map(|p| pt(p, 0)) else { return };
    let Some(list) = usize::try_from(m.group).ok().and_then(|g| w.groups.get(g)) else { return };
    let mut best: Option<(usize, f32)> = None;
    for &k in list {
        let Some(o) = w.mobys.get(k).filter(|o| o.state < 0x80 && o.o_class == CLASS) else { continue };
        let d = dist2(o.pos, start);
        if o.pv_b(FLAGS) & 1 != 0 && best.is_none_or(|(_, b)| d < b) { best = Some((k, d)); }
    }
    let Some((k, _)) = best else { return };
    let src: Vec<i32> = [P_IN, P_PATROL, P_SECOND, P_AFTER, P_RESPAWN, AREA_A, AREA_B].iter().map(|&o| m.pv_i(o)).collect();
    let stand = m.pv_b(STAND);
    let o = &mut w.mobys[k];
    for (off, v) in [P_IN, P_PATROL, P_SECOND, P_AFTER, P_RESPAWN, AREA_A, AREA_B].into_iter().zip(src.iter().copied()) { o.set_pv_i(off, v); }
    o.set_pv_s(T_ALERT, 1000);
    o.set_pv_b(ALARMED, 1);
    o.set_pv_i(PATH, o.pv_i(P_RESPAWN));
    o.set_pv_b(FLAGS, 2);
    o.set_pv_b(STAND, stand);
}

/// `0x32dad0(m, b, health)`: the break pieces of `b` (`+0..` a threshold of 255 health by piece, `+0x10` the health
/// last seen): the first time with health, the pieces with a threshold below it shown (`+0x7c`); a piece shown
/// whose threshold the health falls under breaks off (`0x2df358` mode 1: 90 a second spin, 15 gravity, 2..4 a second
/// 90..120° either way of the camera's heading and 5..8 up), with the level's sound 7 for the first.
fn part_break(w: &mut MobyWorld, ctx: &mut Ctx, id: usize, b: usize, h: usize) -> bool {
    let m = &w.mobys[id];
    let hp = m.pv_f(h);
    let most = m.pv_s(h + 4).max(1) as f32;
    let byte = |hp: f32| (hp.max(0.0) / most * 255.0) as i32 as u8;
    if m.pv_f(b + 0x10) == 0.0 && hp != 0.0 {
        let n = w.class(m.o_class).map_or(0, |c| c.pieces.len()).min(15);
        let mut bits = m.bangles;
        for k in 0..n {
            let t = m.pv_b(b + k);
            if t != 0 && t < byte(hp) { bits |= 1 << k; }
        }
        w.mobys[id].bangles = bits;
    }
    let mut broke = false;
    if hp < w.mobys[id].pv_f(b + 0x10) {
        for k in 0..15u8 {
            let m = &w.mobys[id];
            if m.bangles & (1 << k) == 0 || m.pv_b(b + k as usize) <= byte(hp) { continue; }
            let cam = ctx.host.camera_rows()[0];
            let rng = &mut ctx.game.rng;
            let a = add_rot(rng.randf_signed(90.0, 120.0) * 0.017_453_294, atan2(cam[0], cam[1]));
            let s = rng.randf(2.0, 4.0) * 0.016_666_668;
            let up = rng.randf(5.0, 8.0) * 0.016_666_668;
            let (class, pos, rot) = (m.o_class, m.pos, m.rot);
            crate::moby::shards::spawn(w, ctx, class, Some(id), pos, rot, None, 90.0, 15.0, k, [a.cos() * s, a.sin() * s, up], 0xff, 1);
            w.mobys[id].bangles &= !(1 << k);
            if !broke { ctx.host.play_level_sound(None, 7, 0); }
            broke = true;
        }
    }
    w.mobys[id].set_pv_f(b + 0x10, hp);
    broke
}

/// `0x4144f0`: the turret and guns gone.
fn drop_parts(w: &mut MobyWorld, id: usize) {
    for off in [TURRET, GUNS[0], GUNS[1]] {
        if let Some(k) = ai::link(&w.mobys[id], off) {
            if w.mobys.get(k).is_some_and(|m| m.state < 0xfd) { w.delete(k); }
            ai::set_link(&mut w.mobys[id], off, None);
        }
    }
}

fn blast(w: &mut MobyWorld, ctx: &mut Ctx, id: usize) {
    let e = crate::moby::blast::Explosion { flash: 2.0, flash2: 1.0, near: 9.0, size: 1.0, embers: 4, sound: -1, lod: -1, flags: 0x81_0001, ..Default::default() };
    crate::moby::blast::explode(w, ctx, Some(id), None, &e);
}

fn show(w: &mut MobyWorld, k: usize, on: bool, coll: bool) {
    let has = w.class(w.mobys[k].o_class).is_some_and(|c| c.has_collision);
    let m = &mut w.mobys[k];
    if on { m.mode &= !1; m.drawn = 1; } else { m.mode |= 1; m.drawn = 0; }
    if coll { m.has_collision = on && has; }
}

/// `0x414a00`: the set-up (module docs): its scale × 1.3; the reaction (sequences 0xe / 0x12 / 0x11 / 0x14, keys,
/// gravities, speed 0.45, `+0x90` its `+0x310`); the health block (`+0x28` 3, `+0x2a` 2, `+0x2e` 1, `+0x2f` 8,
/// `+0x30` 1.7, `+0x48` 25, `+0x4a` 16); movement (radius 0.7, height / steps 0.5, slope 45°, turn first 20°,
/// gravity 9.8 a second², options | 0x80); the calm limits; its turret (its distances, colour, glow, scale × 1.3, at
/// its joint 0, its health block, `+0x30` 0.7, `+0x2a` 2, `+0x2f` 4, `+0x4a` 3, `+0x4d` 1, its owner `+0x70`; its
/// break thresholds, one pair of them random by its health) with the guns at its joints 0 and 1 (0x0d66, no update);
/// a reserve flag kept; onto its patrol path; out of the shooters, its group's count 0; with a way in: onto it (1, sub
/// 1, sequence 0xd), else 5 (sub 1, one of sequences 0 / 1); the area searched `+0x2d0`.
fn setup(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    if m.pvars.len() < PVARS { m.pvars.resize(PVARS, 0); }
    m.scale *= 1.3;
    for (k, v) in [(0x6c, 0x3ebb_bbbcu32), (0x70, 0x3dee_eef0), (0x78, 0x3c08_8889), (0x7c, 0x3b88_8889), (0x8c, 0x3ecc_cccd), (0xf8, 0x3ee6_6666), (0x74, 0x3b88_8889), (0x80, 0x3f33_3333), (0x84, 0x3f33_3333), (0x30, 0x3fd9_999a)] { m.set_pv_f(k, f32::from_bits(v)); }
    for (k, v) in [(0xbc, 5u8), (0xbd, 9), (0xbf, 7), (0xc3, 0x11), (0x60, 0xe), (0x61, 0x12), (0x66, 0x14), (0x67, 0xff), (0xc2, 10), (0x62, 0x11), (0xbe, 4), (0x28, 3), (0x2a, 2), (0x2e, 1), (0x4d, 0), (0x2f, 8), (0x48, 25), (0x4a, 16), (0x1e8, 0xc), (0x1e9, 0xb)] { m.set_pv_b(k, v); }
    m.set_pv_f(0x90, m.pv_f(0x310));
    m.set_pv_i(0x98, 9);
    for off in [TURRET, GUNS[0], GUNS[1]] { m.set_pv_i(off, 0); }
    for (k, v) in [(0x100, 0x3f33_3333u32), (0x10c, 0x3f00_0000), (0x104, 0x3f00_0000), (0x108, 0x3f00_0000), (0x110, 0x3f49_0fdc), (0x130, 0x3eb2_b8c4)] { m.set_pv_f(k, f32::from_bits(v)); }
    m.set_pv_f(0x12c, 9.8 * 0.000_277_777_78);
    ai::senses(m, 10.0 * 0.000_277_777_78, 2.0 * 0.016_666_668, 1440.0 * 4.848_137_5e-6, 720.0 * 2.908_882_5e-4);
    m.set_pv_i(0x1dc, 0);
    m.set_pv_i(MV + 0x38, m.pv_i(MV + 0x38) | 0x80);
    m.set_pv_f(0x1d4, 3.0);
    m.set_pv_s(0x1e4, 0);
    m.set_pv_s(0x1e2, 0);
    m.set_pv_i(0x1d0, 0);
    make_turret(w, id, ctx);
    let m = &mut w.mobys[id];
    let f = m.pv_b(FLAGS);
    if f != 0 && f & 2 == 0 { m.set_pv_b(FLAGS, 1); }
    let p = m.pv_i(P_PATROL);
    set_path(w, id, ctx, p, -1);
    release(w, id);
    let off = w.mobys[id].pv_i(SHARED);
    w.set_shared_i(off, 0);
    if w.mobys[id].pv_i(P_IN) == -1 {
        st(w, id, 5, 1);
        let s = (ctx.game.rng.rand() & 1) as u8;
        let b = rr(ctx, 10, 0x14);
        anim(w, id, ctx, s, 0, b);
    } else {
        let m = &mut w.mobys[id];
        m.set_pv_i(PATH, m.pv_i(P_IN));
        m.set_pv_i(MV + 0x38, m.pv_i(MV + 0x38) | 4);
        st(w, id, 1, 1);
        let b = rr(ctx, 10, 0x14);
        anim(w, id, ctx, 0xd, 0, b);
    }
    let m = &mut w.mobys[id];
    m.set_pv_i(AREA, m.pv_i(AREA_A));
}

fn make_turret(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let t = w.create(TURRET_CLASS);
    ai::set_link(&mut w.mobys[id], TURRET, Some(t));
    let (draw, upd, light, ambient, glow, rot) = { let m = &w.mobys[id]; (m.draw_dist, m.update_dist, m.light, m.ambient, m.glow, m.rot) };
    let at = ctx.host.joint(&w.mobys[id], 0).unwrap_or(w.mobys[id].pos);
    let health: Vec<u8> = w.mobys[id].pvars[H..H + 0x30].to_vec();
    let tm = &mut w.mobys[t];
    tm.draw_dist = draw;
    tm.update_dist = upd;
    tm.light = light;
    tm.ambient = ambient;
    tm.glow = glow;
    tm.drawn = 1;
    tm.scale *= 1.3;
    tm.pos = at;
    tm.rot = rot;
    tm.pvars[H..H + 0x30].copy_from_slice(&health);
    tm.set_pv_f(0x30, f32::from_bits(0x3f33_3333));
    for (k, v) in [(0x2a, 2u8), (0x2f, 4), (0x4a, 3), (0x4d, 1)] { tm.set_pv_b(k, v); }
    ai::set_link(tm, 0x70, Some(id));
    w.rebuild(t, &*ctx.host);
    for k in 0..2 {
        let p = ctx.host.joint(&w.mobys[t], k).unwrap_or(at);
        let off = sub(p, w.mobys[t].pos);
        let g = super::debris::spawn(w, ctx, t, GUN_CLASS, off, [0.0; 3], 0, 0.0, None, None);
        let gm = &mut w.mobys[g];
        gm.draw_dist = draw;
        gm.mode |= mode::NO_UPDATE;
        ai::set_link(&mut w.mobys[id], GUNS[k], Some(g));
    }
    let tm = &mut w.mobys[t];
    for (k, v) in THRESHOLDS.iter().enumerate() { tm.set_pv_b(0x50 + k, *v); }
    let most = tm.pv_s(0x24).max(1) as f32;
    let (two, one) = (((2.0 / most) * 255.0) as i32 as u8, ((1.0 / most) * 255.0) as i32 as u8);
    match ctx.game.rng.rand() % 3 {
        0 => { tm.set_pv_b(0x54, two); tm.set_pv_b(0x58, one); }
        1 => { tm.set_pv_b(0x55, two); tm.set_pv_b(0x59, one); }
        _ => { tm.set_pv_b(0x57, two); }
    }
    part_break(w, ctx, t, 0x50, 0x20);
    w.mobys[t].bangles &= 0xfcff;
}

/// `0x415600` (module docs): the set-up in sub 0; a hit on its turret taken as its own; the frame's damage — any:
/// its groups alarmed; dead with its turret: no target, the turret's pieces 8 and 9 shown and its health 0, its
/// break pieces, its replacement, the turret's blast, the parts gone, and (from state 4 on, half the time) the fall
/// 0x11 with its reaction's speeds, else no knock; hurt: the turret's health its own, its pieces broken, its
/// sequence its own, its head lost (piece 7 gone: the stagger 0xd sub 8), pieces 8 / 9 shown with 4 / 5 gone, a gun
/// whose piece is gone let go; its shadow; the target search in its area; the point it aims at.
fn senses(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].sub_state == 0 { setup(w, id, ctx); }
    if let Some(t) = turret(w, id) {
        if w.mobys[id].hit_slot == -1 && w.hit_message(t).is_some() {
            let slot = w.mobys[t].hit_slot;
            w.mobys[id].hit_slot = slot;
            if let Some(r) = usize::try_from(slot).ok().and_then(|s| w.hits.records.get_mut(s)) { r.target = id; }
        }
    }
    let kind = ai::take_damage(w, id, ctx, false);
    if kind != 0 {
        alarm(w, id);
        if let Some(t) = turret(w, id) {
            let health: Vec<u8> = w.mobys[id].pvars[H..H + 0x30].to_vec();
            if kind == 4 {
                w.mobys[id].mode &= !mode::TARGET;
                w.mobys[t].mode &= !mode::TARGET;
                w.mobys[t].bangles |= 0x300;
                w.mobys[id].set_pv_f(H, 0.0);
                let health: Vec<u8> = w.mobys[id].pvars[H..H + 0x30].to_vec();
                w.mobys[t].pvars[H..H + 0x30].copy_from_slice(&health);
                part_break(w, ctx, t, 0x50, 0x20);
                hand_off(w, id, ctx);
                blast(w, ctx, t);
                drop_parts(w, id);
                if w.mobys[id].state < 4 || ctx.game.rng.randi(2) != 0 {
                    w.mobys[id].set_pv_v(REACT + 0x80, [0.0; 3]);
                    w.mobys[id].set_pv_f(REACT + 0x8c, 0.0);
                } else {
                    if seq(w, id, ctx) != 0x11 { ctx.host.set_anim(&w.mobys[id], 0x11, 0, 5); }
                    let m = &mut w.mobys[id];
                    m.set_pv_i(REACT + 0x7c, -1);
                    m.set_pv_f(REACT + 0x60, m.pv_b(0xbf) as i8 as f32);
                    m.set_pv_f(REACT + 0x54, m.pv_b(0xbe) as i8 as f32);
                    let v = m.pv_v(REACT + 0x80);
                    m.set_pv_v(REACT + 0x80, scale(v, 0.05));
                }
                for k in 0..2 {
                    if let Some(g) = ai::link(&w.mobys[id], GUNS[k]).filter(|&g| g < w.mobys.len()) {
                        let gm = &mut w.mobys[g];
                        gm.mode &= !(mode::NO_UPDATE | 1);
                        gm.drawn = 1;
                    }
                    ai::set_link(&mut w.mobys[id], GUNS[k], None);
                }
            } else {
                w.mobys[t].pvars[H..H + 0x30].copy_from_slice(&health);
                part_break(w, ctx, t, 0x50, 0x20);
                let s = seq(w, id, ctx);
                ctx.host.set_anim(&w.mobys[t], s, 0, 3);
                let speed = ctx.host.anim(&w.mobys[id]).map_or(1.0, |a| a.speed);
                ctx.host.set_anim_speed(&w.mobys[t], speed);
                if w.mobys[t].bangles & 0x80 == 0 {
                    w.mobys[id].set_pv_b(HEADLESS, 1);
                    if w.mobys[id].sub_state != 9 { st(w, id, 0xd, 8); }
                }
                let tm = &mut w.mobys[t];
                if tm.bangles & 0x10 == 0 { tm.bangles |= 0x100; }
                if tm.bangles & 0x20 == 0 { tm.bangles |= 0x200; }
                for k in 0..2 {
                    if (w.mobys[t].bangles >> (k + 4)) & 1 != 0 { continue; }
                    if let Some(g) = ai::link(&w.mobys[id], GUNS[k]).filter(|&g| g < w.mobys.len()) { w.mobys[g].mode &= !mode::NO_UPDATE; }
                    ai::set_link(&mut w.mobys[id], GUNS[k], None);
                }
            }
        }
    }
    if w.mobys[id].drawn != 0 { crate::shadow_volumes::probe_near(w, id, ctx, 24.0, 0x16); }
    let area = w.mobys[id].pv_i(AREA);
    ai::find_target(w, id, ctx, 0.0, 0.0, area, None, false);
    aim_point(w, id, ctx);
}

/// `0x415480`: the point it aims at: at Ratchet his aim point (within 5 of him his centre), 0.4 up (0.2 in his state
/// 0xc); at another its place 0.8 up; its head lost, 4 along the turret's facing from 0..1 above itself.
fn aim_point(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let headless = w.mobys[id].pv_b(HEADLESS) != 0;
    let p = match turret(w, id).filter(|_| headless) {
        None => {
            let t = ai::link(&w.mobys[id], T_MOBY);
            if t.is_none() || t == ctx.hero_moby {
                let mut p = if dist2(w.mobys[id].pos, ctx.hero.pos) <= 5.0 { add(ctx.hero.pos, [0.0, 0.0, 1.0]) } else { ctx.hero.aim_point };
                p[2] += if ctx.hero.state == 0xc { 0.2 } else { 0.4 };
                p
            } else {
                add(t_pos(w, id, ctx), [0.0, 0.0, 0.8])
            }
        }
        Some(t) => {
            let mut p = w.mobys[id].pos;
            p[2] += ctx.game.rng.randf(0.0, 1.0);
            add(p, scale(w.mobys[t].rows[0], 4.0))
        }
    };
    w.mobys[id].set_pv_v(T_POINT, p);
}

/// `0x327a50`'s spring on the joint list `list` of `dst` from the block `b` of `id`'s pvars (asks cleared after).
fn aim(w: &mut MobyWorld, id: usize, b: usize, dst: usize, list: u8, k: f32) {
    let mut a = [0.0; 3];
    let m = &mut w.mobys[id];
    let t: [f32; 3] = std::array::from_fn(|i| m.pv_f(b + 0x60 + 4 * i));
    let idle = t == [0.0; 3] && (0..3).all(|i| m.pv_f(b + 0x40 + 4 * i).abs() < 0.005);
    if !idle {
        for i in 0..3 {
            let (mut x, mut v) = (m.pv_f(b + 0x40 + 4 * i), m.pv_f(b + 0x50 + 4 * i));
            crate::hero::physics::turn_spring(t[i], k, 0.3, 0.0, &mut x, &mut v, 0);
            m.set_pv_f(b + 0x40 + 4 * i, x);
            m.set_pv_f(b + 0x50 + 4 * i, v);
            m.set_pv_f(b + 0x60 + 4 * i, 0.0);
            a[i] = x;
        }
    }
    let d = &mut w.mobys[dst];
    d.manips.retain(|x| x.0 != list);
    if !idle { d.manips.push((list, crate::quat::from_rows(&crate::hero::physics::euler_rows(a)))); }
}

/// `0x415ac8`: the turret at its joint 0 with its colour, turned toward `+0x2ec` (its limits), playing at its speed;
/// in states 1..3 its thrusters ([`thrusters`]); seeing its target with its head, its aim at the target's point
/// (pitch and turn); its guns' aims (in its sequences 8..10 at the target's point, gun 0 at most 5° right and gun 1
/// left; its head lost, at random within ±45°).
fn turret_update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let Some(t) = turret(w, id) else { return };
    let amb = w.mobys[id].ambient;
    let at = ctx.host.joint(&w.mobys[id], 0).unwrap_or(w.mobys[id].pos);
    let (a, b, c, want) = { let m = &w.mobys[id]; (m.pv_f(MV + 0x14), m.pv_f(MV + 0x18), m.pv_f(MV + 0x1c), m.pv_f(HEADING)) };
    let (mut r, mut v) = (w.mobys[t].rot[2], w.mobys[id].pv_f(TURRET_V));
    turn_ease(want, a, b, c, &mut r, &mut v, 0);
    w.mobys[id].set_pv_f(TURRET_V, v);
    let tm = &mut w.mobys[t];
    tm.ambient = amb;
    tm.pos = at;
    tm.rot[2] = r;
    let speed = ctx.host.anim(&w.mobys[id]).map_or(1.0, |a| a.speed);
    ctx.host.set_anim_speed(&w.mobys[t], speed);
    if (1..=3).contains(&w.mobys[id].state) { thrusters(w, id, t, ctx); }
    let headless = w.mobys[id].pv_b(HEADLESS) != 0;
    let tp = w.mobys[id].pv_v(T_POINT);
    let turn_of = |p: V3, tm: &crate::moby::Moby| {
        let d = sub(tp, p);
        (-atan2(len2(d), d[2]), sub_rot(atan2(d[0], d[1]), tm.rot[2]))
    };
    if found(w, id) && !headless {
        if let Some(p) = ctx.host.joint(&w.mobys[t], 4) {
            let (pitch, turn) = turn_of(p, &w.mobys[t]);
            let pitch = sub_rot(pitch, w.mobys[t].rot[1]);
            w.mobys[id].set_pv_f(AIM_TURRET + 0x64, pitch);
            w.mobys[id].set_pv_f(AIM_TURRET + 0x68, turn);
        }
    }
    aim(w, id, AIM_TURRET, t, 3, 0.02);
    if !headless {
        let s = seq_a(w, t, ctx);
        if (8..0xb).contains(&s) {
            for k in 0..2 {
                if (w.mobys[t].bangles >> (k + 4)) & 1 == 0 { continue; }
                let Some(p) = ctx.host.joint(&w.mobys[t], k) else { continue };
                let (pitch, mut turn) = turn_of(p, &w.mobys[t]);
                if k == 0 && turn < -0.087_266_475 { turn = f32::from_bits(0xbdb2_b8c4); }
                if k == 1 && 0.087_266_475 < turn { turn = f32::from_bits(0x3db2_b8c4); }
                w.mobys[id].set_pv_f(AIM_GUNS[k] + 0x64, pitch);
                w.mobys[id].set_pv_f(AIM_GUNS[k] + 0x68, turn);
            }
        }
    } else {
        for k in 0..2 {
            let q = f32::from_bits(0x3fc9_0fdb);
            let a = ctx.game.rng.randf(-q, q) * 0.5;
            let b = ctx.game.rng.randf(-q, q) * 0.5;
            w.mobys[id].set_pv_f(AIM_GUNS[k] + 0x64, a);
            w.mobys[id].set_pv_f(AIM_GUNS[k] + 0x68, b);
        }
    }
    for k in 0..2 { aim(w, id, AIM_GUNS[k], t, 7 + k as u8, 0.02); }
}

/// `0x414560`: the turret's thrusters (its joints 5 and 6), down 1.7..3 a second with the legs' velocity: on even
/// frames blue (`0x6000ffff` to `0x80`, 20..24 ticks, 0x50 shrinking), odd ones red (`0xcf0000ff` to `0xcf`, 18..22,
/// 0x28); then a flame at each (`0xefff7f4f` to `0xff0000`, 10..15 ticks, 10..25 shrinking to 0).
fn thrusters(w: &mut MobyWorld, id: usize, t: usize, ctx: &mut Ctx) {
    let down = -(ctx.game.rng.randf(1.7, 3.0) * 0.016_666_668);
    let v = add([0.0, 0.0, down], w.mobys[id].pv_v(MV + 0xa0));
    let (Some(a), Some(b)) = (ctx.host.joint(&w.mobys[t], 5), ctx.host.joint(&w.mobys[t], 6)) else { return };
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let rng = &mut ctx.game.rng;
    for p in [a, b] {
        if ctx.game.frame & 1 == 0 {
            let life = rng.rand_range(0x14, 0x18);
            crate::particles::trail(sys, rng, p, v, 0x6000_ffff, 0x80, life, 0x50, -10, true);
        } else {
            let life = rng.rand_range(0x12, 0x16);
            crate::particles::trail(sys, rng, p, v, 0xcf00_00ff, 0xcf, life, 0x28, -10, true);
        }
    }
    for p in [a, b] {
        let base = rng.rand_range(10, 0x19);
        let life = rng.rand_range(10, 0xf);
        crate::particles::trail(sys, rng, p, v, 0xefff_7f4f, 0xff_0000, life, base as i16, -(base as i16), true);
    }
}

/// `0x415e10`: each gun on its turret's joint (gun 1 mirrored: rows 1 and 2 turned round), shown as the legs are,
/// their colour.
fn guns_update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let Some(t) = turret(w, id) else { return };
    let amb = w.mobys[id].ambient;
    let drawn = w.mobys[id].drawn != 0;
    for k in 0..2 {
        let Some(g) = ai::link(&w.mobys[id], GUNS[k]).filter(|&g| g < w.mobys.len()) else { continue };
        if let Some((r, p)) = ctx.host.joint_matrix(&w.mobys[t], k) {
            let mut r = r.map(|row| set_len(row, 1.0));
            if k == 1 { r[1] = scale(r[1], -1.0); r[2] = scale(r[2], -1.0); }
            ctx.host.advance(&w.mobys[g]);
            let gm = &mut w.mobys[g];
            gm.pos = p;
            gm.rows = r;
            gm.mode |= mode::NO_UPDATE | mode::NO_MATRIX;
        }
        let gm = &mut w.mobys[g];
        if drawn { gm.mode &= !1; gm.drawn = 1; } else { gm.mode |= 1; gm.drawn = 0; }
        gm.ambient = amb;
    }
}

/// `0x4153c0`: alarmed, the alert timer at least 600..1000 when under 300; the calm timer held at 600 while the alert
/// one runs; the timers down; its ticks in the state.
fn timers(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pv_b(ALARMED) != 0 {
        w.mobys[id].set_pv_b(ALARMED, 0);
        if w.mobys[id].pv_s(T_ALERT) < 300 {
            let t = rr(ctx, 600, 1000);
            w.mobys[id].set_pv_s(T_ALERT, t as i16);
        }
    }
    if !tick_s(w, id, T_ALERT) { w.mobys[id].set_pv_s(T_CALM, 600); }
    for off in [T_2E2, T_BURST, T_CALM] { tick_s(w, id, off); }
    let m = &mut w.mobys[id];
    if m.state_ticks != -1 { m.state_ticks += 1; }
}

fn yaw_to(w: &MobyWorld, id: usize, p: V3) -> f32 { let q = w.mobys[id].pos; atan2(p[0] - q[0], p[1] - q[1]) }

/// 0x0f31 (module docs).
pub fn update_0f31(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pvars.len() > H + 0xe && w.mobys[id].pv_b(H + 0xe) == 2 {
        hand_off(w, id, ctx);
        part_break(w, ctx, id, PARTS, H);
        release(w, id);
        drop_parts(w, id);
        blast(w, ctx, id);
        return w.delete(id);
    }
    if ctx.game.scenes.mode == crate::scene::SCENE { return; }
    if w.mobys[id].pvars.len() < PVARS { w.mobys[id].pvars.resize(PVARS, 0); }
    let m = &mut w.mobys[id];
    if m.pv_b(UPD) != 0 && m.update_dist == 0xff { m.update_dist = m.pv_b(UPD); }
    if m.pv_i(SHOW_AREA) != -1 && m.sub_state == 0 {
        if !ctx.level.in_area(m.pv_i(SHOW_AREA), ctx.hero.pos) {
            let v = if m.pv_b(HIDDEN) == 0 { 0xff } else { 0 };
            m.set_pv_b(SHOW, v);
        } else if m.pv_b(SHOW) == 0 {
            m.set_pv_b(SHOW, 1);
        }
    }
    if w.mobys[id].pv_b(FLAGS) & 1 == 0 {
        if w.mobys[id].pv_b(HIDDEN) != 0 {
            let m = &mut w.mobys[id];
            if m.pv_b(FIRST) == 1 {
                m.set_pv_b(FIRST, 0);
                m.state_ticks = 0;
                return;
            }
            if (m.state_ticks as u16 as i32) < 150 {
                m.state_ticks += 1;
                return;
            }
            m.set_pv_b(SHOW, 1);
            senses(w, id, ctx);
            if let Some(start) = path_pts(ctx, w.mobys[id].pv_i(P_RESPAWN)).map(|p| pt(p, 0)) {
                let linked: Vec<usize> = [TURRET, GUNS[0], GUNS[1]].iter().filter_map(|&o| ai::link(&w.mobys[id], o)).collect();
                let rot = w.mobys[id].rot;
                crate::moby::classes::teleport::start(w, id, start, rot, &linked);
            }
        }
    } else {
        let m = &mut w.mobys[id];
        m.set_pv_b(SHOW, 0xff);
        m.set_pv_b(FIRST, 1);
    }
    let c = w.mobys[id].pv_b(SHOW) as i8;
    if c < 0 {
        if w.mobys[id].pv_b(HIDDEN) == 0 {
            show(w, id, false, true);
            if let Some(t) = turret(w, id) { show(w, t, false, true); }
            for k in 0..2 { if let Some(g) = gun(w, id, k) { show(w, g, false, false); } }
        }
        let m = &mut w.mobys[id];
        m.set_pv_b(SHOW, 0);
        m.set_pv_b(HIDDEN, 1);
        return;
    }
    if c == 0 {
        if w.mobys[id].pv_b(HIDDEN) != 0 { return; }
    } else {
        if w.mobys[id].pv_b(HIDDEN) != 0 {
            show(w, id, true, true);
            if let Some(t) = turret(w, id) { show(w, t, true, true); }
            for k in 0..2 { if let Some(g) = gun(w, id, k) { show(w, g, true, false); } }
            w.mobys[id].set_pv_b(HIDDEN, 0);
        }
        w.mobys[id].set_pv_b(SHOW, 0);
    }
    w.mobys[id].has_collision = false;
    if let Some(t) = turret(w, id) { w.mobys[t].has_collision = false; }
    w.mobys[id].drawn = 1;
    w.mobys[id].mode &= !1;
    senses(w, id, ctx);
    if subs(w, id, ctx) { return; }
    if states(w, id, ctx) { return; }
    turret_update(w, id, ctx);
    guns_update(w, id, ctx);
    timers(w, id, ctx);
    let has = w.class(CLASS).is_some_and(|c| c.has_collision);
    if w.mobys[id].state < 0xfd { w.mobys[id].has_collision = has; }
    if let Some(t) = turret(w, id) { w.mobys[t].has_collision = w.class(TURRET_CLASS).is_some_and(|c| c.has_collision); }
}

fn alert_timer(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) { let t = rr(ctx, 600, 1000); w.mobys[id].set_pv_s(T_ALERT, t as i16); }
fn rand_walk(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let s = (ctx.game.rng.rand() & 1) as u8;
    let b = rr(ctx, 10, 0x14);
    anim(w, id, ctx, s, 0, b);
}

/// The sub-states (`+0x95`): 1 the way in, 2 / 3 on the path, 4 the second path, 5 / 6 the patrol, 7 facing its
/// target, 8 wobbling, 9 the hurt's flight. True when it is gone.
fn subs(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    let rotz = w.mobys[id].rot[2];
    let tp = t_pos(w, id, ctx);
    match w.mobys[id].sub_state {
        1 => {
            w.mobys[id].set_pv_f(HEADING, rotz);
            if found(w, id) || w.mobys[id].pv_s(T_ALERT) != 0 {
                let m = &mut w.mobys[id];
                m.set_pv_i(AREA, m.pv_i(AREA_B));
                if m.pv_b(FLAGS) & 1 != 0 {
                    anim_r(w, id, ctx, 0xc);
                    st(w, id, 8, 5);
                } else {
                    alert_timer(w, id, ctx);
                    let s = w.mobys[id].state;
                    st(w, id, s, 2);
                }
            }
        }
        2 | 3 => {
            if w.mobys[id].sub_state == 2 {
                if first_sub(w, id) && w.mobys[id].pv_s(T300) < 5 && w.mobys[id].pv_b(FLAGS) & 2 == 0 {
                    let t = rr(ctx, 0, 0x78);
                    w.mobys[id].set_pv_s(T300, t as i16);
                }
                if tick_s(w, id, T300) {
                    let m = &mut w.mobys[id];
                    m.set_pv_i(AREA, m.pv_i(AREA_B));
                    if m.pv_i(P_IN) == -1 {
                        let p = m.pv_i(P_SECOND);
                        set_path(w, id, ctx, p, -1);
                        anim(w, id, ctx, 2, 0, 10);
                        st(w, id, 4, 3);
                    } else {
                        let p = m.pv_i(P_IN);
                        set_path(w, id, ctx, p, -1);
                        st(w, id, 2, 3);
                    }
                    if w.mobys[id].pv_b(FLAGS) & 2 != 0 {
                        let p = w.mobys[id].pv_i(P_RESPAWN);
                        set_path(w, id, ctx, p, -1);
                    }
                }
            }
            w.mobys[id].set_pv_f(HEADING, rotz);
            if found(w, id) { alert_timer(w, id, ctx); }
        }
        4 => {
            w.mobys[id].set_pv_f(HEADING, rotz);
            if found(w, id) || w.mobys[id].pv_s(T_ALERT) != 0 {
                let p = w.mobys[id].pv_i(P_SECOND);
                set_path(w, id, ctx, p, -1);
                alert_timer(w, id, ctx);
                anim(w, id, ctx, 2, 0, 10);
                st(w, id, 4, 3);
            }
        }
        5 => {
            let y = yaw_to(w, id, tp);
            w.mobys[id].set_pv_f(HEADING, y);
            if w.mobys[id].pv_b(FLAGS) & 2 != 0 {
                rand_walk(w, id, ctx);
                release(w, id);
                alert_timer(w, id, ctx);
                w.mobys[id].set_pv_s(T300, 1);
                let s = w.mobys[id].state;
                st(w, id, s, 2);
            }
            if !found(w, id) && w.mobys[id].pv_s(T_ALERT) == 0 {
                release(w, id);
                rand_walk(w, id, ctx);
                st(w, id, 5, 6);
            }
        }
        6 => {
            w.mobys[id].set_pv_f(HEADING, rotz);
            if found(w, id) || w.mobys[id].pv_s(T_ALERT) != 0 {
                release(w, id);
                alert_timer(w, id, ctx);
                anim_r(w, id, ctx, 3);
                st(w, id, 5, 5);
            } else if w.mobys[id].pv_s(T_CALM) == 0 {
                release(w, id);
                let p = w.mobys[id].pv_i(P_SECOND);
                set_path(w, id, ctx, p, 0);
                rand_walk(w, id, ctx);
                st(w, id, 4, 4);
            }
        }
        7 => {
            let y = yaw_to(w, id, tp);
            w.mobys[id].set_pv_f(HEADING, y);
        }
        8 => {
            let f = ctx.game.frame as f32 * 0.02;
            let a = wrap(f).sin();
            let m = &mut w.mobys[id];
            m.set_pv_f(HEADING, add_rot(m.pv_f(HEADING), (a + 2.0) * 0.023_271_061));
        }
        9 => {
            w.mobys[id].set_pv_f(HEADING, rotz);
            let r = ai::flight(w, id, ctx);
            let (sa, sb) = (seq_a(w, id, ctx), seq(w, id, ctx));
            if w.mobys[id].pv_f(H) <= 0.0 && (sa == 0xf || sb == 0xf) { st(w, id, 0xf, -1); }
            if r & 1 != 0 {
                if 0.0 < w.mobys[id].pv_f(H) { hand_off(w, id, ctx); }
                release(w, id);
                st(w, id, 0xf, -1);
            } else if r & 2 != 0 {
                release(w, id);
                alert_timer(w, id, ctx);
                if sa != 0xf && sb != 0xf {
                    anim_r(w, id, ctx, 0xb);
                    if w.mobys[id].pv_b(HEADLESS) == 0 { st(w, id, 6, 5) } else { st(w, id, 0xd, 8) }
                } else {
                    let p = w.mobys[id].pv_i(P_IN);
                    set_path(w, id, ctx, p, -1);
                    st(w, id, 3, 3);
                    anim_r(w, id, ctx, 0xe);
                }
            }
        }
        _ => {}
    }
    false
}

/// The states (`+0x20`): 1 / 2 / 3 the way in (3 riding it; at its end onto the path after, the jump 0x10, 7), 4 / 5
/// / 12 into the patrol, 6 the walk along its path ([`path_end`]), 7 landing, 8 turning to its target, 9 choosing to
/// shoot, 10 / 11 shooting, 13 staggering (shooting wild), 14 out of the shooters, 15 dead. True when it is gone.
fn states(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    let wr = wrapped(w, id, ctx);
    match w.mobys[id].state {
        1 => { if wr { anim_r(w, id, ctx, 0xe); } }
        2 => {
            if wr {
                anim_r(w, id, ctx, 0xe);
                st(w, id, 3, -1);
            }
        }
        3 => {
            if wr { anim_r(w, id, ctx, 0xe); }
            let p = w.mobys[id].pv_i(PATH);
            if rail(w, id, ctx, p) {
                let m = &mut w.mobys[id];
                m.set_pv_i(MV + 0x38, m.pv_i(MV + 0x38) & !4);
                let p = m.pv_i(P_AFTER);
                set_path(w, id, ctx, p, -1);
                anim_r(w, id, ctx, 0x10);
                st(w, id, 7, 5);
            }
        }
        4 => {
            if wr {
                anim_r(w, id, ctx, 0xb);
                st(w, id, 6, -1);
            }
        }
        5 => {
            if wr {
                anim_r(w, id, ctx, if w.mobys[id].sub_state == 5 { 0xb } else { 0xc });
                st(w, id, 6, -1);
            }
        }
        6 => path_end(w, id, ctx),
        7 => {
            if wr {
                if w.mobys[id].pv_b(PATROL_AFTER) == 0 {
                    anim_r(w, id, ctx, 0xc);
                    st(w, id, 8, 5);
                } else {
                    anim_r(w, id, ctx, 4);
                    st(w, id, 4, 5);
                }
            }
        }
        8 => {
            let tp = w.mobys[id].pv_v(T_POINT);
            let y = yaw_to(w, id, tp);
            let p = w.mobys[id].pos;
            ai::move_step(w, id, ctx, y, p);
            if sub_rot(y, w.mobys[id].rot[2]).abs() < 0.174_532_95 {
                anim_r(w, id, ctx, 5);
                let m = &mut w.mobys[id];
                m.set_pv_b(PICK_D, 0);
                m.set_pv_b(PICK_C, 0);
                st(w, id, 9, -1);
            }
        }
        9 => choose(w, id, ctx, wr),
        10 => {
            if wr {
                if w.mobys[id].pv_b(PICK_D) == 0 { anim(w, id, ctx, 0x13, 0, 1) } else { anim(w, id, ctx, 10, 0, 1) }
                w.mobys[id].set_pv_s(T_SHOT, 10);
                st(w, id, 0xb, -1);
            }
        }
        0xb => {
            let t = w.mobys[id].state_ticks as u16 as i32;
            if 30 < t { fire(w, id, ctx); }
            let t = w.mobys[id].state_ticks as u16 as i32;
            let near = dist2(w.mobys[id].pos, t_pos(w, id, ctx)) <= 3.0;
            if !((t <= 90 || near) && turret(w, id).is_some()) {
                let b = rr(ctx, 0x8c, 0xdc);
                w.mobys[id].set_pv_s(T_BURST, b as i16);
                release(w, id);
                anim_r(w, id, ctx, 0xc);
                st(w, id, 8, 5);
            }
        }
        0xc => {
            if wr {
                anim_r(w, id, ctx, 0xb);
                st(w, id, 6, 5);
            }
        }
        0xd => {
            if wr {
                let s = if ctx.game.rng.rand() & 1 != 0 { 10 } else { 0x13 };
                let b = rr(ctx, 0x19, 0x2d);
                anim(w, id, ctx, s, 0, b);
            }
            let sa = seq_a(w, id, ctx);
            if sa == 10 || sa == 0x13 { fire(w, id, ctx); }
        }
        0xe => release(w, id),
        0xf => {
            w.mobys[id].set_pv_f(H, 0.0);
            part_break(w, ctx, id, PARTS, H);
            release(w, id);
            drop_parts(w, id);
            crate::moby::bolts::drop(w, id, 0, 3, ctx);
            blast(w, ctx, id);
            w.delete(id);
            return true;
        }
        _ => {}
    }
    false
}

/// State 9: within 3 of its target a low shot (10, sequence 8) when one is clear; within 30, its group not at its
/// most shooting and its burst timer out, one tick in nine it tries a shot of its height pick (`+0x30c`; a failed
/// try flips the pick): clear → 10 within 3, else the walk 0xb (6); else the stand 6 at each sequence end; with a
/// stand to take over, at each end sequence 6, and within 3 (or the burst out, within 30, its group not full) 6.
fn choose(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, wr: bool) {
    let d = dist2(w.mobys[id].pos, t_pos(w, id, ctx));
    let (near30, near3) = (d <= 30.0, d < 3.0);
    let m = &w.mobys[id];
    let full = m.pv_i(MOST) <= w.shared_i(m.pv_i(SHARED));
    if w.mobys[id].pv_b(STAND) == 0 {
        let (mut shot, mut tried) = (false, false);
        if !near3 && near30 && !full && ctx.game.rng.randi(9) == 0 && w.mobys[id].pv_s(T_BURST) == 0 {
            tried = true;
            let c = w.mobys[id].pv_b(PICK_C) != 0;
            shot = can_shoot(w, id, ctx, false, c, false);
        }
        let c = w.mobys[id].pv_b(PICK_C) != 0;
        if !c || !shot {
            if near3 || shot {
                low_shot(w, id, ctx, c);
            } else if wr {
                anim_r(w, id, ctx, 6);
            }
        } else if near3 {
            low_shot(w, id, ctx, c);
        } else {
            anim_r(w, id, ctx, 0xb);
            st(w, id, 6, -1);
        }
        if tried && !shot {
            let m = &mut w.mobys[id];
            m.set_pv_b(PICK_C, !m.pv_b(PICK_C));
        }
        return;
    }
    if wr { anim_r(w, id, ctx, 6); }
    if !near3 && (w.mobys[id].pv_s(T_BURST) != 0 || !near30 || full) { return; }
    anim_r(w, id, ctx, 0xb);
    st(w, id, 6, -1);
}

fn low_shot(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, c: bool) {
    if can_shoot(w, id, ctx, false, c, true) { w.mobys[id].set_pv_b(PICK_D, 1); }
    take_slot(w, id);
    anim(w, id, ctx, 8, 0, 7);
    st(w, id, 10, -1);
}

/// `0x416310`: state 6, the walk along its path (`0x4147a8`: the framework's walk, or within its stop distance of its
/// last point); at the end by its sub: 1 / 6 turned back, at a sequence end one of 0 / 1 and 5; 3 off the way in
/// (taking over done), onto the path after it, then 8 (5) or with a patrol after 4 (5); 4 onto the patrol (5, sub 1);
/// 5 turned back (taking over a stand: only from its start) and 8 (sub 5), the shot timer 10. Not at the end, taking
/// over a stand in sub 5: its turret's sequence 10, firing in it.
fn path_end(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let path = w.mobys[id].pv_i(PATH);
    let done = path == -1 || {
        let r = ai::follow_path(w, id, ctx, path);
        r || path_pts(ctx, path).is_some_and(|p| {
            let m = &w.mobys[id];
            dist2(m.pos, pt(p, m.pv_s(LAST) as i32)) < m.pv_f(MV + 0x34)
        })
    };
    if done {
        match w.mobys[id].sub_state {
            1 | 6 => {
                reverse(w, id, ctx);
                if wrapped(w, id, ctx) {
                    rand_walk(w, id, ctx);
                    st(w, id, 5, -1);
                }
            }
            3 => {
                let m = &mut w.mobys[id];
                m.set_pv_i(MV + 0x38, m.pv_i(MV + 0x38) & !4);
                if m.pv_b(FLAGS) & 2 != 0 { m.set_pv_b(FLAGS, 0); }
                let p = m.pv_i(P_AFTER);
                set_path(w, id, ctx, p, -1);
                if w.mobys[id].pv_b(PATROL_AFTER) == 0 {
                    anim_r(w, id, ctx, 0xc);
                    st(w, id, 8, 5);
                } else {
                    anim_r(w, id, ctx, 4);
                    st(w, id, 4, 5);
                }
            }
            4 => {
                let p = w.mobys[id].pv_i(P_PATROL);
                set_path(w, id, ctx, p, -1);
                st(w, id, 5, 1);
            }
            5 => {
                let r = reverse(w, id, ctx);
                if w.mobys[id].pv_b(STAND) != 0 && !r { return; }
                anim_r(w, id, ctx, 0xc);
                st(w, id, 8, 5);
                w.mobys[id].set_pv_s(T_SHOT, 10);
            }
            _ => {}
        }
        return;
    }
    if w.mobys[id].pv_b(STAND) == 0 || w.mobys[id].sub_state != 5 { return; }
    let Some(t) = turret(w, id) else { return };
    let m = &mut w.mobys[id];
    m.set_pv_b(PICK_C, 0);
    m.set_pv_b(PICK_D, 1);
    if seq_a(w, t, ctx) == 10 {
        fire(w, id, ctx);
    } else if seq(w, t, ctx) != 10 {
        ctx.host.set_anim(&w.mobys[t], 10, 0, 10);
    }
}

/// 0x114b (`0x425478`): the turret's own update, its shadow within 24 of the camera (range 0x16).
pub fn update_114b(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].drawn != 0 { crate::shadow_volumes::probe_near(w, id, ctx, 24.0, 0x16); }
}
