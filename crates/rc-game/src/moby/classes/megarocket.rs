//! The Megarocket Cannon, the Minirocket Tube's upgrade (item 0x1b's rows 0x45, 0x63, 0x64,
//! crate::hero::items::UPGRADE_ROWS; L00 `0x266160`: attachment 0, stance 2, use sequence 0x36, the firing arm's 0x46
//! crouched, the rockets' speed 60 a second, 4 a second, damage 18 (30, 60), the blast's 7 (0 on the Mega rows), 25
//! rockets (40, 60)): the hand moby **0x1004** (`0x410ba0`, called by the item pass while it is the hand item), firing
//! the Minirocket Tube's rockets 0x05da (crate::moby::classes::minirocket) in volleys of up to four.
//!
//! **The hand** (pvars: `+0x00` the lock, `+0x04` its hold, `+0x08` its aim height, `+0x0c` (s16) the muzzle's point
//! light, `+0x0e` (s16) its ticks, `+0x10` the last pick, `+0x14` the pick's count, `+0x18` (s16) in the look last tick,
//! `+0x1a` (s16) the charge's ticks, `+0x1c` the cooldown (seconds), `+0x20` the charge glow's phase, `+0x24` the pad
//! rumble's handle, `+0x28` the volley's rockets left (−1 none), `+0x2c` the volley's wait, `+0x30` the charge's loop
//! sound, `+0x34` the volley's size). State 0: the light none, the pick count 5, the sound, the rumble and the volley
//! none, the rest cleared; **1**. Each tick, as the Minirocket Tube's hand: the look stance 1 goes to its twin 0x1d; in the
//! look (0x1d with Ratchet hidden) the lock dropped on entering it, its crosshair, the search 4° by 4°, else 20° by 20°;
//! the cooldown runs down; its sequence run out, back to 1 (blend 2); the search and the lock (75 far, crates too:
//! crate::moby::classes::minirocket's); the look wait. The muzzle: (0.25, −1.1, 0) in the hand's frame.
//! * **The charge**: ○ held with a rocket left counts its ticks, its loop sound (class sound 6), its draw asked for; past
//!   0x16 ticks a rocket it could fire (four at most), one spark a tick off the muzzle (`0x339848(0.45..0.75 × 210000,
//!   500, 1, .., 15 ticks, 0x7f, 0x7f7f7f)` jittered 0.1, flying 1.5..2 a second along the hand's row 2 turned at random
//!   about its row 1). Let go: the sound stops; ○ pressed with none left, its class sound 1; a charge let go is a shot of
//!   1 (to 0x1e ticks), 2 (to 0x3c), 3 (to 0x59) or 4 rockets ([`volley_size`], `0x411d40`); the charge and the glow's
//!   phase cleared.
//! * **The shot** (outside a scene, the cooldown out, the items not kept away, the look wait out, a charge let go, in the
//!   hand 0x17 ticks): with a rocket left — without a lock the point-blank lock; the light's ticks 15; the burst
//!   ([`burst`], `0x411d88`); the volley ([`fire`], `0x411930`: its size, its wait and its rockets left all but one,
//!   class sound 5, the first rocket at once); its sequence 2 (blend 2); the use stance (`0x2af5e0`). The cooldown 1 /
//!   the row's rate.
//! * **The volley** (`0x411a08`): while rockets are left and the wait is out (once out it stays so: the rest come a tick
//!   apart), the next (`k`, counting down): none left ends it; else from the hand's place + the k-th offset of
//!   `$gp+0x2270` in its frame ((0.45, −1.125, ±0.3), (0, −1.125, ±0.3)), a rocket (`0x3ace00(aim height, 4, 8, ..,
//!   hand, at, lock, offset)`: crate::moby::classes::minirocket::spawn_with) aiming that offset off the lock's aim point
//!   (the last with a lock: only its height), one ammo, the hand its owner, its `+0x20` signed by k's parity.
//! * **The light** (`0x411678`): while its ticks run (not put away), a point light of radius 14 at the muzzle + 0.3
//!   Ratchet's up + 1.75 along the aim's way, `(t, 0.8·t, 0.5·t)` for `t` rising over its first 3 ticks then falling
//!   over 12; else, or put away, freed.
//!
//! **The burst** (`0x411d88(hand, n)`): from the hand's place + 0.3 row 0 − row 1, `20·(n + 1)` streaks (type 0x5a,
//! `0x33f940`, falling 0.0022, 10..30 ticks) jittered 0.2, flying 7..60 a second along Ratchet's forward row turned 20°
//! about his row 1, then about his forward by a sum of 0..36° steps; twenty sparks (`0x339848(0.4..2 × 210000, 10000,
//! 0.95, .., 10 ticks, 0x7f, 0x404040)`) jittered 0.8, flying 12..16 a second plus 1.7 × Ratchet's move on the ground
//! (in the air: the last streak's velocity), started a step back, their ways turned 12° and on as the streaks'.
//!
//! **The draw** ([`draw`], `0x411b90`, while charging): for each rocket the charge would fire (no more than are left), a
//! glow at `$gp+0x2230`'s offset in the hand's frame ((0.3, −1.125, ±0.075), (0.15, −1.125, ±0.075)): FX 0xb added,
//! 0.3 .. 0.5 across over 90 ticks of charge, `0x204080` at the charge's ticks × 4 (0x60 at most) + 32 · sin of its phase
//! (8° a tick), and a 0.3 times as large `0x80808080` core.
//!
//! Its glow in the hand (`0x2a7ed8`'s case 0x1b, upgraded rows only, crate::hero::items): its word pulses
//! `0xff20205f` .. `0xffdfdfdf`.
//!
//! Not ported: the pad rumble (`0x2d6d60(0x29fc88)` / `0x2d6f38`), the lock cheat (`0x18b270`), the help text a
//! single-rocket shot counts toward (`0x18cd00`, `0x2f5270(0x9f7, 0x4e)`: every shot here is a volley, which ends the
//! count), the write-only flag `0x1b2cd8`, the gravity frames.

