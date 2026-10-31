//! Break pieces (L00): a moby that breaks throws copies of itself, each drawn as one of its class's pieces
//! (`rc_formats::moby::MobyPiece`, the `+0x7c` mask), which tumble, bounce on the level and fade.
//!
//! * **Spawn** `0x327498(h₀, h₁, v₀, v₁, moby, mask, velocity, sound)` ([`break_into`]): for each bit k of the mask (0..14)
//!   a piece flies off at a random heading, `h₀..h₁` and `v₀..v₁` units a second across and up, plus the velocity;
//!   `0x2dbdb0` makes it ([`spawn_piece`]): a moby of the same class at the moby's place and angles, its scale, mode
//!   `& ~0x20 | 0x100` (`| 0x800` when the moby has it), no collision, its source `+0x54`, a spin of 90..270°/s about a
//!   random axis; `0x2dbff8` mode 2 its body: the velocity, the spin (a quaternion a tick), the piece's centre as the
//!   pivot (in the moby's frame), gravity 10..20 /3600 a tick², 180 ticks, the bounce sound.
//! * **Throw every piece** `0x3210c0` ([`throw_all`]): the level's bolts, then each piece in the mask thrown with its
//!   own velocity, spin and gravity drawn from ranges (Oozla's breakables, crate::moby::classes::smash).
//! * **Tick** `0x39c4a8` → `0x2dc1f0` → `0x2dc390` ([`update`]): with no contact it falls and spins about its pivot
//!   (`0x2dc7a8`); with one or two it pivots about them under its own weight (`0x2dc968`, two: about their line); a
//!   point of the piece that crosses the level (the segment from the pivot to it, `0x2dcb80`) pushes it out, turns the
//!   spin by `velocity × normal` (`0x2dd108`, at most 0.75° a tick, the whole spin 1.5°) and bounces the velocity off
//!   the face (`0x2dd228`: on a floor, the normal within 45° of up, it loses 0.45 of its speed, at least 0.033 a tick);
//!   a bounce that leaves no speed makes the point a contact (`0x2dcde8`: the contacts that no longer hold go,
//!   `0x2dce90`; two make a hinge, `0x2dd1d8`); three contacts: at rest. The life counts down (every fourth tick
//!   while it flies freely); at its end (or once one in ten ticks while the frame rate is low) it fades, 2 alpha a tick,
//!   and goes.
//!
//! **pvars** (`+0x68`, 0x80): `+0x00` velocity, `+0x10` the spin, `+0x20` the pivot, `+0x34` the life (s16), `+0x36`
//! the piece, `+0x38` 0.45 the floor's loss, `+0x3c` its least, `+0x40` 0.5, `+0x44` sin 0.75°, `+0x48` the gravity,
//! `+0x4c` π/1800 the weight's turn, `+0x54` the source, `+0x58` the points, `+0x59..+0x5b` the contacts, `+0x5c`
//! their count, `+0x5d` the mode (2), `+0x5e` the sound (0xff none).
//!
//! The contacts test the level's mesh with the port's segment query, as `0x2e2248` tests the triangles `0x2df8d8`
//! gathers. Mode 0 ([`spawn_whole`]) throws the whole moby. Not ported: the gravity frames (`0x1a8ff0`), the other
//! modes (3 the whole moby as a ball, 4 / 5), mode 1's segments on joints.

use super::{mode, Ctx, MobyWorld};
use crate::hero::physics::{add, dot, len, scale, set_len, sub, V3};
use crate::quat::{axis_quat, mul as qmul, rotate, Q};

fn cross(a: V3, b: V3) -> V3 { [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]] }

pub(crate) const VEL: usize = 0x00;
pub(crate) const SPIN: usize = 0x10;
const PIVOT: usize = 0x20;
/// Mode 1: the ball's radius (the pivot's w).
const RADIUS: usize = 0x2c;
const LIFE: usize = 0x34;
const PIECE: usize = 0x36;
const LOSS: usize = 0x38;
const LEAST: usize = 0x3c;
pub(crate) const GRAVITY: usize = 0x48;
const WEIGHT: usize = 0x4c;
const SOURCE: usize = 0x54;
const POINTS: usize = 0x58;
const CONTACTS: usize = 0x59;
pub(crate) const COUNT: usize = 0x5c;
const MODE: usize = 0x5d;
const SOUND: usize = 0x5e;
const BLOCK: usize = 0x80;

/// `0x2ef318`: a frame vector through the rows.
fn through(rows: &[V3; 3], v: V3) -> V3 { std::array::from_fn(|k| v[0] * rows[0][k] + v[1] * rows[1][k] + v[2] * rows[2][k]) }

