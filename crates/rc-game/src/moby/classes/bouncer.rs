//! The Bouncer (item 0x25, crate::hero::items::ROWS; L00's record `0x264560`: attachment 0, stance 2 (both hands), use
//! sequence 0x89, no firing arm's sequences, icon 0xea73, 0.3 a second, damage 6 (the bomblets' `+0x64` 3), 25 bombs;
//! its upgrade's row `+0x3a` 0x4c): the hand moby **0x0e14** (`0x3ff2e8`, called by the item pass while it is the hand
//! item), the bomb it throws **0x0e64** (`0x4003a0`, made by `0x400260`) and the bomblets the bomb bursts into
//! **0x0ece** (`0x40abb0`, made by `0x40aa58`).
//!
//! **The hand** (pvars: `+0x00` the landing point, `+0x10` its normal, `+0x24` a landing found, `+0x28` the rumble's
//! handle, `+0x2c` the use's ticks; the cooldown in seconds is a global, `gp+0x1bb0`, [`Mem`]). State **0**: the
//! cooldown past one shot's (1 / the row's rate) or below 0 is cleared, **1**. Every tick: Ratchet's look stance 1 is
//! sent to its twin 0x1d (`0x2c4560(0x1d, 1)`); its sequence run out, back to 1 (`0x31b760(hand, 1, 0, 2, 0)`); the
//! muzzle ([`muzzle`], `0x3ff780`: 0.28 along its row 0 and −0.85 along its row 1); outside the twin `0x18c354` is set
//! to 0x28, in it counted down; the throw from the muzzle (`0x37e328(1.6, 10, 0.25 a tick, 14.6 a second², ..)`, no
//! target (the lock cheat's `0x18b270` only): 10 ahead of Ratchet, or with L1 the camera's look;
//! crate::moby::classes::gravity_bomb::launch_with) and the arc followed to where it lands (`0x37e988`,
//! crate::moby::classes::gravity_bomb::landing_with) for the marker ([`draw_marker`], `0x3ff990` → `0x37dfc0(1, ..)`).
//! The cooldown runs down (`0x2eeb40`); run out this tick with a bomb left: its class sound 3. The use's ticks out
//! end the use (`0x2af848`). A press (`0x138324`) with no bomb: class sound 1. In play (`0x1a8f00` = 0), the cooldown
//! out, the items not kept away (`0x18c0dc`), its sequence not blending from 2 (`+0x42` ≤ 1), ○ held, in the hand 0x17
//! ticks and the look wait out: with a bomb, class sound 0, one bomb (`0x2d42d0`), the muzzle's smoke and light
//! ([`smoke`], `0x3ff810`), its sequence 2, the bomb ([`throw`], `0x3ff7d0`), the use stance (`0x2af5e0`: Ratchet's
//! sequence 0x89 standing, nothing on the move), the use's ticks 0x10; with or without, the cooldown 1 / the row's
//! rate.
//!
//! **The bomb** (pvars: `+0x00` its velocity, `+0x10` its spin (a quaternion; rolling: its axis and `+0x1c` its angle),
//! `+0x20` the roll's ticks, `+0x24` its bounces, `+0x28` the smoke's ticks, `+0x2c` the upgraded hand's, `+0x30` its
//! moving frame). Made (`0x400260`): update and draw distance 0xff, its rows kept (`+0x34 | 0x100`) from three random
//! angles, a spin of 3..6° a tick about its row 1, 3 times its size (the upgraded 5), 15 ticks of smoke. **0**
//! (flying): it moves and falls 14.6 a second² (`0x37ec68`); while the smoke's ticks run, three grey puffs along the
//! step, smaller as they run out; the sweep ([`sweep`], `0x37edb8`: a 0.3 sphere (0.5), else the line; not Ratchet nor
//! the hand) bounces it off anything (0.45 of the normal speed kept; its fourth bounce stops on a floor). A surface of
//! kind 1 / 2 (`0x3813b0`) blows it up; else it takes the sweep's place and velocity and its spin. Then every moby
//! with mode 0x4000 within 0.4 (0.6) of it (`0x2e14b0`, flags 1, not Ratchet) takes the row's damage (flags
//! 0x830000 along its way, kind `0x0e64 << 16 | 0x0103`) and it blows up. Else a bounce spins it anew (3..6° a tick
//! about the up × its way across) with its class sound 1; on the fourth it sits at the sphere's centre and rolls: **1**
//! for 30 ticks, along the ground at 0.4·3° a tick (×1.6), its spin 3° a tick about that axis, both ×0.9 a tick, then
//! it blows up. Its blob shadow 0.4 across (×1.6) outside the gravity frames.
//!
//! **The burst** ([`burst`], `0x400a30`): its class sound 0, two blasts (crate::moby::blast, `0x3f8348(s, 20, .., 1,
//! 0xe, 10, 0xf, 10, 0x14, ..)`: s 0.7 then 1, the shake asked), then 17 bomblets (`gp+0x1bf4`) flung 4..9 a second
//! (12) a random way 50..90° up (40..75°) in a frame about the up (`0x37ecc0`), each the hand's (`+0xb8`).
//!
//! **The bomblet** (pvars: `+0x00` its velocity, `+0x10` its spin, `+0x20` (s16) its life, 240..360 ticks, `+0x22`
//! (s16) its arming, 70 + its index, `+0x24` the upgraded hand's, `+0x28` its moving frame; once its size (2)). **0**:
//! it moves, falls as the bomb and sweeps (a 0.125 sphere (0.25), 0.95 of the normal speed kept, always bouncing; not
//! Ratchet, the hand nor a bomb), taking the sweep's place and velocity and its spin. A bounce hops it: of the mobys
//! within 7 (flags 1, not Ratchet) the nearest drawn one of class type 5 with health left, not a crate — its way across
//! turned toward that one's aim point at its speed across (at least 1/15 a tick); its spin anew about the up × its way
//! across (3..6° a tick); its class sound 0. Its life out, or a bounce off a surface of kind 1 / 2, blows it up; else
//! armed (counted down to 0), a crate or a moby with health left and its health block's `+0x06` 0 within 2 (8) of it
//! (`0x2e14b0`, flags 1, not Ratchet) takes the row's `+0x64` (flags 0x830000, kind `0x0ece << 16 | 0x0103`; the first
//! only, `0x31d378`) and it blows up: its class sound 1, a blast (`0x3f8348(0.6 (2.4), 20, .., 0, 6, 5, 6, 8, 10, 0,
//! ..)`), gone. Its blob shadow 0.225 across (0.5).
//!
//! Its glow in the hand: none (`0x2a7ed8` has no case 0x25).
//!
//! Not ported: the upgraded hand 0x0e9e (rows 0x4c on; this module keeps its constants behind the bomb's `+0x2c`); the
//! pad rumble (`0x2d6d60` / `0x2d6f38`); the weapon mods' effects and statistics on a hit (`0x3f9f70`); the lock cheat
//! (`0x18b270`); the gravity frames and the moving frames (`0x326448`'s other gravities, `0x380e08`, `0x37f538` /
//! `0x37f618` / `0x37f5f8`: the port's crate::gravity stands in for the up); the write-only word `0x1b2b64` (0x14 at the
//! start and at an empty press); the blasts' heat shimmer (crate::moby::blast). The sweep's flag `+0x50` (a moby of
//! class type 5 bounces) is left unset by both callers (the stack's leftovers): taken as 0. The bomb's fourth bounce
//! puts it at the sweep's sphere centre (`0x1bf1b0`), which a line hit leaves from an earlier query: the line's point
//! here.

