//! Endako (level 3)'s classes, set B: the level's hint manager and its Swingshot scene, the lifts and
//! the small set pieces around them (L03 addresses; the engine functions they call are named by their L03 addresses
//! with the L00 ones the port knows them by where they differ).
//!
//! * **0x0c25** ([`update_0c25`], `0x3f2000`, one placed by the ship): the hint manager, and the scene 0x34 that gives
//!   Ratchet the Swingshot (item 0xd, crate::moby::classes::swingshot) and the Grind Boots (item 0x14) when he walks
//!   into its cuboid `[0x4c]`.
//! * **0x0982 / 0x0983** ([`update_lift`], `0x3cc4e8`): the lifts between their floor and the cuboid `[0x60]`'s.
//! * **0x0e87 / 0x0961** ([`update_0e87`], `0x3cb5c0`), **0x097a** ([`update_097a`], `0x3cc000`), **0x0ef9**
//!   ([`update_0ef9`], `0x41ac38`): pieces the level's robots (0x093c / 0x0940, not ported) raise, stand on and
//!   watch; their own parts.
//! * **0x099d** ([`update_099d`], `0x3cd7e0`): the Swingshot's pull target turning while drawn; **0x094e**
//!   ([`update_empty`], `0x3c9dd0`: `jr ra`).
//! * **0x0a0a** ([`update_0a0a`], `0x3d3800`): a camera tweak of priority 4 held at its defaults; **0x0a0b**
//!   ([`update_0a0b`], `0x3d38d0`): a script camera sliding along a cuboid beside Ratchet while he is in a polygon.
//! * **0x11e7** ([`update_11e7`], `0x4269e8`): a group of mobys glowing in a wave.
//! * **0x11c4** ([`update_11c4`], `0x426318`, on nine levels): a nanotech boost (+1 health, `0x1a7a30 + k`).
//! * **0x12ea** ([`update_12ea`], `0x3854b8`): a box broken open for bolts.
//!
//! Clank's rescue **0x0a08** (`0x3d2fd0`) and Clank's body **0x0057** (`0x394728`) are crate::moby::classes::endako_clank.
//! The hacking panel **0x0b29** (`0x3e5c90`) is crate::moby::classes::hacker_panel, its puzzle crate::hacker.
//! The mod vendor **0x0ddc / 0x0e0d** (`0x40a8a8` / `0x40b110`) is crate::moby::classes::mod_vendor.
//!
//! Not ported (set B's other classes):
//! * The robots **0x093c / 0x0940** (`0x3c31f8`, 49 functions: they walk the network of **0x0f2d** (`0x41b0b0`, built
//!   by `0x32b318` from its list of paths) and raise 0x0e87 and stand on 0x097a): too large for this pass. The boss
//!   **0x0c8a** (`0x3f5c78`) is crate::moby::classes::endako_boss.
//! * **0x0cfc / 0x094c** (`0x3c9430`: Ghidra has no function there), **0x097b** (`0x3cc2b0`: a scrolling-texture draw
//!   `0x3cc380` through the first draw list, G-REN-003), **0x0f16** (`0x41af70`: the colour flash of scene 7's actor
//!   `0x1bd810` and a pose word of scene 5's `0x1bd814`, the scene actors the port does not hand to classes).

use crate::hero::physics::{add, add_rot, dist, dist2, len, len2, scale, sub, turn_ease, ease, ease_speed, V3};
use crate::moby::{countdown, Ctx, Moby, MobyWorld};

/// The Swingshot (item 0xd) and the Grind Boots (item 0x14).
const SWINGSHOT: i32 = 0xd;
const GRIND_BOOTS: usize = 0x14;

/// `0x1395b8 + k` (crate::game::GameState::flags): `0x1395c1` the Swingshot's scene given, `0x1395c5` Ratchet past
/// the cuboid `[0x6c]`, `0x1395d0` the first nanotech boost found, `0x139630` bit 0 the robots' watch done on Endako
/// (bit 1 on level 0xe), `0x13965a` the grind help's zone reached.
const F_SCENE: usize = 0x09;
const F_PAST: usize = 0x0d;
const F_BOOST: usize = 0x18;
const F_WATCH: usize = 0x78;
const F_GRIND: usize = 0xa2;

fn flag(ctx: &Ctx, k: usize) -> u8 { ctx.game.flags.get(k).copied().unwrap_or(0) }
fn set_flag(ctx: &mut Ctx, k: usize, v: u8) { if let Some(b) = ctx.game.flags.get_mut(k) { *b = v; } }
fn inside(ctx: &Ctx, z: i32) -> bool { ctx.level.in_cuboid(z, ctx.hero.pos) }
fn have(ctx: &Ctx, item: i32) -> bool { ctx.game.items.get(item.max(0) as usize).is_some_and(|&b| b != 0) }
fn alive(w: &MobyWorld, i: i32) -> bool { usize::try_from(i).ok().and_then(|i| w.mobys.get(i)).is_some_and(|m| m.state != 0xfe && m.state != 0xfd) }
fn hooked(m: &Moby) -> bool { m.once & 4 != 0 }

/// `0x2f0ef0` (L00 `0x2efef0`): `a` wrapped to [−π, π) by its fraction of a turn.
fn wrap(a: f32) -> f32 {
    let f = a * 0.159_154_94 + 0.5;
    ((f - f.floor()) - 0.5) * std::f32::consts::TAU
}

/// The members of the moby group `g` (`0x1fb5a0[g]`; none for −1 or past the groups).
fn group(w: &MobyWorld, g: i32) -> Vec<usize> {
    if g & 0xff == 0xff { return Vec::new(); }
    usize::try_from(g).ok().and_then(|g| w.groups.get(g)).cloned().unwrap_or_default()
}