/// `0x327498(h₀, h₁, v₀, v₁, moby, mask, velocity, sound)` (module doc).
#[allow(clippy::too_many_arguments)]
pub fn break_into(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, h: (f32, f32), v: (f32, f32), mask: u32, velocity: V3, sound: u8) {
    for k in 0..MAX_PIECE {
        if mask >> k & 1 == 0 { continue; }
        let rng = &mut ctx.game.rng;
        let yaw = rng.rand_angle();
        let across = rng.randf(h.0, h.1) * 0.016_666_668;
        let up = rng.randf(v.0, v.1) * 0.016_666_668;
        let vel = add([yaw.cos() * across, yaw.sin() * across, up], velocity);
        let spin = rng.randf(90.0, 270.0);
        let gravity = rng.randf(10.0, 20.0);
        spawn_piece(w, id, ctx, spin, gravity, k as u8, vel, sound);
    }
}

/// L00 `0x32b580` (L01 `0x3210c0`, L02 `0x3311f8`) `(moby, h, spin, gravity, modes, sound, skip, flags, mask, 0xff,
/// 0)`: the bolts (flag 1 clear: the level's once; 0x10: nine times, else 0x20: twice), class sound `sound` (−1 none),
/// then each piece of the class (`0x3078b0`) in `mask` but `skip` thrown (`0x2d1520`): pointed (mode 2) when its bit is
/// set in `modes`, else a ball about its segment (mode 1, [`spawn_ball`]); its gravity and spin drawn from their
/// ranges, its velocity from `h` (x from `h[0]..h[4]`, y `h[1]..h[5]`, z `h[2]..h[6]`; flags 2 / 4: x / y of either
/// sign), a sixtieth a tick, through its rows unless flag 8. (Every caller passes the pieces' sound 0xff and no update
/// of its own.)
#[allow(clippy::too_many_arguments)]
pub fn throw_all(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, h: [f32; 8], spin: (f32, f32), gravity: (f32, f32), modes: u32, sound: i32, skip: i32, flags: u32, mask: u32) {
    if flags & 1 == 0 { super::bolts::level_drop(w, id, 0.0, 1, ctx); }
    if flags & 0x10 != 0 {
        super::bolts::level_drop(w, id, 0.0, 9, ctx);
    } else if flags & 0x20 != 0 {
        super::bolts::level_drop(w, id, 0.0, 2, ctx);
    }
    w.mobys[id].piece = None;
    if 0 <= sound { ctx.host.play_sound(&w.mobys[id], sound, 0); }
    let n = w.class(w.mobys[id].o_class).map_or(0, |c| c.pieces.len());
    for k in 0..n {
        if k as i32 == skip || mask >> k & 1 == 0 { continue; }
        let rng = &mut ctx.game.rng;
        let g = rng.randf(gravity.0, gravity.1);
        let s = rng.randf(spin.0, spin.1);
        let x = if flags & 2 == 0 { rng.randf(h[0], h[4]) } else { rng.randf_signed(h[0], h[4]) };
        let y = if flags & 4 == 0 { rng.randf(h[1], h[5]) } else { rng.randf_signed(h[1], h[5]) };
        let z = rng.randf(h[2], h[6]);
        let mut v: V3 = scale([x, y, z], 0.016_666_668);
        if flags & 8 == 0 {
            let r = w.mobys[id].rows;
            v = std::array::from_fn(|i| v[0] * r[0][i] + v[1] * r[1][i] + v[2] * r[2][i]);
        }
        if modes >> k & 1 != 0 {
            spawn_piece(w, id, ctx, s, g, k as u8, v, 0xff);
        } else {
            let m = &w.mobys[id];
            let (o_class, pos, rot) = (m.o_class, m.pos, m.rot);
            spawn_ball(w, ctx, o_class, Some(id), pos, rot, s, g, k as u8, v, 0xff);
        }
    }
}

/// The mask's bits.
const MAX_PIECE: usize = 15;

/// A piece of the source moby `src`, at its place and angles ([`spawn_shard`]).
#[allow(clippy::too_many_arguments)]
pub fn spawn_piece(w: &mut MobyWorld, src: usize, ctx: &mut Ctx, spin: f32, gravity: f32, piece: u8, vel: V3, sound: u8) -> Option<usize> {
    let s = &w.mobys[src];
    let (o_class, pos, rot, rows) = (s.o_class, s.pos, s.rot, s.rows);
    spawn_shard(w, ctx, o_class, Some(src), pos, rot, Some(rows), spin, gravity, piece, vel, sound)
}

