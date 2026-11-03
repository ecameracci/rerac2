//! Level 6's arc shot **0xfe5** (L06 `0x459e00`; thrown by `0x45a428`, the brawler 0x0bba's throw,
//! crate::moby::classes::level06::brawler; drawn by `0x45a548`): a ball of crackling arcs in one of six slots (the
//! level's globals, crate::moby::classes::level06::Globals).
//!
//! The engine functions: `0x31e060` / `0x31f0c0` the line and sphere queries (crate::collision), `0x35d370` its hit
//! (crate::moby::hits::HitTemplate), `0x35d398` = crate::moby::ai::attack_sphere, `0x35bb98` a random vector (two
//! angles, then its length), `0x362678` = L00 `0x321c00` (crate::camera::rot), `0x37b450` =
//! crate::particles::twinkle, `0x3a1d40(7, 4, m, class)` its owner class's sound 7 looped, `0x32b058` = L00
//! `0x2eb998` (crate::fx::FxQuad).
//!
//! Not ported: the light its slot names (`0x1b3778`, faded 1/6 a tick in state 1, `0x33d248`): nothing sets it (the
//! throw leaves it −1), so state 1 always deletes it at once. Its life out with nothing met, the original reads the
//! last query's moby and point (`0x1bfc98`, stale); the port takes the world at its own place.

use crate::fx::FxQuad;
use crate::hero::physics::{add, add_rot, atan2, cross, len, len2, scale, set_len, sub, V3};
use crate::moby::{ai, countdown, mode, Ctx, MobyWorld};

pub const CLASS: i16 = 0x0fe5;

/// Its pvars: `+0x00` its velocity, `+0x38` its life, `+0x3c` its owner (a link), `+0x40` its damage, `+0x48` its
/// slot, `+0x4c` its looped sound.
const VEL: usize = 0;
const LIFE: usize = 0x38;
const OWNER: usize = 0x3c;
const DAMAGE: usize = 0x40;
const SLOT: usize = 0x48;
const SOUND: usize = 0x4c;
const PVARS: usize = 0x50;
/// `$gp+0x28e4` / `$gp+0x28e8`: the arcs' core and glow FX.
const CORE_FX: usize = 0xe;
const GLOW_FX: usize = 0x10;
const ST: [[f32; 2]; 4] = [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]];

/// `0x45a428(damage, owner, vel, pos, slot)`: the shot at `pos` moving `vel` a tick in `slot` (taken), its life 240
/// ticks, its yaw and pitch along `vel`, the owner's colour words; update and draw distances 0xff, drawn.
pub fn fire(w: &mut MobyWorld, ctx: &mut Ctx, damage: f32, owner: usize, vel: V3, pos: V3, slot: usize) -> usize {
    let k = w.create(CLASS);
    let (light, ambient) = (w.mobys[owner].light, w.mobys[owner].ambient);
    w.level06.shot_slots[slot] = 1;
    let m = &mut w.mobys[k];
    if m.pvars.len() < PVARS { m.pvars.resize(PVARS, 0); }
    // The class has no model in the level's list: its update is set here.
    m.update = Some(update_0fe5);
    m.mode &= !mode::NO_UPDATE;
    m.update_dist = 0xff;
    m.draw_dist = 0xff;
    m.drawn = 1;
    m.state = 0;
    m.light = light;
    m.ambient = ambient;
    m.pos = pos;
    m.set_pv_v(VEL, vel);
    m.set_pv_i(LIFE, 0xf0);
    ai::set_link(m, OWNER, Some(owner));
    m.set_pv_f(DAMAGE, damage);
    m.set_pv_i(SLOT, slot as i32);
    m.set_pv_i(SOUND, -1);
    m.rot[2] = atan2(vel[0], vel[1]);
    m.rot[1] = -atan2(len2(vel), vel[2]);
    w.rebuild(k, &*ctx.host);
    k
}

fn stop_sound(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let h = w.mobys[id].pv_i(SOUND);
    if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
    w.mobys[id].set_pv_i(SOUND, -1);
}

