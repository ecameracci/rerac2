//! Level 11's movers: the pieces the Dynamo's targets (crate::moby::classes::oozla::update_0cb5, on in their state 2)
//! and the hologram lift move, and two that move on their own (L11 addresses).
//!
//! * **0x0b6b** (`0x3f90b0`, 13 placed): the wave floor ([`update_0b6b`]).
//! * **0x0d32** (`0x436a68`, 2): the rising columns ([`update_0d32`]).
//! * **0x0c7b** (`0x417dc8`, 9): the sinking floor ([`update_0c7b`]).
//! * **0x0b6f** (`0x3f9498`, 5): the pistons ([`update_0b6f`]).
//! * **0x0c7a** (`0x4175e0`, 1): the hologram lift along its path ([`update_0c7a`]).
//! * **0x0c78 / 0x0b71 / 0x0b72 / 0x0b73 / 0x0b74** (`0x3f9690`, 16): the blocks that slide out as the lift comes
//!   ([`update_slider`]).
//! * **0x0a1f** (`0x3d7778`, 1): a mover up and down its path ([`update_0a1f`]).
//! * **0x0c7c** (`0x418150`, 2): the drawbridges the Swingshot pulls down ([`update_0c7c`]).
//!
//! Each eases by a half cosine: a phase from π on by the step its progress takes, the offset `(cos(phase) + 1)·k`
//! along one of its rows from its place (kept at set-up). Their moving sounds are their class sound 0 looped
//! (`0x374eb0(0, 4, m)` started while it does not play, `0x374bb0` stopped), [`loop_on`] / [`loop_off`]. Ratchet on
//! one rides it (crate::moby::carry_hero: it stands in for his own read of the moving collision), but for the
//! sinking floor, whose platform block says it never moves.

use crate::hero::physics::{add, add_rot, dist, dist2, scale, sub, V3};
use crate::moby::{carry_hero, countdown, Ctx, MobyWorld};
use std::f32::consts::PI;

/// The moby linked at `off` (an index), when live.
fn linked(w: &MobyWorld, id: usize, off: usize) -> Option<usize> {
    usize::try_from(w.mobys[id].pv_i(off)).ok().filter(|&i| i < w.mobys.len() && w.mobys[i].state < 0x80)
}

/// The linked moby's state is 2 (a Dynamo target on).
fn on(w: &MobyWorld, id: usize, off: usize) -> bool { linked(w, id, off).is_some_and(|p| w.mobys[p].state == 2) }

/// Its class sound 0 looped into `off` unless that one plays.
pub fn loop_on(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, off: usize) {
    let h = w.mobys[id].pv_i(off);
    if h != -1 && ctx.host.sound_playing(h) { return; }
    let h = ctx.host.play_sound(&w.mobys[id], 0, 4);
    w.mobys[id].set_pv_i(off, h);
}

/// The loop at `off` stopped, −1.
pub fn loop_off(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, off: usize) {
    let h = w.mobys[id].pv_i(off);
    if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
    w.mobys[id].set_pv_i(off, -1);
}

/// Ratchet standing on moby `id` (`0x337468`): hanging or climbing (group 3, state 0x1b) on its ledge (`0x18a3a8`),
/// else on the ground (`0x18a16e` = 0) with it under his feet (`0x18a15c`).
pub fn stands_on(ctx: &Ctx, id: usize) -> bool {
    let h = &*ctx.hero;
    if h.group == 3 || h.state == 0x1b { h.ledge.moby == Some(id) } else { h.air_ticks == 0 && h.ground_moby == Some(id) }
}

/// The half-cosine step: `progress += step`, `phase = add_rot(phase, step)`; the new progress and phase.
fn step(w: &mut MobyWorld, id: usize, progress: usize, phase: usize, k: f32) -> (f32, f32) {
    let m = &mut w.mobys[id];
    let p = m.pv_f(progress) + k;
    m.set_pv_f(progress, p);
    let a = add_rot(m.pv_f(phase), k);
    m.set_pv_f(phase, a);
    (p, a)
}

/// Its place `home + row·k`.
fn place(w: &mut MobyWorld, id: usize, home: usize, row: usize, k: f32) {
    let m = &mut w.mobys[id];
    m.pos = add(m.pv_v(home), scale(m.rows[row], k));
}