/// `0x2f02d8(out, v, cuboid + 0x40)`: `v` (from the cuboid's place) through the cuboid's inverse.
fn cuboid_local(ctx: &Ctx, z: i32, p: V3) -> Option<V3> {
    let c = usize::try_from(z).ok().and_then(|i| ctx.level.cuboids.get(i))?;
    let d = [p[0] - c.matrix[3][0], p[1] - c.matrix[3][1], p[2] - c.matrix[3][2]];
    let m = &c.inverse;
    Some(std::array::from_fn(|k| d[0] * m[0][k] + d[1] * m[1][k] + d[2] * m[2][k]))
}

/// `0x2bf080(pos, rot, 0, 1)` (L00 `0x2b9e68`): Ratchet put at the cuboid `z`'s place facing its `+0x70` angles, idle.
fn put_hero(ctx: &mut Ctx, z: i32) {
    if let Some((p, r)) = ctx.level.cuboid_place(z) {
        ctx.hero.teleport(p, r);
        ctx.hero.request_state(0, true);
    }
}

/// `0x2f7638(text, slot, kind, zone, off)`: L03's zone help. As crate::moby::classes::oozla::zone_help but for its
/// kind 3, which asks for the text while the slot's levels word lacks bit 31 (never shown yet).
fn zone_help(ctx: &mut Ctx, text: i32, slot: i32, kind: i32, zone: i32, off: i32) {
    if kind != 3 {
        super::oozla::zone_help(ctx, text, slot, kind, zone, off);
        return;
    }
    let k = slot.max(0) as usize;
    if ctx.game.help_counts.get(k).copied().unwrap_or(-1) == -1 { return; }
    if off != -1 && inside(ctx, off) {
        ctx.game.help_counts[k] = -1;
        return;
    }
    if !inside(ctx, zone) || (ctx.game.help_levels[k] as i32) < 0 { return; }
    ctx.game.help(text, slot);
}

/// The help slot `k` asked to stay shut (its count −1), the panel put away when it shows or asks for it.
fn help_off(ctx: &mut Ctx, k: usize, only_slot: bool) {
    ctx.game.help_counts[k] = -1;
    if ctx.game.help_state != 0 || ctx.game.help_req.is_some() { ctx.game.help_cancel(only_slot.then_some(k as i32)); }
}

/// Every member of group `g` deleted (the loops of `0x3f2000` and `0x3f2ab0`).
fn delete_group(w: &mut MobyWorld, g: i32) {
    for i in group(w, g) { w.delete(i); }
}

