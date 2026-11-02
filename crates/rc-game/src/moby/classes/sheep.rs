//! The sheep, class **0x0e7d** (`0x404338`; made by the Sheepinator's transform `0x404c18`,
//! crate::moby::classes::sheepinator::transform): what an enemy becomes. It idles, grazes and wanders, and backs off
//! from Ratchet; a hit pops it.
//!
//! **Made** ([`spawn`], `0x404c18`'s first part; pvars: `+0x54` (s16) 120..300, `+0x56` (s16) 0, `+0x58` its turn's
//! speed, `+0x5c` 0, `+0x60` −1, `+0x68` 1 (4 on an upgraded row), `+0x70` its state's ticks (600), `+0x74` 30, `+0x78`
//! 0 — those but `+0x58` and `+0x70` read by nothing here): updated and drawn to 0x7f, drawn, playing at 0.85..1.15
//! (`+0x48`), at the target's place facing a random way (in the gravity frames the target's rows), state 4; listed in
//! the flock ([`list`], `0x4051a8`: sixteen at most, a full flock pops a random one first); its collision on (`+0x98`).
//!
//! **Each tick** ([`update`], `0x404338`; (in the level's moving frame `0x1adb0c` it rides it: not ported, none on
//! these levels)): a hit (`0x31dce8(m, 0x4010000, 0)`) pops it ([`pop`]); its blob shadow (0.5, `0x31d930`). By state:
//! * **0**: to 1.
//! * **1** (idle): its sequence run out, a new one by `randf(0, 100)`: under 20 sequence 0, under 50 4, else 1 (blend
//!   10); Ratchet within 2 (across) — **2**, sequence 2, playing 0.75; its ticks out (outside the frames) — at random
//!   **6** for 180..360 ticks (sequence 3), else **5** for 400..600 (sequence 2, playing 0.75).
//! * **2** (backing off): Ratchet 2 away — playing 1, **1**; else it turns toward its facing ±30° (`0x31c078`: 0.000145,
//!   0.000436, at most 0.75° a tick, `+0x58`) and steps 1/240 along its facing on walkable ground (`0x31c278(0.5, 0.7,
//!   0.25, 30°, …, flags 2)`); blocked, it turns −90°..−10° off its facing (0.0000582); a moby within 0.5
//!   (`0x2df8d8`, flags 1) puts it back where it was.
//! * **5** (wandering): its sequence run out, sequence 2 again, playing 0.75; Ratchet within 2 — **2**, playing 0.1;
//!   its ticks out — **1** for 400..600; it turns and steps as in 2 (from 0.1 up), blocked turning 20°..135°
//!   (0.0000485).
//! * **6** (grazing): its sequence run out, sequence 3 again; its ticks out — **1** for 120..600, sequence 0 or 1.
//! * **4** (just made): the line 1 below to 4 above it along its up (flags 0) puts it where it meets anything but
//!   Ratchet; met on a face of a type (`0x2e23e0`) above 0 and not 10 it pops; else **0**.
//!
//! Then, outside the gravity frames, it stands on the ground under it (`0x31ce20(0.5)` from 0.5 up).
//!
//! **The pop** ([`pop`], `0x404df0`): `33 − n` puffs (crate::particles::smoke, `0x335a90`: 150000, within 0.37 and
//! 0.35..0.6 up, drifting 0.1..1 a second a random way, `0x4f20507f` → `0x102f`, 30..57 ticks) and `50 − n` tufts of
//! wool (crate::particles::wool, `0x340ce8`: within ±0.15, flung 8 a second toward the camera's side within 90° and
//! 9 up, 300 ticks), n the sheep about (`$gp+0xf6c`), which it takes one off; out of the flock (`0x405158`) and
//! deleted.

use crate::hero::physics::{add, atan2, dist2, scale, set_len, V3};
use crate::moby::{ai, countdown, Ctx, MobyWorld};

/// The class.
pub const CLASS: i16 = 0xe7d;
/// `0x31dce8`'s mask: the hits that pop it.
const HIT_MASK: u32 = 0x401_0000;
const DEG: f32 = 0.017_453_294;

