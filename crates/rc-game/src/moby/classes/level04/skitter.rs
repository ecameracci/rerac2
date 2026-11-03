//! Barlow's skitter **0x0fb6** (L04 `0x44dd48`, 118 placed; its senses `0x44e870`, walk `0x44ebd8`, tail
//! `0x44ee50`), a small critter on the framework ([`crate::moby::ai`]) walking with its own walker
//! (crate::moby::crawler, block `+0x140`): health block `+0x20`, the flash `+0x50`, reaction `+0x60` (a hurt goes to
//! state 6, sub 3), the wander block `+0x110` (home, radius 0.5, speed `+0x124`), target `+0x240` (its moby
//! `+0x250`, found `+0x254`).
//!
//! Its words: its home `+0x2e0` (kept once set: `+0x2ec` 12439.4 marks it), its fence path `+0x2f0` (−1 none), its
//! sight range `+0x2f4`, its rest time `+0x2f8` (seconds), the rest timer `+0x2fc`, the wander timer `+0x300`, its
//! leap timer `+0x302`, alarmed `+0x306`, its heading offset `+0x30c`'s speed (`+0x30c` its run speed a second; the
//! turn speed `+0x310`, the offset `+0x314`, its timer `+0x318`), the wait `+0x31c`, the floor moby it rides `+0x320`
//! and its fall `+0x324`, the seat moby `+0x328` (an index; a 0x0929 to ride) and the moby it waits for `+0x32c`,
//! its bite `+0x330` (the health block's damage byte).
//!
//! The L04 engine functions with the port's: `0x33cbd0` / `0x33c8f0` set_state / set_anim (`0x33c818` the sequence at
//! once), `0x33cca0` / `0x33cd38` / `0x33cce0` randi / randf / rand_range, `0x34a3a8` take_damage, `0x34b0f0` flight,
//! `0x34dc98` find_target, `0x33d440` turn_to, `0x345a58` wander, `0x340700` approach, `0x345ef0` reached,
//! `0x33ee98` attack_joint, `0x33e898` ground_z, `0x341dd8` flash, `0x33f640` the shadow, `0x33f8d0` the hit test,
//! `0x366e30` fence, `0x33c0c0` the bolts, `0x4254e8` the fade (crate::moby::classes::death_fade), `0x3329d8` delete.
//!
//! **The set-up** (state 0): on the ground (0.5 over), level; with a seat moby hidden without collision 1.4 over
//! it; the reaction (state 6 sub 3, sequences 9 / 9 / 10 / 0xb, keys 0.1 / 0.05, gravities 1/327 / 1/1800, speeds
//! 0.35 / 0.35 / 0.4); the walker (radius and height 0.5, steps 2 / 2.5, most its run speed); its home and the
//! wander's; the wander timer 120..700; Ratchet's colour words; its bite; idle (1, sub 0, sequence 0).
//!
//! **Each tick**: drawn within 24 of the camera its shadow (range 0x16); a hit (flags 0x230000) of the kind 8: 8
//! (`+0xbc` 4); the frame's damage — the spin's (kind 5) knock 0.8395 up — alarms its group's skitters (`0x44ef90`);
//! the search (its range, 10 high, from its home). **Subs**: 0 calm: waiting on its seat moby (`+0x328`, `+0x32c`
//! a live other) nothing; else the wait down; seeing Ratchet: on a seat (class 0x0929: sitting on it, 7 sub 3,
//! sequence 7 at once, its heading; else 4 sub 2), off a seat one tick in nine half the time the run (4, sub 2) and
//! half the rear (3, sub 2, half of those its sequence 4) — and shown with collision; alarmed: resting (2, sub 1, its
//! rest 0.8..1.2 × `+0x2f8` s). 1 resting: seeing him the run (4, sub 2); rested, idle (1, sub 0, sequence 2). 2: not
//! seeing him, one tick in 19, resting. 3 the hurt's flight: dead its bolts and the fade; over: the run, or resting
//! (the wander timer 120..700). More than 10 over its home, under z 10 (or on level 11 under 75): its bolts, gone.
//!
//! **States**: 1 / 2 wandering (`0x345a58`(0.35, 0.25), its speed toward its wander speed `$gp+0x2858` 2.5/s by
//! 0.0019 a tick, none in the timer's last 20 ticks; walking: sequence 7 at 0.8; out of time one of its idles 0..2
//! (1: 5..6), at their end the timer 120..500 and the walk); 3 rearing (turned to its target, at the end 4); 4 the run
//! (turned to its target with its offset, ±15..30° every 45..60 ticks, the walker along its facing, the fence;
//! within 1 the bite: 5 sub 2, sequence 8 from frame 2, no offset); 5 the bite (turned; frame 8 its sound 3, frame 10
//! a 0.75 sphere at joint 0 of its bite (flags 0x4000001); at the end, him out of reach: 4); 7 riding its seat (out
//! 6 from home: 4); 8 drawn in ([`pulled`]). Walking (not 7 / 8) it falls 0.00272 a tick² to the ground.
//!
//! Not ported: the floor mobys it rides (`0x343fe0` / `0x343fb0` / `0x345788`: an AI moby's `+0x68`[8] frame record,
//! the critter carried and tilted with it; none of the port's mobys keeps the record), the head's look at its target
//! in sub 2 (`+0x2c8`, the joint aims), the big-head cheat (`0x349928`, `+0x260`), the global gate `0x1c9c40`, and
//! what draws it in (state 8: the moby `0x1af8bc`, its state 0x5a, set by nothing the port has; the hit kind 8 is no
//! weapon's yet).

