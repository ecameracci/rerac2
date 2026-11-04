//! Level 7's nest **0x0d1c** (L07 `0x3e5f10`, 11 placed) and its flyers **0x0d1d** (`0x3e64d8`, 16 placed; their
//! set-up and checks `0x3e69f8`, their move `0x3e6c50`, tail `0x3e6db0`, death `0x3e70c0`, trail `0x3e72e8`).
//!
//! **The nest** (its pvars: `[0]` its flyers' moby group, `[1]` its cuboid, `[4]` a platform it sits on, `[5]` the
//! release timer, `[6]` how open it is, `[7]` its spin): **0** stood on the ground under it (+0.05), `[8]` −1; 1.
//! **1** Ratchet in its cuboid (or it hooked, `+0xbe & 4`): 2. **2** open (sequence 1, `[6]` eased to 1 by 0.05 a
//! tick, spinning 360·1/3438 a tick): every 60 ticks a waiting flyer of its group (state 8, `0x3e6eb0`) is let out at
//! it if its mouth (0.7 up its frame) is clear (a sphere of 0.5; `0x3e6fb8`) toward 0.5 above it, its class sound 0;
//! none waiting and none left: 3. One tick in five while drawn a glint rises from 0.4 about it (0.1..0.2, 30..60
//! ticks, `0x80008080`..`0x80004080`). **3** closing (sequence 0, `[6]` to 0): closed, mode |= 2.
//!
//! **A flyer** on the framework ([`crate::moby::ai`]): health `+0x20` (lives `+0x21a`, damage byte `+0x34`, `+0x2e`
//! = 2 a kill asked by another), reaction `+0x50` (hurts into state 7, sub 3), movement `+0x120` (radius 0.45, no ledge
//! test, no gravity, no turn; walk 18/3600, most 6/60), target `+0x100` (found `+0x114`, Ratchet in the area `+0x214`).
//! It flies at 0.75 above the ground and Ratchet's ground, within 2 under and 3 over its home `+0x220` (the height's
//! speed `+0x1c8` eased to it), toward its goal ([`crate::moby::ai::move_toward`]). Its buzz loops (its class sound 1,
//! `+0x264`) but while waiting (sub-state 4).
//!
//! Sub-states: 0 at home — seeing him, the attack (state 5, sub 2); 2 attacking — him gone, home (1, sub 0). States:
//! 1 home (sequence 0); 3 out (a weave `randf_signed(20°, 45°)` `+0x250`) to within 2 of him → 4 (sequence 2) to its
//! end → 5 the bite (sequence 3): after 10 ticks a sphere of 0.6 on it each tick (damage `+0x34`, flags 0x4000001);
//! after 30, his hurt by it (his state 7 or 0x14) → 6 (sequence 4) to its end → 5. 8 waiting in its nest, hidden. 9 out
//! of the nest (`+0x230`, 1/12 a tick, its trail) → 10 growing to its size (its rows from 1/10: their length eased to
//! 1, the third first, by 40/3600 to 20/60), solid; grown, out (3, sub 2).
//!
//! Its death (a kill, or one asked: its group's `+0x244` set): its bolts (1; not when asked), the buzz stopped, its class
//! sound 0, a blast 0.5 up (0.3: 20 puffs, 45 streaks, the flash), its pieces (mask 3, away from Ratchet at 1/15); with
//! lives left, back in its nest (state 8, sub 4, hidden, full health), else deleted. Its trail (states 9 / 10): two
//! glints a tick ±0.5 up its frame (0.15..0.75, 5..15 ticks, `0x60004080`, drifting up to 1/120 a tick a random way).
//!
//! Not ported: the nest's draw callback (`0x3e63b0`: two layers of five shapes from the templates `0x29f570` /
//! `0x29f5b0` by `0x2e3230` / `0x2e32c8`, coloured by its spin `0x31da88` and opening), the platform rides (the nest's
//! `[4]`, the flyers' `+0x260`: `0x318120` / `0x318370`, `0x3e7488`).

use crate::hero::physics::{add, add_rot, ease, ease_speed, len, scale, set_len, sub, V3};
use crate::moby::classes::props::{set_seq, wrapped};
use crate::moby::{ai, countdown, Ctx, MobyWorld};

