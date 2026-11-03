//! Level 6's small classes (L06 addresses; the engine functions named by their L06 addresses with the L00 ones the
//! port knows them by).
//!
//! * **0x0dda** ([`update_0dda`], `0x4462a8`, 16 placed; the same code on levels 11 `0x4526f8`, 12, 13, 14, 16, 17,
//!   18, 19, 21): the area switch of crate::moby::classes::area_switch by areas.
//! * **0x089d** ([`update_089d`], `0x4019e0`, 17 placed): bobs up and down.
//! * **0x0948** ([`update_0948`], `0x4054b0`, 2 placed): rises and falls with a linked moby's travel.
//! * **0x0fe2** ([`update_0fe2`], `0x459b80`, 2 placed): a chain turned by a linked moby's travel.
//! * **0x02ee** ([`update_02ee`], `0x3e22c0`, 1 placed): a planet's coordinates.
//! * **0x07ef** ([`update_07ef`], `0x3f5e50`, 1 placed): deletes itself.
//! * **0x0c4b** ([`update_0c4b`], `0x42cdf0`, 1 placed): opens and closes when its group pokes it.
//! * **0x0e8e** ([`update_0e8e`], `0x44c848`, 1 placed): sets two flag bytes when Ratchet comes in.

use crate::hero::physics::add_rot;
use crate::moby::{Ctx, MobyWorld};

/// 0x0dda (`0x4462a8`; L11 `0x4526f8`): an area switch (crate::moby::classes::area_switch's 0x0102 with areas). Its
/// update distance `+0x30` 0xff each tick. Inside: Ratchet (`0x189ea0`; with `+0x7e` set the camera, L06 `0x1b9840`,
/// L11 `0x1ba940`) in any of its four areas `+0x68`[0..0x10] (−1 none; `0x36b6d0`, L11 `0x33d5d8` = L00 `0x32a848`,
/// crate::level::LevelData::in_area); `+0x95` other than 0 forces it (1 inside, else outside). When that changes
/// from `+0xbc` (or in state 0): `+0xbc` the result, the state that + 1, and the members of its thirteen groups
/// `[0x10..0x44]` (`0x446238` over `0x1fb3a0`'s lists) and its fourteen mobys `[0x44..0x7c]` (−1 none) switched by
/// the flags `[0x7c]` / `[0x7d]` / `[0x7f]` (`0x446010`, L11 `0x452460`: area_switch::switch, the same code).
pub fn update_0dda(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    m.update_dist = 0xff;
    let inside = match m.sub_state {
        0 => {
            let p = if m.pv_b(0x7e) == 0 { ctx.hero.pos } else { ctx.host.camera_pos() };
            (0..4).map(|k| m.pv_i(4 * k)).any(|a| a != -1 && ctx.level.in_area(a, p))
        }
        s => s == 1,
    };
    if m.state != 0 && (m.timer != 0) == inside { return; }
    m.timer = inside as u8;
    m.state = m.timer + 1;
    let m = &w.mobys[id];
    let flags = [m.pv_b(0x7c) as i8, m.pv_b(0x7d) as i8, m.pv_b(0x7f) as i8];
    let mut targets: Vec<usize> = Vec::new();
    for k in 0..13 {
        let g = m.pv_i(0x10 + 4 * k);
        if g == -1 { continue; }
        if let Some(members) = usize::try_from(g).ok().and_then(|g| w.groups.get(g)) { targets.extend(members); }
    }
    for k in 0..14 {
        if let Ok(i) = usize::try_from(m.pv_i(0x44 + 4 * k)) { targets.push(i); }
    }
    for i in targets { super::super::area_switch::switch(w, i, flags, inside); }
    w.changed = true;
}

/// `$gp+0xe00`: 0x089d's turn, 240°/s.
const BOB_TURN: f32 = 240.0 * 0.000_290_888_25;

