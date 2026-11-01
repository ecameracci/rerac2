//! Maktar Nebula's small objects (L02 class updates; no names in any reference, so they go by o_class).
//!
//! * 0x0afc (`0x3f0618`): no update (an empty function); 0x0873 creates one in its place.
//! * 0x0aaf (`0x3eb9f8`): on its first tick sequence state 1, no collision, mode 0x43 without TARGET ([`update_0aaf`]).
//! * 0x0873 (`0x3c1d20`): on its first tick a 0x0afc in its place ([`update_0873`]).
//! * 0x0a24 (`0x3d90f0`) / 0x0a55 (`0x3dd430`): on their first tick at their path's first point ([`update_0a24`],
//!   [`update_0a55`]).
//! * 0x0a4b (`0x3dd3c8`): its shadow's slab probed while drawn within 64 of the camera, range 0x3c ([`update_0a4b`]).
//! * 0x0aab / 0x0aac (`0x3eb6f8`): sliding doors, set off by a hook ([`update_0aab`]).
//! * 0x0d0d (`0x424cf8`): a breakable target: shot, it breaks into its pieces, debris chunks and smoke ([`update_0d0d`]).
//! * 0x0adf (`0x3ecbb8`): a platform swinging along its cuboid, carrying Ratchet ([`update_0adf`]).
//! * 0x0e93 (`0x42fb20`): a breakable lamp: two glows, shot it breaks into its pieces, a ball, bolts, debris chunks and
//!   smoke ([`update_0e93`]); 0x0aa4 (`0x3e9498`) another breakable ([`update_0aa4`]); the count of the resort's
//!   breakables 0x12cf keeps ([`update_12cf`]: at 10 left, once, a message).
//! * The lock puzzle: the bomb 0x09c8 ([`update_09c8`]) the Tractor Beam drags, lit when taken, flung by its
//!   catapult 0x0aa6 ([`update_0aa6`]); its blast kills the power nodes 0x0c82 ([`update_0c82`]) and breaks the walls
//!   0x0c1b ([`update_0c1b`]); the lock 0x0c81 ([`update_0c81`]), its four nodes dead, hooks its group: the conveyor
//!   bridge 0x0ab0 ([`update_0ab0`]) unfolds and carries Ratchet on its belt surfaces 0x0aaf.
//!
//! * 0x0a9c (`0x3e82e0`): the slot machines: a wrench hit for 10 bolts spins their reels; the payouts ([`update_0a9c`],
//!   the reels [`draw_0a9c`]).
//!
//! * 0x0c51 (`0x4112c8`): Ratchet in its cuboid, the scenes that give Endako (planet 3) ([`update_0c51`]).
//! * 0x0bf6 (`0x405b58`): the ship to the Jamming Array and back ([`update_0bf6`]).
//! * 0x0c8f (`0x414498`): the hover car between two ends, the ride faded to black ([`update_0c8f`]).
//! * 0x0a57 (`0x3dd4a8`): the footsteps' dust on surface 10 ([`update_0a57`], crate::particles::dust).
//! * The teleporter pad 0x0c60 is crate::moby::classes::telepad.
//!
//! Not ported (the lock puzzle): the catapult's draw `0x3eae18` and its aiming camera.
//!
//! Not ported (the rides): 0x0bf6's hero word `0x18a032` = 2 on the prompt (no reader found), its `0x152c26` = 1 on the
//! return from the Jamming Array (a ship-mode byte), the scenes' end callback `0x405ec0` on the Jamming Array; 0x0c8f's debug prints and
//! `0x18c0d6` = 1 as Ratchet boards.
//!
//! The Electrolyzer's puzzle 0x0896 (`0x3c3b80`, two here) is crate::moby::classes::electrolyzer: its group's
//! 0x0873 / 0x0a24 / 0x0a55 are its pieces, pulses and balls (their starts here, the rest its).
//!
//! Not ported (no visible effect here, or a missing system): 0x0af3 (`0x3efea8`, four: a camera guide along its path `+0`, every placed one has −1 and deletes
//! itself on its first tick); 0x0b25 (`0x3f6820`: the area above z 420 — the tfrags and shrubs below or above it
//! moved out `0x2ee5a0`, and within 150 of (500, 200, 500) the gravity frames `0x1a8ff0` (G-LVL-003), the no-item
//! area, its music; its tie group's 32 instances turned while in view; a texture's scroll). 0x0f15, the arena's
//! scene props, is crate::moby::classes::arena's.
//!
//! Not ported: 0x0adf's lift (`0x3ece68`, its update when the cuboid's z scale is 2: up and down to Ratchet's level
//! as he rides it); 0x0e96 (`0x4301b0`, the arena crowd: it waits for its controller past state 1, cheers by the
//! controller's pvars `+0x6c` and waves about (500, 600); the controller is not ported); 0x0e9c (`0x4305f8`: its tie
//! group's instances turned π/400 a tick and a texture's scroll `+0x74`; the engine draws the ties static); 0x0ccc
//! (`0x417bf0`, moves a moby another class links in); 0x000b (`0x396448`, the ship's pad: its hologram 0x477 and the
//! △ prompt that starts the travel, the main dev's progression), 0x12cf (`0x38b458`, that count), 0x0c24
//! (`0x40b4c0`, the help voice lines).

use crate::moby::{carry_hero, mode, Ctx, MobyWorld};
use crate::shadow_volumes::probe_near;

/// 0x0afc.
pub fn update_0afc(_w: &mut MobyWorld, _id: usize, _ctx: &mut Ctx) {}

/// `0x3eb9f8`: pvars `+0x5c` |= 4, state 1, the collision blob `+0x98` cleared, mode 0x43 without TARGET.
pub fn update_0aaf(w: &mut MobyWorld, id: usize, _ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    if m.state != 0 { return; }
    m.set_pv_i(0x5c, m.pv_i(0x5c) | 4);
    m.set_state(1, -1);
    m.has_collision = false;
    m.mode = (m.mode & !mode::TARGET) | 0x43;
    w.changed = true;
}

/// `0x3c1d20`: pvars `+4` / `+8` 0, `+0x10` the yaw, `+0x18` = `+0x14`, update distance 0x80, state 1; a 0x0afc
/// (`0x315438`) at its position with its update and draw distances, mode |= 0x41, kept in `+0x1c` (index + 1).
pub fn update_0873(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state != 0 { return; }
    let m = &mut w.mobys[id];
    m.set_pv_i(4, 0);
    m.update_dist = 0x80;
    m.state = 1;
    m.set_pv_i(8, 0);
    m.set_pv_f(0x10, m.rot[2]);
    m.set_pv_i(0x18, m.pv_i(0x14));
    let (pos, update_dist, draw_dist) = (m.pos, m.update_dist, m.draw_dist);
    let c = w.create(0x0afc);
    let child = &mut w.mobys[c];
    child.update_dist = update_dist;
    child.draw_dist = draw_dist;
    child.pos = pos;
    child.mode |= 0x41;
    w.rebuild(c, ctx.host);
    w.changed = true;
    w.mobys[id].set_pv_i(0x1c, c as i32 + 1);
}

/// The path table's (`0x1ff1c0`) path `i`'s first point (the spline's `+0x10`).
fn path_start(ctx: &Ctx, i: i32) -> Option<[f32; 3]> {
    let p = usize::try_from(i).ok().and_then(|i| ctx.level.paths.get(i)).and_then(|p| p.first())?;
    Some([p[0], p[1], p[2]])
}

/// 0x0a24 (`0x3d90f0`): on its first tick update distance 0x80, pvars `+0x0c` −1.0, `+0x14` 0, at the first point of
/// its path `+4`, state 1. The game also keeps the pvars of its moby `+0` at `+8` and the path's address at `+0x18`
/// (here the indices at `+0` / `+4` stand for them; crate::moby::classes::electrolyzer reads them).
pub fn update_0a24(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state != 0 { return; }
    let at = path_start(ctx, w.mobys[id].pv_i(4));
    let m = &mut w.mobys[id];
    m.update_dist = 0x80;
    m.set_pv_f(0xc, -1.0);
    m.set_pv_i(0x14, 0);
    if let Some(p) = at { m.pos = p; }
    m.set_state(1, -1);
    w.changed = true;
}

/// 0x0a55 (`0x3dd430`): 0x0a24's start with the path at `+0` (its address kept at `+0x10`) and `+8` 0.
pub fn update_0a55(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state != 0 { return; }
    let at = path_start(ctx, w.mobys[id].pv_i(0));
    let m = &mut w.mobys[id];
    m.update_dist = 0x80;
    m.set_pv_f(0xc, -1.0);
    m.set_pv_i(8, 0);
    if let Some(p) = at { m.pos = p; }
    m.set_state(1, -1);
    w.changed = true;
}

/// 0x0a4b (`0x3dd3c8`): drawn and within 64 of the camera (`0x1ba500`), its shadow's slab probed (`0x321db0`) and its
/// range 0x3c.
pub fn update_0a4b(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) { probe_near(w, id, ctx, 64.0, 0x3c); }

/// 0x0adf (`0x3ecbb8`): a platform on the cuboid `+0x60` (−1: none, state 3). At its start it takes its place in the
/// cuboid's frame (the inverse `+0x40` of its offset from the cuboid; x at `+0x74`, z at `+0x7c`), its phase from
/// `+0x64` (`p·2π − π` at `+0x68`), pvars `+0x5c` |= 0x10 (state 2); then it swings along the frame's y, `0.25·sin`
/// of the phase stepping 60°/s (`$gp+0x1d88` / `$gp+0x1d8c`), with its looped sound 0 (handle `+0x80`), carrying
/// Ratchet (`0x326a60`, the block `+0x20`). A cuboid with a z scale of 2 makes it the lift (not ported: it stays).
pub fn update_0adf(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (old, old_yaw) = (w.mobys[id].pos, w.mobys[id].rot[2]);
    let c = w.mobys[id].pv_i(0x60);
    let Some(v) = usize::try_from(c).ok().and_then(|i| ctx.level.cuboids.get(i)).copied() else {
        if w.mobys[id].state == 0 { w.mobys[id].set_state(3, -1); }
        return;
    };
    match w.mobys[id].state {
        0 => {
            if (v.matrix[2][2] - 2.0).abs() < 0.01 { return; }
            let m = &mut w.mobys[id];
            m.set_pv_i(0x5c, m.pv_i(0x5c) | 0x10);
            let d: [f32; 3] = std::array::from_fn(|k| m.pos[k] - v.matrix[3][k]);
            let local: [f32; 3] = std::array::from_fn(|k| d[0] * v.inverse[0][k] + d[1] * v.inverse[1][k] + d[2] * v.inverse[2][k]);
            m.set_pv_f(0x68, m.pv_f(0x64) * std::f32::consts::TAU - std::f32::consts::PI);
            m.set_pv_f(0x74, local[0]);
            m.set_pv_i(0x80, -1);
            m.set_pv_f(0x7c, local[2]);
            m.set_state(2, -1);
        }
        2 => {
            let m = &mut w.mobys[id];
            let a = crate::hero::physics::add_rot(m.pv_f(0x68), 60.0 * 0.000_290_888_25);
            m.set_pv_f(0x68, a);
            m.set_pv_f(0x78, 0.25 * a.sin());
            let h = w.mobys[id].pv_i(0x80);
            if !ctx.host.sound_playing(h) {
                let h = ctx.host.play_sound(&w.mobys[id], 0, 4);
                w.mobys[id].set_pv_i(0x80, h);
            }
            let m = &mut w.mobys[id];
            let l = [m.pv_f(0x74), m.pv_f(0x78), m.pv_f(0x7c)];
            m.pos = std::array::from_fn(|k| l[0] * v.matrix[0][k] + l[1] * v.matrix[1][k] + l[2] * v.matrix[2][k] + v.matrix[3][k]);
        }
        _ => {}
    }
    let m = &w.mobys[id];
    carry_hero(ctx, id, old, old_yaw, m.pos, m.rot[2]);
}

/// 0x0aab / 0x0aac (`0x3eb6f8`): a sliding door. At its start its home (`+0`) and a pulse phase (`+0x18`); closed (1)
/// its colour `+0x78` pulses (`0x800c0c00` | `(4·sin − 3)·208` within 0x50..0xd0, the phase 6° a tick) and, past 100
/// with Ratchet on a 0x0a61, it glows (two glows 315000 across at (0, −1.25, −2), frames 0 and 1, colour 0x20000012,
/// 15 ticks); hooked (`0x32a838`) it plays its sound 0 and opens (2): coloured 0xff0cff0c, it slides along its facing
/// (0x0aac forward, 0x0aab back) by `+0x10`, its speed `+0x14` gaining (2 + 5·the distance)/3600 a tick; at 2.5 it
/// bounces back at −0.5 of its speed until under 1/60 (state 3).
pub fn update_0aab(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let a = ctx.game.rng.rand_angle();
            let m = &mut w.mobys[id];
            m.set_pv_f(0x18, a);
            let pos = m.pos;
            m.set_pv_v(0, pos);
            m.set_state(1, -1);
        }
        1 => {
            let m = &mut w.mobys[id];
            let a = crate::hero::physics::add_rot(m.pv_f(0x18), f32::from_bits(0x3d56_7752));
            m.set_pv_f(0x18, a);
            let v = (((a.sin() * 4.0 - 3.0) * 208.0) as i32).clamp(0x50, 0xd0);
            let on_pad = ctx.hero.ground_moby.and_then(|g| w.mobys.get(g)).is_some_and(|g| g.o_class == 0x0a61);
            if on_pad && 100 < v {
                let (pos, far) = (w.mobys[id].pos, w.mobys[id].draw_dist);
                if let Some(sys) = ctx.parts.as_deref_mut() {
                    for def in [0, 1] {
                        crate::particles::glow_def(sys, &mut ctx.game.rng, 315_000.0, 0.0, [0.0, -1.25, -2.0], id, pos, far, false, 0x2000_0012, 0xf, def);
                    }
                }
            }
            let m = &mut w.mobys[id];
            m.glow = v as u32 | 0x800c_0c00;
            if m.once & 4 == 0 { return; }
            ctx.host.play_sound(&w.mobys[id], 0, 0);
            w.mobys[id].set_state(2, -1);
        }
        2 => {
            let m = &mut w.mobys[id];
            m.glow = 0xff0c_ff0c;
            let v = m.pv_f(0x14) + 2.0 * 0.000_277_777_78 + m.pv_f(0x10) * 0.000_277_777_78 * 5.0;
            m.set_pv_f(0x14, v);
            m.set_pv_f(0x10, m.pv_f(0x10) + v);
            if 2.5 < m.pv_f(0x10) {
                m.set_pv_f(0x10, 2.5);
                if 0.016_666_668 < m.pv_f(0x14) {
                    m.set_pv_f(0x14, m.pv_f(0x14) * -0.5);
                } else {
                    m.set_pv_f(0x14, 0.0);
                    m.set_state(3, -1);
                }
            }
            let d = if m.o_class == 0x0aac { m.pv_f(0x10) } else { -m.pv_f(0x10) };
            let along = crate::hero::physics::set_len(m.rows[0], d);
            m.pos = crate::hero::physics::add(along, m.pv_v(0));
            w.changed = true;
        }
        _ => {}
    }
}