/// 0xfe5 (`0x459e00`). **0**: its roll on 6° a tick; its hit (its damage, from itself, flags 1, along its velocity,
/// kind its class · 0x10000); the line to its next place (the hit on what it meets), else a 0.3 sphere, else its life:
/// none, or its owner met, on it goes. Else (sparks: [`super::brawler::sparks`], 20, a minute): on the world at the
/// point met; on a moby (not class 0x4e) a 0.5 sphere of its damage (flags 0x10001) and at its place; its sound
/// stopped, 1. On its way: its six arcs ([`arcs`]); half the ticks a red and a white twinkle (0.2..0.9, 2 ticks); its
/// owner class's sound 7 looped; drawn ([`draw`]). **1**: its slot freed, gone.
pub fn update_0fe5(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {}
        1 => {
            let s = w.mobys[id].pv_i(SLOT) as usize;
            if let Some(f) = w.level06.shot_slots.get_mut(s) { *f = 0; }
            w.delete(id);
            return;
        }
        _ => return arcs(w, id, ctx),
    }
    let m = &mut w.mobys[id];
    m.rot[0] = add_rot(m.rot[0], f32::from_bits(0x3dd6_7752));
    let (pos, vel) = (m.pos, m.pv_v(VEL));
    let next = add(pos, vel);
    let mut hit = crate::moby::hits::HitTemplate::new(m.pv_f(DAMAGE), Some(id), 1, [vel[0], vel[1], vel[2], m.pv_f(VEL + 0xc)]);
    hit.kind = (CLASS as u16 as u32) << 16;
    let owner = ai::link(&w.mobys[id], OWNER);
    let (coll, scene) = (ctx.coll, ctx.scene);
    let line = coll.and_then(|c| crate::collision::coll_line_m(c, scene, pos, next, crate::collision::QueryFlags(0), Some(id)));
    if let Some(o) = line.as_ref() {
        if let Some(t) = o.moby { w.deliver_hit_any(t, &hit, [o.point[0], o.point[1], o.point[2], 1.0]); }
    }
    let met = line.or_else(|| coll.and_then(|c| crate::collision::coll_sphere_m(c, scene, pos, 0.3, crate::collision::QueryFlags(0), Some(id)))).map(|o| (o.moby, o.point));
    let met = met.or_else(|| tick_life(w, id).then_some((None, pos)));
    match met {
        Some((Some(t), _)) if Some(t) == owner => w.mobys[id].pos = next,
        Some((t, point)) => {
            let mut at = match t {
                None => point,
                Some(t) => {
                    if w.mobys.get(t).is_some_and(|o| o.o_class != 0x4e) {
                        let d = w.mobys[id].pv_f(DAMAGE);
                        ai::attack_sphere(w, id, ctx, 0.5, d, 1.0, pos, 0x1_0001, 0, 1, 0);
                    }
                    pos
                }
            };
            super::brawler::sparks(ctx, &mut at, 0x14, 0x3c);
            stop_sound(w, id, ctx);
            w.mobys[id].set_state(1, -1);
            return;
        }
        None => w.mobys[id].pos = next,
    }
    arcs(w, id, ctx);
}

fn tick_life(w: &mut MobyWorld, id: usize) -> bool {
    let mut t = w.mobys[id].pv_i(LIFE);
    let out = countdown(&mut t);
    w.mobys[id].set_pv_i(LIFE, t);
    out
}

/// Its arcs, twinkles, sound and draw (`0x459e00`'s tail): each arc's timer 2 and alpha 0x40; from its place three
/// steps of 0.2 (a random way, `0x35bb98(0.2, 0.2)`), each turned about the line from the camera by 20..45° the other
/// way from the last.
fn arcs(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let p = w.mobys[id].pos;
    let slot = (w.mobys[id].pv_i(SLOT).max(0) as usize).min(5);
    let cam = ctx.host.camera_pos();
    let rng = &mut ctx.game.rng;
    for i in 0..6 {
        let g = &mut w.level06;
        g.arc_timers[i] = 2;
        g.arc_alpha[i] = 0x40;
        let (az, el) = (rng.rand_angle(), rng.rand_angle());
        let l = rng.randf(0.2, 0.2);
        let mut d = [az.cos() * l * el.cos(), az.sin() * l * el.cos(), el.sin() * l];
        if 0.2 < len(d) { d = set_len(d, 0.2); }
        let arc = &mut g.arcs[slot][i];
        arc[0] = p;
        let mut sign = 1.0;
        for k in 1..4 {
            let axis = sub(arc[k - 1], cam);
            let a = rng.randf(f32::from_bits(0x3eb2_b8c4), f32::from_bits(0x3f49_0fdc)) * sign;
            sign = -sign;
            d = crate::camera::rot(d, a, axis);
            arc[k] = add(arc[k - 1], d);
        }
    }
    if rng.randi(2) == 0 {
        let f = rng.randf(0.2, f32::from_bits(0x3f66_6666));
        let r = rng.randi(2);
        let spin = if r == 0 { -1 } else { r } as i8;
        if let Some(sys) = ctx.parts.as_deref_mut() {
            crate::particles::twinkle(sys, rng, f * 0.1, f, 0.0, p, 2, 0x7f7f_2020, 0, spin, [0.0; 3], 0);
            crate::particles::twinkle(sys, rng, f * 0.07, f * 0.7, 0.0, p, 2, 0x7f7f_7f7f, 1, -spin, [0.0; 3], 0);
        }
    }
    let h = w.mobys[id].pv_i(SOUND);
    if !(h != -1 && ctx.host.sound_playing(h)) {
        let h = ctx.host.play_sound_of(&w.mobys[id], super::brawler::CLASS, 7, 4);
        w.mobys[id].set_pv_i(SOUND, h);
    }
    w.register_draw(id);
}

