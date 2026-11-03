//! Level 12's lift (L12 addresses; the engine functions named by their L12 addresses with the L00 ones the port
//! knows them by).
//!
//! * **0x0fbc** ([`update_0fbc`], `0x41ec70`, 3 placed): a lift between its place and its cuboid's height, called by
//!   Ratchet coming near either end or sent by △ while he stands on it.
//! * **0x0fd8** ([`update_0fd8`], `0x41fe80`, 2 placed): the same lift without the walk or the turn, working while
//!   hooked (`+0xbe` bit 2, `0x32eb40`).
//! * **0x0fd9** ([`update_0fd9`], `0x420220`, one): a door that opens while its moby is near or Ratchet (or the
//!   camera) is in its cuboid.
//!
//! The engine functions: `0x322790` = L00 `0x31ed78` (crate::hero::physics::ease), `0x322c48` = L00 `0x31f230`
//! (crate::hero::physics::turn_ease), `0x32b218` = L00 `0x327a28` (crate::prompt: the lease), `0x2d7a30` = L00
//! `0x2d4440` (crate::hero::Hero::script_walk_to), `0x2d7a80` = L00 `0x2d4490` (his walk: 1 under way, 2 there),
//! `0x2d7ac8` the walk ended (his state 0, or 3 at a speed of 0.045 or more), `0x326080` = L00 `0x322668`
//! (crate::moby::platform::write), `0x365ed8` / `0x365bd8` = L00 `0x3605f0` / `0x3602f0` the class sound and its stop.

use crate::hero::physics::{add_rot, dist2, ease, sub, turn_ease};
use crate::moby::{Ctx, MobyWorld};
use crate::pad::button;

/// The lift's platform block's pvars offset.
const BLOCK: usize = 0x20;

/// Its targets: the height `+0x68`[0x84] and yaw `[0x88]` of the end `[0x64]` (0 its home `[0x70..0x7c]`, 1 its
/// cuboid `[0x60]`: its centre's height, and the cuboid's `+0x78` plus 270°).
fn targets(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let m = &mut w.mobys[id];
    let c = usize::try_from(m.pv_i(0x60)).ok().and_then(|i| ctx.level.cuboids.get(i));
    let (z, yaw) = match (m.pv_i(0x64), c) {
        (0, _) | (_, None) => (m.pv_f(0x78), m.pv_f(0x7c)),
        (_, Some(c)) => (c.matrix[3][2], add_rot(c.extra[2], f32::from_bits(0x4096_cbe5))),
    };
    m.set_pv_f(0x84, z);
    m.set_pv_f(0x88, yaw);
}

