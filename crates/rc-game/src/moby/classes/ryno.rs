//! The RYNO II (item 0x2c, crate::hero::items::ROWS; L00's record `0x264b80`: attachment 0, stance 2 (both hands),
//! use sequence 0x36, the firing arm's 0x46 crouched and none moving, icon 0xea6c, name 0x32ee, no upgrade row, 6.5
//! shots a second, damage 100, the blast's 10 (`+0x64`), 100 missiles; its own speed `+0x40` is 0; sold by the
//! Gadgetron vendor, crate::vendor::GADGETRON_ITEMS): the hand moby **0x0a67** (`0x3c9ed8`, called by the item pass while
//! it is the hand item; `G/GADGET.WAD`), the missile **0x0ed7** (`0x40b9b0`, made by `0x40b348`; in every level's class
//! list; the update has no Ghidra function on L00..L03 (L01 `0x40d2a8`, L02 `0x436f40`, L03 `0x417aa0`): read from its
//! listing) and its explosion, class 0x139c (`0x415520`: the blast's level copy, crate::moby::blast).
//!
//! **The hand** (pvars: `+0x00` the cooldown (seconds), `+0x10` the point-blank pick, `+0x1c` (s16) the muzzle's point
//! light (crate::point_lights, −1 none), `+0x1e` (s16) the light's ticks, `+0x20` cleared each tick (read by nothing),
//! `+0x24` the pad rumble's handle; the port keeps the empty click's flag `$gp+0x3a60` at `+0x04`). State 0: the
//! light none, the rest cleared; **1**. Each tick: the cooldown runs down; the look stance 1 goes to its twin 0x1d,
//! where the look wait (`0x18c354`) counts down (else it is 0x28).
//! * **The search** (`0x3257a0(25.6°, 25.6°, 48, 45°, 45°, hand, way, list of 0x20, no mobys, crates, {0xc87,
//!   0xecc})`, crate::moby::aim::search_list_except): from the hand along Ratchet's forward row (`0x18a580`; the arm
//!   raised at a wall (`0x18c30c`): the gun's −row 1), after its claims of the targets age (crate::moby::claims,
//!   `0x37f440`, the hand the owner). Every target listed is marked (`0x2db498(|0.9 − d·0.0067·0.65|, 0, 90°, hand,
//!   0xff0fff0f, aim point, FX 0x26, −1, 4)`, d the camera's distance to it).
//! * **The shot**: in play (`0x1a8f00` 0), the cooldown out, the items not kept away (`0x18c0dc`), ○ held, in the hand
//!   past 0x16 ticks and the look wait out — with ammo (`0x2d43e0`): the use stance (`0x2af5e0`); the point-blank pick
//!   (the sphere of 1, flags 1, Ratchet left out, at the hand's −row 1 + 0.275 row 0: an enemy-run moby drawn, not
//!   dying, not 0xc87 / 0xecc, with health left or a crate, not of class type 5 makes the sphere's first moby
//!   (`0x1bf198`) the pick); its class sound 1; the light's ticks 15; the burst ([`burst`], `0x3ca920`); its sequence
//!   2 (blend 1) at speed 0.9; without a point-blank pick the claims' pick of the listed targets (`0x37f338`, a fresh one
//!   claimed for 20 ticks); the missile ([`spawn`], `0x40b348(the pick's aim height, 2, 5, 0.6, hand, muzzle, target,
//!   0)`: the target the lock `0x18aa18` when there is one, else the pick; the hand its owner `+0xb8`) at the muzzle;
//!   a missile spent and the flag set. None left: with ○ pressed or the flag set, its class sound 2, the flag cleared.
//!   Either way the cooldown is 1 / the row's rate.
//! * **The muzzle**: (0.513, −1, 0.183) in the hand's rows from its place.
//! * **The light** (`0x2fdb60(8, 0, …)` / `0x2fdc48`): its sequence A at 2 — once run out, back to 1 (blend 1) — frees
//!   it and clears its ticks; else while its ticks run, a point light of radius 8 at the muzzle + the search's way + 0.3
//!   up Ratchet's up row, `(t, 0.8·t, 0.5·t)` for `t` rising over its first 3 ticks (`(15 − n) / 3`) then falling over
//!   12 (`n / 12`); out, freed.
//!
//! **The burst** (`0x3ca920`): the hand's timer (`+0xbc`) 6; from the muzzle ten lines (crate::particles::line,
//! `0x3352c0`: 0.75 long along their way, flying 0.1..0.3 of it a tick, `0x307f7f7f`, 0.05 wide, fading 2 a tick) and
//! ten sparks (`0x339848(0.4..0.75 × 210000, growing 500, drag 0.95, …, 10 ticks, 0x7f, 0x505050)` on type 0x5b's frame
//! 1, jittered 0.4, flying 12..16 a second plus 1.7 × Ratchet's move on the ground, started a step back), their ways
//! Ratchet's forward row turned 20° (the sparks 12°) about his row 1, then about his row 0 by a sum of 0..36° steps.
//!
//! **The missile** (pvars: `+0x00` the target's aim point, `+0x0c` the hand, `+0x10` its target (0: the make clears it
//! and nothing sets it), `+0x14` the aim height, `+0x18` its speed a tick, `+0x1c` its corkscrew's tilt (4..10°; 0 with
//! a target), `+0x20` the tilt's turn a tick (±6..10°), `+0x24` (s16) its life (300 ticks), `+0x26` it homes, `+0x27` its
//! ticks (u8), `+0x28` the corkscrew's angle, `+0x2c` its trail slot, `+0x30` its way, `+0x40` 0, `+0x50` its loop
//! sound, `+0x54` its gravity frame). Made ([`spawn`]): far 0xff, drawn, its rows kept (mode 0x100); the next of the 32
//! trail slots (`$gp+0x1f88`, [`Trails`]); the speed the **Minirocket Tube's** row's (`0x139568[0x1b]`'s `+0x40`, 60 a
//! second: the RYNO II's own is 0); at the muzzle, its rows:
//! * in the stance 0x1d: the camera's, moved to the camera's place − 0.2 its up − 0.25 its left row; its way toward
//!   the target's aim point, else toward what the line 30 along the view (flags 0, Ratchet left out) meets or its end;
//! * the arm raised: the gun's (−row 1, −row 0, row 2);
//! * else Ratchet's forward row, or toward the target's aim point; row 1 the up × row 0.
//!
//! A target's health block marked (`+0x1e |= 0x80`; `+0x0b` set: not homed on); the line from the hand (at the muzzle's
//! height) to the muzzle meeting anything (flags 0, Ratchet left out) blows it up on its first tick there; its trail's
//! first point its exhaust point; its size ×2.5.
//!
//! Each tick ([`update_missile`]): its loop sound (class sound 1, mode 4) kept playing; its rows from its way (row 1
//! the up × it); a corkscrew's tilt above 0.0021 turns them by it about row 1, then the three by the angle about the way
//! and rows 1 and 2 again by it about the new row 0, and the angle turns by `+0x20`. It moves a step along row 0. Out of
//! the world's 2..1021 box it is gone. Its trail's four points shift, its exhaust point (0.4 behind it, `$gp+0x1f8c`)
//! first; from its fourth tick its draw is asked for; its sparks ([`sparks`], `0x40c598`); 0.3 off the ground (flags
//! 2). The step's line (flags 0, Ratchet left out) hits what it meets (`0x2de878` with the template: the row's damage
//! along its way, flags 0x830000, kind the class's `<< 16 | 0x0103`): the world blows it up there, a moby but its hand
//! blows it up there (2). Blowing up: its loop sound stopped, its class sound 0, at the collision's last hit point the
//! explosion (`0x415520(0.5, at, shake, 15, 0, 0, 8, 15, …, light, …, one in five (`0x31b9a0(5)`) the general
//! explosion)`: crate::moby::blast::burst_shaking with a 20-tick shake, and then the general explosion's light of 10,
//! crate::moby::blast::explode), the blast ([`blast`], `0x40c4b8`), gone. Else its life out or 60 from Ratchet: gone.
//!
//! **The blast** (`0x40c4b8`): every moby within 4 (3 with the **Minirocket Tube** upgraded: `0x139568[0x1b]`'s row
//! level; flags 0x10, the missile left out) knocked (`0x31def8(the row's +0x64, 1, 1, missile, …, flags 0x810000, 3,
//! 1)`, crate::moby::MobyWorld::knock_hits), the moby struck spared.
//!
//! **The sparks** (`0x40c598`): along the step from its last place, `⌊speed·10⌋ / 4` grey sparks (`0x339848(21000,
//! 200..1000 a tick, drag 1, …, 30..50 ticks)` rising 0.2..1 a second; by `rand & 2` `0x404040` else `0x7f7f7f`, both at
//! 0x50 normal), turning ±1..3, on the glow puffs' frame.
//!
//! **The draw** ([`draw`], `0x40c830`): the glow (crate::fx::halo, `0x2ed370`) at its exhaust point and the streak
//! (crate::fx::streak, `0x2ed6d8(0, 0.25, …, 4)`) through its slot's four points, both `0x803010` (`$gp+0x1f90`).
//!
//! Its glow in the hand (`0x2a7ed8`'s case 0x2c): crate::hero::items.
//!
//! Not ported: the pad rumble (`0x2d6d60(0x296060)` / `0x2d6f38`; its stop's `0x2af848` only runs while the use is
//! over, where it does nothing), the lock cheat (`0x18b270`), the missile's homing (its `+0x10` target is cleared by the
//! make and set by nothing: the branch `0x40bafc..0x40bda0` never runs), the gravity frames and moving frames
//! (`0x37f538` / `0x37f618` / `0x37f5f8`, `0x380e08`), the projectile list `0x1b1f30` (`0x3216e0` / `0x321760`), the
//! weapon mods' effects on a hit; the 0x139c moby's own update and draw (`0x415850` / `0x4158d0`: its life of 60 ticks,
//! the heat shimmer and screen flash, as crate::moby::blast leaves the 0x13a0's) and its danger area (`0x1adb0c`, 0
//! here); the normal and the ground under the blow-up (`0x31ce20`), which the explosion does not take.

