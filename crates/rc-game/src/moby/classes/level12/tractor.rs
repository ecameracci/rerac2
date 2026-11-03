//! Level 12's Tractor Beam targets (L12 addresses; the engine functions named by their L12 addresses with the L00 /
//! L01 ones the port knows them by). Each is gone once its uid is marked (the save's `0x21f204` or its count in
//! `0x19b4a8`, crate::game::GameState::uid_count), is dragged by the beam while it is a target (mode 0x1000), the
//! drag `0x329790` = L01 `0x31bd40` (crate::moby::classes::tractor::drag: radius 1.5, its block at `+0x68`[0x50],
//! its path `[0xec]`), its glow word pulses once a second (`0x32a698`), and its platform block `[0xa0]` is written
//! with its move (`0x326080` = L00 `0x322668`, crate::moby::platform::write; crate::moby::carry_hero for Ratchet).
//!
//! * **0x0fc4** / **0x0fc5** ([`update_0fc4`], `0x41f210`, 4 and 2 placed): dragged into its cuboid `[0xe8]` it
//!   shuttles between the cuboid's two ends.
//! * **0x0fc6** / **0x0fc7** / **0x0fc8** ([`update_0fc6`], `0x41f850`, 3, 2 and 1 placed): dragged within 1 of its
//!   cuboid `[0xe0]`'s centre it slides into it and stays.
//! * **0x0f6a** ([`update_0f6a`], `0x417840`, one): once its 0x0fc6 is in place it charges it, then both blow apart.
//! * **0x0fb8** / **0x0fb9** ([`update_0fb8`] / [`update_0fb9`], `0x41e378` / `0x41e818`, one each): the same for up
//!   to three 0x0fc6.
//!
//! The engine functions: `0x322ec8` = L00 `0x31f4b0` (crate::hero::physics::ease_speed), `0x325270` = L00 `0x321858`
//! (crate::level::LevelData::in_cuboid), `0x2f2718` = L00 `0x2ef050` the xy distance, `0x365f80` = L00 `0x360698`
//! another class's sound.

use crate::hero::physics::{add, dist, dist2, dot, ease_speed, len, set_len, sub, V3};
use crate::moby::{countdown, mode, Ctx, MobyWorld};

/// The platform block's pvars offset.
const BLOCK: usize = 0xa0;
/// The port's: where 0x0fc6 keeps the handle of its state-4 sound (past the class's pvars).
const SOUND_4: usize = 0x200;

/// Gone with its uid marked (`0x21f204[uid]` ≠ 0, or its count in `0x19b4a8`).
fn uid_done(w: &MobyWorld, id: usize, ctx: &Ctx) -> bool {
    let uid = w.mobys[id].uid;
    ctx.game.uid_dead.get(uid.max(0) as usize).is_some_and(|&d| d != 0) || ctx.game.uid_count(uid) != 0
}

/// `0x32a698(a, c₀, c₁)` at the second's phase: `(frame % 60) / 60 · 2π − π`.
fn pulse(ctx: &Ctx, c0: u32, c1: u32) -> u32 {
    let a = (ctx.game.frame % 60) as f32 * std::f32::consts::TAU / 60.0 - std::f32::consts::PI;
    crate::hud::lerp_rgba(a.sin() * 0.5 + 0.5, c0, c1)
}

/// The tick's end: its platform block written with the move from `old` (its angles `old_rot` → now), Ratchet carried.
fn finish(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, old: V3, old_rot: V3) {
    w.rebuild(id, &*ctx.host);
    let m = &mut w.mobys[id];
    let (pos, rot) = (m.pos, m.rot);
    crate::moby::platform::write(m, BLOCK, sub(pos, old), old_rot, rot);
    crate::moby::carry_hero(ctx, id, old, old_rot[2], pos, rot[2]);
}

/// A cuboid's point `x` along its first row from its centre.
fn cuboid_point(ctx: &Ctx, i: i32, x: f32) -> Option<V3> {
    let c = ctx.level.cuboids.get(usize::try_from(i).ok()?)?;
    let (r, o) = (c.matrix[0], c.matrix[3]);
    Some([r[0] * x + o[0], r[1] * x + o[1], r[2] * x + o[2]])
}

