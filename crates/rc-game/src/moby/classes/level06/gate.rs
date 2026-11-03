//! Level 6's gate **0x031a** (L06 `0x3e29d0`; its field `0x3e26d8`): a plane across its frame that hurts what
//! crosses it while any of five watched mobys lives, then lets its target go (a state set, a fade, its deletion).
//!
//! Its pvars: `+0x00` / `+0x04` the plane's width (across) and height (up), `+0x08` its flags, `+0x0c` the fade's
//! ticks, `+0x10` its target moby (−1 none; its class kept at `+0x14`), the state its target takes on its release
//! `+0x16` and the target's state that ends it `+0x17` (−1 none), five watched mobys at `+0x18` (8 each: the moby, its
//! class at `+4`, a state that counts as gone at `+6`, −1 none), the fade's timer `+0x40`, the target's alpha `+0x44`,
//! the field's reach past the plane's sides and top `+0x48` / `+0x4c` (the first also its sphere's radius), its
//! damage `+0x50`.
//!
//! Its flags: 1 its target kept at the end, 2 no fade, 4 the target's end state not watched, 8 no timer; 0x10 the
//! target to its place (else 0x40 itself to the target's), 0x20 its angles from the target's (else 0x80 the target's
//! from its), 0x100 no field.
//!
//! The engine functions: `0x320c98` = crate::collision::coll_sphere_mobys (flags 0x10), `0x35d838` a hit on a moby
//! (crate::moby::MobyWorld::deliver_hit_any), `0x32ec90` = crate::hero::physics::euler_rows.
//!
//! Not ported: on level 0xd its first row cut to half its length each tick (`0x32e710`; this is level 6's).

use crate::hero::physics::{add, dot, euler_rows, scale, V3};
use crate::moby::{countdown, Ctx, MobyWorld};

pub const CLASS: i16 = 0x031a;

fn moby_at(w: &MobyWorld, i: i32) -> Option<usize> { usize::try_from(i).ok().filter(|&k| k < w.mobys.len()) }

/// The watched mobys still there: of their class, not deleted, not gone in the save (their uid's count 0), not in
/// their gone state.
fn alive(w: &MobyWorld, id: usize, ctx: &Ctx) -> i32 {
    let m = &w.mobys[id];
    let mut n = 0;
    for k in 0..5 {
        let o = 0x18 + 8 * k;
        let Some(t) = moby_at(w, m.pv_i(o)).map(|t| &w.mobys[t]) else { continue };
        if t.o_class != m.pv_s(o + 4) || t.state == 0xfe || t.state == 0xfd || ctx.game.uid_count(t.uid) != 0 { continue; }
        let gone = m.pv_b(o + 6) as i8;
        if gone == -1 || t.state != gone as u8 { n += 1; }
    }
    n
}