/// 0x0c25 (`0x3f2000`, one placed): Endako's hint manager. Its pvars (`+0x68`, words by offset): the cuboids
/// `[0x08]`..`[0x2c]`, `[0x40]`, `[0x4c]`, `[0x54]`, `[0x58]`, `[0x64]`, `[0x6c]`, `[0x70]`, `[0x78]`, `[0x7c]`; the
/// group `[0x80]`; the timer `[0x60]`, the Swingshot aims counted `[0x30]`.
///
/// * **0**: updated at any distance, `[0x74]` 0. With the Swingshot: before Ratchet has passed `[0x6c]` (`0x1395c5`) he
///   is put at `[0x70]` with the Swingshot asked for in his hand (`0x323c80(0xd)`); the group `[0x80]` (the scene's
///   props) deleted. 1.
/// * **1**, every tick:
///   * Ratchet in `[0x6c]` marks it passed; in `[0x4c]` the scene 0x34 is queued and started once (`0x1395c1`), its
///     end (`0x3f2ab0`, the game's callback `0x1a8fd0`): Ratchet put at `[0x58]`, the Grind Boots his (`0x1a7b14`,
///     the new mark `0x1a7b4c`), the Swingshot his (`0x2f56e0`) and in his hand, the group `[0x80]` deleted;
///   * 0x9cb (slot 0x22) as Ratchet's body in `[0x08]` every 600 ticks (`[0x60]`); in `[0x0c]` (any body) slot 0x22
///     off, the panel put away when it is that slot's;
///   * 0x9cc (0x23) in `[0x10]` with the panel idle, unless shown within 0x79 ticks (the stay keeps it fresh); in
///     `[0x14]` slot 0x23 off;
///   * 0x9cd (0x24) in `[0x18]` 0x961 ticks after its last showing; else in `[0x1c]` slot 0x24 off (`0x13965a` set);
///   * 0xa24 (0x7b) in `[0x64]` while unshown (`0x13965a` set);
///   * the zones' hints: 0x9d3 (0x2a, kind 0) in `[0x20]`; 0xa2c (0x83) in `[0x24]` till the robots' watch is done
///     (`0x139630` bit 0), not within 0x439 ticks of its last showing; 0x9d5 (0x2c, kind 0) in `[0x40]`, 0x9e5 (0x3c,
///     kind 2) in `[0x54]`;
///   * in `[0x2c]` slot 0x2b off; else in `[0x28]` the Swingshot's aims (state 0x23, its first tick) counted, past two
///     0x9d4 (0x2b) while aiming;
///   * below 155.5 within 24 of (400, 222, 155) slot 0x25's last showing cleared, then 0x9ce (0x25) in `[0x28]` off in
///     `[0x2c]`, kinds 1 and 3;
///   * 0x9c9 (0x20) in `[0x7c]` once a level; 0x9b5 (0xc, the map) in `[0x78]` past half an hour once a level.
///
/// Not ported: the ship byte `0x152c26` (`0x369cc0`), the stats some hints read (`0x18c840` / `0x18c844`, `0x18c7c0`:
/// G-HERO-005, read as none), the hint 0xa33 (slot 0x8a: its test `0x312640` reads the list `0x1b2918` the port does
/// not have), the end's camera record reset (`0x43cf38([0x5c])`) and autosave (`0x30ea10`), the draw `0x3f27e0`
/// (registered while `$gp+0x1ef0` is set: nothing on the port sets it).
pub fn update_0c25(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let pv = |w: &MobyWorld, o: usize| w.mobys[id].pv_i(o);
    match w.mobys[id].state {
        0 => {
            w.mobys[id].update_dist = 0xff;
            w.mobys[id].set_pv_i(0x74, 0);
            if have(ctx, SWINGSHOT) {
                if flag(ctx, F_PAST) == 0 {
                    put_hero(ctx, pv(w, 0x70));
                    let held = ctx.game.held;
                    ctx.hero.items.ask(held, SWINGSHOT);
                }
                let g = pv(w, 0x80);
                delete_group(w, g);
            }
            w.mobys[id].set_state(1, -1);
            return;
        }
        1 => {}
        _ => return,
    }
    // The scene's end (the port's: `+0xbc` keeps the callback asked for until the chain is over).
    if w.mobys[id].timer == 1 && !ctx.game.scenes.queue.busy && !ctx.game.scenes.in_scene() {
        w.mobys[id].timer = 0;
        scene_end_0c25(w, id, ctx);
    }
    let now = ctx.game.play_time;
    if flag(ctx, F_PAST) == 0 && inside(ctx, pv(w, 0x6c)) { set_flag(ctx, F_PAST, 1); }
    if flag(ctx, F_SCENE) == 0 && inside(ctx, pv(w, 0x4c)) && ctx.game.scenes.queue.push(0x34) {
        ctx.game.scenes.start_queue(ctx.hero.hp);
        set_flag(ctx, F_SCENE, 1);
        w.mobys[id].timer = 1;
    }
    // 0x9cb (slot 0x22): as Ratchet in `[0x08]`; `[0x0c]` turns it off.
    if ctx.hero.mode == 0 && inside(ctx, pv(w, 0x08)) {
        let mut t = pv(w, 0x60);
        if countdown(&mut t) {
            ctx.game.help(0x9cb, 0x22);
            t = 600;
        }
        w.mobys[id].set_pv_i(0x60, t);
    } else if inside(ctx, pv(w, 0x0c)) && ctx.game.help_counts[0x22] != -1 {
        help_off(ctx, 0x22, true);
    }
    // 0x9cc (0x23): in `[0x10]` with the panel idle; `[0x14]` turns it off.
    if inside(ctx, pv(w, 0x10)) && ctx.game.help_state == 0 && ctx.game.help_req.is_none() {
        let last = ctx.game.help_last[0x23];
        if 0x79 <= now.wrapping_sub(last) as u32 { ctx.game.help(0x9cc, 0x23); }
        if last < now { ctx.game.help_last[0x23] = now; }
    } else if inside(ctx, pv(w, 0x14)) && ctx.game.help_counts[0x23] != -1 {
        help_off(ctx, 0x23, false);
    }
    // 0x9cd (0x24): in `[0x18]`; `[0x1c]` turns it off.
    if inside(ctx, pv(w, 0x18)) {
        if 0x961 <= now.wrapping_sub(ctx.game.help_last[0x24]) as u32 { ctx.game.help(0x9cd, 0x24); }
    } else if inside(ctx, pv(w, 0x1c)) {
        set_flag(ctx, F_GRIND, 1);
        help_off(ctx, 0x24, false);
    }
    if inside(ctx, pv(w, 0x64)) && ctx.game.help_counts[0x7b] == 0 {
        set_flag(ctx, F_GRIND, 1);
        ctx.game.help(0xa24, 0x7b);
    }
    zone_help(ctx, 0x9d3, 0x2a, 0, pv(w, 0x20), -1);
    if inside(ctx, pv(w, 0x24)) && flag(ctx, F_WATCH) & 1 == 0 {
        let last = ctx.game.help_last[0x83];
        if (now.wrapping_sub(last) as u32) < 0x439 && last != 0 {
            if last < now { ctx.game.help_last[0x83] = now; }
        } else {
            ctx.game.help(0xa2c, 0x83);
        }
    }
    zone_help(ctx, 0x9d5, 0x2c, 0, pv(w, 0x40), -1);
    zone_help(ctx, 0x9e5, 0x3c, 2, pv(w, 0x54), -1);
    // The Swingshot's aims in `[0x28]` (state 0x23 on its first tick), off in `[0x2c]`.
    if inside(ctx, pv(w, 0x2c)) {
        ctx.game.help_counts[0x2b] = -1;
    } else if inside(ctx, pv(w, 0x28)) {
        let aiming = ctx.hero.state == 0x23;
        if aiming && ctx.hero.timer == 1 {
            let n = pv(w, 0x30) + 1;
            w.mobys[id].set_pv_i(0x30, n);
        }
        if 2 < pv(w, 0x30) && aiming { ctx.game.help(0x9d4, 0x2b); }
    }
    if ctx.hero.pos[2] < 155.5 && dist(ctx.hero.pos, [400.0, 222.0, 155.0]) < 24.0 { ctx.game.help_last[0x25] = 0; }
    zone_help(ctx, 0x9ce, 0x25, 1, pv(w, 0x28), pv(w, 0x2c));
    zone_help(ctx, 0x9ce, 0x25, 3, pv(w, 0x28), pv(w, 0x2c));
    let level_bit = 1u32.checked_shl(ctx.game.level.max(0) as u32).unwrap_or(0);
    // `0x18c840` (a stat, none) → asked whenever its levels' word lacks this level.
    if inside(ctx, pv(w, 0x7c)) && ctx.game.help_levels[0x20] & level_bit == 0 { ctx.game.help(0x9c9, 0x20); }
    // `now − 0x18c7c0` (the map's last use, none) past 0x1a5e0 ticks.
    if 0x1a5e0 < now as u32 && inside(ctx, pv(w, 0x78)) && ctx.game.help_levels[0xc] & level_bit == 0 { ctx.game.help(0x9b5, 0xc); }
}