fn ride(ctx: &mut Ctx, w: &MobyWorld, id: usize, old: V3) {
    let m = &w.mobys[id];
    carry_hero(ctx, id, old, m.rot[2], m.pos, m.rot[2]);
}

/// 0x0b6b (`0x3f90b0`, 13 placed): the wave floor's tiles, each linked to a Dynamo target (`+0x08`). Its pvars: `+0x00`
/// one that dips, `+0x04` its slot in the wave (its tick there ÷ 15) — the leader has 1337 —, `+0x0c` / `+0x10` the
/// phase and progress, `+0x14` / `+0x18` the groups the leader wakes (0xff none), `+0x1c` the leader's waking done,
/// `+0x20` its place, `+0x30` its loop. **0**: the phase π, the wave's tick (`$gp+0x17c8`) 0, its place kept, **1**.
/// **1**: one that dips, the target on and the wave's tick its slot × 15 → **2**. **2**: π/60 a tick, it dips
/// `(cos + 1)·2.2` down its second row and back up (120 ticks) with its loop on; then, the target off, **1** (the loop
/// stopped); either way at its place again. **The leader** ([`wave`]) runs the wave's tick.
pub fn update_0b6b(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let old = w.mobys[id].pos;
    let target_on = on(w, id, 8);
    if w.mobys[id].state == 0 {
        let m = &mut w.mobys[id];
        m.state = 1;
        m.set_pv_f(0xc, PI);
        m.set_pv_f(0x10, 0.0);
        let pos = m.pos;
        m.set_pv_v(0x20, pos);
        m.set_pv_i(0x1c, 0);
        w.level11.wave = 0;
    }
    match w.mobys[id].state {
        1 => {
            let m = &mut w.mobys[id];
            if m.pv_i(0) != 0 && target_on && w.level11.wave == (m.pv_f(4) * 15.0) as i32 {
                m.state = 2;
                m.set_pv_f(0xc, PI);
                m.set_pv_f(0x10, 0.0);
            }
        }
        2 => {
            let (p, a) = step(w, id, 0x10, 0xc, PI / 60.0);
            loop_on(w, id, ctx, 0x30);
            if p < 2.0 * PI {
                place(w, id, 0x20, 1, -(a.cos() + 1.0) * 2.2);
            } else {
                if !target_on {
                    w.mobys[id].state = 1;
                    loop_off(w, id, ctx, 0x30);
                }
                let m = &mut w.mobys[id];
                m.set_pv_f(0x10, 0.0);
                m.set_pv_f(0xc, PI);
                m.pos = m.pv_v(0x20);
            }
        }
        _ => {}
    }
    if w.mobys[id].pv_f(4) == 1337.0 { wave(w, id, target_on); }
    ride(ctx, w, id, old);
}

/// The wave's leader (0x0b6b with 1337, `0x3f92f0..`): its target off, the wave's tick 0. On: the first time, the
/// enemies of its two groups (0x0cf5 / 0x0cf6 / 0x0d07) woken (sub-state 1) and its state 4; then the tick counts
/// 0..0x68 round. The port takes a group out of range (the placed −1) as none.
fn wave(w: &mut MobyWorld, id: usize, target_on: bool) {
    if !target_on {
        w.level11.wave = 0;
        return;
    }
    if w.mobys[id].pv_i(0x1c) != 0 {
        w.level11.wave += 1;
        if 0x68 < w.level11.wave { w.level11.wave = 0; }
        return;
    }
    w.mobys[id].set_pv_i(0x1c, 1);
    w.mobys[id].state = 4;
    for off in [0x14, 0x18] {
        let g = w.mobys[id].pv_i(off);
        if g == 0xff { continue; }
        let Some(list) = usize::try_from(g).ok().and_then(|g| w.groups.get(g)).cloned() else { continue };
        for i in list {
            let Some(m) = w.mobys.get_mut(i) else { continue };
            if m.state < 0x80 && matches!(m.o_class, 0x0cf5 | 0x0cf6 | 0x0d07) { m.sub_state = 1; }
        }
    }
}

