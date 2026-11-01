//! Endako's small enemy **0x0ff3** (L03 `0x41d510`, 87 placed; its senses `0x41e138`, tail `0x41e570`), on the
//! framework ([`crate::moby::ai`]): health block `+0x20`, movement `+0x50`, reaction `+0x140`, target `+0x1f0`; its
//! fence path `+0x290`, area `+0x294`, the big kind `+0x29c` (≠ 0), the spawn cuboid `+0x2a0` (−1 none), the respawn
//! `+0x2a4`, the view cuboid `+0x2a8` (−1 none), home `+0x2b0`, its goal `+0x2c0`.
//!
//! **The set-up** (state 0, in [`senses`]): its size `f` = 0.75 (big: 0.666, health 1), the scale; the reaction (`+0x48`
//! 20, `+0x4a` 6, hurt state 7, `+0x28` 3, `+0x188` 3, `+0x170` / `+0x174` 0.5, `+0x17c` 0.4) and movement (radius
//! f/2, height f/4, steps 0.4, slope 45°, turn left 30°, stop f/20, turns 1440″ / 1440°/s, walk 15·f/s², at most 5·f/s,
//! its fence the path); no fence path: deleted. With a spawn cuboid or a respawn it waits hidden (state 8, sub 4), else idle (1, 0).
//!
//! **Each tick** ([`senses`]): the frame's damage — a kill throws its pieces, a smoke burst, its sound 2 (twice), its
//! bolts, alarms its group's 0x0ff3 (`+0x2d6`, `0x41e768`) and deletes it; its shadow within 24 of the camera (range
//! 0x16); outside the knock its target (Ratchet within 16 in its area).
//!
//! **Subs** (`+0x95`): 0 calm — with a view cuboid it is drawn only while the camera is in it; seeing him it chases
//! (5, sub 2); alarmed it stares (3, sub 1) and its calm timer `+0x2d0` is `rand_range(480, 600)`. 1 wary — seeing him
//! the chase; more than 24 from him with the timer out, calm. 2 chasing — its weave `randf_signed(30°, 45°)`; losing him
//! the stare, timer `rand_range(580, 600)`. 3 the knock: dead (a fall) → as a kill, then deleted or (with the respawn)
//! back to waiting; over, the stare or the bite (6, sub 2).
//!
//! **States**: 1 idle — drawn, 11 ticks in, within 50 of the camera: a stroll to a point 2..5 from home a random way
//! kept 1.5 inside its fence (`0x32bd40`), 2. 2 the stroll (walking) until there or 301 ticks → 1. 3 facing Ratchet,
//! 11 ticks → 4. 4 a turn-free stroll (1.0 inside) facing him → 3. 5 the chase: its sound 5; toward the target with
//! the weave and its offset (flipped when stuck 61 ticks, every 60), facing it; 61 ticks in and within `reach·f`
//! (3.5, big 1) across → 6. 6 the bite (sequence 1): ticks 11..45 a hit sphere `1.25·f` at 0.25 up (damage `+0x34`,
//! flags 0x4000001); then 5. 8 waiting, hidden: hooked (crate::moby::classes::endako_a's locks) and its timer `+0x2d8`
//! out, it teleports in (crate::moby::classes::teleport) at home (Ratchet more than 5 away; health back) or at its spawn
//! cuboid's ground (no other 0x0ff3 there; a hit there pushes away what is; else 10..20 ticks more), its group's 0x0ff3 timed
//! 40..50; 9. 9 into the arena: its sound 3 once, toward 6 along its spawn cuboid's x until inside its fence, then the
//! chase.
//!
//! **The tail** (`0x41e570`): the timers `+0x2d0` / `+0x2d2` down, its ticks; while drawn its eyes' colour pulses
//! (`+0x2e8`: calm green, wary yellow, chasing red; eased 0.2 a tick) on its glow word and a glow there (94500, 0.1
//! toward the camera, at (0.33, 0, 0.075), 2 ticks).
//!
//! With Ratchet steering Endako's crane (state 0x31, crate::moby::classes::endako_crane) states 3 and 4 face the
//! crane (`0x18b650`) instead of him, and its deaths (the frame's damage and the knock's fall) count toward skill
//! point 0x1c (crate::moby::classes::endako_c::crane_kill, `$gp − 0x5fac`).
//!
//! Not ported: the eyes' glow on joint 0 (the port's glows take the rows), the head's aim
//! (`0x327308`, 1.8 at `+0x210`), the spawn spot's world test (`0x2df8d8`: only the mobys there are looked at), state 9
//! without a spawn cuboid (the game reads before the cuboid table; here straight to the chase).