/// 0x0fbc (`0x41ec70`, 3 placed): pvars `[0x60]` its cuboid, `[0x64]` the end it is at or going to, `[0x68]` /
/// `[0x6c]` its turn and climb speeds, `[0x70..0x7c]` its home (`[0x7c]` its yaw), `[0x80]` its looped sound,
/// `[0x8c]` the walk's ticks. **0**: a quarter turn left, its home its place and yaw; 2. **1** (moving): at its start
/// both speeds 0; its height eased to the end's + 0.5 (`0x322790`: up by 1/512 a tick, at most 0.5, braking 1/256)
/// — going up to the cuboid with Ratchet in the cuboid more than 1 under it, to the end's + 3, braking 1/128; within
/// 0.6 of the end's height and under 0.01 a tick: the sound stopped, 2; else its class sound 0 looped while not
/// playing; its yaw eased to the end's (`0x322c48`: 1/7000, at most 6°). **2** (waiting): Ratchet standing on it:
/// the prompt 0x12ec (`0x32b218(8, 0x12ec, 1)`) and △ sends it to the other end (3); else Ratchet within 4 in height
/// of the other end and 32 (xy) of it calls it there (1). **3**: at its start he walks to its place keeping his yaw
/// (`0x2d7a30`); while he walks (up to 360 ticks) it waits, then 1; at 360 the walk ends (`0x2d7ac8`) and it stays (2).
/// **Each tick**: its platform block written with the move.
pub fn update_0fbc(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (old, old_rot) = (w.mobys[id].pos, w.mobys[id].rot);
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.rot[2] = add_rot(m.rot[2], f32::from_bits(0x3fc9_0fdc));
            let (p, yaw) = (m.pos, m.rot[2]);
            m.set_pv_v(0x70, p);
            m.set_pv_f(0x7c, yaw);
            m.set_state(2, -1);
        }
        1 => {
            if w.mobys[id].first() {
                w.mobys[id].set_pv_f(0x68, 0.0);
                w.mobys[id].set_pv_f(0x6c, 0.0);
            }
            targets(w, id, ctx);
            let hero = ctx.hero.pos;
            let m = &mut w.mobys[id];
            let (mut t, mut brake) = (m.pv_f(0x84), f32::from_bits(0x3b7e_dcbb));
            if m.pv_i(0x64) != 0 && hero[2] < m.pos[2] - 1.0 && ctx.level.in_cuboid(m.pv_i(0x60), hero) {
                brake = f32::from_bits(0x3bfe_dcbb);
                t += 3.0;
            } else {
                t += 0.5;
            }
            let (mut z, mut v) = (m.pos[2], m.pv_f(0x6c));
            ease(t, f32::from_bits(0x3afe_dcbb), brake, 0.5, &mut z, &mut v);
            m.pos[2] = z;
            m.set_pv_f(0x6c, v);
            let h = m.pv_i(0x80);
            if (m.pv_f(0x84) - z).abs() < 0.6 && v.abs() < 0.01 {
                if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
                m.set_pv_i(0x80, -1);
                m.set_state(2, -1);
            } else if !(h != -1 && ctx.host.sound_playing(h)) {
                let h = ctx.host.play_sound(&w.mobys[id], 0, 4);
                w.mobys[id].set_pv_i(0x80, h);
            }
            let m = &mut w.mobys[id];
            let (mut a, mut v) = (m.rot[2], m.pv_f(0x68));
            turn_ease(m.pv_f(0x88), f32::from_bits(0x3918_825c), f32::from_bits(0x3918_825c), f32::from_bits(0x3dd6_7752), &mut a, &mut v, 0);
            m.rot[2] = a;
            m.set_pv_f(0x68, v);
        }
        2 => {
            targets(w, id, ctx);
            if ctx.hero.ground_moby == Some(id) {
                ctx.game.prompt.request(8, 0x12ec, 1);
                if ctx.hero.items.pressed & button::TRIANGLE != 0 {
                    let m = &mut w.mobys[id];
                    m.set_pv_i(0x64, (m.pv_i(0x64) + 1) & 1);
                    m.set_state(3, -1);
                }
            } else {
                let hero = ctx.hero.pos;
                let m = &mut w.mobys[id];
                let near = |z: f32| (hero[2] - z).abs() < 4.0 && dist2(hero, m.pos) < 32.0;
                if m.pv_i(0x64) == 0 {
                    let c = usize::try_from(m.pv_i(0x60)).ok().and_then(|i| ctx.level.cuboids.get(i)).map(|c| c.matrix[3][2]);
                    if c.is_some_and(near) {
                        m.set_pv_i(0x64, 1);
                        m.set_state(1, -1);
                    }
                } else if near(m.pv_f(0x78)) {
                    m.set_pv_i(0x64, 0);
                    m.set_state(1, -1);
                }
            }
        }
        3 => {
            if w.mobys[id].first() {
                let (yaw, p) = (ctx.hero.rot[2], w.mobys[id].pos);
                ctx.hero.script_walk_to(yaw, p, false);
                w.mobys[id].set_pv_i(0x8c, 0);
            }
            let m = &mut w.mobys[id];
            let n = m.pv_i(0x8c) + 1;
            m.set_pv_i(0x8c, n);
            if n < 0x169 {
                if !(0x62..0x64).contains(&ctx.hero.state) { m.set_state(1, -1); }
            } else {
                let s = if ctx.hero.eff_len < 0.045 { 0 } else { 3 };
                ctx.hero.request_state(s, true);
                m.set_state(2, -1);
            }
        }
        _ => {}
    }
    let m = &mut w.mobys[id];
    let (pos, rot) = (m.pos, m.rot);
    crate::moby::platform::write(m, BLOCK, sub(pos, old), old_rot, rot);
    crate::moby::carry_hero(ctx, id, old, old_rot[2], pos, rot[2]);
}

