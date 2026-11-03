//! The small critters' walker (L04 `0x33dda8` / `0x33de20` / `0x33da80` / `0x33d640`; level 4's 0x0fb6,
//! crate::moby::classes::level04::skitter): a block in the pvars that walks its moby along its facing toward a goal,
//! with its own ledge, slope and push-out tests, apart from the framework's movement ([`crate::moby::ai`]).
//!
//! The block: `+0x00` its radius (×1024, an integer), `+0x04` the height its spheres and lines start at, `+0x08` /
//! `+0x0c` the step up / down, `+0x10` its speed, `+0x18` its fall speed, `+0x1c` the speed's step, `+0x20` its brake
//! (a share of the speed), `+0x24` the most speed, `+0x28` (1.57, unread), `+0x2c` the stop distance, `+0x30` how far
//! under its place the ground is looked for, `+0x38` options (1 no fall, 4 the heading `+0x4c` instead of its facing,
//! 8 a missing ground only stops it, 0x10 the queries' flag 0x20, 0x20 a moby is no ground).
//!
//! The flags [`step`] returns: 1 pushed, 2 blocked (moved less than a fifth of its speed) or stopped, 4 at the goal,
//! 8 / 0x20 a drop or a steep slope ahead (0x22 together).

use crate::collision::{coll_line_m, coll_sphere_m, CollOutput, QueryFlags};
use crate::hero::physics::{add, atan2, dot, len, len2, scale, set_len, sub, V3};
use crate::moby::{Ctx, MobyWorld};

/// `0x33dda8(b)`: the block's defaults (radius 0.625, height 1, steps 0.5, step 0.3, brake 0.3, most 2.7, stop 0.1).
pub fn init(w: &mut MobyWorld, id: usize, b: usize) {
    let m = &mut w.mobys[id];
    m.set_pv_i(b, 0x280);
    m.set_pv_f(b + 0x2c, 0.1);
    m.set_pv_f(b + 4, 1.0);
    m.set_pv_f(b + 0xc, 0.5);
    m.set_pv_f(b + 0x24, f32::from_bits(0x402c_cccd));
    m.set_pv_f(b + 0x1c, f32::from_bits(0x3e99_999a));
    m.set_pv_f(b + 0x28, f32::from_bits(0x3fc8_f5c3));
    m.set_pv_f(b + 8, 0.5);
    for k in [0x14, 0x10, 0x18] { m.set_pv_f(b + k, 0.0); }
    m.set_pv_f(b + 0x20, f32::from_bits(0x3e99_999a));
}

/// The slope of a face (`0x310178(n.z, |n| across)`): its angle from level.
fn slope(o: &CollOutput) -> f32 { atan2(o.normal[2], len2(o.normal)) }

/// `0x33d640(up, down, p, m, no_moby)`: the ground under `p`: the line from `up` above it to `down` below (not under
/// 0), flags 0x22; a moby's primitive (or with `no_moby` any moby) is none.
fn ground(ctx: &Ctx, id: usize, up: f32, down: f32, p: V3, no_moby: bool) -> Option<CollOutput> {
    let a = [p[0], p[1], p[2] + up];
    let b = [p[0], p[1], (p[2] - down).max(0.0)];
    let o = ctx.coll.and_then(|c| coll_line_m(c, ctx.scene, a, b, QueryFlags(0x22), Some(id)))?;
    if o.kind < 0 || (no_moby && o.moby.is_some()) { return None; }
    Some(o)
}

/// `0x33e2e0(m, b)`: [`step`] at scale 1 toward a point 100 along its facing.
pub fn ahead(w: &mut MobyWorld, id: usize, ctx: &Ctx, b: usize) -> u32 {
    let m = &w.mobys[id];
    let y = m.rot[2];
    let goal = add(m.pos, [y.cos() * 100.0, y.sin() * 100.0, 0.0]);
    step(w, id, ctx, 1.0, b, goal)
}