use crate::hero::physics::{add, add_rot, atan2, dist2, turn_ease, V3};
use crate::hud::lerp_rgba;
use crate::moby::{ai, Ctx, MobyWorld};
use super::props::seq;

pub const CLASS: i16 = 0x0ff3;
const HEALTH: usize = 0x20;
const MV: usize = 0x50;
const REACT: usize = 0x140;
const TARGET: usize = 0x1f0;
const FENCE: usize = 0x290;
const AREA: usize = 0x294;
const BIG: usize = 0x29c;
const SPAWN: usize = 0x2a0;
const RESPAWN: usize = 0x2a4;
const VIEW: usize = 0x2a8;
const HOME: usize = 0x2b0;
const GOAL: usize = 0x2c0;
/// `$gp+0x2b40..`: walk accel (/s²), walk most (/s), turn accel (″), turn most (°/s).
const WALK: (f32, f32, f32, f32) = (15.0, 5.0, 1440.0, 1440.0);

/// `0x32bec8` / `0x32bef0`: the first tick of the state / sub-state.
fn first_sub(m: &mut crate::moby::Moby) -> bool {
    if m.once & 2 != 0 { return false; }
    m.once |= 2;
    true
}

fn size(m: &crate::moby::Moby) -> f32 { if m.pv_i(BIG) != 0 { f32::from_bits(0x3f2a_7efa) } else { 0.75 } }

fn group(w: &MobyWorld, id: usize) -> Vec<usize> {
    usize::try_from(w.mobys[id].group).ok().and_then(|g| w.groups.get(g)).cloned().unwrap_or_default()
}

/// `0x31c9b8(m, seq, 0, blend)` unless it shows `seq`.
fn play(w: &MobyWorld, id: usize, ctx: &mut Ctx, s: u8, blend: u8) {
    if seq(w, id, ctx) != s { ctx.host.set_anim(&w.mobys[id], s, 0, blend); }
}

/// `0x32f4e8(yaw, m)`: its facing eased toward `yaw` at the movement block's turn limits.
fn face(w: &mut MobyWorld, id: usize, yaw: f32) {
    let m = &mut w.mobys[id];
    let (a, b, c) = (m.pv_f(MV + 0x14), m.pv_f(MV + 0x18), m.pv_f(MV + 0x1c));
    let (mut r, mut v) = (m.rot[2], m.pv_f(MV + 0xb4));
    turn_ease(yaw, a, b, c, &mut r, &mut v, 0);
    m.rot[2] = r;
    m.set_pv_f(MV + 0xb4, v);
}

fn yaw_to(w: &MobyWorld, id: usize, p: V3) -> f32 { let q = w.mobys[id].pos; atan2(p[0] - q[0], p[1] - q[1]) }

/// `0x32e328(yaw, −1, −1, m, goal, opts)` / `0x32e408(offset, …)` (with the weave): the frame's move with the movement
/// options `opts` for its length.
fn move_opts(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, goal: V3, opts: i32, weave: Option<f32>) -> i32 {
    let o = MV + 0x38;
    let kept = w.mobys[id].pv_i(o);
    w.mobys[id].set_pv_i(o, opts);
    let r = match weave {
        Some(off) => ai::move_toward(w, id, ctx, off, goal),
        None => { let y = yaw_to(w, id, goal); ai::move_step(w, id, ctx, y, goal) }
    };
    w.mobys[id].set_pv_i(o, kept);
    r
}

