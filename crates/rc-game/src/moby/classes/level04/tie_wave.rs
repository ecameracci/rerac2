//! **0x1137** (L04 `0x4563a8`, 40 placed over levels 4, 6, 9, 12, 13, 14, 16 and 17; the same code at L06
//! `0x461868`, L09 `0x421060`, L12 `0x424b50`, L13 `0x423868`, L14 `0x4307c8`, L16 `0x41d1b0`, L17 `0x41de68`): a
//! tie group's lights in waves. Its words (`+0x68`[k]): the tie group `[0]` (the groups `0x1a9014`), flags `[1]` (1:
//! the ties' colour, 2: glows), the wave kind `[2]`, the wave's start, rise, hold and fall `[4..8]` (seconds × `[8]`),
//! its way `[9..0xc]` and its scale `[0xc]`, the two colours' bytes `[0xd..0x11]` / `[0x11..0x15]`, the glows' facing
//! `[0x15..0x18]`, size `[0x18]`, cone `[0x19]` / `[0x1a]` (degrees), offset `[0x1b..0x1e]` and pull `[0x1e]`, the
//! camera cuboid `[0x20]` (−1: always), the update distance `[0x21]`; its clock `[0x38]` (1/60 a tick, `[0x3a]`) and
//! the wave's period `[0x39]`.
//!
//! * **0**: without the group deleted; the angles in radians, the period the four times' sum × `[8]` (the times
//!   scaled too, `[8]` = 1), the colours packed (`[0x30]` / `[0x31]`), the update distance, the way and the facing made
//!   unit (no way: kind 0); 1.
//! * **1**: the clock on; with the camera in its cuboid each tie of the group within 32 of the camera gets the share
//!   `f` of the wave ([`wave`], `0x456bc8`) at its place's time: kind 1 along the way (the clock + the place's distance
//!   along it × `[0xc]`), 2 out from the moby (the clock + the distance), 3 a hash of its place (the clock + the
//!   period × the fraction of x + y + z), 4 a beam turning about the moby (`f` by the angle off it: 80°..85° in,
//!   85°..95° full, 95°..115° out), else 0; its colour `[0x31]` → `[0x30]` by `f` (`0x310cf0`), with flag 1 the
//!   tie's colour (`0x393b78` = L00 `0x3701d0`, crate::moby::MobyWorld::tie_colours).
//!
//! Not ported: the glows (flag 2: a particle of type 0x67, `0x364f80`, at the tie's offset pulled toward the camera,
//! faded by its facing's cone): the particle type is not in crate::particles.

use crate::hero::physics::{dist, dot, len, sub, V3};
use crate::moby::{Ctx, MobyWorld};

fn group(w: &MobyWorld, id: usize) -> Option<Vec<usize>> {
    usize::try_from(w.mobys[id].pv_i(0)).ok().and_then(|g| w.tie_groups.get(g)).filter(|g| !g.is_empty()).map(|g| g.iter().map(|&t| t as usize).collect())
}

/// `0x456ca8`: the smooth step `x²(3 − 2x)`.
fn smooth(x: f32) -> f32 { x * x * (x * -2.0 + 3.0) }

/// `0x456bc8(x, pv)`: the wave at time `x` (wrapped by the period `[0x39]`): 0 before the start, the smooth rise, 1
/// held, the smooth fall.
fn wave(w: &MobyWorld, id: usize, x: f32) -> f32 {
    let m = &w.mobys[id];
    let period = m.pv_f(0xe4);
    let x = x - ((x / period) as i32) as f32 * period;
    let (start, rise, hold, fall) = (m.pv_f(0x10), m.pv_f(0x14), m.pv_f(0x18), m.pv_f(0x1c));
    if x < start { return 0.0; }
    let x = x - start;
    if x < rise { smooth(x / rise) } else if hold <= x - rise { 1.0 - smooth((x - rise - hold) / fall) } else { 1.0 }
}

fn unit(v: V3) -> V3 { let l = len(v); if l != 0.0 { [v[0] / l, v[1] / l, v[2] / l] } else { v } }

