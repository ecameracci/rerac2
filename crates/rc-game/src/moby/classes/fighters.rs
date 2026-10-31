//! The arena's fighters (Maktar's chunk 1, L02): pooled above the arena (state 1, hidden) in the master's group
//! `+0x40` until the master sends one to a spawner ([`super::arena::attach`]), which lets it out ([`start`]). Each
//! asks the master for a turn to attack (`0x3fd670`, a count per class: `+0x50..+0x5c`) and gives it back
//! (`0x3fd750`); dead, it drops its bolts and goes back to the pool.
//!
//! **0x0c3e** (`0x40d550`; senses `0x40e238`, tail `0x40e630`, its turn `0x40e6b0`, start `0x40e7c8`, back to the pool
//! `0x40e8c8`): a spinning flier, the movement block `+0x60`, its life `+0x20` (from `+0x24`). Out, it rises toward
//! the arena's middle (3) until it is in the floor's cuboid (the master's `+0x84`), circles its path `+0x180` at a
//! height 1..3 above the floor (7), turning `+0x190` off its way and easing its speed down near the path's end; with
//! a turn (`+0x1b8`) it dives at its target (8: Ratchet, or the master's pick `+0x184`), stops (10), bites (9 → 0xb: its
//! four joints hit for `+0x34`, backing off when within 1.7) until he is hurt by it (0xc) or he is 4 away after two
//! seconds (4). Hit, it is knocked away spinning (0xd, sub 4: falling under `+0x8c` until it lands, its blow on what
//! it lands on) and back to the dive or dead. Low over the floor it skims at 0.3, splashing (a twinkle). 180 ticks
//! stuck sends it back to the pool. It hits what it touches for 1 (a sphere of 0.8).
//!
//! Not ported: the shadow call while near the camera (`0x321db0`).

use crate::hero::physics::{add, add_rot, dist2, scale, set_len, sub, sub_rot, V3};
use crate::moby::{ai, mode, Ctx, MobyWorld};
use super::props::{seq, wrapped};

/// The master's class.
pub const MASTER: i16 = 0x0b59;

fn lerp_rot(a: f32, b: f32, t: f32) -> f32 { add_rot(a, sub_rot(b, a) * t) }

fn set_seq(w: &MobyWorld, id: usize, ctx: &mut Ctx, s: u8, blend: u8) {
    if seq(w, id, ctx) != s { ctx.host.set_anim(&w.mobys[id], s, 0, blend); }
}

/// Its master (`+0xb8`).
pub fn master(w: &MobyWorld, id: usize) -> Option<usize> { w.mobys[id].parent.filter(|&m| w.mobys.get(m).is_some_and(|x| x.o_class == MASTER)) }

/// `0x3fd638`: the arena's middle (the master's cuboid `+0x88`).
pub fn middle(w: &MobyWorld, master: usize, ctx: &Ctx) -> Option<V3> { ctx.level.cuboid_place(w.mobys[master].pv_i(0x88)).map(|c| c.0) }

/// `0x3fd670(master, class)`: a turn to attack from the master's count for the class.
pub fn claim(w: &mut MobyWorld, master: usize, class: i16) -> bool {
    let Some(o) = turn_slot(class) else { return false };
    let m = &mut w.mobys[master];
    let n = m.pv_i(o);
    if n <= 0 { return false; }
    m.set_pv_i(o, n - 1);
    true
}

/// `0x3fd750(master, class)`: the turn given back.
pub fn give_back(w: &mut MobyWorld, master: usize, class: i16) {
    let Some(o) = turn_slot(class) else { return };
    let m = &mut w.mobys[master];
    m.set_pv_i(o, m.pv_i(o) + 1);
}

fn turn_slot(class: i16) -> Option<usize> {
    match class {
        0x09ca => Some(0x50),
        0x0b22 => Some(0x54),
        0x0c3b => Some(0x58),
        0x0c3e => Some(0x5c),
        _ => None,
    }
}

/// The four fighters and the two bosses (the master counts them).
pub const FIGHTERS_AND_BOSSES: [i16; 6] = [C9CA, B22, C3B, C3E, 0x09f8, 0x0a44];

/// A fighter back to the pool (`0x40e8c8`, `0x40d0c8`, `0x3f2e58`; the gladiator's [`pool_09ca`]).
pub fn pool(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].o_class {
        C3E => pool_0c3e(w, id, ctx),
        C3B => pool_0c3b(w, id, ctx),
        B22 => pool_0b22(w, id, ctx),
        C9CA => pool_09ca(w, id, ctx),
        _ => {}
    }
}

/// `0x3d39e0(m, 0, 0)`: the gladiator dead with its bolts, faded back to the pool.
pub fn die_09ca_pub(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) { die_09ca(w, id, ctx, false, false) }

/// The spawner's hand-over (`0x3d8e68`): each class's start.
pub fn start(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].o_class {
        C3E => {
            w.mobys[id].pos[2] += 0.5;
            start_0c3e(w, id, ctx, 0);
        }
        C3B => start_0c3b(w, id, ctx, 0),
        C9CA => start_09ca(w, id, ctx, 0),
        B22 => start_0b22(w, id, ctx, 0),
        _ => {}
    }
}

// ---------------------------------------------------------------------------------------------------------------
// 0x0c3e

pub const C3E: i16 = 0x0c3e;
/// The movement block.
const MV: usize = 0x60;
/// Its spin's eased speeds (`$gp+0x29c0`, `0x1b19b4`, `0x1b19b8`: 0, 0, 180° a second).
const SPIN: V3 = [0.0, 0.0, 180.0];

/// `0x40e7c8(m, delay)`: out of the spawner: its life full, rising (3, sub 1) after `delay`, shown and a target.
fn start_0c3e(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, delay: i16) {
    let coll = w.class(C3E).is_some_and(|c| c.has_collision);
    let a = ctx.game.rng.randf(0.0, 90.0);
    let m = &mut w.mobys[id];
    m.set_pv_s(0x1b8, 0);
    let life = m.pv_s(0x24) as f32;
    m.set_pv_f(0x20, life);
    m.set_state(3, 1);
    set_seq(w, id, ctx, 0, 10);
    let m = &mut w.mobys[id];
    m.has_collision = coll;
    m.set_pv_b(0x2e, 1);
    m.drops = 0;
    m.drawn = 1;
    m.mode = (m.mode & 0xffb8) | mode::TARGET;
    m.set_pv_s(0x1ba, delay);
    m.set_pv_s(0x3c, 0);
    m.set_pv_i(0x38, 0);
    m.set_pv_f(0x18c, a.to_radians());
    if m.pv_s(0x50) != 0 {
        m.set_pv_s(0x50, 0);
        let c = m.pv_i(0x1a0) as u32;
        m.set_ambient_word(c);
        m.set_pv_s(0x52, 1);
    }
    w.changed = true;
}

/// `0x40e8c8`: back to the pool — its turn given back, a blast and its sound when in view, hidden (state 1).
fn pool_0c3e(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let h = w.mobys[id].pv_i(0x19c);
    if h != -1 { ctx.host.stop_sound(h); }
    w.mobys[id].set_pv_i(0x19c, -1);
    if w.mobys[id].pv_s(0x1b8) == 1 {
        w.mobys[id].set_pv_s(0x1b8, 0);
        if let Some(ms) = master(w, id) { give_back(w, ms, C3E); }
    }
    if w.mobys[id].drawn != 0 {
        let pos = w.mobys[id].pos;
        crate::moby::blast::burst(w, ctx, pos, &crate::moby::blast::Blast { size: 0.3, bursts: [10, 0x1e, 0], puffs: 10, streaks: 0x2d, light: true, shake: false });
        ctx.host.play_sound_of(&w.mobys[id], C3E, 0, 0);
    }
    w.mobys[id].set_pv_i(0x184, 0);
    w.mobys[id].set_pv_s(0x1b8, 0);
    set_seq(w, id, ctx, 0, 3);
    let m = &mut w.mobys[id];
    m.set_state(1, 0);
    m.drawn = 0;
    m.has_collision = false;
    m.mode = (m.mode & !mode::TARGET) | 0x47;
    w.changed = true;
}

/// Its target's place: the master's pick `+0x184`, else Ratchet.
fn target_0c3e(w: &MobyWorld, id: usize, ctx: &Ctx) -> V3 {
    ai::link(&w.mobys[id], 0x184).and_then(|t| w.mobys.get(t)).map_or(ctx.hero.pos, |t| t.pos)
}

/// `0x40e238`: set-up, its sound, the hits it takes.
fn senses_0c3e(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 0 {
        let r = w.mobys[id].pv_f(0x190);
        let off = ctx.game.rng.randf(-r, r).to_radians();
        let m = &mut w.mobys[id];
        m.set_pv_i(0x18, MV as i32);
        m.scale *= 0.75;
        m.set_pv_i(0x19c, -1);
        m.set_pv_b(0x48, 15);
        m.set_pv_b(0x4a, 4);
        m.set_pv_f(0x190, off);
        m.update_dist = 0x80;
        m.set_pv_i(0x194, 1);
        m.set_pv_f(0x198, m.pv_f(0x198) / 60.0);
        ai::senses(m, f32::from_bits(0x3ce3_8e39), f32::from_bits(0x3fd5_5556), f32::from_bits(0x3a98_825c), f32::from_bits(0x3dd6_7752));
        let c = m.ambient;
        m.set_pv_i(0x1a0, c[0] as i32 | (c[1] as i32) << 8 | (c[2] as i32) << 16);
        m.set_pv_f(MV + 8, 0.75);
        m.set_pv_f(MV, 0.75);
        m.set_pv_f(MV + 4, 0.0);
        m.set_pv_i(0x30, 0);
        m.set_pv_s(0x1b8, 0);
        m.set_pv_f(MV + 0xc, 0.5);
        m.set_pv_f(MV + 0x10, f32::from_bits(0x3f91_361f));
        m.set_pv_f(MV + 0x30, f32::from_bits(0x3f06_0a93));
        m.set_pv_f(MV + 0x2c, f32::from_bits(0x3b36_0b61));
        m.set_pv_i(MV + 0x38, m.pv_i(MV + 0x38) | 0x14);
        m.set_pv_b(0x28, 3);
        pool_0c3e(w, id, ctx);
    }
    let h = w.mobys[id].pv_i(0x19c);
    if w.mobys[id].sub_state < 2 {
        if ctx.host.sound_playing(h) {
            ctx.host.stop_sound(h);
            w.mobys[id].set_pv_i(0x19c, -1);
        }
    } else if !ctx.host.sound_playing(h) {
        let h = ctx.host.play_sound(&w.mobys[id], 1, 4);
        w.mobys[id].set_pv_i(0x19c, h);
    }
    // `0x322040(m, 0x10000, 0)`: the hit (the floor plates' and, unseen, the 0x0b89's do nothing).
    let mut hit = w.hit_message(id).filter(|h| h.tmpl.flags & 0x1_0000 != 0);
    if let Some(h) = hit.as_mut() {
        if let Some(src) = h.tmpl.source.and_then(|s| w.mobys.get(s)) {
            if src.o_class == super::arena::PLATE || (w.mobys[id].drawn == 0 && src.o_class == 0x0b89) { h.tmpl.damage = 0.0; }
        }
    }
    let dmg = ai::damage_of(w, id, hit.as_ref(), false);
    w.clear_hit(id);
    if dmg != 0.0 {
        let h = hit.expect("damage without a hit");
        let dir = [h.tmpl.dir[0], h.tmpl.dir[1], h.tmpl.dir[2]];
        let mut push = add(set_len(dir, 3000.0 / 3600.0), ctx.game.rng.rand_vec(-500.0 / 3600.0, 500.0 / 3600.0));
        push[2] = 0.0;
        let spin = ctx.game.rng.rand_vec(360.0 / 206_265.0, 1440.0 / 206_265.0);
        let m = &mut w.mobys[id];
        m.set_pv_f(0x20, m.pv_f(0x20) - dmg);
        let v = m.pv_v(0x100);
        m.set_pv_v(0x100, add(v, push));
        for k in 0..3 { m.set_pv_f(0x170 + 4 * k, add_rot(m.pv_f(0x170 + 4 * k), spin[k])); }
        ai::flash_start(m, 0x50);
        m.set_state(0xd, 4);
    }
    if 1 < w.mobys[id].state && w.mobys[id].pv_b(0x2e) == 2 { pool_0c3e(w, id, ctx); }
    turn_0c3e(w, id, ctx);
}