/// 0x0d32 (`0x436a68`, 2 placed): a column linked to a Dynamo target (`+0x08`): `+0x0c` / `+0x10` the phase and
/// progress, `+0x14` its loop, `+0x20` its place. **0**: its place kept, mode | 0x800, **1**. **1**: the target on →
/// its loop, **2** (else the loop stopped). **2**: π/120 a tick it rises `(cos + 1)·5` up its third row (10 in 120
/// ticks), then **3** (the loop stopped). **3**: the target off → its loop, **4** (else stopped). **4**: with Ratchet
/// more than 6 away across, it sinks back the same way, then **1** at its place.
pub fn update_0d32(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let old = w.mobys[id].pos;
    let target_on = on(w, id, 8);
    if w.mobys[id].state == 0 {
        let m = &mut w.mobys[id];
        m.state = 1;
        m.set_pv_f(0xc, PI);
        m.set_pv_f(0x10, 0.0);
        let pos = m.pos;
        m.set_pv_v(0x20, pos);
        m.mode |= 0x800;
    }
    let k = PI / 120.0;
    match w.mobys[id].state {
        1 => {
            if target_on {
                loop_on(w, id, ctx, 0x14);
                let m = &mut w.mobys[id];
                m.state = 2;
                m.set_pv_f(0xc, PI);
                m.set_pv_f(0x10, 0.0);
            } else {
                loop_off(w, id, ctx, 0x14);
            }
        }
        2 => {
            let (p, a) = step(w, id, 0x10, 0xc, k);
            if p < PI {
                place(w, id, 0x20, 2, (a.cos() + 1.0) * 5.0);
            } else {
                loop_off(w, id, ctx, 0x14);
                w.mobys[id].state = 3;
            }
        }
        3 => {
            if !target_on {
                loop_on(w, id, ctx, 0x14);
                w.mobys[id].state = 4;
            } else {
                loop_off(w, id, ctx, 0x14);
            }
        }
        4 => {
            if 6.0 < dist2(w.mobys[id].pos, ctx.hero.pos) {
                let (p, a) = step(w, id, 0x10, 0xc, k);
                if 2.0 * PI <= p {
                    loop_off(w, id, ctx, 0x14);
                    let m = &mut w.mobys[id];
                    m.state = 1;
                    m.set_pv_f(0xc, PI);
                    m.set_pv_f(0x10, 0.0);
                    m.pos = m.pv_v(0x20);
                } else {
                    place(w, id, 0x20, 2, (a.cos() + 1.0) * 5.0);
                }
            }
        }
        _ => {}
    }
    ride(ctx, w, id, old);
}

/// 0x0c7b (`0x417dc8`, 9 placed): the sinking floor, linked to a Dynamo target (`+0x68`): `+0x60` one that sinks,
/// `+0x6c` / `+0x70` the phase and progress, `+0x74` its loop, `+0x80` its place, its platform block `+0x20`.
/// **0**: `+0x5c` | 4, its place kept, `$gp+0x1d78` 0, **1**. **1**: one that sinks, the target on → its loop, **2**.
/// **2**: π/60 a tick it sinks `(cos + 1)·4` down its second row (8 in 60 ticks), then **3** (the loop stopped).
/// **3**: the target off → its loop, **4**. **4**: back up the same way, then **1** at its place. Each tick its
/// platform block gets no move and no turn (`0x334dd0(+0x20, 0x1a8f20, angles, angles)`, crate::moby::platform::write;
/// `0x1a8f20` is the zero vector), so Ratchet is not carried.
pub fn update_0c7b(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let target_on = on(w, id, 0x68);
    let k = PI / 60.0;
    let down = |w: &mut MobyWorld, a: f32| place(w, id, 0x80, 1, -(a.cos() + 1.0) * 4.0);
    if w.mobys[id].state == 0 {
        let m = &mut w.mobys[id];
        m.set_pv_i(0x5c, m.pv_i(0x5c) | 4);
        m.state = 1;
        m.set_pv_f(0x6c, PI);
        m.set_pv_f(0x70, 0.0);
        w.level11.g1d78 = 0;
        let pos = w.mobys[id].pos;
        w.mobys[id].set_pv_v(0x80, pos);
    }
    match w.mobys[id].state {
        1 => {
            if w.mobys[id].pv_i(0x60) != 0 && target_on {
                loop_on(w, id, ctx, 0x74);
                let m = &mut w.mobys[id];
                m.state = 2;
                m.set_pv_f(0x6c, PI);
                m.set_pv_f(0x70, 0.0);
            }
        }
        2 => {
            let (p, a) = step(w, id, 0x70, 0x6c, k);
            if p < PI {
                down(w, a);
            } else {
                loop_off(w, id, ctx, 0x74);
                w.mobys[id].state = 3;
            }
        }
        3 => {
            if !target_on {
                loop_on(w, id, ctx, 0x74);
                w.mobys[id].state = 4;
            }
        }
        4 => {
            let (p, a) = step(w, id, 0x70, 0x6c, k);
            if 2.0 * PI <= p {
                loop_off(w, id, ctx, 0x74);
                let m = &mut w.mobys[id];
                m.state = 1;
                m.set_pv_f(0x6c, PI);
                m.set_pv_f(0x70, 0.0);
                m.pos = m.pv_v(0x80);
            } else {
                down(w, a);
            }
        }
        _ => {}
    }
    let m = &mut w.mobys[id];
    let r = m.rot;
    crate::moby::platform::write(m, 0x20, [0.0; 3], r, r);
}

