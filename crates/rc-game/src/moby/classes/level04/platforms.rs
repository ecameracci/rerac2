//! Level 4's moving platforms (L04 addresses).
//!
//! * **0x0968** ([`update_0968`], `0x3e94c8`, 4 placed): a turning wheel of four platforms.
//! * **0x09eb** ([`update_09eb`], `0x3ee890`, 4 placed): a platform that slides to a point and back.

use crate::hero::physics::{add_rot, dist2, euler_rows, set_len, sub, sub_rot, turn_ease, V3};
use crate::moby::{countdown, Ctx, MobyWorld};

/// Its looped sound 0 (`0x383ff0(0, 4, m)`) kept going (a handle no longer playing is restarted); the handle at `off`.
fn keep_loop(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, off: usize) {
    let h = w.mobys[id].pv_i(off);
    if h == -1 || !ctx.host.sound_playing(h) {
        let h = ctx.host.play_sound(&w.mobys[id], 0, 4);
        w.mobys[id].set_pv_i(off, h);
    }
}

/// The looped sound at `off` stopped (`0x383cf0`) while it plays; the handle −1.
fn stop_loop(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, off: usize) {
    let h = w.mobys[id].pv_i(off);
    if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
    w.mobys[id].set_pv_i(off, -1);
}

/// 0x0968's platform `p` moved to `to` (and, with `yaw`, turned to it): its platform block `+0x20` written
/// (`0x3445f0` = L00 `0x322668`, crate::moby::platform: the turn reported only when `turned`), Ratchet carried
/// (crate::moby::carry_hero), its matrix rebuilt (it has no update of its own).
fn move_platform(w: &mut MobyWorld, p: usize, ctx: &mut Ctx, to: V3, yaw: Option<f32>, turned: bool) {
    let m = &mut w.mobys[p];
    let (old, old_rot) = (m.pos, m.rot);
    m.pos = to;
    if let Some(y) = yaw { m.rot[2] = y; }
    let new_rot = if turned { m.rot } else { old_rot };
    crate::moby::platform::write(m, 0x20, sub(to, old), old_rot, new_rot);
    m.drive = true;
    let (yaw0, yaw1) = (old_rot[2], new_rot[2]);
    crate::moby::carry_hero(ctx, p, old, yaw0, to, yaw1);
    w.rebuild(p, &*ctx.host);
}

/// The four platforms (`+0x68`[0..4], moby indices) at the wheel's joints 0..3; with `turn` each at the wheel's yaw
/// + 0.5°, the one `+0x68`[9] names reporting the turn.
fn place_platforms(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, turn: bool) {
    let which = w.mobys[id].pv_i(0x24);
    let yaw = add_rot(w.mobys[id].rot[2], f32::from_bits(0x3c0e_fa36));
    for k in 0..4 {
        let Some(p) = usize::try_from(w.mobys[id].pv_i(4 * k)).ok().filter(|&p| p < w.mobys.len()) else { continue };
        let Some(j) = ctx.host.joint(&w.mobys[id], k) else { continue };
        move_platform(w, p, ctx, j, turn.then_some(yaw), turn && which == k as i32);
    }
}

