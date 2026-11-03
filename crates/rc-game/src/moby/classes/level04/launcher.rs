//! Barlow's launcher **0x0a58** (L04 `0x3ef548`, 2 placed; level 6's 0x0a50 shares the unit) and the carrier
//! **0x0e44** (`0x43d3f8`, made by `0x30d1d0`) that brings each of its mobys in: a glow lobbed on an arc from the
//! launcher to where the moby lands, a blast there, then the moby growing out of the ground.
//!
//! The launcher's pvars: health `+0x20`, the flash `+0x50`, its reach `+0x70`, the cuboid list `+0x74` and polygon
//! path `+0x78` that keep its target (−1 none), the reload `+0x7c` and its timer `+0x7e`, the most out at once `+0x80`
//! and how many are `+0x81`, `+0x82` ≠ 0 the landing cuboid nearest Ratchet (else a random one), `+0x83` ≠ 0 its
//! mobys at a tenth of their size, `+0x84` the group its mobys come from (its members, all deleted on its first tick,
//! are brought back one by one: `0x3ef1f8`), `+0x90..+0x9c` four landing cuboids (−1 none), `+0xa0..+0xf0` twenty
//! moby links (0x0b66 counted out), the target block `+0xf0` (its place, angles, aim and centre; its moby `+0x130`,
//! `+0x134` 0 Ratchet, 1 a decoy, 2 none), `+0x140` how many it has sent.
//!
//! The L04 engine functions with the port's: `0x33f8d0` the hit test, `0x341d18` / `0x341dd8` the flash's start and
//! tick, `0x33e480` / `0x33e388` its group's members (crate::moby::MobyWorld::groups), `0x343c18` the target search
//! ([`find`]), `0x333b18` a joint's place, `0x33e898` ground_z, `0x33cf18` / `0x33cf88` / `0x3433c8` the random
//! vectors, `0x433d50` = L00 `0x3f8348` (crate::moby::blast::burst), `0x30ebb0` the glow sprite (crate::fx::sprite),
//! `0x310cf0` the colour lerp (crate::hud::lerp_rgba), `0x340700` approach.
//!
//! Not ported: the decoys' list the search reads (`0x1b2630`: the port takes every 0x00cb / 0x076c moby), Ratchet's
//! `0x18c0b4` = 0x6f exclusion, the carrier's rides on floor mobys (`0x343fe0`, as crate::moby::classes::level04::
//! skitter's), its revived moby's link back to it (`+0xb8`, read by nothing ported), and the arc speed it works out
//! and keeps nowhere (`0x33fc30`).

use crate::hero::physics::{add, approach, dist, euler_rows, len, scale, set_len, sub, V3};
use crate::moby::classes::props::{seq, set_seq, wrapped};
use crate::moby::{countdown, mode, Ctx, MobyWorld};

pub const CLASS: i16 = 0x0a58;
pub const CARRIER: i16 = 0x0e44;
const FLASH: usize = 0x50;
const REACH: usize = 0x70;
const CUBOIDS: usize = 0x74;
const POLY: usize = 0x78;
const RELOAD: usize = 0x7c;
const TIMER: usize = 0x7e;
const MOST: usize = 0x80;
const OUT: usize = 0x81;
const NEAREST: usize = 0x82;
const SMALL: usize = 0x83;
const POOL: usize = 0x84;
const LANDINGS: usize = 0x90;
const LINKS: usize = 0xa0;
const T_POS: usize = 0xf0;
const T_MOBY: usize = 0x130;
const T_FOUND: usize = 0x134;
const SENT: usize = 0x140;
const PVARS: usize = 0x150;

fn members(w: &MobyWorld, g: i32) -> Vec<usize> { usize::try_from(g).ok().and_then(|g| w.groups.get(g)).cloned().unwrap_or_default() }
fn dead(w: &MobyWorld, k: usize) -> bool { w.mobys.get(k).is_none_or(|m| 0x80 <= m.state) }

/// `0x3ef438`: a hit (flags 0x10000) sets it off (2, its flash's colour 0x78); the record let go; the flash's tick.
fn hit(w: &mut MobyWorld, id: usize) {
    if w.hit_message(id).is_some_and(|h| h.tmpl.flags & 0x10000 != 0) {
        let m = &mut w.mobys[id];
        m.state = 2;
        m.set_pv_b(FLASH + 7, 0x78);
        crate::moby::ai::flash_start(m, FLASH);
    }
    w.clear_hit(id);
    crate::moby::ai::flash(&mut w.mobys[id], FLASH);
}

