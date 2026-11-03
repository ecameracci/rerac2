//! Level 11's sparked stalks **0x10ea** / **0x118c** (`0x4713b0`, 48 + 20 placed; no Ghidra function: read from the
//! disassembly, `0x4713b0..0x471c54`, and its draw `0x471c58`): up to eight sparks circle a point 18 (0x118c: 21) up
//! its third row; shot, it breaks into its pieces, explodes, leaves its stump (0x10eb / 0x118d) smoking and its
//! sparks drift away.
//!
//! Its pvars: `+0x00` unbreakable (≠ 0), `+0x04` / `+0x08` its two glows (particle + 1 here), `+0x0c` the smoke's wait,
//! `+0x10` how many sparks (at most 8), `+0x14` the sparks' drift, `+0x18` their brightness (1 → 0), `+0x20..` the
//! sparks (16 bytes each).
//! * **0**: brightness 1; its two glows (`0x353718`, crate::particles::glow_def: 315000 across, at `(0, 0, 18)`·its
//!   scale in its frame, while it lives) of 0x1fffffff (frame 0) and 0x3f106060 (frame 1); each spark at
//!   `randf(0.75, 1.25)` a random yaw and pitch from its place; **1**.
//! * **1**: its draw ([`draw`]); each spark turned 3° a tick about the centre (its place + 18 up the third row × its
//!   scale, `0x338260`), its pitch drifted 1..3° either way (`0x32ddc0`) and held within ±1.1, its distance drifted by
//!   a sixtieth either way and held to 0.75..1.05. A hit with flags 0x04010000 (`0x330218`) and not unbreakable: **2**.
//!   Its hit record cleared every tick.
//! * **2** (the break): its pieces thrown (`0x33e310`, crate::moby::shards::throw_all: 3 a second either way across,
//!   7..9 up, spin 90..360, gravity 10..20, its class sound 0, the level's bolts); 80 puffs of kind 3 (`0x2ff1d0`,
//!   crate::particles::staged_burst: spread 0.1, speed 0.15, size 1, fall 20, life 1) at `(0, 0, 18 | 21)`; the
//!   explosion (`0x332b08`, crate::moby::blast::explode: no hit, the second flash 1 past 9, size 0.5, 10 embers, 3
//!   billows, 5 flares) at the centre; its glows killed (`0x344d78`); its stump made (`0x339c38`: its class, at its
//!   place and angles, its scale ratio, its light, update distance 0, drawn); itself hidden without collision, the
//!   smoke's wait 120, the drift 600, **3**.
//! * **3**: the smoke — in the first 30 ticks and then every 45..90 — two bursts of 3..6 puffs of kinds 0 and 1 (speed
//!   1/30, size 0.5, fall 15, life 1) at `(0, 0, 17 | 19)`, moving `(±1/30, 0, 1/30 | 0.04)` in its frame. While the
//!   drift lasts its draw, each spark moving 1/30 a tick out from its place across, turned by the drift left × π/1200,
//!   a random way, up by −0.25..0.75; its last 60 ticks fading. Then it stays.
//!
//! The draw (`0x471c58`, the second list): each spark two FX 0x31 sprites facing the camera, pushed 0.08 toward it,
//! at random rolls, additive: 0.2 across in 0x106060 at alpha 63 × brightness, 0.08 across in 0x808080 at 128 ×.

use crate::hero::physics::{add, atan2, dist, dist2, scale, set_len, sub, sub_rot, V3};
use crate::moby::{countdown, Ctx, MobyWorld};

const GLOW_A: usize = 0x04;
const GLOW_B: usize = 0x08;
const SMOKE: usize = 0x0c;
const COUNT: usize = 0x10;
const DRIFT: usize = 0x14;
const BRIGHT: usize = 0x18;
const SPARKS: usize = 0x20;

/// `0x337298(r, yaw, pitch, out)`.
fn polar(r: f32, yaw: f32, pitch: f32) -> V3 { [yaw.cos() * r * pitch.cos(), yaw.sin() * r * pitch.cos(), pitch.sin() * r] }

/// `v` through the rows.
fn through(v: V3, r: &[V3; 3]) -> V3 { std::array::from_fn(|k| v[0] * r[0][k] + v[1] * r[1][k] + v[2] * r[2][k]) }

