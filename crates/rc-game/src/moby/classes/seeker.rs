//! The Seeker Gun (item 0x18, crate::hero::items::ROWS; L00's record `0x263a00`: attachment 0, stance 1, icon 0xea6b,
//! 25 missiles, 12 a second, 2 a second, damage 6, the blast's 1 (`+0x64`), name 0x32ec, its upgrade's row 0x43
//! (`+0x3a`)): the hand moby **0x05ce** (`0x3a5360`, called by the item pass while it is the hand item) and the missile
//! **0x0d01** (`0x3abb40`, made by `0x3ab8f8`). The record's shot class `+0x50` is 0x05d9, which `lvl.vtbl` also maps
//! to `0x3abb40`, but the launcher makes 0x0d01; 0x05d8 (`0x3ab088`, steered by the stick) and 0x05da (made by
//! `0x3ace00` on item 0x1b's row) are other weapons' shots.
//!
//! **The hand** (pvars: `+0x00` the cooldown (seconds); the port keeps the empty click's flag `0x1b27e0` at `+0x04`).
//! Each tick: state 0 → 1; its sequence run out, back to 1 (blend 2); the targets' claims of Ratchet's age
//! (crate::moby::claims, `0x37f440`); its size the class's × 0.75 (`$gp+0x5e4`); the look stance 1 goes to its twin
//! 0x1d. In 0x1d the crosshair (`0x2db498(2, 0, 90°, hand, …, FX 0x27, −1, 4)`: red `0x600000ff` while the cooldown
//! runs with ammo left, else green `0x600fff0f`) and the look wait (`0x18c354`) counts down; outside it is 0x28. The
//! cooldown runs down every tick; out, with the items not kept away (`0x18c0dc`), ○ held (`0x138320 & 0x18b050`), in
//! the hand past 0x16 ticks and the look wait out: a missile (`0x2d42d0(−1, 1)`) — the flag set, from its joint 0
//! (`0x3127c8`), its class sound 1, its sequence 2 (blend 2), the missile launched ([`launch`], `0x3accf0`) with the
//! hand its owner (`+0xb8`); none left: with ○ pressed or the flag set, its class sound 0 and the flag cleared. Either
//! way the cooldown is 1 / the row's rate. No use stance: Ratchet keeps his.
//!
//! **The launch** (`0x3accf0`): the row's speed / 60 a tick along the camera's forward row in the look stance 0x1d, the
//! gun's −row 1 with the arm raised at a wall (`0x18c30c`), else Ratchet's forward row; a lock (`0x18aa18`) turns it to
//! the lock's aim point (`0x32b4d0`) and is handed on; [`spawn`] (`0x3ab8f8(muzzle, vel, hand, lock, 5)`).
//!
//! **The missile** (pvars: `+0x00` its velocity (a tick), `+0x14` its target, `+0x18` (s16) its life (`40 / speed`
//! ticks), `+0x1a` (s16) a random 0..254, `+0x20` its bob's phase (π), `+0x24` its bob, `+0x28` / `+0x2c` its sway's
//! phases, `+0x34` one over its life, `+0x40` its velocity as it found its target, `+0x50` its size word (0.5; eased
//! 1.5 → 0.5 and 0.5 → 1.5 below, read by nothing in the update), `+0x54` / `+0x56` (s16) its timers, `+0x58` its lift's
//! speed, `+0x5c` its loop sound, `+0x74` the lock, `+0x78` the lock's wait (5), `+0x7c` its lift). Made ([`spawn`]):
//! far 0xff, drawn, its size ×1.5, its rows kept along its way (row 1 the up × its way), its sequence 2.
//!
//! Each tick ([`update_missile`]) without a target: no lock — the aim search all round 8 far (`0x3257a0(π, π, 8, 0,
//! 0, …, list of 0x20)`, no crates, crate::moby::aim::search_list) and the pick of its candidates
//! (crate::moby::claims, `0x37f338`; a fresh one claimed for 20 ticks); with a lock, once its wait is out the lock. A
//! target found: **1**, its sequence 0 (blend 10), its velocity kept, 20 ticks. By state:
//! * **0** (flying out): it moves; the line 0.5 down its up (flags 0x12, Ratchet left out) meeting the ground lifts it
//!   by `0.5 − d` (at most 1/15) when that beats its lift, else a lift it has carries it up and eases to 0 (`0x31ebd8(0,
//!   0.02, 0.2, 0)`, under 0.001: 0); its loop sound 0 kept playing; its life out: it blows up.
//! * **1** (turning, 20 ticks, then **2** for 10): it faces its target's aim point (`0x3aca50`: the velocity's length
//!   kept), drifts on along its kept velocity slowed 0.85 a tick and backs off its target 1/12 a tick; its
//!   size word eases 0.5 → 1.5.
//! * **2** (holding): it backs off `(t − 5) / 5` (at least 0) of that; out: **3** — its velocity one a tick,
//!   for `|aim point − it| + 5` ticks (10 without a target), the loop sound stopped, its class sound 1.
//! * **3** (striking): a live target (not state −2 / −3, with collision) is homed on as in 1; it moves; its size word
//!   eases 1.5 → 0.5 over 10 ticks; its time out: it blows up.
//!
//! Then (`0x3ac220`): out of the world's 2..1021 box it is gone. The line 2 down its up (flags 0x12, itself left out)
//! meeting the ground: closer than 0.0001 it blows up, else it rises toward 1 above the ground, 0.025 a tick at most.
//! Its rows along its way (row 1 the up × it); its bob (`0.15·sin`, the phase 0.06 a tick, along the up) and sway
//! (`0x3244e8(0.5, 0.1, 0.025)`) turning row 1 and 2 about row 0 and row 0 and 2 about row 1; its blob shadow (0.2).
//! The line from its last place (flags 0x10, Ratchet left out) meeting anything blows it up; else in 3 a target's aim
//! point within 0.5 of the step (`0x31d038`) is struck there; else (not 3) the 0.2 sphere (flags 0x10, Ratchet left
//! out) touching anything blows it up at the sphere's place. Still flying: its exhaust ([`exhaust`], `0x3acb28`) 0.25
//! behind it along its row 0.
//!
//! **The blast** ([`explode`], `0x3ac7c0`): the moby struck takes the row's damage (`0x31d378`: along its way, flags
//! 0x810000, kind the class's `<< 16 | 0x0203`), then every moby within 2 but it the blast's (`+0x64`, `0x2e14b0`);
//! its class sound 2, the blast (crate::moby::blast, `0x3f8348(0.75, 20, …, shake, 0, 1, 0, 0x14, 0x2d)`), gone.
//!
//! **The exhaust** (`0x3acb28`): twice a glow puff (`0x335bb8(0.04, 1, 0.9, 50000, …, ±0..15, 0, 0x7f204080, 1)`, 10
//! ticks), twice a white one (`0x335bb8(0.02, 1, 0.97, 15000, …, ±16, 0, 0x7fffffff, 1)`, 2 ticks, a random roll).
//!
//! Its glow in the hand (`0x2a7ed8`'s case 0x18): crate::hero::items.
//!
//! Not ported: the lock cheat (`0x18b270` → `0x18aa18`), the weapon mods' effects on a hit (`0x4126b8`), the gravity
//! frames and moving frames (`0x380e08`, `0x37f538` / `0x37f618` / `0x37f5f8`), the light word copied at the make
//! (`0x313330` → `+0x38`, unread), the 0x13a0 blast's heat shimmer (its `+0x18`, crate::moby::blast). The missile's
//! loop sound is let go when it goes (the engine frees a deleted moby's sounds). The upgraded rows (row 0x43 on, the
//! hand 0x0cee) are not in crate::hero::items::UPGRADE_ROWS.