/// 0x0d0d (`0x424cf8`): a breakable. At its start a target with all its pieces (`0x315c10`); a hit with the flags
/// 0x4010000 breaks it (state 2): no pieces drawn (`+0x7c` = 0; here hidden), `+0` 120, no collision, the level's bolts
/// (`0x32c250(1, m, 1)`), its sound 0, each piece thrown (spin 90..360°/s, gravity 10..20, `randf(−5, −2)` back,
/// `randf_signed(0, 2)` across, `randf(1, 6)` up a second through its rows), four pointed chunks `rand_vec(0, 0.25)`
/// about (−0.8, 0, 1.5) in its frame at `rand_vec(0, 5/60)` plus 1/12 back, `randf(0.5, 1)` of the size, colour
/// 0x50404040 (crate::moby::shards::throw_chunks), and its smoke ([`smoke_0d0d`]); then state 3. Not ported: the
/// count it lowers (`$gp−0x3c0`, 0x12cf's).
pub fn update_0d0d(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use crate::hero::physics::{add, set_len};
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.mode |= mode::TARGET;
            m.set_state(1, -1);
        }
        1 => {
            if w.hit_message(id).is_some_and(|h| h.tmpl.flags & 0x0401_0000 != 0) {
                w.maktar.left -= 1;
                w.mobys[id].set_state(2, -1);
            }
            w.clear_hit(id);
        }
        2 => {
            let m = &mut w.mobys[id];
            m.mode |= 1;
            m.set_pv_i(0, 0x78);
            m.has_collision = false;
            crate::moby::bolts::level_drop(w, id, 1.0, 1, ctx);
            ctx.host.play_sound(&w.mobys[id], 0, 0);
            let pieces = w.class(0x0d0d).map_or(0, |c| c.pieces.len());
            for k in 0..pieces.min(0x10) {
                let rng = &mut ctx.game.rng;
                let gravity = rng.randf(10.0, 20.0);
                let spin = rng.randf(90.0, 360.0);
                let v = [rng.randf(-2.0, -5.0) / 60.0, rng.randf_signed(0.0, 2.0) / 60.0, rng.randf(1.0, 6.0) / 60.0];
                let m = &w.mobys[id];
                let v = through(&m.rows, v);
                let (pos, rot) = (m.pos, m.rot);
                crate::moby::shards::spawn_shard(w, ctx, 0x0d0d, Some(id), pos, rot, None, spin, gravity, k as u8, v, 0xff);
            }
            let m = &w.mobys[id];
            let at = add(through(&m.rows, [-0.8, 0.0, 1.5]), m.pos);
            let back = set_len(m.rows[0], -0.083_333_336);
            crate::moby::shards::throw_chunks(w, ctx, 0.25, 5.0, (0.5, 1.0), id, at, 4, true, back, 0x5040_4040, None);
            smoke_0d0d(w, id, ctx, -0.083_333_336, -0.25);
            w.mobys[id].set_state(3, -1);
        }
        _ => {}
    }
    w.mobys[id].hit_slot = -1;
}

/// 0x0e93 (`0x42fb20`), a breakable lamp. At its start two glows 4 above it (`0x346250`: size 409500, colour
/// 0x28208080, the def's first frame; size 367500 pulled 0.75 toward the camera, 0x20208080, its second; far 0x60). A
/// hit with the flags 0x4010000 while `+0x68`[0] = 0 breaks it (state 2): its pieces thrown (`0x3311f8`: gravity
/// 10..20, spin 90..360°/s, `randf(−5, 2)` back, `randf_signed(0, 2)` across, `randf(1, 6)` up a second through its
/// rows; pieces 0, 1 and 3 pointed, the rest balls; piece 1 left out; no bolts there; its sound 0), the level's bolts
/// from 4 up (`0x32c250(4, m, 1)`), piece 1 a ball with no spin and gravity 15 at (0, −0.1, 0.05) a tick through its
/// rows (`0x2df348`), four pointed debris chunks `rand_vec(0, 0.25)` about (0, 0.2, 4.3) in its frame at
/// `rand_vec(0, 5/60)` plus (0, 0.05, 0.025) through its rows, `randf(0.5, 1)` of the size, colour 0x50404040
/// (crate::moby::shards::throw_chunks), its smoke ([`smoke_0e93`]); then it is deleted (`0x3157a0`).
pub fn update_0e93(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use crate::hero::physics::add;
    match w.mobys[id].state {
        0 => {
            let pos = w.mobys[id].pos;
            if let Some(sys) = ctx.parts.as_deref_mut() {
                let rng = &mut ctx.game.rng;
                crate::particles::glow_def(sys, rng, 409_500.0, 0.0, [0.0, 0.0, 4.0], id, pos, 0x60, false, 0x2820_8080, -1, 0);
                crate::particles::glow_def(sys, rng, 367_500.0, 0.75, [0.0, 0.0, 4.0], id, pos, 0x60, false, 0x2020_8080, -1, 1);
            }
            if w.mobys[id].pv_i(0) != 0 { w.maktar.left -= 1; }
            w.mobys[id].set_state(1, -1);
        }
        1 => {
            if w.hit_message(id).is_some_and(|h| h.tmpl.flags & 0x0401_0000 != 0) && w.mobys[id].pv_i(0) == 0 {
                w.maktar.left -= 1;
                w.mobys[id].set_state(2, -1);
            }
            w.clear_hit(id);
            w.mobys[id].hit_slot = -1;
        }
        2 => {
            crate::moby::shards::throw_all(w, id, ctx, [-5.0, 0.0, 1.0, 0.0, 2.0, 2.0, 6.0, 0.0], (90.0, 360.0), (10.0, 20.0), 0xb, 0, 1, 5, 0xffff);
            crate::moby::bolts::level_drop(w, id, 4.0, 1, ctx);
            let m = &w.mobys[id];
            let (rows, pos, rot, o_class) = (m.rows, m.pos, m.rot, m.o_class);
            crate::moby::shards::spawn_ball(w, ctx, o_class, Some(id), pos, rot, 0.0, 15.0, 1, through(&rows, [0.0, -0.1, 0.05]), 0xff);
            let at = add(through(&rows, [0.0, 0.2, 4.3]), pos);
            let vel = through(&rows, [0.0, 0.05, 0.025]);
            crate::moby::shards::throw_chunks(w, ctx, 0.25, 5.0, (0.5, 1.0), id, at, 4, true, vel, 0x5040_4040, None);
            smoke_0e93(w, id, ctx, 0.016_666_668, 0.233_333_33);
            w.delete(id);
        }
        _ => {}
    }
}

/// `0x42fea8(a, b, m)`: thirty staged puffs (crate::particles::staged, def 0x1003c) from 4 above it (each jittered
/// `randf(−0.25, 0.25)` a way, `0x325bd0`), along its +y row ± `randf(−0.8, 0.8)` of its first and `randf(0.25, 1)` of
/// its up at `randf(a, b)` a tick, `randf(0.2, 0.6)` across, growing in over `10·f`, easing over `30·f` (sinking 0.0025
/// a tick of it all) and out over `7·f` ticks (f `randf(0.5, 1.5)`), coloured `0x80004080..0x80408080` to
/// `0x00004080..0x00408080`.
fn smoke_0e93(w: &MobyWorld, id: usize, ctx: &mut Ctx, a: f32, b: f32) {
    use crate::hero::physics::{add, set_len};
    let m = &w.mobys[id];
    let base = add(m.pos, [0.0, 0.0, 4.0]);
    let rows = m.rows;
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let rng = &mut ctx.game.rng;
    for _ in 0..30 {
        let c = set_len(rows[0], rng.randf(-0.8, 0.8));
        let d = set_len(rows[2], rng.randf(0.25, 1.0));
        let v = set_len(add(add(rows[1], c), d), rng.randf(a, b));
        let f = rng.randf(0.5, 1.5);
        let (i, e, o) = ((f * 10.0) as i32, (f * 30.0) as i32, (f * 7.0) as i32);
        let size = rng.randf(0.2, 0.6);
        let sink = -0.0025 * (i + e + o) as f32;
        let c0 = crate::hud::lerp_rgba(rng.randf(0.0, 1.0), 0x8000_4080, 0x8040_8080);
        let c1 = crate::hud::lerp_rgba(rng.randf(0.0, 1.0), 0x0000_4080, 0x0040_8080);
        let mut p = base;
        for x in p.iter_mut() { *x += rng.randf(-0.25, 0.25); }
        crate::particles::staged(sys, rng, p, [v[0], v[1], v[2], size], [v[0], v[1], v[2] + sink, 0.01], c0, c1, i as i16, e as i16, o as i16, 0x1003c);
    }
}

fn through(rows: &[[f32; 3]; 3], v: [f32; 3]) -> [f32; 3] { std::array::from_fn(|k| v[0] * rows[0][k] + v[1] * rows[1][k] + v[2] * rows[2][k]) }

/// `0x425010(a, b, m)`: twenty staged puffs (crate::particles::staged, def 0x1003c) from 1.75 up and 0.5 back in its
/// frame, each along its facing ± `randf(−1, 1)` across and `randf(−0.15, 1)` up at `randf(a, b)` a tick, `randf(0.2,
/// 0.6)` across, growing in over `10·f`, easing over `30·f` (sinking 1/720 a tick of it all) and out over `7·f` ticks
/// (f `randf(0.5, 1.5)`), coloured `0x80004080..0x80408080` to `0x80008040..0x00408040`.
fn smoke_0d0d(w: &MobyWorld, id: usize, ctx: &mut Ctx, a: f32, b: f32) {
    use crate::hero::physics::{add, scale, set_len};
    let m = &w.mobys[id];
    let base = add(add(scale(m.rows[2], 1.75), m.pos), scale(m.rows[0], -0.5));
    let rows = m.rows;
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let rng = &mut ctx.game.rng;
    for _ in 0..20 {
        let c = set_len(rows[1], rng.randf(-1.0, 1.0));
        let d = set_len(rows[2], rng.randf(-0.15, 1.0));
        let v = set_len(add(add(rows[0], c), d), rng.randf(a, b));
        let f = rng.randf(0.5, 1.5);
        let (i, e, o) = ((f * 10.0) as i32, (f * 30.0) as i32, (f * 7.0) as i32);
        let size = rng.randf(0.2, 0.6);
        let sink = -0.001_388_888_9 * (i + e + o) as f32;
        let c0 = crate::hud::lerp_rgba(rng.randf(0.0, 1.0), 0x8000_4080, 0x8040_8080);
        let c1 = crate::hud::lerp_rgba(rng.randf(0.0, 1.0), 0x8000_8040, 0x0040_8040);
        crate::particles::staged(sys, rng, base, [v[0], v[1], v[2], size], [v[0], v[1], v[2] + sink, 0.01], c0, c1, i as i16, e as i16, o as i16, 0x1003c);
    }
}

/// The power node 0x0c82's class and the bomb that kills it.
pub const NODE: i16 = 0x0c82;
pub const BOMB: i16 = 0x09c8;

/// 0x0c82 (`0x413be8`), a power node feeding the lock 0x0c81. Gone at the start once its mission is done; else twice
/// its class's size, mode 0x8000 one time in two, its height kept (`+0x68`[8]) and two random phases. It bobs ½ about
/// that height (`[4]` on 0.0582 a tick), its spin phase `[0x14]` back 0.1047 a tick and its scroll `[0x10]` back 1/15
/// (in −1..1). Alive (1) it keeps its looped sound 0 (the handle `[0x18]`), fades in `[0xc]` (at 1/60, out past 64
/// from the camera) and, drawn, sparkles ([`node_sparkle`]); a hit with the flags 0x10000 from a bomb 0x09c8 within 5
/// kills it (2): no collision, hidden, its sound stopped. Showing, it draws its arc to the lock ([`draw_0c82`]). Not
/// ported: the word `0x18cfdc` its death sets to 0xffff.
pub fn update_0c82(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use crate::hero::physics::{add_rot, approach, dist, dist2};
    match w.mobys[id].state {
        0 => {
            if ctx.game.missions.get(ctx.game.level.max(0) as usize * 0x10 + w.mobys[id].mission as usize).is_some_and(|&b| b & 0x80 != 0) {
                w.delete(id);
                return;
            }
            let s = w.class(NODE).map_or(1.0, |c| c.scale);
            let rng = &mut ctx.game.rng;
            let flip = rng.randi(2) != 0;
            let m = &mut w.mobys[id];
            m.scale = s + s;
            if flip { m.mode |= 0x8000 } else { m.mode &= 0x7fff }
            m.set_state(1, -1);
            m.set_pv_f(8, m.pos[2]);
            let (a, b) = (ctx.game.rng.rand_angle(), ctx.game.rng.rand_angle());
            let m = &mut w.mobys[id];
            m.set_pv_f(4, a);
            m.set_pv_f(0x14, b);
            w.changed = true;
        }
        1 => {
            if w.mobys[id].drawn != 0 { node_sparkle(w, id, ctx); }
            cache_cuboid(w, ctx, w.mobys[id].pv_i(0));
            let h = w.mobys[id].pv_i(0x18);
            if !ctx.host.sound_playing(h) {
                let h = ctx.host.play_sound(&w.mobys[id], 0, 4);
                w.mobys[id].set_pv_i(0x18, h);
            }
            let far = 64.0 < dist2(w.mobys[id].pos, ctx.host.camera_pos());
            let m = &mut w.mobys[id];
            let mut f = m.pv_f(0xc);
            approach(if far { 0.0 } else { 1.0 }, 1.0 / 60.0, &mut f);
            m.set_pv_f(0xc, f);
            if f != 0.0 {
                w.register_draw(id);
                arc_glint(w, id, ctx);
            }
            let bombed = w.hit_message(id).is_some_and(|h| {
                h.tmpl.flags & 0x10000 != 0 && h.tmpl.source.is_some_and(|s| w.mobys.get(s).is_some_and(|b| b.o_class == BOMB && dist(w.mobys[id].pos, b.pos) < 5.0))
            });
            if bombed {
                let m = &mut w.mobys[id];
                m.set_state(2, -1);
                m.has_collision = false;
                m.mode = (m.mode & !mode::TARGET) | 0x41;
                let h = m.pv_i(0x18);
                m.set_pv_i(0x18, -1);
                ctx.host.stop_sound(h);
                w.changed = true;
            }
            w.clear_hit(id);
        }
        _ => {}
    }
    let m = &mut w.mobys[id];
    let a = add_rot(m.pv_f(4), f32::from_bits(0x3d6e_4bb0));
    m.set_pv_f(4, a);
    m.set_pv_f(0x14, add_rot(m.pv_f(0x14), f32::from_bits(0xbdd6_7752)));
    m.pos[2] = a.sin() * 0.5 + m.pv_f(8);
    let mut s = m.pv_f(0x10) - 0.066_666_67;
    if 1.0 < s { s -= 1.0 } else if s < -1.0 { s += 1.0 }
    m.set_pv_f(0x10, s);
    w.changed = true;
}

/// `0x413f50`: two glints 1.5 up its frame — one still there (size 6, 0x60006080, 5..10 ticks), one within 1.5 of it
/// sinking `randf(2, 5.5)` a second (size 0.75, 0x60004090, 40..90 ticks).
fn node_sparkle(w: &MobyWorld, id: usize, ctx: &mut Ctx) {
    use crate::hero::physics::add;
    let m = &w.mobys[id];
    let p = add(through(&m.rows, [0.0, 0.0, 1.5]), m.pos);
    let hero = ctx.hero.pos;
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let rng = &mut ctx.game.rng;
    let v = rng.rand_vec(0.0, 0.0);
    let life = rng.rand_range(5, 10) as u16;
    let rot = rng.randi(0xff) as u8;
    crate::particles::glint(sys, 6.0, p, v, 0x6000_6080, life, rot, false, hero);
    let at = add(rng.rand_vec(0.0, 1.5), p);
    let sink = -(rng.randf(2.0, 5.5) / 60.0);
    let life = rng.rand_range(0x28, 0x5a) as u16;
    let rot = rng.randi(0xff) as u8;
    crate::particles::glint(sys, 0.75, at, [0.0, 0.0, sink], 0x6000_4090, life, rot, false, hero);
}

/// `0x320b50(group, on)`: every member of the group hooked (`once & 4`, `0x330490`).
fn hook_group(w: &mut MobyWorld, group: i32, on: bool) {
    let Some(g) = usize::try_from(group).ok().and_then(|g| w.groups.get(g)).cloned() else { return };
    for i in g {
        if let Some(m) = w.mobys.get_mut(i as usize) {
            if on { m.once |= 4 } else { m.once &= !4 }
        }
    }
}

