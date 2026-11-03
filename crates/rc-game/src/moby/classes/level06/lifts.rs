//! Level 6's lifts (L06 addresses).
//!
//! * **0x08c9** ([`update_08c9`], `0x402d68`, 1 placed): a lift that goes down and out sideways and back up.

use crate::hero::physics::{dist, sub, V3};
use crate::moby::{Ctx, MobyWorld};

/// `0x365618(m)`: Ratchet on moby `id` — climbing (0x1b) or in the ledge group 3, on its ledge (`0x18a3a8`); else
/// grounded (`0x18a16e` = 0) on it (`0x18a15c`).
fn on(ctx: &Ctx, id: usize) -> bool {
    let h = &*ctx.hero;
    if h.group == 3 || h.state == 0x1b { h.ledge.moby == Some(id) } else { h.air_ticks == 0 && h.ground_moby == Some(id) }
}

/// Its class sound `k` looped into `+0x68`[`at`] unless it plays (`0x3a1950`, `0x3a1c98(k, 4)`).
fn loop_sound(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, k: i32, at: usize) {
    let h = w.mobys[id].pv_i(at);
    if !(h != -1 && ctx.host.sound_playing(h)) {
        let h = ctx.host.play_sound(&w.mobys[id], k, 4);
        w.mobys[id].set_pv_i(at, h);
    }
}

/// The loop at `+0x68`[`at`] stopped (`0x3a1998`), −1.
fn stop_sound(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, at: usize) {
    let h = w.mobys[id].pv_i(at);
    if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
    w.mobys[id].set_pv_i(at, -1);
}

/// 0x08c9 (`0x402d68`): its pvars `+0x20` its platform block (crate::moby::platform, flags | 1), `[0x60]` its place
/// (the top), `[0x80]` 1 when it is at the bottom, `[0x84]` its ticks on the way, `[0x88]` its looped sound, `[0x8c]` a
/// cuboid.
/// * **0**: flags | 1, its place kept, its sound none, at the top, 1.
/// * **1**: Ratchet on it ([`on`]): at the top → 3 (sound 0 looped), else → 2 (sound 1). Then: Ratchet
///   below 55 with it at the top → 3 (sound 0); above 57 with it at the bottom and him in the cuboid → 2 (sound 1).
/// * **2** (back up): at its place (within 0.04): its sound stopped, and with Ratchet off it, at the top, at that
///   place, 1. Else 0.05 a tick along its yaw + 90°, the first 0x68 ticks rising 0.07.
/// * **3** (down): below 50.75: its sound stopped, and with Ratchet off it, at the bottom, 1. Else, unless below 53 with
///   Ratchet in the cuboid and under it, 0.05 a tick along its yaw − 90°, past 0x3c ticks sinking 0.07.
/// * **Each tick**: its move into its platform block (`0x363680` = L00 `0x322668`) and Ratchet carried.
///
/// The heights are the original's constants (L06's world). The states 2 and 3 count their ticks while they move.
pub fn update_08c9(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use std::f32::consts::FRAC_PI_2;
    let (old, old_rot) = (w.mobys[id].pos, w.mobys[id].rot);
    let hz = ctx.hero.pos[2];
    let side = |yaw: f32| -> V3 { [yaw.cos() * 0.05, yaw.sin() * 0.05, 0.0] };
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.set_pv_i(0x5c, m.pv_i(0x5c) | 1);
            m.set_pv_v(0x60, m.pos);
            m.state = 1;
            m.set_pv_i(0x88, -1);
            m.set_pv_i(0x80, 0);
            m.set_pv_i(0x84, 0);
        }
        1 => {
            let up = w.mobys[id].pv_i(0x80);
            if on(ctx, id) {
                if up == 0 {
                    w.mobys[id].set_state(3, -1);
                    loop_sound(w, id, ctx, 0, 0x88);
                } else {
                    w.mobys[id].set_state(2, -1);
                    loop_sound(w, id, ctx, 1, 0x88);
                }
            }
            let c = w.mobys[id].pv_i(0x8c);
            if hz < 55.0 && up == 0 {
                loop_sound(w, id, ctx, 0, 0x88);
                w.mobys[id].set_state(3, -1);
            } else if 57.0 < hz && up == 1 && ctx.level.in_cuboid(c, ctx.hero.pos) {
                loop_sound(w, id, ctx, 1, 0x88);
                w.mobys[id].set_state(2, -1);
            }
        }
        2 => {
            let m = &w.mobys[id];
            if dist(m.pos, m.pv_v(0x60)) <= 0.04 {
                let off = !on(ctx, id);
                stop_sound(w, id, ctx, 0x88);
                if off {
                    let m = &mut w.mobys[id];
                    m.set_pv_i(0x80, 0);
                    m.set_state(1, -1);
                    m.pos = m.pv_v(0x60);
                    m.set_pv_i(0x84, 0);
                }
            } else {
                let m = &mut w.mobys[id];
                let d = side(m.rot[2] + FRAC_PI_2);
                m.pos = [m.pos[0] + d[0], m.pos[1] + d[1], m.pos[2]];
                if m.pv_i(0x84) < 0x68 { m.pos[2] += 0.07; }
                m.set_pv_i(0x84, m.pv_i(0x84) + 1);
            }
        }
        3 => {
            let z = w.mobys[id].pos[2];
            if z < 50.75 {
                let off = !on(ctx, id);
                stop_sound(w, id, ctx, 0x88);
                if off {
                    let m = &mut w.mobys[id];
                    m.set_pv_i(0x80, 1);
                    m.set_state(1, -1);
                    m.set_pv_i(0x84, 0);
                }
            } else {
                let c = w.mobys[id].pv_i(0x8c);
                let blocked = z < 53.0 && ctx.level.in_cuboid(c, ctx.hero.pos) && hz < z;
                if !blocked {
                    let m = &mut w.mobys[id];
                    let d = side(m.rot[2] - FRAC_PI_2);
                    m.pos = [m.pos[0] + d[0], m.pos[1] + d[1], m.pos[2]];
                    if 0x3d <= m.pv_i(0x84) { m.pos[2] -= 0.07; }
                    m.set_pv_i(0x84, m.pv_i(0x84) + 1);
                }
            }
        }
        _ => {}
    }
    let m = &mut w.mobys[id];
    let (pos, rot) = (m.pos, m.rot);
    crate::moby::platform::write(m, 0x20, sub(pos, old), old_rot, rot);
    let flags = m.pv_i(0x20 + 0x3c) as u32;
    crate::moby::carry_hero_with(ctx, id, flags, old, old_rot[2], pos, rot[2]);
}
