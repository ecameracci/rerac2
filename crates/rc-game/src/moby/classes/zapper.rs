//! The Clank Zapper (item 9; its upgrade row 0x49): not held — asked for (`0x2b13c8`, crate::hero::items' switch) it
//! takes one of its charges and turns on (`0x18b6f4` = `0x2d42d0(9, 1)`, [`request`]); Clank on Ratchet's back
//! (crate::hero::back) then zaps the enemies about him by himself ([`fire`], `0x2b2408` from the hero update
//! `0x2a6dd0`), each zap the moby **0x0ed8** ([`update`], `0x40cf30`, made by `0x40cd38`).
//!
//! **Firing** ([`fire`], `0x2b2408`): off in state 0x31 (its target `0x18b6f0` dropped). On, as Ratchet (`0x18c0d4` 0),
//! with the Zapper had (`0x1a7b09` / `0x1a7b49`), the back item a pack (`0x2adf18(3)` 2 or 3) and Clank on the back
//! (`0x18c31c` 0): while the wait `0x18b6f8` (120 ticks after a zap) counts down, a target kept gives sparks — ahead
//! of Ratchet (under 80° off his yaw) 20 green ones about the point on Clank's head (`0x18be30`, 0.04 a random way,
//! `0x339848`: 126000, 2 ticks, `0x20ff20`); behind him with the Zapper upgraded (the row's level byte, `0x2624f4`),
//! Clank's glow `0x800000ff` (mode 0x10) and 30 pairs of red ones from his joints 6 and 7 (`0x3127c8`; (−0.1, ±0.01,
//! 0.04) through his rows, wandering 0.004 a step: 100000, 2 ticks, `0x2020ff`). The wait out, the target list
//! (`0x1fac60`) is walked: the last of its mobys of class type 5, alive (not −2 / −3), with a health block
//! (`0x31fa20`), within 16 of Ratchet. Still such a moby and the line from `0x18be30` to it meeting something
//! (`0x2de878`), within 16: with no charge of the row left (`+0x48`) the Zapper turns off and the row's 30 come back;
//! else a zap ([`spawn`]), the wait 120, a charge spent.
//!
//! **The zap** (`0x40cd38(0x18be30, Ratchet, target, upgraded)`): far 0xff, mode 0x100, drawn, its owner Ratchet
//! (`+0xb8`), at `0x18be30`; its pvars: the target `+0x0c`, its end `+0x30` (the target's place raised by its aim
//! height `0x31fa20 +0x10`), its life 12 (`+0x10`, `$gp+0x1f94`; `+0x14` its inverse), upgraded `+0x1c`, three random
//! angles; the target's health block flagged 0x80 (`+0x1e`); the strands' timers (`0x1b2cc0`) −1 and their resets
//! (`0x1b2cd0`) 0x18. (Not ported: its flag 0x80 on the target's health block `+0x1e`.)
//!
//! **Each tick** (`0x40cf30`): Clank's joints 6 / 7 (`+0x60` / `+0x70`); the strands' bend (`+0x40`): 10 below
//! `0x18be30` (`$gp+0x1fb8`), turned a random 0..±45° (by the target's world yaw, below 90° negative) about the line to
//! the end; a dead target dropped. **State 0**: the line from `0x18be30` to the end with the hit (the row's damage
//! `+0x60` — the upgraded row's; flags 0x10000, 0x110000 upgraded; kind `0x0ed8 << 16 | 0x0101`) — ahead, or behind
//! when upgraded; else deleted. It met a moby's primitive (`0x1bf19c` ≥ 1) or nothing: deleted. A wall (no moby) or a
//! moby with collision (`+0x98`): the hit's effect ([`hit_effect`], `0x40e1a0`: 15 or 10 of them) at the hit; Ratchet's
//! class sound 0x15; state 1 hidden (mode 0x41), its life back to 12, a point light at the line's middle (radius 10,
//! green (0.1, 1, 0.1), red (1, 0.1, 0.1) behind; `0x2fdb60`). **State 1**: its life out, the target dropped, the light
//! freed, deleted. Else, behind and upgraded: 10 pairs of sprites along the lines from Clank's joints to the end
//! (`0x33ef78`: 0.35..0.5 ·210000 → ·42000, 3 ticks, `0x2020af`), 20 of four more (0.2..0.35, `0x1010ff` and
//! `0x808080`, half as large), every fifth frame two glows (2100 → 126000, 30 ticks, `0x202080`); ahead: ten pairs of
//! sprites at `0x18be30` (0.25..0.4 ·210000, 2 ticks, `0x108010` / `0x208020`). Then the strands ([`strands`],
//! `0x40c898`) and its draw asked for. Out of the world box (2..1021): deleted.
//!
//! **The strands** (`0x40c898`, the four buffers of ten points `0x256320 + 0xa0·k`): a strand whose timer runs (u16
//! `0x2eeae0`) wanders (each step 0.05 a random way, rebuilt from `0x18be30`); one out takes its reset and is laid
//! anew along the curve `0x31bca0` from `0x18be30` through the bend to a point 0.5 a random way about the end (and its
//! pull 12 past it, `$gp+0x1fbc`), at tenths. After each, strands 1..3's points 2..8 are strand 0's, 0.25 a random way
//! off (`$gp+0x1fb4`).
//!
//! **The draw** ([`draw`], `0x40dcd8`): behind and upgraded, two red bolts (FX 0x17, 0.075 across, `$gp+0x1fa8`,
//! alpha the life · 8, `0x0a0aff`) from Clank's joints to the end, each drawn twice; else the four strands as strips
//! ([`strip`], `0x40e558`): strand 0 (0.07 / 0.15, alpha 0x32, green `0x20ff20`; upgraded 0.2 / 0.35, 0x46,
//! `0x10ff10`), strands 1..3 (0.03 / 0.09, 0xf, `0x10ffff`; upgraded 0.1 / 0.19, 0x20, `0xff1010`).
//!
//! Not ported: the upgraded hit's shock moby 0x0f6d (`0x40f108` / `0x40f458`), the GS test register the strips set
//! (`0x37c948`), the zap's unused aim (`+0x50`, the target's lead).

