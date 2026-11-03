//! Level 13's power lines (L13 addresses; the engine functions named by their L13 addresses with the L00 ones the port
//! knows them by).
//!
//! * **0x0ee7** ([`update_0ee7`], `0x41b828`, 80 placed; its draw [`draw_0ee7`], `0x41c190`): a lightning bolt that
//!   flickers along a line while switched on.
//! * **0x1150** ([`update_1150`], `0x424190`, 20 placed): a transponder: with Ratchet in its cuboid it switches its
//!   group's bolts and a linked effect moby on and off by its timers.
//!
//! The engine functions: `0x325ce0` / `0x325d38` / `0x325d98` / `0x325e00` / `0x325ec0` / `0x325ca0` = L00 `0x31b9e0` /
//! `0x31ba38` / `0x31ba98` / `0x31bb00` / `0x31bbc0` / `0x31b9a0` (crate::rng), `0x2fa748` = L00 `0x2f0080` (the
//! sphere's view test), `0x2fa6c8` = L00 `0x2f0000` (crate::hud::lerp_rgba), `0x33c240` = L00 `0x3327e8`
//! (crate::particles::staged), `0x2f2400` = L00 `0x2e7920` (the draw list), `0x2f63d8` the quad through a matrix
//! (crate::fx::FxQuad), `0x2f9da0` the rows of the angles, `0x2f91c0` / `0x2f9190` the countdowns
//! (crate::moby::countdown), `0x36a308` the level sound after the split, `0x369e10` its stop.

use crate::hero::physics::{add, cross, dot, euler_rows, scale, set_len, sub, V3};
use crate::moby::{Ctx, MobyWorld};

/// 0x0ee7's pvars.
mod pv {
    /// `[0]`: the cuboid giving the bolt's turn (its angles `+0x74` / `+0x78`).
    pub const CUBOID: usize = 0;
    /// `+4` (s16): the ticks of a run of bolts; `+6` (s16) ≠ 0: a pause between runs; `+0xc` (s16): the first wait.
    pub const RUN: usize = 4;
    pub const PAUSES: usize = 6;
    pub const FIRST: usize = 0xc;
    /// `+0x10` (s16): the countdown; `+0x12` (s16): the bolt's ticks left; `+0x620` its ticks in all.
    pub const COUNT: usize = 0x10;
    pub const LIFE: usize = 0x12;
    pub const LIFE_ALL: usize = 0x620;
    /// `+0x14`: its colour now.
    pub const RGBA: usize = 0x14;
    /// `+0x18..0x1c`: the points the four branches leave from.
    pub const ROOTS: usize = 0x18;
    /// `+0x20`: the bolt's 32 points, `+0x220` their velocities (in its frame).
    pub const POINTS: usize = 0x20;
    pub const VELS: usize = 0x220;
    /// `+0x420`: the branches' points (branch `k` point `j` at `+0x420 + k·0x10 + j·0x40`), `+0x520` their velocities.
    pub const BRANCH: usize = 0x420;
    pub const BRANCH_VELS: usize = 0x520;
    /// `+0x624`: its sphere's radius for the view test; `+0x628` ≠ 0: switched on (0x1150).
    pub const RADIUS: usize = 0x624;
    pub const ON: usize = 0x628;
    /// `+0x650`: the angles of its frame.
    pub const ANGLES: usize = 0x650;
}