use crate::collision::QueryFlags;
use crate::hero::items::{row, BOUNCER};
use crate::hero::physics::{add, cross, dist, dot, len, scale, set_len, sub, unit, V3};
use crate::moby::hits::HitTemplate;
use crate::moby::{aim, countdown, mode, Ctx, MobyWorld};
use crate::quat::{axis_angle, mul, Q};

/// The hand's class (item 0x25's row `+0x04`), the upgraded hand's (not ported), the bomb's and the bomblet's.
pub const HAND_CLASS: i16 = 0x0e14;
pub const UPGRADED_HAND: i16 = 0x0e9e;
pub const BOMB: i16 = 0x0e64;
pub const BOMBLET: i16 = 0x0ece;

/// `0x37e328`'s arguments: the least reach (with L1), the reach ahead, the speed across a tick and the gravity a tick²
/// (14.6 a second²; the bomb's and the bomblets' fall too).
const LEAST: f32 = f32::from_bits(0x3fcc_cccd);
const REACH: f32 = 10.0;
const SPEED: f32 = 0.25;
const GRAVITY: f32 = f32::from_bits(0x3b84_e478);
/// `gp+0x1ba0`: the muzzle in the hand's rows.
const MUZZLE: V3 = [f32::from_bits(0x3e8f_5c29), f32::from_bits(0xbf59_999a), 0.0];
/// The use's ticks (`+0x2c`).
const USE_TICKS: i32 = 0x10;
/// The spins' least and most a tick (3°, 6°).
const SPIN_MIN: f32 = f32::from_bits(0x3d56_7752);
const SPIN_MAX: f32 = f32::from_bits(0x3dd6_7752);
/// `gp+0x1bfc` / `+0x1c08`: the bomb's size; `+0x1c00` / `+0x1c0c` its sphere; `+0x1bf0` the bounce's share of the
/// normal speed; `+0x1bf8` its shadow and roll (`+0x1c04` the upgraded's factor); `+0x1c10` the roll's damping;
/// `+0x1c14` its ticks; `+0x1bf4` the bomblets.
const BOMB_SCALE: [f32; 2] = [3.0, 5.0];
const BOMB_RADIUS: [f32; 2] = [f32::from_bits(0x3e99_999a), 0.5];
const BOMB_BOUNCE: f32 = f32::from_bits(0x3ee6_6666);
const BOMB_SHADOW: f32 = f32::from_bits(0x3ecc_cccd);
const UPGRADED: f32 = f32::from_bits(0x3fcc_cccd);
const ROLL_DAMP: f32 = f32::from_bits(0x3f66_6666);
const ROLL_TICKS: i32 = 0x1e;
const BOMBLETS: i32 = 0x11;
/// The smoke's ticks at the throw.
const SMOKE_TICKS: i32 = 0xf;
/// `gp+0x1f64` / `+0x1f68`: the bomblet's size; `+0x1f5c` / `+0x1f6c` its sphere; `+0x1f50` the bounce's share;
/// `+0x1f60` / `+0x1f70` its shadow; `+0x1f54..+0x1f58` its life; `+0x1f74` its arming.
const BOMBLET_SCALE: [f32; 2] = [1.0, 2.0];
const BOMBLET_RADIUS: [f32; 2] = [0.125, 0.25];
const BOMBLET_BOUNCE: f32 = f32::from_bits(0x3f73_3333);
const BOMBLET_SHADOW: [f32; 2] = [f32::from_bits(0x3e66_6666), 0.5];
const BOMBLET_LIFE: (i32, i32) = (0xf0, 0x168);
const BOMBLET_ARMING: i16 = 0x46;
/// Row 0x25's `+0x64`: the bomblets' damage (the port's rows keep `+0x60` only).
const BOMBLET_DAMAGE: f32 = 3.0;
/// `t[3]` of the hits' template (`0x45afdf66`).
const NO_KNOCK_LIMIT: f32 = 5627.987;
/// `0x37edb8`'s least up of a floor at a bounce that may stop (`1.5`: never a floor; the bomb's fourth: 0).
const NEVER_FLOOR: f32 = 1.5;

