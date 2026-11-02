//! The Sheepinator (item 0x10, crate::hero::items::ROWS; L00's record `0x263300`: attachment 0, stance 1, use sequence
//! 0x38 (the firing arm's 0x3e crouched, none moving), the use keeps the stance, icon 0xea77, name 0x3312 "Sheepinator",
//! description 0x2f29, no ammo, upgrade row 0x48 "Black Sheepinator" at 900 points; not in the vendor's stock): the hand
//! moby **0x0a5d** (`0x3c76e0`, called by the item pass while it is the hand item). Held ○ throws a twisting red beam
//! from the gun; an enemy it holds drains its health's worth of time (a meter at the screen's right edge,
//! crate::hud::SideMeter) and turns into a sheep (crate::moby::classes::sheep, [`transform`]).
//!
//! **The hand** (pvars: `+0x00` the muzzle (joint 0, `0x3127c8`), `+0x10` the target, `+0x14` its most health, `+0x18`
//! the drain left, `+0x1c` (s16) a 15-tick use timer, `+0x1e` (s16) the use under way, `+0x44` the lock's ticks, `+0x48`
//! / `+0x4c` the aim's yaw and pitch, `+0x50` the meter's blink phase; the globals `0x1b2984` the point light,
//! `0x1b2988` its ticks, `0x1b298c` the loop sound, [`Mem`]). Its first tick (`+0x20` 0 → 1) clears the globals. Each
//! tick the use timer runs down; in the look stance 0x1d the look wait (`0x18c354`) counts down (else it is 0x28).
//!
//! **Firing** — standing or moving (`0x18c0bc` < 2), ○ held (`0x138320 & 0x18b050`) and the look wait out:
//! * its sequence 2 (blend 0) unless it plays it; the use stance (`0x2af5e0`) unless in use; the use timer 15 when out;
//!   the muzzle read; the look stance 1 goes to its twin 0x1d;
//! * the aim: in 0x1d with Ratchet hidden (first person) the view's forward, the arm raised at a wall (`0x18c30c`) the
//!   gun's −row 1, else Ratchet's forward row (`0x18a550`); its yaw and pitch (`0x2ef510`);
//! * the target ([`target`], `0x3c6fd0`); the lock's ticks down; a target deleted (state −2 / −3) is dropped;
//! * none found and no lock: the meter full (1); else a new target (not the one held, no lock) is taken — its most
//!   health (`+0x04`), the drain its health (`+0x00`; 0 when the most is 1 or less), the lock 30 ticks — and the drain
//!   runs down 10 a second (35 on a row of a Mega upgrade at level 2, the challenge mode's row 0x6d), not below 0;
//!   the meter is what is left of the most;
//! * the meter (`0x2fc520(f, 1, colour, 0)`): its colour blinks between `0x60ffffff` and `0x40ffc0a0` by the sine of
//!   its phase, which turns 360°..1440° a second as `f` runs from 1 to 0 (`0x327240`, `0x31e8d8`);
//! * the drain out on a target: on level 0xc a class-0xfb4 target counts (`$gp+0x1de0`) and the 16th gives skill
//!   point 0x16 (`0x1a7a7e`, crate::skill_points); its class sound 3; the transform ([`transform`], `0x404c18`); the
//!   sheep count (`$gp+0xf6c`) up; the target dropped;
//! * the beam ([`beam`], `0x3c7fe0`); the point light (`0x2fda20(7.5, 0, its place, 0x207f7f)` when it has none: radius
//!   7.5, 0.99 red and green, 0.25 blue) 0.5 along the gun's −row 1 from the muzzle, its ticks 20; the loop sound
//!   (class sound 1, mode 4) kept playing.
//!
//! **Not firing**: the target dropped, the use and the lock cleared, the beam to be laid out again (`0x1b2980`); the light
//! fades (its colour `0.05·t` red and green, a quarter of it blue, at the muzzle's last place) and goes as its 20 ticks
//! run out; the loop sound stopped; its sequence run out, back to 1 (blend 1); the use over (`0x2af848`). Put away
//! (`0x18b064` = 3) the light goes.
//!
//! **The beam** ([`beam`], `0x3c7fe0`; laid out again on its first tick, `0x3c7ed0`: twelve points from the muzzle along
//! the gun's −row 0, 8/12 apart, the scrolls and pulses cleared): its wave turns 11.5° a tick. Its reach: to the target,
//! its distance across Ratchet's up; else 8 along the aim to what the line (flags 0x14, Ratchet left out) meets, the
//! sheep let through (their collision off for the walk, `+0x98`; at most 6 steps and 128 sheep); its point spacing the
//! reach / 12. Its eleven points after the muzzle each follow the last: the way they lay blended toward the aim (the
//! gun's −row 1, from the seventh on bent 0.2 toward the target) by `0.9 − 0.5·i/12`, a spacing long (`0x380e08`, the
//! gravity frames' bend, not ported); round each a ring of 20 points 0.05..1.234 out (across the way and Ratchet's down
//! `0x18a0e0`), and two side points `±` a wave 0.01..0.5 out, turned 22.9° a point. Then sparks at the side ends
//! (twinkles at random, `0x33a6a0`: red `0x7f10107f` and `0x7f104fff` with white), at random a swirl at the muzzle
//! (type 0x4e, crate::particles::swirl: along the beam turned up to 15° each way, its life the reach · 6 (× 3 on a
//! target), one of five reds `0x7f307fff` .. `0x7f2020ff` and a white one following the target), and the pulses: every
//! 45 ticks a new one (three at most, 60 ticks) runs from the muzzle along the points at 8 a second, growing 0.1 → 1.5,
//! its alpha 64 falling over its last 30 ticks; past the end it flies straight on.
//!
//! **Its draw** ([`draw`], `0x3c8e40`, additive): the side strips (`0x3c9238`) — white FX 0xe ribbons 0.05 wide
//! scrolling 0.3 a tick, and FX 0x10 streaks 0.4 wide stretched 0.9 past each point (`0x103fff` on one side, `0x10107f`
//! on the other) — both fading at the ends; the pulses (`0x3c9828`, FX 8, `0x7f` red at their alpha, squares across
//! their way); the muzzle's glow (`0x3c9a50`, FX 0xb: 0.4 of `0x7f207f7f` and 0.1 white, 0.5 along the gun's −row 1);
//! the tube (FX 0xd, `0x400000ff`, two layers scrolling apart, its far end clear) between the rings 1..11, the faces
//! turned away left out.
//!
//! Its glow in the hand (`0x2a7ed8`'s case 0x10): crate::hero::items.
//!
//! Not ported: the upgraded hand **0x0ead** (`0x406cf8`, rows 0x48 "Black Sheepinator" and 0x6e: the muzzle (0.15, −1,
//! 0.06) in its rows, its own targeting `0x4066b8` (no walk through the sheep), drain 12.5 a second (50 at level 3,
//! × 1.33 with item 0x48's row past its base), a black meter `0x60000000`, its beam `0x407640` / draw `0x4084a0` with
//! their own constants (`$gp+0x1de4..`), and the black sheep **0x0eb0** (`0x409cf0`, update `0x409328`: it seeks an
//! enemy (`0x325680`) and charges, its blast `0x409ef0` hurting 10 (55 at level 3) within 3; listed as a decoy
//! `0x3216e0(…, 0x1b1fb0, 0x1f)`)); the weapon's use statistics (`0x18c420..0x18c428`), the gravity frames' branches (the targeting's
//! `0x1a8ff0` / `0x18a168` / `0x1a8ff4` walls-and-ceilings test, `0x380e08`), `0x37c948(0x47, 0x5360b)` after the
//! strips (a GS register reset).