/// `0x343c18(reach, m, block, cuboids, n, path)`: the target: none (2) in Ratchet's state 0x18 or outside the path's
/// polygon (with no cuboids); else the nearest decoy in its state 3 within the reach (and the polygon) (1), else Ratchet
/// (0). The block: its place, its angles, the aim (Ratchet: his aim point and centre; a decoy: 0.6 over it, twice).
fn find(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let m = &w.mobys[id];
    let (mut reach, poly, cub) = (m.pv_f(REACH), m.pv_i(POLY), m.pv_i(CUBOIDS) != -1);
    let inside = |p: V3| cub || poly == -1 || ctx.level.in_polygon(poly, p);
    let mut found = 0;
    let mut target = ctx.hero_moby;
    if ctx.hero.state == 0x18 || !inside(ctx.hero.pos) {
        target = None;
        found = 2;
    }
    let here = m.pos;
    for (k, o) in w.mobys.iter().enumerate() {
        if !matches!(o.o_class, 0x00cb | 0x076c) || o.state != 3 { continue; }
        let d = dist(here, o.pos);
        if d < reach && inside(o.pos) {
            reach = d;
            target = Some(k);
            found = 1;
        }
    }
    let m = &mut w.mobys[id];
    crate::moby::ai::set_link(m, T_MOBY, target);
    m.set_pv_i(T_FOUND, found);
    match target {
        None => for k in 0..4 { m.set_pv_v(T_POS + 0x10 * k, [0.0; 3]); },
        Some(t) if Some(t) == ctx.hero_moby => {
            m.set_pv_v(T_POS, ctx.hero.pos);
            m.set_pv_v(T_POS + 0x10, ctx.hero.rot);
            m.set_pv_v(T_POS + 0x20, ctx.hero.aim_point);
            m.set_pv_v(T_POS + 0x30, add(ctx.hero.pos, [0.0, 0.0, 1.0]));
        }
        Some(t) => {
            let (p, r) = (w.mobys[t].pos, w.mobys[t].rot);
            let m = &mut w.mobys[id];
            m.set_pv_v(T_POS, p);
            m.set_pv_v(T_POS + 0x10, r);
            let up = add(p, [0.0, 0.0, 0.6]);
            m.set_pv_v(T_POS + 0x30, up);
            m.set_pv_v(T_POS + 0x20, up);
        }
    }
}

/// `0x3ef4a8`: its count out (its group's live members, and its live 0x0b66 links); −1 when its last link is a live
/// 0x0b66, else the last free link.
fn count(w: &mut MobyWorld, id: usize) -> i32 {
    let live = members(w, w.mobys[id].pv_i(POOL)).into_iter().filter(|&k| !dead(w, k)).count();
    let mut n = live as u8;
    let mut free = -1;
    for k in 0..0x14 {
        let l = crate::moby::ai::link(&w.mobys[id], LINKS + 4 * k);
        if l.is_some_and(|o| w.mobys[o].o_class == 0x0b66 && w.mobys[o].state < 0xfd) {
            n += 1;
        } else {
            free = k as i32;
        }
    }
    w.mobys[id].set_pv_b(OUT, n);
    free
}

/// `0x3ef1f8(m, pos)`: one of its group's dead members brought back at `pos`: state 0, its class's mode and scale
/// (a tenth with `+0x83`), update and draw distances 0xff, drawn with collision, its sequence from the start, its
/// glow and shadow flags, no hit; its launcher `+0xb8`; Ratchet's colour words; its health full.
fn revive(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, pos: V3) -> Option<usize> {
    let k = members(w, w.mobys[id].pv_i(POOL)).into_iter().find(|&k| dead(w, k) && k < w.mobys.len())?;
    let m = &mut w.mobys[id];
    m.set_pv_b(OUT, m.pv_b(OUT).wrapping_add(1));
    let small = m.pv_b(SMALL) != 0;
    let class = w.class(w.mobys[k].o_class).cloned();
    let (light, ambient) = ctx.hero_moby.and_then(|h| w.mobys.get(h)).map_or((0, [0x80; 3]), |h| (h.light, h.ambient));
    let o = &mut w.mobys[k];
    o.pos = pos;
    o.state = 0;
    o.timer = 0;
    if let Some(c) = class.as_ref() {
        o.mode = c.mode;
        o.scale = c.scale;
        if c.glow != 0 { o.mode |= mode::GLOW; }
        if c.shadow.is_some() { o.mode |= 0x400; }
        o.has_collision = c.has_collision;
        if c.update.is_some() { o.update = c.update; }
    }
    if small { o.scale *= 0.1; }
    o.update_dist = 0xff;
    o.draw_dist = 0xff;
    o.drawn = 1;
    o.hit_slot = -1;
    o.light = light;
    o.ambient = ambient;
    o.drops = 0;
    if o.mode & crate::moby::ai::AI != 0 {
        let h = crate::moby::ai::health(o);
        let most = o.pv_s(h + 4) as f32;
        o.set_pv_f(h, most);
        o.set_pv_f(h + 0x20, 0.0);
        o.set_pv_f(h + 0x24, 0.0);
    }
    ctx.host.set_anim(&w.mobys[k], 0, 0, 0);
    w.rebuild(k, &*ctx.host);
    Some(k)
}