/// The hand's pvars.
mod hand {
    pub const LAND: usize = 0x00;
    pub const NORMAL: usize = 0x10;
    pub const FOUND: usize = 0x24;
    pub const USE: usize = 0x2c;
}

/// The bomb's and the bomblet's pvars.
mod pv {
    pub const VEL: usize = 0x00;
    pub const SPIN: usize = 0x10;
    pub const ANGLE: usize = 0x1c;
    pub const ROLL: usize = 0x20;
    pub const BOUNCES: usize = 0x24;
    pub const SMOKE: usize = 0x28;
    pub const UP: usize = 0x2c;
    /// The bomblet's.
    pub const LIFE: usize = 0x20;
    pub const ARMING: usize = 0x22;
    pub const LET_UP: usize = 0x24;
}

/// The Bouncer's global: its cooldown in seconds (`gp+0x1bb0`), kept across the hand's makings.
#[derive(Clone, Debug, Default)]
pub struct Mem {
    pub cooldown: f32,
}

/// `0x321b28(out, v, q)`: `v` turned by `q` (`q·v·q*`).
fn qrot(q: Q, v: V3) -> V3 {
    let t = mul(mul(q, [v[0], v[1], v[2], 0.0]), [-q[0], -q[1], -q[2], q[3]]);
    [t[0], t[1], t[2]]
}

/// The rows turned by the spin at `+0x10` (`0x321b28` on `+0xc0` / `+0xd0` / `+0xe0`).
fn spin_rows(w: &mut MobyWorld, id: usize, q: Q) {
    let m = &mut w.mobys[id];
    m.rows = m.rows.map(|r| qrot(q, r));
}

/// `0x2eef88(out, a, b)` = b × a.
fn cross_ba(a: V3, b: V3) -> V3 { cross(b, a) }

/// `0x3813b0(surface)`: 2 for 0 / 0xd / 0xe (the waters), 1 for 1 / 3 / 0xb / 0xc, else 0.
fn surface_kind(s: i32) -> i32 {
    match s {
        0 | 0xd | 0xe => 2,
        1 | 3 | 0xb | 0xc => 1,
        _ => 0,
    }
}

/// `0x3ff780`: the muzzle, 0.28 along the hand's row 0 and −0.85 along its row 1.
fn muzzle(w: &MobyWorld, id: usize) -> V3 {
    let m = &w.mobys[id];
    add(m.pos, add(scale(m.rows[0], MUZZLE[0]), scale(m.rows[1], MUZZLE[1])))
}