/// `0x33de20(k, m, b, goal)`: the speed eased toward the goal (its step × `k`, at most the distance's share; within ten
/// ticks of it the brake; within ± the most × `k`), along its heading, falling unless option 1; the ground under the
/// next place (from `+0x30` under it): none, or a slope over 0.5 rad there and at 0.57 of the way (moving more than
/// 0.1), stops it (option 8: unless the ground is no higher than 0.04 over its feet) — then it falls only. Else the
/// move ([`push`]); its ground found, the fall reset; pushed off the ground's step (`+0x08` / `+0x0c`), back where it
/// was.
pub fn step(w: &mut MobyWorld, id: usize, ctx: &Ctx, k: f32, b: usize, goal: V3) -> u32 {
    let m = &mut w.mobys[id];
    let (accel, most) = (m.pv_f(b + 0x1c) * k, m.pv_f(b + 0x24) * k);
    let dist = len2(sub(goal, m.pos));
    let start = m.pos;
    let a = if accel * dist <= accel { accel * dist } else { accel };
    let mut s = m.pv_f(b + 0x10) + a;
    if dist < s * 10.0 { s -= m.pv_f(b + 0x20) * s; }
    let s = s.clamp(-most, most);
    m.set_pv_f(b + 0x10, s);
    let opts = m.pv_i(b + 0x38) as u32;
    let yaw = if opts & 4 == 0 { m.rot[2] } else { m.pv_f(b + 0x4c) };
    let vz = if opts & 1 == 0 {
        let z = m.pv_f(b + 0x18);
        m.set_pv_f(b + 0x18, z - 0.0025);
        z
    } else {
        0.0
    };
    let v = [yaw.cos() * s, yaw.sin() * s, vz];
    let top = m.pos[2] + 0.04;
    let mut next = add(m.pos, v);
    next[2] = m.pos[2] - m.pv_f(b + 0x30);
    let (up, down, below) = (m.pv_f(b + 8), m.pv_f(b + 0xc), m.pv_f(b + 0x30));
    let no_moby = opts & 0x20 != 0;
    // `uVar10` the flags, `stop` its `uVar3 & 2` (labels as the original's `0x33e114` / `0x33e144`).
    let mut out = 0u32;
    let mut stop: Option<bool> = None;
    match ground(ctx, id, up, down, next, no_moby) {
        None => out = if opts & 8 != 0 { 8 } else { 0x22 },
        Some(g) if 0.5 < slope(&g) && 0.1 < len2(v) => {
            let first = g.point[2];
            let at_e114 = |out: &mut u32| { *out = 0x22; if top <= first { Some(true) } else { *out = 8; None } };
            let mut p2 = add(scale(v, f32::from_bits(0x3f11_eb85)), start);
            p2[2] = start[2] - below;
            match ground(ctx, id, up, down, p2, no_moby) {
                None => {
                    out = 0x22;
                    if opts & 8 != 0 { stop = at_e114(&mut out); }
                }
                Some(g2) if slope(&g2) < 0.5 => stop = Some(false),
                Some(g2) => {
                    out = 0x22;
                    if opts & 8 != 0 { stop = if top <= g2.point[2] { Some(true) } else { at_e114(&mut out) }; }
                }
            }
        }
        Some(_) => {}
    }
    let stop = stop.unwrap_or(out & 2 != 0);
    if !stop {
        let sp = len2(v);
        let dir = scale(v, if sp != 0.0 { 1.0 / sp } else { 0.0 });
        let r = w.mobys[id].pv_i(b) as f32 / 1024.0;
        let (h, flags) = (w.mobys[id].pv_f(b + 4), opts);
        let (res, moved) = push(w, id, ctx, h, r, below, v, flags);
        let m = &mut w.mobys[id];
        if res & 2 != 0 { m.set_pv_f(b + 0x18, f32::from_bits(0xbc23_d70a)); }
        if res & 1 != 0 {
            out |= if dot(moved, dir) < sp * 0.2 { 3 } else { 1 };
            let g = crate::moby::ai::ground_z(ctx, 0.5, w.mobys[id].pos, 0);
            if up < start[2] - g || down < g - start[2] {
                out |= 0x22;
                w.mobys[id].pos = start;
            }
        }
    } else if opts & 1 == 0 {
        w.mobys[id].set_pv_f(b + 0x18, f32::from_bits(0xbc23_d70a));
    }
    if dist < w.mobys[id].pv_f(b + 0x2c) { out |= 4; }
    out
}

/// `0x33da80(h, r, down, m, v, flags)`: the move by `v`: a move longer than `r`, a line `h` up to the next place
/// stopping `r` short of what it meets; six push-outs of a sphere of `r` at `h` up (flags | 4); the ground line from
/// `h` up to `down` + 0.05 under (flags | 2): a face of the world there (2; over 0.5 rad 6) lifts it toward its
/// height by 0.3. The flags' 0x20 with option 0x10. Returns the flags (1 pushed) and the move made.
#[allow(clippy::too_many_arguments)]
fn push(w: &mut MobyWorld, id: usize, ctx: &Ctx, h: f32, r: f32, down: f32, v: V3, opts: u32) -> (u32, V3) {
    let f = (opts & 0x10) << 1;
    let start = w.mobys[id].pos;
    let mut pos = add(start, v);
    let mut out = 0;
    let Some(c) = ctx.coll else { w.mobys[id].pos = pos; return (0, v) };
    if r < len(v) {
        let d = set_len(v, r);
        let a = [start[0], start[1], start[2] + h];
        let mut e = add(d, pos);
        e[2] += h;
        if let Some(o) = coll_line_m(c, ctx.scene, a, e, QueryFlags(f), Some(id)) {
            pos = sub(o.point, d);
            out = 1;
            pos[2] -= h;
        }
    }
    for _ in 0..6 {
        let at = [pos[0], pos[1], pos[2] + h];
        let Some(o) = coll_sphere_m(c, ctx.scene, at, r, QueryFlags(f | 4), Some(id)) else { break };
        pos = o.pushed_centre.unwrap_or(at);
        out = 1;
        pos[2] -= h;
    }
    let floor = pos[2] - down;
    let a = [pos[0], pos[1], pos[2] + h];
    let b = [pos[0], pos[1], floor - 0.05];
    let mut res = out;
    if let Some(o) = coll_line_m(c, ctx.scene, a, b, QueryFlags(f | 2), Some(id)) {
        if 0 < o.kind {
            res |= 2;
            if 0.5 < slope(&o) { res |= 6; }
            if floor < o.point[2] { pos[2] += (o.point[2] - floor) * 0.3; }
        }
    }
    w.mobys[id].pos = pos;
    (res, sub(pos, start))
}
