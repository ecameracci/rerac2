//! The Vaporizer, the Pulse Rifle's upgrade (item 0x17's rows 0x42, 0x57, 0x58, crate::hero::items::UPGRADE_ROWS; L00
//! `0x265ec0`: attachment 0, stance 2, use sequence 0x36, the firing arm's 0x46 crouched, damage 20 (40, 80), the blast's
//! 20 (`+0x64`; 0 on the Mega rows), 400 a second, one a second, 8 shots (12, 20)): the hand moby **0x0c04** (`0x3e4e98`,
//! called by the item pass while it is the hand item) and its shot **0x0cf7** (`0x3f6368`, made by `0x3f6108`; no
//! model: its trail, beam and smoke are its draw's).
//!
//! **The hand** (pvars: `+0x00` (s16) in first person last tick, `+0x02` (s16) the shot's delay, `+0x04` the cooldown,
//! `+0x08` the view's angle at the start, `+0x0c` the zoom's angle, `+0x10` the muzzle light (crate::point_lights, −1
//! none), `+0x14` its ticks, `+0x18` its trail slot, `+0x1c` the pad rumble's handle, `+0x28` (s16) the scope has a
//! target, `+0x2a` (s16) its blink, `+0x2c` the target; the port keeps the view turn's saved globals at `+0x40..0x48`).
//! The Pulse Rifle's machine (crate::moby::classes::pulse_rifle) with these differences:
//! * its sequence run out goes back to 1 (blend 2) first of all; put away or in the state 0x6f as the rifle's;
//! * state 0 takes the class's trail slot (crate::moby::classes::chopper::Pool, `0x32b1b8`) and, when it was not the
//!   class's, lays out its memory (`0x32b318`: three trails of 80 points, the three beams, the 120 puffs) and clears
//!   the beams and puffs ([`Mem::clear`]);
//! * the scope (crate::moby::classes::pulse_rifle::scope_at) at `+0x28`;
//! * zooming (R1 / R2, 0.01 a tick) has no sound; the shot needs no zoom sound stopped and asks for no help text;
//! * **the shot** ([`fire`], `0x3e5988`): its speed the row's / 60 a tick from the muzzle (joint 0) along Ratchet's
//!   forward row (the arm raised: the gun's −row 1), toward a lock's aim point (`0x32b4d0`), in first person toward what
//!   the view meets 128 out held within 20° (`0x380ef8`, mode 1); the muzzle light (`0x2fda20(10, 0, muzzle,
//!   0xff0000ff)`, 30 ticks); a moby of class type 5 or a crate first touched by the sphere 1 at the hand's
//!   −1.5 row 1 + 0.275 row 0 (`0x2e14b0`, flags 1, Ratchet left out) takes the impact ([`impact`], `0x3f6830`) there
//!   instead of a shot, else the shot ([`spawn`]), the hand its owner; four twinkles at the muzzle (`0x33a6a0`: 7, 5
//!   `0x401050ff`, 3.5, 2.5 white `0xc0ffffff` / `0xb0ffffff`, 8 ticks, turning either way);
//! * **the light**: radius 10, 1.5 ahead of the muzzle along the shot's way, `(t, 0.7·t, 0)` for `t` its ticks / 30.
//!
//! **The shot** (pvars: `+0x00` its velocity, `+0x10` its life (`128 / speed` ticks), `+0x14` one over it, `+0x18`
//! (s16) its beam, `+0x1a` (s16) the trail slot, `+0x20` (s16) its trail's points, `+0x22` (s16) the trail's time
//! (40), `+0x24` one over it, `+0x28` its mode (1: from first person), `+0x30` / `+0x40` the view's ray's step and
//! point). Made ([`spawn`], `0x3f6108`) when the slot is the Vaporizer's: far 0xff, drawn, its rows along its way, the
//! next of the three beams (`$gp+0x18f4`) started from it ([`make_beam`], `0x3f7430`; none in the look stance's twin).
//! Each tick ([`update_shot`], `0x3f6368`): gone once the slot is not the Vaporizer's; ended (state 2), its tail
//! ([`tail`], `0x3f6b00`). Else it steps (in first person its ray too); four trail points along the step
//! ([`add_point`], `0x3f6bf0`) and two puffs, at its place and halfway ([`puff`], `0x3f7da0`). Out of the world's
//! 2..1021 box it is held in 5..1016 and ends; a hit ([`hit`], `0x3f66d8`) or its life out ends it; else its tail.
//!
//! **The trail** (80 points of `0x30` bytes a beam: `+0x00` the point, `+0x0c` its half-width 0.1, `+0x10` its centre on
//! the shot's way, `+0x20` its angle (`k · 40°` for the k-th, wrapped), `+0x24` its radius 0.3, `+0x28` its turn 1° a
//! tick, `+0x2c` ±0.01..0.5 a second, read by nothing): each point the centre plus the radius along the up turned by
//! its angle about the way from the point before (the first: the up; `0x3f6d10`). **The tail** (`0x3f6b00`): the
//! trail's points turn and are placed again, the trail's scroll (`$gp+0x192c`, shared by every shot's tail: −0.075 a
//! tail, wrapped into −4..0, reset past ±7) moves and its time runs; the beam steps (crate::moby::classes::pulse_rifle::
//! step_beam_at: 40 ticks, scrolling 0.05 and 0.075 a tick); the puffs age, turning; while any of the three runs its
//! draw is asked for, else an ended shot goes.
//!
//! **The puffs** ([`Puff`], 120 in the slot: `+0x00` the place, `+0x10` (s16) the life 45..60, `+0x12` (s16) in use,
//! `+0x14` one over it, `+0x18` the roll, `+0x1c` its turn ±1..4° a tick, `+0x20` / `+0x24` the width at birth / death
//! (0.3..0.5 / 0.75..1.25), `+0x28` / `+0x2c` the height likewise, `+0x30` the wobble's phase, `+0x34` its speed 1..2° a
//! tick, `+0x38` its depth 0.1..0.25 of the width, `+0x3c` the height's (never written: 0)).
//!
//! **The hit** (`0x3f66d8(shot, from)`): the line from `from` to the shot (in first person its ray's point; flags 0,
//! Ratchet left out) hits a moby with the row's damage (flags 0x10000, its way the shot's, kind the class's `<< 16 |
//! 0x0101`); then the impact at the point, and the shot ends unless it met a moby without a collision mesh
//! (`0x31d100`: here every moby met has one).
//!
//! **The impact** ([`impact`], `0x3f6830`, the upgraded rows' branch): the blast (crate::moby::blast::burst,
//! `0x3f8348(1, 20, at, 1, 0x28, 0x14, 0x1e, 0x28, 0x32, 1)`) and the sphere 2 at it (flags 1, Ratchet left out)
//! hitting every moby with the row's `+0x64` (its way the shot's, flags 0x810000, kind the class's `<< 16 | 0x0203`).
//!
//! **The draw** ([`draw`], `0x3f6b90`, each live shot's): the trail (`0x3f6fb0`, while it has two points and its time
//! runs) — through its points two ribbons across the view (FX 0xe, additive), the point's half-width and three times
//! it, s from the scroll to it + 1 along each segment; from the first point to the last the narrow one's colour goes
//! from `0x00004080` to `0xff00ffff` (red to yellow) and the wide one's from `0x001010ff` to `0x80004040`, their
//! alphas × the time's share (half in the look stance's twin); the slot's beams (crate::moby::classes::pulse_rifle::
//! draw_beam, `0x3f77a8`: rings `0x104040` at 64, strips red `0xff` at 80 and 255, from none at the muzzle); each
//! puff (`0x3f7fb0`) a quad facing the camera (FX 0x36, normal), its width and height eased from birth to death, the
//! width wobbling, turned by its roll, `0x30307f` .. `0x307f7f` by `|sin(2π·share)|` at its life's share of 0x40 (half
//! in the twin).
//!
//! Its glow in the hand (`0x2a7ed8`'s case 0x17 upgraded): crate::hero::items.
//!
//! Not ported: the pad rumble (`0x2d6d60(0x296a60)` / `0x2d6f38`), the lock cheat (`0x18b270`), the LOD ranges the zoom
//! widens (`0x36a1c0`, `$gp−0x4ef0`), the moving frames (`$gp−0x14e4`: the shot's mode 2 and its aim straight at a lock
//! or the scope's target, `0x380e58` / `0x380e08`), the gravity frames' up (`0x326448`: world up), the weapon mods'
//! effects on a hit (`0x4126b8(moby, 0x42, owner)`), the impact's base-row branch (green twinkles: the shot is only made
//! by the upgraded hand). The base rifle shot's upgraded impact (`0x3a9668` → `0x320538`) has no caller with an upgraded
//! row (the upgrade swaps the hand to this one).

