//! Level 13's gates and turning walkways (L13 addresses; the engine functions named by their L13 addresses with the
//! L00 ones the port knows them by).
//!
//! * **0x0d97** / **0x0d95** ([`update_0d97`], `0x3ffe88` / `0x3ffc20`, 32 placed each): the two halves of a gate
//!   that swing open as Ratchet comes near.
//! * **0x0d65** ([`update_0d65`], `0x3ff938`, 32 placed): a walkway section that rolls over with Ratchet on it on
//!   its magnetic walkway.
//! * **0x0cd8** ([`update_0cd8`], `0x3f1250`, 6 placed): swings to and fro a quarter turn.
//!
//! The engine functions: `0x3294a8` = L00 `0x31f230` (crate::hero::physics::turn_ease), `0x2f9718` / `0x2f96e0` the
//! distance across / in 3D, `0x32bb68` a vector turned about an axis (`0x32a110` + `0x32ba90`, crate::quat),
//! `0x32e568` Ratchet standing on the moby ([`stands_on`]), `0x32b7c0` = L00 `0x321858`
//! (crate::level::LevelData::in_cuboid), `0x2cf1f8` = L00 `0x2c4560` (Ratchet's state, crate::hero::Hero::request_state),
//! `0x3337d8` the once-flag (crate::moby::Moby::first), `0x36a110` the class sound.

use crate::hero::physics::{add, dist, dist2, scale, set_len, turn_ease};
use crate::moby::{Ctx, MobyWorld};

/// `0x18a168`: Ratchet on a magnetic walkway with the Gravity Boots.
fn on_walkway(ctx: &Ctx) -> bool { ctx.hero.surf.boots == 1 }

/// `0x32e568(m)`: Ratchet standing on moby `id`: hanging or climbing (group 3, state 0x1b) on its ledge (`0x18a3a8`),
/// else on the ground (`0x18a16e` = 0) with it under his feet (`0x18a15c`).
fn stands_on(ctx: &Ctx, id: usize) -> bool {
    let h = &*ctx.hero;
    if h.group == 3 || h.state == 0x1b { h.ledge.moby == Some(id) } else { h.air_ticks == 0 && h.ground_moby == Some(id) }
}

/// The gate halves' turns: 0.1° a tick² both ways, at most 2° a tick (`0x3294a8(t, 0.001745, 0.001745, 0.03491)`).
const GATE_ACCEL: f32 = f32::from_bits(0x3ae4_c38a);
const GATE_MAX: f32 = f32::from_bits(0x3d0e_fa37);

/// 0x0d97 (`0x3ffe88`) / 0x0d95 (`0x3ffc20`): a gate half. Its pvars: `[0]` opens for Ratchet near across, `[4]` its
/// turn speed, `[8]` opens for Ratchet near on a magnetic walkway.
/// * **0**: with `[0]`: Ratchet within 8 across (`0x2f9718`) → 1; else with `[8]`: within 8 (`0x2f96e0`) on the
///   walkway (`0x18a168` = 1) → 2; else (neither) its word `+0x98` 0, 3.
/// * **1**: its yaw eased ([`GATE_ACCEL`]) toward 135° (0x0d95: −45°); **2**: toward 90° (−90°); **3**: nothing.
/// * **4**: 11 ticks along its +y row a sixth a tick (0x0d95: −y), **5** 31 ticks still, **6** 11 ticks back, 0 (its
///   tick count `+0x96`, set directly; nothing here enters 4).
///
/// Not ported: the word `+0x98` (nothing the port has reads it).
pub fn update_0d97(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let left = w.mobys[id].o_class == 0x0d95;
    let hero = ctx.hero.pos;
    let m = &mut w.mobys[id];
    if m.state == 0 {
        if m.pv_i(0) != 0 {
            if dist2(m.pos, hero) < 8.0 { m.state = 1; }
        } else if m.pv_i(8) != 0 {
            if dist(m.pos, hero) < 8.0 && on_walkway(ctx) { m.state = 2; }
        } else {
            m.state = 3;
        }
    }
    let sign = if left { -1.0 } else { 1.0 };
    let target = match (m.state, left) {
        (1, false) => Some(f32::from_bits(0x4016_cbe5)),
        (1, true) => Some(f32::from_bits(0xbf49_0fdc)),
        (2, false) => Some(f32::from_bits(0x3fc9_0fdc)),
        (2, true) => Some(f32::from_bits(0xbfc9_0fdc)),
        _ => None,
    };
    if let Some(t) = target {
        let (mut a, mut v) = (m.rot[2], m.pv_f(4));
        turn_ease(t, GATE_ACCEL, GATE_ACCEL, GATE_MAX, &mut a, &mut v, 0);
        m.rot[2] = a;
        m.set_pv_f(4, v);
    }
    let step = f32::from_bits(0x3e2a_aaab) * sign;
    // 4, 5 and 6 are tested one after the other: a change runs the next state's tick at once.
    if m.state == 4 {
        m.state_ticks += 1;
        m.pos = add(m.pos, scale(m.rows[1], step));
        if 0xb <= m.state_ticks as u16 { (m.state_ticks, m.state) = (0, 5); }
    }
    if m.state == 5 {
        m.state_ticks += 1;
        if 0x1e < m.state_ticks as u16 { (m.state_ticks, m.state) = (0, 6); }
    }
    if m.state == 6 {
        m.state_ticks += 1;
        m.pos = add(m.pos, scale(m.rows[1], -step));
        if 10 < m.state_ticks as u16 { m.state = 0; }
    }
}