/// `0x2dbdb0(spin, gravity, src, 2, class, piece, place, angles, velocity, sound)` and `0x2dbff8` mode 2: a moby of
/// `o_class` drawn as its piece `piece`, at `pos` with the angles `rot` (or the source's rows); from the source its
/// light and ambient (an enemy's before its flash), its scale ratio (its scale over its class's) and its mode 0x800.
#[allow(clippy::too_many_arguments)]
pub fn spawn_shard(w: &mut MobyWorld, ctx: &mut Ctx, o_class: i16, src: Option<usize>, pos: V3, rot: V3, rows: Option<[V3; 3]>, spin: f32, gravity: f32, piece: u8, vel: V3, sound: u8) -> Option<usize> {
    spawn(w, ctx, o_class, src, pos, rot, rows, spin, gravity, piece, vel, sound, 2)
}

/// [`spawn_shard`] in mode 0 (`0x2dbff8`'s): the whole moby (every piece drawn), its pivot and radius the bounding
/// sphere of its sequence (`0x318028`'s, back into its frame), its points its collision blob's vertices (`0x2e08a8`:
/// at most 32, [`ClassInfo::coll_points`](super::ClassInfo::coll_points)); it ticks as mode 2 does.
#[allow(clippy::too_many_arguments)]
pub fn spawn_whole(w: &mut MobyWorld, ctx: &mut Ctx, o_class: i16, src: Option<usize>, pos: V3, rot: V3, spin: f32, gravity: f32, vel: V3, sound: u8) -> Option<usize> {
    spawn(w, ctx, o_class, src, pos, rot, None, spin, gravity, 0, vel, sound, 0)
}

/// [`spawn_shard`] in mode 1 (`0x2dbff8`'s): the piece a ball about its segment's middle (`0x311b50`), half the
/// segment across; no points.
#[allow(clippy::too_many_arguments)]
pub fn spawn_ball(w: &mut MobyWorld, ctx: &mut Ctx, o_class: i16, src: Option<usize>, pos: V3, rot: V3, spin: f32, gravity: f32, piece: u8, vel: V3, sound: u8) -> Option<usize> {
    spawn(w, ctx, o_class, src, pos, rot, None, spin, gravity, piece, vel, sound, 1)
}

#[allow(clippy::too_many_arguments)]
fn spawn(w: &mut MobyWorld, ctx: &mut Ctx, o_class: i16, src: Option<usize>, pos: V3, rot: V3, rows: Option<[V3; 3]>, spin: f32, gravity: f32, piece: u8, vel: V3, sound: u8, body: u8) -> Option<usize> {
    let info = w.class(o_class)?;
    let class_scale = info.scale;
    let (centre, n, radius) = if body == 0 {
        // Set from the sphere once the moby exists.
        ([0.0; 3], info.coll_points.len(), 0.0)
    } else if body == 1 {
        let [a, b] = (*info.segments.get(piece as usize)?)?;
        let mid = scale(add(a, b), 0.5);
        (mid, 0, len(sub(mid, a)))
    } else {
        match info.pieces.get(piece as usize) { Some(Some((c, p))) => (*c, p.len(), 0.0), _ => return None }
    };
    let (ratio, src_mode, light, ambient) = match src.and_then(|s| w.mobys.get(s)) {
        // An enemy's (mode 0x20) colour before its hit flash: its reaction block's `+4..+6` (`0x31fa50`, `0x3241d8`).
        Some(s) => (w.class(s.o_class).map_or(1.0, |c| if c.scale != 0.0 { s.scale / c.scale } else { 1.0 }), s.mode, s.light, if s.mode & 0x20 != 0 {
            let r = super::ai::react(s);
            [s.pv_b(r + 4), s.pv_b(r + 5), s.pv_b(r + 6)]
        } else {
            s.ambient
        }),
        None => (1.0, 0, 0, [0x40; 3]),
    };
    let sc = class_scale * ratio;
    // Its spin: sin(spin / 2 a tick) about a random axis (`0x31bbc0`: its length drawn too, from sin..sin).
    let half = (spin * 0.000_145_444_12).sin();
    let rng = &mut ctx.game.rng;
    let axis = rng.rand_vec(half, half);
    let id = w.create(o_class);
    let m = &mut w.mobys[id];
    (m.pos, m.rot, m.scale) = (pos, rot, sc);
    m.rows = rows.unwrap_or_else(|| crate::hero::physics::euler_rows(rot));
    (m.light, m.ambient) = (light, ambient);
    // `+0x64` its update (`0x3a6370`) whatever its class: a class without one would leave it mode 2.
    m.mode = (m.mode & !(0x20 | mode::NO_UPDATE)) | mode::KEEP_ROWS | (src_mode & 0x800) | mode::NO_MATRIX;
    m.has_collision = false;
    (m.drawn, m.draw_dist, m.update_dist) = (1, 0x1ff, 0xff);
    m.pvars.resize(BLOCK.max(m.pvars.len()), 0);
    m.set_pv_v(VEL, vel);
    m.set_pv_q(SPIN, [axis[0], axis[1], axis[2], (1.0 - half * half).max(0.0).sqrt()]);
    m.set_pv_f(LOSS, 0.45);
    m.set_pv_f(LEAST, 0.033_333_335);
    m.set_pv_f(0x40, 0.5);
    m.set_pv_f(0x44, 0.013_089_969);
    m.set_pv_f(GRAVITY, gravity * 0.000_277_777_8);
    m.set_pv_f(WEIGHT, 0.001_745_329_3);
    m.set_pv_s(LIFE, 0xb4);
    m.set_pv_s(PIECE, piece as i16);
    m.set_pv_i(SOURCE, src.map_or(-1, |s| s as i32));
    m.pvars[POINTS] = n as u8;
    m.pvars[COUNT] = 0;
    m.pvars[MODE] = body;
    m.pvars[SOUND] = sound;
    m.piece = (body != 0).then_some(piece);
    // The pivot: the piece's centre in the moby's frame (`0x311cf0`, then `0x326248`); mode 0 its sequence's sphere.
    let (centre, radius) = if body == 0 {
        ctx.host.sequence_sphere(&w.mobys[id]).map_or(([0.0; 3], 0.0), |s| ([s[0], s[1], s[2]], s[3]))
    } else {
        (centre, radius)
    };
    let m = &mut w.mobys[id];
    let k = sc * 0.000_976_562_5;
    m.set_pv_v(PIVOT, scale(centre, k));
    m.set_pv_f(RADIUS, radius * k);
    m.update = Some(update);
    Some(id)
}

