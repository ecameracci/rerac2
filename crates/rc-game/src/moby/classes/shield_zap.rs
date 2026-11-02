//! The upgraded Shield Charger's zap **0x1195** (`0x414628`, made by `0x4144f8`; its one caller the upgraded shield's
//! update, crate::moby::classes::shield): a bolt from Ratchet to an enemy about him.
//!
//! **Made** ([`spawn`], `0x4144f8(target)`): state 1, far 0xff, its sound none (`+0x18`), 15 ticks (`+0x1e`,
//! `$gp+0x238c`), its rows Ratchet's angles' (`0x189eb0`), the next of four pairs of point buffers (`0x25a120` 10 points
//! and `0x25a3a0` 20 points, `$gp+0x2328` cycling; [`Mem`]) cleared to Ratchet's place (`0x414798`), its reach's wait 6
//! (`+0x1c`), its angles and scroll 0 (`+0x08..+0x14`), its colour `0x7f7f4020` (`$gp+0x232c`, `+0x24`), the target
//! (`+0x20`); at Ratchet's place 0.75 up his up row.
//!
//! **Each tick** (`0x414628`): its ticks out, deleted. Else at Ratchet's place 0.75 up his up row; a target dead (state
//! −2 / −3) is dropped; a live one gives the bolt's end (`+0x30`: its place, raised along its up by its aim height). Then
//! (`0x414858`): with the reach's wait counting down, the target takes the row's damage the tick it runs out (flags
//! 0x810000, along the bolt, kind `0x1195 << 16 | 0x0203`); the bolt reaches `(d + (0.01 − d)·wait/6)` of the way (d
//! the distance to its end), split in tenths. Its angles step (10° a tick; −0.8029; +0.0524). The ten points of the
//! first buffer along the bolt, each (but the ends) 0.1·sin(a + 36°·i) + ±0..0.2 off it along its up; the first 19 of
//! the second, 0.75·sin(c + 0.2094·i)·sin(b + 0.9948·i) + ±0..0.2 up and 0.5·cos(b + 0.9948·i) across (its last point
//! stays where the make put it, at the bolt's start). Its draw is asked for. Reached, five pairs of twinkles at its end
//! (crate::particles::twinkle: 0.4..0.8 large, 15 ticks, its colour and `0x307f7f7f`, flying 1..6 a second a random
//! way) and one more (0.15..3, 12 ticks, still).
//!
//! **Its draw** (`0x414d70`): its scroll −0.2 a tick (back 8 past −8); the two buffers as strips ([`strip`],
//! `0x414e18`: in the bolt's frame — along it, across it and Ratchet's up — facing the camera, FX 0x17 0.025 across in
//! `0x7f7f7f` and FX 0xe 0.25 across in its colour scrolling): the first at alpha 0x80 / 0x40, the second at 0x10 / 0x10
//! fading in over its first segment and out over its next-to-last.
//!
//! Not ported: the weapon mods' effects on a hit, the GS test register the draw sets (`0x37c948`), the loop sound
//! `+0x18` (never started: `0x415518` returns at once; only let go when the state is not 1, which it always is).

use crate::hero::items::{row, SHIELD};
use crate::hero::physics::{add, cross, dist, scale, set_len, sub, sub_rot, add_rot, unit, V3};
use crate::moby::hits::HitTemplate;
use crate::moby::{countdown, Ctx, MobyWorld};

/// The zap's class.
pub const CLASS: i16 = 0x1195;
/// The points of the two buffers.
const SHORT: usize = 10;
const LONG: usize = 20;
/// `$gp+0x232c`: its colour.
const COLOUR: u32 = 0x7f7f_4020;

/// The four pairs of point buffers (`0x25a120 + 0xa0·k`, `0x25a3a0 + 0x140·k`) and the next (`$gp+0x2328`), local to
/// the bolt's frame (x along it).
#[derive(Clone, Debug, Default)]
pub struct Mem {
    pub short: [[V3; SHORT]; 4],
    pub long: [[V3; LONG]; 4],
    pub next: usize,
}

/// Ratchet's place 0.75 up his up row (`0x189ea0` + `0x18c0b0`'s row 2).
fn origin(w: &MobyWorld, ctx: &Ctx) -> V3 {
    let up = ctx.hero_moby.and_then(|h| w.mobys.get(h)).map_or([0.0, 0.0, 1.0], |h| h.rows[2]);
    add(ctx.hero.pos, scale(up, 0.75))
}