/// The death's pieces and smoke (as in `0x41e138` and sub 3): at 0.25 up its sound 2 and a smoke burst (1/3), its
/// pieces thrown 3..6 across and up (mask 0x1f, no velocity).
fn burst(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let at = add(super::endako_a::to_world([0.0, 0.0, 0.25], &m.rows), m.pos);
    ctx.host.play_sound(&w.mobys[id], 2, 0);
    crate::moby::blast::smoke_burst(w, ctx, f32::from_bits(0x3eaa_7efa), 0.0, Some(id), at, -1);
    crate::moby::shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 6.0), 0x1f, [0.0; 3], 0xff);
}

/// `0x41e768`: its group's living 0x0ff3 alarmed (`+0x2d6`).
fn alarm(w: &mut MobyWorld, id: usize) {
    for i in group(w, id) {
        let x = &mut w.mobys[i];
        if x.state < 0x80 && x.o_class == CLASS { x.set_pv_s(0x2d6, 1); }
    }
}

/// `0x41e7e8`: its group's living 0x0ff3 wait 40..50 ticks before they come.
fn stagger(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    for i in group(w, id) {
        if w.mobys[i].state < 0x80 && w.mobys[i].o_class == CLASS {
            let t = ctx.game.rng.rand_range(0x28, 0x32);
            w.mobys[i].set_pv_i(0x2d8, t);
        }
    }
}

/// `0x41e138` (module docs). False when it went.
fn senses(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    if w.mobys[id].state == 0 {
        if w.mobys[id].pv_i(FENCE) == -1 {
            w.delete(id);
            return false;
        }
        let class_scale = w.class(CLASS).map_or(1.0, |c| c.scale);
        let m = &mut w.mobys[id];
        let mut f = 0.75;
        if m.pv_i(BIG) != 0 {
            m.set_pv_f(HEALTH, 1.0);
            m.set_pv_s(HEALTH + 4, 1);
            f = f32::from_bits(0x3f2a_7efa);
        }
        m.scale = class_scale * f;
        m.set_pv_b(0x48, 20);
        m.set_pv_b(0x4a, 6);
        m.set_pv_b(REACT + 0x10, 7);
        m.set_pv_b(0x28, 3);
        m.set_pv_f(MV + 0x18, 0.209_439_51);
        m.set_pv_f(MV + 0x34, f * 0.05);
        m.set_pv_f(MV + 4, f * 0.25);
        m.set_pv_f(MV, f * 0.5);
        m.set_pv_f(MV + 0xc, f32::from_bits(0x3ecc_cccd));
        m.set_pv_f(MV + 0x10, f32::from_bits(0x3f49_0fdc));
        m.set_pv_f(MV + 0x30, f32::from_bits(0x3f06_0a93));
        m.set_pv_i(MV + 0x3c, m.pv_i(FENCE));
        m.set_pv_i(0x2ac, 0);
        m.set_pv_i(REACT + 0x48, 3);
        m.set_pv_f(REACT + 0x30, 0.5);
        m.set_pv_f(REACT + 0x34, 0.5);
        m.set_pv_f(REACT + 0x3c, f32::from_bits(0x3ecc_cccd));
        m.set_pv_f(MV + 0x14, 0.209_439_51);
        m.set_pv_f(MV + 8, f32::from_bits(0x3ecc_cccd));
        let pos = m.pos;
        m.set_pv_v(HOME, pos);
        // `ld +0x2a0` against 0xffffffff: no spawn cuboid and no respawn.
        if m.pv_i(SPAWN) == -1 && m.pv_i(RESPAWN) == 0 {
            m.set_state(1, 0);
            play(w, id, ctx, 0, 0);
        } else {
            let m = &mut w.mobys[id];
            m.set_state(8, 4);
            m.has_collision = false;
            m.mode = (m.mode & 0xefff) | 0x41;
        }
        let m = &mut w.mobys[id];
        let a = WALK.0 * f * 0.000_277_777_78;
        m.set_pv_f(MV + 0x24, a);
        m.set_pv_f(MV + 0x20, a);
        m.set_pv_f(MV + 0x28, WALK.1 * f * 0.016_666_668);
        m.set_pv_f(MV + 0x18, WALK.2 * 4.848_137_5e-6);
        m.set_pv_f(MV + 0x1c, WALK.3 * 0.000_290_888_25);
        m.set_pv_f(MV + 0x14, WALK.2 * 4.848_137_5e-6);
    }
    let kind = ai::take_damage(w, id, ctx, false);
    if kind != 0 && w.mobys[id].pv_f(HEALTH) <= 0.0 {
        burst(w, id, ctx);
        super::endako_c::crane_kill(w, ctx);
        ctx.host.play_sound(&w.mobys[id], 2, 0);
        alarm(w, id);
        crate::moby::bolts::drop(w, id, 0, 3, ctx);
        w.delete(id);
        return false;
    }
    crate::shadow_volumes::probe_near(w, id, ctx, 24.0, 0x16);
    if w.mobys[id].sub_state < 3 {
        let area = w.mobys[id].pv_i(AREA);
        ai::find_target(w, id, ctx, 16.0, 0.0, area, None, false);
    }
    true
}