use crate::collision::QueryFlags;
use crate::hero::items::{row, ItemRow, SEEKER};
use crate::hero::physics::{add, add_rot, cross, dist, dot, len, scale, set_len, spring, sub, unit, V3};
use crate::moby::hits::HitTemplate;
use crate::moby::{aim, countdown, Ctx, MobyWorld};

/// The hand's class (item 0x18's row `+0x04`) and the missile's (`0x3ab8f8`).
pub const HAND_CLASS: i16 = 0x5ce;
pub const MISSILE: i16 = 0xd01;
/// `t[3]` of the missile's hit template (`0x45afdf66`).
const NO_KNOCK_LIMIT: f32 = 5627.925;
/// `$gp+0x5e4`: the hand's size against its class's.
const HAND_SIZE: f32 = 0.75;
/// `0x40490fdc`: the search's limits all round.
const ALL_ROUND: f32 = f32::from_bits(0x4049_0fdc);
/// The claim a fresh pick makes (ticks).
const CLAIM_TICKS: i32 = 0x14;

mod pv {
    pub const VEL: usize = 0x00;
    pub const TARGET: usize = 0x14;
    pub const LIFE: usize = 0x18;
    pub const ROLL: usize = 0x1a;
    pub const BOB_PHASE: usize = 0x20;
    pub const BOB: usize = 0x24;
    pub const SWAY_A: usize = 0x28;
    pub const SWAY_B: usize = 0x2c;
    pub const INV_LIFE: usize = 0x34;
    pub const KEPT: usize = 0x40;
    pub const SIZE: usize = 0x50;
    pub const T1: usize = 0x54;
    pub const T2: usize = 0x56;
    pub const LIFT_V: usize = 0x58;
    pub const SOUND: usize = 0x5c;
    pub const LOCK: usize = 0x74;
    pub const LOCK_WAIT: usize = 0x78;
    pub const LIFT: usize = 0x7c;
}