/// 0x0b6f (`0x3f9498`, 5 placed): a piston, on its own: `+0x0c` / `+0x10` the phase and progress, `+0x20` its place.
/// **0**: its class sound 0 (`0x374eb0(0, 0, m)`), its place kept, **2**. **2**: 2.83° a tick (64 ticks) it goes out
/// `cos + 1` along its first row (twice its length), then **4**. **4**: π/120 a tick back in (120 ticks); then its sound
/// 0 again, **2** at its place. No pvars: deleted (`0x323a00`).
pub fn update_0b6f(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pvars.is_empty() { return w.delete(id); }
    let old = w.mobys[id].pos;
    let restart = |w: &mut MobyWorld, ctx: &mut Ctx, keep: bool| {
        ctx.host.play_sound(&w.mobys[id], 0, 0);
        let m = &mut w.mobys[id];
        m.state = 2;
        m.set_pv_f(0xc, PI);
        m.set_pv_f(0x10, 0.0);
        if keep { let p = m.pos; m.set_pv_v(0x20, p); } else { m.pos = m.pv_v(0x20); }
    };
    match w.mobys[id].state {
        0 | 2 => {
            if w.mobys[id].state == 0 { restart(w, ctx, true); }
            let (p, a) = step(w, id, 0x10, 0xc, f32::from_bits(0x3d4a_8d23));
            if p < PI { place(w, id, 0x20, 0, a.cos() + 1.0); } else { w.mobys[id].state = 4; }
        }
        4 => {
            let (p, a) = step(w, id, 0x10, 0xc, PI / 120.0);
            if p < 2.0 * PI { place(w, id, 0x20, 0, a.cos() + 1.0); } else { restart(w, ctx, false); }
        }
        _ => {}
    }
    ride(ctx, w, id, old);
}