/// `0x40e6b0`: its turn to attack — kept while it has one (its target from `0x32ad38(100, 100, m, +0x1b0)`), asked
/// for while seen; without one the dive and the bite go back to circling (or 5 without a path).
fn turn_0c3e(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state < 4 {
        w.mobys[id].set_pv_i(0x184, 0);
        w.mobys[id].set_pv_s(0x1b8, 0);
        return;
    }
    let mut has = w.mobys[id].pv_s(0x1b8) == 1;
    if !has {
        if w.mobys[id].drawn != 0 && master(w, id).is_some_and(|ms| claim(w, ms, C3E)) {
            w.mobys[id].set_pv_s(0x1b8, 1);
            has = true;
        } else {
            w.mobys[id].set_pv_s(0x1b8, 0);
        }
    }
    if has {
        let area = w.mobys[id].pv_i(0x1b0);
        let found = ai::find_target(w, id, ctx, 100.0, 100.0, area, None, false);
        let m = &mut w.mobys[id];
        m.set_pv_s(0x1b6, found as i16);
        let t = m.pv_i(0x160);
        m.set_pv_i(0x184, t);
        return;
    }
    let s = w.mobys[id].state;
    if s == 0xb || s == 8 {
        if w.mobys[id].pv_i(0x180) == -1 { w.mobys[id].set_state(5, 2); } else { w.mobys[id].set_state(7, 3); }
    }
}

/// `0x40e630`: the colour flash and its end, the state's ticks.
fn tail_0c3e(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    ai::flash(m, 0x50);
    if m.pv_s(0x50) == 0 {
        if m.pv_i(0x1ac) == 0 {
            let c = m.pv_i(0x1a0) as u32;
            m.set_ambient_word(c);
            m.set_pv_i(0x1ac, 1);
        }
    } else {
        m.set_pv_i(0x1ac, 0);
    }
    if m.state_ticks != -1 { m.state_ticks = m.state_ticks.wrapping_add(1); }
}

fn ease_z(w: &mut MobyWorld, id: usize, z: f32) {
    let m = &mut w.mobys[id];
    let (mut x, mut v) = (m.pos[2], m.pv_f(MV + 0xa8));
    crate::hero::physics::ease(z, f32::from_bits(0x3b36_0b61), f32::from_bits(0x3b36_0b61), f32::from_bits(0x3e2a_aaab), &mut x, &mut v);
    m.pos[2] = x;
    m.set_pv_f(MV + 0xa8, v);
}

/// `0x332798(m, goal)`: the move toward `goal`.
fn go(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, goal: V3) {
    let m = &w.mobys[id];
    let yaw = crate::hero::physics::atan2(goal[0] - m.pos[0], goal[1] - m.pos[1]);
    ai::move_step(w, id, ctx, yaw, goal);
}

/// 0x0c3e (`0x40d550`). The fighters' distances are all `0x2f2f50`'s, across (xy).
pub fn update_0c3e(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 1 { return; }
    senses_0c3e(w, id, ctx);
    if w.mobys[id].state == 1 { return; }
    let tgt = target_0c3e(w, id, ctx);
    // `0x2f2f50`: the xy distance (as are the path's nearest point and its 2 to the next; out of view the ball is held
    // at the floor while its path height is 1 to 3 above it).
    let d = dist2(w.mobys[id].pos, tgt);
    if w.mobys[id].sub_state == 4 {
        // Knocked away: it falls until it lands (or, in the air, one tick in 30), its blow on what it lands on.
        let old = w.mobys[id].pos;
        let m = &mut w.mobys[id];
        let mut v = m.pv_v(0x100);
        v[2] -= m.pv_f(MV + 0x2c);
        m.set_pv_v(0x100, v);
        m.pos = add(m.pos, v);
        let new = m.pos;
        let landed = w.coll_line(ctx, old, new, 0, Some(id)).is_some()
            || ctx.coll.is_some_and(|c| crate::collision::coll_sphere_m(c, ctx.scene, new, 0.3, crate::collision::QueryFlags(4), Some(id)).is_some())
            || ctx.game.rng.randi(0x1e) == 0;
        if landed {
            let dmg = w.mobys[id].pv_b(0x34) as f32 * 3.0;
            ai::attack_sphere(w, id, ctx, 0.5, dmg, 1.0, new, 0x1_0000, 0, 1, 0);
            if 0.0 < w.mobys[id].pv_f(0x20) {
                w.mobys[id].set_state(8, 3);
                set_seq(w, id, ctx, 0, 5);
            } else {
                crate::moby::bolts::drop(w, id, 0, 3, ctx);
                pool_0c3e(w, id, ctx);
            }
        }
    }
    let mid = master(w, id).and_then(|ms| middle(w, ms, ctx));
    let floor = mid.map_or(0.0, |p| p[2]);
    match w.mobys[id].state {
        3 => {
            if w.mobys[id].first() {
                ai::senses(&mut w.mobys[id], f32::from_bits(0x3c88_8889), f32::from_bits(0x3eaa_aaab), f32::from_bits(0x3a98_825c), f32::from_bits(0x3dd6_7752));
                w.mobys[id].timer = 0;
            }
            let m = &w.mobys[id];
            if m.timer == 0 && (m.pv_s(0x1ba) as i32) < m.state_ticks as u16 as i32 {
                w.mobys[id].timer = 1;
            } else if m.timer == 1 {
                let floor_box = master(w, id).map_or(-1, |ms| w.mobys[ms].pv_i(0x84));
                if ctx.level.in_cylinder(floor_box, w.mobys[id].pos) { w.mobys[id].set_state(7, 3); }
                if let Some(p) = mid { go(w, id, ctx, p); }
            }
        }
        4 => {
            if 0x19 < w.mobys[id].state_ticks as u16 {
                set_seq(w, id, ctx, 0, 5);
                w.mobys[id].set_state(8, 3);
            }
        }
        7 => {
            let path = w.mobys[id].pv_i(0x180);
            let pts = usize::try_from(path).ok().and_then(|i| ctx.level.paths.get(i)).cloned().unwrap_or_default();
            let n = pts.len() as i32;
            if w.mobys[id].first() {
                ai::senses(&mut w.mobys[id], f32::from_bits(0x3ce3_8e39), f32::from_bits(0x4055_5556), f32::from_bits(0x3a98_825c), f32::from_bits(0x3dd6_7752));
                let h = floor + ctx.game.rng.randf(1.0, 3.0);
                w.mobys[id].set_pv_f(0x188, h);
                // The nearest point (of all but the last).
                let pos = w.mobys[id].pos;
                let mut best = 0;
                let mut near = pts.first().map_or(0.0, |p| dist2(pos, [p[0], p[1], p[2]]));
                for (k, p) in pts.iter().enumerate().take((n - 1).max(0) as usize) {
                    let dk = dist2(pos, [p[0], p[1], p[2]]);
                    if dk < near { near = dk; best = k; }
                }
                w.mobys[id].set_pv_i(0x194, best as i32);
                let dir = ctx.game.rng.randi(2);
                w.mobys[id].set_pv_s(0x1b4, dir as i16);
            } else if 0 < n {
                let k = w.mobys[id].pv_i(0x194).clamp(0, n - 1) as usize;
                let goal = [pts[k][0], pts[k][1], w.mobys[id].pv_f(0x188)];
                let m = &mut w.mobys[id];
                let step = set_len(sub(goal, m.pos), m.pv_f(0x198));
                m.pos = add(m.pos, step);
                let near = dist2(m.pos, goal) < 2.0;
                let mut i = m.pv_i(0x194);
                if m.pv_s(0x1b4) == 0 {
                    if near { i += 1; }
                    if n <= i { i = 0; }
                } else {
                    if near { i -= 1; }
                    if i < 0 { i = n - 1; }
                }
                m.set_pv_i(0x194, i);
                if n - 6 <= i && 0.100_000_01 <= m.pv_f(0x198) { m.set_pv_f(0x198, m.pv_f(0x198) - 0.0025); }
                let off = m.pv_f(0x190);
                ai::move_toward(w, id, ctx, off, goal);
                let h = w.mobys[id].pv_f(0x188);
                ease_z(w, id, h);
            }
            if w.mobys[id].pv_s(0x1b8) != 0 { w.mobys[id].set_state(8, 3); }
        }
        8 => {
            if w.mobys[id].first() {
                ai::senses(&mut w.mobys[id], f32::from_bits(0x3c08_8889), 1.0, f32::from_bits(0x3a98_825c), f32::from_bits(0x3dd6_7752));
            }
            if 1.7 <= d {
                let off = w.mobys[id].pv_f(0x190);
                ai::move_toward(w, id, ctx, off, tgt);
            }
            let hz = floor + 0.7;
            ease_z(w, id, hz);
            if d < 1.7 && (w.mobys[id].pos[2] - hz).abs() < 0.1 {
                w.mobys[id].set_state(10, -1);
                w.mobys[id].state_ticks = 0;
            }
        }
        9 => {
            if w.mobys[id].first() { set_seq(w, id, ctx, 2, 3); }
            if wrapped(w, id, ctx) {
                set_seq(w, id, ctx, 3, 5);
                w.mobys[id].set_state(0xb, -1);
            }
        }
        10 => {
            if 0x2c < w.mobys[id].state_ticks as u16 {
                w.mobys[id].set_state(9, -1);
                set_seq(w, id, ctx, 2, 2);
                ctx.host.set_anim_speed(&w.mobys[id], 1.0);
            }
        }
        0xb => {
            if w.mobys[id].once & 2 == 0 {
                w.mobys[id].once |= 2;
                ai::senses(&mut w.mobys[id], f32::from_bits(0x3a91_a2b4), f32::from_bits(0x3d08_8889), f32::from_bits(0x3a98_825c), f32::from_bits(0x3dd6_7752));
                w.mobys[id].state_ticks = 0;
            }
            ctx.host.set_anim_speed(&w.mobys[id], 1.75);
            ease_z(w, id, floor + 0.7);
            let pos = w.mobys[id].pos;
            if 1.7 <= d {
                go(w, id, ctx, tgt);
            } else {
                go(w, id, ctx, add(pos, set_len(sub(pos, tgt), 2.0)));
            }
            let dmg = w.mobys[id].pv_b(0x34) as f32;
            for j in 0..4 { ai::attack_joint(w, id, ctx, 0.5, dmg, 1.0, j, 0x400_0001, 0, 1, 0); }
            if ctx.hero.damage.hit_by == Some(id) {
                w.mobys[id].set_state(0xc, -1);
                set_seq(w, id, ctx, 4, 5);
                ctx.host.set_anim_speed(&w.mobys[id], 1.0);
            }
            if 4.0 < d && 0x78 < w.mobys[id].state_ticks as u16 {
                set_seq(w, id, ctx, 5, 0xf);
                w.mobys[id].set_state(4, -1);
                ctx.host.set_anim_speed(&w.mobys[id], 1.0);
            }
        }
        0xc => {
            if wrapped(w, id, ctx) {
                set_seq(w, id, ctx, 0, 0xf);
                w.mobys[id].set_state(8, -1);
            }
        }
        _ => {}
    }
    // The spin (its speeds eased a sixteenth of the way to their ends each tick).
    {
        let m = &mut w.mobys[id];
        for k in 0..3 {
            let s = lerp_rot(m.pv_f(0x170 + 4 * k), SPIN[k].to_radians() / 60.0, 1.0 / 16.0);
            m.set_pv_f(0x170 + 4 * k, s);
            m.rot[k] = add_rot(m.rot[k], s);
        }
    }
    if mid.is_some() && w.mobys[id].pos[2] <= floor + 0.4 {
        // Skimming the floor: held at 0.3 above it (in view, only when its sphere touches the floor or a moby), with
        // a splash.
        // `0x2e2e70(0.3, pos, 6, 0)`: the sphere query (the world, then the mobys).
        let q = ctx.coll.and_then(|c| crate::collision::coll_sphere_m(c, ctx.scene, w.mobys[id].pos, 0.3, crate::collision::QueryFlags(6), None));
        let skim = w.mobys[id].drawn == 0 || q.is_some();
        if skim {
            let m = &mut w.mobys[id];
            m.set_pv_f(MV + 0xa8, f32::from_bits(0x3c88_8889));
            m.pos[2] = floor + 0.3;
            if m.drawn != 0 { splash(w, id, ctx); }
        }
    }
    if 0xb4 < w.mobys[id].pv_i(MV + 0x80) {
        crate::moby::bolts::drop(w, id, 0, 3, ctx);
        pool_0c3e(w, id, ctx);
        w.mobys[id].set_pv_i(MV + 0x80, 0);
    }
    let pos = w.mobys[id].pos;
    ai::attack_sphere(w, id, ctx, 0.8, 1.0, 1.0, pos, 0x400_0000, 0, 1, 0);
    tail_0c3e(w, id);
}

/// The skim's splash: a twinkle `randf(0.3, 0.8)` (a tenth of that at its end) just under it, rising.
fn splash(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let rng = &mut ctx.game.rng;
    let pos = w.mobys[id].pos;
    let p = [pos[0] + rng.randf(-0.01, 0.01), pos[1] + rng.randf(-0.01, 0.01), pos[2] - 0.3];
    let s = rng.randf(0.3, 0.8);
    let spin = if rng.randi(2) == 0 { -1 } else { 1 };
    let mut v = rng.rand_vec(0.05 / 60.0, 3.0 / 60.0);
    v[2] = rng.randf(1.0, 5.0) / 60.0;
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    crate::particles::twinkle(sys, &mut ctx.game.rng, s * 0.1, s, f32::from_bits(0x3b03_126f), p, 0x3c, 0x7f00_7f7f, 0, spin, v, 0);
}

// ---------------------------------------------------------------------------------------------------------------
// 0x0c3b

