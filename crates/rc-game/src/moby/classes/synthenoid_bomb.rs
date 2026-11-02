//! The upgraded Synthenoids' bomb **0x0cf3** (`0x3f56b0`, made by `0x3f6008`; its one caller the upgraded drone's shot
//! `0x3d5528`, crate::moby::classes::synthenoid).
//!
//! **The throw** ([`throw`]): from the shot's point toward the target's aim point, 20 a second across and up as the
//! fall of 7 a second² needs to reach it (`0x31e048`); the bomb made (`0x3f6008`): far 0xff / 0xff, drawn, state 0, the
//! drone its owner (`+0x24`), its velocity (`+0x10`), its fall (`+0x1c`, 7/3600 a tick²), 180 ticks (`+0x20`), its smoke
//! 12 ticks (`+0x28`, `$gp+0x18c4`), its yaw along its way, its scale 0.005 (`$gp+0x1268`, the drone's shot sets it over
//! the make's 1.3 times).
//!
//! **Each tick**: a sphere of 0.65 (`0x2df8d8`, flags 0) meeting a moby not its drone, or its ticks out, sets **3**.
//! * **0** / **1** (flying; 1 rising): its velocity takes the fall along the down (`0x326520`); rising, once it falls
//!   (outside the gravity frames its velocity's z below 0) **0**. While its smoke lasts, three grey puffs along its step
//!   (crate::particles::puff, `0x335bb8(0.025, 1, 1, 5000 + 45000·ticks/12, …, ±0..5, 0, 0x80808080, 1)`, 120 ticks). Its
//!   step's line (`0x2de878`, flags 0) puts it where it meets anything but its drone, else at its end. A sphere of 0.208
//!   there meeting anything but its drone while falling (or rising, under the contact point): it sits 0.208 off the
//!   face, its velocity reflected (`0x2ef2a8`), its class sound 1, and **3**. It rolls 1°..8° a tick about y by its
//!   speed across (0.2..6 a second).
//! * **3**: the drone's class sound 5 at it (`0x360698`), the general explosion (crate::moby::blast, `0x320538(0, 0, 6, 2, 9,
//!   1, 15, …, 5, 2, 4, 0, shake, 1, −1, 0, 0x810001)`), every moby within 6 hit for the row's `+0x64` (flags 0x810000,
//!   kind `0x0cf3 << 16 | 0x0102`, `0x2e14b0`), deleted.
//!
//! Not ported: the gravity frames' bends (`0x380e08`, `0x380e58`), the weapon mods' effects on a hit (`0x4126b8`), the
//! moby set-up `0x313dc0`, the rolling state 2 (a bounce while falling sets 3 first, so its count never passes 1: the
//! tinted slide of `+0x78` / `+0x2c` and the bounces' slow-downs of `0x3f56b0` are unreachable).

use crate::hero::items::{row, SYNTHENOID};
use crate::hero::physics::{add, atan2, len2, scale, set_len, sub, unit, V3};
use crate::moby::hits::HitTemplate;
use crate::moby::{countdown, Ctx, MobyWorld};

/// The bomb's class.
pub const CLASS: i16 = 0xcf3;
/// `$gp+0x1260` / `$gp+0x1264`: the throw's speed across (a second) and its fall (a second²).
const SPEED: f32 = 20.0;
const FALL: f32 = 7.0;
/// `$gp+0x18c0`: the bounce's sphere.
const RADIUS: f32 = f32::from_bits(0x3e54_fdf4);
/// `$gp+0x18c4`: the smoke's ticks.
const SMOKE: i16 = 0xc;

/// A bomb's damage (the row's `+0x64`; crate::hero::items::ItemRow keeps only `+0x60`).
fn damage(game: &crate::game::GameState) -> f32 {
    match row(game, SYNTHENOID).map(|r| r.item) {
        Some(0x5d) => 25.0,
        Some(0x5e) => 50.0,
        _ => 5.0,
    }
}

/// The shot's bomb from `from` toward `to` (module docs; `0x31e048` + `0x3f6008`).
pub fn throw(w: &mut MobyWorld, _ctx: &mut Ctx, drone: usize, from: V3, to: V3) -> Option<usize> {
    w.class(CLASS)?;
    let (speed, fall) = (SPEED * 0.016_666_668, FALL * 0.000_277_777_8);
    // `0x31e048(speed, −fall, from, to)`: the ticks across, the rise that meets `to` then.
    let t = len2(sub(to, from)) / speed;
    let up = if t != 0.0 { -((from[2] - to[2]) + -fall * t * t * 0.5) / t } else { 0.0 };
    let d = sub(to, from);
    let mut vel = set_len([d[0], d[1], 0.0], speed);
    vel[2] = up;
    let id = w.create(CLASS);
    let m = &mut w.mobys[id];
    m.pvars.resize(crate::moby::PVAR_BYTES, 0);
    m.update_dist = 0xff;
    m.draw_dist = 0xff;
    m.drawn = 1;
    m.state = 0;
    m.parent = Some(drone);
    m.set_pv_i(0x24, drone as i32);
    m.pos = from;
    m.set_pv_v(0x10, vel);
    m.set_pv_f(0x1c, fall);
    m.set_pv_i(0x20, 0xb4);
    m.set_pv_s(0x28, SMOKE);
    m.set_pv_s(0x2a, 0);
    m.rot = [0.0, 0.0, atan2(vel[0], vel[1])];
    m.rows = crate::hero::physics::euler_rows(m.rot);
    m.scale = f32::from_bits(0x3ba3_d70a);
    Some(id)
}

