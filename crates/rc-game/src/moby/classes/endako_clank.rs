//! Endako (level 3)'s Clank section: Clank's rescue set piece and Clank's body (L03 addresses), on the body switch
//! (crate::hero::body).
//!
//! * **0x0a08** ([`update_0a08`], `0x3d2fd0`, one placed): the rescue. Ratchet in its cuboid `[1]` plays the scene 0x31
//!   (its end `0x3d35b8`: Ratchet put at the cuboid `[6]`, the camera record `[7]` on again); the level hooks it, and it
//!   plays 0x32 (its end `0x3d3620`: Ratchet becomes Clank on the moby `[0]`); Clank standing on it plays 0x33 and ends
//!   it (`0x3d3700`: back to Ratchet, Clank on his back, the mission done).
//! * **0x0057** ([`update_0057`], `0x394728`): Clank's body. Ratchet within 1.7 of it becomes Clank on it
//!   (`0x2b8508(1, 0x3f, it)`). The rescue keeps the placed one hidden until its scene 0x32 and makes a second, out of
//!   the moby loop (mode 2), posed in its sequence 0x13 at (200.15, 327.8, 134.87) (`$gp+0x1620`, angles y 0.25,
//!   z −2): Clank held where the scenes show him.
//!
//! The rescue's pvars (`+0x68`, words): the body moby `[0]`, the cuboid `[1]`, the groups `[2]` (hooked at the end)
//! and `[3]` (hooked meanwhile), the cuboid `[6]` and camera record `[7]` of the first scene's end, the music tracks
//! `[8]` (Clank's) and `[9]` (after), the camera record `[0xa]`, the held Clank `[0xb]`, the glow's phase `[0xc]`.
//! The flags (`0x1395b8 + k`, crate::game::GameState::flags): `0x1395be` the scene 0x31 played, `0x1395bf` 0x32,
//! `0x1395c0` the rescue over; `0x13965a` (the grind help's zone reached, crate::moby::classes::endako_b).
//!
//! Not ported: the ship byte (`0x369cc0`), the autosaves (`0x30ea10`), `0x373160` after the scene 0x32 is asked for
//! (`$gp − 0x6024` = 1.2, `0x2e8888(6)` while `0x1b2460` < 1), the items' new marks `0x1a7b3a..0x1a7b3c`; the camera
//! reset of the placings (`0x2bf080`'s last argument, `0x2dade0`), as crate::moby::classes::endako_b's `put_hero`.

use crate::hero::physics::{add_rot, dist, euler_rows, V3};
use crate::moby::{mode, Ctx, MobyWorld};

/// Clank's body class, and the held Clank's sequence.
pub const CLANK_CLASS: i16 = 0x57;
const HELD_SEQ: u8 = 0x13;
/// `$gp+0x1620` (L03 `0x1b0610`): the held Clank's place; `0x1b0624` / `0x1b0628` its angles y / z.
const HELD_AT: V3 = [f32::from_bits(0x4348_2666), f32::from_bits(0x43a3_e666), f32::from_bits(0x4306_deb8)];
const HELD_ROT: [f32; 2] = [0.25, -2.0];
/// `0x3d3620`: Ratchet's yaw off the body's by −π/3 (`0xbf860a93`) for the placing, then back.
const TURN: f32 = f32::from_bits(0x3f86_0a93);
/// `0x3d2fd0` state 2: the glow's phase step (`0x3ddc6c69`), its colours and its ease 0.15 (`0x3e19999a`).
const GLOW_STEP: f32 = f32::from_bits(0x3ddc_6c69);
const GLOW: [u32; 2] = [0x808c_8c8c, 0x8028_2828];
const GLOW_EASE: f32 = f32::from_bits(0x3e19_999a);
/// `0x3d3700`: the glow word at the end.
const GLOW_END: u32 = 0x8046_8cdc;

/// `0x1395b8 + k`: the scene 0x31 played, 0x32 played, the rescue over; the grind help's zone reached.
const F_SCENE_1: usize = 0x06;
const F_SCENE_2: usize = 0x07;
const F_DONE: usize = 0x08;
const F_GRIND: usize = 0xa2;

/// The callbacks the scenes run at their end (`0x373150(f, moby)`), kept in `+0xbc` until the chain is over.
const CB_31: u8 = 1;
const CB_32: u8 = 2;
const CB_33: u8 = 3;