use crate::hero::physics::{add, cross, len, rot_dist, scale, set_len, sub, unit, V3};
use crate::moby::hits::HitTemplate;
use crate::moby::{countdown, mode, Ctx, MobyWorld};
use crate::point_lights::PointLight;

/// The zap's class (the row's `+0x04`).
pub const ZAP: i16 = 0x0ed8;
/// The item and its upgraded row.
pub const ITEM: i32 = 9;
pub const PLUS_ROW: usize = 0x49;
/// `0x18b6f8`'s wait after a zap.
const WAIT: i32 = 0x78;
/// The row's charges (`+0x48`).
const CHARGES: i32 = 30;
/// `$gp+0x1f94`: a zap's life.
const LIFE: i32 = 12;
/// `$gp+0x1fb0`: behind Ratchet past this many degrees.
const BEHIND_DEG: f32 = 80.0;
/// `0x2b2408`'s own 80° (a literal, not `$gp+0x1fb0`).
const BEHIND_FIRE: f32 = 1.396_263_6;
/// `$gp+0x1fb8` / `$gp+0x1fbc` / `$gp+0x1fb4`: the bend's drop, the curve's pull past the end, the copies' wander.
const DROP: f32 = 10.0;
const PULL: f32 = 12.0;
const WANDER: f32 = 0.25;
/// `$gp+0x1fa8`: the red bolts' half width.
const BOLT: f32 = 0.075;
/// The strands (`0x256320`): four of ten points.
pub const STRANDS: usize = 4;
pub const POINTS: usize = 10;
/// `0x1fe8c0[0x19]`, `[0x2d]`, `[0x61]`: the sparks', the glows' and the sprites' frames.
const SPARK_DEF: u8 = 0x19;
const GLOW_DEF: u8 = 0x2d;
const SPRITE_DEF: u8 = 0x61;
/// The world's box (2..1021).
const LOW: f32 = 2.0;
const HIGH: f32 = 1021.0;

/// The Zapper's globals: on (`0x18b6f4`), its target (`0x18b6f0`), the wait (`0x18b6f8`), the rows' charges (`+0x48`
/// of row 9 / 0x49), the strands (`0x256320`) with their timers (`0x1b2cc0`) and resets (`0x1b2cd0`).
#[derive(Clone, Debug)]
pub struct Mem {
    pub on: bool,
    pub target: Option<usize>,
    pub wait: i32,
    pub charges: [i32; 2],
    pub strands: [[V3; POINTS]; STRANDS],
    pub timers: [i32; STRANDS],
    pub resets: [i32; STRANDS],
}