/// 0x0c7a (`0x4175e0`, 1 placed): the hologram lift. Its pvars: `+0x60` its path, `+0x64` the point it heads for,
/// `+0x68` its camera tweak, `+0x6c` the cuboid that sends it home, `+0x70` its loop, `+0x74` its glow's phase, its
/// platform block `+0x20`. Solid only while poked (its timer byte `+0xbc`, the Dynamo's targets); sub-state 0 → 1 with
/// mode | 0x2000 once.
/// * **0** (home): Ratchet off it, its loop stopped; on it ([`stands_on`]) its loop, a camera tweak of priority 5
///   (`0x2d7468`: distance 10, the stick off, `+0x3c` and the firm springs on, facing back along its first row once),
///   **1** — its mission (`+0xb0`) done: the tweak let go, **6** at its path's last point.
/// * **1**: a thirtieth of the way to its point a tick, the next within 0.05 across; past the last, Ratchet off it:
///   **2**, the tweak let go, its group's pokes cleared (`0x31c970(group, 0)`); its loop stopped. Ratchet in its
///   cuboid: home (its path's first point, **0**, the pokes cleared, the loop stopped, the tweak let go).
/// * **2**: Ratchet in its cuboid: home the same way. **3** / **5**: back point by point at a fifteenth a tick (5: in
///   3D, its loop stopped past the first; nothing here sets them). **6**: on at a fifteenth (in 3D); past the last,
///   Ratchet off it: **2** (the pokes cleared, the loop stopped).
///
/// Each tick its block (`0x334dd0`) takes its move and turn, and its glow (`0x33ef00`, [`glow`]). The port keeps the
/// tweak's slot (+1) in `+0x68` and lets it go once (the game lets the pointer go again at the cuboid in state 2).
pub fn update_0c7a(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (old, old_rot) = (w.mobys[id].pos, w.mobys[id].rot);
    let solid = w.class(w.mobys[id].o_class).is_some_and(|c| c.has_collision);
    {
        let m = &mut w.mobys[id];
        if m.sub_state == 0 {
            m.sub_state = 1;
            m.mode |= 0x2000;
        }
        m.has_collision = m.timer != 0 && solid;
    }
    let path = ctx.level.paths.get(usize::try_from(w.mobys[id].pv_i(0x60)).unwrap_or(usize::MAX)).cloned().unwrap_or_default();
    let n = path.len() as i32;
    let point = |k: i32| -> V3 { path.get(k.max(0) as usize).map_or([0.0; 3], |p| [p[0], p[1], p[2]]) };
    let release = |w: &mut MobyWorld, ctx: &mut Ctx| {
        let s = w.mobys[id].pv_i(0x68);
        if 0 < s { ctx.game.cam_tweaks.release(s as usize - 1); }
        w.mobys[id].set_pv_i(0x68, 0);
    };
    let home = |w: &mut MobyWorld, ctx: &mut Ctx| {
        let g = w.mobys[id].group as i32;
        w.poke_group(g, 0);
        let m = &mut w.mobys[id];
        m.pos = point(0);
        m.state = 0;
        m.set_pv_i(0x64, 0);
        loop_off(w, id, ctx, 0x70);
        release(w, ctx);
    };
    let toward = |w: &mut MobyWorld, k: f32| {
        let m = &mut w.mobys[id];
        let to = point(m.pv_i(0x64));
        m.pos = add(m.pos, scale(sub(to, m.pos), k));
        to
    };
    let riding = stands_on(ctx, id);
    if !riding {
        if w.mobys[id].state == 0 { loop_off(w, id, ctx, 0x70); }
    } else if w.mobys[id].state == 0 {
        loop_on(w, id, ctx, 0x70);
        if let Some(s) = ctx.game.cam_tweaks.request(5) {
            let t = &mut ctx.game.cam_tweaks.slots[s];
            t.dist = 10.0;
            t.no_stick = true;
            t.g3c = true;
            t.firm = true;
            let r = w.mobys[id].rows[0];
            t.dir = scale(r, -1.0);
            t.face_once = true;
            w.mobys[id].set_pv_i(0x68, s as i32 + 1);
        }
        w.mobys[id].state = 1;
        if super::mission_done(ctx, w.mobys[id].mission as i32) {
            release(w, ctx);
            let m = &mut w.mobys[id];
            m.state = 6;
            m.set_pv_i(0x64, n - 1);
        }
    }
    match w.mobys[id].state {
        1 => {
            let k = w.mobys[id].pv_i(0x64);
            if k < n {
                let to = toward(w, 1.0 / 30.0);
                if dist2(w.mobys[id].pos, to) < 0.05 { w.mobys[id].set_pv_i(0x64, k + 1); }
            } else {
                if !riding {
                    w.mobys[id].state = 2;
                    release(w, ctx);
                    let g = w.mobys[id].group as i32;
                    w.poke_group(g, 0);
                }
                loop_off(w, id, ctx, 0x70);
            }
            if ctx.level.in_cuboid(w.mobys[id].pv_i(0x6c), ctx.hero.pos) { home(w, ctx); }
        }
        2 => {
            if ctx.level.in_cuboid(w.mobys[id].pv_i(0x6c), ctx.hero.pos) { home(w, ctx); }
        }
        3 | 5 => {
            let k = w.mobys[id].pv_i(0x64);
            if k < 0 {
                let was = w.mobys[id].state;
                let m = &mut w.mobys[id];
                m.state = 0;
                m.set_pv_i(0x64, 0);
                if was == 5 { loop_off(w, id, ctx, 0x70); }
            } else {
                let to = toward(w, 1.0 / 15.0);
                let p = w.mobys[id].pos;
                let near = if w.mobys[id].state == 3 { dist2(p, to) } else { dist(p, to) };
                if near < 0.05 { w.mobys[id].set_pv_i(0x64, k - 1); }
            }
        }
        6 => {
            let k = w.mobys[id].pv_i(0x64);
            if k < n {
                let to = toward(w, 1.0 / 15.0);
                if dist(w.mobys[id].pos, to) < 0.05 { w.mobys[id].set_pv_i(0x64, k + 1); }
            } else if !riding {
                let m = &mut w.mobys[id];
                m.state = 2;
                m.set_pv_i(0x64, 0);
                let g = m.group as i32;
                w.poke_group(g, 0);
                loop_off(w, id, ctx, 0x70);
            }
        }
        _ => {}
    }
    let m = &mut w.mobys[id];
    let (mv, r) = (sub(m.pos, old), m.rot);
    crate::moby::platform::write(m, 0x20, mv, old_rot, r);
    let poked = m.timer != 0;
    glow(w, id, ctx, 0x74, poked);
    let m = &w.mobys[id];
    carry_hero(ctx, id, old, old_rot[2], m.pos, m.rot[2]);
}