/// `0x4144f8(target)` (module docs).
pub fn spawn(w: &mut MobyWorld, ctx: &mut Ctx, target: usize) -> Option<usize> {
    w.class(CLASS)?;
    let k = w.shield.zap.next.min(3);
    w.shield.zap.next = (k + 1) % 4;
    w.shield.zap.short[k] = [[0.0; 3]; SHORT];
    w.shield.zap.long[k] = [[0.0; 3]; LONG];
    let at = origin(w, ctx);
    let id = w.create(CLASS);
    let rot = ctx.hero_moby.and_then(|h| w.mobys.get(h)).map_or([0.0; 3], |h| h.rot);
    let m = &mut w.mobys[id];
    m.pvars.resize(crate::moby::PVAR_BYTES, 0);
    m.state = 1;
    m.update_dist = 0xff;
    m.set_pv_i(0, k as i32);
    m.set_pv_i(0x18, -1);
    m.set_pv_s(0x1e, 0xf);
    m.rot = rot;
    m.rows = crate::hero::physics::euler_rows(rot);
    m.set_pv_i(0x24, COLOUR as i32);
    m.set_pv_i(0x20, target as i32);
    m.pos = at;
    // `0x414798`.
    m.set_pv_f(8, 0.0);
    m.set_pv_s(0x1c, 6);
    m.set_pv_f(0xc, 0.0);
    m.set_pv_f(0x10, 0.0);
    m.set_pv_f(0x14, 0.0);
    Some(id)
}

/// `0x414628` (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].pvars.resize(crate::moby::PVAR_BYTES, 0);
    let mut life = w.mobys[id].pv_s(0x1e) as i32;
    let out = countdown(&mut life);
    w.mobys[id].set_pv_s(0x1e, life as i16);
    if out {
        w.delete(id);
        return;
    }
    w.mobys[id].pos = origin(w, ctx);
    if let Some(t) = w.mobys[id].pv_link(0x20) {
        match w.mobys.get(t) {
            Some(m) if m.state != 0xfe && m.state != 0xfd => {
                let mut end = m.pos;
                if let Some((_, h)) = crate::moby::aim::health(w, t) { end = add(end, scale(m.rows[2], h)); }
                w.mobys[id].set_pv_v(0x30, end);
            }
            _ => w.mobys[id].set_pv_i(0x20, 0),
        }
    }
    if w.mobys[id].state == 1 { tick(w, id, ctx); }
}

/// `0x414858` (module docs).
fn tick(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (pos, end) = (w.mobys[id].pos, w.mobys[id].pv_v(0x30));
    let d = dist(pos, end);
    let mut wait = w.mobys[id].pv_s(0x1c) as i32;
    if wait != 0 && countdown(&mut wait) {
        w.mobys[id].set_pv_s(0x1c, wait as i16);
        if let Some(t) = w.mobys[id].pv_link(0x20).filter(|&t| w.mobys.get(t).is_some()) {
            let damage = row(ctx.game, SHIELD).map_or(5.0, |r| r.damage);
            let v = set_len(sub(end, pos), 1.0);
            let mut tm = HitTemplate::new(damage, Some(id), 0x81_0000, [v[0], v[1], v[2], 5627.925]);
            tm.kind = (CLASS as u16 as u32) << 16 | 0x0203;
            w.deliver_hit_any(t, &tm, [0.0; 4]);
        }
    }
    w.mobys[id].set_pv_s(0x1c, wait as i16);
    let step = (d + (0.01 - d) * (wait as f32 / 6.0)) / 10.0;
    let deg = 0.017_453_294;
    let (a, b, c) = {
        let m = &mut w.mobys[id];
        let a = add_rot(m.pv_f(8), 10.0 * deg);
        let b = sub_rot(m.pv_f(0xc), f32::from_bits(0x3f4d_87ae));
        let c = add_rot(m.pv_f(0x10), f32::from_bits(0x3d56_7770));
        m.set_pv_f(8, a);
        m.set_pv_f(0xc, b);
        m.set_pv_f(0x10, c);
        (a, b, c)
    };
    let k = w.mobys[id].pv_i(0) as usize % 4;
    let g = &mut ctx.game.rng;
    for i in 0..SHORT {
        let z = if i == 0 || i == SHORT - 1 { 0.0 } else { 0.1 * (a + i as f32 * (36.0 * deg)).sin() + g.randf_signed(0.0, 0.2) };
        w.shield.zap.short[k][i] = [step * i as f32, 0.0, z];
    }
    for i in 0..LONG - 1 {
        let (u, v) = (b + i as f32 * f32::from_bits(0x3f7e_adb0), c + i as f32 * f32::from_bits(0x3e56_7770));
        let z = 0.75 * v.sin() * u.sin() + g.randf_signed(0.0, 0.2);
        w.shield.zap.long[k][i] = [step * i as f32, 0.5 * u.cos(), z];
    }
    w.register_draw(id);
    if wait < 1 {
        let colour = w.mobys[id].pv_i(0x24) as u32;
        let Some(sys) = ctx.parts.as_deref_mut() else { return };
        for _ in 0..5 {
            let f = g.randf(1.0, 6.0) * 0.016_666_668;
            let (yaw, pitch) = (g.rand_angle(), g.rand_angle());
            let v = super::shot::polar(f, yaw, pitch);
            let mut spin = g.rand_range(1, 4);
            if g.randi(2) != 0 { spin = -spin; }
            let size = g.randf(0.4, 0.8);
            crate::particles::twinkle(sys, g, size * 0.3, size, 0.0, end, 0xf, colour | 0x7f00_0000, 0, spin as i8, v, 0);
            crate::particles::twinkle(sys, g, size * 0.15, size * 0.5, 0.0, end, 0xf, 0x307f_7f7f, 0, -spin as i8, v, 0);
        }
        let mut spin = g.rand_range(1, 8);
        if g.randi(2) != 0 { spin = -spin; }
        let size = g.randf(0.15, 3.0);
        crate::particles::twinkle(sys, g, size * 0.1, size, 0.0, end, 0xc, colour | 0x7f00_0000, 0, spin as i8, [0.0; 3], 0);
    }
}

