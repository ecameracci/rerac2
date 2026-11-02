//! The Minirocket Tube (item 0x1b, crate::hero::items::ROWS; L00's record `0x263ca0`: attachment 0, stance 2 (both
//! hands), use sequence 0x36, the firing arm's 0x46 crouched and none moving, icon 0xea70, name 0x32f3, its upgrade's row
//! 0x45 (`+0x3a`), the rockets' speed 60 a second, 1.7 a second, damage 12, the blast's 6 (`+0x64`), 25 rockets): the
//! hand moby **0x05cf** (`0x3a5d00`, called by the item pass while it is the hand item) and the rocket **0x05da**
//! (`0x3ad308`, made by `0x3ace00`; the update has no Ghidra function on any level: read from its listing).
//!
//! **The hand** (pvars: `+0x00` the cooldown (seconds), `+0x10` the lock, `+0x14` the lock's hold, `+0x18` its aim
//! height, `+0x1c` (s16) the muzzle's point light (crate::point_lights, −1 none), `+0x1e` (s16) the light's ticks,
//! `+0x20` the last search's pick, `+0x24` the ticks it must stay picked, `+0x28` in the look last tick, `+0x2c` the
//! pad rumble's handle; the port keeps the empty click's flag `0x1b27e4` at `+0x04`). State 0: the light none, the pick
//! count 5, the rest cleared; **1**. Each tick: the muzzle smoke ([`smoke`], `0x3a5620`) while its timer (`+0xbc`)
//! runs; the cooldown runs down; the look stance 1 goes to its twin 0x1d; its sequence run out, back to 1 (blend 2).
//! * **The look** (0x1d with Ratchet hidden: the view lock, crate::first_person): entering it drops the lock (the
//!   count 5); the crosshair (`0x2db498(1.25, 0, 90°, hand, …, FX 0x27, −1, 4)`: red `0x600000ff` while the cooldown
//!   runs with ammo left, else green `0x600fff0f`); the search 4° by 4° (and near). Outside it, 18° by 17°.
//! * **The aim** (`0x325f90`, crate::moby::aim::search, crates too): 75 far, from 0.75 up Ratchet's up row along his
//!   forward row (`0x18a580`; the arm raised at a wall (`0x18c30c`) outside the look: the gun's −row 1). Without a
//!   lock, the same pick 5 ticks running becomes it (the hold 5, the aim height its health block's `+0x10`, else 0.1).
//!   With one — live (not state −2 / −3, health left when it has a block), drawn, the arm not raised — picked again
//!   it is held (the hold 5) and shown: the reticle FX 0x26 (`0xff0fff0f`, `|0.9 − d·0.0067·0.65|` large at the
//!   camera's distance d to it) at its aim point, Ratchet aiming at it (`0x18aa18`); not picked, the hold counts down
//!   and out drops it (the count 5); else it is dropped.
//! * **The shot**: the look wait (`0x18c354`) counts down in 0x1d, else is 0x28; outside a scene, the cooldown out,
//!   the items not kept away (`0x18c0dc`), the hand's sequence at most 1, ○ held, in the hand past 0x16 ticks and
//!   the wait out: a hand still in its draw-out sequence 0 goes to 1 (blend 4); a rocket (`0x2d42d0(−1, 1)`): the flag
//!   set; without a lock the point-blank sphere (1, flags 1, Ratchet left out) at the hand's −row 1 + 0.275 row 0 —
//!   an enemy-run moby (drawn, not dying, not 0xc87 / 0xecc, health left or a crate) not of class type 5 makes the
//!   sphere's first moby (`0x1bf198`) the lock; its class sound 0, the light's ticks 15, the burst ([`burst`],
//!   `0x3a58b0`), the rocket ([`spawn`] through `0x3a67a0`: `0x3ace00(aim height, 2, 5, 0.6, hand, joint 0, lock, 0)`,
//!   the hand its owner `+0xb8`), the use stance (standing, or walking (state 2) with the stick under 0.7: Ratchet's
//!   sequence 0x36 blended over 10 (`0x2d2508(10, 0x36, 1)`); else `0x2af5e0`), the hand's sequence 2 (no blend), the
//!   cooldown 1 / the row's rate. None left: with ○ pressed or the flag set, its class sound 1 and the flag cleared.
//! * **The light** (`0x2fdb60(8, 0, …)` / `0x2fdc48`): while the hand's sequence A is not 2 and its ticks run, a point
//!   light of radius 8 at joint 0 + the aim's way + 0.3 up Ratchet's up row, `(t, 0.8·t, 0.5·t)` for `t` rising over
//!   its first 3 ticks (`(15 − n) / 3`) then falling over 12 (`n / 12`); else freed. The hand's sequence B at 2 frees
//!   it and clears its ticks: the shot plays sequence 2 at once (`0x31b518` sets A and B), so the shot's ticks are
//!   cleared the tick they are set and the light never shows — the original's own order, kept.
//!
//! **The smoke** (`0x3a5620`, the timer down a tick, 6 ticks after a shot): from the hand's place + 0.1 row 1 + 0.275
//! row 0, riding 0.1 along row 1 a tick, four glow puffs (`0x335bb8(0.04, 1, 0.9, 52500, …, 0..15 either way,
//! 0x7f204080)`, 8 ticks) and three white ones (`0x335bb8(0.02, 1, 1, 15750 − 3150·k, …, ±16, 0x7fffffff)`, 2, 4 and 8
//! ticks, a random roll).
//!
//! **The burst** (`0x3a58b0`): the smoke's timer 6; from the hand's place + 0.3 row 0 − row 1, twenty lines
//! (crate::particles::line, `0x3352c0`: 0.75 long along their way, flying 0.1..0.3 of it a tick, `0x307f7f7f`, 0.05
//! wide, fading 2 a tick) and twenty grey sparks (`0x339848(0.4..0.75 × 210000, growing 500, drag 0.95, …, 10 ticks,
//! 0x7f, 0x404040)` jittered 0.4, flying 12..16 a second plus 1.7 × Ratchet's move on the ground, started a step
//! back), their ways Ratchet's forward row turned 20° (the sparks 12°) about his row 1, then about his row 0 by a
//! sum of 0..36° steps.
//!
//! **The rocket** (pvars: `+0x00` the target's aim point last tick, `+0x0c` the hand, `+0x10` the target, `+0x14` the
//! lock's aim height, `+0x18` its speed a tick, `+0x1c` 2..5 (0 with a lock), `+0x20` ±6..10, `+0x24` (s16) its life
//! (75 / speed ticks), `+0x26` it homes, `+0x27` its ticks (u8), `+0x28` a random angle, `+0x2c` its trail slot, `+0x30`
//! its way, `+0x40` the aim point's offset (0); `+0x1c`, `+0x20` and `+0x28` are read by nothing). Made ([`spawn`]):
//! far 0xff, drawn, its rows kept (mode 0x100); the next of the eight trail slots (`$gp+0x760`, [`Trails`]); at the
//! muzzle; its rows: the arm raised, the gun's (−row 1, −row 0, row 2); without a lock Ratchet's forward row (in first
//! person toward what the view meets 75 out, held within 30°: `0x380ef8`), with one toward its aim point, row 1 the up
//! × row 0; a lock's health block marked (`+0x1e |= 0x80`), one with `+0x0b` set not homed on; the line from the hand
//! (at the muzzle's height) to the muzzle meeting anything (flags 0, Ratchet left out) blows it up on its first tick
//! there; its exhaust point the slot's first point; its size ×2.5.
//!
//! Each tick ([`update_rocket`]): a target dying (state −2 / −3) or out of health is dropped; else its aim point (its
//! place raised along its up row by its health block's aim height). With a target homed on and life under 0x127: the
//! lead — `s = |moved| / speed · sin θ` for the target's move since last tick and θ the angle (signed by the side)
//! between it and the way back to the rocket, wrapped into ±1 (none past 100); `asin s` over 1e-5 turns the way to the
//! target by it about `(rocket − target) × moved`, row 1 the up × it; a target moving and within a step with nothing
//! between (flags 0x12, Ratchet left out) is struck. Else its rows from its way (`+0x30`). It moves a step along row 0;
//! a target of class 0xcea passed is dropped. Out of the world's 2..1021 box it is gone. Its trail's four points shift,
//! its exhaust point first; from its fourth tick its draw is asked for; its sparks ([`sparks`], `0x3adea0`); 0.25 off
//! the ground (flags 2). Then a struck target takes the hit, else the step's line (flags 0, Ratchet left out) hits what
//! it meets (`0x2de878` with the template: the row's damage, its way 1.5 long, flags 0x830000, kind the class's `<< 16
//! | 0x0103`): the world blows it up there, a moby but Ratchet and the hand blows it up there (2). Blowing up: its class
//! sound 0, the big explosion (crate::moby::classes::explosion, `0x3fabe8`) at it facing the camera (a moby struck) or
//! off the face hit, with the ground under it (`0x31ce20(0.5)`) for a moby or a wall (the face's normal under 0.1 up),
//! the blast ([`blast`], `0x3add28`), gone (a rocket the make's line already blew up whose tick meets nothing faces the
//! camera: the game reads the collision's last hit there, whatever query left it). Else its life out or 75 from
//! Ratchet: gone.
//!
//! **The blast** (`0x3add28`): every moby within 4 (flags 0x10, the rocket left out) knocked (`0x31def8(the row's
//! +0x64, 1, 1, rocket, …, flags 0x810000, 3, 1)`, crate::moby::MobyWorld::knock_hits).
//!
//! **The sparks** (`0x3adea0`): along the step from its last place, `speed·10` grey sparks (`0x339848(21000, 200..1000
//! a tick, …, 30..45 ticks)` rising 0.2..1 a second; by `rand & 2` `0x7f7f7f` at 0x40 additive, else `0x404040` at
//! 0x20 normal growing twice as fast), turning ±1..2, on the glow puffs' frame.
//!
//! **The draw** ([`draw`], `0x3ae138`): the glow (crate::fx::halo, `0x2ed370`) at its exhaust point (0.4 behind it
//! along row 0) and the streak (crate::fx::streak, `0x2ed6d8(0, 0.25, …, 4)`) through its slot's four points, both
//! `0x103080` (`$gp+0x768`).
//!
//! Its glow in the hand (`0x2a7ed8`'s case 0x1b): only on the upgraded rows (crate::hero::items).
//!
//! Not ported: the pad rumble (`0x2d6d60(0x294840)` / `0x2d6f38`), the lock cheat (`0x18b270`), the weapon mods'
//! effects on a hit (`0x3f9f70(moby, 0x1b, owner)`), the gravity frames and moving frames (the way flattened to the
//! frame at the make, `0x380e08`, `0x37f538` / `0x37f618` / `0x37f5f8`), the projectile list `0x1b1f30` the rocket
//! joins (`0x3216e0` / `0x321760`: no reader ported), the frame (1) of the standing use's sequence (the use stance's
//! request carries none: frame 0). The upgraded rows (row 0x45 on, the hand 0x1004) are the Megarocket Cannon,
//! crate::moby::classes::megarocket, firing these rockets.

