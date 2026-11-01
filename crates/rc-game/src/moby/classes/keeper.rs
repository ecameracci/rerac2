//! Maktar's creature keeper, **0x09c5** (L02 `0x3cc648`; its senses `0x3cdd08`, tail `0x3ce220`, death `0x3cecb0`),
//! on the framework ([`crate::moby::ai`]) with the creature's layout ([`super::critter`]): the reaction and damage
//! block `+0x50`, movement `+0x100`, target `+0x1f0` (found `+0x204`), home `+0x340`, its goal `+0x350`. It carries a
//! launcher (class 0xa20, 0xa21 with `+0x310`; `+0x370`) at its joint 0, fires the homing shot 0x0b5a from it
//! ([`update_shot`]), and throws the creatures waiting in its group `+0x328` at Ratchet.
//!
//! **pvars**: `+0x312` it starts hidden (state 0x17); `+0x314` a group whose losses alarm it (`+0x37a` the count
//! alive, every 16 frames); `+0x318` its fence path; `+0x31c` its area; `+0x320` / `+0x324` a path it walks once
//! Ratchet is in the cuboid; `+0x328` / `+0x32c` the creatures' group and the cuboid that sets them off; `+0x334` a
//! cuboid its strafes keep to; `+0x338` its comebacks; `+0x374..+0x378` / `+0x386` timers (`+0x386` it keeps on Ratchet
//! while set); `+0x37c` the throw's sway; `+0x380` the shots left; `+0x382` alarmed; `+0x388` / `+0x38c` the glows'
//! phases.
//!
//! **Subs** (`+0x95`): 0 calm — seeing him: to the stare (8) when `+0x33c`, else the guard (10, sub 2); alarmed, alert (3,
//! sub 1). 1 alert — seeing him, the guard; its timer out and he farther than 26 across: calm. 2 on guard — not seeing
//! him and its sequence done: home when 4 from it (6), else one of the strafe (4), idle (5), the turn (7) or wait (3).
//! 3 the knock (the framework's flight): dead — the launcher hidden, its bolts, the fade (crate::moby::classes::
//! death_fade) and [`gone`]; over — the path (0x12) or the guard or alert. 4 the turn-in-place's speed 0.25.
//!
//! **States**: 1 idle (0) → 5 or a stroll (2: a point 2..5 from home kept in its fence, [`spot_near`]) → 5. 3 waiting
//! (9): facing away from him by 15° → 9, else now and then 5, 4 or 7. 4 / 0xd the strafe (0x11 / 0x12, mirrored
//! swapped) to a side spot ([`spot_side`]). 5 idle (1, or 3 / 4). 6 home (6). 7 the look about (0x14 / 0x15). 8 the
//! stare (2). 9 / 10 turning to him and the guard (9): close (3 across, ½ up) the swing (0xe), its line to him clear
//! (or a wall 0x0c1b, a crate or a target, or Ratchet) the volley (0xb), else now and then the strafe. 0xb the volley
//! (10): three to five shots, one each time the sequence passes frame 5, at the point he stood (½ up); his flinch from
//! a shot (group 7) the taunt (0xc). 0xe the swing (0xd: frames 11..12.5 a hit at joint 3, radius 0.6, damage `+0x35`,
//! push 3) → 0xf (0xb from frame 10). 0x11 / 0x12 the path; 0x13 / 0x14 / 0x15 the throws (sequence 9 swaying ±20°,
//! then 10: from frame 5 a waiting creature of its group thrown from the launcher along its forward row at 0.47,
//! crate::moby::classes::critter::wake). 0x16 home (6). 0x17 hidden.
//!
//! Not ported: the joint aims (`0x3286f0` / `0x32c1d0`, the angles kept at `+0x278` / `+0x2f4`), the state 0x10 (its
//! shots while down; nothing here enters it), the launcher's matrix as the game's frame takes it (here its joint 0's
//! world frame), the moby's address in the 16-frame spacing (here its index).

use crate::hero::physics::{add, atan2, cross, dist, dist2, dot, len, len2, scale, set_len, sub, V3};
use crate::hud::lerp_rgba;
use crate::moby::{ai, countdown, mode, Ctx, MobyWorld};
use super::props::{seq, set_seq, wrapped};

/// The class, its launchers and its shot.
pub const CLASS: i16 = 0x09c5;
const LAUNCHER: [i16; 2] = [0x0a20, 0x0a21];
pub const SHOT: i16 = 0x0b5a;
/// The other class the shot's code serves (L03 `0x3e2958`, the same code as `0x402450`): Endako's boss's rocket
/// (crate::moby::classes::endako_boss). Its end is the big one ([`update_shot`]).
pub const ROCKET: i16 = 0x0b1b;
/// The movement block, the target's point and found word, home, the goal.
const MV: usize = 0x100;
const TARGET: usize = 0x1f0;
const FOUND: usize = 0x204;
const HOME: usize = 0x340;
const GOAL: usize = 0x350;
/// The launcher's link (`+0x370`).
const GUN: usize = 0x370;
/// `0x1b0070` / `0x1b0074`: its glow's ends; `0x1b0068` / `0x1b006c` the launcher's; `$gp+0x108c` / `+0x1088` their
/// phases' speeds (degrees a second).
const GLOW: (u32, u32) = (0x8080_8020, 0x8080_8080);
const GUN_GLOW: (u32, u32) = (0x8080_ff80, 0x8060_0060);
const GLOW_SPEED: f32 = 210.0;
const GUN_SPEED: f32 = 500.0;
/// The shot's speed (`0x3e6eeef0`).
const SHOT_SPEED: f32 = 0.466_666_7;

fn first_sub(m: &mut crate::moby::Moby) -> bool {
    if m.once & 2 != 0 { return false; }
    m.once |= 2;
    true
}

fn gun(w: &MobyWorld, id: usize) -> Option<usize> {
    ai::link(&w.mobys[id], GUN).filter(|&g| w.mobys.get(g).is_some_and(|x| x.state < 0x80 && LAUNCHER.contains(&x.o_class)))
}