/// `$gp+0x25cc` / `+0x25d0`: a bolt's ticks (10..=40); `+0x25d4` / `+0x25d8`: its fade's turns (15, 0).
const LIFE_RANGE: (i32, i32) = (10, 40);
const FADE_AT: i16 = 15;
const GONE_AT: i16 = 0;
/// `$gp+0x2600` / `+0x2604` / `+0x2608`: its colours (new, at the fade's turn, last).
const RGBA_NEW: u32 = 0x2040_1040;
const RGBA_FULL: u32 = 0xff80_3040;
const RGBA_LAST: u32 = 0x2040_0000;
/// `$gp+0x2650` / `+0x2654`: a point's spread off the line and its drift (per second).
const SPREAD: f32 = 0.3;
const DRIFT: f32 = 1.0;
/// `$gp+0x25ec`: the FX texture; ALPHA `$gp+0x25f0..0x25fc` (0, 2, 0, 1): `Cs·As + Cd`.
const FX: usize = 0xe;
/// `$gp+0x2610` / `0x1b1604`: the corners' s (plus a quarter a segment, by fours) and t.
const S0: [f32; 4] = [0.0, 0.0, 0.25, 0.25];
const T0: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
/// `$gp+0x265c` / `+0x2660` / `+0x2668` / `+0x2670` / `+0x2674`: the sparks' colours, size and defs.
const SPARK_C: (u32, u32) = (0x1080_6080, 0x1080_6060);
const SPARK_SIZE: f32 = 2.0;
const SPARK_DEF: (i32, i32) = (0x18, 0x35);

fn branch_at(k: usize, j: usize) -> usize { pv::BRANCH + k * 0x10 + j * 0x40 }
fn branch_vel(k: usize, j: usize) -> usize { pv::BRANCH_VELS + k * 0x10 + j * 0x40 }

/// The frame of its angles `+0x650` (`0x2f9da0`): world = its place + the rows by the point.
fn frame(w: &MobyWorld, id: usize) -> ([V3; 3], V3) {
    let m = &w.mobys[id];
    (euler_rows(m.pv_v(pv::ANGLES)), m.pos)
}

fn to_world(rows: &[V3; 3], pos: V3, p: V3) -> V3 { add(pos, add(add(scale(rows[0], p[0]), scale(rows[1], p[1])), scale(rows[2], p[2]))) }

