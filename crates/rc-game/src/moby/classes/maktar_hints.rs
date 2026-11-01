//! Maktar's hint manager, **0x0c24** (L02 `0x40b4c0`, one placed): the level's help texts by where Ratchet is and
//! what he has, read from its pvars (`+0x68`, words `[k]`): the zones (cuboids) `[0]`, `[4]` / `[5]`, `[8]` / `[9]`,
//! `[0xa]`, `[0xc]` / `[0xd]`, `[0xe]`, `[0x11]`, `[0x13]`, `[0x1e]`, `[0x1f]`, `[0x20]`, `[0x27]`, the cuboid `[0x21]`,
//! the mobys `[0x1a]` (the arena master 0x0b59), `[0x1b]` and `[0x22]`, its timers `[0x24..0x26]`. Its first tick:
//! updated at any distance, `[0x14]`, `[0x18]`, `[0x19]`, `[0x24]` 0, `[0x26]` 600, `[0x25]` 0x4b0, `[0xb]` 0xb4 (1).
//! Then every tick, in the game's order:
//! * 0xa3c (slot 0x93, the missions on the map), queued (`0x2fa860`) once (`0x13962d` bit 4) in `[0x27]` with mission
//!   2 done and mission 0 not (`0x315368`: the missions' states, 2 read here as done);
//! * 0x9bd (0x14, the Tractor Beam's use) in `[0]` while Ratchet lacks the Tractor Beam (item 0x2e), the first time
//!   only marked shown, again past 0x438 ticks of its last showing; off for good once he has it;
//! * 0x9c5 (0x1c, the gravity boots) with the sphere's frames on (`0x1a8ff0`, crate::gravity) and the panel idle:
//!   never shown yet, or after 0xe10 ticks in a row (`+2` counting, `+4` the stay's start);
//! * the zones' hints (`0x2fa5c0`, crate::moby::classes::oozla::zone_help): 0x9bf (0x16, kind 2) in `[4]`, off in
//!   `[5]`; 0x9c2 (0x19, kind 1) in `[0xc]`, off in `[0xd]`; 0x9c4 (0x1b, kind 0) in `[0x11]`; with the Tractor Beam
//!   0x9f1 (0x48, kind 2) in `[0x1f]`;
//! * 0x9c1 (0x18) in `[8]` (not in `[9]`, which turns it off for good), past 0x438 ticks of its last showing;
//! * 0x9c3 (0x1a, the Electrolyzer) once the arena is on (`0x1a9010`, `[0xf]` kept) in `[0xe]`; off once Ratchet has
//!   the Electrolyzer (item 0x26);
//! * in the arena (the master `[0x1a]` in its state 8 with chunk 1 in, `0x402410`): 0x9c9 (0x20) queued once a level
//!   (`0x13962e`) after `[0x25]` ticks; 0x9e3 (0x3a, strafing) after `[0x26]` ticks once `[0x1b]` is past its state 2,
//!   till shown;
//! * with the frames on: 0x9c6 (0x1d) queued in `[0x13]` while unshown, or every `[0x24]` ticks (0x34bc0: an hour);
//! * 0x9e3 (0x3a) in `[0x1e]` till shown, unless Ratchet has been in `[0xe]` (`0x139634`);
//! * in the first three frames, with the Electrolyzer, Ratchet in `[0x20]` and `[0x22]` not hooked: Ratchet put at the
//!   cuboid `[0x21]` (`0x2bd368`);
//! * 0xa2b (0x82) in `[0xa]`, past 0x78 ticks of its last showing.
//!
//! Not ported: the stats some ask (`0x18c918` / `0x18c91c`, `0x18c7a4`, `0x18c840` / `0x18c844`, `0x18c90c` /
//! `0x18c910`, `0x18c8f8`: G-HERO-005, read as none — each of those hints shows as to a player who never did the
//! thing), the start's ship-mode byte `0x152c26` = 1 with the move to `[0x21]`.

use crate::moby::{countdown, Ctx, MobyWorld};

/// The Tractor Beam (item 0x2e) and the Electrolyzer (item 0x26).
const TRACTOR: usize = 0x2e;
const ELECTROLYZER: usize = 0x26;

