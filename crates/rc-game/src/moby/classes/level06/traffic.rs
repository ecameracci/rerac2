//! Level 6's traffic: the hover cars **0x086b** / **0x086d** ([`update_086b`], `0x3fef50`, 6 and 4 placed; L06
//! addresses), a group of them going round a path in a line, each its group's member. The wrench brings one down.
//!
//! Their pvars: `[0]` the path (the group's first member's is used), `[4]` the side jitter, `[8]` / `[0xc]` the speed
//! range (units a second), `[0x10]` the place along the path (in points), `[0x14]` the speed (points a tick), `[0x18]`
//! its change a tick, `[0x1c]` the room it keeps, `[0x20]` / `[0x28]` a side offset (kept, not read), `[0x30]` the
//! path, `[0x34]` the car ahead, `[0x38]` the speed it wants, `[0x40]` the path a loop, `[0x44]` / `[0x48]` the
//! light's blink timer and period, `[0x4c]` / `[0x50]` the bob's phase and turn, `[0x54]` its looped sound,
//! `[0x58]` the fall's ticks, `[0x5c]` the speed kept while stopped, `[0x60]` one over the first segment's length,
//! `[0x64]` an area that hides the group while the camera is in it (−1 none).
//!
//! * **0**: update distance 0xfe; the group set up ([`set_up`], `0x3ff400`), which puts every member in 1.
//! * **1**: the camera in the area (`0x36b6d0`): every member hidden, 4 (this tick goes on). The drive
//!   ([`drive`], `0x3ff6c0`); its class sound 0 looped. Drawn last frame only: its light (mode 0x10) blinks on for 5
//!   ticks every `[0x48]`; a hit with the flags 0x4010000 (`0x35df18`): every piece but its piece 2 (0x086b; 0x086d
//!   3) thrown (`0x36c408` = L00 `0x32b580`, crate::moby::shards::throw_all: `$gp+0xc40`, its class sound 1, the
//!   level's bolts), it drawn as that piece alone (`+0x7c`), the blast (`0x43b580`: size 1, sprites 0x19 / 0x14 /
//!   0x14, 0x14 puffs, 0x14 streaks, its light), no speed, 180 ticks, no collision, 2; the hit let go; its yaw
//!   turned π.
//! * **2**: its pitch eases toward 45° (×0.95 a tick), it sinks 0.02 a tick; one tick in four (0x086b only) a puff
//!   of smoke (`0x32c7b0` = L03 `0x2ee160`, crate::moby::classes::endako_c::dark_puffs: 1/24 out, ×2, growing to
//!   1.02, rising 1/12 a tick, 0x40f0e0d0) at (−1.5, 0, 1) in its frame; the ticks out: hidden, 3.
//! * **3**: nothing. **4**: the camera out of the area: every member shown, 1.

use crate::hero::physics::{add, add_rot, dist, V3};
use crate::moby::{shards, Ctx, MobyWorld};

/// `$gp+0xc28` / `+0xc30` / `+0xc40`: the pieces' gravity, spin and velocity ranges.
const GRAVITY: (f32, f32) = (10.0, 15.0);
const SPIN: (f32, f32) = (90.0, 360.0);
const H: [f32; 8] = [-3.0, -3.0, 7.0, 0.0, 3.0, 3.0, 10.0, 0.0];
/// `$gp+0xc18..+0xc24`: the blink period's range (ticks) and the bob's turn range (°/s).
const BLINK: (i32, i32) = (30, 90);
const BOB_TURN: (f32, f32) = (60.0, 120.0);
/// `$gp+0xc60`: the bob's height.
const BOB: f32 = 0.15;

fn path<'a>(ctx: &Ctx<'a>, i: i32) -> Option<&'a [[f32; 4]]> { usize::try_from(i).ok().and_then(|i| ctx.level.paths.get(i)).map(|p| p.as_slice()).filter(|p| 2 <= p.len()) }

/// The group's members (`0x1fb3a0[+0x21]`).
fn members(w: &MobyWorld, id: usize) -> Vec<usize> {
    usize::try_from(w.mobys[id].group as i32).ok().and_then(|g| w.groups.get(g)).cloned().unwrap_or_default()
}