use crate::hero::physics::{add_rot, approach, atan2, dist2};
use crate::moby::classes::props::{seq, set_seq, wrapped};
use crate::moby::{ai, crawler, Ctx, MobyWorld};

pub const CLASS: i16 = 0x0fb6;
const FLASH: usize = 0x50;
const WANDER: usize = 0x110;
const WALK: usize = 0x140;
const T_POINT: usize = 0x240;
const FOUND: usize = 0x254;
const HOME: usize = 0x2e0;
const FENCE: usize = 0x2f0;
const RANGE: usize = 0x2f4;
const REST: usize = 0x2f8;
const T_REST: usize = 0x2fc;
const T_WANDER: usize = 0x300;
const T_LEAP: usize = 0x302;
const ALARMED: usize = 0x306;
const RUN: usize = 0x30c;
const TURN_V: usize = 0x310;
const OFFSET: usize = 0x314;
const T_OFFSET: usize = 0x318;
const WAIT: usize = 0x31c;
const FALL: usize = 0x324;
const SEAT: usize = 0x328;
const WAIT_FOR: usize = 0x32c;
const BITE: usize = 0x330;
const PVARS: usize = 0x340;
/// `$gp+0x2850`: its radius; `0x2854` its reach; `0x2858` its wander speed (a second).
const RADIUS: f32 = 0.5;
const REACH: f32 = 1.0;
const WANDER_SPEED: f32 = 2.5;
/// The home marker (`+0x2ec`).
const MARK: f32 = 12439.4;

fn st(w: &mut MobyWorld, id: usize, s: u8, sub: i32) { w.mobys[id].set_state(s, sub); }
fn found(w: &MobyWorld, id: usize) -> bool { w.mobys[id].pv_i(FOUND) == 1 }
fn link_ok(w: &MobyWorld, k: i32, id: usize) -> Option<usize> {
    usize::try_from(k).ok().filter(|&k| k != id && w.mobys.get(k).is_some_and(|m| m.state != 0xfe && m.state != 0xfd))
}
fn tick_i(w: &mut MobyWorld, id: usize, off: usize) -> bool {
    let mut t = w.mobys[id].pv_i(off);
    let out = crate::moby::countdown(&mut t);
    w.mobys[id].set_pv_i(off, t);
    out
}
fn tick_s(w: &mut MobyWorld, id: usize, off: usize) -> bool {
    let mut t = w.mobys[id].pv_s(off) as i32;
    let out = crate::moby::countdown(&mut t);
    w.mobys[id].set_pv_s(off, t as i16);
    out
}
fn show(w: &mut MobyWorld, id: usize) {
    let has = w.class(CLASS).is_some_and(|c| c.has_collision);
    let m = &mut w.mobys[id];
    m.mode &= 0xffbe;
    m.has_collision = has;
}
/// Resting: 2 sub 1, the rest timer 0.8..1.2 × its rest seconds.
fn rest(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    st(w, id, 2, 1);
    let f = w.mobys[id].pv_f(REST);
    let t = ctx.game.rng.randf(f * 0.8, f * 1.2);
    w.mobys[id].set_pv_i(T_REST, (t * 60.0) as i32);
}
fn run(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, blend: u8) {
    st(w, id, 4, 2);
    set_seq(w, id, ctx, 7, 0, blend);
}

