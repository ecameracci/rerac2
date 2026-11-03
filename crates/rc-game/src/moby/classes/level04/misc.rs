//! Level 4's small classes (L04 addresses; the copies on other levels noted per class).
//!
//! * **0x0969** (`0x3e9a40`, 16 placed), **0x0e1b** (`0x43cb58`, one) and level 11's **0x0e1c** (L11 `0x459b98`):
//!   an empty update (`jr ra`), [`update_empty`].
//! * **0x1293** ([`update_1293`], `0x457ef8`, 12 placed; L20 `0x44aaf0`): a breakable.
//! * **0x0a99** ([`update_0a99`], `0x3f76b8`, 13 placed; L11 `0x3df6f8`): a spinning beacon of three parts.
//! * **0x0ab8** ([`update_0ab8`], `0x3fa730`, 6 placed; L11 `0x3e2770`): the beacon's glow alone.

use crate::hero::physics::{add, add_rot, set_len, sub};
use crate::moby::{countdown, Ctx, MobyWorld};

/// An update that does nothing (`jr ra`).
pub fn update_empty(_w: &mut MobyWorld, _id: usize, _ctx: &mut Ctx) {}

/// 0x1293 (`0x457ef8`): **0** → 1. **1**: a hit with the flags 0x4010000 (`0x33f8d0`) while `+0x68`[0] = 0 breaks
/// it (2); the hit record let go. **2**: the level's bolts (`0x3499a8` = L00 `0x327388`, crate::moby::bolts::level_drop,
/// height 0), its sound 0, thirty debris chunks ([`chunks_1293`], `0x458010`), deleted.
pub fn update_1293(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => w.mobys[id].set_state(1, -1),
        1 => {
            if w.hit_message(id).is_some_and(|h| h.tmpl.flags & 0x0401_0000 != 0) && w.mobys[id].pv_i(0) == 0 { w.mobys[id].set_state(2, -1); }
            w.clear_hit(id);
        }
        2 => {
            crate::moby::bolts::level_drop(w, id, 0.0, 1, ctx);
            ctx.host.play_sound(&w.mobys[id], 0, 0);
            chunks_1293(w, id, ctx, 0.4, f32::from_bits(0x408c_cccd), 1.0, f32::from_bits(0x3daa_aaab), 0x1e);
            w.delete(id);
        }
        _ => {}
    }
}

/// `0x458010(lo, hi, r, speed, m, n)`: `n` debris chunks (`0x349c68` = L00 `0x327648`, crate::moby::shards::
/// throw_chunks: pointed, 1..3.5 of the class's size, colour `0x80402000`), each `r` out a random way and `lo..hi`
/// up from its place, moving `speed`·0.75..1.25 a tick out along the way with 0..1 up.
#[allow(clippy::too_many_arguments)]
fn chunks_1293(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, lo: f32, hi: f32, r: f32, speed: f32, n: i32) {
    for _ in 0..n {
        let rng = &mut ctx.game.rng;
        let a = rng.randf(-std::f32::consts::PI, std::f32::consts::PI);
        let mut d = [a.cos() * r, a.sin() * r, rng.randf(0.0, 1.0)];
        let vel = set_len(d, rng.randf(speed * 0.75, speed * 1.25));
        d[2] = rng.randf(lo, hi);
        let at = add(w.mobys[id].pos, d);
        crate::moby::shards::throw_chunks(w, ctx, 0.0, 0.0, (1.0, 3.5), id, at, 1, true, vel, 0x8040_2000, None);
    }
}

/// Whether the beacon's gate moby (`+0x68`[k], −1 none) lets it work: none, or in state 2 or 3.
fn gate(w: &MobyWorld, id: usize, k: usize) -> bool {
    let g = w.mobys[id].pv_i(4 * k);
    g == -1 || usize::try_from(g).ok().and_then(|g| w.mobys.get(g)).is_some_and(|m| (2..4).contains(&m.state))
}