fn yaw_to(w: &MobyWorld, id: usize, p: V3) -> f32 {
    let m = &w.mobys[id];
    atan2(p[0] - m.pos[0], p[1] - m.pos[1])
}

/// `|turn|` from its facing to the target.
fn off_target(w: &MobyWorld, id: usize) -> f32 {
    let t = w.mobys[id].pv_v(TARGET);
    crate::hero::physics::rot_dist(w.mobys[id].rot[2], yaw_to(w, id, t)).abs()
}

/// Its group's keepers (the game writes every member).
fn group(w: &MobyWorld, id: usize) -> Vec<usize> {
    usize::try_from(w.mobys[id].group).ok().and_then(|g| w.groups.get(g)).map_or(Vec::new(), |v| v.iter().copied().filter(|&i| w.mobys.get(i).is_some_and(|m| m.o_class == CLASS)).collect())
}

/// `0x3cee18`: its group alarmed (`+0x386` 180, `+0x382` 1).
fn alarm(w: &mut MobyWorld, id: usize) {
    for o in group(w, id) {
        let m = &mut w.mobys[o];
        m.set_pv_s(0x386, 0xb4);
        m.set_pv_s(0x382, 1);
    }
}

/// `0x3ce7f0`: the group's members other than the one nearest Ratchet wait 25 more (`+0x378`).
fn hold_others(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let g = group(w, id);
    let mut best = None;
    let mut near = 1000.0;
    for &o in &g {
        let d = dist(w.mobys[o].pos, ctx.hero.pos);
        if d < near { near = d; best = Some(o); }
    }
    if best.is_none() { return; }
    for o in g {
        if Some(o) == best { continue; }
        let m = &mut w.mobys[o];
        m.set_pv_s(0x378, m.pv_s(0x378).wrapping_add(0x19));
    }
}

/// `0x3cea00`: the group's waits `rand_range(30, 90)` each.
fn rest_group(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    for o in group(w, id) {
        let t = ctx.game.rng.rand_range(0x1e, 0x5a);
        w.mobys[o].set_pv_s(0x378, t as i16);
    }
}

/// `0x3ce3f8`: its head's aim at the volley's point (state 0xb) or his head, in its frame: yaw within ±45° (`+0x278`),
/// pitch within ±30° (`+0x2f4`).
fn aim(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let m = &mut w.mobys[id];
    let at = if m.state == 0xb { m.pv_v(0x360) } else { add(ctx.hero.pos, [0.0, 0.0, 1.0]) };
    let d = sub(at, add(m.pos, [0.0, 0.0, 1.0]));
    let r = m.rows;
    let l = [dot(r[0], d), dot(r[1], d), dot(r[2], d)];
    let q = std::f32::consts::FRAC_PI_4;
    m.set_pv_f(0x278, atan2(l[0], l[1]).clamp(-q, q));
    let s = f32::from_bits(0x3f06_0a93);
    m.set_pv_f(0x2f4, (-atan2(len2(l), l[2])).clamp(-s, s));
}

/// `0x3ce740`: a point `randf(2, 5)` from home a random way, kept in its fence.
fn spot_near(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let r = ctx.game.rng.randf(2.0, 5.0);
    let a = ctx.game.rng.rand_angle();
    let m = &w.mobys[id];
    let home = m.pv_v(HOME);
    let p = [a.cos() * r + home[0], a.sin() * r + home[1], home[2]];
    let p = super::critter::keep_in_path(ctx, m.pv_f(MV), home, p, m.pv_i(0x318));
    w.mobys[id].set_pv_v(GOAL, p);
}

/// `0x3ce540`: a spot to a side: without its cuboid `+0x334` 1.5 from home across the line to the target (the side
/// it is on), else on the cuboid's ring toward the target (its first row's length out); kept in its fence. True for
/// the side that takes the strafe's sequence 0x12.
fn spot_side(w: &mut MobyWorld, id: usize, ctx: &Ctx) -> bool {
    let m = &w.mobys[id];
    let (home, t) = (m.pv_v(HOME), m.pv_v(TARGET));
    let k = m.pv_i(0x334);
    let (p, side) = match usize::try_from(k).ok().and_then(|k| ctx.level.cuboids.get(k)) {
        None => {
            let a = set_len(cross(sub(t, home), [0.0, 0.0, 1.0]), 1.5);
            if dot(sub(m.pos, home), a) < 0.0 { (add(home, a), true) } else { (sub(home, a), false) }
        }
        Some(c) => {
            let o = [c.matrix[3][0], c.matrix[3][1], c.matrix[3][2]];
            let l = len([c.matrix[0][0], c.matrix[0][1], c.matrix[0][2]]);
            let p = add(o, set_len(sub(t, o), l));
            (p, 0.0 <= dot(sub(p, m.pos), m.rows[1]))
        }
    };
    let p = super::critter::keep_in_path(ctx, m.pv_f(MV), home, p, m.pv_i(0x318));
    w.mobys[id].set_pv_v(GOAL, p);
    side
}

/// `0x3327f0(yaw, −1, −1, m, goal, 0x10)`: the move with its options `0x10` for the tick.
fn strafe_to(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, goal: V3) -> i32 {
    let yaw = yaw_to(w, id, goal);
    let o = w.mobys[id].pv_i(MV + 0x38);
    w.mobys[id].set_pv_i(MV + 0x38, 0x10);
    let f = ai::move_step(w, id, ctx, yaw, goal);
    w.mobys[id].set_pv_i(MV + 0x38, o);
    f
}

/// `0x332798(m, goal)`: the move toward `goal`.
fn go(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, goal: V3) -> i32 {
    let yaw = yaw_to(w, id, goal);
    ai::move_step(w, id, ctx, yaw, goal)
}

/// `0x333980(yaw, m)`: its facing eased toward `yaw`.
fn face(w: &mut MobyWorld, id: usize, yaw: f32) {
    let m = &mut w.mobys[id];
    let (a, b, c) = (m.pv_f(MV + 0x14), m.pv_f(MV + 0x18), m.pv_f(MV + 0x1c));
    let (mut r, mut v) = (m.rot[2], m.pv_f(MV + 0xb4));
    crate::hero::physics::turn_ease(yaw, a, b, c, &mut r, &mut v, 0);
    m.rot[2] = r;
    m.set_pv_f(MV + 0xb4, v);
}