/// `0x41b9e0`: a new bolt. Its ticks `randi(10, 40)` (`+0x12`, `+0x620`), its length `randf(4, 8)`; its 32 points
/// along its y from +½ to −½ the length, all but the three at each end `randvec(0, 0.3)` off the line and drifting
/// `randvec(0, 1/60)` a tick; then each point but the ends (and its drift) the mean of it and its neighbours (×
/// 0.33325). Four branches: each from a random point of `4 + randf(6k, 6k + 6)`, going a random way 0.3 a step
/// (`randvec(0.3, 0.3)`) plus `randvec(0.1, 0.1)`, its three points drifting `randvec(0, 1/60)` a tick. Then, the
/// camera 4 or more away across: its frame turned (`+0x650`: a random yaw about x, the cuboid's y angle ±0..30°, its z
/// angle) and sparks (`0x33c240`, the def 0x18 or 0x35 at random) at points 0, 8, 16, 24 and the tips of branches 0
/// and 1 (def 0x35), each moving its point's drift, size 2, in 0 / 4 / 8 / 12 (the branches: the root's index / 2)
/// ticks, 15 easing, out 20..29 ticks, between 0x10806080 and 0x10806060.
fn build(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let cam = ctx.host.camera_pos();
    let rng = &mut ctx.game.rng;
    let life = rng.rand_range(LIFE_RANGE.0, LIFE_RANGE.1) as i16;
    let length = rng.randf(4.0, 8.0);
    let mut p = [[0.0f32; 3]; 32];
    let mut v = [[0.0f32; 3]; 32];
    for i in 0..32 {
        if (3..29).contains(&i) {
            p[i] = rng.rand_vec(0.0, SPREAD);
            v[i] = rng.rand_vec(0.0, DRIFT * 0.016_666_668);
        }
        p[i][1] += (0.5 - i as f32 / 31.0) * length;
    }
    let k3 = f32::from_bits(0x3eaa_7efa);
    let m = &mut w.mobys[id];
    m.set_pv_s(pv::LIFE, life);
    m.set_pv_i(pv::LIFE_ALL, life as i32);
    for i in 0..32 {
        let (pi, vi) = if i == 0 || i == 31 { (p[i], v[i]) } else { (scale(add(add(p[i - 1], p[i]), p[i + 1]), k3), scale(add(add(v[i - 1], v[i]), v[i + 1]), k3)) };
        m.set_pv_v(pv::POINTS + i * 0x10, pi);
        m.set_pv_v(pv::VELS + i * 0x10, vi);
    }
    for k in 0..4 {
        let dir = rng.rand_vec(SPREAD, SPREAD);
        let root = (rng.randf(k as f32 * 6.0, (k + 1) as f32 * 6.0) as i32 + 4) as usize;
        m.set_pv_b(pv::ROOTS + k, root as u8);
        let (at, vel) = (m.pv_v(pv::POINTS + root * 0x10), m.pv_v(pv::VELS + root * 0x10));
        m.set_pv_v(branch_at(k, 0), at);
        m.set_pv_v(branch_vel(k, 0), vel);
        for j in 1..4 {
            let jitter = rng.rand_vec(0.1, 0.1);
            let drift = rng.rand_vec(0.0, DRIFT * 0.016_666_668);
            m.set_pv_v(branch_vel(k, j), drift);
            let next = add(add(m.pv_v(branch_at(k, j - 1)), dir), jitter);
            m.set_pv_v(branch_at(k, j), next);
        }
    }
    if crate::hero::physics::dist2(m.pos, cam) < 4.0 { return; }
    let c = usize::try_from(m.pv_i(pv::CUBOID)).ok().and_then(|c| ctx.level.cuboids.get(c)).map_or([0.0; 4], |c| c.extra);
    let yaw = c[1] + rng.randf_signed(0.0, 30.0) * 0.017_453_294;
    let roll = rng.rand_angle();
    m.set_pv_v(pv::ANGLES, [roll, yaw, c[2]]);
    let (rows, pos) = (euler_rows(m.pv_v(pv::ANGLES)), m.pos);
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    for i in (0..32).step_by(8) {
        let at = to_world(&rows, pos, m.pv_v(pv::POINTS + i * 0x10));
        let d = m.pv_v(pv::VELS + i * 0x10);
        let def = if rng.randi(2) != 0 { SPARK_DEF.0 } else { SPARK_DEF.1 };
        let out = rng.randi(10) + 0x14;
        let v4 = [d[0], d[1], d[2], SPARK_SIZE];
        crate::particles::staged(sys, rng, at, v4, v4, SPARK_C.0, SPARK_C.1, (i >> 1) as i16, 0xf, out as i16, def);
    }
    for k in 0..2 {
        let at = to_world(&rows, pos, m.pv_v(branch_at(k, 3)));
        let d = m.pv_v(branch_vel(k, 3));
        rng.randi(2);
        let out = rng.randi(10) + 0x14;
        let v4 = [d[0], d[1], d[2], SPARK_SIZE];
        let root = m.pv_b(pv::ROOTS + k);
        crate::particles::staged(sys, rng, at, v4, v4, SPARK_C.0, SPARK_C.1, (root >> 1) as i16, 0xf, out as i16, SPARK_DEF.1);
    }
}

/// `0x41bfc0`: a tick of the bolt. While it lives: one tick off, points 2..24 and the branches' points 1..3 moved by
/// their drift, each branch's root back on its point. Its colour: past the fade's turn (15) from 0x20401040 at its
/// birth to 0xff803040, then down to 0x20400000 at 0 (`0x2fa6c8`); 0 when gone.
fn step(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    let life = m.pv_s(pv::LIFE);
    if life != 0 {
        m.set_pv_s(pv::LIFE, life - 1);
        for i in 2..25 {
            let o = pv::POINTS + i * 0x10;
            m.set_pv_v(o, add(m.pv_v(o), m.pv_v(pv::VELS + i * 0x10)));
        }
        for k in 0..4 {
            for j in 1..4 {
                m.set_pv_v(branch_at(k, j), add(m.pv_v(branch_at(k, j)), m.pv_v(branch_vel(k, j))));
            }
        }
        for k in 0..4 {
            let root = m.pv_b(pv::ROOTS + k) as usize;
            m.set_pv_v(branch_at(k, 0), m.pv_v(pv::POINTS + root * 0x10));
        }
    }
    let life = m.pv_s(pv::LIFE);
    let rgba = if FADE_AT < life {
        let t = (life - FADE_AT) as f32 / (m.pv_i(pv::LIFE_ALL) - FADE_AT as i32) as f32;
        crate::hud::lerp_rgba(t, RGBA_FULL, RGBA_NEW)
    } else if GONE_AT < life {
        let t = (life - GONE_AT) as f32 / (FADE_AT - GONE_AT) as f32;
        crate::hud::lerp_rgba(t, RGBA_LAST, RGBA_FULL)
    } else {
        0
    };
    m.set_pv_i(pv::RGBA, rgba as i32);
}