/// `0x3f2ab0`: the Swingshot's scene over (module docs).
fn scene_end_0c25(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    put_hero(ctx, w.mobys[id].pv_i(0x58));
    ctx.game.grind_boots = true;
    if let Some(b) = ctx.game.items.get_mut(GRIND_BOOTS) { *b = 1; }
    ctx.game.get_item(SWINGSHOT);
    let held = ctx.game.held;
    ctx.hero.items.ask(held, SWINGSHOT);
    let g = w.mobys[id].pv_i(0x80);
    delete_group(w, g);
}

/// 0x0982 / 0x0983 (`0x3cc4e8`, two placed each): a lift between its place and the height of the cuboid `[0x60]`
/// (the floor below on Endako). Its pvars:
/// the platform block `+0x20` (crate::moby::platform), `[0x68]` up, `[0x6c]` its speed, its home `[0x70]`, the looped
/// sound `[0x80]`, the stand latch `[0x84]`.
///
/// * **0**: no pvars, deleted; else its home kept: 1.
/// * **1**: at rest (within 2 of its goal: the home's height, or up the cuboid's): Ratchet standing on it (his ground
///   moby, on the ground) sends it the other way once a stand (the latch, let go when he is off it or 0.2 above it);
///   home with Ratchet on the cuboid's floor (within 4 of its height, 24 across) it is called there; at the cuboid with
///   him on its home floor it is called home. At the cuboid, its goal rises 0.15 while it is below the camera with Ratchet above its
///   floor less 1, else 3 (5 for class 0xbf5) with Ratchet in the cuboid or the camera over it (within 1 across in
///   the cuboid's frame), braking twice as hard then. It eases to its goal (`0x320220`: 7/3600 a tick², 14/3600 or
///   28/3600 to brake, at most 0.5 a tick), landing on it when within its speed.
/// * Moving (above 0.01 a tick) its sound 0 loops; still, it stops. Its move goes into its block and carries Ratchet.
pub fn update_lift(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let (old, old_rot) = (m.pos, m.rot);
    match m.state {
        0 => {
            if m.pvars.is_empty() {
                w.delete(id);
                return;
            }
            let m = &mut w.mobys[id];
            let p = m.pos;
            m.set_pv_v(0x70, p);
            m.set_state(1, -1);
        }
        1 => lift_tick(w, id, ctx),
        _ => {}
    }
    let speed = w.mobys[id].pv_f(0x6c);
    let h = w.mobys[id].pv_i(0x80);
    if 0.01 < speed.abs() {
        if !ctx.host.sound_playing(h) {
            let h = ctx.host.play_sound(&w.mobys[id], 0, 4);
            w.mobys[id].set_pv_i(0x80, h);
        }
    } else {
        if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
        w.mobys[id].set_pv_i(0x80, -1);
    }
    let m = &mut w.mobys[id];
    let (pos, rot) = (m.pos, m.rot);
    crate::moby::platform::write(m, 0x20, sub(pos, old), rot, rot);
    let flags = m.pv_i(0x20 + 0x3c) as u32;
    crate::moby::carry_hero_with(ctx, id, flags, old, old_rot[2], pos, rot[2]);
}

fn lift_tick(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let cuboid = w.mobys[id].pv_i(0x60);
    let top = ctx.level.cuboid_place(cuboid).map_or(0.0, |(p, _)| p[2]);
    let hp = ctx.hero.pos;
    let cam = ctx.host.camera_pos();
    let m = &mut w.mobys[id];
    let home = m.pv_v(0x70);
    let mut goal = if m.pv_i(0x68) == 0 { home[2] } else { top };
    if (goal - m.pos[2]).abs() < 2.0 {
        let on = ctx.hero.ground_moby == Some(id);
        if on && ctx.hero.grounded_ticks != 0 {
            if m.pv_i(0x84) == 0 { m.set_pv_i(0x68, (m.pv_i(0x68) + 1) & 1); }
            m.set_pv_i(0x84, 1);
        } else {
            if !on || 0.2 < ctx.hero.height { m.set_pv_i(0x84, 0); }
            let near = 4.0 > (hp[2] - if m.pv_i(0x68) == 0 { top } else { home[2] }).abs() && dist2(hp, m.pos) < 24.0;
            if near { m.set_pv_i(0x68, (m.pv_i(0x68) == 0) as i32); }
        }
    }
    let mut brake = 0.003_888_889;
    if m.pv_i(0x68) != 0 {
        if m.pos[2] - 1.0 <= hp[2] && m.pos[2] <= cam[2] {
            goal += 0.15;
        } else {
            let over = cuboid_local(ctx, cuboid, cam).is_some_and(|l| l[0].abs() < 1.0 && l[1].abs() < 1.0);
            if ctx.level.in_cuboid(cuboid, hp) || over {
                goal += if m.o_class == 0xbf5 { 5.0 } else { 3.0 };
            }
            brake = 0.007_777_778;
        }
    }
    let (mut z, mut v) = (m.pos[2], m.pv_f(0x6c));
    ease(goal, 0.001_944_444_5, brake, 0.5, &mut z, &mut v);
    if (z - goal).abs() <= v {
        z = goal;
        if v != 0.0 { v = 0.0; }
    }
    m.pos[2] = z;
    m.set_pv_f(0x6c, v);
}

