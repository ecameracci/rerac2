//! Level 12's small props (L12 addresses; the engine functions named by their L12 addresses with the L00 ones the
//! port knows them by).
//!
//! * **0x0f69** ([`update_0f69`], `0x4177e8`, 22 placed): a wheel turning 1° a tick.
//! * **0x087f** ([`update_087f`], `0x3c0aa8`, 16 placed): the same turn with its looped sound; a hit deletes it.
//! * **0x115f** ([`update_115f`], `0x4254e0`, 2 placed): pulses the glow of its group and of up to four mobys.
//!
//! The engine functions: `0x2f3548` = L00 `0x2efe60` (crate::hero::physics::add_rot), `0x365ed8` = L00 `0x3605f0`
//! the class sound, `0x321700` = L00 `0x31dce8` the hit test, `0x3150e0` the delete, `0x320328` = L00 `0x31c8e0`
//! (crate::moby::classes::props::group_alive), `0x32a698` the pulse ([`pulse`]: `0x2f2b48` = L00 `0x2ef460` the sine,
//! `0x2f36e8` = L00 `0x2f0000` crate::hud::lerp_rgba).

use crate::hero::physics::add_rot;
use crate::moby::{Ctx, MobyWorld};

/// 1° (`0x3c8efa36`): the turn a tick of 0x0f69 and 0x087f.
const TURN: f32 = f32::from_bits(0x3c8e_fa36);

/// 0x0f69 (`0x4177e8`, 22 placed). **0**: update and draw distances 0x20, 1. **Each tick**: its yaw `+0xf8` on by 1°
/// (`0x2f3548`).
pub fn update_0f69(w: &mut MobyWorld, id: usize, _ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    if m.state == 0 {
        m.state = 1;
        m.update_dist = 0x20;
        m.draw_dist = 0x20;
    }
    m.rot[2] = add_rot(m.rot[2], TURN);
}

/// 0x087f (`0x3c0aa8`, 16 placed): `+0x68`[0] its looped sound's handle. **0**: update and draw distances 0x20, no
/// sound (−1), 1. **Each tick**: its yaw on by 1°; its class sound 0 looped (`0x365ed8(0, 4, m)`) when the handle's
/// sound is not playing for it; a hit with the flags 0x10000 deletes it (the record is not let go, the sound not
/// stopped).
pub fn update_087f(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    if m.state == 0 {
        m.state = 1;
        m.update_dist = 0x20;
        m.draw_dist = 0x20;
        m.set_pv_i(0, -1);
    }
    m.rot[2] = add_rot(m.rot[2], TURN);
    let h = m.pv_i(0);
    if !(h != -1 && ctx.host.sound_playing(h)) {
        let h = ctx.host.play_sound(&w.mobys[id], 0, 4);
        w.mobys[id].set_pv_i(0, h);
    }
    if w.hit_message(id).is_some_and(|h| h.tmpl.flags & 0x1_0000 != 0) { w.delete(id); }
}

/// `0x32a698(a, c₀, c₁)`: between `c₀` and `c₁` by `0.5 + 0.5·sin a`.
fn pulse(a: f32, c0: u32, c1: u32) -> u32 { crate::hud::lerp_rgba(a.sin() * 0.5 + 0.5, c0, c1) }

/// 0x115f (`0x4254e0`, 2 placed): pvars `[0..0xc]` / `[0x10..0x1c]` the two colours' bytes (r, g, b, a as words),
/// `[0x20]` the period in ticks, `[0x24]` the count, `[0x28]` / `[0x2c]` the colours, `[0x30..0x3c]` up to four
/// mobys (−1 ends them). **0**: the colours packed, the count of mobys before the first −1; 2 with any (1 without;
/// both print a debug line). **1**: with two of its group (`+0x21`) alive or more (`0x320328`), each drawn member's
/// glow word `+0x78` the pulse ([`pulse`]) at `(frame % period) / period · 2π − π`; then as 2. **2**: the same for
/// each drawn moby of its list.
pub fn update_115f(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    match m.state {
        0 => {
            let pack = |b: usize| (0..4).fold(0u32, |c, k| c | (m.pv_i(b + 4 * k) as u32) << (8 * k));
            let (c0, c1) = (pack(0), pack(0x10));
            let n = (0..4).take_while(|&k| m.pv_i(0x30 + 4 * k) != -1).count() as i32;
            let m = &mut w.mobys[id];
            m.set_pv_i(0x28, c0 as i32);
            m.set_pv_i(0x2c, c1 as i32);
            m.set_pv_i(0x24, n);
            m.set_state(if n == 0 { 1 } else { 2 }, -1);
            return;
        }
        1 | 2 => {}
        _ => return,
    }
    let period = m.pv_i(0x20);
    if period == 0 { return; }
    let (c0, c1) = (m.pv_i(0x28) as u32, m.pv_i(0x2c) as u32);
    let a = (ctx.game.frame % period) as f32 * std::f32::consts::TAU / period as f32 - std::f32::consts::PI;
    let glow = pulse(a, c0, c1);
    let mut list: Vec<usize> = Vec::new();
    if m.state == 1 && super::super::props::group_alive(w, m.group as u8 as i32) >= 2 {
        list.extend(w.groups.get(m.group as u8 as usize).into_iter().flatten().copied());
    }
    let n = m.pv_i(0x24);
    list.extend((0..n.clamp(0, 4) as usize).filter_map(|k| usize::try_from(m.pv_i(0x30 + 4 * k)).ok()));
    for i in list {
        if let Some(o) = w.mobys.get_mut(i) { if o.drawn != 0 { o.glow = glow; } }
    }
}
