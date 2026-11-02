//! The Meteor Gun, the Lava Gun's upgrade (item 0x1d's rows 0x3f, 0x5f, 0x60, crate::hero::items::UPGRADE_ROWS; L00
//! `0x265c20`: attachment 0, stance 2, use sequence 0x36, the firing arm's 0x46 crouched, 4.5 a second (7.5), damage 8
//! (12, 24), 200 meteors (300, 400)): the hand moby **0x0c59** (`0x3eb9c0`, called by the item pass while it is the hand
//! item) and the meteor it lobs **0x0b2e** (`0x3dc568`, made by `0x3dd3a0`, drawn by `0x3ddaa8`).
//!
//! **The hand** (pvars: `+0x00` the cooldown in seconds, `+0x04` its trail slot, `+0x0c` 0). Outside Ratchet's look
//! stance 0x1d the look wait (`0x18c354`) is set to 0x28, in it counted down; its sequence run out, back to 1 (blend 2).
//! State **0** clears the cooldown, takes the class's trail slot (crate::moby::classes::chopper::Pool, `0x32b1b8`), →
//! **1**. In 1 the cooldown runs down (`0x2eeb40`); out, the look wait out, in the hand 0x17 ticks, ○ held, the items
//! not kept away and a meteor taken (`0x2d42d0`): the meteor ([`spawn`], `0x3dd3a0`) from joint 0, its class sound 0,
//! its sequence 2 (unless on it), its playback speed 2, the meteor the hand's (`+0xb8`), the cooldown 1 / the row's rate.
//! Else ○ pressed with none left (`0x2d43e0`) and the items not kept away: its class sound 2.
//!
//! **The meteor** (pvars: `+0x00` its velocity, `+0x10..+0x4c` its trail's four points, `+0x50` the face it met,
//! `+0x60` its life, `+0x64` / `+0x68` its spin and the spin a tick, `+0x6c` the scorch's alpha, `+0x70` / `+0x74` the
//! hand's trail slot and class, `+0x7c` it met a moby). Made: update and draw distance 0xff, a random spin
//! (0.37..0.63 a tick), its size 0.01..0.0125, its glow word `0x7f4080ff`, its rows kept; thrown ([`aim`], `0x3dd610`)
//! at the target the search picks (`0x2ae900(8, Ratchet's yaw, 22.5°, 22.5°, .., class type 5)`) through
//! `0x37e328(15.5, 8..15.5, 15 a second, 11 a second², ..)` (crate::moby::classes::gravity_bomb::launch_with); none
//! found: Ratchet's look stance 1 goes to 0x1d and the throw turns ±11.25° about his up. Its scorch's alpha 0x60, its
//! life 680 ticks.
//! - **0** → **1**, every trail point at its place.
//! - **1** (flying): it moves and falls 11 a second² (`0x37ec68`); the trail's points each move a fifth of the way to
//!   the one before, the first its place; its spin on, its rows from the angles (¼, ½, ⅜ of it), their diagonal
//!   stretched (× 0.4, 0.5, 0.75 of 2 + sin / cos / sin); its glow word a random grey; one tick in ten a lava spark
//!   (crate::particles::lava) near it. Its life out: gone. The line from its last place (flags 0, not itself), else a
//!   0.05 sphere: a face, Ratchet, a moby without mode 0x1000 / 0x4000 or the Lava Gun's hand: it stops there, hidden
//!   (mode 0x41), six sparks off the face; a target: the row's damage along Ratchet to it (flags 0x910000, kind
//!   `class << 16 | 0x0105`), three sparks, the moby flag. Either way **2** for 15 ticks, the burst ([`burst`],
//!   `0x3dd830`: its class sound 0, 24 streaks fanned about the face, 24 embers up off it).
//! - **2** (burning): each tick a smoke puff off the face (particle type 0x6c) and three scorches on it (FX 0xb,
//!   `0x103080`, ¾..1.2 · 1, · ¾, · ½, alphas 0x3f..0x7f, random rolls; drawn by the update itself, `0x3ddf18`); the
//!   trail's points close in (a fifth, after a moby a half); its size +0.0005. Its life out: gone.
//!
//! Flying, its smoke: four puffs (type 0x17: 210000, `0x7f204080`), three (157500, `0x7f4080ff`), two (105000, white),
//! 5 / 2 / 2 ticks. Its trail slot no longer its hand's (`0x32b388`): gone; else its draw is asked for.
//!
//! **The draw** ([`draw`], `0x3ddaa8`): the trail ([`trail`], `0x3de098`: from its place through the four points, two
//! strips across the view, FX 0x2b added, 0.2125 and twice as wide at its place, × 0.7 a point, `0x103080` at 0x7f and
//! 0x3f fading in and out at the ends); flying, a glow (FX 0xb, its size · 95, `0x103080` at 0x17..0x1b, a random roll,
//! 1 back from the camera); burning, its scorch's alpha down a tick and, on a face, two scorches (its size · 45
//! `0x103080`, · 33.75 `0x3f3f3f`).
//!
//! Its glow in the hand (`0x2a7ed8`'s case 0x1d upgraded, crate::hero::items): the word pulses `0xff00ffff` ..
//! `0xff104080` slower and a sprite at (−0.07, −0.67, 0), 99750, pull 0.15, its alpha 0x30 uncapped.
//!
//! Not ported: the hand's draw `0x3ebc50` (two glows at `0x1b2b20`, a word nothing writes: the world's origin); the
//! lock cheat (`0x18b270` → `0x18aa18`, `0x32b4d0`); the first-person target search (`0x2ae668`); the targets of class
//! 0xa8d / 0xa8e (`0x37e328`'s health-block height); the throw's jitter `0x3dd3a0` works out and drops (its three draws
//! are made); the pools `0x3dd7a0` clears in the trail slot's memory and the ten records `0x3dd720` ages (nothing writes
//! them); the gravity frames' trackers and bends (`0x37f538` / `0x37f618` / `0x37f5f8`, `0x380e08`); the weapon mods'
//! effects on a hit (`0x3f9f70`).