/// 0x0968 (`0x3e94c8`): a wheel that turns a quarter at a time, its four platforms on its joints. Its words: the
/// platforms `+0x68`[0..4], `+0x10` turning the other way (≠ 0), its height `+0x14`, the frame of its sequence it
/// stops at next `+0x18` (30, 60, 90, 0), the pause `+0x1c` (from `+0x20`), the platform whose turn is reported
/// `+0x24`, its looped sound `+0x28`, its turn speed `+0x2c`, the yaw it turns to `+0x30`.
/// * **0**: update and draw distances 0x80 (the platforms' too, and their block's flag 1), no sound, its height
///   kept, the first stop at frame 30, platform 2 reported, the yaw a quarter on (−90° turning the other way),
///   sequence 1 (the other way: 2) at a third of its speed; 1.
/// * **1**: its looped sound 0; the yaw eased to (`0x340e18`: 40″, 40″, 4°), its rows; the platforms on its joints,
///   turning with it; at its stop frame the pause (`+0x1c` = `+0x20`), its sequence stopped, the next stop 30 on
///   (120 → 0), 2, its turn speed 0.
/// * **2**: the sound stopped; the platforms on its joints; the pause's last 25 ticks it dips (`0.25·sin(t·7.2°)`
///   under its height); over: the next platform reported (past 3: 0, the game's message), the next quarter, 1, at
///   its height, a third of its sequence's speed.
pub fn update_0968(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let quarter = std::f32::consts::FRAC_PI_2;
    let next_yaw = |m: &crate::moby::Moby| if m.pv_i(0x10) != 0 { sub_rot(m.rot[2], quarter) } else { add_rot(m.rot[2], quarter) };
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.update_dist = 0x80;
            m.draw_dist = 0x80;
            m.set_pv_i(0x28, -1);
            m.set_pv_f(0x14, m.pos[2]);
            m.set_pv_f(0x18, 30.0);
            m.set_pv_i(0x24, 2);
            let y = next_yaw(m);
            m.set_pv_f(0x30, y);
            ctx.host.set_anim(&w.mobys[id], 1, 0, 0);
            w.mobys[id].set_state(1, -1);
            for k in 0..4 {
                if let Some(p) = usize::try_from(w.mobys[id].pv_i(4 * k)).ok().filter(|&p| p < w.mobys.len()) {
                    let pm = &mut w.mobys[p];
                    pm.set_pv_i(0x5c, pm.pv_i(0x5c) | 1);
                    pm.draw_dist = 0x80;
                    pm.update_dist = 0x80;
                }
            }
            if w.mobys[id].pv_i(0x10) != 0 { ctx.host.set_anim(&w.mobys[id], 2, 0, 0); }
            ctx.host.set_anim_speed(&w.mobys[id], f32::from_bits(0x3eaa_aaab));
        }
        1 => {
            let t = ctx.host.frame_time(&w.mobys[id]);
            keep_loop(w, id, ctx, 0x28);
            let m = &mut w.mobys[id];
            let (mut a, mut v) = (m.rot[2], m.pv_f(0x2c));
            turn_ease(m.pv_f(0x30), f32::from_bits(0x394b_587b), f32::from_bits(0x394b_587b), f32::from_bits(0x3d8e_fa37), &mut a, &mut v, 0);
            m.rot[2] = a;
            m.set_pv_f(0x2c, v);
            m.rows = euler_rows(m.rot);
            let turn = m.pv_i(0x10) != 0;
            place_platforms(w, id, ctx, turn);
            let m = &mut w.mobys[id];
            if t == m.pv_f(0x18) {
                m.set_pv_i(0x1c, m.pv_i(0x20));
                ctx.host.set_anim_speed(&w.mobys[id], 0.0);
                let m = &mut w.mobys[id];
                let next = m.pv_f(0x18) + 30.0;
                m.set_pv_f(0x18, if 120.0 <= next { 0.0 } else { next });
                m.set_state(2, -1);
                m.set_pv_f(0x2c, 0.0);
            }
        }
        2 => {
            stop_loop(w, id, ctx, 0x28);
            let m = &mut w.mobys[id];
            let mut t = m.pv_i(0x1c);
            let out = countdown(&mut t);
            m.set_pv_i(0x1c, t);
            if !out {
                if t < 0x19 { m.pos[2] = m.pv_f(0x14) - (t as f32 * 0.125_663_61).sin() * 0.25; }
            } else {
                let k = m.pv_i(0x24) + 1;
                m.set_pv_i(0x24, if 3 < k { 0 } else { k });
                let y = next_yaw(m);
                m.set_pv_f(0x30, y);
                m.set_state(1, -1);
                m.pos[2] = m.pv_f(0x14);
                ctx.host.set_anim_speed(&w.mobys[id], f32::from_bits(0x3eaa_aaab));
            }
            place_platforms(w, id, ctx, false);
        }
        _ => {}
    }
    let m = &mut w.mobys[id];
    if m.state_ticks != -1 { m.state_ticks += 1; }
    w.changed = true;
}