/// 0x0e87 / 0x0961 (`0x3cb5c0`, one placed): a piece the robots raise. **0**: a fifth of its class's size, its block's
/// flag 1 (`+0x5c`), its home `[0x90]`: 1. **1**: hooked (`0x32bf38`, the robot's) → 2. **2**: held `[0xa0]` over its
/// home while hooked; let go → 3. **3**: it eases up to 2.8 over its home (`0x320220`: 1/2400 a tick² both ways, at
/// most 0.05 a tick, the speed `[0xa4]`) and there stops updating (mode 2).
pub fn update_0e87(w: &mut MobyWorld, id: usize, _ctx: &mut Ctx) {
    let class_scale = w.class(w.mobys[id].o_class).map_or(1.0, |c| c.scale);
    let m = &mut w.mobys[id];
    match m.state {
        0 => {
            m.scale = class_scale * 0.2;
            m.set_pv_i(0x5c, m.pv_i(0x5c) | 1);
            let p = m.pos;
            m.set_pv_v(0x90, p);
            m.set_state(1, -1);
        }
        1 => if hooked(m) { m.set_state(2, -1) },
        2 => {
            m.pos[2] = m.pv_f(0xa0) + m.pv_f(0x98);
            if !hooked(m) { m.set_state(3, -1); }
        }
        3 => {
            let goal = m.pv_f(0x98) + 2.8;
            let (mut z, mut v) = (m.pos[2], m.pv_f(0xa4));
            ease(goal, 0.000_416_666_68, 0.000_416_666_68, 0.050_000_004, &mut z, &mut v);
            m.pos[2] = z;
            m.set_pv_f(0xa4, v);
            if goal <= m.pos[2] { m.mode |= 2; }
        }
        _ => {}
    }
}

/// 0x097a (`0x3cc000`, two placed): an unseen floor the robots use. **0**: hidden and frozen (mode 0x41), no target,
/// no collision: 1. **1**: hooked → its class's collision on, 2. **2**: let go → off, 1.
pub fn update_097a(w: &mut MobyWorld, id: usize, _ctx: &mut Ctx) {
    let solid = w.class(w.mobys[id].o_class).is_some_and(|c| c.has_collision);
    let m = &mut w.mobys[id];
    match m.state {
        0 => {
            m.set_state(1, -1);
            m.has_collision = false;
            m.mode = m.mode & 0xefff | 0x41;
        }
        1 if hooked(m) => {
            m.set_state(2, -1);
            m.has_collision = solid;
        }
        2 if !hooked(m) => {
            m.set_state(1, -1);
            m.has_collision = false;
        }
        _ => {}
    }
}

/// 0x0ef9 (`0x41ac38`, one placed): a watcher of two mobys (`[0x08]`, `[0x0c]`). Its pvars: `[0x00]` / `[0x04]` the
/// mobys it hooks, `[0x10]` drawn, `[0x14]` hidden (mode 0x41 at its start, no collision every tick), `[0x1c]` a moby.
/// **0**: with both watched mobys 1, else 2. **0 / 1**: both gone (−1 or deleted) → 2. **3** (set by its robots):
/// once the moby `[0x1c]` is gone it hooks `[0x00]` and `[0x04]`, marks the watch done (`0x139630` bit 0 on Endako,
/// bit 1 on level 0xe) and goes to 4.
///
/// Not ported: its marker drawn while `[0x10]` is set (`0x41af10` → `0x33a558`, which the port has no particle for).
pub fn update_0ef9(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let pv = |w: &MobyWorld, o: usize| w.mobys[id].pv_i(o);
    if pv(w, 0x14) != 0 { w.mobys[id].has_collision = false; }
    let watch = match w.mobys[id].state {
        0 => {
            if pv(w, 0x14) != 0 { w.mobys[id].mode |= 0x41; }
            let s = if pv(w, 0x08) != -1 && pv(w, 0x0c) != -1 { 1 } else { 2 };
            w.mobys[id].set_state(s, -1);
            true
        }
        1 => true,
        3 => {
            if let Some(k) = w.mobys[id].pv_link(0x1c) {
                if !alive(w, k as i32) {
                    for o in [0x00, 0x04] {
                        if let Some(t) = usize::try_from(pv(w, o)).ok().filter(|&t| t < w.mobys.len()) { w.mobys[t].once |= 4; }
                    }
                    let bit = match ctx.game.level { 3 => 1, 0xe => 2, _ => 0 };
                    if bit != 0 { set_flag(ctx, F_WATCH, flag(ctx, F_WATCH) | bit); }
                    w.mobys[id].set_state(4, -1);
                }
            }
            false
        }
        _ => false,
    };
    if watch && !alive(w, pv(w, 0x08)) && !alive(w, pv(w, 0x0c)) { w.mobys[id].set_state(2, -1); }
}

/// 0x099d (`0x3cd7e0`, one placed): the Swingshot's pull target (crate::hero::swing, `0x2c4340`): drawn, it turns 6°
/// a tick about y and 2.67° about z.
pub fn update_099d(w: &mut MobyWorld, id: usize, _ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    if m.drawn == 0 { return; }
    m.rot[1] = add_rot(m.rot[1], 0.104_719_77);
    m.rot[2] = add_rot(m.rot[2], 0.046_542_123);
}

/// 0x094e (`0x3c9dd0`, one placed): an update that does nothing (`jr ra`): the moby stays in the run list.
pub fn update_empty(_w: &mut MobyWorld, _id: usize, _ctx: &mut Ctx) {}

/// 0x0a0a (`0x3d3800`, one placed): a camera tweak of priority 4 (`0x2d8038(4)`, L00 `0x2d7468`, kept at `$gp+0x1640`)
/// put back to its defaults every tick (every field −1, the springs not firm, the stick on, no facing, the rest 0x78
/// ticks, the rate 6°), so the lower tweaks give way; a tick with `$gp+0x446c` set skips it and clears the flag
/// (nothing on the port sets it). Updated at any distance. The port keeps the slot in its pvar word 0 (+1).
pub fn update_0a0a(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    match m.state {
        0 => {
            let slot = ctx.game.cam_tweaks.request(4).map_or(0, |s| s as i32 + 1);
            m.set_pv_i(0, slot);
            m.set_state(1, -1);
            m.update_dist = 0xff;
        }
        1 => {
            let Some(s) = usize::try_from(m.pv_i(0) - 1).ok().and_then(|s| ctx.game.cam_tweaks.slots.get_mut(s)) else { return };
            *s = crate::camera::CamTweak { priority: s.priority, active: s.active, life: s.life, g3c: s.g3c, ..Default::default() };
        }
        _ => {}
    }
}

