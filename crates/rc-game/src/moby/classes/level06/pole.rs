//! Level 6's falling pole **0x12c4** (L06 `0x466008`; its splash `0x466558`): shot down, it topples over (hitting
//! what is under it), breaks and falls into the water.
//!
//! Its pvars: `+0x20` its health (6, a health block: `0x35df78` = crate::moby::ai::damage_of), the colour flash
//! `+0x50`, `+0x68` its topple speed (placed, growing), `+0x6c` the water's height, `+0x70` its fall a tick, `+0x80`
//! 1 once it has splashed.
//!
//! The engine functions: `0x36b650` the state's first tick, `0x3662f8` a point in its frame, `0x35d398` =
//! crate::moby::ai::attack_sphere, `0x43b580` = crate::moby::blast::burst, `0x367668` =
//! crate::moby::shards::break_into, `0x367558` = crate::moby::bolts::level_drop, `0x378a58` = crate::particles::
//! water::drop, `0x35bc10` a random point across within a radius.
//!
//! Not ported: the word `+0x7c` = 0x8100 its break writes (no reader known).

use crate::hero::physics::{add, add_rot, V3};
use crate::moby::{ai, Ctx, MobyWorld};

pub const CLASS: i16 = 0x12c4;
/// `$gp+0x2d30` (`0x1b1d20`): its top in its frame.
const TOP: V3 = [f32::from_bits(0x3f69_fbe7), 0.0, f32::from_bits(0x410d_1687)];
/// The splash moby (`0x3e25e0`, class 0x307).
const SPLASH: i16 = 0x0307;

fn through(r: &[V3; 3], v: V3) -> V3 { std::array::from_fn(|k| v[0] * r[0][k] + v[1] * r[1][k] + v[2] * r[2][k]) }
fn frame_point(w: &MobyWorld, id: usize, v: V3) -> V3 { add(through(&w.mobys[id].rows, v), w.mobys[id].pos) }

/// 0x12c4 (`0x466008`). **0**: its health 6, its fall (1/15 across its frame, 6..9 a second up), 1. **1**: a hit
/// (flags 0x230000) doing damage: the flash (`0xff`, `0x80`, `0x80`), its health less; under 0, 2. **2** (toppling):
/// on entry a blast (1, bursts 0x19 / 0x14 / 0x14, 20 puffs, 45 streaks, the flash ball and the shake) and its class
/// sound 1; its pitch on by its topple speed (1/3438 a tick more each tick); past 36° (Ratchet not in his state 0x8c)
/// without collision six 0.8 spheres along it (flags 0x30009) and a 2.5 one at its top (flags 0x4030000), damage 4;
/// past 72°, 3. **3** (falling): on entry its pieces (mask 0x6ff, 3..6 / 3..7, up 1/15 a tick), its class sound 0,
/// nine times the level's bolts, the shakes (0.2 up and down for 60 ticks, 0.1 rolling for 90) and its place moved to
/// its top; each tick the 2.5 sphere at its place, it falls (its fall a tick, 0.003 less up each tick); under the water
/// the splash ([`splash`]), 4 under it gone.
pub fn update_12c4(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.set_pv_s(0x24, 6);
            m.set_pv_f(0x20, 6.0);
            m.set_pv_i(0x80, 0);
            let up = ctx.game.rng.randf(6.0, 9.0) * f32::from_bits(0x3c88_8889);
            w.rebuild(id, &*ctx.host);
            let m = &mut w.mobys[id];
            let v = through(&m.rows, [f32::from_bits(0x3d88_8889), 0.0, up]);
            m.set_pv_v(0x70, v);
            m.set_state(1, -1);
        }
        1 => {
            let hit = w.hit_message(id).filter(|h| h.tmpl.flags & 0x23_0000 != 0);
            let d = ai::damage_of(w, id, hit.as_ref(), false);
            let m = &mut w.mobys[id];
            if 0.0 < d {
                m.set_pv_b(0x57, 0xff);
                m.set_pv_b(0x59, 0x80);
                m.set_pv_b(0x58, 0x80);
                ai::flash_start(m, 0x50);
                m.set_pv_f(0x20, m.pv_f(0x20) - d);
            }
            if m.pv_f(0x20) < 0.0 { m.set_state(2, -1); }
            ai::flash(&mut w.mobys[id], 0x50);
            w.clear_hit(id);
        }
        2 => {
            if w.mobys[id].first() {
                let pos = w.mobys[id].pos;
                let b = crate::moby::blast::Blast { size: 1.0, bursts: [0x19, 0x14, 0x14], puffs: 0x14, streaks: 0x2d, light: true, shake: true };
                crate::moby::blast::burst(w, ctx, pos, &b);
                ctx.host.play_sound(&w.mobys[id], 1, 0);
            }
            let m = &mut w.mobys[id];
            ai::flash(m, 0x50);
            let s = m.pv_f(0x68) + f32::from_bits(0x3998_825c);
            m.set_pv_f(0x68, s);
            m.rot[1] = add_rot(m.rot[1], s);
            if f32::from_bits(0x3f20_d97c) < m.rot[1] && ctx.hero.state != 0x8c {
                m.has_collision = false;
                for i in 0..6 {
                    let f = i as f32 * 0.2;
                    let p = frame_point(w, id, [f * 1.5 - 0.5, 0.0, f * 7.0 + 2.0]);
                    ai::attack_sphere(w, id, ctx, f32::from_bits(0x3f4c_cccd), 4.0, 1.0, p, 0x3_0009, 0, 1, 0);
                }
                let p = frame_point(w, id, TOP);
                ai::attack_sphere(w, id, ctx, 2.5, 4.0, 1.0, p, 0x403_0000, 0, 1, 0);
            }
            if f32::from_bits(0x3fa0_d97c) < w.mobys[id].rot[1] { w.mobys[id].set_state(3, -1); }
        }
        3 => {
            if w.mobys[id].first() {
                crate::moby::shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 7.0), 0x6ff, [0.0, 0.0, f32::from_bits(0x3d88_8889)], 0xff);
                ctx.host.play_sound(&w.mobys[id], 0, 0);
                crate::moby::bolts::level_drop(w, id, 0.0, 9, ctx);
                let s = &mut ctx.game.shakes.0;
                (s[0].amp, s[0].timer, s[0].max) = (0.2, 0x3c, 0);
                (s[2].amp, s[2].timer) = (0.1, 0x5a);
                let m = &mut w.mobys[id];
                m.pos = add(m.pos, through(&m.rows, TOP));
            }
            let pos = w.mobys[id].pos;
            ai::attack_sphere(w, id, ctx, 2.5, 4.0, 1.0, pos, 0x403_0000, 0, 1, 0);
            let m = &mut w.mobys[id];
            m.pos = add(m.pos, m.pv_v(0x70));
            m.set_pv_f(0x78, m.pv_f(0x78) - 0.003);
            w.changed = true;
            let (z, water) = (w.mobys[id].pos[2], w.mobys[id].pv_f(0x6c));
            if w.mobys[id].pv_i(0x80) == 0 && z < water {
                splash(w, id, ctx);
                w.mobys[id].set_pv_i(0x80, 1);
                return;
            }
            if z < water - 4.0 { w.delete(id); }
        }
        _ => {}
    }
}