/// `0x3ff2e8` (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].pvars.resize(crate::moby::PVAR_BYTES, 0);
    let Some(r) = row(ctx.game, BOUNCER).copied() else { return };
    let shot = 1.0 / r.rate;
    if w.mobys[id].state == 0 {
        w.mobys[id].state = 1;
        let c = w.bouncer.cooldown;
        if shot < c || c < 0.0 { w.bouncer.cooldown = 0.0; }
        w.mobys[id].set_pv_i(hand::USE, 0);
        w.mobys[id].set_pv_i(0x28, -1);
    }
    let hero_state = ctx.hero.state;
    // `0x2c4560(0x1d, 1)`: at once in the original (the look wait below already counts).
    if hero_state == 1 { ctx.hero.request_state(0x1d, true); }
    if let Some(a) = ctx.host.anim(&w.mobys[id]) {
        if a.seq_a == a.seq_b && a.flags & 2 != 0 { ctx.host.set_anim(&w.mobys[id], 1, 0, 2); }
    }
    let from = muzzle(w, id);
    let look_ok = if matches!(hero_state, 0x1d | 1) {
        countdown(&mut ctx.hero.items.look_wait)
    } else {
        ctx.hero.items.look_wait = 0x28;
        true
    };
    let vel = super::gravity_bomb::launch_with(ctx, LEAST, REACH, SPEED, GRAVITY, from, None);
    let found = super::gravity_bomb::landing_with(w, ctx, id, from, vel, GRAVITY);
    {
        let m = &mut w.mobys[id];
        if let Some((p, n)) = found {
            m.set_pv_v(hand::LAND, p);
            m.set_pv_v(hand::NORMAL, n);
        }
        m.set_pv_i(hand::FOUND, found.is_some() as i32);
    }
    // `0x2eeb40(gp+0x1bb0)`: run out this tick with a bomb left, its class sound 3.
    let was_out = w.bouncer.cooldown <= 0.0;
    let c = w.bouncer.cooldown;
    w.bouncer.cooldown = if 1.0 / 60.0 < c { c - 1.0 / 60.0 } else { 0.0 };
    if !was_out && w.bouncer.cooldown <= 0.0 && ctx.hero.items.has_ammo(ctx.game) { ctx.host.play_sound(&w.mobys[id], 3, 0); }
    {
        let mut t = w.mobys[id].pv_i(hand::USE);
        if t != 0 && countdown(&mut t) {
            let idle = ctx.hero.idle_seq();
            ctx.hero.items.use_end(hero_state, ctx.hero_view.seq_b, idle);
        }
        w.mobys[id].set_pv_i(hand::USE, t);
    }
    let it = ctx.hero.items;
    if it.pressed & it.trigger != 0 && !it.has_ammo(ctx.game) { ctx.host.play_sound(&w.mobys[id], 1, 0); }
    let blending_from_fire = ctx.host.anim(&w.mobys[id]).is_some_and(|a| 1 < a.seq_a);
    if ctx.game.scenes.mode == crate::scene::PLAY && w.bouncer.cooldown == 0.0 && it.no_use == 0 && !blending_from_fire && it.held & it.trigger != 0 && 0x17 <= it.item_ticks && look_ok {
        if ctx.hero.items.has_ammo(ctx.game) {
            ctx.host.play_sound(&w.mobys[id], 0, 0);
            ctx.hero.items.take_ammo(ctx.game, 1);
            smoke(w, ctx, from, vel);
            ctx.host.set_anim(&w.mobys[id], 2, 0, 2);
            if let Some(b) = throw(w, ctx, id, from, vel) { w.mobys[b].parent = Some(id); }
            ctx.hero.items.use_start(ctx.hero.group);
            w.mobys[id].set_pv_i(hand::USE, USE_TICKS);
        }
        w.bouncer.cooldown = shot;
    }
    w.register_draw(id);
}

/// `0x3ff990` → `0x37dfc0(1, landing, normal, 0)`: the marker where the throw lands, when one was found; not in
/// Ratchet's states 0x17..0x1b, 0x6f, 0x62 or in a scene.
pub fn draw_marker(w: &MobyWorld, id: usize, hero_state: i32, scene_mode: i32, frame: i32, out: &mut Vec<crate::fx::FxQuad>) {
    let m = &w.mobys[id];
    if m.pvars.len() < 0x28 || m.pv_i(hand::FOUND) == 0 || matches!(hero_state, 0x17..=0x1b | 0x6f | 0x62) || scene_mode == crate::scene::SCENE { return; }
    super::gravity_bomb::marker_quads(m.pv_v(hand::LAND), m.pv_v(hand::NORMAL), super::gravity_bomb::MARKER_RGBA, frame, out);
}

/// `0x3ff810(hand, at, vel)`: twenty grey puffs at the muzzle (`0x335bb8(0.3, 1, 1.01, 70000..150000, at, 0, a random
/// way 1/60000..1/150 a tick, grey 10..0x7f, 1)`: 120 ticks), two draws of `0x31b9a0(2)` before each left unused;
/// then the light `0x29f250` 1.5 out along the throw (`0x39e7f0`: white-blue, radius 10, 10 ticks).
fn smoke(w: &mut MobyWorld, ctx: &mut Ctx, at: V3, vel: V3) {
    if let Some(sys) = ctx.parts.as_deref_mut() {
        let g = &mut ctx.game.rng;
        for _ in 0..20 {
            g.randi(2);
            g.randi(2);
            let size = g.randf(70_000.0, 150_000.0);
            let c = g.rand_range(10, 0x7f) as u32;
            let v = g.rand_vec(f32::from_bits(0x378b_cf66), f32::from_bits(0x3bda_740f));
            if let Some(i) = crate::particles::puff(sys, g, f32::from_bits(0x3e99_999a), 1.0, f32::from_bits(0x3f81_47ae), size, at, 0, v, 0x7f00_0000 | c << 16 | c << 8 | c, 1) {
                let r = &mut sys.pool.recs[i];
                crate::particles::rec::set_i16(r, 0xa, 0x78);
                crate::particles::rec::set_u32(r, 0x24, 2);
                r[0x2a] = 0x7f;
                r[0x2b] = r[0xa];
            }
        }
    }
    let t = super::light::Template { offset: set_len(vel, 1.5), ..LIGHT };
    super::light::spawn(w, ctx, &t, at);
}

