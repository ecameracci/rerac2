//! Level 12's debris pieces: the spawn `0x32ad40` and the class 0x0d66 (`0x400850`; the trooper's guns,
//! crate::moby::classes::level12::trooper, held on its turret until it loses them).
//!
//! The engine functions: `0x2f2da0` the rows of Euler angles (crate::hero::physics::euler_rows), `0x2e2e80` the
//! sphere query (crate::collision::coll_sphere_m), `0x31f690` = L00 `0x31bbc0` (crate::rng::Rng::rand_vec),
//! `0x323f50` = L00 `0x320538` (crate::moby::blast::explode).
//!
//! Not ported: the smoke `0x324d88(0.3, 10, m, at, −1)` a piece with its flag 2 goes off in (no spawn here sets it).

use crate::hero::physics::{add, cross, dot, euler_rows, len, scale, set_len, sub, V3};
use crate::moby::{countdown, Ctx, MobyWorld};

/// The piece's pvars: `+0x00` its velocity, `+0x10` its spin (Euler a tick), `+0x20` its sphere (centre in its frame,
/// `w` the radius), `+0x30` its life, `+0x34` its flags, `+0x38` its gravity.
const VEL: usize = 0;
const SPIN: usize = 0x10;
const SPHERE: usize = 0x20;
const LIFE: usize = 0x30;
const FLAGS: usize = 0x34;
const GRAVITY: usize = 0x38;

fn local(rows: &[V3; 3], v: V3) -> V3 { add(add(scale(rows[0], v[0]), scale(rows[1], v[1])), scale(rows[2], v[2])) }

/// `0x32ad40(gravity, src, class, pos, rot, life, flags, vel, spin, sphere)`: a piece of `class` at `pos` with the
/// angles `rot`, its scale `src`'s scaled by the classes' (`+0x24` of each), drawn, rows kept, state 1, its colour words
/// (`+0x38`) `src`'s, update distance 0xff, draw distance 0x80, no collision; its life `life` (0: 60..120), its flags;
/// its gravity 10..15 a second² (the pvars are new: `gravity` is never taken); its velocity `vel` (none: 2..4 a second
/// at a random heading and 5..8 up), its spin `spin` (none: 3..6° a tick a random way), its sphere `sphere` (none: the
/// class's, scaled). The update distance is read by nothing here.
#[allow(clippy::too_many_arguments)]
pub fn spawn(w: &mut MobyWorld, ctx: &mut Ctx, src: usize, class: i16, pos: V3, rot: V3, life: i32, flags: f32, vel: Option<V3>, spin: Option<V3>) -> usize {
    let k = w.create(class);
    let s_scale = w.mobys[src].scale;
    let s_base = w.class(w.mobys[src].o_class).map_or(1.0, |c| c.scale);
    let base = w.class(class).map_or(1.0, |c| c.scale);
    let (light, ambient) = (w.mobys[src].light, w.mobys[src].ambient);
    let rng = &mut ctx.game.rng;
    let life = if life == 0 { rng.randf(60.0, 120.0) as i32 } else { life };
    let gravity = rng.randf(10.0, 15.0) * 0.000_277_777_78;
    let vel = vel.filter(|v| len(*v) != 0.0).unwrap_or_else(|| {
        let a = rng.rand_angle();
        let s = rng.randf(2.0, 4.0) * 0.016_666_668;
        let up = rng.randf(5.0, 8.0) * 0.016_666_668;
        [a.cos() * s, a.sin() * s, up]
    });
    let spin = spin.filter(|v| len(*v) != 0.0).unwrap_or_else(|| rng.rand_vec(f32::from_bits(0x3d56_7752), f32::from_bits(0x3dd6_7752)));
    let m = &mut w.mobys[k];
    m.drawn = 1;
    m.mode |= crate::moby::mode::KEEP_ROWS;
    m.scale = base * (s_scale / s_base);
    m.state = 1;
    m.light = light;
    m.ambient = ambient;
    m.pos = pos;
    m.rot = rot;
    m.rows = euler_rows(rot);
    m.update_dist = 0xff;
    m.draw_dist = 0x80;
    m.has_collision = false;
    m.set_pv_v(VEL, vel);
    m.set_pv_v(SPIN, spin);
    let r = m.scale / 1024.0;
    let b = m.bsphere;
    m.set_pv_v(SPHERE, [b[0] * r, b[1] * r, b[2] * r]);
    m.set_pv_f(SPHERE + 0xc, b[3] * r);
    m.set_pv_i(LIFE, life);
    m.set_pv_f(FLAGS, flags);
    m.set_pv_f(GRAVITY, gravity);
    w.rebuild(k, &*ctx.host);
    k
}