pub const C3B: i16 = 0x0c3b;
/// Its movement block, its spot (`0x40d200`), its target (`+0x2d4`, from the target block's `+0x200`), turn offset.
const MV_3B: usize = 0x100;
const SPOT: usize = 0x2c0;
const TARGET_3B: usize = 0x2d4;
/// `$gp+0x2968..+0x2970`: a raised plate within 4.8 and 85° of its way is gone round 5° a tick.
const PLATE_NEAR: f32 = 4.8;
const PLATE_CONE: f32 = 85.0;
const PLATE_TURN: f32 = 5.0;
/// Its glows' colours (`0x40cbf8`): the visor orange, the chest and the mouth blue.
const GLOW_3B: [u32; 2] = [0x0000_36f6, 0x0010_50b0];

/// `0x40cfb0(m, delay)`: out of the spawner, standing (3, sub 1, sequence 2) for `delay` ticks, then walking in.
fn start_0c3b(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, delay: i16) {
    let coll = w.class(C3B).is_some_and(|c| c.has_collision);
    let m = &mut w.mobys[id];
    m.set_pv_s(0x2e2, 1);
    let life = m.pv_s(0x24) as f32;
    m.set_pv_f(0x20, life);
    ai::senses(m, f32::from_bits(0x3b91_a2b4), f32::from_bits(0x3e08_8889), f32::from_bits(0x3a98_825c), f32::from_bits(0x3dd6_7752));
    m.set_state(3, 1);
    set_seq(w, id, ctx, 2, 0);
    let m = &mut w.mobys[id];
    m.has_collision = coll;
    m.set_pv_b(0x6a, 0);
    m.set_pv_b(0x2e, 1);
    m.drawn = 1;
    m.drops = 0;
    m.mode = (m.mode & 0xffb8) | mode::TARGET;
    m.set_pv_s(0x2e8, delay);
    m.set_pv_s(0x88, 0);
    if m.pv_s(0x50) != 0 {
        m.set_pv_s(0x50, 0);
        let c = [m.pv_b(0x54), m.pv_b(0x55), m.pv_b(0x56)];
        m.ambient = c;
        m.set_pv_s(0x52, 1);
    }
    w.changed = true;
}

/// `0x40d0c8`: back to the pool — in view, a blast, 0x0c3e's sound 0 and its pieces thrown; hidden (state 1).
fn pool_0c3b(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].set_pv_s(0x2e2, 0);
    w.mobys[id].set_pv_i(TARGET_3B, 0);
    if w.mobys[id].drawn != 0 {
        let pos = w.mobys[id].pos;
        crate::moby::blast::burst(w, ctx, pos, &crate::moby::blast::Blast { size: 0.5, bursts: [5, 5, 0x19], puffs: 0x14, streaks: 10, light: true, shake: false });
        ctx.host.play_sound_of(&w.mobys[id], C3E, 0, 0);
        let v = add(ctx.game.rng.rand_vec(f32::from_bits(0x3daa_aaab), f32::from_bits(0x3dee_eef0)), w.mobys[id].pv_v(0xd0));
        crate::moby::shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 6.0), 0xff, v, 0xff);
    }
    set_seq(w, id, ctx, 0, 3);
    let m = &mut w.mobys[id];
    m.set_state(1, 0);
    m.drawn = 0;
    m.has_collision = false;
    m.mode = (m.mode & !mode::TARGET) | 0x47;
    w.changed = true;
}

/// `0x40c840`: set-up, its shadow, its joints' points (the glows), the hits it takes and its target.
fn senses_0c3b(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 0 {
        let a = ctx.game.rng.randf(15.0, 40.0).to_radians();
        let neg = ctx.game.rng.randi(2) != 0;
        let mirror = ctx.game.rng.randi(2) == 1;
        let m = &mut w.mobys[id];
        m.set_pv_i(0x18, MV_3B as i32);
        m.set_pv_b(0x48, 15);
        m.set_pv_b(0x4a, 12);
        m.set_pv_f(0x2d8, if neg { -a } else { a });
        if mirror { m.mode |= mode::MIRROR; }
        m.set_pv_i(0x98, 4);
        m.set_pv_b(0x60, 8);
        m.set_pv_f(0x100, 0.8);
        m.set_pv_f(0x104, 0.5);
        m.set_pv_f(0x10c, 0.4);
        m.set_pv_f(0x110, f32::from_bits(0x3fc9_0fdc));
        m.set_pv_f(0x130, f32::from_bits(0x3f06_0a93));
        m.set_pv_f(0x12c, f32::from_bits(0x3bb6_0b61));
        m.set_pv_f(0x80, 0.5);
        m.set_pv_f(0x84, 0.5);
        m.set_pv_f(0x8c, 0.4);
        m.set_pv_f(0x108, 0.4);
        ai::senses(m, f32::from_bits(0x3b91_a2b4), f32::from_bits(0x3e08_8889), f32::from_bits(0x3a98_825c), f32::from_bits(0x3dd6_7752));
        m.set_pv_b(0x28, 3);
        pool_0c3b(w, id, ctx);
    }
    crate::shadow_volumes::probe_near(w, id, ctx, 24.0, 0x16);
    for (k, o) in [(1, 0x290), (2, 0x2a0)] {
        if let Some(p) = ctx.host.joint(&w.mobys[id], k) { w.mobys[id].set_pv_v(o, p); }
    }
    if let Some((r, t)) = ctx.host.joint_matrix(&w.mobys[id], 3) { w.mobys[id].set_pv_v(0x2b0, add(t, scale(r[0], 0.25))); }
    if w.mobys[id].drawn == 0 || w.mobys[id].state <= 2 { return; }
    if ai::take_damage(w, id, ctx, false) == 5 {
        w.mobys[id].set_pv_f(0xe0, 0.0);
        w.mobys[id].set_pv_f(0xe4, f32::from_bits(0x3f56_7752));
    }
    if 1 < w.mobys[id].state && w.mobys[id].pv_b(0x2e) == 2 { pool_0c3b(w, id, ctx); }
    let area = w.mobys[id].pv_i(0x2dc);
    let found = ai::find_target(w, id, ctx, 100.0, 100.0, area, None, false);
    let m = &mut w.mobys[id];
    m.set_pv_s(0x2e0, found as i16);
    let t = m.pv_i(0x200);
    m.set_pv_i(TARGET_3B, t);
}

/// `0x40d200(m, p)` (0x0c3b; 0x09ca's `0x3d3b30`, 0x0b22's `0x3f2ff8` the same at their own words): a spot 6..10 from
/// `p` on its side (±45°), at `out`.
fn pick_spot(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, p: V3, out: usize) {
    let m = &w.mobys[id];
    let a = crate::hero::physics::atan2(m.pos[0] - p[0], m.pos[1] - p[1]) + ctx.game.rng.randf(-std::f32::consts::FRAC_PI_4, std::f32::consts::FRAC_PI_4);
    let r = ctx.game.rng.randf(-2.0, 2.0) + 8.0;
    w.mobys[id].set_pv_v(out, [p[0] + r * a.cos(), p[1] + r * a.sin(), p[2]]);
}

/// `0x40d2e8`: the turn about (6, sub 3), its state and sub kept (`+0x2e4` / `+0x2e6`).
fn turn_about(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    let (s, b) = (m.state, m.sub_state);
    m.set_pv_s(0x2e4, s as i16);
    m.set_pv_s(0x2e6, b as i16);
    m.set_state(6, 3);
}

/// `0x321c00(angle, out, v, axis)`: `v` turned `angle` about `axis`.
fn rotate(v: V3, angle: f32, axis: V3) -> V3 {
    if angle.abs() < 1e-5 { return v; }
    let k = crate::hero::physics::set_len(axis, 1.0);
    let (s, c) = angle.sin_cos();
    let d = k[0] * v[0] + k[1] * v[1] + k[2] * v[2];
    let x = crate::hero::physics::cross(k, v);
    [0, 1, 2].map(|i| v[i] * c + x[i] * s + k[i] * d * (1.0 - c))
}