use crate::hero::items::{row, MINIROCKET};
use crate::hero::physics::{add, add_rot, scale, sub, V3};
use crate::moby::{countdown, Ctx, MobyWorld};
use crate::point_lights::PointLight;
use super::minirocket::{self, LockPv};

/// The hand's class (rows 0x45 / 0x63 / 0x64 `+0x04`).
pub const HAND_CLASS: i16 = 0x1004;

mod hv {
    pub const LOCK: usize = 0x00;
    pub const HOLD: usize = 0x04;
    pub const AIMH: usize = 0x08;
    pub const LIGHT: usize = 0x0c;
    pub const TICKS: usize = 0x0e;
    pub const LAST: usize = 0x10;
    pub const COUNT: usize = 0x14;
    pub const LOOK: usize = 0x18;
    pub const CHARGE: usize = 0x1a;
    pub const COOL: usize = 0x1c;
    pub const PHASE: usize = 0x20;
    pub const RUMBLE: usize = 0x24;
    pub const LEFT: usize = 0x28;
    pub const WAIT: usize = 0x2c;
    pub const SOUND: usize = 0x30;
    pub const SIZE: usize = 0x34;
}
const LOCK: LockPv = LockPv { lock: hv::LOCK, hold: hv::HOLD, aimh: hv::AIMH, last: hv::LAST, count: hv::COUNT };
/// The rocket's `+0x20` (minirocket's pvars).
const ROCKET_R20: usize = 0x20;

