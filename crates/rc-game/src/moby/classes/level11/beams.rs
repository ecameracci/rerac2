//! Level 11's electric fence **0x0ee0** (`0x46d4a8`, 3 placed; the Hacker's gate's beams, deleted by
//! crate::moby::classes::level11::props::update_117e): two crackling strands along its path, the path's segments
//! hurting what they meet.
//!
//! Its pvars: `+0x00` / `+0x10` the ends of its line (its place, and 6..177 along y from it), `+0x20` the line's
//! length, `+0x2c` the points drawn, `+0x34` its path, `+0x38` its strands' slot (the beam buffer `0x2647e0`, 0x280 a
//! slot: the strand `+0x000` and the glow strand `+0x140`, 20 points each; kept here in [`Strands`]), `+0x3c` on
//! anywhere. The strands' phases and counts are the level's (`$gp+0x3600..0x3694`, [`super::Globals`]).
//!
//! Each tick (on levels 4 and 0xb, or anywhere once `+0x3c` is set, which a slot of 20 or more sets; with a slot
//! under 20 or Ratchet at 85 or lower):
//! * its line's second end 6..177 out along y (`+0x20 += (6 − it)·0.07 + 12`), the points 20 (on levels 4 and 0xb,
//!   else `randi(20)`);
//! * `0x46d6c0`: once (`$gp+0x3604`), the strand laid straight along its line in steps of 15 / points and the phases
//!   0;
//! * `0x46d7d0` (the crackle): the phases `$gp+0x3620` / `+0x3628` a random angle each, `+0x363c` on by 3° a tick;
//!   the strand at the path's points, each but the first moved by `±(0..0.3)` along z, x and y (`0x32ddc0`); the glow
//!   strand at the same points plus `0.8·sin(+0x363c + 12°·k)·sin(+0x3628 + 57°·k) ± (0..0.1)` up and
//!   `0.5·cos(+0x3628 + 57°·k)` across the strand; past the second point, at every second one and the last, the
//!   path's segment from two points back hurts what it meets (`0x2f0678(a, b, 0x10, m, hit)`: 10, flags 1);
//! * the strand's last four points at the path's; its draw ([`draw`], `0x46dbd0`); the first tick **1**.
//!
//! Not ported: the point light its first tick takes (`0x30fd98(0, 0, place, 0)`: radius 0, black, never set again),
//! the GS test the draw sets after (`0x391158(0x47, 0x5360b)`).

use crate::hero::physics::{add, add_rot, atan2, cross, len, scale, set_len, sub, V3};
use crate::moby::{Ctx, MobyWorld};

const POINTS: usize = 20;

/// The beam buffer's slot (`0x2647e0 + 0x280·slot`): the strand and the glow strand.
#[derive(Clone, Debug, Default)]
pub struct Strands {
    pub core: Vec<V3>,
    pub glow: Vec<V3>,
}

/// The level's fence state (`$gp+0x3600..`).
#[derive(Clone, Debug)]
pub struct Fence {
    /// `+0x3600` the points, `+0x3604` the straight lay still to do, `+0x3620` / `+0x3628` / `+0x363c` the phases,
    /// `+0x3660` the strand's texture scroll.
    pub count: i32,
    pub lay: bool,
    pub phase_a: f32,
    pub phase_b: f32,
    pub phase_c: f32,
    pub scroll: f32,
    /// By the moby: its slot's strands.
    pub strands: std::collections::HashMap<usize, Strands>,
}

impl Default for Fence {
    fn default() -> Self { Fence { count: 0x14, lay: true, phase_a: 0.0, phase_b: 0.0, phase_c: 0.0, scroll: 0.0, strands: Default::default() } }
}

/// `0x332a10`: an angle wrapped to [−π, π).
fn wrap(a: f32) -> f32 { ((a + std::f32::consts::PI) * 0.159_154_94).rem_euclid(1.0) * std::f32::consts::TAU - std::f32::consts::PI }