/// `0x40d320`: a raised plate of its group `+0x2d0` close ahead is gone round (its speed `+0x134` 0.0001 for the move).
fn round_plates(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool { round_plates_with(w, id, ctx, 0x2d0, MV_3B + 0x34) }

/// [`round_plates`] with the plates' group at `group` and the speed word at `speed`.
fn round_plates_with(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, group: usize, speed: usize) -> bool {
    let g = w.mobys[id].pv_i(group);
    let Some(list) = usize::try_from(g).ok().and_then(|g| w.groups.get(g)).cloned() else { return false };
    for p in list {
        if w.mobys.get(p).is_none_or(|x| x.o_class != super::arena::PLATE) || !super::arena::raised(w, p) { continue; }
        let (me, at) = (w.mobys[id].pos, w.mobys[p].pos);
        if PLATE_NEAR <= dist2(me, at) { continue; }
        let yaw = crate::hero::physics::atan2(at[0] - me[0], at[1] - me[1]);
        let face = w.mobys[id].rot[2];
        if PLATE_CONE.to_radians() <= crate::hero::physics::rot_dist(face, yaw) { continue; }
        let mut a = PLATE_TURN.to_radians();
        if 0.0 <= sub_rot(face, yaw) { a = -a; }
        let v = rotate(sub(me, at), a, w.mobys[id].rows[2]);
        let goal = add(v, at);
        let keep = w.mobys[id].pv_f(speed);
        w.mobys[id].set_pv_f(speed, f32::from_bits(0x38d1_b717));
        go(w, id, ctx, goal);
        w.mobys[id].set_pv_f(speed, keep);
        return true;
    }
    false
}

/// The flame jet (sequence 0xd, in view): from joint 4 along its second row — each tick three flame sprites (and,
/// within 3 of its target, three spheres of 0.3 doing `+0x34`, each of the three times) and two smoke puffs; the
/// jet of 0x0b0a's ([`super::enemies`]) with a life of 9.
fn flame(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, d: f32) {
    let at = w.mobys[id].pos;
    if ctx.view.is_some_and(|v| v.culled(128.0, [at[0], at[1], at[2], 2.0])) { return; }
    let Some((r, t0)) = ctx.host.joint_matrix(&w.mobys[id], 4) else { return };
    let mut b = add(t0, set_len(r[1], 0.85));
    let mut c = add(t0, set_len(r[1], 1.6));
    b[2] -= 0.1;
    c[2] -= 0.1;
    let vel = set_len(r[1], 0.25);
    let dmg = w.mobys[id].pv_b(0x34) as f32;
    let mut t = t0;
    for _ in 0..3 {
        for k in 0..3 { t[k] += ctx.game.rng.randf(-0.1, 0.1); }
        let g = &mut ctx.game.rng;
        let s0 = g.randf(0.2, 0.4) * 210_000.0;
        let s1 = g.randf(1.0, 1.5) * 210_000.0;
        let fall = g.randf_signed(0.0, 0.01);
        if let Some(sys) = ctx.parts.as_deref_mut() { crate::particles::sprite(sys, g, s0, s1, fall, t, vel, 9, 0x80_8080, 0x1e, 0x7f, 0x20, 0); }
        if d < 3.0 {
            for p in [t, b, c] { ai::attack_sphere(w, id, ctx, 0.3, dmg, 1.0, p, 0x400_0001, 0, 1, 0); }
        }
    }
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let def = sys.def_first(0x17);
    for _ in 0..2 {
        let g = &mut ctx.game.rng;
        if let Some(i) = crate::particles::spark(sys, g, 105_000.0, 21_000.0, 1.0, -0.0002, 0.0, b, [0.0, 0.0, 0.1], 0xf, 0x3c, 0x60_6060, 3) {
            use crate::particles::rec;
            let rc = &mut sys.pool.recs[i];
            rc[3] = 0x44;
            rc[2] = def;
            if g.rand_range(0, 1) != 0 {
                rc[3] = 0x48;
                rec::set_u32(rc, 4, 0x0070_707f);
                rec::set_i16(rc, 0x3e, 0x7f);
                rc[0x31] = 0x70;
                rc[0x32] = 0x70;
                rc[0x33] = 0x70;
            }
        }
    }
}

/// 0x0c3b (`0x40bbf0`): a flame-jet robot on the framework (react block `+0x50`, movement `+0x100`). Out, it walks
/// in to the floor's cuboid (3), then (4 / sub 2) wanders between spots 6..10 from the middle, now and then idling
/// (a random sequence of 2, 3, 5, 6), turning about (6) for one behind it; told to fight (`+0x2e2` cleared: the
/// master's), it closes on its target (7) — within 15 and facing him within 90° it opens its flame (sequence 0xd),
/// else it walks to 2.8 of him (round raised plates, its yaw `+0x2d8` off) or turns to him. Knocked (sub 4), its
/// flight: dead (or one in 30 while in its death sequence 0x10), its bolts and back to the pool; over, the fight.
/// It hits what it touches for 1 (1.3), and draws three glows (the visor, the chest, the mouth).
pub fn update_0c3b(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 1 { return; }
    senses_0c3b(w, id, ctx);
    if w.mobys[id].state == 1 { return; }
    if w.mobys[id].pv_s(0x2e2) == 0 && w.mobys[id].sub_state == 3 {
        w.mobys[id].set_state(4, 2);
        let s = if ctx.game.rng.randi(2) == 0 { 3 } else { 2 };
        set_seq(w, id, ctx, s, 10);
    }
    let tgt = ai::link(&w.mobys[id], TARGET_3B).and_then(|t| w.mobys.get(t)).map_or(ctx.hero.pos, |t| t.pos);
    let pos = w.mobys[id].pos;
    let yaw_t = crate::hero::physics::atan2(tgt[0] - pos[0], tgt[1] - pos[1]);
    let d = dist2(pos, tgt);
    let mut sub_done = false;
    match w.mobys[id].sub_state {
        2 => {
            if w.mobys[id].once & 2 == 0 {
                w.mobys[id].once |= 2;
                set_seq(w, id, ctx, 10, 10);
                w.mobys[id].timer = 0;
                ai::senses(&mut w.mobys[id], f32::from_bits(0x3b91_a2b4), f32::from_bits(0x3e08_8889), f32::from_bits(0x3a18_825c), f32::from_bits(0x3d56_7752));
                if let Some(p) = master(w, id).and_then(|ms| middle(w, ms, ctx)) { pick_spot(w, id, ctx, p, SPOT); }
                w.mobys[id].state_ticks = 10000;
            }
            if w.mobys[id].timer == 1 && 0x12d <= w.mobys[id].state_ticks as u16 {
                match ctx.game.rng.randi(5) {
                    0 => set_seq(w, id, ctx, 2, 10),
                    1 => set_seq(w, id, ctx, 3, 10),
                    2 => set_seq(w, id, ctx, 5, 10),
                    3 => set_seq(w, id, ctx, 6, 10),
                    _ => {}
                }
                w.mobys[id].state_ticks = ctx.game.rng.randi(100) as i16;
            }
            if w.mobys[id].pv_s(0x2e2) != 0 {
                w.mobys[id].set_state(5, 3);
                let s = if ctx.game.rng.randi(2) == 0 { 4 } else { 1 };
                set_seq(w, id, ctx, s, 10);
            }
            let spot = w.mobys[id].pv_v(SPOT);
            let p = w.mobys[id].pos;
            let yaw_s = crate::hero::physics::atan2(spot[0] - p[0], spot[1] - p[1]);
            if 2.792_527_2 < crate::hero::physics::rot_dist(w.mobys[id].rot[2], yaw_s) { turn_about(w, id); }
            if w.mobys[id].timer != 0 || 0.5 <= dist2(w.mobys[id].pos, spot) {
                go(w, id, ctx, spot);
            } else {
                w.mobys[id].timer = 1;
            }
            sub_done = true;
        }
        4 => {
            let f = ai::flight(w, id, ctx);
            let dying = ctx.host.anim(&w.mobys[id]).is_some_and(|a| a.seq_a == 0x10) && ctx.game.rng.randi(0x1e) == 0;
            if dying || f & 1 != 0 {
                crate::moby::bolts::drop(w, id, 0, -1, ctx);
                pool_0c3b(w, id, ctx);
            } else if f & 2 != 0 {
                w.mobys[id].set_state(7, 3);
            }
            sub_done = true;
        }
        _ => {}
    }
    let _ = sub_done;
    match w.mobys[id].state {
        5 => {
            if wrapped(w, id, ctx) { w.mobys[id].set_state(7, -1); }
        }
        3 => {
            if w.mobys[id].first() {
                w.mobys[id].timer = 0;
                set_seq(w, id, ctx, 2, 0);
                w.mobys[id].set_pv_i(0x13c, -1);
            }
            let m = &w.mobys[id];
            if m.timer == 0 && (m.pv_s(0x2e8) as i32) < m.state_ticks as u16 as i32 {
                set_seq(w, id, ctx, 10, 10);
                w.mobys[id].timer = 1;
            } else if w.mobys[id].timer == 1 {
                if let Some(ms) = master(w, id) {
                    let floor_box = w.mobys[ms].pv_i(0x84);
                    if ctx.level.in_cylinder(floor_box, w.mobys[id].pos) {
                        w.mobys[id].set_state(4, 2);
                        w.mobys[id].set_pv_i(0x13c, -1);
                    }
                    if let Some(p) = middle(w, ms, ctx) { go(w, id, ctx, p); }
                }
            }
        }
        6 => {
            if w.mobys[id].first() {
                set_seq(w, id, ctx, 9, 10);
                ai::senses(&mut w.mobys[id], f32::from_bits(0x3b91_a2b4), f32::from_bits(0x3e08_8889), f32::from_bits(0x3b64_c38a), f32::from_bits(0x3ed6_7752));
            }
            // `0x332b50(yaw, m)`: turning in place.
            let p = w.mobys[id].pos;
            ai::move_step(w, id, ctx, yaw_t, p);
            if wrapped(w, id, ctx) { w.mobys[id].set_state(7, -1); }
        }
        7 => {
            if w.mobys[id].first() {
                ai::senses(&mut w.mobys[id], f32::from_bits(0x3b91_a2b4), f32::from_bits(0x3e08_8889), f32::from_bits(0x3a18_825c), f32::from_bits(0x3d56_7752));
                set_seq(w, id, ctx, 10, 0xf);
            }
            let firing = ctx.host.anim(&w.mobys[id]).is_some_and(|a| a.seq_a == 0xd);
            let p = w.mobys[id].pos;
            let hero = ctx.hero.pos;
            let yaw_h = crate::hero::physics::atan2(hero[0] - p[0], hero[1] - p[1]);
            let close = !firing && dist2(p, hero) < 15.0 && crate::hero::physics::rot_dist(w.mobys[id].rot[2], yaw_h) < std::f32::consts::FRAC_PI_2;
            if close {
                set_seq(w, id, ctx, 0xd, 10);
                ai::senses(&mut w.mobys[id], f32::from_bits(0x3a91_a2b4), f32::from_bits(0x3d08_8889), f32::from_bits(0x3a18_825c), f32::from_bits(0x3d56_7752));
            } else if 2.8 < dist2(p, hero) {
                if !round_plates(w, id, ctx) {
                    let off = w.mobys[id].pv_f(0x2d8);
                    ai::move_toward(w, id, ctx, off, tgt);
                }
            } else {
                ai::move_step(w, id, ctx, yaw_h, p);
            }
            if 2.792_527_2 < crate::hero::physics::rot_dist(w.mobys[id].rot[2], yaw_t) { turn_about(w, id); }
            if ctx.host.anim(&w.mobys[id]).is_some_and(|a| a.seq_a == 0xd) { flame(w, id, ctx, d); }
        }
        _ => {}
    }
    // `0x40cb68`: the ticks, its blow on what it touches.
    let m = &mut w.mobys[id];
    if m.state_ticks != -1 { m.state_ticks = m.state_ticks.wrapping_add(1); }
    let pos = w.mobys[id].pos;
    ai::attack_sphere(w, id, ctx, f32::from_bits(0x3fa6_6666), 1.0, 1.0, pos, 0x400_0000, 0, 1, 0);
    w.register_draw(id);
}

/// `0x40cbf8` (0x0c3b, drawn, not standing): its three glows (`0x40cd30` = [`crate::fx::glow`]): the visor at joint 1
/// (0.125, orange, 0x20 / 0x20), the chest at joint 2 (0.15, blue, 0x30 / 0x20), the mouth (0.15, blue, 0x10 / 0x30).
pub fn draw_0c3b(w: &MobyWorld, id: usize, rng: &mut crate::rng::Rng, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let m = &w.mobys[id];
    if m.sub_state == 0 { return; }
    crate::fx::glow(out, rng, 0.125, m.pv_v(0x290), GLOW_3B[0], 0x20, 0x20, cam);
    crate::fx::glow(out, rng, 0.15, m.pv_v(0x2a0), GLOW_3B[1], 0x30, 0x20, cam);
    crate::fx::glow(out, rng, 0.15, m.pv_v(0x2b0), GLOW_3B[1], 0x10, 0x30, cam);
}

// ---------------------------------------------------------------------------------------------------------------
// 0x09ca and its shield 0x09d8

pub const C9CA: i16 = 0x09ca;
pub const SHIELD: i16 = 0x09d8;
/// Its target (`+0x2e0`, from the target block's `+0x200`), its shield (`+0x2e4`), its turn (`+0x2f6`: 1 the
/// master's, 2 sparring with another gladiator), its spot (`+0x2d0`), its yaw offset (`+0x2e8`).
const TARGET_9CA: usize = 0x2e0;
const SHIELD_LINK: usize = 0x2e4;
const TURN_9CA: usize = 0x2f6;
const SPOT_9CA: usize = 0x2d0;

fn shield(w: &MobyWorld, id: usize) -> Option<usize> {
    ai::link(&w.mobys[id], SHIELD_LINK).filter(|&s| w.mobys.get(s).is_some_and(|x| x.state < 0x80 && x.o_class == SHIELD))
}

/// `0x3d3398`: its shield made (class 0x09d8: its update and draw distances, shown, its light; no collision; `+0x3c`
/// carried).
fn make_shield(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.class(SHIELD).is_none() {
        w.mobys[id].set_pv_i(SHIELD_LINK, 0);
        return;
    }
    let s = w.create(SHIELD);
    let (ud, dd, light, amb, pos) = { let m = &w.mobys[id]; (m.update_dist, m.draw_dist, m.light, m.ambient, m.pos) };
    let x = &mut w.mobys[s];
    (x.update_dist, x.draw_dist, x.drawn, x.light, x.ambient, x.pos) = (ud, dd, 1, light, amb, pos);
    x.has_collision = false;
    x.set_pv_i(0x3c, 1);
    w.rebuild(s, ctx.host);
    ai::set_link(&mut w.mobys[id], SHIELD_LINK, Some(s));
    w.changed = true;
}

/// `0x3d3428`: its shield dropped — in view thrown as a tumbling piece (`0x32c760`: a new 0x09d8 at its place and
/// angles, `randvec(0.1, 0.167)` a tick, falling 1/360 a tick); the carried one deleted.
fn drop_shield(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let Some(s) = shield(w, id) else { return };
    if w.mobys[id].drawn != 0 {
        let v = ctx.game.rng.rand_vec(f32::from_bits(0x3dcc_ccce), f32::from_bits(0x3e2a_aaab));
        let x = &w.mobys[s];
        let (pos, rot, rows) = (x.pos, x.rot, x.rows);
        let d = sub([x.bsphere[0], x.bsphere[1], x.bsphere[2]], pos);
        let dot = crate::hero::physics::dot;
        let sphere = [dot(d, rows[0]), dot(d, rows[1]), dot(d, rows[2]), x.bsphere[3]];
        throw_piece(w, id, ctx, SHIELD, pos, rot, v, f32::from_bits(0x3b36_0b61), sphere);
    }
    w.delete(s);
    w.mobys[id].set_pv_i(SHIELD_LINK, 0);
}

/// `0x32c760(gravity, m, class, pos, angles, life 0, 0, vel, spin 0, centre 0)`: a tumbling piece — a moby of `class`
/// scaled as `m` is to its class, state 1, its life `randf(60, 120)` ticks, its spin a random `randvec(3°, 6°)` a
/// tick, about its sphere's centre (here the carried piece's sphere, in its frame; [`update_shield`] moves it).
#[allow(clippy::too_many_arguments)]
fn throw_piece(w: &mut MobyWorld, src: usize, ctx: &mut Ctx, class: i16, pos: V3, rot: V3, vel: V3, gravity: f32, sphere: [f32; 4]) -> Option<usize> {
    let c = w.class(class)?.clone();
    let own = w.class(w.mobys[src].o_class).map_or(1.0, |c| c.scale);
    let k = w.create(class);
    let life = ctx.game.rng.randf(60.0, 120.0) as i32;
    let spin = ctx.game.rng.rand_vec(f32::from_bits(0x3d56_7752), f32::from_bits(0x3dd6_7752));
    let (light, amb, scale_m) = { let m = &w.mobys[src]; (m.light, m.ambient, m.scale) };
    let x = &mut w.mobys[k];
    x.drawn = 1;
    x.mode |= mode::KEEP_ROWS;
    x.scale = c.scale * (scale_m / own);
    x.state = 1;
    (x.light, x.ambient) = (light, amb);
    x.pos = pos;
    x.rot = rot;
    x.rows = crate::moby::attach::rows_of(rot);
    x.update_dist = 0xff;
    x.draw_dist = 0x80;
    x.has_collision = false;
    x.set_pv_v(0, vel);
    x.set_pv_v(0x10, spin);
    x.set_pv_v(0x20, [sphere[0], sphere[1], sphere[2]]);
    x.set_pv_f(0x2c, sphere[3]);
    x.set_pv_i(0x30, life);
    x.set_pv_f(0x38, gravity);
    w.rebuild(k, ctx.host);
    w.changed = true;
    Some(k)
}

/// 0x09d8 (`0x3d3f38` → `0x3d3f80` unless carried, `+0x3c`): a thrown shield tumbles — its rows turned by its spin
/// about its centre, moving its velocity, falling `+0x38`; on touching the world (a sphere of `+0x2c` at its centre)
/// it bounces off at half the way in (its speed kept) and spins about the new way. Its life over (1), it fades over
/// 10 ticks (3) and goes.
pub fn update_shield(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pv_i(0x3c) != 0 { return; }
    match w.mobys[id].state {
        1 | 3 => {
            let mut t = w.mobys[id].pv_i(0x30);
            let over = crate::moby::countdown(&mut t);
            w.mobys[id].set_pv_i(0x30, t);
            if w.mobys[id].state == 1 && over {
                let m = &mut w.mobys[id];
                m.state = 3;
                m.set_pv_i(0x30, 10);
                m.mode |= 0x2000;
                return;
            }
            if w.mobys[id].state == 3 {
                if over {
                    w.delete(id);
                    return;
                }
                w.mobys[id].alpha = (t as f32 * 12.8) as u8;
            }
        }
        _ => {
            w.delete(id);
            return;
        }
    }
    let m = &mut w.mobys[id];
    let spin = m.pv_v(0x10);
    let centre = m.pv_v(0x20);
    let to = |c: V3, r: &[V3; 3]| [0, 1, 2].map(|k| c[0] * r[0][k] + c[1] * r[1][k] + c[2] * r[2][k]);
    let before = to(centre, &m.rows);
    m.rows = crate::moby::attach::turn(&crate::moby::attach::rows_of(spin), &m.rows);
    m.rot = crate::moby::attach::angles_of(&m.rows);
    let after = to(centre, &m.rows);
    m.pos = sub(m.pos, sub(after, before));
    let mut v = m.pv_v(0);
    m.pos = add(m.pos, v);
    v[2] -= m.pv_f(0x38);
    m.set_pv_v(0, v);
    let c = add(to(centre, &m.rows), m.pos);
    let r = m.pv_f(0x2c);
    let hit = ctx.coll.and_then(|col| crate::collision::coll_sphere_m(col, ctx.scene, c, r, crate::collision::QueryFlags(0), Some(id)));
    let Some(h) = hit else { return };
    let n = set_len(h.normal, 1.0);
    let speed = crate::hero::physics::len(v);
    let d = crate::hero::physics::dot(v, n);
    if d < 0.0 {
        // Off the face: the normal part turned back, half of the way in kept against it, the speed kept.
        let r = sub(v, scale(n, 2.0 * d));
        let v = set_len(add(r, scale(n, d * 0.5)), speed);
        let m = &mut w.mobys[id];
        m.set_pv_v(0, v);
        let axis = set_len(crate::hero::physics::cross(n, v), 1.0);
        let a = crate::hero::physics::len(spin);
        let q = crate::quat::axis_angle(a, axis);
        let rows = crate::quat::rows(q);
        m.set_pv_v(0x10, crate::moby::attach::angles_of(&rows));
    }
}

/// `0x3d37a0(m, delay)`: out of the spawner, standing (3, sub 1) for `delay` ticks, with a new shield.
fn start_09ca(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, delay: i16) {
    let coll = w.class(C9CA).is_some_and(|c| c.has_collision);
    let m = &mut w.mobys[id];
    m.set_pv_s(TURN_9CA, 0);
    let life = m.pv_s(0x24) as f32;
    m.set_pv_f(0x20, life);
    ai::senses(m, f32::from_bits(0x3bb6_0b61), f32::from_bits(0x3dd3_a06e), f32::from_bits(0x3b64_c38a), f32::from_bits(0x3ed6_7752));
    m.set_state(3, 1);
    set_seq(w, id, ctx, 0, 10);
    let m = &mut w.mobys[id];
    m.has_collision = coll;
    m.set_pv_b(0x6a, 0);
    m.set_pv_b(0x2e, 1);
    m.drops = 0;
    m.drawn = 1;
    m.mode = (m.mode & 0xffb8) | mode::TARGET;
    make_shield(w, id, ctx);
    let m = &mut w.mobys[id];
    m.set_pv_s(0x2fc, delay);
    m.set_pv_s(0x88, 0);
    if m.pv_s(0x50) != 0 {
        m.set_pv_s(0x50, 0);
        m.ambient = [m.pv_b(0x54), m.pv_b(0x55), m.pv_b(0x56)];
        m.set_pv_s(0x52, 1);
    }
    let amb = w.mobys[id].ambient;
    if let Some(s) = shield(w, id) { w.mobys[s].ambient = amb; }
}

/// `0x3d38e8`: back to the pool — its shield dropped, its turn given back; hidden (state 1).
pub fn pool_09ca(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    drop_shield(w, id, ctx);
    if w.mobys[id].pv_s(TURN_9CA) == 1 {
        if let Some(ms) = master(w, id) { give_back(w, ms, C9CA); }
    }
    w.mobys[id].set_pv_s(TURN_9CA, 0);
    w.mobys[id].set_pv_i(TARGET_9CA, 0);
    w.mobys[id].set_pv_b(0x291, 0);
    set_seq(w, id, ctx, 0, 3);
    let m = &mut w.mobys[id];
    m.set_state(1, 0);
    m.drawn = 0;
    m.has_collision = false;
    m.mode = (m.mode & !mode::TARGET) | 0x47;
    w.changed = true;
}

/// `0x3d39e0(m, no_bolts, at_once)`: dead — its shield dropped, its head knocked off (once: its piece 5 thrown), no
/// collision, out of the target list; its bolts unless `no_bolts`; back to the pool now or after the fade.
fn die_09ca(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, no_bolts: bool, at_once: bool) {
    drop_shield(w, id, ctx);
    head_off(w, id, ctx);
    let m = &mut w.mobys[id];
    m.has_collision = false;
    m.mode &= !mode::TARGET;
    if !no_bolts { crate::moby::bolts::drop(w, id, 0, -1, ctx); }
    if at_once { pool_09ca(w, id, ctx); } else { super::death_fade::start(w, id, ctx, 0.0, Some(pool_09ca)); }
}

/// Its head knocked off (`+0x291` once): its piece 5 thrown at `randvec(1/60, 1/20)` (`0x327498`). Not ported: the
/// head joint shrunk to nothing (`0x316598`, a scale manipulator on joint 3).
fn head_off(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pv_b(0x291) != 0 { return; }
    w.mobys[id].set_pv_b(0x291, 1);
    w.mobys[id].set_pv_b(0x292, 1);
    let v = ctx.game.rng.rand_vec(f32::from_bits(0x3c88_8889), f32::from_bits(0x3d4c_ccce));
    crate::moby::shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 6.0), 0x20, v, 0xff);
}