/// The piece's points in its frame (`0x311cf0`'s list, scaled; mode 0 the collision blob's vertices, `0x2e08a8`).
fn points(w: &MobyWorld, id: usize) -> Vec<V3> {
    let m = &w.mobys[id];
    let k = m.scale * 0.000_976_562_5;
    if m.pvars[MODE] == 0 {
        return w.class(m.o_class).map_or(Vec::new(), |c| c.coll_points.iter().map(|&q| scale(q, k)).collect());
    }
    let piece = m.pv_s(PIECE).max(0) as usize;
    w.class(m.o_class).and_then(|c| c.pieces.get(piece)).and_then(Option::as_ref).map(|(_, p)| p.iter().map(|&q| scale(q, k)).collect()).unwrap_or_default()
}

/// `0x39c4a8`.
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            if step(w, id, ctx) {
                w.mobys[id].set_state(1, -1);
                w.mobys[id].mode |= 0x2800;
            }
        }
        1 => {
            step(w, id, ctx);
            if w.mobys[id].alpha < 2 { w.delete(id); } else { w.mobys[id].alpha -= 2; }
        }
        _ => {}
    }
}

/// `0x2dc1f0` mode 2: true when its life is over (or it left the world).
pub(crate) fn step(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    if w.mobys[id].pvars[MODE] == 1 { return ball(w, id, ctx); }
    let pts = points(w, id);
    let mut done = body(w, id, ctx, &pts);
    let m = &mut w.mobys[id];
    if m.pvars[COUNT] != 0 || ctx.game.frame & 3 == 0 {
        let t = m.pv_s(LIFE);
        if t <= 1 { done = true; }
        m.set_pv_s(LIFE, (t - 1).max(0));
    }
    let p = m.pos;
    let inside = |c: f32| (2.0..=1021.0).contains(&c);
    done || !(inside(p[0]) && inside(p[1]) && inside(p[2]))
}

/// `0x2dc390`: one tick of the body; true when it lies still (three contacts).
fn body(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, pts: &[V3]) -> bool {
    if 3 <= w.mobys[id].pvars[COUNT] { return true; }
    if w.mobys[id].pvars[COUNT] == 0 { fly(w, id) } else { pivot(w, id, pts) }
    let Some((k, normal)) = contact(w, id, ctx, pts) else { return false };
    let n = set_len(normal, 1.0);
    let m = &mut w.mobys[id];
    let vel = m.pv_v(VEL);
    let turn = cross(vel, n);
    let spin = add_turn(m.pv_q(SPIN), turn);
    m.set_pv_q(SPIN, spin);
    let (loss, least) = (m.pv_f(LOSS), m.pv_f(LEAST));
    let (moving, vel) = bounce(loss, least, vel, n, [0.0, 0.0, 1.0]);
    m.set_pv_v(VEL, vel);
    let sound = m.pvars[SOUND];
    let free = m.pvars[COUNT] == 0;
    if !moving {
        if sound != 0xff && free { ctx.host.play_sound(&w.mobys[id], sound as i32, 0); }
        add_contact(w, id, k, pts);
    } else if sound != 0xff {
        ctx.host.play_sound(&w.mobys[id], sound as i32, 0);
    }
    false
}