pub const NEST: i16 = 0x0d1c;
pub const FLYER: i16 = 0x0d1d;
const MV: usize = 0x120;
const TARGET: usize = 0x100;
const SEEN: usize = 0x114;
const HOME: usize = 0x220;
const OUT_TO: usize = 0x230;
const WEAVE: usize = 0x250;
const BUZZ: usize = 0x264;
/// `$gp+0x1a60` / `$gp+0x1a64`: its walk's accel (/3600) and most (/60).
const WALK: (f32, f32) = (18.0 * 0.000_277_777_78, 6.0 * 0.016_666_668);
/// `$gp+0x1a68`: its height above the ground.
const ABOVE: f32 = 0.75;

fn members(w: &MobyWorld, group: i32) -> Vec<usize> {
    usize::try_from(group).ok().filter(|&g| g != 0xff).and_then(|g| w.groups.get(g)).cloned().unwrap_or_default()
}

/// 0x0d1c (module docs).
pub fn update_0d1c(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let m = &w.mobys[id];
            let (a, b) = ([m.pos[0], m.pos[1], m.pos[2] + 0.5], [m.pos[0], m.pos[1], f32::from_bits(0x3c23_d70a)]);
            if let Some(hit) = ctx.coll.and_then(|c| crate::collision::coll_line_m(c, ctx.scene, a, b, crate::collision::QueryFlags(2), Some(id))) {
                w.mobys[id].pos = hit.point;
            }
            let m = &mut w.mobys[id];
            m.pos[2] += 0.05;
            m.set_pv_i(0x20, -1);
            m.set_state(1, -1);
            w.changed = true;
        }
        1 => {
            let m = &w.mobys[id];
            if ctx.level.in_cuboid(m.pv_i(4), ctx.hero.pos) || m.once & 4 != 0 { w.mobys[id].set_state(2, -1); }
        }
        2 => {
            if w.mobys[id].first() { set_seq(w, id, ctx, 1, 0, 0x14); }
            let mut t = w.mobys[id].pv_i(0x14);
            let out = countdown(&mut t);
            w.mobys[id].set_pv_i(0x14, t);
            if out {
                let group = w.mobys[id].pv_i(0);
                let all = members(w, group);
                let alive = all.iter().filter(|&&i| w.mobys.get(i).is_some_and(|x| x.o_class == FLYER && x.state != 0xfe && x.state != 0xfd)).count();
                let waiting = all.iter().copied().filter(|&i| w.mobys.get(i).is_some_and(|x| x.state == 8)).last();
                match waiting {
                    None => if alive == 0 { w.mobys[id].set_state(3, -1) },
                    Some(f) => {
                        let m = &w.mobys[id];
                        let mouth = add(m.pos, scale(m.rows[2], f32::from_bits(0x3f33_3333)));
                        let blocked = ctx.coll.and_then(|c| crate::collision::coll_sphere_m(c, ctx.scene, mouth, 0.5, crate::collision::QueryFlags(0), Some(id))).is_some();
                        if !blocked {
                            let (at, to) = (m.pos, [m.pos[0], m.pos[1], m.pos[2] + 0.5]);
                            release(w, f, at, to);
                            ctx.host.play_sound(&w.mobys[id], 0, 0);
                            w.mobys[id].set_pv_i(0x14, 0x3c);
                        }
                    }
                }
            }
            let m = &mut w.mobys[id];
            let mut open = m.pv_f(0x18);
            crate::hero::physics::approach(1.0, f32::from_bits(0x3d4c_ccce), &mut open);
            m.set_pv_f(0x18, open);
            m.set_pv_f(0x1c, add_rot(m.pv_f(0x1c), 360.0 * 0.000_290_888_25));
            if w.mobys[id].drawn == 0 || ctx.game.rng.randi(5) != 0 { return; }
            let pos = w.mobys[id].pos;
            let hero = ctx.hero.pos;
            let rng = &mut ctx.game.rng;
            let a = rng.rand_angle();
            let at = [pos[0] + a.cos() * 0.4, pos[1] + a.sin() * 0.4, pos[2]];
            let size = rng.randf(0.2, 0.1);
            let c = crate::hud::lerp_rgba(rng.randf(0.0, 1.0), 0x8000_8080, 0x8000_4080);
            let up = rng.randf(1.0, 2.0);
            let life = rng.rand_range(0x1e, 0x3c);
            let rot = rng.randi(0xff) as u8;
            if let Some(sys) = ctx.parts.as_deref_mut() { crate::particles::glint(sys, size, at, [0.0, 0.0, up / life as f32], c, life as u16, rot, false, hero); }
        }
        3 => {
            if w.mobys[id].first() { set_seq(w, id, ctx, 0, 0, 0x14); }
            let m = &mut w.mobys[id];
            let mut open = m.pv_f(0x18);
            crate::hero::physics::approach(0.0, f32::from_bits(0x3d4c_ccce), &mut open);
            m.set_pv_f(0x18, open);
            let still = ctx.host.anim(&w.mobys[id]).is_none_or(|a| a.seq_a == a.seq_b);
            if still && w.mobys[id].pv_f(0x18) <= 0.0 { w.mobys[id].mode |= 2; }
        }
        _ => {}
    }
}

