//! The Swingshot (item 0xd, crate::hero::items::ROWS; L03's record `0x267910`: attachment 1, stance 1, icon 0xea8b, no
//! ammo; class 0xd0 from `G/GADGET.WAD`): the hand moby **0xd0** (L03 `0x3a0a70`, L00 `0x39afd0`; called by the item
//! pass while it is the hand item) and its hook **0xd1** (L03 `0x3a1d00`, made by `0x3a1c20`; a class of the level's
//! own list). Ratchet's side — the node searches, the aim 0x23, the pull 0x24, the swing 0x25, the hook swing 0x2b and
//! the pull 0x8a / 0x8b — is crate::hero::swing; the hand fires the hook, follows it and sets the states that come next.
//!
//! **The hand** (pvars: `+0x14` the hook (its id + 1 here), `+0x18` the line's wave phase, `+0x1c` its period, `+0x20`
//! its height). Every tick: Ratchet in the look stance 1 goes to its scripted twin 0x1d (`0x2c7288(0x1d, 1)`); no hook,
//! one is made (`0x3a1c20`, its speed 0.8); in the look stance (1 / 0x1d) after its first two ticks the markers
//! (crate::hud::marker, FX 0x24): the red crosshair 0xff0000ff (1.0) at the screen's centre and a blue 0xffff0000
//! (1.4) on the Swingshot node in sight (`0x18a8c8` / `0x18a8c4`), else on the hook node found (`0x18a8f0`). Then by
//! state (`+0x20`); the hook's flights ([`fly`]) go at its speed (`+4` of its pvars), from the hand plus Ratchet's
//! carry this tick (`0x189f70`); the line ([`draw_line`]) is drawn from state 2 on:
//! * **0 / 1** (in the hand): its sequence over → 1 (sequence 1); the hook at its joint 0 with its rows. Ratchet in
//!   the aim 0x23 → the hook fired (the wave 0.28, period 1, phase 0; speed 0.8; sequence 3; class sound 0) → **2**; in
//!   the pull's aim 0x8a → the same, **0xf**; in the hook swing 0x2b → speed the distance to the hook node over 7 (5
//!   when the item has been out under 30 ticks), **9**.
//! * **2** (to the node, `0x18a8c4`): the line from 0.5 above Ratchet to the node (flags 2) clear, the hook flies
//!   (its speed + the grind's `0x18a464` in group 0xf; 0.8 from a jump or a fall (previous group 4 / 2) more than 2 up)
//!   and at the node **3**; blocked: Ratchet falls (6) more than 0.5 up, else idles (0, his idle sequence blended over
//!   13), **6**. **3**: class sound 1, **4**.
//! * **4**: the line clear, the wave dies (× 0.9 a tick, its phase 40° on) and below 0.168: with the hook within 3.5 of
//!   Ratchet he idles (0), **6**; the trigger let go he lets go early (`0x18a8e8`) and idles, **6**; else class sound 2,
//!   the pull 0x24 (the node's kind 0) or the swing 0x25, **5**. Blocked: idle (0) on the ground, else the fall 6, **6**.
//! * **5**: the hook held at the node while Ratchet is in 0x24 / 0x25, then **6**. **6**: the hook's speed 4/3, **7**:
//!   reeled in to the hand's joint 0, then **1** (sequence 1).
//! * **9** (to the hook node `0x18a8f4`): there, class sound 1, **0xb**: the wave dies (× 0.4) and below 0.168 Ratchet
//!   is attached (`0x18a8fc`), **0xc**: held there while he swings (0x2b), **0xd**: speed 4/3, **0xe**: reeled in
//!   (then 1); back in 0x2b on the way, speed a tenth of the distance, the wave flat, **9**. **0xa**: **0xb**.
//! * **0xf** (to the pull target `0x18a930`): as 9 with the line test of 2 (blocked: the fall 6 above 0.5, else idle;
//!   **0x13**), **0x11**: the wave dies, then the target hooked (`0x18a93c`), **0x12**: held there while Ratchet is in
//!   the group 0x21, **0x13**: speed 4/3, **0x14**: reeled in; back in the group 0x21, **0xf** again. **0x10**: **0x11**.
//!
//! In every state past 1 its sequence ending plays 4 (blend 10).
//!
//! **The hook** (pvars: `+0x00` the hand, `+0x04` its speed): made at the hand's joint 0 with its rows, update and draw
//! distance 0xff, drawn, its colour 0x80 (`0x314718`), sequence 0, the matrix rebuilt (`0x315188`), mode | 4. Each tick
//! it goes when its hand is gone, not class 0xd0 or no longer has it; hidden (mode | 0x41) while the hand is or the hand
//! item is tucked away (`0x18c0df`). Its sequences follow the hand: 0 → 1 at its end; 1 → 3 as the hand fires (2), → 2
//! at its state 8 (none sets it), each with its speed 1; 2 ends → deleted; 3 → 4 at its end; 4 → 5 while the hand is
//! out at the node (3..5); 5 stops at its end (speed 0), → 7 (speed 1) as the hand reels it in (6 / 7); 7 and the rest
//! → 1. Then its matrix from its rows as they are (`0x315328`; the port's rebuild with mode 0x100).
//!
//! Not ported: the grind's Swingshot (group 0xf with `0x18a4a4`: the rail's nodes; the grind has no such target here),
//! the stats counters.