/// 0x0fd8 (`0x41fe80`, 2 placed): pvars `[0x60]` its cuboid, `[0x68]` its end (0 home, 1 the cuboid's height),
/// `[0x6c]` its climb speed, `[0x70..0x7c]` its home, `[0x80]` its looped sound. **0**: its home its place; 1. **1**,
/// while hooked: the end's height `t` (home or the cuboid's centre); within 2 of it: Ratchet standing on it gets the
/// prompt 0x12ec (`0x32b218(8, 0x12ec, 1)`) and △ sends it to the other end; else Ratchet within 4 in height of the
/// other end and 24 (xy) of it calls it there. Its height eased (`0x322790`: up by 1/512, at most 0.5, braking 1/256)
/// to `t` (home), `t` + 0.5 (the cuboid), or with Ratchet in the cuboid more than 1 under it `t` + 3 braking 1/128
/// (`t` the end's height before this tick's change). Under 0.8 from that its sound stopped, else its class sound 0
/// looped while not playing. **Each tick**: its platform block (`[0x20]`) written with the move.
pub fn update_0fd8(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (old, rot) = (w.mobys[id].pos, w.mobys[id].rot);
    let state = w.mobys[id].state;
    if state == 0 {
        let m = &mut w.mobys[id];
        let p = m.pos;
        m.set_pv_v(0x70, p);
        m.set_state(1, -1);
    } else if state == 1 && w.mobys[id].once & 4 != 0 {
        let hero = ctx.hero.pos;
        let m = &mut w.mobys[id];
        let cz = usize::try_from(m.pv_i(0x60)).ok().and_then(|i| ctx.level.cuboids.get(i)).map(|c| c.matrix[3][2]);
        let mut t = if m.pv_i(0x68) == 0 { m.pv_f(0x78) } else { cz.unwrap_or(0.0) };
        if (t - m.pos[2]).abs() < 2.0 {
            if ctx.hero.ground_moby == Some(id) {
                ctx.game.prompt.request(8, 0x12ec, 1);
                if ctx.hero.items.pressed & button::TRIANGLE != 0 { m.set_pv_i(0x68, (m.pv_i(0x68) + 1) & 1); }
            } else {
                let near = |z: f32| (hero[2] - z).abs() < 4.0 && dist2(hero, m.pos) < 24.0;
                if m.pv_i(0x68) == 0 {
                    if cz.is_some_and(near) { m.set_pv_i(0x68, 1); }
                } else if near(m.pv_f(0x78)) {
                    m.set_pv_i(0x68, 0);
                }
            }
        }
        let mut brake = f32::from_bits(0x3b7e_dcbb);
        if m.pv_i(0x68) != 0 {
            if hero[2] < m.pos[2] - 1.0 && ctx.level.in_cuboid(m.pv_i(0x60), hero) {
                brake = f32::from_bits(0x3bfe_dcbb);
                t += 3.0;
            } else {
                t += 0.5;
            }
        }
        let (mut z, mut v) = (m.pos[2], m.pv_f(0x6c));
        ease(t, f32::from_bits(0x3afe_dcbb), brake, 0.5, &mut z, &mut v);
        m.pos[2] = z;
        m.set_pv_f(0x6c, v);
        let h = m.pv_i(0x80);
        if (t - z).abs() < 0.8 {
            if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
            m.set_pv_i(0x80, -1);
        } else if !(h != -1 && ctx.host.sound_playing(h)) {
            let h = ctx.host.play_sound(&w.mobys[id], 0, 4);
            w.mobys[id].set_pv_i(0x80, h);
        }
    }
    let m = &mut w.mobys[id];
    let pos = m.pos;
    crate::moby::platform::write(m, BLOCK, sub(pos, old), rot, rot);
    crate::moby::carry_hero(ctx, id, old, rot[2], pos, rot[2]);
}

/// 0x0fd9 (`0x420220`, one): pvars `[0]` its moby, `[4]` its cuboid, `[0x10..0x1c]` its home, `[0x20]` the hold
/// timer, `[0x24]` how far open (0..1), `[0x28]` its speed, `[0x2c]` armed, `[0x30]` the ticks it has moved. **0**:
/// its home its place, update distance 0x7f, 1. **1**: its moby within 5 (xy) arms it and holds it open 60 ticks;
/// armed, Ratchet or the camera in the cuboid holds it too; while held it eases open (`0x322790` to 1: 1/1800, at most
/// 1/15), else shut (to 0); moving, its place is its home plus `(1, 0, 3)` through its rows times how far open, its
/// class sound 0 every 120 ticks of moving (from the first); still, the count 0.
pub fn update_0fd9(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            let p = m.pos;
            m.set_pv_v(0x10, p);
            m.set_state(1, -1);
            m.update_dist = 0x7f;
        }
        1 => {
            let near = usize::try_from(w.mobys[id].pv_i(0)).ok().and_then(|o| w.mobys.get(o)).is_some_and(|o| dist2(w.mobys[id].pos, o.pos) < 5.0);
            let cam = ctx.host.camera_pos();
            let m = &mut w.mobys[id];
            if near {
                m.set_pv_i(0x2c, 1);
                m.set_pv_i(0x20, 0x3c);
            } else if m.pv_i(0x2c) != 0 && (ctx.level.in_cuboid(m.pv_i(4), ctx.hero.pos) || ctx.level.in_cuboid(m.pv_i(4), cam)) {
                m.set_pv_i(0x20, 0x3c);
            }
            let mut t = m.pv_i(0x20);
            let out = crate::moby::countdown(&mut t);
            m.set_pv_i(0x20, t);
            let (mut x, mut v) = (m.pv_f(0x24), m.pv_f(0x28));
            let a = f32::from_bits(0x3a11_a2b4);
            ease(if out { 0.0 } else { 1.0 }, a, a, f32::from_bits(0x3d88_8889), &mut x, &mut v);
            m.set_pv_f(0x24, x);
            m.set_pv_f(0x28, v);
            if v == 0.0 {
                m.set_pv_i(0x30, 0);
            } else {
                let r = m.rows;
                let d: crate::hero::physics::V3 = std::array::from_fn(|k| (r[0][k] + 3.0 * r[2][k]) * x);
                m.pos = crate::hero::physics::add(m.pv_v(0x10), d);
                let n = m.pv_i(0x30);
                if n % 0x78 == 0 { ctx.host.play_sound(&w.mobys[id], 0, 0); }
                w.mobys[id].set_pv_i(0x30, n + 1);
            }
        }
        _ => {}
    }
}