impl Default for Mem {
    fn default() -> Self {
        Mem { on: false, target: None, wait: 0, charges: [CHARGES; 2], strands: [[[0.0; 3]; POINTS]; STRANDS], timers: [0; STRANDS], resets: [0; STRANDS] }
    }
}

mod pv {
    pub const TARGET: usize = 0x0c;
    pub const LIFE: usize = 0x10;
    pub const INV: usize = 0x14;
    pub const LIGHT: usize = 0x18;
    pub const PLUS: usize = 0x1c;
    pub const END: usize = 0x30;
    pub const BEND: usize = 0x40;
    pub const J6: usize = 0x60;
    pub const J7: usize = 0x70;
    /// Port: behind Ratchet this tick, for the draw (the original's draw tests it again).
    pub const BEHIND: usize = 0x7c;
}

/// The Zapper upgraded: its row's level byte (`0x2624f4 + 0xe0·0x139571`).
fn upgraded(ctx: &Ctx) -> bool { crate::upgrades::level_of(ctx.game.item_row(ITEM)) != 0 }

/// The charges of the row in use.
fn charges<'a>(w: &'a mut MobyWorld, ctx: &Ctx) -> &'a mut i32 { &mut w.zapper.charges[upgraded(ctx) as usize] }

/// `0x2b13c8`'s item 9: on as a charge is taken (`0x2d42d0(9, 1)`).
pub fn request(w: &mut MobyWorld, ctx: &mut Ctx) {
    w.zapper.on = match ctx.game.ammo.get_mut(ITEM as usize) {
        Some(a) if 1 <= *a => { *a -= 1; true }
        _ => false,
    };
}

/// The target's yaw off Ratchet's (`0x2eff48(atan2(target − Ratchet), 0x189eb8)`).
fn off_yaw(ctx: &Ctx, at: V3) -> f32 { rot_dist((at[1] - ctx.hero.pos[1]).atan2(at[0] - ctx.hero.pos[0]), ctx.hero.rot[2]) }

/// A live moby of class type 5 with a health block.
fn zappable(w: &MobyWorld, id: usize) -> bool {
    let Some(m) = w.mobys.get(id) else { return false };
    m.state != 0xfe && m.state != 0xfd && w.class(m.o_class).is_some_and(|c| c.kind == 5) && crate::moby::aim::health(w, id).is_some()
}

/// `0x2b2408` (module docs).
pub fn fire(w: &mut MobyWorld, ctx: &mut Ctx) {
    if ctx.hero.state == 0x31 {
        w.zapper.on = false;
        w.zapper.target = None;
    }
    let had = ctx.game.items.get(ITEM as usize).is_some_and(|&b| b != 0) || ctx.game.items.get(PLUS_ROW).is_some_and(|&b| b != 0);
    if !(w.zapper.on && ctx.hero.mode == 0 && had && matches!(ctx.game.pack(), 2 | 3) && !ctx.hero.body.clank_away) { return; }
    let head = ctx.hero.back.head_point;
    if !countdown(&mut w.zapper.wait) {
        let Some(t) = w.zapper.target.filter(|&t| t < w.mobys.len()) else { return };
        let at = w.mobys[t].pos;
        let off = off_yaw(ctx, at);
        if BEHIND_FIRE < off && upgraded(ctx) {
            if let Some(c) = ctx.hero.back.clank.filter(|&c| c < w.mobys.len()) {
                behind_sparks(w, ctx, c);
            }
        }
        if off < BEHIND_FIRE {
            let Some(sys) = ctx.parts.as_deref_mut() else { return };
            let g = &mut ctx.game.rng;
            let def = sys.def_first(SPARK_DEF);
            for _ in 0..20 {
                let p = [head[0] + g.randf_signed(0.0, 0.04), head[1] + g.randf_signed(0.0, 0.04), head[2] + g.randf_signed(0.0, 0.04)];
                let spin = g.randf_signed(1.0, -1.0) as i32;
                if let Some(i) = crate::particles::spark(sys, g, 126_000.0, 500.0, 1.0, 0.0, 0.0, p, [0.0; 3], 2, 0x7f, 0x20_ff20, spin) { sys.pool.recs[i][2] = def; }
            }
        }
        return;
    }
    let targets = w.targets(ctx.host.camera_pos());
    for t in &targets {
        if !zappable(w, t.id) { continue; }
        if len(sub(w.mobys[t.id].pos, ctx.hero.pos)) <= 16.0 { w.zapper.target = Some(t.id); }
    }
    let Some(t) = w.zapper.target.filter(|&t| zappable(w, t)) else { return };
    let at = w.mobys[t].pos;
    if w.coll_line(ctx, head, at, 0, ctx.hero_moby).is_none() { return; }
    if 16.0 <= len(sub(at, ctx.hero.pos)) { return; }
    if *charges(w, ctx) < 1 {
        w.zapper.on = false;
        *charges(w, ctx) = CHARGES;
    } else {
        let plus = upgraded(ctx);
        spawn(w, ctx, head, t, plus);
        w.zapper.wait = WAIT;
        *charges(w, ctx) -= 1;
    }
}