use crate::collision::{CollOutput, QueryFlags};

/// What the rocket's tick reads of the collision's last hit (`0x1bf180`: `+0x18` the moby, `+0x1c` the kind, `+0x20`
/// the point, `+0x40` the face's normal).
#[derive(Clone, Copy, Debug)]
struct Hit {
    moby: Option<usize>,
    kind: i32,
    point: V3,
    normal: V3,
}

impl From<CollOutput> for Hit {
    fn from(o: CollOutput) -> Hit { Hit { moby: o.moby, kind: o.kind, point: o.point, normal: o.normal } }
}
use crate::hero::items::{row, ItemRow, MINIROCKET};
use crate::hero::physics::{add, add_rot, cross, dist, dot, len, scale, set_len, sub, unit, V3};
use crate::moby::hits::HitTemplate;
use crate::moby::{aim, countdown, Ctx, MobyWorld};
use crate::point_lights::PointLight;

/// The hand's class (item 0x1b's row `+0x04`) and the rocket's (`0x3ace00`).
pub const HAND_CLASS: i16 = 0x5cf;
pub const ROCKET: i16 = 0x5da;
/// `t[3]` of the hit templates (`0x45afdf66`).
const NO_KNOCK_LIMIT: f32 = 5627.925;
/// `$gp+0x764`: the exhaust point along row 0.
const EXHAUST: f32 = -0.4;
/// `$gp+0x768`: the draw's colour.
const RGB: u32 = 0x10_3080;
/// The trail slots (`0x23b690`, 0x40 bytes each: four points).
pub const SLOTS: usize = 9;