use crate::hero::items::{row, LAVA};
use crate::hero::physics::{add, add_rot, atan2, cross, dist, dist2, euler_rows, rot_dist, scale, set_len, sub, unit, V3};
use crate::moby::hits::HitTemplate;
use crate::moby::{countdown, mode, Ctx, MobyWorld};

/// The hand's class (rows 0x3f / 0x5f / 0x60 `+0x04`) and the meteor's.
pub const HAND_CLASS: i16 = 0x0c59;
pub const METEOR: i16 = 0x0b2e;

/// `$gp+0x1394` / `+0x1398` / `+0x139c` / `+0x13a0` / `+0x13a4`: the throw's speed a second, the jitter's least speed,
/// the search's range (the reach's least), the least reach, the gravity a second².
const SPEED: f32 = 15.0;
const JITTER_SPEED: f32 = 10.0;
const RANGE: f32 = 8.0;
const LEAST: f32 = 15.5;
const GRAVITY: f32 = 11.0;
/// The life (ticks) and the burning's.
const LIFE: i32 = 0x2a8;
const BURN: i32 = 0xf;
/// `t[3]` of the hits' template (`0x45afdf66`).
const NO_KNOCK_LIMIT: f32 = 5627.925;
/// `$gp+0x13a8` / `+0x13ac`: the ALPHA words the particles take in turn by `rand() % 2`.
const ALPHAS: [u8; 2] = [0x44, 0x48];
/// The quads' template (`0x3ddc70`): a ±1 square across the local x, and its ST.
const SQUARE: [V3; 4] = [[0.0, -1.0, -1.0], [0.0, -1.0, 1.0], [0.0, 1.0, -1.0], [0.0, 1.0, 1.0]];
const ST: [[f32; 2]; 4] = [[0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [1.0, 1.0]];

mod pv {
    pub const VEL: usize = 0x00;
    pub const POINTS: usize = 0x10;
    pub const NORMAL: usize = 0x50;
    pub const LIFE: usize = 0x60;
    pub const SPIN: usize = 0x64;
    pub const SPIN_STEP: usize = 0x68;
    pub const FADE: usize = 0x6c;
    pub const SLOT: usize = 0x70;
    pub const HAND: usize = 0x74;
    pub const MET_MOBY: usize = 0x7c;
}

/// `0x2eeb40(+0x00)`: the cooldown down a frame (1/60 s); true when out.
fn cooldown(w: &mut MobyWorld, id: usize) -> bool {
    let c = w.mobys[id].pv_f(0);
    if 1.0 / 60.0 < c {
        w.mobys[id].set_pv_f(0, c - 1.0 / 60.0);
        false
    } else {
        w.mobys[id].set_pv_f(0, 0.0);
        true
    }
}

/// `0x3eb9c0` (module docs).
pub fn update_hand(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].pvars.resize(crate::moby::PVAR_BYTES, 0);
    let look_ok = if ctx.hero.state == 0x1d {
        countdown(&mut ctx.hero.items.look_wait)
    } else {
        ctx.hero.items.look_wait = 0x28;
        true
    };
    if let Some(a) = ctx.host.anim(&w.mobys[id]) {
        if a.seq_a == a.seq_b && a.flags & 2 != 0 { ctx.host.set_anim(&w.mobys[id], 1, 0, 2); }
    }
    if w.mobys[id].state == 0 {
        let class = w.mobys[id].o_class;
        let mut pool = std::mem::take(&mut w.chopper);
        let (slot, _) = pool.slot(w, class);
        w.chopper = pool;
        let m = &mut w.mobys[id];
        m.state = 1;
        m.set_pv_i(0x0c, 0);
        m.set_pv_f(0, 0.0);
        m.set_pv_i(0x04, slot as i32);
    } else if w.mobys[id].state != 1 {
        return;
    }
    let out = cooldown(w, id);
    let it = ctx.hero.items;
    let fire = out && look_ok && 0x17 <= it.item_ticks && it.held & it.trigger != 0 && it.no_use == 0 && ctx.hero.items.take_ammo(ctx.game, 1);
    if !fire {
        let it = ctx.hero.items;
        if it.pressed & it.trigger != 0 && !it.has_ammo(ctx.game) && it.no_use == 0 { ctx.host.play_sound(&w.mobys[id], 2, 0); }
        return;
    }
    let dir = scale(w.mobys[id].rows[1], -1.0);
    let muzzle = ctx.host.joint(&w.mobys[id], 0).unwrap_or(w.mobys[id].pos);
    let slot = w.mobys[id].pv_i(0x04);
    let meteor = spawn(w, ctx, id, slot, muzzle, dir);
    ctx.host.play_sound(&w.mobys[id], 0, 0);
    if ctx.host.anim(&w.mobys[id]).is_none_or(|a| a.seq_b != 2) { ctx.host.set_anim(&w.mobys[id], 2, 0, 0); }
    ctx.host.set_anim_speed(&w.mobys[id], 2.0);
    if let Some(m) = meteor { w.mobys[m].parent = Some(id); }
    let rate = row(ctx.game, LAVA).map_or(1.0, |r| r.rate);
    w.mobys[id].set_pv_f(0, 1.0 / rate);
}

