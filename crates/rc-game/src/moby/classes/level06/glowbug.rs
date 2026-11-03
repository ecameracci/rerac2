//! Level 6's glowbug **0x0db0** (L06 `0x444828`, 87 placed; its senses `0x444fe0`, tail `0x4453c0`, glow
//! `0x445710` and the group's glow draw `0x4458a8`), a small critter on the framework ([`crate::moby::ai`]): health
//! `+0x20`, reaction `+0x50` (a hurt goes to state 8, sub 3), movement `+0x100`, target `+0x1f0` (its moby `+0x200`),
//! its area `+0x210` (its first path its fence), `+0x214` ≠ 0 a start raised and hidden (9, sub 4, sequence 10).
//!
//! Its words: its home `+0x2e0`, the wander goal `+0x2f0`, timers `+0x310` / `+0x312`, seeing `+0x314`, alarmed
//! `+0x316`, its crowd offset `+0x31c`, the bite's heading `+0x320`, its scale `+0x324` (1), the glow's phase `+0x328`
//! and pulse `+0x32c`, the frame its group's glow was asked `+0x330`.
//!
//! The L06 engine functions with the port's: `0x36b678` / `0x36b650` the sub's / state's first tick, `0x36e6c8`
//! senses, `0x3682a0` take_damage, `0x368fe8` flight, `0x36bbc0` find_target, `0x36d588` / `0x36d5e0` the move toward
//! a point (with an offset: move_toward), `0x36e660` the move in place facing a heading, `0x3649f8` reached,
//! `0x35d4e0` attack_joint, `0x35adb8` the bolts, `0x367668` = L00 `0x327498` (crate::moby::shards::break_into),
//! `0x43b580` = L00 `0x3f8348` (crate::moby::blast::burst), `0x361b20` = L00 `0x3210a8` (smoke_burst), `0x366ec0`
//! the glow sprite (crate::fx::sprite, FX 0xb), `0x3516d0` delete.
//!
//! Not ported: the head's look at Ratchet in the chase (`0x445610` into `+0x2c0` / `+0x2c8`: the joint aims), the
//! big-head cheat (`0x3674d8` on `+0x260`).

use crate::hero::physics::{add, add_rot, atan2, scale, set_len, sub, V3};
use crate::moby::classes::props::{seq, wrapped};
use crate::moby::{ai, countdown, mode, Ctx, MobyWorld};

pub const CLASS: i16 = 0x0db0;
const MV: usize = 0x100;
const T_POINT: usize = 0x1f0;
const T_MOBY: usize = 0x200;
const AREA: usize = 0x210;
const RAISED: usize = 0x214;
const HOME: usize = 0x2e0;
const GOAL: usize = 0x2f0;
const T310: usize = 0x310;
const T312: usize = 0x312;
const SEEING: usize = 0x314;
const ALARMED: usize = 0x316;
const CROWD: usize = 0x31c;
const BITE_YAW: usize = 0x320;
const SCALE: usize = 0x324;
const PHASE: usize = 0x328;
const PULSE: usize = 0x32c;
const DRAWN_AT: usize = 0x330;
const PVARS: usize = 0x340;
/// `$gp+0x2050` / `0x2054`: the walk's speed a second, alert and calm.
const ALERT_SPEED: f32 = 4.0;
const CALM_SPEED: f32 = 4.0;
/// The limits of `0x36e6c8` (each sub's first tick).
const LIMITS: [u32; 4] = [0x3ba3_d70a, 0x3d88_8889, 0x3b64_c38a, 0x3ed6_7752];

fn first_sub(w: &mut MobyWorld, id: usize) -> bool {
    let m = &mut w.mobys[id];
    if m.once & 2 != 0 { return false; }
    m.once |= 2;
    true
}
fn st(w: &mut MobyWorld, id: usize, s: u8, sub: i32) { w.mobys[id].set_state(s, sub); }
fn anim(w: &MobyWorld, id: usize, ctx: &mut Ctx, s: u8, frame: u8, blend: u8) {
    if seq(w, id, ctx) != s { ctx.host.set_anim(&w.mobys[id], s, frame, blend); }
}
fn limits(w: &mut MobyWorld, id: usize) {
    let [a, b, c, d] = LIMITS.map(f32::from_bits);
    ai::senses(&mut w.mobys[id], a, b, c, d);
}
fn seeing(w: &MobyWorld, id: usize) -> bool { w.mobys[id].pv_s(SEEING) == 1 }
fn tick_s(w: &mut MobyWorld, id: usize, off: usize) {
    let mut t = w.mobys[id].pv_s(off) as i32;
    countdown(&mut t);
    w.mobys[id].set_pv_s(off, t as i16);
}
fn lose(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    st(w, id, 3, 1);
    let t = ctx.game.rng.rand_range(0x244, 600);
    w.mobys[id].set_pv_s(T310, t as i16);
}
fn walk_to(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, p: V3) -> i32 {
    let q = w.mobys[id].pos;
    ai::move_step(w, id, ctx, atan2(p[0] - q[0], p[1] - q[1]), p)
}