/// `0x44e870` (module docs).
fn senses(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].alpha = 0x80;
    if w.mobys[id].state == 0 {
        let z = ai::ground_z(ctx, 0.5, w.mobys[id].pos, 0);
        let m = &mut w.mobys[id];
        if m.pvars.len() < PVARS { m.pvars.resize(PVARS, 0); }
        m.pos[2] = z;
        m.rot = [0.0; 3];
        if let Some(s) = usize::try_from(m.pv_i(SEAT)).ok().filter(|&s| s < w.mobys.len()) {
            let sz = w.mobys[s].pos[2];
            let m = &mut w.mobys[id];
            m.has_collision = false;
            m.mode |= 0x41;
            m.pos[2] = sz + 1.4;
        }
        let m = &mut w.mobys[id];
        m.set_pv_i(0x320, 0);
        m.set_pv_f(FALL, 0.0);
        m.set_pv_b(0x48, 15);
        m.set_pv_b(0x4a, 4);
        for (k, v) in [(0x7c, 0x3dcc_ccceu32), (0x94, 0x3eb3_3333), (0x90, 0x3eb3_3333), (0x80, 0x3d4c_ccce), (0x88, 0x3b48_3fb8), (0x8c, 0x3a11_a2b4), (0x9c, 0x3ecc_cccd), (0x84, 0x3b48_3fb8)] { m.set_pv_f(k, f32::from_bits(v)); }
        for (k, v) in [(0x29, 1u8), (0x72, 9), (0x76, 10), (0xd2, 6), (0xd3, 0xb), (0x70, 6), (0x71, 9), (0x77, 0xb), (0xcc, 6), (0xcd, 0xb), (0xce, 6), (0xcf, 0xb)] { m.set_pv_b(k, v); }
        m.set_pv_i(0xa8, 3);
        crawler::init(w, id, WALK);
        let m = &mut w.mobys[id];
        m.set_pv_f(WALK + 0xc, 2.5);
        m.set_pv_f(WALK + 8, 2.0);
        m.set_pv_i(WALK, (RADIUS * 1024.0) as i32);
        m.set_pv_f(WALK + 4, RADIUS);
        m.set_pv_f(WALK + 0x24, m.pv_f(RUN) * 0.016_666_668);
        if m.pv_f(HOME + 0xc) != MARK {
            let p = m.pos;
            m.set_pv_v(HOME, p);
            m.set_pv_f(HOME + 0xc, MARK);
        }
        let h = m.pv_v(HOME);
        m.set_pv_v(WANDER, h);
        m.set_pv_f(WANDER + 0xc, MARK);
        m.set_pv_f(WANDER + 0x10, 0.5);
        let t = ctx.game.rng.rand_range(0x78, 700);
        let m = &mut w.mobys[id];
        m.set_pv_i(WAIT, 0);
        m.set_pv_s(T_WANDER, t as i16);
        if let Some(hm) = ctx.hero_moby.and_then(|h| w.mobys.get(h)) { let (l, a) = (hm.light, hm.ambient); let m = &mut w.mobys[id]; m.light = l; m.ambient = a; }
        let m = &mut w.mobys[id];
        let (scale, h30) = (m.scale, m.pv_f(0x30));
        m.set_pv_b(0x28, 0);
        m.set_pv_i(T_OFFSET, 0);
        m.set_pv_f(0x230, scale);
        m.set_pv_f(0x220, h30);
        m.set_pv_f(BITE, m.pv_b(0x34) as f32);
        st(w, id, 1, 0);
        set_seq(w, id, ctx, 0, 0, 0);
    }
    if w.mobys[id].drawn != 0 { crate::shadow_volumes::probe_near(w, id, ctx, 24.0, 0x16); }
    if let Some(h) = w.hit_message(id) {
        if h.tmpl.flags & 0x23_0000 == 0 {
            w.clear_hit(id);
        } else if h.tmpl.kind & 0xff == 8 {
            let m = &mut w.mobys[id];
            m.state = 8;
            m.timer = 4;
        }
    }
    let k = ai::take_damage(w, id, ctx, false);
    if k != 0 {
        if k == 5 {
            let m = &mut w.mobys[id];
            m.set_pv_f(0xf0, 0.0);
            m.set_pv_f(0xf4, f32::from_bits(0x3f56_7752));
        }
        alarm(w, id);
    }
    let (range, home) = (w.mobys[id].pv_f(RANGE), w.mobys[id].pv_v(HOME));
    ai::find_target(w, id, ctx, range, 10.0, -1, Some(home), false);
}