/// The Seeker Gun's row as `0x139568[0x18]` gives it (crate::upgrades).
fn cur_row(game: &crate::game::GameState) -> Option<&'static ItemRow> { row(game, SEEKER) }

/// The record's `+0x64` (the blast's damage) by row.
fn blast_damage(_row: &ItemRow) -> f32 { 1.0 }

/// `0x2eef88(out, a, b)` = b × a.
fn cross_ba(a: V3, b: V3) -> V3 { cross(b, a) }

/// `v` turned by `a` about `axis` (`0x31fe98` + `0x321b28`).
fn turn(v: V3, axis: V3, a: f32) -> V3 { crate::gravity::rotate(v, set_len(axis, 1.0), a) }

/// `0x2eeb40(+0x00)`: the cooldown down a frame (1/60 s); true when out.
fn cooldown(w: &mut MobyWorld, id: usize) -> bool {
    let c = w.mobys[id].pv_f(0);
    if 1.0 / 60.0 < c {
        w.mobys[id].set_pv_f(0, c - 1.0 / 60.0);
        false
    } else {
        w.mobys[id].set_pv_f(0, 0.0);
        true
    }
}

/// `0x2eeae0` on the s16 at `off`: true when it was 0 or just reached it.
fn count_s(w: &mut MobyWorld, id: usize, off: usize) -> bool {
    let mut n = w.mobys[id].pv_s(off) as i32;
    let done = countdown(&mut n);
    w.mobys[id].set_pv_s(off, n as i16);
    done
}

/// A moby id kept in a pvar (−1 none), when it still exists.
fn moby_at(w: &MobyWorld, id: usize, off: usize) -> Option<usize> {
    usize::try_from(w.mobys[id].pv_i(off)).ok().filter(|&t| w.mobys.get(t).is_some())
}