/// 0x0ee7 (`0x41b828`). Out of view (but in state 0: its sphere of `+0x624` against the view at 52, `0x2fa748`) or
/// switched off (`+0x628` = 0) nothing.
/// * **0**: a bolt ([`build`]); the countdown its first wait `+0xc`; 2; its radius 2.5; `+8` / `+0xa` 1.
/// * **1**: with a bolt to spare (`$gp+0x25c8`, 0 on this level) a new bolt once this one is gone.
/// * **2**: the countdown out: 3, the countdown the run `+4`, a new bolt.
/// * **3**: a new bolt each time the last is gone; the countdown out: 2 if `+6` (the pause), the countdown 5.
///
/// Then while its bolt lives: its tick ([`step`], unless `$gp+0x25e0`, 0 here) and, past its last tick (`$gp+0x25d8`,
/// 0) and not hidden (`$gp+0x25e4`, 0), its draw (`0x2f2400(0x41c190)`, [`draw_0ee7`]).
pub fn update_0ee7(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    if m.state != 0 {
        let s = [m.pos[0], m.pos[1], m.pos[2], m.pv_f(pv::RADIUS)];
        if ctx.view.as_ref().is_some_and(|v| v.culled(52.0, s)) { return; }
    }
    if m.pv_i(pv::ON) == 0 { return; }
    match w.mobys[id].state {
        0 => {
            build(w, id, ctx);
            let m = &mut w.mobys[id];
            m.set_pv_s(pv::COUNT, m.pv_s(pv::FIRST));
            m.state = 2;
            m.set_pv_f(pv::RADIUS, 2.5);
            m.set_pv_s(0xa, 1);
            m.set_pv_s(8, 1);
        }
        2 => {
            let m = &mut w.mobys[id];
            let mut c = m.pv_s(pv::COUNT) as i32;
            let out = crate::moby::countdown(&mut c);
            m.set_pv_s(pv::COUNT, c as i16);
            if out {
                m.state = 3;
                m.set_pv_s(pv::COUNT, m.pv_s(pv::RUN));
                build(w, id, ctx);
            }
        }
        3 => {
            let m = &mut w.mobys[id];
            let mut c = m.pv_s(pv::COUNT) as i32;
            let out = crate::moby::countdown(&mut c);
            m.set_pv_s(pv::COUNT, c as i16);
            if out {
                if m.pv_s(pv::PAUSES) != 0 { m.state = 2; }
                m.set_pv_s(pv::COUNT, 5);
            } else if m.pv_s(pv::LIFE) == 0 {
                build(w, id, ctx);
            }
        }
        _ => {}
    }
    if w.mobys[id].pv_s(pv::LIFE) == 0 { return; }
    step(w, id);
    if GONE_AT < w.mobys[id].pv_s(pv::LIFE) { w.register_draw(id); }
}

