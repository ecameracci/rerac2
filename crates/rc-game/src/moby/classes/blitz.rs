//! The Blitz Gun (item 0x1a, crate::hero::items::ROWS; L00's record `0x263bc0`: attachment 0, stance 2 (both hands),
//! use sequence 0x89, the firing arm's 0x8a moving and none crouched, icon 0xea67, 2 shots a second, damage 3, 40
//! shells): the hand moby **0x05d1** (`0x3a6818`, called by the item pass while it is the hand item) and the tracer
//! it leaves along each pellet **0x0d5e** (`0x3fd6c8`, made by `0x3fd5a8`; no model, hidden: it draws itself).
//!
//! **The hand** (pvars: `+0x00` the cooldown (seconds), `+0x04` (s16) the pump's wait, `+0x06` (s16) the shot's
//! delay, `+0x08` / `+0x0a` (s16) the muzzle's starburst and flash ticks, `+0x0c` the starburst's roll, `+0x10` the
//! muzzle, `+0x20` / `+0x24` one over the starburst's and the flash's ticks, `+0x28` / `+0x2c` the flash's two spins,
//! `+0x30` the shot's way, `+0x40` the screen flash's ticks, `+0x44` one over them, `+0x48` the shot came in a use
//! already running (the bigger kick), `+0x4c` the rumble's handle; `+0x50` the up at the muzzle, the port's, for the
//! draw). Every tick: state **0** clears the starburst and goes to **1**; **6** waits out the pump (0x1c ticks), then
//! its class sound 2 and **1**; its sequence run out, back to 1 (blend 2); Ratchet in the look stance 1 is sent to its
//! twin 0x1d; the cooldown runs down. In the twin it queues the crosshair at the screen's centre ([`crosshair`],
//! `0x2db498(1.3, 0, 90, …, FX 0x21, −1, 4)`: its mode from the −1, so FX 0x21 draws as a sprite, mode 2; red
//! `0x600000ff` while the cooldown runs with shells left, else green `0x600fff0f`). Outside the twin `0x18c354` is set to 0x28, in it
//! counted down (the first shot from the look stance waits it out). There is no aim of its own (`0x18aa18` is only
//! the lock cheat's, `0x18b270`).
//!
//! **1** (the items not kept away, the cooldown out, in the hand 0x17 ticks, the look wait out): ○ held (`0x18b050`)
//! and a shell taken: when Ratchet already plays the use sequence, moves (group ≠ 0) or raises the arm at a wall
//! (`0x18c30c`), the shot waits 5 ticks and kicks harder; else 2. The use stance (`0x2af5e0`), the rumble, **2**, the
//! starburst's roll, the hand's sequence 2, the starburst 2 ticks and the flash 8, their spins, the screen flash 3.
//! Without a shell a press plays its class sound 4. While the delay runs nothing counts; then each tick the flash,
//! the starburst and the screen flash count down and, while either of the first two runs, the draw is asked for.
//!
//! The delay out: a lock (`0x18aa18`) turns the shot's way from the muzzle to its aim point (`0x32b4d0`), Ratchet's
//! forward row (`0x18a550`) taking it. **2** → **3**: the muzzle (joint 0), the muzzle's burst ([`burst`],
//! `0x3a73f0`), the shot's way kept (`0x18a580`: his forward row or the lock's way), the draw, its class sound 0 and the
//! camera's kick (`0x1b8d60`: up and down and back and forth 0.06 for 0x1e ticks; 0.12 with `+0x48`). **3** → **6** a
//! tick later: the pump 0x1c; the mobys within 1.5 of the gun's point 1.35 out its barrel (−row 1) and 0.275 along its
//! row 0 are hit with the row's damage outright (`0x2e14b0(1.5, …, 1, Ratchet)`, the hit's way −1.5·row 1); then 20
//! pellets ([`pellets`]); the cooldown 1 / the row's rate.
//!
//! **The pellets** (`0x3a71f0` each): from the muzzle (joint 0) offset −0.1 along Ratchet's up row (the arm raised:
//! +0.1 along the gun's row 2), along his forward row (the arm raised: the gun's −row 1): each turned 0.01..0.327 about
//! his left row (the gun's −row 0), then about the way by a roll that grows 30°..90° a pellet; 32 long. The line
//! (flags 0x10, Ratchet left out; the mobys an earlier pellet hit left out too, their collision off until the last
//! pellet, `+0x98`) hits a moby: the row's damage, ×0.75 from 10 away, ×0.5 from 14, ×0.125 from 20 (`0x31d378`; the
//! hit's way the pellet's at 1.5 long, flags 0x10000, its kind the class's `<< 16 | 0x0203`). A tracer from the pellet's
//! start to its end, or to what it met.
//!
//! **The tracer** ([`update_tracer`]; pvars: `+0x20` its tail's step, `+0x30` its texture's scroll, `+0x34` it met
//! something, `+0x38` its life (20), `+0x3c` one over it, `+0x50` its tail): at the end point, its tail 1.5 behind the
//! start, stepping 0.133 a tick toward the end; made with a puff of 15 sparks at the end (`0x3fdcc0`). Each tick its
//! two glows at the end (`0x33ef78`: 420000, alpha 0x40 → 0x10 over 2 ticks, `0x202080`, `def[0x35]`), the draw;
//! gone when its life is out or out of the world's 2..1021 box.
//!
//! **The draws** (crate::fx): the hand's ([`draw`], `0x3a7968`) — the starburst (eight rays 4 long round the muzzle,
//! FX 0xb's right half, additive, `0x2040ff` at its ticks' share of 255), the flash (two quads FX 0x38, additive,
//! `0x8080ff` at its share of 20, sliding 16 out along the shot's way as it fades and growing 1.5 → 20, spun 2° and
//! −2.5° a frame) and, while it runs, the screen's flash (an additive full-screen fill `0x808080` at its share of 32,
//! [`crate::moby::MobyWorld::screen_fills`]); the tracer's ([`draw_tracer`], `0x3fd908`).
//!
//! Its glow word in the hand (`0x2a7ed8`'s case 0x1a, crate::hero::items) and Ratchet's swap face when it is drawn
//! (`0x2b1380`, crate::hero::items::ItemBlock::swap_face).
//!
//! Not ported: the pad rumble (`0x2d6d60` / `0x2d6f38`), the weapon mods' effects on a hit (`0x3f9f70`), the lock cheat
//! (`0x18b270`), the gravity frames' bends of the tracer (`0x380e08`), the glare quad the screen flash draws 0.1 in
//! front of the camera toward the muzzle (FX 0xb under ALPHA 0x49, `Cd·As + Cd`: the engine's effect pass has no such
//! blend). The muzzle burst's carry in the air (`auStack_d0`, only set on the ground: the stack's leftovers) is 0. The
//! upgraded Blitz (rows 0x44, 0x5b, 0x5c: the hand 0x0af1 and its tracer 0x0d9c) is crate::moby::classes::blitz_mega;
//! its pellets' lines are [`pellet`]'s, its tracer this module's with its own look ([`TracerLook`]).

