//! Level 4's moving platforms (L04 addresses).
//!
//! * **0x0968** ([`update_0968`], `0x3e94c8`, 4 placed): a turning wheel of four platforms.
//! * **0x09eb** ([`update_09eb`], `0x3ee890`, 4 placed): a platform that slides to a point and back.
//! * **0x09ea** ([`update_09ea`], `0x3ee0f8`, 3 placed): a lift between two heights.
//! * **0x09bd** ([`update_09bd`], `0x3ecd90`, 3 placed): a boat that ferries between two points.

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

/// `0x3476d8(m)`: Ratchet on moby `id` — climbing (0x1b) or in the ledge group 3 with the ledge on it, else on the
/// ground on it.
pub(super) fn hero_on(ctx: &Ctx, id: usize) -> bool {
    let h = &*ctx.hero;
    if h.group == 3 || h.state == 0x1b { h.ledge.moby == Some(id) } else { h.air_ticks == 0 && h.ground_moby == Some(id) }
}

/// 0x09ea's call to come to Ratchet: unless (its mission not done, its uid not gone since the checkpoint and its
/// save count 0) and it was never ridden here (`+0x9c`).
fn lift_comes(w: &MobyWorld, id: usize, ctx: &Ctx) -> bool {
    let m = &w.mobys[id];
    if m.pv_i(0x9c) == 1 { return true; }
    let k = ctx.game.level.max(0) as usize * 0x10 + m.mission as usize;
    if ctx.game.missions.get(k).is_some_and(|b| b & 0x80 != 0) { return true; }
    if ctx.game.uid_dead.get(m.uid.max(0) as usize).copied().unwrap_or(0) != 0 { return true; }
    ctx.game.uid_count(m.uid) != 0
}

/// 0x09ea's ride toward `to` (`0x340960` = L00 `0x31ed78`, crate::hero::physics::ease: 1/514, 1/600, at most 0.1 a
/// tick; its speed `+0xa0`), its ticks `+0x84` counted.
fn lift_ease(w: &mut MobyWorld, id: usize, to: f32) {
    let m = &mut w.mobys[id];
    let (mut z, mut v) = (m.pos[2], m.pv_f(0xa0));
    crate::hero::physics::ease(to, f32::from_bits(0x3afe_dcbb), f32::from_bits(0x3ada_740e), f32::from_bits(0x3dcc_ccce), &mut z, &mut v);
    m.pos[2] = z;
    m.set_pv_f(0xa0, v);
    m.set_pv_i(0x84, m.pv_i(0x84) + 1);
}