use crate::collision::QueryFlags;
use crate::first_person::Tune;
use crate::hero::items::{row, ItemRow, PULSE};
use crate::hero::physics::{add, add_rot, cross, len, scale, set_len, sub, unit, V3};
use crate::moby::hits::HitTemplate;
use crate::moby::{aim, countdown, Ctx, MobyWorld};
use crate::point_lights::PointLight;
use super::pulse_rifle::{self, Beam, BeamLook, BEAMS};

/// The hand's class (rows 0x42 / 0x57 / 0x58 `+0x04`) and the shot's (`0x3f6108`).
pub const HAND_CLASS: i16 = 0xc04;
pub const SHOT: i16 = 0xcf7;
/// `t[3]` of the hit templates (`0x45afdf66`).
const NO_KNOCK_LIMIT: f32 = 5627.925;
/// `0x3e32b8c4`: the zoom's narrowest (10°).
const NARROW: f32 = 0.174_532_95;
/// `0x3c2b92a8`: the view's most turn a tick at the narrowest (0.6°).
const NARROW_TURN: f32 = 0.010_471_977;
/// A slot's trail points a beam, puffs and the muzzle light's ticks.
pub const POINTS: usize = 80;
pub const PUFFS: usize = 120;
const LIGHT_TICKS: i32 = 0x1e;
/// `$gp+0x18f8`: the trail's time.
const TRAIL_TICKS: i16 = 0x28;
/// `$gp+0x1958`: a beam's life.
const BEAM_TICKS: i16 = 0x28;
/// `0x3f77a8`'s colours (`$gp+0x194c..0x197c`).
const BEAM_LOOK: BeamLook = BeamLook { ring: (0x10_4040, 64.0, 0xff), wide: (0xff, 80.0, 0x4040), thin: (0xff, 255.0, 0xffff) };