/// `0x3a5360` (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].pvars.resize(crate::moby::PVAR_BYTES, 0);
    if w.mobys[id].state == 0 { w.mobys[id].state = 1; }
    if let Some(a) = ctx.host.anim(&w.mobys[id]) {
        if a.seq_a == a.seq_b && a.flags & 2 != 0 { ctx.host.set_anim(&w.mobys[id], 1, 0, 2); }
    }
    if let Some(o) = ctx.hero_moby {
        let mut c = std::mem::take(&mut w.claims);
        c.age(w, o);
        w.claims = c;
    }
    let class_scale = w.class(w.mobys[id].o_class).map_or(1.0, |c| c.scale);
    w.mobys[id].scale = class_scale * HAND_SIZE;
    if ctx.hero.state == 1 { ctx.hero.request_state(0x1d, true); }
    let look = ctx.hero.state == 0x1d;
    if look {
        let colour = if 0.0 < w.mobys[id].pv_f(0) && ctx.hero.items.has_ammo(ctx.game) { 0x6000_00ff } else { 0x600f_ff0f };
        crate::hud::marker(&mut ctx.game.markers, 2.0, id, colour, None, 0x27);
    }
    let ok = if look { countdown(&mut ctx.hero.items.look_wait) } else {
        ctx.hero.items.look_wait = 0x28;
        true
    };
    let Some(r) = cur_row(ctx.game).copied() else { return };
    let it = ctx.hero.items;
    if cooldown(w, id) && it.no_use == 0 && it.held & it.trigger != 0 && 0x16 < it.item_ticks && ok {
        if ctx.hero.items.take_ammo(ctx.game, 1) {
            w.mobys[id].set_pv_i(4, 1);
            let muzzle = ctx.host.joint(&w.mobys[id], 0).unwrap_or(w.mobys[id].pos);
            ctx.host.play_sound(&w.mobys[id], 1, 0);
            ctx.host.set_anim(&w.mobys[id], 2, 0, 2);
            if let Some(m) = launch(w, id, ctx, muzzle, &r) { w.mobys[m].parent = Some(id); }
        } else if it.pressed & it.trigger != 0 || w.mobys[id].pv_i(4) == 1 {
            ctx.host.play_sound(&w.mobys[id], 0, 0);
            w.mobys[id].set_pv_i(4, 0);
        }
        w.mobys[id].set_pv_f(0, 1.0 / r.rate);
    }
}

/// `0x3accf0(muzzle, hand)` (module docs).
fn launch(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, muzzle: V3, r: &ItemRow) -> Option<usize> {
    let f = r.speed * 0.016_666_668;
    let mut vel = if ctx.hero.state == 0x1d {
        set_len(ctx.host.camera_rows()[0], f)
    } else if ctx.hero.items.raised {
        set_len(w.mobys[id].rows[1], -f)
    } else {
        set_len(ctx.hero.rows[0], f)
    };
    let lock = ctx.hero.items.aim_target.map(|(t, _)| t).filter(|&t| w.mobys.get(t).is_some());
    if let Some(t) = lock { vel = set_len(sub(aim::aim_point(w, t), muzzle), f); }
    spawn(w, ctx, muzzle, vel, lock, 5, r)
}