/// 0x0c81 (`0x412e98`), the lock: four power nodes (`+0x68`[0..3]) hold it. At the start, its mission done, it hooks
/// its group `[4]` and goes; else hidden, update distance 0x65, its fade `[7]` 1 within 66 of the camera, its panels
/// `[0xc..0xf]` 1, its phases `[8]` 0, `[9]` ½, `[0xb]` random. With all four nodes dead (state 2) it fades out (2:
/// `[7]` down 1/60 a tick) and at 0 deletes them, hooks its group (the doors and bridges), does its mission, keeps a
/// checkpoint at its cuboid `[5]` and goes. Showing (`[7]` ≠ 0): Ratchet against it gives a glint where a line from
/// his centre meets it; in view its phases turn (`[0xb]` 0.0407, `[0xa]` 1/48, `[8]` / `[9]` 0.0111 a tick); its fade
/// eases to 1 within 68 of the camera (1/120). A panel goes with the nodes it stands for (`[0xc]` node 0, `[0xd]`
/// nodes 0 and 1, `[0xe]` 2 and 3, `[0xf]` 3).
pub fn update_0c81(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use crate::hero::physics::{add_rot, approach, dist2};
    let node = |w: &MobyWorld, k: usize| usize::try_from(w.mobys[id].pv_i(4 * k)).ok().filter(|&n| n < w.mobys.len());
    let dead = |w: &MobyWorld, k: usize| node(w, k).is_some_and(|n| w.mobys[n].state == 2);
    let cam = ctx.host.camera_pos();
    match w.mobys[id].state {
        0 => {
            let done = ctx.game.missions.get(ctx.game.level.max(0) as usize * 0x10 + w.mobys[id].mission as usize).is_some_and(|&b| b & 0x80 != 0);
            if done {
                hook_group(w, w.mobys[id].pv_i(0x10), true);
                w.delete(id);
                return;
            }
            let b = ctx.game.rng.rand_angle();
            let m = &mut w.mobys[id];
            m.set_pv_f(0x20, 0.0);
            m.set_pv_f(0x24, 0.5);
            m.set_pv_f(0x2c, b);
            m.set_pv_f(0x1c, if 66.0 < dist2(cam, m.pos) { 0.0 } else { 1.0 });
            for k in 0..4 { m.set_pv_f(0x30 + 4 * k, 1.0); }
            m.update_dist = 0x65;
            m.set_state(1, -1);
            m.drawn = 0;
            m.mode |= 1;
        }
        1 => {
            if (0..4).all(|k| dead(w, k)) { w.mobys[id].set_state(2, -1); }
        }
        2 => {
            let m = &mut w.mobys[id];
            let mut f = m.pv_f(0x1c);
            approach(0.0, 1.0 / 60.0, &mut f);
            m.set_pv_f(0x1c, f);
            if f <= 0.0 {
                for k in 0..4 { if let Some(n) = node(w, k) { w.delete(n); } }
                hook_group(w, w.mobys[id].pv_i(0x10), true);
                ctx.game.mission_done(w.mobys[id].mission);
                if let Some((pos, rot)) = ctx.level.cuboid_place(w.mobys[id].pv_i(0x14)) { ctx.game.save_checkpoint(pos, rot); }
                w.delete(id);
                return;
            }
        }
        _ => {}
    }
    let f = w.mobys[id].pv_f(0x1c);
    if f == 0.0 {
        if dist2(w.mobys[id].pos, cam) < 64.0 {
            let mut f = f;
            approach(1.0, 0.008_333_334, &mut f);
            w.mobys[id].set_pv_f(0x1c, f);
        }
    } else {
        let h = &*ctx.hero;
        if h.wall_moby == Some(id) || h.push_moby == Some(id) {
            // `0x2e1e10(0x189f00, (its x, his centre's y, z), 2)`: the line from his centre; a glint where it meets.
            let from = [h.pos[0], h.pos[1], h.pos[2] + 1.0];
            let to = [w.mobys[id].pos[0], from[1], from[2]];
            if let Some(o) = ctx.coll.and_then(|c| crate::collision::coll_line_m(c, ctx.scene, from, to, crate::collision::QueryFlags(2), ctx.hero_moby)) {
                let hero = h.pos;
                if let Some(sys) = ctx.parts.as_deref_mut() {
                    let rng = &mut ctx.game.rng;
                    let mut p = o.point;
                    p[2] += rng.randf(-0.5, 0.5);
                    let life = rng.rand_range(0x1e, 0x3c) as u16;
                    let rot = rng.randi(0xff) as u8;
                    crate::particles::glint(sys, 1.0, p, [0.0; 3], 0x6010_6070, life, rot, false, hero);
                }
            }
        }
        // `0x2f4008(80, its place)`: in view, its phases run and it draws ([`draw_0c81`]).
        let p = w.mobys[id].pos;
        if !ctx.view.is_some_and(|v| v.culled(80.0, [p[0], p[1], p[2], 1.0])) {
            let m = &mut w.mobys[id];
            m.set_pv_f(0x2c, add_rot(m.pv_f(0x2c), f32::from_bits(0x3d26_ce95)));
            let wrap = |x: f32| if 1.0 < x { x - 1.0 } else { x };
            m.set_pv_f(0x28, wrap(m.pv_f(0x28) + 0.020_833_334));
            m.set_pv_f(0x20, wrap(m.pv_f(0x20) + f32::from_bits(0x3c35_dcc7)));
            m.set_pv_f(0x24, wrap(m.pv_f(0x24) + f32::from_bits(0x3c35_dcc7)));
            w.register_draw(id);
        }
        let m = &mut w.mobys[id];
        let far = 68.0 < dist2(m.pos, cam);
        let mut f = m.pv_f(0x1c);
        approach(if far { 0.0 } else { 1.0 }, 0.008_333_334, &mut f);
        m.set_pv_f(0x1c, f);
    }
    let alive: [bool; 4] = std::array::from_fn(|k| !dead(w, k));
    let m = &mut w.mobys[id];
    if m.pv_f(0x34) != 0.0 && !alive[0] && !alive[1] { m.set_pv_f(0x34, 0.0); }
    if m.pv_f(0x38) != 0.0 && !alive[2] && !alive[3] { m.set_pv_f(0x38, 0.0); }
    if m.pv_f(0x30) != 0.0 && !alive[0] { m.set_pv_f(0x30, 0.0); }
    if m.pv_f(0x3c) != 0.0 && !alive[3] { m.set_pv_f(0x3c, 0.0); }
}

/// Maktar's globals: the conveyor bridges' (`$gp+0x1d20..0x1d2c`), the cuboids the draws use, the breakables' count.
#[derive(Clone, Debug, Default)]
pub struct Globals {
    /// `$gp−0x3c0`: the breakables left (0x12cf counts them, [`update_12cf`]; each takes one off when it breaks).
    pub left: i32,
    /// `$gp+0x11b0`: the arena boss 0x09f8 hit by nothing but the wrench (or Ratchet) since it came out (its skill
    /// point, crate::moby::classes::bosses).
    pub wrench_only: bool,
    /// `$gp+0x1d24`: the belts' way, −1 (the data's) or 1; `reversed` holds 1.
    pub reversed: bool,
    /// `$gp+0x1d28`: the belts' speed a tick.
    pub speed: f32,
    /// The cuboids' matrices (`0x1b368c`), copied as the belts and the power nodes read them for their draws.
    pub cuboids: Vec<(i32, [[f32; 4]; 4])>,
    /// `$gp+0x1118`: the arena gladiators' (0x09ca) count of swing ends at exactly 4 from their target.
    pub combo: i32,
    /// `0x139647`: the slot machines' plays (a save byte in the game, at most 127; here for the level's load).
    pub plays: u8,
    /// `0x1b08fc` / `0x1b090c` moved: the jackpot's odds raised (past 36 plays, once).
    pub odds_raised: bool,
    /// `$gp+0x193c` spent: the bolts the broken machines have thrown (of 300).
    pub coins_out: i32,
}

/// `$gp+0x1d2c`: the beam's pulse, degrees a second.
const BELT_PULSE: f32 = 200.0;

/// The group's members of class `class`.
fn group_of(w: &MobyWorld, id: usize, class: i16) -> Vec<usize> {
    let g = w.mobys[id].group;
    if g < 0 { return Vec::new(); }
    w.groups.get(g as usize).map_or(Vec::new(), |v| v.iter().map(|&i| i as usize).filter(|&i| w.mobys.get(i).is_some_and(|m| m.o_class == class)).collect())
}

fn to_frame(v: crate::hero::physics::V3, r: &[crate::hero::physics::V3; 3]) -> crate::hero::physics::V3 {
    [crate::hero::physics::dot(v, r[0]), crate::hero::physics::dot(v, r[1]), crate::hero::physics::dot(v, r[2])]
}

/// 0x0ab0 (`0x3eba50`), a conveyor bridge's piece, in a group with its belt surfaces 0x0aaf. `+0x68`[0] its role: 0
/// a panel, else an end (the first end found becomes 2, the belt). The first to start sets the group up (`0x3ebfd8`):
/// each piece keeps its place `[0xc..0xe]`, its yaw `[0xf]` and the first's `[9]` (`[0xa]`, the frame the last panel
/// left); the ends wait (2); the panels (1) are folded, hidden and not solid, half a unit along the first's second
/// row and down from the ends' midpoint, turned π about x and a quarter turn more about z. Hooked (`once & 4`) for 6
/// ticks an end starts its looped sound 0 (`[0xb]`) and sets them off (`0x3ec2a8`): role 1 to 6 (done), 2 to 7, the
/// panels to 3. A panel waits 10 frames after the last one left and for none farther from home along its line
/// (`0x3ec380`); then shown and solid (4) it slides home along its line (its speed `[3]`: `0x323170`, accel 1/60, 60 at
/// first), then (5) turns home from where it stopped (`[4]` 0 → 1: accel and brake 1/360, at most 1/6; the place
/// lerped, its yaw a quarter turn and its roll π less by it) and rests (6). The belt (7) waits for every piece to rest,
/// stops its sound and makes the group solid (`0x3ec608`); then (8) runs: its speed eases (by 1/225) to the way / 15,
/// the belt surfaces carry Ratchet (and write their platform blocks) by it along its second row (`0x3ec6a8`); Ratchet
/// walking against it for 30 ticks turns the way; its pulse `[5]`, scroll `[6]` and fade `[7]` run and it draws
/// ([`draw_0ab0`]). Not ported: `[1]`'s camera (`0x1395bb`, `0x311738`), the switch `$gp+0x1d20` that opens them all.
pub fn update_0ab0(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use crate::hero::physics::{add, add_rot, approach, dot, ease, scale, sub};
    use crate::moby::attach::rows_of;
    use std::f32::consts::{FRAC_PI_2, PI};
    let pieces = group_of(w, id, 0x0ab0);
    match w.mobys[id].state {
        0 => {
            let mut sum = [0.0; 3];
            let mut first_end = false;
            for &p in &pieces {
                let m = &mut w.mobys[p];
                let (pos, yaw) = (m.pos, m.rot[2]);
                m.set_pv_v(0x30, pos);
                m.set_pv_f(0x3c, yaw);
                m.set_pv_i(0x28, id as i32 + 1);
                if m.pv_i(0) == 0 {
                    m.set_state(1, -1);
                } else {
                    if !first_end {
                        first_end = true;
                        m.set_pv_i(0, 2);
                    }
                    sum = add(sum, pos);
                    m.set_state(2, -1);
                }
            }
            let mid = scale(sum, 0.5);
            let mut off = scale(w.mobys[id].rows[1], 0.5);
            off[2] -= 0.5;
            for &p in &pieces {
                let m = &mut w.mobys[p];
                if m.pv_i(0) != 0 { continue; }
                m.update_dist = 0xff;
                m.rot[2] = add_rot(m.rot[2], FRAC_PI_2);
                m.rot[0] = PI;
                m.pos = add(mid, off);
                m.has_collision = false;
                m.mode = (m.mode & !mode::TARGET) | 0x41;
                w.rebuild(p, ctx.host);
            }
            w.changed = true;
            return;
        }
        2 => {
            let m = &mut w.mobys[id];
            if m.once & 4 == 0 { return; }
            let t = m.state_ticks;
            m.state_ticks = t.wrapping_add(1);
            if t < 6 { return; }
            let h = ctx.host.play_sound(&w.mobys[id], 0, 4);
            w.mobys[id].set_pv_i(0x2c, h);
            for &p in &pieces {
                let m = &mut w.mobys[p];
                let s = match m.pv_i(0) { 1 => 6, 2 => 7, _ => 3 };
                m.set_state(s, -1);
            }
            return;
        }
        3 => {
            let lead = usize::try_from(w.mobys[id].pv_i(0x28) - 1).ok().filter(|&l| l < w.mobys.len()).unwrap_or(id);
            let frame = ctx.game.frame as f32;
            if (frame - w.mobys[lead].pv_f(0x24)).abs() <= 10.0 { return; }
            let off = |w: &MobyWorld, p: usize| {
                let m = &w.mobys[p];
                let r = rows_of([0.0, 0.0, m.pv_f(0x3c)]);
                to_frame(sub(m.pv_v(0x30), m.pos), &r)[1].abs()
            };
            let mine = off(w, id);
            if pieces.iter().any(|&p| p != id && w.mobys[p].state == 3 && mine < off(w, p)) { return; }
            w.mobys[lead].set_pv_f(0x24, frame);
            let solid = w.class(0x0ab0).is_some_and(|c| c.has_collision);
            let m = &mut w.mobys[id];
            m.set_state(4, -1);
            m.mode = (m.mode & !0x41) | mode::TARGET;
            m.has_collision = solid;
            m.set_pv_f(0xc, 0.0);
            w.changed = true;
        }
        4 => {
            let first = w.mobys[id].state_ticks == 0;
            let m = &mut w.mobys[id];
            m.state_ticks = m.state_ticks.wrapping_add(1);
            let accel = if first { 60.0 } else { 1.0 / 60.0 };
            let r = rows_of([0.0, 0.0, m.pv_f(0x3c)]);
            let home = m.pv_v(0x30);
            let mut local = to_frame(sub(m.pos, home), &r);
            let mut v = m.pv_f(0xc);
            if local[1].abs() <= v.abs() {
                local[1] = 0.0;
                m.set_state(5, -1);
            } else {
                ease(0.0, accel, accel, 1.0, &mut local[1], &mut v);
            }
            m.set_pv_f(0xc, v);
            m.pos = add(home, through(&r, local));
            w.rebuild(id, ctx.host);
            w.changed = true;
            return;
        }
        5 => {
            let m = &mut w.mobys[id];
            if m.state_ticks == 0 {
                let p = m.pos;
                m.set_pv_v(0x40, p);
                m.set_pv_f(0x10, 0.0);
                m.set_pv_f(0xc, 0.0);
            }
            m.state_ticks = m.state_ticks.wrapping_add(1);
            let (mut t, mut v) = (m.pv_f(0x10), m.pv_f(0xc));
            if (t - 1.0).abs() < v.abs() {
                t = 1.0;
                m.set_state(6, -1);
            } else {
                ease(1.0, 0.002_777_778, 0.002_777_778, 0.166_666_67, &mut t, &mut v);
            }
            m.set_pv_f(0x10, t);
            m.set_pv_f(0xc, v);
            let (from, home) = (m.pv_v(0x40), m.pv_v(0x30));
            m.pos = add(from, scale(sub(home, from), t));
            m.rot[2] = add_rot(m.pv_f(0x3c), (1.0 - t) * FRAC_PI_2);
            m.rot[0] = (1.0 - t) * PI;
            w.rebuild(id, ctx.host);
            w.changed = true;
        }
        7 => {
            if pieces.iter().any(|&p| !matches!(w.mobys[p].state, 6 | 7)) { return; }
            let h = w.mobys[id].pv_i(0x2c);
            ctx.host.stop_sound(h);
            w.mobys[id].set_pv_i(0x2c, -1);
            if let Some(g) = usize::try_from(w.mobys[id].group).ok().and_then(|g| w.groups.get(g)).cloned() {
                for i in g {
                    let i = i as usize;
                    let solid = w.class(w.mobys[i].o_class).is_some_and(|c| c.has_collision);
                    w.mobys[i].has_collision = solid;
                }
            }
            w.mobys[id].set_state(8, -1);
            w.changed = true;
        }
        8 => {
            let c = &mut w.maktar;
            let target = if c.reversed { 1.0 } else { -1.0 } * 0.066_666_67;
            approach(target, 0.004_444_444, &mut c.speed);
            let speed = c.speed;
            let v = scale(w.mobys[id].rows[1], speed);
            if belt_carry(w, id, ctx, v) {
                let m = &mut w.mobys[id];
                if speed == target && dot(ctx.hero.vel, v) < -0.001 {
                    let n = m.pv_i(0x20) + 1;
                    m.set_pv_i(0x20, n);
                    if 0x1e < n { w.maktar.reversed = !w.maktar.reversed; }
                } else {
                    m.set_pv_i(0x20, 0);
                }
            }
            let m = &mut w.mobys[id];
            m.set_pv_f(0x14, add_rot(m.pv_f(0x14), BELT_PULSE * 0.000_290_888_25));
            let mut s = m.pv_f(0x18) + speed * 0.5;
            if 1.0 < s { s -= 1.0 } else if s < -1.0 { s += 1.0 }
            m.set_pv_f(0x18, s);
            let mut f = m.pv_f(0x1c);
            approach(1.0, 1.0 / 60.0, &mut f);
            m.set_pv_f(0x1c, f);
            let k = m.pv_i(8);
            if let Some(cb) = usize::try_from(k).ok().and_then(|i| ctx.level.cuboids.get(i)) {
                let mat = cb.matrix;
                let cs = &mut w.maktar.cuboids;
                match cs.iter_mut().find(|(i, _)| *i == k) { Some(e) => e.1 = mat, None => cs.push((k, mat)) }
            }
            w.register_draw(id);
        }
        _ => {}
    }
}