/// `0x44ef90`: its group's live skitters alarmed.
fn alarm(w: &mut MobyWorld, id: usize) {
    let members = usize::try_from(w.mobys[id].group).ok().and_then(|g| w.groups.get(g)).cloned().unwrap_or_default();
    for k in members {
        if let Some(m) = w.mobys.get_mut(k).filter(|m| m.state < 0x80 && m.o_class == CLASS) { m.set_pv_s(ALARMED, 1); }
    }
}

/// 0x0fb6 (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    senses(w, id, ctx);
    match w.mobys[id].sub_state {
        1 => {
            if found(w, id) {
                run(w, id, ctx, 0xe);
            } else if w.mobys[id].pv_i(T_REST) == 0 {
                st(w, id, 1, 0);
                set_seq(w, id, ctx, 2, 0, 0xc);
            }
        }
        0 => {
            let m = &w.mobys[id];
            let waiting = m.pv_i(SEAT) != -1 && link_ok(w, m.pv_i(WAIT_FOR), id).is_some();
            if !waiting {
                tick_i(w, id, WAIT);
                if found(w, id) {
                    let seat = w.mobys[id].pv_i(SEAT);
                    if seat == -1 {
                        if ctx.game.rng.randi(9) == 0 {
                            if ctx.game.rng.randi(2) == 0 {
                                run(w, id, ctx, 0xe);
                            } else {
                                st(w, id, 3, 2);
                                set_seq(w, id, ctx, 3, 0, 0xe);
                                if ctx.game.rng.randi(2) == 1 { set_seq(w, id, ctx, 4, 0, 0xe); }
                            }
                        }
                    } else {
                        show(w, id);
                        let s = seat as usize;
                        if w.mobys.get(s).is_some_and(|o| o.o_class == 0x0929) {
                            st(w, id, 7, 3);
                            ctx.host.set_anim(&w.mobys[id], 7, 0, 0);
                            let y = w.mobys[s].rot[2];
                            let m = &mut w.mobys[id];
                            m.rot[2] = y;
                            m.set_pv_i(SEAT, -1);
                            return tail(w, id, ctx);
                        }
                        w.mobys[id].set_pv_i(SEAT, -1);
                        run(w, id, ctx, 0xe);
                    }
                    show(w, id);
                } else if w.mobys[id].pv_s(ALARMED) != 0 {
                    w.mobys[id].set_pv_s(ALARMED, 0);
                    rest_alarmed(w, id, ctx);
                }
            }
        }
        2 => {
            if !found(w, id) && ctx.game.rng.randi(0x13) == 0 { rest_alarmed(w, id, ctx); }
        }
        3 => {
            if w.mobys[id].state != 7 {
                let r = ai::flight(w, id, ctx);
                if r & 1 != 0 {
                    w.mobys[id].set_pv_s(0x98, 0);
                    crate::moby::bolts::drop(w, id, 0, 3, ctx);
                    crate::moby::classes::death_fade::start(w, id, ctx, 0.0, None);
                    return;
                }
                if r & 2 != 0 {
                    if found(w, id) {
                        run(w, id, ctx, 0xe);
                    } else {
                        rest(w, id, ctx);
                        let t = ctx.game.rng.rand_range(0x78, 700);
                        w.mobys[id].set_pv_s(T_WANDER, t as i16);
                    }
                }
            }
        }
        _ => {}
    }
    tail(w, id, ctx)
}

