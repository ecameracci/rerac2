//! Maktar's creature, **0x0a4e** (L02 `0x3ceec8`; its senses `0x3cfa28`, tail `0x3cfe60`, glow `0x3d0398`), also on
//! level 26, on the framework ([`crate::moby::ai`]): health block through the pvars' pointers, the reaction and
//! damage block `+0x50`, movement `+0x100`, target `+0x1f0`, its area `+0x210`, home `+0x2e0`.
//!
//! **The set-up** (state 0, in [`senses`]): its area's first path (`+0x218`, its fence `+0x13c`; none: −1; no area:
//! deleted), its size `randf(1, 1.333)` (`+0x324`, the scale), the reaction (`+0x48` 15, `+0x4a` 6, `+0x60` 0xb, `+0x28`
//! 3, `+0x30` 1/3), movement (radius 0.3·size, height 0.2·size, steps 0.4, slope 90°, turn left 30°, stop 0.05, options
//! |= 0x280, its weave `randf_signed(18°, 22°)` over 120..180 ticks, walk sequences 7 / 6, its speed's base 1/15 and
//! limit 3), its pulse's phase; then idle (state 1, sub 0), or with `+0x214` hidden for its drop (state 0xd, sub 4,
//! sequence 10: [`DROP`] follows when something sets it).
//!
//! **Each tick** ([`senses`]): its shadow within 24 of the camera (range 0x16); the frame's damage (a hurt alarms its
//! group's 0x0a4e, `+0x316`; a kill → state 0xc); outside the knock (subs 3 / 4) its target (Ratchet within 16 across,
//! in its area) into `+0x314`.
//!
//! **Subs** (`+0x95`): 0 calm — seeing him (after `rand_range(0, 30)` ticks) it chases (state 9) or stares (8, one in
//! three); alarmed it turns (6 / 7) and its idle timer `+0x310` is set `rand_range(480, 600)`. 1 hunting — seeing him,
//! the chase; idle timer out, calm (state 1). 2 wary — not seeing him (after `rand_range(0, 30)`) it backs off (state
//! 3, sub 1). 3 the knock (the framework's flight): dead → [`die`]; over, the chase or the back-off.
//!
//! **States**: 1 idle (sequence 0, or 1 one time in four, for `rand_range(60, 120)` ticks) → 2 a stroll to a point
//! `randf(2, 5)` from home a random way (kept in its path, `0x3302b8`), at least 31 ticks, or to it (181 at most) → 1.
//! 3 alert (sequence 4 / 5): within 8 of Ratchet → 4, else at the sequence's end 5 (stroll then 3), 6 or 7. 4 retreat
//! home until 8..14 + 0.5 from him (or home reached) → 6 / 7. 6 / 7 turning to the target (sequences 2..3 / 0xc) → 3.
//! 8 the stare (2..3) → 9. 9 the chase (sequence 6): toward the target with its group spread 5° a member to a side
//! (`0x32f738`, every 4 ticks), its head's aim (`0x3d01c8`, ±45° / ±30°); within 1.25 → 10. 10 the bite (sequence 8
//! from frame 2): facing the target, at frame 13 a hit from joint 1 (radius 0.75, damage `+0x34`, flags 0x4000001) →
//! 9. 0xc dying ([`die`]). 0xe the drop ([`DROP`]).
//!
//! **The glow** (`0x3d0398`): its colour `+0x78` pulses (12° a tick, `0x80808080` ↔ `0x00202000`); once a frame for
//! its group each drawn 0x0a4e's joint 0 is kept (`+0x220`) and the group's eyes drawn ([`draw`]).
//!
//! Not ported: the gravity frames (`0x3347e8`, level 0x1a's set-up `0x32aba0`), the head's and tail's aims (`0x3286f0`,
//! `0x32c1d0`; the aim angles are kept at `+0x2c0` / `+0x2c8`), the moby word `+0x9c` (3 / 0xc).

use crate::hero::physics::{add, add_rot, atan2, dist, dist2, len2, sub, turn_ease, V3};
use crate::hud::lerp_rgba;
use crate::moby::{ai, Ctx, MobyWorld};
use super::props::{seq, set_seq, wrapped};