/// 0x089d (`0x4019e0`): its pvars `[0]` a moby that holds it (> 0; its pvar byte `+0x14` ≠ 0 stops the bob), `[4]`
/// the start phase, `[8]` its height, `[0xc]` the phase. Nothing without pvars. **0**: sequence 0 at frame 0
/// (`0x35b510`), the phase from `[4]`, its height kept, 1. **Each tick**: within a turn of 0 or π (`0x32f520`) its
/// class sound 0 (`0x3a1c98`); unless held the phase turns 4° (`0x32f438`); its height `[8] − 2 + 2·sin(phase)`
/// (`0x32ea38`, the VU0 sine).
///
/// Not ported: the moby's word `+0x48` set to 0 in state 0 (nothing the port has reads it).
pub fn update_089d(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pvars.is_empty() { return; }
    if w.mobys[id].state == 0 {
        ctx.host.set_anim(&w.mobys[id], 0, 0, 0);
        let m = &mut w.mobys[id];
        m.set_pv_f(0xc, m.pv_f(4));
        m.state = 1;
        m.set_pv_f(8, m.pos[2]);
    }
    let link = w.mobys[id].pv_i(0);
    let held = usize::try_from(link).ok().filter(|&i| 0 < i).and_then(|i| w.mobys.get(i)).is_some_and(|o| o.pv_b(0x14) != 0);
    let apart = |a: f32, b: f32| { let d = (a - b).abs(); if std::f32::consts::PI <= d { std::f32::consts::TAU - d } else { d } };
    let phase = w.mobys[id].pv_f(0xc);
    if apart(phase, 0.0) < BOB_TURN || apart(phase, f32::from_bits(0x4049_0fdc)) < BOB_TURN {
        ctx.host.play_sound(&w.mobys[id], 0, 0);
    }
    let m = &mut w.mobys[id];
    if !held { m.set_pv_f(0xc, add_rot(phase, BOB_TURN)); }
    let s = m.pv_f(0xc).sin();
    m.pos[2] = m.pv_f(8) - 2.0 + s + s;
}

/// The looped-sound handling 0x0948 and 0x0fe2 share, by a linked moby's value `v` (its first pvar, 0..1) against
/// the last tick's `last`, the loop's handle at `+0x68`[`at`]: unchanged, the loop stops (`0x3a1950` / `0x3a1998`);
/// at an end (0 or 1) the loop stops and its class sound 1 plays once; between, its class sound 0 loops
/// (`0x3a1c98(0, 4)`) unless already playing.
fn travel_sound(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, v: f32, last: f32, at: usize) {
    let h = w.mobys[id].pv_i(at);
    let playing = h != -1 && ctx.host.sound_playing(h);
    if v == last || v == 1.0 || v == 0.0 {
        if playing { ctx.host.stop_sound(h); w.mobys[id].set_pv_i(at, -1); }
        if v != last { ctx.host.play_sound(&w.mobys[id], 1, 0); }
    } else if !playing {
        let h = ctx.host.play_sound(&w.mobys[id], 0, 4);
        w.mobys[id].set_pv_i(at, h);
    }
}

/// 0x0948 (`0x4054b0`, 2 placed): rides a linked moby's travel. Its pvars: `[0]` the moby (−1 none), `[4]` its
/// rise over the travel, `[8]` its height at 0, `[0xc]` its looped sound, `[0x10]` the last travel. Nothing without
/// pvars. **0**: its height kept, Ratchet's moby's light and ambient (`+0x38`), the last travel 0, 1. **1**: its
/// height `travel × [4] + [8]` (the travel the linked moby's first pvar); the sound ([`travel_sound`]).
pub fn update_0948(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pvars.is_empty() { return; }
    match w.mobys[id].state {
        0 => {
            let lit = ctx.hero_moby.and_then(|h| w.mobys.get(h)).map(|h| (h.light, h.ambient));
            let m = &mut w.mobys[id];
            m.set_pv_f(8, m.pos[2]);
            m.state = 1;
            if let Some((light, ambient)) = lit { (m.light, m.ambient) = (light, ambient); }
            m.set_pv_i(0x10, 0);
        }
        1 => {
            let Some(v) = usize::try_from(w.mobys[id].pv_i(0)).ok().and_then(|i| w.mobys.get(i)).map(|o| o.pv_f(0)) else { return };
            let m = &mut w.mobys[id];
            m.pos[2] = v * m.pv_f(4) + m.pv_f(8);
            let last = m.pv_f(0x10);
            travel_sound(w, id, ctx, v, last, 0xc);
            w.mobys[id].set_pv_f(0x10, v);
        }
        _ => {}
    }
}