use crate::hero::items::row_at;
use crate::hero::physics::{add, add_rot, atan2, cross, dist, dist2, dot, len, len2, rot_dist, scale, set_len, sub, V3};
use crate::moby::{ai, countdown, Ctx, MobyWorld};
use crate::point_lights::PointLight;

/// The hand's class (item 0x10's row `+0x04`; the challenge mode's level-2 row 0x6d too).
pub const HAND_CLASS: i16 = 0xa5d;
/// The item.
pub const ITEM: i32 = 0x10;
/// The classes the beam and the targeting let through: the sheep and the black sheep (crate::moby::classes::sheep).
pub const SHEEP_CLASSES: [i16; 2] = [crate::moby::classes::sheep::CLASS, 0xeb0];
/// `$gp+0xf64`: the reach.
const RANGE: f32 = 8.0;
/// `$gp+0xf8c`: one twelfth.
const TWELFTH: f32 = f32::from_bits(0x3daa_a8eb);
/// `$gp+0xf60`: the targeting's cone (15°).
const CONE: f32 = 15.0 * 0.017_453_294;
/// The points, the ring's points.
pub const POINTS: usize = 12;
pub const RING: usize = 20;
/// The point light's radius and ticks.
const LIGHT_RADIUS: f32 = 7.5;
const LIGHT_TICKS: i32 = 0x14;
/// The point light's colour (`0x207f7f`, each byte / 128).
const LIGHT_RGB: V3 = [0x7f as f32 / 128.0, 0x7f as f32 / 128.0, 0x20 as f32 / 128.0];
/// The sixtieth of a second.
const TICK: f32 = 0.016_666_668;

/// One pulse running along the beam (`0x1b29a0 + 0x10·k` its place, `0x1b29d0` its way, `0x1b2a00` its ticks,
/// `0x1b2a10` its alpha, `0x1b2a20` how far it has run, `0x1b2a30` its size, `0x1b2a40` live).
#[derive(Clone, Copy, Debug, Default)]
pub struct Pulse {
    pub pos: V3,
    pub way: V3,
    pub ticks: i32,
    pub alpha: f32,
    pub run: f32,
    pub size: f32,
    pub live: bool,
}

/// The beam's globals.
#[derive(Clone, Debug)]
pub struct Beam {
    /// `0x1b2980`: lay it out again on the next tick.
    pub reset: bool,
    /// `0x1b2990`: the points' spacing (the reach / 12); `0x1b2994`: the wave's angle.
    pub spacing: f32,
    pub wave: f32,
    /// `0x240340`: the points; `0x240400` / `0x2404c0`: the side points; `0x240580 + 0x140·i`: the rings.
    pub points: [V3; POINTS],
    pub side: [[V3; POINTS]; 2],
    pub rings: [[V3; RING]; POINTS],
    /// The pulses and the ticks to the next (`0x1b2a4c`).
    pub pulses: [Pulse; 3],
    pub next_pulse: i32,
    /// The draw's scrolls: the strips' (`$gp+0x103c`), the tube's along it (`$gp+0xfb4`) and its two layers' round it
    /// (`$gp+0xfbc` / `$gp+0xfc0`).
    pub strip_scroll: f32,
    pub tube_scroll: [f32; 3],
    /// Ratchet's down (`0x18a0e0`) as the last beam tick saw it (the draw reads it).
    pub down: V3,
    /// Its draw's muzzle way: the gun's row 1 (the draw reads `hand + 0xd0`).
    pub gun_row1: V3,
}