/// `0x3ec6a8(belt, v)`: each belt surface 0x0aaf of the group moves things on it by `v` this tick (its platform
/// block, crate::moby::platform, when it has one; Ratchet by crate::moby::carry_hero); true when Ratchet stands on one.
fn belt_carry(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, v: crate::hero::physics::V3) -> bool {
    let mut on = false;
    for s in group_of(w, id, 0x0aaf) {
        if ctx.hero.ground_moby == Some(s) && ctx.hero.grounded_ticks != 0 { on = true; }
        let m = &mut w.mobys[s];
        if let Some(b) = crate::moby::platform::block(m) {
            let r = m.rot;
            crate::moby::platform::write(m, b, v, r, r);
        }
        crate::moby::carry_hero(ctx, s, [0.0; 3], 0.0, v, 0.0);
    }
    on
}

/// `0x3ec7c8`, the belt's draw: over its cuboid `[2]` (`0x1b368c`), a strip of n quads along the cuboid's second axis
/// (n its length, whole), each from x = 1 to −1 and y a 2/n step of −1..1 in the cuboid's frame, twice: effect 5 with
/// texture (0.5, 0.5..1.5) coloured `sin(pulse)` of the way from 0x30805000 to 0x20805000, and effect 0x10 with s
/// scrolling by `[6]`, t 0.15..0.85, coloured `sin(pulse + π)` of the way from 0x30508000 to 0x20508000; the alphas
/// × its fade `[7]`; blended (`0x2f06b0`'s blend 0: ALPHA 0x44).
pub fn draw_0ab0(w: &MobyWorld, id: usize, out: &mut Vec<crate::fx::FxQuad>) {
    use crate::hero::physics::len;
    let m = &w.mobys[id];
    let k = m.pv_i(8);
    let Some(&(_, mat)) = w.maktar.cuboids.iter().find(|(i, _)| *i == k) else { return };
    let n = len([mat[1][0], mat[1][1], mat[1][2]]) as i32;
    if n < 1 { return; }
    let (p, scroll, fade) = (m.pv_f(0x14), m.pv_f(0x18), m.pv_f(0x1c));
    let fade_rgba = |c: u32| if fade < 1.0 { crate::hud::lerp_rgba(fade, c & 0xff_ffff, c) } else { c };
    let c0 = fade_rgba(crate::hud::lerp_rgba(p.sin() * 0.5 + 0.5, 0x3080_5000, 0x2080_5000));
    let c1 = fade_rgba(crate::hud::lerp_rgba((p + std::f32::consts::PI).sin() * 0.5 + 0.5, 0x3050_8000, 0x2050_8000));
    let world = |x: f32, y: f32| -> crate::hero::physics::V3 { std::array::from_fn(|j| x * mat[0][j] + y * mat[1][j] + mat[3][j]) };
    let step = 2.0 / n as f32;
    for i in 0..n {
        let y0 = -1.0 + step * i as f32;
        let corners = [world(1.0, y0 + step), world(1.0, y0), world(-1.0, y0 + step), world(-1.0, y0)];
        out.push(crate::fx::FxQuad { fx: 5, additive: false, corners, st: [[0.5, 0.5], [0.5, 0.5], [0.5, 1.5], [0.5, 1.5]], rgba: [c0; 4] });
        out.push(crate::fx::FxQuad { fx: 0x10, additive: false, corners, st: [[scroll, 0.15], [1.0 + scroll, 0.15], [scroll, 0.85], [1.0 + scroll, 0.85]], rgba: [c1; 4] });
    }
}

/// `$gp+0x1104`: the bomb's blast size; `$gp+0x1108` its glow at rest; `0x1b00fc` / `0x1b0100` its pulse's ends.
const BOMB_BLAST: f32 = 1.5;
const BOMB_GLOW: u32 = 0x8080_8080;
const BOMB_PULSE: (u32, u32) = (0x8020_2040, 0x8040_40ff);

/// 0x09c8 (`0x3d1020`), the bomb, while the level's chunk is 0. Its pvars: the drag block `+0x50` (the Tractor
/// Beam's, crate::moby::classes::tractor::drag: reach `[0x84]` 2.75, range `[0x88]` 10, flags `[0x70]` 6), its home
/// `[0xa0]` (0.7 over the ground under where it was placed), its velocity flying `[0xb0]`, its fuse `[0xc0]` seconds
/// and ticks `[0xd4]`, its pulse `[0xd8]`, its sphere `[0xe0]` (⅓) and its misses `[0xe4]`.
/// * 1: dragged; the beam taking it (`0x18a3e0`) lights its fuse (2).
/// * 2: dragged; it beeps (the TNT crate's sound 1, at 10 into each second, each half second in the last two); its
///   fuse out it explodes (5); hooked (a catapult 0x0aa6 takes it) it waits (3) for the launch ([`launch_bomb`], 4).
/// * 4: flying; gravity 1/240 a tick²; its sphere meeting anything, or 1 under home, it explodes (5); out of the world
///   (2..1021) it goes home (6).
/// * 5: on its first tick the blast (crate::moby::blast, size 1.5, with a camera shake within 24) and hidden, its
///   sound 1; each tick a sphere of half its ticks, 5 damage, flags 0x830000 (the nodes and walls take it; one met
///   first that is a node takes 4 off its misses); at 11 its misses + 1 grow its sphere past 8 (to 1), a last sphere of
///   6 doing 10, and home (6).
/// * 6: once nothing is within 0.75 of its home it goes back there (`0x3d1790`: shown, unhooked, the teleport-in).
///
/// Drawn it spins and its glow pulses but at rest. Not ported: its cuboid `[0xc4]` (`0x3d1848`: carried into it the
/// screen fades and Ratchet is moved; none on Maktar's bombs is set), the words `0x18ca60` / `$gp+0x1100`.
pub fn update_09c8(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use crate::hero::physics::{add, add_rot, dist};
    if ctx.game.variant != 0 { return; }
    let sphere = |ctx: &Ctx, p: crate::hero::physics::V3, r: f32, flags: u32| {
        ctx.coll.and_then(|c| crate::collision::coll_sphere_m(c, ctx.scene, p, r, crate::collision::QueryFlags(flags), Some(id))).is_some()
    };
    match w.mobys[id].state {
        0 => {
            let ground = crate::moby::ai::ground_z(ctx, 0.5, w.mobys[id].pos, 0);
            let m = &mut w.mobys[id];
            m.update_dist = 0xff;
            m.mode |= mode::TARGET;
            m.set_pv_f(0xe0, f32::from_bits(0x3eaa_7efa));
            m.set_pv_f(0x84, 2.75);
            m.set_pv_i(0x70, m.pv_i(0x70) | 6);
            m.set_state(1, -1);
            m.pos[2] = ground + 0.7;
            m.set_pv_f(0x88, 10.0);
            let p = m.pos;
            m.set_pv_v(0xa0, p);
            m.set_pv_i(0x90, -1);
        }
        1 => {
            super::tractor::drag(w, id, ctx, 0.5, -1);
            if super::tractor::held(w) == Some(id) {
                let m = &mut w.mobys[id];
                m.set_state(2, -1);
                m.set_pv_i(0xd4, (m.pv_f(0xc0) * 60.0) as i32);
            }
        }
        2 => {
            if w.mobys[id].state_ticks == 0 { w.mobys[id].set_pv_i(0xdc, 0); }
            super::tractor::drag(w, id, ctx, 0.5, -1);
            let t = w.mobys[id].pv_i(0xd4);
            if t % 0x3c == 10 || (t < 0x78 && t % 0x1e == 10) { ctx.host.play_sound_of(&w.mobys[id], 0x1f9, 1, 0); }
            let mut t = t;
            let out = crate::moby::countdown(&mut t);
            w.mobys[id].set_pv_i(0xd4, t);
            if out {
                w.mobys[id].set_state(5, -1);
            } else if w.mobys[id].once & 4 != 0 {
                w.mobys[id].set_state(3, -1);
            }
        }
        4 => {
            let m = &mut w.mobys[id];
            let mut v = m.pv_v(0xb0);
            m.pos = add(m.pos, v);
            v[2] -= 0.004_166_667;
            m.set_pv_v(0xb0, v);
            let (p, r, home) = (m.pos, m.pv_f(0xe0), m.pv_v(0xa0));
            if sphere(ctx, p, r, 0) || p[2] < home[2] - 1.0 {
                w.mobys[id].set_state(5, -1);
            }
            let p = w.mobys[id].pos;
            if p.iter().any(|&x| !(2.0..=1021.0).contains(&x)) { w.mobys[id].set_state(6, -1); }
        }
        5 => {
            let ticks = w.mobys[id].state_ticks;
            let pos = w.mobys[id].pos;
            if ticks == 0 {
                crate::moby::blast::burst(w, ctx, pos, &crate::moby::blast::Blast { size: BOMB_BLAST, bursts: [0x19, 0x14, 0x14], puffs: 0x14, streaks: 0x2d, light: false, shake: true });
                let m = &mut w.mobys[id];
                m.drawn = 0;
                m.mode = (m.mode & !mode::TARGET) | 1;
                ctx.host.play_sound(&w.mobys[id], 1, 0);
                let s = ((24.0 - dist(ctx.host.camera_pos(), pos)) * 0.006_250_000_6).clamp(0.0, 0.15);
                if s != 0.0 { ctx.game.shakes.ask(0, s, 0x5a); }
            }
            crate::moby::ai::attack_sphere(w, id, ctx, ticks as f32 * 0.5, 5.0, 1.0, pos, 0x83_0000, 0, 1, 0);
            if w.hits.last_target.is_some_and(|t| w.mobys.get(t).is_some_and(|n| n.o_class == NODE)) {
                let m = &mut w.mobys[id];
                m.set_pv_i(0xe4, (m.pv_i(0xe4) - 4).max(0));
            }
            if 0xb <= ticks {
                let m = &mut w.mobys[id];
                let n = m.pv_i(0xe4);
                m.set_pv_i(0xe4, n + 1);
                let r = if n + 1 < 8 { f32::from_bits(0x3eaa_7efa) } else { ((n - 6) as f32 * 0.095_142_86 + 0.333).min(1.0) };
                m.set_pv_f(0xe0, r);
                crate::moby::ai::attack_sphere(w, id, ctx, 6.0, 10.0, 1.0, pos, 0x83_0000, 0, 1, 0);
                w.mobys[id].set_state(6, -1);
            }
        }
        6 => {
            let home = w.mobys[id].pv_v(0xa0);
            if !sphere(ctx, [home[0], home[1], home[2] + 0.7], 0.75, 5) { bomb_home(w, id, ctx); }
        }
        _ => {}
    }
    let state = w.mobys[id].state;
    let m = &mut w.mobys[id];
    if m.drawn != 0 {
        let a = add_rot(m.rot[1], 0.026_179_94);
        m.rot[1] = a;
        m.rot[2] = add_rot(a, 0.015_126_189);
        m.glow = if state != 1 {
            let p = add_rot(m.pv_f(0xd8), 0.209_439_51);
            m.set_pv_f(0xd8, p);
            crate::hud::lerp_rgba(p.sin() * 0.5 + 0.5, BOMB_PULSE.0, BOMB_PULSE.1)
        } else {
            BOMB_GLOW
        };
    }
    let m = &mut w.mobys[id];
    m.state_ticks = m.state_ticks.wrapping_add(1);
    w.rebuild(id, ctx.host);
    w.changed = true;
}

/// `0x3d1748(bomb, v)`: a catapult's launch — flying (4) at `v` a tick.
pub fn launch_bomb(w: &mut MobyWorld, id: usize, v: crate::hero::physics::V3) {
    let m = &mut w.mobys[id];
    m.set_state(4, -1);
    m.set_pv_v(0xb0, v);
}

/// `0x3d1790`: home — at rest (1) at its home, its drag stopped, shown, a target, unhooked, through the teleport-in
/// carrier (crate::moby::classes::teleport::start).
fn bomb_home(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    m.set_pv_f(0xd8, 0.0);
    m.glow = crate::hud::lerp_rgba(0.5, BOMB_PULSE.0, BOMB_PULSE.1);
    m.set_state(1, -1);
    let home = m.pv_v(0xa0);
    m.pos = home;
    m.set_pv_v(0x60, home);
    m.set_pv_v(0x50, [0.0; 3]);
    m.drawn = 1;
    m.mode = (m.mode & !1) | mode::TARGET;
    m.once &= !4;
    let rot = m.rot;
    super::teleport::start(w, id, home, rot, &[]);
    w.rebuild(id, ctx.host);
}

/// 0x0c1b (`0x446a48`), a wall a bomb breaks: gone at the start once broken (its uid counted); hit with the flags
/// 0x10000 by a bomb 0x09c8 it leaves its wreck 0x0e43 in its place (`0x3303c0`), breaks into its pieces (mask 0x3fff:
/// 3..6 out and up), plays its sound 0, counts its uid and goes.
pub fn update_0c1b(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let uid = w.mobys[id].uid;
    match w.mobys[id].state {
        0 => {
            let gone = ctx.game.uid_dead.get(uid.max(0) as usize).is_some_and(|&d| d != 0) || ctx.game.uid_count(uid) != 0;
            if gone { w.delete(id) } else { w.mobys[id].set_state(1, -1) }
        }
        1 => {
            let bombed = w.hit_message(id).is_some_and(|h| h.tmpl.flags & 0x10000 != 0 && h.tmpl.source.is_some_and(|s| w.mobys.get(s).is_some_and(|b| b.o_class == BOMB)));
            if !bombed {
                w.mobys[id].hit_slot = -1;
                return;
            }
            if w.class(0x0e43).is_some() {
                let c = w.create(0x0e43);
                let (pos, rot, light, ambient) = { let m = &w.mobys[id]; (m.pos, m.rot, m.light, m.ambient) };
                let x = &mut w.mobys[c];
                (x.update_dist, x.draw_dist, x.drawn) = (0xff, 0xff, 1);
                (x.pos, x.rot, x.light, x.ambient) = (pos, rot, light, ambient);
                x.mode &= !7;
                w.rebuild(c, ctx.host);
            }
            crate::moby::shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 6.0), 0x3fff, [0.0; 3], 0xff);
            ctx.host.play_sound(&w.mobys[id], 0, 0);
            ctx.game.count_uid(uid);
            w.delete(id);
        }
        _ => {}
    }
}