/// `0x2b2408`'s upgraded sparks behind: Clank's glow and the red sparks from his joints 6 / 7.
fn behind_sparks(w: &mut MobyWorld, ctx: &mut Ctx, c: usize) {
    let m = &mut w.mobys[c];
    m.glow = 0x8000_00ff;
    m.mode |= mode::GLOW;
    let rows = m.rows;
    let through = |v: V3| std::array::from_fn::<f32, 3, _>(|k| v[0] * rows[0][k] + v[1] * rows[1][k] + v[2] * rows[2][k]);
    let mut a = add(ctx.host.joint(&w.mobys[c], 6).unwrap_or(w.mobys[c].pos), through([-0.1, 0.01, 0.04]));
    let mut b = add(ctx.host.joint(&w.mobys[c], 7).unwrap_or(w.mobys[c].pos), through([-0.1, -0.01, 0.04]));
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let g = &mut ctx.game.rng;
    let def = sys.def_first(SPARK_DEF);
    let step = f32::from_bits(0x3b83_126f);
    for _ in 0..30 {
        for k in 0..3 { a[k] += g.randf_signed(0.0, step); }
        for k in 0..3 { b[k] += g.randf_signed(0.0, step); }
        for p in [a, b] {
            let spin = g.randf_signed(1.0, -1.0) as i32;
            if let Some(i) = crate::particles::spark(sys, g, 100_000.0, 500.0, 1.0, 0.0, 0.0, p, [0.0; 3], 2, 0x7f, 0x20_20ff, spin) { sys.pool.recs[i][2] = def; }
        }
    }
}

/// `0x40cd38(from, Ratchet, target, upgraded)` (module docs).
pub fn spawn(w: &mut MobyWorld, ctx: &mut Ctx, from: V3, target: usize, plus: bool) -> Option<usize> {
    w.class(ZAP)?;
    let mut end = w.mobys[target].pos;
    if let Some((_, h)) = crate::moby::aim::health(w, target) { end[2] += h; }
    let id = w.create(ZAP);
    let g = &mut ctx.game.rng;
    let angles = [g.rand_angle(), g.rand_angle(), g.rand_angle()];
    let more = [g.rand_angle(), g.rand_angle(), g.rand_angle()];
    let m = &mut w.mobys[id];
    m.pvars.resize(crate::moby::PVAR_BYTES, 0);
    m.update_dist = 0xff;
    m.mode = 0x100;
    m.draw_dist = 0xff;
    m.drawn = 1;
    m.parent = ctx.hero_moby;
    m.state = 0;
    m.pos = from;
    m.rot = angles;
    m.set_pv_v(pv::END, end);
    m.set_pv_i(pv::LIFE, LIFE);
    m.set_pv_f(pv::INV, 1.0 / LIFE as f32);
    m.set_pv_i(pv::PLUS, plus as i32);
    m.set_pv_i(pv::TARGET, target as i32);
    m.set_pv_i(pv::LIGHT, -1);
    for (k, a) in more.into_iter().enumerate() { m.set_pv_f(4 * k, a); }
    let z = &mut w.zapper;
    for k in 0..3 {
        z.timers[k] = -1;
        z.resets[k] = 0x18;
    }
    Some(id)
}