use crate::collision::QueryFlags;
use crate::hero::items::{row, ItemRow, BLITZ};
use crate::hero::physics::{add, add_rot, atan2, cross, dist, scale, set_len, sub, unit, V3};
use crate::moby::hits::HitTemplate;
use crate::moby::{countdown, Ctx, MobyWorld};

/// The hand's class (item 0x1a's row `+0x04`) and the tracer's; the upgraded Blitz's tracer (`0x3fdee0`).
pub const HAND_CLASS: i16 = 0x5d1;
pub const TRACER: i16 = 0xd5e;
pub const MEGA_TRACER: i16 = 0xd9c;
/// `t[3]` of the hits' template (`0x45afdf66`).
const NO_KNOCK_LIMIT: f32 = 5627.987;
/// `$gp+0x640` / `+0x644`: the shot's delay in a running use / from a standing start; `+0x648` / `+0x64c` /
/// `+0x650`: the starburst's ticks, the flash's (the same either way).
const DELAY_RUNNING: i16 = 5;
const DELAY_START: i16 = 2;
const STARBURST: i16 = 2;
const FLASH: i16 = 8;
/// `$gp+0x674`: the screen flash's ticks.
const SCREEN_FLASH: i32 = 3;
/// The pump's wait (state 6).
const PUMP: i16 = 0x1c;
/// The pellets and their length.
const PELLETS: usize = 20;
const PELLET_LEN: f32 = 32.0;

/// The Blitz Gun's row as `0x139568[0x1a]` gives it (crate::upgrades).
fn cur_row(game: &crate::game::GameState) -> Option<&'static ItemRow> { row(game, BLITZ) }

/// `0x321b28(out, v, 0x31fe98(a, axis))`: `v` turned by the quaternion of `a` about `axis` (`q·v·q*`).
pub(crate) fn qturn(v: V3, axis: V3, a: f32) -> V3 {
    use crate::quat::{axis_angle, mul};
    let q = axis_angle(a, axis);
    let t = mul(mul(q, [v[0], v[1], v[2], 0.0]), [-q[0], -q[1], -q[2], q[3]]);
    [t[0], t[1], t[2]]
}