/// 0x0aa6 (`0x3e9d48`), the catapult for its bomb `+0x68`[0] (none: it idles, 4). Its pvars: the bomb's velocity
/// `[4]` (`+0x10`), its turn speed `[0x10]`, its launch speed `[0x11]`, its yaw at the start `[0x12]`, its looped turn
/// sound `[0x16]`.
/// * 1: the bomb a target, held by the Tractor Beam within 4 across: the bomb is hooked (its fuse stops counting, its
///   state 3), its velocity cleared (2).
/// * 2: it turns (accel 0.1°, brake 0.3°, at most 6° a tick) to face the bomb (or away, whichever is nearer) and pulls
///   it into its cup 2 up, a tenth of a unit toward Ratchet ([`catapult_pull`]); the beam letting go or Ratchet more
///   than 15 across off, the beam drops it and it fires (3) at `(10 + 2·max(0, d − 4))/60` a tick (d Ratchet's
///   distance), its sound 0.
/// * 3: the bomb flies off at 25° along its yaw away from Ratchet, its velocity's change at most 100/3600 a tick
///   (`0x3ea400`); once past the catapult it is unhooked and launched ([`launch_bomb`]) (1).
///
/// While it turns faster than ¼° a tick its sound 1 loops. Not ported: its draw `0x3eae18`, the camera while aiming
/// (`0x451ee0`, `0x451bb0`, `0x451af8`), the cannonball 0x09bc and its pad 0x0a61.
pub fn update_0aa6(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use crate::hero::physics::{add_rot, atan2, dist, dist2, sub_rot, turn_ease};
    use std::f32::consts::{FRAC_PI_2, PI};
    let bomb = usize::try_from(w.mobys[id].pv_i(0)).ok().filter(|&b| b < w.mobys.len());
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.set_pv_f(0x48, m.rot[2]);
            m.set_pv_i(0x58, -1);
            if bomb.is_none() { m.set_state(4, -1) } else { m.set_state(1, -1) }
        }
        1 => {
            let Some(b) = bomb else { return };
            let near = dist2(w.mobys[b].pos, w.mobys[id].pos) < 4.0;
            if w.mobys[b].mode & mode::TARGET != 0 && super::tractor::held(w) == Some(b) && near {
                w.mobys[b].once |= 4;
                w.mobys[id].set_pv_v(0x10, [0.0; 3]);
                w.mobys[id].set_state(2, -1);
            }
        }
        2 => {
            let Some(b) = bomb else { return };
            let (bp, cp) = (w.mobys[b].pos, w.mobys[id].pos);
            let mut yaw = add_rot(atan2(bp[0] - cp[0], bp[1] - cp[1]), PI);
            if FRAC_PI_2 < sub_rot(yaw, w.mobys[id].rot[2]).abs() { yaw = add_rot(yaw, PI); }
            let m = &mut w.mobys[id];
            let (mut a, mut v) = (m.rot[2], m.pv_f(0x40));
            turn_ease(yaw, 360.0 * 4.848_137_5e-6, 360.0 * 4.848_137_5e-6 * 3.0, 360.0 * 0.000_290_888_25, &mut a, &mut v, 0);
            m.rot[2] = a;
            m.set_pv_f(0x40, v);
            catapult_pull(w, id, b, ctx);
            let hero = ctx.hero.pos;
            if super::tractor::held(w) != Some(b) || 15.0 < dist2(w.mobys[id].pos, hero) {
                super::tractor::release(w);
                let t = (dist(w.mobys[id].pos, hero) - 4.0).max(0.0);
                w.mobys[b].mode &= !mode::TARGET;
                w.mobys[id].set_pv_f(0x44, 10.0 / 60.0 + 2.0 * t / 60.0);
                ctx.host.play_sound(&w.mobys[id], 0, 0);
                w.mobys[id].set_state(3, -1);
            } else if w.mobys[b].mode & mode::TARGET == 0 {
                w.mobys[id].set_state(1, -1);
            }
        }
        3 => {
            let Some(b) = bomb else { return };
            if w.mobys[id].state_ticks == 0 {
                w.mobys[id].set_pv_v(0x10, [0.0; 3]);
                ctx.host.play_sound(&w.mobys[id], 0, 0);
            }
            w.mobys[id].state_ticks = w.mobys[id].state_ticks.wrapping_add(1);
            if let Some(v) = catapult_fly(w, id, b, ctx) {
                w.mobys[b].once &= !4;
                launch_bomb(w, b, v);
                w.mobys[id].set_state(1, -1);
            }
        }
        _ => {}
    }
    let m = &mut w.mobys[id];
    let h = m.pv_i(0x58);
    if 0.004_363_324 < m.pv_f(0x40).abs() {
        if !ctx.host.sound_playing(h) {
            let h = ctx.host.play_sound(&w.mobys[id], 1, 4);
            w.mobys[id].set_pv_i(0x58, h);
        }
        if w.mobys[id].state != 2 { w.mobys[id].set_pv_f(0x40, 0.0); }
    } else {
        if h != -1 { ctx.host.stop_sound(h); }
        w.mobys[id].set_pv_i(0x58, -1);
    }
    w.rebuild(id, ctx.host);
    w.changed = true;
}

/// `0x3ea258`: the bomb sprung into the catapult's cup (2 up; held, a tenth of a unit toward Ratchet's centre): its
/// velocity `[4]` takes `10·d/3600` of its offset d back and loses a thirtieth, and it moves by it.
fn catapult_pull(w: &mut MobyWorld, id: usize, b: usize, ctx: &Ctx) {
    use crate::hero::physics::{add, len, set_len, sub};
    let p = w.mobys[id].pos;
    let mut cup = [p[0], p[1], p[2] + 2.0];
    if super::tractor::held(w) == Some(b) {
        let h = ctx.hero.pos;
        let d = sub([h[0], h[1], h[2] + 1.0], cup);
        let mut l = len(d);
        crate::hero::physics::approach(0.1, 6.0, &mut l);
        cup = add(cup, set_len(d, l));
    }
    let off = sub(w.mobys[b].pos, cup);
    let mut v = w.mobys[id].pv_v(0x10);
    let spring = set_len(off, -(10.0 * len(off) * 0.000_277_777_8));
    let damp = set_len(v, -(len(v) / 30.0));
    v = add(add(v, spring), damp);
    w.mobys[id].set_pv_v(0x10, v);
    let bm = &mut w.mobys[b];
    bm.pos = add(bm.pos, v);
}

/// `0x3ea400`: the bomb flies off at 25° along the catapult's yaw (turned away from Ratchet) at its launch speed, its
/// velocity's change at most 100/3600 a tick; the velocity once it moves away from the catapult, else none.
fn catapult_fly(w: &mut MobyWorld, id: usize, b: usize, ctx: &Ctx) -> Option<crate::hero::physics::V3> {
    use crate::hero::physics::{add, add_rot, atan2, dot, len, set_len, sub, sub_rot};
    let m = &w.mobys[id];
    let mut yaw = m.rot[2];
    let h = ctx.hero.pos;
    if sub_rot(yaw, atan2(h[0] - m.pos[0], h[1] - m.pos[1])).abs() < std::f32::consts::FRAC_PI_2 { yaw = add_rot(yaw, std::f32::consts::PI); }
    let (s, pitch) = (m.pv_f(0x44), 25.0f32.to_radians());
    let want = [yaw.cos() * s * pitch.cos(), yaw.sin() * s * pitch.cos(), pitch.sin() * s];
    let mut v = m.pv_v(0x10);
    let mut dv = sub(want, v);
    let cap = 100.0 * 0.000_277_777_8;
    if cap < len(dv) { dv = set_len(dv, cap); }
    v = add(v, dv);
    w.mobys[id].set_pv_v(0x10, v);
    let bm = &mut w.mobys[b];
    bm.pos = add(bm.pos, v);
    let mut away = sub(bm.pos, w.mobys[id].pos);
    away[2] = 0.0;
    if 0.0 < dot(v, away) {
        w.mobys[id].set_pv_v(0x10, want);
        Some(want)
    } else {
        None
    }
}

/// The cuboid `k`'s matrix kept for the draws (`0x1b368c`; [`Globals::cuboids`]).
fn cache_cuboid(w: &mut MobyWorld, ctx: &Ctx, k: i32) {
    let Some(cb) = usize::try_from(k).ok().and_then(|i| ctx.level.cuboids.get(i)) else { return };
    let mat = cb.matrix;
    let cs = &mut w.maktar.cuboids;
    match cs.iter_mut().find(|(i, _)| *i == k) { Some(e) => e.1 = mat, None => cs.push((k, mat)) }
}

/// The power node's arc (`0x414118`): 13 points from 1.5 up its frame to its lock's cuboid `[0]` (`0x1b368c + 0x30`),
/// each but the first sagging `sin(phase_k)·sin(k·π/13)` (phase_k from `[5]` on by 2π/13 a point), half-width 0.4.
fn node_arc(w: &MobyWorld, id: usize) -> Option<Vec<[f32; 4]>> {
    use crate::hero::physics::{add, add_rot, scale, sub};
    let m = &w.mobys[id];
    let k = m.pv_i(0);
    let &(_, mat) = w.maktar.cuboids.iter().find(|(i, _)| *i == k)?;
    let start = add(through(&m.rows, [0.0, 0.0, 1.5]), m.pos);
    let step = scale(sub([mat[3][0], mat[3][1], mat[3][2]], start), 1.0 / 12.0);
    let mut pts: Vec<[f32; 4]> = (0..13).map(|i| { let p = add(start, scale(step, i as f32)); [p[0], p[1], p[2], 0.4] }).collect();
    let mut ph = m.pv_f(0x14);
    let a = std::f32::consts::PI / 13.0;
    for (i, p) in pts.iter_mut().enumerate().skip(1) {
        p[2] += ph.sin() * (i as f32 * a).sin();
        ph = add_rot(ph, a + a);
    }
    Some(pts)
}

/// `0x414118`, the power node's draw: its arc ([`node_arc`]) as a ribbon ([`crate::fx::ribbon`]: effect 0x17 in
/// 0x60000080, its glow 0x0e three times as wide in 0x60404040, alphas × its fade `[3]`; scrolled by `[4]`).
pub fn draw_0c82(w: &MobyWorld, id: usize, cam: crate::hero::physics::V3, out: &mut Vec<crate::fx::FxQuad>) {
    let Some(pts) = node_arc(w, id) else { return };
    let m = &w.mobys[id];
    let f = m.pv_f(0xc);
    let c = |x: u32| crate::hud::lerp_rgba(f, x & 0xff_ffff, x);
    crate::fx::ribbon(out, 3.0, m.pv_f(0x10), &pts, cam, [0x17, 0x0e], [c(0x6000_0080), c(0x6040_4040)]);
}

/// The draw's glint (`0x414118`, made here in the update: the draws send no particles): somewhere along a random
/// segment of the arc, sinking `randf(2, 5.5)` a second, size 0.75, 0x60004090, 40..90 ticks.
fn arc_glint(w: &MobyWorld, id: usize, ctx: &mut Ctx) {
    use crate::hero::physics::{add, scale, sub};
    if ctx.game.scenes.mode != crate::scene::PLAY { return; }
    let Some(pts) = node_arc(w, id) else { return };
    let hero = ctx.hero.pos;
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let rng = &mut ctx.game.rng;
    let k = rng.randi(0xc) as usize;
    let t = rng.randf(0.0, 1.0);
    let (a, b) = ([pts[k][0], pts[k][1], pts[k][2]], [pts[k + 1][0], pts[k + 1][1], pts[k + 1][2]]);
    let p = add(a, scale(sub(b, a), t));
    let sink = -(rng.randf(2.0, 5.5) / 60.0);
    let life = rng.rand_range(0x28, 0x5a) as u16;
    let rot = rng.randi(0xff) as u8;
    crate::particles::glint(sys, 0.75, p, [0.0, 0.0, sink], 0x6000_4090, life, rot, false, hero);
}

/// The lock's bands (`0x1b1b80..0x1b1c3c`): each quad's four corners `(0, 1, z)` (two at each height), its effect and
/// which colour and spread it takes.
const LOCK_BANDS: [(f32, f32); 3] = [(2.5, 4.5), (2.3, 2.7), (4.3, 4.7)];

/// `0x413460`, the lock's draw: a ring of light about its up axis, twice (the second 6 higher). The frame is its rows
/// with the first 4 long (turned to the camera's side) and the second 10, at its place. Each ring is 16 segments of
/// −π/16 (`0x2f07e8`: a half turn) of seven bands (`0x2f0750`): the main band 2.5..4.5 (effect 5, texture (½, ½),
/// colour `sin([0xb])` of the way from 0x60203040 to 0x40203040) and six bars of effect 0x0e (texture scrolled by
/// `[0xa]`): 2.3..2.7 and 4.3..4.7 (`sin([0xb])` of 0x40408080 → 0x60408080), the same pair drawn apart by 2·`[8]`
/// and by 2·`[9]` (0x60408080 fading out as the phase goes past 0.2, in before it). The alphas × its fade `[7]`.
pub fn draw_0c81(w: &MobyWorld, id: usize, cam: crate::hero::physics::V3, out: &mut Vec<crate::fx::FxQuad>) {
    use crate::hero::physics::{dot, set_len, sub};
    use crate::hud::lerp_rgba;
    let m = &w.mobys[id];
    let mut r0 = set_len(m.rows[0], 4.0);
    let r1 = set_len(m.rows[1], 10.0);
    let r2 = m.rows[2];
    if dot(sub(cam, m.pos), m.rows[0]) < 0.0 { r0 = [-r0[0], -r0[1], -r0[2]]; }
    let pulse = m.pv_f(0x2c).sin() * 0.5 + 0.5;
    let fade = m.pv_f(0x1c);
    let faded = |c: u32| if fade < 1.0 { lerp_rgba(fade, c & 0xff_ffff, c) } else { c };
    let spread = |t: f32| { let t = if t < 0.2 { 1.0 - t * 5.0 } else { t }; lerp_rgba(t, 0x6040_8080, 0x0040_8080) };
    let main = faded(lerp_rgba(pulse, 0x6020_3040, 0x4020_3040));
    let bar = faded(lerp_rgba(pulse, 0x4040_8080, 0x6040_8080));
    let (s8, s9) = (m.pv_f(0x20), m.pv_f(0x24));
    let (c8, c9) = (faded(spread(s8)), faded(spread(s9)));
    let scroll = m.pv_f(0x28);
    // (fx, colour, band, its offsets of the low / high corners)
    let bands: [(usize, u32, (f32, f32), (f32, f32)); 7] = [
        (5, main, LOCK_BANDS[0], (0.0, 0.0)),
        (0xe, bar, LOCK_BANDS[1], (0.0, 0.0)),
        (0xe, bar, LOCK_BANDS[2], (0.0, 0.0)),
        (0xe, c8, LOCK_BANDS[1], (-2.0 * s8, -2.0 * s8)),
        (0xe, c8, LOCK_BANDS[2], (2.0 * s8, 2.0 * s8)),
        (0xe, c9, LOCK_BANDS[1], (-2.0 * s9, -2.0 * s9)),
        (0xe, c9, LOCK_BANDS[2], (2.0 * s9, 2.0 * s9)),
    ];
    let world = |x: f32, y: f32, z: f32| -> crate::hero::physics::V3 { std::array::from_fn(|j| x * r0[j] + y * r1[j] + z * r2[j] + m.pos[j]) };
    let step = -std::f32::consts::PI / 16.0;
    for lift in [0.0, 6.0] {
        for &(fx, c, (lo, hi), (dlo, dhi)) in &bands {
            let st = if fx == 5 { [[0.5, 0.5]; 4] } else { [[scroll, 0.0], [scroll, 1.0], [1.0 + scroll, 0.0], [1.0 + scroll, 1.0]] };
            let (zl, zh) = (lo + dlo + lift, hi + dhi + lift);
            for k in 1..=16 {
                let (a, b) = (step * k as f32, step * (k + 1) as f32);
                let corner = |ang: f32, z: f32| world(-ang.sin(), ang.cos(), z);
                // The pair turned last leads: odd steps draw (b, a), even (a, b), as `0x2f07e8` turns them in turn.
                let (p, q) = if k % 2 == 1 { (b, a) } else { (a, b) };
                out.push(crate::fx::FxQuad { fx, additive: true, corners: [corner(p, zl), corner(p, zh), corner(q, zl), corner(q, zh)], st, rgba: [c; 4] });
            }
        }
    }
}

/// 0x0aa4 (`0x3e9498`), a breakable of the resort: update distance 0xff (with `+0x68`[0] set, it is not counted). In
/// chunk 0, a hit with the flags 0x4010000 while `[0]` = 0 breaks it (2), one off the count: its pieces thrown but
/// piece 5 (`0x3311f8`: gravity 10..20, spin 90..360, `randf(−6, 0)` back, `randf(−4, 4)` across, `randf(4, 7)` up a
/// second; no bolts there), the level's bolts 5 times from ½ up, the blast (size 1, with the shake and the ring); then
/// (3) only its piece 5 drawn, the wreck. Not ported: its draw once a frame (`0x3e9668`).
pub fn update_0aa4(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            w.mobys[id].update_dist = 0xff;
            w.mobys[id].set_state(1, -1);
            if w.mobys[id].pv_i(0) != 0 { w.maktar.left -= 1; }
        }
        1 => {
            if ctx.game.variant == 1 { return; }
            if w.hit_message(id).is_some_and(|h| h.tmpl.flags & 0x0401_0000 != 0) && w.mobys[id].pv_i(0) == 0 {
                w.maktar.left -= 1;
                w.mobys[id].set_state(2, -1);
            }
            w.mobys[id].hit_slot = -1;
        }
        2 => {
            crate::moby::shards::throw_all(w, id, ctx, [-6.0, -4.0, 4.0, 0.0, 0.0, 4.0, 7.0, 0.0], (90.0, 360.0), (10.0, 20.0), 0xffff, 0, 5, 1, 0xffff);
            crate::moby::bolts::level_drop(w, id, 0.5, 5, ctx);
            let pos = w.mobys[id].pos;
            crate::moby::blast::burst(w, ctx, pos, &crate::moby::blast::Blast { size: 1.0, bursts: [0x19, 0x14, 0x14], puffs: 0x14, streaks: 0x2d, light: false, shake: true });
            w.mobys[id].set_state(3, -1);
        }
        3 => {
            w.mobys[id].piece = Some(5);
            w.changed = true;
        }
        _ => {}
    }
}