/// `0x3d34f8(p, n)`: `n` pairs of sparks off the shield (two twinkles each: pink `0x7f007f40` and a smaller blue one,
/// `randf($gp+0x1134, +0x1138)`, 0.3 toward the camera, along a way turned ±60° about up).
fn shield_sparks(ctx: &mut Ctx, p: V3, n: i32) {
    let cam = ctx.host.camera_pos();
    let toward = set_len(sub(cam, p), 0.3);
    let way = { let d = sub(cam, p); [d[0], d[1], 0.0] };
    for _ in 0..n {
        let g = &mut ctx.game.rng;
        let s = g.randf(SPARK[0], SPARK[1]);
        let spin = if g.randi(2) == 0 { -1 } else { 1 };
        let a = g.randf(-1.047_197_6, 1.047_197_6);
        let mut v = set_len(rotate(way, a, [0.0, 0.0, 1.0]), g.randf(SPARK[2], SPARK[3]) / 60.0);
        v[2] = g.randf(SPARK[4], SPARK[5]) / 60.0;
        let mut at = add(p, toward);
        for k in 0..3 { at[k] += g.randf_signed(0.0, 0.15); }
        let Some(sys) = ctx.parts.as_deref_mut() else { return };
        crate::particles::twinkle(sys, &mut ctx.game.rng, s * 0.1, s, SPARK[6], at, 0x3c, 0x7f00_7f40, 0, spin, v, 0);
        crate::particles::twinkle(sys, &mut ctx.game.rng, s * 0.07, s * 0.7, SPARK[6], at, 0x3c, 0x047f_047f, 1, -spin, v, 0);
    }
}

/// `$gp+0x1134`, `+0x1138` (size), `+0x1120`, `+0x1124` (speed), `+0x1128`, `+0x112c` (rise), `+0x1130` (fall).
const SPARK: [f32; 7] = [0.3, 0.6, 3.0, 6.0, 1.0, 4.0, 0.002];

/// `0x3d2648`: set-up, its shadow, the hits (the shield takes those it can: the flag, sparks every 60 ticks), dying
/// (0x15 / sub 7), its turn and target.
fn senses_09ca(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 0 {
        let a = ctx.game.rng.randf(10.0, 30.0).to_radians();
        let neg = ctx.game.rng.randi(2) != 0;
        let mirror = ctx.game.rng.randi(2) == 0;
        let m = &mut w.mobys[id];
        m.set_pv_i(0x18, 0x100);
        m.set_pv_b(0x48, 15);
        m.set_pv_b(0x4a, 16);
        m.set_pv_s(0x2f4, 0);
        m.set_pv_f(0x2e8, if neg { -a } else { a });
        m.set_pv_b(0x60, 0x14);
        m.set_pv_i(0x98, 7);
        m.set_pv_f(0x84, 1.3);
        m.set_pv_f(0x8c, 0.4);
        m.set_pv_b(0x1e6, 5);
        m.set_pv_b(0x1e8, 2);
        m.set_pv_f(0x1d4, 0.5);
        m.set_pv_f(0x100, 1.0);
        m.set_pv_f(0x104, 0.2);
        m.set_pv_f(0x10c, 0.3);
        m.set_pv_f(0x110, f32::from_bits(0x3fc9_0fdc));
        m.set_pv_f(0x130, f32::from_bits(0x3f06_0a93));
        m.set_pv_f(0x134, 0.01);
        m.set_pv_f(0x12c, f32::from_bits(0x3bb6_0b61));
        m.set_pv_b(0x28, 3);
        m.set_pv_b(0x29, 1);
        m.set_pv_f(0x30, 0.75);
        m.set_pv_f(0x80, 1.3);
        m.set_pv_b(0x69, 2);
        m.set_pv_b(0x1e9, 3);
        m.set_pv_f(0x108, 0.3);
        if mirror { m.mode |= mode::MIRROR; }
        m.set_pv_i(SHIELD_LINK, 0);
        pool_09ca(w, id, ctx);
    }
    if w.mobys[id].drawn == 0 { return; }
    crate::shadow_volumes::probe_near(w, id, ctx, 24.0, 0x16);
    if w.mobys[id].drawn == 0 || w.mobys[id].state < 3 { return; }
    w.mobys[id].set_pv_f(0xfc, 1.0);
    let hit = w.hit_message(id).filter(|h| h.tmpl.flags & 0x1_0000 != 0);
    let blocked = hit.is_some() && shield(w, id).is_some() && {
        let m = &w.mobys[id];
        m.state == 0x13 || m.state == 4 || m.sub_state == 4 || (0x10..=0x12).contains(&m.state)
    };
    if !blocked {
        ai::take_damage(w, id, ctx, false);
    } else {
        w.mobys[id].set_pv_b(0x60, 0x13);
        ai::take_damage(w, id, ctx, true);
        w.mobys[id].set_pv_b(0x60, 0x14);
        if let Some(s) = shield(w, id) {
            if w.mobys[id].pv_s(0x2f4) == 0 && w.mobys[id].drawn != 0 {
                let p = w.mobys[s].pos;
                shield_sparks(ctx, p, 2);
                w.mobys[id].set_pv_s(0x2f4, 0x3c);
            }
        }
    }
    if w.mobys[id].pv_f(0x20) <= 0.0 && w.mobys[id].state != 0x15 {
        w.mobys[id].set_state(0x15, 7);
        set_seq(w, id, ctx, 0x10, 0);
    }
    if 1 < w.mobys[id].state && w.mobys[id].pv_b(0x2e) == 2 {
        die_09ca(w, id, ctx, true, true);
        return;
    }
    turn_09ca(w, id, ctx);
}