/// `0x2eef88(out, a, b)` = b × a.
pub(crate) fn cross_ba(a: V3, b: V3) -> V3 { cross(b, a) }

/// `0x2db498(1.3, 0, 90, hand, colour, 0, 0x21, −1, 4)` in the twin 0x1d (the upgraded Blitz's the same): the
/// crosshair at the screen's centre, red while the cooldown (`+0x00`) runs with shells left, else green
/// (crate::hud::marker).
pub(crate) fn crosshair(w: &MobyWorld, id: usize, ctx: &mut Ctx) {
    let red = 0.0 < w.mobys[id].pv_f(0) && ctx.hero.items.has_ammo(ctx.game);
    crate::hud::marker(&mut ctx.game.markers, 1.3, id, if red { 0x6000_00ff } else { 0x600f_ff0f }, None, 0x21);
}

/// `0x2eeb40(+0x00)`: the cooldown down a frame (1/60 s).
pub(crate) fn cooldown(w: &mut MobyWorld, id: usize) {
    let c = w.mobys[id].pv_f(0);
    w.mobys[id].set_pv_f(0, if 1.0 / 60.0 < c { c - 1.0 / 60.0 } else { 0.0 });
}

/// `0x2eeae0` on the s16 at `off`: true when it was 0 or just reached it.
pub(crate) fn count_s(w: &mut MobyWorld, id: usize, off: usize) -> bool {
    let mut n = w.mobys[id].pv_s(off) as i32;
    let done = countdown(&mut n);
    w.mobys[id].set_pv_s(off, n as i16);
    done
}

/// `0x3a6818` (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].pvars.resize(crate::moby::PVAR_BYTES, 0);
    if w.mobys[id].state == 0 {
        let m = &mut w.mobys[id];
        m.state = 1;
        m.set_pv_i(0x4c, -1);
        m.set_pv_s(8, 0);
        m.set_pv_s(0xa, 0);
    }
    if w.mobys[id].state == 6 && count_s(w, id, 4) {
        ctx.host.play_sound(&w.mobys[id], 2, 0);
        w.mobys[id].state = 1;
    }
    // Its sequence run out: back to 1 (`0x31b760(hand, 1, 0, 2, 0)`).
    if let Some(a) = ctx.host.anim(&w.mobys[id]) {
        if a.seq_a == a.seq_b && a.flags & 2 != 0 { ctx.host.set_anim(&w.mobys[id], 1, 0, 2); }
    }
    let hero_state = ctx.hero.state;
    if hero_state == 1 { ctx.hero.request_state(0x1d, true); }
    cooldown(w, id);
    if hero_state == 0x1d { crosshair(w, id, ctx); }
    let ok = if hero_state == 0x1d { countdown(&mut ctx.hero.items.look_wait) } else { ctx.hero.items.look_wait = 0x28; true };
    let Some(r) = cur_row(ctx.game).copied() else { return };
    let it = ctx.hero.items;
    if w.mobys[id].state == 1 && it.no_use == 0 && w.mobys[id].pv_f(0) == 0.0 && 0x17 <= it.item_ticks && ok {
        if it.held & it.trigger != 0 && ctx.hero.items.take_ammo(ctx.game, 1) {
            let running = ctx.hero_view.seq_b as i32 == r.use_seq || ctx.hero.group != 0 || it.raised;
            {
                let m = &mut w.mobys[id];
                m.set_pv_s(6, if running { DELAY_RUNNING } else { DELAY_START });
                m.set_pv_s(8, STARBURST);
                m.set_pv_s(0xa, FLASH);
                m.set_pv_i(0x48, running as i32);
            }
            ctx.hero.items.use_start(ctx.hero.group);
            let g = &mut ctx.game.rng;
            let roll = g.rand_angle();
            {
                let m = &mut w.mobys[id];
                m.state = 2;
                m.set_pv_f(0xc, roll);
            }
            ctx.host.set_anim(&w.mobys[id], 2, 0, 0);
            let (a, b) = (ctx.game.rng.rand_angle(), ctx.game.rng.rand_angle());
            let m = &mut w.mobys[id];
            m.set_pv_f(0x20, 1.0 / STARBURST as f32);
            m.set_pv_f(0x24, 1.0 / FLASH as f32);
            m.set_pv_f(0x28, a);
            m.set_pv_f(0x2c, b);
            m.set_pv_i(0x40, SCREEN_FLASH);
            m.set_pv_f(0x44, 1.0 / SCREEN_FLASH as f32);
        } else if it.pressed & it.trigger != 0 {
            ctx.host.play_sound(&w.mobys[id], 4, 0);
        }
    }
    // The delay out: the flashes count down, the draw while they run.
    if w.mobys[id].pv_s(6) == 0 {
        count_s(w, id, 0xa);
        count_s(w, id, 8);
        let mut n = w.mobys[id].pv_i(0x40);
        countdown(&mut n);
        w.mobys[id].set_pv_i(0x40, n);
        if w.mobys[id].pv_s(8) != 0 || w.mobys[id].pv_s(0xa) != 0 { register(w, id, ctx); }
    }
    if !matches!(w.mobys[id].state, 2 | 3) || !count_s(w, id, 6) { return; }
    // `0x18a550` / `0x18a580`: Ratchet's forward row, or the way from the muzzle (as it stands) to the lock.
    let mut fwd = ctx.hero.rows[0];
    if let Some((t, _)) = ctx.hero.items.aim_target.filter(|&(t, _)| w.mobys.get(t).is_some()) {
        fwd = unit(sub(crate::moby::aim::aim_point(w, t), w.mobys[id].pv_v(0x10)));
    }
    if w.mobys[id].state == 2 {
        w.mobys[id].state = 3;
        let muzzle = ctx.host.joint(&w.mobys[id], 0).unwrap_or(w.mobys[id].pos);
        w.mobys[id].set_pv_v(0x10, muzzle);
        burst(ctx, muzzle, fwd);
        w.mobys[id].set_pv_v(0x30, fwd);
        register(w, id, ctx);
        ctx.host.play_sound(&w.mobys[id], 0, 0);
        // `0x1b8d60`: the kick up and down and back and forth (the second's length kept).
        let amp = if w.mobys[id].pv_i(0x48) != 0 { 0.12 } else { 0.06 };
        ctx.game.shakes.ask(0, amp, 0x1e);
        let s = &mut ctx.game.shakes.0[1];
        (s.amp, s.timer) = (amp, 0x1e);
        return;
    }
    w.mobys[id].state = 6;
    w.mobys[id].set_pv_s(4, PUMP);
    let class = w.mobys[id].o_class;
    let (pos, rows) = (w.mobys[id].pos, w.mobys[id].rows);
    let mut t = HitTemplate::new(r.damage, Some(id), 0x10000, [0.0; 4]);
    t.kind = (class as u16 as u32) << 16 | 0x0203;
    // `0x2e14b0(1.5, …, 1, Ratchet)`: point-blank.
    let at = add(add(pos, scale(rows[1], -1.35)), scale(rows[0], 0.275));
    if let Some(sc) = ctx.scene {
        let d = scale(rows[1], -1.5);
        t.dir = [d[0], d[1], d[2], NO_KNOCK_LIMIT];
        for m in crate::collision::coll_sphere_mobys(sc, at, 1.5, QueryFlags(1), ctx.hero_moby) {
            if Some(m) == ctx.hero_moby { continue; }
            w.deliver_hit_any(m, &t, [0.0; 4]);
        }
    }
    pellets(w, id, ctx, &mut t, fwd, r.damage);
    w.mobys[id].set_pv_f(0, 1.0 / r.rate);
}