/// `0x40cf30` (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].pvars.resize(crate::moby::PVAR_BYTES, 0);
    let head = ctx.hero.back.head_point;
    if let Some(c) = ctx.hero.back.clank.filter(|&c| c < w.mobys.len()) {
        let (a, b) = (ctx.host.joint(&w.mobys[c], 6), ctx.host.joint(&w.mobys[c], 7));
        if let Some(a) = a { w.mobys[id].set_pv_v(pv::J6, a); }
        if let Some(b) = b { w.mobys[id].set_pv_v(pv::J7, b); }
    }
    let end = w.mobys[id].pv_v(pv::END);
    let axis = set_len(sub(end, head), 1.0);
    let target_at = w.zapper.target.and_then(|t| w.mobys.get(t)).map_or(end, |m| m.pos);
    let world_yaw = (target_at[1] - ctx.hero.pos[1]).atan2(target_at[0] - ctx.hero.pos[0]);
    let most = if world_yaw < 1.570_796_5 { -f32::from_bits(0x3f49_0fdc) } else { f32::from_bits(0x3f49_0fdc) };
    let a = ctx.game.rng.randf(0.0, most);
    let below = [head[0], head[1], head[2] - DROP];
    let bend = add(head, turn(sub(below, head), axis, a));
    w.mobys[id].set_pv_v(pv::BEND, bend);
    if let Some(t) = w.mobys[id].pv_link(pv::TARGET) {
        let gone = w.mobys.get(t).is_none_or(|m| m.state == 0xfe || m.state == 0xfd || len(sub(m.pos, w.mobys[id].pos)) == 0.0);
        if gone { w.mobys[id].set_pv_i(pv::TARGET, 0); }
    }
    let off = off_yaw(ctx, target_at);
    let behind = BEHIND_DEG * 0.017_453_294 < off;
    let plus = w.mobys[id].pv_i(pv::PLUS) != 0;
    w.mobys[id].set_pv_i(pv::BEHIND, behind as i32);
    match w.mobys[id].state {
        0 => {
            if behind && !plus {
                w.delete(id);
                return;
            }
            let up = upgraded(ctx);
            let flags = if up { 0x11_0000 } else { 0x1_0000 };
            let damage = crate::hero::items::row(ctx.game, ITEM).map_or(16.0, |r| r.damage);
            let mut t = HitTemplate::new(damage, Some(id), flags, [0.0, 0.0, 0.0, 5627.925]);
            t.kind = (ZAP as u16 as u32) << 16 | 0x0101;
            let Some(o) = w.coll_line(ctx, head, end, 0, ctx.hero_moby) else {
                w.delete(id);
                return;
            };
            let count = match o.moby {
                None => Some(0xf),
                Some(m) => {
                    w.deliver_hit_any(m, &t, [o.point[0], o.point[1], o.point[2], 1.0]);
                    w.mobys[m].has_collision.then_some(10)
                }
            };
            if let Some(n) = count { hit_effect(ctx, o.point, n, behind); }
            ctx.host.play_sound_of(&w.mobys[id], 0, 0x15, 0);
            let mid = add(head, scale(sub(end, head), 0.5));
            let m = &mut w.mobys[id];
            m.state = 1;
            m.mode |= 0x41;
            m.set_pv_i(pv::LIFE, LIFE);
            let color = if behind { [1.0, 0.1, 0.1] } else { [0.1, 1.0, 0.1] };
            let s = w.point_lights.alloc(PointLight { color, intensity: 0.0, pos: mid, radius: 10.0 }).map_or(-1, |s| s as i32);
            w.mobys[id].set_pv_i(pv::LIGHT, s);
        }
        1 => {
            let mut life = w.mobys[id].pv_i(pv::LIFE);
            let out = countdown(&mut life);
            w.mobys[id].set_pv_i(pv::LIFE, life);
            if out {
                w.mobys[id].set_pv_i(pv::TARGET, 0);
                let s = w.mobys[id].pv_i(pv::LIGHT);
                if s != -1 { w.point_lights.free(s as usize); }
                w.delete(id);
                return;
            }
            if behind {
                if plus { behind_sprites(w, id, ctx); }
            } else {
                ahead_sprites(ctx, head);
            }
            strands(w, id, ctx);
            w.register_draw(id);
        }
        _ => {}
    }
    let p = w.mobys[id].pos;
    if p.iter().any(|&c| !(LOW..=HIGH).contains(&c)) { w.delete(id); }
}

/// `0x321c00(a, out, v, axis)`: `v` turned by `a` about `axis`.
fn turn(v: V3, axis: V3, a: f32) -> V3 {
    if a.abs() < 1e-5 || len(axis) == 0.0 { return v; }
    crate::gravity::rotate(v, unit(axis), a)
}