/// `0x3d2ef0`: its turn — the master's (kept once had), else a sparring partner: the nearest free gladiator of its
/// group (both then `+0x2f6` 2, each the other's target), kept while the other still has it; its target from
/// `0x32ad38(100, 100, m, +0x2f0)` with the master's turn.
fn turn_09ca(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state <= 3 {
        w.mobys[id].set_pv_i(TARGET_9CA, 0);
        w.mobys[id].set_pv_s(TURN_9CA, 0);
        return;
    }
    let has = w.mobys[id].pv_s(TURN_9CA) == 1 || {
        let got = master(w, id).is_some_and(|ms| claim(w, ms, C9CA));
        if got { w.mobys[id].set_pv_s(TURN_9CA, 1); }
        got
    };
    if has {
        let area = w.mobys[id].pv_i(0x2f0);
        let found = ai::find_target(w, id, ctx, 100.0, 100.0, area, None, false);
        let m = &mut w.mobys[id];
        m.set_pv_s(0x2fa, found as i16);
        let t = m.pv_i(0x200);
        m.set_pv_i(TARGET_9CA, t);
        return;
    }
    if w.mobys[id].pv_s(TURN_9CA) == 2 {
        if let Some(o) = ai::link(&w.mobys[id], TARGET_9CA) {
            let x = &w.mobys[o];
            let keep = x.state < 0x80 && x.sub_state > 1 && x.pv_s(TURN_9CA) == 2 && ai::link(x, TARGET_9CA) == Some(id);
            if keep {
                w.mobys[id].set_pv_s(0x2fa, 2);
                w.mobys[o].set_pv_s(0x2fa, 2);
                return;
            }
            w.mobys[id].set_pv_s(TURN_9CA, 0);
            w.mobys[id].set_pv_i(TARGET_9CA, 0);
            return;
        }
    }
    let g = w.mobys[id].group;
    let members = usize::try_from(g).ok().and_then(|g| w.groups.get(g)).cloned().unwrap_or_default();
    let mut best = None;
    let mut near = 1000.0;
    for o in members {
        if o == id { continue; }
        let x = &w.mobys[o];
        if x.state >= 0x80 || x.state < 3 || x.o_class != C9CA || x.pv_s(TURN_9CA) != 0 { continue; }
        let d = dist2(w.mobys[id].pos, x.pos);
        if d < near { near = d; best = Some(o); }
    }
    match best {
        Some(o) => {
            for (a, b) in [(id, o), (o, id)] {
                let m = &mut w.mobys[a];
                ai::set_link(m, TARGET_9CA, Some(b));
                m.set_pv_s(0x2fa, 2);
                m.set_pv_s(TURN_9CA, 2);
            }
        }
        None => w.mobys[id].set_pv_s(TURN_9CA, 0),
    }
}

/// `0x3d2a50`: back to its guard — with a shield 4 / sub 2 (sequence 0), without 5 / sub 2 (sequence 1).
fn guard_09ca(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if shield(w, id).is_none() {
        w.mobys[id].set_state(5, 2);
        set_seq(w, id, ctx, 1, 10);
    } else {
        w.mobys[id].set_state(4, 2);
        set_seq(w, id, ctx, 0, 10);
    }
}

/// `0x3d2ae0`: to the fight (6, sub 3, sequence 2).
fn fight_09ca(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    ai::senses(&mut w.mobys[id], f32::from_bits(0x3bb6_0b61), f32::from_bits(0x3d01_b4e8), f32::from_bits(0x3be4_c38a), f32::from_bits(0x3f56_7752));
    w.mobys[id].set_state(6, 3);
    set_seq(w, id, ctx, 2, 10);
}

/// `0x3d2b68`: the stand-off (10, sub 4, sequence 6).
fn standoff_09ca(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    ai::senses(&mut w.mobys[id], f32::from_bits(0x3bb6_0b61), f32::from_bits(0x3d88_8889), f32::from_bits(0x3b64_c38a), f32::from_bits(0x3ed6_7752));
    w.mobys[id].set_state(10, 4);
    set_seq(w, id, ctx, 6, 5);
}

/// `0x3d2e30`: a gladiator of its group sparring with the master's turn in the fight's turn (sub 6).
fn sparring(w: &MobyWorld, id: usize) -> bool {
    let g = w.mobys[id].group;
    usize::try_from(g).ok().and_then(|g| w.groups.get(g)).is_some_and(|v| v.iter().any(|&o| {
        let x = &w.mobys[o];
        x.state < 0x80 && x.o_class == C9CA && x.sub_state == 6 && x.pv_s(TURN_9CA) == 1
    }))
}

/// `0x3d2bf0`: within 4 of its target and facing it within 15°, it attacks: one time in five the taunt (0xc, sub
/// 5, sequence 7), else the swing (0xf with the sword alone, 0x12 with the shield; sub 6, from frame 8).
fn attack_09ca(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    if sparring(w, id) { return false; }
    let m = &w.mobys[id];
    let (d, yaw) = match ai::link(m, TARGET_9CA).and_then(|t| w.mobys.get(t)) {
        Some(t) => (dist2(m.pos, t.pos), crate::hero::physics::atan2(t.pos[0] - m.pos[0], t.pos[1] - m.pos[1])),
        None => (100.0, 1.0),
    };
    if 4.0 <= d || 0.261_799_42 <= crate::hero::physics::rot_dist(yaw, m.rot[2]) { return false; }
    if ctx.game.rng.randi(5) == 0 {
        w.mobys[id].set_state(0xc, 5);
        set_seq(w, id, ctx, 7, 10);
    } else if shield(w, id).is_none() {
        w.mobys[id].set_state(0xf, 6);
        if seq(w, id, ctx) != 10 { ctx.host.set_anim(&w.mobys[id], 10, 8, 0xf); }
    } else {
        w.mobys[id].set_state(0x12, 6);
        if seq(w, id, ctx) != 0xd { ctx.host.set_anim(&w.mobys[id], 0xd, 8, 0xf); }
    }
    true
}

/// `0x3d2d78(m, p)`: how far `p` is to its side (its second row on the way to `p`).
fn side(w: &MobyWorld, id: usize, p: V3) -> f32 {
    let m = &w.mobys[id];
    let d = set_len(sub(p, m.pos), 1.0);
    crate::hero::physics::dot(m.rows[1], d)
}

/// `0x3d3190`: the swing's blow, frames 19..31: a sphere on the blade (joint 0 → 1) at the point nearest its target
/// (0.15..0.5 wide by how far along), the sword's own at joint 1; its sound 5 on the first that lands.
fn swing_09ca(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let ft = ctx.host.frame_time(&w.mobys[id]);
    if !(19.0..=31.0).contains(&ft) { return; }
    let (Some(a), Some(b)) = (ctx.host.joint(&w.mobys[id], 0), ctx.host.joint(&w.mobys[id], 1)) else { return };
    let t = ai::link(&w.mobys[id], TARGET_9CA).and_then(|t| w.mobys.get(t)).map_or(ctx.hero.pos, |t| t.pos);
    let ab = sub(a, b);
    let l2 = crate::hero::physics::dot(ab, ab);
    let k = if 0.0 < l2 { (crate::hero::physics::dot(sub(t, b), ab) / l2).clamp(0.0, 1.0) } else { 0.0 };
    let p = add(b, scale(ab, k));
    let f = (crate::hero::physics::len(sub(p, b)) / crate::hero::physics::len(ab).max(1e-6)).clamp(0.0, 1.0);
    let dmg = w.mobys[id].pv_b(0x34) as f32;
    let n = ai::attack_sphere(w, id, ctx, f * 0.35 + 0.15, dmg, 1.0, p, 0x1_0001, 0, 1, 0);
    if n != 0 && w.mobys[id].pv_s(0x2f8) == 0 {
        ctx.host.play_sound(&w.mobys[id], 5, 0);
        w.mobys[id].set_pv_s(0x2f8, 1);
    }
    ai::attack_sphere(w, id, ctx, 0.5, dmg, 1.0, b, 0x1_0001, 0, 1, 0);
}