/// 0x0ee0 (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    if 0x14 <= m.pv_i(0x38) { m.set_pv_i(0x3c, 1); }
    let lvl = ctx.game.level;
    if m.pv_i(0x3c) == 0 && lvl != 4 && lvl != 0xb { return; }
    if 0x14 <= m.pv_i(0x38) && 85.0 < ctx.hero.pos[2] { return; }
    let a = m.pos;
    m.set_pv_v(0, a);
    let l = m.pv_f(0x20);
    let l = l + (6.0 - l) * 0.07 + 12.0;
    m.set_pv_f(0x20, l);
    let b = add(a, [0.0, l, 0.0]);
    m.set_pv_v(0x10, b);
    let count = if lvl == 4 || lvl == 0xb { 0x14 } else { ctx.game.rng.randi(0x14) };
    let n = count.clamp(0, POINTS as i32) as usize;
    let path = usize::try_from(w.mobys[id].pv_i(0x34)).ok().and_then(|p| ctx.level.paths.get(p)).cloned().unwrap_or_default();
    let point = |k: usize| -> V3 { path.get(k).map_or([0.0; 3], |p| [p[0], p[1], p[2]]) };
    w.mobys[id].set_pv_i(0x2c, count);
    let f = &mut w.level11.fence;
    f.count = count;
    let s = f.strands.entry(id).or_insert_with(|| Strands { core: vec![[0.0; 3]; POINTS], glow: vec![[0.0; 3]; POINTS] });
    // `0x46d6c0`.
    if f.lay {
        f.lay = false;
        let step = set_len(sub(b, a), 15.0 / count.max(1) as f32);
        s.core[0] = a;
        for k in 1..n { s.core[k] = add(s.core[k - 1], step); }
        f.phase_a = 0.0;
        f.phase_b = 0.0;
        f.phase_c = 0.0;
    }
    // `0x46d7d0`.
    let rng = &mut ctx.game.rng;
    f.phase_a = rng.rand_angle();
    f.phase_b = rng.rand_angle();
    f.phase_c = add_rot(f.phase_c, f32::from_bits(0x3d56_7770));
    s.core[0] = point(0);
    s.glow[0] = point(0);
    let mut lines = Vec::new();
    for k in 1..n {
        let (j1, j2, j3) = (rng.randf_signed(0.0, 0.3), rng.randf_signed(0.0, 0.3), rng.randf_signed(0.0, 0.3));
        let mut p = point(k);
        p[2] += j1;
        p[0] += j2;
        p[1] += j3;
        s.core[k] = p;
        let d = sub(s.core[k], s.core[k - 1]);
        let u3 = wrap(f.phase_b + k as f32 * f32::from_bits(0x3f7e_adb0));
        let u4 = wrap(f.phase_c + k as f32 * f32::from_bits(0x3e56_7751));
        let up = 0.8 * u4.sin() * u3.sin() + rng.randf_signed(0.0, 0.1);
        let yaw = add_rot(atan2(d[0], d[1]), f32::from_bits(0x3fc9_0fdc));
        let across = 0.5 * u3.cos();
        s.glow[k] = add(add(p, [0.0, 0.0, up]), [yaw.cos() * across, yaw.sin() * across, 0.0]);
        if 1 < k && (k == n - 1 || k & 1 == 0) { lines.push((point(k - 2), point(k))); }
    }
    for k in n.saturating_sub(4)..n { s.core[k] = point(k); }
    for (p0, p1) in lines {
        let t = crate::moby::hits::HitTemplate::new(10.0, Some(id), 1, [0.0; 4]);
        if let Some(o) = ctx.coll.and_then(|c| crate::collision::coll_line_m(c, ctx.scene, p0, p1, crate::collision::QueryFlags(0x10), Some(id))) {
            if let Some(t_id) = o.moby { w.deliver_hit_any(t_id, &t, [o.point[0], o.point[1], o.point[2], 1.0]); }
        }
    }
    w.register_draw(id);
    if w.mobys[id].state == 0 {
        let m = &mut w.mobys[id];
        m.state = 1;
        m.set_pv_i(0x30, -1);
    }
}