/// The point `t` of the way from `a` to `b` (`a + set_len(b − a, t·|b − a|)`).
fn lerp(a: V3, b: V3, t: f32) -> V3 { add(a, scale(sub(b, a), t)) }

/// `0x40cf30` state 1's sprites behind (module docs).
fn behind_sprites(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let (end, j6, j7) = (m.pv_v(pv::END), m.pv_v(pv::J6), m.pv_v(pv::J7));
    let target = m.pv_link(pv::TARGET).is_some();
    let frame = ctx.game.frame;
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let g = &mut ctx.game.rng;
    let (d7, d6, glow) = (sys.def_frame(SPRITE_DEF, 7), sys.def_frame(SPRITE_DEF, 6), sys.def_first(GLOW_DEF));
    let k = 210_000.0;
    for _ in 0..10 {
        let p = lerp(j6, end, g.randf(0.0, 1.0));
        let q = lerp(j7, end, g.randf(0.0, 1.0));
        let spin = g.randi(0xff) as u8;
        let s = g.randf(f32::from_bits(0x3eb3_3333), 0.5);
        for at in [p, q] {
            if let Some(i) = crate::particles::sprite(sys, g, s * k, s * 42_000.0, 0.0, at, [0.0; 3], 3, 0x20_20af, spin, 0x30, 0, 0) { sys.pool.recs[i][2] = d7; }
        }
    }
    for _ in 0..20 {
        let mut p = lerp(j6, end, g.randf(0.0, 1.0));
        let mut q = lerp(j7, end, g.randf(0.0, 1.0));
        let spin = g.randi(0xff) as u8;
        let s = g.randf(0.2, f32::from_bits(0x3eb3_3333));
        let j = f32::from_bits(0x3ca3_d70a);
        for c in p.iter_mut() { *c += g.randf_signed(0.0, j); }
        for (size, rgb) in [(s, 0x10_10ffu32), (s * 0.5, 0x80_8080)] {
            if let Some(i) = crate::particles::sprite(sys, g, size * k, size * 42_000.0, 0.0, p, [0.0; 3], 3, rgb, spin, 0x30, 0, 0) { sys.pool.recs[i][2] = d6; }
        }
        for c in q.iter_mut() { *c += g.randf_signed(0.0, j); }
        for (size, rgb) in [(s * 0.5, 0x10_10ffu32), (s * 0.25, 0x80_8080)] {
            if let Some(i) = crate::particles::sprite(sys, g, size * k, size * 42_000.0, 0.0, q, [0.0; 3], 3, rgb, spin, 0x30, 0, 0) { sys.pool.recs[i][2] = d6; }
        }
    }
    if target && frame % 5 == 0 {
        let p = lerp(j6, end, g.randf(0.0, 1.0));
        let q = lerp(j7, end, g.randf(0.0, 1.0));
        for at in [p, q] {
            if let Some(i) = crate::particles::sprite(sys, g, 2100.0, f32::from_bits(0x47f6_1801), 0.0, at, [0.0; 3], 0x1e, 0x20_2080, 0, 0x40, 0, 0) { sys.pool.recs[i][2] = glow; }
        }
    }
}

/// `0x40cf30` state 1's sprites ahead: ten pairs at the point on Clank's head.
fn ahead_sprites(ctx: &mut Ctx, head: V3) {
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let g = &mut ctx.game.rng;
    let def = sys.def_first(SPARK_DEF);
    for _ in 0..10 {
        let s = g.randf(0.25, f32::from_bits(0x3ecc_cccd)) * 210_000.0;
        for (rgb, spin) in [(0x10_8010u32, 2u8), (0x20_8020, 0xff)] {
            if let Some(i) = crate::particles::sprite(sys, g, s, s, 0.0, head, [0.0; 3], 2, rgb, spin, 0x7f, 0x10, 0) { sys.pool.recs[i][2] = def; }
        }
    }
}