/// `0x32bb68(a, out, v, axis)`: `v` turned by `a` about `axis` (made unit; under 1e-5 of a turn, `v` as it is).
fn turn_about(a: f32, v: [f32; 3], axis: [f32; 3]) -> [f32; 3] {
    if a.abs() < 1e-5 { return v; }
    crate::quat::rotate(crate::quat::axis_angle(a, set_len(axis, 1.0)), v)
}

/// Rolls moby `id` by `a` about its +y row (its rows 0 and 2 turned, `0x32bb68` twice).
fn roll(w: &mut MobyWorld, id: usize, a: f32) {
    let m = &mut w.mobys[id];
    let y = m.rows[1];
    m.rows[0] = turn_about(a, m.rows[0], y);
    m.rows[2] = turn_about(a, m.rows[2], y);
}

/// 0x0d65 (`0x3ff938`): a walkway section. Its pvars: `[0]` it rolls, `[4]` rolled back once, `[8]` a cuboid (−1
/// none), `[0xc]` a moby (none at the start; another moby writes it). Deleted without pvars. Its tick count `+0x96`.
/// * **0**: `[0xc]` and `[4]` 0, its rows kept (mode 0x100); 1, or 4 without `[0]` (4: nothing).
/// * **1**: unless Ratchet stands on it ([`stands_on`]) its ticks 0; else on the walkway (`0x18a168` = 1) inside
///   cuboid `[8]` (`0x32b7c0`): ticks 0, Ratchet to state 0x6f (`0x2cf1f8(0x6f, 1)`), 2; and, without a cuboid, the
///   moby `[0xc]` in state 0xfe / 0xfd with Ratchet on the walkway: ticks 0, 2.
/// * **2**: 91 ticks rolling 1° a tick about its +y row ([`roll`]); then Ratchet to state 0 (`0x2cf1f8(0, 1)`), 7.
/// * **3**: (`[4]` 0) 2° a tick back, two ticks a tick, to 91 or once `[4]` is set; `[4]` 1, 1, ticks 0.
/// * **5**: rolls back 1° a tick as its ticks count down to 0; then 1, ticks 0.
/// * **7**: (the word `0x18a034` 2 within 4 across of Ratchet, else 0) and on as **6**: 5 once Ratchet is off the
///   walkway.
///
/// Not ported: the word `0x18a034` (nothing the port has reads it); Ratchet rolled with it on the walkway (the
/// magnetic walkways' frame mode 1, G-LVL-003); the writer of `[0xc]` (none on this level's code found here), so
/// the moby test never passes.
pub fn update_0d65(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pvars.is_empty() {
        w.delete(id);
        return;
    }
    let one = f32::from_bits(0x3c8e_fa36);
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.set_pv_i(0xc, 0);
            m.set_pv_i(4, 0);
            m.mode |= crate::moby::mode::KEEP_ROWS;
            m.state = if m.pv_i(0) == 0 { 4 } else { 1 };
        }
        1 => {
            if !stands_on(ctx, id) {
                w.mobys[id].state_ticks = 0;
                return;
            }
            let cuboid = w.mobys[id].pv_i(8);
            if on_walkway(ctx) && ctx.level.in_cuboid(cuboid, ctx.hero.pos) {
                let m = &mut w.mobys[id];
                m.state_ticks = 0;
                ctx.hero.request_state(0x6f, true);
                m.state = 2;
            }
            if cuboid != -1 { return; }
            let other = usize::try_from(w.mobys[id].pv_i(0xc)).ok().filter(|&i| i != 0).and_then(|i| w.mobys.get(i));
            if other.is_some_and(|o| matches!(o.state, 0xfe | 0xfd)) && on_walkway(ctx) {
                let m = &mut w.mobys[id];
                m.state = 2;
                m.state_ticks = 0;
            }
        }
        2 => {
            if (w.mobys[id].state_ticks as u16) < 0x5b {
                roll(w, id, one);
                w.mobys[id].state_ticks += 1;
                return;
            }
            ctx.hero.request_state(0, true);
            w.mobys[id].state = 7;
        }
        3 => {
            if w.mobys[id].pv_i(4) == 0 {
                roll(w, id, f32::from_bits(0xbd0e_fa36));
                w.mobys[id].state_ticks += 2;
            }
            let m = &mut w.mobys[id];
            if (m.state_ticks as u16) < 0x5b && m.pv_i(4) == 0 { return; }
            m.set_pv_i(4, 1);
            m.state = 1;
            m.state_ticks = 0;
        }
        5 => {
            roll(w, id, -one);
            let m = &mut w.mobys[id];
            m.state_ticks -= 1;
            if m.state_ticks != 0 { return; }
            m.state = 1;
            m.state_ticks = 0;
        }
        6 | 7 => {
            if !on_walkway(ctx) { w.mobys[id].state = 5; }
        }
        _ => {}
    }
}