/// `0x3dd3a0(hand, slot, at, dir)`: the meteor (module docs), None when its class is not loaded.
fn spawn(w: &mut MobyWorld, ctx: &mut Ctx, hand: usize, slot: i32, at: V3, _dir: V3) -> Option<usize> {
    if w.class(METEOR).is_none() {
        eprintln!("rc_game::meteor: class {METEOR:#x} is not loaded: no meteor");
        return None;
    }
    let id = w.create(METEOR);
    let g = &mut ctx.game.rng;
    let spin = g.rand_angle();
    let step = g.randf(f32::from_bits(0x3ebe_a2f4), f32::from_bits(0x3f20_d97d));
    let size = g.randf(0.01, f32::from_bits(0x3c4c_cccd));
    let hand_class = w.mobys[hand].o_class;
    let m = &mut w.mobys[id];
    m.update = Some(update);
    m.mode = (m.mode & !mode::NO_UPDATE) | mode::KEEP_ROWS;
    m.pvars.resize(crate::moby::PVAR_BYTES, 0);
    m.state = 0;
    m.draw_dist = 0xff;
    m.update_dist = 0xff;
    m.drawn = 1;
    m.pos = at;
    m.scale = size;
    m.glow = 0x7f40_80ff;
    m.set_pv_f(pv::SPIN, spin);
    m.set_pv_f(pv::SPIN_STEP, step);
    m.set_pv_i(pv::MET_MOBY, 0);
    m.set_pv_i(pv::HAND, hand_class as i32);
    m.set_pv_i(pv::SLOT, slot);
    let vel = aim(w, ctx, at);
    // The jitter the original works out from the gun's way and drops.
    let g = &mut ctx.game.rng;
    for _ in 0..2 { g.randf(f32::from_bits(0xbe19_999a), f32::from_bits(0x3e19_999a)); }
    g.randf(JITTER_SPEED * 0.016_666_668, SPEED * 0.016_666_668);
    let m = &mut w.mobys[id];
    m.set_pv_v(pv::VEL, vel);
    m.set_pv_i(pv::FADE, 0x60);
    m.set_pv_i(pv::LIFE, LIFE);
    w.rebuild(id, &*ctx.host);
    w.changed = true;
    Some(id)
}