mod hv {
    pub const COOL: usize = 0x00;
    pub const FLAG: usize = 0x04;
    pub const LOCK: usize = 0x10;
    pub const HOLD: usize = 0x14;
    pub const AIMH: usize = 0x18;
    pub const LIGHT: usize = 0x1c;
    pub const TICKS: usize = 0x1e;
    pub const LAST: usize = 0x20;
    pub const COUNT: usize = 0x24;
    pub const LOOK: usize = 0x28;
    pub const RUMBLE: usize = 0x2c;
}

mod pv {
    pub const POINT: usize = 0x00;
    pub const OWNER: usize = 0x0c;
    pub const TARGET: usize = 0x10;
    pub const AIMH: usize = 0x14;
    pub const SPEED: usize = 0x18;
    pub const R1C: usize = 0x1c;
    pub const R20: usize = 0x20;
    pub const LIFE: usize = 0x24;
    pub const HOMING: usize = 0x26;
    pub const TICKS: usize = 0x27;
    pub const R28: usize = 0x28;
    pub const SLOT: usize = 0x2c;
    pub const DIR: usize = 0x30;
    pub const OFFSET: usize = 0x40;
}

/// The rockets' trail slots: the next (`$gp+0x760`) and each slot's four points (`0x23b690 + 0x40·k`).
#[derive(Clone, Debug, Default)]
pub struct Trails {
    pub next: i32,
    pub points: [[V3; 4]; SLOTS],
}

/// The lock's words in a hand's pvars (the base hand's [`BASE_LOCK`]; the Megarocket Cannon's,
/// crate::moby::classes::megarocket): the lock, its hold, its aim height, the last pick, the pick's count.
pub(super) struct LockPv {
    pub lock: usize,
    pub hold: usize,
    pub aimh: usize,
    pub last: usize,
    pub count: usize,
}
const BASE_LOCK: LockPv = LockPv { lock: hv::LOCK, hold: hv::HOLD, aimh: hv::AIMH, last: hv::LAST, count: hv::COUNT };

/// The Minirocket Tube's row as `0x139568[0x1b]` gives it (crate::upgrades).
fn cur_row(game: &crate::game::GameState) -> Option<&'static ItemRow> { row(game, MINIROCKET) }

/// The record's `+0x64` (the blast's damage) by row: 6 on row 0x1b, 7 on the Megarocket Cannon's 0x45, 0 on its Mega
/// rows 0x63 / 0x64.
fn blast_damage(r: &ItemRow) -> f32 {
    match r.item {
        0x45 => 7.0,
        0x63 | 0x64 => 0.0,
        _ => 6.0,
    }
}

/// `0x2eef88(out, a, b)` = b × a.
fn cross_ba(a: V3, b: V3) -> V3 { cross(b, a) }

/// `0x321c00(a, out, v, axis)`: `v` turned by `a` about `axis` (under 1e-5 kept).
fn turn(v: V3, axis: V3, a: f32) -> V3 {
    if a.abs() < 1e-5 || len(axis) == 0.0 { return v; }
    crate::gravity::rotate(v, unit(axis), a)
}

/// `0x2eeb40(+0x00)`: the cooldown down a frame (1/60 s).
fn cooldown(w: &mut MobyWorld, id: usize) {
    let c = w.mobys[id].pv_f(hv::COOL);
    w.mobys[id].set_pv_f(hv::COOL, if 1.0 / 60.0 < c { c - 1.0 / 60.0 } else { 0.0 });
}

/// `0x2eeae0` on the s16 at `off`: true when it was 0 or just reached it.
fn count_s(w: &mut MobyWorld, id: usize, off: usize) -> bool {
    let mut n = w.mobys[id].pv_s(off) as i32;
    let done = countdown(&mut n);
    w.mobys[id].set_pv_s(off, n as i16);
    done
}

/// `0x2eeab0` on the word at `off`.
fn count_i(w: &mut MobyWorld, id: usize, off: usize) -> bool {
    let mut n = w.mobys[id].pv_i(off);
    let done = countdown(&mut n);
    w.mobys[id].set_pv_i(off, n);
    done
}

/// The muzzle light let go (`0x2fdc48`).
fn free_light(w: &mut MobyWorld, id: usize) {
    let s = w.mobys[id].pv_s(hv::LIGHT);
    if 0 <= s { w.point_lights.free(s as usize); }
    w.mobys[id].set_pv_s(hv::LIGHT, -1);
}

