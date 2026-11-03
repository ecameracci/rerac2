//! Level 12's walker 0x0fb4 (`0x3e1fc8`, 53 placed; its senses `0x3e2a10`, tail `0x3e2dc8`), on the framework
//! ([`crate::moby::ai`]): health block `+0x20` (its last hitter `+0x38`, its damage byte `+0x34`), the colour flash
//! `+0x50`, reaction `+0x60` (a hurt goes to sub 3), target `+0x110` (its moby `+0x120`, its area `+0x130`), movement
//! `+0x150`. Its words: its home `+0x340`, the wander goal `+0x350`, timers `+0x360` / `+0x362`, seeing `+0x364`,
//! alarmed `+0x366` ([`alarm`]), what it faces `+0x368` (its target, else Ratchet), its range `+0x36c` (16), `+0x370`
//! (24, unread), the moby it takes `+0x374` (set by another class), its start `+0x378` (−1: at once) and `+0x13c`
//! (−1: on foot; else raised).
//!
//! The L12 engine functions with the L00 ones the port has: `0x32eaf8` = L00 `0x32a7f0` (the sub's first tick),
//! `0x331d30` = `0x32dcf0` (crate::moby::ai::senses), `0x32b628` = `0x327e38` (take_damage), `0x32f040` = `0x32ad38`
//! (find_target), `0x32c370` = `0x328b80` (flight), `0x330c28` = `0x32cd20` (move_step), `0x330a08` the move_step
//! toward a point ([`walk_to`]), `0x330da0` = `0x32ce98` (hold), `0x323c00` = `0x3201e8` (flash), `0x320968` =
//! `0x31cf68` (crate::level::LevelData::in_polygon), `0x320cc8` = `0x31d2c8` (attack_joint), `0x320ec8` = `0x31d4c8`
//! (a hit on itself, crate::moby::MobyWorld::deliver_hit_any), `0x31e7c8` = `0x31adc0` (crate::moby::bolts::drop),
//! `0x321470` = `0x31da58` (crate::shadow_volumes::probe_near), `0x324ac0` = `0x3210a8` (crate::moby::blast::
//! smoke_burst), `0x322a48` = `0x31f030` (approach_angle), `0x3152d8` = `0x311880` (the frame time), `0x3ecf98` the
//! dying fade (crate::moby::classes::death_fade).
//!
//! **The set-up** (state 0, [`senses`]): its class's `+0x46` 5, a target (mode 0x1000); the reaction (state 8, sub 3,
//! sequences 0xb / 0xb / 0xd / 0xe, keys 1/6 / 1/12, gravities 1/180 / 1/120 / 1/240, the speeds 0.5 / 0.5, 0.4, its
//! bytes 7 / 10 / 7 / 10 / 8 / 0xc), the health block's `+0x28` 3 and `+0x29` 1, `+0x48` 15, `+0x4a` 6; its range 16 and
//! `+0x370` 24 when unset; with `+0x140` its health 1; its home its place; movement (radius 0.5, height 0.25, steps
//! 0.4, slope 45°, gravity 1/180, the turn before walking 30°, stop 1). Then with a start (`+0x378`) it waits (0xe,
//! sub 5); on foot it stands (1, sub 0, sequence 0); raised: 9 sub 4, sequence 0xe, 4 up, hidden without collision.
//!
//! **Each tick** ([`senses`]): drawn within 24 of the camera its shadow (range 0x16); a hit taken alarms its group
//! ([`alarm`]); the target search (its range, 3 high) into seeing (lost when the target is outside its area). **Subs**:
//! 0 calm (limits 0.00111 / 2/60 / 0.00087 / 0.105): seeing → 5 (sub 2, sequence 2); alarmed → 3 (sub 1, sequence 3),
//! the alarm cleared, `+0x360` 480..600 when under 300. 1 alert (0.00333 / 6/60 / 0.00174 / 0.209): seeing → 6 (sub
//! 2, sequence 7); `+0x360` out → 1 (sub 0, sequence 0). 2 fighting (as 1): not seeing → 3 (sub 1, sequence 3,
//! `+0x360` 580..600). 3 the hurt's flight: last hit by a mine 0x0fab, a hit on itself of 50 from Ratchet (flags
//! 0x10000, from the mine); dead: its bolts, the fade begun and gone at once (the game deletes it after); over: seeing
//! → 6 (sub 2, sequence 7), else as 2's loss. 4 (0.00444 / 6/60 / 0.00349 / 0.209). 5 waiting for its mission's bit
//! 0x80 (hidden, no collision; then on foot or raised, shown).
//!
//! **States**: 1 standing (held): `+0x362` out → 2, the goal 3..5 from its home at a random heading, sequence 0xf. 2
//! toward the goal: at it (after 30 ticks) or after 600 → 1, `+0x362` 300..500, one of sequences 0 / 1 at random. 3
//! held. 5 facing from its target to Ratchet (in place); its sequence's end → 6 (sequence 7); at 20 ticks its group
//! alarmed. 6 toward its target: there → 7 (sequence 8). 7 turning to its target (0.0582 a tick), between frames 12
//! and 14 a 0.25 sphere at joint 2 (its damage byte, flags 1); held; its end → 6 (sequence 7). 10 facing the moby it
//! takes; on sequence 5 below its home by its fall × 10: sequence 6; 6's end → 11 (sequence 7). 11 toward it: within
//! 2 → 12 (sequence 8). 12 toward it; from frame 11 it is drawn into its hand ([`take`]), at 14 let go; the end → 13
//! (sequence 9). 13: the end → 1 (sub 0, sequence 0).
//!
//! Not ported: the joint aims (`0x327a50` on `+0x240` / `+0x2c0`; the framework's aims are not ported), the big-head
//! cheat (`0x32a7b0`, `$gp+0x1a00` 1.9), the count of taken mobys (`$gp+0x19b4`, read by nothing ported), its class's
//! `+0x46` (5).

