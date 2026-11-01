//! Maktar's Tractor Beam puzzle by the resort: a little inspection robot (the critter), **0x09bc** (L02 `0x3cba00`,
//! one placed), that Ratchet drags with the Tractor Beam onto the two security pads, **0x0a61** (`0x3e2fa8`, two
//! placed, the robot in their `[0]`). Ratchet on a pad gets "Access Denied. Only Inspection Personnel Allowed" (0x131d);
//! the robot on it opens its two doors (`[1]` / `[2]`, the sliding doors 0x0aab / 0x0aac of
//! crate::moby::classes::maktar, hooked).
//!
//! **The critter** (`0x3cba00`; pvars: its movement block at `+0x190` (`+0x9c` its address), the drag block at
//! `+0x140` (crate::moby::classes::tractor::drag_at: the hold distance `+0x174` 4, the range `+0x178` 12), its path
//! network's list `+0x390` / fence path `+0x394` / cuboid `+0x398`, the glow's phase `+0x3b0`, the sound's timer
//! `+0x3b4`, the idle's length `+0x3c0`):
//! * Every tick: drawn within 24 of the camera, its shadow's slab probed (range 0x16, `0x321db0`); in sub-state 0,
//!   held by the Tractor Beam (`0x18a3e0`) it is dragged (5, sub 1; `+0x150` its place), else a target (mode 0x1000)
//!   while Ratchet is within `+0x178` across. Drawn, its glow `+0x78` pulses (`sin` of the phase stepping 400°/s, from
//!   0x80c0c0c0 to 0x80606060).
//! * **0**: its scale the class's, the movement block's set-up (`+0x50..+0x88`), the ranges, its path network built
//!   (`0x32f8b0`); updated within 0x80, a target (1).
//! * **1**: idles: on its first tick a sequence of the two idles (`randi(2)`; set, `randi(2)` again, when it is not the
//!   one it plays).
//! * **5**, dragged: its sequence by how it turns (along its facing the drag's velocity under −1/30: 10, over 1/30: 9,
//!   else 7); it turns to face Ratchet (`0x333980`: `0x323628` with its block's limits, those of its walk: accel and
//!   brake 0.05°, at most 6° a tick), is dragged (`0x32af70(0.7, 0.5, 0.1, m, +0x140, +0x394)`) and makes its sound;
//!   hooked (`+0xbe` bit 2) 6; let go, 3 (sub 0).
//! * **6**: sequence 0xd, it faces Ratchet and makes its sound.
//! * Its sound (`0x3cc5e8`): class sound 1 every `rand_range(30, 180)` ticks (`+0x3b4`, at once when dragged).
//!
//! **A pad** (`0x3e2fa8`; pvars `[0]` the critter, `[1]` / `[2]` the doors, `[5]` a moby gone once it is done, `[6]` /
//! `[7]` its phases, `[8]` its fade, `[0xa]` / `[0xb]` its colours, `[0xc]` a count, `[0x10]` its deliveries,
//! `[0x11]` / `[0x12]` its voice lines):
//! * **0**: done before (its uid dead or counted): the doors hooked, `[5]` deleted (5); else 1.
//! * **1**: `[6]` turns 5° a tick, its fade toward 0 (1/30 a tick); drawn, one time in 5 a glint (type 0x3c,
//!   `0x342128`) on its rim 1.15 out, 0.1 up, rising `randf(1, 2)` over its life of `rand_range(60, 180)` ticks, size
//!   `randf(0.2, 0.1)`, coloured `randf(0, 1)` of the way from 0x80008080 to 0x80004080. Ratchet on it: 2 (the first
//!   time on any pad, `$gp+0x1748`, he walks to its middle, facing its way: `0x2d7940`); the critter within 1.5 across
//!   and 0.5 up or down: 3.
//! * **2**: its first tick: class sound 1, colours 0x80000080 / 0x80004080, the message 0x131d (180 ticks), the voice
//!   line `[0x11]` (with the dialogue free); Ratchet off it: 1.
//! * **3**: its first tick: `[0x10]` + 1, colours 0x80008000 / 0x80408000, the message 0x131e, the voice line `[0x12]`;
//!   from its 61st tick the critter let go (if held) and the doors hooked; after 180 ticks 4.
//! * 2 and 3: `[6]` 5°, `[7]` 2° a tick, `[0xc]` + 1 every other frame (to 0xff), the fade toward 1.
//! * **4**: the fade toward 0; out, its uid counted (`0x19b4a8`), 5.
//!
//! Not ported: the critter's walks (states 2, 3, 4: about its path network `0x32f8b0` / `0x32fd40` / `0x330180` /
//! `0x3329b8`, the network's paths `+0x290` and the one nearest Ratchet `+0x3c4` every 32 frames: a missing system,
//! G-CLS-011): it idles where it is instead (its idle's end `+0x3c0` does not start a walk; let go, it idles (1, sub 0)
//! rather than walk off (3)); its fall (7: no code found that enters it). Of the pads: their draw (`0x3e36b8`, rings of
//! effect quads by `[6]` / `[7]` / `[8]` / `[0xa]` / `[0xb]`), the voice line `[0x12]` taken even with the dialogue
//! busy (here only when free), the dialogue's start `0x3354e0` (`0x1a641a` = 3).