/// `0x2dc1f0` mode 1 (`0x2dc518`): it flies ([`fly`], `0x2dc888`); a ball about its pivot (radius `+0x2c`) that meets
/// the level or a moby (flags 2, not its source; `0x2dcd48`) is pushed out and its spin turned by `velocity × normal`.
/// On a floor (the normal within 45° of up) half its speed into the face bounces off; under 1/30 it stays on the
/// floor and rolls — its speed eased toward 1/30 by 0.00194 a tick and its spin a tenth of the way to the roll for its
/// radius — and from 1/30 into the face it sounds (its sound, else the level's 8; the slot's position `0x3609d0` is not
/// ported). Against a wall it
/// reflects. Its life counts down every tick; true when it is over (or it left the world).
fn ball(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    fly(w, id);
    let m = &w.mobys[id];
    let centre = add(through(&m.rows, m.pv_v(PIVOT)), m.pos);
    let (r, src) = (m.pv_f(RADIUS), usize::try_from(m.pv_i(SOURCE)).ok());
    let hit = ctx.coll.and_then(|c| crate::collision::coll_sphere_m(c, ctx.scene, centre, r, crate::collision::QueryFlags(2), src));
    if let Some(h) = hit {
        let m = &mut w.mobys[id];
        if let Some(p) = h.pushed_centre { m.pos = add(m.pos, sub(p, centre)); }
        let n = set_len(h.normal, 1.0);
        let mut vel = m.pv_v(VEL);
        let spin = add_turn(m.pv_q(SPIN), cross(vel, n));
        m.set_pv_q(SPIN, spin);
        if 0.7 < dot([0.0, 0.0, 1.0], n) {
            let vn = dot(vel, n);
            let half = vn * 0.5;
            let rolling = half.abs() < 0.033_333_335;
            vel = sub(vel, set_len(n, if rolling { vn } else { vn + half }));
            if rolling {
                let mut speed = len(vel);
                crate::hero::physics::approach(0.033_333_335, 0.001_944_444_5, &mut speed);
                vel = set_len(vel, speed);
                let axis = cross(n, vel);
                let s = if r != 0.0 { (len(axis) * -0.5) / r } else { 0.0 };
                let roll = set_len(axis, s);
                let q = [roll[0], roll[1], roll[2], (1.0 - s * s).max(0.0).sqrt()];
                m.set_pv_q(SPIN, crate::quat::nlerp(m.pv_q(SPIN), q, 0.1));
            }
            m.set_pv_v(VEL, vel);
            let sound = m.pvars[SOUND];
            if 0.033_333_335 <= vn.abs() {
                // Its class sound, or without one the level's 8 on it (`0x360750(8, 0, moby)` at `0x2dc730`).
                if sound != 0xff { ctx.host.play_sound(&w.mobys[id], sound as i32, 0); } else { ctx.host.play_level_sound(Some(&w.mobys[id]), 8, 0); }
            }
        } else {
            let d = dot(vel, n);
            if d < 0.0 { vel = sub(vel, scale(n, 2.0 * d)); }
            m.set_pv_v(VEL, vel);
        }
    }
    let m = &mut w.mobys[id];
    let t = m.pv_s(LIFE);
    m.set_pv_s(LIFE, (t - 1).max(0));
    let p = m.pos;
    let inside = |c: f32| (2.0..=1021.0).contains(&c);
    t <= 1 || !(inside(p[0]) && inside(p[1]) && inside(p[2]))
}

/// `0x2dc7a8`: falls, moves, and spins about its pivot.
fn fly(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    let mut vel = m.pv_v(VEL);
    vel[2] -= m.pv_f(GRAVITY);
    m.set_pv_v(VEL, vel);
    m.pos = add(m.pos, vel);
    let pivot = m.pv_v(PIVOT);
    let before = through(&m.rows, pivot);
    let q = m.pv_q(SPIN);
    m.rows = m.rows.map(|r| rotate(q, r));
    let after = through(&m.rows, pivot);
    m.pos = sub(m.pos, sub(after, before));
}