use super::props::{set_seq, wrapped};
use crate::hero::physics::{add, cross, dist, len, set_len, sub, V3};
use crate::moby::{mode, Ctx, MobyWorld};

/// The hand moby's class (the item record's `+0x04`) and the hook's (`0x3a1c20`).
pub const HAND_CLASS: i16 = 0xd0;
pub const HOOK_CLASS: i16 = 0xd1;

/// The hand's pvars: the hook (id + 1), the wave's phase, period and height.
const HOOK: usize = 0x14;
const PHASE: usize = 0x18;
const PERIOD: usize = 0x1c;
const WAVE: usize = 0x20;
/// The hook's pvars: the hand (id + 1), its speed.
const HAND: usize = 0;
const SPEED: usize = 4;

/// `0x3f4cccce`: the hook's speed at the fire.
const FIRE_SPEED: f32 = 0.800_000_07;
/// `0x3faaaaab`: its speed reeled in.
const REEL_SPEED: f32 = 1.333_333_4;
/// `0x3e8f5c29`: the wave's height at the fire.
const WAVE_HEIGHT: f32 = 0.28;
/// The wave's phase step (40°) and the height under which it is over.
const PHASE_STEP: f32 = 0.698_131_8;
const WAVE_END: f32 = 0.168_000_01;
/// The markers' FX texture.
const MARKER: u16 = 0x24;

/// `0x2f0ef0` (L00 `0x2efef0`): `a` wrapped to [−π, π) by its fraction of a turn.
fn wrap(a: f32) -> f32 {
    let f = a * 0.159_154_94 + 0.5;
    ((f - f.floor()) - 0.5) * std::f32::consts::TAU
}

/// The hand's hook (`+0x14`), while it lives.
fn hook_of(w: &MobyWorld, id: usize) -> Option<usize> {
    usize::try_from(w.mobys[id].pv_i(HOOK) - 1).ok().filter(|&h| w.mobys.get(h).is_some_and(|m| m.o_class == HOOK_CLASS && m.state < 0xfd))
}