/// `0x3e6fb8(flyer, at, to, platform)`: a waiting flyer let out — shown and targetable but not solid yet (mode `& 0xffbe
/// | 0x1000`), state 9 (sub 4) at `at` heading for `to`, its rows a tenth long and kept.
fn release(w: &mut MobyWorld, f: usize, at: V3, to: V3) {
    let m = &mut w.mobys[f];
    if m.state != 8 { return; }
    m.has_collision = false;
    m.mode = (m.mode & 0xffbe) | 0x1000 | crate::moby::mode::KEEP_ROWS;
    m.set_state(9, 4);
    m.pos = at;
    m.set_pv_v(OUT_TO, to);
    m.set_pv_i(0x260, 0);
    m.rows = m.rows.map(|r| set_len(r, f32::from_bits(0x3dcc_cccd)));
    w.changed = true;
}

/// `0x3e69f8` (module docs). Returns whether it is to go.
fn checks(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    if w.mobys[id].state == 0 {
        let m = &mut w.mobys[id];
        m.set_pv_b(0x48, 15);
        m.set_pv_b(0x4a, 2);
        m.set_pv_b(0x60, 7);
        m.set_pv_b(0xc7, 1);
        m.set_pv_b(0x2e, 1);
        m.set_pv_i(0x98, 3);
        m.set_pv_f(0x80, 0.5);
        m.set_pv_f(0x8c, 0.15);
        m.set_pv_f(0xac, 1.0);
        m.set_pv_f(MV, f32::from_bits(0x3ee6_6666));
        m.set_pv_f(MV + 0x24, WALK.0);
        m.set_pv_f(MV + 0x28, WALK.1);
        m.set_pv_i(MV + 0x38, m.pv_i(MV + 0x38) | 0x15);
        m.set_pv_f(0x84, 0.0);
        m.set_pv_f(0xa8, 1.0);
        m.set_pv_f(MV + 4, 0.0);
        m.set_pv_f(MV + 0x20, WALK.0);
        m.set_pv_i(0x30, 0);
        m.set_pv_i(BUZZ, -1);
        let home = m.pos;
        m.set_pv_v(HOME, home);
        if m.pv_s(0x218) == 0 {
            m.has_collision = false;
            m.mode = (m.mode & 0xefff) | 0x41;
            m.set_state(8, 4);
        } else {
            m.set_state(1, 0);
        }
        w.changed = true;
    }
    crate::shadow_volumes::probe_near(w, id, ctx, 24.0, 0x16);
    if w.mobys[id].sub_state != 4 {
        let h = w.mobys[id].pv_i(BUZZ);
        if h < 0 || !ctx.host.sound_playing(h) {
            let h = ctx.host.play_sound(&w.mobys[id], 1, 4);
            w.mobys[id].set_pv_i(BUZZ, h);
        }
    }
    if 1 < ai::take_damage(w, id, ctx, false) || w.mobys[id].pv_b(0x2e) == 2 {
        for i in members(w, w.mobys[id].group as i32) {
            if let Some(m) = w.mobys.get_mut(i) { m.set_pv_i(0x244, 1); }
        }
        if die(w, id, ctx) { return true; }
    }
    let area = w.mobys[id].pv_i(0x214);
    ai::find_target(w, id, ctx, 0.0, 0.0, area, None, false);
    false
}