/// The platform's step toward `to`, 0.07 a tick (`0x30fda0(0.07, ·, to − pos)`; it writes no platform block, so
/// nothing rides it along); its looped sound with `+0x38` = 1.
/// Within `near` across: there, `next` its state, its sound 1 and the loop stopped (with `+0x38` = 1).
fn slide(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, to: V3, near: f32, snap: bool, next: u8) {
    let m = &mut w.mobys[id];
    let p = m.pos;
    m.pos = crate::hero::physics::add(p, set_len(sub(to, p), 0.07));
    w.changed = true;
    let sound = w.mobys[id].pv_i(0x38) == 1;
    if sound { keep_loop(w, id, ctx, 0x34); }
    if near <= dist2(w.mobys[id].pos, to) { return; }
    if snap { w.mobys[id].pos = to; }
    w.mobys[id].state = next;
    if sound {
        ctx.host.play_sound(&w.mobys[id], 1, 0);
        stop_loop(w, id, ctx, 0x34);
    }
}

/// Whether moby `i`'s once bit 2 is set (`0x34d798`: its hook).
fn hooked(w: &MobyWorld, i: i32) -> bool { usize::try_from(i).ok().and_then(|i| w.mobys.get(i)).is_some_and(|m| m.once & 4 != 0) }

/// 0x09eb (`0x3ee890`): a platform that slides to (340.57, 219.92) at its own height and back. Its words: the mobys
/// that send it `+0x68`[0] / [1] (watched, kept at `+0x0c` as the index), the wait `+0x08`, home `+0x10`, the far
/// point `+0x20`, its looped sound `+0x34`, its sounds on (`+0x38` = 1).
/// * **0**: watching the first moby, home its place, the far point, no sound; 1.
/// * **1**: the watched moby hooked (its once bit 2) and the wait out: 3, watching the second.
/// * **3**: to the far point ([`slide`]); within 0.4: 2.
/// * **2**: the second moby hooked: shown, solid as its class, 4.
/// * **4**: back home; within 0.1 at home, 5 (nothing more).
pub fn update_09eb(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.set_pv_i(0xc, m.pv_i(0));
            m.state = 1;
            let p = m.pos;
            m.set_pv_v(0x10, p);
            m.set_pv_f(0x20, f32::from_bits(0x43aa_48f6));
            m.set_pv_f(0x24, f32::from_bits(0x435b_eb85));
            m.set_pv_i(0x34, -1);
            m.set_pv_f(0x28, p[2]);
        }
        1 => {
            if hooked(w, w.mobys[id].pv_i(0xc)) {
                let m = &mut w.mobys[id];
                let mut t = m.pv_i(8);
                let out = countdown(&mut t);
                m.set_pv_i(8, t);
                if out {
                    m.state = 3;
                    m.set_pv_i(0xc, m.pv_i(4));
                }
            }
        }
        2 => {
            if hooked(w, w.mobys[id].pv_i(0xc)) {
                let solid = w.class(w.mobys[id].o_class).is_some_and(|c| c.has_collision);
                let m = &mut w.mobys[id];
                m.mode &= 0xffbe;
                m.state = 4;
                m.has_collision = solid;
            }
        }
        3 => {
            let to = w.mobys[id].pv_v(0x20);
            slide(w, id, ctx, to, 0.4, false, 2);
        }
        4 => {
            let home = w.mobys[id].pv_v(0x10);
            slide(w, id, ctx, home, 0.1, true, 5);
        }
        _ => {}
    }
}