/// `0x3f56b0` (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].pvars.resize(crate::moby::PVAR_BYTES, 0);
    let owner = w.mobys[id].pv_link(0x24);
    let pos = w.mobys[id].pos;
    let near = ctx.coll.and_then(|c| crate::collision::coll_sphere_m(c, ctx.scene, pos, f32::from_bits(0x3f26_6666), crate::collision::QueryFlags(0), Some(id)));
    if near.is_some_and(|h| h.moby.is_some() && h.moby != owner) { w.mobys[id].state = 3; }
    let mut n = w.mobys[id].pv_i(0x20);
    if countdown(&mut n) { w.mobys[id].state = 3; }
    w.mobys[id].set_pv_i(0x20, n);
    match w.mobys[id].state {
        3 => {
            explode(w, ctx, id);
            return;
        }
        0 | 1 => fly(w, id, ctx, owner),
        _ => return,
    }
    // Its roll by its speed across.
    let v = w.mobys[id].pv_v(0x10);
    let lo = f32::from_bits(0x3b5a_740f);
    let s = len2(v).clamp(lo, f32::from_bits(0x3dcc_ccce));
    let a = ((s - lo) / f32::from_bits(0x3dc5_f92e) * 7.0 + 1.0) * f32::from_bits(0x3c8e_fa36);
    let m = &mut w.mobys[id];
    m.rot[1] = crate::hero::physics::add_rot(a, m.rot[1]);
    m.rows = crate::hero::physics::euler_rows(m.rot);
}

/// States 0 and 1 (module docs).
fn fly(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, owner: Option<usize>) {
    let pos = w.mobys[id].pos;
    let mut vel = w.mobys[id].pv_v(0x10);
    let next = add(pos, vel);
    let up = ctx.hero.gravity.up_at(pos);
    let down = scale(up, -1.0);
    vel = add(vel, scale(down, w.mobys[id].pv_f(0x1c)));
    if w.mobys[id].state == 1 {
        let falling = if ctx.hero.gravity.on { 0.0 < crate::hero::physics::dot(vel, down) } else { vel[2] < 0.0 };
        if falling { w.mobys[id].state = 0; }
    }
    let mut smoke = w.mobys[id].pv_s(0x28) as i32;
    if smoke != 0 { smoke -= 1; }
    w.mobys[id].set_pv_s(0x28, smoke as i16);
    if smoke != 0 {
        for i in 0..3 {
            let t = i as f32 * f32::from_bits(0x3eaa_a64c);
            let p: V3 = std::array::from_fn(|k| pos[k] + (next[k] - pos[k]) * t);
            let g = &mut ctx.game.rng;
            let mut spin = g.randi(6);
            if g.randi(2) != 0 { spin = -spin; }
            let size = smoke as f32 / SMOKE as f32 * 45_000.0 + 5000.0;
            let Some(sys) = ctx.parts.as_deref_mut() else { continue };
            if let Some(k) = crate::particles::puff(sys, g, f32::from_bits(0x3ccc_cccd), 1.0, 1.0, size, p, spin, [0.0; 3], 0x8080_8080, 1) {
                let r = &mut sys.pool.recs[k];
                crate::particles::rec::set_i16(r, 0xa, 0x78);
                crate::particles::rec::set_u32(r, 0x24, 2);
                r[0x2a] = 0x7f;
                r[0x2b] = r[0xa];
            }
        }
    }
    let line = w.coll_line(ctx, pos, next, 0, Some(id));
    let (at, met) = match line.filter(|o| o.moby.is_none() || o.moby != owner) {
        Some(o) => (o.point, Some(o.point)),
        None => (next, None),
    };
    // Under the contact point (outside the gravity frames its z below the line's point's; the port's record is the
    // line's, else none).
    let below = met.is_some_and(|p| if ctx.hero.gravity.on { 0.0 < crate::hero::physics::dot(sub(at, p), down) } else { at[2] - p[2] < 0.0 });
    let mut pos = at;
    let hit = ctx.coll.and_then(|c| crate::collision::coll_sphere_m(c, ctx.scene, pos, RADIUS, crate::collision::QueryFlags(0), Some(id)));
    if let Some(h) = hit {
        if (w.mobys[id].state == 0 || below) && (h.moby.is_none() || h.moby != owner) {
            let n = unit(h.normal);
            pos = add(h.point, scale(n, RADIUS));
            // `0x2ef2a8`: reflected when it runs into the face.
            let d = crate::hero::physics::dot(vel, n);
            if d < 0.0 { vel = sub(vel, scale(n, 2.0 * d)); }
            ctx.host.play_sound(&w.mobys[id], 1, 0);
            if w.mobys[id].state == 0 {
                w.mobys[id].state = 3;
                let b = w.mobys[id].pv_s(0x2a);
                w.mobys[id].set_pv_s(0x2a, b + 1);
            }
        }
    }
    let m = &mut w.mobys[id];
    m.pos = pos;
    m.set_pv_v(0x10, vel);
}

/// State 3 (module docs).
fn explode(w: &mut MobyWorld, ctx: &mut Ctx, id: usize) {
    ctx.host.play_sound_of(&w.mobys[id], super::synthenoid::PLUS_DRONE, 5, 0);
    let e = crate::moby::blast::Explosion { flash: 6.0, flash2: 2.0, near: 9.0, size: 1.0, light: 15.0, embers: 5, billows: 2, flares: 4, sound: 0, shake: true, debris: 1, lod: -1, flags: 0x81_0001, ..Default::default() };
    crate::moby::blast::explode(w, ctx, Some(id), None, &e);
    let mut t = HitTemplate::new(damage(ctx.game), Some(id), 0x81_0000, [0.0; 4]);
    t.kind = (CLASS as u16 as u32) << 16 | 1 << 8 | 2;
    let pos = w.mobys[id].pos;
    w.sphere_hit(ctx.scene, pos, 6.0, 0x10, Some(id), Some(&t));
    w.delete(id);
}