/// `0x29f250`: the throw's light (its offset written by [`smoke`]): blue-white flaring for 10 ticks, its radius 10.
const LIGHT: super::light::Template = super::light::Template {
    offset: [0.0; 3],
    bytes: [0x00, 0x00, 0xff, 0x00, 0x19, 0x19, 0x00, 0xff, 0x00, 0x19, 0x19, 0x00, 0x64, 0x00, 0x19, 0x19],
    radius: [10.0; 3],
    grow: 0.01,
    shrink: 0.01,
    life: 10,
    flags: 0x0008_0000,
    draw: 0xff,
    holds: [0; 4],
    periods: [0; 4],
    delay: 0,
};

/// `0x3ff7d0` → `0x400260(at, vel, upgraded)`: the bomb (module docs), None when its class is not loaded.
fn throw(w: &mut MobyWorld, ctx: &mut Ctx, hand: usize, at: V3, vel: V3) -> Option<usize> {
    if w.class(BOMB).is_none() {
        eprintln!("rc_game::bouncer: class {BOMB:#x} is not loaded: no bomb");
        return None;
    }
    let up = (w.mobys[hand].o_class == UPGRADED_HAND) as usize;
    let id = w.create(BOMB);
    let g = &mut ctx.game.rng;
    let rot = [g.rand_angle(), g.rand_angle(), g.rand_angle()];
    let rows = crate::moby::attach::rows_of(rot);
    let spin = g.randf(SPIN_MIN, SPIN_MAX);
    let m = &mut w.mobys[id];
    m.update = Some(update_bomb);
    m.mode = (m.mode & !mode::NO_UPDATE) | mode::KEEP_ROWS;
    m.pvars.resize(crate::moby::PVAR_BYTES, 0);
    m.update_dist = 0xff;
    m.draw_dist = 0xff;
    m.drawn = 1;
    m.state = 0;
    m.pos = at;
    (m.rot, m.rows) = (rot, rows);
    m.set_pv_v(pv::VEL, vel);
    m.set_pv_i(pv::UP, up as i32);
    m.set_pv_i(pv::BOUNCES, 0);
    m.set_pv_i(pv::SMOKE, SMOKE_TICKS);
    m.set_pv_q(pv::SPIN, axis_angle(spin, rows[1]));
    m.scale *= BOMB_SCALE[up];
    w.rebuild(id, &*ctx.host);
    w.changed = true;
    Some(id)
}

/// What [`sweep`] found: 0 nothing, 1 a bounce (the velocity reflected), 2 a stop; the place (the sphere's pushed
/// centre or the line's point; a stop: the hit's point), the velocity, the surface, the sphere's centre.
struct Swept {
    res: i32,
    at: V3,
    vel: V3,
    surface: i32,
    centre: V3,
}

/// `0x37edb8` with a record of `radius` (no lift), the bounce's share of the normal speed `keep` and the least up of a
/// floor `floor`, flags 0, the moby `id` left out, the classes `pass` (a hit on one is none): the sphere at `to` (its
/// pushed-out centre), else the line from `from` (its point). The unit normal less than `floor` along the up at the
/// moby bounces `vel` (reflected, `keep` of the normal speed kept); else it stops at the hit's point.
fn sweep(w: &MobyWorld, ctx: &Ctx, id: usize, from: V3, to: V3, vel: V3, radius: f32, keep: f32, floor: f32, pass: &[i16]) -> Swept {
    let none = Swept { res: 0, at: to, vel, surface: -1, centre: to };
    let Some(c) = ctx.coll else { return none };
    let up = ctx.hero.gravity.up_at(w.mobys[id].pos);
    let hit = crate::collision::coll_sphere_m(c, ctx.scene, to, radius, QueryFlags(0), Some(id)).map(|h| (h.pushed_centre.unwrap_or(h.point), h))
        .or_else(|| w.coll_line(ctx, from, to, 0, Some(id)).map(|h| (h.point, h)));
    let Some((at, h)) = hit else { return none };
    if h.moby.is_some_and(|t| w.mobys.get(t).is_some_and(|x| pass.contains(&x.o_class))) { return none; }
    let surface = h.surface_id();
    let n = unit(h.normal);
    if dot(up, n) < floor {
        // `0x2ef2a8`: reflected; then the normal part `keep`.
        let r = sub(vel, scale(n, 2.0 * dot(vel, n)));
        let along = scale(n, dot(r, n));
        let v = add(sub(r, along), scale(along, keep));
        return Swept { res: 1, at, vel: v, surface, centre: at };
    }
    Swept { res: 2, at: h.point, vel, surface, centre: at }
}

/// The blob shadow outside the gravity frames (`0x1a8ff0` / `0x1a8ff4`, `0x31d930`).
fn shadow(w: &mut MobyWorld, ctx: &Ctx, size: f32, id: usize) {
    if !ctx.hero.gravity.on { crate::shadows::request(w, ctx, size, id); }
}

