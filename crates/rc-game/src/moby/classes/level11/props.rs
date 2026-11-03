//! Level 11's small objects (L11 addresses).
//!
//! * **0x117a** (`0x4748a0`, 2 placed; no Ghidra function): hidden ([`update_117a`]).
//! * **0x117e** (`0x4748c8`, 1): the Hacker's gate ([`update_117e`]).
//! * **0x123a** (`0x476340`, 1): the updraft's drain ([`update_123a`]).
//! * **0x0c2d** (`0x414290`, 1): the level's help texts ([`update_0c2d`]).
//! * **0x0dfd** (`0x458678`, 23): the drifting sparks ([`update_0dfd`] / [`draw_0dfd`]).
//! * **0x12c7** (`0x399768`, 1): the level's skill points ([`update_12c7`]).
//! * **0x08e2** (`0x3d0750`, 1): the updraft pad ([`update_08e2`]).

use crate::hero::physics::{add, atan2, dist, dist2, sub_rot, V3};
use crate::moby::{countdown, Ctx, MobyWorld};

/// 0x117a (`0x4748a0`): on its first tick state 1 and mode | 0x41 (hidden, no animation).
pub fn update_117a(w: &mut MobyWorld, id: usize, _ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    if m.state == 0 {
        m.state = 1;
        m.mode |= 0x41;
    }
}

/// Ratchet's place for the gate's start (`$gp`-free constants at `0x474960..`): (363.18, 680.68, 155.73), yaw 2.356.
const GATE_START: (V3, V3) = ([f32::from_bits(0x43b5_970a), f32::from_bits(0x442a_2b85), f32::from_bits(0x431b_bae1)], [0.0, 0.0, f32::from_bits(0x4016_cbe5)]);

/// 0x117e (`0x4748c8`, 1 placed): the gate the Hacker's panel (crate::moby::classes::hacker_panel, its hooked moby)
/// opens: its pvars `+0x00..+0x1c` the gate's mobys (the hidden 0x117a pair and the 0x0ee0 beams; −1 none), `+0x20`
/// the hum's slot. **0**: with the Hacker had (`0x1a7b33`), its uid neither gone for good (`0x21fdc4`) nor counted
/// (`0x19b4a8`) and its place in the loaded variant (`0x334048`): Ratchet put at [`GATE_START`] (`0x2d1a58(pos, rot, 0,
/// 1)`, idle); then **1**. Each tick: on level 0xb with Ratchet within 32 across, the level's sound 0x1b past the split
/// looped on Ratchet (`0x3750a8(0x1b, 6)`) while its slot does not play; farther, stopped. Hooked, or its uid gone for
/// good or counted already: the gate's mobys deleted, its uid counted (`0x19b4a8`, at most 7), the hum stopped,
/// deleted.
///
/// Not ported: the ship byte `0x152c26` = 1 (`0x37cdc8`) with Ratchet's place. The port starts the hum's slot at −1
/// (its `sound_playing` tests the slot alone, not its owner).
pub fn update_117e(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let uid = w.mobys[id].uid;
    if w.mobys[id].state == 0 {
        w.mobys[id].set_pv_i(0x20, -1);
        let g = &*ctx.game;
        let had = g.items.get(0x33).is_some_and(|&b| b != 0);
        let gone = g.uid_dead.get(uid.max(0) as usize).is_some_and(|&d| d != 0);
        if had && !gone && g.uid_count(uid) == 0 && ctx.level.in_variant(w.mobys[id].pos, g.variant) {
            ctx.hero.teleport(GATE_START.0, GATE_START.1);
            ctx.hero.request_state(0, true);
        }
        w.mobys[id].state = 1;
    }
    let h = w.mobys[id].pv_i(0x20);
    if dist2(w.mobys[id].pos, ctx.hero.pos) < 32.0 {
        if ctx.game.level == 0xb && !(h != -1 && ctx.host.sound_playing(h)) {
            let k = ctx.host.play_level_sound_after(ctx.hero_moby.and_then(|i| w.mobys.get(i)), 0x1b, 6);
            w.mobys[id].set_pv_i(0x20, k);
        }
    } else {
        if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
        w.mobys[id].set_pv_i(0x20, -1);
    }
    let gone = ctx.game.uid_dead.get(uid.max(0) as usize).is_some_and(|&d| d != 0);
    if w.mobys[id].once & 4 == 0 && !gone && ctx.game.uid_count(uid) == 0 { return; }
    for k in 0..8 {
        let i = w.mobys[id].pv_i(4 * k);
        if let Some(i) = usize::try_from(i).ok().filter(|&i| i < w.mobys.len()) { w.delete(i); }
    }
    ctx.game.count_uid(uid);
    let h = w.mobys[id].pv_i(0x20);
    if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
    w.mobys[id].set_pv_i(0x20, -1);
    w.delete(id);
}