/// `0x41c190`: the bolt's quads (FX 0xe, additive), in its frame (`+0x650` at its place): each of its 31 segments a
/// quad 0.2 across facing the camera (the side `0.1·unit((p[i+1] − camera) × (p[i] − p[i+2]))`, none past segment 29;
/// a segment's start takes the last one's side, none at the first), then each branch from a point below 31 its three
/// segments the same way (the first's start its own side); the colour `+0x14` at every corner, s a quarter a segment
/// by fours.
pub fn draw_0ee7(w: &MobyWorld, id: usize, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let (rows, pos) = frame(w, id);
    let m = &w.mobys[id];
    let rel = sub(cam, pos);
    let cam_l = [dot(rel, rows[0]), dot(rel, rows[1]), dot(rel, rows[2])];
    let rgba = m.pv_i(pv::RGBA) as u32;
    let side = |a: V3, b: V3, c: V3| set_len(cross(sub(b, cam_l), sub(a, c)), 0.1);
    let mut quad = |a: V3, b: V3, sa: V3, sb: V3, i: usize| {
        let f = (i % 4) as f32 * 0.25;
        let corners = [sub(a, sa), add(a, sa), sub(b, sb), add(b, sb)].map(|p| to_world(&rows, pos, p));
        let st = std::array::from_fn(|k| [S0[k] + f, T0[k]]);
        out.push(crate::fx::FxQuad { fx: FX, additive: true, corners, st, rgba: [rgba; 4] });
    };
    let pt = |i: usize| m.pv_v(pv::POINTS + i * 0x10);
    let mut last = [0.0; 3];
    for i in 0..31 {
        let d = if i < 30 { side(pt(i), pt(i + 1), pt(i + 2)) } else { [0.0; 3] };
        quad(pt(i), pt(i + 1), last, d, i);
        last = d;
    }
    for k in 0..4 {
        if 0x1f <= m.pv_b(pv::ROOTS + k) { continue; }
        let bp = |j: usize| m.pv_v(branch_at(k, j));
        let mut last = [0.0; 3];
        for j in 0..3 {
            let d = if j < 2 { side(bp(j), bp(j + 1), bp(j + 2)) } else { [0.0; 3] };
            if j == 0 { last = d; }
            quad(bp(j), bp(j + 1), last, d, j);
            last = d;
        }
    }
}

/// 0x1150's pvars: `[0]` its group of bolts, `[4]` its effect moby, `[8]` / `[0xc]` the waits before the bolts / the
/// effect go on, `[0x10]` its cuboid, `[0x14]` / `[0x18]` the bolts / the effect on, `[0x1c]` / `[0x20]` the waits
/// before the effect / the bolts go off, `[0x24]` / `[0x28]` the countdowns (bolts, effect), `[0x2c]` its looped sound.
mod tp {
    pub const GROUP: usize = 0;
    pub const EFFECT: usize = 4;
    pub const BOLTS_ON: usize = 8;
    pub const EFFECT_ON: usize = 0xc;
    pub const CUBOID: usize = 0x10;
    pub const BOLTS: usize = 0x14;
    pub const LIT: usize = 0x18;
    pub const EFFECT_OFF: usize = 0x1c;
    pub const BOLTS_OFF: usize = 0x20;
    pub const COUNT_BOLTS: usize = 0x24;
    pub const COUNT_EFFECT: usize = 0x28;
    pub const LOOP: usize = 0x2c;
}

/// The 0x0ee7 bolts of the group `g` switched (their `+0x628`).
fn switch_bolts(w: &mut MobyWorld, g: i32, on: bool) {
    let Some(members) = usize::try_from(g).ok().and_then(|g| w.groups.get(g)).cloned() else { return };
    for i in members {
        if let Some(b) = w.mobys.get_mut(i).filter(|b| b.o_class == 0x0ee7) { b.set_pv_i(pv::ON, on as i32); }
    }
}

/// Its loop stopped, −1 (`0x369e10` while it plays for it).
fn loop_off(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let h = w.mobys[id].pv_i(tp::LOOP);
    if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
    w.mobys[id].set_pv_i(tp::LOOP, -1);
}