/// Sub 0's alarm (and sub 2's tick in 19): resting with the wander timer cleared.
fn rest_alarmed(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    rest(w, id, ctx);
    w.mobys[id].set_pv_s(T_WANDER, 0);
}

/// The height checks and the states, then the walk and the tail.
fn tail(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let z = w.mobys[id].pos[2];
    let lost = 10.0 < z - w.mobys[id].pv_f(HOME + 8) || z < 10.0 || (z < 75.0 && ctx.game.level == 0xb);
    if lost {
        let m = &mut w.mobys[id];
        m.set_pv_s(0x98, 0);
        m.rot = [0.0; 3];
        crate::moby::bolts::drop(w, id, 0, 3, ctx);
        return w.delete(id);
    }
    if states(w, id, ctx) {
        crate::moby::bolts::drop(w, id, 0, 3, ctx);
        return w.delete(id);
    }
    if !matches!(w.mobys[id].state, 7 | 8) { walk(w, id, ctx); }
    post(w, id, ctx);
}

fn idle_or_walk(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, idles: (i32, u8)) {
    let t = { tick_s(w, id, T_WANDER); w.mobys[id].pv_s(T_WANDER) };
    let s = seq(w, id, ctx);
    if s == 0 && t != 0 { set_seq(w, id, ctx, 7, 0, 0xe); }
    let t = w.mobys[id].pv_s(T_WANDER);
    let want = if t < 0x15 { 0.0 } else { WANDER_SPEED * 0.016_666_668 };
    let mut v = w.mobys[id].pv_f(WANDER + 0x14);
    approach(want, f32::from_bits(0x3afe_dcbb), &mut v);
    w.mobys[id].set_pv_f(WANDER + 0x14, v);
    let s = seq(w, id, ctx);
    if s == 7 {
        ctx.host.set_anim_speed(&w.mobys[id], 0.8);
        if t == 0 {
            let r = ctx.game.rng.randi(idles.0) as u8 + idles.1;
            if s != r {
                let n = ctx.game.rng.randi(idles.0) as u8 + idles.1;
                ctx.host.set_anim(&w.mobys[id], n, 0, 0xc);
            }
        }
        ai::wander(w, id, ctx, f32::from_bits(0x3eb3_3333), 0.25, WANDER);
    } else if (idles.1..idles.1 + idles.0 as u8).contains(&s) && wrapped(w, id, ctx) {
        let t = ctx.game.rng.rand_range(0x78, 500);
        w.mobys[id].set_pv_s(T_WANDER, t as i16);
        if seq(w, id, ctx) != 7 { ctx.host.set_anim(&w.mobys[id], 7, 0, 0xc); }
    }
}

fn turn(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    let t = m.pv_v(T_POINT);
    let yaw = add_rot(atan2(t[0] - m.pos[0], t[1] - m.pos[1]), m.pv_f(OFFSET));
    ai::turn_to(m, yaw, f32::from_bits(0x3d4c_cccd), f32::from_bits(0x3e99_999a), f32::from_bits(0x3dd6_7752), TURN_V);
}