/// `0x445430`: its group alarmed (its glowbugs' `+0x316`, its 0x0bba's `+0x258`).
fn alarm(w: &mut MobyWorld, id: usize) {
    let members = usize::try_from(w.mobys[id].group).ok().and_then(|g| w.groups.get(g)).cloned().unwrap_or_default();
    for k in members {
        let Some(m) = w.mobys.get_mut(k).filter(|m| m.state < 0x80) else { continue };
        if m.o_class == CLASS { m.set_pv_s(ALARMED, 1); } else if m.o_class == 0x0bba { m.set_pv_i(0x258, 1); }
    }
}

/// `0x4454d8`: dead — its bolts, its class sound 1, a small blast (1/3: 10 puffs, 20 streaks), its pieces 0..4
/// thrown (3..6 a second across and up, a tenth of the way from Ratchet to it across), gone.
fn die(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    crate::moby::bolts::drop(w, id, 0, 3, ctx);
    ctx.host.play_sound(&w.mobys[id], 1, 0);
    let pos = w.mobys[id].pos;
    let b = crate::moby::blast::Blast { size: f32::from_bits(0x3eaa_7efa), bursts: [0; 3], puffs: 10, streaks: 0x14, light: false, shake: false };
    crate::moby::blast::burst(w, ctx, pos, &b);
    let d = sub(pos, ctx.hero.pos);
    let v = scale([d[0], d[1], 0.0], f32::from_bits(0x3dcc_ccce));
    crate::moby::shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 6.0), 0x1f, v, 0xff);
    w.delete(id);
}

/// `0x444fe0` (module docs); true when it is gone.
fn senses(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    if w.mobys[id].state == 0 {
        let m = &mut w.mobys[id];
        if m.pvars.len() < PVARS { m.pvars.resize(PVARS, 0); }
        m.update_dist = 0x40;
        let p = m.pos;
        m.set_pv_v(HOME, p);
        m.set_pv_f(SCALE, 1.0);
        let k = m.pv_f(SCALE);
        m.set_pv_b(0x48, 15);
        m.set_pv_s(0x88, 0);
        m.set_pv_b(0x4a, 4);
        m.set_pv_b(0x60, 8);
        m.set_pv_b(0x28, 3);
        m.set_pv_f(0x30, f32::from_bits(0x3eaa_7efa));
        m.set_pv_f(MV, k * 0.6);
        m.set_pv_f(MV + 0xc, 0.5);
        m.set_pv_i(MV + 0x38, m.pv_i(MV + 0x38) | 0x80);
        m.set_pv_i(0x98, 3);
        m.set_pv_f(MV + 8, 0.5);
        for (o, v) in [(0x84, 0x3e4c_cccdu32), (0x8c, 0x3ecc_cccd), (MV + 0x34, 0x3d4c_cccd), (MV + 0x10, 0x3fc9_0fdc), (MV + 0x30, 0x3f06_0a93), (0x80, 0x3e4c_cccd)] { m.set_pv_f(o, f32::from_bits(v)); }
        m.set_pv_f(MV + 4, k * 0.5);
        let weave = ctx.game.rng.randf_signed(18.0, 22.0) * 0.017_453_294;
        let m = &mut w.mobys[id];
        m.set_pv_b(MV + 0xe8, 7);
        m.set_pv_b(MV + 0xe9, 6);
        m.set_pv_f(MV + 0xdc, weave);
        m.set_pv_s(MV + 0xe2, 0x78);
        m.set_pv_s(MV + 0xe4, 0xb4);
        m.set_pv_f(MV + 0xd4, 3.0);
        m.set_pv_f(MV + 0xd0, ALERT_SPEED * 0.016_666_668);
        let area = m.pv_i(AREA);
        if let Some(a) = usize::try_from(area).ok().and_then(|a| ctx.level.areas.get(a)) {
            if let Some(&p) = a.lists[0].first() { w.mobys[id].set_pv_i(MV + 0x3c, p); }
        }
        let ph = ctx.game.rng.rand_angle();
        w.mobys[id].set_pv_f(PHASE, ph);
        if w.mobys[id].pv_i(RAISED) == 0 {
            st(w, id, 1, 0);
            anim(w, id, ctx, 0, 0, 0);
        } else {
            let m = &mut w.mobys[id];
            m.has_collision = false;
            m.mode = (m.mode & !mode::TARGET) | 0x41;
            st(w, id, 9, 4);
            anim(w, id, ctx, 10, 0, 0);
        }
    }
    if w.mobys[id].drawn != 0 { crate::shadow_volumes::probe_near(w, id, ctx, 24.0, 0x16); }
    let k = ai::take_damage(w, id, ctx, false);
    if k != 0 {
        alarm(w, id);
        if 4 <= k {
            die(w, id, ctx);
            return true;
        }
    }
    if w.mobys[id].pos[2] < 48.0 {
        crate::moby::bolts::drop(w, id, 0, 3, ctx);
        let pos = w.mobys[id].pos;
        crate::moby::blast::smoke_burst(w, ctx, 1.0, 13.0, Some(id), pos, -1);
        w.delete(id);
        return true;
    }
    if !(3..=4).contains(&w.mobys[id].sub_state) {
        let area = w.mobys[id].pv_i(AREA);
        let s = ai::find_target(w, id, ctx, 16.0, 0.0, area, None, false);
        w.mobys[id].set_pv_s(SEEING, s as i16);
    }
    false
}