/// One trail point (module docs).
#[derive(Clone, Copy, Debug, Default)]
pub struct Point {
    pub pos: V3,
    pub half: f32,
    pub centre: V3,
    pub angle: f32,
    pub radius: f32,
    pub turn: f32,
    pub drift: f32,
}

/// One puff (module docs).
#[derive(Clone, Copy, Debug, Default)]
pub struct Puff {
    pub pos: V3,
    pub life: i16,
    pub live: bool,
    pub inv: f32,
    pub roll: f32,
    pub spin: f32,
    pub w: [f32; 2],
    pub h: [f32; 2],
    pub phase: f32,
    pub phase_step: f32,
    pub wobble: [f32; 2],
}

/// One trail slot's memory as the Vaporizer lays it out (`0x3e4e98`'s state 0).
#[derive(Clone, Debug, Default)]
pub struct Slot {
    pub trails: [Vec<Point>; BEAMS],
    pub beams: [Beam; BEAMS],
    pub puffs: Vec<Puff>,
}

/// The Vaporizer's part of the two trail slots (`0x268da0 + 0xa430·k`), its next beam (`$gp+0x18f4`) and the trails'
/// scroll (`$gp+0x192c`).
#[derive(Clone, Debug, Default)]
pub struct Mem {
    pub slots: [Slot; 2],
    pub next_beam: i32,
    pub scroll: f32,
}

impl Mem {
    fn slot(&mut self, slot: usize) -> &mut Slot {
        let s = &mut self.slots[slot.min(1)];
        if s.puffs.len() != PUFFS { s.puffs = vec![Puff::default(); PUFFS]; }
        for t in s.trails.iter_mut() {
            if t.len() != POINTS { *t = vec![Point::default(); POINTS]; }
        }
        s
    }

    /// The layout's clears (`0x2eebf0(beams, 0x120)`, `(puffs, 0x1e00)`).
    pub fn clear(&mut self, slot: usize) {
        let s = self.slot(slot);
        s.beams = Default::default();
        s.puffs = vec![Puff::default(); PUFFS];
    }
}

/// The Pulse Rifle's row as `0x139568[0x17]` gives it (crate::upgrades).
fn cur_row(game: &crate::game::GameState) -> Option<&'static ItemRow> { row(game, PULSE) }

/// The record's `+0x64` by row (the impact's sphere).
fn blast_damage(r: &ItemRow) -> f32 { if r.item == 0x42 { 20.0 } else { 0.0 } }

/// `0x320440(a)`: `a` wrapped into −π..π (`0x2eff90`: modf, the fraction keeping the sign).
fn wrap(a: f32) -> f32 { ((a + std::f32::consts::PI) * 0.159_154_94).fract() * 6.283_185_5 - std::f32::consts::PI }

/// `0x2eef88(out, a, b)` = b × a.
fn cross_ba(a: V3, b: V3) -> V3 { cross(b, a) }

/// `0x2eeae0` on the s16 at `off`.
fn count_s(w: &mut MobyWorld, id: usize, off: usize) -> bool {
    let mut n = w.mobys[id].pv_s(off) as i32;
    let done = countdown(&mut n);
    w.mobys[id].set_pv_s(off, n as i16);
    done
}

/// The muzzle light let go (`0x2fdc48`).
fn free_light(w: &mut MobyWorld, id: usize) {
    let s = w.mobys[id].pv_i(0x10);
    if 0 <= s { w.point_lights.free(s as usize); }
    w.mobys[id].set_pv_i(0x10, -1);
}