pub const CLASS: i16 = 0x0a4e;
/// The movement block.
const MV: usize = 0x100;
/// The target block's point.
const TARGET: usize = 0x1f0;
/// `0x1b00b8` / `0x1b00bc`: Ratchet's distance that makes it retreat, and its retreat's reach (+ 0.5).
const NEAR: f32 = 8.0;
const FAR: f32 = 14.0;

/// `0x32a7f0`: the first time since the last sub-state change.
fn first_sub(m: &mut crate::moby::Moby) -> bool {
    if m.once & 2 != 0 { return false; }
    m.once |= 2;
    true
}

fn group(w: &MobyWorld, id: usize) -> Vec<usize> {
    usize::try_from(w.mobys[id].group).ok().and_then(|g| w.groups.get(g)).cloned().unwrap_or_default()
}

/// `0x3d0110(n)`: the first member of moby group `n` waiting (state 13: a hidden 0x0a4e kept for its drop).
pub fn waiting(w: &MobyWorld, n: i32) -> Option<usize> {
    let g = usize::try_from(n).ok().filter(|&g| g != 0xff)?;
    w.groups.get(g)?.iter().copied().find(|&i| w.mobys.get(i).is_some_and(|m| m.state == 0xd))
}

/// `0x3d02c8(m, pos, vel)`: a waiting moby (state 13) woken at `pos`: shown and solid again (mode `& ~0x41 | 0x1000`,
/// its class's collision), the drop ([`DROP`]) facing along `vel`, marked out of its fence unless inside it
/// (crate::moby::ai::mark_fence), its velocity `vel` (`+0x1a0`), `+0x120` / `+0x124` 0, options `|= 0x11`.
pub fn wake(w: &mut MobyWorld, id: usize, ctx: &Ctx, pos: V3, vel: V3) {
    if w.mobys[id].state != 0xd { return; }
    let solid = w.class(w.mobys[id].o_class).is_some_and(|c| c.has_collision);
    let m = &mut w.mobys[id];
    m.mode = (m.mode & !0x41) | 0x1000;
    m.has_collision = solid;
    m.set_state(DROP, -1);
    m.pos = pos;
    m.rot[2] = atan2(vel[0], vel[1]);
    ai::mark_fence(w, id, ctx, MV);
    let m = &mut w.mobys[id];
    m.set_pv_v(MV + 0xa0, vel);
    m.set_pv_i(MV + 0x20, 0);
    m.set_pv_i(MV + 0x24, 0);
    m.set_pv_i(MV + 0x38, m.pv_i(MV + 0x38) | 0x11);
    w.changed = true;
}

/// `0x3cff00`: its group's living 0x0a4e alarmed (`+0x316`).
fn alarm(w: &mut MobyWorld, id: usize) {
    for m in group(w, id) {
        let x = &mut w.mobys[m];
        if x.state < 0x80 && x.o_class == CLASS { x.set_pv_s(0x316, 1); }
    }
}

