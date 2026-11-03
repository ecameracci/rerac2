//! Barlow's ambush (L04 addresses): the trigger **0x09f0** ([`update_09f0`], `0x3eebf8`, 2 placed) that sends the
//! riders in behind a fly-by, and the gates **0x0ce6** ([`update_0ce6`], `0x42d678`, 2 placed) it lowers and raises.

use crate::hero::physics::spring;
use crate::moby::{Ctx, MobyWorld};

/// The moby whose index is pvar word `off`.
fn moby_at(w: &MobyWorld, id: usize, off: usize) -> Option<usize> { usize::try_from(w.mobys[id].pv_i(off)).ok().filter(|&i| i < w.mobys.len()) }

/// Its mission (`+0xb0`) done on this level.
fn mission_done(w: &MobyWorld, id: usize, ctx: &Ctx) -> bool {
    let k = ctx.game.level.max(0) as usize * 0x10 + w.mobys[id].mission as usize;
    ctx.game.missions.get(k).is_some_and(|b| b & 0x80 != 0)
}

/// The gates `+0x74` / `+0x78` to `state` (with `sound` the first's class sound 0 first, `0x384098(0, 0, gate,
/// 0x0ce6)`, when it stood in state 1).
fn gates(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, state: u8, sound: bool) {
    let (a, b) = (moby_at(w, id, 0x74), moby_at(w, id, 0x78));
    if let Some(a) = a {
        if sound && w.mobys[a].state == 1 { ctx.host.play_sound_of(&w.mobys[a], 0x0ce6, 0, 0); }
        w.mobys[a].state = state;
    }
    if let Some(b) = b { w.mobys[b].state = state; }
}

/// Ratchet at the cuboid `+0x14` (its place and angles straight into his, `0x189ea0` / `0x189eb0`).
fn place_hero(w: &MobyWorld, id: usize, ctx: &mut Ctx) {
    if let Some((p, r)) = ctx.level.cuboid_place(w.mobys[id].pv_i(0x14)) {
        ctx.hero.pos = p;
        ctx.hero.rot = r;
    }
}

/// 0x09f0 (`0x3eebf8`). Its words: four groups `+0x68`[0..4], the fly-by camera record `+0x10`, Ratchet's cuboid
/// after it `+0x14`, its kind `+0x18` (0 / 1), fourteen riders `+0x20..+0x58` (−1 none), the trigger cuboid `+0x70`,
/// the gates `+0x74` / `+0x78`.
/// * **0** → 1.
/// * **1**: some group alive: kind 0 with Ratchet in the trigger cuboid and its mission not done: the fly-by armed
///   (crate::cameras::Cameras::arm, `0x46cfd8`), 2. None alive, kind 0: the gates down (2, the first's sound); with
///   Ratchet in the cuboid and the mission not done the fly-by, 2, the gates up (3). Kind 1 and the mission not done:
///   the fly-by, 4, the gates down; the mission done (`0x3345f8`) and the checkpoint at its place
///   (`0x37d520` = L00 `0x359b20`).
/// * **2**: the fly-by over (`0x46d000`, crate::cameras::Cameras::flyby_over): the level's looped sound 0x17 on
///   Ratchet (`0x3841e8`, its handle kept in the sub-state byte), each rider's near area and path made its far ones
///   (`+0x250` = `+0x254`, `+0x260` = `+0x264`), shown and solid; Ratchet at `+0x14`'s cuboid; its once bit 2; 5.
/// * **3**: the fly-by over: Ratchet at the cuboid; the first rider (`+0x20`) into sub 4, state 0x11, solid; its and
///   `+0x24`'s `+0x270` = `+0x274`; 5.
/// * **4**: the fly-by over: Ratchet put at the cuboid (`0x2dfe20`), 5, its once bit 2.
/// * **5**: after 60 ticks, the fly-by over: the sound stopped.
///
/// Not ported: state 3's global `0x1b9804` (set to the rider `+0x24` while 0, `0x1b9808` = 0), which nothing ported
/// reads; `0x2dfe20`'s state 0x6f argument (Ratchet's state).
pub fn update_09f0(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let fade = ctx.game.scenes.fade;
    match w.mobys[id].state {
        0 => w.mobys[id].state = 1,
        1 => {
            let alive = (0..4).any(|k| super::super::props::group_alive(w, w.mobys[id].pv_i(4 * k)) != 0);
            let kind = w.mobys[id].pv_i(0x18);
            let cam = w.mobys[id].pv_i(0x10);
            let inside = ctx.level.in_cuboid(w.mobys[id].pv_i(0x70), ctx.hero.pos);
            if alive {
                if kind == 0 && inside && !mission_done(w, id, ctx) {
                    ctx.game.cameras.arm(cam);
                    w.mobys[id].state = 2;
                }
            } else if kind == 0 {
                gates(w, id, ctx, 2, true);
                if inside && !mission_done(w, id, ctx) {
                    ctx.game.cameras.arm(cam);
                    w.mobys[id].state = 2;
                    gates(w, id, ctx, 3, false);
                }
            } else if kind == 1 && !mission_done(w, id, ctx) {
                ctx.game.cameras.arm(cam);
                w.mobys[id].state = 4;
                if let Some(a) = moby_at(w, id, 0x74) { ctx.host.play_sound_of(&w.mobys[a], 0x0ce6, 0, 0); }
                gates(w, id, ctx, 2, false);
                if !mission_done(w, id, ctx) {
                    let (mission, pos, rot) = { let m = &w.mobys[id]; (m.mission, m.pos, m.rot) };
                    ctx.game.mission_done(mission);
                    ctx.game.save_checkpoint(pos, rot);
                }
            }
        }
        2 => if ctx.game.cameras.flyby_over(fade) {
            let h = ctx.host.play_level_sound_after(ctx.hero_moby.and_then(|i| w.mobys.get(i)), 0x17, 4);
            w.mobys[id].sub_state = h as u8;
            for k in 0..14 {
                let Some(r) = moby_at(w, id, 0x20 + 4 * k) else { continue };
                let solid = w.class(w.mobys[r].o_class).is_some_and(|c| c.has_collision);
                let m = &mut w.mobys[r];
                m.set_pv_i(0x260, m.pv_i(0x264));
                m.set_pv_i(0x250, m.pv_i(0x254));
                m.mode &= 0xffbe;
                m.has_collision = solid;
            }
            place_hero(w, id, ctx);
            let m = &mut w.mobys[id];
            m.once |= 4;
            m.state_ticks = 0;
            m.state = 5;
        },
        3 => if ctx.game.cameras.flyby_over(fade) {
            place_hero(w, id, ctx);
            if let Some(r) = moby_at(w, id, 0x20) {
                let solid = w.class(w.mobys[r].o_class).is_some_and(|c| c.has_collision);
                let m = &mut w.mobys[r];
                m.sub_state = 4;
                m.state = 0x11;
                m.has_collision = solid;
                m.set_pv_i(0x270, m.pv_i(0x274));
            }
            if let Some(r) = moby_at(w, id, 0x24) {
                let m = &mut w.mobys[r];
                m.set_pv_i(0x270, m.pv_i(0x274));
            }
            w.mobys[id].state = 5;
        },
        4 => if ctx.game.cameras.flyby_over(fade) {
            if let Some((p, r)) = ctx.level.cuboid_place(w.mobys[id].pv_i(0x14)) { ctx.hero.teleport(p, r); }
            let m = &mut w.mobys[id];
            m.state = 5;
            m.once |= 4;
        },
        5 => {
            let m = &mut w.mobys[id];
            if (m.state_ticks as u16) < 0x3c {
                m.state_ticks += 1;
            } else if ctx.game.cameras.flyby_over(fade) {
                let h = m.sub_state as i8 as i32;
                if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
                w.mobys[id].sub_state = 0xff;
            }
        }
        _ => {}
    }
}