/// `0x3d3c18`: [`round_plates`] for the gladiator (its plates' group `+0x2ec`, `$gp+0x113c..+0x1144`: 4.8, 85°, 5°).
fn round_plates_09ca(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool { round_plates_with(w, id, ctx, 0x2ec, 0x134) }

/// 0x09ca (`0x3d1960`): a gladiator with a sword and a shield (0x09d8 at its joint 2) on the framework (react `+0x50`,
/// movement `+0x100`, health `+0x20`). Out, it walks in to the floor's cuboid (3), then guards (4 / 5, sub 2): it
/// wanders between spots 6..10 from the middle, now and then idling. With a turn (the master's, or sparring with
/// another gladiator) it fights (6 → 7 closing in at its yaw offset round raised plates; 10 the stand-off, side-
/// stepping (8 / 9) to face its target, stepping in to 4; 0xb guarding close): within 4 and facing it, the taunt
/// (0xc) or a swing (0xf alone, 0x12 with its shield; [`swing_09ca`]), then the taunt if it hurt Ratchet. Hits the
/// shield takes while it guards or swings do less (the flag) and spark; at 1 health its shield falls, at 0 its head;
/// knocked (sub 7), dead it fades back to the pool with its bolts.
pub fn update_09ca(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 1 { return; }
    senses_09ca(w, id, ctx);
    if w.mobys[id].state == 1 { return; }
    if w.mobys[id].pv_s(TURN_9CA) == 0 {
        w.mobys[id].set_pv_i(TARGET_9CA, 0);
        if !matches!(w.mobys[id].sub_state, 1 | 2 | 7) { guard_09ca(w, id, ctx); }
    }
    let tgt = ai::link(&w.mobys[id], TARGET_9CA).and_then(|t| w.mobys.get(t)).map_or(ctx.hero.pos, |t| t.pos);
    let pos = w.mobys[id].pos;
    let yaw_t = crate::hero::physics::atan2(tgt[0] - pos[0], tgt[1] - pos[1]);
    let mut d = dist2(pos, tgt);
    let mut flags = 0;
    match w.mobys[id].sub_state {
        2 => {
            if w.mobys[id].once & 2 == 0 {
                w.mobys[id].once |= 2;
                set_seq(w, id, ctx, 3, 10);
                w.mobys[id].timer = 0;
                if let Some(p) = master(w, id).and_then(|ms| middle(w, ms, ctx)) { pick_spot(w, id, ctx, p, SPOT_9CA); }
                w.mobys[id].state_ticks = 10000;
            }
            if w.mobys[id].timer == 1 && 0x12d <= w.mobys[id].state_ticks as u16 {
                match ctx.game.rng.randi(2) {
                    0 => set_seq(w, id, ctx, 1, 10),
                    _ => set_seq(w, id, ctx, 0, 10),
                }
                w.mobys[id].state_ticks = ctx.game.rng.randi(100) as i16;
            }
            if w.mobys[id].pv_s(TURN_9CA) != 0 { fight_09ca(w, id, ctx); }
            let spot = w.mobys[id].pv_v(SPOT_9CA);
            if w.mobys[id].timer == 0 && dist2(w.mobys[id].pos, spot) < 0.5 {
                w.mobys[id].timer = 1;
            } else {
                ai::move_step(w, id, ctx, yaw_t, spot);
            }
        }
        3 => {
            if !round_plates_09ca(w, id, ctx) {
                match ctx.host.anim(&w.mobys[id]).map_or(0xff, |a| a.seq_a) {
                    2 => go(w, id, ctx, tgt),
                    3 => {
                        let off = w.mobys[id].pv_f(0x2e8);
                        ai::move_toward(w, id, ctx, off, tgt);
                    }
                    _ => {}
                }
            }
        }
        4 => {
            if 14.0 < d {
                fight_09ca(w, id, ctx);
            } else if !attack_09ca(w, id, ctx) {
                let facing = crate::hero::physics::rot_dist(w.mobys[id].rot[2], yaw_t) < 0.261_799_42;
                if d < 4.0 && facing {
                    w.mobys[id].set_state(0xb, -1);
                } else if w.mobys[id].state == 0xb && 5.0 < d {
                    w.mobys[id].set_state(10, -1);
                }
            }
        }
        5 => {
            if wrapped(w, id, ctx) { standoff_09ca(w, id, ctx); }
        }
        6 => {
            let p = w.mobys[id].pos;
            ai::move_step(w, id, ctx, yaw_t, p);
        }
        7 => {
            flags = ai::flight(w, id, ctx);
            if w.mobys[id].pv_f(0x20) <= 1.0 { drop_shield(w, id, ctx); }
            if w.mobys[id].pv_f(0x20) <= 0.0 { head_off(w, id, ctx); }
            if flags & 1 != 0 || (w.mobys[id].pv_f(0x20) <= 0.0 && 0x3c < w.mobys[id].state_ticks as u16) {
                w.mobys[id].has_collision = false;
                crate::moby::bolts::drop(w, id, 0, -1, ctx);
                super::death_fade::start(w, id, ctx, 0.0, Some(pool_09ca));
                tail_09ca(w, id, ctx);
                return;
            }
        }
        _ => {}
    }
    d = dist2(w.mobys[id].pos, tgt);
    match w.mobys[id].state {
        3 => {
            if w.mobys[id].first() {
                w.mobys[id].timer = 0;
                set_seq(w, id, ctx, 0, 0);
                w.mobys[id].set_pv_i(0x13c, -1);
            }
            let m = &w.mobys[id];
            if m.timer == 0 && (m.pv_s(0x2fc) as i32) < m.state_ticks as u16 as i32 {
                set_seq(w, id, ctx, 3, 10);
                w.mobys[id].timer = 1;
            } else if w.mobys[id].timer == 1 {
                if let Some(ms) = master(w, id) {
                    let floor_box = w.mobys[ms].pv_i(0x84);
                    if ctx.level.in_cylinder(floor_box, w.mobys[id].pos) {
                        w.mobys[id].set_state(4, 2);
                        w.mobys[id].set_pv_i(0x13c, -1);
                    }
                    if let Some(p) = middle(w, ms, ctx) { go(w, id, ctx, p); }
                }
            }
        }
        6 => {
            if d <= 14.0 {
                if d < 4.0 || 0x1e < w.mobys[id].pv_i(0x180) { standoff_09ca(w, id, ctx); }
            } else {
                ai::senses(&mut w.mobys[id], f32::from_bits(0x3bb6_0b61), f32::from_bits(0x3dd3_a06e), f32::from_bits(0x3b64_c38a), f32::from_bits(0x3ed6_7752));
                w.mobys[id].set_state(7, 3);
                set_seq(w, id, ctx, 3, 0);
            }
        }
        7 => {
            if d < 4.0 || 0x1c2 < w.mobys[id].state_ticks as u16 || 0x1e < w.mobys[id].pv_i(0x180) { fight_09ca(w, id, ctx); }
        }
        s @ (8 | 9) => {
            let x = side(w, id, tgt);
            let done = if s == 8 { -0.01 < x } else { x < 0.01 };
            if done {
                w.mobys[id].set_state(10, -1);
                set_seq(w, id, ctx, 6, 10);
                let (r, p) = (w.mobys[id].rot[2], w.mobys[id].pos);
                ai::move_step(w, id, ctx, r, p);
            } else if 5.0 < ctx.host.frame_time(&w.mobys[id]) {
                let keep = w.mobys[id].pv_f(0x130);
                w.mobys[id].set_pv_f(0x130, f32::from_bits(0x3c8e_fa36));
                let p = w.mobys[id].pos;
                ai::move_step(w, id, ctx, yaw_t, p);
                w.mobys[id].set_pv_f(0x130, keep);
            }
        }
        10 => {
            let ft = ctx.host.frame_time(&w.mobys[id]);
            let x = side(w, id, tgt);
            let mut turn = true;
            if 0.01 < x {
                w.mobys[id].set_state(9, -1);
                set_seq(w, id, ctx, 5, 10);
            } else if x < -0.01 {
                w.mobys[id].set_state(8, -1);
                set_seq(w, id, ctx, 4, 10);
            } else if 4.0 <= d {
                if ft < 2.0 {
                    ai::senses(&mut w.mobys[id], f32::from_bits(0x3bb6_0b61), f32::from_bits(0x3d08_8889), f32::from_bits(0x3b64_c38a), f32::from_bits(0x3ed6_7752));
                    go(w, id, ctx, tgt);
                    turn = false;
                } else if !((10.0 < ft && ft < 17.0) || 20.0 <= ft) {
                    go(w, id, ctx, tgt);
                    turn = false;
                }
            } else {
                w.mobys[id].set_state(0xb, -1);
            }
            if turn {
                let p = w.mobys[id].pos;
                ai::move_step(w, id, ctx, yaw_t, p);
            }
        }
        s @ (0xb | 0x13 | 0x14) => {
            if s == 0xb {
                if w.mobys[id].first() { set_seq(w, id, ctx, 1, 10); }
                let p = w.mobys[id].pos;
                ai::move_step(w, id, ctx, yaw_t, p);
            }
            if flags & 2 != 0 { guard_09ca(w, id, ctx); }
        }
        s @ (0xd | 0x10) => {
            if wrapped(w, id, ctx) {
                ctx.host.set_anim_speed(&w.mobys[id], 0.5);
                let (next, q) = if s == 0xd { (0xe, 9) } else { (0x11, 0xc) };
                w.mobys[id].set_state(next, -1);
                set_seq(w, id, ctx, q, 0);
            }
        }
        s @ (0xe | 0x11) => {
            if d < 4.0 {
                w.maktar.combo = 0;
                ctx.host.set_anim_speed(&w.mobys[id], 1.0);
                let (next, q) = if s == 0xe { (0xf, 10) } else { (0x12, 0xd) };
                w.mobys[id].set_state(next, -1);
                set_seq(w, id, ctx, q, 10);
            } else if d <= 4.0 {
                if wrapped(w, id, ctx) {
                    let c = w.maktar.combo;
                    w.maktar.combo = c + 1;
                    if (s == 0xe && 3 <= c) || (s == 0x11 && 2 < c) {
                        w.maktar.combo = 0;
                        ctx.host.set_anim_speed(&w.mobys[id], 1.0);
                        guard_09ca(w, id, ctx);
                    }
                }
            } else {
                ctx.host.set_anim_speed(&w.mobys[id], 1.0);
                guard_09ca(w, id, ctx);
            }
        }
        0xf | 0x12 => {
            if wrapped(w, id, ctx) {
                w.mobys[id].set_pv_s(0x2f8, 0);
                if ctx.hero.damage.hit_by == Some(id) {
                    w.mobys[id].set_state(0xc, 5);
                    set_seq(w, id, ctx, 7, 10);
                } else {
                    guard_09ca(w, id, ctx);
                }
            }
            swing_09ca(w, id, ctx);
        }
        _ => {}
    }
    tail_09ca(w, id, ctx);
}

/// `0x3d2988`: the shield's wait, the ticks, its shield at its joint 2 (`0x3297f8`), its blow on what it touches (1.5).
fn tail_09ca(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    let mut t = m.pv_s(0x2f4) as i32;
    crate::moby::countdown(&mut t);
    m.set_pv_s(0x2f4, t as i16);
    if m.state_ticks != -1 { m.state_ticks = m.state_ticks.wrapping_add(1); }
    if let Some(s) = shield(w, id) {
        if let Some((r, t)) = ctx.host.joint_matrix(&w.mobys[id], 2) {
            ctx.host.advance(&w.mobys[s]);
            let hidden = w.mobys[id].mode & 1 != 0;
            let pm = &mut w.mobys[s];
            pm.pos = t;
            pm.rows = r.map(|row| set_len(row, 1.0));
            pm.rot = crate::moby::attach::angles_of(&pm.rows);
            pm.mode = if hidden { pm.mode | 0x41 } else { pm.mode & 0xffbe };
            pm.mode |= mode::NO_MATRIX | mode::KEEP_ROWS;
            pm.mode &= !mode::NO_UPDATE;
            w.rebuild(s, ctx.host);
        }
    }
    if w.mobys[id].state < 0x80 && w.mobys[id].state != 1 {
        let pos = w.mobys[id].pos;
        ai::attack_sphere(w, id, ctx, 1.5, 1.0, 1.0, pos, 0x400_0000, 0, 1, 0);
    }
}

// ---------------------------------------------------------------------------------------------------------------
// 0x0b22

pub const B22: i16 = 0x0b22;
/// Its target (`+0x4b0`, from the target block's `+0x210`), turn (`+0x4ea`), spot (`+0x4a0`), parts (`+0x4c0`).
const TARGET_B22: usize = 0x4b0;
const TURN_B22: usize = 0x4ea;
const SPOT_B22: usize = 0x4a0;
const PARTS_B22: usize = 0x4c0;
const PART_JOINTS_B22: [usize; 2] = [0, 1];

/// `0x3f2d20(m, delay)`: out of the spawner, standing (3) for `delay` ticks, its parts shown.
fn start_0b22(w: &mut MobyWorld, id: usize, _ctx: &mut Ctx, delay: i16) {
    let coll = w.class(B22).is_some_and(|c| c.has_collision);
    let pcoll = w.class(0x0aa2).is_some_and(|c| c.has_collision);
    let m = &mut w.mobys[id];
    m.set_pv_s(0x98, 0);
    let life = m.pv_s(0x24) as f32;
    m.set_pv_f(0x20, life);
    ai::senses(m, f32::from_bits(0x3b91_a2b4), f32::from_bits(0x3e08_8889), f32::from_bits(0x3a98_825c), f32::from_bits(0x3dd6_7752));
    m.set_state(3, 0);
    m.has_collision = coll;
    m.set_pv_b(0x7a, 0);
    m.drawn = 1;
    m.drops = 0;
    m.mode = (m.mode & 0xffb8) | mode::TARGET;
    for p in super::enemies::parts(w, id, PARTS_B22).into_iter().flatten() {
        let x = &mut w.mobys[p];
        x.mode &= !1;
        x.drawn = 1;
        x.has_collision = pcoll;
    }
    let m = &mut w.mobys[id];
    m.set_pv_s(0x4f0, delay);
    m.set_pv_s(TURN_B22, 0);
    if m.pv_s(0x60) != 0 {
        m.set_pv_s(0x60, 0);
        m.ambient = [m.pv_b(0x64), m.pv_b(0x65), m.pv_b(0x66)];
        m.set_pv_s(0x62, 1);
    }
    w.changed = true;
}

/// `0x3f2e58`: back to the pool — its turn given back; in view a blast, 0x0c3e's sound 0 and its pieces thrown; its
/// parts hidden; hidden (state 1).
fn pool_0b22(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pv_s(TURN_B22) == 1 {
        if let Some(ms) = master(w, id) { give_back(w, ms, B22); }
    }
    let m = &mut w.mobys[id];
    m.set_pv_s(TURN_B22, 0);
    m.set_pv_i(TARGET_B22, 0);
    m.set_pv_b(0x7a, 0);
    m.set_pv_b(0x2e, 1);
    if w.mobys[id].drawn != 0 {
        let pos = w.mobys[id].pos;
        crate::moby::blast::burst(w, ctx, pos, &crate::moby::blast::Blast { size: 0.5, bursts: [0x1e, 5, 5], puffs: 5, streaks: 0x2d, light: true, shake: false });
        ctx.host.play_sound_of(&w.mobys[id], C3E, 0, 0);
        let v = add(ctx.game.rng.rand_vec(f32::from_bits(0x3d08_8889), f32::from_bits(0x3daa_aaab)), w.mobys[id].pv_v(0xe0));
        crate::moby::shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 6.0), 0x7f, v, 0xff);
    }
    for p in super::enemies::parts(w, id, PARTS_B22).into_iter().flatten() {
        let x = &mut w.mobys[p];
        x.mode |= 1;
        x.drawn = 0;
        x.has_collision = false;
    }
    set_seq(w, id, ctx, 0, 3);
    let m = &mut w.mobys[id];
    m.set_state(1, 0);
    m.drawn = 0;
    m.has_collision = false;
    m.mode = (m.mode & !mode::TARGET) | 0x47;
    w.changed = true;
}

/// `0x3f62d8(angle, m, parts, sparks, 0)`: each part at its joint, its rows turned by `angle` about z (rows · Rz);
/// with `sparks`, one tick in six a spark pair off it (`0x3f5f48`).
fn spin_parts_b22(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, angle: f32, sparks: bool) {
    let rz = crate::moby::attach::rows_of([0.0, 0.0, angle]);
    for (p, j) in super::enemies::parts(w, id, PARTS_B22).into_iter().zip(PART_JOINTS_B22) {
        let Some(p) = p else { continue };
        super::enemies::place_part(w, id, p, j, ctx);
        let r = w.mobys[p].rows;
        w.mobys[p].rows = crate::moby::attach::turn(&r, &rz);
        w.rebuild(p, ctx.host);
        if sparks && ctx.game.rng.randi(6) == 0 {
            let at = w.mobys[p].pos;
            part_sparks(ctx, at);
        }
    }
}

/// `0x3f5f48(p, 1, 0)`: two twinkles off a part (`randf(0.1, 0.25)`; pink-white and white), flung up 20°..88° at
/// `randf(0.05, 0.1)` a tick, falling 0.006.
fn part_sparks(ctx: &mut Ctx, p: V3) {
    let g = &mut ctx.game.rng;
    let s = g.randf(0.1, 0.25);
    let spin = if g.randi(2) == 0 { -1 } else { 1 };
    let yaw = g.rand_angle();
    let pitch = g.randf(f32::from_bits(0x3eb2_b8c4), f32::from_bits(0x3fc4_980a));
    let l = g.randf(0.05, 0.1);
    let v = [yaw.cos() * pitch.cos() * l, yaw.sin() * pitch.cos() * l, pitch.sin() * l];
    let at = [p[0] + g.randf_signed(0.0, 0.15), p[1] + g.randf_signed(0.0, 0.15), p[2]];
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    crate::particles::twinkle(sys, &mut ctx.game.rng, s * 0.1, s, 0.006, at, 0x1e, 0x7f20_7f7f, 0, spin, v, 0);
    crate::particles::twinkle(sys, &mut ctx.game.rng, s * 0.07, s * 0.7, 0.006, at, 0x1e, 0x7f7f_7f7f, 1, -spin, v, 0);
}