/// 0x0a58 (`0x3ef548`). **0**: its health 1 and `+0x29` 1, its group's members all deleted (the most out at once no
/// more than them), 1. **1**: hit ([`hit`]); at the end of its sequence 1, sequence 0; each reload: one tick in seven
/// the target search ([`find`]), else its target's place (a target gone: none); with a target and room (fewer out
/// than its most): a landing point — a landing cuboid (the first set from a random one, or the nearest Ratchet with
/// `+0x82`) at a random place in it, else a random point within its reach (`+0x82`: half way to the target, 1 to 1
/// along) — on the ground under it (from 3 up); the next moby brought back at its place ([`revive`]) and carried
/// there from its joint 0 (1 up) on an arc 3 high over 60 ticks ([`carry`]); its count up, the reload again, its
/// class sound 0, sequence 1; after 18, 2. **2**: a blast (0.75), its class sound 1, its bolts, gone.
pub fn update_0a58(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pvars.len() < PVARS { w.mobys[id].pvars.resize(PVARS, 0); }
    hit(w, id);
    if w.mobys[id].drawn != 0 { crate::shadow_volumes::probe_near(w, id, ctx, 24.0, 0x16); }
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.set_pv_b(0x29, 1);
            m.set_pv_f(0x20, 1.0);
            m.set_pv_i(SENT, 0);
            let list = members(w, w.mobys[id].pv_i(POOL));
            let mut n = 0;
            for k in list {
                if k != id && !dead(w, k) { w.delete(k); n += 1; }
            }
            let m = &mut w.mobys[id];
            if n < m.pv_b(MOST) as i32 { m.set_pv_b(MOST, n as u8); }
            m.state = 1;
        }
        1 => step(w, id, ctx),
        2 => {
            let pos = w.mobys[id].pos;
            let b = crate::moby::blast::Blast { size: 0.75, bursts: [0x19, 0x14, 0x14], puffs: 0x14, streaks: 0x14, light: false, shake: false };
            crate::moby::blast::burst(w, ctx, pos, &b);
            w.mobys[id].set_pv_f(0x20, 0.0);
            ctx.host.play_sound(&w.mobys[id], 1, 0);
            crate::moby::bolts::drop(w, id, 0, 3, ctx);
            w.delete(id);
        }
        _ => {}
    }
}

