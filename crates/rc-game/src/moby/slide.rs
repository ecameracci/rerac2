//! `0x31c618(up, r, ground, m, &vel, flags)`: a moby's move along its velocity against the world and the mobys (the
//! Synthenoids' drones, crate::moby::classes::synthenoid). The moby steps by `vel`; a step longer than `r` is tested
//! first by a line from its old place to `r` past the new one, both `up` high (`0x2de878`, flags `(flags & 0x10) << 1`,
//! the moby left out), stopping `r` short of what it meets; then a sphere of `r` at `up` pushes it out of what it
//! overlaps (`0x2df8d8`, those flags | 4), six times at most; then a line down from `up` above it to `ground + 0.05`
//! below (flags | 2) finds the ground: on a world face it is grounded (2; 6 on a face steeper than 0.5 rad), and when
//! it sank below that point it rises 0.3 of the way back. `vel` becomes the move made. The result: 1 when the line or
//! the sphere met something, | the ground's flags.

use crate::collision::QueryFlags;
use crate::hero::physics::{add, atan2, len, len2, set_len, sub, V3};
use crate::moby::{Ctx, MobyWorld};

/// `0x31c618` (module docs).
#[allow(clippy::too_many_arguments)]
pub fn slide(w: &mut MobyWorld, id: usize, ctx: &Ctx, up: f32, r: f32, ground: f32, vel: &mut V3, flags: u32) -> u32 {
    let f = (flags & 0x10) << 1;
    let old = w.mobys[id].pos;
    let mut pos = add(old, *vel);
    let mut out = 0;
    let line = |a: V3, b: V3, fl: u32| ctx.coll.and_then(|c| crate::collision::coll_line_m(c, ctx.scene, a, b, QueryFlags(fl), Some(id)));
    if r < len(*vel) {
        let ahead = set_len(*vel, r);
        let a = [old[0], old[1], old[2] + up];
        let mut b = add(ahead, pos);
        b[2] += up;
        if let Some(o) = line(a, b, f) {
            pos = sub(o.point, ahead);
            pos[2] -= up;
            out = 1;
        }
    }
    for _ in 0..6 {
        let c = [pos[0], pos[1], pos[2] + up];
        let Some(o) = ctx.coll.and_then(|m| crate::collision::coll_sphere_m(m, ctx.scene, c, r, QueryFlags(f | 4), Some(id))) else { break };
        pos = o.pushed_centre.unwrap_or(c);
        pos[2] -= up;
        out = 1;
    }
    let low = pos[2] - ground;
    let mut flags_out = out;
    if let Some(o) = line([pos[0], pos[1], pos[2] + up], [pos[0], pos[1], low - 0.05], f | 2).filter(|o| 0 < o.kind) {
        flags_out |= 2;
        if 0.5 < atan2(o.normal[2], len2(o.normal)) { flags_out = out | 6; }
        if low < o.point[2] { pos[2] += (o.point[2] - low) * 0.3; }
    }
    *vel = sub(pos, old);
    w.mobys[id].pos = pos;
    flags_out
}