/// `0x3a5d00` (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].pvars.resize(crate::moby::PVAR_BYTES, 0);
    if w.mobys[id].state == 0 {
        let m = &mut w.mobys[id];
        m.state = 1;
        m.set_pv_s(hv::LIGHT, -1);
        m.set_pv_i(hv::COUNT, 5);
        m.set_pv_i(hv::RUMBLE, -1);
        m.set_pv_s(hv::TICKS, 0);
        m.set_pv_i(hv::LOCK, 0);
        m.set_pv_i(hv::LAST, 0);
        m.set_pv_i(hv::LOOK, 0);
    }
    smoke(w, id, ctx);
    cooldown(w, id);
    if ctx.hero.state == 1 { ctx.hero.request_state(0x1d, true); }
    if let Some(a) = ctx.host.anim(&w.mobys[id]) {
        if a.seq_a == a.seq_b && a.flags & 2 != 0 { ctx.host.set_anim(&w.mobys[id], 1, 0, 2); }
    }
    // The look (0x1d with Ratchet hidden) or not: the search's limits and way.
    let hidden = ctx.hero.view_lock != 0;
    let look = ctx.hero.state == 0x1d && hidden;
    let (yaw, pitch) = if look {
        if w.mobys[id].pv_i(hv::LOOK) == 0 {
            w.mobys[id].set_pv_i(hv::LOCK, 0);
            w.mobys[id].set_pv_i(hv::COUNT, 5);
        }
        w.mobys[id].set_pv_i(hv::LOOK, 1);
        let red = 0.0 < w.mobys[id].pv_f(hv::COOL) && ctx.hero.items.has_ammo(ctx.game);
        crate::hud::marker(&mut ctx.game.markers, 1.25, id, if red { 0x6000_00ff } else { 0x600f_ff0f }, None, 0x27);
        (f32::from_bits(0x3d8e_fa36), f32::from_bits(0x3d8e_fa36))
    } else {
        w.mobys[id].set_pv_i(hv::LOOK, 0);
        (f32::from_bits(0x3ea0_d97d), f32::from_bits(0x3e97_e9d9))
    };
    let way = if !look && ctx.hero.items.raised { scale(w.mobys[id].rows[1], -1.0) } else { ctx.hero.rows[0] };
    aim(w, id, ctx, yaw, pitch, way, &BASE_LOCK);
    let ok = if ctx.hero.state == 0x1d { countdown(&mut ctx.hero.items.look_wait) } else {
        ctx.hero.items.look_wait = 0x28;
        true
    };
    let seq = ctx.host.anim(&w.mobys[id]).map_or((0, 0), |a| (a.seq_a, a.seq_b));
    let it = ctx.hero.items;
    if ctx.game.scenes.mode != 2 && w.mobys[id].pv_f(hv::COOL) == 0.0 && it.no_use == 0 && seq.0 <= 1 && it.held & it.trigger != 0 && 0x16 < it.item_ticks && ok {
        if seq.1 == 0 { ctx.host.set_anim(&w.mobys[id], 1, 0, 4); }
        if let Some(r) = cur_row(ctx.game).copied() {
            if ctx.hero.items.take_ammo(ctx.game, 1) {
                fire(w, id, ctx, &r);
            } else if it.pressed & it.trigger != 0 || w.mobys[id].pv_i(hv::FLAG) == 1 {
                ctx.host.play_sound(&w.mobys[id], 1, 0);
                w.mobys[id].set_pv_i(hv::FLAG, 0);
            }
        }
    }
    light(w, id, ctx);
}

/// The aim's search and lock (module docs), its words at `o`.
pub(super) fn aim(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, yaw: f32, pitch: f32, way: V3, o: &LockPv) {
    let h = &*ctx.hero;
    let from = add(h.pos, scale(h.rows[2], 0.75));
    let cone = aim::Cone { yaw, pitch, range: 75.0, near_pitch: yaw, near_yaw: yaw };
    let found = aim::search(w, ctx, cone, from, way, true);
    let aimh = |w: &MobyWorld, t: usize| aim::health(w, t).map_or(0.1, |(_, a)| a);
    match w.mobys[id].pv_link(o.lock) {
        None => {
            if found != w.mobys[id].pv_link(o.last) {
                w.mobys[id].set_pv_i(o.count, 5);
            } else if count_i(w, id, o.count) {
                let a = found.map_or(0.1, |f| aimh(w, f));
                let m = &mut w.mobys[id];
                m.set_pv_i(o.lock, found.map_or(0, |f| f as i32));
                m.set_pv_i(o.hold, 5);
                m.set_pv_f(o.aimh, a);
            }
        }
        Some(lock) => {
            let live = w.mobys.get(lock).is_some_and(|t| !matches!(t.state, 0xfe | 0xfd) && t.drawn != 0)
                && aim::health(w, lock).is_none_or(|(hp, _)| 0.0 < hp)
                && !ctx.hero.items.raised;
            if !live {
                w.mobys[id].set_pv_i(o.lock, 0);
                w.mobys[id].set_pv_i(o.count, 5);
            } else if found == Some(lock) {
                let a = aimh(w, lock);
                w.mobys[id].set_pv_i(o.hold, 5);
                w.mobys[id].set_pv_f(o.aimh, a);
                let t = &w.mobys[lock];
                let at = add(t.pos, scale(t.rows[2], a));
                let d = dist(ctx.host.camera_pos(), t.pos);
                crate::hud::marker(&mut ctx.game.markers, (0.9 - d * 0.006_666_667 * 0.65).abs(), id, 0xff0f_ff0f, Some(at), 0x26);
                ctx.hero.items.aim_target = Some((lock, a));
            } else if count_i(w, id, o.hold) {
                w.mobys[id].set_pv_i(o.lock, 0);
                w.mobys[id].set_pv_i(o.count, 5);
            }
        }
    }
    w.mobys[id].set_pv_i(o.last, found.map_or(0, |f| f as i32));
}