fn flag(ctx: &Ctx, k: usize) -> u8 { ctx.game.flags.get(k).copied().unwrap_or(0) }
fn set_flag(ctx: &mut Ctx, k: usize) { if let Some(b) = ctx.game.flags.get_mut(k) { *b = 1; } }
fn moby_at(w: &MobyWorld, i: i32) -> Option<usize> { usize::try_from(i).ok().filter(|&i| i < w.mobys.len() && w.mobys[i].state < 0x80) }
/// The held Clank `[0xb]` (a pointer in the original: 0 none).
fn held(w: &MobyWorld, id: usize) -> Option<usize> { moby_at(w, w.mobys[id].pv_i(0x2c)).filter(|&i| i != 0) }
fn in_scene(ctx: &Ctx) -> bool { matches!(ctx.game.scenes.mode, 2 | 6) }

/// `0x32bf18`-style shows: hidden and out of the targets with no collision (`mode | 0x41`, `& ~0x1000`, `+0x98` 0),
/// or shown as a target with its class's collision.
fn show(w: &mut MobyWorld, i: usize, on: bool) {
    let solid = w.class(w.mobys[i].o_class).is_some_and(|c| c.has_collision);
    let m = &mut w.mobys[i];
    if on {
        m.mode = (m.mode & !0x41) | mode::TARGET;
        m.has_collision = solid;
    } else {
        m.mode = (m.mode | 0x41) & !mode::TARGET;
        m.has_collision = false;
    }
    w.changed = true;
}

/// `0x2bf080(pos, rot, state, 1)`: Ratchet placed, in `state`.
fn place_hero(ctx: &mut Ctx, pos: V3, rot: V3, state: i32) {
    ctx.hero.teleport(pos, rot);
    ctx.hero.request_state(state, true);
}

/// `0x3254e0(cuboid, cam)` (`0x325520`): Ratchet placed at the cuboid in the body's idle (0, 0x3f; 0x56 for body 2),
/// and the camera record `cam` on again.
fn place_at_cuboid(ctx: &mut Ctx, cuboid: i32, cam: i32) {
    let Some((p, r)) = ctx.level.cuboid_place(cuboid) else { return };
    let state = match ctx.hero.mode { 1 => 0x3f, 2 => 0x56, _ => 0 };
    place_hero(ctx, p, r, state);
    if cam != -1 { ctx.game.cameras.reset_placed(cam); }
}

/// The items Clank brings back (`0x1a7b02..0x1a7b04`: Clank and his packs) and Clank on Ratchet's back (`0x18c31c` 0).
fn clank_back(ctx: &mut Ctx) {
    for k in 2..=4 {
        if let Some(b) = ctx.game.items.get_mut(k) { *b = 1; }
    }
    ctx.game.clank = true;
    ctx.hero.body.clank_away = false;
}

/// 0x0057 (`0x394728`): Clank's body — Ratchet within 1.7 of it becomes Clank on it.
pub fn update_0057(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if ctx.hero.mode == 0 && dist(ctx.hero.pos, w.mobys[id].pos) < 1.7 { ctx.hero.switch_body(1, 0x3f, id); }
}