/// `0x2dc968`: tips over its contact (or the line of its two) under its weight.
fn pivot(w: &mut MobyWorld, id: usize, pts: &[V3]) {
    let m = &mut w.mobys[id];
    let n = m.pvars[COUNT];
    let c0 = m.pvars[CONTACTS] as usize;
    let c1 = m.pvars[CONTACTS + 1] as usize;
    let (Some(&p0), p1) = (pts.get(c0), pts.get(c1).copied()) else { return };
    let (at, hinge) = if n < 2 { (p0, None) } else {
        let p1 = p1.unwrap_or(p0);
        (scale(add(p0, p1), 0.5), Some(through(&m.rows, sub(p0, p1))))
    };
    let before = through(&m.rows, at);
    let arm = set_len(through(&m.rows, sub(at, m.pv_v(PIVOT))), 1.0);
    let torque = set_len(cross([0.0, 0.0, 1.0], arm), m.pv_f(WEIGHT));
    let mut spin = add_turn(m.pv_q(SPIN), torque);
    if let Some(h) = hinge { spin = hinge_spin(spin, h); }
    m.set_pv_q(SPIN, spin);
    m.rows = m.rows.map(|r| rotate(spin, r));
    let after = through(&m.rows, at);
    m.pos = sub(m.pos, sub(after, before));
}

/// `0x2dd108(moby, spin, turn)`: the spin turned by `turn` (its half angle at most sin 0.75°; the spin at most 1.5°).
fn add_turn(spin: Q, turn: V3) -> Q {
    let l = len(turn);
    if l == 0.0 { return spin; }
    let (a, b) = (0.013_089_969_f32.sin(), 0.026_179_939_f32.sin());
    let q = qmul(spin, axis_quat((l * 0.5).min(a), turn));
    if b < len([q[0], q[1], q[2]]) { axis_quat(b, [q[0], q[1], q[2]]) } else { q }
}

/// `0x2dd1d8(moby, spin, d)`: the spin kept about `d` only.
fn hinge_spin(spin: Q, d: V3) -> Q {
    let u = set_len(d, 1.0);
    let s = dot(u, [spin[0], spin[1], spin[2]]);
    axis_quat(s.abs().min(1.0), scale(u, s.signum()))
}

/// `0x2dd228(loss, least, velocity, normal, up)`: the velocity off the face (`0x2ef2a8`); on a floor (`normal · up >
/// 0.7`) it loses `loss` of its speed, at least `least`. Whether any speed is left.
fn bounce(loss: f32, least: f32, vel: V3, n: V3, up: V3) -> (bool, V3) {
    let d = dot(vel, n);
    let mut v = if d < 0.0 { sub(vel, scale(n, 2.0 * d)) } else { vel };
    let mut l = len(v);
    if 0.7 < dot(up, n) {
        let f = (l * loss).max(least);
        l = if l <= f { 0.0 } else { l - f };
        v = set_len(v, l);
    }
    (l != 0.0, v)
}

/// `0x2dcb80`: the first point (not a contact) whose segment from the pivot crosses the level: the piece pushed out
/// along the face's normal, the point and the normal; None when none does.
fn contact(w: &mut MobyWorld, id: usize, ctx: &Ctx, pts: &[V3]) -> Option<(u8, V3)> {
    let coll = ctx.coll?;
    let m = &w.mobys[id];
    let centre = add(through(&m.rows, m.pv_v(PIVOT)), m.pos);
    let held = &m.pvars[CONTACTS..CONTACTS + m.pvars[COUNT].min(3) as usize];
    for (i, &p) in pts.iter().enumerate() {
        if held.contains(&(i as u8)) { continue; }
        let at = add(through(&m.rows, p), m.pos);
        let Some(o) = crate::collision::coll_line_m(coll, None, centre, at, crate::collision::QueryFlags(0), None) else { continue };
        let mut n = set_len(o.normal, 1.0);
        // The face's side the pivot is on.
        if dot(sub(centre, o.point), n) < 0.0 { n = scale(n, -1.0); }
        let depth = dot(sub(at, o.point), n);
        let m = &mut w.mobys[id];
        m.pos = sub(m.pos, scale(n, depth));
        return Some((i as u8, n));
    }
    None
}

/// `0x2dcde8(moby, point, up)`: the point a contact ([`settle`]); two or three make a hinge of the first two.
fn add_contact(w: &mut MobyWorld, id: usize, k: u8, pts: &[V3]) {
    let m = &mut w.mobys[id];
    let slot = m.pvars[COUNT].min(2) as usize;
    m.pvars[CONTACTS + slot] = k;
    m.pvars[COUNT] = slot as u8 + 1;
    settle(m, slot, [0.0, 0.0, 1.0], pts);
    if m.pvars[COUNT] != 1 {
        let (a, b) = (m.pvars[CONTACTS] as usize, m.pvars[CONTACTS + 1] as usize);
        if let (Some(&pa), Some(&pb)) = (pts.get(a), pts.get(b)) {
            let d = through(&m.rows, sub(pa, pb));
            let s = hinge_spin(m.pv_q(SPIN), d);
            m.set_pv_q(SPIN, s);
        }
    }
}