/// 0x1150 (`0x424190`).
/// * **0**: its loop −1; without its group's list (`0x1fb3a0`) or its effect moby (−1): deleted ("Moby 4432 Needs a
///   Transponder Group" / "… a Powerline Effect"); else both off, the countdowns `[0xc]` / `[8]`, 1.
/// * **1**: Ratchet in its cuboid (`0x32b7c0`) → 3; its loop stopped.
/// * **3**: (its once-flag set) its loop, the level's sound 0x13 after the split (`0x36a308(0x13, 4, m)`), kept
///   playing; the bolts' countdown out: its group's bolts on (`+0x628` 1); the effect's out: the effect moby's
///   `+0x3c` 1. Both on: the countdowns `[0x20]` / `[0x1c]`, 4.
/// * **4**: the bolts' countdown out: the bolts off; the effect's: its `+0x3c` 0. Both off: the countdowns `[8]` /
///   `[0xc]`, 1.
///
/// Not ported: the group walk's end at the level's group count `$gp+0x38c8` (the port's groups end with the level's).
pub fn update_1150(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let tick = |w: &mut MobyWorld, off: usize| -> bool {
        let m = &mut w.mobys[id];
        let mut c = m.pv_i(off);
        let out = crate::moby::countdown(&mut c);
        m.set_pv_i(off, c);
        out
    };
    let set_effect = |w: &mut MobyWorld, v: i32| {
        let e = w.mobys[id].pv_i(tp::EFFECT);
        if let Some(e) = usize::try_from(e).ok().and_then(|e| w.mobys.get_mut(e)) { e.set_pv_i(0x3c, v); }
    };
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.set_pv_i(tp::LOOP, -1);
            let g = m.pv_i(tp::GROUP);
            let has_group = usize::try_from(g).ok().and_then(|g| w.groups.get(g)).is_some_and(|l| !l.is_empty());
            let m = &mut w.mobys[id];
            if !has_group || m.pv_i(tp::EFFECT) == -1 {
                w.delete(id);
                return;
            }
            m.set_pv_i(tp::LIT, 0);
            m.set_pv_i(tp::BOLTS, 0);
            m.set_pv_i(tp::COUNT_EFFECT, m.pv_i(tp::EFFECT_ON));
            m.set_pv_i(tp::COUNT_BOLTS, m.pv_i(tp::BOLTS_ON));
            m.set_state(1, -1);
        }
        1 => {
            if ctx.level.in_cuboid(w.mobys[id].pv_i(tp::CUBOID), ctx.hero.pos) { w.mobys[id].set_state(3, -1); }
            loop_off(w, id, ctx);
        }
        3 => {
            w.mobys[id].first();
            let h = w.mobys[id].pv_i(tp::LOOP);
            if h == -1 || !ctx.host.sound_playing(h) {
                let h = ctx.host.play_level_sound_after(Some(&w.mobys[id]), 0x13, 4);
                w.mobys[id].set_pv_i(tp::LOOP, h);
            }
            if tick(w, tp::COUNT_BOLTS) && w.mobys[id].pv_i(tp::BOLTS) == 0 {
                switch_bolts(w, w.mobys[id].pv_i(tp::GROUP), true);
                w.mobys[id].set_pv_i(tp::BOLTS, 1);
            }
            if tick(w, tp::COUNT_EFFECT) && w.mobys[id].pv_i(tp::LIT) == 0 {
                set_effect(w, 1);
                w.mobys[id].set_pv_i(tp::LIT, 1);
            }
            let m = &mut w.mobys[id];
            if m.pv_i(tp::LIT) != 0 && m.pv_i(tp::BOLTS) != 0 {
                m.set_pv_i(tp::COUNT_EFFECT, m.pv_i(tp::EFFECT_OFF));
                m.set_pv_i(tp::COUNT_BOLTS, m.pv_i(tp::BOLTS_OFF));
                m.set_state(4, -1);
            }
        }
        4 => {
            if tick(w, tp::COUNT_BOLTS) && w.mobys[id].pv_i(tp::BOLTS) != 0 {
                switch_bolts(w, w.mobys[id].pv_i(tp::GROUP), false);
                w.mobys[id].set_pv_i(tp::BOLTS, 0);
            }
            if tick(w, tp::COUNT_EFFECT) && w.mobys[id].pv_i(tp::LIT) != 0 {
                set_effect(w, 0);
                w.mobys[id].set_pv_i(tp::LIT, 0);
            }
            let m = &mut w.mobys[id];
            if m.pv_i(tp::LIT) == 0 && m.pv_i(tp::BOLTS) == 0 {
                m.set_pv_i(tp::COUNT_BOLTS, m.pv_i(tp::BOLTS_ON));
                m.set_pv_i(tp::COUNT_EFFECT, m.pv_i(tp::EFFECT_ON));
                m.set_state(1, -1);
            }
        }
        _ => {}
    }
}