use crate::collision::QueryFlags;
use crate::hero::items::{row, ItemRow, MINIROCKET};
use crate::hero::physics::{add, add_rot, cross, dist, len, scale, set_len, sub, unit, V3};
use crate::moby::hits::HitTemplate;
use crate::moby::{aim, countdown, Ctx, MobyWorld};
use crate::point_lights::PointLight;

/// The hand's class (item 0x2c's row `+0x04`), the missile's (`0x40b348`) and the explosion's (`0x415520`).
pub const HAND_CLASS: i16 = 0xa67;
pub const MISSILE: i16 = 0xed7;
pub const EXPLOSION: i16 = 0x139c;
/// The item.
pub const ITEM: i32 = 0x2c;
/// `t[3]` of the hit templates (`0x45afdf66`).
const NO_KNOCK_LIMIT: f32 = 5627.925;
/// `$gp+0x1f8c`: the exhaust point along row 0.
const EXHAUST: f32 = -0.4;
/// `$gp+0x1f90`: the draw's colour.
const RGB: u32 = 0x80_3010;
/// The trail slots (`0x255b20`, 0x40 bytes each: four points).
pub const SLOTS: usize = 0x20;
/// The record's `+0x64`: the blast's damage (one row).
const BLAST_DAMAGE: f32 = 10.0;
/// The muzzle in the hand's rows (`0x3f0353f8`, −1, `0x3e3b645a`).
const MUZZLE: V3 = [f32::from_bits(0x3f03_53f8), -1.0, f32::from_bits(0x3e3b_645a)];
/// The search's limits (`0x3edf66f4`; near `0x3f490fdc`) and range.
const SEARCH: aim::Cone = aim::Cone {
    yaw: f32::from_bits(0x3edf_66f4),
    pitch: f32::from_bits(0x3edf_66f4),
    range: 48.0,
    near_pitch: f32::from_bits(0x3f49_0fdc),
    near_yaw: f32::from_bits(0x3f49_0fdc),
};
/// `0x1b00a0`: the search's excluded classes.
const EXCLUDED: [i16; 2] = [0xc87, 0xecc];
/// The claim a fresh pick makes (ticks).
const CLAIM_TICKS: i32 = 0x14;
/// `0x3c888889`.
const SIXTIETH: f32 = 0.016_666_668;
/// `0x3c8efa36`: a degree.
const DEG: f32 = 0.017_453_294;