/// `0x2dce90(moby, slot, up)`: flat in the plane across `up`, the contacts that lie beyond the new one (slot `slot`)
/// from it go (each tested once: the one moved into a freed place is not); of three, one goes unless they hold the
/// piece up — the first two's lines from the third must lie on either side. As the original, a contact is the new
/// one when its point's number equals the slot's.
fn settle(m: &mut super::Moby, slot: usize, up: V3, pts: &[V3]) {
    let flat = |v: V3| sub(v, scale(up, dot(up, v)));
    let pt = |m: &super::Moby, c: usize| pts.get(m.pvars[CONTACTS + c] as usize).copied().unwrap_or([0.0; 3]);
    let base = flat(through(&m.rows, pt(m, slot)));
    let mut e = [[0.0f32; 3]; 3];
    let mut j = 0;
    while j < m.pvars[COUNT] as usize {
        let cj = m.pvars[CONTACTS + j] as usize;
        if cj == slot {
            e[j] = base;
        } else {
            let p = pts.get(cj).copied().unwrap_or([0.0; 3]);
            let q = flat(through(&m.rows, sub(p, pt(m, slot))));
            e[j] = q;
            if 0.0 < dot(q, base) {
                let last = m.pvars[COUNT] as usize - 1;
                m.pvars[CONTACTS + j] = m.pvars[CONTACTS + last];
                m.pvars[COUNT] -= 1;
            }
        }
        j += 1;
    }
    if m.pvars[COUNT] == 3 {
        // `0x2eef88(out, a, b)` is `b × a`.
        let (a, b) = (cross(e[2], e[0]), cross(e[2], e[1]));
        let (c, d) = (cross(e[1], e[0]), cross(e[0], e[1]));
        if dot(a, c) < 0.0 {
            m.pvars[COUNT] -= 1;
            m.pvars[CONTACTS + 1] = m.pvars[CONTACTS + 2];
        } else if dot(b, d) < 0.0 {
            m.pvars[COUNT] -= 1;
            m.pvars[CONTACTS] = m.pvars[CONTACTS + 2];
        }
    }
}

/// `0x31e100(density, m, most, &out, mask)`: points spread through the moby's collision primitives (in `mask`, with the
/// flag 0x20000), each in a primitive picked by its volume: `round(density · volume)` of them, at most `most`. A
/// sphere (kind 1) gives a point inside it (drawn in a cube until inside, through the rows); an upright cylinder (kind
/// 3) one at a random angle and `√randf(0, r²)` out, `randf(0, height)` up. Not ported: the joint kinds 2 / 4 (no
/// caller's class has them).
pub fn random_points(w: &MobyWorld, id: usize, ctx: &mut Ctx, density: f32, most: usize, mask: u32) -> Vec<V3> {
    let m = &w.mobys[id];
    let Some(blob) = ctx.scene.and_then(|sc| sc.classes.get(&m.o_class)) else { return Vec::new() };
    let k = m.scale * 0.000_976_562_5;
    let f = |b: u32| f32::from_bits(b);
    let mut prims = Vec::new();
    let mut total = 0.0;
    for (i, p) in blob.prims.iter().enumerate() {
        let w0 = p.q0()[0];
        if mask >> (i & 0x1f) & 1 != 0 && w0 & 0x20000 != 0 {
            let q1 = p.q1();
            let r = f(q1[3]) * k;
            let v = match p.kind() {
                3 => r * r * 4.187_743 * r + r * r * std::f32::consts::FRAC_PI_2 * f(p.q0()[1]) * k,
                1 => r * r * 4.188_790_3 * r,
                _ => 0.0,
            };
            prims.push((p.kind(), [f(q1[0]) * k, f(q1[1]) * k, f(q1[2]) * k], r, f(p.q0()[1]) * k, v));
            total += v;
        }
        if (w0 as i32) < 0 { break; }
    }
    let n = ((density * total + 0.5) as i32).min(most as i32);
    let mut out = Vec::new();
    let rng = &mut ctx.game.rng;
    for _ in 0..n.max(0) {
        let mut t = rng.randf(0.0, total);
        let mut pick = 0;
        while pick + 1 < prims.len() && prims[pick].4 <= t {
            t -= prims[pick].4;
            pick += 1;
        }
        let Some(&(kind, c, r, h, _)) = prims.get(pick) else { break };
        if kind == 3 {
            let s = rng.randf(0.0, r * r);
            let z = rng.randf(0.0, h);
            let a = rng.rand_angle();
            let s = s.sqrt();
            out.push(add(add([a.cos() * s, a.sin() * s, z], m.pos), c));
        } else {
            let v = loop {
                let v = [rng.randf(-r, r), rng.randf(-r, r), rng.randf(-r, r)];
                if len(v) <= r { break v; }
            };
            out.push(add(through(&m.rows, add(c, v)), m.pos));
        }
    }
    out
}

