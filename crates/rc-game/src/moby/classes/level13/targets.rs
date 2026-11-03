//! Level 13's breakable targets on the enemy framework's health (L13 addresses; the engine functions named by their
//! L13 addresses with the L00 ones the port knows them by). Both keep the framework's blocks: the health at `+0x20`
//! (`[0]` = 0x20; 20 for 0x0c86, 60 for 0x0c87), the reaction at `+0x50` (`[0x10]` = 0x50), the pieces' thresholds at
//! `+0x100` (`[0x1c]`).
//!
//! * **0x0c86** ([`update_0c86`], `0x3ee240`, 26 placed): any hit hurts it; it blows up at no health.
//! * **0x0c87** ([`update_0c87`], `0x3ee3a0`, 14 placed): takes the hits of flag 1 and half the flag-0x10000 ones; it
//!   blows up at no health.
//!
//! The engine functions: `0x330d20` = L00 `0x327e38` (crate::moby::ai::take_damage), `0x32a460` = L00 `0x3201e8`
//! (crate::moby::ai::flash), `0x327fc0` = L00 `0x31dd48` (crate::moby::ai::damage_of), `0x327f60` = L00 `0x31dce8`
//! (the hit with flags), `0x2f91c0` the countdown, `0x3fa180` = L00 `0x3f8348` (crate::moby::blast::burst),
//! `0x36a110` the class sound.
//!
//! Not ported: the class list `0x2323d0` each adds its class to at the start (`0x38ad28`; nothing the port has reads
//! it); the damage pieces (`0x3331c8` on the block `+0x100`: its pieces shown by health thresholds through `+0x7c`, then
//! thrown off as balls with their own update `0x3a6130` and the level sound 7 as the health drops, G-ENM-006).

use crate::hero::physics::{add, scale, V3};
use crate::moby::{ai, Ctx, MobyWorld};

/// The health and reaction blocks (`+0x20`, `+0x50`).
const HEALTH: usize = 0x20;
const REACT: usize = 0x50;

/// The blast both give at no health: `0x3fa180(0.75, 20, at, 0, 0x19, 0x14, 0x14, 0x14, 0x2d, 1, 1, …)` at its place 2
/// down its +y row and 1.5 along its +x row; deleted.
fn blow_up(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let at: V3 = add(add(m.pos, scale(m.rows[1], -2.0)), scale(m.rows[0], 1.5));
    crate::moby::blast::burst(w, ctx, at, &crate::moby::blast::Blast { size: 0.75, bursts: [0x19, 0x14, 0x14], puffs: 0x14, streaks: 0x2d, light: true, shake: false });
    w.delete(id);
}

/// 0x0c86 (`0x3ee240`). **0**: 1. **Each tick**: its reaction set so any hit hurts (`+0x62` / `+0x61` 0xff, `+0x6a`
/// 2, the hurt's share `+0xa8` 0.01); the frame's damage (`0x330d20(m, +0x50, 0)`, crate::moby::ai::take_damage):
/// a kind, its class sound 0; no health left: [`blow_up`].
pub fn update_0c86(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    if m.state == 0 { m.state = 1; }
    m.set_pv_b(REACT + 0x12, 0xff);
    m.set_pv_b(REACT + 0x1a, 2);
    m.set_pv_f(REACT + 0x58, f32::from_bits(0x3c23_d70a));
    m.set_pv_b(REACT + 0x11, 0xff);
    if ai::take_damage(w, id, ctx, false) != 0 { ctx.host.play_sound(&w.mobys[id], 0, 0); }
    if w.mobys[id].pv_f(HEALTH) <= 0.0 { blow_up(w, id, ctx); }
}

/// 0x0c87 (`0x3ee3a0`). **0**: 1, and on as 1. **1**: its flash's tick (`0x32a460`); a hit of flag 1 (`0x327f60`)
/// unless its health's wait (`+0x26`, counted down every tick) runs, its damage × `+0xf8` from its own class; else a
/// hit of 0x10000 at half its damage (× `+0xf8` from its own class too); the damage (`0x327fc0`) off its health, the
/// hit let go; no health left: [`blow_up`].
pub fn update_0c87(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => w.mobys[id].state = 1,
        1 => {}
        _ => return,
    }
    ai::flash(&mut w.mobys[id], REACT);
    let first = w.hit_message(id).filter(|h| h.tmpl.flags & 1 != 0);
    let m = &mut w.mobys[id];
    let mut wait = m.pv_s(HEALTH + 6) as i32;
    let over = crate::moby::countdown(&mut wait);
    m.set_pv_s(HEALTH + 6, wait as i16);
    let own = |w: &MobyWorld, h: &crate::moby::hits::HitRecord| h.tmpl.source.and_then(|s| w.mobys.get(s)).is_some_and(|s| s.o_class == w.mobys[id].o_class);
    let k = w.mobys[id].pv_f(REACT + 0xa8);
    let hit = match first.filter(|_| over) {
        Some(mut h) => {
            if k != 0.0 && own(w, &h) { h.tmpl.damage *= k; }
            Some(h)
        }
        None => w.hit_message(id).filter(|h| h.tmpl.flags & 0x10000 != 0).map(|mut h| {
            h.tmpl.damage *= 0.5;
            if k != 0.0 && own(w, &h) { h.tmpl.damage *= k; }
            h
        }),
    };
    let wrench = hit.and_then(|h| h.tmpl.source).and_then(|s| w.mobys.get(s)).is_some_and(|s| s.o_class == crate::hero::items::WRENCH_CLASS);
    let dmg = ai::damage_of(w, id, hit.as_ref(), wrench);
    w.clear_hit(id);
    let m = &mut w.mobys[id];
    m.set_pv_f(HEALTH, m.pv_f(HEALTH) - dmg);
    if m.pv_f(HEALTH) <= 0.0 { blow_up(w, id, ctx); }
}