/// 0x0fe2 (`0x459b80`, 2 placed): a chain of links turned by a linked moby's travel. Its pvars: `[0]` the moby (−1
/// none), `[4]` the chain's length left, `[8]` its yaw at rest, `[0xc]` the turn (π × travel × it), `[0x10]` the
/// link it hangs from (0: the placed head), `[0x14]` the turn now, `[0x18]` its pitch at rest, `[0x1c]` a link's
/// length (its joint 0 from its place, `0x352810` = L00 `0x3127c8`), `[0x20]` its looped sound, `[0x24]` the last
/// travel.
/// * **0** (the placed head): its yaw and pitch kept, `[0x10]` 0, the link's length taken off `[4]`, the turn 1 when
///   0; the chain hung from it ([`chain_0fe2`]); the last travel 0, 1.
/// * **1**: with the linked moby: its yaw `[8]` plus the turn (none when `[0xc]` is 0, `0x32e3a0`), past 0.6 of the
///   travel its pitch `[0x18]` less `(travel − 0.6) × 7.5°` (`0x32f480`); the head only, the sound ([`travel_sound`]
///   on `[0x20]`); the last travel kept. A link then goes to its parent's joint 0.
pub fn update_0fe2(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use crate::hero::physics::{len, sub, sub_rot};
    match w.mobys[id].state {
        0 => {
            let j = ctx.host.joint(&w.mobys[id], 0).unwrap_or(w.mobys[id].pos);
            let m = &mut w.mobys[id];
            m.set_pv_f(8, m.rot[2]);
            m.set_pv_i(0x10, 0);
            m.set_pv_f(0x18, m.rot[1]);
            let d = len(sub(j, m.pos));
            m.set_pv_f(0x1c, d);
            m.set_pv_f(4, m.pv_f(4) - d);
            if m.pv_f(0xc) == 0.0 { m.set_pv_f(0xc, 1.0); }
            chain_0fe2(w, id, ctx);
            let m = &mut w.mobys[id];
            m.set_pv_i(0x24, 0);
            m.state = 1;
        }
        1 => {
            let link = w.mobys[id].pv_i(0);
            if link != -1 {
                let Some(v) = usize::try_from(link).ok().and_then(|i| w.mobys.get(i)).map(|o| o.pv_f(0)) else { return };
                let m = &mut w.mobys[id];
                let k = m.pv_f(0xc);
                let t = if 0.0 < k.abs() { v * k * std::f32::consts::PI } else { 0.0 };
                m.set_pv_f(0x14, t);
                m.rot[2] = add_rot(m.pv_f(8), t);
                if 0.6 < v { m.rot[1] = sub_rot(m.pv_f(0x18), (v - 0.6) * 0.130_899_71); }
                if m.pv_i(0x10) == 0 {
                    let last = m.pv_f(0x24);
                    travel_sound(w, id, ctx, v, last, 0x20);
                }
                w.mobys[id].set_pv_f(0x24, v);
            }
            let parent = w.mobys[id].pv_i(0x10);
            if let Some(p) = usize::try_from(parent).ok().filter(|&p| p != 0).and_then(|p| w.mobys.get(p)) {
                if let Some(j) = ctx.host.joint(p, 0) { w.mobys[id].pos = j; }
            }
        }
        _ => {}
    }
}