/// The shot of `0x3a5d00` (module docs).
fn fire(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, r: &ItemRow) {
    w.mobys[id].set_pv_i(hv::FLAG, 1);
    if w.mobys[id].pv_link(hv::LOCK).is_none() { point_blank(w, id, ctx, hv::LOCK); }
    ctx.host.play_sound(&w.mobys[id], 0, 0);
    w.mobys[id].set_pv_s(hv::TICKS, 0xf);
    burst(w, id, ctx);
    // `0x3a67a0`: from joint 0, the hand the rocket's owner (`+0xb8`).
    let muzzle = ctx.host.joint(&w.mobys[id], 0).unwrap_or(w.mobys[id].pos);
    let (aimh, lock) = (w.mobys[id].pv_f(hv::AIMH), w.mobys[id].pv_link(hv::LOCK));
    if let Some(k) = spawn(w, ctx, id, muzzle, aimh, lock, r) { w.mobys[k].parent = Some(id); }
    let h = &*ctx.hero;
    if h.group == 0 || (h.state == 2 && h.stick_mag < 0.7) {
        ctx.hero.items.hero_anim = Some((10.0, 0x36));
    } else {
        let g = ctx.hero.group;
        ctx.hero.items.use_start(g);
    }
    ctx.host.set_anim(&w.mobys[id], 2, 0, 0);
    w.mobys[id].set_pv_f(hv::COOL, 1.0 / r.rate);
}

/// The point-blank lock (module docs): `0x2e14b0(1, at, 1, Ratchet)` and the first moby it touched (`0x1bf198`), into
/// the word `lock`.
pub(super) fn point_blank(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, lock_pv: usize) {
    let m = &w.mobys[id];
    let at = add(sub(m.pos, m.rows[1]), scale(m.rows[0], 0.275));
    let Some(sc) = ctx.scene else { return };
    let ids = crate::collision::mobys::coll_sphere_mobys(sc, at, 1.0, QueryFlags(1), ctx.hero_moby);
    w.hits.last_target = ids.first().copied();
    for &t in &ids {
        let x = &w.mobys[t];
        if x.drawn == 0 || x.draw_dist <= 0 || matches!(x.state, 0xfe | 0xfd) || matches!(x.o_class, 0xc87 | 0xecc) { continue; }
        let Some((hp, _)) = aim::health(w, t) else { continue };
        if !(0.0 < hp) && !aim::is_crate(w, t) { continue; }
        if w.class(x.o_class).is_some_and(|c| c.kind == 5) { continue; }
        let lock = w.hits.last_target.map_or(0, |l| l as i32);
        w.mobys[id].set_pv_i(lock_pv, lock);
        break;
    }
}

/// The light's tail of `0x3a5d00` (module docs).
fn light(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let seq = |ctx: &Ctx, w: &MobyWorld| ctx.host.anim(&w.mobys[id]).map_or((0, 0), |a| (a.seq_a, a.seq_b));
    let n = w.mobys[id].pv_s(hv::TICKS) as i32;
    if seq(ctx, w).0 != 2 && 0 < n {
        let (k, s) = if n < 0xd { (1.0 / 12.0, n as f32) } else { (1.0 / 3.0, (0xf - n) as f32) };
        let (r, g, b) = (s * k, s * k * 0.8, s * k * 0.5);
        let way = if ctx.hero.items.raised { scale(w.mobys[id].rows[1], -1.0) } else { ctx.hero.rows[0] };
        let muzzle = ctx.host.joint(&w.mobys[id], 0).unwrap_or(w.mobys[id].pos);
        let pos = add(add(muzzle, way), scale(ctx.hero.rows[2], 0.3));
        let l = PointLight { color: [r, g, b], intensity: 0.0, pos, radius: 8.0 };
        let s = w.mobys[id].pv_s(hv::LIGHT);
        if s == -1 {
            let s = w.point_lights.alloc(l).map_or(-1, |s| s as i16);
            w.mobys[id].set_pv_s(hv::LIGHT, s);
        } else {
            w.point_lights.set(s as usize, l);
        }
        count_s(w, id, hv::TICKS);
    } else if w.mobys[id].pv_s(hv::LIGHT) != -1 {
        free_light(w, id);
        w.mobys[id].set_pv_s(hv::TICKS, 0);
    }
    if seq(ctx, w).1 == 2 {
        free_light(w, id);
        w.mobys[id].set_pv_s(hv::TICKS, 0);
    }
}

/// `0x3a5620` (module docs).
fn smoke(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].timer == 0 { return; }
    w.mobys[id].timer -= 1;
    let m = &w.mobys[id];
    let at = add(add(m.pos, scale(m.rows[1], 0.1)), scale(m.rows[0], 0.275));
    let vel = scale(m.rows[1], f32::from_bits(0x3dcc_ccce));
    let far = ((m.draw_dist >> 5) << 4) as u8;
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let g = &mut ctx.game.rng;
    for _ in 0..4 {
        let r = g.randi(0x10);
        let spin = if g.randi(2) == 0 { r } else { -r };
        if let Some(i) = crate::particles::puff(sys, g, 0.04, 1.0, 0.9, 52_500.0, at, spin, vel, 0x7f20_4080, 0) {
            let rec = &mut sys.pool.recs[i];
            rec[0xa..0xc].copy_from_slice(&8i16.to_le_bytes());
            rec[9] = far.wrapping_add(4);
            rec[0x24..0x28].copy_from_slice(&2u32.to_le_bytes());
            rec[0x2a] = 0x7f;
            rec[0x2b] = rec[0xa];
        }
    }
    let (mut spin, mut size, mut life) = (0x10, 15_750.001f32, 2i16);
    for _ in 0..3 {
        if let Some(i) = crate::particles::puff(sys, g, 0.02, 1.0, 1.0, size, at, spin, vel, 0x7fff_ffff, 0) {
            let roll = g.randi(0xff) as u8;
            let rec = &mut sys.pool.recs[i];
            rec[0xa..0xc].copy_from_slice(&life.to_le_bytes());
            rec[9] = far.wrapping_add(4);
            rec[8] = roll;
            rec[0x24..0x28].copy_from_slice(&2u32.to_le_bytes());
            rec[0x2a] = 0x7f;
            rec[0x2b] = rec[0xa];
        }
        spin = -spin;
        life <<= 1;
        size -= 3150.0;
    }
}