/// 0x0d66 (`0x400850`). **0** / **1**: flying ([`fly`]); in 1 its life out → 3, ten ticks, mode | 0x2000. **2**: it
/// goes off at its sphere's centre (the flash 1 and 0.5, size 1/3, 5 embers, 3 billows, 5 flares) and goes. **3**:
/// fading (alpha 12.8 a tick left), flying; out, gone.
pub fn update_0d66(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        1 => {
            let mut t = w.mobys[id].pv_i(LIFE);
            let out = countdown(&mut t);
            w.mobys[id].set_pv_i(LIFE, t);
            if out {
                let m = &mut w.mobys[id];
                m.state = 3;
                m.set_pv_i(LIFE, 10);
                m.mode |= 0x2000;
                return;
            }
            fly(w, id, ctx);
        }
        0 => fly(w, id, ctx),
        2 => {
            let m = &w.mobys[id];
            let at = add(local(&m.rows, m.pv_v(SPHERE)), m.pos);
            if (m.pv_f(FLAGS) as i32) & 2 == 0 {
                let e = crate::moby::blast::Explosion {
                    flash: 1.0, flash2: 0.5, near: 9.0, size: f32::from_bits(0x3eaa_7efa), embers: 5, billows: 3, flares: 5,
                    sound: -1, lod: -1, flags: 0x81_0001, ..Default::default()
                };
                crate::moby::blast::explode(w, ctx, Some(id), Some(at), &e);
            }
            w.delete(id);
        }
        3 => {
            let mut t = w.mobys[id].pv_i(LIFE);
            if countdown(&mut t) {
                w.delete(id);
                return;
            }
            w.mobys[id].set_pv_i(LIFE, t);
            w.mobys[id].alpha = (t as f32 * 12.8) as i32 as u8;
            fly(w, id, ctx);
        }
        _ => {}
    }
}

/// The flight: turned by its spin about its sphere's centre, moved by its velocity, its velocity down by its
/// gravity; its sphere meeting the world or a moby (flags 0) and moving into it: bounced (half its speed into the
/// face kept, then its speed as it was), its spin about the line across the face and its way at the spin's rate.
fn fly(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    let c = m.pv_v(SPHERE);
    let turn = euler_rows(m.pv_v(SPIN));
    let a = local(&m.rows, c);
    m.rows = m.rows.map(|r| local(&turn, r));
    m.rot = crate::moby::attach::angles_of(&m.rows);
    let b = local(&m.rows, c);
    let mut v = m.pv_v(VEL);
    m.pos = add(sub(m.pos, sub(b, a)), v);
    v[2] -= m.pv_f(GRAVITY);
    m.set_pv_v(VEL, v);
    let at = add(local(&m.rows, c), m.pos);
    let r = m.pv_f(SPHERE + 0xc);
    let Some(coll) = ctx.coll else { return };
    let Some(o) = crate::collision::coll_sphere_m(coll, ctx.scene, at, r, crate::collision::QueryFlags(0), Some(id)) else { return };
    let n = set_len(o.normal, 1.0);
    let speed = len(v);
    let d = dot(v, n);
    if d < 0.0 {
        let mut v = sub(v, scale(n, 2.0 * d));
        v = add(v, scale(n, d * 0.5));
        v = set_len(v, speed);
        let m = &mut w.mobys[id];
        m.set_pv_v(VEL, v);
        let axis = set_len(cross(n, v), 1.0);
        let q = crate::quat::axis_angle(len(m.pv_v(SPIN)), axis);
        m.set_pv_v(SPIN, crate::moby::attach::angles_of(&crate::quat::rows(q)));
    }
}