/// `0x2ae900(range, yaw, yaw cone, pitch cone, .., class type 5, no line)` (crate::hero::Hero::target_search's score,
/// third-person): of the target list, the moby of class type 5 with health left, within `range` and both cones of
/// Ratchet's yaw, with the lowest `d + d·(yaw off + pitch off)` (a crate's + 0.8·range).
fn search(w: &MobyWorld, ctx: &Ctx, range: f32, yaw_cone: f32, pitch_cone: f32) -> Option<usize> {
    let h = &*ctx.hero;
    let eye = if h.state == 0x14 { [h.pos[0], h.pos[1], h.ground_z] } else { h.pos };
    let mut best = 1e8f32;
    let mut out = None;
    for t in w.targets(ctx.host.camera_pos()) {
        if t.kind != 5 || crate::moby::aim::health(w, t.id).is_none_or(|(hp, _)| hp == 0.0) { continue; }
        let d = dist(eye, t.pos);
        let a_yaw = rot_dist(atan2(t.pos[0] - eye[0], t.pos[1] - eye[1]), h.rot[2]);
        let a_pitch = rot_dist(atan2(dist2(eye, t.pos), t.pos[2] - eye[2]), 0.0);
        let skip = range < d || (0.0 < yaw_cone && yaw_cone < a_yaw) || (0.0 < pitch_cone && pitch_cone < a_pitch);
        let mut score = d + (a_yaw + a_pitch) * d;
        if t.crate_like || t.kind == 0x16 { score += range * 0.8; }
        if skip || best <= score { continue; }
        best = score;
        out = Some(t.id);
    }
    out
}

/// `0x3dd610(meteor, vel)`: the throw (module docs).
fn aim(w: &MobyWorld, ctx: &mut Ctx, from: V3) -> V3 {
    let cone = f32::from_bits(0x3ec9_0fdb);
    let target = search(w, ctx, RANGE, cone, cone);
    let reach = ctx.game.rng.randf(LEAST, RANGE);
    let v = super::gravity_bomb::launch_with(ctx, LEAST, reach, SPEED * 0.016_666_668, GRAVITY * 0.000_277_777_78, from, target.map(|t| w.mobys[t].pos));
    if target.is_some() { return v; }
    if ctx.hero.state == 1 { ctx.hero.request_state(0x1d, true); }
    let up = ctx.hero_moby.and_then(|h| w.mobys.get(h)).map_or([0.0, 0.0, 1.0], |h| h.rows[2]);
    let a = ctx.game.rng.randf(f32::from_bits(0xbe49_0fdb), f32::from_bits(0x3e49_0fdb));
    crate::gravity::rotate(v, unit(up), a)
}

