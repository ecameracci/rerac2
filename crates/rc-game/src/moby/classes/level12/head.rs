//! Level 12's watching head 0x086a (L12 addresses; the engine functions named by their L12 addresses with the L00
//! ones the port knows them by where they differ).
//!
//! The engine functions: `0x2f2bf8` the angle of (x, y) (crate::hero::physics::atan2), `0x2f3590` / `0x2f3548`
//! (sub_rot / add_rot), `0x2f2718` the xy distance, `0x322c48` = L00 `0x31f230` (turn_ease), `0x327a50` a joint list's
//! turn ([`aim`]), `0x321700` = L00 `0x31dce8` the hit test, `0x32b088` = L00 `0x327898`
//! (crate::moby::classes::props::spawn_in_place), `0x365ed8` / `0x365bd8` the class sound and its stop, `0x31eff8` its
//! sequence, `0x3177c8` the rebuild.

use crate::hero::physics::{add_rot, atan2, dist2, euler_rows, sub_rot, turn_spring};
use crate::moby::{Ctx, Moby, MobyWorld};

/// Its looped sound's handle (`+0x68`[0x114]).
const SOUND: usize = 0x114;
/// The hit flags it answers.
const BREAK_FLAGS: u32 = 0x0401_0000;

fn set_manip(m: &mut Moby, list: u8, q: Option<crate::quat::Q>) {
    m.manips.retain(|x| x.0 != list);
    if let Some(q) = q { m.manips.push((list, q)); }
}

/// `0x327a50(k, d, m, block, list)`: the block's angles (`+0x40..`, their speeds `+0x50..`) spring toward the turn
/// asked for (`+0x60..`, `0x322ae0`: k, damping d) and turn joint list `list` of `m` by them (its node `+0x10`); asked
/// for nothing and all back within 0.005, the node goes. The ask is cleared after.
fn aim(m: &mut Moby, b: usize, list: u8) {
    let t: [f32; 3] = std::array::from_fn(|k| m.pv_f(b + 0x60 + 4 * k));
    let a0: [f32; 3] = std::array::from_fn(|k| m.pv_f(b + 0x40 + 4 * k));
    if t == [0.0; 3] && a0.iter().all(|a| a.abs() < 0.005) {
        set_manip(m, list, None);
        return;
    }
    let mut a = [0.0; 3];
    for k in 0..3 {
        let (mut x, mut v) = (m.pv_f(b + 0x40 + 4 * k), m.pv_f(b + 0x50 + 4 * k));
        turn_spring(t[k], 0.03, 0.3, 0.0, &mut x, &mut v, 0);
        m.set_pv_f(b + 0x40 + 4 * k, x);
        m.set_pv_f(b + 0x50 + 4 * k, v);
        m.set_pv_f(b + 0x60 + 4 * k, 0.0);
        a[k] = x;
    }
    set_manip(m, list, Some(crate::quat::from_rows(&euler_rows(a))));
}

fn stop_sound(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let h = w.mobys[id].pv_i(SOUND);
    if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
    w.mobys[id].set_pv_i(SOUND, -1);
}

/// 0x086a (`0x3bfaa8`, three placed): a head that follows Ratchet. Its pvars: two joint blocks (`+0x00` list 0, `+0x80`
/// list 1), `[0x100]` / `[0x104]` its turn and nod and `[0x108]` / `[0x10c]` their speeds, `[0x110]` ≠ 0 unbreakable,
/// `[0x114]` its looped sound. Every tick mode | 4. **0**: 1. **1**: Ratchet within 16 (xy) → 2 and its sequence 1
/// (blend 30). **2**: its turn toward him (his angle less its yaw, plus 90°) and its nod toward his height (the angle of
/// the xy distance and its height over his); past 20 → 1 and its sequence 0. In 1 and 2 a hit with the flags 0x4010000
/// (unless unbreakable) → 3, the hit cleared. **3**: a 0x1132 in its place (`0x32b088`, its update distance its own),
/// its class sound 0, its sound stopped, gone. Then (not 3): its looped sound 1 while its turn is more than
/// `$gp+0xc34` (1°) off what it asks, else stopped; the turn and nod eased (1/576 up and down, at most 6°); both joint
/// lists turned by half of each ([`aim`]: x the nod, z the turn); rebuilt, its sphere's radius `$gp+0xc30` (3100).
pub fn update_086a(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].mode |= 4;
    let (mut turn, mut nod) = (0.0, 0.0);
    let hero = ctx.hero.pos;
    let state = w.mobys[id].state;
    match state {
        0 => w.mobys[id].set_state(1, -1),
        1 | 2 => {
            let m = &w.mobys[id];
            let d = dist2(m.pos, hero);
            if state == 1 {
                if d < 16.0 {
                    w.mobys[id].state = 2;
                    if ctx.host.anim(&w.mobys[id]).is_none_or(|a| a.seq_b != 1) { ctx.host.set_anim(&w.mobys[id], 1, 0, 0x1e); }
                }
            } else {
                let to = atan2(hero[0] - m.pos[0], hero[1] - m.pos[1]);
                turn = add_rot(sub_rot(to, m.rot[2]), f32::from_bits(0x3fc9_0fdc));
                nod = atan2(d, m.pos[2] - hero[2]);
                if 20.0 < d {
                    w.mobys[id].state = 1;
                    if ctx.host.anim(&w.mobys[id]).is_none_or(|a| a.seq_b != 0) { ctx.host.set_anim(&w.mobys[id], 0, 0, 0x1e); }
                }
            }
            if w.hit_message(id).is_some_and(|h| h.tmpl.flags & BREAK_FLAGS != 0) && w.mobys[id].pv_i(0x110) == 0 {
                w.mobys[id].set_state(3, -1);
            }
            w.clear_hit(id);
        }
        3 => {
            let u = w.mobys[id].update_dist;
            crate::moby::classes::props::spawn_in_place(w, id, 0x1132, u, &*ctx.host);
            ctx.host.play_sound(&w.mobys[id], 0, 0);
            stop_sound(w, id, ctx);
            return w.delete(id);
        }
        _ => {}
    }
    let m = &w.mobys[id];
    if 1.0_f32.to_radians() < sub_rot(turn, m.pv_f(0x100)).abs() {
        let h = m.pv_i(SOUND);
        if !(h != -1 && ctx.host.sound_playing(h)) {
            let h = ctx.host.play_sound(&w.mobys[id], 1, 4);
            w.mobys[id].set_pv_i(SOUND, h);
        }
    } else {
        stop_sound(w, id, ctx);
    }
    let m = &mut w.mobys[id];
    let (accel, max) = (f32::from_bits(0x3ae4_c38a), f32::from_bits(0x3dd6_7752));
    for (t, a, v) in [(turn, 0x100, 0x108), (nod, 0x104, 0x10c)] {
        let (mut x, mut s) = (m.pv_f(a), m.pv_f(v));
        crate::hero::physics::turn_ease(t, accel, accel, max, &mut x, &mut s, 0);
        m.set_pv_f(a, x);
        m.set_pv_f(v, s);
    }
    let (n, t) = (m.pv_f(0x104) * 0.5, m.pv_f(0x100) * 0.5);
    for b in [0, 0x80] {
        m.set_pv_f(b + 0x60, n);
        m.set_pv_f(b + 0x68, t);
    }
    aim(m, 0, 0);
    aim(m, 0x80, 1);
    w.rebuild(id, &*ctx.host);
    w.mobys[id].bsphere[3] = 3100.0;
}