/// 0x0fc4 / 0x0fc5 (`0x41f210`): pvars `[0xe0]` / `[0xe4]` two mobys it passes to `0x41d258` (an empty function),
/// `[0xe8]` its cuboid (−1 none), `[0x130]` its end (±1 along the cuboid's first row), `[0x134]` its speed, `[0x138]`
/// the wait, `[0x13c]` / `[0x140]` the acceleration and braking (°/s² style: over 3600 a tick², 3..9). **0**: mode
/// 0x1000, the drag's flag 1 (`[0xdc]`), both rates 5 (`$gp+0x2db8`); 1. **1**: the drag; once in its cuboid
/// (`0x325270`): the wait 60, not a target, the end +1, 3. **2**: toward the end's point `T` (its xy distance `d`):
/// every 30 frames while Ratchet is not on it, his displacement's share along `T − it` under 0.9 of its speed brings
/// the acceleration down 0.5 (to 3) and the braking up (to 9), else the other way; its speed eased to `d`
/// (`0x322ec8`, at most `$gp+0x2da8`/60 = 0.098); Ratchet in the cuboid (his height snapped to its centre's within
/// 2.5) ahead of it within 2 (`$gp+0x2da4` / `+0x2da0`) holds it (its speed kept from before); else it moves on its
/// xy step; within 0.01 the wait 60, the end turned round, 3. **3**: the wait out → 2. Drawn: its glow word between
/// 0x80808080 and 0x80404040 (`$gp+0x2db0`, `0x1b1da4`).
pub fn update_0fc4(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (old, old_rot) = (w.mobys[id].pos, w.mobys[id].rot);
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.mode |= mode::TARGET;
            m.set_pv_i(0xdc, m.pv_i(0xdc) | 1);
            m.state = 1;
            m.set_pv_f(0x140, 5.0);
            m.set_pv_f(0x13c, 5.0);
        }
        1 => {
            let path = w.mobys[id].pv_i(0xec);
            super::super::tractor::drag(w, id, ctx, 1.5, path);
            let c = w.mobys[id].pv_i(0xe8);
            if c != -1 && ctx.level.in_cuboid(c, w.mobys[id].pos) {
                let m = &mut w.mobys[id];
                m.set_pv_i(0x138, 0x3c);
                m.set_state(3, -1);
                m.mode &= !mode::TARGET;
                m.set_pv_f(0x130, 1.0);
            }
        }
        2 => {
            let c = w.mobys[id].pv_i(0xe8);
            let Some(t) = cuboid_point(ctx, c, w.mobys[id].pv_f(0x130)) else { return finish(w, id, ctx, old, old_rot) };
            let m = &mut w.mobys[id];
            let d = dist2(m.pos, t);
            if ctx.hero.ground_moby != Some(id) && ctx.game.frame % 0x1e == 0 {
                let share = dot(sub(m.pos, t), ctx.hero.disp).abs();
                let (mut acc, mut brk) = (m.pv_f(0x13c), m.pv_f(0x140));
                if share < m.pv_f(0x134) * 0.9 {
                    if 3.0 < acc { acc -= 0.5; }
                    if brk < 9.0 { brk += 0.5; }
                } else {
                    if acc < 9.0 { acc += 0.5; }
                    if 3.0 < brk { brk -= 0.5; }
                }
                m.set_pv_f(0x13c, acc);
                m.set_pv_f(0x140, brk);
            }
            let before = m.pv_f(0x134);
            let mut speed = before;
            ease_speed(d, m.pv_f(0x13c) * 0.000_277_777_8, m.pv_f(0x140) * 0.000_277_777_8, f32::from_bits(0x40bc_4189) * 0.016_666_668, &mut speed);
            m.set_pv_f(0x134, speed);
            let mut to = sub(t, m.pos);
            to[2] = 0.0;
            let step = set_len(to, speed);
            let mut h = ctx.hero.pos;
            let centre = ctx.level.cuboids.get(c as usize).map_or(h[2], |v| v.matrix[3][2]);
            if (h[2] - centre).abs() < 2.5 { h[2] = centre; }
            let blocked = ctx.level.in_cuboid(c, h) && 0.0 <= dot(sub(h, m.pos), step) && dist(h, m.pos) < 2.0;
            if blocked {
                m.set_pv_f(0x134, before);
            } else {
                m.pos = add(m.pos, step);
                if d < 0.01 {
                    m.set_pv_i(0x138, 0x3c);
                    m.set_pv_f(0x130, -m.pv_f(0x130));
                    m.set_state(3, -1);
                }
            }
        }
        3 => {
            let m = &mut w.mobys[id];
            let mut t = m.pv_i(0x138);
            let out = countdown(&mut t);
            m.set_pv_i(0x138, t);
            if out { m.set_state(2, -1); }
        }
        _ => {}
    }
    if w.mobys[id].drawn != 0 { w.mobys[id].glow = pulse(ctx, 0x8080_8080, 0x8040_4040); }
    finish(w, id, ctx, old, old_rot);
}