fn face_target(w: &mut MobyWorld, id: usize) {
    let t = w.mobys[id].pv_v(TARGET);
    let y = yaw_to(w, id, t);
    face(w, id, y);
}

/// The launcher's muzzle (its joint 0 in the world) and frame.
fn muzzle(w: &MobyWorld, id: usize, ctx: &Ctx) -> Option<([V3; 3], V3)> {
    let g = gun(w, id)?;
    ctx.host.joint_matrix(&w.mobys[g], 0).or_else(|| Some((w.mobys[g].rows, w.mobys[g].pos)))
}

/// A launcher (`0x315438(0xa20 / 0xa21)`, `0x3303c0`): drawn (update and draw distance 0xff), at the keeper with its
/// light, its rows kept (mode 0x100).
fn make_gun(w: &mut MobyWorld, id: usize) -> Option<usize> {
    let class = LAUNCHER[(w.mobys[id].pv_s(0x310) != 0) as usize];
    w.class(class)?;
    let g = w.create(class);
    let (pos, rot, light, ambient) = { let m = &w.mobys[id]; (m.pos, m.rot, m.light, m.ambient) };
    let x = &mut w.mobys[g];
    (x.update_dist, x.draw_dist, x.drawn) = (0xff, 0xff, 1);
    x.parent = Some(id);
    (x.pos, x.rot, x.light, x.ambient) = (pos, rot, light, ambient);
    x.mode = (x.mode & !7) | mode::KEEP_ROWS;
    ai::set_link(&mut w.mobys[id], GUN, Some(g));
    w.changed = true;
    Some(g)
}

fn hide_gun(w: &mut MobyWorld, id: usize) {
    if let Some(g) = gun(w, id) {
        let x = &mut w.mobys[g];
        x.mode = (x.mode | 0x41) & !mode::TARGET;
        x.has_collision = false;
    }
}

/// `0x3cdd08` (module docs): true when it goes (no fence or area, no launcher).
fn senses(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    if w.mobys[id].state == 0 {
        let m = &mut w.mobys[id];
        m.set_pv_b(0x48, 15);
        m.set_pv_b(0x4a, 19);
        m.set_pv_b(0x29, 1);
        if m.pv_i(0x318) == -1 || m.pv_i(0x31c) == -1 { return true; }
        let pos = m.pos;
        m.set_pv_v(HOME, pos);
        m.set_pv_v(GOAL, pos);
        m.set_pv_b(0x60, 0x10);
        m.set_pv_f(0x84, 0.5);
        m.set_pv_b(0x69, 0xfe);
        m.set_pv_f(0x80, 0.5);
        m.set_pv_f(0x8c, 0.4);
        m.set_pv_i(0x98, 3);
        m.set_pv_i(0xcc, m.pv_i(0x330));
        m.set_pv_f(0x90, pos[2] - 6.0);
        m.set_pv_b(0x65, 0x13);
        m.set_pv_b(0x28, 3);
        m.set_pv_f(0x30, 1.25);
        if make_gun(w, id).is_none() { return true; }
        let (a, b) = (ctx.game.rng.rand_angle(), ctx.game.rng.rand_angle());
        let m = &mut w.mobys[id];
        m.set_pv_f(0x38c, a);
        m.set_pv_f(0x388, b);
        m.set_pv_f(MV + 0x34, 0.05);
        m.set_pv_f(MV + 4, 0.2);
        m.set_pv_f(MV, 0.825);
        m.set_pv_f(MV + 0xc, 0.4);
        m.set_pv_f(MV + 0x10, f32::from_bits(0x3fc9_0fdc));
        m.set_pv_f(MV + 0x30, f32::from_bits(0x3f06_0a93));
        m.set_pv_i(MV + 0x3c, m.pv_i(0x318));
        m.set_pv_b(0x2e, 1);
        m.set_pv_f(MV + 8, 0.4);
        let flip = ctx.game.rng.randi(2) != 0;
        let m = &mut w.mobys[id];
        if flip { m.mode |= mode::MIRROR } else { m.mode &= !mode::MIRROR }
        if m.pv_s(0x312) != 0 {
            m.set_state(0x17, 4);
            m.has_collision = false;
            m.mode = (m.mode & !mode::TARGET) | 0x41;
            hide_gun(w, id);
        } else if m.pv_i(0x320) != -1 && m.pv_i(0x324) != -1 {
            m.set_state(0x11, 4);
        } else if m.pv_i(0x328) != -1 {
            m.set_state(0x13, 4);
        } else {
            m.set_state(1, 0);
        }
    }
    if w.mobys[id].drawn != 0 { crate::shadow_volumes::probe_near(w, id, ctx, 24.0, 0x16); }
    // The launcher at its joint 0.
    if let (Some(g), Some((rows, at))) = (gun(w, id), ctx.host.joint_matrix(&w.mobys[id], 0)) {
        let x = &mut w.mobys[g];
        (x.rows, x.pos) = (rows, at);
    }
    // Its kind's shots do it no harm.
    if w.hit_message(id).is_some_and(|h| h.tmpl.flags & 0x10000 != 0 && h.tmpl.source.and_then(|s| w.mobys.get(s)).is_some_and(|s| s.o_class == SHOT)) {
        w.clear_hit(id);
    }
    if ai::take_damage(w, id, ctx, false) != 0 { alarm(w, id); }
    if w.mobys[id].pv_b(0x2e) == 2 {
        gone(w, id, ctx);
        if w.mobys[id].state >= 0x80 { return false; }
    }
    let watch = w.mobys[id].pv_i(0x314);
    if watch != -1 && (ctx.game.frame & 0xf) as usize == id & 0xf {
        let n = super::props::group_alive(w, watch) as i16;
        let m = &mut w.mobys[id];
        if n < m.pv_s(0x37a) { m.set_pv_s(0x386, 300); }
        m.set_pv_s(0x37a, n);
    }
    let m = &w.mobys[id];
    if !(m.sub_state == 3 || m.state == 0x17) || ctx.game.frame & 0xf == 0 {
        let area = m.pv_i(0x31c);
        ai::find_target(w, id, ctx, 16.0, 0.0, area, None, false);
    }
    let m = &mut w.mobys[id];
    if m.pv_i(FOUND) != 1 {
        if m.pv_s(0x386) == 0 { return false; }
        ai::set_link(m, 0x200, ctx.hero_moby);
        m.set_pv_v(TARGET, ctx.hero.aim_point);
        m.set_pv_i(FOUND, 1);
        return false;
    }
    if m.pv_i(0x33c) != 0 {
        let me = Some(id);
        let seen = ctx.hero.push_moby == me || ctx.hero.wall_moby == me || (ctx.hero.grounded_ticks != 0 && ctx.hero.ground_moby.is_none()) || off_target(w, id) < std::f32::consts::FRAC_PI_2;
        if !seen {
            w.mobys[id].set_pv_i(FOUND, 0);
            return false;
        }
    }
    w.mobys[id].set_pv_s(0x386, 300);
    false
}