/// `0x3cfa28` (module docs).
fn senses(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 0 {
        let area = w.mobys[id].pv_i(0x210);
        let Some(a) = usize::try_from(area).ok().and_then(|a| ctx.level.areas.get(a)) else {
            w.delete(id);
            return;
        };
        let path = a.lists[0].first().copied().unwrap_or(-1);
        let class_scale = w.class(CLASS).map_or(1.0, |c| c.scale);
        let rng = &mut ctx.game.rng;
        let s = rng.randf(1.0, f32::from_bits(0x3faa_9fbe));
        let weave = rng.randf_signed(18.0, 22.0);
        let phase = rng.rand_angle();
        let m = &mut w.mobys[id];
        m.set_pv_f(0x90, m.pos[2] - 1.0);
        m.set_pv_i(0x218, path);
        if path != -1 { m.set_pv_i(0x13c, path); }
        m.set_pv_f(0x324, s);
        m.scale = class_scale * s;
        m.set_pv_b(0x48, 15);
        m.set_pv_b(0x4a, 6);
        m.set_pv_b(0x60, 0xb);
        m.set_pv_b(0x28, 3);
        m.set_pv_f(0x30, f32::from_bits(0x3eaa_7efa));
        m.set_pv_f(MV + 4, s * 0.2);
        m.set_pv_f(MV, s * 0.3);
        m.set_pv_f(MV + 0xc, 0.4);
        m.set_pv_f(MV + 0x10, std::f32::consts::FRAC_PI_2);
        m.set_pv_f(MV + 0x30, f32::from_bits(0x3f06_0a93));
        m.set_pv_i(MV + 0x38, m.pv_i(MV + 0x38) | 0x280);
        m.set_pv_i(0x98, 3);
        m.set_pv_f(0x80, 0.2);
        m.set_pv_f(0x84, 0.2);
        m.set_pv_f(0x8c, 0.4);
        m.set_pv_f(MV + 8, 0.4);
        m.set_pv_f(MV + 0x34, 0.05);
        m.set_pv_b(MV + 0xe8, 7);
        m.set_pv_s(MV + 0xe2, 0x78);
        m.set_pv_f(MV + 0xdc, weave * 0.017_453_294);
        m.set_pv_s(MV + 0xe4, 0xb4);
        m.set_pv_b(MV + 0xe9, 6);
        m.set_pv_f(MV + 0xd4, 3.0);
        m.set_pv_f(MV + 0xd0, f32::from_bits(0x3d88_8889));
        m.set_pv_f(0x328, phase);
        if m.pv_i(0x214) == 0 {
            m.set_state(1, 0);
            set_seq(w, id, ctx, 0, 0, 0);
        } else {
            m.has_collision = false;
            m.mode = (m.mode & !0x1000) | 0x41;
            m.set_state(0xd, 4);
            set_seq(w, id, ctx, 10, 0, 0);
        }
        let m = &mut w.mobys[id];
        let pos = m.pos;
        m.set_pv_v(0x2e0, pos);
    }
    crate::shadow_volumes::probe_near(w, id, ctx, 24.0, 0x16);
    let kind = ai::take_damage(w, id, ctx, false);
    if kind != 0 {
        alarm(w, id);
        if 4 <= kind { w.mobys[id].set_state(0xc, -1); }
    }
    if !matches!(w.mobys[id].sub_state, 3 | 4) {
        let area = w.mobys[id].pv_i(0x210);
        let found = ai::find_target(w, id, ctx, 16.0, 0.0, area, None, false);
        w.mobys[id].set_pv_s(0x314, found as i16);
    }
}

/// `0x3cfe08(m, goal, toward)`: a move to `goal`, straight there or (`toward`) offset by the spread `+0x31c`.
fn go(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, goal: V3, toward: bool) -> i32 {
    if toward {
        let off = w.mobys[id].pv_f(0x31c);
        ai::move_toward(w, id, ctx, off, goal)
    } else {
        let p = w.mobys[id].pos;
        ai::move_step(w, id, ctx, atan2(goal[0] - p[0], goal[1] - p[1]), goal)
    }
}

/// `0x333980(yaw, m)`: its facing eased toward `yaw` at the movement block's turn limits.
fn face(w: &mut MobyWorld, id: usize, yaw: f32) {
    let m = &mut w.mobys[id];
    let (a, b, c) = (m.pv_f(MV + 0x14), m.pv_f(MV + 0x18), m.pv_f(MV + 0x1c));
    let (mut r, mut v) = (m.rot[2], m.pv_f(MV + 0xb4));
    turn_ease(yaw, a, b, c, &mut r, &mut v, 0);
    m.rot[2] = r;
    m.set_pv_f(MV + 0xb4, v);
}

/// `0x3339e0(m, point)`: [`face`] toward a point.
fn face_point(w: &mut MobyWorld, id: usize, p: V3) {
    let q = w.mobys[id].pos;
    face(w, id, atan2(p[0] - q[0], p[1] - q[1]));
}

/// `0x3302b8(r, from, to, &out, path)`: a point toward `to` kept inside the path (crate::path_network::keep_in).
pub(crate) fn keep_in_path(ctx: &Ctx, r: f32, from: V3, to: V3, path: i32) -> V3 {
    let Some(pts) = usize::try_from(path).ok().and_then(|p| ctx.level.paths.get(p)) else { return to };
    crate::path_network::keep_in(pts, r, from, to)
}