/// 0x0fc6 / 0x0fc7 / 0x0fc8 (`0x41f850`): pvars `[0xe0]` its cuboid (−1 none), `[0xf0]` its speed. Gone once its
/// uid is marked. **0**: mode 0x1000, the drag's flag 1 (`[0xdc]`), the level word `$gp+0x4d38` = 0 (read by nothing
/// ported); 1. **1**: the drag; within 1 (xy) of the cuboid's centre: not a target, its speed the drag's, 2. **2**:
/// toward the centre, its speed eased to the distance (`0x322ec8`: 1/180 up and down, at most 1/15); within 0.001: 3.
/// **3**: at its start class 0x10e9's sound 2 (`0x365f80`). Drawn: its glow word between 0x80808080 and 0x80404040
/// (0x80007f00 and 0x80004000 in 3; in 4, set by its 0x0f6a ([`update_0f6a`]), 0x8080807f and 0x8040407f with class
/// 0x10e9's sound 2 started again whenever it stops: the game keeps one handle for the level (`0x1b3d28`, its moby
/// `0x1b3d2c`), the port in the moby's [`SOUND_4`].
pub fn update_0fc6(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if uid_done(w, id, ctx) { return w.delete(id); }
    let (old, old_rot) = (w.mobys[id].pos, w.mobys[id].rot);
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.set_pv_i(SOUND_4, -1);
            m.mode |= mode::TARGET;
            m.set_pv_i(0xdc, m.pv_i(0xdc) | 1);
            m.state = 1;
        }
        1 => {
            let path = w.mobys[id].pv_i(0xec);
            super::super::tractor::drag(w, id, ctx, 1.5, path);
            if let Some(c) = cuboid_point(ctx, w.mobys[id].pv_i(0xe0), 0.0) {
                let m = &mut w.mobys[id];
                if dist2(m.pos, c) < 1.0 {
                    m.mode &= !mode::TARGET;
                    let v = len(m.pv_v(0x50));
                    m.set_pv_f(0xf0, v);
                    m.state = 2;
                }
            }
        }
        2 => {
            if let Some(c) = cuboid_point(ctx, w.mobys[id].pv_i(0xe0), 0.0) {
                let m = &mut w.mobys[id];
                let to = sub(c, m.pos);
                let d = len(to);
                let mut speed = m.pv_f(0xf0);
                ease_speed(d, f32::from_bits(0x3bb6_0b61), f32::from_bits(0x3bb6_0b61), f32::from_bits(0x3d88_8889), &mut speed);
                m.set_pv_f(0xf0, speed);
                m.pos = add(m.pos, set_len(to, speed));
                if d < 0.001 { m.set_state(3, -1); }
            }
        }
        3 => if w.mobys[id].first() { ctx.host.play_sound_of(&w.mobys[id], 0x10e9, 2, 0); },
        _ => {}
    }
    let m = &w.mobys[id];
    if m.drawn != 0 {
        let glow = match m.state {
            3 => pulse(ctx, 0x8000_7f00, 0x8000_4000),
            4 => pulse(ctx, 0x8080_807f, 0x8040_407f),
            _ => pulse(ctx, 0x8080_8080, 0x8040_4040),
        };
        w.mobys[id].glow = glow;
        let h = w.mobys[id].pv_i(SOUND_4);
        if w.mobys[id].state == 4 && !(h != -1 && ctx.host.sound_playing(h)) {
            let h = ctx.host.play_sound_of(&w.mobys[id], 0x10e9, 2, 0);
            w.mobys[id].set_pv_i(SOUND_4, h);
        }
    }
    finish(w, id, ctx, old, old_rot);
}