/// `0x404c18`'s making of the sheep at `target` (module docs); `up` the item's row is past its base.
pub fn spawn(w: &mut MobyWorld, ctx: &mut Ctx, target: usize, up: bool) -> Option<usize> {
    w.class(CLASS)?;
    let id = w.create(CLASS);
    let speed = ctx.game.rng.randf(f32::from_bits(0x3f59_999a), f32::from_bits(0x3f93_3333));
    let (pos, rows) = (w.mobys[target].pos, w.mobys[target].rows);
    let frames = ctx.hero.gravity.on;
    let yaw = if frames { 0.0 } else { ctx.game.rng.rand_angle() };
    let pick = ctx.game.rng.rand_range(0x78, 300) as i16;
    {
        let m = &mut w.mobys[id];
        m.pvars.resize(crate::moby::PVAR_BYTES, 0);
        m.update = Some(update);
        m.draw_dist = 0x7f;
        m.update_dist = 0x7f;
        m.drawn = 1;
        m.pos = pos;
        m.rot = if frames { crate::moby::attach::angles_of(&rows) } else { [0.0, 0.0, yaw] };
        m.state = 0;
        m.set_pv_s(0x56, 0);
        m.set_pv_s(0x54, pick);
        m.set_pv_i(0x70, 600);
        m.set_pv_i(0x74, 0x1e);
        m.set_pv_i(0x5c, 0);
        m.set_pv_i(0x78, 0);
        m.state = 4;
    }
    ctx.host.set_anim_speed(&w.mobys[id], speed);
    list(w, ctx, id);
    let c = w.class(CLASS).is_some_and(|c| c.has_collision);
    let m = &mut w.mobys[id];
    m.has_collision = c;
    m.set_pv_f(0x68, if up { 4.0 } else { 1.0 });
    m.set_pv_i(0x60, -1);
    w.changed = true;
    Some(id)
}

/// `0x4051a8`: the sheep into the flock's first free place; a full flock (`0x405120`) pops a random one
/// (`randi(16)`, [`pop`]) and takes its place.
fn list(w: &mut MobyWorld, ctx: &mut Ctx, id: usize) {
    if let Some(slot) = w.sheep.flock.iter_mut().find(|s| s.is_none()) {
        *slot = Some(id);
        return;
    }
    let k = ctx.game.rng.randi(0x10) as usize;
    if let Some(old) = w.sheep.flock[k] { pop(w, ctx, old); }
    w.sheep.flock[k] = Some(id);
}

/// `0x405158`: the sheep out of the flock (not found: the original's debug line only).
fn unlist(w: &mut MobyWorld, id: usize) {
    if let Some(s) = w.sheep.flock.iter_mut().find(|s| **s == Some(id)) { *s = None; }
}

/// `0x404df0` (module docs).
pub fn pop(w: &mut MobyWorld, ctx: &mut Ctx, id: usize) {
    let pos = w.mobys[id].pos;
    let n = w.sheep.count;
    let cam = ctx.host.camera_pos();
    if let Some(sys) = ctx.parts.as_deref_mut() {
        let g = &mut ctx.game.rng;
        for _ in 0..(0x21 - n).max(0) {
            let (r, a, b) = (g.randf(0.0, f32::from_bits(0x3ebd_70a4)), g.rand_angle(), g.rand_angle());
            let mut p = polar(r, a, b);
            p[2] += g.randf(f32::from_bits(0x3eb3_3333), f32::from_bits(0x3f19_999a));
            let p = add(p, pos);
            let (s, a, b) = (g.randf(f32::from_bits(0x3ada_740f), f32::from_bits(0x3c88_8889)), g.rand_angle(), g.rand_angle());
            let v = polar(s, a, b);
            let life = g.rand_range(0x1e, 0x39);
            crate::particles::smoke::smoke(sys, g, 150_000.0, p, v, 0x4f20_507f, 0x102f, life);
        }
        for _ in 0..(0x32 - n).max(0) {
            let yaw = atan2(cam[0] - pos[0], cam[1] - pos[1]);
            // `0x2effb8(randf_signed(1, 2))`: drawn and dropped.
            let _ = g.randf_signed(1.0, 2.0);
            let p = [pos[0] + g.randf_signed(0.0, 0.15), pos[1] + g.randf_signed(0.0, 0.15), pos[2] + g.randf_signed(0.0, 0.15)];
            let a = crate::hero::physics::add_rot(yaw, g.randf_signed(0.0, f32::from_bits(0xbfc9_0fdc)));
            let v = [a.cos() * 0.016_666_668 * 8.0, a.sin() * 0.016_666_668 * 8.0, 9.0 * 0.016_666_668];
            crate::particles::wool::wool(sys, g, p, v, 300);
        }
    }
    w.sheep.count = n - 1;
    unlist(w, id);
    w.delete(id);
}