use crate::hero::physics::{add, approach_angle, atan2, scale, set_len, sub, V3};
use crate::moby::classes::props::{seq, set_seq, wrapped};
use crate::moby::{ai, mode, Ctx, MobyWorld};

const HEALTH: usize = 0x20;
const FLASH: usize = 0x50;
const REACT: usize = 0x60;
const T_MOBY: usize = 0x120;
const AREA: usize = 0x130;
const RAISED: usize = 0x13c;
const MV: usize = 0x150;
const HOME: usize = 0x340;
const GOAL: usize = 0x350;
const T360: usize = 0x360;
const T362: usize = 0x362;
const SEEING: usize = 0x364;
const ALARMED: usize = 0x366;
const FACE: usize = 0x368;
const RANGE: usize = 0x36c;
const TAKE: usize = 0x374;
const START: usize = 0x378;
const PVARS: usize = 0x400;
/// The mine whose hits finish it.
const MINE: i16 = 0x0fab;
/// `$gp+0x19f0`: its hand in its frame.
const HAND: V3 = [f32::from_bits(0x4030_8312), f32::from_bits(0xbea9_78d5), f32::from_bits(0x3f14_bc6a)];

fn first_sub(w: &mut MobyWorld, id: usize) -> bool {
    let m = &mut w.mobys[id];
    if m.once & 2 != 0 { return false; }
    m.once |= 2;
    true
}

fn limits(w: &mut MobyWorld, id: usize, l: [u32; 4]) {
    let [a, b, c, d] = l.map(f32::from_bits);
    ai::senses(&mut w.mobys[id], a, b, c, d);
}

/// `0x331d30`'s limits per sub (the second `$gp+0x19e0` / `0x19e4` a second).
const CALM: [u32; 4] = [0x3a91_a2b4, 0x3d08_8889, 0x3a64_c38a, 0x3dd6_7752];
const ALERT: [u32; 4] = [0x3b5a_740e, 0x3dcc_cccd, 0x3ae4_c38a, 0x3e56_7752];
const SUB4: [u32; 4] = [0x3b91_a2b4, 0x3dcc_cccd, 0x3b64_c38a, 0x3e56_7752];