/// `0x2e7920(0x3a7968, hand)`, with the up at the muzzle the draw takes (`0x326448(muzzle, out, 1)`).
fn register(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let up = ctx.hero.gravity.up_at(w.mobys[id].pv_v(0x10));
    w.mobys[id].set_pv_v(0x50, up);
    w.register_draw(id);
}

/// The volley (`0x3a6818`'s state 3): 20 pellets from the muzzle ([`pellet`]); a moby hit is left out of the later
/// pellets' lines.
fn pellets(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, t: &mut HitTemplate, fwd: V3, damage: f32) {
    let muzzle = ctx.host.joint(&w.mobys[id], 0).unwrap_or(w.mobys[id].pos);
    let (way, side, off) = if ctx.hero.items.raised {
        let g = w.mobys[id].rows;
        (scale(g[1], -1.0), scale(g[0], -1.0), scale(g[2], 0.1))
    } else {
        let h = ctx.hero.rows;
        (fwd, h[1], scale(h[2], -0.1))
    };
    w.mobys[id].set_pv_v(0x30, way);
    let third = std::f32::consts::FRAC_PI_3;
    let mut roll = 0.0f32;
    let mut hit: Vec<usize> = Vec::new();
    for _ in 0..PELLETS {
        let a = ctx.game.rng.randf(0.01, 0.327_249_26);
        let mut dir = qturn(way, side, a);
        let mut o = qturn(off, side, a);
        let b = ctx.game.rng.randf(third * 0.5, third * 1.5);
        roll = add_rot(b, roll);
        dir = qturn(dir, way, roll);
        o = qturn(o, way, roll);
        dir = scale(dir, PELLET_LEN);
        let start = add(muzzle, o);
        let end = add(start, dir);
        if let Some(m) = pellet(w, ctx, t, start, end, damage, &hit, false) { hit.push(m); }
    }
}

