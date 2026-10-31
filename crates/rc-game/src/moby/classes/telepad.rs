//! The teleporter pad, **0x0c60** (L02 `0x411b98`; the same unit on levels 2, 6, 7, 13, 16, 18 and 20, ported from
//! L02's copy): a pad with three arms (class 0x0c61) that sends Ratchet to its partner pad, or into another chunk of
//! the level. On Maktar it stands at the resort (about (500, 498, 110)) with no partner: the pad the arena sends
//! Ratchet back to, which powers down as he arrives.
//!
//! Its pvars: the partner `+0` (a moby index; −1: none, it never sends), the other partner `+4` used once the mission
//! `+0xc` is done, the mission `+8` it waits for (−1: none), the prompt `+0x10` ([`prompt_text`]), the arms
//! `+0x14..+0x1c`, the arms' reach `+0x20` and tilt `+0x24` (their speeds `+0x28` / `+0x2c`), the beam `+0x34`, the
//! chunk `+0x38` it sends into (−1: none; −1 on Maktar) and `+0x40`, the white `+0x3c`.
//!
//! * **0** its arms made (`0x4125a0`: three 0x0c61 at its place, draw distance 0x40, its mode, a third of a turn
//!   apart) and placed ([`place_arms`], `0x412448`), 1.
//! * **1** with its mission not done it is hidden with its arms, no collision and not a target; else shown with
//!   them, a target with its class's collision. With a partner and Ratchet on it (his ground moby, grounded) its
//!   prompt (owner 0xb, `0x412be8`); △ with the prompt his: its sound 0, he walks to its middle facing its yaw
//!   (`0x2d7940`), a 1.5 sphere pushes things off it (20, `0x3214d8`), 2.
//! * **2** the arms reach out and tilt up (each eased to 1 in turn, `0x323170`: 12/3600, 12/3600, at most 32/60;
//!   `$gp+0x2aa8..+0x2ab0`); then, his prompt gone or the reach under 0.25: Ratchet at the middle (his state 100)
//!   stands still (0x6f) with the beam full (1.95), 3; not walking, 4.
//! * **3** the beam (drawn) fades by 1/15 a tick; out: without a chunk Ratchet is put on the partner 0.2 up, facing
//!   its way (`0x2bd368`), the partner powers down (4: its arms out, the beam from 0, its sound 1), and this pad is
//!   back at 1 with its beam full and its arms in; with a chunk, 5.
//! * **4** the beam (drawn) grows back to 1.95 by 1/15, then the arms tilt down and come in (eased to 0), 1.
//! * **5** (into a chunk) the white `+0x3c` from 1 down by 0.02 a tick; past −0.2 the partner arrives (6), the chunk
//!   switch (`0x3082e0(+0x38, +0x40)`, crate::moby::Ctx::switch_chunk: its music track `+0x40`, 0 when negative), Ratchet on the partner 0.2 up standing still (0x6f),
//!   1. **6** (arriving from a chunk) its arms out, the white up by 0.02 to 1.2, then Ratchet idle (`0x2cc330`), its
//!   sound 1, 4.
//!
//! Each tick its arms take its mode's 0x80 bit (hidden with it). The flash: the beam's draw (`0x4126b0`) sets the
//! white fade (`$gp+0x3ff4`) to `(0.2 − beam)·5` (at least 0) while the beam is under 1.15 and the pad has no chunk;
//! it is set here where the draw is asked for, after the beam's step.
//!
//! Not ported: the beam's draw itself (`0x4126b0`: four bands of light over the pad, with Ratchet's shrink
//! `0x18be40..` / `0x18c320` and his pull to its middle `0x189f20`), the white's draw `0x412c78` of the states 5 and
//! 6, the arms' lighting (`+0x38` from Ratchet's moby), the cutscene lock `0x18c0d5` while the beam is out, the
//! prompt texts of the other levels' copies (L02's switch is used).

use crate::hero::physics::{add, add_rot, approach, ease, set_len};
use crate::moby::attach::rows_of;
use crate::moby::{ai, mode, Ctx, MobyWorld};