/// `0x338260(x, y, z, m, out)`: `(x, y, z)` in its frame (its rows × its scale, at its place).
fn local(w: &MobyWorld, id: usize, v: V3) -> V3 {
    let m = &w.mobys[id];
    add(scale(through(v, &m.rows), m.scale), m.pos)
}

fn count(w: &MobyWorld, id: usize) -> usize { w.mobys[id].pv_i(COUNT).clamp(0, 8) as usize }

/// 0x10ea / 0x118c (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let big = w.mobys[id].o_class == 0x118c;
    let (height, stump, smoke_up) = if big { (21.0, 0x118d, 19.0) } else { (18.0, 0x10eb, 17.0) };
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.set_pv_f(BRIGHT, 1.0);
            if 8 < m.pv_i(COUNT) { m.set_pv_i(COUNT, 8); }
            let offset = [0.0, 0.0, height * m.scale];
            let (pos, far) = (m.pos, m.draw_dist);
            if let Some(sys) = ctx.parts.as_deref_mut() {
                let rng = &mut ctx.game.rng;
                let a = crate::particles::glow_def(sys, rng, 315_000.0, 0.0, offset, id, pos, far, false, 0x1fff_ffff, -1, 0);
                let b = crate::particles::glow_def(sys, rng, 315_000.0, 0.0, offset, id, pos, far, false, 0x3f10_6060, -1, 1);
                let m = &mut w.mobys[id];
                m.set_pv_i(GLOW_A, a.map_or(0, |i| i as i32 + 1));
                m.set_pv_i(GLOW_B, b.map_or(0, |i| i as i32 + 1));
            }
            for k in 0..count(w, id) {
                let rng = &mut ctx.game.rng;
                let r = rng.randf(0.75, 1.25);
                let (yaw, pitch) = (rng.rand_angle(), rng.rand_angle());
                let m = &mut w.mobys[id];
                let p = add(polar(r, yaw, pitch), m.pos);
                m.set_pv_v(SPARKS + 0x10 * k, p);
            }
            w.mobys[id].set_state(1, -1);
        }
        1 => {
            w.register_draw(id);
            let c = local(w, id, [0.0, 0.0, height]);
            for k in 0..count(w, id) {
                let p = w.mobys[id].pv_v(SPARKS + 0x10 * k);
                let yaw = atan2(p[0] - c[0], p[1] - c[1]);
                let pitch = atan2(dist2(c, p), p[2] - c[2]);
                let yaw = sub_rot(yaw, f32::from_bits(0x3d56_7751));
                let rng = &mut ctx.game.rng;
                let d = rng.randf_signed(1.0, 3.0);
                let pitch = sub_rot(pitch, d * f32::from_bits(0x3c8e_fa36)).clamp(-1.1, 1.1);
                let r = (dist(c, p) + rng.randf_signed(1.0 / 60.0, 1.0 / 60.0)).clamp(0.75, f32::from_bits(0x3f86_6666));
                w.mobys[id].set_pv_v(SPARKS + 0x10 * k, add(polar(r, yaw, pitch), c));
            }
            let hit = w.hit_message(id).is_some_and(|h| h.tmpl.flags & 0x0401_0000 != 0);
            if hit && w.mobys[id].pv_i(0) == 0 { w.mobys[id].set_state(2, -1); }
            w.clear_hit(id);
        }
        2 => {
            crate::moby::shards::throw_all(w, id, ctx, [-3.0, -3.0, 7.0, 0.0, 3.0, 3.0, 9.0, 0.0], (90.0, 360.0), (10.0, 20.0), 0xffff, 0, -1, 0, 0xffff);
            let m = &w.mobys[id];
            let (rows, pos, s) = (m.rows, m.pos, m.scale);
            if let Some(sys) = ctx.parts.as_deref_mut() {
                crate::particles::staged_burst(sys, &mut ctx.game.rng, 0.1, 0.15, 1.0, 20.0, 1.0, &rows, pos, s, [0.0, 0.0, height], [0.0; 3], 0x50, 3);
            }
            let at = local(w, id, [0.0, 0.0, 18.0]);
            let e = crate::moby::blast::Explosion { flash2: 1.0, near: 9.0, size: 0.5, embers: 10, billows: 3, flares: 5, sound: -1, lod: -1, flags: 0x0081_0001, ..Default::default() };
            crate::moby::blast::explode(w, ctx, Some(id), Some(at), &e);
            for off in [GLOW_A, GLOW_B] {
                let g = w.mobys[id].pv_i(off);
                if let (Ok(i), Some(sys)) = (usize::try_from(g - 1), ctx.parts.as_deref_mut()) { sys.kill_part(i); }
                w.mobys[id].set_pv_i(off, 0);
            }
            if w.class(stump).is_some() {
                let ratio = { let m = &w.mobys[id]; m.scale / w.class(m.o_class).map_or(1.0, |c| c.scale) };
                let c = w.create(stump);
                let (pos, rot, light, ambient) = { let m = &w.mobys[id]; (m.pos, m.rot, m.light, m.ambient) };
                let m = &mut w.mobys[c];
                m.draw_dist = 0xff;
                m.drawn = 1;
                m.scale *= ratio;
                m.pos = pos;
                m.rot = rot;
                m.light = light;
                m.ambient = ambient;
                m.update_dist = 0;
                m.mode = 0;
                w.rebuild(c, &*ctx.host);
            }
            let m = &mut w.mobys[id];
            m.mode |= 1;
            m.has_collision = false;
            m.drawn = 0;
            m.set_pv_i(SMOKE, 0x78);
            m.set_pv_i(DRIFT, 0x258);
            m.set_state(3, -1);
        }
        3 => {
            let m = &mut w.mobys[id];
            let mut t = m.pv_i(SMOKE);
            let out = countdown(&mut t);
            m.set_pv_i(SMOKE, t);
            if out || 0x5b <= t {
                let (rows, pos, s) = (m.rows, m.pos, m.scale);
                let k = 1.0 / 30.0;
                if let Some(sys) = ctx.parts.as_deref_mut() {
                    let rng = &mut ctx.game.rng;
                    for (kind, v) in [(0u8, [k, 0.0, k]), (1, [-k, 0.0, f32::from_bits(0x3d23_d70b)])] {
                        let n = rng.rand_range(3, 6);
                        crate::particles::staged_burst(sys, rng, 0.0, k, 0.5, 15.0, 1.0, &rows, pos, s, [0.0, 0.0, smoke_up], through(v, &rows), n, kind);
                    }
                }
                let n = ctx.game.rng.rand_range(0x2d, 0x5a);
                w.mobys[id].set_pv_i(SMOKE, n);
            }
            let m = &mut w.mobys[id];
            let mut t = m.pv_i(DRIFT);
            let done = countdown(&mut t);
            m.set_pv_i(DRIFT, t);
            if !done {
                w.register_draw(id);
                for k in 0..count(w, id) {
                    let (o, p) = (w.mobys[id].pos, w.mobys[id].pv_v(SPARKS + 0x10 * k));
                    let mut d = sub(p, o);
                    d[2] = 0.0;
                    let d = set_len(d, 1.0);
                    let a = t as f32 * f32::from_bits(0xbb2b_92a8);
                    let (sn, cs) = a.sin_cos();
                    let mut v = [d[0] * cs - d[1] * sn, d[0] * sn + d[1] * cs, d[2]];
                    let rng = &mut ctx.game.rng;
                    let r = rng.rand_vec(1.0, 1.0);
                    v[2] += rng.randf(-0.25, 0.75);
                    let v = set_len(add(r, v), 2.0 * rng.randf(1.0 / 60.0, 1.0 / 60.0));
                    w.mobys[id].set_pv_v(SPARKS + 0x10 * k, add(p, v));
                }
            }
            let t = w.mobys[id].pv_i(DRIFT);
            if t < 0x3c { w.mobys[id].set_pv_f(BRIGHT, t as f32 * (1.0 / 60.0)); }
        }
        _ => {}
    }
}

/// `0x471c58` (module docs).
pub fn draw(w: &MobyWorld, id: usize, rng: &mut crate::rng::Rng, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let m = &w.mobys[id];
    let b = m.pv_f(BRIGHT);
    let c0 = ((b * 63.0) as i32 as u32) << 24 | 0x10_6060;
    let c1 = ((b * 128.0) as i32 as u32) << 24 | 0x80_8080;
    let push = f32::from_bits(0x3da3_d70a);
    for k in 0..count(w, id) {
        let p = m.pv_v(SPARKS + 0x10 * k);
        crate::fx::sprite(out, 0.2, push, rng.rand_angle(), p, cam, 0x31, c0, true);
        crate::fx::sprite(out, 0.2 * 0.4, push, rng.rand_angle(), p, cam, 0x31, c1, true);
    }
}