/// 0x09ea (`0x3ee0f8`, 3 placed): a lift between its place and the height `+0x8c` (starting at the top with
/// `+0x90` ≠ 0). Its words: the platform block `+0x20` (flag 1), home `+0x60`, which end it is at `+0x80` (0 the
/// first, `+0x94`; 1 the second, `+0x98`), its ride ticks `+0x84`, its looped sound `+0x88`, ridden `+0x9c`, its speed
/// `+0xa0`, a cuboid `+0xa4`.
/// * **0**: the ends set (`+0x94` its own height and `+0x98` `+0x8c`, or swapped with `+0x90`), no sound; 1.
/// * **1**: Ratchet on it ([`hero_on`]): ridden, its uid's save count up (`0x19b4a8`'s nibble, at most 7), off to the
///   other end (3 from the first, 2 from the second), its looped sound 0. Ratchet more than 3 below the first end's
///   lift (or above the second's) and it comes ([`lift_comes`]): its sound, to him (3 / 2).
/// * **2**: toward `+0x94` ([`lift_ease`]); within 0.04 and Ratchet off it: at the first end (0), the sound stopped, 1.
/// * **3**: toward `+0x98`, but not while Ratchet stands in the cuboid `+0xa4` under it within 2.8; at or past it with
///   him off: the second end (1), 1; the sound stopped.
/// * Every tick its platform block written (`0x3445f0`, crate::moby::platform) and Ratchet carried.
pub fn update_09ea(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (old, old_rot) = (w.mobys[id].pos, w.mobys[id].rot);
    let hz = ctx.hero.pos[2];
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.set_pv_i(0x9c, 0);
            m.set_pv_i(0x5c, m.pv_i(0x5c) | 1);
            let p = m.pos;
            m.set_pv_v(0x60, p);
            m.state = 1;
            let (own, other) = (p[2], m.pv_f(0x8c));
            if m.pv_i(0x90) == 0 {
                m.set_pv_f(0x94, own);
                m.set_pv_f(0x98, other);
                m.set_pv_i(0x80, 0);
            } else {
                m.set_pv_i(0x80, 1);
                m.set_pv_f(0x94, other);
                m.set_pv_f(0x98, own);
            }
            m.set_pv_i(0x84, 0);
            m.set_pv_i(0x88, -1);
        }
        1 => {
            if hero_on(ctx, id) {
                let uid = w.mobys[id].uid;
                ctx.game.bump_uid(uid);
                let m = &mut w.mobys[id];
                m.set_pv_i(0x9c, 1);
                let s = if m.pv_i(0x80) == 0 { 3 } else { 2 };
                m.set_state(s, -1);
                keep_loop(w, id, ctx, 0x88);
            }
            let m = &w.mobys[id];
            let z = m.pos[2];
            let far = 3.0 < (z - hz).abs();
            if far && m.pv_i(0x80) == 0 && hz < z && lift_comes(w, id, ctx) {
                keep_loop(w, id, ctx, 0x88);
                w.mobys[id].set_state(3, -1);
            } else if far && w.mobys[id].pv_i(0x80) == 1 && z < hz && lift_comes(w, id, ctx) {
                keep_loop(w, id, ctx, 0x88);
                w.mobys[id].set_state(2, -1);
            }
        }
        2 => {
            let m = &w.mobys[id];
            let to = [m.pos[0], m.pos[1], m.pv_f(0x94)];
            if crate::hero::physics::dist(m.pos, to) <= 0.04 {
                if !hero_on(ctx, id) {
                    w.mobys[id].set_pv_i(0x80, 0);
                    stop_loop(w, id, ctx, 0x88);
                    let m = &mut w.mobys[id];
                    m.set_state(1, -1);
                    m.pos = to;
                    m.set_pv_i(0x84, 0);
                } else {
                    stop_loop(w, id, ctx, 0x88);
                }
            } else {
                lift_ease(w, id, to[2]);
            }
        }
        3 => {
            let m = &mut w.mobys[id];
            if m.once & 1 == 0 { m.once |= 1; }
            let (z, other) = (m.pos[2], m.pv_f(0x98));
            if z <= other {
                if !hero_on(ctx, id) && z <= other {
                    w.mobys[id].set_pv_i(0x80, 1);
                    stop_loop(w, id, ctx, 0x88);
                    let m = &mut w.mobys[id];
                    m.set_state(1, -1);
                    m.set_pv_i(0x84, 0);
                } else {
                    stop_loop(w, id, ctx, 0x88);
                }
            } else if z - hz < 2.8 && hz < z {
                if !ctx.level.in_cuboid(w.mobys[id].pv_i(0xa4), ctx.hero.pos) { lift_ease(w, id, other); }
            } else {
                lift_ease(w, id, other);
            }
        }
        _ => {}
    }
    let m = &mut w.mobys[id];
    let (pos, rot) = (m.pos, m.rot);
    crate::moby::platform::write(m, 0x20, sub(pos, old), old_rot, rot);
    crate::moby::carry_hero(ctx, id, old, old_rot[2], pos, rot[2]);
    w.changed = true;
}

/// The path's point `k`.
fn path_point(ctx: &Ctx, path: i32, k: usize) -> Option<V3> {
    usize::try_from(path).ok().and_then(|p| ctx.level.paths.get(p)).and_then(|p| p.get(k)).map(|q| [q[0], q[1], q[2]])
}