/// `0x3ff400(group)`: every member of the group, from its path (its first member's `[0]`): the speed `[8]..[0xc]` a
/// second over the first segment's length, its change a tick (the range over 3600 segment lengths), the room 3.5
/// over it; spaced evenly round the path (the list's last member first along it), each behind the one before it in
/// the list (the first behind the last); the side offset ±`[4]`, the blink period, the bob's phase and turn at
/// random; drawn far (0xff), state 1, no sound.
fn set_up(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let list = members(w, id);
    let Some(&first) = list.first() else { return };
    let p = w.mobys[first].pv_i(0);
    let Some(points) = path(ctx, p) else { return };
    let p3 = |k: usize| [points[k][0], points[k][1], points[k][2]];
    let count = points.len() as f32;
    let inv = 1.0 / dist(p3(0), p3(1));
    let looped = dist(p3(0), p3(points.len() - 1)) < 5.0;
    let n = list.len();
    let mut prev = *list.last().unwrap();
    for (j, &i) in list.iter().enumerate() {
        let k = (n - 1 - j) as f32;
        let rng = &mut ctx.game.rng;
        let m = &mut w.mobys[i];
        let r = rng.randf(m.pv_f(8), m.pv_f(0xc));
        m.set_pv_f(0x14, inv * 0.016_666_668 * r);
        m.set_pv_f(0x18, inv * 0.000_277_777_8 * (m.pv_f(0xc) - m.pv_f(8)));
        m.set_pv_i(0x34, prev as i32);
        m.set_pv_f(0x60, inv);
        m.set_pv_i(0x30, p);
        m.set_pv_i(0x40, looped as i32);
        m.set_pv_f(0x5c, m.pv_f(0x14));
        m.set_pv_f(0x38, m.pv_f(0x14));
        m.set_pv_f(0x1c, inv * 3.5);
        m.set_pv_f(0x10, k / n as f32 * count);
        m.set_pv_v(0x20, [0.0; 3]);
        m.set_pv_f(0x2c, 0.0);
        m.set_pv_f(0x20, rng.randf_signed(0.0, m.pv_f(4)));
        m.set_pv_f(0x28, rng.randf_signed(0.0, m.pv_f(4)));
        m.set_pv_i(0x48, rng.rand_range(BLINK.0, BLINK.1));
        m.set_pv_f(0x4c, rng.rand_angle());
        m.set_pv_f(0x50, rng.randf_signed(BOB_TURN.0, BOB_TURN.1) * 0.000_290_888_25);
        m.state = 1;
        m.draw_dist = 0xff;
        m.set_pv_i(0x54, -1);
        prev = i;
    }
}

/// `0x3ff6c0`: on the path at its place (`0x3852f8` = crate::paths::pose: its place, pitch and yaw), bobbing 0.15
/// by the bob's phase; on along it by its speed (round the path's points), the speed a step toward the one it wants.
/// Alone, it wants a new speed in its range every tick. Behind another: wanting more than the one ahead and closer
/// than its stopping distance (the difference squared over twice the change) plus both rooms, the two swap what
/// they want; the one ahead falling (2) within three rooms: it stops (what it wanted kept in `[0x5c]`); the one
/// ahead gone (3): it wants that again and follows the one that one followed; still wanting nothing, it takes the
/// one ahead's.
fn drive(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let Some(points) = path(ctx, w.mobys[id].pv_i(0x30)) else { return };
    let count = points.len() as f32;
    let m = &mut w.mobys[id];
    let (pos, pitch, yaw, _) = crate::paths::pose(points, m.pv_f(0x10), m.pv_i(0x40) != 0);
    m.pos = pos;
    m.rot[1] = pitch;
    m.rot[2] = yaw;
    let phase = add_rot(m.pv_f(0x4c), m.pv_f(0x50));
    m.set_pv_f(0x4c, phase);
    m.pos[2] += BOB * phase.sin();
    let mut t = m.pv_f(0x10) + m.pv_f(0x14);
    if count <= t { t -= count; }
    m.set_pv_f(0x10, t);
    let s = m.pv_f(0x14);
    m.set_pv_f(0x14, if m.pv_f(0x38) < s { s - m.pv_f(0x18) } else { s + m.pv_f(0x18) });
    let ahead = usize::try_from(m.pv_i(0x34)).ok().filter(|&a| a < w.mobys.len());
    let Some(a) = ahead.filter(|&a| a != id) else {
        let m = &mut w.mobys[id];
        let r = ctx.game.rng.randf(m.pv_f(8), m.pv_f(0xc));
        m.set_pv_f(0x38, m.pv_f(0x60) * 0.016_666_668 * r);
        return;
    };
    let mut gap = w.mobys[a].pv_f(0x10) - t;
    if gap < 0.0 { gap += count; }
    let (theirs, mine) = (w.mobys[a].pv_f(0x38), w.mobys[id].pv_f(0x38));
    if theirs < mine {
        let (step, room) = (w.mobys[id].pv_f(0x18), w.mobys[id].pv_f(0x1c));
        if gap < (mine - theirs) * (mine - theirs) / (step + step) + w.mobys[a].pv_f(0x1c) + room {
            w.mobys[a].set_pv_f(0x38, mine);
            w.mobys[id].set_pv_f(0x38, theirs);
        }
    }
    let state = w.mobys[a].state;
    if state == 2 && gap < w.mobys[id].pv_f(0x1c) * 3.0 {
        let m = &mut w.mobys[id];
        m.set_pv_f(0x5c, m.pv_f(0x38));
        m.set_pv_f(0x38, 0.0);
    }
    if state == 3 {
        let next = w.mobys[a].pv_i(0x34);
        let m = &mut w.mobys[id];
        m.set_pv_f(0x38, m.pv_f(0x5c));
        m.set_pv_i(0x34, next);
    }
    let theirs = w.mobys[a].pv_f(0x38);
    let m = &mut w.mobys[id];
    if m.pv_f(0x38) == 0.0 && theirs != 0.0 { m.set_pv_f(0x38, theirs); }
}