/// `0x3cecb0`, its death's end: with no comebacks left (`+0x338`) it and its launcher go; else one fewer, hidden and
/// not solid at home with its health back (`+0x24`), waiting (0x17, sub 4).
fn gone(w: &mut MobyWorld, id: usize, _ctx: &mut Ctx) {
    if w.mobys[id].pv_i(0x338) == 0 {
        if let Some(g) = gun(w, id) { w.delete(g); }
        w.delete(id);
        return;
    }
    let mut n = w.mobys[id].pv_i(0x338);
    countdown(&mut n);
    w.mobys[id].set_pv_i(0x338, n);
    if gun(w, id).is_none() && make_gun(w, id).is_none() {
        w.delete(id);
        return;
    }
    hide_gun(w, id);
    let m = &mut w.mobys[id];
    m.pos = m.pv_v(HOME);
    m.rot[0] = 0.0;
    m.rot[1] = 0.0;
    m.set_pv_f(0x20, m.pv_s(0x24) as f32);
    m.has_collision = false;
    m.mode = (m.mode & !mode::TARGET) | 0x41;
    m.set_pv_s(0x88, 0);
    m.set_pv_b(0x2e, 1);
    m.set_state(0x17, 4);
    w.changed = true;
}

/// The fade's end (crate::moby::classes::death_fade): [`gone`].
fn fade_done(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) { gone(w, id, ctx); }

/// `0x3ce220`: its glow (`+0x78`: `4·sin + 4` of its phase, within 0..1, of the way from [`GLOW`]'s first to its
/// second) and the launcher's (`sin` of its own phase), its timers down and its state's ticks on; outside a scene
/// (`0x1a8f00` ≠ 2), or with Ratchet in state 0x6f, `+0x386` is cleared (it holds within a tick: a hurt keeper's group
/// sees it in their own senses after it).
fn tail(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let k = std::f32::consts::PI / 180.0 / 60.0;
    if w.mobys[id].drawn != 0 {
        let m = &mut w.mobys[id];
        let p = crate::hero::physics::add_rot(m.pv_f(0x38c), GLOW_SPEED * k);
        m.set_pv_f(0x38c, p);
        let f = (p.sin() * 4.0 + 4.0).clamp(0.0, 1.0);
        m.glow = lerp_rgba(f, GLOW.0, GLOW.1);
        if let Some(g) = gun(w, id) {
            let m = &mut w.mobys[id];
            let q = crate::hero::physics::add_rot(m.pv_f(0x388), GUN_SPEED * k);
            m.set_pv_f(0x388, q);
            w.mobys[g].glow = lerp_rgba(q.sin() * 0.5 + 0.5, GUN_GLOW.0, GUN_GLOW.1);
        }
    }
    let m = &mut w.mobys[id];
    for off in [0x386, 0x374, 0x376, 0x378] {
        let mut t = m.pv_s(off) as i32;
        countdown(&mut t);
        m.set_pv_s(off, t as i16);
    }
    if m.state_ticks != -1 { m.state_ticks = m.state_ticks.wrapping_add(1); }
    // `0x1a8f00` (the scene mode) not 2, or Ratchet in state 0x6f: `+0x386` lasts the tick only.
    if ctx.game.scenes.mode != crate::scene::SCENE || ctx.hero.state == 0x6f { m.set_pv_s(0x386, 0); }
}