/// The stroll's start (states 2 / 5): sequence 6, a point `randf(2, 5)` from home a random way, kept in its path.
fn stroll_start(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    set_seq(w, id, ctx, 6, 0, 0x14);
    let rng = &mut ctx.game.rng;
    let a = rng.rand_angle();
    let r = rng.randf(2.0, 5.0);
    let m = &w.mobys[id];
    let home = m.pv_v(0x2e0);
    let mut p = [a.cos() * r + home[0], a.sin() * r + home[1], home[2]];
    let path = m.pv_i(0x218);
    if path != -1 { p = keep_in_path(ctx, 1.5, home, p, path); }
    w.mobys[id].set_pv_v(0x2f0, p);
}

/// The stroll's step: at least 31 ticks, done there or after 181.
fn stroll(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    let p = w.mobys[id].pv_v(0x2f0);
    go(w, id, ctx, p, false);
    let m = &w.mobys[id];
    !(m.state_ticks < 0x1f || (0.5 < m.pv_f(MV + 0xcc) && m.state_ticks < 0xb5))
}

/// `0x32f758(m, at, group)`: its group's others to the right of the way to `at` less those to the left.
fn spread(w: &MobyWorld, id: usize, at: V3) -> i32 {
    let m = &w.mobys[id];
    let yaw = atan2(at[0] - m.pos[0], at[1] - m.pos[1]);
    let (s, c) = yaw.sin_cos();
    let mut n = 0;
    for o in group(w, id) {
        let x = &w.mobys[o];
        if x.state >= 0x80 || o == id { continue; }
        let d = sub(x.pos, m.pos);
        if 0.0 < -d[0] * s + d[1] * c { n -= 1 } else { n += 1 }
    }
    n
}

/// `0x3d01c8`: the head's aim at the target in its frame, yaw within ±45° (`+0x2c0`), pitch within ±30° (`+0x2c8`).
fn aim(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    let d = sub(m.pv_v(TARGET), m.pos);
    let r = m.rows;
    let l = [r[0][0] * d[0] + r[0][1] * d[1] + r[0][2] * d[2], r[1][0] * d[0] + r[1][1] * d[1] + r[1][2] * d[2], r[2][0] * d[0] + r[2][1] * d[1] + r[2][2] * d[2]];
    let q = std::f32::consts::FRAC_PI_4;
    m.set_pv_f(0x2c0, atan2(l[0], l[1]).clamp(-q, q));
    let s = f32::from_bits(0x3f06_0a93);
    m.set_pv_f(0x2c8, (-atan2(len2(l), l[2])).clamp(-s, s));
}

/// `0x3cff80`: dying — its bolts (3), its sound 1, a small blast (size 1/3: ten puffs, twenty streaks, the flash), its
/// pieces flung away from Ratchet (3..6 across and up, at 0.1) and it goes.
fn die(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    crate::moby::bolts::drop(w, id, 0, 3, ctx);
    ctx.host.play_sound(&w.mobys[id], 1, 0);
    let pos = w.mobys[id].pos;
    let b = crate::moby::blast::Blast { size: f32::from_bits(0x3eaa_7efa), bursts: [0; 3], puffs: 10, streaks: 20, light: true, shake: false };
    crate::moby::blast::burst(w, ctx, pos, &b);
    let mut away = sub(pos, ctx.hero.pos);
    away[2] = 0.0;
    let v = crate::hero::physics::set_len(away, f32::from_bits(0x3dcc_ccce));
    crate::moby::shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 6.0), 0x1f, v, 0xff);
    w.delete(id);
}