/// `0x33ef00(m, &phase, on)`: a hologram's look. Seen (alpha `+0x23` ≠ 0) or on: the phase on by 8.33° a tick, the
/// ambient (`0x3241d8`) between 0x8000c020 and 0x80008000 by `(sin + 0.5)·0.5` (`0x2f0000`), one tick in twenty
/// (`randf(0, 1) < 0.05`) a third of the way to 0x8080ffff; the alpha up by 3 to 0x40 while on, else down by 3 to 0.
pub fn glow(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, phase: usize, on: bool) {
    let m = &mut w.mobys[id];
    if m.alpha == 0 && !on { return; }
    let a = add_rot(m.pv_f(phase), f32::from_bits(0x3e14_ef4e));
    m.set_pv_f(phase, a);
    let mut c = crate::hud::lerp_rgba((a.sin() + 0.5) * 0.5, 0x8000_c020, 0x8000_8000);
    if ctx.game.rng.randf(0.0, 1.0) < 0.05 { c = crate::hud::lerp_rgba(f32::from_bits(0x3e99_999a), c, 0x8080_ffff); }
    m.set_ambient_word(c);
    m.alpha = if on {
        if m.alpha < 0x3d { m.alpha + 3 } else { 0x40 }
    } else if m.alpha < 4 {
        0
    } else {
        m.alpha - 3
    };
}

