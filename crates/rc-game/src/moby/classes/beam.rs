//! The beam, class **0x0dc1** (made by `0x3fe9d8`, update `0x3feb50`, draw `0x3feba0`; no model, hidden): a bolt of
//! light from its place to an end point that flares and fades over its life — the upgrade screen's lightning
//! (crate::menus::upgrade, `0x380ac0`). Its draw is the shape the Blitz Guns' tracers draw too ([`quads`]: `0x3fd908`,
//! `0x3fe250`, crate::moby::classes::blitz::draw_tracer).
//!
//! **Made** (`0x3fe9d8(w₀, w₁, w₂, w₃, at, end, life, c₀, c₁, c₂, c₃, c₄)`; pvars `+0x00` its life, `+0x04` one over
//! it, `+0x08` the texture's scroll, `+0x10..+0x1c` the widths, `+0x20..+0x2c` and `+0x3c` the colours, `+0x30` the
//! end): updated and drawn at any distance, hidden (mode 0x41), its draw asked for at once. **The update**
//! (`0x3feb50`): its life down; out, gone; else its draw. **The draw** (`0x3feba0`): at `sin(life/its life · π)` of
//! 128 the beam [`quads`].

use crate::hero::physics::{add, scale, set_len, sub, V3};
use crate::moby::{countdown, mode, Ctx, MobyWorld};

/// The class.
pub const CLASS: i16 = 0x0dc1;

/// `0x3fe9d8` (module docs): widths `w` (the beam's at its place and its middle, the glow's at its place and its end),
/// colours `c` (the beam's at its place, middle and end, the glow's at its place and end).
pub fn spawn(w: &mut MobyWorld, widths: [f32; 4], at: V3, end: V3, life: i32, c: [u32; 5]) -> Option<usize> {
    w.class(CLASS)?;
    let id = w.create(CLASS);
    let m = &mut w.mobys[id];
    m.update = Some(update);
    m.mode = (m.mode & !mode::NO_UPDATE) | 0x41;
    m.pvars.resize(crate::moby::PVAR_BYTES, 0);
    m.update_dist = 0xff;
    m.draw_dist = 0xff;
    m.drawn = 1;
    m.state = 0;
    m.pos = at;
    m.set_pv_v(0x30, end);
    m.set_pv_i(0, life);
    m.set_pv_f(4, 1.0 / life as f32);
    m.set_pv_f(8, 0.0);
    for (k, x) in widths.iter().enumerate() { m.set_pv_f(0x10 + 4 * k, *x); }
    for (k, x) in c.iter().take(4).enumerate() { m.set_pv_i(0x20 + 4 * k, *x as i32); }
    m.set_pv_i(0x3c, c[4] as i32);
    w.register_draw(id);
    w.changed = true;
    Some(id)
}

/// `0x3feb50`.
pub fn update(w: &mut MobyWorld, id: usize, _ctx: &mut Ctx) {
    let mut t = w.mobys[id].pv_i(0);
    let out = countdown(&mut t);
    w.mobys[id].set_pv_i(0, t);
    if out { w.delete(id) } else { w.register_draw(id) }
}

/// `0x3feba0`.
pub fn draw(w: &mut MobyWorld, id: usize, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let m = &mut w.mobys[id];
    let f = (m.pv_i(0) as f32 * m.pv_f(4) * std::f32::consts::PI).sin();
    let mut phase = m.pv_f(8);
    let widths = [0x10, 0x14, 0x18, 0x1c].map(|o| m.pv_f(o));
    let colours = [m.pv_i(0x20), m.pv_i(0x24), m.pv_i(0x28), m.pv_i(0x2c), m.pv_i(0x3c)].map(|c| c as u32);
    quads(out, m.pos, m.pv_v(0x30), cam, &mut phase, -0.1, ((f * 128.0) as i32 as u32) << 24, ((f * 64.0) as i32 as u32) << 24, widths, colours);
    m.set_pv_f(8, phase);
}

/// The beam from `pos` to `end` (`0x3fd908` / `0x3fe250` / `0x3feba0`): two quads of FX 0x17 (normal blend) across the
/// view to the midpoint and on to the end — `widths[0]` across at `pos`, `widths[1]` from the midpoint on — in
/// `colours[0]` → `colours[1]` → `colours[2]` (the first two at `alpha`), then one of FX 0xe (additive) from `widths[2]`
/// at `pos` to `widths[3]` at the end in `colours[3]` (at `over`) → `colours[4]`. The texture's s is `s − phase` (7 added
/// when it comes to 7 or less); the phase then moves by `scroll` (down: 4 added at 4 or less; up: 4 taken at 4 or more).
#[allow(clippy::too_many_arguments)]
pub fn quads(out: &mut Vec<crate::fx::FxQuad>, pos: V3, end: V3, cam: V3, phase: &mut f32, scroll: f32, alpha: u32, over: u32, widths: [f32; 4], colours: [u32; 5]) {
    const ST: [[f32; 2]; 4] = [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]];
    let st: [[f32; 2]; 4] = ST.map(|[s, t]| {
        let s = s - *phase;
        [if s <= 7.0 { s + 7.0 } else { s }, t]
    });
    let p = *phase + scroll;
    *phase = if scroll < 0.0 { if p <= 4.0 { p + 4.0 } else { p } } else if 4.0 <= p { p - 4.0 } else { p };
    let half = scale(sub(end, pos), 0.5);
    let mid = add(pos, half);
    let across = crate::hero::physics::cross(half, sub(pos, cam));
    let side = set_len(across, widths[0]);
    let (c0, c1) = (alpha | colours[0], alpha | colours[1]);
    let wide = set_len(across, widths[1]);
    let (m0, m1) = (add(mid, wide), sub(mid, wide));
    let quad = |out: &mut Vec<crate::fx::FxQuad>, fx, additive, corners, st, rgba| out.push(crate::fx::FxQuad { fx, additive, corners, st, rgba });
    quad(out, 0x17, false, [add(pos, side), sub(pos, side), m0, m1], st, [c0, c0, c1, c1]);
    quad(out, 0x17, false, [m0, m1, add(m0, half), add(m1, half)], st, [c1, c1, colours[2], colours[2]]);
    let tail = add(mid, half);
    let c = over | colours[3];
    let (s0, s1) = (set_len(across, widths[2]), set_len(across, widths[3]));
    quad(out, 0xe, true, [add(pos, s0), sub(pos, s0), add(tail, s1), sub(tail, s1)], ST, [c, c, colours[4], colours[4]]);
}