impl Default for Beam {
    fn default() -> Self {
        Beam {
            reset: false,
            spacing: 0.0,
            wave: 0.0,
            points: [[0.0; 3]; POINTS],
            side: [[[0.0; 3]; POINTS]; 2],
            rings: [[[0.0; 3]; RING]; POINTS],
            pulses: [Pulse::default(); 3],
            next_pulse: 0,
            strip_scroll: 0.0,
            tube_scroll: [0.0; 3],
            down: [0.0, 0.0, -1.0],
            gun_row1: [0.0, 1.0, 0.0],
        }
    }
}

/// The weapon's globals (the overlay's BSS).
#[derive(Clone, Debug)]
pub struct Mem {
    /// `0x1b2984`: the point light (crate::point_lights, −1 none); `0x1b2988` its ticks; `0x1b298c` the loop sound.
    pub light: i32,
    pub light_ticks: i32,
    pub sound: i32,
    pub beam: Beam,
    /// `$gp+0xf6c`: the sheep about (the transforms up, the pops down).
    pub count: i32,
    /// `$gp+0x1de0`: level 0xc's class-0xfb4 targets turned.
    pub skill_count: i32,
    /// `0x1b2bb0`: the sheep (16 at most, crate::moby::classes::sheep).
    pub flock: [Option<usize>; 16],
}

impl Default for Mem {
    fn default() -> Self { Mem { light: -1, light_ticks: 0, sound: -1, beam: Beam::default(), count: 0, skill_count: 0, flock: [None; 16] } }
}

/// The level whose class-0xfb4 targets count toward the skill point, the class, the count past which it is given.
const SKILL_LEVEL: i32 = 0xc;
const SKILL_CLASS: i16 = 0xfb4;
const SKILL_POINT: usize = crate::skill_points::point::SHEEP;

/// `0x2eeeb8(t, out, a, b)`: `a + (b − a)·t`.
fn lerp3(t: f32, a: V3, b: V3) -> V3 { add(a, scale(sub(b, a), t)) }

/// `0x31e8d8(a, b, t)`.
fn lerp(a: f32, b: f32, t: f32) -> f32 { a + (b - a) * t }

/// `0x2eef88(out, a, b)` = b × a.
fn cross_ba(a: V3, b: V3) -> V3 { cross(b, a) }

/// `0x325100(r, yaw, pitch)`.
fn polar(r: f32, yaw: f32, pitch: f32) -> V3 { [yaw.cos() * r * pitch.cos(), yaw.sin() * r * pitch.cos(), pitch.sin() * r] }

/// Ratchet's moby's up row (`iRam0018c0b0 + 0xe0`).
fn hero_up(w: &MobyWorld, ctx: &Ctx) -> V3 { ctx.hero_moby.and_then(|h| w.mobys.get(h)).map_or(ctx.hero.rows[2], |m| m.rows[2]) }

/// The row the item's current record is on, its level (`0x2624f4`) and whether it has a Mega upgrade (`0x2624f0`).
fn row_level(game: &crate::game::GameState) -> (u8, bool) {
    let r = game.item_row(ITEM);
    (crate::upgrades::level_of(r), row_at(r).is_some() && MEGA_ROWS.contains(&r))
}

/// The item's records with a Mega upgrade (`−0x10` ≠ 0): its base row and its upgrades 0x48, 0x6d, 0x6e.
const MEGA_ROWS: [usize; 4] = [0x10, 0x48, 0x6d, 0x6e];

/// `0x3c76e0` (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].pvars.resize(crate::moby::PVAR_BYTES, 0);
    let away = ctx.hero.items.ready == 3;
    if w.mobys[id].state == 0 {
        w.mobys[id].state = 1;
        w.sheep.sound = -1;
        w.sheep.light = -1;
        w.sheep.light_ticks = 0;
    }
    {
        let m = &mut w.mobys[id];
        let mut t = m.pv_s(0x1c) as i32;
        countdown(&mut t);
        m.set_pv_s(0x1c, t as i16);
    }
    let ready = if ctx.hero.state == 0x1d {
        countdown(&mut ctx.hero.items.look_wait)
    } else {
        ctx.hero.items.look_wait = 0x28;
        true
    };
    let it = ctx.hero.items;
    if ctx.hero.group < 2 && it.held & it.trigger != 0 && ready {
        fire(w, id, ctx);
    } else {
        stop(w, id, ctx);
    }
    if away && 0 <= w.sheep.light {
        w.point_lights.free(w.sheep.light as usize);
        w.sheep.light = -1;
    }
}