/// `0x3a1c20(speed, hand)`: the hook (module docs); None when the level has no class 0xd1.
fn make_hook(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> Option<usize> {
    w.class(HOOK_CLASS)?;
    let at = ctx.host.joint(&w.mobys[id], 0).unwrap_or(w.mobys[id].pos);
    let rows = w.mobys[id].rows;
    let h = w.create(HOOK_CLASS);
    {
        let m = &mut w.mobys[h];
        m.set_pv_i(HAND, id as i32 + 1);
        m.set_pv_f(SPEED, FIRE_SPEED);
        m.draw_dist = 0xff;
        m.update_dist = 0xff;
        m.drawn = 1;
        m.ambient = [0x80; 3];
        m.pos = at;
        m.rows = rows;
    }
    set_seq(w, h, ctx, 0, 0, 10);
    w.rebuild(h, &*ctx.host);
    let m = &mut w.mobys[h];
    m.state = 0;
    // Mode 4; and from here its rebuild keeps the rows (`0x315328`).
    m.mode |= mode::NO_MATRIX | mode::KEEP_ROWS;
    Some(h)
}

/// `+0x60 & 2` and not sequence 4: sequence 4 (blend 10), as every state past 1 does.
fn out_seq(w: &MobyWorld, id: usize, ctx: &mut Ctx) {
    if wrapped(w, id, ctx) { set_seq(w, id, ctx, 4, 0, 10); }
}

/// The wave's dying tick: its height × `k`, its phase 40° on; true while it lasts.
fn wave_tick(w: &mut MobyWorld, id: usize, k: f32) -> bool {
    let m = &mut w.mobys[id];
    let a = m.pv_f(WAVE) * k;
    m.set_pv_f(WAVE, a);
    m.set_pv_f(PHASE, m.pv_f(PHASE) + PHASE_STEP);
    WAVE_END <= a
}

/// The hook moved toward `to` by `step`; true when it got there (it is put there).
fn fly(w: &mut MobyWorld, hook: usize, to: V3, step: f32) -> bool {
    let p = w.mobys[hook].pos;
    if dist(p, to) <= step {
        w.mobys[hook].pos = to;
        return true;
    }
    w.mobys[hook].pos = add(p, set_len(sub(to, p), step));
    false
}

/// The fire: the wave 0.28 high, period 1, phase 0; the hook's speed `speed`; sequence 3; class sound 0.
fn fire(w: &mut MobyWorld, id: usize, hook: usize, ctx: &mut Ctx, speed: f32) {
    {
        let m = &mut w.mobys[id];
        m.set_pv_f(WAVE, WAVE_HEIGHT);
        m.set_pv_f(PERIOD, 1.0);
        m.set_pv_f(PHASE, 0.0);
    }
    w.mobys[hook].set_pv_f(SPEED, speed);
    set_seq(w, id, ctx, 3, 0, 10);
}

/// `0x2c7288(0, 0)` and `0x2d3088(13, 0x2ab6c8(0), 0)`: Ratchet idles, his idle sequence blended over 13.
fn idle(ctx: &mut Ctx) {
    ctx.hero.request_state(0, false);
    let s = ctx.hero.idle_seq();
    ctx.hero.anim_req = Some((13.0, s, 0));
}

/// The line from 0.5 above Ratchet to `to` is clear (`0x2df4f8(a, b, 2, 0, 0)`).
fn clear(w: &MobyWorld, ctx: &Ctx, to: V3) -> bool {
    let p = ctx.hero.pos;
    w.coll_line(ctx, [p[0], p[1], p[2] + 0.5], to, 2, None).is_none()
}

/// A target's place as it is now (its moby), else as the search saw it.
fn place(w: &MobyWorld, id: Option<usize>, seen: V3) -> V3 { id.and_then(|i| w.mobys.get(i)).map_or(seen, |m| m.pos) }

/// The reel-in (7 / 0xe / 0x14): toward the hand's joint 0 at `step`; there, sequence 1 and **1**. Returns whether it
/// arrived.
fn reel(w: &mut MobyWorld, id: usize, hook: usize, ctx: &mut Ctx, step: f32) -> bool {
    let j = ctx.host.joint(&w.mobys[id], 0).unwrap_or(w.mobys[id].pos);
    if fly(w, hook, j, step) {
        set_seq(w, id, ctx, 1, 0, 10);
        w.mobys[id].state = 1;
        true
    } else {
        w.register_draw_first(id);
        false
    }
}

/// 0xd0 (L03 `0x3a0a70`): the hand (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if ctx.hero.state == 1 { ctx.hero.request_state(0x1d, true); }
    let mut hook = hook_of(w, id);
    if hook.is_none() && w.mobys[id].pv_i(HOOK) == 0 {
        hook = make_hook(w, id, ctx);
        w.mobys[id].set_pv_i(HOOK, hook.map_or(0, |h| h as i32 + 1));
    }
    let hs = ctx.hero.state;
    let sw = ctx.hero.swing;
    if (hs == 1 || hs == 0x1d) && 2 <= ctx.hero.timer {
        crate::hud::marker(&mut ctx.game.markers, 1.0, id, 0xff00_00ff, None, MARKER);
        let at = if !sw.found || sw.node.is_none() { sw.hook.map(|n| place(w, Some(n.id), n.pos)) } else { Some(place(w, sw.node, sw.node_pos)) };
        if let Some(at) = at { crate::hud::marker(&mut ctx.game.markers, 1.4, id, 0xffff_0000, Some(at), MARKER); }
    }
    let state = w.mobys[id].state;
    let carry = len(ctx.hero.carry);
    let node = place(w, sw.node, sw.node_pos);
    let hook_node = sw.hook.map(|n| place(w, Some(n.id), n.pos));
    let pull = sw.pull.map(|n| place(w, Some(n.id), n.pos));
    if state <= 1 {
        if wrapped(w, id, ctx) {
            set_seq(w, id, ctx, 1, 0, 10);
            w.mobys[id].state = 1;
        }
        let Some(h) = hook else { return };
        let j = ctx.host.joint(&w.mobys[id], 0).unwrap_or(w.mobys[id].pos);
        let rows = w.mobys[id].rows;
        (w.mobys[h].pos, w.mobys[h].rows) = (j, rows);
        if hs == 0x23 || hs == 0x8a {
            fire(w, id, h, ctx, FIRE_SPEED);
            ctx.host.play_sound(&w.mobys[id], 0, 0);
            w.mobys[id].state = if hs == 0x23 { 2 } else { 0xf };
        } else if hs == 0x2b {
            let Some(to) = hook_node else { return };
            let k = if ctx.hero.items.item_ticks < 0x1e { 5.0 } else { 7.0 };
            let s = dist(to, w.mobys[h].pos) / k;
            fire(w, id, h, ctx, s);
            ctx.host.play_sound(&w.mobys[id], 0, 0);
            w.mobys[id].state = 9;
        }
        return;
    }
    let Some(h) = hook else { return };
    let speed = w.mobys[h].pv_f(SPEED);
    match state {
        2 => {
            w.register_draw_first(id);
            out_seq(w, id, ctx);
            let mut step = speed + carry;
            if ctx.hero.group == 0xf {
                step += ctx.hero.grind.speed;
            } else if matches!(ctx.hero.prev_group, 4 | 2) && 2.0 < ctx.hero.height {
                step = 0.800_000_1;
            }
            if clear(w, ctx, node) {
                if fly(w, h, node, step) { w.mobys[id].state = 3; }
                return;
            }
            if 0.5 < ctx.hero.height { ctx.hero.request_state(6, true) } else { idle(ctx) }
            w.mobys[id].state = 6;
        }
        3 => {
            ctx.host.play_sound(&w.mobys[id], 1, 0);
            w.register_draw_first(id);
            w.mobys[h].pos = node;
            w.mobys[id].state = 4;
        }
        4 => {
            if clear(w, ctx, node) {
                out_seq(w, id, ctx);
                w.register_draw_first(id);
                if wave_tick(w, id, 0.9) { return; }
                if dist(w.mobys[h].pos, ctx.hero.pos) <= 3.5 {
                    ctx.hero.request_state(0, true);
                } else if ctx.hero.items.held & ctx.hero.items.trigger == 0 {
                    ctx.hero.swing.early = true;
                    idle(ctx);
                } else {
                    ctx.host.play_sound(&w.mobys[id], 2, 0);
                    ctx.hero.request_state(if sw.node_kind == 0.0 { 0x24 } else { 0x25 }, true);
                    w.mobys[id].state = 5;
                    return;
                }
            } else {
                ctx.hero.request_state(if ctx.hero.air_ticks == 0 { 0 } else { 6 }, true);
            }
            w.mobys[id].state = 6;
        }
        5 => {
            w.mobys[h].pos = node;
            out_seq(w, id, ctx);
            wave_tick(w, id, 0.9);
            w.register_draw_first(id);
            if !(0x24..=0x25).contains(&hs) { w.mobys[id].state = 6; }
        }
        6 | 0xd | 0x13 => {
            if state != 6 { out_seq(w, id, ctx); }
            w.register_draw_first(id);
            w.mobys[h].set_pv_f(SPEED, REEL_SPEED);
            w.mobys[id].state = state + 1;
        }
        7 => {
            reel(w, id, h, ctx, speed + carry);
        }
        9 | 0xf => {
            w.register_draw_first(id);
            out_seq(w, id, ctx);
            let to = if state == 9 { hook_node } else { pull };
            let Some(to) = to else { return };
            if state == 0xf && !clear(w, ctx, to) {
                if 0.5 < ctx.hero.height { ctx.hero.request_state(6, true) } else { idle(ctx) }
                w.mobys[id].state = 0x13;
                return;
            }
            if fly(w, h, to, speed) {
                w.mobys[id].state = state + 2;
                ctx.host.play_sound(&w.mobys[id], 1, 0);
            }
        }
        0xa | 0x10 => {
            w.register_draw_first(id);
            w.mobys[id].state = state + 1;
        }
        0xb | 0x11 => {
            out_seq(w, id, ctx);
            w.register_draw_first(id);
            if wave_tick(w, id, 0.399_999_98) { return; }
            if state == 0xb { ctx.hero.swing.attached = true } else { ctx.hero.swing.hooked = true }
            w.mobys[id].state = state + 1;
        }
        0xc | 0x12 => {
            let to = if state == 0xc { hook_node } else { pull };
            if let Some(to) = to { w.mobys[h].pos = to; }
            out_seq(w, id, ctx);
            wave_tick(w, id, 0.399_999_98);
            w.register_draw_first(id);
            let stay = if state == 0xc { hs == 0x2b } else { ctx.hero.group == 0x21 };
            if !stay { w.mobys[id].state = state + 1; }
        }
        0xe | 0x14 => {
            out_seq(w, id, ctx);
            reel(w, id, h, ctx, speed);
            let (back, to) = if state == 0xe { (hs == 0x2b, hook_node) } else { (ctx.hero.group == 0x21, pull) };
            if !back { return; }
            let Some(to) = to else { return };
            let s = dist(to, w.mobys[h].pos) * 0.1;
            {
                let m = &mut w.mobys[id];
                m.set_pv_f(PERIOD, 1.0);
                m.set_pv_f(PHASE, 0.0);
                m.set_pv_f(WAVE, 0.0);
            }
            w.mobys[h].set_pv_f(SPEED, s);
            set_seq(w, id, ctx, 3, 0, 10);
            w.mobys[id].state = if state == 0xe { 9 } else { 0xf };
        }
        _ => {}
    }
}