mod hv {
    pub const COOL: usize = 0x00;
    pub const FLAG: usize = 0x04;
    pub const BLANK: usize = 0x10;
    pub const LIGHT: usize = 0x1c;
    pub const TICKS: usize = 0x1e;
    pub const CLEARED: usize = 0x20;
    pub const RUMBLE: usize = 0x24;
}

mod pv {
    pub const POINT: usize = 0x00;
    pub const OWNER: usize = 0x0c;
    pub const TARGET: usize = 0x10;
    pub const AIMH: usize = 0x14;
    pub const SPEED: usize = 0x18;
    pub const TILT: usize = 0x1c;
    pub const TURN: usize = 0x20;
    pub const LIFE: usize = 0x24;
    pub const HOMING: usize = 0x26;
    pub const TICKS: usize = 0x27;
    pub const ANGLE: usize = 0x28;
    pub const SLOT: usize = 0x2c;
    pub const DIR: usize = 0x30;
    pub const OFFSET: usize = 0x40;
    pub const SOUND: usize = 0x50;
}

/// The missiles' trail slots: the next (`$gp+0x1f88`) and each slot's four points (`0x255b20 + 0x40·k`).
#[derive(Clone, Debug, Default)]
pub struct Trails {
    pub next: i32,
    pub points: [[V3; 4]; SLOTS],
}

/// The RYNO II's row as `0x139568[0x2c]` gives it.
fn cur_row(game: &crate::game::GameState) -> Option<&'static ItemRow> { row(game, ITEM) }