/// `0x4003a0` (module docs).
pub fn update_bomb(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let up = (w.mobys[id].pv_i(pv::UP) != 0) as usize;
    let shadow_size = if up != 0 { BOMB_SHADOW * UPGRADED } else { BOMB_SHADOW };
    match w.mobys[id].state {
        0 => {
            let Some(r) = row(ctx.game, BOUNCER).copied() else { return };
            let world_up = ctx.hero.gravity.up_at(w.mobys[id].pos);
            let old = w.mobys[id].pos;
            let to = add(old, w.mobys[id].pv_v(pv::VEL));
            w.mobys[id].pos = to;
            // `0x37ec68(14.6/3600, pos, vel)`: the fall along the up at the new place.
            let vel = sub(w.mobys[id].pv_v(pv::VEL), scale(ctx.hero.gravity.up_at(to), GRAVITY));
            w.mobys[id].set_pv_v(pv::VEL, vel);
            // The smoke while its ticks run: three puffs along the step, `(n / 50)·65000 + 10000` large.
            let mut n = w.mobys[id].pv_i(pv::SMOKE);
            countdown(&mut n);
            w.mobys[id].set_pv_i(pv::SMOKE, n);
            if n != 0 {
                if let Some(sys) = ctx.parts.as_deref_mut() {
                    let g = &mut ctx.game.rng;
                    for k in 0..3 {
                        let p = add(old, scale(sub(to, old), k as f32 * f32::from_bits(0x3eaa_a64c)));
                        let mut spin = g.randi(6);
                        if g.randi(2) != 0 { spin = -spin; }
                        let size = (n as f32 / 50.0) * 65_000.0 + 10_000.0;
                        if let Some(i) = crate::particles::puff(sys, g, f32::from_bits(0x3ccc_cccd), 1.0, 1.0, size, p, spin, [0.0; 3], 0x8080_8080, 1) {
                            let r = &mut sys.pool.recs[i];
                            crate::particles::rec::set_i16(r, 0xa, 0x32);
                            crate::particles::rec::set_u32(r, 0x24, 2);
                            r[0x2a] = 0x7f;
                            r[0x2b] = r[0xa];
                        }
                    }
                }
            }
            let floor = if w.mobys[id].pv_i(pv::BOUNCES) == 3 { 0.0 } else { NEVER_FLOOR };
            let s = sweep(w, ctx, id, old, to, vel, BOMB_RADIUS[up], BOMB_BOUNCE, floor, &[0, HAND_CLASS]);
            if 0 < s.res && matches!(surface_kind(s.surface), 1 | 2) {
                burst(w, id, ctx);
                return;
            }
            w.mobys[id].pos = s.at;
            w.mobys[id].set_pv_v(pv::VEL, s.vel);
            let q = w.mobys[id].pv_q(pv::SPIN);
            spin_rows(w, id, q);
            let pos = w.mobys[id].pos;
            let d = unit(s.vel);
            let mut t = HitTemplate::new(r.damage, Some(id), 0x83_0000, [d[0], d[1], d[2], NO_KNOCK_LIMIT]);
            t.kind = (BOMB as u16 as u32) << 16 | 0x0103;
            if w.sphere_hit(ctx.scene, pos, BOMB_RADIUS[up] + 0.1, 1, ctx.hero_moby, Some(&t)) != 0 {
                burst(w, id, ctx);
                return;
            }
            if 0 < s.res {
                let v = w.mobys[id].pv_v(pv::VEL);
                let axis = unit(cross_ba([v[0], v[1], 0.0], world_up));
                let spin = ctx.game.rng.randf(SPIN_MIN, SPIN_MAX);
                w.mobys[id].set_pv_q(pv::SPIN, axis_angle(spin, axis));
                ctx.host.play_sound(&w.mobys[id], 1, 0);
                let b = w.mobys[id].pv_i(pv::BOUNCES) + 1;
                w.mobys[id].set_pv_i(pv::BOUNCES, b);
                if 3 < b {
                    // The fourth: it sits at the sphere's centre and rolls along the ground.
                    let speed = if up != 0 { BOMB_SHADOW * SPIN_MIN * UPGRADED } else { BOMB_SHADOW * SPIN_MIN };
                    let ground_up = ctx.hero.gravity.up_at(s.centre);
                    let m = &mut w.mobys[id];
                    m.pos = s.centre;
                    m.state = 1;
                    m.set_pv_i(pv::ROLL, ROLL_TICKS);
                    let across = sub(v, scale(ground_up, dot(ground_up, v)));
                    m.set_pv_v(pv::VEL, set_len(across, speed));
                    m.set_pv_v(pv::SPIN, axis);
                    m.set_pv_f(pv::ANGLE, SPIN_MIN);
                }
            }
        }
        1 => {
            let m = &mut w.mobys[id];
            m.pos = add(m.pos, m.pv_v(pv::VEL));
            let q = axis_angle(m.pv_f(pv::ANGLE), m.pv_v(pv::SPIN));
            spin_rows(w, id, q);
            let m = &mut w.mobys[id];
            m.set_pv_f(pv::ANGLE, m.pv_f(pv::ANGLE) * ROLL_DAMP);
            m.set_pv_v(pv::VEL, scale(m.pv_v(pv::VEL), ROLL_DAMP));
            let mut t = m.pv_i(pv::ROLL);
            let done = countdown(&mut t);
            m.set_pv_i(pv::ROLL, t);
            if done {
                burst(w, id, ctx);
                return;
            }
        }
        _ => {}
    }
    shadow(w, ctx, shadow_size, id);
    w.rebuild(id, &*ctx.host);
}