/// 0x0db0 (`0x444828`). **Subs**: 0 calm (its walk 1/150 a tick², stop 0.5, the calm speed): seeing Ratchet, four in
/// five the chase (6, sub 2), else the rear (5, sub 2); alarmed, wary (3, sub 1; `+0x310` 480..600 when under 300).
/// 1 wary (stop 0.05, the alert speed): seeing him, the chase; `+0x310` out, idle (1, sub 0). 2 chasing: losing him,
/// wary (`+0x310` 580..600). 3 the hurt's flight: dead ([`die`]); over, the chase or wary. **States**: 1 idle (on
/// entry one of sequences 0 / 1, `+0x312` 60..120; sequence 1 back to 0 at its end; then 2); 2 a stroll to 2..5 from
/// home (sequence 6; 30..180 ticks, or there); 3 an idle 4 / 5; 4 home (sequence 6; there: 3); 5 a rear 2 / 3 (its end:
/// 6); 6 the chase (sequence 6, toward its target's point turned by its crowd offset `0x36aa60` × 5° every four ticks,
/// within 1.25: 7); 7 the bite (facing its target, sequence 8 from frame 3; at frame 13 a 0.75 sphere at joint 1 of its
/// damage byte, flags 0x4000001; its end: 6); 10 dropping in (sequence 10; on the floor: 5 sub 2).
pub fn update_0db0(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if senses(w, id, ctx) { return; }
    match w.mobys[id].sub_state {
        0 => {
            if first_sub(w, id) {
                limits(w, id);
                let m = &mut w.mobys[id];
                m.set_pv_f(MV + 0x24, f32::from_bits(0x3bda_740e));
                m.set_pv_f(MV + 0x34, 0.5);
            }
            w.mobys[id].set_pv_f(MV + 0xd0, CALM_SPEED * 0.016_666_668);
            if seeing(w, id) {
                if ctx.game.rng.randi(5) != 0 { st(w, id, 6, 2) } else { st(w, id, 5, 2) }
            } else if w.mobys[id].pv_s(ALARMED) != 0 {
                st(w, id, 3, 1);
                w.mobys[id].set_pv_s(ALARMED, 0);
                if w.mobys[id].pv_s(T310) < 300 {
                    let t = ctx.game.rng.rand_range(0x1e0, 600);
                    w.mobys[id].set_pv_s(T310, t as i16);
                }
            }
        }
        1 => {
            if first_sub(w, id) {
                limits(w, id);
                w.mobys[id].set_pv_f(MV + 0x34, 0.05);
            }
            w.mobys[id].set_pv_f(MV + 0xd0, ALERT_SPEED * 0.016_666_668);
            if seeing(w, id) {
                st(w, id, 6, 2);
            } else if w.mobys[id].pv_s(T310) == 0 {
                st(w, id, 1, 0);
            }
        }
        2 => {
            if first_sub(w, id) {
                limits(w, id);
                let m = &mut w.mobys[id];
                m.set_pv_f(MV + 0x24, f32::from_bits(0x3bda_740e));
                m.set_pv_f(MV + 0x34, 0.05);
            }
            w.mobys[id].set_pv_f(MV + 0xd0, ALERT_SPEED * 0.016_666_668);
            if !seeing(w, id) { lose(w, id, ctx); }
        }
        3 => {
            let r = ai::flight(w, id, ctx);
            if r & 1 != 0 { return die(w, id, ctx); }
            if r & 2 != 0 {
                if seeing(w, id) { st(w, id, 6, 2) } else { lose(w, id, ctx) }
            }
        }
        _ => {}
    }
    states(w, id, ctx);
    tail(w, id, ctx);
}