/// `0x2eef88(out, a, b)` = b × a.
fn cross_ba(a: V3, b: V3) -> V3 { cross(b, a) }

/// `0x31fe98` + `0x321b28`: `v` turned by `a` about `axis`.
fn turn(v: V3, axis: V3, a: f32) -> V3 {
    if len(axis) == 0.0 { return v; }
    crate::gravity::rotate(v, unit(axis), a)
}

/// `0x2ef318(out, v, rows)` + the place: `v` in moby `m`'s rows from its place.
fn in_rows(m: &crate::moby::Moby, v: V3) -> V3 {
    let r = m.rows;
    add(m.pos, std::array::from_fn(|c| r[0][c] * v[0] + r[1][c] * v[1] + r[2][c] * v[2]))
}

/// The muzzle (module docs).
fn muzzle(w: &MobyWorld, id: usize) -> V3 { in_rows(&w.mobys[id], MUZZLE) }

/// Ratchet's moby's up row (`0x18c0b0 + 0xe0`).
fn hero_up(w: &MobyWorld, ctx: &Ctx) -> V3 { ctx.hero_moby.and_then(|h| w.mobys.get(h)).map_or(ctx.hero.rows[2], |m| m.rows[2]) }

/// The search's way: Ratchet's forward row, the gun's −row 1 with the arm raised at a wall.
fn way(w: &MobyWorld, id: usize, ctx: &Ctx) -> V3 { if ctx.hero.items.raised { scale(w.mobys[id].rows[1], -1.0) } else { ctx.hero.rows[0] } }

/// `0x2eeb40(+0x00)`: the cooldown down a frame (1/60 s).
fn cooldown(w: &mut MobyWorld, id: usize) {
    let c = w.mobys[id].pv_f(hv::COOL);
    w.mobys[id].set_pv_f(hv::COOL, if SIXTIETH < c { c - SIXTIETH } else { 0.0 });
}

/// `0x2eeae0` on the s16 at `off`: true when it was 0 or just reached it.
fn count_s(w: &mut MobyWorld, id: usize, off: usize) -> bool {
    let mut n = w.mobys[id].pv_s(off) as i32;
    let done = countdown(&mut n);
    w.mobys[id].set_pv_s(off, n as i16);
    done
}

/// The muzzle light let go (`0x2fdc48`) and its ticks cleared.
fn free_light(w: &mut MobyWorld, id: usize) {
    let s = w.mobys[id].pv_s(hv::LIGHT);
    if s != -1 {
        if 0 <= s { w.point_lights.free(s as usize); }
        w.mobys[id].set_pv_s(hv::LIGHT, -1);
        w.mobys[id].set_pv_s(hv::TICKS, 0);
    }
}

/// `0x3c9ed8` (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].pvars.resize(crate::moby::PVAR_BYTES, 0);
    if w.mobys[id].state == 0 {
        let m = &mut w.mobys[id];
        m.set_pv_s(hv::TICKS, 0);
        m.set_pv_s(hv::LIGHT, -1);
        m.set_pv_i(hv::BLANK, 0);
        m.set_pv_i(hv::CLEARED, 0);
        m.state = 1;
        m.set_pv_i(hv::RUMBLE, -1);
    }
    cooldown(w, id);
    if ctx.hero.state == 1 { ctx.hero.request_state(0x1d, true); }
    let ok = if ctx.hero.state == 0x1d { countdown(&mut ctx.hero.items.look_wait) } else {
        ctx.hero.items.look_wait = 0x28;
        true
    };
    w.mobys[id].set_pv_i(hv::CLEARED, 0);
    let dir = way(w, id, ctx);
    {
        let mut c = std::mem::take(&mut w.claims);
        c.age(w, id);
        w.claims = c;
    }
    let from = w.mobys[id].pos;
    let list = aim::search_list_except(w, ctx, SEARCH, from, dir, true, 0x20, &[], &EXCLUDED);
    let cam = ctx.host.camera_pos();
    for &t in &list {
        let at = aim::aim_point(w, t);
        let d = dist(cam, w.mobys[t].pos);
        crate::hud::marker(&mut ctx.game.markers, (0.9 - d * 0.006_666_667 * 0.65).abs(), id, 0xff0f_ff0f, Some(at), 0x26);
    }
    let it = ctx.hero.items;
    if ctx.game.scenes.mode == crate::scene::PLAY && w.mobys[id].pv_f(hv::COOL) == 0.0 && it.no_use == 0 && it.held & it.trigger != 0 && 0x16 < it.item_ticks && ok {
        if let Some(r) = cur_row(ctx.game).copied() {
            if ctx.hero.items.has_ammo(ctx.game) {
                fire(w, id, ctx, &list);
            } else if it.pressed & it.trigger != 0 || w.mobys[id].pv_i(hv::FLAG) == 1 {
                ctx.host.play_sound(&w.mobys[id], 2, 0);
                w.mobys[id].set_pv_i(hv::FLAG, 0);
            }
            w.mobys[id].set_pv_f(hv::COOL, 1.0 / r.rate);
        }
    }
    light(w, id, ctx);
}