/// The trail's points close in on the one before (the newest its place).
fn close_in(w: &mut MobyWorld, id: usize, t: f32) {
    let m = &mut w.mobys[id];
    for k in (1..4).rev() {
        let (a, b) = (m.pv_v(pv::POINTS + 0x10 * k), m.pv_v(pv::POINTS + 0x10 * (k - 1)));
        m.set_pv_v(pv::POINTS + 0x10 * k, add(a, scale(sub(b, a), t)));
    }
    let p = m.pos;
    m.set_pv_v(pv::POINTS, p);
}

/// Sparks off a point (`0x336148(-1, at, vel, 0)`): along `normal` at 1/6..1/3, jittered ±0.15 on each axis, at
/// 1/30..0.1 a tick.
fn sparks(ctx: &mut Ctx, at: V3, normal: V3, n: usize) {
    let g = &mut ctx.game.rng;
    let j = (f32::from_bits(0xbe19_999a), f32::from_bits(0x3e19_999a));
    for _ in 0..n {
        let mut v = set_len(normal, g.randf(f32::from_bits(0x3e2a_aaab), f32::from_bits(0x3eaa_aaab)));
        for x in v.iter_mut() { *x += g.randf(j.0, j.1); }
        v = set_len(v, g.randf(f32::from_bits(0x3d08_8889), f32::from_bits(0x3dcc_ccce)));
        if let Some(sys) = ctx.parts.as_deref_mut() { crate::particles::lava::spark(sys, g, -1.0, at, v, false); }
    }
}

/// `0x3dc568` (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].pvars.resize(crate::moby::PVAR_BYTES, 0);
    if w.mobys[id].state == 0 {
        let m = &mut w.mobys[id];
        let p = m.pos;
        for k in 0..4 { m.set_pv_v(pv::POINTS + 0x10 * k, p); }
        m.state = 1;
    }
    match w.mobys[id].state {
        1 => {
            if !fly(w, id, ctx) { return; }
        }
        2 => {
            if !burn(w, id, ctx) { return; }
        }
        _ => {}
    }
    if w.mobys[id].state == 1 { smoke(w, id, ctx); }
    let (class, slot) = (w.mobys[id].pv_i(pv::HAND) as i16, w.mobys[id].pv_i(pv::SLOT));
    if usize::try_from(slot).is_ok_and(|s| w.chopper.holds(class, s)) {
        w.register_draw(id);
    } else {
        w.delete(id);
    }
}