/// A stroll's goal: `randf(2, 5)` from home a random way, kept `r` inside its fence (`0x32bd40`).
fn stroll_goal(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, r: f32) {
    let rng = &mut ctx.game.rng;
    let d = rng.randf(2.0, 5.0);
    let a = rng.rand_angle();
    let m = &w.mobys[id];
    let home = m.pv_v(HOME);
    let to = add([a.cos() * d, a.sin() * d, 0.0], home);
    let p = super::critter::keep_in_path(ctx, r, home, to, m.pv_i(FENCE));
    w.mobys[id].set_pv_v(GOAL, p);
}

/// The spawn cuboid's record (its place and angles; its x axis).
fn spawn_cuboid(ctx: &Ctx, i: i32) -> Option<(V3, V3, V3)> {
    let c = usize::try_from(i).ok().and_then(|i| ctx.level.cuboids.get(i))?;
    let (p, r) = ctx.level.cuboid_place(i)?;
    Some((p, r, [c.matrix[0][0], c.matrix[0][1], c.matrix[0][2]]))
}

/// 0x0ff3 (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if !senses(w, id, ctx) { return; }
    let f = size(&w.mobys[id]);
    let reach = if w.mobys[id].pv_i(BIG) != 0 { 1.0 } else { 3.5 };
    let found = |w: &MobyWorld| w.mobys[id].pv_i(TARGET + 0x14) == 1;
    // The stare: from sub 2 its idle sequence too; after the knock not.
    let stare = |w: &mut MobyWorld, ctx: &mut Ctx, anim: bool| {
        w.mobys[id].set_state(3, 1);
        if anim { play(w, id, ctx, 0, 10); }
        let t = ctx.game.rng.rand_range(0x244, 600);
        w.mobys[id].set_pv_s(0x2d0, t as i16);
    };
    let chase = |w: &mut MobyWorld, ctx: &mut Ctx| {
        w.mobys[id].set_state(5, 2);
        play(w, id, ctx, 0, 10);
    };
    match w.mobys[id].sub_state {
        0 => {
            let view = w.mobys[id].pv_i(VIEW);
            if view != -1 {
                let m = &mut w.mobys[id];
                if !ctx.level.in_cuboid(view, ctx.host.camera_pos()) {
                    m.drawn = 0;
                    m.mode |= 1;
                    return;
                }
                m.drawn = 1;
                m.mode &= !1;
            }
            if found(w) {
                chase(w, ctx);
            } else if w.mobys[id].pv_s(0x2d6) != 0 {
                w.mobys[id].set_state(3, 1);
                w.mobys[id].set_pv_s(0x2d6, 0);
                if w.mobys[id].pv_s(0x2d0) < 300 {
                    let t = ctx.game.rng.rand_range(0x1e0, 600);
                    w.mobys[id].set_pv_s(0x2d0, t as i16);
                }
            }
        }
        1 => {
            if found(w) {
                chase(w, ctx);
            } else if 24.0 < dist2(w.mobys[id].pos, ctx.hero.pos) && w.mobys[id].pv_s(0x2d0) == 0 {
                w.mobys[id].set_state(1, 0);
            }
        }
        2 => {
            if first_sub(&mut w.mobys[id]) {
                let s = ctx.game.rng.randf_signed(30.0, 45.0);
                w.mobys[id].set_pv_f(0x2e0, s * 0.017_453_294);
            }
            if !found(w) { stare(w, ctx, true); }
        }
        3 => {
            let r = ai::flight(w, id, ctx);
            if r & 1 != 0 {
                burst(w, id, ctx);
                super::endako_c::crane_kill(w, ctx);
                crate::moby::bolts::drop(w, id, 0, 3, ctx);
                if w.mobys[id].pv_i(RESPAWN) == 0 {
                    w.delete(id);
                    return;
                }
                let m = &mut w.mobys[id];
                m.set_state(8, 4);
                m.has_collision = false;
                m.mode = (m.mode & 0xefff) | 0x41;
            } else if r & 2 != 0 {
                if !found(w) { stare(w, ctx, false); } else { w.mobys[id].set_state(6, 2); }
            }
        }
        _ => {}
    }
    match w.mobys[id].state {
        1 => {
            let m = &w.mobys[id];
            if m.drawn != 0 && 0xb <= m.state_ticks as u16 && dist2(m.pos, ctx.host.camera_pos()) < 50.0 {
                stroll_goal(w, id, ctx, 1.5);
                w.mobys[id].set_state(2, -1);
            }
        }
        2 => {
            if w.mobys[id].first() {
                let o = MV + 0x38;
                let v = w.mobys[id].pv_i(o) & !0x10;
                w.mobys[id].set_pv_i(o, v);
            }
            let g = w.mobys[id].pv_v(GOAL);
            let y = yaw_to(w, id, g);
            let r = ai::move_step(w, id, ctx, y, g);
            if r & 0x80 != 0 || 0x12d <= w.mobys[id].state_ticks as u16 { w.mobys[id].set_state(1, -1); }
        }
        3 => {
            let y = yaw_to(w, id, watched(w, ctx));
            face(w, id, y);
            if 0xb <= w.mobys[id].state_ticks as u16 { w.mobys[id].set_state(4, -1); }
        }
        4 => {
            if w.mobys[id].first() {
                let o = MV + 0x38;
                let v = w.mobys[id].pv_i(o) | 0x10;
                w.mobys[id].set_pv_i(o, v);
                stroll_goal(w, id, ctx, 1.0);
            }
            let y = yaw_to(w, id, watched(w, ctx));
            face(w, id, y);
            let g = w.mobys[id].pv_v(GOAL);
            let y = yaw_to(w, id, g);
            let r = ai::move_step(w, id, ctx, y, g);
            if r & 0x80 != 0 || 0x12d <= w.mobys[id].state_ticks as u16 { w.mobys[id].set_state(3, -1); }
        }
        5 => {
            if w.mobys[id].first() { ctx.host.play_sound(&w.mobys[id], 5, 0); }
            let m = &mut w.mobys[id];
            let mut t = m.pv_i(0x2e4);
            let out = crate::moby::countdown(&mut t);
            m.set_pv_i(0x2e4, t);
            if out && 0x3d <= m.pv_i(MV + 0x80) {
                m.set_pv_i(0x2e4, 0x3c);
                m.set_pv_f(0x2e0, -m.pv_f(0x2e0));
            }
            let p = w.mobys[id].pv_v(TARGET);
            let y = yaw_to(w, id, p);
            face(w, id, y);
            let off = w.mobys[id].pv_f(0x2e0);
            move_opts(w, id, ctx, p, 0x10, Some(off));
            let m = &w.mobys[id];
            if 0x3c < m.state_ticks as u16 && dist2(m.pos, ctx.hero.pos) < reach * f {
                w.mobys[id].set_state(6, -1);
                play(w, id, ctx, 1, 10);
            }
        }
        6 => {
            let t = w.mobys[id].state_ticks as u16;
            if t < 0x2e {
                if 10 < t {
                    let m = &w.mobys[id];
                    let at = [m.pos[0], m.pos[1], m.pos[2] + 0.25];
                    let dmg = m.pv_b(HEALTH + 0x14) as f32;
                    ai::attack_sphere(w, id, ctx, f * 1.25, dmg, 1.0, at, 0x0400_0001, 0, 1, 0);
                }
            } else {
                w.mobys[id].set_state(5, -1);
                play(w, id, ctx, 0, 10);
            }
            let p = w.mobys[id].pv_v(TARGET);
            let y = yaw_to(w, id, p);
            face(w, id, y);
            move_opts(w, id, ctx, p, 0x10, None);
        }
        8 => wait(w, id, ctx),
        9 => {
            if w.mobys[id].pv_i(0x2ac) == 0 {
                ctx.host.play_sound(&w.mobys[id], 3, 0);
                w.mobys[id].set_pv_i(0x2ac, 1);
            }
            let Some((p, _, x)) = spawn_cuboid(ctx, w.mobys[id].pv_i(SPAWN)) else {
                w.mobys[id].set_state(5, 2);
                return tail(w, id, ctx);
            };
            let g = add(p, [x[0] * 6.0, x[1] * 6.0, x[2] * 6.0]);
            let y = yaw_to(w, id, g);
            ai::move_step(w, id, ctx, y, g);
            if inside_fence(w, id, ctx) {
                let fence = w.mobys[id].pv_i(FENCE);
                w.mobys[id].set_pv_i(MV + 0x3c, fence);
                w.mobys[id].set_state(5, 2);
            }
        }
        _ => {}
    }
    tail(w, id, ctx);
}