/// 0x0c24 (`0x40b4c0`): see the module.
pub fn update_0c24(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let pv = |w: &MobyWorld, k: usize| w.mobys[id].pv_i(4 * k);
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.update_dist = 0xff;
            for k in [0x18, 0x19, 0x14, 0x24] { m.set_pv_i(4 * k, 0); }
            m.set_pv_i(4 * 0x26, 600);
            m.set_pv_i(4 * 0x25, 0x4b0);
            m.set_pv_i(4 * 0xb, 0xb4);
            m.set_state(1, -1);
            return;
        }
        1 => {}
        _ => return,
    }
    let inside = |ctx: &Ctx, z: i32| z != -1 && ctx.level.in_cuboid(z, ctx.hero.pos);
    let have = |ctx: &Ctx, item: usize| ctx.game.items.get(item).is_some_and(|&b| b != 0);
    let done = |ctx: &Ctx, k: usize| ctx.game.frame > 4 && ctx.game.missions.get(ctx.game.level.max(0) as usize * 0x10 + k).is_some_and(|&b| b & 0x80 != 0);
    let level_bit = 1u32.checked_shl(ctx.game.level.max(0) as u32).unwrap_or(0);
    let now = ctx.game.play_time;
    // The missions on the map.
    if done(ctx, 2) && !done(ctx, 0) && ctx.game.hint_flags[0] & 4 == 0 && inside(ctx, pv(w, 0x27)) {
        ctx.game.hint_flags[0] |= 4;
        ctx.game.help_later(0xa3c, 0x93);
    }
    // The Tractor Beam's use (slot 0x14, the kind 2 zone hint written out).
    if ctx.game.help_counts[0x14] != -1 {
        if !have(ctx, TRACTOR) {
            if inside(ctx, pv(w, 0)) {
                let g = &mut *ctx.game;
                let last = g.help_last[0x14];
                if g.help_counts[0x14] == 0 {
                    g.help_counts[0x14] = 1;
                    if last < now { g.help_last[0x14] = now; }
                    g.help_levels[0x14] |= level_bit | 0x8000_0000;
                } else if (now.wrapping_sub(last) as u32) < 0x439 && last != 0 {
                    if last < now { g.help_last[0x14] = now; }
                } else {
                    g.help(0x9bd, 0x14);
                }
            }
        } else {
            ctx.game.help_counts[0x14] = -1;
        }
    }
    // The gravity boots, with the sphere's frames on and the panel idle.
    if ctx.hero.gravity.on && ctx.game.help_state == 0 && ctx.game.help_req.is_none() {
        let g = &mut *ctx.game;
        if (g.help_levels[0x1c] as i32) >= 0 { g.help(0x9c5, 0x1c); }
        let (last, stay) = (g.help_last[0x1c], g.help_aux[0x1c] as u16);
        if (last as u32).wrapping_add(stay as u32) < (now as u32).wrapping_sub(2) {
            if last < now { g.help_last[0x1c] = now; }
            g.help_aux[0x1c] = 0;
        } else {
            let stay = stay.wrapping_add(1);
            g.help_aux[0x1c] = stay as i16;
            if 0xe0f < stay {
                g.help(0x9c5, 0x1c);
                g.help_aux[0x1c] = 0;
            }
        }
    }
    super::oozla::zone_help(ctx, 0x9bf, 0x16, 2, pv(w, 4), pv(w, 5));
    // Slot 0x18: in `[8]`, not in `[9]` (which turns it off).
    if ctx.game.help_counts[0x18] != -1 {
        if !inside(ctx, pv(w, 9)) {
            if inside(ctx, pv(w, 8)) {
                let g = &mut *ctx.game;
                let last = g.help_last[0x18];
                if 0x438 < now.wrapping_sub(last) as u32 || last == 0 {
                    g.help(0x9c1, 0x18);
                } else if last < now {
                    g.help_last[0x18] = now;
                }
            }
        } else {
            ctx.game.help_counts[0x18] = -1;
        }
    }
    super::oozla::zone_help(ctx, 0x9c2, 0x19, 1, pv(w, 0xc), pv(w, 0xd));
    // The Electrolyzer, once the arena is on.
    if ctx.game.help_counts[0x1a] == 0 {
        if pv(w, 0xf) == 0 && ctx.game.arena_on { w.mobys[id].set_pv_i(4 * 0xf, 1); }
        if have(ctx, ELECTROLYZER) { ctx.game.help_counts[0x1a] = -1; }
        if pv(w, 0xf) != 0 && inside(ctx, pv(w, 0xe)) { ctx.game.help(0x9c3, 0x1a); }
    }
    super::oozla::zone_help(ctx, 0x9c4, 0x1b, 0, pv(w, 0x11), -1);
    // The arena: the master in its state 8 with chunk 1 in (`0x402410`).
    let moby = |w: &MobyWorld, k: usize| usize::try_from(pv(w, k)).ok().filter(|&i| i < w.mobys.len());
    let master = moby(w, 0x1a).is_some_and(|i| w.mobys[i].o_class == 0x0b59 && ctx.game.variant == 1 && w.mobys[i].state == 8);
    if master {
        if tick_down(w, id, 0x25) && (ctx.game.hint_flags[3] as u32 >> ctx.game.level.clamp(0, 31)) & 1 == 0 {
            ctx.game.help_later(0x9c9, 0x20);
            ctx.game.hint_flags[3] |= level_bit as u8;
        }
        // `0x3d8118`: past its state 2.
        if moby(w, 0x1b).is_some_and(|i| 2 < w.mobys[i].state) && tick_down(w, id, 0x26) && (ctx.game.help_levels[0x3a] as i32) >= 0 {
            ctx.game.help(0x9e3, 0x3a);
        }
    }
    // The gravity boots again: in `[0x13]` while unshown, or every `[0x24]` ticks.
    if ctx.hero.gravity.on {
        if inside(ctx, pv(w, 0x13)) && ctx.game.help_counts[0x1d] == 0 {
            ctx.game.help_later(0x9c6, 0x1d);
            w.mobys[id].set_pv_i(4 * 0x24, 0x34bc0);
        }
        if tick_down(w, id, 0x24) {
            ctx.game.help_later(0x9c6, 0x1d);
            w.mobys[id].set_pv_i(4 * 0x24, 0x34bc0);
        }
        // `now − 0x18c8f8 < 0x3c` (the stat, read as none: 0).
        if now < 0x3c { w.mobys[id].set_pv_i(4 * 0x24, 0x34bc0); }
    }
    // Strafing, till shown, unless `[0xe]` was reached.
    if inside(ctx, pv(w, 0x1e)) && ctx.game.hint_flags[4] == 0 && (ctx.game.help_levels[0x3a] as i32) >= 0 {
        ctx.game.help(0x9e3, 0x3a);
    }
    if inside(ctx, pv(w, 0xe)) { ctx.game.hint_flags[4] = 1; }
    if have(ctx, TRACTOR) { super::oozla::zone_help(ctx, 0x9f1, 0x48, 2, pv(w, 0x1f), -1); }
    // The start: with the Electrolyzer, out of `[0x20]` to `[0x21]` unless `[0x22]` is hooked.
    if ctx.game.frame <= 2 && have(ctx, ELECTROLYZER) {
        let hooked = moby(w, 0x22).is_some_and(|i| w.mobys[i].once & 4 != 0);
        if !hooked && inside(ctx, pv(w, 0x20)) {
            if let Some((p, r)) = ctx.level.cuboid_place(pv(w, 0x21)) { ctx.hero.teleport(p, r); }
        }
    }
    if inside(ctx, pv(w, 0xa)) {
        let g = &mut *ctx.game;
        let last = g.help_last[0x82];
        if (now.wrapping_sub(last) as u32) < 0x79 && last != 0 {
            if last < now { g.help_last[0x82] = now; }
        } else {
            g.help(0xa2b, 0x82);
        }
    }
}

/// `0x2f2998(&[k])`: the pvar word `[k]` counted down; true when it was 0 or reaches it.
fn tick_down(w: &mut MobyWorld, id: usize, k: usize) -> bool {
    let m = &mut w.mobys[id];
    let mut t = m.pv_i(4 * k);
    let r = countdown(&mut t);
    m.set_pv_i(4 * k, t);
    r
}