/// `0x4599c0(m)`: while the chain has length left (`[4]` > 0), a link (CreateMoby 0x0fe2) hung from `m`: drawn,
/// draw and update distances 0xff, in state 1, `m`'s scale, light and ambient; `[0x10]` `m`, its linked moby and
/// link length `m`'s, the length left less one link, its yaw at rest and now `m`'s turned by π, `m`'s other angles;
/// its turn from `m`'s (±0.0001 → ±1, ±1 → ∓0.0001, else 0); at `m`'s joint 0, its matrix rebuilt (`0x353db8`), and
/// on down the chain.
fn chain_0fe2(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let mut at = id;
    while 0.0 < w.mobys[at].pv_f(4) {
        let j = ctx.host.joint(&w.mobys[at], 0).unwrap_or(w.mobys[at].pos);
        let c = w.create(0x0fe2);
        let p = w.mobys[at].clone();
        let m = &mut w.mobys[c];
        (m.draw_dist, m.state, m.drawn, m.update_dist) = (0xff, 1, 1, 0xff);
        m.scale = p.scale;
        (m.light, m.ambient) = (p.light, p.ambient);
        m.set_pv_i(0x10, at as i32);
        m.set_pv_i(0, p.pv_i(0));
        let d = p.pv_f(0x1c);
        m.set_pv_f(0x1c, d);
        m.set_pv_f(4, p.pv_f(4) - d);
        let yaw = add_rot(p.rot[2], std::f32::consts::PI);
        m.set_pv_f(8, yaw);
        m.rot = [p.rot[0], p.rot[1], yaw];
        let k = p.pv_f(0xc);
        let turn = if k == 0.0001 { 1.0 } else if k == -0.0001 { -1.0 } else if k == 1.0 { f32::from_bits(0xb8d1_b717) } else if k == -1.0 { f32::from_bits(0x38d1_b717) } else { 0.0 };
        m.set_pv_f(0xc, turn);
        m.pos = j;
        w.rebuild(c, &*ctx.host);
        at = c;
    }
}

/// 0x02ee (`0x3e22c0`, 1 placed): a planet's coordinates, `+0x68`[0] the planet. Its pvars: `[4]` a count, `[8]` the
/// ticks the name shows, `[0xc]` a fade. **0**: update distance 0xff; the planet unlocked already (`0x1a7bd0`):
/// deleted, else 1. **1**: Ratchet within 2 across and 2 up or down (`0x32e628`, `0x32e3a0`) — or the planet 0x18
/// unlocked by now — unlocks it (`0x363748` = L00 `0x322730`, crate::game::GameState::unlock_planet), mode 0x100,
/// 180 ticks, 2. **2**: while drawn its first two rows shrink 1/15 a tick and its third grows to match (`0x35ed48`,
/// `0x32e710`; at 0 hidden); the planet's name held on the prompt (`0x3e2580` from the frame's callbacks `0x326e78`,
/// `0x367f48(8, text, 1)` = L00 `0x327ae0`, crate::prompt::Prompt::force: `0x29cb60`[planet − 1], 0x11f0 + planet −
/// 1; 0x18's 0x1243); `[4]` up every even frame; the fade toward 1 by 0.05 for the 180 ticks, then toward 0, and at 0
/// deleted.
pub fn update_02ee(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use crate::hero::physics::{approach, dist2, len, set_len};
    let planet = w.mobys[id].pv_i(0);
    let unlocked = |g: &crate::game::GameState, p: i32| usize::try_from(p).ok().and_then(|p| g.planets.get(p)).is_some_and(|&b| b != 0);
    match w.mobys[id].state {
        0 => {
            w.mobys[id].update_dist = 0xff;
            if unlocked(ctx.game, planet) { return w.delete(id); }
            w.mobys[id].set_state(1, -1);
        }
        1 => {
            let m = &w.mobys[id];
            let near = dist2(m.pos, ctx.hero.pos) < 2.0 && (m.pos[2] - ctx.hero.pos[2]).abs() < 2.0;
            if near || (planet == 0x18 && unlocked(ctx.game, 0x18)) {
                ctx.game.unlock_planet(planet);
                let m = &mut w.mobys[id];
                m.set_state(2, -1);
                m.mode |= crate::moby::mode::KEEP_ROWS;
                m.set_pv_i(8, 0xb4);
            }
        }
        2 => {
            let m = &mut w.mobys[id];
            if m.mode & 1 == 0 {
                let mut s = len(m.rows[0]);
                approach(0.0, f32::from_bits(0x3d88_8889), &mut s);
                m.rows[0] = set_len(m.rows[0], s);
                m.rows[1] = set_len(m.rows[1], s);
                m.rows[2] = set_len(m.rows[2], 2.0 - s);
                if s <= 0.0 {
                    m.drawn = 0;
                    m.mode |= 1;
                }
            }
            let text = if planet == 0x18 { 0x1243 } else if (1..=0x14).contains(&planet) { 0x11f0 + planet as u32 - 1 } else { 0 };
            ctx.game.prompt.force(8, text, 1);
            let m = &mut w.mobys[id];
            if ctx.game.frame & 1 == 0 { m.set_pv_i(4, m.pv_i(4) + 1); }
            let mut t = m.pv_i(8);
            let over = crate::moby::countdown(&mut t);
            m.set_pv_i(8, t);
            let mut f = m.pv_f(0xc);
            approach(if over { 0.0 } else { 1.0 }, f32::from_bits(0x3d4c_ccce), &mut f);
            m.set_pv_f(0xc, f);
            if over && f <= 0.0 { w.delete(id); }
        }
        _ => {}
    }
}