/// 0x0a08 (`0x3d2fd0`): Clank's rescue (module docs).
///
/// * **0** (updated at any distance): the mission done — Ratchet put at it (before the grind help's zone, `0x13965a`),
///   as Clank first back to Ratchet (the music `[9]`, the body deleted, the mission marked, put at it again); Clank
///   back on his back, the group `[2]` hooked and `[3]` let go, and it goes. Else as Clank already: 2 (the music `[8]`);
///   as Ratchet: 1, the body hidden, the held Clank made (from the cuboid `[1]`, draw distance 0x30, out of the
///   loop, sequence 0x13, no collision, at `$gp+0x1620`), Ratchet put at `[6]` once the first scene played. The
///   group `[3]` hooked, Clank away from Ratchet's back.
/// * **1**: Ratchet in `[1]` plays 0x31 once. The held Clank hidden in a scene, shown (a target with its collision)
///   after three ticks out of one. Not hooked: as Clank (a load as Clank) the music `[8]`, the body shown, the held
///   Clank hidden, 2. Hooked: the held Clank hidden, the scene 0x32 asked for (played already: its end at once), 2.
/// * **2**: Clank not standing on it: its glow pulses (phase `[0xc]` + 0.1076, between 0x808c8c8c and 0x80282828,
///   eased by 0.15). Standing on it: the group `[2]` hooked, `[3]` let go, the scene 0x33 asked for: 3.
pub fn update_0a08(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    // The scenes' ends (the game's callback `0x1a8fd0`, here when the chain is over).
    let cb = w.mobys[id].timer;
    if cb != 0 && !ctx.game.scenes.queue.busy && !ctx.game.scenes.in_scene() {
        w.mobys[id].timer = 0;
        match cb {
            CB_31 => end_31(w, id, ctx),
            CB_32 => end_32(w, id, ctx),
            _ => end_33(w, id, ctx),
        }
    }
    let pv = |w: &MobyWorld, k: usize| w.mobys[id].pv_i(4 * k);
    match w.mobys[id].state {
        0 => start(w, id, ctx),
        1 => {
            if flag(ctx, F_SCENE_1) == 0 && ctx.level.in_cuboid(pv(w, 1), ctx.hero.pos) && ctx.game.scenes.queue.push(0x31) {
                ctx.game.scenes.start_queue(ctx.hero.hp);
                w.mobys[id].timer = CB_31;
                set_flag(ctx, F_SCENE_1);
            }
            let held = held(w, id);
            if let Some(h) = held {
                if in_scene(ctx) {
                    show(w, h, false);
                } else {
                    let t = w.mobys[h].state_ticks;
                    w.mobys[h].state_ticks = t.wrapping_add(1);
                    if 3 < t { show(w, h, true); }
                }
            }
            if w.mobys[id].once & 4 == 0 {
                if ctx.hero.mode != 1 { return; }
                ctx.host.music_request(pv(w, 8), -1);
                if let Some(b) = moby_at(w, pv(w, 0)) { show(w, b, true); }
                if let Some(h) = held { show(w, h, false); }
                w.mobys[id].set_state(2, -1);
                return;
            }
            if let Some(h) = held { show(w, h, false); }
            if flag(ctx, F_SCENE_2) == 0 {
                if ctx.game.scenes.queue.push(0x32) {
                    ctx.game.scenes.start_queue(ctx.hero.hp);
                    w.mobys[id].timer = CB_32;
                }
            } else {
                end_32(w, id, ctx);
            }
            w.mobys[id].set_state(2, -1);
        }
        2 => {
            if ctx.hero.ground_moby != Some(id) || ctx.hero.grounded_ticks == 0 {
                let m = &mut w.mobys[id];
                let a = add_rot(m.pv_f(0x30), GLOW_STEP);
                m.set_pv_f(0x30, a);
                let c = crate::hud::lerp_rgba(a.sin() * 0.5 + 0.5, GLOW[0], GLOW[1]);
                m.glow = crate::hud::lerp_rgba(GLOW_EASE, m.glow, c);
                return;
            }
            super::maktar::hook_group(w, pv(w, 2), true);
            super::maktar::hook_group(w, pv(w, 3), false);
            if !ctx.game.scenes.queue.push(0x33) { return; }
            ctx.game.scenes.start_queue(ctx.hero.hp);
            w.mobys[id].timer = CB_33;
            w.mobys[id].set_state(3, -1);
        }
        _ => {}
    }
}

