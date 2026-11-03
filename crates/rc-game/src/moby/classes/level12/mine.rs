//! Level 12's proximity mine 0x0fab (L12 addresses; the engine functions named by their L12 addresses with the L00
//! ones the port knows them by where they differ).
//!
//! The engine functions: `0x2f2718` the xy distance, `0x321700` = L00 `0x31dce8` the hit test, `0x32ead0` = L00
//! `0x32a7c8` (crate::moby::Moby::first), `0x365ed8` its class sound, `0x2eac38` its draw ([`draw`]), `0x322790` = L00
//! `0x31ed78` (crate::camera::approach), `0x322530` = L00 `0x31eb18` (a speed toward a goal by a step), `0x2f2190`
//! (crate::moby::countdown), `0x3407b8` = L00 `0x33b5e0` (crate::particles::glint), `0x2f36e8` the colour lerp
//! (crate::hud::lerp_rgba), `0x2e1e20` the line query (crate::collision::coll_line_m) with `0x320b58`'s hit.
//!
//! Not ported: the class 0xff6 piece it throws as it goes off (`0x32ad40(0, m, 0xff6, ..)`: 1/60..2/60 a tick across,
//! 5/60..8/60 up; the class is in no level's list the port reads), and the Clank and Spiderbot checks (`0x3df858` /
//! `0x405bc0` with their mobys `$gp+0x18cc` / `0x1b1320`: either within 2.5 sets it off, and the ring hurts them,
//! `0x3e0d10(1, 0)` / `0x407578(1, 0)`); the port has neither mode.

use crate::hero::physics::{add, add_rot, dist2, scale, V3};
use crate::moby::{countdown, mode, Ctx, MobyWorld};

/// `$gp+0x2c38`: the ring's radius to start.
const RING_START: f32 = 0.4;
/// `$gp+0x2c3c`: Ratchet's distance that sets it off.
const RANGE: f32 = 5.0;
/// `$gp+0x2c40`: the ring's last radius.
const RING_END: f32 = 8.0;
/// `$gp+0x2c44`: the fuse, seconds.
const FUSE: f32 = 1.0;
/// `$gp+0x2c48`: the ring's growth, a second.
const RING_SPEED: f32 = 8.0;
/// `$gp+0x2c4c` / `0x2c50`: the ring's colours (full, faded).
const RING_RGBA: [u32; 2] = [0x8080_6000, 0x0080_0060];
/// `$gp+0x2c54` / `0x2c58`: the sparks' colours.
const SPARK_RGBA: [u32; 2] = [0x8080_6000, 0x8080_0060];
/// `$gp+0x2c5c..0x2c60`: the sparks' sizes.
const SPARK_SIZE: (f32, f32) = (0.2, 0.4);
/// `$gp+0x2c64`: the burst's sparks' elevation, ± degrees.
const BURST_ELEV: f32 = 5.0;
/// `$gp+0x2c68`: the burst's sparks.
const BURST: i32 = 100;
/// `$gp+0x2c78`: the ring's FX texture.
const RING_FX: usize = 0x17;
/// 30°, the ring's segment (`0x3f060a93`).
const SEG: f32 = f32::from_bits(0x3f06_0a93);

/// 0x0fab (`0x41bbf8`, 23 placed): a mine. Its pvars: `+0x50` its home, `[0x60]` its rise speed, `[0x64]` its spin,
/// `[0x68]` its fuse, `[0x6c]` the ring's radius, `[0x70]` the ring's phase; the byte `[0x34]` its damage. **0**: its
/// home its place; 1. **1**: Ratchet within 5 (xy), or a hit with the flags 0x30000 → 2: the fuse 60 ticks, the ring
/// 0.4, its phase 0. **2**: its class sound 0 on entry; drawn ([`draw`]); the phase on by 8/60; its height eased to 0.5
/// over its home (`0x322790(z, 1, 1/120, 1)`); its spin toward 0.419 by 0.1 a tick, onto its yaw; a spark
/// ([`spark`]: 15° up or down, its life 7 × 1..1.5); the fuse out → 3: the fuse 20, 100 sparks (5° up or down, each
/// living as long as the ring takes to reach its end × 1..1.5). **3**: hidden (no target, mode | 0x41, no collision);
/// while the ring is within 8: drawn, the ring and its phase on by 8/60, and 12 lines around it (flags 9) each hitting
/// what it meets for the damage byte (flags 0x10001). Past 8 it stays (`$gp+0x2c6c` 0: no return to its home).
pub fn update_0fab(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let step = RING_SPEED * 0.016_666_668;
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            let p = m.pos;
            for k in 0..3 { m.set_pv_f(0x50 + 4 * k, p[k]); }
            m.state = 1;
        }
        1 => {
            let near = dist2(w.mobys[id].pos, ctx.hero.pos) < RANGE;
            // `0x321700(m, 0x30000, 0)`: its record when the flags match (a record without them is dropped).
            let hit = match w.hit_message(id) {
                Some(h) if h.tmpl.flags & 0x30000 != 0 => true,
                Some(_) => { w.clear_hit(id); false }
                None => false,
            };
            if near || hit {
                let m = &mut w.mobys[id];
                m.state = 2;
                m.set_pv_i(0x68, (FUSE * 60.0) as i32);
                m.set_pv_f(0x6c, RING_START);
                m.set_pv_f(0x70, 0.0);
            }
        }
        2 => {
            if w.mobys[id].first() { ctx.host.play_sound(&w.mobys[id], 0, 0); }
            w.register_draw(id);
            let m = &mut w.mobys[id];
            m.set_pv_f(0x70, m.pv_f(0x70) + step);
            let (mut z, mut v) = (m.pos[2], m.pv_f(0x60));
            crate::camera::approach(m.pv_f(0x58) + 0.5, 1.0, f32::from_bits(0x3c08_8889), 1.0, &mut z, &mut v);
            m.pos[2] = z;
            m.set_pv_f(0x60, v);
            let s = m.pv_f(0x64);
            let s = s + (f32::from_bits(0x3ed6_7752) - s).clamp(-0.1, 0.1);
            m.set_pv_f(0x64, s);
            m.rot[2] = add_rot(m.rot[2], s);
            spark(w, id, ctx, 15.0, 7.0);
            let mut fuse = w.mobys[id].pv_i(0x68);
            let out = countdown(&mut fuse);
            w.mobys[id].set_pv_i(0x68, fuse);
            if out {
                w.mobys[id].state = 3;
                w.mobys[id].set_pv_i(0x68, 0x14);
                for _ in 0..BURST { spark(w, id, ctx, BURST_ELEV, RING_END / step); }
            }
        }
        3 => {
            let m = &mut w.mobys[id];
            m.has_collision = false;
            m.mode = (m.mode & !mode::TARGET) | 0x41;
            if RING_END < m.pv_f(0x6c) { return; }
            w.register_draw(id);
            let m = &mut w.mobys[id];
            let r = m.pv_f(0x6c) + step;
            m.set_pv_f(0x6c, r);
            m.set_pv_f(0x70, m.pv_f(0x70) + step);
            let hit = crate::moby::hits::HitTemplate::new(m.pv_b(0x34) as f32, Some(id), 0x1_0001, [0.0; 4]);
            let pos = m.pos;
            let at = |a: f32| add(pos, [a.cos() * r, a.sin() * r, 0.0]);
            let (mut prev, mut a) = (at(0.0), SEG);
            for _ in 0..12 {
                let p = at(a);
                let o = ctx.coll.and_then(|c| crate::collision::coll_line_m(c, ctx.scene, p, prev, crate::collision::QueryFlags(9), Some(id)));
                if let Some(o) = o {
                    if let Some(t) = o.moby { w.deliver_hit_any(t, &hit, [o.point[0], o.point[1], o.point[2], 1.0]); }
                }
                a = add_rot(a, SEG);
                prev = p;
            }
        }
        _ => {}
    }
}