/// `$gp+0x2220`: the muzzle in the hand's frame.
const MUZZLE: V3 = [0.25, f32::from_bits(0xbf8c_cccd), 0.0];
/// `$gp+0x2230`: the charge glows' places in the hand's frame.
const GLOWS: [V3; 4] = [
    [f32::from_bits(0x3e99_999a), -1.125, f32::from_bits(0x3d99_999a)],
    [f32::from_bits(0x3e99_999a), -1.125, f32::from_bits(0xbd99_999a)],
    [f32::from_bits(0x3e19_999a), -1.125, f32::from_bits(0x3d99_999a)],
    [f32::from_bits(0x3e19_999a), -1.125, f32::from_bits(0xbd99_999a)],
];
/// `$gp+0x2270`: the volley's rockets' places in the hand's frame (by `k`, counting down).
const VOLLEY: [V3; 4] = [
    [f32::from_bits(0x3ee6_6666), -1.125, f32::from_bits(0x3e99_999a)],
    [f32::from_bits(0x3ee6_6666), -1.125, f32::from_bits(0xbe99_999a)],
    [0.0, -1.125, f32::from_bits(0x3e99_999a)],
    [0.0, -1.125, f32::from_bits(0xbe99_999a)],
];

/// A point in the hand's frame.
fn in_hand(w: &MobyWorld, id: usize, p: V3) -> V3 {
    let m = &w.mobys[id];
    add(m.pos, crate::gravity::to_world(p, &m.rows))
}

/// `0x321c00(a, out, v, axis)`: `v` turned by `a` about `axis` (under 1e-5 kept).
fn turn(v: V3, axis: V3, a: f32) -> V3 {
    if a.abs() < 1e-5 || crate::hero::physics::len(axis) == 0.0 { return v; }
    crate::gravity::rotate(v, crate::hero::physics::unit(axis), a)
}

/// `0x2eeab0` on the word at `off`.
fn count_i(w: &mut MobyWorld, id: usize, off: usize) -> bool {
    let mut n = w.mobys[id].pv_i(off);
    let done = countdown(&mut n);
    w.mobys[id].set_pv_i(off, n);
    done
}

/// `0x411d40`: the rockets a charge of `ticks` fires.
pub fn volley_size(ticks: i16) -> i32 {
    if 0x5a <= ticks { 4 } else if 0x3d <= ticks { 3 } else if 0x1e < ticks { 2 } else { 1 }
}

/// The loop sound let go when still its (`0x3602f0`).
fn stop_sound(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let s = w.mobys[id].pv_i(hv::SOUND);
    if s != -1 && ctx.host.sound_playing(s) { ctx.host.stop_sound(s); }
    w.mobys[id].set_pv_i(hv::SOUND, -1);
}