/// The states (module docs); true when it is drawn in and goes.
fn states(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    match w.mobys[id].state {
        1 => idle_or_walk(w, id, ctx, (3, 0)),
        2 => {
            idle_or_walk(w, id, ctx, (2, 5));
            ctx.host.set_anim_speed(&w.mobys[id], 1.0);
        }
        3 => {
            turn(w, id);
            if wrapped(w, id, ctx) {
                st(w, id, 4, -1);
                set_seq(w, id, ctx, 7, 0, 0xf);
            }
        }
        4 => {
            ctx.host.set_anim_speed(&w.mobys[id], 1.0);
            let t = w.mobys[id].pv_v(T_POINT);
            if dist2(w.mobys[id].pos, t) <= REACH {
                st(w, id, 5, 2);
                set_seq(w, id, ctx, 8, 2, 6);
                w.mobys[id].set_pv_f(OFFSET, 0.0);
            } else if tick_i(w, id, T_OFFSET) {
                let f = ctx.game.rng.randf(f32::from_bits(0x3e86_0a93), f32::from_bits(0x3f06_0a93));
                let f = if ctx.game.rng.randi(2) == 0 { -f } else { f };
                let t = ctx.game.rng.rand_range(0x2d, 0x3c);
                let m = &mut w.mobys[id];
                m.set_pv_f(OFFSET, f);
                m.set_pv_i(T_OFFSET, t);
            }
        }
        5 => {
            let sa = ctx.host.anim(&w.mobys[id]).map_or(0xff, |a| a.seq_a);
            if sa != 8 { return false; }
            if ai::reached(w, id, ctx, 8.0) { ctx.host.play_sound(&w.mobys[id], 3, 0); }
            if ai::reached(w, id, ctx, 10.0) {
                let d = w.mobys[id].pv_f(BITE);
                ai::attack_joint(w, id, ctx, 0.75, d, 1.0, 0, 0x400_0001, 0, 1, 0);
                return false;
            }
            let t = w.mobys[id].pv_v(T_POINT);
            if wrapped(w, id, ctx) && REACH < dist2(w.mobys[id].pos, t) {
                st(w, id, 4, 2);
                set_seq(w, id, ctx, 7, 0, 6);
            }
        }
        7 => {
            if 6.0 < dist2(w.mobys[id].pos, w.mobys[id].pv_v(HOME)) { st(w, id, 4, 2); }
        }
        8 => return pulled(w, id, ctx),
        _ => {}
    }
    false
}

/// `0x44f010`: drawn in by the moby `0x1af8bc` (none in the port: it goes at once).
fn pulled(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    let s = 1;
    if seq(w, id, ctx) != s { ctx.host.set_anim(&w.mobys[id], s, 0, 10); }
    true
}

/// `0x44ebd8`: in 4 turned to its target and walked along its facing (the walker at its radius), kept in its fence;
/// in 5 turned. Off the hurt's flight it falls (0.00272 a tick²) to the ground 0.5 over it, its fall reset there.
fn walk(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        4 => {
            turn(w, id);
            let m = &mut w.mobys[id];
            m.set_pv_i(WALK, (RADIUS * 1024.0) as i32);
            m.set_pv_f(WALK + 4, RADIUS);
            crawler::ahead(w, id, ctx, WALK);
            let f = w.mobys[id].pv_i(FENCE);
            if let Some(p) = usize::try_from(f).ok().and_then(|f| ctx.level.paths.get(f)) {
                if let Some(q) = ai::fence(p, RADIUS, w.mobys[id].pos) { w.mobys[id].pos = q; }
            }
        }
        5 => turn(w, id),
        _ => {}
    }
    if w.mobys[id].sub_state == 3 { return; }
    let m = &mut w.mobys[id];
    let f = m.pv_f(FALL) + 0.002_722_222_4;
    m.set_pv_f(FALL, f);
    m.pos[2] -= f - 2.0;
    let g = ai::ground_z(ctx, 0.5, w.mobys[id].pos, 0);
    let m = &mut w.mobys[id];
    m.pos[2] -= 2.0;
    if g <= m.pos[2] { return; }
    m.set_pv_i(0x320, 0);
    m.pos[2] = g;
    m.set_pv_f(FALL, 0.0);
}

/// `0x44ee50`: the flash, the rest and leap timers down, its ticks in the state.
fn post(w: &mut MobyWorld, id: usize, _ctx: &mut Ctx) {
    ai::flash(&mut w.mobys[id], FLASH);
    tick_i(w, id, T_REST);
    tick_s(w, id, T_LEAP);
    let m = &mut w.mobys[id];
    if m.state_ticks != -1 { m.state_ticks += 1; }
}