/// `0x3a71f0(hand, start, end, tmpl)`: one pellet's line (flags 0x10, Ratchet and `skip` left out), its tracer and its
/// hit; the moby hit. The upgraded Blitz's `0x3d3e50` (`mega`) is the same with its own tracer.
#[allow(clippy::too_many_arguments)]
pub(crate) fn pellet(w: &mut MobyWorld, ctx: &mut Ctx, t: &mut HitTemplate, start: V3, end: V3, damage: f32, skip: &[usize], mega: bool) -> Option<usize> {
    let d = set_len(sub(end, start), 1.5);
    t.dir = [d[0], d[1], d[2], NO_KNOCK_LIMIT];
    let met = ctx.coll.and_then(|c| crate::collision::coll_line_live(c, ctx.scene, start, end, QueryFlags(0x10), ctx.hero_moby, &|m| !skip.contains(&m)));
    let Some(o) = met else {
        spawn_tracer(w, ctx, start, sub(end, start), end, false, mega);
        return None;
    };
    spawn_tracer(w, ctx, start, sub(o.point, start), o.point, true, mega);
    let m = o.moby?;
    let r = dist(o.point, start);
    t.damage = if r < 10.0 {
        damage
    } else if r < 14.0 {
        damage * 0.75
    } else if r < 20.0 {
        damage * 0.5
    } else {
        damage * 0.125
    };
    w.deliver_hit_any(m, t, [0.0; 4]);
    Some(m)
}

/// `0x3fd5a8(start, d, at, met)`: the tracer at `at`, its tail stepping 0.133 a tick along `d` from 1.5 behind
/// `start`, for 20 ticks; the puff at `at` (`0x3fdcc0`, [`impact`]). The upgraded Blitz's (`0x3fdee0`, `mega`):
/// the tracer 0x0d9c, a third of a unit a tick from a quarter behind, for 12 ticks (`gp+0x1b74`); its puff
/// [`mega_impact`] (`0x3fe618`).
fn spawn_tracer(w: &mut MobyWorld, ctx: &mut Ctx, start: V3, d: V3, at: V3, met: bool, mega: bool) {
    let (class, speed, back, life) = if mega { (MEGA_TRACER, 1.0 / 3.0, -0.25, 12) } else { (TRACER, 0.133_333_34, -1.5, 0x14) };
    if w.class(class).is_none() { return; }
    let id = w.create(class);
    let step = set_len(d, speed);
    {
        let m = &mut w.mobys[id];
        m.state = 0;
        m.update_dist = 0xff;
        m.draw_dist = 0xff;
        m.drawn = 1;
        m.mode |= 0x41;
        m.set_pv_v(0x20, step);
        m.pos = at;
        m.set_pv_v(0x50, add(start, set_len(step, back)));
        m.set_pv_i(0x38, life);
        m.set_pv_i(0x34, met as i32);
        m.set_pv_f(0x3c, 1.0 / life as f32);
    }
    if mega { mega_impact(ctx, at) } else { impact(ctx, at) }
}

/// `0x3fe618(at)`: fifteen grey sparks (`0x339848`: 0.1..0.3 × 210000, growing 500 a tick, 0x1e ticks, alpha 0x7f,
/// `0x7f7f7f`) jittered 0.2 about `at`, flying 0.5..1.25 a second up to 80° up and 80° either side of the way to the
/// camera; then five pairs of twinkles (`0x33a6a0`: 0.25..0.75 · 0.1 growing to it, red `0x7fff1010`, and half that
/// in white `0x7f7f7f7f`, 60 ticks, falling 0.003 a tick, turning one way and the other) jittered 0.15, drifting 1..4
/// a second a random way.
fn mega_impact(ctx: &mut Ctx, at: V3) {
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let g = &mut ctx.game.rng;
    let c = sub(ctx.host.camera_pos(), at);
    let yaw0 = atan2(c[0], c[1]);
    for _ in 0..15 {
        let f = g.randf(0.5, 1.25);
        let spin = g.randf_signed(1.0, 2.0) as i32;
        let pitch = g.randf(0.0, 1.396_263_6);
        let yaw = add_rot(yaw0, g.randf_signed(0.0, 1.396_263_6));
        let vel = crate::moby::classes::shot::polar(f * (1.0 / 60.0), yaw, pitch);
        let p: V3 = std::array::from_fn(|k| at[k] + g.randf_signed(0.0, 0.2));
        let size = g.randf(0.1, 0.3) * 210_000.0;
        crate::particles::spark(sys, g, size, 500.0, 1.0, 0.0, 0.0, p, vel, 0x1e, 0x7f, 0x7f_7f7f, spin);
    }
    for _ in 0..5 {
        let s = g.randf(0.25, 0.75);
        let k = g.randi(2);
        let k = if k != 0 { k } else { -1 };
        let f = g.randf(1.0, 4.0) * (1.0 / 60.0);
        let v = g.rand_vec(f, f);
        let p: V3 = std::array::from_fn(|i| at[i] + g.randf_signed(0.0, 0.15));
        let fall = f32::from_bits(0x3b44_9ba6);
        for (size, end, rgba, rot, spin) in [(s * 0.1, s, 0x7fff_1010u32, 0u8, k), (s * 0.05, s * 0.5, 0x7f7f_7f7f, 1, -k)] {
            if let Some(i) = crate::particles::twinkle(sys, g, size, end, fall, p, 0x3c, rgba, rot, spin as i8, v, 0) {
                sys.pool.recs[i][9] = 4 + 0x70;
            }
        }
    }
}