/// 0x031a (`0x3e29d0`). The count of its watched mobys alive (none once its mission is done). **0**: its target's
/// class and alpha kept, the places and angles shared (its flags), the watched mobys' classes kept; drawn as large as
/// the plane's longer side; 1. **1**: none alive: its target to its release state, the fade's timer, 2. **2**: the
/// target faded with the timer (unless flag 2); the target in its end state (unless flag 4) or the timer out (unless
/// flag 8): 3. **3**: the target deleted (unless flag 1), itself gone. In 1 and 2 the field ([`field`]).
pub fn update_031a(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let target = moby_at(w, m.pv_i(0x10));
    let done = m.mission != 0xff && ctx.game.missions.get(ctx.game.level.max(0) as usize * 0x10 + m.mission as usize).is_some_and(|b| b & 0x80 != 0);
    let n = if !done && m.state != 0 { alive(w, id, ctx) } else { 0 };
    let flags = w.mobys[id].pv_i(8) as u32;
    match w.mobys[id].state {
        0 => {
            if let Some(t) = target {
                let (c, a) = (w.mobys[t].o_class, w.mobys[t].alpha);
                let m = &mut w.mobys[id];
                m.set_pv_s(0x14, c);
                m.set_pv_f(0x44, a as f32);
                if flags & 0x10 != 0 {
                    w.mobys[t].pos = w.mobys[id].pos;
                } else if flags & 0x40 != 0 {
                    w.mobys[id].pos = w.mobys[t].pos;
                }
                if flags & 0x20 != 0 {
                    w.mobys[id].rot = w.mobys[t].rot;
                } else if flags & 0x80 != 0 {
                    w.mobys[t].rot = w.mobys[id].rot;
                }
            }
            for k in 0..5 {
                let o = 0x18 + 8 * k;
                if let Some(t) = moby_at(w, w.mobys[id].pv_i(o)) {
                    let c = w.mobys[t].o_class;
                    w.mobys[id].set_pv_s(o + 4, c);
                }
            }
            let s = w.class(CLASS).map_or(1.0, |c| c.scale);
            let m = &mut w.mobys[id];
            m.state = 1;
            m.mode |= 0x100;
            m.scale = m.pv_f(0).max(m.pv_f(4)) * s;
            w.changed = true;
            return;
        }
        1 => {
            if n == 0 {
                let s = w.mobys[id].pv_b(0x16) as i8;
                if let Some(t) = target.filter(|_| s != -1) { w.mobys[t].state = s as u8; }
                let m = &mut w.mobys[id];
                let l = m.pv_i(0xc);
                m.set_pv_i(0x40, l);
                m.state = 2;
            }
        }
        2 => {
            if let Some(t) = target.filter(|_| flags & 2 == 0) {
                let m = &w.mobys[id];
                let a = m.pv_f(0x44) * m.pv_i(0x40) as f32 / m.pv_i(0xc) as f32;
                w.mobys[t].alpha = a as u8;
            }
            if flags & 4 == 0 {
                let e = w.mobys[id].pv_b(0x17) as i8;
                if target.is_some_and(|t| e != -1 && w.mobys[t].state == e as u8) { w.mobys[id].state = 3; }
            }
            if flags & 8 == 0 {
                let mut t = w.mobys[id].pv_i(0x40);
                let out = countdown(&mut t);
                w.mobys[id].set_pv_i(0x40, t);
                if out { w.mobys[id].state = 3; }
            }
        }
        3 => {
            if let Some(t) = target.filter(|_| flags & 1 == 0) { w.delete(t); }
            w.delete(id);
            return;
        }
        _ => {}
    }
    field(w, id, ctx);
}

/// `0x3e26d8` (unless flag 0x100): each live moby within half the plane's longer side of its middle (`0x320c98`,
/// flags 0x10) is put on the plane (its place across and up held to the plane's sides once past them by more than
/// the reach); one within the reach of that point (the same query) takes a hit of its damage (flags 0x10001).
fn field(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let flags = m.pv_i(8) as u32;
    let Some(sc) = ctx.scene.filter(|_| flags & 0x100 == 0) else { return };
    let r = euler_rows(m.rot);
    let through = |v: V3| -> V3 { add(add(scale(r[0], v[0]), scale(r[1], v[1])), scale(r[2], v[2])) };
    let (wd, ht, reach, top, dmg, pos) = (m.pv_f(0), m.pv_f(4), m.pv_f(0x48), m.pv_f(0x4c), m.pv_f(0x50), m.pos);
    let half = wd.max(ht) * 0.5;
    let mid = add(through([0.0, 0.0, half]), pos);
    let near = crate::collision::coll_sphere_mobys(sc, mid, half, crate::collision::QueryFlags(0x10), Some(id));
    let hit = crate::moby::hits::HitTemplate::new(dmg, Some(id), 0x1_0001, [0.0; 4]);
    for t in near {
        let Some(o) = w.mobys.get(t).filter(|o| o.state != 0xfe && o.state != 0xfd) else { continue };
        let d = crate::hero::physics::sub(o.pos, pos);
        let mut l = [dot(d, r[0]), 0.0, dot(d, r[2])];
        if l[0] < -(wd * 0.5 + reach) { l[0] = -wd * 0.5 } else if wd * 0.5 + reach < l[0] { l[0] = wd * 0.5 }
        if l[2] < -(ht * 0.5 + top) { l[2] = -ht * 0.5 } else if ht * 0.5 + top < l[2] { l[2] = ht * 0.5 }
        let p = add(through(l), pos);
        if crate::collision::coll_sphere_mobys(sc, p, reach, crate::collision::QueryFlags(0x10), Some(id)).contains(&t) {
            w.deliver_hit_any(t, &hit, [p[0], p[1], p[2], 1.0]);
        }
    }
}