/// What states 3 and 4 face: Ratchet, or while he steers (state 0x31) the moby he steers (`0x18b650`,
/// crate::moby::classes::endako_crane).
fn watched(w: &MobyWorld, ctx: &Ctx) -> V3 {
    match w.endako.steered.and_then(|s| w.mobys.get(s)) {
        Some(m) if ctx.hero.state == 0x31 => m.pos,
        _ => ctx.hero.pos,
    }
}

/// `0x41e898`: inside its fence path's polygon and clear of its walls by its radius.
fn inside_fence(w: &MobyWorld, id: usize, ctx: &Ctx) -> bool {
    let m = &w.mobys[id];
    let path = m.pv_i(FENCE);
    let Some(pts) = usize::try_from(path).ok().and_then(|p| ctx.level.paths.get(p)) else { return false };
    ctx.level.in_polygon(path, m.pos) && ai::fence(pts, m.pv_f(MV), m.pos).is_none()
}

/// State 8 (module docs).
fn wait(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].once & 4 == 0 { return; }
    let m = &mut w.mobys[id];
    let mut t = m.pv_i(0x2d8);
    let out = crate::moby::countdown(&mut t);
    m.set_pv_i(0x2d8, t);
    if !out { return; }
    let solid = w.class(CLASS).is_some_and(|c| c.has_collision);
    let spawn = w.mobys[id].pv_i(SPAWN);
    if spawn == -1 {
        let home = w.mobys[id].pv_v(HOME);
        if dist2(ctx.hero.pos, home) <= 5.0 { return; }
        w.mobys[id].once &= !4;
        stagger(w, id, ctx);
        let m = &mut w.mobys[id];
        m.set_pv_s(REACT + 0x38, 0);
        let most = m.pv_s(HEALTH + 4) as f32;
        m.set_pv_f(HEALTH, most);
        m.set_state(9, -1);
        m.mode = (m.mode & 0xffbe) | 0x1000;
        m.has_collision = solid;
        play(w, id, ctx, 0, 0);
        let a = ctx.game.rng.rand_angle();
        super::teleport::start(w, id, home, [0.0, 0.0, a], &[]);
        return;
    }
    let Some((p, rot, _)) = spawn_cuboid(ctx, spawn) else { return };
    let mut at = p;
    at[2] = ai::ground_z(ctx, 0.5, p, 0);
    let r = w.mobys[id].pv_f(MV);
    let up = [at[0], at[1], at[2] + r + 0.1];
    let taken = ctx.scene.is_some_and(|sc| {
        crate::collision::coll_sphere_mobys(sc, up, r, crate::collision::QueryFlags(0), Some(id)).first().is_some_and(|&o| w.mobys.get(o).is_some_and(|o| o.o_class == CLASS))
    });
    if taken {
        let t = ctx.game.rng.rand_range(10, 0x14);
        w.mobys[id].set_pv_i(0x2d8, t);
        return;
    }
    w.mobys[id].once &= !4;
    w.mobys[id].set_pv_i(MV + 0x3c, -1);
    stagger(w, id, ctx);
    let m = &mut w.mobys[id];
    m.set_state(9, -1);
    m.mode = (m.mode & 0xffbe) | 0x1000;
    m.has_collision = solid;
    play(w, id, ctx, 0, 0);
    w.mobys[id].set_pv_s(REACT + 0x38, 0);
    ai::attack_sphere(w, id, ctx, r, 2.0, 1.0, up, 0x10001, 0, 1, 0);
    super::teleport::start(w, id, at, rot, &[]);
    w.changed = true;
}