/// `0x3a58b0` (module docs).
fn burst(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    let at = add(m.pos, add(scale(m.rows[0], 0.3), scale(m.rows[1], -1.0)));
    m.timer = 6;
    let (fwd, side) = (ctx.hero.rows[0], ctx.hero.rows[1]);
    let carry = if ctx.hero.grounded_ticks != 0 { scale(ctx.hero.disp, 1.7) } else { [0.0; 3] };
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let g = &mut ctx.game.rng;
    let start = turn(fwd, side, f32::from_bits(0x3eb2_b8c4));
    let (mut d, mut acc) = (start, 0.0f32);
    for _ in 0..20 {
        acc = add_rot(acc, g.randf(0.0, f32::from_bits(0x3f20_d97d)));
        let s = g.randf(f32::from_bits(0x3dcc_ccce), f32::from_bits(0x3e99_999a));
        if let Some(i) = crate::particles::line::line(sys, g, at, scale(d, s)) {
            let rec = &mut sys.pool.recs[i];
            crate::particles::rec::set_v3(rec, 0x10, add(at, scale(d, 0.75)));
            crate::particles::rec::set_v3(rec, 0x20, at);
            crate::particles::rec::set_ff(rec, 0x3c, 2.0);
            crate::particles::rec::set_u32(rec, 0xc, 0x307f_7f7f);
            crate::particles::rec::set_ff(rec, 0x1c, 0.05);
            crate::particles::rec::set_ff(rec, 0x2c, 0.5);
            crate::particles::rec::set_u32(rec, 4, 0x307f_7f7f);
        }
        if 1e-5 < acc.abs() { d = turn(start, fwd, acc); }
    }
    let start = turn(fwd, side, 12.0 * 0.017_453_294);
    let (mut d, mut acc) = (start, 0.0f32);
    for _ in 0..20 {
        acc = add_rot(acc, g.randf(0.0, f32::from_bits(0x3f20_d97d)));
        let s = g.randf(12.0 * 0.016_666_668, 16.0 * 0.016_666_668);
        let v = add(scale(d, s), carry);
        let spin = g.randf_signed(1.0, 2.0) as i32;
        let p: V3 = std::array::from_fn(|k| at[k] + g.randf_signed(0.0, 0.4));
        let size = g.randf(0.4, 0.75);
        crate::particles::spark(sys, g, size * 210_000.0, 500.0, 0.95, 0.0, 0.0, sub(p, v), v, 10, 0x7f, 0x40_4040, spin);
        if 1e-5 < acc.abs() { d = turn(start, fwd, acc); }
    }
}

/// `0x3ace00(aim height, 2, 5, 0.6, hand, muzzle, lock, 0)` (module docs), when the class is loaded.
pub fn spawn(w: &mut MobyWorld, ctx: &mut Ctx, hand: usize, muzzle: V3, aimh: f32, lock: Option<usize>, r: &ItemRow) -> Option<usize> {
    spawn_with(w, ctx, hand, muzzle, aimh, (2.0, 5.0), lock, [0.0; 3], r)
}

/// `0x3ace00(aim height, lo, hi, .., hand, muzzle, lock, offset)`: [`spawn`] with its `+0x1c` drawn from `lo..hi` and
/// the aim point's offset `offset` (`+0x40`; the Megarocket Cannon's volleys, crate::moby::classes::megarocket).
#[allow(clippy::too_many_arguments)]
pub(super) fn spawn_with(w: &mut MobyWorld, ctx: &mut Ctx, hand: usize, muzzle: V3, aimh: f32, range: (f32, f32), lock: Option<usize>, offset: V3, r: &ItemRow) -> Option<usize> {
    if w.class(ROCKET).is_none() { return None; }
    let id = w.create(ROCKET);
    let mut n = w.rockets.next;
    if 8 < n { n = 0; }
    w.rockets.next = (n + 1) % 8;
    let speed = r.speed * 0.016_666_668;
    let lock = lock.filter(|&t| w.mobys.get(t).is_some());
    let at = lock.map(|t| add(w.mobys[t].pos, scale(w.mobys[t].rows[2], aimh)));
    let (hrows, hpos) = (w.mobys[hand].rows, w.mobys[hand].pos);
    let up = ctx.hero.rows[2];
    let rows = if ctx.hero.items.raised {
        [set_len(hrows[1], -1.0), scale(hrows[0], -1.0), hrows[2]]
    } else {
        let r0 = match at {
            Some(p) => unit(sub(p, muzzle)),
            None if ctx.hero.items.fp.on => super::pulse_rifle::view_aim(ctx, muzzle, 1.0, 75.0, f32::from_bits(0x3f06_0a93)),
            None => ctx.hero.rows[0],
        };
        let r1 = unit(cross_ba(r0, up));
        [r0, r1, cross_ba(r1, r0)]
    };
    let g = &mut ctx.game.rng;
    let r1c = g.randf(range.0, range.1);
    let r20 = g.randf_signed(6.0, 10.0);
    let r28 = g.rand_angle();
    {
        let m = &mut w.mobys[id];
        m.pvars.resize(crate::moby::PVAR_BYTES, 0);
        m.update_dist = 0xff;
        m.drawn = 1;
        m.state = 0;
        m.draw_dist = 0xff;
        m.mode |= crate::moby::mode::KEEP_ROWS;
        m.pos = muzzle;
        m.rows = rows;
        m.set_pv_i(pv::SLOT, n);
        m.set_pv_i(pv::OWNER, hand as i32);
        m.set_pv_f(pv::R1C, if lock.is_some() { 0.0 } else { r1c });
        m.set_pv_f(pv::SPEED, speed);
        m.set_pv_f(pv::R20, r20);
        m.set_pv_f(pv::R28, r28);
        m.set_pv_b(pv::HOMING, 1);
        m.set_pv_i(pv::TARGET, lock.map_or(0, |t| t as i32));
        m.set_pv_b(pv::TICKS, 1);
        m.set_pv_v(pv::DIR, rows[0]);
        m.set_pv_v(pv::OFFSET, offset);
        m.set_pv_s(pv::LIFE, (75.0 / speed) as i32 as i16);
    }
    if let Some(t) = lock {
        if aim::health(w, t).is_some() {
            let h = crate::moby::ai::health(&w.mobys[t]);
            let flag = w.mobys[t].pv_b(h + 0x1e) | 0x80;
            w.mobys[t].set_pv_b(h + 0x1e, flag);
            if w.mobys[t].pv_b(h + 0xb) != 0 { w.mobys[id].set_pv_b(pv::HOMING, 0); }
        }
        let m = &mut w.mobys[id];
        m.set_pv_v(pv::POINT, at.unwrap_or(muzzle));
        m.set_pv_f(pv::AIMH, aimh);
    }
    // The line from the hand (at the muzzle's height) to the muzzle (`0x2de878(…, 0, Ratchet)`).
    let from = [hpos[0], hpos[1], muzzle[2]];
    if let Some(o) = w.coll_line(ctx, from, muzzle, 0, ctx.hero_moby) {
        w.mobys[id].timer = 1;
        w.mobys[id].pos = o.point;
    }
    let e = exhaust_point(&w.mobys[id]);
    w.rockets.points[n as usize][0] = e;
    w.rebuild(id, &*ctx.host);
    w.mobys[id].scale *= 2.5;
    Some(id)
}