/// 0x0f6a (`0x417840`, one): pvars `[0]` its 0x0fc6 (−1 none), `[4]` its rise speed, `[8]` its home height, `[0xc]`
/// a timer. **0**: its home height, 1; gone once its uid is marked. **1**: its moby in state 3 (in place, `0x41fd50`)
/// or none: the timer 80, 2. **2**: the timer down; still counting (so at once): 3 with the timer 200 (`$gp+0x28b0`).
/// **3**: the timer down; at 100 its moby to state 4 (charging); out: at its moby the blast (`0x3fb0a0`: size 2,
/// sprites 0xe / 10 / 0xf, 10 puffs, 0x14 streaks, its flash ball, the shake), it hidden without collision, its moby
/// broken ([`break_away`]), its own pieces 0..5 thrown (`0x32a940` = L00 `0x327498`: 3..6 a second across and up, plus
/// 0.1 a tick back along its first row), its moby not a target and deleted, its uid counted, deleted. **4** (set by
/// nothing ported): rising 1/720 a tick more each tick with its moby; 8 over its home: 5. **5**: its uid counted,
/// deleted.
pub fn update_0f6a(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let p = usize::try_from(w.mobys[id].pv_i(0)).ok().filter(|&p| p < w.mobys.len());
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.state = 1;
            let z = m.pos[2];
            m.set_pv_f(8, z);
            if uid_done(w, id, ctx) { w.delete(id); }
        }
        1 => if p.is_none_or(|p| w.mobys[p].state == 3) {
            let m = &mut w.mobys[id];
            m.state = 2;
            m.set_pv_i(0xc, 0x50);
        },
        2 => {
            let m = &mut w.mobys[id];
            let mut t = m.pv_i(0xc);
            if !countdown(&mut t) {
                m.set_state(3, -1);
                t = 200;
            }
            m.set_pv_i(0xc, t);
        }
        3 => {
            let mut t = w.mobys[id].pv_i(0xc);
            let out = countdown(&mut t);
            w.mobys[id].set_pv_i(0xc, t);
            if !out {
                if t == 100 { if let Some(p) = p { w.mobys[p].state = 4; } }
                return;
            }
            if let Some(p) = p {
                let at = w.mobys[p].pos;
                crate::moby::blast::burst(w, ctx, at, &crate::moby::blast::Blast { size: 2.0, bursts: [0xe, 10, 0xf], puffs: 10, streaks: 0x14, light: true, shake: true });
            }
            let m = &mut w.mobys[id];
            m.has_collision = false;
            m.mode = (m.mode | 0x41) & !mode::TARGET;
            if let Some(p) = p { break_away(w, p, ctx); }
            let v = set_len(w.mobys[id].rows[0], -0.1);
            crate::moby::shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 6.0), 0x3f, v, 0xff);
            if let Some(p) = p {
                w.mobys[p].mode &= !mode::TARGET;
                w.delete(p);
            }
            let uid = w.mobys[id].uid;
            ctx.game.count_uid(uid);
            w.delete(id);
        }
        4 => {
            let m = &mut w.mobys[id];
            let v = m.pv_f(4) + 0.001_388_888_9;
            m.set_pv_f(4, v);
            m.pos[2] += v;
            let up = m.pv_f(8) + 8.0 < m.pos[2];
            if up { m.state = 5; }
            if let Some(p) = p { w.mobys[p].pos[2] += v; }
        }
        5 => {
            let uid = w.mobys[id].uid;
            ctx.game.count_uid(uid);
            w.delete(id);
        }
        _ => {}
    }
}

/// `0x41fd60(m)`: moby `m` broken away from Ratchet: its pieces 0..4 thrown (`0x32a940`: 6..12 a second across and
/// up, plus 0.1 a tick along the line from him flattened into its own xy plane, `0x3290b0`), its uid counted, its
/// class sound 1.
fn break_away(w: &mut MobyWorld, p: usize, ctx: &mut Ctx) {
    let m = &w.mobys[p];
    let d = sub(m.pos, ctx.hero.pos);
    let r = m.rows;
    let local = [dot(d, r[0]), dot(d, r[1])];
    let flat: V3 = std::array::from_fn(|k| local[0] * r[0][k] + local[1] * r[1][k]);
    let v = set_len(flat, 0.1);
    crate::moby::shards::break_into(w, p, ctx, (6.0, 12.0), (6.0, 12.0), 0x1f, v, 0xff);
    let uid = w.mobys[p].uid;
    ctx.game.count_uid(uid);
    ctx.host.play_sound(&w.mobys[p], 1, 0);
}

/// The three mobys of 0x0fb8 / 0x0fb9 (`+0x68`[0..3], −1 none).
fn links(w: &MobyWorld, id: usize) -> Vec<usize> {
    (0..3).filter_map(|k| usize::try_from(w.mobys[id].pv_i(4 * k)).ok()).filter(|&p| p < w.mobys.len()).collect()
}

/// The shared states of 0x0fb8 / 0x0fb9: **1** all its mobys in place (state 3) → 2; **2** the timer `t`, 3; **4**
/// rising 1/720 a tick more each tick with its mobys, 8 over its home → 5; **5** its mobys no longer targets and
/// deleted, its uid counted, deleted. True when the state was one of these.
fn charger_common(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, t: i32) -> bool {
    let ps = links(w, id);
    match w.mobys[id].state {
        1 => if ps.iter().all(|&p| w.mobys[p].state == 3) { w.mobys[id].state = 2 },
        2 => {
            w.mobys[id].set_state(3, -1);
            w.mobys[id].set_pv_i(0x14, t);
        }
        4 => {
            let m = &mut w.mobys[id];
            let v = m.pv_f(0xc) + 0.001_388_888_9;
            m.set_pv_f(0xc, v);
            m.pos[2] += v;
            if m.pv_f(0x10) + 8.0 < m.pos[2] { m.state = 5; }
            for &p in &ps { w.mobys[p].pos[2] += v; }
        }
        5 => {
            for &p in &ps {
                w.mobys[p].mode &= !mode::TARGET;
                w.delete(p);
            }
            let uid = w.mobys[id].uid;
            ctx.game.count_uid(uid);
            w.delete(id);
        }
        _ => return false,
    }
    true
}