/// The shot of `0x3c9ed8` (module docs).
fn fire(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, list: &[usize]) {
    let g = ctx.hero.group;
    ctx.hero.items.use_start(g);
    let blank = point_blank(w, id, ctx);
    ctx.host.play_sound(&w.mobys[id], 1, 0);
    w.mobys[id].set_pv_s(hv::TICKS, 0xf);
    burst(w, id, ctx);
    ctx.host.set_anim(&w.mobys[id], 2, 0, 1);
    ctx.host.set_anim_speed(&w.mobys[id], 0.9);
    // The rumble's handle (`0x2d6d60` gives its slot): the rumble itself is not ported.
    w.mobys[id].set_pv_i(hv::RUMBLE, 0);
    let at = muzzle(w, id);
    let pick = match blank {
        Some(b) => Some(b),
        None => match w.claims.pick(id, list) {
            Some((t, fresh)) => {
                if fresh { w.claims.claim(id, t, CLAIM_TICKS); }
                Some(t)
            }
            None => None,
        },
    };
    let aimh = pick.and_then(|p| aim::health(w, p)).map_or(0.0, |(_, a)| a);
    let target = ctx.hero.items.aim_target.map(|(t, _)| t).filter(|&t| w.mobys.get(t).is_some()).or(pick);
    if let Some(k) = spawn(w, ctx, id, at, aimh, target) { w.mobys[k].parent = Some(id); }
    ctx.hero.items.take_ammo(ctx.game, 1);
    w.mobys[id].set_pv_i(hv::FLAG, 1);
}