/// `0x3ae0c8`: the exhaust point, `$gp+0x764` along row 0.
fn exhaust_point(m: &crate::moby::Moby) -> V3 { add(m.pos, scale(m.rows[0], EXHAUST)) }

/// The rocket's rows from a way: row 1 the up × it, row 2 it × row 1.
fn rows_from(r0: V3, up: V3) -> [V3; 3] {
    let r1 = unit(cross_ba(r0, up));
    [r0, r1, cross_ba(r1, r0)]
}

/// Gone (`0x311688`).
fn gone(w: &mut MobyWorld, id: usize) { w.delete(id); }

/// `0x3ad308` (module docs).
pub fn update_rocket(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].pvars.resize(crate::moby::PVAR_BYTES, 0);
    let start = w.mobys[id].pos;
    let up = ctx.hero.gravity.up_at(start);
    let mut target = w.mobys[id].pv_link(pv::TARGET).filter(|&t| w.mobys.get(t).is_some());
    let mut point = [0.0; 3];
    if let Some(t) = target {
        let hb = aim::health(w, t);
        if matches!(w.mobys[t].state, 0xfe | 0xfd) || hb.is_some_and(|(hp, _)| hp <= 0.0) {
            w.mobys[id].set_pv_i(pv::TARGET, 0);
            target = None;
        } else {
            let x = &w.mobys[t];
            point = add(x.pos, scale(x.rows[2], hb.map_or(0.0, |(_, a)| a)));
        }
    }
    let mut struck: Option<Hit> = None;
    let speed = w.mobys[id].pv_f(pv::SPEED);
    let homing = target.is_some_and(|t| w.mobys[id].pv_s(pv::LIFE) < 0x127 && w.mobys[t].state < 0x80 && w.mobys[id].pv_b(pv::HOMING) != 0);
    if let (true, Some(t)) = (homing, target) {
        let pos = w.mobys[id].pos;
        let moved = sub(point, w.mobys[id].pv_v(pv::POINT));
        let lm = len(moved);
        let rel = sub(pos, add(point, w.mobys[id].pv_v(pv::OFFSET)));
        let lr = len(rel);
        if lr != 0.0 && lm != 0.0 {
            let mut a = std::f32::consts::FRAC_PI_2 - (dot(rel, moved) / (lr * lm)).clamp(-1.0, 1.0).asin();
            let axis = cross_ba(moved, rel);
            if dot(rel, cross_ba(moved, axis)) < 0.0 { a = -a; }
            let mut s = lm / speed * a.sin();
            if 100.0 < s.abs() { s = 0.0; }
            while 1.0 < s { s -= 1.0; }
            while s < -1.0 { s += 1.0; }
            let b = s.asin();
            let to = unit(scale(rel, -1.0));
            if 1e-5 < b.abs() {
                let r0 = crate::gravity::rotate(to, unit(axis), b);
                let m = &mut w.mobys[id];
                m.rows = rows_from(r0, up);
                m.set_pv_v(pv::DIR, r0);
            }
            if dist(point, pos) <= speed && w.coll_line(ctx, pos, point, 0x12, ctx.hero_moby).is_none() {
                struck = Some(Hit { moby: Some(t), kind: -1, point, normal: [0.0; 3] });
            }
        }
    } else {
        let m = &mut w.mobys[id];
        m.rows = rows_from(m.pv_v(pv::DIR), up);
    }
    if target.is_some() { w.mobys[id].set_pv_v(pv::POINT, point); }
    {
        let m = &mut w.mobys[id];
        m.pos = add(m.pos, scale(m.rows[0], speed));
    }
    if target.is_some_and(|t| w.mobys[t].o_class == 0xcea) {
        if dot(sub(point, w.mobys[id].pos), w.mobys[id].rows[0]) < 0.0 {
            w.mobys[id].set_pv_i(pv::TARGET, 0);
            target = None;
            struck = None;
        }
    }
    if !w.mobys[id].pos.iter().all(|&c| (2.0..=1021.0).contains(&c)) {
        gone(w, id);
        return;
    }
    // The trail's points shift, the exhaust point first; from the fourth tick the draw.
    let slot = (w.mobys[id].pv_i(pv::SLOT).clamp(0, SLOTS as i32 - 1)) as usize;
    let e = exhaust_point(&w.mobys[id]);
    let p = &mut w.rockets.points[slot];
    p.copy_within(0..3, 1);
    p[0] = e;
    let ticks = w.mobys[id].pv_b(pv::TICKS).wrapping_add(1);
    w.mobys[id].set_pv_b(pv::TICKS, ticks);
    if 4 <= ticks { w.register_draw(id); }
    sparks(w, id, ctx, start);
    // 0.25 off the ground (flags 2).
    let pos = w.mobys[id].pos;
    if let Some(o) = w.coll_line(ctx, pos, sub(pos, scale(up, 0.25)), 2, None) {
        w.mobys[id].pos = add(o.point, scale(up, 0.25));
    }
    let Some(r) = cur_row(ctx.game).copied() else { return };
    let (class, vel) = (w.mobys[id].o_class, scale(w.mobys[id].rows[0], speed));
    let d = set_len(vel, 1.5);
    let mut t = HitTemplate::new(r.damage, Some(id), 0x83_0000, [d[0], d[1], d[2], NO_KNOCK_LIMIT]);
    t.kind = (class as u16 as u32) << 16 | 0x0103;
    let coll = match struck {
        Some(o) => {
            if let Some(s) = target { w.deliver_hit_any(s, &t, [0.0; 4]); }
            Some(o)
        }
        None => {
            let pos = w.mobys[id].pos;
            let o = w.coll_line(ctx, start, pos, 0, ctx.hero_moby).map(Hit::from);
            if let Some(h) = o {
                if let Some(m) = h.moby { w.deliver_hit_any(m, &t, [h.point[0], h.point[1], h.point[2], 1.0]); }
            }
            o
        }
    };
    let mut exclude = None;
    if let Some(o) = &coll {
        match o.moby {
            None => {
                if 0 < o.kind {
                    w.mobys[id].timer = 1;
                    w.mobys[id].pos = o.point;
                }
            }
            Some(m) if Some(m) == ctx.hero_moby || Some(m) == w.mobys[id].pv_link(pv::OWNER) => {}
            Some(_) => {
                w.mobys[id].timer = 2;
                w.mobys[id].pos = o.point;
                exclude = Some(id);
            }
        }
    }
    if w.mobys[id].timer != 0 {
        ctx.host.play_sound(&w.mobys[id], 0, 0);
        let pos = w.mobys[id].pos;
        let up2 = ctx.hero.gravity.up_at(pos);
        let (normal, ground) = match &coll {
            Some(o) if o.moby.is_none() || 0 < o.kind => {
                let n = unit(o.normal);
                (n, dot(n, up2).abs() < 0.1)
            }
            _ => (unit(sub(ctx.host.camera_pos(), pos)), true),
        };
        let g = if ground { ground_under(w, ctx, pos, 0.5) } else { 0.0 };
        super::explosion::start(w, ctx, g, pos, normal);
        blast(w, id, ctx, &r, exclude);
        gone(w, id);
        return;
    }
    if count_s(w, id, pv::LIFE) || 75.0 < dist(w.mobys[id].pos, ctx.hero.pos) { gone(w, id); }
}