/// `0x325100(r, yaw, pitch)`.
fn polar(r: f32, yaw: f32, pitch: f32) -> V3 { [yaw.cos() * r * pitch.cos(), yaw.sin() * r * pitch.cos(), pitch.sin() * r] }

/// The turn and step of states 2 and 5 (`0x31c078` / `0x31c278`); `lift` the step's height (2: 0.5, 5: 0.1), `away`
/// the blocked turn's range in degrees and its spring. Whether it moved (on to the sphere test).
fn walk(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, lift: f32, away: (f32, f32), k: f32) -> bool {
    let r = ctx.game.rng.randf(-30.0, 30.0);
    let m = &mut w.mobys[id];
    let yaw = crate::hero::physics::add_rot(m.rot[2], r * DEG);
    ai::turn_to(m, yaw, f32::from_bits(0x3918_825c), f32::from_bits(0x39e4_c38a), f32::from_bits(0x3c56_7752), 0x58);
    let mut pos = m.pos;
    let mut target = add(pos, set_len(m.rows[0], f32::from_bits(0x3b88_8889)));
    let ok = ai::step_walk(ctx, id, lift, f32::from_bits(0x3f33_3333), 0.25, f32::from_bits(0x3f06_0a93), &mut pos, &mut target, 2);
    w.mobys[id].pos = pos;
    w.changed = true;
    if !ok {
        let r = ctx.game.rng.randf(away.0, away.1);
        let m = &mut w.mobys[id];
        let yaw = crate::hero::physics::add_rot(m.rot[2], r * DEG);
        ai::turn_to(m, yaw, k, f32::from_bits(0x39e4_c38a), f32::from_bits(0x3c56_7752), 0x58);
    }
    ok
}

/// `0x2df8d8(0.5, place, 1, m)`: a moby within 0.5 puts it back at `was`.
fn crowd(w: &mut MobyWorld, id: usize, ctx: &Ctx, was: V3) {
    let pos = w.mobys[id].pos;
    let hit = ctx.coll.and_then(|c| crate::collision::coll_sphere_m(c, ctx.scene, pos, 0.5, crate::collision::QueryFlags(1), Some(id)));
    if hit.is_some_and(|h| h.moby.is_some()) { w.mobys[id].pos = was; }
}