/// The debris chunks' class (its four pieces are both balls and pointed pieces).
pub const CHUNK: i16 = 0x043e;

/// `0x39c6b8`: a debris chunk's update — its body ([`step`]; over, it fades 2 alpha a tick and goes), and its colour
/// (`+0x50`, the high byte set for a tinted one) lit toward white by how squarely it faces the camera (the fifth
/// power of its facing's dot with the camera's), or by a 20-tick sparkle that starts one tick in 45 (a tinted one
/// 0.8 as much).
pub fn update_chunk(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            if step(w, id, ctx) {
                w.mobys[id].set_state(1, -1);
                w.mobys[id].mode |= 0x2800;
            }
        }
        1 => {
            step(w, id, ctx);
            if w.mobys[id].alpha < 2 {
                w.delete(id);
                return;
            }
            w.mobys[id].alpha -= 2;
        }
        _ => {}
    }
    let cam = ctx.host.camera_rows()[0];
    let m = &w.mobys[id];
    let mut f = dot(m.rows[0], cam).abs().powi(5);
    if m.state_ticks == 0 {
        if ctx.game.rng.randi(45) == 0 { w.mobys[id].state_ticks = 20; }
    } else {
        let m = &mut w.mobys[id];
        let k = (m.state_ticks as i32 - 10).abs();
        m.state_ticks -= 1;
        f = (10 - k) as f32 * 0.1;
    }
    let m = &mut w.mobys[id];
    let c = m.pv_i(0x50) as u32;
    if c >> 24 != 0 { f *= 0.8; }
    let lit = crate::hud::lerp_rgba(f, c, 0xffff_ffff);
    m.set_ambient_word(lit);
}

/// `0x327648(reach, speed, s₀, s₁, src, at, n, pointed, vel, colour, rot)`: `n` debris chunks (class 0x43e), each
/// `rand_vec(0, reach)` about `at` moving `rand_vec(0, speed/60) + vel` a tick at the angles `rot` (else a random yaw
/// and roll), a ball (mode 1) or with `pointed` a pointed piece (mode 2), piece `randi(4)`, spinning 180°/s with
/// gravity 10..20, `randf(s₀, s₁)` of the class's scale, the colour's rgb as its ambient and tint (`+0x50`, high byte
/// set when not grey) and its alpha; its update [`update_chunk`].
#[allow(clippy::too_many_arguments)]
pub fn throw_chunks(w: &mut MobyWorld, ctx: &mut Ctx, reach: f32, speed: f32, s: (f32, f32), src: usize, at: V3, n: i32, pointed: bool, vel: V3, colour: u32, rot: Option<V3>) {
    let class_scale = w.class(CHUNK).map_or(1.0, |c| c.scale);
    for _ in 0..n {
        let rng = &mut ctx.game.rng;
        let v = add(rng.rand_vec(0.0, speed * 0.016_666_668), vel);
        let p = add(rng.rand_vec(0.0, reach), at);
        let (a, b) = (rng.rand_angle(), rng.rand_angle());
        let r = rot.unwrap_or([a, 0.0, b]);
        let piece = rng.randi(4) as u8;
        let gravity = rng.randf(10.0, 20.0);
        let c = if pointed {
            spawn_shard(w, ctx, CHUNK, Some(src), p, r, None, 180.0, gravity, piece, v, 0xff)
        } else {
            spawn_ball(w, ctx, CHUNK, Some(src), p, r, 180.0, gravity, piece, v, 0xff)
        };
        let Some(c) = c else { continue };
        let k = ctx.game.rng.randf(s.0, s.1);
        let (red, green, blue) = (colour & 0xff, colour >> 8 & 0xff, colour >> 16 & 0xff);
        let tinted = (red != green || red != blue) as u32;
        let x = &mut w.mobys[c];
        x.update = Some(update_chunk);
        // The class has an update in the game (`0x41af90`, not ported), so its init never sets mode 2.
        x.mode &= !mode::NO_UPDATE;
        x.scale = class_scale * k;
        x.ambient = [red as u8, green as u8, blue as u8];
        x.alpha = (colour >> 24) as u8;
        x.set_pv_i(0x50, (tinted << 24 | blue << 16 | green << 8 | red) as i32);
    }
}