/// Every member of the group shown or hidden (`+0x31`, mode 1).
fn show_group(w: &mut MobyWorld, id: usize, shown: bool) {
    for i in members(w, id) {
        let m = &mut w.mobys[i];
        if shown { (m.drawn, m.mode) = (1, m.mode & !1) } else { (m.drawn, m.mode) = (0, m.mode | 1) }
    }
}

/// `0x3662f8(x, y, z, m)`: a point in its frame (its rows, unscaled).
fn frame_point(w: &MobyWorld, id: usize, v: V3) -> V3 {
    let m = &w.mobys[id];
    add(std::array::from_fn(|k| v[0] * m.rows[0][k] + v[1] * m.rows[1][k] + v[2] * m.rows[2][k]), m.pos)
}

/// 0x086b / 0x086d (module docs).
pub fn update_086b(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            w.mobys[id].update_dist = 0xfe;
            set_up(w, id, ctx);
        }
        1 => {
            let area = w.mobys[id].pv_i(0x64);
            if area != -1 && ctx.level.in_area(area, ctx.host.camera_pos()) {
                show_group(w, id, false);
                w.mobys[id].set_state(4, -1);
            }
            drive(w, id, ctx);
            let h = w.mobys[id].pv_i(0x54);
            if !(h != -1 && ctx.host.sound_playing(h)) {
                let h = ctx.host.play_sound(&w.mobys[id], 0, 4);
                w.mobys[id].set_pv_i(0x54, h);
            }
            if w.mobys[id].drawn == 0 { return; }
            let m = &mut w.mobys[id];
            let mut t = m.pv_i(0x44);
            if crate::moby::countdown(&mut t) {
                if m.mode & 0x10 != 0 { m.mode &= !0x10; t = m.pv_i(0x48); } else { m.mode |= 0x10; t = 5; }
            }
            m.set_pv_i(0x44, t);
            let hit = w.hit_message(id).is_some_and(|h| h.tmpl.flags & 0x0401_0000 != 0);
            if hit {
                let keep = if w.mobys[id].o_class == 0x086b { 2 } else { 3 };
                shards::throw_all(w, id, ctx, H, SPIN, GRAVITY, 0xffff, 1, keep, 0, 0xffff);
                w.mobys[id].piece = Some(keep as u8);
                let pos = w.mobys[id].pos;
                crate::moby::blast::burst(w, ctx, pos, &crate::moby::blast::Blast { size: 1.0, bursts: [0x19, 0x14, 0x14], puffs: 0x14, streaks: 0x14, light: true, shake: false });
                let m = &mut w.mobys[id];
                m.set_state(2, -1);
                m.set_pv_f(0x38, 0.0);
                m.set_pv_i(0x58, 0xb4);
                m.set_pv_f(0x14, 0.0);
                m.has_collision = false;
                w.changed = true;
            }
            w.clear_hit(id);
            let m = &mut w.mobys[id];
            m.rot[2] = add_rot(m.rot[2], f32::from_bits(0x4049_0fdc));
        }
        2 => {
            let q = std::f32::consts::FRAC_PI_4;
            let m = &mut w.mobys[id];
            m.rot[1] = (m.rot[1] - q) * 0.95 + q;
            m.pos[2] -= 0.02;
            if ctx.game.rng.randi(4) == 0 && w.mobys[id].o_class == 0x086b {
                let at = frame_point(w, id, [-1.5, 0.0, 1.0]);
                super::super::endako_c::dark_puffs(ctx, f32::from_bits(0x3d2a_aaab), 2.0, f32::from_bits(0x3f82_8f5c), at, [0.0, 0.0, f32::from_bits(0x3daa_aaab)], 1, 0x40f0_e0d0, false);
            }
            let m = &mut w.mobys[id];
            let mut t = m.pv_i(0x58);
            let out = crate::moby::countdown(&mut t);
            m.set_pv_i(0x58, t);
            if out {
                (m.drawn, m.mode) = (0, m.mode | 1);
                m.set_state(3, -1);
            }
        }
        4 => {
            let area = w.mobys[id].pv_i(0x64);
            if !ctx.level.in_area(area, ctx.host.camera_pos()) {
                show_group(w, id, true);
                w.mobys[id].set_state(1, -1);
            }
        }
        _ => {}
    }
}