/// State 1 of `0x3dc568`; false when the meteor is gone.
fn fly(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    let prev = w.mobys[id].pos;
    let vel = w.mobys[id].pv_v(pv::VEL);
    let pos = add(prev, vel);
    w.mobys[id].pos = pos;
    // `0x37ec68(11/3600, pos, vel)`: the fall along the up at the new place.
    let vel = sub(vel, scale(ctx.hero.gravity.up_at(pos), GRAVITY * 0.000_277_777_78));
    w.mobys[id].set_pv_v(pv::VEL, vel);
    close_in(w, id, 0.2);
    {
        let g = &mut ctx.game.rng;
        let m = &mut w.mobys[id];
        let s = add_rot(m.pv_f(pv::SPIN), m.pv_f(pv::SPIN_STEP));
        m.set_pv_f(pv::SPIN, s);
        let mut rows = euler_rows([s * 0.25, s * 0.5, s * 0.375]);
        rows[0][0] *= 0.4 * (s.sin() + 2.0);
        rows[1][1] *= 0.5 * (s.cos() + 2.0);
        rows[2][2] *= 0.75 * (s.sin() + 2.0);
        m.rows = rows;
        m.glow = g.rand_range(0x7f, 0xff) as u32 * 0x1_0101 | 0x7f00_0000;
    }
    if ctx.game.rng.rand() as i32 % 10 == 0 {
        let g = &mut ctx.game.rng;
        let j = (f32::from_bits(0xbe19_999a), f32::from_bits(0x3e19_999a));
        let at = [pos[0] + g.randf(j.0, j.1), pos[1] + g.randf(j.0, j.1), pos[2] + g.randf(j.0, j.1)];
        let k = 0.025;
        let v = [g.randf(-k, k), g.randf(-k, k), g.randf(0.0, k)];
        let v = set_len(v, 0.05);
        if let Some(sys) = ctx.parts.as_deref_mut() { crate::particles::lava::spark(sys, g, -1.0, at, v, false); }
    }
    let mut life = w.mobys[id].pv_i(pv::LIFE);
    let gone = countdown(&mut life);
    w.mobys[id].set_pv_i(pv::LIFE, life);
    if gone {
        w.delete(id);
        return false;
    }
    let met = w.coll_line(ctx, prev, pos, 0, Some(id)).or_else(|| ctx.coll.and_then(|c| crate::collision::coll_sphere_m(c, ctx.scene, pos, 0.05, crate::collision::QueryFlags(0), Some(id))));
    let Some(o) = met else { return true };
    let normal = unit(o.normal);
    let target = o.moby.filter(|&t| Some(t) != ctx.hero_moby && w.mobys.get(t).is_some_and(|x| x.mode & (mode::TARGET | mode::LINE) != 0 && x.o_class != super::lava_gun::HAND_CLASS));
    {
        let m = &mut w.mobys[id];
        m.pos = o.point;
        m.state = 2;
        m.mode |= 0x41;
        m.set_pv_v(pv::NORMAL, normal);
    }
    match target {
        None => sparks(ctx, o.point, normal, 6),
        Some(t) => {
            let damage = row(ctx.game, LAVA).map_or(0.0, |r| r.damage);
            let from = ctx.hero_moby.and_then(|h| w.mobys.get(h)).map_or(ctx.hero.pos, |h| h.pos);
            let d = set_len(sub(w.mobys[t].pos, from), 1.0);
            let mut tm = HitTemplate::new(damage, Some(id), 0x91_0000, [d[0], d[1], d[2], NO_KNOCK_LIMIT]);
            tm.kind = (w.mobys[t].o_class as u16 as u32) << 16 | 0x0105;
            w.deliver_hit_any(t, &tm, [0.0; 4]);
            w.mobys[id].set_pv_i(pv::MET_MOBY, 1);
            sparks(ctx, o.point, normal, 3);
        }
    }
    w.mobys[id].set_pv_i(pv::LIFE, BURN);
    burst(w, id, ctx, o.point, normal);
    true
}

/// `0x3dd830(meteor, 15, at, normal)` (module docs): its class sound 0; 24 streaks (type 0x6a, 0.0417 a tick, 20 ticks)
/// from 0.15 off the face, fanned about it 2π/23 apart from a random start; 24 embers (type 0x6b) rising off it, the
/// k-th to 1.5·k/23, 30..50 ticks; its velocity the face's unit normal.
fn burst(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, at: V3, normal: V3) {
    ctx.host.play_sound(&w.mobys[id], 0, 0);
    let c = add(at, set_len(normal, 2.0));
    let across = set_len(cross(c, at), 1.0);
    let base = add(at, set_len(normal, f32::from_bits(0x3e19_999a)));
    let up = set_len(scale(normal, 1.0 / 1024.0), 1.0);
    let g = &mut ctx.game.rng;
    let mut a = g.rand_angle();
    for _ in 0..24 {
        let v = crate::gravity::rotate(across, unit(normal), a);
        let alpha = ALPHAS[(g.rand() as i32 % 2) as usize];
        if let Some(sys) = ctx.parts.as_deref_mut() { crate::particles::meteor::streak(sys, g, f32::from_bits(0x3d2a_aaab), base, v, alpha, 0x14); }
        a = add_rot(a, f32::from_bits(0x3e8b_de82));
    }
    for k in 0..24 {
        let far = k as f32 * 0.065_217_39;
        let alpha = ALPHAS[(g.rand() as i32 % 2) as usize];
        let life = g.rand_range(0x1e, 0x32);
        if let Some(sys) = ctx.parts.as_deref_mut() { crate::particles::meteor::ember(sys, g, far, at, up, alpha, life); }
    }
    w.mobys[id].set_pv_v(pv::VEL, up);
}