fn st(w: &mut MobyWorld, id: usize, state: u8, sub: i32) { w.mobys[id].set_state(state, sub); }

fn pos_of(w: &MobyWorld, o: Option<usize>, ctx: &Ctx) -> V3 { o.and_then(|o| w.mobys.get(o)).map_or(ctx.hero.pos, |m| m.pos) }

/// `0x330a08(m, p)`: the move toward `p` (its heading the one to it).
fn walk_to(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, p: V3) -> i32 {
    let q = w.mobys[id].pos;
    ai::move_step(w, id, ctx, atan2(p[0] - q[0], p[1] - q[1]), p)
}

/// `0x3e2e80`: every live 0x0fb4 of its group alarmed (`+0x366` 1).
fn alarm(w: &mut MobyWorld, id: usize) {
    let members = usize::try_from(w.mobys[id].group).ok().and_then(|g| w.groups.get(g)).cloned().unwrap_or_default();
    for k in members {
        if let Some(m) = w.mobys.get_mut(k).filter(|m| m.state < 0x80 && m.o_class == 0x0fb4) { m.set_pv_s(ALARMED, 1); }
    }
}

/// On foot from waiting or the set-up: standing (1, sub 0, sequence 0); raised: 9 sub 4, sequence 0xe, 4 up, hidden
/// without collision.
fn appear(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, blend: u8) {
    if w.mobys[id].pv_i(RAISED) == -1 {
        st(w, id, 1, 0);
        set_seq(w, id, ctx, 0, 0, blend);
    } else {
        st(w, id, 9, 4);
        set_seq(w, id, ctx, 0xe, 0, 0);
        let m = &mut w.mobys[id];
        m.has_collision = false;
        m.pos[2] += 4.0;
        m.mode = (m.mode & !mode::TARGET) | 0x41;
    }
}

/// `0x3e2a10` (module docs).
fn senses(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 0 {
        let m = &mut w.mobys[id];
        if m.pvars.len() < PVARS { m.pvars.resize(PVARS, 0); }
        m.mode |= mode::TARGET;
        m.set_pv_b(0x48, 15);
        m.set_pv_b(0x4a, 6);
        m.set_pv_b(0x29, 1);
        m.set_pv_b(0x72, 0xb);
        m.set_pv_b(0x76, 0xd);
        m.set_pv_b(0x77, 0xe);
        m.set_pv_f(0x7c, f32::from_bits(0x3e2a_aaab));
        m.set_pv_f(0x80, f32::from_bits(0x3daa_aaab));
        m.set_pv_f(0x88, f32::from_bits(0x3c08_8889));
        m.set_pv_f(0x8c, f32::from_bits(0x3b88_8889));
        for (k, v) in [(0xce, 7u8), (0xcf, 10), (0xd2, 8), (0xd3, 0xc), (0x70, 8), (0x71, 0xb), (0xcc, 7), (0xcd, 10)] { m.set_pv_b(k, v); }
        m.set_pv_i(0xa8, 3);
        m.set_pv_f(0x84, f32::from_bits(0x3bb6_0b61));
        m.set_pv_f(0x90, 0.5);
        m.set_pv_f(0x94, 0.5);
        m.set_pv_f(0x9c, f32::from_bits(0x3ecc_cccd));
        if m.pv_f(RANGE) == 0.0 { m.set_pv_f(RANGE, 16.0); }
        if m.pv_f(0x370) == 0.0 { m.set_pv_f(0x370, 24.0); }
        if m.pv_i(0x140) != 0 { m.set_pv_f(HEALTH, 1.0); }
        m.set_pv_b(0x28, 3);
        let p = m.pos;
        m.set_pv_v(HOME, p);
        m.set_pv_f(MV + 0x34, 1.0);
        m.set_pv_f(MV + 4, 0.25);
        m.set_pv_f(MV, 0.5);
        m.set_pv_f(MV + 0xc, f32::from_bits(0x3ecc_cccd));
        m.set_pv_f(MV + 0x10, f32::from_bits(0x3f49_0fdc));
        m.set_pv_f(MV + 0x30, f32::from_bits(0x3f06_0a93));
        m.set_pv_f(MV + 0x2c, f32::from_bits(0x3bb6_0b61));
        m.set_pv_f(MV + 8, f32::from_bits(0x3ecc_cccd));
        if m.pv_i(START) != -1 {
            st(w, id, 0xe, 5);
        } else {
            appear(w, id, ctx, 0);
        }
    }
    if w.mobys[id].drawn != 0 { crate::shadow_volumes::probe_near(w, id, ctx, 24.0, 0x16); }
    if ai::take_damage(w, id, ctx, false) != 0 { alarm(w, id); }
    let range = w.mobys[id].pv_f(RANGE);
    let seen = ai::find_target(w, id, ctx, range, 3.0, -1, None, false);
    let m = &mut w.mobys[id];
    m.set_pv_s(SEEING, seen as i16);
    let t = ai::link(m, T_MOBY);
    let face = t.or(ctx.hero_moby);
    ai::set_link(m, FACE, face);
    let area = m.pv_i(AREA);
    if seen && area != -1 && !ctx.level.in_polygon(area, pos_of(w, t, ctx)) { w.mobys[id].set_pv_s(SEEING, 0); }
}