/// 0x0cd8 (`0x3f1250`): its pvars `[0]` its yaw a quarter turn on, `[4]` its turn speed, `[8]` its yaw at rest. Its
/// tick count `+0x96`.
/// * **0**: update distance 0xff, `[8]` its yaw, `[0]` that + 45° (`0x2fa528`), 1, ticks 0.
/// * **1** / **3**: the first tick (`0x3337d8`) its class sound 0 (`0x36a110`); its yaw eased toward `[0]` / `[8]`
///   (`0x3294a8`: 0.05° a tick² both ways, at most 6° a tick); after 121 ticks 3 / 1 (`0x325bd0`).
pub fn update_0cd8(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let state = w.mobys[id].state;
    match state {
        0 => {
            let m = &mut w.mobys[id];
            m.update_dist = 0xff;
            let yaw = m.rot[2];
            m.set_pv_f(8, yaw);
            m.set_pv_f(0, crate::hero::physics::add_rot(yaw, f32::from_bits(0x3f49_0fdc)));
            m.state = 1;
            m.state_ticks = 0;
        }
        1 | 3 => {
            if w.mobys[id].first() { ctx.host.play_sound(&w.mobys[id], 0, 0); }
            let m = &mut w.mobys[id];
            let t = m.pv_f(if state == 1 { 0 } else { 8 });
            let k = f32::from_bits(0x3a64_c38a);
            let (mut a, mut v) = (m.rot[2], m.pv_f(4));
            turn_ease(t, k, k, f32::from_bits(0x3dd6_7752), &mut a, &mut v, 0);
            m.rot[2] = a;
            m.set_pv_f(4, v);
            if (m.state_ticks as u16) < 0x79 {
                m.state_ticks += 1;
                return;
            }
            m.set_state(if state == 1 { 3 } else { 1 }, -1);
        }
        _ => {}
    }
}