/// 0x0a4e (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    senses(w, id, ctx);
    if w.mobys[id].state >= 0x80 { return; }
    match w.mobys[id].sub_state {
        0 => {
            let rng = &mut ctx.game.rng;
            if first_sub(&mut w.mobys[id]) {
                ai::senses(&mut w.mobys[id], 0.005, f32::from_bits(0x3d88_8889), 0.001_745_329_3, 0.418_879);
                let t = rng.rand_range(0, 0x1e);
                let m = &mut w.mobys[id];
                m.set_pv_f(MV + 0xd0, f32::from_bits(0x3d88_8889));
                m.set_pv_f(MV + 0x24, f32::from_bits(0x3bda_740e));
                m.set_pv_f(MV + 0x34, 0.5);
                m.set_pv_s(0x33a, t as i16);
            }
            let rng = &mut ctx.game.rng;
            let m = &mut w.mobys[id];
            let mut seen = false;
            if m.pv_s(0x314) == 1 {
                let mut t = m.pv_s(0x33a) as i32;
                seen = crate::moby::countdown(&mut t);
                m.set_pv_s(0x33a, t as i16);
                if seen {
                    let s = if rng.randi(3) != 0 { 9 } else { 8 };
                    m.set_state(s, 2);
                }
            }
            if !seen && m.pv_s(0x316) != 0 {
                let s = if rng.randi(2) == 0 { 6 } else { 7 };
                m.set_state(s, 1);
                m.set_pv_s(0x316, 0);
                if m.pv_s(0x310) < 300 { m.set_pv_s(0x310, rng.rand_range(0x1e0, 600) as i16); }
            }
        }
        1 => {
            if first_sub(&mut w.mobys[id]) {
                ai::senses(&mut w.mobys[id], 0.005, f32::from_bits(0x3d88_8889), f32::from_bits(0x3c8e_fa36), f32::from_bits(0x3f86_0a93));
                let m = &mut w.mobys[id];
                m.set_pv_f(MV + 0xd0, f32::from_bits(0x3d88_8889));
                m.set_pv_f(MV + 0x34, 0.05);
            }
            let m = &mut w.mobys[id];
            if m.pv_s(0x314) == 1 { m.set_state(9, 2) } else if m.pv_s(0x310) == 0 { m.set_state(1, 0) }
        }
        2 => {
            if first_sub(&mut w.mobys[id]) {
                ai::senses(&mut w.mobys[id], 0.005, f32::from_bits(0x3d88_8889), f32::from_bits(0x3c8e_fa36), f32::from_bits(0x3f86_0a93));
                let t = ctx.game.rng.rand_range(0, 0x1e);
                let m = &mut w.mobys[id];
                m.set_pv_f(MV + 0xd0, f32::from_bits(0x3d88_8889));
                m.set_pv_f(MV + 0x24, f32::from_bits(0x3bda_740e));
                m.set_pv_f(MV + 0x34, 0.05);
                m.set_pv_s(0x33a, t as i16);
            }
            if w.mobys[id].pv_s(0x314) != 1 {
                let m = &mut w.mobys[id];
                let mut t = m.pv_s(0x33a) as i32;
                let done = crate::moby::countdown(&mut t);
                m.set_pv_s(0x33a, t as i16);
                if done { back_off(w, id, ctx); }
            }
        }
        3 => {
            let res = ai::flight(w, id, ctx);
            if res & 1 != 0 {
                die(w, id, ctx);
                return;
            }
            if res & 2 != 0 {
                if w.mobys[id].pv_s(0x314) == 1 { w.mobys[id].set_state(9, 2) } else { back_off(w, id, ctx) }
            }
        }
        _ => {}
    }
    states(w, id, ctx);
    if w.mobys[id].state >= 0x80 { return; }
    tail(w, id, ctx);
    if w.mobys[id].drawn != 0 { glow(w, id, ctx); }
}

/// State 3, sub 1, the idle timer `rand_range(580, 600)`.
fn back_off(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let t = ctx.game.rng.rand_range(0x244, 600);
    let m = &mut w.mobys[id];
    m.set_state(3, 1);
    m.set_pv_s(0x310, t as i16);
}