/// `0x3e4e98` (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].pvars.resize(crate::moby::PVAR_BYTES, 0);
    if let Some(a) = ctx.host.anim(&w.mobys[id]) {
        if a.seq_a == a.seq_b && a.flags & 2 != 0 { ctx.host.set_anim(&w.mobys[id], 1, 0, 2); }
    }
    if ctx.hero.items.ready == 3 {
        if w.mobys[id].state == 0 { return; }
        pulse_rifle::put_back(w, id, ctx, true);
        free_light(w, id);
        return;
    }
    if ctx.hero.state == 0x6f {
        let was = w.mobys[id].state;
        if was == 4 { return; }
        w.mobys[id].state = 4;
        free_light(w, id);
        pulse_rifle::put_back(w, id, ctx, was != 0);
        return;
    }
    if w.mobys[id].state == 4 { w.mobys[id].state = 1; }
    let fp = ctx.hero.items.fp.on;
    if fp { pulse_rifle::scope_at(w, id, ctx, 0x28); }
    match w.mobys[id].state {
        0 => start(w, id, ctx),
        1 => ready(w, id, ctx, fp),
        2 => {
            if count_s(w, id, 2) || w.mobys[id].pv_s(0) != 0 {
                if let Some(r) = cur_row(ctx.game).copied() {
                    ctx.host.play_sound(&w.mobys[id], 0, 0);
                    fire(w, id, ctx, fp, &r);
                    w.mobys[id].set_pv_f(4, 1.0 / r.rate);
                }
                w.mobys[id].state = 3;
            }
        }
        3 => {
            let c = w.mobys[id].pv_f(4);
            if 1.0 / 60.0 < c {
                w.mobys[id].set_pv_f(4, c - 1.0 / 60.0);
            } else {
                w.mobys[id].set_pv_f(4, 0.0);
                w.mobys[id].state = 1;
            }
        }
        _ => {}
    }
    light(w, id, ctx);
}

/// State 0 (module docs).
fn start(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let begin = ctx.game.cam_tweaks.view_tan().atan() * 2.0;
    let tune = ctx.game.cameras.fp.tune;
    {
        let m = &mut w.mobys[id];
        m.set_pv_s(0, 0);
        m.set_pv_f(4, f32::from_bits(0x3ebb_bbbc));
        m.state = 3;
        m.set_pv_f(8, begin);
        m.set_pv_f(0xc, begin + (NARROW - begin) * 0.25);
        m.set_pv_f(0x40, tune.k);
        m.set_pv_f(0x44, tune.d);
        m.set_pv_f(0x48, tune.turn);
        m.set_pv_i(0x14, 0);
        m.set_pv_s(0x28, 0);
        m.set_pv_s(0x2a, 0);
        m.set_pv_i(0x2c, 0);
        m.set_pv_i(0x10, -1);
        m.set_pv_i(0x1c, -1);
    }
    let class = w.mobys[id].o_class;
    let mut pool = std::mem::take(&mut w.chopper);
    let (slot, found) = pool.slot(w, class);
    w.chopper = pool;
    w.mobys[id].set_pv_i(0x18, slot as i32);
    if !found { w.vapor.clear(slot); }
}

/// State 1 (module docs).
fn ready(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, fp: bool) {
    if ctx.hero.state == 1 { ctx.hero.request_state(0x1d, true); }
    if fp { w.mobys[id].set_pv_s(0, 1); }
    let begin = w.mobys[id].pv_f(8);
    if !fp {
        if w.mobys[id].pv_s(0) != 0 {
            w.mobys[id].set_pv_f(0xc, begin + (NARROW - begin) * 0.25);
            ctx.game.cam_tweaks.set_fov(begin, 0.005, 0.2, 0.0, 3);
            w.mobys[id].set_pv_s(0, 0);
        }
    } else {
        let held = ctx.hero.items.held;
        let mut a = w.mobys[id].pv_f(0xc);
        if held & crate::pad::button::R1 != 0 {
            a -= 0.01;
        } else if held & crate::pad::button::R2 != 0 {
            a += 0.01;
        }
        if begin < a {
            a = begin;
        } else if a < NARROW {
            a = NARROW;
        }
        w.mobys[id].set_pv_f(0xc, a);
        ctx.game.cam_tweaks.set_fov(a, 0.02, 0.2, 0.0, 3);
    }
    {
        let m = &w.mobys[id];
        let f = (m.pv_f(0xc) - NARROW) / (begin - NARROW);
        let t = &mut ctx.game.cameras.fp.tune;
        *t = Tune { k: (m.pv_f(0x40) - 0.03) * f + 0.03, d: (m.pv_f(0x44) - 0.4) * f + 0.4, turn: (m.pv_f(0x48) - NARROW_TURN) * f + NARROW_TURN };
    }
    let ok = if fp { countdown(&mut ctx.hero.items.look_wait) } else {
        ctx.hero.items.look_wait = 0x28;
        true
    };
    let it = ctx.hero.items;
    if it.no_use == 0 && it.pressed & it.trigger != 0 && ok {
        if ctx.hero.items.take_ammo(ctx.game, 1) {
            ctx.hero.items.use_start(ctx.hero.group);
            let m = &mut w.mobys[id];
            m.set_pv_s(2, 2);
            m.state = 2;
            ctx.host.set_anim(&w.mobys[id], 2, 0, 0);
        } else {
            ctx.host.play_sound(&w.mobys[id], 1, 0);
        }
    }
}