/// The firing part of `0x3c76e0` (module docs).
fn fire(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if ctx.host.anim(&w.mobys[id]).is_none_or(|a| a.seq_b != 2) { ctx.host.set_anim(&w.mobys[id], 2, 0, 0); }
    if !ctx.hero.items.using { ctx.hero.items.use_start(ctx.hero.group); }
    if w.mobys[id].pv_s(0x1e) == 0 { w.mobys[id].set_pv_s(0x1e, 1); }
    if w.mobys[id].pv_s(0x1c) == 0 { w.mobys[id].set_pv_s(0x1c, 0xf); }
    let muzzle = ctx.host.joint(&w.mobys[id], 0).unwrap_or(w.mobys[id].pos);
    w.mobys[id].set_pv_v(0, muzzle);
    if ctx.hero.state == 1 { ctx.hero.request_state(0x1d, true); }
    let hidden = ctx.hero_moby.and_then(|h| w.mobys.get(h)).is_some_and(|m| m.mode & 1 != 0);
    let dir = if ctx.hero.state == 0x1d && hidden {
        ctx.host.camera_rows()[0]
    } else if ctx.hero.items.raised {
        scale(w.mobys[id].rows[1], -1.0)
    } else {
        ctx.hero.rows[0]
    };
    w.mobys[id].set_pv_f(0x48, atan2(dir[0], dir[1]));
    w.mobys[id].set_pv_f(0x4c, atan2(len2(dir), dir[2]));
    let found = target(w, id, ctx, muzzle);
    let mut lock = w.mobys[id].pv_i(0x44);
    countdown(&mut lock);
    w.mobys[id].set_pv_i(0x44, lock);
    let held = ai::link(&w.mobys[id], 0x10);
    if held.is_some_and(|t| w.mobys.get(t).is_none_or(|m| matches!(m.state, 0xfe | 0xfd))) {
        ai::set_link(&mut w.mobys[id], 0x10, None);
        w.mobys[id].set_pv_i(0x44, 0);
    }
    let meter = if found.is_none() && w.mobys[id].pv_i(0x44) == 0 {
        1.0
    } else {
        if let Some(t) = found {
            let held = ai::link(&w.mobys[id], 0x10);
            if Some(t) != held && w.mobys[id].pv_i(0x44) == 0 {
                let h = ai::health(&w.mobys[t]);
                let most = w.mobys[t].pv_s(h + 4) as f32;
                let hp = w.mobys[t].pv_f(h);
                let m = &mut w.mobys[id];
                ai::set_link(m, 0x10, Some(t));
                m.set_pv_f(0x14, most);
                m.set_pv_i(0x44, 0x1e);
                m.set_pv_f(0x18, if most <= 1.0 { 0.0 } else { hp });
            }
        }
        let (level, mega) = row_level(ctx.game);
        let rate = if mega && level == 2 { 35.0 } else { 10.0 };
        let m = &mut w.mobys[id];
        let left = (m.pv_f(0x18) - rate * TICK).max(0.0);
        m.set_pv_f(0x18, left);
        left / m.pv_f(0x14)
    };
    // `0x327240(lerp(1440, 360, f) · π/180/60, &+0x50, 0x60ffffff, 0x40ffc0a0)`: the sine of the phase before its turn.
    let speed = lerp(1440.0, 360.0, meter) * 0.000_290_888_25;
    let phase = w.mobys[id].pv_f(0x50);
    w.mobys[id].set_pv_f(0x50, add_rot(phase, speed));
    let colour = crate::hud::lerp_rgba(phase.sin() * 0.5 + 0.5, 0x60ff_ffff, 0x40ff_c0a0);
    ctx.game.side_meter.show(meter, 1, colour, 0);
    if let Some(t) = ai::link(&w.mobys[id], 0x10) {
        if w.mobys[id].pv_f(0x18) <= 0.0 {
            if ctx.game.level == SKILL_LEVEL && w.mobys[t].o_class == SKILL_CLASS {
                w.sheep.skill_count += 1;
                if 0xf < w.sheep.skill_count { ctx.game.award_skill_point(SKILL_POINT); }
            }
            ctx.host.play_sound(&w.mobys[id], 3, 0);
            transform(w, ctx, t);
            w.sheep.count += 1;
            ai::set_link(&mut w.mobys[id], 0x10, None);
        }
    }
    beam(w, id, ctx);
    if w.sheep.light == -1 {
        let l = PointLight { color: LIGHT_RGB, intensity: 0.0, pos: w.mobys[id].pos, radius: LIGHT_RADIUS };
        w.sheep.light = w.point_lights.alloc(l).map_or(-1, |s| s as i32);
    }
    if 0 <= w.sheep.light {
        w.sheep.light_ticks = LIGHT_TICKS;
        light_at(w, id, None);
    }
    let s = w.sheep.sound;
    if s == -1 || !ctx.host.sound_playing(s) { w.sheep.sound = ctx.host.play_sound(&w.mobys[id], 1, 4); }
}

/// The light 0.5 along the gun's −row 1 from the muzzle, radius 7.5; `colour` rewritten when given (the original's
/// record keeps the colour it was given or last faded to).
fn light_at(w: &mut MobyWorld, id: usize, colour: Option<V3>) {
    let s = w.sheep.light as usize;
    let m = &w.mobys[id];
    let pos = add(m.pv_v(0), set_len(m.rows[1], -0.5));
    let color = colour.or_else(|| w.point_lights.slots.get(s).copied().flatten().map(|l| l.color)).unwrap_or(LIGHT_RGB);
    w.point_lights.set(s, PointLight { color, intensity: 0.0, pos, radius: LIGHT_RADIUS });
}

/// The part of `0x3c76e0` when it does not fire (module docs).
fn stop(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    {
        let m = &mut w.mobys[id];
        ai::set_link(m, 0x10, None);
        m.set_pv_s(0x1e, 0);
        m.set_pv_i(0x44, 0);
    }
    w.sheep.beam.reset = true;
    if 0 <= w.sheep.light {
        let f = w.sheep.light_ticks as f32 * 0.05;
        light_at(w, id, Some([f, f, f * 0.25]));
        let mut t = w.sheep.light_ticks;
        let out = countdown(&mut t);
        w.sheep.light_ticks = t;
        if out || t == 0 {
            w.point_lights.free(w.sheep.light as usize);
            w.sheep.light = -1;
        }
    }
    if 0 <= w.sheep.sound {
        if ctx.host.sound_playing(w.sheep.sound) { ctx.host.stop_sound(w.sheep.sound); }
        w.sheep.sound = -1;
    }
    if ctx.host.anim(&w.mobys[id]).is_some_and(|a| a.flags & 2 != 0) { ctx.host.set_anim(&w.mobys[id], 1, 0, 1); }
    let idle = ctx.hero.idle_seq();
    ctx.hero.items.use_end(ctx.hero.state, ctx.hero_view.seq_b, idle);
}