fn states(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let next = match w.mobys[id].state {
        1 => {
            if w.mobys[id].drawn == 0 { return; }
            if w.mobys[id].first() {
                let r = ctx.game.rng.randi(4);
                if (seq(w, id, ctx) != 0) != (r == 0) { ctx.host.set_anim(&w.mobys[id], (r == 0) as u8, 0, 10); }
                let t = ctx.game.rng.rand_range(0x3c, 0x78);
                w.mobys[id].set_pv_s(0x312, t as i16);
            }
            if seq(w, id, ctx) == 1 {
                if wrapped(w, id, ctx) { ctx.host.set_anim(&w.mobys[id], 0, 0, 0x14); }
                return;
            }
            // The gravity frames' upright test (`+0x338`) is not ported.
            if w.mobys[id].pv_s(0x312) != 0 { return; }
            2
        }
        2 | 5 => {
            let s = w.mobys[id].state;
            if w.mobys[id].first() { stroll_start(w, id, ctx); }
            if !stroll(w, id, ctx) { return; }
            if s == 2 { 1 } else { 3 }
        }
        3 => {
            if w.mobys[id].first() {
                let r = ctx.game.rng.randi(2) as u8 + 4;
                if seq(w, id, ctx) != r {
                    let r = ctx.game.rng.randi(2) as u8 + 4;
                    set_seq(w, id, ctx, r, 0, 10);
                }
            }
            if dist(w.mobys[id].pos, ctx.hero.pos) < NEAR { 4 } else {
                if !wrapped(w, id, ctx) { return; }
                match ctx.game.rng.randi(3) { 0 => 6, 1 => 7, _ => 5 }
            }
        }
        4 => {
            if w.mobys[id].first() {
                set_seq(w, id, ctx, 6, 0, 10);
                let r = ctx.game.rng.randf(NEAR, FAR) + 0.5;
                w.mobys[id].set_pv_f(0x334, r);
            }
            let home = w.mobys[id].pv_v(0x2e0);
            let flags = go(w, id, ctx, home, false);
            if flags & 0x80 == 0 && dist2(w.mobys[id].pos, ctx.hero.pos) <= w.mobys[id].pv_f(0x334) { return; }
            if ctx.game.rng.randi(2) == 0 { 7 } else { 6 }
        }
        6 | 7 => {
            let s = w.mobys[id].state;
            if w.mobys[id].first() {
                if s == 6 {
                    let r = ctx.game.rng.randi(2) as u8 + 2;
                    if seq(w, id, ctx) != r {
                        let r = ctx.game.rng.randi(2) as u8 + 2;
                        set_seq(w, id, ctx, r, 0, 10);
                    }
                } else {
                    set_seq(w, id, ctx, 0xc, 0, 10);
                }
            }
            let t = w.mobys[id].pv_v(TARGET);
            face_point(w, id, t);
            if !wrapped(w, id, ctx) { return; }
            3
        }
        8 => {
            if w.mobys[id].first() {
                let r = ctx.game.rng.randi(2) as u8 + 2;
                if seq(w, id, ctx) != r {
                    let r = ctx.game.rng.randi(2) as u8 + 2;
                    set_seq(w, id, ctx, r, 0, 10);
                }
            }
            if !wrapped(w, id, ctx) { return; }
            9
        }
        9 => {
            if w.mobys[id].first() { set_seq(w, id, ctx, 6, 0, 5); }
            if w.mobys[id].state_ticks & 3 == 0 {
                let n = spread(w, id, ctx.hero.pos);
                w.mobys[id].set_pv_f(0x31c, n as f32 * 0.087_266_475);
            }
            let t = w.mobys[id].pv_v(TARGET);
            go(w, id, ctx, t, true);
            aim(w, id);
            if 1.25 < w.mobys[id].pv_f(MV + 0xcc) { return; }
            10
        }
        10 => {
            if w.mobys[id].first() {
                let m = &w.mobys[id];
                let t = m.pv_v(TARGET);
                let yaw = atan2(t[0] - m.pos[0], t[1] - m.pos[1]);
                w.mobys[id].set_pv_f(0x320, yaw);
                if seq(w, id, ctx) != 8 { ctx.host.set_anim(&w.mobys[id], 8, 2, 10); }
            }
            let yaw = w.mobys[id].pv_f(0x320);
            face(w, id, yaw);
            if ai::reached(w, id, ctx, 13.0) {
                let damage = w.mobys[id].pv_b(0x34) as f32;
                ai::attack_joint(w, id, ctx, 0.75, damage, 1.0, 1, 0x400_0001, 0, 1, 0);
            }
            if !wrapped(w, id, ctx) { return; }
            9
        }
        0xc => {
            die(w, id, ctx);
            return;
        }
        0xe => {
            drop_in(w, id, ctx);
            return;
        }
        _ => return,
    };
    w.mobys[id].set_state(next, -1);
}