/// 0x09c5 (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if senses(w, id, ctx) {
        if let Some(g) = gun(w, id) { w.delete(g); }
        w.delete(id);
        return;
    }
    if w.mobys[id].state >= 0x80 { return; }
    let (a, b, c) = (f32::from_bits(0x3b64_c38a), f32::from_bits(0x3ed6_7752), f32::from_bits(0x3dcc_ccce));
    match w.mobys[id].sub_state {
        0 => {
            if first_sub(&mut w.mobys[id]) { ai::senses(&mut w.mobys[id], f32::from_bits(0x3a91_a2b4), f32::from_bits(0x3d4c_ccce), a, b); }
            let m = &mut w.mobys[id];
            if m.pv_i(FOUND) == 1 {
                if m.pv_i(0x33c) == 0 { m.set_state(10, 2) } else { m.set_pv_i(0x33c, 0); m.set_state(8, 2) }
            } else if m.pv_s(0x382) != 0 {
                m.set_state(3, 1);
                m.set_pv_s(0x382, 0);
                let t = ctx.game.rng.rand_range(0x1e0, 600);
                w.mobys[id].set_pv_s(0x374, t as i16);
            }
        }
        1 => {
            if first_sub(&mut w.mobys[id]) { ai::senses(&mut w.mobys[id], f32::from_bits(0x3b11_a2b4), c, a, b); }
            let hero = ctx.hero.pos;
            let m = &mut w.mobys[id];
            if m.pv_i(FOUND) == 1 {
                m.set_state(10, 2);
            } else if m.pv_s(0x374) == 0 {
                if dist2(m.pos, hero) <= 26.0 { m.set_pv_s(0x374, 0x78) } else { m.set_state(1, 0) }
            }
        }
        2 => {
            if first_sub(&mut w.mobys[id]) {
                hold_others(w, id, ctx);
                ai::senses(&mut w.mobys[id], f32::from_bits(0x3b11_a2b4), c, a, b);
            }
            if w.mobys[id].pv_i(FOUND) != 1 && wrapped(w, id, ctx) && w.mobys[id].state != 0xb {
                let m = &w.mobys[id];
                if 4.0 < dist(m.pos, m.pv_v(HOME)) {
                    w.mobys[id].set_state(6, -1);
                } else {
                    match ctx.game.rng.randi(4) {
                        1 => w.mobys[id].set_state(4, 1),
                        2 => w.mobys[id].set_state(5, 1),
                        3 => w.mobys[id].set_state(7, 1),
                        _ => w.mobys[id].set_state(3, 1),
                    }
                }
                let t = ctx.game.rng.rand_range(0x244, 600);
                w.mobys[id].set_pv_s(0x374, t as i16);
            }
        }
        3 => {
            let f = ai::flight(w, id, ctx);
            if f & 1 != 0 {
                hide_gun(w, id);
                crate::moby::bolts::drop(w, id, 0, 3, ctx);
                super::death_fade::start(w, id, ctx, 0.0, Some(fade_done));
                return;
            }
            if f & 2 != 0 {
                w.mobys[id].set_pv_i(0x33c, 0);
                if w.mobys[id].pv_i(0x320) != -1 {
                    w.mobys[id].set_state(0x12, 4);
                } else if w.mobys[id].pv_i(FOUND) == 1 {
                    w.mobys[id].set_state(10, 2);
                } else {
                    w.mobys[id].set_state(3, 1);
                    let t = ctx.game.rng.rand_range(0x244, 600);
                    w.mobys[id].set_pv_s(0x374, t as i16);
                }
            }
        }
        4 => {
            if first_sub(&mut w.mobys[id]) {
                ai::senses(&mut w.mobys[id], f32::from_bits(0x3b5a_740e), c, a, b);
                w.mobys[id].set_pv_f(MV + 0x34, 0.25);
            }
        }
        _ => {}
    }
    let next = state_tick(w, id, ctx);
    if w.mobys[id].state >= 0x80 { return; }
    if let Some(n) = next { w.mobys[id].set_state(n, -1); }
    tail(w, id, ctx);
}

/// The strafe's sequence: 0x12 on the side [`spot_side`] picks, else 0x11; mirrored the other.
fn strafe_seq(w: &MobyWorld, id: usize, side: bool) -> u8 {
    let s = if side { 0x12 } else { 0x11 };
    if w.mobys[id].mode & mode::MIRROR != 0 { if s == 0x11 { 0x12 } else { 0x11 } } else { s }
}