/// 0x0a0b (`0x3d38d0`, two placed): a script camera beside a walkway, while `$gp+0x1644` is 0 (nothing on the port
/// sets it). Its pvars: `[0]` a polygon (the level's path), `[1]` a cuboid, `[6]` the move's rate, `[7]` / `[8]` the
/// turns' speeds.
///
/// * **0**: 1; with no polygon deleted.
/// * **1**: Ratchet in the polygon (`0x31e370`) → the script camera in where the view is (mode 1: on its targets), 2.
/// * **2** (`0x3d3b20`): its place on the cuboid's x axis by Ratchet's place in the cuboid, held back 0.5 less a 16th
///   of his distance from the camera (at least 0), within the cuboid; it looks at Ratchet's place at its own height
///   plus 2. The camera moves toward the place by the rate times the way left, the rate easing to the distance
///   (`0x320958`: 1/360 a tick² both ways, at most 1); its pitch and yaw turn toward the look (`0x3206d8`: 0.05° a
///   tick², at most 1). Ratchet out of the polygon → released (kind 1), 1.
pub fn update_0a0b(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let poly = w.mobys[id].pv_i(0);
    match w.mobys[id].state {
        0 => {
            w.mobys[id].set_state(1, -1);
            if poly == -1 { w.delete(id); }
        }
        1 => {
            if ctx.level.in_polygon(poly, ctx.hero.pos) {
                let (p, e) = ctx.game.cameras.view_euler();
                ctx.game.cameras.camera_script(p, e, crate::script_camera::MODE_SNAP, 0, false, ctx.hero);
                w.mobys[id].set_state(2, -1);
            }
        }
        2 => {
            let (at, look) = camera_0a0b(w, id, ctx);
            let (view, mut euler) = ctx.game.cameras.view_euler();
            let d = sub(at, view);
            let m = &mut w.mobys[id];
            let mut rate = m.pv_f(0x18);
            ease_speed(len(d), 0.002_777_777_8, 0.002_777_777_8, 1.0, &mut rate);
            m.set_pv_f(0x18, rate);
            let p = add(view, scale(d, rate));
            for (k, off) in [(1usize, 0x1c), (2, 0x20)] {
                let mut v = m.pv_f(off);
                turn_ease(look[k], 0.000_872_664_7, 0.000_872_664_7, 1.0, &mut euler[k], &mut v, 0);
                m.set_pv_f(off, v);
            }
            ctx.game.cameras.camera_script_targets(Some(p), Some(euler));
            if !ctx.level.in_polygon(poly, ctx.hero.pos) {
                ctx.game.cameras.camera_script2(crate::script_camera::RELEASE_KEEP, ctx.hero);
                w.mobys[id].set_state(1, -1);
            }
        }
        _ => {}
    }
}

/// `0x3d3b20(m, &pos, &euler)`: 0x0a0b's place and look (its docs).
fn camera_0a0b(w: &MobyWorld, id: usize, ctx: &Ctx) -> (V3, V3) {
    let m = &w.mobys[id];
    let z = m.pv_i(4);
    let hp = ctx.hero.pos;
    let Some(c) = usize::try_from(z).ok().and_then(|i| ctx.level.cuboids.get(i)) else { return (ctx.host.camera_pos(), [0.0; 3]) };
    let local = cuboid_local(ctx, z, hp).unwrap_or([0.0; 3]);
    let back = (0.5 - dist2(hp, ctx.host.camera_pos()) * 0.0625).max(0.0);
    let x = (local[0] - back).clamp(-1.0, 1.0);
    let at: V3 = std::array::from_fn(|k| x * c.matrix[0][k] + c.matrix[3][k]);
    let mut d = sub([hp[0], hp[1], m.pos[2]], at);
    d[2] += 2.0;
    (at, [0.0, -d[2].atan2(len2(d)), d[1].atan2(d[0])])
}

/// 0x11e7 (`0x4269e8`, one placed): a group (`[0]`) glowing in a wave. **0**: with members, the step `[1]` −3° a tick,
/// `[2]` from degrees to radians, updated at any distance; each member glows (mode 0x10), its colours `[1]` 0x80a0a0a0
/// and `[2]` 0x80000000, its phase `[3]` its own `[0]` × 30°, updated at any distance, drawn to 0x1ff: 1. No group:
/// deleted. **1**: each member's phase steps, its glow word (`+0x78`) between its two colours by `sin · 0.5 + 0.5`.
pub fn update_11e7(w: &mut MobyWorld, id: usize, _ctx: &mut Ctx) {
    let g = w.mobys[id].pv_i(0);
    let members = group(w, g);
    match w.mobys[id].state {
        0 => {
            if members.is_empty() {
                w.delete(id);
                return;
            }
            let m = &mut w.mobys[id];
            m.set_pv_f(4, -0.052_359_88);
            m.set_pv_f(8, m.pv_f(8) * 0.017_453_294);
            m.update_dist = 0xff;
            for i in members {
                let t = &mut w.mobys[i];
                t.mode |= 0x10;
                t.set_pv_i(8, 0x8000_0000u32 as i32);
                t.set_pv_i(4, 0x80a0_a0a0u32 as i32);
                let a = wrap((t.pv_i(0) * 0x1e) as f32 * 0.017_453_294);
                t.set_pv_f(0xc, a);
                t.update_dist = 0xff;
                t.draw_dist = 0x1ff;
            }
            w.mobys[id].state = 1;
        }
        1 => {
            let step = w.mobys[id].pv_f(4);
            for i in members {
                let t = &mut w.mobys[i];
                let a = add_rot(t.pv_f(0xc), step);
                t.set_pv_f(0xc, a);
                t.glow = crate::hud::lerp_rgba(a.sin() * 0.5 + 0.5, t.pv_i(4) as u32, t.pv_i(8) as u32);
            }
        }
        _ => {}
    }
}