/// The classes 0x12cf counts on Maktar.
const COUNTED: [i16; 5] = [0x0a9c, 0x0d0d, 0x0aa4, 0x0e93, 0x1259];

/// 0x12cf (`0x38b458`), the count of the resort's breakables (`$gp−0x3c0`, [`Globals::left`]): at its start (update
/// distance 0xff) on Maktar it counts the placed 0x0a9c, 0x0d0d, 0x0aa4, 0x0e93 and 0x1259 under z 400; at the level's
/// frame 2 the count is set to 113 (the game's own figure); at 10 left or fewer, skill point 0xe (`0x1a7a76`,
/// `0x38b5e8`: crate::skill_points). Not ported: level 3's count (classes 0xf37, 0xf38, 0xf3e: 112, at 11 skill point
/// 0x15).
pub fn update_12cf(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            w.mobys[id].update_dist = 0xff;
            if ctx.game.level == 2 {
                let n = w.mobys.iter().filter(|m| m.state < 0x80 && COUNTED.contains(&m.o_class) && m.pos[2] < 400.0).count() as i32;
                w.maktar.left += n;
            }
            w.mobys[id].set_state(1, -1);
        }
        1 => {
            if ctx.game.level != 2 { return; }
            if ctx.game.frame == 2 { w.maktar.left = 0x71; }
            if 10 < w.maktar.left { return; }
            ctx.game.award_skill_point(crate::skill_points::point::RESORT);
        }
        _ => {}
    }
}

/// 0x1259 (`0x440d10`), a floating breakable of the resort (counted by 0x12cf unless `+0x68`[0x10]). Its pvars: home
/// `[0]`, its sway phases `[4]` / `[5]` and their steps `[6]` / `[7]` (`randf_signed(20, 40)` and `(40, 60)` degrees a
/// second), its flight velocity `[8]`, its spins `[0xc]` / `[0xd]`, `+0x3c` it flew into something, `+0x3e` it sinks
/// when stood on, its idle timer `+0x48`, its flight timer `+0x4a`, its cylinder `[0x14]`.
/// * 1: drawn it sways (its angles x and y 0.05 of its phases' sines) and circles its home 0.75 out (the second row of
///   those angles); stood on with `+0x3e` it sinks 1/15 a tick, into its cylinder: explodes (3). Hit with the flags
///   0x4210000 (counted: one off) it is knocked a quarter a tick along the hit's way for 30..60 ticks, spinning by where
///   it was hit; a line along that flight meeting the world: it explodes at once (3), else it flies (2).
/// * 2: its sound 0 first; it spins and moves; out of time, it explodes (3).
/// * 3: the explosion (crate::moby::blast::explode: size 10·its scale, 10 embers, 5 billows, 15 flares), its pieces
///   thrown on its way (`0x32c360`: 3..6 out and up), the level's bolts 3 times, its sound 1; gone.
///
/// Out of its start it changes its idle (`0x4412e0`) every 135..225 ticks: one time in five (`[0x11]` 0) to sequence
/// 0, else between 1 and 2. Not ported: its flight's sparks (`0x2f0f28`, one tick in two), its start's random frame.
pub fn update_1259(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use crate::hero::physics::{add, add_rot, dot, scale, set_len, sub};
    use crate::moby::attach::rows_of;
    match w.mobys[id].state {
        0 => {
            if w.mobys[id].pv_i(0x40) != 0 { w.maktar.left -= 1; }
            let g = &mut ctx.game.rng;
            let (a, b) = (g.rand_angle(), g.rand_angle());
            let (sa, sb) = (g.randf_signed(20.0, 40.0) * 0.000_290_888_25, g.randf_signed(40.0, 60.0) * 0.000_290_888_25);
            let seq = g.randi(2) as u8 + 1;
            let m = &mut w.mobys[id];
            let p = m.pos;
            m.set_pv_v(0, p);
            m.set_pv_f(0x10, a);
            m.set_pv_f(0x14, b);
            m.set_pv_f(0x18, sa);
            m.set_pv_f(0x1c, sb);
            m.set_pv_i(0x40, 0);
            m.set_pv_f(0x4c, 0.104_719_76);
            m.set_pv_s(0x48, 0xb4);
            if ctx.host.anim(&w.mobys[id]).is_none_or(|a| a.seq_b != seq) { ctx.host.set_anim(&w.mobys[id], seq, 0, 5); }
            w.mobys[id].set_state(1, -1);
            return;
        }
        1 => {
            let m = &mut w.mobys[id];
            if m.drawn != 0 {
                let (a, b) = (add_rot(m.pv_f(0x10), m.pv_f(0x18)), add_rot(m.pv_f(0x14), m.pv_f(0x1c)));
                m.set_pv_f(0x10, a);
                m.set_pv_f(0x14, b);
                m.rot[0] = 0.05 * a.sin();
                m.rot[1] = 0.05 * b.sin();
                m.pos = add(scale(rows_of([a, 0.0, b])[1], 0.75), m.pv_v(0));
            }
            if m.pv_s(0x3e) != 0 {
                if ctx.hero.ground_moby == Some(id) && ctx.hero.grounded_ticks != 0 { m.set_pv_s(0x3e, 2); }
                if m.pv_s(0x3e) == 2 {
                    let mut h = m.pv_v(0);
                    h[2] -= 0.066_666_67;
                    m.set_pv_v(0, h);
                    let cyl = m.pv_i(0x50);
                    if cyl != 0 && ctx.level.in_cylinder(cyl, m.pos) { m.set_state(3, -1); }
                }
            }
            if w.mobys[id].state == 1 && w.mobys[id].pv_i(0x40) == 0 {
                if let Some(h) = w.hit_message(id).filter(|h| h.tmpl.flags & 0x0421_0000 != 0) {
                    w.maktar.left -= 1;
                    let g = &mut ctx.game.rng;
                    let t = g.rand_range(0x1e, 0x3c);
                    let m = &mut w.mobys[id];
                    m.set_pv_s(0x4a, t as i16);
                    let v = set_len([h.tmpl.dir[0], h.tmpl.dir[1], h.tmpl.dir[2]], 0.25);
                    m.set_pv_v(0x20, v);
                    let d = sub([h.point[0], h.point[1], h.point[2]], m.pos);
                    let l = [dot(d, m.rows[0]), dot(d, m.rows[1]), dot(d, m.rows[2])];
                    m.set_pv_f(0x30, -(l[2].clamp(-1.0, 1.0) * 0.069_813_18));
                    m.set_pv_f(0x34, l[1].clamp(-1.0, 1.0) * 0.052_359_886);
                    let p = m.pos;
                    let to = add(p, scale(v, t as f32));
                    if ctx.coll.and_then(|c| crate::collision::coll_line_m(c, ctx.scene, p, to, crate::collision::QueryFlags(2), None)).is_some() {
                        w.mobys[id].set_pv_s(0x3c, 1);
                    }
                    let s = if w.mobys[id].pv_s(0x3c) != 0 { 3 } else { 2 };
                    w.mobys[id].set_state(s, -1);
                }
            }
            w.clear_hit(id);
        }
        2 => {
            if w.mobys[id].state_ticks == 0 { ctx.host.play_sound(&w.mobys[id], 0, 0); }
            ctx.game.rng.randi(2);
            let m = &mut w.mobys[id];
            m.state_ticks = m.state_ticks.wrapping_add(1);
            m.rot[1] = add_rot(m.rot[1], m.pv_f(0x30));
            m.rot[2] = add_rot(m.rot[2], m.pv_f(0x34));
            m.pos = add(m.pos, m.pv_v(0x20));
            let mut t = m.pv_s(0x4a) as i32;
            let out = crate::moby::countdown(&mut t);
            m.set_pv_s(0x4a, t as i16);
            if out { m.set_state(3, -1); }
        }
        3 => {
            if w.mobys[id].pv_s(0x3c) != 0 { w.mobys[id].set_pv_v(0x20, [0.0; 3]); }
            let size = w.mobys[id].scale * 10.0;
            let e = crate::moby::blast::Explosion { near: 9.0, size, embers: 10, billows: 5, flares: 0xf, sound: -1, lod: -1, flags: 0x81_0001, ..Default::default() };
            crate::moby::blast::explode(w, ctx, Some(id), None, &e);
            let v = w.mobys[id].pv_v(0x20);
            crate::moby::shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 6.0), 0x3fff, v, 0xff);
            crate::moby::bolts::level_drop(w, id, 0.0, 3, ctx);
            ctx.host.play_sound(&w.mobys[id], 1, 0);
            w.delete(id);
            return;
        }
        _ => {}
    }
    // `0x4412e0`: the idle's change.
    let m = &mut w.mobys[id];
    let mut t = m.pv_s(0x48) as i32;
    let due = crate::moby::countdown(&mut t);
    m.set_pv_s(0x48, t as i16);
    if due {
        let calm = m.pv_i(0x44) == 0 && 0.8 <= ctx.game.rng.randf(0.0, 1.0);
        let a = ctx.host.anim(&w.mobys[id]);
        if calm {
            if a.is_some_and(|a| a.seq_b != 0) { ctx.host.set_anim(&w.mobys[id], 0, 0, 0x1e); }
        } else {
            let seq = if a.is_some_and(|a| a.seq_a == 1) { 2 } else { 1 };
            if a.is_none_or(|a| a.seq_b != seq) { ctx.host.set_anim(&w.mobys[id], seq, 0, 0x1e); }
        }
        let t = ctx.game.rng.randf(135.0, 225.0) as i16;
        w.mobys[id].set_pv_s(0x48, t);
    }
    w.rebuild(id, ctx.host);
    w.changed = true;
}

/// `0x1b08f8`: the slot machine's odds in a thousand — nothing, bolts, it blows up, ammo, nanotech, the jackpot (past
/// 36 plays 40 move from the bolts to the jackpot, once).
const SLOT_ODDS: [i32; 6] = [260, 370, 120, 150, 105, 5];
/// `0x2a5470`: the symbols (1..5) round each reel.
const REELS: [[i32; 9]; 3] = [[1, 2, 3, 1, 2, 4, 1, 2, 5], [3, 2, 1, 4, 5, 3, 2, 1, 4], [1, 4, 3, 1, 2, 1, 2, 3, 5]];
/// `0x1b09c0`: each symbol's FX texture.
const SYMBOL_FX: [usize; 6] = [0, 0x42, 0x45, 0x43, 0x44, 0x41];
/// `0x2a53b0`: each reel's window in the machine's frame: its top edge (two corners), then its bottom edge.
const REEL_QUADS: [[[f32; 3]; 4]; 3] = [
    [[0.91, 0.1, 3.05], [0.91, 0.5, 3.05], [1.37, 0.1, 2.25], [1.37, 0.5, 2.25]],
    [[0.91, 0.55, 3.05], [0.91, 0.95, 3.05], [1.37, 0.55, 2.25], [1.37, 0.95, 2.25]],
    [[0.91, 1.0, 3.05], [0.91, 1.4, 3.05], [1.37, 1.0, 2.25], [1.37, 1.4, 2.25]],
];
/// A symbol's height on the reel's window (0..1) and its inverse.
const SYMBOL_H: f32 = 0.430_941_58;
const SYMBOL_ST: f32 = 2.320_500_1;

/// 0x0a9c (`0x3e82e0`), a slot machine. Its pvars: `+0x54` its spin's sound, `+0x58` the enemies' spawn list, `+0x5c`
/// the result, the reels' places `+0x60..0x68` (in symbols, 0..9), timers `+0x6c..0x74`, states `+0x78..0x80`
/// (1 spinning, 2 stopping, 0 stopped), speeds `+0x84..0x8c` and symbols to stop on `+0x90..0x98`; `+0xa4` it blew up.
/// * 0: its speeds `randf(6, 12)` a second, mirrored one time in two (mode 0x8000), hittable (0x4000); then 1.
/// * 1: a wrench hit with 10 bolts or more takes 10 and spins it (2, [`slot_spin`]).
/// * 2: its sequence 1 (a play counted: past 36, once, the jackpot's odds raised); from its frame 10 its looped
///   sound 1; at its end sequence 0 and the reels run (3, [`slot_reels`]); then the payout (4, [`slot_pay`]).
/// * 5: blown up or broken: its sound 0, its debris ([`slot_debris`]); blown up, a blast 1.5 up (a sphere of 4 doing
///   2 damage, flags 0x810001; the burst of size 1.5) — then it is deleted.
///
/// A hit with the flags 0x10000 for more than 2 from anything but the wrench (and not Ratchet) breaks it (5, counted).
/// Blown up, two creatures of its group `+0x58` waiting (state 13) drop out at (0.5, ∓0.45, 0) in its frame facing
/// Ratchet (crate::moby::classes::critter::waiting / wake). Its reels draw while it is drawn ([`draw_0a9c`]). Not
/// ported: the HUD's bolt count shown for 240 ticks (`0x3119f0`), the skill point's reward flag `0x1a903c`
/// (crate::skill_points).
pub fn update_0a9c(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].drawn != 0 { w.register_draw(id); }
    if w.mobys[id].pvars.len() < 0xa8 { w.mobys[id].pvars.resize(0xa8, 0); }
    match w.mobys[id].state {
        0 => {
            w.mobys[id].set_state(1, -1);
            let m = &mut w.mobys[id];
            m.set_pv_f(0x20, 3.0);
            m.set_pv_s(0x24, 3);
            m.set_pv_f(0x30, 1.0);
            for o in [0x60, 0xa4, 0x68, 0x64] { m.set_pv_i(o, 0); }
            // The game finds its sound by the handle's owner (`0x1886e8`); the port's handles have none, so none here.
            m.set_pv_i(0x54, -1);
            for k in 0..3 {
                let v = ctx.game.rng.randf(6.0, 12.0) * 0.016_666_668;
                w.mobys[id].set_pv_f(0x84 + 4 * k, v);
            }
            let flip = ctx.game.rng.randi(2) != 0;
            let m = &mut w.mobys[id];
            if flip { m.mode |= mode::MIRROR } else { m.mode &= !mode::MIRROR }
            m.mode |= mode::LINE;
        }
        2 => {
            if w.mobys[id].first() {
                if ctx.host.anim(&w.mobys[id]).is_some_and(|a| a.seq_b != 1) { ctx.host.set_anim(&w.mobys[id], 1, 0, 1); }
                let g = &mut w.maktar;
                g.plays = (g.plays + 1).min(0x7f);
                if 0x24 < g.plays && !g.odds_raised { g.odds_raised = true; }
            }
            if let Some(a) = ctx.host.anim(&w.mobys[id]).filter(|a| a.seq_b == 1) {
                if a.flags & 2 == 0 {
                    if crate::moby::ai::reached(w, id, ctx, 10.0) && !ctx.host.sound_playing(w.mobys[id].pv_i(0x54)) {
                        let h = ctx.host.play_sound(&w.mobys[id], 1, 4);
                        w.mobys[id].set_pv_i(0x54, h);
                    }
                } else {
                    ctx.host.set_anim(&w.mobys[id], 0, 0, 1);
                    w.mobys[id].set_state(3, -1);
                }
            }
        }
        3 => slot_reels(w, id),
        4 => {
            if w.mobys[id].first() {
                w.mobys[id].set_pv_i(0x50, 0x3c);
                let h = w.mobys[id].pv_i(0x54);
                if ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
                w.mobys[id].set_pv_i(0x54, -1);
            }
            slot_pay(w, id, ctx);
        }
        5 => {
            let h = w.mobys[id].pv_i(0x54);
            if ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
            w.mobys[id].set_pv_i(0x54, -1);
            ctx.host.set_anim(&w.mobys[id], 0, 0, 0);
            ctx.host.play_sound(&w.mobys[id], 0, 0);
            slot_debris(w, id, ctx);
            let m = &mut w.mobys[id];
            m.mode = m.mode & !mode::LINE | mode::NO_UPDATE;
            if m.pv_i(0xa4) != 0 {
                use crate::moby::hits::HitTemplate;
                let at = add_v(through(&m.rows, [0.0, 0.0, 1.5]), m.pos);
                let mut t = HitTemplate::new(2.0, Some(id), 0x81_0001, [0.0; 4]);
                t.kind = (m.o_class as u16 as u32) << 16 | 0x102;
                w.sphere_hit(ctx.scene, at, 4.0, 0x10, Some(id), Some(&t));
                let pos = w.mobys[id].pos;
                crate::moby::blast::burst(w, ctx, pos, &crate::moby::blast::Blast { size: 1.5, bursts: [0x19, 0x14, 0x14], puffs: 0x14, streaks: 0x2d, light: false, shake: true });
                // Two of its group `+0x58`'s waiting creatures woken at (0.5, ∓0.45, 0) in its frame, facing Ratchet.
                let group = w.mobys[id].pv_i(0x58);
                if group != 0 {
                    for i in 0..2 {
                        let Some(c) = crate::moby::classes::critter::waiting(w, group) else { continue };
                        let m = &w.mobys[id];
                        let at = add_v(through(&m.rows, [0.5, i as f32 * 0.9 - 0.45, 0.0]), m.pos);
                        crate::moby::classes::critter::wake(w, c, ctx, at, [0.0; 3]);
                        let hero = ctx.hero.pos;
                        let c = &mut w.mobys[c];
                        c.rot[2] = crate::hero::physics::atan2(hero[0] - c.pos[0], hero[1] - c.pos[1]);
                    }
                }
            }
            w.delete(id);
            return;
        }
        _ => {}
    }
    // The hit (`0x322040(m, 0x10000, 0)`).
    if let Some(h) = w.hit_message(id).filter(|h| h.tmpl.flags & 0x10000 != 0) {
        let class = h.tmpl.source.and_then(|s| w.mobys.get(s)).map(|s| s.o_class);
        if class.is_some_and(|c| c != 0 && c != 0x47) {
            if 2.0 < h.tmpl.damage {
                w.maktar.left -= 1;
                w.mobys[id].set_state(5, -1);
            }
        } else if class == Some(0x47) && w.mobys[id].state == 1 && 9 < ctx.game.bolts {
            ctx.game.bolts -= 10;
            slot_spin(w, id, ctx);
            w.mobys[id].set_state(2, -1);
        }
    }
    w.mobys[id].hit_slot = -1;
}