/// `0x3e70c0` (module docs). Returns whether it is gone.
fn die(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    if w.mobys[id].pv_b(0x2e) != 2 { crate::moby::bolts::drop(w, id, 0, 1, ctx); }
    let h = w.mobys[id].pv_i(BUZZ);
    if 0 <= h && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
    w.mobys[id].set_pv_i(BUZZ, -1);
    ctx.host.play_sound(&w.mobys[id], 0, 0);
    let pos = w.mobys[id].pos;
    let b = crate::moby::blast::Blast { size: f32::from_bits(0x3e99_999a), bursts: [0; 3], puffs: 0x14, streaks: 0x2d, light: true, shake: false };
    crate::moby::blast::burst(w, ctx, [pos[0], pos[1], pos[2] + 0.5], &b);
    let d = sub(pos, ctx.hero.pos);
    let v = set_len([d[0], d[1], 0.0], f32::from_bits(0x3d88_8889));
    crate::moby::shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 6.0), 3, v, 0xff);
    let m = &mut w.mobys[id];
    let lives = m.pv_s(0x21a) - 1;
    m.set_pv_s(0x21a, lives);
    if lives == -1 { return true; }
    m.set_pv_s(0x88, 0);
    m.has_collision = false;
    m.mode = (m.mode & 0xefff) | 0x41;
    m.set_state(8, 4);
    m.set_pv_b(0x2e, 1);
    m.set_pv_f(0x20, m.pv_s(0x24) as f32);
    w.changed = true;
    false
}

/// `0x3e6c50(offset, m, goal)`: the height held (0.75 above Ratchet's ground, kept within 2 under and 3 over home, and
/// 0.75 over the ground under it: the height's speed eased to it), then [`ai::move_toward`]. Returns the goal's
/// distance.
fn fly(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, offset: f32, goal: V3) -> f32 {
    let m = &w.mobys[id];
    let home = m.pv_v(HOME);
    let mut z = (ctx.hero.ground_point[2] + ABOVE).clamp(home[2] - 2.0, home[2] + 3.0);
    let g = ai::ground_z(ctx, 0.5, m.pos, 0);
    if z < g + ABOVE { z = g + ABOVE; }
    let mut v = m.pv_f(MV + 0xa8);
    ease_speed(z - m.pos[2], WALK.0, WALK.0, WALK.1, &mut v);
    w.mobys[id].set_pv_f(MV + 0xa8, v);
    ai::move_toward(w, id, ctx, offset, goal);
    w.mobys[id].pv_f(MV + 0xcc)
}