use crate::hero::physics::{add, add_rot, approach, atan2, dist2, dot, turn_ease};
use crate::moby::{mode, Ctx, MobyWorld};
use super::tractor;

/// The critter's class and the pads'.
pub const CRITTER: i16 = 0x09bc;
pub const PAD: i16 = 0x0a61;

/// The critter's drag block (crate::moby::classes::tractor::drag_at) and movement block.
const DRAG: usize = 0x140;
const MOVE: usize = 0x190;
/// The movement block's turn limits (`+0x14` accel, `+0x18` brake, `+0x1c` most) and turn speed (`+0xb4`).
const ACCEL: usize = MOVE + 0x14;
const BRAKE: usize = MOVE + 0x18;
const MOST: usize = MOVE + 0x1c;
const TURN_SPEED: usize = MOVE + 0xb4;
/// The sound's timer (a short).
const SOUND: usize = 0x3b4;
/// The glow's phase.
const GLOW: usize = 0x3b0;

/// The moby at pvar word `off` (a moby index), if any.
fn moby_at(w: &MobyWorld, id: usize, off: usize) -> Option<usize> { usize::try_from(w.mobys[id].pv_i(off)).ok().filter(|&i| i < w.mobys.len()) }

/// `0x330490(m, 1)`: hooked.
fn hook(w: &mut MobyWorld, i: Option<usize>) { if let Some(i) = i { w.mobys[i].once |= 4; } }

/// `0x31f6b8(m, s, 0, 10)` unless `s` plays.
fn play(w: &MobyWorld, id: usize, ctx: &mut Ctx, s: u8) {
    if super::props::seq(w, id, ctx) != s { ctx.host.set_anim(&w.mobys[id], s, 0, 10); }
}

/// `0x3cc5e8`: class sound 1 every `rand_range(30, 180)` ticks.
fn chirp(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let mut t = w.mobys[id].pv_s(SOUND) as i32;
    let due = crate::moby::countdown(&mut t);
    w.mobys[id].set_pv_s(SOUND, t as i16);
    if due {
        let n = ctx.game.rng.rand_range(30, 180);
        w.mobys[id].set_pv_s(SOUND, n as i16);
        ctx.host.play_sound(&w.mobys[id], 1, 0);
    }
}

/// `0x333980(yaw, m)`: its yaw eased toward Ratchet's direction by its movement block's limits.
fn face_hero(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let m = &mut w.mobys[id];
    let to = atan2(ctx.hero.pos[0] - m.pos[0], ctx.hero.pos[1] - m.pos[1]);
    let (a, b, c) = (m.pv_f(ACCEL), m.pv_f(BRAKE), m.pv_f(MOST));
    let (mut yaw, mut v) = (m.rot[2], m.pv_f(TURN_SPEED));
    turn_ease(to, a, b, c, &mut yaw, &mut v, 0);
    m.rot[2] = yaw;
    m.set_pv_f(TURN_SPEED, v);
}