/// `0x410ba0` (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].pvars.resize(crate::moby::PVAR_BYTES, 0);
    if w.mobys[id].state == 0 {
        let m = &mut w.mobys[id];
        m.state = 1;
        m.set_pv_s(hv::LIGHT, -1);
        m.set_pv_i(hv::COUNT, 5);
        m.set_pv_i(hv::SOUND, -1);
        m.set_pv_s(hv::TICKS, 0);
        m.set_pv_i(hv::LOCK, 0);
        m.set_pv_i(hv::LAST, 0);
        m.set_pv_s(hv::LOOK, 0);
        m.set_pv_s(hv::CHARGE, 0);
        m.set_pv_f(hv::PHASE, 0.0);
        m.set_pv_i(hv::RUMBLE, -1);
        m.set_pv_i(hv::SIZE, -1);
        m.set_pv_i(hv::LEFT, -1);
    }
    if ctx.hero.state == 1 { ctx.hero.request_state(0x1d, true); }
    let muzzle = in_hand(w, id, MUZZLE);
    let look = ctx.hero.state == 0x1d && ctx.hero.view_lock != 0;
    let cone = if look {
        if w.mobys[id].pv_s(hv::LOOK) == 0 {
            w.mobys[id].set_pv_i(hv::LOCK, 0);
            w.mobys[id].set_pv_i(hv::COUNT, 5);
        }
        w.mobys[id].set_pv_s(hv::LOOK, 1);
        let red = 0.0 < w.mobys[id].pv_f(hv::COOL) && ctx.hero.items.has_ammo(ctx.game);
        crate::hud::marker(&mut ctx.game.markers, 1.25, id, if red { 0x6000_00ff } else { 0x600f_ff0f }, None, 0x27);
        f32::from_bits(0x3d8e_fa36)
    } else {
        w.mobys[id].set_pv_s(hv::LOOK, 0);
        f32::from_bits(0x3eb2_b8c4)
    };
    let way = if !look && ctx.hero.items.raised { scale(w.mobys[id].rows[1], -1.0) } else { ctx.hero.rows[0] };
    {
        let c = w.mobys[id].pv_f(hv::COOL);
        w.mobys[id].set_pv_f(hv::COOL, if 1.0 / 60.0 < c { c - 1.0 / 60.0 } else { 0.0 });
    }
    if let Some(a) = ctx.host.anim(&w.mobys[id]) {
        if a.seq_a == a.seq_b && a.flags & 2 != 0 { ctx.host.set_anim(&w.mobys[id], 1, 0, 2); }
    }
    minirocket::aim(w, id, ctx, cone, cone, way, &LOCK);
    let look_ok = if ctx.hero.state == 0x1d { countdown(&mut ctx.hero.items.look_wait) } else {
        ctx.hero.items.look_wait = 0x28;
        true
    };
    let it = ctx.hero.items;
    let mut released = false;
    let mut size = 0;
    if it.held & it.trigger != 0 && it.has_ammo(ctx.game) {
        let n = w.mobys[id].pv_s(hv::CHARGE).wrapping_add(1);
        w.mobys[id].set_pv_s(hv::CHARGE, n);
        let s = w.mobys[id].pv_i(hv::SOUND);
        if s == -1 || !ctx.host.sound_playing(s) {
            let h = ctx.host.play_sound(&w.mobys[id], 6, 4);
            w.mobys[id].set_pv_i(hv::SOUND, h);
        }
        w.register_draw(id);
        let can = ctx.game.ammo.get(MINIROCKET as usize).copied().unwrap_or(0).min(4);
        if ((can * 0x16) as i64) < n as i64 { charge_spark(w, id, ctx, muzzle); }
    } else {
        stop_sound(w, id, ctx);
        if it.pressed & it.trigger != 0 && !it.has_ammo(ctx.game) { ctx.host.play_sound(&w.mobys[id], 1, 0); }
        let n = w.mobys[id].pv_s(hv::CHARGE);
        released = n != 0;
        if released { size = volley_size(n); }
        w.mobys[id].set_pv_s(hv::CHARGE, 0);
        w.mobys[id].set_pv_f(hv::PHASE, 0.0);
    }
    let it = ctx.hero.items;
    if ctx.game.scenes.mode == crate::scene::PLAY && w.mobys[id].pv_f(hv::COOL) == 0.0 && it.no_use == 0 && look_ok && released && 0x17 <= it.item_ticks {
        if it.has_ammo(ctx.game) {
            if w.mobys[id].pv_link(hv::LOCK).is_none() { minirocket::point_blank(w, id, ctx, hv::LOCK); }
            w.mobys[id].set_pv_s(hv::TICKS, 0xf);
            burst(w, id, ctx, size);
            fire(w, id, ctx, size);
            ctx.host.set_anim(&w.mobys[id], 2, 0, 2);
            let g = ctx.hero.group;
            ctx.hero.items.use_start(g);
        }
        let rate = row(ctx.game, MINIROCKET).map_or(1.0, |r| r.rate);
        w.mobys[id].set_pv_f(hv::COOL, 1.0 / rate);
    }
    if 0 <= w.mobys[id].pv_i(hv::LEFT) && count_i(w, id, hv::WAIT) { next_rocket(w, id, ctx); }
    light(w, id, ctx, muzzle);
}

/// One spark of the charge (module docs).
fn charge_spark(w: &MobyWorld, id: usize, ctx: &mut Ctx, muzzle: V3) {
    let rows = w.mobys[id].rows;
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let g = &mut ctx.game.rng;
    let a = g.rand_angle();
    let v = turn(rows[2], rows[1], a);
    let v = scale(v, g.randf(1.5, 2.0) * 0.016_666_668);
    let spin = g.randf_signed(1.0, 3.0) as i32;
    let size = g.randf(f32::from_bits(0x3ee6_6666), 0.75);
    let p: V3 = std::array::from_fn(|k| muzzle[k] + g.randf_signed(0.0, f32::from_bits(0x3dcc_cccd)));
    crate::particles::spark(sys, g, size * 210_000.0, 500.0, 1.0, 0.0, 0.0, p, v, 0xf, 0x7f, 0x7f_7f7f, spin);
}