/// `0x41e570` (module docs).
fn tail(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    for o in [0x2d0, 0x2d2] {
        let v = m.pv_s(o);
        if v != 0 { m.set_pv_s(o, v - 1); }
    }
    if m.state_ticks != -1 { m.state_ticks = m.state_ticks.wrapping_add(1); }
    if m.drawn == 0 && 1 < ctx.game.frame { return; }
    let phase = m.pv_f(0x2e8);
    let (a, b, step) = match m.sub_state {
        1 => (0x5020_d0d0, 0x5020_7070, f32::from_bits(0x3e17_e9da)),
        2 => (0x5020_20d0, 0x5020_2070, f32::from_bits(0x3e17_e9da)),
        _ => (0x5020_d020, 0x5020_7020, f32::from_bits(0x3d89_051f)),
    };
    let c = lerp_rgba(phase.sin() * 0.5 + 0.5, a, b);
    m.set_pv_f(0x2e8, add_rot(phase, step));
    m.glow = if ctx.game.frame < 2 { c } else { lerp_rgba(0.2, m.glow, c) };
    let (glow, pos, far) = (m.glow, m.pos, m.draw_dist);
    if m.state != 0xfe && m.state != 0xfd {
        if let Some(sys) = ctx.parts.as_deref_mut() {
            crate::particles::glow_def(sys, &mut ctx.game.rng, 94_500.0, 0.1, [f32::from_bits(0x3ea8_f5c3), 0.0, f32::from_bits(0x3d99_999a)], id, pos, far, false, glow, 2, 1);
        }
    }
}