fn states(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        1 => {
            if w.mobys[id].first() {
                let s = (ctx.game.rng.randi(4) == 0) as u8;
                if seq(w, id, ctx) != s { ctx.host.set_anim(&w.mobys[id], s, 0, 10); }
                let t = ctx.game.rng.rand_range(0x3c, 0x78);
                w.mobys[id].set_pv_s(T312, t as i16);
            }
            if seq(w, id, ctx) == 1 {
                if wrapped(w, id, ctx) { ctx.host.set_anim(&w.mobys[id], 0, 0, 0x14); }
                return;
            }
            if w.mobys[id].pv_s(T312) == 0 { st(w, id, 2, -1); }
        }
        2 => {
            if w.mobys[id].first() {
                anim(w, id, ctx, 6, 0, 0x14);
                let a = ctx.game.rng.rand_angle();
                let d = ctx.game.rng.randf(2.0, 5.0);
                let h = w.mobys[id].pv_v(HOME);
                w.mobys[id].set_pv_v(GOAL, add([a.cos() * d, a.sin() * d, 0.0], h));
            }
            let g = w.mobys[id].pv_v(GOAL);
            walk_to(w, id, ctx, g);
            let t = w.mobys[id].state_ticks as u16;
            if t < 0x1f || (0.5 < w.mobys[id].pv_f(MV + 0xcc) && t < 0xb5) { return; }
            st(w, id, 1, -1);
        }
        3 => {
            if w.mobys[id].first() {
                let s = seq(w, id, ctx);
                if s as i32 != ctx.game.rng.randi(2) + 4 {
                    let n = ctx.game.rng.randi(2) as u8 + 4;
                    ctx.host.set_anim(&w.mobys[id], n, 0, 10);
                }
            }
        }
        4 => {
            if w.mobys[id].first() { anim(w, id, ctx, 6, 0, 10); }
            let h = w.mobys[id].pv_v(HOME);
            if walk_to(w, id, ctx, h) & 0x80 != 0 { st(w, id, 3, -1); }
        }
        5 => {
            if w.mobys[id].first() {
                let s = seq(w, id, ctx);
                if s as i32 != ctx.game.rng.randi(2) + 2 {
                    let n = ctx.game.rng.randi(2) as u8 + 2;
                    ctx.host.set_anim(&w.mobys[id], n, 0, 10);
                }
            }
            if wrapped(w, id, ctx) { st(w, id, 6, -1); }
        }
        6 => {
            if w.mobys[id].first() { anim(w, id, ctx, 6, 0, 5); }
            if w.mobys[id].state_ticks & 3 == 0 {
                let t = ai::link(&w.mobys[id], T_MOBY).and_then(|t| w.mobys.get(t)).map_or(ctx.hero.pos, |t| t.pos);
                let n = crowd(w, id, t);
                w.mobys[id].set_pv_f(CROWD, n as f32 * 0.087_266_475);
            }
            let (off, goal) = (w.mobys[id].pv_f(CROWD), w.mobys[id].pv_v(T_POINT));
            ai::move_toward(w, id, ctx, off, goal);
            if w.mobys[id].pv_f(MV + 0xcc) <= 1.25 { st(w, id, 7, -1); }
        }
        7 => {
            if w.mobys[id].first() {
                let (t, p) = (w.mobys[id].pv_v(T_POINT), w.mobys[id].pos);
                w.mobys[id].set_pv_f(BITE_YAW, atan2(t[0] - p[0], t[1] - p[1]));
                anim(w, id, ctx, 8, 3, 6);
            }
            let (y, p) = (w.mobys[id].pv_f(BITE_YAW), w.mobys[id].pos);
            ai::move_step(w, id, ctx, y, p);
            if ai::reached(w, id, ctx, 13.0) {
                let d = w.mobys[id].pv_b(0x34) as f32;
                ai::attack_joint(w, id, ctx, 0.75, d, 1.0, 1, 0x400_0001, 0, 1, 0);
            }
            if wrapped(w, id, ctx) { st(w, id, 6, -1); }
        }
        10 => {
            if w.mobys[id].first() {
                anim(w, id, ctx, 10, 0, 5);
                let f = ctx.game.frame;
                w.mobys[id].set_pv_i(MV + 0xbc, f);
            }
            let p = set_len(w.mobys[id].pv_v(MV + 0xa0), 1.0);
            walk_to(w, id, ctx, p);
            let m = &mut w.mobys[id];
            if m.pos[2] <= m.pv_f(MV + 0x90) {
                m.set_pv_i(MV + 0x38, m.pv_i(MV + 0x38) & !0x11);
                st(w, id, 5, 2);
            }
        }
        _ => {}
    }
}