/// The point-blank pick (module docs): `0x2e14b0(1, at, 1, Ratchet)` and the first moby it touched (`0x1bf198`).
fn point_blank(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> Option<usize> {
    let m = &w.mobys[id];
    let at = add(sub(m.pos, m.rows[1]), scale(m.rows[0], 0.275));
    let sc = ctx.scene?;
    let ids = crate::collision::mobys::coll_sphere_mobys(sc, at, 1.0, QueryFlags(1), ctx.hero_moby);
    w.hits.last_target = ids.first().copied();
    for &t in &ids {
        let x = &w.mobys[t];
        if x.drawn == 0 || x.draw_dist <= 0 || matches!(x.state, 0xfe | 0xfd) || EXCLUDED.contains(&x.o_class) { continue; }
        let Some((hp, _)) = aim::health(w, t) else { continue };
        if !(0.0 < hp) && !aim::is_crate(w, t) { continue; }
        if w.class(x.o_class).is_some_and(|c| c.kind == 5) { continue; }
        let pick = w.hits.last_target;
        w.mobys[id].set_pv_i(hv::BLANK, pick.map_or(0, |p| p as i32));
        return pick;
    }
    None
}

/// The light's tail of `0x3c9ed8` (module docs).
fn light(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let seq_a = |ctx: &Ctx, w: &MobyWorld| ctx.host.anim(&w.mobys[id]).map(|a| (a.seq_a, a.flags));
    if let Some((2, flags)) = seq_a(ctx, w) {
        if flags & 2 != 0 { ctx.host.set_anim(&w.mobys[id], 1, 0, 1); }
        if seq_a(ctx, w).is_some_and(|(s, _)| s == 2) {
            free_light(w, id);
            return;
        }
    }
    let n = w.mobys[id].pv_s(hv::TICKS) as i32;
    if n <= 0 {
        free_light(w, id);
        return;
    }
    let (k, s) = if n < 0xd { (1.0 / 12.0, n as f32) } else { (1.0 / 3.0, (0xf - n) as f32) };
    let (r, g, b) = (s * k, s * k * 0.8, s * k * 0.5);
    let pos = add(add(muzzle(w, id), way(w, id, ctx)), scale(hero_up(w, ctx), 0.3));
    let l = PointLight { color: [r, g, b], intensity: 0.0, pos, radius: 8.0 };
    let s = w.mobys[id].pv_s(hv::LIGHT);
    if s == -1 {
        let s = w.point_lights.alloc(l).map_or(-1, |s| s as i16);
        w.mobys[id].set_pv_s(hv::LIGHT, s);
    } else if 0 <= s {
        w.point_lights.set(s as usize, l);
    }
    count_s(w, id, hv::TICKS);
}

/// `0x3ca920` (module docs).
fn burst(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let at = muzzle(w, id);
    w.mobys[id].timer = 6;
    let (fwd, side) = (ctx.hero.rows[0], ctx.hero.rows[1]);
    let carry = if ctx.hero.grounded_ticks != 0 { scale(ctx.hero.disp, 1.7) } else { [0.0; 3] };
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let g = &mut ctx.game.rng;
    let start = turn(fwd, side, f32::from_bits(0x3eb2_b8c4));
    let (mut d, mut acc) = (start, 0.0f32);
    for _ in 0..10 {
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
    let start = turn(fwd, side, 12.0 * DEG);
    let (mut d, mut acc) = (start, 0.0f32);
    for _ in 0..10 {
        acc = add_rot(acc, g.randf(0.0, f32::from_bits(0x3f20_d97d)));
        let s = g.randf(12.0 * SIXTIETH, 16.0 * SIXTIETH);
        let v = add(scale(d, s), carry);
        let spin = g.randf_signed(1.0, 2.0) as i32;
        let p: V3 = std::array::from_fn(|k| at[k] + g.randf_signed(0.0, 0.4));
        let size = g.randf(0.4, 0.75);
        if let Some(i) = crate::particles::spark(sys, g, size * 210_000.0, 500.0, 0.95, 0.0, 0.0, sub(p, v), v, 10, 0x7f, 0x50_5050, spin) {
            let def = sys.def_frame(0x5b, 1);
            sys.pool.recs[i][2] = def;
        }
        if 1e-5 < acc.abs() { d = turn(start, fwd, acc); }
    }
}

/// `0x40b348(aim height, 2, 5, 0.6, hand, muzzle, target, 0)` (module docs), when the class is loaded.
pub fn spawn(w: &mut MobyWorld, ctx: &mut Ctx, hand: usize, muzzle: V3, aimh: f32, target: Option<usize>) -> Option<usize> {
    if w.class(MISSILE).is_none() { return None; }
    let id = w.create(MISSILE);
    let mut n = w.ryno.next;
    if (SLOTS as i32) < n { n = 0; }
    w.ryno.next = (n + 1) % SLOTS as i32;
    let speed = row(ctx.game, MINIROCKET).map_or(0.0, |r| r.speed) * SIXTIETH;
    let target = target.filter(|&t| w.mobys.get(t).is_some());
    let point = target.map(|t| add(w.mobys[t].pos, scale(w.mobys[t].rows[2], aimh)));
    let up = hero_up(w, ctx);
    let (hrows, hpos) = (w.mobys[hand].rows, w.mobys[hand].pos);
    let mut pos = muzzle;
    let rows = if ctx.hero.state == 0x1d {
        let [f, l, u] = ctx.host.camera_rows();
        let cam = ctx.host.camera_pos();
        pos = add(add(cam, scale(u, -0.2)), scale(l, -0.25));
        let to = match point {
            Some(p) => p,
            None => {
                let far = add(cam, scale(f, 30.0));
                w.coll_line(ctx, cam, far, 0, ctx.hero_moby).map_or(far, |o| o.point)
            }
        };
        let r0 = set_len(sub(to, pos), 1.0);
        let r1 = set_len(cross_ba(r0, up), 1.0);
        [r0, r1, cross_ba(r1, r0)]
    } else if ctx.hero.items.raised {
        [set_len(hrows[1], -1.0), scale(hrows[0], -1.0), hrows[2]]
    } else {
        let (r0, up) = match point {
            Some(p) => (set_len(sub(p, pos), 1.0), up),
            None => (ctx.hero.rows[0], ctx.hero.rows[2]),
        };
        let r1 = set_len(cross_ba(r0, up), 1.0);
        [r0, r1, cross_ba(r1, r0)]
    };
    let g = &mut ctx.game.rng;
    let tilt = g.randf(4.0, 10.0);
    let spin = g.randf_signed(6.0, 10.0);
    let angle = g.rand_angle();
    {
        let m = &mut w.mobys[id];
        m.pvars.resize(crate::moby::PVAR_BYTES, 0);
        m.drawn = 1;
        m.update_dist = 0xff;
        m.state = 0;
        m.draw_dist = 0xff;
        m.mode |= crate::moby::mode::KEEP_ROWS;
        m.set_pv_i(pv::SOUND, -1);
        m.set_pv_i(pv::SLOT, n);
        m.pos = pos;
        m.rows = rows;
        m.set_pv_i(pv::OWNER, hand as i32);
        m.set_pv_f(pv::TILT, if target.is_some() { 0.0 } else { tilt });
        m.set_pv_f(pv::SPEED, speed);
        m.set_pv_f(pv::TURN, spin);
        m.set_pv_f(pv::ANGLE, angle);
        m.set_pv_b(pv::HOMING, 1);
        m.set_pv_i(pv::TARGET, 0);
        m.set_pv_b(pv::TICKS, 1);
        m.set_pv_v(pv::DIR, rows[0]);
        m.set_pv_v(pv::OFFSET, [0.0; 3]);
    }
    if let Some(t) = target {
        if aim::health(w, t).is_some() {
            let h = crate::moby::ai::health(&w.mobys[t]);
            let flag = w.mobys[t].pv_b(h + 0x1e) | 0x80;
            w.mobys[t].set_pv_b(h + 0x1e, flag);
            if w.mobys[t].pv_b(h + 0xb) != 0 { w.mobys[id].set_pv_b(pv::HOMING, 0); }
        }
    }
    w.mobys[id].set_pv_s(pv::LIFE, 300);
    if let Some(t) = target {
        let p = w.mobys[t].pos;
        let m = &mut w.mobys[id];
        m.set_pv_v(pv::POINT, [p[0], p[1], p[2] + aimh]);
        m.set_pv_f(pv::AIMH, aimh);
    }
    // The line from the hand (at the muzzle's height) to the muzzle (`0x2de878(…, 0, Ratchet)`).
    let from = [hpos[0], hpos[1], muzzle[2]];
    if let Some(o) = w.coll_line(ctx, from, muzzle, 0, ctx.hero_moby) {
        w.mobys[id].timer = 1;
        w.mobys[id].pos = o.point;
    }
    let e = exhaust_point(&w.mobys[id]);
    w.ryno.points[n as usize][0] = e;
    w.rebuild(id, &*ctx.host);
    w.mobys[id].scale *= 2.5;
    Some(id)
}

/// `0x40c7c0`: the exhaust point, `$gp+0x1f8c` along row 0.
fn exhaust_point(m: &crate::moby::Moby) -> V3 { add(m.pos, scale(m.rows[0], EXHAUST)) }

/// The loop sound let go (`0x3602f0`).
fn stop_sound(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let s = w.mobys[id].pv_i(pv::SOUND);
    if s != -1 && ctx.host.sound_playing(s) { ctx.host.stop_sound(s); }
    w.mobys[id].set_pv_i(pv::SOUND, -1);
}

/// `0x40b9b0` (module docs).
pub fn update_missile(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].pvars.resize(crate::moby::PVAR_BYTES, 0);
    let start = w.mobys[id].pos;
    let up = ctx.hero.gravity.up_at(start);
    let s = w.mobys[id].pv_i(pv::SOUND);
    if s == -1 || !ctx.host.sound_playing(s) {
        let h = ctx.host.play_sound(&w.mobys[id], 1, 4);
        w.mobys[id].set_pv_i(pv::SOUND, h);
    }
    // Its rows from its way, then the corkscrew.
    {
        let m = &mut w.mobys[id];
        let r0 = m.pv_v(pv::DIR);
        let r1 = set_len(cross_ba(r0, up), 1.0);
        m.rows = [r0, r1, cross_ba(r1, r0)];
        let tilt = m.pv_f(pv::TILT);
        if 0.0021 < tilt.abs() {
            let f0 = m.rows[0];
            let [mut a, b, mut c] = m.rows;
            a = turn(a, b, tilt * DEG);
            c = turn(c, b, tilt * DEG);
            let angle = m.pv_f(pv::ANGLE);
            let (a2, b2, c2) = (turn(a, f0, angle), turn(b, f0, angle), turn(c, f0, angle));
            let (b3, c3) = (turn(b2, a2, angle), turn(c2, a2, angle));
            m.rows = [a2, b3, c3];
            let t = m.pv_f(pv::TURN);
            m.set_pv_f(pv::ANGLE, add_rot(angle, t * DEG));
        }
        let speed = m.pv_f(pv::SPEED);
        m.pos = add(m.pos, scale(m.rows[0], speed));
    }
    if !w.mobys[id].pos.iter().all(|&c| (2.0..=1021.0).contains(&c)) {
        w.delete(id);
        return;
    }
    // The trail's points shift, the exhaust point first; from the fourth tick the draw.
    let slot = (w.mobys[id].pv_i(pv::SLOT).clamp(0, SLOTS as i32 - 1)) as usize;
    let e = exhaust_point(&w.mobys[id]);
    let p = &mut w.ryno.points[slot];
    p.copy_within(0..3, 1);
    p[0] = e;
    let ticks = w.mobys[id].pv_b(pv::TICKS).wrapping_add(1);
    w.mobys[id].set_pv_b(pv::TICKS, ticks);
    if 4 <= ticks { w.register_draw(id); }
    sparks(w, id, ctx, start);
    // 0.3 off the ground (flags 2).
    let pos = w.mobys[id].pos;
    let mut last_hit = None;
    if let Some(o) = w.coll_line(ctx, pos, sub(pos, scale(up, 0.3)), 2, None) {
        w.mobys[id].pos = add(o.point, scale(up, 0.3));
        last_hit = Some(o.point);
    }
    let damage = cur_row(ctx.game).map_or(0.0, |r| r.damage);
    let (class, speed) = (w.mobys[id].o_class, w.mobys[id].pv_f(pv::SPEED));
    let d = set_len(scale(w.mobys[id].rows[0], speed), 1.0);
    let mut t = HitTemplate::new(damage, Some(id), 0x83_0000, [d[0], d[1], d[2], NO_KNOCK_LIMIT]);
    t.kind = (class as u16 as u32) << 16 | 0x0103;
    let pos = w.mobys[id].pos;
    let mut spared = None;
    if let Some(o) = w.coll_line(ctx, start, pos, 0, ctx.hero_moby) {
        last_hit = Some(o.point);
        if let Some(m) = o.moby { w.deliver_hit_any(m, &t, [o.point[0], o.point[1], o.point[2], 1.0]); }
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
                spared = Some(id);
            }
        }
    }
    if w.mobys[id].timer != 0 {
        stop_sound(w, id, ctx);
        ctx.host.play_sound(&w.mobys[id], 0, 0);
        // At the collision's last hit (`0x1bf1a0`): this tick's line's or the ground's; a missile the make's line
        // already blew up whose tick met nothing stays where that line left it.
        if let Some(p) = last_hit { w.mobys[id].pos = p; }
        let at = w.mobys[id].pos;
        let light = ctx.game.rng.randi(5) == 0;
        let b = crate::moby::blast::Blast { size: 0.5, bursts: [0xf, 0, 0], puffs: 8, streaks: 0xf, light: true, shake: true };
        crate::moby::blast::burst_shaking(w, ctx, at, &b, 0x14);
        if light {
            // `0x320538(0, 0, 0, 0, 9, 0, 20·0.5, …)`: only its light.
            let e = crate::moby::blast::Explosion { near: 9.0, light: 10.0, sound: -1, ..Default::default() };
            crate::moby::blast::explode(w, ctx, None, Some(at), &e);
        }
        blast(w, id, ctx, spared);
        w.delete(id);
        return;
    }
    if count_s(w, id, pv::LIFE) || 60.0 < dist(w.mobys[id].pos, ctx.hero.pos) {
        stop_sound(w, id, ctx);
        w.delete(id);
    }
}