/// 0x123a (`0x476340`, 1 placed): Ratchet floating on an updraft (state 0x82) in its cuboid (`+0x00`) loses his lift
/// ten times as fast: nine more frames off it (`0x300ee0(0x18a3d0)`, a sixtieth each, not below 0).
pub fn update_123a(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if !ctx.level.in_cuboid(w.mobys[id].pv_i(0), ctx.hero.pos) || ctx.hero.state != 0x82 { return; }
    let lift = &mut ctx.hero.gear.lift;
    for _ in 0..9 { *lift = if 1.0 / 60.0 < *lift { *lift - 1.0 / 60.0 } else { 0.0 }; }
}

/// `0x308a80(text, slot, kind, zone, off)`: L11's zone help, as L03's (crate::moby::classes::oozla::zone_help, whose
/// kind 3 here asks while the slot's levels word lacks bit 31).
fn zone_help(ctx: &mut Ctx, text: i32, slot: i32, kind: i32, zone: i32, off: i32) {
    if kind != 3 { return super::super::oozla::zone_help(ctx, text, slot, kind, zone, off); }
    let k = slot.max(0) as usize;
    if ctx.game.help_counts.get(k).copied().unwrap_or(-1) == -1 { return; }
    if off != -1 && ctx.level.in_cuboid(off, ctx.hero.pos) {
        ctx.game.help_counts[k] = -1;
        return;
    }
    if !ctx.level.in_cuboid(zone, ctx.hero.pos) || (ctx.game.help_levels[k] as i32) < 0 { return; }
    ctx.game.help(text, slot);
}

/// 0x0c2d (`0x414290`, 1 placed): the level's help texts. Its pvars: `+0x00` a cuboid, `+0x04` / `+0x0c` two zones,
/// `+0x08` / `+0x10` two timers (420 and 300 ticks). **0**: update distance 0xff, the timers, **1**. **1** each tick:
/// with item 8 had, 0x9ff (slot 0x56, kind 3) in `+0x04`; with the arena on (`0x1a9010`) after the second frame the
/// slot 0x56 off; Ratchet in `+0x00` with the timer out, 0xa00 (slot 0x57; `0x3076b8`) and the timer again; with the
/// Charge Boots (0x36), 0xa03 (slot 0x5a, kind 0) in `+0x0c`; with item 0x13 and the slot 0x94 never shown, 0xa3d
/// (slot 0x94) when the second timer runs out (once).
///
/// Not ported: the halfword `0x194158` the 0xa00 help also waits on (nothing in the code writes it; taken as 0).
pub fn update_0c2d(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    if m.state == 0 {
        m.state = 1;
        m.update_dist = 0xff;
        m.set_pv_i(8, 0x1a4);
        m.set_pv_i(0x10, 300);
    }
    if m.state != 1 { return; }
    let item = |ctx: &Ctx, k: usize| ctx.game.items.get(k).is_some_and(|&b| b != 0);
    if item(ctx, 8) { zone_help(ctx, 0x9ff, 0x56, 3, w.mobys[id].pv_i(4), -1); }
    if ctx.game.arena_on && 2 < ctx.game.frame { ctx.game.help_counts[0x56] = -1; }
    if ctx.level.in_cuboid(w.mobys[id].pv_i(0), ctx.hero.pos) {
        let m = &mut w.mobys[id];
        let mut t = m.pv_i(8);
        let out = countdown(&mut t);
        m.set_pv_i(8, t);
        if out {
            ctx.game.help(0xa00, 0x57);
            w.mobys[id].set_pv_i(8, 0x1a4);
        }
    }
    if item(ctx, 0x36) { zone_help(ctx, 0xa03, 0x5a, 0, w.mobys[id].pv_i(0xc), -1); }
    if item(ctx, 0x13) && ctx.game.help_counts.get(0x94).copied() == Some(0) {
        let m = &mut w.mobys[id];
        let mut t = m.pv_i(0x10);
        let out = countdown(&mut t);
        m.set_pv_i(0x10, t);
        if out { ctx.game.help(0xa3d, 0x94); }
    }
}

/// `0x337298(r, yaw, pitch, out)`: the point `r` away at the yaw and pitch.
fn polar(r: f32, yaw: f32, pitch: f32) -> V3 { [yaw.cos() * r * pitch.cos(), yaw.sin() * r * pitch.cos(), pitch.sin() * r] }