/// `$gp+0x2e40`: the four points the boost's sparks start from (a tetrahedron), `+0x2e80` their distance out,
/// `+0x2e84` their speed in (a second), `+0x2e88` their life, `+0x2e90..` the four angles' turns (degrees a second),
/// `+0x2ea0` the tilt.
const BOOST_POINTS: [V3; 4] = [[0.0, 0.0, 1.0], [0.866_025_4, 0.0, -0.5], [-0.433_012_7, 0.75, -0.5], [-0.433_012_7, -0.75, -0.5]];
const BOOST_OUT: f32 = 1.5;
const BOOST_IN: f32 = 1.0;
const BOOST_LIFE: i16 = 60;
const BOOST_TURNS: [f32; 4] = [37.0, 92.0, 180.0, -180.0];
const BOOST_TILT: f32 = 0.05;
/// The boost's help (`0x2f78d8`, queued): "you found a nanotech boost".
const BOOST_TEXT: (i32, i32) = (0xa31, 0x88);

/// 0x11c4 (`0x426318`, one on Endako): a nanotech boost, `[0]` its number k (`0x1a7a30 + k`,
/// crate::game::GameState::nanotech).
///
/// * **0**: 1.1 times its class's size, `+0xbc` 0; found already, deleted. Else (`0x426708`) its four glows' phases
///   `[0x10..]` at random (0..255), their steps `[0x20..]` (−1, −2.25, 1.25, 2.5), their counters `[0x30..]` (0x3f,
///   0x7f, 0xbf, 0xff) and sizes `[0x40..]` (2, 2.46, 2.46, 2). On level 5 the ship (class 0xfbf) is looked for.
/// * **1**: it turns 0.03 a tick; Ratchet within 1.5 (on level 5 the ship within 3) → 2. Its four angles `[0x50..]`
///   turn 37, 92, 180 and −180 degrees a second; from each corner of a tetrahedron turned by them, 1.5 out, a blast
///   sprite flies in at its speed (`0x341dc0`, L00 `0x340830`: size 0.07..0.12 × 210000 toward 42000, 60 ticks,
///   0x804020, alpha 0 → 0x6f, frame 6, its spin ±2..5); it tilts by the fourth angle (0.05 · cos / sin about x / y).
///   Its glows ([`boost_glows`]).
/// * **2**: found (`0x1a7a30 + k` = 1; the first also `0x1395d0`), its help queued while slot 0x88 is unshown,
///   deleted.
pub fn update_11c4(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let k = w.mobys[id].pv_i(0);
    let found = |ctx: &Ctx| usize::try_from(k).ok().and_then(|k| ctx.game.nanotech.get(k)).copied().unwrap_or(false);
    let class_scale = w.class(w.mobys[id].o_class).map_or(1.0, |c| c.scale);
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.state = 1;
            m.timer = 0;
            m.scale = class_scale * 1.1;
            if found(ctx) {
                w.delete(id);
                return;
            }
            let m = &mut w.mobys[id];
            for j in 0..4 {
                let r = ctx.game.rng.randi(0xff);
                m.set_pv_f(0x10 + 4 * j, r as f32);
            }
            for (j, s) in [-1.0f32, -2.25, 1.25, 2.5].iter().enumerate() { m.set_pv_f(0x20 + 4 * j, *s); }
            for j in 0..4 { m.set_pv_i(0x30 + 4 * j, 0x3f + 0x40 * j as i32); }
            for (j, s) in [2.0f32, 2.46, 2.46, 2.0].iter().enumerate() { m.set_pv_f(0x40 + 4 * j, *s); }
            m.set_pv_f(0x44, f32::from_bits(0x401d_70a4));
            m.set_pv_f(0x48, f32::from_bits(0x401d_70a4));
        }
        1 => {
            let m = &mut w.mobys[id];
            m.rot[2] = add_rot(m.rot[2], 0.03);
            let pos = m.pos;
            let mut take = dist(ctx.hero.pos, pos) < 1.5;
            if ctx.game.level == 5 {
                take |= w.mobys.iter().any(|s| s.o_class == 0xfbf && s.state < 0x80 && dist(s.pos, pos) < 3.0);
            }
            if take { w.mobys[id].state = 2; }
            let m = &mut w.mobys[id];
            let mut a = [0.0f32; 4];
            for (j, t) in BOOST_TURNS.iter().enumerate() {
                a[j] = add_rot(m.pv_f(0x50 + 4 * j), t * 0.000_290_888_25);
                m.set_pv_f(0x50 + 4 * j, a[j]);
            }
            let rows = crate::moby::attach::rows_of([a[0], a[1], a[2]]);
            let hero = ctx.hero.pos;
            for p in BOOST_POINTS.iter() {
                let spin = ctx.game.rng.randf_signed(2.0, 5.0) as i32 as u8;
                let d: V3 = std::array::from_fn(|c| p[0] * rows[0][c] + p[1] * rows[1][c] + p[2] * rows[2][c]);
                let at = add(scale(d, BOOST_OUT), pos);
                let vel = scale(sub(pos, at), BOOST_IN / 60.0);
                let size = ctx.game.rng.randf(0.07, 0.12) * 210_000.0;
                if let Some(sys) = ctx.parts.as_deref_mut() {
                    crate::particles::burst(sys, &mut ctx.game.rng, size, 42_000.0, 0.0, at, vel, BOOST_LIFE, 0x80_4020, spin, 0, 0x6f, 6);
                }
            }
            let m = &mut w.mobys[id];
            m.rot[0] = a[3].cos() * BOOST_TILT;
            m.rot[1] = a[3].sin() * BOOST_TILT;
            let cam = ctx.host.camera_pos();
            if let Some(sys) = ctx.parts.as_deref_mut() { boost_glows(&mut w.mobys[id], sys, cam, hero); }
        }
        2 => {
            if let Some(b) = usize::try_from(k).ok().and_then(|k| ctx.game.nanotech.get_mut(k)) { *b = true; }
            if k == 0 { set_flag(ctx, F_BOOST, 1); }
            if ctx.game.help_counts[BOOST_TEXT.1 as usize] == 0 { ctx.game.help_later(BOOST_TEXT.0, BOOST_TEXT.1); }
            w.delete(id);
        }
        _ => {}
    }
}