/// `0x404338` (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].pvars.resize(crate::moby::PVAR_BYTES, 0);
    // `0x31dce8(m, 0x4010000, 0)`: a record whose flags miss is dropped.
    if let Some(h) = w.hit_message(id) {
        if h.tmpl.flags & HIT_MASK == 0 {
            w.clear_hit(id);
        } else {
            pop(w, ctx, id);
        }
    }
    crate::shadows::request(w, ctx, 0.5, id);
    let frames = ctx.hero.gravity.on;
    let hero = ctx.hero.pos;
    let seq_done = |w: &MobyWorld, ctx: &Ctx| ctx.host.anim(&w.mobys[id]).is_some_and(|a| a.flags & 2 != 0);
    match w.mobys[id].state {
        0 => w.mobys[id].set_state(1, -1),
        1 => {
            if seq_done(w, ctx) {
                let r = ctx.game.rng.randf(0.0, 100.0);
                let seq = if r < 20.0 { 0 } else if r < 50.0 { 4 } else { 1 };
                ctx.host.set_anim(&w.mobys[id], seq, 0, 10);
            }
            if dist2(w.mobys[id].pos, hero) < 2.0 && !frames {
                w.mobys[id].set_state(2, -1);
                ctx.host.set_anim(&w.mobys[id], 2, 0, 10);
                ctx.host.set_anim_speed(&w.mobys[id], 0.75);
            }
            let mut t = w.mobys[id].pv_i(0x70);
            let out = countdown(&mut t);
            w.mobys[id].set_pv_i(0x70, t);
            if out && !frames {
                if ctx.game.rng.randi(2) == 0 {
                    w.mobys[id].set_state(6, -1);
                    let n = ctx.game.rng.rand_range(0xb4, 0x168);
                    w.mobys[id].set_pv_i(0x70, n);
                    ctx.host.set_anim(&w.mobys[id], 3, 0, 10);
                } else {
                    w.mobys[id].set_state(5, -1);
                    let n = ctx.game.rng.rand_range(400, 600);
                    w.mobys[id].set_pv_i(0x70, n);
                    ctx.host.set_anim(&w.mobys[id], 2, 0, 10);
                    ctx.host.set_anim_speed(&w.mobys[id], 0.75);
                }
            }
        }
        2 => {
            if 2.0 <= dist2(w.mobys[id].pos, hero) {
                ctx.host.set_anim_speed(&w.mobys[id], 1.0);
                w.mobys[id].set_state(1, -1);
            } else {
                let was = w.mobys[id].pos;
                if walk(w, id, ctx, 0.5, (-90.0, -10.0), f32::from_bits(0x3874_03c6)) { crowd(w, id, ctx, was); }
            }
        }
        4 => {
            let up = ctx.hero.gravity.up_at(w.mobys[id].pos);
            let pos = w.mobys[id].pos;
            let (top, bottom) = (add(pos, scale(up, 4.0)), add(pos, scale(up, -1.0)));
            let hit = w.coll_line(ctx, bottom, top, 0, Some(id));
            if let Some(h) = hit.filter(|h| h.moby != ctx.hero_moby || h.moby.is_none()) { w.mobys[id].pos = h.point; }
            // `0x2e23e0`: the face type of the line's hit (−1 none).
            let kind = hit.map_or(-1, |h| if 0 <= h.kind && h.kind & 0x1f != 0x1f { h.kind & 0x1f } else { -1 });
            if 0 < kind && kind != 10 {
                pop(w, ctx, id);
                return;
            }
            w.mobys[id].set_state(0, -1);
        }
        5 => {
            if seq_done(w, ctx) {
                ctx.host.set_anim(&w.mobys[id], 2, 0, 10);
                ctx.host.set_anim_speed(&w.mobys[id], 0.75);
            }
            if dist2(w.mobys[id].pos, hero) < 2.0 {
                w.mobys[id].set_state(2, -1);
                ctx.host.set_anim_speed(&w.mobys[id], 0.1);
                ctx.host.set_anim(&w.mobys[id], 2, 0, 10);
            }
            let mut t = w.mobys[id].pv_i(0x70);
            let out = countdown(&mut t);
            w.mobys[id].set_pv_i(0x70, t);
            if out {
                w.mobys[id].set_state(1, -1);
                let n = ctx.game.rng.rand_range(400, 600);
                w.mobys[id].set_pv_i(0x70, n);
            }
            let was = w.mobys[id].pos;
            if walk(w, id, ctx, 0.1, (20.0, 135.0), f32::from_bits(0x384b_587b)) { crowd(w, id, ctx, was); }
        }
        6 => {
            if seq_done(w, ctx) { ctx.host.set_anim(&w.mobys[id], 3, 0, 10); }
            let mut t = w.mobys[id].pv_i(0x70);
            let out = countdown(&mut t);
            w.mobys[id].set_pv_i(0x70, t);
            if out {
                w.mobys[id].set_state(1, -1);
                let n = ctx.game.rng.rand_range(0x78, 600);
                w.mobys[id].set_pv_i(0x70, n);
                let s = ctx.game.rng.randi(2) as u8;
                ctx.host.set_anim(&w.mobys[id], s, 0, 10);
            }
        }
        _ => {}
    }
    if !frames && w.mobys[id].state < 0x80 {
        let p = w.mobys[id].pos;
        let z = ai::ground_z(ctx, 0.5, [p[0], p[1], p[2] + 0.5], 0);
        w.mobys[id].pos[2] = z;
        w.changed = true;
    }
}