/// 0x0dfd (`0x458678`, 23 placed): sparks drifting round it. Its pvars (floats): `+0x00..` up to 8 points (16 bytes
/// each), `+0x80` how many (at most 8), `+0x84` / `+0x88` the nearest and farthest, `+0x8c` / `+0x90` the highest and
/// lowest pitch, `+0x94` / `+0x98` the distance's drift (a second), `+0x9c` the turn (°/tick), `+0xa0` / `+0xa4` the
/// pitch's drift (°). **0**: each point at a random distance (`randf`) a random yaw and pitch (`0x32de28`) from it, **1**.
/// **1**: its draw ([`draw_0dfd`], `0x4588f8`); each point turned by the turn about it, its pitch drifted
/// (`0x32ddc0`, crate::rng::Rng::randf_signed) and held to the range, its distance drifted and held to the range.
pub fn update_0dfd(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let rng = &mut ctx.game.rng;
    let m = &mut w.mobys[id];
    let deg = 0.017_453_294;
    if m.state == 0 {
        if 8 < m.pv_i(0x80) { m.set_pv_i(0x80, 8); }
        m.state = 1;
        for k in 0..m.pv_i(0x80).max(0) as usize {
            let r = rng.randf(m.pv_f(0x84), m.pv_f(0x88));
            let (yaw, pitch) = (rng.rand_angle(), rng.rand_angle());
            let p = add(polar(r, yaw, pitch), m.pos);
            m.set_pv_v(0x10 * k, p);
        }
    }
    if m.state != 1 { return; }
    w.register_draw(id);
    let m = &mut w.mobys[id];
    for k in 0..m.pv_i(0x80).max(0) as usize {
        let (o, p) = (m.pos, m.pv_v(0x10 * k));
        let yaw = sub_rot(atan2(p[0] - o[0], p[1] - o[1]), m.pv_f(0x9c) * deg);
        let pitch = atan2(dist2(o, p), p[2] - o[2]);
        let drift = rng.randf_signed(m.pv_f(0xa4), m.pv_f(0xa0));
        let pitch = sub_rot(pitch, drift * deg).min(m.pv_f(0x8c)).max(m.pv_f(0x90));
        let grow = rng.randf_signed(m.pv_f(0x98) / 60.0, m.pv_f(0x94) / 60.0);
        let r = (dist(o, p) + grow).min(m.pv_f(0x88)).max(m.pv_f(0x84));
        m.set_pv_v(0x10 * k, add(polar(r, yaw, pitch), o));
    }
}

/// `0x4588f8`: 0x0dfd's sparks — at each point two sprites of FX 0x31 (`$gp+0x2e24`) facing the camera, pushed 0.08
/// toward it, each at a random roll (`0x2edf78`, crate::fx::sprite): 0.2 across in 0x3f106060 (`$gp+0x2e2c` /
/// `+0x2e28`), then 0.08 in 0x80808080, additive.
pub fn draw_0dfd(w: &MobyWorld, id: usize, rng: &mut crate::rng::Rng, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let m = &w.mobys[id];
    for k in 0..m.pv_i(0x80).clamp(0, 8) as usize {
        let p = m.pv_v(0x10 * k);
        let push = f32::from_bits(0x3da3_d70a);
        crate::fx::sprite(out, 0.2, push, rng.rand_angle(), p, cam, 0x31, 0x3f10_6060, true);
        crate::fx::sprite(out, 0.2 * 0.4, push, rng.rand_angle(), p, cam, 0x31, 0x8080_8080, true);
    }
}

/// Skill point 0x12 (`0x1a7a7a`): the level's enemies (0x0cf5 / 0x0cf6 / 0x0d07) all dead; 0x14 (`0x1a7a7c`): twelve
/// birds 0x1105 killed (crate::skill_points).
const POINT_ENEMIES: usize = 0x12;
const POINT_BIRDS: usize = 0x14;