/// `0x3ab8f8(pos, vel, hand, lock, wait)` (module docs), when the class is loaded.
pub fn spawn(w: &mut MobyWorld, ctx: &mut Ctx, pos: V3, vel: V3, lock: Option<usize>, wait: i32, r: &ItemRow) -> Option<usize> {
    if w.class(MISSILE).is_none() { return None; }
    let id = w.create(MISSILE);
    let life = 40.0 / (r.speed * 0.016_666_668);
    let roll = ctx.game.rng.randi(0xff);
    let up = ctx.hero.gravity.up_at(pos);
    let r0 = unit(vel);
    let r1 = unit(cross_ba(r0, up));
    let r2 = cross_ba(r1, r0);
    let m = &mut w.mobys[id];
    m.pvars.resize(crate::moby::PVAR_BYTES, 0);
    m.drawn = 1;
    m.update_dist = 0xff;
    m.draw_dist = 0xff;
    m.state = 0;
    m.set_pv_i(pv::LOCK, lock.map_or(-1, |t| t as i32));
    m.set_pv_i(pv::LOCK_WAIT, wait);
    m.set_pv_i(pv::TARGET, -1);
    m.set_pv_f(pv::INV_LIFE, 1.0 / life);
    m.set_pv_s(pv::LIFE, life as i32 as i16);
    m.set_pv_f(pv::BOB_PHASE, std::f32::consts::PI);
    m.set_pv_f(pv::BOB, 0.0);
    m.set_pv_f(pv::SWAY_A, 0.0);
    m.set_pv_f(pv::SWAY_B, 0.0);
    m.set_pv_s(pv::ROLL, roll as i16);
    m.set_pv_f(pv::LIFT, 0.0);
    m.set_pv_f(pv::LIFT_V, 0.0);
    m.set_pv_f(pv::SIZE, 0.5);
    m.set_pv_i(pv::SOUND, -1);
    m.scale *= 1.5;
    m.pos = pos;
    m.set_pv_v(pv::VEL, vel);
    m.mode |= crate::moby::mode::KEEP_ROWS;
    m.rows = [r0, r1, r2];
    ctx.host.set_anim(&w.mobys[id], 2, 0, 0);
    Some(id)
}

/// `0x3aca50`: the velocity turned to the target's aim point (raised along the up at the missile), its length kept;
/// not for a target gone or dying.
fn face(w: &mut MobyWorld, id: usize, ctx: &Ctx, target: Option<usize>) {
    let Some(t) = target.filter(|&t| !matches!(w.mobys[t].state, 0xfe | 0xfd)) else { return };
    let p = target_point(w, id, ctx, t);
    let m = &mut w.mobys[id];
    let v = m.pv_v(pv::VEL);
    let l = len(v);
    m.set_pv_v(pv::VEL, set_len(sub(p, m.pos), l));
}

/// The target's position raised by its aim height (`0x31fa20 + 0x10`) along the up at the missile.
fn target_point(w: &MobyWorld, id: usize, ctx: &Ctx, t: usize) -> V3 {
    let up = ctx.hero.gravity.up_at(w.mobys[id].pos);
    let h = aim::health(w, t).map_or(0.0, |(_, a)| a);
    add(w.mobys[t].pos, scale(up, h))
}

/// Gone: its loop sound let go, deleted (`0x311688`).
fn gone(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let s = w.mobys[id].pv_i(pv::SOUND);
    if s != -1 && ctx.host.sound_playing(s) { ctx.host.stop_sound(s); }
    w.delete(id);
}