/// `0x400a30` + `0x311688` (module docs): the bomb's burst, then it is gone.
fn burst(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let up = (w.mobys[id].pv_i(pv::UP) != 0) as usize;
    let (pos, owner) = (w.mobys[id].pos, w.mobys[id].parent);
    ctx.host.play_sound(&w.mobys[id], 0, 0);
    let mut s = f32::from_bits(0x3f33_3333);
    for _ in 0..2 {
        let b = crate::moby::blast::Blast { size: s, bursts: [0xe, 0xa, 0xf], puffs: 0xa, streaks: 0x14, light: false, shake: true };
        crate::moby::blast::burst(w, ctx, pos, &b);
        s += f32::from_bits(0x3e99_999a);
    }
    // The frame about the up (`0x37ecc0`): (z, z, −(x + y)) made unit, the up × it, the up.
    let n = ctx.hero.gravity.up_at(pos);
    let a = unit([n[2], n[2], -(n[0] + n[1])]);
    let b = cross_ba(a, n);
    for k in 0..BOMBLETS {
        let g = &mut ctx.game.rng;
        let yaw = g.rand_angle();
        let pitch = if up != 0 { g.randf(f32::from_bits(0x3f32_b8c4), f32::from_bits(0x3fa7_8d37)) } else { g.randf(f32::from_bits(0x3f5f_66f4), f32::from_bits(0x3fc9_0fdc)) };
        let speed = g.randf(4.0, if up != 0 { 12.0 } else { 9.0 });
        let v = super::shot::polar(speed * f32::from_bits(0x3c88_8889), yaw, pitch);
        let v = add(add(scale(a, v[0]), scale(b, v[1])), scale(n, v[2]));
        if let Some(c) = bomblet(w, ctx, pos, v, k, up) { w.mobys[c].parent = owner; }
    }
    w.delete(id);
}

/// `0x40aa58(at, vel, k, upgraded)`: the bomblet (module docs), None when its class is not loaded.
fn bomblet(w: &mut MobyWorld, ctx: &mut Ctx, at: V3, vel: V3, k: i32, up: usize) -> Option<usize> {
    if w.class(BOMBLET).is_none() {
        eprintln!("rc_game::bouncer: class {BOMBLET:#x} is not loaded: no bomblet");
        return None;
    }
    let id = w.create(BOMBLET);
    let g = &mut ctx.game.rng;
    let life = g.rand_range(BOMBLET_LIFE.0, BOMBLET_LIFE.1);
    let rot = [g.rand_angle(), g.rand_angle(), g.rand_angle()];
    let rows = crate::moby::attach::rows_of(rot);
    let spin = g.randf(SPIN_MIN, SPIN_MAX);
    let m = &mut w.mobys[id];
    m.update = Some(update_bomblet);
    m.mode = (m.mode & !mode::NO_UPDATE) | mode::KEEP_ROWS;
    m.pvars.resize(crate::moby::PVAR_BYTES, 0);
    m.update_dist = 0xff;
    m.draw_dist = 0xff;
    m.drawn = 1;
    m.state = 0;
    m.pos = at;
    (m.rot, m.rows) = (rot, rows);
    m.set_pv_v(pv::VEL, vel);
    m.set_pv_s(pv::LIFE, life as i16);
    m.set_pv_q(pv::SPIN, axis_angle(spin, rows[1]));
    m.set_pv_i(pv::LET_UP, up as i32);
    m.set_pv_s(pv::ARMING, BOMBLET_ARMING + k as i16);
    m.scale *= BOMBLET_SCALE[up];
    w.rebuild(id, &*ctx.host);
    w.changed = true;
    Some(id)
}