/// `0x3fdcc0(at)`: fifteen grey sparks (`0x339848`: 0.05..0.2 × 210000, growing 500 a tick, 0x1e ticks, alpha 0x7f,
/// `0x7f7f7f`) jittered 0.1 about `at`, flying 0.5..1 a second up to 80° up and 80° either side of the way to the
/// camera.
fn impact(ctx: &mut Ctx, at: V3) {
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let g = &mut ctx.game.rng;
    let c = sub(ctx.host.camera_pos(), at);
    let yaw0 = atan2(c[0], c[1]);
    for _ in 0..15 {
        let f = g.randf(0.5, 1.0);
        let spin = g.randf_signed(1.0, 2.0) as i32;
        g.randf(0.05, 0.2);
        let pitch = g.randf(0.0, 1.396_263_6);
        let yaw = add_rot(yaw0, g.randf_signed(0.0, 1.396_263_6));
        let vel = crate::moby::classes::shot::polar(f * (1.0 / 60.0), yaw, pitch);
        let p: V3 = std::array::from_fn(|k| at[k] + g.randf_signed(0.0, 0.1));
        let size = g.randf(0.05, 0.2) * 210_000.0;
        crate::particles::spark(sys, g, size, 500.0, 1.0, 0.0, 0.0, p, vel, 0x1e, 0x7f, 0x7f_7f7f, spin);
    }
}

/// `0x3a73f0(hand, muzzle)`: the muzzle's burst along `fwd` (Ratchet's forward row, or the lock's way). Twenty times,
/// from 0.4 ahead of the muzzle: a dark puff (`0x335bb8`, overwritten to 60 ticks of `0x7f202020`, normal blend, its
/// growth 5) jittered 0.3, riding Ratchet's move, and a grey spark (`0x339848`) jittered 0.6 flying 0.2..0.27 a tick
/// along `fwd` turned 20° about his left row then rolled 0..36° more each time about `fwd`, with 1.7 of his move on
/// the ground; then two embers (`0x3348c0`: 20000, `0x4f007fff` → `0x1f00007f`, 60..120 ticks, trailing) flying 7..10.5
/// a second 15..80° up a random way plus 8..12 a second along `fwd`, 0.05 a tick up.
fn burst(ctx: &mut Ctx, muzzle: V3, fwd: V3) {
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let h = &*ctx.hero;
    let (left, carry, disp) = (h.rows[1], if h.grounded_ticks != 0 { scale(h.disp, 1.7) } else { [0.0; 3] }, h.disp);
    let g = &mut ctx.game.rng;
    let tilted = if 1e-5 <= crate::hero::physics::len(left) { qturn(fwd, set_len(left, 1.0), 0.349_065_9) } else { fwd };
    let mut way = tilted;
    let base = add(muzzle, scale(fwd, 0.4));
    let mut roll = 0.0f32;
    for _ in 0..PELLETS {
        roll = add_rot(roll, g.randf(0.0, 0.628_318_6));
        let s = g.randf(0.2, 0.266_666_68);
        let vel = add(scale(way, s), carry);
        let p: V3 = std::array::from_fn(|k| base[k] + g.randf_signed(0.0, 0.3));
        let spin = g.randf_signed(1.0, 2.0) as i32;
        let size = g.randf(0.2, 0.75) * 210_000.0;
        if let Some(i) = crate::particles::puff(sys, g, 0.0, 1.0, 1.01, size, p, spin, disp, 0x7f20_2020, 0) {
            let r = &mut sys.pool.recs[i];
            r[0xa..0xc].copy_from_slice(&0x3ci16.to_le_bytes());
            r[3] = 0x44;
            r[9] = 4 + 0x40;
            r[0x3c..0x40].copy_from_slice(&5.0f32.to_le_bytes());
            r[0x24..0x28].copy_from_slice(&3u32.to_le_bytes());
            r[0x2a] = 0x30;
            r[0x2b] = r[0xa];
        }
        let p: V3 = std::array::from_fn(|k| base[k] + g.randf_signed(0.0, 0.6));
        let size = g.randf(0.2, 1.75) * 210_000.0;
        crate::particles::spark(sys, g, size, 500.0, 0.95, 0.0, 0.0, sub(p, vel), vel, 6, 0x7f, 0x40_4040, spin);
        if 1e-5 < roll.abs() { way = qturn(tilted, fwd, roll); }
    }
    let ahead = scale(fwd, g.randf(8.0, 12.0) * (1.0 / 60.0));
    for _ in 0..2 {
        let pitch = g.randf(0.261_799_42, 1.396_263_6);
        let yaw = g.rand_angle();
        let l = g.randf(7.0, 10.5);
        let mut v = add(crate::moby::classes::shot::polar(1.0 / 60.0 * 0.5 * l, yaw, pitch), ahead);
        v[2] += 0.05;
        let life = g.rand_range(0x3c, 0x78);
        crate::particles::ember(sys, g, 20_000.0, muzzle, v, 0x4f00_7fff, 0x1f00_007f, life, 1, None, None, 0);
    }
}