/// `0x2de878`'s walk through the sheep: the line from `from` to `to` (Ratchet or `ignore` left out); a sheep it meets has
/// its collision taken off (`+0x98` = 0) and the walk goes on from where it met it, six steps and 128 sheep at most.
/// The hit it stopped at (`None`: clear or out of steps) and the sheep taken off, whose collision the caller puts back.
fn walk(w: &mut MobyWorld, ctx: &Ctx, from: V3, to: V3, flags: u32, ignore: Option<usize>, taken: &mut Vec<usize>) -> Walk {
    let mut start = from;
    let mut steps = 0;
    loop {
        let Some(h) = w.coll_line(ctx, start, to, flags, ignore) else { return Walk::Clear };
        steps += 1;
        if 6 < steps { return Walk::Clear; }
        match h.moby {
            Some(m) if SHEEP_CLASSES.contains(&w.mobys[m].o_class) => {
                taken.push(m);
                w.mobys[m].has_collision = false;
                if 0x7f < taken.len() { return Walk::Full(h.point); }
                start = h.point;
            }
            moby => return Walk::Met(moby, h.point),
        }
    }
}

/// What [`walk`] came to.
enum Walk {
    /// Nothing met (or the steps ran out).
    Clear,
    /// The 128th sheep: its point.
    Full(V3),
    /// A moby (or the world) and the point.
    Met(Option<usize>, V3),
}

/// The sheep's collision back (`+0x98` = the class's `+0x10`).
fn give_back(w: &mut MobyWorld, taken: &[usize]) {
    for &s in taken {
        let c = w.class(w.mobys[s].o_class).is_some_and(|c| c.has_collision);
        w.mobys[s].has_collision = c;
    }
}

/// `0x3c6fd0(hand, muzzle, &yaw, &pitch)`: the target. None when the line from Ratchet's middle (his place + 0.5 his
/// up) to the muzzle meets anything (flags 6, Ratchet left out). Else over the target list (`0x1fac60`) — not deleted,
/// not a sheep, with a health block not dying (`+0xe` ≠ 3) and of class type 5, its point raised its aim height along
/// its up: one within 2.5 of the muzzle, within 60° of Ratchet's facing and 2 of the muzzle's height with the line to it
/// clear (flags 6) is taken at once; else one nearer than the best so far (or the one held) within the reach whose angle
/// off the aim (the point the aim reaches at its distance, the pitch raised half the ground's slope `0x18a144`; less
/// the target's radius `+0x0a` / 8 seen from there) is under 15° and the line to it (flags 6, the hand left out) clear
/// but for sheep. A pick sets the pitch to minus its elevation (the yaw when within 10° of it); the one held is kept
/// at once (its sheep left without collision, as the original's early return leaves them).
pub fn target(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, from: V3) -> Option<usize> {
    let centre = add(ctx.hero.pos, scale(hero_up(w, ctx), 0.5));
    if w.coll_line(ctx, centre, from, 6, ctx.hero_moby).is_some() { return None; }
    let held = ai::link(&w.mobys[id], 0x10);
    let mut best = RANGE;
    let mut found = None;
    let mut taken = Vec::new();
    let hero_yaw = ctx.hero_moby.and_then(|h| w.mobys.get(h)).map_or(ctx.hero.rot[2], |m| m.rot[2]);
    for t in w.targets(ctx.host.camera_pos()) {
        let tid = t.id;
        let m = &w.mobys[tid];
        if matches!(m.state, 0xfe | 0xfd) || SHEEP_CLASSES.contains(&m.o_class) || m.mode & ai::AI == 0 { continue; }
        let h = ai::health(m);
        if m.pv_b(h + 0xe) == 3 { continue; }
        let q = add(m.pos, scale(m.rows[2], m.pv_f(h + 0x10)));
        if t.kind != 5 { continue; }
        let radius = m.pv_b(h + 0xa) as f32 * 0.125;
        let yaw_to = atan2(q[0] - from[0], q[1] - from[1]);
        let elev = atan2(dist2(from, q), q[2] - from[2]);
        let yaw = w.mobys[id].pv_f(0x48);
        let off_yaw = rot_dist(yaw, yaw_to);
        let d = dist(from, q);
        let mut near = false;
        if d < 2.5 {
            let p = w.mobys[tid].pos;
            let facing = rot_dist(hero_yaw, atan2(p[0] - ctx.hero.pos[0], p[1] - ctx.hero.pos[1]));
            if facing < 1.047_197_7 && (from[2] - q[2]).abs() < 2.0 { near = true; }
        }
        if near && w.coll_line(ctx, from, q, 6, ctx.hero_moby).is_none() {
            found = Some(tid);
            w.mobys[id].set_pv_f(0x4c, -elev);
            w.mobys[id].set_pv_f(0x48, yaw_to);
            break;
        }
        if !((d <= best || Some(tid) == held) && d <= RANGE) { continue; }
        let pitch = w.mobys[id].pv_f(0x4c);
        let v = add(polar(d, yaw, pitch + ctx.hero.posture.feet.ground_pitch * 0.5), from);
        let mut a = 2.0 * (dist(v, q) / (d + d)).clamp(-1.0, 1.0).asin();
        if CONE < a {
            let off = (radius / d).clamp(-1.0, 1.0).asin();
            if radius < d {
                a -= off;
                if a < 0.0 { a = 0.0; }
            }
        }
        if CONE <= a { continue; }
        let clear = match walk(w, ctx, from, q, 6, Some(id), &mut taken) {
            Walk::Clear | Walk::Full(_) => true,
            Walk::Met(m, _) => m == Some(tid),
        };
        if !clear { continue; }
        w.mobys[id].set_pv_f(0x4c, -elev);
        if off_yaw < 0.174_532_95 { w.mobys[id].set_pv_f(0x48, yaw_to); }
        found = Some(tid);
        best = d;
        if Some(tid) == held { return Some(tid); }
    }
    give_back(w, &taken);
    found
}