/// `0x4267e0`: the boost's four glows, from 0.3 of the way toward the camera behind it, a tenth of that apart: each
/// phase steps (wrapped in 0..255) and gives the glow's angle; its counter runs down from 0xff and the colour goes
/// between 0x50c04020 and 0x10804020 by `|0.5 − (0xff − counter)/255|`. A sparkle (`0x33c9f8`, L00 `0x33b468`) of the
/// size `[0x40..]`, frame list 0x35, soft, two ticks.
fn boost_glows(m: &mut Moby, sys: &mut crate::particles::Particles, cam: V3, hero: V3) {
    let d = scale(sub(cam, m.pos), -0.3);
    let step = scale(d, 0.1);
    let mut at = add(d, m.pos);
    for j in 0..4 {
        let mut ph = m.pv_f(0x10 + 4 * j) + m.pv_f(0x20 + 4 * j);
        if 255.0 <= ph { ph -= 255.0; } else if ph <= 0.0 { ph += 255.0; }
        m.set_pv_f(0x10 + 4 * j, ph);
        let mut c = m.pv_i(0x30 + 4 * j);
        if countdown(&mut c) { c = 0xff; }
        m.set_pv_i(0x30 + 4 * j, c);
        let t = (0.5 - (0xff - c) as f32 * 0.003_921_569).abs();
        let rgba = crate::hud::lerp_rgba(t, 0x50c0_4020, 0x1080_4020);
        if let Some(i) = crate::particles::sparkle(sys, m.pv_f(0x40 + 4 * j), at, rgba, ph as i32 as u8, 0x35, true, 2, false, hero) {
            sys.pool.recs[i][9] = 0x61;
        }
        at = add(at, step);
    }
}

/// 0x12ea (`0x3854b8`, one placed): a box Ratchet breaks open. **0**: a target (mode 0x1000), 1. **1**: a hit with the
/// flags 0x4010000 (`0x31f0f0`, L00 `0x31dce8`) → 2, or 3 from the Dynamo's shot (class 0x825); the hit record let go.
/// **2**: the first broken on the level (`$gp−0x400`) throws ten bolts of 100 from 0.5 above it (`0x3849f0`: a random
/// way across at 0..1 a second, up 5..10 a second); its pieces fly (`0x32cd30`, L00 `0x32b580`: ±1..3 across, 3..9
/// up, 90..360°/s, gravity 10..20, all pointed, its class sound 0), a blast (`0x403f10`: size 0.5, sprites 0x19 /
/// 0x14 / 0x14, 0x14 puffs, 0x2d streaks, its light); deleted. **3**: back to 1 after 60 ticks.
///
/// Not ported: the Dynamo's side of 3 (`0x2da9d0(15, 1)` on its first tick, and once `$gp+0x3470` (then `0x1b2460`)
/// reaches 1 the moby 0x1335 made at the camera, `0x1a8fb4` 2 and `0x1a8fb0` 0xf, then 1): the values it reads are
/// set by code the port does not have, so it waits the 60 ticks.
pub fn update_12ea(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            w.mobys[id].mode |= 0x1000;
            w.mobys[id].set_state(1, -1);
        }
        1 => {
            if let Some(h) = w.hit_message(id) {
                if h.tmpl.flags & 0x0401_0000 != 0 {
                    let dynamo = h.tmpl.source.and_then(|s| w.mobys.get(s)).is_some_and(|s| s.o_class == 0x825);
                    w.mobys[id].set_state(if dynamo { 3 } else { 2 }, -1);
                }
            }
            w.clear_hit(id);
        }
        2 => {
            // `$gp−0x400`, the level's first box broken: Endako has the one, so it always throws them.
            {
                let at = add(w.mobys[id].pos, [0.0, 0.0, 0.5]);
                for _ in 0..10 {
                    let g = &mut ctx.game.rng;
                    let yaw = g.rand_angle();
                    let s = g.randf(0.0, 1.0) * 0.016_666_668;
                    let (x, y) = (yaw.cos() * s, yaw.sin() * s);
                    let z = ctx.game.rng.randf(5.0, 10.0) * 0.016_666_668;
                    crate::moby::bolts::create_bolt(w, id, at, [x, y, z], 0, 100, ctx);
                }
            }
            crate::moby::shards::throw_all(w, id, ctx, [1.0, 1.0, 3.0, 0.0, 3.0, 3.0, 9.0, 0.0], (90.0, 360.0), (10.0, 20.0), 0xffff, 0, -1, 6, 0xffff);
            let at = w.mobys[id].pos;
            crate::moby::blast::burst(w, ctx, at, &crate::moby::blast::Blast { size: 0.5, bursts: [0x19, 0x14, 0x14], puffs: 0x14, streaks: 0x2d, light: true, shake: false });
            w.delete(id);
        }
        3 => {
            let m = &mut w.mobys[id];
            let t = m.state_ticks;
            m.state_ticks = t.wrapping_add(1);
            if 0x3c < t as u16 { m.set_state(1, -1); }
        }
        _ => {}
    }
}