pub const CLASS: i16 = 0x0c60;
/// The arms (`0x4125a0`).
pub const ARM: i16 = 0x0c61;
/// `$gp+0x2a98`: the arms' drop from its place; `$gp+0x2aa0`: their resting tilt (degrees).
const ARM_DROP: f32 = -0.4;
const ARM_TILT: f32 = -20.0;
/// `$gp+0x2aa8..+0x2ab0`: the arms' ease (a second², a second², a second).
const EASE: (f32, f32, f32) = (12.0, 12.0, 32.0);
/// `0x3ff9999a`: the beam full; `0x3d888889`: its step.
const BEAM_FULL: f32 = 1.95;
const BEAM_STEP: f32 = 0.066_666_67;

mod pv {
    pub const PARTNER: usize = 0;
    pub const PARTNER2: usize = 4;
    pub const MISSION: usize = 8;
    pub const MISSION2: usize = 0xc;
    pub const TEXT: usize = 0x10;
    pub const ARMS: usize = 0x14;
    pub const REACH: usize = 0x20;
    pub const TILT: usize = 0x24;
    pub const REACH_V: usize = 0x28;
    pub const TILT_V: usize = 0x2c;
    pub const BEAM: usize = 0x34;
    pub const CHUNK: usize = 0x38;
    pub const WHITE: usize = 0x3c;
    /// The music track the chunk switch starts.
    pub const TRACK: usize = 0x40;
}

/// `0x412be8`: the prompt by `+0x10` (L02's switch).
fn prompt_text(k: i32) -> u32 {
    match k {
        1 => 0x130b,
        3 => 0x133b,
        4 => 0x1348,
        5 => 0x133d,
        6 => 0x1344,
        7 => 0x1345,
        8 => 0x1346,
        9 => 0x1347,
        _ => 0x12e5,
    }
}

fn arms(w: &MobyWorld, id: usize) -> Vec<usize> {
    (0..3).filter_map(|k| usize::try_from(w.mobys[id].pv_i(pv::ARMS + 4 * k)).ok()).filter(|&a| a != 0 && a < w.mobys.len()).collect()
}

fn mission_done(ctx: &Ctx, mission: i32) -> bool {
    let g = &*ctx.game;
    usize::try_from(mission).ok().and_then(|k| g.missions.get(g.level.max(0) as usize * 0x10 + k)).is_some_and(|b| b & 0x80 != 0)
}

/// The pad and its arms shown (mode 0x41 off, a target, the class's collision) or hidden (no collision, mode 0x41,
/// not a target).
fn show(w: &mut MobyWorld, id: usize, on: bool) {
    let list = arms(w, id);
    for i in std::iter::once(id).chain(list) {
        let solid = w.class(w.mobys[i].o_class).is_some_and(|c| c.has_collision);
        let m = &mut w.mobys[i];
        let was = (m.mode, m.has_collision);
        if on {
            m.mode = (m.mode & !0x41) | mode::TARGET;
            m.has_collision = solid;
        } else {
            m.mode = (m.mode & !mode::TARGET) | 0x41;
            m.has_collision = false;
        }
        if was != (m.mode, m.has_collision) { w.changed = true; }
    }
}

/// `0x412448`: each arm at the pad's place [`ARM_DROP`] lower, out along its facing (tilted [`ARM_TILT`]) by
/// `reach·2.4 − 1.2`, then tilted by `tilt·−60° − 20°`.
fn place_arms(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let m = &w.mobys[id];
    let (pos, out, tilt) = (m.pos, m.pv_f(pv::REACH) * 2.4 - 1.2, m.pv_f(pv::TILT) * -1.047_197_7 - 0.349_065_9);
    for a in arms(w, id) {
        let am = &mut w.mobys[a];
        am.rot[1] = ARM_TILT * 0.017_453_294;
        let rows = rows_of(am.rot);
        am.pos = add([pos[0], pos[1], pos[2] + ARM_DROP], set_len(rows[0], out));
        am.rot[1] = tilt;
        am.drive = true;
        w.rebuild(a, &*ctx.host);
    }
}

/// `0x323170(t, …, &x, &v)` on the pvars `x` / `v`.
fn ease_to(w: &mut MobyWorld, id: usize, t: f32, x: usize, v: usize) {
    let m = &mut w.mobys[id];
    let (mut p, mut s) = (m.pv_f(x), m.pv_f(v));
    ease(t, EASE.0 * 0.000_277_777_78, EASE.1 * 0.000_277_777_78, EASE.2 * 0.016_666_668, &mut p, &mut s);
    m.set_pv_f(x, p);
    m.set_pv_f(v, s);
}