/// `0x45a548`: each arc of its slot with its timer running ([`strip`], widths 0.025 / 0.5, the glow 0.2 longer at
/// each end, its alpha), then its glow (`0x45ac38`: FX 8, additive, a ±0.2 square facing the camera 0.2 toward it,
/// a quarter of the frames `0x7f7f4040`, else `0x307f4040`).
pub fn draw(w: &MobyWorld, id: usize, game: &mut crate::game::GameState, cam: V3, out: &mut Vec<FxQuad>) {
    let m = &w.mobys[id];
    let g = &w.level06;
    let slot = (m.pv_i(SLOT).max(0) as usize).min(5);
    for i in 0..6 {
        if g.arc_timers[i] == 0 { continue; }
        let a = g.arc_alpha[i] as u8 as u32;
        strip(out, f32::from_bits(0x3ccc_cccd), 0.5, f32::from_bits(0x3e4c_cccd), &g.arcs[slot][i], a, a, cam);
    }
    let rgba = if game.rng.randi(4) == 0 { 0x7f7f_4040 } else { 0x307f_4040 };
    let c = set_len(sub(cam, m.pos), 1.0);
    let b = set_len(cross(c, [0.0, 0.0, 1.0]), 1.0);
    let a = cross(b, c);
    let p = add(m.pos, scale(c, 0.2));
    let corner = |y: f32, z: f32| add(p, add(scale(b, y * 0.2), scale(a, z * 0.2)));
    out.push(FxQuad { fx: 8, additive: true, corners: [corner(-1.0, 1.0), corner(-1.0, -1.0), corner(1.0, 1.0), corner(1.0, -1.0)], st: ST, rgba: [rgba; 4] });
}

/// `0x45a638(core, glow, ext, pts, a_core, a_glow, n)`: an arc's strip: its core (FX 0xe, white, `core` wide across
/// the line from the camera) over its segments but the last, fading in over the first and out from the one before the
/// last (with four points the second is clear); then its glow (FX 0x10, `0x7f2020`, `glow` wide) over every segment,
/// its ends set at one distance from the camera and pushed `ext` of the segment out, fading in over the first and out
/// over the one before the last.
#[allow(clippy::too_many_arguments)]
fn strip(out: &mut Vec<FxQuad>, core: f32, glow: f32, ext: f32, pts: &[V3; 4], a_core: u32, a_glow: u32, cam: V3) {
    let n = pts.len();
    let side = |i: usize| -> V3 {
        let s = cross(sub(pts[i + 1], pts[i]), sub(pts[i], cam));
        let l = len(s);
        scale(s, core * if l != 0.0 { 1.0 / l } else { 0.0 })
    };
    let full = a_core << 24 | 0xff_ffff;
    let mut col = [full; 4];
    let s = side(0);
    let (mut a, mut b) = (add(pts[0], s), sub(pts[0], s));
    for i in 1..n - 1 {
        let s = side(i);
        let (c, d) = (add(pts[i], s), sub(pts[i], s));
        if i == 1 {
            col[0] = 0xff_ffff;
            col[1] = 0xff_ffff;
        } else if i == n - 2 {
            col[2] = 0xff_ffff;
            col[3] = 0xff_ffff;
        } else if i == 2 {
            col[0] = full;
            col[1] = full;
        }
        out.push(FxQuad { fx: CORE_FX, additive: true, corners: [a, b, c, d], st: ST, rgba: col });
        (a, b) = (c, d);
    }
    let full = a_glow << 24 | 0x7f_2020;
    let mut col = [full; 4];
    for i in 0..n - 1 {
        let (mut e, mut d) = (sub(pts[i], cam), sub(pts[i + 1], cam));
        let (le, ld) = (len(e), len(d));
        if ld < le {
            if ld != 0.0 { d = scale(d, le / ld); }
        } else if le != 0.0 {
            e = scale(e, ld / le);
        }
        let (mut p, mut q) = (add(cam, e), add(cam, d));
        let seg = sub(q, p);
        let x = scale(seg, ext);
        p = sub(p, x);
        q = add(q, x);
        let s = cross(sub(cam, p), seg);
        let l = len(s);
        let s = scale(s, glow * if l != 0.0 { 1.0 / l } else { 0.0 });
        if i == 0 {
            col[0] = 0x7f_2020;
            col[1] = 0x7f_2020;
        } else if i == n - 2 {
            col[2] = 0x7f_2020;
            col[3] = 0x7f_2020;
        } else if i == 1 {
            col[0] = full;
            col[1] = full;
        }
        out.push(FxQuad { fx: GLOW_FX, additive: true, corners: [add(p, s), sub(p, s), add(q, s), sub(q, s)], st: ST, rgba: col });
    }
}