/// `0x3c7ed0`: the beam laid out again (module docs).
fn lay_out(w: &mut MobyWorld, id: usize) {
    let m = &w.mobys[id];
    let step = scale(m.rows[0], -(RANGE / POINTS as f32));
    let b = &mut w.sheep.beam;
    b.reset = false;
    b.points[0] = m.pv_v(0);
    for i in 1..POINTS { b.points[i] = add(b.points[i - 1], step); }
    b.tube_scroll = [0.0; 3];
    b.wave = 0.0;
    b.next_pulse = 0;
    for p in b.pulses.iter_mut() {
        p.live = false;
        p.way = step;
    }
}

/// `0x3c7fe0` (module docs).
pub fn beam(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.sheep.beam.reset { lay_out(w, id); }
    let muzzle = w.mobys[id].pv_v(0);
    let rows = w.mobys[id].rows;
    let held = ai::link(&w.mobys[id], 0x10).filter(|&t| w.mobys.get(t).is_some_and(|m| m.mode & ai::AI != 0));
    let target = ai::link(&w.mobys[id], 0x10);
    w.sheep.beam.wave = add_rot(w.sheep.beam.wave, f32::from_bits(0x3e4d_87ae));
    let mut aim = set_len(rows[1], -w.sheep.beam.spacing);
    let end = match held {
        None => add(polar(RANGE, w.mobys[id].pv_f(0x48), w.mobys[id].pv_f(0x4c)), muzzle),
        Some(t) => {
            let m = &w.mobys[t];
            add(m.pos, scale(m.rows[2], m.pv_f(ai::health(m) + 0x10)))
        }
    };
    let up = hero_up(w, ctx);
    let across = |d: V3| { let t = dot(up, d); len(sub(d, scale(up, t))) };
    let reach = match target {
        None => {
            let mut taken = Vec::new();
            let r = match walk(w, ctx, muzzle, end, 0x14, ctx.hero_moby, &mut taken) {
                Walk::Clear => RANGE,
                Walk::Full(p) | Walk::Met(_, p) => across(sub(muzzle, p)),
            };
            give_back(w, &taken);
            r
        }
        Some(t) => across(sub(muzzle, w.mobys[t].pos)),
    };
    let target_pos = target.map(|t| w.mobys[t].pos);
    let down = ctx.hero.gravity.down;
    let b = &mut w.sheep.beam;
    b.down = down;
    b.gun_row1 = rows[1];
    b.spacing = reach * TWELFTH;
    b.points[0] = muzzle;
    b.side[0][0] = muzzle;
    b.side[1][0] = muzzle;
    for i in 1..POINTS {
        let d = sub(b.points[i], b.points[i - 1]);
        let f = i as f32 * TWELFTH;
        if let (true, Some(tp)) = (6 < i, target_pos) {
            let q = set_len(sub(tp, b.points[i]), b.spacing);
            aim = lerp3(0.2, aim, q);
        }
        let q = lerp3(f * -0.499_999_97 + 0.9, d, aim);
        let l = len(q);
        b.points[i] = if l == 0.0 { b.points[i - 1] } else { add(b.points[i - 1], scale(q, b.spacing / l)) };
        let s = sub(b.points[i], b.points[i - 1]);
        let r = set_len(cross_ba(s, down), f32::from_bits(0x3d4c_cccd) + (f32::from_bits(0x3f9d_f3b6) - f32::from_bits(0x3d4c_cccd)) * f);
        for k in 0..RING { b.rings[i][k] = add(crate::camera::rot(r, k as f32 * f32::from_bits(0x3ea0_d97a), s), b.points[i]); }
        let r0 = set_len(s, 1.0);
        let r1 = set_len(r, 1.0);
        let r2 = cross_ba(r1, r0);
        let amp = 0.01 + (0.5 - 0.01) * f;
        let ang = add_rot(b.wave, f32::from_bits(0x3ecc_a2ea) * i as f32);
        let (y, z) = (amp * ang.sin(), amp * ang.cos());
        b.side[0][i] = add(b.points[i], add(scale(r1, y), scale(r2, z)));
        b.side[1][i] = add(b.points[i], sub(scale(r1, y), scale(r2, z)));
    }
    w.register_draw(id);
    sparks(w, id, ctx, reach, target);
    pulses(w);
}

/// The beam's particles (module docs).
fn sparks(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, reach: f32, target: Option<usize>) {
    let b = &w.sheep.beam;
    let (tips, points, down, row1) = ([b.side[0][POINTS - 1], b.side[1][POINTS - 1]], [b.points[0], b.points[1]], b.down, w.mobys[id].rows[1]);
    let g = &mut ctx.game.rng;
    let s = g.randf(0.1, f32::from_bits(0x3fd9_999a));
    let r = g.randi(2);
    let mut spin: i8 = if r != 0 { r as i8 } else { -1 };
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let frame = sys.def_frame(0x61, 7);
    for (tip, red) in tips.into_iter().zip([0x7f10_107fu32, 0x7f10_4fff]) {
        if g.randi(2) != 0 { continue; }
        let fall = f32::from_bits(0x3b44_9ba6);
        if let Some(i) = crate::particles::twinkle(sys, g, s * 0.1, s, fall, tip, 0x1e, red, 0, spin, [0.0; 3], 1) { sys.pool.recs[i][2] = frame; }
        spin = -spin;
        if let Some(i) = crate::particles::twinkle(sys, g, s * 0.07, s * 0.7, fall, tip, 0x1e, 0x7f7f_7f7f, 1, spin, [0.0; 3], 1) { sys.pool.recs[i][2] = frame; }
    }
    let d = set_len(sub(points[1], points[0]), 0.2);
    if g.randi(2) != 0 { return; }
    let at = add(points[0], set_len(row1, -0.25));
    const REDS: [u32; 5] = [0x7f30_7fff, 0x7f00_64ff, 0x7f20_20ff, 0x7f00_7fff, 0x7f20_20ff];
    let c = REDS[g.randi(5) as usize];
    let a = g.randf_signed(0.0, f32::from_bits(0x3e86_0a93));
    let bb = g.randf_signed(0.0, f32::from_bits(0x3e86_0a93));
    let v = crate::camera::rot(d, bb, down);
    let v = crate::camera::rot(v, a, cross_ba(d, down));
    let mut n = ((reach * 1.2) / 0.2) as i32;
    if target.is_some() { n *= 3; }
    // Both follow the target (`t2` = `+0x10` for each call), its aim height kept (`+0x3c`, read by nothing).
    let follow = target.map(|t| (t, aim_height(w, t)));
    let frame = sys.def_frame(0x61, 6);
    if let Some(i) = crate::particles::swirl::swirl(sys, g, s * 0.1, s, at, n as i16, c, 0, spin, v, follow) { sys.pool.recs[i][2] = frame; }
    if let Some(i) = crate::particles::swirl::swirl(sys, g, s * 0.07, s * 0.7, at, n as i16, 0x7f7f_7f7f, 1, -spin, v, follow) { sys.pool.recs[i][2] = frame; }
}