fn add_v(a: [f32; 3], b: [f32; 3]) -> [f32; 3] { [a[0] + b[0], a[1] + b[1], a[2] + b[2]] }

/// `0x3e8f68`: the spin's result by the odds (`randi(1000)`; the nanotech with Ratchet's health full and the jackpot
/// in the first 4 plays become bolts), each reel to stop on its symbol; nothing: three random symbols not all one.
fn slot_spin(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let mut odds = SLOT_ODDS;
    if w.maktar.odds_raised { odds[1] -= 40; odds[5] += 40; }
    let mut r = ctx.game.rng.randi(1000);
    let mut out = 0;
    for (k, o) in odds.iter().enumerate() {
        r -= o;
        if r < 0 { out = k as i32; break; }
    }
    if out == 4 && ctx.hero.hp == ctx.game.max_health { out = 1; }
    if w.maktar.plays < 4 && out == 5 { out = 1; }
    let m = &mut w.mobys[id];
    m.set_pv_i(0x5c, out);
    for k in 0..3 { m.set_pv_i(0x90 + 4 * k, out); }
    if out == 0 {
        let rng = &mut ctx.game.rng;
        let (a, b) = (rng.rand_range(1, 5), rng.rand_range(1, 5));
        let mut c = rng.rand_range(1, 5);
        while a == b && c == a { c = rng.rand_range(1, 5); }
        let m = &mut w.mobys[id];
        m.set_pv_i(0x90, a);
        m.set_pv_i(0x94, b);
        m.set_pv_i(0x98, c);
    }
}

/// `0x3e8b18`, state 3: at its start the reels spin for 50, 100 and 180 ticks (mirrored, 180, 100 and 50); each runs
/// back by its speed (wrapping at 0 by 9), then stops once its symbol is in the middle less than 0.2 past; all three
/// stopped, the payout (4).
fn slot_reels(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    if m.first() {
        let mirrored = m.mode & mode::MIRROR != 0;
        m.set_pv_i(0x6c, if mirrored { 0xb4 } else { 0x32 });
        m.set_pv_i(0x70, 100);
        m.set_pv_i(0x74, if mirrored { 0x32 } else { 0xb4 });
        for k in 0..3 { m.set_pv_i(0x78 + 4 * k, 1); }
    }
    for k in 0..3 {
        let st = m.pv_i(0x78 + 4 * k);
        if st == 1 || st == 2 {
            let mut p = m.pv_f(0x60 + 4 * k) - m.pv_f(0x84 + 4 * k);
            if p < 0.0 { p += 9.0; }
            m.set_pv_f(0x60 + 4 * k, p);
            if st == 1 {
                let mut t = m.pv_i(0x6c + 4 * k);
                if crate::moby::countdown(&mut t) { m.set_pv_i(0x78 + 4 * k, 2); }
                m.set_pv_i(0x6c + 4 * k, t);
            } else if REELS[k][(p + 9.0) as i32 as usize % 9] == m.pv_i(0x90 + 4 * k) && p - (p as i32 as f32) < 0.2 {
                m.set_pv_i(0x78 + 4 * k, 0);
            }
        }
        if (0..3).all(|j| m.pv_i(0x78 + 4 * j) == 0) { m.set_state(4, -1); }
    }
}

/// `0x3e8d50`, state 4: back to 1 and the payout at (3, 0, 2) in its frame plus a random way across (`1 − r²` long, r in 0..1): bolts (15..65,
/// `0x326ee8`), it blows up (5, counted), an ammo pickup thrown at 0.35 of its speed (`0x31ead8` / `0x3ed9a8`), a
/// nanotech cluster (`0x3a9bf0`) or the jackpot (300 bolts; skill point 0x19, crate::skill_points); each but the blow-up
/// with its sound 2, nothing with its sound 3.
fn slot_pay(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].set_state(1, -1);
    // `0x31fce0(1)`: a heading, then `0x31fc10(1)` its length (`1 − r²`, r 12 bits).
    let a = ctx.game.rng.rand_angle();
    let r12 = ((ctx.game.rng.rand() >> 16) & 0xfff) as f32 * 0.000_244_140_62;
    let r = 1.0 - r12 * r12;
    let m = &w.mobys[id];
    let at = add_v(add_v(through(&m.rows, [3.0, 0.0, 2.0]), m.pos), [a.cos() * r, a.sin() * r, 0.0]);
    let out = m.pv_i(0x5c);
    if out != 2 { ctx.host.play_sound(&w.mobys[id], if (1..=5).contains(&out) { 2 } else { 3 }, 0x20); }
    match out {
        1 => crate::moby::bolts::spawn(w, id, 0.0, 0xf, 0x41, 3, 4, ctx),
        2 => {
            w.mobys[id].set_pv_i(0xa4, 1);
            w.maktar.left -= 1;
            w.mobys[id].set_state(5, -1);
        }
        3 => {
            if let (Some(item), _) = crate::moby::classes::pickup::pick(ctx) {
                if let Some(p) = crate::moby::classes::pickup::spawn(w, ctx, at, item, None) {
                    let v = w.mobys[p].pv_v(crate::moby::shards::VEL);
                    w.mobys[p].set_pv_v(crate::moby::shards::VEL, v.map(|x| x * 0.35));
                }
            }
        }
        4 => { crate::moby::classes::nanotech::create_at(w, at, 0); }
        5 => {
            // Skill point 0x19 (`0x3e8f04`).
            ctx.game.award_skill_point(crate::skill_points::point::JACKPOT);
            ctx.game.bolts += 300;
        }
        _ => {}
    }
}

/// `0x3e88e0`: the debris classes 0xca5..0xca9 thrown whole (crate::moby::shards::spawn_whole: at `randf(1, 4)`
/// forward, `randf_signed(1, 3)` across and `randf(4, 9)` up a second through its rows, spinning 90..180°/s, gravity
/// 15..25), then while the machines' 300 bolts last five 5-bolts 1.5 up its frame (`0x38abb0`), thrown the same way.
fn slot_debris(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let throw = |w: &MobyWorld, ctx: &mut Ctx| {
        let rng = &mut ctx.game.rng;
        let v = [rng.randf(1.0, 4.0) * 0.016_666_668, rng.randf_signed(1.0, 3.0) * 0.016_666_668, rng.randf(4.0, 9.0) * 0.016_666_668];
        through(&w.mobys[id].rows, v)
    };
    for class in 0x0ca5..0x0caa {
        let vel = throw(w, ctx);
        let spin = ctx.game.rng.randf(90.0, 180.0);
        let gravity = ctx.game.rng.randf(15.0, 25.0);
        let (pos, rot) = (w.mobys[id].pos, w.mobys[id].rot);
        crate::moby::shards::spawn_whole(w, ctx, class, Some(id), pos, rot, spin, gravity, vel, 0xff);
    }
    let at = add_v(through(&w.mobys[id].rows, [0.0, 0.0, 1.5]), w.mobys[id].pos);
    for _ in 0..5 {
        if 300 - w.maktar.coins_out < 1 { return; }
        let vel = throw(w, ctx);
        crate::moby::bolts::create_bolt(w, id, at, vel, 0, 5, ctx);
        w.maktar.coins_out += 5;
    }
}

/// `0x3e90c8`, the reels' draw: on each reel's window ([`REEL_QUADS`]) the three symbols about its place `+0x60..`
/// (each [`SYMBOL_H`] of the window, cut to 0.1..0.9 with its texture), the symbol's FX texture ([`SYMBOL_FX`]),
/// flipped across when mirrored, coloured 0x7f7f7f7f. Its ALPHA (0x8000000000) writes the texels as they are; here
/// 0x44 at the colour's alpha.
pub fn draw_0a9c(w: &MobyWorld, id: usize, out: &mut Vec<crate::fx::FxQuad>) {
    let m = &w.mobys[id];
    let (u0, u1) = if m.mode & mode::MIRROR != 0 { (1.0, 0.0) } else { (0.0, 1.0) };
    let lerp = |a: [f32; 3], b: [f32; 3], t: f32| -> [f32; 3] { std::array::from_fn(|j| a[j] + (b[j] - a[j]) * t) };
    let world = |v: [f32; 3]| add_v(through(&m.rows, v), m.pos);
    for (k, q) in REEL_QUADS.iter().enumerate() {
        let f = m.pv_f(0x60 + 4 * k) + 0.15;
        for i in 0..3 {
            let mut t0 = ((f as i32) as f32 - f) * SYMBOL_H + i as f32 * SYMBOL_H;
            let mut t1 = t0 + SYMBOL_H;
            let sym = REELS[k][((f + i as f32) - 1.0 + 9.0) as i32 as usize % 9];
            if t1 < 0.1 || 0.9 < t0 { continue; }
            let (mut v0, mut v1) = (0.0, 1.0);
            if t0 < 0.1 {
                v0 = (0.1 - t0) * SYMBOL_ST;
                t0 = 0.1;
            } else if 0.9 < t1 {
                v1 = (0.9 - t1) * SYMBOL_ST + 1.0;
                t1 = 0.9;
            }
            let corners = [world(lerp(q[0], q[2], t0)), world(lerp(q[1], q[3], t0)), world(lerp(q[0], q[2], t1)), world(lerp(q[1], q[3], t1))];
            out.push(crate::fx::FxQuad { fx: SYMBOL_FX[sym as usize], additive: false, corners, st: [[u0, v0], [u1, v0], [u0, v1], [u1, v1]], rgba: [0x7f7f_7f7f; 4] });
        }
    }
}

/// The level's mission `mission` done (`0x19b2e8 + level·16`, bit 7).
fn mission_done(g: &crate::game::GameState, level: i32, mission: i32) -> bool {
    usize::try_from(mission).ok().and_then(|k| g.missions.get(level.max(0) as usize * 0x10 + k)).is_some_and(|b| b & 0x80 != 0)
}

/// The flag byte `0x1395b8 + k` (crate::game::GameState::flags).
fn flag(g: &crate::game::GameState, k: usize) -> u8 { g.flags.get(k).copied().unwrap_or(0) }
fn set_flag(g: &mut crate::game::GameState, k: usize, v: u8) {
    if let Some(b) = g.flags.get_mut(k) { *b = v; }
}

/// `0x2bd368(pos, rot, state, 1)`: Ratchet put at `pos` facing `rot`, in `state`.
fn place_hero(ctx: &mut Ctx, pos: [f32; 3], rot: [f32; 3], state: i32) {
    ctx.hero.teleport(pos, rot);
    ctx.hero.request_state(state, true);
}

/// The local point of `p` in moby `id`'s frame (`0x32a6a0(m, out, v, 0)`: through its rows' transpose).
fn local_of(w: &MobyWorld, id: usize, p: [f32; 3]) -> [f32; 3] {
    let m = &w.mobys[id];
    let d = [p[0] - m.pos[0], p[1] - m.pos[1], p[2] - m.pos[2]];
    std::array::from_fn(|k| d[0] * m.rows[k][0] + d[1] * m.rows[k][1] + d[2] * m.rows[k][2])
}

/// 0x0c51 (`0x4112c8`): the scenes that give Endako. Until the planet 3 is unlocked (`0x1a7bd3`) and while its flag
/// `0x1395bc` is clear, Ratchet in its cuboid `+0x0c` (`0x325c50`) queues the scene 0x2d (refused, it tries again),
/// the movie 3 and the scene 0x2e (`0x2f8e70`), starts them (`0x2f8fb8`) and sets the flag (2); once the scenes are
/// over (`0x379338`) the planet 3 is unlocked (`0x326b28`) with its banner (`0x3296d0(3)`: the message box), 3.
pub fn update_0c51(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => w.mobys[id].set_state(1, -1),
        1 => {
            let g = &mut *ctx.game;
            if g.planets.get(3).is_some_and(|&b| b != 0) || flag(g, 4) != 0 { return; }
            if !ctx.level.in_cuboid(w.mobys[id].pv_i(0xc), ctx.hero.pos) { return; }
            if !g.scenes.queue.push(0x2d) { return; }
            g.scenes.queue.push(3);
            g.scenes.queue.push(0x2e);
            g.scenes.start_queue(ctx.hero.hp);
            set_flag(g, 4, 1);
            w.mobys[id].set_state(2, -1);
        }
        2 => {
            if ctx.game.scenes.in_scene() { return; }
            ctx.game.unlock_planet(3);
            crate::travel::new_planet_banner(ctx.game, 3);
            w.mobys[id].set_state(3, -1);
        }
        _ => {}
    }
}

/// 0x0bf6 (`0x405b58`; the same class on the Jamming Array, level 0x1a): the ship to the Jamming Array and back. At
/// its start it plays its sequence 2, keeps its place (`+0x20`, `+0x30`) and whether its mission is done (`+0x1e`;
/// done, it hooks the moby `+4`); on Maktar, back from the Jamming Array (the flag `0x139629` = 0x1a, set to 1),
/// Ratchet is put at the cuboid `+0x14` with the placed view `+0x10` reset (`0x328648`) and made the checkpoint
/// (`0x360b18`), with Barlow (planet 4) unlocked its banner. Ratchet grounded on it and more than 0.6 under it in its
/// frame gets the prompt (owner 8: on the Jamming Array 0x1303, else 0x131f with the mission done or 0x1302); △ makes
/// the checkpoint and leaves for the other level (`0x2f9080(level, 1)`; to the Jamming Array the flag `0x1395b9`
/// set). On the Jamming Array, its mission done and Barlow not yet unlocked, it queues the scene 0xa7, the movie 4
/// and the scene 0xa8, starts them, clears the no-item area (`0x18c310`), unlocks Barlow (`0x326b28(4)`) and stops
/// (3).
pub fn update_0bf6(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let level = ctx.game.level;
    match w.mobys[id].state {
        0 => {
            super::props::set_seq(w, id, ctx, 2, 0, 1);
            let done = mission_done(ctx.game, level, w.mobys[id].mission as i32);
            let m = &mut w.mobys[id];
            m.set_state(1, -1);
            m.alpha = 0xff;
            if level == 0x1a { m.draw_dist = 0xff; }
            let (p, r) = (m.pos, m.rot);
            m.set_pv_v(0x20, p);
            m.set_pv_v(0x30, r);
            m.set_pv_s(0x1e, done as i16);
            let hook = m.pv_i(4);
            if done && hook != 0 {
                if let Some(h) = usize::try_from(hook).ok().and_then(|h| w.mobys.get_mut(h)) { h.once |= 4; }
            }
            if level == 2 && flag(ctx.game, 0x71) == 0x1a {
                set_flag(ctx.game, 0x71, 1);
                let (at, view) = (w.mobys[id].pv_i(0x14), w.mobys[id].pv_i(0x10));
                if let Some((p, r)) = ctx.level.cuboid_place(at) {
                    place_hero(ctx, p, r, 0);
                    if view != -1 { ctx.game.cameras.reset_placed(view); }
                }
                let (p, r) = (ctx.hero.pos, ctx.hero.rot);
                ctx.game.save_checkpoint(p, r);
                // Barlow unlocked (`0x1a7bd4`) and the flag `0x13965d` not set: the banner, once.
                if ctx.game.planets.get(4).is_some_and(|&b| b != 0) && flag(ctx.game, 0xa5) == 0 {
                    set_flag(ctx.game, 0xa5, 1);
                    crate::travel::new_planet_banner(ctx.game, 4);
                }
            }
        }
        1 => {}
        _ => return,
    }
    if ctx.hero.ground_moby == Some(id) && ctx.hero.grounded_ticks != 0 && local_of(w, id, ctx.hero.pos)[2] < -0.6 {
        let (text, to) = if level == 0x1a {
            (0x1303, 2)
        } else if mission_done(ctx.game, level, w.mobys[id].mission as i32) {
            (0x131f, 0x1a)
        } else {
            (0x1302, 0x1a)
        };
        ctx.game.prompt.request(8, text, 1);
        if ctx.hero.items.pressed & crate::pad::button::TRIANGLE != 0 {
            let (p, r) = (ctx.hero.pos, ctx.hero.rot);
            ctx.game.save_checkpoint(p, r);
            ctx.game.go_to_level(to, true);
            if to == 0x1a { set_flag(ctx.game, 1, 1); }
        }
    }
    let g = &mut *ctx.game;
    if level == 0x1a && mission_done(g, 0x1a, w.mobys[id].mission as i32) && g.planets.get(4).is_some_and(|&b| b == 0) && g.scenes.queue.push(0xa7) {
        g.scenes.queue.push(4);
        g.scenes.queue.push(0xa8);
        g.scenes.start_queue(ctx.hero.hp);
        g.no_item_area = false;
        g.unlock_planet(4);
        w.mobys[id].set_state(3, -1);
    }
}