/// 0x1137 (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 0 {
        if group(w, id).is_none() {
            w.delete(id);
            return;
        }
        let m = &mut w.mobys[id];
        let s = m.pv_f(0x20);
        m.set_pv_f(0xe8, f32::from_bits(0x3c88_8889));
        m.set_pv_f(0x64, m.pv_f(0x64) * 0.017_453_294);
        m.set_pv_f(0x68, m.pv_f(0x68) * 0.017_453_294);
        let times = [m.pv_f(0x10), m.pv_f(0x14), m.pv_f(0x18), m.pv_f(0x1c)];
        m.set_pv_f(0xe4, times[0] * s + times[1] * s + times[2] * s + times[3] * s);
        let b = |m: &crate::moby::Moby, k: usize| m.pv_i(4 * k) as u32;
        let ca = b(m, 0x10) << 24 | b(m, 0xf) << 16 | b(m, 0xe) << 8 | b(m, 0xd);
        let cb = b(m, 0x14) << 24 | b(m, 0x13) << 16 | b(m, 0x12) << 8 | b(m, 0x11);
        m.set_pv_i(0xc0, ca as i32);
        m.set_pv_f(0x20, 1.0);
        m.set_pv_f(0xe0, 0.0);
        for (k, t) in times.iter().enumerate() { m.set_pv_f(0x10 + 4 * k, t * s); }
        m.set_pv_i(0xc4, cb as i32);
        m.update_dist = m.pv_i(0x84) as u8;
        let way = m.pv_v(0x24);
        if way == [0.0; 3] { m.set_pv_i(8, 0); } else { m.set_pv_v(0x90, unit(way)); }
        let face = m.pv_v(0x54);
        m.set_pv_v(0xa0, unit(face));
        let off = m.pv_v(0x6c);
        m.set_pv_v(0xb0, off);
        m.set_state(1, -1);
    } else if w.mobys[id].state != 1 {
        return;
    }
    let m = &mut w.mobys[id];
    m.set_pv_f(0xe0, m.pv_f(0xe0) + m.pv_f(0xe8));
    let cam = ctx.host.camera_pos();
    let cub = m.pv_i(0x80);
    if cub != -1 && !ctx.level.in_cuboid(cub, cam) { return; }
    let Some(ties) = group(w, id) else { return };
    let m = &w.mobys[id];
    let (kind, t, flags) = (m.pv_i(8), m.pv_f(0xe0), m.pv_i(4));
    let (ca, cb) = (m.pv_i(0xc0) as u32, m.pv_i(0xc4) as u32);
    let (pos, rows, way) = (m.pos, m.rows, m.pv_v(0x90));
    let mut out = Vec::new();
    for tie in ties {
        let Some(mat) = w.ties.get(tie) else { break };
        let p = [mat[3][0], mat[3][1], mat[3][2]];
        if 32.0 < len(sub(cam, p)) { continue; }
        let f = match kind {
            2 => wave(w, id, t + dist(pos, p)),
            1 => wave(w, id, t + dot(p, way) * w.mobys[id].pv_f(0x30) / len(way)),
            3 => { let s = p[0] + p[1] + p[2]; wave(w, id, t + w.mobys[id].pv_f(0xe4) * (s - (s as i32) as f32)) }
            4 => {
                let (a, b) = (t * std::f32::consts::FRAC_PI_2, t * 0.5 * std::f32::consts::PI);
                let v = [b.sin() * a.cos(), b.sin() * a.sin(), b.cos()];
                let v: V3 = std::array::from_fn(|k| v[0] * rows[0][k] + v[1] * rows[1][k] + v[2] * rows[2][k]);
                let u = unit(sub(p, pos));
                let ang = std::f32::consts::FRAC_PI_2 - dot(u, v).clamp(-1.0, 1.0).asin();
                if ang <= 1.396_263_6 { 0.0 }
                else if ang <= 1.483_53 { smooth((ang - 1.396_263_6) / 0.087_266_445) }
                else if ang <= 1.658_062_9 { 1.0 }
                else if 2.007_129 < ang { 0.0 }
                else { 1.0 - smooth((ang - 1.658_062_9) / 0.349_066_02) }
            }
            _ => 0.0,
        };
        let c = crate::hud::lerp_rgba(f, cb, ca);
        if flags & 1 != 0 { out.push((tie, [c as u8 & 0xfc, (c >> 8) as u8 & 0xfc, (c >> 16) as u8 & 0xfc])); }
    }
    w.tie_colours.extend(out);
}