/// The light's tail of `0x3e4e98` (module docs).
fn light(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let mut n = w.mobys[id].pv_i(0x14);
    if n == 0 { return; }
    let out = countdown(&mut n);
    w.mobys[id].set_pv_i(0x14, n);
    if out {
        free_light(w, id);
        return;
    }
    if w.mobys[id].pv_i(0x10) == -1 {
        let l = PointLight { color: [1.0, 0.0, 0.0], intensity: 0.0, pos: w.mobys[id].pos, radius: 10.0 };
        let s = w.point_lights.alloc(l).map_or(-1, |s| s as i32);
        w.mobys[id].set_pv_i(0x10, s);
    }
    let s = w.mobys[id].pv_i(0x10);
    if s < 0 { return; }
    let muzzle = ctx.host.joint(&w.mobys[id], 0).unwrap_or(w.mobys[id].pos);
    let off = if ctx.hero.items.raised { set_len(w.mobys[id].rows[1], -1.5) } else { set_len(ctx.hero.rows[0], 1.5) };
    let t = n as f32 / LIGHT_TICKS as f32;
    w.point_lights.set(s as usize, PointLight { color: [t, t * 0.7, 0.0], intensity: 0.0, pos: add(muzzle, off), radius: 10.0 });
}

/// `0x3e5988` (module docs).
fn fire(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, fp: bool, r: &ItemRow) {
    let f = r.speed * 0.016_666_668;
    let muzzle = ctx.host.joint(&w.mobys[id], 0).unwrap_or(w.mobys[id].pos);
    let mut dir = if ctx.hero.items.raised { set_len(w.mobys[id].rows[1], -f) } else { set_len(ctx.hero.rows[0], f) };
    free_light(w, id);
    let l = PointLight { color: [1.0, 0.0, 0.0], intensity: 0.0, pos: muzzle, radius: 10.0 };
    let s = w.point_lights.alloc(l).map_or(-1, |s| s as i32);
    w.mobys[id].set_pv_i(0x10, s);
    w.mobys[id].set_pv_i(0x14, LIGHT_TICKS);
    let mut mode = 0;
    if !fp {
        if let Some((t, _)) = ctx.hero.items.aim_target.filter(|&(t, _)| w.mobys.get(t).is_some()) { dir = set_len(sub(aim::aim_point(w, t), muzzle), f); }
    } else {
        mode = 1;
        dir = pulse_rifle::view_aim(ctx, muzzle, f, 128.0, 0.349_065_9);
    }
    // The point-blank sphere (`0x2e14b0(1, at, 1, Ratchet)`) and its first moby (`0x1bf198`).
    let m = &w.mobys[id];
    let at = add(add(m.pos, scale(m.rows[1], -1.5)), scale(m.rows[0], 0.275));
    let first = ctx.scene.and_then(|sc| crate::collision::mobys::coll_sphere_mobys(sc, at, 1.0, QueryFlags(1), ctx.hero_moby).first().copied());
    if ctx.scene.is_some() { w.hits.last_target = first; }
    let near = first.is_some_and(|n| w.class(w.mobys[n].o_class).is_some_and(|c| c.kind == 5) || aim::is_crate(w, n));
    if near {
        impact(w, id, ctx, at, r);
    } else {
        let slot = w.mobys[id].pv_i(0x18);
        if let Some(k) = spawn(w, ctx, muzzle, dir, slot, mode, r) { w.mobys[k].parent = Some(id); }
    }
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let g = &mut ctx.game.rng;
    for (size, rgba, spin) in [(7.0, 0x4010_50ffu32, 1i8), (5.0, 0x4010_50ff, -1), (3.5, 0xc0ff_ffff, 1), (2.5, 0xb0ff_ffff, -1)] {
        crate::particles::twinkle(sys, g, size, 0.1, 0.0, muzzle, 8, rgba, 2, spin, [0.0; 3], 1);
    }
}

/// `0x3f6108(pos, vel, slot, mode)` (module docs), when the slot is still the Vaporizer's.
pub fn spawn(w: &mut MobyWorld, ctx: &mut Ctx, pos: V3, vel: V3, slot: i32, mode: i32, r: &ItemRow) -> Option<usize> {
    if slot < 0 || !w.chopper.holds(HAND_CLASS, slot as usize) || w.class(SHOT).is_none() { return None; }
    let id = w.create(SHOT);
    let life = (128.0 / (r.speed * 0.016_666_668)) as i32;
    let up = ctx.hero.gravity.up_at(pos);
    let r0 = unit(vel);
    let r1 = unit(cross_ba(r0, scale(up, -1.0)));
    let r2 = cross_ba(r1, r0);
    let mut b = w.vapor.next_beam;
    if 3 < b { b = 0; }
    w.vapor.next_beam = (b + 1) % 3;
    {
        let m = &mut w.mobys[id];
        m.pvars.resize(crate::moby::PVAR_BYTES, 0);
        m.update_dist = 0xff;
        m.draw_dist = 0xff;
        m.drawn = 1;
        m.pos = pos;
        m.state = 0;
        m.set_pv_v(0, vel);
        m.set_pv_i(0x10, life);
        m.set_pv_f(0x14, 1.0 / life as f32);
        m.set_pv_s(0x1a, slot as i16);
        m.set_pv_s(0x20, 0);
        m.set_pv_s(0x22, TRAIL_TICKS);
        m.set_pv_f(0x24, 1.0 / TRAIL_TICKS as f32);
        m.set_pv_i(0x28, mode);
        m.set_pv_s(0x18, b as i16);
        m.rows = [r0, r1, r2];
        m.mode |= crate::moby::mode::KEEP_ROWS;
    }
    if ctx.hero.state != 0x1d { make_beam(w, id, ctx); }
    if mode == 1 {
        let (cam, rows) = (ctx.host.camera_pos(), ctx.host.camera_rows());
        let m = &mut w.mobys[id];
        m.set_pv_v(0x40, cam);
        m.set_pv_v(0x30, set_len(rows[0], len(vel)));
    }
    Some(id)
}