/// A tracer's look: the base Blitz's (`0x3fd6c8` / `0x3fd908`) or the upgraded one's (`0x3fe010` / `0x3fe250`,
/// `gp+0x1b78..0x1b98`).
pub struct TracerLook {
    /// Its two glows each tick: size and colour.
    pub glow: (f32, u32),
    /// The texture's scroll a frame (it wraps the other way for the upgraded one: down by 0.1, up 4 at 4 or less).
    pub scroll: f32,
    /// The beam's alpha scale; its half-widths at the end, the midpoint (and on); its colours at the end, the
    /// midpoint and the tail.
    pub alpha: f32,
    pub beam: [f32; 2],
    pub colours: [u32; 3],
    /// The glow over it (FX 0xe): half-widths at the end and the tail; colour (its alpha 64 · life), at the tail.
    pub over: [f32; 2],
    pub over_colours: [u32; 2],
}

/// The tracers' looks.
pub const LOOK: TracerLook = TracerLook { glow: (420_000.0, 0x20_2080), scroll: 0.1, alpha: 64.0, beam: [0.1, 0.1], colours: [0x10_8080, 0x10_1080, 0x10_8080], over: [0.6, 0.6], over_colours: [0x40ff, 0x40ff] };
pub const MEGA_LOOK: TracerLook = TracerLook { glow: (630_000.0, 0x80_2020), scroll: -0.1, alpha: 128.0, beam: [0.1, 0.4], colours: [0x80_8080, 0x80_8080, 0xff_1010], over: [0.3, 0.8], over_colours: [0xff_1010, 0xff_1010] };

fn look_of(o_class: i16) -> &'static TracerLook { if o_class == MEGA_TRACER { &MEGA_LOOK } else { &LOOK } }

/// `0x3fd6c8` (module docs); the upgraded Blitz's `0x3fe010`, the same with its [`MEGA_LOOK`].
pub fn update_tracer(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].pvars.resize(crate::moby::PVAR_BYTES, 0);
    if w.mobys[id].state == 0 {
        {
            let m = &mut w.mobys[id];
            let tail = add(m.pv_v(0x50), m.pv_v(0x20));
            m.set_pv_v(0x50, tail);
        }
        let mut life = w.mobys[id].pv_i(0x38);
        let out = countdown(&mut life);
        w.mobys[id].set_pv_i(0x38, life);
        if out {
            w.delete(id);
            return;
        }
        let spin = ctx.game.rng.randi(0xff);
        let pos = w.mobys[id].pos;
        if let Some(sys) = ctx.parts.as_deref_mut() {
            let def = sys.def_first(0x35);
            for s in [spin, (spin + 0x80) % 0xff] {
                let (size, rgb) = look_of(w.mobys[id].o_class).glow;
                if let Some(i) = crate::particles::sprite(sys, &mut ctx.game.rng, size, size, 0.0, pos, [0.0; 3], 2, rgb, s as u8, 0x40, 0x10, 0) { sys.pool.recs[i][2] = def; }
            }
        }
        w.register_draw(id);
    }
    let p = w.mobys[id].pos;
    if p.iter().any(|&c| !(2.0..=1021.0).contains(&c)) { w.delete(id); }
}