/// 0x0c78 / 0x0b71 / 0x0b72 / 0x0b73 / 0x0b74 (`0x3f9690`, 7 + 2 + 4 + 2 + 1 placed): the blocks that slide out of the
/// way of the hologram lift (`+0x6c`, 0x0c7a): `+0x60` the cuboid, `+0x64` the slide's ticks, `+0x68` its length along
/// its first row, `+0x70` its loop. Nothing once its mission (`+0xb0`) is done. **0**: 0x0b71's `+0x5c` | 1, **1**.
/// **1**: the lift's place in the cuboid → its loop, **2** over 45 ticks: out by length / 45 a tick, then **3** (the
/// loop stopped). **3**: the lift out of it → its loop, **4**: back in over 45 ticks, then **1** (the loop stopped).
pub fn update_slider(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if super::mission_done(ctx, w.mobys[id].mission as i32) { return; }
    let old = w.mobys[id].pos;
    let lift = linked(w, id, 0x6c).map(|p| w.mobys[p].pos);
    let inside = |w: &MobyWorld| lift.is_some_and(|p| ctx.level.in_cuboid(w.mobys[id].pv_i(0x60), p));
    if w.mobys[id].state == 0 {
        let m = &mut w.mobys[id];
        if m.o_class == 0x0b71 { m.set_pv_i(0x5c, m.pv_i(0x5c) | 1); }
        m.state = 1;
    }
    let slide = |w: &mut MobyWorld, ctx: &mut Ctx, way: f32, next: u8| {
        let m = &mut w.mobys[id];
        let v = scale(m.rows[0], m.pv_f(0x68) / 45.0 * way);
        m.pos = add(m.pos, v);
        let mut t = m.pv_i(0x64);
        let done = countdown(&mut t);
        m.set_pv_i(0x64, t);
        if done {
            loop_off(w, id, ctx, 0x70);
            w.mobys[id].state = next;
        }
    };
    if w.mobys[id].state == 1 && inside(w) {
        loop_on(w, id, ctx, 0x70);
        w.mobys[id].state = 2;
        w.mobys[id].set_pv_i(0x64, 0x2d);
    }
    if w.mobys[id].state == 2 { slide(w, ctx, 1.0, 3); }
    if w.mobys[id].state == 3 && !inside(w) {
        loop_on(w, id, ctx, 0x70);
        w.mobys[id].state = 4;
        w.mobys[id].set_pv_i(0x64, 0x2d);
    }
    if w.mobys[id].state == 4 { slide(w, ctx, -1.0, 1); }
    ride(ctx, w, id, old);
}

/// 0x0a1f (`0x3d7778`, 1 placed): a mover up and down its path (`+0x00`; −1: mode | 2, no update): `+0x04` the trip's
/// seconds, `+0x08` the wait's, `+0x0c` / `+0x18` the speed and the place along it (0..1), `+0x10` the ease's
/// acceleration `1/(900·T²)` a tick², `+0x14` the end it heads for, `+0x1c` the wait's ticks. **0**: update distance
/// 0xff, at the path's first point, **1**. **1**: the wait out → **2** heading for the other end. **2**: the place eased
/// to the end (`0x331348(end, a, a, 1)`, crate::hero::physics::ease), the mobys's place between the path's points there
/// (`0x301258`); at the end, **1** with the wait again.
pub fn update_0a1f(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let old = w.mobys[id].pos;
    let path = ctx.level.paths.get(usize::try_from(w.mobys[id].pv_i(0)).unwrap_or(usize::MAX)).cloned();
    let m = &mut w.mobys[id];
    match m.state {
        0 => {
            if m.pv_i(0) == -1 {
                m.mode |= 2;
                return;
            }
            m.update_dist = 0xff;
            m.set_state(1, -1);
            if let Some(p) = path.as_ref().and_then(|p| p.first()) { m.pos = [p[0], p[1], p[2]]; }
            m.set_pv_i(0x1c, (m.pv_f(8) * 60.0) as i32);
            let t = m.pv_f(4);
            m.set_pv_f(0x10, 0.001_111_111_1 / (t * t));
        }
        1 => {
            let mut t = m.pv_i(0x1c);
            let done = countdown(&mut t);
            m.set_pv_i(0x1c, t);
            if done {
                m.set_state(2, -1);
                m.set_pv_i(0x14, (m.pv_i(0x14) + 1) & 1);
            }
        }
        2 => {
            let Some(p) = path.filter(|p| 2 <= p.len()) else { return };
            let end = m.pv_i(0x14);
            let a = m.pv_f(0x10);
            let (mut x, mut v) = (m.pv_f(0x18), m.pv_f(0xc));
            crate::hero::physics::ease(end as f32, a, a, 1.0, &mut x, &mut v);
            m.set_pv_f(0x18, x);
            m.set_pv_f(0xc, v);
            let n = p.len() as i32;
            let s = x * (n - 1) as f32;
            let (mut k, mut f) = (s as i32, s - (s as i32) as f32);
            if n - 2 < k {
                f = 1.0;
                k = n - 2;
            }
            let k = k.max(0) as usize;
            let (a, b) = (p[k], p[k + 1]);
            m.pos = std::array::from_fn(|j| a[j] + (b[j] - a[j]) * f);
            let at_end = if end == 0 { x <= 0.0 } else { end as f32 <= x };
            if at_end {
                m.set_state(1, -1);
                m.set_pv_i(0x1c, (m.pv_f(8) * 60.0) as i32);
            }
        }
        _ => {}
    }
    ride(ctx, w, id, old);
}