/// `0x3e2dc8`: the flash, the two timers down, its ticks in the state counted.
fn tail(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    ai::flash(m, FLASH);
    for off in [T360, T362] {
        let mut t = m.pv_s(off) as i32;
        crate::moby::countdown(&mut t);
        m.set_pv_s(off, t as i16);
    }
    if m.state_ticks != -1 { m.state_ticks += 1; }
}

fn seeing(w: &MobyWorld, id: usize) -> bool { w.mobys[id].pv_s(SEEING) == 1 }

/// Lost: 3 (sub 1, sequence 3), `+0x360` 580..600.
fn lose(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    st(w, id, 3, 1);
    set_seq(w, id, ctx, 3, 0, 0x14);
    let t = ctx.game.rng.rand_range(0x244, 600);
    w.mobys[id].set_pv_s(T360, t as i16);
}

/// 0x0fb4 (module docs).
pub fn update_0fb4(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    senses(w, id, ctx);
    match w.mobys[id].sub_state {
        0 => {
            if first_sub(w, id) { limits(w, id, CALM); }
            if seeing(w, id) {
                st(w, id, 5, 2);
                set_seq(w, id, ctx, 2, 0, 10);
            } else if w.mobys[id].pv_s(ALARMED) != 0 {
                st(w, id, 3, 1);
                set_seq(w, id, ctx, 3, 0, 0x14);
                w.mobys[id].set_pv_s(ALARMED, 0);
                if w.mobys[id].pv_s(T360) < 300 {
                    let t = ctx.game.rng.rand_range(0x1e0, 600);
                    w.mobys[id].set_pv_s(T360, t as i16);
                }
            }
        }
        1 => {
            if first_sub(w, id) { limits(w, id, ALERT); }
            if seeing(w, id) {
                st(w, id, 6, 2);
                set_seq(w, id, ctx, 7, 0, 10);
            } else if w.mobys[id].pv_s(T360) == 0 {
                st(w, id, 1, 0);
                set_seq(w, id, ctx, 0, 0, 0x14);
            }
        }
        2 => {
            if first_sub(w, id) { limits(w, id, ALERT); }
            if !seeing(w, id) { lose(w, id, ctx); }
        }
        3 => {
            let r = ai::flight(w, id, ctx);
            let h = ai::health(&w.mobys[id]);
            if let Some(k) = ai::link(&w.mobys[id], h + 0x18).filter(|&k| w.mobys.get(k).is_some_and(|o| o.o_class == MINE)) {
                let (at, p) = (w.mobys[k].pos, w.mobys[id].pos);
                let d = set_len(sub(p, at), 1.0);
                let hit = crate::moby::hits::HitTemplate::new(50.0, ctx.hero_moby, 0x10000, [d[0], d[1], d[2], 0.0]);
                w.deliver_hit_any(id, &hit, [at[0], at[1], at[2], 1.0]);
            }
            if r & 1 != 0 {
                crate::moby::bolts::drop(w, id, 0, 3, ctx);
                crate::moby::classes::death_fade::start(w, id, ctx, 0.0, None);
                w.delete(id);
                return;
            }
            if r & 2 != 0 {
                if seeing(w, id) {
                    st(w, id, 6, 2);
                    set_seq(w, id, ctx, 7, 0, 0x14);
                } else {
                    lose(w, id, ctx);
                }
            }
        }
        4 => {
            if first_sub(w, id) { limits(w, id, SUB4); }
        }
        5 => {
            if first_sub(w, id) {
                let m = &mut w.mobys[id];
                m.has_collision = false;
                m.drawn = 0;
                m.mode |= 1;
            }
            let m = &w.mobys[id];
            let level = ctx.game.level.max(0) as usize;
            let on = ctx.game.missions.get(level * 0x10 + m.mission as usize).is_some_and(|b| b & 0x80 != 0);
            if on {
                appear(w, id, ctx, 0);
                let m = &mut w.mobys[id];
                m.drawn = 1;
                m.has_collision = true;
                m.mode &= !1;
            }
        }
        _ => {}
    }
    states(w, id, ctx);
    tail(w, id);
}