/// The states (module docs); the next state.
fn state_tick(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> Option<u8> {
    let sub_state = w.mobys[id].sub_state;
    let ticks = w.mobys[id].state_ticks;
    match w.mobys[id].state {
        1 => {
            if w.mobys[id].first() { set_seq(w, id, ctx, 0, 0, 0x14); }
            if !wrapped(w, id, ctx) { return None; }
            if ctx.game.rng.randi(4) == 0 { Some(5) } else if w.mobys[id].pv_i(0x33c) != 0 { None } else { Some(2) }
        }
        2 => {
            if w.mobys[id].first() {
                set_seq(w, id, ctx, 5, 0, 0x14);
                spot_near(w, id, ctx);
            }
            let g = w.mobys[id].pv_v(GOAL);
            let f = go(w, id, ctx, g);
            if f & 0x80 == 0 && w.mobys[id].state_ticks < 0xb5 && w.mobys[id].pv_i(0x180) < 0x15 { return None; }
            Some(5)
        }
        3 => {
            if w.mobys[id].first() { set_seq(w, id, ctx, 9, 0, 0x14); }
            if 0.261_799_42 < off_target(w, id) { return Some(9); }
            if !wrapped(w, id, ctx) || ctx.game.rng.randi(2) == 0 { return None; }
            match ctx.game.rng.randi(3) { 0 => Some(4), 1 => Some(5), _ => Some(7) }
        }
        4 | 0xd => {
            let s = w.mobys[id].state;
            if w.mobys[id].first() {
                let side = spot_side(w, id, ctx);
                let q = strafe_seq(w, id, side);
                set_seq(w, id, ctx, q, 0, 10);
            }
            let g = w.mobys[id].pv_v(GOAL);
            let f = strafe_to(w, id, ctx, g);
            let ticks = w.mobys[id].state_ticks;
            if s == 4 {
                if f & 0x80 == 0 && ticks < 0x79 { return None; }
                Some(3)
            } else {
                if f & 0x80 == 0 && ticks <= 0x78 && w.mobys[id].pv_i(0x180) < 0x15 { return None; }
                Some(10)
            }
        }
        5 => {
            if w.mobys[id].first() {
                if sub_state == 0 {
                    set_seq(w, id, ctx, 1, 0, 0x14);
                } else if sub_state == 1 {
                    let cur = seq(w, id, ctx);
                    if cur as i32 != ctx.game.rng.randi(2) + 3 {
                        let r = ctx.game.rng.randi(2) as u8 + 3;
                        ctx.host.set_anim(&w.mobys[id], r, 0, 0x14);
                    }
                }
            }
            if !wrapped(w, id, ctx) { return None; }
            match sub_state { 0 => Some(1), 1 => Some(3), _ => None }
        }
        6 => {
            if w.mobys[id].first() { set_seq(w, id, ctx, 6, 0, 0xf); }
            let h = w.mobys[id].pv_v(HOME);
            let f = go(w, id, ctx, h);
            if f & 0x80 == 0 && w.mobys[id].state_ticks < 0xf1 { return None; }
            Some(3)
        }
        7 => {
            if w.mobys[id].first() {
                let s = if ctx.game.rng.randi(2) != 0 { 0x14 } else { 0x15 };
                set_seq(w, id, ctx, s, 0, 0xf);
            }
            if !wrapped(w, id, ctx) { return None; }
            Some(3)
        }
        8 => {
            if w.mobys[id].first() { set_seq(w, id, ctx, 2, 0, 10); }
            face_target(w, id);
            aim(w, id, ctx);
            if !wrapped(w, id, ctx) { return None; }
            Some(10)
        }
        9 => {
            if w.mobys[id].first() { set_seq(w, id, ctx, 5, 0, 10); }
            face_target(w, id);
            aim(w, id, ctx);
            if 0.087_266_475 <= off_target(w, id) { return None; }
            match sub_state { 1 => Some(3), 2 => Some(10), _ => None }
        }
        10 => {
            if w.mobys[id].first() { set_seq(w, id, ctx, 9, 0, 0x14); }
            aim(w, id, ctx);
            if 0.523_598_85 < off_target(w, id) { return Some(9); }
            if w.mobys[id].pv_s(0x378) == 0 && 0xf < ticks {
                let Some((_, p)) = muzzle(w, id, ctx) else { return None };
                if let Some(h) = w.coll_line(ctx, p, ctx.hero.aim_point, 0x10, Some(id)) {
                    let hit = h.moby;
                    if hit.and_then(|k| w.mobys.get(k)).is_some_and(|x| x.o_class == 0x0c1b) { return Some(0xd); }
                    let wanted = hit.is_some_and(|k| crate::moby::aim::is_crate(w, k) || w.class(w.mobys[k].o_class).is_some_and(|c| c.kind == 0x16));
                    if hit != ctx.hero_moby && w.mobys[id].pv_s(0x386) == 0 && (hit.is_none() || !wanted) { return None; }
                }
                return Some(0xb);
            }
            let m = &w.mobys[id];
            if dist2(m.pos, ctx.hero.pos) < 3.0 && (m.pos[2] - ctx.hero.pos[2]).abs() < 0.5 { return Some(0xe); }
            if !wrapped(w, id, ctx) || ctx.game.rng.randi(2) != 0 { return None; }
            Some(0xd)
        }
        0xb => {
            if w.mobys[id].first() {
                set_seq(w, id, ctx, 10, 0, 10);
                let n = ctx.game.rng.rand_range(3, 5);
                w.mobys[id].set_pv_s(0x380, n as i16);
                rest_group(w, id, ctx);
                let t = w.mobys[id].pv_v(TARGET);
                w.mobys[id].set_pv_v(0x360, [t[0], t[1], t[2] + 0.5]);
            }
            aim(w, id, ctx);
            if (ctx.game.frame & 0xf) as usize == id & 0xf { rest_group(w, id, ctx); }
            if ai::reached(w, id, ctx, 5.0) {
                if let Some((_, p)) = muzzle(w, id, ctx) {
                    let v = set_len(sub(w.mobys[id].pv_v(0x360), p), SHOT_SPEED);
                    let (lo, hi) = if w.mobys[id].pv_s(0x380) < 2 { (0x1e, 0x50) } else { (10, 0x1e) };
                    let dmg = w.mobys[id].pv_b(0x34) as f32;
                    spawn_shot(w, id, dmg, dmg, f32::from_bits(0x3bb6_0b61), p, scale(v, 0.25), v, (lo, hi, 1));
                }
                return None;
            }
            if !wrapped(w, id, ctx) { return None; }
            let Some((_, p)) = muzzle(w, id, ctx) else { return None };
            let hit = w.coll_line(ctx, p, ctx.hero.aim_point, 0, Some(id));
            let mut n = w.mobys[id].pv_s(0x380) as i32;
            let out = countdown(&mut n);
            w.mobys[id].set_pv_s(0x380, n as i16);
            let blocked = hit.as_ref().is_some_and(|h| h.moby != ctx.hero_moby);
            if !out && !blocked {
                if 0.523_598_85 < off_target(w, id) { return Some(9); }
                let flinch = ctx.hero.group == 7 && ctx.hero.damage.hit_by.and_then(|k| w.mobys.get(k)).is_some_and(|x| x.o_class == SHOT);
                return flinch.then_some(0xc);
            }
            let m = &mut w.mobys[id];
            m.set_pv_s(0x378, m.pv_s(0x378).wrapping_add(0xf));
            if ctx.game.rng.randi(3) == 0 { Some(0xc) } else { Some(0xd) }
        }
        0xc => {
            if w.mobys[id].first() {
                let s = if ctx.hero.group == 7 { 0xc } else { 0xb };
                set_seq(w, id, ctx, s, 0, 0x14);
            }
            aim(w, id, ctx);
            if !wrapped(w, id, ctx) { return None; }
            Some(10)
        }
        0xe => {
            if w.mobys[id].first() { set_seq(w, id, ctx, 0xd, 0, 10); }
            face_target(w, id);
            let t = ctx.host.frame_time(&w.mobys[id]);
            if (11.0..12.5).contains(&t) {
                let dmg = w.mobys[id].pv_b(0x35) as f32;
                ai::attack_joint(w, id, ctx, f32::from_bits(0x3f19_999a), dmg, 3.0, 3, 1, 0, 1, 0);
            }
            if !wrapped(w, id, ctx) { return None; }
            Some(0xf)
        }
        0xf => {
            if w.mobys[id].first() { set_seq(w, id, ctx, 0xb, 10, 10); }
            aim(w, id, ctx);
            if !wrapped(w, id, ctx) { return None; }
            if off_target(w, id) <= 0.523_598_85 { Some(0xb) } else { Some(9) }
        }
        0x11 => {
            if w.mobys[id].first() { set_seq(w, id, ctx, 0, 0, 0x14); }
            let k = w.mobys[id].pv_i(0x324);
            if !ctx.level.in_cuboid(k, ctx.hero.pos) && w.mobys[id].pv_i(FOUND) != 1 { return None; }
            Some(0x12)
        }
        0x12 => {
            let path = w.mobys[id].pv_i(0x320);
            if w.mobys[id].first() {
                set_seq(w, id, ctx, 6, 0, 0x14);
                ai::mark_fence(w, id, ctx, MV);
            }
            let end = usize::try_from(path).ok().and_then(|p| ctx.level.paths.get(p)).and_then(|p| p.last().copied());
            let d = end.map_or(0.0, |e| dist2(w.mobys[id].pos, [e[0], e[1], e[2]]));
            if w.mobys[id].pv_f(MV + 0x34) <= d && !ai::follow_path(w, id, ctx, path) { return None; }
            // The turn to him when he is off to a side (the game's test, `0x3cd7..`).
            if seq(w, id, ctx) != 2 && 0.349_065_9 < off_target(w, id) { ctx.host.set_anim(&w.mobys[id], 2, 5, 10); }
            let t = w.mobys[id].pv_v(TARGET);
            let y = yaw_to(w, id, t);
            face(w, id, y);
            if off_target(w, id) < 0.087_266_475 && (seq(w, id, ctx) != 2 || wrapped(w, id, ctx)) {
                let m = &mut w.mobys[id];
                m.set_pv_i(0x320, -1);
                let pos = m.pos;
                m.set_pv_v(HOME, pos);
                if m.pv_i(0x328) == -1 { m.set_state(1, 0) } else { m.set_state(0x13, 4) }
            }
            None
        }
        0x13 => {
            if w.mobys[id].first() { set_seq(w, id, ctx, 0, 0, 0x14); }
            let k = w.mobys[id].pv_i(0x32c);
            if k != -1 && !ctx.level.in_cuboid(k, ctx.hero.pos) && w.mobys[id].pv_i(FOUND) != 1 { return None; }
            Some(0x14)
        }
        0x14 => {
            if w.mobys[id].first() {
                set_seq(w, id, ctx, 9, 0, 0x14);
                let sway = w.mobys[id].pv_f(0x37c);
                let v = if sway == 0.0 { if ctx.game.rng.randi(2) == 0 { -0.349_065_9 } else { 0.349_065_9 } } else { -sway };
                w.mobys[id].set_pv_f(0x37c, v);
            }
            let m = &mut w.mobys[id];
            m.set_pv_f(0x278, m.pv_f(0x37c));
            if m.state_ticks < 0xb { return None; }
            if super::critter::waiting(w, w.mobys[id].pv_i(0x328)).is_none() {
                w.mobys[id].set_state(3, 1);
                return None;
            }
            Some(0x15)
        }
        0x15 => {
            if w.mobys[id].first() { set_seq(w, id, ctx, 10, 0, 10); }
            let m = &mut w.mobys[id];
            m.set_pv_f(0x278, m.pv_f(0x37c));
            if !ai::reached(w, id, ctx, 5.0) {
                if wrapped(w, id, ctx) { w.mobys[id].set_state(0x14, -1); }
                return None;
            }
            if let (Some(c), Some((rows, at))) = (super::critter::waiting(w, w.mobys[id].pv_i(0x328)), muzzle(w, id, ctx)) {
                super::critter::wake(w, c, ctx, at, set_len(rows[0], SHOT_SPEED));
            }
            None
        }
        0x16 => {
            if w.mobys[id].first() {
                set_seq(w, id, ctx, 6, 0, 0x14);
                ai::mark_fence(w, id, ctx, MV);
            }
            let h = w.mobys[id].pv_v(HOME);
            if go(w, id, ctx, h) & 0x80 != 0 {
                let m = &mut w.mobys[id];
                m.set_state(1, 0);
                let pos = m.pos;
                m.set_pv_v(HOME, pos);
            }
            None
        }
        _ => None,
    }
}

/// `0x402c78(damage, b, accel, pos, vel, goal, lo, hi, n)`: a shot 0x0b5a at `pos` (update and draw distance 0xff,
/// scale 0.015), its velocity `vel` easing toward `goal` by `accel` a tick (`+0x64`), 270 ticks (`+0x70`), its
/// damage `+0x6c`, `b` at `+0x68`, its trail's life range and first count `+0x75..+0x77`.
#[allow(clippy::too_many_arguments)]
pub fn spawn_shot(w: &mut MobyWorld, src: usize, damage: f32, b: f32, accel: f32, pos: V3, vel: V3, goal: V3, trail: (u8, u8, u8)) -> Option<usize> {
    w.class(SHOT)?;
    spawn_shot_of(w, SHOT, src, damage, b, accel, pos, vel, goal, trail)
}

/// The same shot of class `class`: the keeper's 0x0b5a, or Endako's boss's rocket [`ROCKET`] (L03 `0x3e2ee8`, the same
/// code; created even when the level's class table lacks it, as the overlay's update table holds it).
#[allow(clippy::too_many_arguments)]
pub fn spawn_shot_of(w: &mut MobyWorld, class: i16, src: usize, damage: f32, b: f32, accel: f32, pos: V3, vel: V3, goal: V3, trail: (u8, u8, u8)) -> Option<usize> {
    let s = w.create(class);
    let x = &mut w.mobys[s];
    x.update = Some(update_shot);
    x.mode &= !mode::NO_UPDATE;
    (x.pos, x.update_dist, x.draw_dist, x.drawn, x.scale, x.state) = (pos, 0xff, 0xff, 1, f32::from_bits(0x3c75_c28f), 0);
    x.parent = Some(src);
    if x.pvars.len() < 0x80 { x.pvars.resize(0x80, 0); }
    x.set_pv_v(0, vel);
    x.set_pv_v(0x10, goal);
    x.set_pv_f(0x64, accel);
    x.set_pv_i(0x70, 0x10e);
    x.set_pv_b(0x75, trail.0);
    x.set_pv_b(0x76, trail.1);
    x.set_pv_b(0x77, trail.2);
    x.set_pv_f(0x68, b);
    x.set_pv_f(0x6c, damage);
    w.changed = true;
    Some(s)
}

/// The shot's head, 0.3 behind its centre along its first row.
fn shot_head(m: &crate::moby::Moby) -> V3 { add(m.pos, scale(m.rows[0], -0.3)) }

/// `0x402450`, the shot: facing along its velocity, its scroll `+0x60` back 0.1 a tick (in −8..0), it moves; a line
/// from where it was meeting the world or a moby ends it (a moby: a hit of its damage, flags 0x10001, along its
/// velocity; its sound 1; a small burst: crate::moby::blast, size 0.3, 20 puffs, 45 streaks, the flash). Out of its
/// 270 ticks or the world it ends where it was. Else its velocity eases toward its goal, its looped sound 0 plays
/// (`+0x7c`) and every third tick its trail's four points (`+0x20..+0x50`) step back, the head first. Its draw:
/// [`draw_shot`]. Not ported: its trail's particles (type 0x16, `0x402b08` → `0x33c5a8`, every tick).
pub fn update_shot(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    match m.state {
        2 => {
            let h = m.pv_i(0x7c);
            if ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
            w.delete(id);
            return;
        }
        0 => {
            m.set_state(1, -1);
            let r = ctx.game.rng.rand() & 1;
            m.set_pv_f(0x60, 0.0);
            m.set_pv_i(0x7c, -1);
            m.set_pv_b(0x74, r as u8);
            let v = m.pv_v(0);
            m.rot[2] = atan2(v[0], v[1]);
            m.rot[1] = -atan2(len2(v), v[2]);
            m.rows = crate::hero::physics::euler_rows(m.rot);
            let head = shot_head(m);
            let step = set_len(head, 0.01);
            m.set_pv_v(0x20, head);
            for k in 1..4 { let p = m.pv_v(0x20 + 0x10 * (k - 1)); m.set_pv_v(0x20 + 0x10 * k, add(p, step)); }
            m.set_pv_b(0x79, 2);
        }
        1 => {}
        _ => return,
    }
    let m = &mut w.mobys[id];
    let v = m.pv_v(0);
    m.rot[2] = atan2(v[0], v[1]);
    m.rot[1] = -atan2(len2(v), v[2]);
    let mut s = m.pv_f(0x60) - 0.1;
    if s <= -8.0 { s += 8.0; }
    m.set_pv_f(0x60, s);
    let head = shot_head(m);
    let old = m.pos;
    m.pos = add(m.pos, v);
    let new = m.pos;
    if !ctx.host.sound_playing(m.pv_i(0x7c)) {
        let h = ctx.host.play_sound(&w.mobys[id], 0, 4);
        w.mobys[id].set_pv_i(0x7c, h);
    }
    if let Some(hit) = w.coll_line(ctx, old, new, 0, None) {
        if let Some(t) = hit.moby {
            let m = &w.mobys[id];
            let mut tmpl = crate::moby::hits::HitTemplate::new(m.pv_f(0x6c), Some(id), 0x1_0001, [v[0], v[1], v[2], 0.0]);
            tmpl.kind = (m.o_class as u16 as u32) << 16 | 0x305;
            w.deliver_hit(t, &tmpl, [0.0, 0.0, 0.0, 1.0]);
        }
        ctx.host.play_sound(&w.mobys[id], 1, 0);
        let pos = w.mobys[id].pos;
        if w.mobys[id].o_class == SHOT {
            crate::moby::blast::burst(w, ctx, pos, &crate::moby::blast::Blast { size: 0.3, bursts: [0; 3], puffs: 0x14, streaks: 0x2d, light: true, shake: false });
        } else {
            // L03 `0x3e3008`, any other class: a sphere of 0.25 hits for its damage (push 1, flags 0x10001, `0x31e588`)
            // and the big explosion (crate::moby::classes::explosion) faces the camera.
            let dmg = w.mobys[id].pv_f(0x6c);
            ai::attack_sphere(w, id, ctx, 0.25, dmg, 1.0, pos, 0x1_0001, 0, 1, 0);
            let dir = set_len(sub(ctx.host.camera_pos(), pos), 1.0);
            super::explosion::start(w, ctx, 0.0, pos, dir);
        }
        let m = &mut w.mobys[id];
        m.state = 2;
        m.mode |= 0x41;
    }
    let m = &mut w.mobys[id];
    let mut life = m.pv_i(0x70);
    let over = countdown(&mut life);
    m.set_pv_i(0x70, life);
    let inside = |c: f32| (2.0..=1021.0).contains(&c);
    if over || !(inside(m.pos[0]) && inside(m.pos[1]) && inside(m.pos[2])) {
        m.state = 2;
        m.mode |= 0x41;
        m.pos = old;
        return;
    }
    if m.pv_b(0x7a) == 0 {
        let d = sub(m.pv_v(0x10), v);
        let a = m.pv_f(0x64);
        let d = if a < len(d) { set_len(d, a) } else { m.set_pv_b(0x7a, 1); d };
        m.set_pv_v(0, add(v, d));
    }
    let c = m.pv_b(0x77);
    m.set_pv_b(0x77, if c == 0 { m.pv_b(0x78) } else { c - 1 });
    if m.pv_b(0x79) == 0 {
        for k in (1..4).rev() { let p = m.pv_v(0x20 + 0x10 * (k - 1)); m.set_pv_v(0x20 + 0x10 * k, p); }
        m.set_pv_v(0x20, head);
        m.set_pv_b(0x79, 2);
    } else {
        m.set_pv_b(0x79, m.pv_b(0x79) - 1);
    }
    if m.state == 1 { w.register_draw(id); }
}

/// `$gp+0x277c`: the shot's colour (rgb; `+0x2778` the other class's, 0x0b1b).
const SHOT_RGB: u32 = 0x0010_8030;
/// L03 `$gp+0x1aa0`: the rocket's colour.
const ROCKET_RGB: u32 = 0x0010_3080;

/// `0x402bc8`, the shot's draw: its glow at its head (crate::fx::halo) and its streak through its trail's points from
/// it (crate::fx::streak, scroll `+0x60`, 0.25 of the texture a segment).
pub fn draw_shot(w: &MobyWorld, id: usize, rng: &mut crate::rng::Rng, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let m = &w.mobys[id];
    let head = shot_head(m);
    // L03 `0x3e31e8`: `$gp+0x1aa0` for any class but 0x0b5a (`+0x1aa4`).
    let rgb = if m.o_class == SHOT { SHOT_RGB } else { ROCKET_RGB };
    crate::fx::halo(out, rng, head, rgb, cam);
    let pts: Vec<V3> = (0..4).map(|k| m.pv_v(0x20 + 0x10 * k)).collect();
    crate::fx::streak(out, rng, m.pv_f(0x60), 0.25, head, &pts, rgb, cam);
}