/// `0x40e1a0(at, n, upgraded)`: the zap's hit — 20 glow puffs drifting against the gravity and 20 sparks thrown toward
/// the camera's side (green, red upgraded). (Its `n` is not read; the upgraded shock moby 0x0f6d is not ported.)
fn hit_effect(ctx: &mut Ctx, at: V3, _n: i32, plus: bool) {
    let rgb = if plus { 0x7f20_20ff } else { 0x7f20_ff20 };
    let cam = ctx.host.camera_pos();
    let yaw = (cam[1] - at[1]).atan2(cam[0] - at[0]);
    let down = ctx.hero.gravity.down;
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let g = &mut ctx.game.rng;
    let def = sys.def_frame(SPRITE_DEF, 6);
    let k = 210_000.0;
    for _ in 0..20 {
        let v = scale(down, -(g.randf(0.2, 3.0) * 0.016_666_668));
        let spin = g.randf_signed(1.0, 2.0) as i32;
        let size = g.randf(0.3, 0.75) * k;
        let p = [at[0] + g.randf_signed(0.0, 0.2), at[1] + g.randf_signed(0.0, 0.2), at[2] + g.randf_signed(0.0, 0.2)];
        if let Some(i) = crate::particles::puff(sys, g, 0.0, 1.0, f32::from_bits(0x3f81_47ae), size, p, spin, v, 0x7f20_2020, 0) {
            let r = &mut sys.pool.recs[i];
            r[10..12].copy_from_slice(&0x5au16.to_le_bytes());
            r[3] = 0x44;
            r[9] = 0x44;
            r[0x3c..0x40].copy_from_slice(&5.0f32.to_le_bytes());
            r[0x24..0x28].copy_from_slice(&3u32.to_le_bytes());
            r[0x2a] = 0x30;
            r[0x2b] = 0x5a;
        }
        let f = g.randf(1.0, 2.0);
        let pitch = g.randf(0.0, f32::from_bits(0x3fb2_b8c4));
        let off = g.randf_signed(0.0, f32::from_bits(0x3fb2_b8c4));
        let v = super::shot::polar(f * 0.016_666_668, crate::hero::physics::add_rot(yaw, off), pitch);
        let q = [at[0] + g.randf_signed(0.0, 0.1), at[1] + g.randf_signed(0.0, 0.1), at[2] + g.randf_signed(0.0, 0.1)];
        let size = g.randf(0.25, f32::from_bits(0x3f33_3333)) * k;
        if let Some(i) = crate::particles::spark(sys, g, size, 500.0, 1.0, 0.0, 0.0, q, v, 0x1e, 0x7f, rgb & 0xff_ffff, spin) { sys.pool.recs[i][2] = def; }
    }
}

/// `0x40c898` (module docs).
fn strands(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let head = ctx.hero.back.head_point;
    let (end, bend) = (w.mobys[id].pv_v(pv::END), w.mobys[id].pv_v(pv::BEND));
    let g = &mut ctx.game.rng;
    let z = &mut w.zapper;
    for k in 0..STRANDS {
        if !countdown(&mut z.timers[k]) {
            let s = &mut z.strands[k];
            let mut d = [[0.0f32; 3]; POINTS - 1];
            for (j, dj) in d.iter_mut().enumerate() {
                *dj = sub(s[j + 1], s[j]);
                for c in dj.iter_mut() { *c += g.randf_signed(0.0, f32::from_bits(0x3d4c_cccd)); }
            }
            s[0] = head;
            for j in 0..POINTS - 1 { s[j + 1] = add(s[j], d[j]); }
        } else {
            z.timers[k] = z.resets[k];
            let mut c = end;
            for v in c.iter_mut() { *v += g.randf(-0.5, 0.5); }
            let pull = add(c, set_len(sub(c, head), PULL));
            for j in 0..POINTS {
                let t = if j == POINTS - 1 { 1.0 } else { j as f32 * 0.1 };
                z.strands[k][j] = super::zodiac::curve(t, head, c, bend, pull);
            }
        }
        for o in 1..STRANDS {
            for j in 2..9 {
                let p = z.strands[0][j];
                z.strands[o][j] = [p[0] + g.randf_signed(0.0, WANDER), p[1] + g.randf_signed(0.0, WANDER), p[2] + g.randf_signed(0.0, WANDER)];
            }
        }
    }
}

