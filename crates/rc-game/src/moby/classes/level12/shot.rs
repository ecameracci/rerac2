//! Level 12's blaster shot 0x03a6 (`0x3a7ae0`; fired by `0x3a7fb8`, the trooper's guns,
//! crate::moby::classes::level12::trooper).
//!
//! The engine functions: `0x33ea28` = L00 `0x339848` (crate::particles::spark), `0x325618` = L00 `0x321c00` (a vector
//! turned about an axis), `0x2e1e20` / `0x2e2e80` the line and sphere queries (crate::collision), `0x2e4a58` = L00
//! `0x2e14b0` the sphere's hits (crate::moby::MobyWorld::sphere_hit), `0x323f50` = L00 `0x320538`
//! (crate::moby::blast::explode).
//!
//! Not ported: its trail (`0x33fd00`, particle type 0x37: 0.1 wide, 0.01 long, `0xf0208080`, 0x60 ticks; L00 has
//! no such type) and the half of the ticks' sparkles `0x33a960(25000, …, 0x4f007fff, 0x1fffffff, 0xd, 1)` (L00
//! `0x335780`, unported); the random draws they take are kept.

use crate::hero::physics::{add, len, scale, set_len, sub, V3};
use crate::moby::{countdown, Ctx, MobyWorld};

pub const CLASS: i16 = 0x03a6;

/// Its pvars: `+0x00` its velocity, `+0x38` its life, `+0x3c` its owner (a link), `+0x40` its damage.
const VEL: usize = 0;
const LIFE: usize = 0x38;
const OWNER: usize = 0x3c;
const DAMAGE: usize = 0x40;

/// `0x3a7fb8(damage, owner, vel, pos)`: a shot at `pos` moving `vel` a tick, its life 240 ticks, its yaw and pitch
/// along `vel`; update and draw distances 0xff, the owner's colour words; hidden until its first update.
pub fn fire(w: &mut MobyWorld, ctx: &mut Ctx, damage: f32, owner: usize, vel: V3, pos: V3) -> usize {
    let k = w.create(CLASS);
    let (light, ambient) = (w.mobys[owner].light, w.mobys[owner].ambient);
    let m = &mut w.mobys[k];
    m.update_dist = 0xff;
    m.draw_dist = 0xff;
    m.state = 0;
    m.light = light;
    m.ambient = ambient;
    m.pos = pos;
    m.set_pv_v(VEL, vel);
    crate::moby::ai::set_link(m, OWNER, Some(owner));
    m.set_pv_i(LIFE, 0xf0);
    m.set_pv_f(DAMAGE, damage);
    m.rot[2] = crate::hero::physics::atan2(vel[0], vel[1]);
    m.rot[1] = -crate::hero::physics::atan2(len([vel[0], vel[1], 0.0]), vel[2]);
    w.rebuild(k, &*ctx.host);
    let m = &mut w.mobys[k];
    m.drawn = 0;
    m.mode |= 1;
    k
}

/// 0x03a6 (`0x3a7ae0`), state 0: its roll on 6° a tick; a spark at it (40000 growing 1000, life 7, 0xb0b0b0) drifting
/// with a random part of its velocity and a little either way; its hit (its damage, from its owner, flags 0x10001, kind
/// its class · 0x10000 | 4, along the owner-to-it line across, 5/3 long); the line to its next place (flags 0, the
/// hit on what it meets), else a 0.3 sphere (flags 1), else its life: none of them, on it goes. Else: through its
/// owner on it goes; on a moby (not class 0x4e) a 0.5 sphere of the hit; a burst (2 billows, 10 flares, size 2/3; on
/// the world 8 flares, size 1/2), its class sound 0, gone.
pub fn update_03a6(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state != 0 { return; }
    let m = &mut w.mobys[id];
    m.rot[0] = crate::hero::physics::add_rot(m.rot[0], f32::from_bits(0x3dd6_7752));
    let (pos, vel) = (m.pos, m.pv_v(VEL));
    let next = add(pos, vel);
    m.drawn = 1;
    m.mode &= !1;
    {
        let rng = &mut ctx.game.rng;
        let speed = len(vel);
        let r = [rng.randf(-1.0, 1.0), rng.randf(-1.0, 1.0), rng.randf(-1.0, 1.0)];
        let r = scale(r, rng.randf(0.04, 0.08) * 0.016_666_668);
        let v = add(r, scale(vel, rng.randf(0.2, f32::from_bits(0x3f7d_70a4))));
        let _along = rng.randf(0.0, 1.0) * speed;
        if let Some(sys) = ctx.parts.as_deref_mut() {
            crate::particles::spark(sys, &mut ctx.game.rng, 40000.0, 1000.0, 1.0, f32::from_bits(0xb9d1_b717), 0.0, pos, v, 7, 0x7f, 0xb0_b0b0, 3);
        }
        let rng = &mut ctx.game.rng;
        let _turn = rng.rand_angle();
        let _sparkle = rng.rand() & 1;
    }
    let owner = crate::moby::ai::link(&w.mobys[id], OWNER);
    let opos = owner.and_then(|o| w.mobys.get(o)).map_or(pos, |o| o.pos);
    let d = sub(pos, opos);
    let d = set_len([d[0], d[1], 0.0], f32::from_bits(0x3fd5_5556));
    let mut hit = crate::moby::hits::HitTemplate::new(w.mobys[id].pv_f(DAMAGE), owner, 0x1_0001, [d[0], d[1], d[2], 5_627.987]);
    hit.kind = (CLASS as u16 as u32) << 16 | 4;
    let (coll, scene) = (ctx.coll, ctx.scene);
    let line = coll.and_then(|c| crate::collision::coll_line_m(c, scene, pos, next, crate::collision::QueryFlags(0), Some(id)));
    if let Some(o) = line.as_ref() {
        if let Some(t) = o.moby { w.deliver_hit_any(t, &hit, [o.point[0], o.point[1], o.point[2], 1.0]); }
    }
    let met = match line {
        Some(o) => Some(o.moby),
        None => coll.and_then(|c| crate::collision::coll_sphere_m(c, scene, pos, 0.3, crate::collision::QueryFlags(1), Some(id))).map(|o| o.moby),
    };
    let met = match met {
        Some(mo) => Some(mo),
        None => {
            let mut t = w.mobys[id].pv_i(LIFE);
            let out = countdown(&mut t);
            w.mobys[id].set_pv_i(LIFE, t);
            out.then_some(None)
        }
    };
    let Some(mo) = met else {
        w.mobys[id].pos = next;
        return;
    };
    let (size, flares) = match mo {
        None => (0.5, 8),
        Some(t) if Some(t) == owner => {
            w.mobys[id].pos = next;
            return;
        }
        Some(t) => {
            if w.mobys.get(t).is_some_and(|o| o.o_class != 0x4e) { w.sphere_hit(scene, pos, 0.5, 0, owner, Some(&hit)); }
            (f32::from_bits(0x3f2a_7efa), 10)
        }
    };
    let e = crate::moby::blast::Explosion { near: 9.0, size, billows: 2, flares, sound: -1, debris: 1, lod: -1, flags: 0x81_0001, ..Default::default() };
    crate::moby::blast::explode(w, ctx, Some(id), None, &e);
    ctx.host.play_sound(&w.mobys[id], 0, 0);
    w.delete(id);
}
