//! Level 7's icicle **0x12a8** (L07 `0x412000`, 344 placed): hit, it falls and shatters on the ground.
//!
//! Its pvars: `[0]` ≠ 0 it does not fall, `[1]` its gravity a tick², `[2]` the ground's height (0: found under it),
//! `[3]` its length (1 twice, −1 half), `[4..7]` its fall a tick, `[8]` its tip under its place, `[9]` ≠ 0 it is
//! thrown (its fall placed, a second's worth) and hurts.
//!
//! The engine functions: `0x3140c0` the hit (crate::moby::MobyWorld::hit_message), `0x311ae0` set_state,
//! `0x2e5778` = crate::hero::physics::euler_rows, `0x2e4fa0` scale, `0x311c48` randf, `0x3130c8` =
//! crate::moby::ai::ground_z, `0x31c098` a point in its frame, `0x313428` = crate::moby::ai::attack_sphere,
//! `0x356570` its class sound, `0x31dd30` = crate::moby::shards::break_into, `0x3078e8` delete.

use crate::hero::physics::{add, euler_rows, len, scale, V3};
use crate::moby::{ai, Ctx, MobyWorld};

pub const CLASS: i16 = 0x12a8;

/// 0x12a8 (`0x412000`). **0**: mode | 0x100; its rows from its angles, the third ×2 (`[3]` 1) or ×½ (−1); its fall
/// (unless thrown: 3..6 a second down; thrown, a sixtieth of the placed one), its gravity 13..20 a second², its tip
/// −10 × its scale's share × the third row's length, the ground under it; 1. **1**: a hit (flags 0x4010000) with
/// `[0]` 0: 2. **2**: it falls; thrown, a sphere at its tip (twice its scale's share, damage 8, flags 0x4010000);
/// its tip under the ground, 4. **4**: its class sound 0; drawn, its pieces (3..6 / 3..6, mask 3, up 0.05); gone.
pub fn update_12a8(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.mode |= 0x100;
            m.rows = euler_rows(m.rot);
            match m.pv_i(0xc) {
                1 => m.rows[2] = scale(m.rows[2], 2.0),
                -1 => m.rows[2] = scale(m.rows[2], 0.5),
                _ => {}
            }
            if m.pv_i(0x24) == 0 {
                let v = ctx.game.rng.randf(3.0, 6.0);
                let m = &mut w.mobys[id];
                m.set_pv_v(0x10, [0.0, 0.0, -(v * 0.016_666_668)]);
            } else {
                let v = scale(m.pv_v(0x10), f32::from_bits(0x3c88_8889));
                m.set_pv_v(0x10, v);
            }
            let g = ctx.game.rng.randf(13.0, 20.0) * 0.000_277_777_8;
            let share = share(w, id);
            let m = &mut w.mobys[id];
            m.set_pv_f(4, g);
            let tip = share * len(m.rows[2]) * -10.0;
            m.set_pv_f(0x20, tip);
            if m.pv_f(8) == 0.0 {
                let z = ai::ground_z(ctx, 0.5, w.mobys[id].pos, 0);
                w.mobys[id].set_pv_f(8, z);
            }
            w.mobys[id].set_state(1, -1);
        }
        1 => {
            let hit = w.hit_message(id).filter(|h| h.tmpl.flags & 0x401_0000 != 0);
            if hit.is_some() && w.mobys[id].pv_i(0) == 0 { w.mobys[id].set_state(2, -1); }
            w.clear_hit(id);
        }
        2 => {
            let m = &mut w.mobys[id];
            m.pos = add(m.pos, m.pv_v(0x10));
            m.set_pv_f(0x18, m.pv_f(0x18) - m.pv_f(4));
            w.changed = true;
            if w.mobys[id].pv_i(0x24) != 0 {
                let m = &w.mobys[id];
                let r = m.rows;
                let tip: V3 = [0.0, 0.0, m.pv_f(0x20)];
                let p = add(add(add(scale(r[0], tip[0]), scale(r[1], tip[1])), scale(r[2], tip[2])), m.pos);
                let s = share(w, id);
                ai::attack_sphere(w, id, ctx, s + s, 8.0, 1.0, p, 0x401_0000, 0, 1, 0);
            }
            let m = &w.mobys[id];
            if m.pos[2] + m.pv_f(0x20) < m.pv_f(8) { w.mobys[id].set_state(4, -1); }
        }
        4 => {
            ctx.host.play_sound(&w.mobys[id], 0, 0);
            if w.mobys[id].drawn != 0 {
                crate::moby::shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 6.0), 3, [0.0, 0.0, f32::from_bits(0x3d4c_ccce)], 0xff);
            }
            w.delete(id);
        }
        _ => {}
    }
}

/// Its scale over its class's.
fn share(w: &MobyWorld, id: usize) -> f32 {
    let m = &w.mobys[id];
    m.scale / w.class(m.o_class).map_or(1.0, |c| c.scale)
}