/// The FX quad's corners and colours as `0x2eb998` takes them.
pub(crate) fn quad(out: &mut Vec<crate::fx::FxQuad>, fx: usize, additive: bool, corners: [V3; 4], st: [[f32; 2]; 4], rgba: [u32; 4]) {
    out.push(crate::fx::FxQuad { fx, additive, corners, st, rgba });
}

pub(crate) const ST: [[f32; 2]; 4] = [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]];

/// `0x3a7968` (module docs): the starburst, the flash, the screen flash.
pub fn draw(w: &mut MobyWorld, id: usize, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let m = &mut w.mobys[id];
    let muzzle = m.pv_v(0x10);
    let up = m.pv_v(0x50);
    let to_cam = unit(sub(cam, muzzle));
    if m.pv_s(8) != 0 {
        let f = m.pv_s(8) as f32 * m.pv_f(0x20);
        let c = ((f * 255.0) as i32 as u32) << 24 | 0x20_40ff;
        let mut ray = qturn(scale(up, 4.0), to_cam, m.pv_f(0xc));
        for i in 0..8 {
            let side = set_len(cross_ba(to_cam, ray), 0.1);
            let (a, b) = (add(muzzle, side), sub(muzzle, side));
            let side = scale(side, 0.5 / 0.1);
            let tip = add(muzzle, ray);
            quad(out, 0xb, true, [a, b, add(tip, side), sub(tip, side)], [[0.5, 0.0], [0.5, 1.0], [1.0, 0.0], [1.0, 1.0]], [c; 4]);
            if i < 7 { ray = qturn(ray, to_cam, std::f32::consts::FRAC_PI_4); }
        }
    }
    let f = m.pv_s(0xa) as f32 * m.pv_f(0x24);
    m.set_pv_f(0x28, add_rot(m.pv_f(0x28), 2.0 * 0.017_453_294));
    m.set_pv_f(0x2c, add_rot(m.pv_f(0x2c), -2.5 * 0.017_453_294));
    let c = ((f * 20.0) as i32 as u32) << 24 | 0x80_80ff;
    let centre = add(muzzle, scale(m.pv_v(0x30), f * -16.0 + 16.0));
    let view = unit(sub(cam, centre));
    let across = cross_ba(cross_ba(view, up), view);
    let size = 20.0 + (1.5 - 20.0) * f;
    for (k, spin) in [m.pv_f(0x28), m.pv_f(0x2c)].into_iter().enumerate() {
        let d = qturn(across, view, spin);
        // The first quad's half-width set to the size, the second's scaled by it (`0x2ef138` / `0x2eeee0`).
        let d = if k == 0 { set_len(d, size) } else { scale(d, size) };
        let e = set_len(cross_ba(view, d), size);
        let (p, q) = (add(centre, d), sub(centre, d));
        quad(out, 0x38, true, [add(p, e), sub(p, e), add(q, e), sub(q, e)], ST, [c; 4]);
    }
    let n = m.pv_i(0x40);
    if n != 0 {
        let f = n as f32 * m.pv_f(0x44);
        w.screen_fills.push(((32.0 * f) as i32 as u32) << 24 | 0x80_8080);
    }
}

/// `0x3fd908`: the tracer from its end to its tail — two quads 0.1 wide across the view (FX 0x17, normal blend), the
/// first to the midpoint `0x108080` → `0x101080`, the second on to the tail fading to none, at its life's share of 64;
/// the texture scrolling 0.1 a frame (wrapping at 4); over them one quad 0.6 wide, additive (FX 0xe), `0x40ff` fading
/// to none at the tail (crate::moby::classes::beam::quads, the beam's shape; the upgraded Blitz's `0x3fe250` with its
/// [`MEGA_LOOK`]).
pub fn draw_tracer(w: &mut MobyWorld, id: usize, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let look = look_of(w.mobys[id].o_class);
    let m = &mut w.mobys[id];
    let f = m.pv_i(0x38) as f32 * m.pv_f(0x3c);
    let mut phase = m.pv_f(0x30);
    let (alpha, over) = (((f * look.alpha) as i32 as u32) << 24, ((f * 64.0) as i32 as u32) << 24);
    let widths = [look.beam[0], look.beam[1], look.over[0], look.over[1]];
    let colours = [look.colours[0], look.colours[1], look.colours[2], look.over_colours[0], look.over_colours[1]];
    super::beam::quads(out, m.pos, m.pv_v(0x50), cam, &mut phase, look.scroll, alpha, over, widths, colours);
    m.set_pv_f(0x30, phase);
}