/// 0x12c7 (`0x399768`, 1 placed): the level's two skill points. Its pvars: `+0x00` the live enemies counted in this
/// round, `+0x04` the moby the count has got to. **0**: update distance 0xff, the birds' kill count (`$gp−0x334`)
/// 0, **1**. **1** each tick: Ratchet in the death fall (0x74) or out of health, `0x1a9050` and `$gp−0x338` 0; after
/// the fifth frame, Ratchet idle (state 0) with no health, `0x1a9050` 1. With `0x1a904c` set and point 0x12 not had,
/// the next 40 mobys of the table are looked at (`0x1b361c..0x1b3620`) and the live 0x0cf5 / 0x0cf6 / 0x0d07 among
/// them counted; at the table's end a round with none awards it, and the count starts again. More than 11 birds
/// killed: point 0x14 (`0x1a7a68[k] = 1`, the level's sound 1, the message 0x1233, `0x1a903c`: GameState::
/// award_skill_point).
///
/// Not ported: who sets `0x1a904c` (nothing in the port does; kept as [`super::Globals::g904c`], so the enemies'
/// point does not come yet — their classes have no port either).
pub fn update_12c7(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.update_dist = 0xff;
            w.level11.bird_kills = 0;
            w.mobys[id].set_state(1, -1);
            return;
        }
        1 => {}
        _ => return,
    }
    let health = ctx.hero.hp;
    if ctx.hero.state == 0x74 || health <= 0 {
        w.level11.g9050 = 0;
        w.level11.g338 = 0;
    }
    if 5 < ctx.game.frame && ctx.hero.state == 0 && health < 1 { w.level11.g9050 = 1; }
    if w.level11.g904c != 0 && ctx.game.skill_points.get(POINT_ENEMIES).copied() == Some(0) {
        for _ in 0..0x28 {
            let k = w.mobys[id].pv_i(4).max(0) as usize;
            if let Some(m) = w.mobys.get(k) {
                if matches!(m.o_class, 0x0cf5 | 0x0cf6 | 0x0d07) && m.state < 0x80 {
                    let n = w.mobys[id].pv_i(0) + 1;
                    w.mobys[id].set_pv_i(0, n);
                }
            }
            let next = k + 1;
            w.mobys[id].set_pv_i(4, next as i32);
            if w.mobys.len() <= next {
                if w.mobys[id].pv_i(0) == 0 { ctx.game.award_skill_point(POINT_ENEMIES); }
                w.mobys[id].set_pv_i(4, 0);
                w.mobys[id].set_pv_i(0, 0);
            }
        }
    }
    if 0xb < w.level11.bird_kills { ctx.game.award_skill_point(POINT_BIRDS); }
}

/// 0x08e2 (`0x3d0750`, 1 placed): the updraft pad. Its pvars: `+0x00` a moby (−1 none), `+0x04` the glow's phase,
/// `+0x08` a cuboid (−1 none), `+0x0c` the lift it gives (0: Ratchet's own). **0**: mode | 0x10 (the glow list), **1**.
/// **1**: Ratchet standing on it with item 8: the prompt 0x1349 (owner 0xc, `0x339e80`, crate::prompt), the updraft's
/// 10 ticks (`0x18a012`), so a jump floats (state 0x82), with its lift (`0x18a3d8`) when it has one. Ratchet floating
/// (0x82) in its cuboid: the fall (`0x2d9208(6, 1)`). Drawn, its glow word pulses 6° a tick between 0x20 and 0x80 of
/// red (green with item 8).
///
/// Not ported: its moby `+0x00` let go on the first tick (`0x48bd00()`, no twin found; −1 on the disc).
pub fn update_08e2(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let has = ctx.game.items.get(8).is_some_and(|&b| b != 0);
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.mode |= 0x10;
            m.set_state(1, -1);
        }
        1 => {
            if w.mobys[id].first() && w.mobys[id].pv_i(0) != -1 { w.mobys[id].set_pv_i(0, -1); }
            if ctx.hero.ground_moby == Some(id) && ctx.hero.grounded_ticks != 0 {
                if has {
                    ctx.game.prompt.force(0xc, 0x1349, 1);
                    ctx.hero.gear.updraft = 10;
                    let lift = w.mobys[id].pv_f(0xc);
                    if lift != 0.0 { ctx.hero.gear.lift_store = lift; }
                }
            } else if ctx.hero.state == 0x82 && 0 <= w.mobys[id].pv_i(8) && ctx.level.in_cuboid(w.mobys[id].pv_i(8), ctx.hero.pos) {
                ctx.hero.request_state(6, true);
            }
        }
        _ => {}
    }
    let m = &mut w.mobys[id];
    if m.drawn == 0 { return; }
    let a = crate::hero::physics::add_rot(m.pv_f(4), f32::from_bits(0x3dd6_7752));
    m.set_pv_f(4, a);
    let v = ((a.sin() * 128.0) as i32).clamp(0x20, 0x80) as u32;
    m.glow = if has { v << 8 } else { v } | 0x8000_0000;
}