/// 0x07ef (`0x3f5e50`, 1 placed): on its first update the six words `0x1b375c..0x1b3774` (0x0fc2's, `0x458a00`, not
/// ported) cleared, and deleted.
pub fn update_07ef(w: &mut MobyWorld, id: usize, _ctx: &mut Ctx) {
    if w.mobys[id].state == 0 { w.delete(id); }
}

/// 0x0c4b (`0x42cdf0`, 1 placed): opens when its group pokes it. Its pvar `[0]` the ticks it stays open. **0**:
/// 1500 ticks, 1. **1**: poked (`+0xbc` ≠ 0): sequence 1 (`0x35b5e8` = L00 `0x31b5f0`), no collision, 2. **2**: at
/// the sequence's end (`+0x60 & 2`) sequence 2, 3. **3**: the ticks out (`0x32e0a0`): sequence 3, 4. **4**: at its
/// end sequence 0, its collision back (the class's), 1500 ticks, its group's pokes cleared (`0x35cb88(group, 0)`),
/// 1.
pub fn update_0c4b(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use super::super::props::{set_seq, wrapped};
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.state = 1;
            m.set_pv_i(0, 0x5dc);
        }
        1 => if w.mobys[id].timer != 0 {
            w.mobys[id].state = 2;
            set_seq(w, id, ctx, 1, 0, 0);
            w.mobys[id].has_collision = false;
            w.changed = true;
        },
        2 => if wrapped(w, id, ctx) {
            w.mobys[id].state = 3;
            set_seq(w, id, ctx, 2, 0, 0);
        },
        3 => {
            let mut t = w.mobys[id].pv_i(0);
            let out = crate::moby::countdown(&mut t);
            w.mobys[id].set_pv_i(0, t);
            if out {
                w.mobys[id].state = 4;
                set_seq(w, id, ctx, 3, 0, 0);
            }
        }
        4 => if wrapped(w, id, ctx) {
            w.mobys[id].state = 1;
            set_seq(w, id, ctx, 0, 0, 0);
            let solid = w.class(w.mobys[id].o_class).is_some_and(|c| c.has_collision);
            let m = &mut w.mobys[id];
            m.has_collision = solid;
            m.set_pv_i(0, 0x5dc);
            let g = m.group as i32;
            w.changed = true;
            w.poke_group(g, 0);
        },
        _ => {}
    }
}

/// 0x0e8e (`0x44c848`, 1 placed): in state 0, Ratchet in its cuboid `+0x68`[0] (−1 none; `0x3622d0`) sets the
/// game's flag bytes `0x1395cb` and `0x1395cc` (crate::game::GameState::flags `[0x13]` / `[0x14]`), 1. (Its two
/// debug prints are not ported.)
pub fn update_0e8e(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let c = m.pv_i(0);
    if m.state != 0 || c == -1 || !ctx.level.in_cuboid(c, ctx.hero.pos) { return; }
    for k in [0x13, 0x14] {
        if let Some(b) = ctx.game.flags.get_mut(k) { *b = 1; }
    }
    w.mobys[id].set_state(1, -1);
}
