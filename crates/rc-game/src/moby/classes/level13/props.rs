//! Level 13's small classes (L13 addresses; the engine functions named by their L13 addresses with the L00 ones the
//! port knows them by).
//!
//! * **0x0b5c** / **0x0b67** / **0x0b68** / **0x0b5b** / **0x0b61** ([`update_0b5c`], `0x3e2748` / `0x3e27b8` /
//!   `0x3e27f0` / `0x3e2710` / `0x3e2780`; 4, 2, 1, 1 and 1 placed): turn about their pitch.
//! * **0x0dd1** / **0x0da3** / **0x0da1** ([`update_0dd1`], `0x42d390` / `0x42d368` / `0x42d340`, one each): mode
//!   0x800 at the start, nothing more.
//! * **0x0ef2** / **0x113c** ([`update_0ef2`], `0x42d3b8` / `0x42d4b0`, one each): sink 6 once a linked moby is in
//!   its state 2.
//! * **0x12ed** ([`update_12ed`], `0x38d8b8`, 1 placed): two cuboids that each ask for their music.
//! * **0x0f20** ([`update_0f20`], `0x38d770`, 1 placed): hides the level's enemies in its scene 3.
//! * **0x040f** ([`update_040f`], `0x38d600`, 2 placed): cuboids that kill Ratchet.
//!
//! The engine functions: `0x2fa528` add_rot, `0x36a110` the class sound, `0x328ff0` = L00 `0x31ed78`
//! (crate::hero::physics::ease), `0x32b7c0` = L00 `0x321858` (crate::level::LevelData::in_cuboid), `0x3387a8` the music
//! request (crate::audio::chunk `music_request`), `0x2cf1f8` = L00 `0x2c4560` (Ratchet's state), `0x2be1f8` = L00
//! `0x2b31f0` (his death, crate::hero::Hero::die).

use crate::hero::physics::{add_rot, ease};
use crate::moby::{Ctx, MobyWorld};

/// 0x0b5c / 0x0b67 / 0x0b68 / 0x0b5b (`0x3e2748` / `0x3e27b8` / `0x3e27f0` / `0x3e2710`): its pitch `+0xf4` turned
/// 1° a tick; 0x0b61 (`0x3e2780`) 1.67° (`0x2fa528`).
pub fn update_0b5c(w: &mut MobyWorld, id: usize, _ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    let k = if m.o_class == 0x0b61 { f32::from_bits(0x3cee_4bb0) } else { f32::from_bits(0x3c8e_fa37) };
    m.rot[1] = add_rot(m.rot[1], k);
}

/// 0x0dd1 / 0x0da3 / 0x0da1 (`0x42d390` / `0x42d368` / `0x42d340`). **0**: mode | 0x800, 1.
pub fn update_0dd1(w: &mut MobyWorld, id: usize, _ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    if m.state == 0 {
        m.state = 1;
        m.mode |= 0x800;
    }
}

/// 0x0ef2 (`0x42d3b8`) / 0x113c (`0x42d4b0`): its pvars `[0]` a moby, `[4]` its height to sink to, `[8]` its height
/// at the start, `[0xc]` its speed. **0**: mode | 0x800, `[8]` its height, `[4]` 6 below, 1. **1**: the moby `[0]` in
/// its state 2: its class sound 0 (`0x36a110`), 2 (0x113c also prints a debug line, `0x2af8f0`). **2**: its height
/// eased toward `[4]` (`0x328ff0`: 1/600 a tick² both ways, at most 0.2 a tick). The states set directly.
pub fn update_0ef2(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 0 {
        let m = &mut w.mobys[id];
        m.mode |= 0x800;
        let z = m.pos[2];
        m.set_pv_f(8, z);
        m.set_pv_f(4, z - 6.0);
        m.state = 1;
    }
    if w.mobys[id].state == 1 {
        let link = usize::try_from(w.mobys[id].pv_i(0)).ok().and_then(|i| w.mobys.get(i)).map(|o| o.state);
        if link == Some(2) {
            ctx.host.play_sound(&w.mobys[id], 0, 0);
            w.mobys[id].state = 2;
        }
    }
    if w.mobys[id].state == 2 {
        let m = &mut w.mobys[id];
        let k = f32::from_bits(0x3ada_740e);
        let (mut z, mut v) = (m.pos[2], m.pv_f(0xc));
        ease(m.pv_f(4), k, k, f32::from_bits(0x3e4c_ccce), &mut z, &mut v);
        m.pos[2] = z;
        m.set_pv_f(0xc, v);
    }
}