/// 0x09bc (`0x3cba00`): see the module.
pub fn update_critter(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    crate::shadow_volumes::probe_near(w, id, ctx, 24.0, 0x16);
    if w.mobys[id].sub_state == 0 {
        if tractor::held(w) == Some(id) {
            let m = &mut w.mobys[id];
            m.set_state(5, 1);
            let p = m.pos;
            m.set_pv_v(DRAG + 0x10, p);
        } else {
            let m = &mut w.mobys[id];
            if dist2(m.pos, ctx.hero.pos) < m.pv_f(DRAG + 0x38) { m.mode |= mode::TARGET } else { m.mode &= !mode::TARGET }
        }
    }
    match w.mobys[id].state {
        0 => {
            let class_scale = w.class(CRITTER).map_or(1.0, |c| c.scale);
            let m = &mut w.mobys[id];
            m.scale = class_scale;
            m.set_pv_s(0x24, 1);
            let fence = m.pv_i(0x394);
            m.set_pv_i(0x8c, fence);
            for (o, v) in [(0x20, 1.0), (0x30, 1.5), (0x84, 0.05), (0x54, 0.2), (0x50, f32::from_bits(0x3f53_3333)), (0x5c, 0.4), (0x58, 0.4)] { m.set_pv_f(o, v); }
            m.set_pv_f(0x60, f32::from_bits(0x3fc9_0fdc));
            m.set_pv_f(0x80, f32::from_bits(0x3f06_0a93));
            m.set_pv_i(0x88, m.pv_i(0x88) | 1);
            m.set_pv_f(DRAG + 0x34, 4.0);
            m.set_pv_f(DRAG + 0x38, 12.0);
            // The turn limits its first walk sets (`0x333ac0(0x3a91a2b4, 4/60, 0x3a64c38a, 0x3dd67752)`; the game walks
            // at once, before it can be taken): accel and brake 0.05°, at most 6° a tick.
            m.set_pv_f(ACCEL, f32::from_bits(0x3a64_c38a));
            m.set_pv_f(BRAKE, f32::from_bits(0x3a64_c38a));
            m.set_pv_f(MOST, f32::from_bits(0x3dd6_7752));
            // The port's: no slide sound playing yet (the drag's handle).
            m.set_pv_i(DRAG + 0x40, -1);
            m.set_state(1, -1);
            m.update_dist = 0x80;
            m.mode |= mode::TARGET;
            w.rebuild(id, &*ctx.host);
        }
        1 => {
            if w.mobys[id].first() {
                let r = ctx.game.rng.randi(2) as u8;
                if super::props::seq(w, id, ctx) != r {
                    let s = ctx.game.rng.randi(2) as u8;
                    ctx.host.set_anim(&w.mobys[id], s, 0, 10);
                }
            }
        }
        5 => {
            if w.mobys[id].first() {
                play(w, id, ctx, 7);
                w.mobys[id].set_pv_s(SOUND, 0);
            }
            let m = &w.mobys[id];
            let d = dot(m.rows[0], m.pv_v(DRAG));
            let s = if d < -1.0 / 30.0 { 10 } else if 1.0 / 30.0 < d { 9 } else { 7 };
            play(w, id, ctx, s);
            face_hero(w, id, ctx);
            let fence = w.mobys[id].pv_i(0x394);
            tractor::drag_at(w, id, ctx, 0.5, fence, DRAG);
            w.rebuild(id, &*ctx.host);
            chirp(w, id, ctx);
            if w.mobys[id].once & 4 != 0 {
                w.mobys[id].set_state(6, -1);
            } else if tractor::held(w) != Some(id) {
                w.mobys[id].set_state(1, 0);
            }
        }
        6 => {
            if w.mobys[id].first() { play(w, id, ctx, 0xd); }
            face_hero(w, id, ctx);
            chirp(w, id, ctx);
        }
        _ => {}
    }
    let m = &mut w.mobys[id];
    if m.drawn != 0 {
        let a = add_rot(m.pv_f(GLOW), 400.0 * f32::from_bits(0x3998_825c));
        m.set_pv_f(GLOW, a);
        m.glow = crate::hud::lerp_rgba(a.sin() * 0.5 + 0.5, 0x80c0_c0c0, 0x8060_6060);
    }
    m.state_ticks = m.state_ticks.wrapping_add(1);
}

/// `$gp+0x1730` / `+0x1734`: the pads' phases' speeds (degrees a second).
const SPIN: f32 = 300.0;
const SPIN2: f32 = 120.0;
/// Degrees a second to radians a tick (`0x3998825c`).
const DEG_TICK: f32 = 0.000_290_888_25;
/// The fade's step (`0x3d088889`, 1/30).
const FADE: f32 = 0.033_333_335;

/// The rim's glint (state 1).
fn glint(w: &MobyWorld, id: usize, ctx: &mut Ctx) {
    let pos = w.mobys[id].pos;
    let hero = ctx.hero.pos;
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let rng = &mut ctx.game.rng;
    let a = rng.rand_angle();
    let mut p = add([a.cos() * 1.15, a.sin() * 1.15, 0.0], pos);
    p[2] += 0.1;
    let size = rng.randf(0.2, 0.1);
    let t = rng.randf(0.0, 1.0);
    let rgba = crate::hud::lerp_rgba(t, 0x8000_8080, 0x8000_4080);
    let rise = rng.randf(1.0, 2.0);
    let life = rng.rand_range(0x3c, 0xb4);
    let rot = rng.randi(0xff) as u8;
    crate::particles::glint(sys, size, p, [0.0, 0.0, rise / life as f32], rgba, life as u16, rot, false, hero);
}