/// `0x414d70` (module docs).
pub fn draw(w: &mut MobyWorld, id: usize, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let s = w.mobys[id].pv_f(0x14) - 0.2;
    w.mobys[id].set_pv_f(0x14, if s <= -8.0 { s + 8.0 } else { s });
    let k = w.mobys[id].pv_i(0) as usize % 4;
    let short = w.shield.zap.short[k];
    let long = w.shield.zap.long[k];
    strip(w, id, &short, 0x80, 0x40, false, cam, out);
    strip(w, id, &long, 0x10, 0x10, true, cam, out);
}

/// `0x414e18(zap, points, alpha, glow alpha, n)` (module docs): `fade` the second buffer's (its glow alpha
/// `$gp+0x2380`).
#[allow(clippy::too_many_arguments)]
fn strip(w: &MobyWorld, id: usize, local: &[V3], alpha: u32, glow: u32, fade: bool, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let m = &w.mobys[id];
    let (pos, end, colour, s) = (m.pos, m.pv_v(0x30), m.pv_i(0x24) as u32, m.pv_f(0x14));
    // The bolt's frame: along it, `up × along`, `along × that`.
    let f = unit(sub(end, pos));
    let r = unit(cross(m.rows[2], f));
    let u = cross(f, r);
    let pts: Vec<V3> = local.iter().map(|p| add(pos, add(add(scale(f, p[0]), scale(r, p[1])), scale(u, p[2])))).collect();
    let n = pts.len();
    let (wa, wb) = (f32::from_bits(0x3ccc_cccd), f32::from_bits(0x3ccc_cccd) * 10.0);
    let across = |i: usize| {
        let d = unit(sub(pts[i + 1], pts[i]));
        unit(cross(d, sub(cam, pts[i])))
    };
    let edge = |p: V3, c: V3, w: f32| (add(p, scale(c, w)), sub(p, scale(c, w)));
    let ca = 0x7f_7f7f | alpha << 24;
    let cb = colour & 0xff_ffff | glow << 24;
    let mut c = across(0);
    let (mut a, mut b) = (edge(pts[0], c, wa), edge(pts[0], c, wb));
    let st_a = [[1.0, 0.0], [1.0, 1.0], [0.0, 0.0], [0.0, 1.0]];
    let st_b = [[s, 0.0], [s, 1.0], [s + 1.0, 0.0], [s + 1.0, 1.0]];
    for i in 1..n {
        // The last point keeps the way before it.
        if i != n - 1 { c = across(i); }
        let (na, nb) = (edge(pts[i], c, wa), edge(pts[i], c, wb));
        let (mut a0, mut a1, mut b0, mut b1) = (ca, ca, cb, cb);
        if fade && i == 1 {
            (a0, b0) = (ca & 0xff_ffff, cb & 0xff_ffff);
        } else if fade && i == n - 2 {
            (a1, b1) = (ca & 0xff_ffff, cb & 0xff_ffff);
        }
        out.push(crate::fx::FxQuad { fx: 0x17, additive: true, corners: [a.0, a.1, na.0, na.1], st: st_a, rgba: [a0, a0, a1, a1] });
        out.push(crate::fx::FxQuad { fx: 0xe, additive: true, corners: [b.0, b.1, nb.0, nb.1], st: st_b, rgba: [b0, b0, b1, b1] });
        (a, b) = (na, nb);
    }
}