/// 0xd1 (L03 `0x3a1d00`): the hook (module docs).
pub fn update_hook(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let hand = usize::try_from(w.mobys[id].pv_i(HAND) - 1).ok().filter(|&h| {
        w.mobys.get(h).is_some_and(|m| m.o_class == HAND_CLASS && m.state != 0xfe && m.state != 0xfd && m.pv_i(HOOK) == id as i32 + 1)
    });
    let Some(hand) = hand else {
        w.delete(id);
        return;
    };
    {
        let hidden = w.mobys[hand].mode & 1 != 0 || ctx.hero.items.tucked;
        let m = &mut w.mobys[id];
        if hidden { m.mode |= 0x41 } else { m.mode &= 0xffbe }
    }
    let hs = w.mobys[hand].state;
    fn to(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, s: u8) {
        set_seq(w, id, ctx, s, 0, 10);
        w.mobys[id].state = s;
    }
    match w.mobys[id].state {
        0 => {
            if wrapped(w, id, ctx) {
                to(w, id, ctx, 1);
                ctx.host.set_anim_speed(&w.mobys[id], 1.0);
            }
        }
        1 => {
            let s = match hs { 2 => 3, 8 => 2, _ => 0 };
            if s != 0 {
                to(w, id, ctx, s);
                ctx.host.set_anim_speed(&w.mobys[id], 1.0);
            }
        }
        2 => {
            if wrapped(w, id, ctx) {
                w.delete(id);
                return;
            }
        }
        3 => {
            if wrapped(w, id, ctx) { to(w, id, ctx, 4); }
        }
        4 => {
            if (3..=5).contains(&hs) { to(w, id, ctx, 5); }
        }
        5 => {
            if wrapped(w, id, ctx) { ctx.host.set_anim_speed(&w.mobys[id], 0.0); }
            if (6..=7).contains(&hs) {
                ctx.host.set_anim_speed(&w.mobys[id], 1.0);
                to(w, id, ctx, 7);
            }
        }
        6 => {}
        7 => {
            if wrapped(w, id, ctx) { to(w, id, ctx, 1); }
        }
        _ => to(w, id, ctx, 1),
    }
    w.rebuild(id, &*ctx.host);
}