/// `0x46dbd0`: the scroll back 0.3 a frame (held above −8), then the strand (`0x46dc98(0.05, 0.4, 0.9, strand,
/// 0x80, 0x40, points − 2)`) and the glow strand (`(0.1, 0.3, 0.1, glow strand, 0x40, 0x40, points)`). Each: a ribbon
/// of FX 0xe (`$gp+0x3690`) 0.05 to either side across the view from point to point, 0x7f7f7f at the first alpha, its
/// ends faded to 0, its s scrolling; and over each segment (both ends set the camera's distance of the farther one,
/// lengthened by the third number of it at each end) a quad of FX 0x10 (`$gp+0x3694`) the second number across,
/// 0x803040 at the second alpha, its ends faded. Additive (ALPHA 0x48).
pub fn draw(w: &mut MobyWorld, id: usize, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let f = &mut w.level11.fence;
    let t = f.scroll - 0.3;
    f.scroll = if t <= -8.0 { t + 8.0 } else { t };
    let n = w.mobys[id].pv_i(0x2c);
    let Some(s) = f.strands.get(&id) else { return };
    strip(out, &s.core, n - 2, f.scroll, 0.4, 0.9, 0x80, 0x40, cam);
    strip(out, &s.glow, n, f.scroll, 0.3, 0.1, 0x40, 0x40, cam);
}

/// `0x46dc98(_, glow width, lengthen, points, core alpha, glow alpha, n)`.
#[allow(clippy::too_many_arguments)]
fn strip(out: &mut Vec<crate::fx::FxQuad>, p: &[V3], n: i32, scroll: f32, width: f32, lengthen: f32, a_core: u32, a_glow: u32, cam: V3) {
    let n = n.clamp(0, p.len() as i32) as usize;
    if n < 2 { return; }
    let core = 0x7f_7f7f | a_core << 24;
    let glow = 0x80_3040 | a_glow << 24;
    let side = |a: V3, b: V3, k: f32| { let c = cross(sub(b, a), sub(a, cam)); let l = len(c); scale(c, if l != 0.0 { k / l } else { 0.0 }) };
    let st = [[scroll, 0.0], [scroll, 1.0], [scroll + 1.0, 0.0], [scroll + 1.0, 1.0]];
    let s0 = side(p[0], p[1], 0.05);
    let (mut q0, mut q1) = (add(p[0], s0), sub(p[0], s0));
    let mut rgba = [core; 4];
    for i in 1..n - 1 {
        let s = side(p[i], p[i + 1], 0.05);
        let (a, b) = (add(p[i], s), sub(p[i], s));
        if i == 1 {
            rgba[0] = 0x7f_7f7f;
            rgba[1] = 0x7f_7f7f;
        } else if i == n - 2 {
            rgba[2] = 0x7f_7f7f;
            rgba[3] = 0x7f_7f7f;
        } else if i == 2 {
            rgba[0] = core;
            rgba[1] = core;
        }
        out.push(crate::fx::FxQuad { fx: 0xe, additive: true, corners: [q0, q1, a, b], st, rgba });
        (q0, q1) = (a, b);
    }
    let st = [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]];
    let mut rgba = [glow; 4];
    for i in 0..n - 2 {
        let (mut e, mut d) = (sub(p[i], cam), sub(p[i + 1], cam));
        let (le, ld) = (len(e), len(d));
        if ld < le {
            if ld != 0.0 { d = scale(d, le / ld); }
        } else if le != 0.0 {
            e = scale(e, ld / le);
        }
        let (mut a, mut b) = (add(cam, e), add(cam, d));
        let dd = scale(sub(b, a), lengthen);
        a = sub(a, dd);
        b = add(b, dd);
        let c = cross(sub(cam, a), sub(b, a));
        let l = len(c);
        let nn = scale(c, if l != 0.0 { width / l } else { 0.0 });
        if i == 0 {
            rgba[0] = 0x80_3040;
            rgba[1] = 0x80_3040;
        } else if i as i32 == n as i32 - 3 {
            rgba[2] = 0x80_3040;
            rgba[3] = 0x80_3040;
        } else if i == 1 {
            rgba[0] = glow;
            rgba[1] = glow;
        }
        out.push(crate::fx::FxQuad { fx: 0x10, additive: true, corners: [add(a, nn), sub(a, nn), add(b, nn), sub(b, nn)], st, rgba });
    }
}