fn states(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let face = ai::link(&w.mobys[id], FACE);
    let take = ai::link(&w.mobys[id], TAKE);
    match w.mobys[id].state {
        1 => {
            if w.mobys[id].pv_s(T362) == 0 {
                st(w, id, 2, -1);
                let a = ctx.game.rng.rand_angle();
                let r = ctx.game.rng.randf(3.0, 5.0);
                let m = &mut w.mobys[id];
                let g = add([a.cos() * r, a.sin() * r, 0.0], m.pv_v(HOME));
                m.set_pv_v(GOAL, g);
                set_seq(w, id, ctx, 0xf, 0, 10);
            }
            ai::hold(w, id, ctx);
        }
        2 => {
            let g = w.mobys[id].pv_v(GOAL);
            let f = walk_to(w, id, ctx, g);
            let t = w.mobys[id].state_ticks as u16;
            if (0x1f <= t && f & 0x80 != 0) || 0x259 <= t {
                st(w, id, 1, -1);
                let n = ctx.game.rng.rand_range(300, 500);
                w.mobys[id].set_pv_s(T362, n as i16);
                let s = seq(w, id, ctx);
                if s as i32 != ctx.game.rng.randi(2) {
                    let n = ctx.game.rng.randi(2) as u8;
                    ctx.host.set_anim(&w.mobys[id], n, 0, 0x14);
                }
            }
        }
        3 => { ai::hold(w, id, ctx); }
        5 => {
            let (h, t) = (ctx.hero.pos, pos_of(w, face, ctx));
            let (yaw, p) = (atan2(h[0] - t[0], h[1] - t[1]), w.mobys[id].pos);
            ai::move_step(w, id, ctx, yaw, p);
            if wrapped(w, id, ctx) {
                st(w, id, 6, -1);
                set_seq(w, id, ctx, 7, 0, 0x14);
            } else if w.mobys[id].state_ticks == 0x14 {
                alarm(w, id);
            }
        }
        6 => {
            let t = pos_of(w, face, ctx);
            if walk_to(w, id, ctx, t) & 0x80 != 0 {
                st(w, id, 7, -1);
                set_seq(w, id, ctx, 8, 0, 0x14);
            }
        }
        7 => {
            if wrapped(w, id, ctx) {
                st(w, id, 6, -1);
                set_seq(w, id, ctx, 7, 0, 10);
                return;
            }
            let (t, p) = (pos_of(w, face, ctx), w.mobys[id].pos);
            let yaw = atan2(t[0] - p[0], t[1] - p[1]);
            approach_angle(yaw, f32::from_bits(0x3d6e_4bb0), &mut w.mobys[id].rot[2], 0);
            let fr = ctx.host.frame_time(&w.mobys[id]);
            if (12.0..=14.0).contains(&fr) {
                let dmg = w.mobys[id].pv_b(0x34) as f32;
                ai::attack_joint(w, id, ctx, 0.25, dmg, 1.0, 2, 1, 0, 1, 0);
            }
            ai::hold(w, id, ctx);
        }
        10 => {
            let Some(o) = take else { return };
            let (t, p) = (pos_of(w, Some(o), ctx), w.mobys[id].pos);
            ai::move_step(w, id, ctx, atan2(t[0] - p[0], t[1] - p[1]), p);
            let m = &w.mobys[id];
            if seq(w, id, ctx) == 5 && m.pos[2] < m.pv_f(HOME + 8) + m.pv_f(MV + 0xa8) * 10.0 {
                ctx.host.set_anim(&w.mobys[id], 6, 0, 10);
            }
            if seq(w, id, ctx) == 6 && wrapped(w, id, ctx) {
                st(w, id, 0xb, -1);
                set_seq(w, id, ctx, 7, 0, 0x14);
            }
        }
        0xb => {
            let Some(o) = take else { return };
            let t = pos_of(w, Some(o), ctx);
            walk_to(w, id, ctx, t);
            if w.mobys[id].pv_f(MV + 0xcc) < 2.0 {
                st(w, id, 0xc, -1);
                set_seq(w, id, ctx, 8, 0, 0x14);
            }
        }
        0xc => {
            if let Some(o) = take {
                let t = pos_of(w, Some(o), ctx);
                walk_to(w, id, ctx, t);
                let fr = ctx.host.frame_time(&w.mobys[id]);
                if 11.0 <= fr {
                    let m = &w.mobys[id];
                    let r = m.rows;
                    let hand = add(m.pos, add(add(scale(r[0], HAND[0]), scale(r[1], HAND[1])), scale(r[2], HAND[2])));
                    let yaw = m.rot[2];
                    let last = 14.0 <= fr;
                    take_one(w, o, ctx, hand, yaw, last);
                    if last { ai::set_link(&mut w.mobys[id], TAKE, None); }
                }
            }
            if wrapped(w, id, ctx) {
                st(w, id, 0xd, -1);
                set_seq(w, id, ctx, 9, 0, 10);
            }
        }
        0xd => {
            if wrapped(w, id, ctx) {
                st(w, id, 1, 0);
                set_seq(w, id, ctx, 0, 0, 10);
            }
        }
        _ => {}
    }
}

/// `0x3e1e10(o, hand, rot, last)`: the moby taken: its sequence 10 (blend 5), its state 8 (sub 1) unless its `+0xbc`
/// is 8; a sixth of the way to the hand, its yaw toward the taker's by 12° a tick; the last tick: hidden without
/// collision, a smoke burst (1) at it, state 1 (sub 0).
fn take_one(w: &mut MobyWorld, o: usize, ctx: &mut Ctx, hand: V3, yaw: f32, last: bool) {
    set_seq(w, o, ctx, 10, 0, 5);
    if w.mobys[o].timer != 8 { st(w, o, 8, 1); }
    let m = &mut w.mobys[o];
    let d = scale(sub(hand, m.pos), f32::from_bits(0x3e2a_aaab));
    m.pos = add(m.pos, d);
    approach_angle(yaw, f32::from_bits(0x3e56_7752), &mut m.rot[2], 0);
    if last {
        m.has_collision = false;
        m.mode = (m.mode & !mode::TARGET) | 0x41;
        let at = m.pos;
        crate::moby::blast::smoke_burst(w, ctx, 1.0, 0.0, Some(o), at, -1);
        st(w, o, 1, 0);
    }
}