/// State 2 of `0x3dc568`; false when the meteor is gone.
fn burn(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    let (pos, up, normal) = { let m = &w.mobys[id]; (m.pos, m.pv_v(pv::VEL), m.pv_v(pv::NORMAL)) };
    let c = add(set_len(up, 2.0), pos);
    let across = set_len(cross(c, pos), 1.0);
    let g = &mut ctx.game.rng;
    let a = g.rand_angle();
    let across = crate::gravity::rotate(across, unit(up), a);
    let s = g.randf(f32::from_bits(0x3c08_8889), f32::from_bits(0x3d4c_ccce));
    let alpha = ALPHAS[(g.rand() as i32 % 2) as usize];
    let life = g.rand_range(0x1e, 0x32);
    if let Some(sys) = ctx.parts.as_deref_mut() { crate::particles::meteor::smoke(sys, g, s, pos, up, across, alpha, life); }
    // `0x3ddf18` ×3: the scorches, drawn by the update.
    let hi = f32::from_bits(0x3f99_999a);
    for (k, lo, hi_a) in [(1.0f32, 0x3f, 0x5f), (0.75, 0x4f, 0x6f), (0.5, 0x5f, 0x7f)] {
        let s = g.randf(0.75, hi) * k;
        let a = g.rand_range(lo, hi_a) as u32;
        let roll = g.rand_angle();
        w.update_quads.push(scorch(s, roll, pos, 0x10_3080 | a << 24, normal));
    }
    let mut life = w.mobys[id].pv_i(pv::LIFE);
    let gone = countdown(&mut life);
    w.mobys[id].set_pv_i(pv::LIFE, life);
    if gone {
        w.delete(id);
        return false;
    }
    let t = if w.mobys[id].pv_i(pv::MET_MOBY) == 0 { 0.2 } else { 0.5 };
    close_in(w, id, t);
    w.mobys[id].scale += 0.0005;
    true
}

/// Flying, the smoke of `0x3dc568`'s tail: three sets of puffs (type 0x17) at its place, each `(count, roll range,
/// size, colour, life, ALPHA's)`.
fn smoke(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let at = w.mobys[id].pos;
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let g = &mut ctx.game.rng;
    let sets: [(usize, i32, f32, f32, u32, i16, u8); 3] = [
        (4, 5, 0.2, 210_000.0, 0x7f20_4080, 5, 0x30),
        (3, 3, 0.05, 157_500.0, 0x7f40_80ff, 2, 0x40),
        (2, 3, 0.05, 105_000.0, 0x7fff_ffff, 2, 0x60),
    ];
    for (n, roll, jitter, size, rgba, life, a) in sets {
        for _ in 0..n {
            let r = g.randi(roll);
            let spin = if g.randi(2) == 0 { r } else { -r };
            if let Some(i) = crate::particles::puff(sys, g, jitter, 1.0, f32::from_bits(0x3f66_6666), size, at, spin, [0.0; 3], rgba, 1) {
                let rec = &mut sys.pool.recs[i];
                rec[0xa..0xc].copy_from_slice(&life.to_le_bytes());
                rec[9] = 4 + 0x40;
                rec[0x24..0x28].copy_from_slice(&2u32.to_le_bytes());
                rec[0x2a] = a;
                rec[0x2b] = rec[0xa];
            }
        }
    }
}

/// A quad of the template × `size` in the Euler frame `(roll, pitch, yaw)` at `at`.
fn quad(size: f32, rot: V3, at: V3, rgba: u32) -> crate::fx::FxQuad {
    let rows = euler_rows(rot);
    let corners = SQUARE.map(|c| add(at, crate::gravity::to_world(scale(c, size), &rows)));
    crate::fx::FxQuad { fx: 0xb, additive: true, corners, st: ST, rgba: [rgba; 4] }
}

/// `0x3ddf18(size, roll, at, rgb, alpha, 0xb, 2, normal)`: a quad lying on the face (its x along the normal, `roll`
/// about it), 0.025 off it; FX 0xb, added.
fn scorch(size: f32, roll: f32, at: V3, rgba: u32, n: V3) -> crate::fx::FxQuad {
    let yaw = n[1].atan2(n[0]);
    let pitch = -n[2].atan2((n[0] * n[0] + n[1] * n[1]).sqrt());
    quad(size, [roll, pitch, yaw], add(at, set_len(n, 0.025)), rgba)
}