/// `0x411930(hand, muzzle, n)` (module docs; `0x411a08` does not read the muzzle).
fn fire(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, n: i32) {
    let m = &mut w.mobys[id];
    m.set_pv_i(hv::SIZE, n);
    m.set_pv_i(hv::WAIT, n - 1);
    m.set_pv_i(hv::LEFT, n - 1);
    ctx.host.play_sound(&w.mobys[id], 5, 0);
    next_rocket(w, id, ctx);
}

/// `0x411a08` (module docs).
fn next_rocket(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if !ctx.hero.items.has_ammo(ctx.game) {
        w.mobys[id].set_pv_i(hv::LEFT, -1);
        return;
    }
    let k = w.mobys[id].pv_i(hv::LEFT);
    let sign = if k & 1 == 0 { -1.0 } else { 1.0 };
    w.mobys[id].set_pv_i(hv::LEFT, k - 1);
    let local = VOLLEY[k.clamp(0, 3) as usize];
    let off = crate::gravity::to_world(local, &w.mobys[id].rows);
    let at = add(w.mobys[id].pos, off);
    let lock = w.mobys[id].pv_link(hv::LOCK);
    let offset = if k - 1 == -1 && lock.is_some() { [0.0, off[1], 0.0] } else { off };
    let Some(r) = row(ctx.game, MINIROCKET).copied() else { return };
    let aimh = w.mobys[id].pv_f(hv::AIMH);
    let Some(rocket) = minirocket::spawn_with(w, ctx, id, at, aimh, (4.0, 8.0), lock, offset, &r) else { return };
    ctx.hero.items.take_ammo(ctx.game, 1);
    w.mobys[rocket].parent = Some(id);
    let f = w.mobys[rocket].pv_f(ROCKET_R20);
    if (0.0 < f && sign < 0.0) || (f < 0.0 && 0.0 < sign) { w.mobys[rocket].set_pv_f(ROCKET_R20, -f); }
}

/// `0x411d88(hand, n)` (module docs).
fn burst(w: &MobyWorld, id: usize, ctx: &mut Ctx, n: i32) {
    let m = &w.mobys[id];
    let at = add(m.pos, add(scale(m.rows[0], f32::from_bits(0x3e99_999a)), scale(m.rows[1], -1.0)));
    let (fwd, side) = (ctx.hero.rows[0], ctx.hero.rows[1]);
    let grounded = ctx.hero.grounded_ticks != 0;
    let disp = ctx.hero.disp;
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let g = &mut ctx.game.rng;
    let start = turn(fwd, side, f32::from_bits(0x3eb2_b8c4));
    let (mut d, mut acc) = (start, 0.0f32);
    let mut last = [0.0; 3];
    for _ in 0..(n * 0x14 + 0x14) {
        acc = add_rot(acc, g.randf(0.0, f32::from_bits(0x3f20_d97d)));
        last = scale(d, g.randf(7.0, 60.0) * 0.016_666_668);
        let life = g.rand_range(10, 0x1e) as i16;
        let p: V3 = std::array::from_fn(|k| at[k] + g.randf_signed(0.0, f32::from_bits(0x3e4c_cccd)));
        crate::particles::streak(sys, f32::from_bits(0x3b11_a2b4), p, last, life);
        if 1e-5 < acc.abs() { d = turn(start, fwd, acc); }
    }
    let carry = if grounded { scale(disp, f32::from_bits(0x3fd9_999a)) } else { last };
    let start = turn(fwd, side, 12.0 * 0.017_453_294);
    let (mut d, mut acc) = (start, 0.0f32);
    let (hi, grow, jitter) = if n != 0 { (2.0, 10_000.0, f32::from_bits(0x3f4c_cccd)) } else { (1.0, 2000.0, f32::from_bits(0x3ecc_cccd)) };
    for _ in 0..20 {
        acc = add_rot(acc, g.randf(0.0, f32::from_bits(0x3f20_d97d)));
        let v = add(scale(d, g.randf(12.0 * 0.016_666_668, 16.0 * 0.016_666_668)), carry);
        let spin = g.randf_signed(1.0, 2.0) as i32;
        let p: V3 = std::array::from_fn(|k| at[k] + g.randf_signed(0.0, jitter));
        let size = g.randf(f32::from_bits(0x3ecc_cccd), hi);
        crate::particles::spark(sys, g, size * 210_000.0, grow, f32::from_bits(0x3f73_3333), 0.0, 0.0, sub(p, v), v, 10, 0x7f, 0x40_4040, spin);
        if 1e-5 < acc.abs() { d = turn(start, fwd, acc); }
    }
}