/// 0x12ed (`0x38d8b8`): its pvars `[0]` / `[4]` two cuboids, `[0x10]` / `[0x14]` their tracks, `[0x18]` / `[0x1c]`
/// their stingers. **0**: update distance 0xff, 1. **1**: Ratchet in cuboid `[0]` (`0x32b7c0`): the music request
/// (`0x3387a8([0x10], [0x18])`), 2. **2**: in cuboid `[4]`: `0x3387a8([0x14], [0x1c])`, 1.
pub fn update_12ed(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let p = ctx.hero.pos;
    let m = &mut w.mobys[id];
    match m.state {
        0 => {
            m.update_dist = 0xff;
            m.set_state(1, -1);
        }
        1 | 2 => {
            let k = (m.state - 1) as usize * 4;
            if !ctx.level.in_cuboid(m.pv_i(k), p) { return; }
            ctx.host.music_request(m.pv_i(0x10 + k), m.pv_i(0x18 + k));
            let next = 3 - m.state;
            m.set_state(next, -1);
        }
        _ => {}
    }
}

/// The classes 0x0f20 hides.
const ENEMIES: [i16; 4] = [0x01f4, 0x0ea3, 0x0ea4, 0x11eb];

/// 0x0f20 (`0x38d770`). **0**: update distance 0xff, 1. **1**: while a scene plays (`0x1a8f00` = 2) and it is the
/// level's scene 3 (`0x1bd570`, the scene loaded, `0x30f9e0`), every moby of the classes 0x01f4, 0x0ea3, 0x0ea4 and
/// 0x11eb (the moby list `$gp+0x38b8` by `+0x28`) without collision (`+0x98` 0), hidden and not a target (mode
/// `| 0x41 & 0xefff`).
pub fn update_0f20(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.state = 1;
            m.update_dist = 0xff;
        }
        1 => {
            let s = &ctx.game.scenes;
            if s.mode != crate::scene::SCENE || s.player.as_ref().map(|p| p.id) != Some(3) { return; }
            for m in w.mobys.iter_mut().filter(|m| m.state < 0x80 && ENEMIES.contains(&m.o_class)) {
                m.has_collision = false;
                m.mode = (m.mode | 0x41) & 0xefff;
            }
        }
        _ => {}
    }
}

/// 0x040f (`0x38d600`): its pvars `[0..0x80]` 32 cuboids that kill, `[0x80..0x100]` 32 that kill at once (−1 none),
/// `[0x100]` ≠ 0: the first kind only slows a glide. **0**: update distance 0xff, 1. **1**: Ratchet in one of the first
/// (`0x32b7c0`): with `[0x100]` his glide's sink 0.25 (`0x18c2e8`); else as Ratchet (`0x18c0d4` = 0) his fall out of
/// the level, state 0x74 (`0x2cf1f8(0x74, 1)`, not again while in it), another body his death (`0x2be1f8`, after a
/// fade `0x2f2770(10)`); in one of the second his death.
///
/// Not ported: the blocking fade to black of 10 frames (`0x2f2770`: it draws the frames itself before the death).
pub fn update_040f(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    match m.state {
        0 => {
            m.update_dist = 0xff;
            m.state = 1;
        }
        1 => {
            let p = ctx.hero.pos;
            let glide = m.pv_i(0x100) != 0;
            for k in 0..32 {
                let c = m.pv_i(4 * k);
                if c < 0 || !ctx.level.in_cuboid(c, p) { continue; }
                if glide {
                    ctx.hero.glide_sink = 0.25;
                } else if ctx.hero.mode == 0 {
                    if ctx.hero.state != 0x74 { ctx.hero.request_state(0x74, true); }
                } else {
                    ctx.hero.die();
                }
            }
            for k in 0..32 {
                let c = m.pv_i(0x80 + 4 * k);
                if 0 <= c && ctx.level.in_cuboid(c, p) { ctx.hero.die(); }
            }
        }
        _ => {}
    }
}