/// 0x0fb8 (`0x41e378`, one): 0x0f6a's charge for up to three 0x0fc6 ([`charger_common`]). **0**: its home height, 1;
/// its uid marked: gone. **3**: the timer (180, `$gp+0x2d60`) down; at half its mobys to state 4 (charging); out: it
/// hidden without collision, its pieces thrown (`0x32a940`: 3..6 a second across and up, 0.1 a tick back along its
/// first row), each moby broken ([`break_away`]) under a blast (`0x3fb0a0`: size 2, sprites 0xe / 10 / 0xf, 10 puffs,
/// 0x14 streaks, its flash ball, the shake) and deleted, its uid counted, deleted.
pub fn update_0fb8(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.state = 1;
            let z = m.pos[2];
            m.set_pv_f(0x10, z);
            if uid_done(w, id, ctx) { w.delete(id); }
        }
        3 => {
            let mut t = w.mobys[id].pv_i(0x14);
            let out = countdown(&mut t);
            w.mobys[id].set_pv_i(0x14, t);
            let ps = links(w, id);
            if !out {
                if t == 0xb4 / 2 { for &p in &ps { w.mobys[p].state = 4; } }
                return;
            }
            let m = &mut w.mobys[id];
            m.has_collision = false;
            m.mode = (m.mode & !mode::TARGET) | 0x41;
            let v = set_len(m.rows[0], -0.1);
            crate::moby::shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 6.0), 0x3f, v, 0xff);
            for p in ps {
                break_away(w, p, ctx);
                let at = w.mobys[p].pos;
                crate::moby::blast::burst(w, ctx, at, &crate::moby::blast::Blast { size: 2.0, bursts: [0xe, 10, 0xf], puffs: 10, streaks: 0x14, light: true, shake: true });
                w.delete(p);
            }
            let uid = w.mobys[id].uid;
            ctx.game.count_uid(uid);
            w.delete(id);
        }
        _ => { charger_common(w, id, ctx, 0xb4); }
    }
}

/// 0x0fb9 (`0x41e818`, one): as 0x0fb8 with the timer 200 (`$gp+0x2d80`); out: the blast at `$gp+0x2d90` (353, 340.45,
/// 149.5), it hidden without collision, its pieces 0..11 thrown 0.1 a tick away from Ratchet (flattened into its own
/// xy plane, `0x3290b0`), 5, its mobys to state 4 and broken ([`break_away`]); at half its mobys to state 4.
pub fn update_0fb9(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.state = 1;
            let z = m.pos[2];
            m.set_pv_f(0x10, z);
            if uid_done(w, id, ctx) { w.delete(id); }
        }
        3 => {
            let mut t = w.mobys[id].pv_i(0x14);
            let out = countdown(&mut t);
            w.mobys[id].set_pv_i(0x14, t);
            let ps = links(w, id);
            if out {
                let at = [353.0, f32::from_bits(0x43aa_399a), 149.5];
                crate::moby::blast::burst(w, ctx, at, &crate::moby::blast::Blast { size: 2.0, bursts: [0xe, 10, 0xf], puffs: 10, streaks: 0x14, light: true, shake: true });
                let m = &mut w.mobys[id];
                m.has_collision = false;
                m.mode = (m.mode & !mode::TARGET) | 0x41;
                let d = sub(m.pos, ctx.hero.pos);
                let r = m.rows;
                let local = [dot(d, r[0]), dot(d, r[1])];
                let flat: V3 = std::array::from_fn(|k| local[0] * r[0][k] + local[1] * r[1][k]);
                crate::moby::shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 6.0), 0xfff, set_len(flat, 0.1), 0xff);
                w.mobys[id].set_state(5, -1);
                for &p in &ps {
                    w.mobys[p].state = 4;
                    break_away(w, p, ctx);
                }
            }
            if w.mobys[id].pv_i(0x14) == 0xc8 / 2 { for &p in &ps { w.mobys[p].state = 4; } }
        }
        _ => { charger_common(w, id, ctx, 0xc8); }
    }
}