/// One spark (`0x3407b8`): from its place, `randf(−e, e)`° up or down at a random heading, 8/60 a tick; its colour
/// lerped between the two at `randf(0, 1)`, its size `randf(0.2, 0.4)`, its life `life × randf(1, 1.5)`, a random roll.
fn spark(w: &MobyWorld, id: usize, ctx: &mut Ctx, e: f32, life: f32) {
    let rng = &mut ctx.game.rng;
    let el = rng.randf(-e, e) * 0.017_453_294;
    let (s, c) = (el.sin(), el.cos());
    let h = rng.rand_angle();
    let v = scale([h.cos() * c, h.sin() * c, s], RING_SPEED * 0.016_666_668);
    let t = rng.randf(0.0, 1.0);
    let rgba = crate::hud::lerp_rgba(t, SPARK_RGBA[0], SPARK_RGBA[1]);
    let size = rng.randf(SPARK_SIZE.0, SPARK_SIZE.1);
    let life = (rng.randf(1.0, 1.5) * life) as i32 as u16;
    let rot = rng.randi(0xff) as u8;
    let (pos, hero) = (w.mobys[id].pos, ctx.hero.pos);
    if let Some(sys) = ctx.parts.as_deref_mut() { crate::particles::glint(sys, size, pos, v, rgba, life, rot, false, hero); }
}

/// `0x41c358`: the ring, three bands of 12 quads (FX 0x17, additive) about its place in its frame, each turned 30° on
/// from the last; band `i` 10° × `i` on and `0.05 × i` inside the radius, its height ± its phase / 2 (the ring's phase
/// plus a third for each band, wrapped), its colour the faded one lerped toward the full one by (1 − its phase), all
/// faded as the ring nears its end (`(8 − r) / 2`, within 0..1).
pub fn draw(w: &MobyWorld, id: usize, out: &mut Vec<crate::fx::FxQuad>) {
    let m = &w.mobys[id];
    let (r, p) = (m.pv_f(0x6c), m.pv_f(0x70));
    let fade = ((RING_END - r) * 0.5).clamp(0.0, 1.0);
    let mut phase = p - p as i32 as f32;
    let mut bands = [([[0.0; 3]; 4], 0u32); 3];
    for (i, b) in bands.iter_mut().enumerate() {
        let a = i as f32 * 0.174_532_95;
        let a2 = add_rot(a, SEG);
        let k = r - i as f32 * 0.05;
        let h = phase * 0.5;
        let (x0, y0, x1, y1) = (a.cos() * k, a.sin() * k, a2.cos() * k, a2.sin() * k);
        b.0 = [[x0, y0, h], [x0, y0, -h], [x1, y1, h], [x1, y1, -h]];
        b.1 = crate::hud::lerp_rgba((1.0 - phase) * fade, RING_RGBA[1], RING_RGBA[0]);
        phase += if 0.666 <= phase { -0.666 } else { 0.333 };
    }
    let (rows, pos) = (m.rows, m.pos);
    for n in 1..=12 {
        let (s, c) = (n as f32 * SEG).sin_cos();
        let world = |l: V3| {
            let (x, y) = (l[0] * c - l[1] * s, l[0] * s + l[1] * c);
            add(pos, add(add(scale(rows[0], x), scale(rows[1], y)), scale(rows[2], l[2])))
        };
        for (q, rgba) in &bands {
            out.push(crate::fx::FxQuad {
                fx: RING_FX,
                additive: true,
                corners: q.map(world),
                st: [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]],
                rgba: [*rgba; 4],
            });
        }
    }
}