/// `0x3abb40` (module docs).
pub fn update_missile(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].pvars.resize(crate::moby::PVAR_BYTES, 0);
    let start = w.mobys[id].pos;
    if moby_at(w, id, pv::TARGET).is_none() && w.mobys[id].pv_i(pv::TARGET) == -1 {
        let mut found = None;
        match moby_at(w, id, pv::LOCK) {
            None if w.mobys[id].pv_i(pv::LOCK) == -1 => {
                let (pos, vel) = (w.mobys[id].pos, w.mobys[id].pv_v(pv::VEL));
                let cone = aim::Cone { yaw: ALL_ROUND, pitch: ALL_ROUND, range: 8.0, near_pitch: 0.0, near_yaw: 0.0 };
                let list = aim::search_list(w, ctx, cone, pos, vel, false, 0x20);
                if let Some(o) = ctx.hero_moby {
                    if let Some((t, fresh)) = w.claims.pick(o, &list) {
                        if fresh { w.claims.claim(o, t, CLAIM_TICKS); }
                        found = Some(t);
                    }
                }
            }
            lock => {
                let mut n = w.mobys[id].pv_i(pv::LOCK_WAIT);
                if countdown(&mut n) {
                    found = lock;
                    w.mobys[id].set_pv_i(pv::LOCK, -1);
                }
                w.mobys[id].set_pv_i(pv::LOCK_WAIT, n);
            }
        }
        if let Some(t) = found {
            let m = &mut w.mobys[id];
            m.set_pv_i(pv::TARGET, t as i32);
            m.state = 1;
            let v = m.pv_v(pv::VEL);
            m.set_pv_v(pv::KEPT, v);
            m.set_pv_s(pv::T1, 0x14);
            ctx.host.set_anim(&w.mobys[id], 0, 0, 10);
        }
    }
    let target = moby_at(w, id, pv::TARGET);
    let up = ctx.hero.gravity.up_at(w.mobys[id].pos);
    match w.mobys[id].state {
        0 => {
            {
                let m = &mut w.mobys[id];
                m.pos = add(m.pos, m.pv_v(pv::VEL));
            }
            let pos = w.mobys[id].pos;
            let down = scale(up, 0.5);
            let mut lifted = false;
            if let Some(o) = ctx.coll.and_then(|c| crate::collision::coll_line_m(c, ctx.scene, pos, sub(pos, down), QueryFlags(0x12), ctx.hero_moby)) {
                let f = (0.5 - len(sub(pos, o.point))).min(0.066_666_67);
                let m = &mut w.mobys[id];
                if m.pv_f(pv::LIFT) < f {
                    lifted = true;
                    m.pos = add(m.pos, scale(down, f + f));
                    m.set_pv_f(pv::LIFT, f);
                }
            }
            if !lifted && w.mobys[id].pv_f(pv::LIFT) != 0.0 {
                let m = &mut w.mobys[id];
                m.pos = add(m.pos, scale(up, m.pv_f(pv::LIFT)));
                let (mut x, mut v) = (m.pv_f(pv::LIFT), m.pv_f(pv::LIFT_V));
                spring(0.0, 0.02, 0.2, 0.0, &mut x, &mut v);
                if x < 0.001 { x = 0.0; }
                m.set_pv_f(pv::LIFT, x);
                m.set_pv_f(pv::LIFT_V, v);
            }
            let s = w.mobys[id].pv_i(pv::SOUND);
            if s == -1 || !ctx.host.sound_playing(s) {
                let h = ctx.host.play_sound(&w.mobys[id], 0, 4);
                w.mobys[id].set_pv_i(pv::SOUND, h);
            }
            if count_s(w, id, pv::LIFE) {
                blow(w, id, ctx, None);
                return;
            }
        }
        1 => {
            if count_s(w, id, pv::T1) {
                w.mobys[id].state = 2;
                w.mobys[id].set_pv_s(pv::T2, 10);
            }
            face(w, id, ctx, target);
            let m = &mut w.mobys[id];
            let kept = scale(m.pv_v(pv::KEPT), 0.85);
            m.set_pv_v(pv::KEPT, kept);
            m.pos = add(m.pos, kept);
            let t = m.pv_s(pv::T1) as f32 / 20.0;
            m.set_pv_f(pv::SIZE, crate::camera::cos_interp(1.5, 0.5, t));
            if let Some(t) = target {
                let away = set_len(sub(w.mobys[id].pos, w.mobys[t].pos), 0.083_333_336);
                let m = &mut w.mobys[id];
                m.pos = add(m.pos, away);
            }
        }
        2 => {
            let f = ((w.mobys[id].pv_s(pv::T2) as f32 - 5.0) / 5.0).max(0.0);
            if let Some(t) = target {
                let away = set_len(sub(w.mobys[id].pos, w.mobys[t].pos), f * 0.083_333_336);
                let m = &mut w.mobys[id];
                m.pos = add(m.pos, away);
            }
            if count_s(w, id, pv::T2) {
                let v = unit(w.mobys[id].pv_v(pv::VEL));
                w.mobys[id].state = 3;
                w.mobys[id].set_pv_v(pv::VEL, v);
                let n = match target {
                    None => 10,
                    Some(t) => (dist(target_point(w, id, ctx, t), w.mobys[id].pos) + 5.0) as i32 as i16,
                };
                w.mobys[id].set_pv_s(pv::T1, n);
                let s = w.mobys[id].pv_i(pv::SOUND);
                if s != -1 && ctx.host.sound_playing(s) { ctx.host.stop_sound(s); }
                w.mobys[id].set_pv_i(pv::SOUND, -1);
                ctx.host.play_sound(&w.mobys[id], 1, 0);
            }
        }
        3 => {
            if count_s(w, id, pv::T1) {
                blow(w, id, ctx, None);
                return;
            }
            let t = w.mobys[id].pv_s(pv::T1) as f32 / 10.0;
            w.mobys[id].set_pv_f(pv::SIZE, crate::camera::cos_interp(0.5, 1.5, t));
            let live = target.is_none_or(|t| !matches!(w.mobys[t].state, 0xfe | 0xfd) && w.mobys[t].has_collision);
            if live { face(w, id, ctx, target); }
            let m = &mut w.mobys[id];
            m.pos = add(m.pos, m.pv_v(pv::VEL));
        }
        _ => {}
    }
    finish(w, id, ctx, start, up, target);
}