/// `0x40abb0` (module docs).
pub fn update_bomblet(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let up = (w.mobys[id].pv_i(pv::LET_UP) != 0) as usize;
    if w.mobys[id].state == 0 {
        let world_up = ctx.hero.gravity.up_at(w.mobys[id].pos);
        let old = w.mobys[id].pos;
        let to = add(old, w.mobys[id].pv_v(pv::VEL));
        w.mobys[id].pos = to;
        let vel = sub(w.mobys[id].pv_v(pv::VEL), scale(ctx.hero.gravity.up_at(to), GRAVITY));
        let s = sweep(w, ctx, id, old, to, vel, BOMBLET_RADIUS[up], BOMBLET_BOUNCE, NEVER_FLOOR, &[0, HAND_CLASS, BOMB]);
        let kind = surface_kind(s.surface);
        w.mobys[id].pos = s.at;
        w.mobys[id].set_pv_v(pv::VEL, s.vel);
        let q = w.mobys[id].pv_q(pv::SPIN);
        spin_rows(w, id, q);
        if 0 < s.res { hop(w, id, ctx, world_up); }
        // The hit's template: the bomblets' damage along its way.
        let v = w.mobys[id].pv_v(pv::VEL);
        let d = unit(v);
        let mut t = HitTemplate::new(BOMBLET_DAMAGE, Some(id), 0x83_0000, [d[0], d[1], d[2], NO_KNOCK_LIMIT]);
        t.kind = (BOMBLET as u16 as u32) << 16 | 0x0103;
        let mut life = w.mobys[id].pv_s(pv::LIFE) as i32;
        let out = countdown(&mut life);
        w.mobys[id].set_pv_s(pv::LIFE, life as i16);
        let mut blow = out || (0 < s.res && matches!(kind, 1 | 2));
        if !blow {
            let arming = w.mobys[id].pv_s(pv::ARMING);
            if arming == 0 {
                let pos = w.mobys[id].pos;
                let near = ctx.scene.map(|sc| crate::collision::coll_sphere_mobys(sc, pos, if up != 0 { 8.0 } else { 2.0 }, QueryFlags(1), ctx.hero_moby)).unwrap_or_default();
                for m in near {
                    let hb = aim::health(w, m);
                    let crate_ = aim::is_crate(w, m);
                    let alive = hb.is_some_and(|(hp, _)| 0.0 < hp) && w.mobys[m].pv_s(crate::moby::ai::health(&w.mobys[m]) + 6) == 0;
                    if crate_ || alive {
                        // `0x31d378(m, tmpl)`.
                        w.deliver_hit_any(m, &t, [0.0; 4]);
                        blow = true;
                        break;
                    }
                }
            } else {
                w.mobys[id].set_pv_s(pv::ARMING, (arming - 1).max(0));
            }
        }
        if blow {
            ctx.host.play_sound(&w.mobys[id], 1, 0);
            let pos = w.mobys[id].pos;
            let size = if up != 0 { f32::from_bits(0x4019_999a) } else { f32::from_bits(0x3f19_999a) };
            let b = crate::moby::blast::Blast { size, bursts: [6, 5, 6], puffs: 8, streaks: 10, light: false, shake: false };
            crate::moby::blast::burst(w, ctx, pos, &b);
            w.delete(id);
            return;
        }
    }
    shadow(w, ctx, BOMBLET_SHADOW[up], id);
    w.rebuild(id, &*ctx.host);
}

/// A bomblet's bounce (`0x40abb0`): its way across turned toward the nearest enemy within 7 (module docs), a new spin
/// about the up × its way across (`0x31ba98(0, 30°)` drawn for a turn of that axis about the up that only a draw under
/// 1e-5 makes, which does next to nothing), its class sound 0.
fn hop(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, up: V3) {
    let pos = w.mobys[id].pos;
    let near = ctx.scene.map(|sc| crate::collision::coll_sphere_mobys(sc, pos, 7.0, QueryFlags(1), ctx.hero_moby)).unwrap_or_default();
    let mut best: Option<usize> = None;
    let mut most = 10_000.0f32;
    for m in near {
        let x = &w.mobys[m];
        if x.drawn == 0 || aim::is_crate(w, m) { continue; }
        let Some((hp, _)) = aim::health(w, m) else { continue };
        if hp <= 0.0 { continue; }
        if w.class(x.o_class).is_none_or(|c| c.kind != 5) { continue; }
        let d = dist(x.pos, pos);
        if d < most {
            most = d;
            best = Some(m);
        }
    }
    if let Some(t) = best {
        // Its aim point: its place raised by its aim height (in the gravity frames along its up row).
        let a = aim::health(w, t).map_or(0.0, |(_, a)| a);
        let mut tp = w.mobys[t].pos;
        if ctx.hero.gravity.on { tp = add(tp, scale(w.mobys[t].rows[2], a)) } else { tp[2] += a }
        let v = w.mobys[id].pv_v(pv::VEL);
        let vert = scale(up, dot(up, v));
        let across = sub(v, vert);
        let mut s = len(across);
        if s != 0.0 {
            let d = sub(tp, pos);
            let dh = sub(d, scale(up, dot(d, up)));
            if s < f32::from_bits(0x3d88_8889) { s = f32::from_bits(0x3d88_8889); }
            w.mobys[id].set_pv_v(pv::VEL, add(set_len(dh, s), vert));
        }
    }
    let v = w.mobys[id].pv_v(pv::VEL);
    let mut axis = unit(cross_ba([v[0], v[1], 0.0], up));
    let a = ctx.game.rng.randf_signed(0.0, f32::from_bits(0x3f06_0a93));
    if a.abs() < 1e-5 { axis = qrot(axis_angle(a, up), axis); }
    let spin = ctx.game.rng.randf(SPIN_MIN, SPIN_MAX);
    w.mobys[id].set_pv_q(pv::SPIN, axis_angle(spin, axis));
    ctx.host.play_sound(&w.mobys[id], 0, 0);
}