/// 0x0ce6 (`0x42d678`): a gate the ambush (0x09f0) moves. **0** → 1 (it waits). **2**: down 17/60 a tick while at 30
/// or above; under 30, 4. **3**: up toward 55 on a spring (`0x3407c0` = L00 `0x31ebd8`, crate::hero::physics::spring:
/// 1/120, 1/72, at most 0.45, its speed `+0x68`[0]); at 55 its sound 0, `+0x68`[1] = 0, 1.
pub fn update_0ce6(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    if m.state == 0 { m.state = 1; }
    if m.state == 2 {
        if 30.0 <= m.pos[2] { m.pos[2] -= f32::from_bits(0x3e91_1112); }
        if m.pos[2] < 30.0 { m.state = 4; }
    }
    if m.state == 3 {
        let (mut z, mut v) = (m.pos[2], m.pv_f(0));
        spring(55.0, f32::from_bits(0x3c08_8889), f32::from_bits(0x3c63_8e39), f32::from_bits(0x3ee6_6667), &mut z, &mut v);
        m.pos[2] = z;
        m.set_pv_f(0, v);
        if 55.0 <= z {
            ctx.host.play_sound(&w.mobys[id], 0, 0);
            let m = &mut w.mobys[id];
            m.set_pv_i(4, 0);
            m.state = 1;
        }
    }
    w.changed = true;
}

/// 0x1362 (`0x3ac840`, one placed; Ghidra has no function there): the fight's loop. Ratchet in its cuboid `+0x68`[0]
/// keeps the level's looped sound 0x17 past the split on it (`0x3841e8(0x17, 4, m)`, its handle `+0x68`[2]); with
/// fewer than seven of its group `+0x68`[1] alive the sound is stopped and it is deleted; else farther than 50 across
/// from Ratchet the sound stops.
pub fn update_1362(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let (cub, group, h) = (m.pv_i(0), m.pv_i(4), m.pv_i(8));
    let playing = h != -1 && ctx.host.sound_playing(h);
    if ctx.level.in_cuboid(cub, ctx.hero.pos) && !playing {
        let k = ctx.host.play_level_sound_after(Some(&w.mobys[id]), 0x17, 4);
        w.mobys[id].set_pv_i(8, k);
    }
    let h = w.mobys[id].pv_i(8);
    let playing = h != -1 && ctx.host.sound_playing(h);
    if super::super::props::group_alive(w, group) < 7 {
        if playing { ctx.host.stop_sound(h); }
        w.mobys[id].set_pv_i(8, -1);
        w.delete(id);
        return;
    }
    if playing && 50.0 < crate::hero::physics::dist2(w.mobys[id].pos, ctx.hero.pos) {
        ctx.host.stop_sound(h);
        w.mobys[id].set_pv_i(8, -1);
    }
}