/// 0x0c8f's pvars: the ends' cuboids `+0x7c` / `+0x80`, Ratchet's places at them `+0x60` / `+0x64` and their placed
/// views `+0x68` / `+0x6c` (by the end `+0xbc`), the mission `+0x78`, the hum `+0x84`, its base place `+0x90` /
/// `+0xa0`, the bob's phases `+0xb0..+0xb8`, the speed `+0xcc`.
const CAR_END: usize = 0xbc;
const CAR_BASE: usize = 0x90;
const CAR_BASE_ROT: usize = 0xa0;
const CAR_SPEED: usize = 0xcc;
const CAR_HUM: usize = 0x84;
/// `$gp+0x2ca0` / `+0x2ca4`: the top speed (a second) and the gain (a second²).
const CAR_TOP: f32 = 5.0;
const CAR_GAIN: f32 = 7.0;
/// `$gp+0x2cb0`: the bumpers 2 to each side (y mirrored) of this point in its frame.
const CAR_BUMPER: [f32; 3] = [-3.0, 2.0, 0.5];
/// Where Ratchet walks to on boarding, in its frame.
const CAR_SEAT: [f32; 3] = [-1.2, 0.0, -0.69];
/// `$gp+0x2cc4..+0x2cd8`: the bob — the roll 0.01 at 61°/s, the pitch 0.05 at 83°/s, the height 0.25 at 100°/s.
const CAR_BOB: [(f32, f32); 3] = [(61.0, 0.01), (83.0, 0.05), (100.0, 0.25)];
const DEG_TICK: f32 = 0.000_290_888_25;

/// 0x0c8f's end `e` (0 / 1): its cuboid's place for the base (`+0x7c + 4e`).
fn car_end(w: &mut MobyWorld, id: usize, ctx: &Ctx, e: i32) {
    if let Some((p, r)) = ctx.level.cuboid_place(w.mobys[id].pv_i(0x7c + 4 * e as usize)) {
        let m = &mut w.mobys[id];
        m.set_pv_v(CAR_BASE, p);
        m.set_pv_v(CAR_BASE_ROT, r);
    }
}

/// `0x414e78`: at its base, bobbing ([`CAR_BOB`]: each phase on by its rate, the sine's share added).
fn car_bob(w: &mut MobyWorld, id: usize) {
    use crate::hero::physics::add_rot;
    let m = &mut w.mobys[id];
    m.pos = m.pv_v(CAR_BASE);
    m.rot = m.pv_v(CAR_BASE_ROT);
    let b = add_rot(m.pv_f(0xb8), CAR_BOB[0].0 * DEG_TICK);
    m.set_pv_f(0xb8, b);
    m.rot[0] += CAR_BOB[0].1 * b.sin();
    let b = add_rot(m.pv_f(0xb4), CAR_BOB[1].0 * DEG_TICK);
    m.set_pv_f(0xb4, b);
    m.rot[1] += CAR_BOB[1].1 * b.sin();
    let b = add_rot(m.pv_f(0xb0), CAR_BOB[2].0 * DEG_TICK);
    m.set_pv_f(0xb0, b);
    m.pos[2] += CAR_BOB[2].1 * b.sin();
}

/// 0x0c8f (`0x414498`, two): the hover car between two ends (the cuboids `+0x7c` / `+0x80`). With a mission `+0x78`
/// it is gone once the item 0x26 is had (`0x1a7b26`), and waits hidden (4) for the mission, then shows when out of
/// view (`0x2f4008(64, its sphere)`). Waiting (1), every 64th frame it moves to the end nearer Ratchet (across); with
/// Ratchet grounded on it, more than 0.6 under it in its frame, he stands still (0x6f) and it goes (2): the script
/// camera at the view (snapping), he walks to its seat ([`CAR_SEAT`]), its sound 1 and its sequence 0; it speeds
/// along its first row ([`CAR_TOP`] / [`CAR_GAIN`], flat), its bumpers hitting what is in the way, the camera rising
/// a sixtieth a tick turned toward him (0.75° / 1.5° a tick); from tick 45 he stands still again, its hum loops, and
/// from tick 121 the screen fades to black (1/30 a tick). Black: he is put at the other end's place (`+0x60`, 0.9 back
/// along its first row) and walks to its middle, the camera behind him with the end's placed view reset (`+0x68`),
/// the hum stopped, sequence 2; the fade back (3), then 1. Drawn, it bobs ([`car_bob`]); it carries Ratchet
/// (`0x326a60`).
pub fn update_0c8f(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use crate::hero::physics::{add, approach, approach_angle, atan2, dist2, set_len};
    use crate::script_camera::{MODE_SNAP, RELEASE_CUT_BEHIND};
    let (old, old_yaw) = (w.mobys[id].pos, w.mobys[id].rot[2]);
    let gate = w.mobys[id].pv_i(0x78);
    let had = ctx.game.items.get(0x26).is_some_and(|&b| b != 0);
    let solid = w.class(w.mobys[id].o_class).is_some_and(|c| c.has_collision);
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.alpha = 0xff;
            m.set_pv_i(CAR_HUM, -1);
            if m.pv_i(0x7c) == -1 || m.pv_i(0x80) == -1 {
                w.delete(id);
                return;
            }
            if gate == -1 {
                m.set_state(1, -1);
            } else {
                if had {
                    w.delete(id);
                    return;
                }
                m.has_collision = false;
                m.mode = (m.mode & !mode::TARGET) | 0x41;
                m.set_state(4, -1);
                w.changed = true;
            }
            super::props::set_seq(w, id, ctx, 2, 0, 1);
            car_end(w, id, ctx, 0);
        }
        1 => {
            if gate != -1 && had {
                w.delete(id);
                return;
            }
            if ctx.hero.ground_moby == Some(id) && ctx.hero.grounded_ticks != 0 {
                if local_of(w, id, ctx.hero.pos)[2] < -0.6 {
                    ctx.hero.request_state(0x6f, true);
                    w.mobys[id].set_state(2, -1);
                }
            } else if ctx.game.frame & 0x3f == 0 {
                let cur = w.mobys[id].pv_i(CAR_END);
                let other = (cur + 1) & 1;
                let at = |e: i32| ctx.level.cuboid_place(w.mobys[id].pv_i(0x7c + 4 * e as usize)).map(|(p, _)| dist2(ctx.hero.pos, p));
                if let (Some(a), Some(b)) = (at(other), at(cur)) {
                    if a < b {
                        w.mobys[id].set_pv_i(CAR_END, other);
                        car_end(w, id, ctx, other);
                        let m = &mut w.mobys[id];
                        m.pos = m.pv_v(CAR_BASE);
                        m.rot = m.pv_v(CAR_BASE_ROT);
                    }
                }
            }
        }
        2 => 'ride: {
            if ctx.hero.grounded_ticks == 0 {
                ctx.game.cameras.camera_script2(RELEASE_CUT_BEHIND, ctx.hero);
                break 'ride;
            }
            if w.mobys[id].first() {
                let (vp, ve) = ctx.game.cameras.view_euler();
                ctx.game.cameras.camera_script(vp, ve, MODE_SNAP, 0, false, ctx.hero);
                let m = &w.mobys[id];
                let seat = add(through(&m.rows, CAR_SEAT), m.pos);
                ctx.hero.script_walk_to(m.rot[2], seat, false);
                ctx.host.play_sound(&w.mobys[id], 1, 0);
                if super::props::seq(w, id, ctx) != 0 { ctx.host.set_anim(&w.mobys[id], 0, 0, 0x2d); }
            }
            let m = &mut w.mobys[id];
            let mut v = m.pv_f(CAR_SPEED);
            approach(CAR_TOP * 0.016_666_668, CAR_GAIN * 0.000_277_777_78, &mut v);
            m.set_pv_f(CAR_SPEED, v);
            let mut step = set_len(m.rows[0], v);
            step[2] = 0.0;
            m.set_pv_v(CAR_BASE, add(m.pv_v(CAR_BASE), step));
            for side in [1.0, -1.0] {
                let m = &w.mobys[id];
                let at = add(through(&m.rows, [CAR_BUMPER[0], CAR_BUMPER[1] * side, CAR_BUMPER[2]]), m.pos);
                crate::moby::ai::attack_sphere(w, id, ctx, 1.0, 1.0, 1.0, at, 0x10000, 0, 1, 0);
            }
            // The camera: where the view is, a sixtieth higher, turned toward Ratchet.
            let (vp, mut e) = ctx.game.cameras.view_euler();
            let eye = [vp[0], vp[1], vp[2] + 0.016_666_668];
            let h = ctx.hero.pos;
            let up = atan2(dist2(eye, h), h[2] - eye[2]);
            approach_angle(-up, f32::from_bits(0x3c56_7752), &mut e[1], 0);
            approach_angle(atan2(h[0] - eye[0], h[1] - eye[1]), f32::from_bits(0x3cd6_7752), &mut e[2], 0);
            ctx.game.cameras.camera_script_targets(Some(eye), Some(e));
            let m = &mut w.mobys[id];
            let t = m.state_ticks;
            m.state_ticks = t.wrapping_add(1);
            if (t as u16) < 0x79 {
                if m.state_ticks == 0x2d { ctx.hero.request_state(0x6f, true); }
                let hum = w.mobys[id].pv_i(CAR_HUM);
                if !ctx.host.sound_playing(hum) {
                    let hum = ctx.host.play_sound(&w.mobys[id], 0, 4);
                    w.mobys[id].set_pv_i(CAR_HUM, hum);
                }
            } else {
                approach(1.0, f32::from_bits(0x3d08_8889), &mut ctx.game.fade_black);
            }
            if ctx.game.fade_black < 1.0 { break 'ride; }
            // Black: off at the other end.
            ctx.hero.request_state(0, true);
            let hum = w.mobys[id].pv_i(CAR_HUM);
            if hum != -1 { ctx.host.stop_sound(hum); }
            w.mobys[id].set_pv_i(CAR_HUM, -1);
            ctx.host.set_anim(&w.mobys[id], 2, 0, 0);
            w.mobys[id].set_state(3, -1);
            let e = (w.mobys[id].pv_i(CAR_END) + 1) & 1;
            w.mobys[id].set_pv_i(CAR_END, e);
            car_end(w, id, ctx, e);
            let k = 4 * e as usize;
            let c = usize::try_from(w.mobys[id].pv_i(0x60 + k)).ok().and_then(|i| ctx.level.cuboids.get(i)).copied();
            if let Some(c) = c {
                let mid = [c.matrix[3][0], c.matrix[3][1], c.matrix[3][2]];
                let at = add(mid, [c.matrix[0][0] * -0.9, c.matrix[0][1] * -0.9, c.matrix[0][2] * -0.9]);
                let rot = [c.extra[0], c.extra[1], c.extra[2]];
                place_hero(ctx, at, rot, 0);
                ctx.hero.script_walk_to(c.extra[2], mid, true);
            }
            ctx.game.cameras.camera_script2(RELEASE_CUT_BEHIND, ctx.hero);
            ctx.game.cameras.reset_placed(w.mobys[id].pv_i(0x68 + k));
            car_bob(w, id);
        }
        3 => {
            approach(0.0, f32::from_bits(0x3d08_8889), &mut ctx.game.fade_black);
            if ctx.game.fade_black <= 0.0 {
                let m = &mut w.mobys[id];
                m.set_pv_f(CAR_SPEED, 0.0);
                m.set_state(1, -1);
            }
        }
        4 => {
            if mission_done(ctx.game, ctx.game.level, gate) {
                let b = w.mobys[id].bsphere;
                let s = [b[0] / 1024.0, b[1] / 1024.0, b[2] / 1024.0, b[3]];
                if ctx.view.is_none_or(|v| v.culled(64.0, s)) {
                    let m = &mut w.mobys[id];
                    m.mode = (m.mode & !0x41) | mode::TARGET;
                    m.has_collision = solid;
                    m.set_state(1, -1);
                    w.changed = true;
                }
            }
        }
        _ => {}
    }
    if w.mobys[id].drawn != 0 { car_bob(w, id); }
    let m = &w.mobys[id];
    carry_hero(ctx, id, old, old_yaw, m.pos, m.rot[2]);
}

/// 0x0a57 (`0x3dd4a8`): the footsteps' dust. At its start (`0x1a7b02` clear: `0x18c31c` = 1, which the port's
/// physics derives from Clank) 1; then each tick [`footstep_dust`]. Not ported: Ratchet in its cuboid `+0` sets the
/// hero word `0x18a03a` = 5 (`0x325c50`; no reader found).
pub fn update_0a57(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.alpha = 0xff;
            m.set_state(1, -1);
        }
        1 => footstep_dust(w, ctx),
        _ => {}
    }
}

/// `0x3dd540`: on surface 10 (`0x18b2b0`) and grounded, a foot down — at the walk's start (state 2, its timer 1:
/// either foot, `randi(2)`), or with no blend running (`0x18aa4c`) the walk's (3) key times 49.5 / 17.0 or the run's
/// (4) 12.5 / 1.0 passed this tick (`0x2b6fa0`; the right foot when its time passed, else the left) — leaves a dust
/// puff (crate::particles::dust: `randf(1, 1.5)` big, growing 5250 a tick) at the foot's joint (0x16 + foot,
/// `0x3168e0`) 0.15 ahead along his yaw, 0.04 above the ground under it (`0x321020`).
fn footstep_dust(w: &MobyWorld, ctx: &mut Ctx) {
    let h = &*ctx.hero;
    if h.surface != 10 || h.grounded_ticks == 0 { return; }
    let v = ctx.hero_view;
    let passed = |f: f32| f < v.frame && v.frame - f <= v.frame_step;
    let foot = if h.timer == 1 && h.state == 2 {
        ctx.game.rng.randi(2)
    } else {
        let (l, r) = match v.seq_b {
            3 if !v.blending() => (49.5, 17.0),
            4 if !v.blending() => (12.5, 1.0),
            _ => return,
        };
        let left = passed(l);
        if passed(r) { 1 } else if left { 0 } else { return }
    };
    let Some(hm) = ctx.hero_moby.and_then(|i| w.mobys.get(i)) else { return };
    let Some(mut p) = ctx.host.joint(hm, 0x16 + foot as usize) else { return };
    let yaw = ctx.hero.rot[2];
    p[0] += yaw.cos() * 0.15;
    p[1] += yaw.sin() * 0.15;
    let z = crate::moby::ground_under(ctx, p);
    let size = ctx.game.rng.randf(1.0, 1.5);
    // No ground under it (the original takes the line's end): no puff here.
    let Some(z) = z else { return };
    p[2] = z + 0.04;
    if let Some(sys) = ctx.parts.as_deref_mut() { crate::particles::dust::dust(sys, &mut ctx.game.rng, size, 5250.0, p, None); }
}