/// `0x466558`: its class sound 2; the splash moby 0x307 at its place (Ratchet's colour words, ambient (0x28, 0x28,
/// 0x46), 11.25 its class's scale, a random yaw; [`update_0307`]); 90 drops of water (`0x378a58`) within 8 across of
/// it from up to 0.2 under it, thrown up to 1/6 a tick across (`0x35bc10`) and 10..17.5 a second up, falling or ringing
/// the water at random, for 450..600 ticks, 4..6 times their size.
fn splash(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    ctx.host.play_sound(&w.mobys[id], 2, 0);
    let p = w.mobys[id].pos;
    if w.class(SPLASH).is_some() {
        let k = w.create(SPLASH);
        let light = ctx.hero_moby.and_then(|h| w.mobys.get(h)).map(|h| h.light);
        let yaw = ctx.game.rng.rand_angle();
        let m = &mut w.mobys[k];
        if let Some(l) = light { m.light = l; }
        m.ambient = [0x28, 0x28, 0x46];
        m.update = Some(update_0307);
        m.mode &= !crate::moby::mode::NO_UPDATE;
        m.update_dist = 0xff;
        m.draw_dist = 0x40;
        m.scale *= 11.25;
        m.pos = p;
        m.rot[2] = yaw;
        w.rebuild(k, &*ctx.host);
    }
    let rng = &mut ctx.game.rng;
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    for _ in 0..90 {
        let a = rng.rand_angle();
        let f = ((rng.rand() >> 16) & 0xfff) as f32 * 0.000_244_140_62;
        let r = (1.0 - f * f) * (5.0 * 0.033_333_335);
        let (dx, dy) = (a.cos() * r, a.sin() * r);
        let vz = 5.0 * 0.016_666_668 * rng.randf(2.0, 3.5);
        let at = [p[0] + dx * 8.0, p[1] + dy * 8.0, p[2] - rng.randf(0.0, 0.2)];
        let kind = rng.randi(2);
        let life = rng.rand_range(0x5a, 0x78) * 5;
        if let Some(i) = crate::particles::water::drop(sys, rng, at, [dx, dy, vz], kind, life) {
            let s = crate::particles::rec::ff(&sys.pool.recs[i], 0xc) * rng.randf(4.0, 6.0);
            crate::particles::rec::set_ff(&mut sys.pool.recs[i], 0xc, s);
        }
    }
}

/// 0x0307 (`0x3e2688`): the splash fades 3 a tick, growing 2.5%; under 4 gone.
pub fn update_0307(w: &mut MobyWorld, id: usize, _ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    m.alpha = m.alpha.wrapping_sub(3);
    m.scale *= 1.025;
    if m.alpha < 4 { w.delete(id); }
}