/// `0x3f7430`: the shot's beam started at it — its way, the up across it, from its place to one step on (40 ticks).
fn make_beam(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let m = &w.mobys[id];
    let (pos, vel, slot, b) = (m.pos, m.pv_v(0), m.pv_s(0x1a) as usize, m.pv_s(0x18) as usize);
    let axis = unit(vel);
    let s = unit(cross_ba(axis, ctx.hero.gravity.up_at(pos)));
    let bm = &mut w.vapor.slot(slot).beams[b.min(BEAMS - 1)];
    bm.axis = axis;
    bm.side = cross_ba(s, axis);
    bm.p[0] = pos;
    bm.p[1] = add(pos, vel);
    bm.p[2] = bm.p[1];
    bm.life = BEAM_TICKS;
    bm.inv = 1.0 / BEAM_TICKS as f32;
    bm.scroll = [0.0; 2];
    bm.live = true;
}

/// `0x3f6368` (module docs).
pub fn update_shot(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].pvars.resize(crate::moby::PVAR_BYTES, 0);
    let slot = w.mobys[id].pv_s(0x1a);
    if slot < 0 || !w.chopper.holds(HAND_CLASS, slot as usize) {
        w.delete(id);
        return;
    }
    if w.mobys[id].state == 2 {
        tail(w, id, ctx);
        return;
    }
    let first = w.mobys[id].pv_i(0x28) == 1;
    let start = {
        let m = &mut w.mobys[id];
        let start = if first { m.pv_v(0x40) } else { m.pos };
        m.pos = add(m.pos, m.pv_v(0));
        if first {
            let p = add(m.pv_v(0x40), m.pv_v(0x30));
            m.set_pv_v(0x40, p);
        }
        if m.state == 0 { m.state = 1; }
        start
    };
    let pos = w.mobys[id].pos;
    for k in 0..4 {
        let t = k as f32 * 0.25;
        add_point(w, id, ctx, add(start, scale(sub(pos, start), t)));
    }
    puff(w, id, ctx, pos);
    puff(w, id, ctx, add(start, scale(sub(pos, start), 0.5)));
    let in_box = pos.iter().all(|&c| (2.0..=1021.0).contains(&c));
    if in_box {
        if !hit(w, id, ctx, start) && !count_i(w, id, 0x10) {
            tail(w, id, ctx);
            return;
        }
    } else {
        let m = &mut w.mobys[id];
        m.pos = m.pos.map(|c| c.clamp(5.0, 1016.0));
    }
    w.mobys[id].state = 2;
}

/// `0x2eeab0` on the word at `off`.
fn count_i(w: &mut MobyWorld, id: usize, off: usize) -> bool {
    let mut n = w.mobys[id].pv_i(off);
    let done = countdown(&mut n);
    w.mobys[id].set_pv_i(off, n);
    done
}

/// `0x3f6bf0(shot, at)`: the trail's next point (module docs), placed.
fn add_point(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, at: V3) {
    let m = &w.mobys[id];
    let (slot, b, k) = (m.pv_s(0x1a) as usize, (m.pv_s(0x18) as usize).min(BEAMS - 1), m.pv_s(0x20) as usize);
    if POINTS <= k { return; }
    let g = &mut ctx.game.rng;
    let turn = g.randf(1.0, 1.0) * 0.017_453_294;
    let drift = g.randf_signed(0.01, 0.5) * 0.016_666_668;
    let up = ctx.hero.gravity.up_at(at);
    let t = &mut w.vapor.slot(slot).trails[b];
    t[k].centre = at;
    t[k].angle = wrap(k as f32 * 0.017_453_294 * 40.0);
    t[k].turn = turn;
    t[k].radius = 0.3;
    t[k].drift = drift;
    place(t, k, up);
    w.mobys[id].set_pv_s(0x20, k as i16 + 1);
}

/// `0x3f6d10(trail, k)`: point k the centre plus its radius along the up, turned by its angle about the way from the
/// point before (the first, or under 1e-5: the up itself); its half-width `$gp+0x18fc`.
fn place(t: &mut [Point], k: usize, up: V3) {
    let p = t[k];
    let v = if k != 0 && 1e-5 <= p.angle.abs() {
        let d = unit(sub(p.centre, t[k - 1].centre));
        crate::gravity::rotate(up, d, p.angle)
    } else {
        up
    };
    t[k].pos = add(p.centre, scale(v, p.radius));
    t[k].half = 0.1;
}