/// The drop (state 0xe, `0x3d0710` and the state's tick): sequence 10, its moves (the goal the unit of its velocity,
/// as the game passes it), glints falling about it, a hit around it (radius 0.6, flags 0x4000000); landed (under the
/// movement block's floor `+0x190`) it chases or strolls, options `&= ~0x11`.
pub const DROP: u8 = 0xe;
fn drop_in(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].first() {
        set_seq(w, id, ctx, 10, 0, 5);
        let f = ctx.game.frame;
        w.mobys[id].set_pv_i(MV + 0xbc, f);
    }
    let v = w.mobys[id].pv_v(MV + 0xa0);
    let goal = crate::hero::physics::set_len(v, 1.0);
    go(w, id, ctx, goal, false);
    let pos = w.mobys[id].pos;
    let hero = ctx.hero.pos;
    if let Some(sys) = ctx.parts.as_deref_mut() {
        let rng = &mut ctx.game.rng;
        let life = rng.rand_range(5, 10) as u16;
        let rot = rng.randi(0xff) as u8;
        crate::particles::glint(sys, 1.5, pos, [0.0; 3], 0x6080_6000, life, rot, false, hero);
        let off = rng.rand_vec(0.0, 0.5);
        let vz = -(rng.randf(0.0, 1.0) * 0.016_666_668);
        let life = rng.rand_range(40, 90) as u16;
        let rot = rng.randi(0xff) as u8;
        crate::particles::glint(sys, 1.0, add(pos, off), [0.0, 0.0, vz], 0x6090_4000, life, rot, false, hero);
    }
    ai::attack_sphere(w, id, ctx, 0.6, 1.0, 1.0, pos, 0x400_0000, 0, 1, 0);
    let m = &mut w.mobys[id];
    if m.pos[2] <= m.pv_f(MV + 0x90) {
        if m.pv_s(0x314) == 1 { m.set_state(8, 2) } else { m.set_state(2, 0) }
        m.set_pv_i(MV + 0x38, m.pv_i(MV + 0x38) & !0x11);
    }
}

/// `0x3cfe60`: its timers `+0x310` / `+0x312` down, its state's ticks on (the head's and tail's aims not ported).
fn tail(w: &mut MobyWorld, id: usize, _ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    for off in [0x310, 0x312] {
        let mut t = m.pv_s(off) as i32;
        crate::moby::countdown(&mut t);
        m.set_pv_s(off, t as i16);
    }
    if m.state_ticks != -1 { m.state_ticks = m.state_ticks.wrapping_add(1); }
}

/// `0x3d0398` (module docs).
fn glow(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    let p = add_rot(m.pv_f(0x328), f32::from_bits(0x3e56_7752));
    m.set_pv_f(0x328, p);
    let k = p.sin() * 0.5 + 0.5;
    m.set_pv_f(0x32c, k);
    m.glow = lerp_rgba(k, 0x8080_8080, 0x0020_2000);
    let frame = ctx.game.frame;
    if m.pv_i(0x330) == frame { return; }
    for o in group(w, id) {
        let x = &w.mobys[o];
        if x.drawn == 0 || x.o_class != CLASS || x.state >= 0x80 { continue; }
        let Some((rows, at)) = ctx.host.joint_matrix(x, 0) else { continue };
        let x = &mut w.mobys[o];
        for (k, r) in rows.iter().enumerate() { x.set_pv_v(0x220 + 0x10 * k, *r); }
        x.set_pv_v(0x250, at);
        x.set_pv_i(0x330, frame);
    }
    w.register_draw(id);
}

/// `0x3d0530` (the group's draw callback): each drawn, living 0x0a4e's eye, a glow (effect texture 0xb, additive)
/// `0.2·size` across, 0.08 toward the camera, at `(0.25, 0, 0.1)·size` in its joint 0's frame, its colour the pulse's
/// way from `0x80602000` to `0x40402000`.
pub fn draw(w: &MobyWorld, id: usize, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    for o in group(w, id) {
        let x = &w.mobys[o];
        if x.drawn == 0 || x.o_class != CLASS || x.state >= 0x80 { continue; }
        let s = x.pv_f(0x324);
        let rows = [x.pv_v(0x220), x.pv_v(0x230), x.pv_v(0x240)];
        let l = [0.25 * s, 0.0, 0.1 * s];
        let p: V3 = std::array::from_fn(|k| l[0] * rows[0][k] + l[1] * rows[1][k] + l[2] * rows[2][k] + x.pv_f(0x250 + 4 * k));
        let rgba = lerp_rgba(x.pv_f(0x32c), 0x8060_2000, 0x4040_2000);
        crate::fx::sprite(out, s * 0.2, f32::from_bits(0x3da3_d70a), 0.0, p, cam, 0xb, rgba, true);
    }
}