/// `0x40c4b8(missile, up, spared)` (module docs).
fn blast(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, spared: Option<usize>) {
    let radius = if ctx.game.upgrade_level(MINIROCKET) != 0 { 3.0 } else { 4.0 };
    let pos = w.mobys[id].pos;
    let Some(sc) = ctx.scene else { return };
    let ids = crate::collision::mobys::coll_sphere_mobys(sc, pos, radius, QueryFlags(0x10), Some(id));
    w.hits.last_target = ids.first().copied();
    w.knock_hits(BLAST_DAMAGE, 1.0, 1.0, id, pos, &ids, spared, 0x81_0000, 3, 1);
}

/// `0x40c598(missile, from)` (module docs).
fn sparks(w: &MobyWorld, id: usize, ctx: &mut Ctx, from: V3) {
    let m = &w.mobys[id];
    let speed = m.pv_f(pv::SPEED);
    let n = (speed * 10.0) as i32 / 4;
    if n <= 0 { return; }
    let step = set_len(sub(m.pos, from), speed / n as f32);
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let g = &mut ctx.game.rng;
    let mut p = from;
    for _ in 0..n {
        let spin = g.randf_signed(1.0, 3.0) as i32;
        let f = g.randf(0.2, 1.0);
        let vel = scale(crate::particles::GRAVITY, -(f * SIXTIETH));
        let life = g.rand_range(0x1e, 0x32) as i16;
        let grow = g.randf(200.0, 1000.0);
        let rgb = if g.rand() & 2 != 0 { 0x40_4040 } else { 0x7f_7f7f };
        let i = crate::particles::spark(sys, g, 21_000.0, grow, 1.0, 0.0, 1.0, p, vel, life, 0x50, rgb, spin);
        p = add(p, step);
        if let Some(i) = i {
            let def = sys.def_first(0x17);
            let rec = &mut sys.pool.recs[i];
            rec[2] = def;
            rec[3] = 0x44;
        }
    }
}

/// `0x40c830` (module docs).
pub fn draw(w: &MobyWorld, id: usize, rng: &mut crate::rng::Rng, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let m = &w.mobys[id];
    let head = exhaust_point(m);
    let slot = (m.pv_i(pv::SLOT).clamp(0, SLOTS as i32 - 1)) as usize;
    crate::fx::halo(out, rng, head, RGB, cam);
    crate::fx::streak(out, rng, 0.0, 0.25, head, &w.ryno.points[slot], RGB, cam);
}