/// `0x3f6b00` (module docs).
fn tail(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let a = step_trail(w, id, ctx);
    let m = &w.mobys[id];
    let (pos, vel, slot, b) = (m.pos, m.pv_v(0), m.pv_s(0x1a) as usize, (m.pv_s(0x18) as usize).min(BEAMS - 1));
    let beam = pulse_rifle::step_beam_at(&mut w.vapor.slot(slot).beams[b], pos, vel, [0.05, 0.075]);
    let n = step_puffs(w, slot);
    if !a && !beam && n == 0 {
        if w.mobys[id].state == 2 { w.delete(id); }
    } else {
        w.register_draw(id);
    }
}

/// `0x3f6e90`: the trail's points turn and are placed again; the scroll moves; its time runs (true while it does).
fn step_trail(w: &mut MobyWorld, id: usize, ctx: &Ctx) -> bool {
    let m = &w.mobys[id];
    let (slot, b, n) = (m.pv_s(0x1a) as usize, (m.pv_s(0x18) as usize).min(BEAMS - 1), (m.pv_s(0x20).max(0) as usize).min(POINTS));
    let t = &mut w.vapor.slot(slot).trails[b];
    for k in 0..n {
        t[k].angle = add_rot(t[k].angle, t[k].turn);
        let up = ctx.hero.gravity.up_at(t[k].centre);
        place(t, k, up);
    }
    let s = &mut w.vapor.scroll;
    if 7.0 < *s || *s < -7.0 { *s = 0.0; }
    *s -= 0.075;
    if *s <= -4.0 { *s += 4.0; }
    !count_s(w, id, 0x22)
}

/// `0x3f7da0(shot, at)`: a puff in the slot's first free record (module docs); none when all 120 are in use.
fn puff(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, at: V3) {
    let slot = w.mobys[id].pv_s(0x1a) as usize;
    let s = w.vapor.slot(slot);
    let Some(p) = s.puffs.iter_mut().find(|p| !p.live) else { return };
    let g = &mut ctx.game.rng;
    p.live = true;
    p.pos = at;
    p.life = g.rand_range(0x2d, 0x3c) as i16;
    p.inv = 1.0 / p.life as f32;
    p.roll = g.rand_angle();
    p.spin = g.randf_signed(1.0, 4.0) * 0.017_453_294;
    p.w[0] = g.randf(0.3, 0.5);
    p.w[1] = g.randf(0.75, 1.25);
    p.h[0] = g.randf(0.3, 0.5);
    p.h[1] = g.randf(0.75, 1.25);
    p.phase = 0.0;
    p.phase_step = g.randf(1.0, 2.0) * 0.017_453_294;
    p.wobble[0] = g.randf(0.1, 0.25);
}

/// `0x3f7ef8`: every live puff ages; the ones still alive turn. Their count.
fn step_puffs(w: &mut MobyWorld, slot: usize) -> usize {
    let mut n = 0;
    for p in w.vapor.slot(slot).puffs.iter_mut().filter(|p| p.live) {
        let mut life = p.life as i32;
        let out = countdown(&mut life);
        p.life = life as i16;
        if out {
            p.live = false;
            continue;
        }
        n += 1;
        p.roll = add_rot(p.roll, p.spin);
    }
    n
}

/// `0x3f66d8(shot, from)` (module docs): true when the shot ends.
fn hit(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, from: V3) -> bool {
    let Some(r) = cur_row(ctx.game).copied() else { return false };
    let m = &w.mobys[id];
    let to = if m.pv_i(0x28) == 1 { m.pv_v(0x40) } else { m.pos };
    let d = unit(m.pv_v(0));
    let mut t = HitTemplate::new(r.damage, Some(id), 0x10000, [d[0], d[1], d[2], NO_KNOCK_LIMIT]);
    t.kind = (m.o_class as u16 as u32) << 16 | 0x0101;
    let Some(o) = ctx.coll.and_then(|c| crate::collision::coll_line_m(c, ctx.scene, from, to, QueryFlags(0), ctx.hero_moby)) else { return false };
    if let Some(target) = o.moby { w.deliver_hit_any(target, &t, [o.point[0], o.point[1], o.point[2], 1.0]); }
    impact(w, id, ctx, o.point, &r);
    match o.moby {
        None => true,
        Some(m) => w.mobys.get(m).is_some_and(|x| x.has_collision),
    }
}

/// `0x3f6830(source, at)` (module docs): the template's way the source's first pvar word as a vector (the shot's
/// velocity; for the point-blank hit, called with the hand, its first four pvar words as they are).
fn impact(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, at: V3, r: &ItemRow) {
    crate::moby::blast::burst(w, ctx, at, &crate::moby::blast::Blast { size: 1.0, bursts: [0x28, 0x14, 0x1e], puffs: 0x28, streaks: 0x32, light: true, shake: true });
    let d = unit(w.mobys[id].pv_v(0));
    let mut t = HitTemplate::new(blast_damage(r), Some(id), 0x81_0000, [d[0], d[1], d[2], NO_KNOCK_LIMIT]);
    t.kind = (w.mobys[id].o_class as u16 as u32) << 16 | 0x0203;
    w.sphere_hit(ctx.scene, at, 2.0, 1, ctx.hero_moby, Some(&t));
}