/// `0x3d2fd0` state 0 (the update's docs).
fn start(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let pv = |w: &MobyWorld, k: usize| w.mobys[id].pv_i(4 * k);
    w.mobys[id].update_dist = 0xff;
    let mission = w.mobys[id].mission;
    let done = mission != 0xff && ctx.game.missions.get(ctx.game.level.max(0) as usize * 0x10 + mission as usize).is_some_and(|b| b & 0x80 != 0);
    let (pos, rot) = (w.mobys[id].pos, w.mobys[id].rot);
    if done {
        if flag(ctx, F_GRIND) == 0 { place_hero(ctx, pos, rot, 0); }
        if ctx.hero.mode == 1 {
            ctx.host.music_request(pv(w, 9), -1);
            ctx.hero.body_back(in_scene(ctx));
            if let Some(b) = moby_at(w, pv(w, 0)) { w.delete(b); }
            ctx.game.mission_done(mission);
            place_hero(ctx, pos, rot, 0);
        }
        clank_back(ctx);
        super::maktar::hook_group(w, pv(w, 2), true);
        super::maktar::hook_group(w, pv(w, 3), false);
        w.delete(id);
        return;
    }
    let group = pv(w, 3);
    if ctx.hero.mode == 1 {
        w.mobys[id].set_state(2, -1);
        ctx.host.music_request(pv(w, 8), -1);
    } else {
        w.mobys[id].set_state(1, -1);
        if let Some(b) = moby_at(w, pv(w, 0)) { show(w, b, false); }
        // `0x312738(0x57)` and `0x32be48(it, self)`: its parent, place, angles and light; updated at any distance,
        // drawn; then at the cuboid `[1]`, draw distance 0x30, out of the loop, sequence 0x13, no collision, and at
        // its held place.
        let (light, ambient) = (w.mobys[id].light, w.mobys[id].ambient);
        let c = w.create(CLANK_CLASS);
        let at = ctx.level.cuboid_place(pv(w, 1));
        let m = &mut w.mobys[c];
        (m.parent, m.light, m.ambient, m.update_dist, m.drawn) = (Some(id), light, ambient, 0xff, 1);
        m.mode &= !7;
        if let Some((_, r)) = at { m.rot = r; }
        m.draw_dist = 0x30;
        m.mode |= mode::NO_UPDATE;
        m.has_collision = false;
        m.pos = HELD_AT;
        (m.rot[1], m.rot[2]) = (HELD_ROT[0], HELD_ROT[1]);
        m.rows = euler_rows(m.rot);
        ctx.host.set_anim(&w.mobys[c], HELD_SEQ, 0, 0);
        w.mobys[id].set_pv_i(0x2c, c as i32);
        w.changed = true;
        if flag(ctx, F_SCENE_1) != 0 { place_at_cuboid(ctx, pv(w, 6), pv(w, 7)); }
    }
    super::maktar::hook_group(w, group, true);
    ctx.hero.body.clank_away = true;
}

/// `0x3d35b8`: the scene 0x31 over — Ratchet at `[6]` (camera `[7]`), the held Clank hidden and its ticks from 0.
fn end_31(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (z, cam) = (w.mobys[id].pv_i(0x18), w.mobys[id].pv_i(0x1c));
    place_at_cuboid(ctx, z, cam);
    if let Some(h) = held(w, id) {
        show(w, h, false);
        w.mobys[h].state_ticks = 0;
    }
}

/// `0x3d3620`: the scene 0x32 over — the body shown (a target, its collision), Ratchet put at it turned −π/3, Clank
/// on it (`0x2b8508(1, 0x3f, it)`) turned back, the camera record `[0xa]` on again, the music `[8]`, `0x1395bf`.
fn end_32(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let pv = |w: &MobyWorld, k: usize| w.mobys[id].pv_i(4 * k);
    if let Some(b) = moby_at(w, pv(w, 0)) {
        show(w, b, true);
        let (pos, mut rot) = (w.mobys[b].pos, w.mobys[b].rot);
        rot[2] = add_rot(rot[2], -TURN);
        place_hero(ctx, pos, rot, 0);
        ctx.hero.switch_body(1, 0x3f, b);
        ctx.hero.rot[2] = add_rot(ctx.hero.rot[2], TURN);
    }
    ctx.game.cameras.reset_placed(pv(w, 0xa));
    ctx.host.music_request(pv(w, 8), -1);
    set_flag(ctx, F_SCENE_2);
}

/// `0x3d3700`: the scene 0x33 over — back to Ratchet, the mission done, `0x1395c0`, Clank back on his back, Ratchet
/// put at it, the held Clank and the body deleted, its glow 0x80468cdc, the music `[9]`, a checkpoint where he is.
fn end_33(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let pv = |w: &MobyWorld, k: usize| w.mobys[id].pv_i(4 * k);
    ctx.hero.body_back(in_scene(ctx));
    ctx.game.mission_done(w.mobys[id].mission);
    set_flag(ctx, F_DONE);
    clank_back(ctx);
    let (pos, rot) = (w.mobys[id].pos, w.mobys[id].rot);
    place_hero(ctx, pos, rot, 0);
    if let Some(h) = held(w, id) { w.delete(h); }
    if let Some(b) = moby_at(w, pv(w, 0)) { w.delete(b); }
    w.mobys[id].glow = GLOW_END;
    ctx.host.music_request(pv(w, 9), -1);
    let (hp, hr) = (ctx.hero.pos, ctx.hero.rot);
    ctx.game.save_checkpoint(hp, hr);
}