/// L03 `0x3a06d0` (L00 `0x39ac30`, on the frame's first list): the line from the hook to the hand's joint 0, in
/// segments of at most 0.2 (FX 0xf, grey 0x80808080, ALPHA 0x44; s along it, t across), each 0.05 to either side
/// across the view (`(camera − point) × along`) and waved along `along × (along with z − 0.1)` by
/// `sin(π·t / length) · sin(π·t / period) · cos(phase) · height` (`+0x1c`, `+0x18`, `+0x20`); its first edge 0.05
/// below and above the hook.
pub fn draw_line(w: &MobyWorld, host: &dyn crate::moby::MobyHost, id: usize, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let Some(hook) = hook_of(w, id) else { return };
    let m = &w.mobys[id];
    let a = host.joint(m, 0).unwrap_or(m.pos);
    let h = w.mobys[hook].pos;
    let along = sub(a, h);
    let l = len(along);
    let bend = cross(along, [along[0], along[1], along[2] - 0.1]);
    let wave = wrap(m.pv_f(PHASE)).cos();
    let (period, height) = (m.pv_f(PERIOD), m.pv_f(WAVE));
    let (mut p, mut prev) = (h, ([h[0], h[1], h[2] - 0.05], [h[0], h[1], h[2] + 0.05]));
    let st = [[0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [1.0, 1.0]];
    let mut t = 0.0;
    let mut rest = l;
    while 0.0 < rest {
        let step = rest.min(0.2);
        t += step;
        let k = (t * std::f32::consts::PI / l).sin() * wrap(t * std::f32::consts::PI / period).sin() * wave;
        p = add(p, set_len(along, step));
        let side = set_len(cross(sub(cam, p), along), 0.05);
        let off = set_len(bend, k * height);
        let (a1, b1) = (add(add(p, side), off), add(sub(p, side), off));
        out.push(crate::fx::FxQuad { fx: 0xf, additive: false, corners: [prev.0, a1, prev.1, b1], st, rgba: [0x8080_8080; 4] });
        prev = (a1, b1);
        if l <= t { break; }
        rest = l - t;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Host;
    impl crate::moby::MobyHost for Host {}

    /// A flat line (no wave): one quad per 0.2 from the hook to the hand, the last ending 0.05 across the view at the
    /// hand.
    #[test]
    fn the_line_runs_from_the_hook_to_the_hand_in_steps_of_a_fifth() {
        let mut w = MobyWorld::new(Vec::new());
        let hand = w.create(HAND_CLASS);
        let hook = w.create(HOOK_CLASS);
        w.mobys[hand].pos = [1.0, 0.0, 0.0];
        w.mobys[hand].set_pv_i(HOOK, hook as i32 + 1);
        w.mobys[hand].set_pv_f(PERIOD, 1.0);
        w.mobys[hook].pos = [0.0, 0.0, 0.0];
        let mut out = Vec::new();
        draw_line(&w, &Host, hand, [0.5, 0.0, 10.0], &mut out);
        // Five steps of 0.2 (a sixth of rounding at most, as the game's own sum of steps leaves).
        assert!((5..=6).contains(&out.len()), "{}", out.len());
        let last = out[out.len() - 1].corners;
        for c in [last[1], last[3]] {
            assert!((c[0] - 1.0).abs() < 1e-5 && (c[1].abs() - 0.05).abs() < 1e-5 && c[2].abs() < 1e-5, "{c:?}");
        }
        assert_eq!(out[0].corners[0], [0.0, 0.0, -0.05]);
    }
}
