//! Level 12's force field 0x0f00 (L12 addresses; the engine functions named by their L12 addresses with the L00 ones
//! the port knows them by where they differ).
//!
//! The engine functions: `0x32eb40` its hook (crate::moby::Moby::once & 4), `0x2e4a58` the sphere's mobys
//! (crate::collision::coll_sphere_mobys, flags 0x10), `0x321020` a hit on a moby (crate::moby::MobyWorld::
//! deliver_hit_any), `0x316d58` = L00 `0x314718` its ambient, `0x365ed8` / `0x365bd8` its class sound and the stop.
//!
//! Not ported: the scroll of its class's model's texture (`0x3154d8(class, d, 0)` every other frame: `d` 0xc0 added to
//! each packet's u, wrapped at 0x1000, `+0x68`[0xc]); the port's models have no such scroll.

use crate::hero::physics::{add, dot, euler_rows, scale, sub, V3};
use crate::moby::{Ctx, MobyWorld};

/// Its looped sound's handle (`+0x68`[0x10]).
const SOUND: usize = 0x10;

fn uid_done(w: &MobyWorld, id: usize, ctx: &Ctx) -> bool {
    let uid = w.mobys[id].uid;
    ctx.game.uid_dead.get(uid.max(0) as usize).is_some_and(|&d| d != 0) || ctx.game.uid_count(uid) != 0
}

fn stop_sound(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let h = w.mobys[id].pv_i(SOUND);
    if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
    w.mobys[id].set_pv_i(SOUND, -1);
}

/// 0x0f00 (`0x413cf0`, two placed): a force field, a plane `+0x68`[8] (s16) across and `[0xa]` (s16) high on its
/// x–z, that hits what touches it; `[0]` the moby it stands for (an index, −1 none), `[4]` its strength. Hooked, or its
/// uid marked: the flag byte `0x1395da` set, its uid counted, its sound stopped, gone. **0**: at full alpha (0x14),
/// mode | 0xa08, its strength 1, its ambient 0x80; 1. **1**: its class sound 0 looped; its moby gone (deleted, or its
/// uid marked): its strength 1, its uid counted (it stays off), 2; the field's hits ([`zap`]). **2**: its strength
/// down by `$gp+0x2780` (0.05) a tick, its alpha 20 × it; below 0: 3. **3**: its sound stopped, gone.
pub fn update_0f00(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].once & 4 != 0 || uid_done(w, id, ctx) {
        if let Some(f) = ctx.game.flags.get_mut(0x22) { *f = 1; }
        let uid = w.mobys[id].uid;
        ctx.game.count_uid(uid);
        stop_sound(w, id, ctx);
        return w.delete(id);
    }
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.state = 1;
            m.alpha = 0x14;
            m.mode |= 0xa08;
            m.set_pv_i(SOUND, -1);
            m.set_pv_f(4, 1.0);
            m.ambient = [0x80; 3];
        }
        1 => {
            let h = w.mobys[id].pv_i(SOUND);
            if !(h != -1 && ctx.host.sound_playing(h)) {
                let h = ctx.host.play_sound(&w.mobys[id], 0, 4);
                w.mobys[id].set_pv_i(SOUND, h);
            }
            let other = usize::try_from(w.mobys[id].pv_i(0)).ok();
            if let Some(o) = other {
                let gone = w.mobys.get(o).is_none_or(|om| 0xfd <= om.state) || ctx.game.uid_count(w.mobys[o].uid) != 0;
                if gone {
                    let m = &mut w.mobys[id];
                    m.set_pv_f(4, 1.0);
                    m.state = 2;
                    let uid = m.uid;
                    ctx.game.count_uid(uid);
                }
            }
            zap(w, id, ctx);
        }
        2 => {
            let m = &mut w.mobys[id];
            let f = m.pv_f(4) - 0.05;
            m.set_pv_f(4, f);
            if f < 0.0 { m.state = 3 } else { m.alpha = (f * 20.0) as i32 as u8 }
        }
        3 => {
            stop_sound(w, id, ctx);
            w.delete(id);
        }
        _ => {}
    }
}

/// `0x4139a0`: the mobys within half its height or width (the larger) of its middle (half its height up its z row);
/// for each, its place in the field's frame held to the plane (across within ±half the width when more than 0.7
/// outside it, up within ±half the height when more than 1.5 outside, on the plane): within 0.7 of that point it
/// takes a hit of 1 (flags 0x10001).
fn zap(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let Some(sc) = ctx.scene else { return };
    let m = &w.mobys[id];
    let rows = euler_rows(m.rot);
    let pos = m.pos;
    let (wd, ht) = (m.pv_s(8) as f32, m.pv_s(0xa) as f32);
    let r = wd.max(ht) * 0.5;
    let world = |l: V3| add(pos, add(add(scale(rows[0], l[0]), scale(rows[1], l[1])), scale(rows[2], l[2])));
    let middle = world([0.0, 0.0, r]);
    let near = crate::collision::coll_sphere_mobys(sc, middle, r, crate::collision::QueryFlags(0x10), Some(id));
    for t in near {
        let Some(tm) = w.mobys.get(t).filter(|tm| tm.state < 0xfd) else { continue };
        let d = sub(tm.pos, pos);
        let mut l = [dot(d, rows[0]), 0.0, dot(d, rows[2])];
        let (hw, hh) = (wd * 0.5, ht * 0.5);
        if l[0] < -(hw + 0.7) { l[0] = -hw } else if hw + 0.7 < l[0] { l[0] = hw }
        if l[2] < -(hh + 1.5) { l[2] = -hh } else if hh + 1.5 < l[2] { l[2] = hh }
        let p = world(l);
        if crate::collision::coll_sphere_mobys(sc, p, f32::from_bits(0x3f33_3333), crate::collision::QueryFlags(0x10), Some(id)).contains(&t) {
            let hit = crate::moby::hits::HitTemplate::new(1.0, Some(id), 0x1_0001, [0.0; 4]);
            w.deliver_hit_any(t, &hit, [p[0], p[1], p[2], 1.0]);
        }
    }
}