/// `0x3f6b90` (module docs): gone once the slot is not the Vaporizer's; else the shot's trail and the slot's beams and
/// puffs. `hero_state`: Ratchet's (`0x18c0b4`).
pub fn draw(w: &mut MobyWorld, id: usize, hero_state: i32, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let m = &w.mobys[id];
    let slot = m.pv_s(0x1a);
    if slot < 0 || !w.chopper.holds(HAND_CLASS, slot as usize) {
        w.delete(id);
        return;
    }
    let (b, n, time) = ((m.pv_s(0x18) as usize).min(BEAMS - 1), (m.pv_s(0x20).max(0) as usize).min(POINTS), m.pv_s(0x22) as f32 * m.pv_f(0x24));
    let scroll = w.vapor.scroll;
    let s = w.vapor.slot(slot as usize);
    if 1 < n && time != 0.0 { draw_trail(&s.trails[b][..n], time, scroll, hero_state, cam, out); }
    for bm in s.beams.iter().filter(|b| b.live) { pulse_rifle::draw_beam(bm, cam, &BEAM_LOOK, out); }
    draw_puffs(s, hero_state, cam, out);
}

/// `0x3f6fb0` (module docs).
fn draw_trail(t: &[Point], time: f32, scroll: f32, hero_state: i32, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    use crate::fx::FxQuad;
    let f = if hero_state == 0x1d { time * 0.5 } else { time };
    let word = |a: u32, c: u32| ((a as f32 * f) as i32 as u32) << 24 | c;
    let (n0, n1, w0, w1) = (word(0x00, 0x00_4080), word(0xff, 0x00_ffff), word(0x00, 0x10_10ff), word(0x80, 0x00_4040));
    let st = [[scroll, 0.0], [scroll, 1.0], [scroll + 1.0, 0.0], [scroll + 1.0, 1.0]];
    let edge = |k: usize| {
        let p = t[k].pos;
        let d = if k == 0 { sub(t[1].pos, p) } else { sub(p, t[k - 1].pos) };
        let s = set_len(cross_ba(d, sub(p, cam)), t[k].half);
        let wide = scale(s, 3.0);
        ([add(p, s), sub(p, s)], [add(p, wide), sub(p, wide)])
    };
    let (mut a, mut b) = edge(0);
    let (mut ca, mut cb) = (n0, w0);
    let step = 1.0 / t.len() as f32;
    for k in 1..t.len() {
        let (a1, b1) = edge(k);
        let x = k as f32 * step;
        let (na, nb) = (crate::hud::lerp_rgba(x, n0, n1), crate::hud::lerp_rgba(x, w0, w1));
        out.push(FxQuad { fx: 0xe, additive: true, corners: [a[0], a[1], a1[0], a1[1]], st, rgba: [ca, ca, na, na] });
        out.push(FxQuad { fx: 0xe, additive: true, corners: [b[0], b[1], b1[0], b1[1]], st, rgba: [cb, cb, nb, nb] });
        (a, b, ca, cb) = (a1, b1, na, nb);
    }
}

/// `0x1aa310`: the sprites' template corners (y, z).
const SQUARE: [[f32; 2]; 4] = [[-1.0, 1.0], [-1.0, -1.0], [1.0, 1.0], [1.0, -1.0]];

/// `0x3f7fb0` (module docs): the wobble's phase steps here, in the draw.
fn draw_puffs(s: &mut Slot, hero_state: i32, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    use crate::fx::FxQuad;
    let most = if hero_state == 0x1d { 32.0 } else { 64.0 };
    for p in s.puffs.iter_mut().filter(|p| p.live) {
        let f = p.life as f32 * p.inv;
        let a = (most * f) as i32 as u32;
        let c = a << 24 | crate::hud::lerp_rgba((f * std::f32::consts::TAU).sin().abs(), 0x30_307f, 0x30_7f7f);
        let w = p.w[1] + (p.w[0] - p.w[1]) * f;
        let h = p.h[1] + (p.h[0] - p.h[1]) * f;
        p.phase = add_rot(p.phase, p.phase_step);
        let (ws, wc) = p.phase.sin_cos();
        let (y_size, z_size) = (w + p.wobble[0] * w * wc, h + p.wobble[1] * h * ws);
        let to_cam = unit(sub(cam, p.pos));
        let side = unit(cross_ba(to_cam, [0.0, 0.0, 1.0]));
        let up2 = cross_ba(side, to_cam);
        let (sn, cs) = p.roll.sin_cos();
        let corners = SQUARE.map(|[y, z]| {
            let (y, z) = (y * y_size, z * z_size);
            add(p.pos, add(scale(side, y * cs - z * sn), scale(up2, y * sn + z * cs)))
        });
        out.push(FxQuad { fx: 0x36, additive: false, corners, st: [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]], rgba: [c; 4] });
    }
}