/// `0x40dcd8` (module docs).
pub fn draw(w: &mut MobyWorld, id: usize, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let m = &w.mobys[id];
    let (behind, plus) = (m.pv_i(pv::BEHIND) != 0, m.pv_i(pv::PLUS) != 0);
    if behind {
        if !plus { return; }
        let (pos, end, life) = (m.pos, m.pv_v(pv::END), m.pv_i(pv::LIFE) as u32);
        let rgba = life << 27 | 0x0a_0aff;
        let st = [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]];
        for j in [m.pv_v(pv::J6), m.pv_v(pv::J7)] {
            let c = set_len(cross(sub(pos, cam), sub(end, j)), BOLT);
            let q = crate::fx::FxQuad { fx: 0x17, additive: true, corners: [add(end, c), sub(end, c), add(j, c), sub(j, c)], st, rgba: [rgba; 4] };
            out.push(q);
            out.push(q);
        }
        return;
    }
    let z = &w.zapper;
    let (first, rest) = if plus {
        ((0.2, f32::from_bits(0x3eb3_3333), 0x46, 0x10_ff10), (0.1, f32::from_bits(0x3e42_8f5c), 0x20, 0xff_1010))
    } else {
        ((f32::from_bits(0x3d8f_5c29), f32::from_bits(0x3e19_999a), 0x32, 0x20_ff20), (f32::from_bits(0x3cf5_c28f), f32::from_bits(0x3db8_51ec), 0xf, 0x10_ffff))
    };
    for (k, s) in z.strands.iter().enumerate() {
        let (a, b, alpha, rgb) = if k == 0 { first } else { rest };
        strip(s, a, b, 0.2, alpha, alpha, 0x7f_7f7f, rgb, cam, out);
    }
}

/// `0x40e558(w1, w2, ext, points, α1, α2, n, c1, c2)`: the strand as two camera-facing strips — FX 0xe `w1` across
/// through its first n − 1 points in `c1` (alpha α1), fading in over the first segment and out over the last; FX 0x10
/// `w2` across over every segment, its ends pushed to the farther one's distance from the camera and each stretched
/// `ext` of its length both ways, in `c2` (α2), fading in over the first and out over the last.
#[allow(clippy::too_many_arguments)]
fn strip(p: &[V3; POINTS], w1: f32, w2: f32, ext: f32, a1: u32, a2: u32, c1: u32, c2: u32, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let n = POINTS;
    let st = [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]];
    let full1 = c1 | a1 << 24;
    let across = |a: V3, b: V3, w: f32| {
        let c = cross(sub(b, a), sub(a, cam));
        let l = len(c);
        scale(c, if l != 0.0 { w / l } else { 0.0 })
    };
    let c = across(p[0], p[1], w1);
    let mut prev = (add(p[0], c), sub(p[0], c));
    let mut rgba = [full1; 4];
    for i in 1..n - 1 {
        let c = across(p[i], p[i + 1], w1);
        let cur = (add(p[i], c), sub(p[i], c));
        if i == 1 {
            rgba = [c1, c1, rgba[2], rgba[3]];
        } else if i == n - 2 {
            rgba = [rgba[0], rgba[1], c1, c1];
        } else if i == 2 {
            rgba = [full1; 4];
        }
        out.push(crate::fx::FxQuad { fx: 0xe, additive: true, corners: [prev.0, prev.1, cur.0, cur.1], st, rgba });
        prev = cur;
    }
    let full2 = c2 | a2 << 24;
    let mut rgba = [full2; 4];
    for i in 0..n - 1 {
        let (mut e0, mut e1) = (sub(p[i], cam), sub(p[i + 1], cam));
        let (l0, l1) = (len(e0), len(e1));
        if l1 < l0 {
            if l1 == 0.0 { e1 = e0 } else { e1 = scale(e1, l0 / l1) }
        } else if l0 == 0.0 {
            e0 = e1;
        } else {
            e0 = scale(e0, l1 / l0);
        }
        let (mut q0, mut q1) = (add(cam, e0), add(cam, e1));
        let d = sub(q1, q0);
        let push = scale(d, ext);
        q0 = sub(q0, push);
        q1 = add(q1, push);
        let x = cross(sub(cam, q0), d);
        let l = len(x);
        let a = scale(x, if l != 0.0 { w2 / l } else { 0.0 });
        if i == 0 {
            rgba[0] = c2;
            rgba[1] = c2;
        } else if i == n - 2 {
            rgba[2] = c2;
            rgba[3] = c2;
        } else if i == 1 {
            rgba[0] = full2;
            rgba[1] = full2;
        }
        out.push(crate::fx::FxQuad { fx: 0x10, additive: true, corners: [add(q0, a), sub(q0, a), add(q1, a), sub(q1, a)], st, rgba });
    }
}