/// `0x3ed5f8`: the boat on the water — at its kept place `+0x1b0`; with Ratchet standing on it (his first tick there
/// stops its sway's speeds and lets them speed up 20 times as fast) his place in its frame tips it (across: −0.333 of
/// his offset to the side and 1.5 of it ahead) and sinks it 0.333; the tips and the sink eased (`0x340960`: 0.4·(1 +
/// |tip|)/3600, 2/3600 for the sink, at most 0.1) into its angles and height `+0x1c4..+0x1cc` (speeds `+0x1d0..`);
/// over them its rocking: 2° at 100°/s and 70°/s on its two tilts (`+0x1a8` / `+0x1ac`), 0.1 at 60°/s on its height
/// (`+0x1c0`).
fn sway(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let h = &*ctx.hero;
    let m = &mut w.mobys[id];
    m.pos = m.pv_v(0x1b0);
    let mut off = [0.0f32; 3];
    let mut k = 1.0;
    let on = h.ground_moby == Some(id) && h.grounded_ticks != 0;
    if on {
        if h.grounded_ticks == 1 { for o in [0x1d0, 0x1d4, 0x1d8] { m.set_pv_f(o, 0.0); } k = 20.0; }
        let d = sub(h.pos, m.pos);
        let r = m.rows;
        let l = [r[0][0] * d[0] + r[0][1] * d[1] + r[0][2] * d[2], r[1][0] * d[0] + r[1][1] * d[1] + r[1][2] * d[2]];
        off[0] -= l[1] * 0.333;
        off[1] += l[0] * 1.5;
    }
    for i in 0..2 {
        let a = 0.4 * (off[i].abs() + 1.0) * 0.000_277_777_78;
        let t = if on { 0.1 * off[i] } else { 0.0 };
        let (mut x, mut v) = (m.pv_f(0x1c4 + 4 * i), m.pv_f(0x1d0 + 4 * i));
        crate::hero::physics::ease(t, a * k, a, 0.1, &mut x, &mut v);
        m.set_pv_f(0x1c4 + 4 * i, x);
        m.set_pv_f(0x1d0 + 4 * i, v);
    }
    let a = 2.0 * 0.000_277_777_78;
    let t = if on { f32::from_bits(0xbeaa_7efa) } else { 0.0 };
    let (mut x, mut v) = (m.pv_f(0x1cc), m.pv_f(0x1d8));
    crate::hero::physics::ease(t, a * k, a, 0.1, &mut x, &mut v);
    m.set_pv_f(0x1cc, x);
    m.set_pv_f(0x1d8, v);
    let deg = 0.017_453_294;
    m.rot[0] = m.pv_f(0x1c4);
    m.rot[1] = m.pv_f(0x1c8);
    m.pos[2] += m.pv_f(0x1cc);
    for (ph, rate, amp, axis) in [(0x1a8, 100.0, 2.0 * deg, 0), (0x1ac, 70.0, 2.0 * deg, 1), (0x1c0, 60.0, 0.1, 2)] {
        let p = add_rot(m.pv_f(ph), rate * 0.000_290_888_25);
        m.set_pv_f(ph, p);
        if axis == 2 { m.pos[2] += amp * p.sin() } else { m.rot[axis] += amp * p.sin() }
    }
}

/// 0x09bd's run to its path's point `k` (states 5 / 6): unless Ratchet stands within 2 of its lever and not on it,
/// the framework's move from its kept place toward the point (`0x34fa80`), kept; within 0.5: `next`, its sound 1, and
/// the walk set up back (`last`, `first`).
fn boat_run(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, lever: Option<usize>, k: usize, next: u8, back: (i32, i32)) {
    let path = w.mobys[id].pv_i(0x184);
    let Some(p) = path_point(ctx, path, k) else { return };
    let near_lever = lever.is_some_and(|l| crate::hero::physics::dist2(w.mobys[l].pos, ctx.hero.pos) < 2.0);
    if near_lever && ctx.hero.ground_moby != Some(id) { return; }
    let base = w.mobys[id].pv_v(0x1b0);
    w.mobys[id].pos = base;
    let y = crate::hero::physics::atan2(p[0] - base[0], p[1] - base[1]);
    crate::moby::ai::move_step(w, id, ctx, y, p);
    let now = w.mobys[id].pos;
    w.mobys[id].set_pv_v(0x1b0, now);
    if crate::hero::physics::dist2(now, p) <= 0.5 {
        w.mobys[id].set_state(next, -1);
        ctx.host.play_sound(&w.mobys[id], 1, 0);
        if next == 3 { if let Some(l) = lever { w.mobys[l].once &= !4; } }
        let o = MV_BOAT;
        if let Some(pts) = usize::try_from(path).ok().and_then(|i| ctx.level.paths.get(i)) {
            let n = pts.len() as i16;
            let m = &mut w.mobys[id];
            m.set_pv_i(o + 0xc4, path + 1);
            m.set_pv_s(o + 0xca, if back.0 == -1 { n - 1 } else { back.0 as i16 });
            m.set_pv_s(o + 0xc8, back.1 as i16);
        }
    }
}