/// `0x3ac220`: the ground, the rows, the bob and sway, the hits, the exhaust (module docs).
fn finish(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, start: V3, up: V3, target: Option<usize>) {
    let in_box = |p: V3| p.iter().all(|&c| (2.0..=1021.0).contains(&c));
    if !in_box(w.mobys[id].pos) {
        gone(w, id, ctx);
        return;
    }
    let pos = w.mobys[id].pos;
    if let Some(o) = ctx.coll.and_then(|c| crate::collision::coll_line_m(c, ctx.scene, pos, add(pos, scale(up, -2.0)), QueryFlags(0x12), Some(id))) {
        let c = sub(pos, o.point);
        let f = len(c);
        if f < 0.0001 {
            blow(w, id, ctx, None);
            return;
        }
        let mut a = sub(add(o.point, scale(c, 1.0 / f)), pos);
        if 0.0 < dot(a, up) {
            let l = len(a);
            if 0.025 < l { a = scale(a, 0.025 / l); }
            let m = &mut w.mobys[id];
            m.pos = add(m.pos, a);
        }
    }
    {
        let m = &mut w.mobys[id];
        let r0 = unit(m.pv_v(pv::VEL));
        let r1 = unit(cross_ba(r0, up));
        let r2 = cross_ba(r1, r0);
        m.rows = [r0, r1, r2];
        let ph = add_rot(m.pv_f(pv::BOB_PHASE), 0.06);
        m.set_pv_f(pv::BOB_PHASE, ph);
        m.pos = sub(m.pos, scale(up, m.pv_f(pv::BOB)));
        let b = ph.sin() * 0.15;
        m.set_pv_f(pv::BOB, b);
        m.pos = add(m.pos, scale(up, b));
        super::props::sway(m, 0.5, 0.1, 0.025, pv::SWAY_A, pv::SWAY_B);
        if 1e-5 < m.rot[0].abs() {
            let (a, ax) = (m.rot[0], m.rows[0]);
            m.rows[1] = turn(m.rows[1], ax, a);
            m.rows[2] = turn(m.rows[2], ax, a);
        }
        if 1e-5 < m.rot[1].abs() {
            let (a, ax) = (m.rot[1], m.rows[1]);
            m.rows[0] = turn(m.rows[0], ax, a);
            m.rows[2] = turn(m.rows[2], ax, a);
        }
    }
    crate::shadows::request(w, ctx, 0.2, id);
    let pos = w.mobys[id].pos;
    // The step's line (flags 0x10, Ratchet left out): anything met blows it up, the moby it met struck.
    if let Some(o) = ctx.coll.and_then(|c| crate::collision::coll_line_m(c, ctx.scene, start, pos, QueryFlags(0x10), ctx.hero_moby)) {
        blow(w, id, ctx, o.moby);
        return;
    }
    if w.mobys[id].state == 3 {
        if let Some(t) = target {
            let b = target_point(w, id, ctx, t);
            let a = closest(start, pos, b);
            if dist(a, b) <= 0.5 {
                w.mobys[id].pos = a;
                blow(w, id, ctx, Some(t));
                return;
            }
        }
    } else if let Some(o) = ctx.coll.and_then(|c| crate::collision::coll_sphere_m(c, ctx.scene, pos, 0.2, QueryFlags(0x10), ctx.hero_moby)) {
        w.mobys[id].pos = o.pushed_centre.unwrap_or(pos);
        blow(w, id, ctx, o.moby);
        return;
    }
    let at = add(pos, scale(w.mobys[id].rows[0], -0.25));
    exhaust(ctx, at);
}