/// The beam's draw asked for (`0x4126b0`): its flash (module docs).
fn beam_flash(w: &MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let beam = m.pv_f(pv::BEAM);
    if beam < 1.15 && m.pv_i(pv::CHUNK) < 0 { ctx.game.fade_white = ((0.2 - beam) * 5.0).max(0.0); }
}

/// 0x0c60 (`0x411b98`): see the module.
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let mut arms_placed = true;
    match w.mobys[id].state {
        0 => {
            w.mobys[id].alpha = 0xff;
            w.mobys[id].state = 1;
            let (pos, rot, mode_bits) = (w.mobys[id].pos, w.mobys[id].rot, w.mobys[id].mode);
            for k in 0..3 {
                let a = w.create(ARM);
                let am = &mut w.mobys[a];
                am.draw_dist = 0x40;
                am.drawn = 1;
                am.mode = mode_bits;
                am.pos = pos;
                am.rot = [rot[0], rot[1], add_rot(rot[2], k as f32 * 2.094_395_4)];
                w.mobys[id].set_pv_i(pv::ARMS + 4 * k, a as i32);
            }
        }
        1 => {
            arms_placed = false;
            let mission = w.mobys[id].pv_i(pv::MISSION);
            if 0 <= mission && !mission_done(ctx, mission) {
                show(w, id, false);
            } else {
                show(w, id, true);
                if w.mobys[id].pv_i(pv::PARTNER) != -1 && ctx.hero.ground_moby == Some(id) && ctx.hero.air_ticks == 0 {
                    ctx.game.prompt.request(0xb, prompt_text(w.mobys[id].pv_i(pv::TEXT)), 1);
                    if ctx.hero.items.pressed & crate::pad::button::TRIANGLE != 0 && ctx.game.prompt.owner == 0xb {
                        ctx.host.play_sound(&w.mobys[id], 0, 0);
                        let m = &mut w.mobys[id];
                        m.state = 2;
                        m.set_pv_i(pv::REACH_V, 0);
                        m.set_pv_i(pv::TILT_V, 0);
                        let (yaw, at) = (m.rot[2], m.pos);
                        ctx.hero.script_walk_to(yaw, at, false);
                        ai::attack_sphere(w, id, ctx, 1.5, 20.0, 1.0, at, 0x10000, 0, 1, 0);
                    }
                }
            }
        }
        2 => {
            show(w, id, true);
            // `0x2d7990`: walking there (0x62..0x63), there (100), else not.
            let walk = match ctx.hero.state { 0x62..=0x63 => 1, 100 => 2, _ => 0 };
            let m = &w.mobys[id];
            if m.pv_f(pv::REACH) < 1.0 {
                ease_to(w, id, 1.0, pv::REACH, pv::REACH_V);
            } else if m.pv_f(pv::TILT) < 1.0 {
                ease_to(w, id, 1.0, pv::TILT, pv::TILT_V);
            } else if ctx.game.prompt.owner != 0xb || m.pv_f(pv::REACH) <= 0.25 {
                if walk == 2 {
                    ctx.hero.request_state(0x6f, true);
                    let m = &mut w.mobys[id];
                    m.state = 3;
                    m.set_pv_f(pv::BEAM, BEAM_FULL);
                } else if walk == 0 {
                    w.mobys[id].state = 4;
                }
            }
        }
        3 => {
            show(w, id, true);
            arms_placed = false;
            if w.mobys[id].pv_f(pv::BEAM) == 0.0 {
                if w.mobys[id].pv_i(pv::CHUNK) < 0 {
                    let m = &w.mobys[id];
                    let mut to = m.pv_i(pv::PARTNER);
                    if m.pv_i(pv::PARTNER2) != -1 && mission_done(ctx, m.pv_i(pv::MISSION2)) { to = m.pv_i(pv::PARTNER2); }
                    if let Some(p) = usize::try_from(to).ok().filter(|&p| p < w.mobys.len()) {
                        let (at, rot) = (w.mobys[p].pos, w.mobys[p].rot);
                        ctx.hero.teleport([at[0], at[1], at[2] + 0.2], rot);
                        ctx.hero.request_state(0, true);
                        let pm = &mut w.mobys[p];
                        pm.state = 4;
                        for o in [pv::BEAM, pv::REACH_V, pv::TILT_V] { pm.set_pv_i(o, 0); }
                        pm.set_pv_f(pv::TILT, 1.0);
                        pm.set_pv_f(pv::REACH, 1.0);
                        ctx.host.play_sound(&w.mobys[p], 1, 0);
                    }
                    let m = &mut w.mobys[id];
                    m.state = 1;
                    for o in [pv::REACH, pv::TILT, pv::REACH_V, pv::TILT_V] { m.set_pv_i(o, 0); }
                    m.set_pv_f(pv::BEAM, BEAM_FULL);
                    place_arms(w, id, ctx);
                } else {
                    w.mobys[id].set_state(5, -1);
                }
            } else {
                let m = &mut w.mobys[id];
                let mut b = m.pv_f(pv::BEAM);
                approach(0.0, BEAM_STEP, &mut b);
                m.set_pv_f(pv::BEAM, b);
                beam_flash(w, id, ctx);
            }
        }
        4 => {
            w.mobys[id].first();
            show(w, id, true);
            let m = &w.mobys[id];
            if m.pv_f(pv::BEAM) < 1.95 {
                let m = &mut w.mobys[id];
                let mut b = m.pv_f(pv::BEAM);
                approach(BEAM_FULL, BEAM_STEP, &mut b);
                m.set_pv_f(pv::BEAM, b);
                beam_flash(w, id, ctx);
            } else if 0.0 < m.pv_f(pv::TILT) {
                ease_to(w, id, 0.0, pv::TILT, pv::REACH_V);
            } else if 0.0 < m.pv_f(pv::REACH) {
                ease_to(w, id, 0.0, pv::REACH, pv::TILT_V);
            } else {
                w.mobys[id].state = 1;
            }
        }
        5 => {
            arms_placed = false;
            let m = &mut w.mobys[id];
            if m.first() { m.set_pv_f(pv::WHITE, 1.0); }
            let white = m.pv_f(pv::WHITE) - 0.02;
            m.set_pv_f(pv::WHITE, white);
            if white <= -0.2 {
                let to = usize::try_from(m.pv_i(pv::PARTNER)).ok().filter(|&p| p < w.mobys.len());
                let chunk = w.mobys[id].pv_i(pv::CHUNK);
                if let Some(p) = to { w.mobys[p].state = 6; }
                let m = &mut w.mobys[id];
                m.state = 1;
                m.set_pv_f(pv::BEAM, BEAM_FULL);
                for o in [pv::REACH, pv::TILT, pv::REACH_V, pv::TILT_V] { m.set_pv_i(o, 0); }
                // `0x3082e0(+0x38, +0x40)`: the music track 0 when negative.
                let track = w.mobys[id].pv_i(pv::TRACK).max(0);
                ctx.switch_chunk(chunk, track);
                if let Some(p) = to {
                    let (at, rot) = (w.mobys[p].pos, w.mobys[p].rot);
                    ctx.hero.teleport([at[0], at[1], at[2] + 0.2], rot);
                    ctx.hero.request_state(0x6f, true);
                }
                place_arms(w, id, ctx);
                w.mobys[id].set_state(1, -1);
            }
        }
        6 => {
            arms_placed = false;
            if w.mobys[id].first() {
                let m = &mut w.mobys[id];
                m.set_pv_f(pv::REACH, 1.0);
                m.set_pv_f(pv::TILT, 1.0);
                place_arms(w, id, ctx);
                w.mobys[id].set_pv_f(pv::WHITE, 0.0);
            }
            let m = &mut w.mobys[id];
            let white = m.pv_f(pv::WHITE) + 0.02;
            m.set_pv_f(pv::WHITE, white);
            if 1.2 <= white {
                ctx.hero.request_state(0, true);
                m.state = 4;
                for o in [pv::REACH_V, pv::TILT_V, pv::BEAM] { m.set_pv_i(o, 0); }
                m.set_pv_f(pv::TILT, 1.0);
                m.set_pv_f(pv::REACH, 1.0);
                ctx.host.play_sound(&w.mobys[id], 1, 0);
            }
        }
        _ => arms_placed = false,
    }
    if arms_placed { place_arms(w, id, ctx); }
    // Each arm takes the pad's 0x80 bit.
    let hidden = w.mobys[id].mode & 0x80;
    for a in arms(w, id) {
        let am = &mut w.mobys[a];
        am.mode = (am.mode & !0x80) | hidden;
    }
}