fn step(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    hit(w, id);
    if seq(w, id, ctx) == 1 && wrapped(w, id, ctx) { ctx.host.set_anim(&w.mobys[id], 0, 0, 5); }
    let mut t = w.mobys[id].pv_s(TIMER) as i32;
    let ready = countdown(&mut t);
    w.mobys[id].set_pv_s(TIMER, t as i16);
    if !ready { return; }
    if ctx.game.rng.randi(7) == 0 {
        find(w, id, ctx);
    } else {
        match crate::moby::ai::link(&w.mobys[id], T_MOBY).filter(|&t| !dead(w, t)) {
            None => {
                let m = &mut w.mobys[id];
                crate::moby::ai::set_link(m, T_MOBY, None);
                m.set_pv_i(T_FOUND, 2);
            }
            Some(t) => {
                let p = w.mobys[t].pos;
                w.mobys[id].set_pv_v(T_POS, p);
            }
        }
    }
    if w.mobys[id].pv_i(T_FOUND) == 2 { return; }
    let free = count(w, id);
    let m = &w.mobys[id];
    if m.pv_b(MOST) <= m.pv_b(OUT) || free == -1 { return; }
    let pos = m.pos;
    let landings: Vec<i32> = (0..4).map(|k| m.pv_i(LANDINGS + 4 * k)).collect();
    let mut cub: Option<usize> = None;
    let mut e;
    if m.pv_b(NEAREST) == 0 {
        let s = ctx.game.rng.rand_range(0, 3);
        cub = (0..4).map(|i| landings[((i + s) % 4) as usize]).find(|&c| c != -1).and_then(|c| usize::try_from(c).ok());
        e = [0.0; 3];
        if cub.is_none() {
            let r = w.mobys[id].pv_f(REACH).max(1.0);
            let rng = &mut ctx.game.rng;
            let a = rng.rand_angle();
            let u = (((rng.rand() >> 16) & 0xfff) as f32) * 0.000_244_140_62;
            let l = (1.0 - u * u) * r;
            e = add([a.cos() * l, a.sin() * l, 0.0], pos);
        }
    } else {
        let hero = ctx.hero.pos;
        let mut best = 100_000.0;
        for c in landings.iter().filter_map(|&c| usize::try_from(c).ok()) {
            let Some((p, _)) = ctx.level.cuboid_place(c as i32) else { continue };
            let d = dist(hero, p);
            if d < best { best = d; cub = Some(c); }
        }
        e = [0.0; 3];
        if cub.is_none() {
            let mut d = scale(sub(w.mobys[id].pv_v(T_POS), pos), 0.5);
            if 1.0 < w.mobys[id].pv_f(REACH) && 1.0 < len(d) { d = set_len(d, 1.0); }
            if len(d) < 1.0 { d = set_len(d, 1.0); }
            e = add(d, pos);
        }
    }
    if let Some(c) = cub.and_then(|c| ctx.level.cuboids.get(c)) {
        let (r0, r1) = ([c.matrix[0][0], c.matrix[0][1], c.matrix[0][2]], [c.matrix[1][0], c.matrix[1][1], c.matrix[1][2]]);
        e = [c.matrix[3][0], c.matrix[3][1], c.matrix[3][2]];
        let a = ctx.game.rng.randf(-0.5, 0.5);
        e = add(e, scale(r0, a));
        let b = ctx.game.rng.randf(-0.5, 0.5);
        e = add(e, scale(r1, b));
    }
    let g = crate::moby::ai::ground_z(ctx, 3.0, e, 0);
    if g != 0.0 { e[2] = g; }
    let mut from = ctx.host.joint(&w.mobys[id], 0).unwrap_or(pos);
    {
        let rng = &mut ctx.game.rng;
        // `0x3433c8(0.1, ..)` on a vector nothing keeps: its draws.
        for _ in 0..3 { rng.randf(-0.1, 0.1); }
    }
    let Some(k) = revive(w, id, ctx, pos) else { return };
    if let Some(c) = w.class(w.mobys[k].o_class) { w.mobys[k].scale = c.scale; }
    let r = ctx.game.rng.rand_vec(2.0, 3.0);
    let to = add([r[0], r[1], 0.0], e);
    from[2] += 1.0;
    carry(w, ctx, k, from, [0.0, 0.0, 3.0], to, [0.0, 0.0, 3.0], [0.0; 3], 0x3c);
    let m = &mut w.mobys[id];
    let n = m.pv_i(SENT) + 1;
    m.set_pv_b(OUT, m.pv_b(OUT).wrapping_add(1));
    m.set_pv_s(TIMER, m.pv_s(RELOAD));
    m.set_pv_i(SENT, n);
    if 0x12 < n { m.set_state(2, -1); }
    ctx.host.play_sound(&w.mobys[id], 0, 0);
    set_seq(w, id, ctx, 1, 0, 5);
}

/// `0x30d1d0(m, from, up₀, to, up₁, rot, ticks)`: a carrier 0x0e44 for `m`, which is put at `to` with the angles
/// `rot`, hidden without collision and kept still (mode | 0x103); the carrier keeps the arc, `m`, its mode and its
/// colour (`| 0x80000000`), and its step `1 / ticks`.
#[allow(clippy::too_many_arguments)]
fn carry(w: &mut MobyWorld, ctx: &mut Ctx, m: usize, from: V3, up0: V3, to: V3, up1: V3, rot: V3, ticks: i32) {
    let c = w.create(CARRIER);
    let o = &mut w.mobys[m];
    o.pos = to;
    o.rot = rot;
    o.rows = euler_rows(rot);
    let (mode, amb) = (o.mode, o.ambient);
    o.has_collision = false;
    o.drawn = 0;
    o.mode |= 0x103;
    let cm = &mut w.mobys[c];
    // The class has no model in the level's list: its update is set here (as crate::moby::classes::death_fade's).
    cm.update = Some(update_0e44);
    cm.mode &= !mode::NO_UPDATE;
    cm.update_dist = 0xff;
    cm.pos = to;
    cm.set_pv_v(0, from);
    cm.set_pv_v(0x10, up0);
    cm.set_pv_v(0x20, to);
    cm.set_pv_v(0x30, up1);
    crate::moby::ai::set_link(cm, 0x48, Some(m));
    cm.set_pv_i(0x50, mode as i32);
    cm.set_pv_i(0x54, (amb[0] as u32 | (amb[1] as u32) << 8 | (amb[2] as u32) << 16 | 0x8000_0000) as i32);
    cm.set_pv_f(0x5c, 1.0 / ticks as f32);
    let _ = ctx;
}