/// `0x3f2808`: set-up (its two parts at joints 0 and 1), its shadow, its parts' spin (`+0x4cc`° a tick), the hits,
/// its turn and target (`0x3f2c60`).
fn senses_0b22(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 0 {
        let a = ctx.game.rng.randf(15.0, 40.0).to_radians();
        let neg = ctx.game.rng.randi(2) != 0;
        let mirror = ctx.game.rng.randi(2) == 1;
        let m = &mut w.mobys[id];
        m.set_pv_i(0x18, 0x110);
        m.set_pv_f(0x4b4, if neg { -a } else { a });
        m.set_pv_b(0x48, 15);
        m.set_pv_b(0x4a, 12);
        if mirror { m.mode |= mode::MIRROR; }
        m.set_pv_f(0x4bc, 0.0);
        m.set_pv_f(0x4cc, 8.0);
        m.set_pv_b(0x70, 10);
        m.set_pv_i(0xa8, 5);
        m.set_pv_f(0x94, 0.8);
        m.set_pv_f(0x110, 0.6);
        m.set_pv_f(0x114, 0.2);
        m.set_pv_f(0x11c, 0.4);
        m.set_pv_f(0x120, f32::from_bits(0x3fc9_0fdc));
        m.set_pv_f(0x140, f32::from_bits(0x3f06_0a93));
        m.set_pv_f(0x13c, f32::from_bits(0x3bb6_0b61));
        m.set_pv_b(0x28, 3);
        m.set_pv_b(0x2e, 1);
        m.set_pv_f(0x90, 0.8);
        m.set_pv_f(0x9c, 0.4);
        m.set_pv_f(0x118, 0.4);
        if w.class(0x0aa2).is_some() { super::enemies::make_parts_on(w, id, id, PARTS_B22, PART_JOINTS_B22, ctx); }
        w.mobys[id].set_pv_s(TURN_B22, 0);
        pool_0b22(w, id, ctx);
    }
    crate::shadow_volumes::probe_near(w, id, ctx, 24.0, 0x16);
    let m = &mut w.mobys[id];
    let a = add_rot(m.pv_f(0x4bc), m.pv_f(0x4cc).to_radians());
    m.set_pv_f(0x4bc, a);
    spin_parts_b22(w, id, ctx, a, true);
    w.register_draw(id);
    if w.mobys[id].drawn == 0 || w.mobys[id].state <= 2 { return; }
    if ai::take_damage(w, id, ctx, false) == 5 {
        w.mobys[id].set_pv_f(0xf0, 0.0);
        w.mobys[id].set_pv_f(0xf4, f32::from_bits(0x3f56_7752));
    }
    if 1 < w.mobys[id].state && w.mobys[id].pv_b(0x2e) == 2 { pool_0b22(w, id, ctx); }
    if w.mobys[id].state < 4 {
        w.mobys[id].set_pv_i(TARGET_B22, 0);
        w.mobys[id].set_pv_s(TURN_B22, 0);
        return;
    }
    if w.mobys[id].pv_s(TURN_B22) != 1 {
        if !master(w, id).is_some_and(|ms| claim(w, ms, B22)) {
            w.mobys[id].set_pv_s(TURN_B22, 0);
            return;
        }
        w.mobys[id].set_pv_s(TURN_B22, 1);
    }
    let area = w.mobys[id].pv_i(0x4e4);
    let found = ai::find_target(w, id, ctx, 100.0, 100.0, area, None, false);
    let m = &mut w.mobys[id];
    m.set_pv_s(0x4e8, found as i16);
    let t = m.pv_i(0x210);
    m.set_pv_i(TARGET_B22, t);
}

/// `0x3f30e0`: the turn about (6, sub 4), its state and sub kept to go back to (`+0x4ec` / `+0x4ee`).
fn turn_about_b22(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    let (s, b) = (m.state, m.sub_state);
    m.set_pv_s(0x4ec, s as i16);
    m.set_pv_s(0x4ee, b as i16);
    m.set_state(6, 4);
}

/// `0x3f3118`: [`round_plates`] with its group `+0x4e0` and speed word `+0x144` (`$gp+0x1f78..`: 4.8, 85°, 5°).
fn round_plates_b22(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool { round_plates_with(w, id, ctx, 0x4e0, 0x144) }

/// 0x0b22 (`0x3f1d08`): a bladed robot (Oozla's 0x0b0b's build: two spinning parts 0x0aa2 at its joints 0 and 1, the
/// same glows) on the framework (react `+0x60`, movement `+0x110`). Out, it walks in to the floor's cuboid (3), then
/// wanders between spots 6..10 from the middle (4 / sub 2, idling now and then; 6 turns about for one behind it);
/// with a turn (the master's) it closes on its target (7: running from 15 out, walking within 7; round raised plates,
/// its yaw `+0x4b4` off) and within 2.8 facing it within 22° slashes (8, sequence 0xc from frame 4: frames 19..25 its
/// joints 0 (0.35), 1 and 2 (0.15) hit for `+0x34`; its parts spin 25° a tick from frame 17), then the taunt (9) if it
/// hurt Ratchet. Knocked (sub 5), dead (or one in 30 in its death sequence) its bolts and back to the pool. It hits
/// what it touches for 1 (1.3).
///
/// Not ported: the slash's trail (`0x3f2b78` keeps its points, `0x3f2c18` → `0x2efd98` draws them).
pub fn update_0b22(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 1 { return; }
    senses_0b22(w, id, ctx);
    if w.mobys[id].state == 1 { return; }
    if w.mobys[id].pv_s(TURN_B22) == 0 {
        w.mobys[id].set_pv_i(TARGET_B22, 0);
        if w.mobys[id].sub_state == 4 {
            w.mobys[id].set_state(4, 2);
            let s = if ctx.game.rng.randi(2) == 0 { 3 } else { 2 };
            set_seq(w, id, ctx, s, 10);
        }
    }
    let tgt = ai::link(&w.mobys[id], TARGET_B22).and_then(|t| w.mobys.get(t)).map_or(ctx.hero.pos, |t| t.pos);
    let pos = w.mobys[id].pos;
    let yaw_t = crate::hero::physics::atan2(tgt[0] - pos[0], tgt[1] - pos[1]);
    let d = dist2(pos, tgt);
    match w.mobys[id].sub_state {
        2 => {
            if w.mobys[id].once & 2 == 0 {
                w.mobys[id].once |= 2;
                set_seq(w, id, ctx, 0xb, 10);
                w.mobys[id].timer = 0;
                ai::senses(&mut w.mobys[id], f32::from_bits(0x3b11_a2b4), f32::from_bits(0x3d88_8889), f32::from_bits(0x3a98_825c), f32::from_bits(0x3dd6_7752));
                if let Some(p) = master(w, id).and_then(|ms| middle(w, ms, ctx)) { pick_spot(w, id, ctx, p, SPOT_B22); }
                w.mobys[id].state_ticks = 10000;
            }
            if w.mobys[id].timer == 1 && 0x12d <= w.mobys[id].state_ticks as u16 {
                let s = [2, 3, 4, 6, 7][ctx.game.rng.randi(5) as usize % 5];
                set_seq(w, id, ctx, s, 10);
                w.mobys[id].state_ticks = ctx.game.rng.randi(100) as i16;
            }
            if w.mobys[id].pv_s(TURN_B22) != 0 {
                w.mobys[id].set_state(5, 4);
                let s = if ctx.game.rng.randi(2) == 0 { 5 } else { 1 };
                set_seq(w, id, ctx, s, 10);
            }
            let spot = w.mobys[id].pv_v(SPOT_B22);
            let p = w.mobys[id].pos;
            let yaw_s = crate::hero::physics::atan2(spot[0] - p[0], spot[1] - p[1]);
            if 2.792_527_2 < crate::hero::physics::rot_dist(w.mobys[id].rot[2], yaw_s) { turn_about_b22(w, id); }
            if w.mobys[id].timer != 0 || 0.5 <= dist2(w.mobys[id].pos, spot) {
                go(w, id, ctx, spot);
            } else {
                w.mobys[id].timer = 1;
            }
        }
        5 => {
            let f = ai::flight(w, id, ctx);
            let dying = ctx.host.anim(&w.mobys[id]).is_some_and(|a| a.seq_a == 0x10) && ctx.game.rng.randi(0x1e) == 0;
            if dying || f & 1 != 0 {
                crate::moby::bolts::drop(w, id, 0, -1, ctx);
                pool_0b22(w, id, ctx);
            } else if f & 2 != 0 {
                w.mobys[id].set_state(7, 4);
            }
        }
        _ => {}
    }
    match w.mobys[id].state {
        3 => {
            if w.mobys[id].first() {
                w.mobys[id].timer = 0;
                set_seq(w, id, ctx, 2, 0);
                w.mobys[id].set_pv_i(0x14c, -1);
            }
            let m = &w.mobys[id];
            if m.timer == 0 && (m.pv_s(0x4f0) as i32) < m.state_ticks as u16 as i32 {
                set_seq(w, id, ctx, 0xb, 10);
                w.mobys[id].timer = 1;
            } else if w.mobys[id].timer == 1 {
                if let Some(ms) = master(w, id) {
                    let floor_box = w.mobys[ms].pv_i(0x84);
                    if ctx.level.in_cylinder(floor_box, w.mobys[id].pos) {
                        w.mobys[id].set_state(4, 2);
                        w.mobys[id].set_pv_i(0x14c, -1);
                    }
                    if let Some(p) = middle(w, ms, ctx) { go(w, id, ctx, p); }
                }
            }
        }
        5 => {
            if wrapped(w, id, ctx) { w.mobys[id].set_state(7, -1); }
        }
        6 => {
            if w.mobys[id].first() {
                set_seq(w, id, ctx, 10, 10);
                ai::senses(&mut w.mobys[id], f32::from_bits(0x3b91_a2b4), f32::from_bits(0x3e08_8889), f32::from_bits(0x3be4_c38a), f32::from_bits(0x3f56_7752));
            }
            let p = w.mobys[id].pos;
            ai::move_step(w, id, ctx, yaw_t, p);
            if wrapped(w, id, ctx) {
                let (s, b) = (w.mobys[id].pv_s(0x4ec), w.mobys[id].pv_s(0x4ee));
                w.mobys[id].set_state(s as u8, b as i32);
            }
        }
        7 => {
            if w.mobys[id].first() { set_seq(w, id, ctx, 0xb, 0xf); }
            let sa = ctx.host.anim(&w.mobys[id]).map_or(0xff, |a| a.seq_a);
            if sa == 8 && 15.0 < d {
                set_seq(w, id, ctx, 0xb, 4);
                ai::senses(&mut w.mobys[id], f32::from_bits(0x3b91_a2b4), f32::from_bits(0x3e08_8889), f32::from_bits(0x3a98_825c), f32::from_bits(0x3dd6_7752));
            } else if sa == 0xb && d < 7.0 {
                set_seq(w, id, ctx, 8, 4);
                ai::senses(&mut w.mobys[id], f32::from_bits(0x3b91_a2b4), f32::from_bits(0x3d08_8889), f32::from_bits(0x3a98_825c), f32::from_bits(0x3dd6_7752));
            }
            let facing = crate::hero::physics::rot_dist(w.mobys[id].rot[2], yaw_t) < f32::from_bits(0x3ec4_9a46);
            if d < 2.8 && facing {
                w.mobys[id].set_state(8, -1);
                if seq(w, id, ctx) != 0xc { ctx.host.set_anim(&w.mobys[id], 0xc, 4, 3); }
                w.mobys[id].set_pv_i(0x4d0, 0);
            } else if d <= 2.8 {
                let p = w.mobys[id].pos;
                ai::move_step(w, id, ctx, yaw_t, p);
            } else if !round_plates_b22(w, id, ctx) {
                let off = w.mobys[id].pv_f(0x4b4);
                ai::move_toward(w, id, ctx, off, tgt);
            }
            if 2.792_527_2 < crate::hero::physics::rot_dist(w.mobys[id].rot[2], yaw_t) { turn_about_b22(w, id); }
        }
        8 => {
            if wrapped(w, id, ctx) {
                w.mobys[id].set_pv_f(0x4cc, 8.0);
                if ctx.hero.damage.hit_by == Some(id) {
                    w.mobys[id].set_state(9, -1);
                    set_seq(w, id, ctx, 0xd, 7);
                } else {
                    w.mobys[id].set_state(7, -1);
                }
            }
            let ft = ctx.host.frame_time(&w.mobys[id]);
            if ctx.host.anim(&w.mobys[id]).is_some_and(|a| a.seq_a == 0xc) {
                if (19.0..=25.0).contains(&ft) {
                    let dmg = w.mobys[id].pv_b(0x34) as f32;
                    ai::attack_joint(w, id, ctx, 0.35, dmg, 1.0, 0, 0x1_0001, 0, 1, 0);
                    ai::attack_joint(w, id, ctx, 0.15, dmg, 1.0, 1, 0x1_0001, 0, 1, 0);
                    ai::attack_joint(w, id, ctx, 0.15, dmg, 1.0, 2, 0x1_0001, 0, 1, 0);
                }
                if (17.0..=25.0).contains(&ft) { w.mobys[id].set_pv_f(0x4cc, 25.0); }
            }
            let p = w.mobys[id].pos;
            ai::move_step(w, id, ctx, yaw_t, p);
        }
        9 => {
            if wrapped(w, id, ctx) { w.mobys[id].set_state(7, -1); }
        }
        _ => {}
    }
    // `0x3f2ae8`: the ticks, its blow on what it touches.
    let m = &mut w.mobys[id];
    if m.state_ticks != -1 { m.state_ticks = m.state_ticks.wrapping_add(1); }
    let pos = w.mobys[id].pos;
    ai::attack_sphere(w, id, ctx, f32::from_bits(0x3fa6_6666), 1.0, 1.0, pos, 0x400_0000, 0, 1, 0);
}