/// `0x31fa20(m)`'s `+0x10`: the aim height (0 without a health block).
fn aim_height(w: &MobyWorld, t: usize) -> f32 { w.mobys.get(t).filter(|m| m.mode & ai::AI != 0).map_or(0.0, |m| m.pv_f(ai::health(m) + 0x10)) }

/// The pulses' tick (module docs).
fn pulses(w: &mut MobyWorld) {
    let b = &mut w.sheep.beam;
    if countdown(&mut b.next_pulse) {
        b.next_pulse = 0x2d;
        if let Some(p) = b.pulses.iter_mut().find(|p| !p.live) {
            *p = Pulse { pos: b.points[0], way: set_len(sub(b.points[1], b.points[0]), 1.0), ticks: 0x3c, alpha: 64.0, run: 0.0, size: 0.1, live: true };
        }
    }
    let (points, spacing) = (b.points, b.spacing);
    for p in b.pulses.iter_mut().filter(|p| p.live) {
        if countdown(&mut p.ticks) {
            p.live = false;
            continue;
        }
        if p.ticks < 0x1e { p.alpha -= f32::from_bits(0x4008_887b); }
        let step = 8.0 * TICK;
        p.run += step;
        let t = (p.run / RANGE).min(1.0);
        p.size = 0.1 + (1.5 - 0.1) * t;
        if RANGE <= p.run {
            p.pos = add(p.pos, scale(p.way, step));
            continue;
        }
        let k = (p.run / spacing) as i32;
        let r = p.run - k as f32 * spacing;
        let k = k.clamp(0, POINTS as i32 - 1) as usize;
        p.pos = points[k];
        let seg = if k == POINTS - 1 { sub(points[POINTS - 1], points[POINTS - 2]) } else { sub(points[k + 1], points[k]) };
        let l = len(seg);
        if l == 0.0 { continue; }
        let inv = 1.0 / l;
        p.pos = add(p.pos, scale(seg, r * inv));
        let s = set_len(seg, inv);
        p.way = set_len(lerp3(0.1, p.way, s), 1.0);
    }
}

/// The square template `0x1b0050` (y, z of four corners).
const SQUARE: [[f32; 2]; 4] = [[-1.0, 1.0], [-1.0, -1.0], [1.0, 1.0], [1.0, -1.0]];
const ST: [[f32; 2]; 4] = [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]];

fn quad(out: &mut Vec<crate::fx::FxQuad>, fx: usize, corners: [V3; 4], st: [[f32; 2]; 4], rgba: [u32; 4]) {
    out.push(crate::fx::FxQuad { fx, additive: true, corners, st, rgba });
}

/// `0x3c8e40` (module docs).
pub fn draw(w: &mut MobyWorld, _id: usize, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let b = &mut w.sheep.beam;
    let s = b.strip_scroll - 0.3;
    b.strip_scroll = if s <= -8.0 { s + 8.0 } else { s };
    let b = &*b;
    strips(out, &b.side[0], b.strip_scroll, 0x10_3fff, cam);
    strips(out, &b.side[1], b.strip_scroll, 0x10_107f, cam);
    draw_pulses(out, b);
    // `0x3c9a50`: the muzzle's glow.
    {
        let c = add(b.points[0], set_len(b.gun_row1, -0.5));
        let to_cam = set_len(sub(cam, c), 1.0);
        let side = set_len(cross_ba(to_cam, b.down), -1.0);
        let up = cross_ba(side, to_cam);
        for (size, rgba) in [(0.4f32, 0x7f20_7f7fu32), (0.1, 0xffff_ffff)] {
            let corners = SQUARE.map(|[y, z]| add(c, add(scale(side, y * size), scale(up, z * size))));
            quad(out, 0xb, corners, ST, [rgba; 4]);
        }
    }
    // The tube.
    let b = &mut w.sheep.beam;
    let t = &mut b.tube_scroll;
    let s = t[0] - f32::from_bits(0x3d23_d70a);
    t[0] = if s <= -8.0 { s + 8.0 } else { s };
    let s = t[1] - f32::from_bits(0x3c23_d70a);
    t[1] = if s <= -7.0 { s + 7.0 } else { s };
    let s = t[2] + f32::from_bits(0x3c23_d70a);
    t[2] = if 5.0 <= s { s - 5.0 } else { s };
    let b = &*b;
    let along = f32::from_bits(0x3d88_88b5);
    let round = f32::from_bits(0x3d4c_cccd);
    let mut u = b.tube_scroll[0];
    let mut colours = [0x4000_00ffu32; 4];
    for i in 2..POINTS {
        let (mut v1, mut v2) = (b.tube_scroll[1], b.tube_scroll[2]);
        for k in 0..RING {
            let j = (k + 1) % RING;
            let c = [b.rings[i][k], b.rings[i][j], b.rings[i - 1][k], b.rings[i - 1][j]];
            let n = cross(sub(c[0], c[1]), sub(c[2], c[1]));
            if 0.0 <= dot(n, sub(c[1], cam)) {
                v1 += round;
                v2 += round;
                continue;
            }
            if i == POINTS - 1 {
                colours[0] = 0xff;
                colours[1] = 0xff;
            }
            for v in [&mut v1, &mut v2] {
                quad(out, 0xd, c, [[u, *v], [u, *v + round], [u - along, *v], [u - along, *v + round]], colours);
                *v += round;
            }
        }
        u += along;
    }
}