/// 0x0c7c (`0x418150`, 2 placed): the drawbridge the Swingshot pulls down: its pull target (`+0x00`, a 0x099d) held at
/// its end (5.7 up its third row, 0.35 along its second) every tick; `+0x04` the fall's speed. **1**: the target hooked
/// (`0x33d5c8`) → its class sound 0, **2**; its mission (`+0xb0`) done: its pitch a quarter turn down at once, **6**.
/// Then it falls to −90° and settles in two bounces, its pitch eased each time (`0x331348`, crate::hero::physics::ease;
/// the speed 0 at each end): **2** down from rest (1/1876 a tick², no brake to speak of), its class sound 1, **3** up to
/// −75°, **4** down to −90°, sound 1, **5** up to −85°, **6** down to −90°, sound 1, **7** (done).
///
/// The target's hook is the hero's: at the pull's transition 0x8a → 0x8b (L11 `0x2e0898`) he hooks his pull target
/// (`0x33d5a8(target, 1)`), which crate::hero::swing does not; this update stands in for it (Ratchet in 0x8b with its
/// target his pull target hooks it).
pub fn update_0c7c(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use crate::hero::physics::ease;
    let child = linked(w, id, 0);
    if let Some(c) = child {
        if ctx.hero.state == 0x8b && ctx.hero.swing.pull.as_ref().is_some_and(|p| p.id == c) { w.mobys[c].once |= 4; }
    }
    let half = std::f32::consts::FRAC_PI_2;
    // (target, accel, brake, most, the end's test, the next state, its sound 1).
    let leg = |w: &mut MobyWorld, ctx: &mut Ctx, t: u32, a: u32, b: u32, max: u32, down: bool, next: u8| {
        let m = &mut w.mobys[id];
        let (mut x, mut v) = (m.rot[0], m.pv_f(4));
        ease(f32::from_bits(t), f32::from_bits(a), f32::from_bits(b), f32::from_bits(max), &mut x, &mut v);
        m.rot[0] = x;
        m.set_pv_f(4, v);
        let done = if down { x <= -half } else { f32::from_bits(t) <= x };
        if done {
            m.set_pv_f(4, 0.0);
            m.set_state(next, -1);
            if down { ctx.host.play_sound(&w.mobys[id], 1, 0); }
        }
    };
    match w.mobys[id].state {
        0 => w.mobys[id].set_state(1, -1),
        1 => {
            if child.is_some_and(|c| w.mobys[c].once & 4 != 0) {
                ctx.host.play_sound(&w.mobys[id], 0, 0);
                w.mobys[id].set_state(2, -1);
            }
            if super::mission_done(ctx, w.mobys[id].mission as i32) {
                let m = &mut w.mobys[id];
                m.rot[0] = crate::hero::physics::sub_rot(m.rot[0], f32::from_bits(0x3fc9_0fdc));
                m.set_state(6, -1);
            }
        }
        2 => leg(w, ctx, 0xbfc9_0fdc, 0x3a0b_ccd4, 0x432e_886f, 0x4391_71b3, true, 3),
        3 => leg(w, ctx, 0xbfa7_8d37, 0x3c8e_fa36, 0x39e4_c38a, 0x3f5f_66f5, false, 4),
        4 => leg(w, ctx, 0xbfc9_0fdc, 0x39e4_c38a, 0x3e32_b8c4, 0x403a_2b22, true, 5),
        5 => leg(w, ctx, 0xbfbd_e450, 0x3c8e_fa36, 0x39e4_c38a, 0x3f5f_66f5, false, 6),
        6 => leg(w, ctx, 0xbfc9_0fdc, 0x39e4_c38a, 0x3e32_b8c4, 0x403a_2b22, true, 7),
        _ => {}
    }
    if let Some(c) = child {
        let m = &w.mobys[id];
        let p = add(add(m.pos, scale(m.rows[2], f32::from_bits(0x40b6_6666))), scale(m.rows[1], f32::from_bits(0x3eb3_3333)));
        w.mobys[c].pos = p;
    }
}