/// `0x31d038(out, a, b, p)`: the point of the segment a..b nearest `p`.
fn closest(a: V3, b: V3, p: V3) -> V3 {
    let ab = sub(b, a);
    let d = dot(ab, ab);
    let t = if d == 0.0 { 0.0 } else { (dot(ab, sub(p, a)) / d).clamp(0.0, 1.0) };
    add(a, scale(ab, t))
}

/// The blast and the missile gone (`0x3ac7c0`, `0x311688`).
fn blow(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, struck: Option<usize>) {
    explode(w, id, ctx, struck);
    gone(w, id, ctx);
}

/// `0x3ac7c0` (module docs).
fn explode(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, struck: Option<usize>) {
    let r = cur_row(ctx.game).copied();
    let (damage, splash) = r.map_or((0.0, 0.0), |r| (r.damage, blast_damage(&r)));
    let m = &w.mobys[id];
    let d = unit(m.pv_v(pv::VEL));
    let pos = m.pos;
    let mut t = HitTemplate::new(damage, Some(id), 0x81_0000, [d[0], d[1], d[2], NO_KNOCK_LIMIT]);
    t.kind = (m.o_class as u16 as u32) << 16 | 0x0203;
    if let Some(s) = struck { w.deliver_hit_any(s, &t, [0.0; 4]); }
    t.damage = splash;
    w.sphere_hit(ctx.scene, pos, 2.0, 1, struck, Some(&t));
    ctx.host.play_sound(&w.mobys[id], 2, 0);
    crate::moby::blast::burst(w, ctx, pos, &crate::moby::blast::Blast { size: 0.75, bursts: [0, 1, 0], puffs: 0x14, streaks: 0x2d, light: false, shake: true });
}

/// `0x3acb28(missile, at)` (module docs).
fn exhaust(ctx: &mut Ctx, at: V3) {
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let g = &mut ctx.game.rng;
    for _ in 0..2 {
        let r = g.randi(0x10);
        let spin = if g.randi(2) == 0 { r } else { -r };
        if let Some(i) = crate::particles::puff(sys, g, 0.04, 1.0, 0.9, 50_000.0, at, spin, [0.0; 3], 0x7f20_4080, 1) {
            let rec = &mut sys.pool.recs[i];
            rec[0xa..0xc].copy_from_slice(&10i16.to_le_bytes());
            rec[9] = 4 + 0x40;
            rec[0x24..0x28].copy_from_slice(&2u32.to_le_bytes());
            rec[0x2a] = 0x7f;
            rec[0x2b] = rec[0xa];
        }
    }
    let mut spin = 0x10;
    for _ in 0..2 {
        let i = crate::particles::puff(sys, g, 0.02, 1.0, 0.97, 15_000.0, at, spin, [0.0; 3], 0x7fff_ffff, 1);
        spin = -spin;
        if let Some(i) = i {
            let roll = g.randi(0xff) as u8;
            let rec = &mut sys.pool.recs[i];
            rec[0xa..0xc].copy_from_slice(&2i16.to_le_bytes());
            rec[9] = 4 + 0x40;
            rec[8] = roll;
            rec[0x24..0x28].copy_from_slice(&2u32.to_le_bytes());
            rec[0x2a] = 0x7f;
            rec[0x2b] = rec[0xa];
        }
    }
}