/// 0x09bd's movement block.
const MV_BOAT: usize = 0x50;

/// 0x09bd (`0x3ecd90`, 3 placed): a boat that ferries between its path's first two points when its lever moby
/// `+0x180` is hooked (its once bit 2) or Ratchet stands in the cuboid at its end (`+0x190` at point 0, `+0x194` at
/// point 1). Its words: the movement block `+0x50`, the platform block `+0x140`, its path `+0x184`, speed `+0x188`, turn
/// speed `+0x18c`, its kept place `+0x1b0`, its sway ([`sway`], `0x3ed5f8`), its looped sound `+0x1dc`.
/// * **0**: without the cuboid `+0x190` deleted; the movement block (radius 0.05, height 0.5, steps 100, slope 95°,
///   turn brake 40″, the walk 2·speed/s² up to speed/s, no gravity, the turn before walking 360°), facing point 1,
///   its rocking phases at random, its place kept; 3 (no lever: 1, nothing).
/// * **2**: facing point 1 in place; the lever hooked: its sound 1, 6.
/// * **3** (at point 0) / **4** (at point 1): turning toward the other point (`0x33d440`: 240″ / 240″ / 6°); the lever
///   hooked (its sound 1) or Ratchet in that end's cuboid: 6 / 5.
/// * **6** / **5**: the run ([`boat_run`]) to point 1 / 0, then 4 / 3 (at point 0 the lever unhooked).
/// * Every tick: the sway, its looped sound 0, its platform block (`0x3445f0`) and Ratchet carried, the lever 2.2
///   ahead and 0.5 up its frame, and its wake: three puffs 0.2 under it (`0x359048`: two `0xdf204080` 630000 that
///   fade in 15 ticks, one white `0xdfffffff` 315000 for 25, sinking 0.2 a tick).
pub fn update_09bd(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (old, old_rot) = (w.mobys[id].pos, w.mobys[id].rot);
    let lever = usize::try_from(w.mobys[id].pv_i(0x180)).ok().filter(|&l| l < w.mobys.len());
    let hooked = lever.is_some_and(|l| w.mobys[l].once & 4 != 0);
    let path = w.mobys[id].pv_i(0x184);
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.set_pv_i(0x18, MV_BOAT as i32);
            if m.pv_i(0x190) == -1 {
                w.delete(id);
                return;
            }
            let o = MV_BOAT;
            let speed = m.pv_f(0x188);
            m.set_pv_f(o, f32::from_bits(0x3d4c_cccd));
            m.set_pv_f(o + 4, 0.5);
            m.set_pv_f(o + 0xc, 100.0);
            m.set_pv_f(o + 0x10, f32::from_bits(0x3fd4_3b68));
            m.set_pv_f(o + 0x20, (speed + speed) * 0.000_277_777_78);
            m.set_pv_f(o + 0x18, f32::from_bits(0x394b_587b));
            m.set_pv_f(o + 8, 100.0);
            m.set_pv_f(o + 0x30, f32::from_bits(0x40c9_0fdc));
            m.set_pv_f(o + 0x28, speed * 0.016_666_668);
            m.set_pv_i(0x1dc, -1);
            m.set_pv_f(o + 0x2c, 0.0);
            m.set_pv_f(o + 0x14, 0.0);
            m.set_pv_f(o + 0x1c, 0.0);
            if let Some(p) = path_point(ctx, path, 1) { let q = m.pos; m.rot[2] = crate::hero::physics::atan2(p[0] - q[0], p[1] - q[1]); }
            for o in [0x1a8, 0x1ac, 0x1c0] { let a = ctx.game.rng.rand_angle(); w.mobys[id].set_pv_f(o, a); }
            let m = &mut w.mobys[id];
            let p = m.pos;
            m.set_pv_v(0x1b0, p);
            let s = if m.pv_i(0x180) == -1 { 1 } else { 3 };
            m.set_state(s, -1);
        }
        2 => {
            if !hooked {
                if let Some(p) = path_point(ctx, path, 1) {
                    let q = w.mobys[id].pos;
                    let y = crate::hero::physics::atan2(p[0] - q[0], p[1] - q[1]);
                    crate::moby::ai::move_step(w, id, ctx, y, q);
                }
            } else {
                ctx.host.play_sound(&w.mobys[id], 1, 0);
                w.mobys[id].set_state(6, -1);
            }
        }
        3 | 4 => {
            let at_first = w.mobys[id].state == 3;
            let cub = w.mobys[id].pv_i(if at_first { 0x190 } else { 0x194 });
            if hooked || ctx.level.in_cuboid(cub, ctx.hero.pos) {
                if hooked { ctx.host.play_sound(&w.mobys[id], 1, 0); }
                w.mobys[id].set_state(if at_first { 6 } else { 5 }, -1);
            } else if let Some(p) = path_point(ctx, path, if at_first { 1 } else { 0 }) {
                let q = w.mobys[id].pos;
                let y = crate::hero::physics::atan2(p[0] - q[0], p[1] - q[1]);
                crate::moby::ai::turn_to(&mut w.mobys[id], y, f32::from_bits(0x3a98_825c), f32::from_bits(0x3a98_825c), f32::from_bits(0x3dd6_7752), 0x18c);
            }
        }
        5 => boat_run(w, id, ctx, lever, 0, 3, (1, 0)),
        6 => boat_run(w, id, ctx, lever, 1, 4, (0, 1)),
        _ => {}
    }
    sway(w, id, ctx);
    keep_loop(w, id, ctx, 0x1dc);
    let m = &mut w.mobys[id];
    m.rows = euler_rows(m.rot);
    let (pos, rot, rows) = (m.pos, m.rot, m.rows);
    crate::moby::platform::write(m, 0x140, sub(pos, old), old_rot, rot);
    crate::moby::carry_hero(ctx, id, old, old_rot[2], pos, rot[2]);
    w.changed = true;
    if let Some(l) = lever {
        let lm = &mut w.mobys[l];
        lm.pos = crate::hero::physics::add(crate::hero::physics::add(pos, crate::hero::physics::scale(rows[0], 2.2)), crate::hero::physics::scale(rows[2], 0.5));
        w.rebuild(l, &*ctx.host);
    }
    let at = [pos[0], pos[1], pos[2] - 0.2];
    let vel = [0.0, 0.0, -0.2];
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let rng = &mut ctx.game.rng;
    let mut spin = rng.randi(0x10);
    if rng.randi(2) != 0 { spin = -spin; }
    use crate::particles::rec;
    let first = crate::particles::puff(sys, rng, 0.1, 1.0, f32::from_bits(0x3f66_6666), 630_000.0, at, spin, vel, 0xdf20_4080, 0);
    if let Some(i) = first {
        let r = &mut sys.pool.recs[i];
        rec::set_i16(r, 0xa, 0xf);
        r[9] = 0x74;
        rec::set_u32(r, 0x24, 2);
        r[0x2a] = 0xdf;
        r[0x2b] = 0xf;
    }
    // The second's life and frame bytes go to the first's record (as the game writes them).
    if let Some(j) = crate::particles::puff(sys, rng, 0.1, 1.0, f32::from_bits(0x3f66_6666), 630_000.0, at, spin, vel, 0xdf20_4080, 0) {
        if let Some(i) = first { let r = &mut sys.pool.recs[i]; rec::set_i16(r, 0xa, 0xf); r[9] = 0x74; }
        let b = first.map_or(0, |i| sys.pool.recs[i][0xa]);
        let r = &mut sys.pool.recs[j];
        rec::set_u32(r, 0x24, 2);
        r[0x2a] = 0xdf;
        r[0x2b] = b;
    }
    if let Some(k) = crate::particles::puff(sys, rng, 0.05, 1.0, f32::from_bits(0x3f78_51ec), 315_000.0, at, 0x10, vel, 0xdfff_ffff, 0) {
        let roll = rng.randi(0xff) as u8;
        let b = first.map_or(0, |i| sys.pool.recs[i][0xa]);
        let r = &mut sys.pool.recs[k];
        r[9] = 0x74;
        rec::set_i16(r, 0xa, 0x19);
        r[8] = roll;
        r[0x2a] = 0xdf;
        rec::set_u32(r, 0x24, 2);
        r[0x2b] = b;
    }
}