/// `0x31ce20(h, at, 0)`: the ground under `at` — the line from `h` above it down to 0.01 (flags 2) meeting it, its
/// height; else 0.
fn ground_under(w: &MobyWorld, ctx: &Ctx, at: V3, h: f32) -> f32 {
    w.coll_line(ctx, [at[0], at[1], at[2] + h], [at[0], at[1], 0.01], 2, None).map_or(0.0, |o| o.point[2])
}

/// `0x3add28(rocket, up, spared)` (module docs).
fn blast(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, r: &ItemRow, spared: Option<usize>) {
    let radius = if ctx.game.upgrade_level(MINIROCKET) != 0 { 3.0 } else { 4.0 };
    let pos = w.mobys[id].pos;
    let Some(sc) = ctx.scene else { return };
    let ids = crate::collision::mobys::coll_sphere_mobys(sc, pos, radius, QueryFlags(0x10), Some(id));
    w.hits.last_target = ids.first().copied();
    w.knock_hits(blast_damage(r), 1.0, 1.0, id, pos, &ids, spared, 0x81_0000, 3, 1);
}

/// `0x3adea0(rocket, from)` (module docs).
fn sparks(w: &MobyWorld, id: usize, ctx: &mut Ctx, from: V3) {
    let m = &w.mobys[id];
    let speed = m.pv_f(pv::SPEED);
    let n = (speed * 10.0) as i32;
    if n <= 0 { return; }
    let step = set_len(sub(m.pos, from), speed / n as f32);
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let g = &mut ctx.game.rng;
    let mut p = from;
    for _ in 0..n {
        let spin = g.randf_signed(1.0, 3.0) as i32;
        let f = g.randf(0.2, 1.0);
        let vel = scale(crate::particles::GRAVITY, -(f * 0.016_666_668));
        let life = g.rand_range(0x1e, 0x2d) as i16;
        let mut grow = g.randf(200.0, 1000.0);
        let (alpha, rgb) = if g.rand() & 2 == 0 { (0x40, 0x7f_7f7f) } else {
            grow += grow;
            (0x20, 0x40_4040)
        };
        let i = crate::particles::spark(sys, g, 21_000.0, grow, 1.0, 0.0, 0.0, p, vel, life, alpha, rgb, spin);
        p = add(p, step);
        if let Some(i) = i {
            let def = sys.def_first(0x17);
            let rec = &mut sys.pool.recs[i];
            rec[2] = def;
            rec[3] = if alpha == 0x40 { 0x48 } else { 0x44 };
        }
    }
}

/// `0x3ae138` (module docs).
pub fn draw(w: &MobyWorld, id: usize, rng: &mut crate::rng::Rng, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let m = &w.mobys[id];
    let head = exhaust_point(m);
    let slot = (m.pv_i(pv::SLOT).clamp(0, SLOTS as i32 - 1)) as usize;
    crate::fx::halo(out, rng, head, RGB, cam);
    crate::fx::streak(out, rng, 0.0, 0.25, head, &w.rockets.points[slot], RGB, cam);
}