/// `0x3c9238(points, 128, 64, 12, 0.05, 0.4, 0.9, rgb)`: a side's strips (module docs).
fn strips(out: &mut Vec<crate::fx::FxQuad>, p: &[V3; POINTS], phase: f32, rgb: u32, cam: V3) {
    let n = POINTS;
    let white = 0x80u32 << 24 | 0x7f_7f7f;
    let mut c = [white; 4];
    let st = [[phase, 0.0], [phase, 1.0], [phase + 1.0, 0.0], [phase + 1.0, 1.0]];
    let side = |a: V3, b: V3| { let s = cross_ba(sub(b, a), sub(a, cam)); let l = len(s); scale(s, 0.05 * if l == 0.0 { 1.0 } else { 1.0 / l }) };
    let s0 = side(p[0], p[1]);
    let mut near = [add(p[0], s0), sub(p[0], s0)];
    for k in 1..n - 1 {
        let s = side(p[k], p[k + 1]);
        let far = [add(p[k], s), sub(p[k], s)];
        if k == 1 {
            c[0] = 0x7f_7f7f;
            c[1] = 0x7f_7f7f;
        } else if k == n - 2 {
            c[2] = 0x7f_7f7f;
            c[3] = 0x7f_7f7f;
        } else if k == 2 {
            c[0] = white;
            c[1] = white;
        }
        quad(out, 0xe, [near[0], near[1], far[0], far[1]], st, c);
        near = far;
    }
    let lit = 0x40u32 << 24 | rgb;
    let mut c = [lit; 4];
    for k in 0..n - 1 {
        let (mut a, mut e) = (sub(p[k], cam), sub(p[k + 1], cam));
        let (la, le) = (len(a), len(e));
        if le < la {
            e = if le == 0.0 { a } else { scale(e, la / le) };
        } else {
            a = if la == 0.0 { e } else { scale(a, le / la) };
        }
        let (mut s0, mut s2) = (add(cam, a), add(cam, e));
        let stretch = scale(sub(s2, s0), 0.9);
        s0 = sub(s0, stretch);
        s2 = add(s2, stretch);
        let wv = cross_ba(sub(cam, s0), sub(s2, s0));
        let l = len(wv);
        let wv = scale(wv, 0.4 * if l == 0.0 { 1.0 } else { 1.0 / l });
        if k == 0 {
            c[0] = rgb;
            c[1] = rgb;
        } else if k == n - 2 {
            c[2] = rgb;
            c[3] = rgb;
        } else if k == 1 {
            c[0] = lit;
            c[1] = lit;
        }
        quad(out, 0x10, [add(s0, wv), sub(s0, wv), add(s2, wv), sub(s2, wv)], ST, c);
    }
}

/// `0x3c9828`: each live pulse a square across its way (module docs).
fn draw_pulses(out: &mut Vec<crate::fx::FxQuad>, b: &Beam) {
    for p in b.pulses.iter().filter(|p| p.live) {
        let c = (p.alpha as i32 as u32) << 24 | 0x7f;
        let up = scale(b.down, -1.0);
        let r1 = set_len(cross_ba(p.way, up), 1.0);
        let r2 = cross_ba(r1, p.way);
        let corners = SQUARE.map(|[y, z]| add(p.pos, add(scale(r1, y * p.size), scale(r2, z * p.size))));
        quad(out, 8, corners, ST, [c; 4]);
    }
}

/// `0x404c18(target)`: the target turned into a sheep — crate::moby::classes::sheep::spawn makes it at the target's
/// place; the target's health block takes the weapon as its last hitter (`+0x17` = 0x10, so the kill's experience is
/// the Sheepinator's), its bolts drop when it is a target (mode 0x1000, `0x31adc0(m, 0, 3)`), and it is deleted unless it
/// dies its own way (`+0xe` 1 or 2: marked 2).
pub fn transform(w: &mut MobyWorld, ctx: &mut Ctx, target: usize) -> Option<usize> {
    let up = row_level(ctx.game).0 != 0;
    let sheep = crate::moby::classes::sheep::spawn(w, ctx, target, up)?;
    let m = &w.mobys[target];
    let block = (m.mode & ai::AI != 0).then(|| ai::health(m));
    if let Some(h) = block { w.mobys[target].set_pv_b(h + 0x17, ITEM as u8); }
    if w.mobys[target].mode & crate::moby::mode::TARGET != 0 { crate::moby::bolts::drop(w, target, 0, 3, ctx); }
    match block.map(|h| (h, w.mobys[target].pv_b(h + 0xe))) {
        Some((h, 1 | 2)) => w.mobys[target].set_pv_b(h + 0xe, 2),
        _ => w.delete(target),
    }
    w.rebuild(sheep, &*ctx.host);
    Some(sheep)
}