/// The glow's set-up (`0x3f73e8` / `0x3fa460`) at `b`: four phases at random (0..255), their steps −1 / −2.25 /
/// 1.25 / 2.5 a tick, their fade timers 0x3f / 0x7f / 0xbf / 0xff, sizes 2 / 2.5 / 2.5 / 2.
fn glow_setup(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, b: usize) {
    for k in 0..4 {
        let r = ctx.game.rng.randi(0xff) as f32;
        w.mobys[id].set_pv_f(b + 4 * k, r);
    }
    let m = &mut w.mobys[id];
    for (k, s) in [-1.0, -2.25, 1.25, 2.5].into_iter().enumerate() { m.set_pv_f(b + 0x10 + 4 * k, s); }
    for (k, t) in [0x3f, 0x7f, 0xbf, 0xff].into_iter().enumerate() { m.set_pv_i(b + 0x20 + 4 * k, t); }
    for (k, s) in [2.0, 2.5, 2.5, 2.0].into_iter().enumerate() { m.set_pv_f(b + 0x30 + 4 * k, s); }
}

/// The glow (`0x3f74c0` / `0x3fa538`) at `b`: four sparkles (`0x35e930` = L00 `0x33b468`, crate::particles::sparkle:
/// frame `*def[0x35]`, soft, two ticks) 0.3, 0.4, 0.5, 0.6 beyond its place 0.25 up away from the camera, each its
/// phase turned by its step (wrapped in 0..255) as its roll, its fade timer down (out: 0xff again), its colour between
/// `0x4040ffff` and `0x1040ffff` by |0.5 − (0xff − timer)/255|.
fn glow(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, b: usize) {
    let m = &w.mobys[id];
    let p = [m.pos[0], m.pos[1], m.pos[2] + 0.25];
    let d = set_len(sub(ctx.host.camera_pos(), p), -0.3);
    let step = set_len(d, 0.1);
    let mut at = add(d, p);
    let hero = ctx.hero.pos;
    for k in 0..4 {
        let m = &mut w.mobys[id];
        let mut ph = m.pv_f(b + 4 * k) + m.pv_f(b + 0x10 + 4 * k);
        if 255.0 <= ph { ph -= 255.0 } else if ph <= 0.0 { ph += 255.0 }
        m.set_pv_f(b + 4 * k, ph);
        let mut t = m.pv_i(b + 0x20 + 4 * k);
        if countdown(&mut t) { t = 0xff; }
        m.set_pv_i(b + 0x20 + 4 * k, t);
        let f = (0xff - t) as f32;
        let c = crate::hud::lerp_rgba((0.5 - f * 0.003_921_569).abs(), 0x4040_ffff, 0x1040_ffff);
        let size = m.pv_f(b + 0x30 + 4 * k);
        if let Some(sys) = ctx.parts.as_deref_mut() { crate::particles::sparkle(sys, size, at, c, ph as i32 as u8, 0x35, true, 2, false, hero); }
        at = add(at, step);
    }
}

/// The beacon's show (its timer `+0x68`[0] out: drawn, `glow` runs) or hide (counting: hidden), for it and `parts`.
fn show(w: &mut MobyWorld, id: usize, parts: &[usize]) -> bool {
    let mut t = w.mobys[id].pv_i(0);
    let out = countdown(&mut t);
    w.mobys[id].set_pv_i(0, t);
    for &i in std::iter::once(&id).chain(parts) {
        let m = &mut w.mobys[i];
        m.mode = if out { m.mode & 0xffbe } else { m.mode | 0x41 };
    }
    out
}