/// `0x36aa60(m, p, group)`: how its group crowds it on the way to `p`: each other live member on its left −1, on its
/// right +1.
fn crowd(w: &MobyWorld, id: usize, p: V3) -> i32 {
    let Some(list) = usize::try_from(w.mobys[id].group).ok().and_then(|g| w.groups.get(g)) else { return 0 };
    let m = &w.mobys[id];
    let y = atan2(p[0] - m.pos[0], p[1] - m.pos[1]);
    let left = [-y.sin(), y.cos(), 0.0];
    list.iter().filter_map(|&k| w.mobys.get(k as usize).filter(|o| o.state < 0x80).filter(|_| k as usize != id)).map(|o| {
        let d = sub(o.pos, m.pos);
        if 0.0 < d[0] * left[0] + d[1] * left[1] { -1 } else { 1 }
    }).sum()
}

/// `0x4453c0` and `0x445710`: the timers down, its ticks; drawn, its glow's pulse (12° a tick, its glow word between
/// `0x80808080` and `0x00202000`) and once a frame its group's glows drawn ([`draw`]).
fn tail(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    tick_s(w, id, T310);
    tick_s(w, id, T312);
    let m = &mut w.mobys[id];
    if m.state_ticks != -1 { m.state_ticks += 1; }
    if m.drawn == 0 { return; }
    let ph = add_rot(m.pv_f(PHASE), f32::from_bits(0x3e56_7752));
    m.set_pv_f(PHASE, ph);
    let pulse = ph.sin() * 0.5 + 0.5;
    m.set_pv_f(PULSE, pulse);
    m.glow = crate::hud::lerp_rgba(pulse, 0x8080_8080, 0x0020_2000);
    let f = ctx.game.frame;
    if m.pv_i(DRAWN_AT) != f {
        m.set_pv_i(DRAWN_AT, f);
        w.register_draw(id);
    }
}

/// `0x4458a8`: each drawn glowbug of its group a glow (FX 0xb, its scale × 0.2, pulled 0.08 toward the camera) at
/// (0.25, 0, 0.1) × its scale in its joint 0's frame, its colour between `0x80602000` and `0x40402000` by its pulse.
pub fn draw(w: &MobyWorld, id: usize, host: &dyn crate::moby::MobyHost, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let list: Vec<usize> = usize::try_from(w.mobys[id].group).ok().and_then(|g| w.groups.get(g)).map(|l| l.iter().map(|&k| k as usize).collect()).unwrap_or_else(|| vec![id]);
    for k in list {
        let Some(m) = w.mobys.get(k).filter(|m| m.o_class == CLASS && m.drawn != 0 && m.state < 0xfd) else { continue };
        let Some((r, t)) = host.joint_matrix(m, 0) else { continue };
        let s = m.pv_f(SCALE);
        let p = add(t, add(add(scale(r[0], 0.25 * s), scale(r[1], 0.0)), scale(r[2], 0.1 * s)));
        let rgba = crate::hud::lerp_rgba(m.pv_f(PULSE), 0x8060_2000, 0x4040_2000);
        crate::fx::sprite(out, s * 0.2, f32::from_bits(0x3da3_d70a), 0.0, p, cam, 0xb, rgba, true);
    }
}