/// 0x0d1d (module docs).
pub fn update_0d1d(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if checks(w, id, ctx) {
        w.delete(id);
        return;
    }
    match w.mobys[id].sub_state {
        0 => if w.mobys[id].pv_i(SEEN) == 1 { w.mobys[id].set_state(5, 2) },
        2 => if w.mobys[id].pv_i(SEEN) != 1 { w.mobys[id].set_state(1, 0) },
        _ => {}
    }
    let next = match w.mobys[id].state {
        1 => {
            if w.mobys[id].first() { set_seq(w, id, ctx, 0, 0, 0x14); }
            let home = w.mobys[id].pv_v(HOME);
            fly(w, id, ctx, 0.0, home);
            None
        }
        3 => {
            if w.mobys[id].first() {
                let a = ctx.game.rng.randf_signed(20.0, 45.0) * 0.017_453_294;
                w.mobys[id].set_pv_f(WEAVE, a);
            }
            let (o, t) = (w.mobys[id].pv_f(WEAVE), w.mobys[id].pv_v(TARGET));
            if fly(w, id, ctx, o, t) < 2.0 { Some(4) } else { None }
        }
        4 => {
            if w.mobys[id].first() { set_seq(w, id, ctx, 2, 0, 5); }
            let (o, t) = (w.mobys[id].pv_f(WEAVE), w.mobys[id].pv_v(TARGET));
            fly(w, id, ctx, o, t);
            wrapped(w, id, ctx).then_some(5)
        }
        5 => {
            if w.mobys[id].first() { set_seq(w, id, ctx, 3, 0, 2); }
            let (o, t) = (w.mobys[id].pv_f(WEAVE), w.mobys[id].pv_v(TARGET));
            fly(w, id, ctx, o, t);
            if 0xb <= w.mobys[id].state_ticks {
                let (damage, pos) = (w.mobys[id].pv_b(0x34) as f32, w.mobys[id].pos);
                ai::attack_sphere(w, id, ctx, f32::from_bits(0x3f19_999a), damage, 1.0, pos, 0x400_0001, 0, 1, 0);
            }
            let hurt = matches!(ctx.hero.state, 7 | 0x14) && ctx.hero.damage.hit_by == Some(id);
            (0x1f <= w.mobys[id].state_ticks && hurt).then_some(6)
        }
        6 => {
            if w.mobys[id].first() { set_seq(w, id, ctx, 4, 0, 1); }
            wrapped(w, id, ctx).then_some(5)
        }
        9 => {
            let m = &mut w.mobys[id];
            let mut d = sub(m.pv_v(OUT_TO), m.pos);
            if f32::from_bits(0x3daa_aaab) < len(d) { d = set_len(d, f32::from_bits(0x3daa_aaab)) } else { m.set_state(10, -1) }
            m.pos = add(m.pos, d);
            w.changed = true;
            trail(w, id, ctx);
            None
        }
        10 => {
            if w.mobys[id].first() {
                let solid = w.class(FLYER).is_some_and(|c| c.has_collision);
                w.mobys[id].has_collision = solid;
            }
            let m = &mut w.mobys[id];
            let (mut sx, mut sz) = (len(m.rows[0]), len(m.rows[2]));
            let (a, most) = (40.0 * 0.000_277_777_78, 20.0 * 0.016_666_668);
            let (mut vz, mut vx) = (m.pv_f(0x254), m.pv_f(0x258));
            ease(1.0, a, a, most, &mut sz, &mut vz);
            if 0.9 < sz { ease(1.0, a, a, most, &mut sx, &mut vx); }
            m.set_pv_f(0x254, vz);
            m.set_pv_f(0x258, vx);
            m.rows = [set_len(m.rows[0], sx), set_len(m.rows[1], sx), set_len(m.rows[2], sz)];
            if 1.0 <= sx {
                m.rows = m.rows.map(|r| set_len(r, 1.0));
                m.mode &= !crate::moby::mode::KEEP_ROWS;
                m.set_state(3, 2);
            }
            w.changed = true;
            trail(w, id, ctx);
            None
        }
        _ => None,
    };
    if let Some(s) = next { w.mobys[id].set_state(s, -1); }
    // `0x3e6db0`: its timers and its state's ticks.
    let m = &mut w.mobys[id];
    for o in [0x240, 0x242] {
        let mut t = m.pv_s(o) as i32;
        countdown(&mut t);
        m.set_pv_s(o, t as i16);
    }
    if m.state_ticks != -1 { m.state_ticks = m.state_ticks.wrapping_add(1); }
}

/// `0x3e72e8` (module docs).
fn trail(w: &MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let (pos, up) = (m.pos, m.rows[2]);
    let hero = ctx.hero.pos;
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let rng = &mut ctx.game.rng;
    for k in 0..2 {
        let side = if k == 0 { -0.5 } else { 0.5 };
        let r = rng.randf(0.0, 0.5 * 0.016_666_668);
        let (a, b) = (rng.rand_angle(), rng.rand_angle());
        let vel = [a.cos() * b.sin() * r, a.sin() * b.sin() * r, b.cos() * r];
        let life = rng.rand_range(5, 15) as u16;
        let at = add(pos, scale(up, side));
        let size = rng.randf(0.75, 0.15);
        let rot = rng.randi(0xff) as u8;
        crate::particles::glint(sys, size, at, vel, 0x6000_4080, life, rot, false, hero);
    }
}