/// The muzzle light let go (`0x2fdc48`).
fn free_light(w: &mut MobyWorld, id: usize) {
    let s = w.mobys[id].pv_s(hv::LIGHT);
    if 0 <= s { w.point_lights.free(s as usize); }
    w.mobys[id].set_pv_s(hv::LIGHT, -1);
    w.mobys[id].set_pv_s(hv::TICKS, 0);
}

/// `0x411678(hand, muzzle)` (module docs).
fn light(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, muzzle: V3) {
    let away = ctx.hero.items.ready == 3;
    let n = w.mobys[id].pv_s(hv::TICKS) as i32;
    if away || n < 1 {
        if w.mobys[id].pv_s(hv::LIGHT) != -1 { free_light(w, id); }
    } else {
        let (k, s) = if n < 0xd { (1.0 / 12.0, n as f32) } else { (1.0 / 3.0, (0xf - n) as f32) };
        let (r, g, b) = (s * k, s * k * 0.8, s * k * 0.5);
        let way = if ctx.hero.items.raised { scale(w.mobys[id].rows[1], -1.0) } else { ctx.hero.rows[0] };
        let pos = add(add(muzzle, scale(ctx.hero.rows[2], f32::from_bits(0x3e99_999a))), scale(way, 1.75));
        let l = PointLight { color: [r, g, b], intensity: 0.0, pos, radius: 14.0 };
        let s = w.mobys[id].pv_s(hv::LIGHT);
        if s == -1 {
            let s = w.point_lights.alloc(l).map_or(-1, |s| s as i16);
            w.mobys[id].set_pv_s(hv::LIGHT, s);
        } else {
            w.point_lights.set(s as usize, l);
        }
        let mut t = n;
        countdown(&mut t);
        w.mobys[id].set_pv_s(hv::TICKS, t as i16);
    }
    if away {
        if w.mobys[id].pv_s(hv::LIGHT) == -1 { w.mobys[id].set_pv_s(hv::TICKS, 0); } else { free_light(w, id); }
    }
}

/// `0x411b90` (module docs); `ammo` the rockets left (`0x139688[item]`).
pub fn draw(w: &mut MobyWorld, id: usize, ammo: i32, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let charge = w.mobys[id].pv_s(hv::CHARGE);
    let t = (charge as f32 / 90.0).min(1.0);
    let a = ((charge as i32) << 2).min(0x60);
    let size = 0.3 + (0.5 - 0.3) * t;
    let phase = w.mobys[id].pv_f(hv::PHASE);
    let wave = (phase.sin() * 32.0) as i32;
    w.mobys[id].set_pv_f(hv::PHASE, add_rot(phase, f32::from_bits(0x3e0e_fa36)));
    let rgba = ((wave + a) as u32) << 24 | 0x20_4080;
    let n = ammo.min(volley_size(charge));
    for p in GLOWS.iter().take(n.max(0) as usize) {
        let at = in_hand(w, id, *p);
        crate::fx::sprite(out, size, 0.0, 0.0, at, cam, 0xb, rgba, true);
        crate::fx::sprite(out, size * 0.3, 0.0, 0.0, at, cam, 0xb, 0x8080_8080, true);
    }
}