/// 0x0e44 (`0x43d3f8`). **0**: 1. **1**: drawn ([`draw`]); its progress `+0x4c` on by its step; at 1, 2 and a blast
/// at its landing (0.5, bursts 20 / 40 / 20, 40 puffs, 40 streaks, its light). **2**: its moby shown with collision,
/// its rows from its angles with the up row × the growth `+0x58`, its colour from blue (`0x800000ff`) to its own by it,
/// the growth on by 2 a second; at 1 its moby's mode and colour back, the carrier gone.
pub fn update_0e44(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => w.mobys[id].set_state(1, -1),
        1 => {
            w.register_draw(id);
            let m = &mut w.mobys[id];
            let mut t = m.pv_f(0x4c);
            approach(1.0, m.pv_f(0x5c), &mut t);
            m.set_pv_f(0x4c, t);
            if 1.0 <= t {
                m.set_state(2, -1);
                let at = m.pv_v(0x20);
                let b = crate::moby::blast::Blast { size: 0.5, bursts: [0x14, 0x28, 0x14], puffs: 0x28, streaks: 0x28, light: true, shake: false };
                crate::moby::blast::burst(w, ctx, at, &b);
            }
        }
        2 => {
            let Some(k) = crate::moby::ai::link(&w.mobys[id], 0x48).filter(|&k| k < w.mobys.len()) else { return w.delete(id) };
            let g = w.mobys[id].pv_f(0x58);
            let saved = w.mobys[id].pv_i(0x54) as u32;
            if 1.0 <= g {
                let md = w.mobys[id].pv_i(0x50) as u16;
                let o = &mut w.mobys[k];
                o.mode = md;
                o.set_ambient_word(saved);
                return w.delete(id);
            }
            let has = w.class(w.mobys[k].o_class).is_some_and(|c| c.has_collision);
            let o = &mut w.mobys[k];
            o.mode &= !1;
            o.drawn = 1;
            o.has_collision = has;
            let mut r = euler_rows(o.rot);
            r[2] = scale(r[2], g);
            o.rows = r;
            o.set_ambient_word(crate::hud::lerp_rgba(g, 0x8000_00ff, saved));
            let m = &mut w.mobys[id];
            let mut g = g;
            approach(1.0, 2.0 * 0.016_666_668, &mut g);
            m.set_pv_f(0x58, g);
        }
        _ => {}
    }
}

/// `0x43d688`: four glows (FX 0x31 / 0x31 / 0xb / 0xb, sizes 1 / 0.6 / 1.4 / 0.9, colours lerped at random between
/// their pairs, random rolls) at the arc's point: from its start to its landing by the progress, each end raised by
/// its up vector × sin(progress · π).
pub fn draw(w: &MobyWorld, id: usize, game: &mut crate::game::GameState, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let m = &w.mobys[id];
    let t = m.pv_f(0x4c);
    let s = (t * std::f32::consts::PI).sin();
    let a = add(m.pv_v(0), scale(m.pv_v(0x10), s));
    let b = add(m.pv_v(0x20), scale(m.pv_v(0x30), s));
    let p = add(a, scale(sub(b, a), t));
    for (size, fx, c0, c1) in [(1.0, 0x31, 0x5010_8080u32, 0x6010_8080u32), (f32::from_bits(0x3f19_999a), 0x31, 0x50ff_ffff, 0x60ff_ffff), (f32::from_bits(0x3fb3_3333), 0xb, 0x0810_3080, 0x1810_3080), (f32::from_bits(0x3f66_6666), 0xb, 0x2000_0080, 0x4000_0080)] {
        let u = game.rng.randf(0.0, 1.0);
        let rgba = crate::hud::lerp_rgba(u, c0, c1);
        let roll = game.rng.rand_angle();
        crate::fx::sprite(out, size, 0.0, roll, p, cam, fx, rgba, true);
    }
}