/// The pads' phases (`[6]`, and `[7]` with `[0xc]` in states 2 and 3).
fn turn_phases(w: &mut MobyWorld, id: usize, ctx: &Ctx, both: bool) {
    let m = &mut w.mobys[id];
    m.set_pv_f(0x18, add_rot(m.pv_f(0x18), SPIN * DEG_TICK));
    if !both { return; }
    m.set_pv_f(0x1c, add_rot(m.pv_f(0x1c), SPIN2 * DEG_TICK));
    if ctx.game.frame & 1 != 0 && m.pv_i(0x30) < 0xff { m.set_pv_i(0x30, m.pv_i(0x30) + 1); }
}

fn fade_to(w: &mut MobyWorld, id: usize, t: f32) {
    let m = &mut w.mobys[id];
    let mut f = m.pv_f(0x20);
    approach(t, FADE, &mut f);
    m.set_pv_f(0x20, f);
}

/// 0x0a61 (`0x3e2fa8`): see the module.
pub fn update_pad(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let doors = |w: &mut MobyWorld| {
        let (a, b) = (moby_at(w, id, 4), moby_at(w, id, 8));
        hook(w, a);
        hook(w, b);
    };
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            for o in [0x38, 0x3c, 0x40] { m.set_pv_i(o, 0); }
            let uid = m.uid;
            let done = ctx.game.uid_dead.get(uid.max(0) as usize).is_some_and(|&d| d != 0) || ctx.game.uid_count(uid) != 0;
            if done {
                doors(w);
                if let Some(i) = moby_at(w, id, 0x14) { w.delete(i); }
                w.mobys[id].set_state(5, -1);
            } else {
                w.mobys[id].set_state(1, -1);
            }
        }
        1 => {
            turn_phases(w, id, ctx, false);
            if w.mobys[id].pv_f(0x20) != 0.0 { fade_to(w, id, 0.0); }
            if w.mobys[id].drawn != 0 && ctx.game.rng.randi(5) == 0 { glint(w, id, ctx); }
            if ctx.hero.ground_moby == Some(id) {
                w.mobys[id].set_state(2, -1);
                if !w.maktar.pad_walked {
                    w.maktar.pad_walked = true;
                    let m = &w.mobys[id];
                    ctx.hero.script_walk_to(m.rot[2], m.pos, true);
                }
            } else if let Some(c) = moby_at(w, id, 0) {
                let (p, q) = (w.mobys[id].pos, w.mobys[c].pos);
                if dist2(p, q) < 1.5 && (p[2] - q[2]).abs() < 0.5 { w.mobys[id].set_state(3, -1); }
            }
        }
        2 => {
            if w.mobys[id].first() {
                ctx.host.play_sound(&w.mobys[id], 1, 0);
                let m = &mut w.mobys[id];
                m.set_pv_i(0x28, 0x8000_0080u32 as i32);
                m.set_pv_f(0x20, 0.0);
                m.set_pv_i(0x30, 0);
                m.set_pv_i(0x2c, 0x8000_4080u32 as i32);
                ctx.game.messages.push(crate::msgbox::Request::Text { text: 0x131d, ticks: Some(0xb4) });
                let line = (w.mobys[id].pv_i(0x44) & 0x3fff) + 2000;
                ctx.host.request_speech(line);
            }
            if ctx.hero.ground_moby != Some(id) { w.mobys[id].set_state(1, -1); }
            turn_phases(w, id, ctx, true);
            fade_to(w, id, 1.0);
        }
        3 => {
            if w.mobys[id].first() {
                let m = &mut w.mobys[id];
                m.set_pv_i(0x40, m.pv_i(0x40) + 1);
                m.set_pv_i(0x28, 0x8000_8000u32 as i32);
                m.set_pv_f(0x20, 0.0);
                m.set_pv_i(0x30, 0);
                m.set_pv_i(0x2c, 0x8040_8000u32 as i32);
                ctx.game.messages.push(crate::msgbox::Request::Text { text: 0x131e, ticks: Some(0xb4) });
                let line = (w.mobys[id].pv_i(0x48) & 0x3fff) + 2000;
                ctx.host.request_speech(line);
            }
            let t = w.mobys[id].state_ticks as u16;
            w.mobys[id].state_ticks = t.wrapping_add(1) as i16;
            if t < 0xb5 {
                if 0x3c < t.wrapping_add(1) {
                    if tractor::held(w).is_some_and(|h| Some(h) == moby_at(w, id, 0)) { tractor::release(w); }
                    doors(w);
                }
            } else {
                w.mobys[id].set_state(4, -1);
            }
            turn_phases(w, id, ctx, true);
            fade_to(w, id, 1.0);
        }
        4 => {
            fade_to(w, id, 0.0);
            if w.mobys[id].pv_f(0x20) <= 0.0 {
                let uid = w.mobys[id].uid;
                ctx.game.count_uid(uid);
                w.mobys[id].set_state(5, -1);
            }
        }
        _ => {}
    }
}