/// `0x3ddd60(size, roll, push, at, rgb, alpha, 0xb, 2)`: a quad facing the camera, `push` toward it; FX 0xb, added.
fn glow(size: f32, roll: f32, push: f32, at: V3, cam: V3, rgba: u32) -> crate::fx::FxQuad {
    let d = sub(cam, at);
    let yaw = d[1].atan2(d[0]);
    let pitch = -d[2].atan2(dist2(at, cam));
    quad(size, [roll, pitch, yaw], add(at, set_len(d, push)), rgba)
}

/// `0x3de098(0, 0.85, 0.25, at, points, 0x103080, 4)`: the trail (module docs).
fn trail(out: &mut Vec<crate::fx::FxQuad>, rng: &mut crate::rng::Rng, at: V3, points: [V3; 4], cam: V3) {
    const RGB: u32 = 0x10_3080;
    let (s_step, mut w) = (0.25f32, f32::from_bits(0x3f59_999a) * 0.25);
    let (ta, tb) = if rng.rand() & 1 == 0 { (0.0, 1.0) } else { (1.0, 0.0) };
    let across = |p: V3, next: V3| set_len(cross(set_len(sub(next, p), 1.0), sub(cam, p)), 1.0);
    let corners = |p: V3, a: V3, w: f32| [add(p, scale(a, w)), sub(p, scale(a, w)), add(p, scale(a, 2.0 * w)), sub(p, scale(a, 2.0 * w))];
    let mut a = across(at, points[0]);
    let mut prev = corners(at, a, w);
    let mut c_prev = (RGB, RGB);
    for k in 0..4 {
        let p = points[k];
        if k < 3 { a = across(p, points[k + 1]); }
        w *= 0.7;
        let now = corners(p, a, w);
        let mut s0 = s_step * (k + 1) as f32;
        let mut s1 = s0 + s_step;
        if 1.0 <= s0 {
            s0 -= 1.0;
            s1 -= 1.0;
        }
        let st = [[s0, ta], [s0, tb], [s1, ta], [s1, tb]];
        let c_now = if k == 0 || k == 3 { (RGB, RGB) } else { (RGB | 0x7f00_0000, RGB | 0x3f00_0000) };
        out.push(crate::fx::FxQuad { fx: 0x2b, additive: true, corners: [prev[0], prev[1], now[0], now[1]], st, rgba: [c_prev.0, c_prev.0, c_now.0, c_now.0] });
        out.push(crate::fx::FxQuad { fx: 0x2b, additive: true, corners: [prev[2], prev[3], now[2], now[3]], st, rgba: [c_prev.1, c_prev.1, c_now.1, c_now.1] });
        prev = now;
        c_prev = if k == 0 { (RGB, RGB) } else { (RGB | 0x7f00_0000, RGB | 0x3f00_0000) };
    }
}

/// `0x3ddaa8` (module docs).
pub fn draw(w: &mut MobyWorld, id: usize, rng: &mut crate::rng::Rng, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let state = w.mobys[id].state;
    if state != 1 && state != 2 { return; }
    if state == 2 {
        let m = &mut w.mobys[id];
        let f = m.pv_i(pv::FADE) - 1;
        m.set_pv_i(pv::FADE, f.max(0));
    }
    let m = &w.mobys[id];
    let points = [0, 1, 2, 3].map(|k| m.pv_v(pv::POINTS + 0x10 * k));
    trail(out, rng, m.pos, points, cam);
    if state == 1 {
        let a = (rng.rand_range(0x5f, 0x6f) >> 2) as u32;
        let roll = rng.rand_angle();
        out.push(glow(m.scale * 95.0, roll, -1.0, m.pos, cam, 0x10_3080 | a << 24));
    } else if m.pv_i(pv::MET_MOBY) == 0 {
        let a = (m.pv_i(pv::FADE) as u32) << 24;
        let n = m.pv_v(pv::NORMAL);
        out.push(scorch(m.scale * 45.0, 0.0, m.pos, 0x10_3080 | a, n));
        out.push(scorch(m.scale * 33.75, 0.0, m.pos, 0x3f_3f3f | a, n));
    }
}