/// 0x0a99 (`0x3f76b8`; L11 `0x3df6f8`, the same code): while its gate moby (`+0x68`[4]) is in state 2 or 3 (or it
/// has none). **0**: 0.5 up (that height kept, `+0x68`[0x16]), its bob's phase and timer 0, update distance 0xff, draw
/// distance 0x7f, its class's size, the glow's set-up ([`glow_setup`], `+0x14`); 1. Then its three parts 0x0b1c /
/// 0x0b1d / 0x0b1e (`+0x68`[1..4]) made where missing (`0x401ed8`: update and draw distances 0xff, drawn, its place
/// and angles, the class's size); with all three: it spins 6° a tick and bobs (`0x346770`: the phase 6° a tick, 0.01
/// of its sine over the kept height), the parts at its place and angles, their glow words (`+0x78`) lit one at a time
/// by the level's frame (`% 60 / 15`: `0x804040ff` on the first, second, third, then none; the others `0x80000040`);
/// shown or hidden by its timer ([`show`]), the glow while shown ([`glow`]).
pub fn update_0a99(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if !gate(w, id, 4) { return; }
    if w.mobys[id].state == 0 {
        let scale = w.class(w.mobys[id].o_class).map_or(1.0, |c| c.scale);
        let m = &mut w.mobys[id];
        m.pos[2] += 0.5;
        m.set_pv_i(0x54, 0);
        m.set_pv_i(0, 0);
        m.set_pv_f(0x58, m.pos[2]);
        m.update_dist = 0xff;
        m.draw_dist = 0x7f;
        m.scale = scale;
        for k in 1..4 { m.set_pv_i(4 * k, 0); }
        glow_setup(w, id, ctx, 0x14);
        w.mobys[id].state = 1;
    }
    for (k, class) in [(1, 0x0b1c), (2, 0x0b1d), (3, 0x0b1e)] {
        if w.mobys[id].pv_i(4 * k) == 0 {
            let (pos, rot) = (w.mobys[id].pos, w.mobys[id].rot);
            let p = w.create(class);
            let scale = w.class(class).map_or(1.0, |c| c.scale);
            let pm = &mut w.mobys[p];
            pm.update_dist = 0xff;
            pm.draw_dist = 0xff;
            pm.drawn = 1;
            pm.state = 0;
            pm.scale = scale;
            pm.pos = pos;
            pm.rot = rot;
            w.rebuild(p, &*ctx.host);
            crate::moby::ai::set_link(&mut w.mobys[id], 4 * k, Some(p));
        }
    }
    let parts: Vec<usize> = (1..4).filter_map(|k| crate::moby::ai::link(&w.mobys[id], 4 * k)).collect();
    if parts.len() < 3 { return; }
    let frame = ctx.game.frame;
    let m = &mut w.mobys[id];
    m.rot[2] = add_rot(m.rot[2], f32::from_bits(0x3dd6_7752));
    let base = m.pv_f(0x58);
    if 0.0 < base {
        let ph = add_rot(m.pv_f(0x54), f32::from_bits(0x3dd6_7752));
        m.set_pv_f(0x54, ph);
        m.pos[2] = ph.sin() * 0.01 + base;
    }
    let (pos, rot) = (m.pos, m.rot);
    let lit = match (frame % 0x3c) / 0xf { 0 => Some(0), 1 => Some(1), 2 => Some(2), _ => None };
    for (k, &p) in parts.iter().enumerate() {
        let pm = &mut w.mobys[p];
        pm.pos = pos;
        pm.rot = rot;
        pm.glow = if lit == Some(k) { 0x8040_40ff } else { 0x8000_0040 };
        pm.drive = true;
        w.rebuild(p, &*ctx.host);
    }
    w.changed = true;
    if show(w, id, &parts) { glow(w, id, ctx, 0x14); }
}

/// 0x0ab8 (`0x3fa730`; L11 `0x3e2770`, the same code): while its gate moby (`+0x68`[2]) is in state 2 or 3 (or it
/// has none). **0**: its timer 0, update distance 0xff, draw distance 0x7f, the glow's set-up ([`glow_setup`],
/// `+0x10`); 1. Then shown or hidden by its timer `+0x68`[0] ([`show`]), the glow while shown ([`glow`]).
pub fn update_0ab8(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if !gate(w, id, 2) { return; }
    if w.mobys[id].state == 0 {
        let m = &mut w.mobys[id];
        m.set_pv_i(0, 0);
        m.update_dist = 0xff;
        m.draw_dist = 0x7f;
        m.state = 1;
        glow_setup(w, id, ctx, 0x10);
    }
    if show(w, id, &[]) { glow(w, id, ctx, 0x10); }
}
