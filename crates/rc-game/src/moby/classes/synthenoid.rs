//! The Synthenoids (item 0x1f, crate::hero::items::ROWS; L00's record `0x264020`: class 0xcb2, icon 0xea6d, name
//! 0x32ef, shots 60 a second, 2 a second, damage 0.75, 24 shots each, 12 of them): not held. Asked for (the quick
//! select, the Weapons page: `0x3227d8(0x1f)`), the item change (`0x2b13c8`, crate::hero::items) sends them out
//! instead of taking them in the hand — `0x3ee110(Ratchet + 1.5 up, 0xcb2)` — and Ratchet keeps his weapon.
//!
//! **The deploy** ([`deploy`], `0x3ee110`): four tries of `0x3eca90` ([`make`]): a free one of the four slots
//! (`$gp+0x1758`: none, another class's or a deleted moby), one ammo taken (`0x2d42d0(0x1f, 1)`), a drone **0x0cb2**
//! made at the place: Ratchet its owner (`+0x34`), drawn, far 0xff / 0x7f, his light words (`+0x38`), his rows,
//! angles 0, mode 0x100, a tenth of its scale, its slot (`+0x38`), its shots (`+0x40`: the row's `+0x48`, 24). None
//! made: the class's sound 0 at Ratchet (`0x3603c8`).
//!
//! **The drone** ([`update`], `0x3ed5c0`; pvars: `+0x00` its place in the formation, `+0x10` its velocity, `+0x20`
//! its leaving's ticks, `+0x24` (u8) the muzzle's toggle, `+0x25` (u8) the shots fired, `+0x26` (u8) the ticks stuck,
//! `+0x27` (u8) its loop sound (the port keeps the handle at `+0x70`), `+0x28` its target, `+0x2c` 5400 (no reader),
//! `+0x30` its turn's speed, `+0x34` its owner, `+0x38` its slot, `+0x3c` the cooldown (seconds), `+0x40` its shots,
//! `+0x46` (s16) hidden by a scene, `+0x48` / `+0x4c` its sway's phases, `+0x50` / `+0x54` its bob and phase, `+0x60`
//! Ratchet's place last tick; the timer byte `+0xbc` its move's state). Outside play (`0x1a8f00` ≠ 0) it hides (mode
//! 0x41, `+0x46` = 1) and does nothing; back in play it shows. Its scale eases a tenth a tick to its class's.
//! * **0**: the formation point and velocity cleared, its sequence 0 (blend 10) unless already, far 0xff / 0xff, no
//!   shots, Ratchet's place kept, its loop sound (class sound 1, looped), the phases random, → **1**.
//! * **1**: its sequence run out, its sequence 1 (blend 10); its loop sound kept playing. Ratchet more than 20 from
//!   where he was last tick (not in the state 0x81) — on level 7 riding the moby 0x11d9 it moves with him instead —
//!   it goes at once (its slot freed). With shots left, Ratchet not in the groups 0x16 / 0x14 and in his own body:
//!   its place in the formation ([`formation`], `0x3ecc40`) and the move ([`steer`], `0x3ed0e8`); else its velocity
//!   cleared, 30 ticks, its class sound 2, → **2**. Then its shot ([`fire`], `0x3ecd68`).
//! * **2** (leaving): the loop sound stops; the ticks out, it goes (its slot freed); else a twinkle in a random colour
//!   0.3 about it (`0x33a6a0`: 0.5..3, 20..40 ticks, falling 0.00125), mode 0x200, its alpha down 0x80 / 30 a tick, its
//!   yaw spun 24° a tick.
//!
//! Past state 0: it rides Ratchet's platform (in his state 0x27 by his whole move `0x189f40`; else toward its place
//! plus the platform's step `0x189f70`, by the cosine share of 0.2 and then 0.625 of what is left), its blob shadow
//! (0.3, with the gravity frames off), and while drawn its exhaust from joint 0 (`0x3ed4f8`: three puffs, `0x33d400`,
//! sinking 1..2 a second). Its rows are Ratchet's (in his group 0xf: left as forward, back as left) turned by its yaw,
//! pitch and roll about its own up, left and forward.
//!
//! **The formation** (`0x3ecc40`): at `a` = slot · 60°, Ratchet's place + his left · 1.2 cos a + his up · (1.2 sin a +
//! 2.2) + his forward · 2 sin a: an arc over his head. **The move** (`0x3ed0e8`, by `+0xbc`): 0 turns toward the
//! point ([`face`], `0x3edfb8`: 0.1°, 0.05°, 11° a tick) and goes to 2; 2 turns toward its target (or the point;
//! 1220″, 90″, 1220′ a tick), and steers: its velocity takes 0.13 (0.25 within 1.3 of Ratchet across) of
//! `0.04 · (point − place) − velocity`, the height's 0.9 of that; leaving at most 0.045 a tick; the move against the
//! world (crate::moby::slide, `0x31c618(1, 0.5, 0)`); stopped (under 0.005) over 20 ticks and not drawn, it jumps to
//! the point; within 1.1 of Ratchet across it is pushed out by a tenth of the overlap; another value goes to 0 (2 when
//! farther than 1.3). Then its bob (0.1 · sin of a phase 0.03 a tick, along its up) and sway (`0x3244e8(0.25, 0.025,
//! 0.00625)`).
//!
//! **The shot** (`0x3ecd68`): the cooldown out, on its frame (`0x1b1948 % 4` = its slot): from its joint 1, the
//! search (crate::moby::aim, 20 far, 90° all ways, along the camera's forward row) — not 0x0ee1 / 0x0ec0 / 0x0ed4
//! whose `+0x3b1` is −1, not 0x0ef4 — gives the target (none clears it); its sequence 2 (blend 5); fired once that
//! sequence is in (not in a scene) and the drone more than 1.1 above Ratchet along its up: two sprites at the muzzle
//! (3 ticks, `0x202080` / `0x601080`), the shot ([`spawn`], `0x3ee200`) from 0.3 of the way to the target's aim point
//! (its health block's height, else 0.3 up), its class sound 0, a shot counted, the cooldown 1 / the row's rate +
//! 0..0.17.
//!
//! **The shot 0x0cbb** ([`update_shot`], `0x3ee618`; pvars: `+0x04..+0x0c` its wobble's phases, `+0x1c` its target,
//! `+0x20` its velocity, `+0x2c` its speed, `+0x30` its texture's scroll, `+0x38` its life (20) and `+0x3c` one over
//! it, `+0x44..+0x4c` its spin a tick). Made: far 0xff, drawn, mode 0x100, the drone its owner (`+0xb8`), six times
//! its class's scale, its speed the row's / 60, its angles random, its spin 0..4° / 8° / 16° a tick; along the
//! drone's forward (Ratchet's arm raised at a wall: the drone's −left; a target: toward its aim point); from the look
//! stance's twin with Ratchet hidden, from below the camera toward what its view meets 20 out. A target's health block
//! gets `+0x1e` | 0x80. Each tick: it homes on a live target (gone, or dead and not a crate: dropped); it moves, spins
//! and wobbles (its rows at 0.35 ± 0.15 of their length, 0.5 rad a tick, `0x3ef580`); its line (flags 0, Ratchet left
//! out) hits a moby with the row's damage (flags 0x10000, its kind the class's) — sparks and smoke at the point
//! ([`burst`], `0x3ef018`) — or, its life out, it fizzles ([`fizzle`], `0x3ef360`): either way it hides, its target
//! dropped, 6 more ticks of its draw (→ 1). Flying it leaves two glows (2 ticks), five smoke sprites along its step
//! (15 ticks) and every tenth frame a long glow (30 ticks). Out of the world's 2..1021 box it goes.
//!
//! **Its draw** ([`draw`], `0x3eecd0`): a streak 0.1 wide back along its way 2 (FX 0x17, `0x40108080` → `0x40101080`)
//! and 2 more fading out (to `0x801010`), its texture scrolling 0.1 a tick; over it one 0.6 wide and 4 long (FX 0xe
//! additive, `0x40ff0020` fading out).
//!
//! Not ported: the upgraded Synthenoids (row 0x3d: the drone 0x0b02, `0x3d5230` / `0x3d5e70`, which also replaces a
//! drone in the field), the gravity frames' bends (`0x380e08`, `0x380e58`) and the moving frames (`$gp−0x14e4`), the
//! lock cheat (`0x18b270`), the weapon mods' effects on a hit (`0x4126b8`), the unreachable branch of state 1 (its
//! leaving's ticks past 20 there: a random sequence-1 blend and speed).

use crate::collision::QueryFlags;
use crate::hero::items::{row, ItemRow, SYNTHENOID};
use crate::hero::physics::{add, add_rot, atan2, cross, dist2, dot, len, scale, set_len, sub, turn_spring, V3};
use crate::moby::hits::HitTemplate;
use crate::moby::{aim, countdown, Ctx, MobyWorld};

/// The drone's class (item 0x1f's row `+0x04`) and its shot's.
pub const DRONE: i16 = 0xcb2;
pub const SHOT: i16 = 0xcbb;
/// The shots a drone has (the row's `+0x48`; crate::hero::items::ItemRow does not keep it).
const SHOTS: i32 = 24;
/// The drones in the field at most (`$gp+0x1758`).
pub const SLOTS: usize = 4;
/// `t[3]` of a shot's hit template (`0x45afdf66`).
const NO_KNOCK_LIMIT: f32 = 5627.925;
/// The exhaust's descriptors (`$gp+0x1790`, `+0x17a0`, `+0x17b0`: crate::particles::EXHAUST_DESCS).
const EXHAUST: [usize; 3] = [8, 9, 10];
/// The port's slot for the loop sound's handle (the game's byte `+0x27`).
const SOUND: usize = 0x70;
const DEG: f32 = 0.017_453_294;

/// The Synthenoids' row as `0x139568[0x1f]` gives it (crate::upgrades).
fn cur_row(game: &crate::game::GameState) -> Option<&'static ItemRow> { row(game, SYNTHENOID) }

/// `v` turned by `a` about `axis` (`0x31fe98` + `0x321b28`; under 1e-5 none).
fn turn(v: V3, axis: V3, a: f32) -> V3 {
    if a.abs() <= 1e-5 || len(axis) == 0.0 { return v; }
    crate::gravity::rotate(v, set_len(axis, 1.0), a)
}

/// `0x2eef88(out, a, b)` = b × a.
fn cross_ba(a: V3, b: V3) -> V3 { cross(b, a) }

/// The drone in slot `k` (a live moby of the class holding it).
fn slot(w: &MobyWorld, k: usize) -> Option<usize> { w.mobys.iter().position(|m| m.o_class == DRONE && m.state < 0xfd && m.pvars.len() >= 0x3c && m.pv_i(0x38) == k as i32) }

/// `0x3ee110(Ratchet + 1.5 up, 0xcb2)` (module docs).
pub fn deploy(w: &mut MobyWorld, ctx: &mut Ctx) {
    let mut at = ctx.hero.pos;
    at[2] += 1.5;
    let mut any = false;
    for _ in 0..4 {
        if make(w, ctx, at).is_some() { any = true; }
    }
    if !any {
        if let Some(h) = ctx.hero_moby { ctx.host.play_sound_of(&w.mobys[h], DRONE, 0, 0); }
    }
}

/// `0x3eca90(Ratchet, at)` (module docs).
fn make(w: &mut MobyWorld, ctx: &mut Ctx, at: V3) -> Option<usize> {
    let k = (0..SLOTS).find(|&k| slot(w, k).is_none())?;
    if w.class(DRONE).is_none() { return None; }
    // `0x2d42d0(0x1f, 1)`.
    let a = ctx.game.ammo.get_mut(SYNTHENOID as usize)?;
    if *a < 1 { return None; }
    *a -= 1;
    let id = w.create(DRONE);
    let (light, ambient) = ctx.hero_moby.and_then(|h| w.mobys.get(h)).map_or((0, [0; 3]), |h| (h.light, h.ambient));
    let rows = ctx.hero.rows;
    let m = &mut w.mobys[id];
    m.pvars.resize(crate::moby::PVAR_BYTES, 0);
    m.set_pv_i(0x34, ctx.hero_moby.map_or(0, |h| h as i32));
    m.parent = ctx.hero_moby;
    m.drawn = 1;
    m.update_dist = 0xff;
    m.draw_dist = 0x7f;
    (m.light, m.ambient) = (light, ambient);
    m.rows = rows;
    m.rot = [0.0; 3];
    m.mode |= crate::moby::mode::KEEP_ROWS;
    m.pos = at;
    m.scale *= 0.1;
    m.set_pv_i(0x38, k as i32);
    m.set_pv_f(0x3c, 0.0);
    m.set_pv_i(0x40, SHOTS);
    m.set_pv_i(SOUND, -1);
    m.state = 0;
    Some(id)
}

/// The loop sound let go when still its own and playing (`0x3602f0`).
fn stop_loop(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let s = w.mobys[id].pv_i(SOUND);
    if s != -1 && ctx.host.sound_playing(s) { ctx.host.stop_sound(s); }
    w.mobys[id].set_pv_i(SOUND, -1);
}

/// The loop sound (class sound 1, looped) started when it is not playing.
fn keep_loop(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let s = w.mobys[id].pv_i(SOUND);
    if s == -1 || !ctx.host.sound_playing(s) {
        let h = ctx.host.play_sound(&w.mobys[id], 1, 4);
        w.mobys[id].set_pv_i(SOUND, h);
    }
}

/// The drone gone: its sound, alpha, velocity, mode 0x200, its slot, deleted (`0x311688`).
fn vanish(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    stop_loop(w, id, ctx);
    let m = &mut w.mobys[id];
    m.alpha = 0x80;
    m.set_pv_v(0x10, [0.0; 3]);
    m.mode &= !0x200;
    w.delete(id);
}

/// `0x3ed5c0` (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].pvars.resize(crate::moby::PVAR_BYTES, 0);
    if ctx.game.scenes.mode != 0 {
        let m = &mut w.mobys[id];
        m.set_pv_s(0x46, 1);
        m.mode |= 0x41;
        return;
    }
    if w.mobys[id].pv_s(0x46) == 1 { w.mobys[id].mode &= !0x41; }
    let class_scale = w.class(DRONE).map_or(1.0, |c| c.scale);
    {
        let m = &mut w.mobys[id];
        m.scale += (class_scale - m.scale) * 0.1;
    }
    match w.mobys[id].state {
        0 => start(w, id, ctx),
        1 => {
            if !active(w, id, ctx) { return; }
        }
        2 => {
            stop_loop(w, id, ctx);
            let mut n = w.mobys[id].pv_i(0x20);
            let out = countdown(&mut n);
            w.mobys[id].set_pv_i(0x20, n);
            if out {
                vanish(w, id, ctx);
                return;
            }
            leaving_twinkle(w, id, ctx);
            let m = &mut w.mobys[id];
            m.mode |= 0x200;
            m.alpha = m.alpha.wrapping_sub((0x80 / 30) as u8);
            m.rot[2] = add_rot(m.rot[2], f32::from_bits(0x3ed6_7752));
        }
        _ => {}
    }
    if w.mobys[id].state != 0 { tail(w, id, ctx); }
    set_rows(w, id, ctx);
}

/// State 0 (module docs).
fn start(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    {
        let m = &mut w.mobys[id];
        m.set_pv_v(0, [0.0; 3]);
        m.set_pv_v(0x10, [0.0; 3]);
        m.set_pv_b(0x26, 0);
        m.set_pv_i(0x20, 0);
    }
    if ctx.host.anim(&w.mobys[id]).is_some_and(|a| a.seq_b != 0) { ctx.host.set_anim(&w.mobys[id], 0, 0, 10); }
    let hero_pos = ctx.hero.pos;
    {
        let m = &mut w.mobys[id];
        m.update_dist = 0xff;
        m.draw_dist = 0xff;
        m.set_pv_b(0x25, 0);
        m.set_pv_f(0x30, 0.0);
        m.set_pv_i(0x2c, 5400);
        m.set_pv_v(0x60, hero_pos);
    }
    keep_loop(w, id, ctx);
    let g = &mut ctx.game.rng;
    let (a, b, c) = (g.rand_angle(), g.rand_angle(), g.rand_angle());
    let m = &mut w.mobys[id];
    m.set_pv_v(0x10, [0.0; 3]);
    m.set_pv_f(0x48, a);
    m.set_pv_f(0x4c, b);
    m.set_pv_f(0x54, c);
    m.set_pv_f(0x50, 0.0);
    m.state = 1;
}

/// State 1 (module docs); false when the drone went.
fn active(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    if let Some(a) = ctx.host.anim(&w.mobys[id]) {
        if a.flags & 2 != 0 && a.seq_b != 1 { ctx.host.set_anim(&w.mobys[id], 1, 0, 10); }
    }
    keep_loop(w, id, ctx);
    let hero_pos = ctx.hero.pos;
    let last = w.mobys[id].pv_v(0x60);
    let far = {
        let d = sub(last, hero_pos);
        400.0 < dot(d, d)
    };
    if ctx.hero.state != 0x81 && far {
        if ctx.game.level == 7 && ctx.hero.ground_moby.is_some() && ctx.hero.ground_class == 0x11d9 {
            let m = &mut w.mobys[id];
            m.pos = add(m.pos, sub(hero_pos, last));
        } else {
            vanish(w, id, ctx);
            return false;
        }
    }
    w.mobys[id].set_pv_v(0x60, hero_pos);
    if w.mobys[id].pv_i(0x20) >= 0x15 {
        // Unreachable in state 1 (the ticks are 0 there; module docs): kept as the game has it.
        if ctx.host.anim(&w.mobys[id]).is_some_and(|a| a.flags & 2 != 0) {
            if ctx.host.anim(&w.mobys[id]).is_some_and(|a| a.seq_b != 1) {
                let b = ctx.game.rng.rand_range(7, 12) as u8;
                ctx.host.set_anim(&w.mobys[id], 1, 0, b);
            }
            let s = ctx.game.rng.randf(f32::from_bits(0x3f73_3333), f32::from_bits(0x3f86_6666));
            ctx.host.set_anim_speed(&w.mobys[id], s);
        }
    }
    let m = &w.mobys[id];
    if (m.pv_b(0x25) as i32) < m.pv_i(0x40) && ctx.hero.group != 0x16 && ctx.hero.group != 0x14 && ctx.hero.mode == 0 {
        formation(w, id, ctx);
        steer(w, id, ctx);
    } else {
        let m = &mut w.mobys[id];
        m.set_pv_v(0x10, [0.0; 3]);
        m.set_pv_i(0x20, 30);
        stop_loop(w, id, ctx);
        ctx.host.play_sound(&w.mobys[id], 2, 0);
        w.mobys[id].state = 2;
    }
    fire(w, id, ctx);
    true
}

/// State 2's twinkle (module docs).
fn leaving_twinkle(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (pos, n) = (w.mobys[id].pos, w.mobys[id].pv_i(0x20));
    let g = &mut ctx.game.rng;
    // `0x3217d8(0.3, p)`.
    let p: V3 = std::array::from_fn(|k| pos[k] + g.randf(-0.3, 0.3));
    let (r, gr, b) = (g.rand(), g.rand(), g.rand());
    let rgba = (r.wrapping_add(0x30) & 0x3f) | (gr.wrapping_add(0x20) & 0x3f) << 8 | (b & 0x2f) << 16 | 0xff << 24;
    let s = (g.rand() % 2) as i8;
    let spin = if n & 1 != 0 { -s } else { s };
    let life = g.rand_range(20, 40) as i16;
    let size = g.randf(0.5, 3.0);
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    crate::particles::twinkle(sys, g, size, 0.001, -4.5 * 0.000_277_777_8, p, life, rgba, 2, spin, [0.0; 3], 0);
}

/// The tail past state 0 (module docs): the platform's ride, the shadow, the exhaust.
fn tail(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    {
        let m = &mut w.mobys[id];
        if ctx.hero.state == 0x27 {
            m.pos = add(m.pos, ctx.hero.eff);
        } else {
            let to = add(m.pos, ctx.hero.carry);
            let s = (1.0 - (0.2 * std::f32::consts::PI).cos()) * 0.5;
            let p = add(m.pos, scale(sub(to, m.pos), s));
            m.pos = add(p, scale(sub(to, p), 0.625));
        }
    }
    if !ctx.hero.gravity.on { crate::shadows::request(w, ctx, 0.3, id); }
    // `0x3ed4f8`: while drawn, three exhaust puffs at joint 0 sinking 1..2 a second in its frame.
    if w.mobys[id].drawn != 0 {
        let f = ctx.game.rng.randf(1.0, 2.0);
        let vel = [0.0, 0.0, -(f * 0.016_666_668)];
        let at = ctx.host.joint(&w.mobys[id], 0).unwrap_or(w.mobys[id].pos);
        if let Some(sys) = ctx.parts.as_deref_mut() {
            for d in EXHAUST { crate::particles::exhaust(sys, &mut ctx.game.rng, &w.mobys[id], id, at, vel, d); }
        }
    }
}

/// The rows (module docs).
fn set_rows(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let hr = ctx.hero.rows;
    let m = &mut w.mobys[id];
    let mut r = hr;
    if ctx.hero.group == 0xf {
        r[0] = hr[1];
        r[1] = scale(hr[0], -1.0);
    }
    let [roll, pitch, yaw] = m.rot;
    if 1e-5 < yaw.abs() {
        r[0] = turn(r[0], r[2], yaw);
        r[1] = turn(r[1], r[2], yaw);
    }
    if 1e-5 < pitch.abs() {
        r[0] = turn(r[0], r[1], pitch);
        r[2] = turn(r[2], r[1], pitch);
    }
    if 1e-5 < roll.abs() {
        r[1] = turn(r[1], r[0], roll);
        r[2] = turn(r[2], r[0], roll);
    }
    m.rows = r;
}

/// `0x3ecc40` (module docs): the formation point into `+0x00`.
fn formation(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let a = w.mobys[id].pv_i(0x38) as f32 * (std::f32::consts::PI / 3.0);
    let (s, c) = a.sin_cos();
    let h = ctx.hero.rows;
    let mut p = add(scale(h[1], c * 1.2), ctx.hero.pos);
    p = add(p, scale(h[2], s * 1.2 + 2.2));
    p = add(p, scale(h[0], s + s));
    w.mobys[id].set_pv_v(0, p);
}

/// `0x3edfb8(k, d, most, drone, point, &speed)`: its yaw (relative to Ratchet's facing) sprung toward `point` across
/// Ratchet's up.
fn face(w: &mut MobyWorld, id: usize, ctx: &Ctx, (k, d, most): (f32, f32, f32), point: V3) {
    let h = ctx.hero.rows;
    let m = &mut w.mobys[id];
    let mut v = sub(point, m.pos);
    v = sub(v, scale(h[2], dot(v, h[2])));
    let (f, l) = (dot(v, h[0]), len(v));
    if l == 0.0 { return; }
    let mut a = std::f32::consts::FRAC_PI_2 - (f / l).clamp(-1.0, 1.0).asin();
    if dot(v, h[1]) < 0.0 { a = -a; }
    let (mut yaw, mut sp) = (m.rot[2], m.pv_f(0x30));
    turn_spring(a, k, d, most, &mut yaw, &mut sp, 0);
    m.rot[2] = yaw;
    m.set_pv_f(0x30, sp);
}

/// `0x3ed0e8` (module docs).
fn steer(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let point = w.mobys[id].pv_v(0);
    let pos = w.mobys[id].pos;
    let mut d = sub(point, pos);
    let near = dist2(pos, ctx.hero.pos);
    let slow = (f32::from_bits(0x3ae4_c38a), f32::from_bits(0x3a64_c38a), f32::from_bits(0x3e44_980b));
    match w.mobys[id].timer {
        0 => {
            face(w, id, ctx, slow, point);
            w.mobys[id].timer = 2;
        }
        2 => {
            match w.mobys[id].pv_link(0x28).filter(|&t| w.mobys.get(t).is_some()) {
                None => face(w, id, ctx, slow, point),
                Some(t) => {
                    let p = w.mobys[t].pos;
                    face(w, id, ctx, (1220.0 * 4.848_137_5e-6, 90.0 * 4.848_137_5e-6, 1220.0 * 0.000_290_888_25), p);
                }
            }
            let mut vel = w.mobys[id].pv_v(0x10);
            d = scale(d, f32::from_bits(0x3d23_d70a));
            d = sub(d, vel);
            d = scale(d, if 1.3 < near { f32::from_bits(0x3e05_1eb8) } else { 0.25 });
            d[2] *= 0.9;
            vel = add(vel, d);
            if w.mobys[id].state == 2 {
                let most = f32::from_bits(0x3d38_51ec);
                if most < len(vel) { vel = set_len(vel, most); }
            }
            let met = crate::moby::slide::slide(w, id, ctx, 1.0, 0.5, 0.0, &mut vel, 0);
            let speed = len(vel);
            if met == 0 {
                w.mobys[id].set_pv_b(0x26, 0);
            } else if speed < 0.005 {
                let n = w.mobys[id].pv_b(0x26).wrapping_add(1);
                w.mobys[id].set_pv_b(0x26, n);
                if 0x14 < n && w.mobys[id].drawn == 0 {
                    let m = &mut w.mobys[id];
                    m.pos = point;
                    m.set_pv_b(0x26, 0);
                }
            }
            if near < 1.1 {
                let p = w.mobys[id].pos;
                let a = atan2(ctx.hero.pos[0] - p[0], ctx.hero.pos[1] - p[1]);
                vel[0] -= (1.1 - near) * a.cos() * 0.1;
                vel[1] -= (1.1 - near) * a.sin() * 0.1;
            }
            w.mobys[id].set_pv_v(0x10, vel);
        }
        _ => {
            w.mobys[id].timer = 0;
            if 1.3 < near { w.mobys[id].timer = 2; }
        }
    }
    // The bob and the sway.
    let m = &mut w.mobys[id];
    let ph = add_rot(m.pv_f(0x54), f32::from_bits(0x3cf5_c28f));
    m.set_pv_f(0x54, ph);
    let up = m.rows[2];
    m.pos = sub(m.pos, scale(up, m.pv_f(0x50)));
    let bob = ph.sin() * f32::from_bits(0x3dcc_cccd);
    m.set_pv_f(0x50, bob);
    m.pos = add(m.pos, scale(up, bob));
    super::props::sway(m, 0.25, f32::from_bits(0x3ccc_cccd), f32::from_bits(0x3bcc_cccd), 0x48, 0x4c);
}

/// `0x3ecd68` (module docs).
fn fire(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    {
        let m = &mut w.mobys[id];
        let c = m.pv_f(0x3c);
        if 1.0 / 60.0 < c {
            m.set_pv_f(0x3c, c - 1.0 / 60.0);
            return;
        }
        m.set_pv_f(0x3c, 0.0);
    }
    if ctx.game.frame.rem_euclid(4) != w.mobys[id].pv_i(0x38) { return; }
    let muzzle = ctx.host.joint(&w.mobys[id], 1).unwrap_or(w.mobys[id].pos);
    let t = w.mobys[id].pv_b(0x24) ^ 1;
    w.mobys[id].set_pv_b(0x24, t);
    let wide = 90.0 * DEG;
    let cone = aim::Cone { yaw: wide, pitch: wide, range: 20.0, near_pitch: wide, near_yaw: wide };
    let dir = ctx.host.camera_rows()[0];
    let found = aim::search(w, ctx, cone, muzzle, dir, false).filter(|&t| {
        let x = &w.mobys[t];
        (!matches!(x.o_class, 0xee1 | 0xec0 | 0xed4) || x.pv_b(0x3b1) != 0xff) && x.o_class != 0xef4
    });
    let Some(t) = found else {
        w.mobys[id].set_pv_i(0x28, 0);
        return;
    };
    w.mobys[id].set_pv_i(0x28, t as i32);
    if ctx.host.anim(&w.mobys[id]).is_none_or(|a| a.seq_b != 2) { ctx.host.set_anim(&w.mobys[id], 2, 0, 5); }
    let above = dot(sub(w.mobys[id].pos, ctx.hero.pos), w.mobys[id].rows[2]);
    if ctx.game.scenes.mode == crate::scene::SCENE { return; }
    let Some(a) = ctx.host.anim(&w.mobys[id]) else { return };
    if a.seq_a != a.seq_b || a.seq_a != 2 || above <= 1.1 { return; }
    let mut aim_at = w.mobys[t].pos;
    aim_at[2] += aim::health(w, t).map_or(0.3, |(_, h)| h);
    if let Some(sys) = ctx.parts.as_deref_mut() {
        let g = &mut ctx.game.rng;
        let spin = g.randi(0xff);
        let def = sys.def_first(0x35);
        for (rgb, s) in [(0x20_2080, spin), (0x60_1080, (spin + 0x80) % 0xff)] {
            if let Some(i) = crate::particles::sprite(sys, g, 420_000.0, 420_000.0, 0.0, muzzle, [0.0; 3], 3, rgb, s as u8, 0x7f, 0x10, 0) { sys.pool.recs[i][2] = def; }
        }
    }
    let from = add(muzzle, scale(sub(aim_at, muzzle), 0.3));
    spawn(w, ctx, from, id, Some(t));
    ctx.host.play_sound(&w.mobys[id], 0, 0);
    let rate = cur_row(ctx.game).map_or(2.0, |r| r.rate);
    let extra = ctx.game.rng.randf(0.0, f32::from_bits(0x3e2e_147b));
    let m = &mut w.mobys[id];
    m.set_pv_b(0x25, m.pv_b(0x25).wrapping_add(1));
    m.set_pv_f(0x3c, 1.0 / rate + extra);
}

/// `0x3ef580`: the shot's rows from its angles, each at 0.35 + 0.15 · sin of its phase; the phases 0.5 rad on.
fn wobble(m: &mut crate::moby::Moby) {
    let mut r = crate::moby::attach::rows_of(m.rot);
    for (k, off) in [4usize, 8, 0xc].into_iter().enumerate() {
        r[k] = scale(r[k], m.pv_f(off).sin() * 0.15 + f32::from_bits(0x3eb3_3334));
    }
    m.rows = r;
    for off in [4usize, 8, 0xc] { m.set_pv_f(off, add_rot(m.pv_f(off), 0.5)); }
}

/// `0x3ee200(from, drone, target)` (module docs): the shot.
pub fn spawn(w: &mut MobyWorld, ctx: &mut Ctx, from: V3, drone: usize, target: Option<usize>) -> Option<usize> {
    if w.class(SHOT).is_none() { return None; }
    let id = w.create(SHOT);
    let speed = cur_row(ctx.game).map_or(60.0, |r| r.speed) * 0.016_666_668;
    {
        let m = &mut w.mobys[id];
        m.pvars.resize(crate::moby::PVAR_BYTES, 0);
        m.update_dist = 0xff;
        m.mode = 0x100;
        m.drawn = 1;
        m.draw_dist = 0xff;
        m.parent = Some(drone);
        m.state = 0;
    }
    let hidden = ctx.hero_moby.and_then(|h| w.mobys.get(h)).is_some_and(|h| h.mode & 1 != 0);
    let (pos, vel) = if ctx.hero.state == 0x1d && hidden {
        let (cam, cr) = (ctx.host.camera_pos(), ctx.host.camera_rows());
        let p = add(add(add(cam, scale(cr[2], -0.5)), scale(cr[1], -0.25)), scale(cr[0], f32::from_bits(0x3dcc_cccd)));
        let mut end = add(cam, scale(cr[0], 20.0));
        if let Some(o) = ctx.coll.and_then(|c| crate::collision::coll_line_m(c, ctx.scene, cam, end, QueryFlags(0), ctx.hero_moby)) { end = o.point; }
        (p, set_len(sub(end, p), speed))
    } else if ctx.hero.items.raised {
        (from, set_len(w.mobys[drone].rows[1], -speed))
    } else {
        match target.filter(|&t| w.mobys.get(t).is_some()) {
            None => (from, set_len(w.mobys[drone].rows[0], speed)),
            Some(t) => {
                let mut a = w.mobys[t].pos;
                if let Some((_, h)) = aim::health(w, t) { a = add(a, scale(ctx.hero.gravity.up_at(a), h)); }
                (from, set_len(sub(a, from), speed))
            }
        }
    };
    let class_scale = w.class(SHOT).map_or(1.0, |c| c.scale);
    {
        let m = &mut w.mobys[id];
        m.pos = pos;
        m.set_pv_v(0x20, vel);
        m.set_pv_f(0x2c, speed);
        wobble(m);
        m.set_pv_i(0x38, 20);
        m.set_pv_f(0x3c, 1.0 / 20.0);
        m.state = 0;
        m.scale = class_scale * 6.0;
        m.set_pv_i(0x1c, target.map_or(0, |t| t as i32));
    }
    // A target's health block `+0x1e` | 0x80.
    if let Some(t) = target.filter(|&t| aim::health(w, t).is_some()) {
        let h = crate::moby::ai::health(&w.mobys[t]);
        let x = &mut w.mobys[t];
        let f = x.pv_s(h + 0x1e) as u16 | 0x80;
        x.set_pv_s(h + 0x1e, f as i16);
    }
    let g = &mut ctx.game.rng;
    let spins = [g.randf(0.0, f32::from_bits(0x3d8e_fa36)), g.randf(0.0, f32::from_bits(0x3e0e_fa36)), g.randf(0.0, f32::from_bits(0x3e8e_fa36))];
    let rot = [g.rand_angle(), g.rand_angle(), g.rand_angle()];
    let ph = [g.rand_angle(), g.rand_angle(), g.rand_angle()];
    {
        let m = &mut w.mobys[id];
        m.set_pv_f(0x30, 0.0);
        m.set_pv_f(0x40, 1.0);
        m.set_pv_f(0x44, spins[0]);
        m.set_pv_f(0x48, spins[1]);
        m.set_pv_f(0x4c, spins[2]);
        m.rot = rot;
        m.set_pv_f(4, ph[0]);
        m.set_pv_f(8, ph[1]);
        m.set_pv_f(0xc, ph[2]);
    }
    w.rebuild(id, &*ctx.host);
    Some(id)
}

/// `0x3ef6a8(m)`: a dead target — not a crate, its health at most 0.
fn dead(w: &MobyWorld, t: usize) -> bool { !aim::is_crate(w, t) && aim::health(w, t).is_some_and(|(hp, _)| hp <= 0.0) }

/// `0x3ee618` (module docs).
pub fn update_shot(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].pvars.resize(crate::moby::PVAR_BYTES, 0);
    let start = w.mobys[id].pos;
    // The homing.
    if let Some(t) = w.mobys[id].pv_link(0x1c) {
        let gone = w.mobys.get(t).is_none_or(|x| x.state == 0xfe || x.state == 0xfd);
        if gone || dead(w, t) {
            w.mobys[id].set_pv_i(0x1c, 0);
        } else {
            let mut d = sub(w.mobys[t].pos, start);
            if let Some((_, h)) = aim::health(w, t) { d = add(d, scale(w.mobys[t].rows[2], h)); }
            let l = len(d);
            if l == 0.0 {
                w.mobys[id].set_pv_i(0x1c, 0);
            } else {
                let s = w.mobys[id].pv_f(0x2c);
                w.mobys[id].set_pv_v(0x20, scale(d, s / l));
            }
        }
    }
    match w.mobys[id].state {
        0 => fly(w, id, ctx, start),
        1 => {
            let mut n = w.mobys[id].pv_i(0x38);
            let out = countdown(&mut n);
            w.mobys[id].set_pv_i(0x38, n);
            if out {
                w.delete(id);
                return;
            }
            w.register_draw(id);
        }
        _ => {}
    }
    let p = w.mobys[id].pos;
    if p.iter().any(|&c| !(2.0..=1021.0).contains(&c)) { w.delete(id); }
}

/// State 0 of the shot (module docs).
fn fly(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, start: V3) {
    let damage = cur_row(ctx.game).map_or(0.0, |r| r.damage);
    let to = {
        let m = &mut w.mobys[id];
        m.pos = add(m.pos, m.pv_v(0x20));
        for (k, off) in [0x44usize, 0x48, 0x4c].into_iter().enumerate() { m.rot[k] = add_rot(m.rot[k], m.pv_f(off)); }
        wobble(m);
        m.pos
    };
    let d = set_len(w.mobys[id].pv_v(0x20), 1.0);
    let mut tmpl = HitTemplate::new(damage, Some(id), 0x10000, [d[0], d[1], d[2], NO_KNOCK_LIMIT]);
    tmpl.kind = (SHOT as u16 as u32) << 16 | 0x0101;
    let hit = ctx.coll.and_then(|c| crate::collision::coll_line_m(c, ctx.scene, start, to, QueryFlags(0), ctx.hero_moby));
    let ended = match hit {
        Some(o) => {
            if let Some(t) = o.moby { w.deliver_hit_any(t, &tmpl, [o.point[0], o.point[1], o.point[2], 1.0]); }
            // `0x3ef018(point, 15 / 10, …)`: the same burst with a moby or not (a moby's weapon mods not ported).
            burst(ctx, o.point);
            true
        }
        None => {
            let mut n = w.mobys[id].pv_i(0x38);
            let out = countdown(&mut n);
            w.mobys[id].set_pv_i(0x38, n);
            if out { fizzle(ctx, to); }
            out
        }
    };
    if ended {
        let m = &mut w.mobys[id];
        m.state = 1;
        m.set_pv_i(0x1c, 0);
        m.mode |= 0x41;
        m.set_pv_i(0x38, 6);
    }
    let pos = w.mobys[id].pos;
    let frame = ctx.game.frame;
    let Some(sys) = ctx.parts.as_deref_mut() else {
        w.register_draw(id);
        return;
    };
    let g = &mut ctx.game.rng;
    let def = sys.def_first(0x35);
    let spin = g.randi(0xff);
    for s in [spin, (spin + 0x80) % 0xff] {
        if let Some(i) = crate::particles::sprite(sys, g, 420_000.0, 420_000.0, 0.0, pos, [0.0; 3], 2, 0x20_2080, s as u8, 0x40, 0x10, 1) { sys.pool.recs[i][2] = def; }
    }
    let step = scale(sub(pos, start), f32::from_bits(0x3dcc_cccd));
    let mut p = start;
    for _ in 0..5 {
        let s = g.randi(0xff);
        let f = g.randf(f32::from_bits(0x3dcc_cccd), f32::from_bits(0x3e99_999a));
        let j: V3 = std::array::from_fn(|k| p[k] + g.randf_signed(0.0, f32::from_bits(0x3d4c_cccd)));
        crate::particles::sprite(sys, g, f * 210_000.0, f * 42_000.0, 0.0, j, [0.0; 3], 0xf, 0x80_8080, s as u8, 0x20, 0, 1);
        p = add(p, step);
    }
    if frame % 10 == 0 {
        if let Some(i) = crate::particles::sprite(sys, g, 2100.0, 420_000.0, 0.0, pos, [0.0; 3], 0x1e, 0x20_2080, 0, 0x20, 0, 1) { sys.pool.recs[i][2] = sys.def_first(0x2d); }
    }
    w.register_draw(id);
}

/// `0x3ef018(at)`: twenty times a dark puff rising 0.2..3 a second (`0x335bb8`: 0.3..0.75, `0x7f202020`, 90 ticks)
/// and a grey spark (`0x339848`: 0.1..0.5, 30 ticks) flying 1..2 a second up to 80° high within ±80° of the way to the
/// camera, each jittered about `at`.
fn burst(ctx: &mut Ctx, at: V3) {
    let cam = ctx.host.camera_pos();
    let to_cam = atan2(cam[0] - at[0], cam[1] - at[1]);
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let g = &mut ctx.game.rng;
    let down = crate::particles::GRAVITY;
    for _ in 0..20 {
        let f = g.randf(f32::from_bits(0x3e4c_cccd), 3.0);
        let v = scale(down, -(f * f32::from_bits(0x3c88_8889)));
        let spin = g.randf_signed(1.0, 2.0) as i32;
        let size = g.randf(f32::from_bits(0x3e99_999a), 0.75) * 210_000.0;
        let p: V3 = std::array::from_fn(|k| at[k] + g.randf_signed(0.0, f32::from_bits(0x3e4c_cccd)));
        if let Some(i) = crate::particles::puff(sys, g, 0.0, 1.0, f32::from_bits(0x3f81_47ae), size, p, spin, v, 0x7f20_2020, 1) {
            let r = &mut sys.pool.recs[i];
            crate::particles::rec::set_i16(r, 0xa, 0x5a);
            r[3] = 0x44;
            r[9] = 4 + 0x40;
            crate::particles::rec::set_ff(r, 0x3c, 5.0);
            crate::particles::rec::set_u32(r, 0x24, 3);
            r[0x2a] = 0x30;
            r[0x2b] = r[0xa];
        }
        spark(sys, g, spin, to_cam, at, (1.0, 2.0), (0.1, 0.5));
    }
}

/// `0x3ef360(at)`: twenty grey sparks as [`burst`]'s, flying 0.5..1 a second, 0.05..0.2 large.
fn fizzle(ctx: &mut Ctx, at: V3) {
    let cam = ctx.host.camera_pos();
    let to_cam = atan2(cam[0] - at[0], cam[1] - at[1]);
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let g = &mut ctx.game.rng;
    for _ in 0..20 {
        let f = g.randf(0.5, 1.0);
        let spin = g.randf_signed(1.0, 2.0) as i32;
        // A draw the game makes and drops.
        g.randf(f32::from_bits(0x3d4c_cccd), f32::from_bits(0x3e4c_cccd));
        let most = f32::from_bits(0x3fb2_b8c4);
        let pitch = g.randf(0.0, most);
        let yaw = add_rot(to_cam, g.randf_signed(0.0, most));
        let v = crate::moby::classes::shot::polar(f * 0.016_666_668, yaw, pitch);
        let p: V3 = std::array::from_fn(|k| at[k] + g.randf_signed(0.0, f32::from_bits(0x3dcc_cccd)));
        let size = g.randf(f32::from_bits(0x3d4c_cccd), f32::from_bits(0x3e4c_cccd)) * 210_000.0;
        crate::particles::spark(sys, g, size, 500.0, 1.0, 0.0, 1.0, p, v, 0x1e, 0x7f, 0x7f_7f7f, spin);
    }
}

/// [`burst`]'s spark: speed `fly` a second, size `big` (× 210000).
fn spark(sys: &mut crate::particles::Particles, g: &mut crate::rng::Rng, spin: i32, to_cam: f32, at: V3, fly: (f32, f32), big: (f32, f32)) {
    let f = g.randf(fly.0, fly.1);
    let most = f32::from_bits(0x3fb2_b8c4);
    let pitch = g.randf(0.0, most);
    let yaw = add_rot(to_cam, g.randf_signed(0.0, most));
    let v = crate::moby::classes::shot::polar(f * f32::from_bits(0x3c88_8889), yaw, pitch);
    let p: V3 = std::array::from_fn(|k| at[k] + g.randf_signed(0.0, f32::from_bits(0x3dcc_cccd)));
    let size = g.randf(big.0, big.1) * 210_000.0;
    crate::particles::spark(sys, g, size, 500.0, 1.0, 0.0, 1.0, p, v, 0x1e, 0x7f, 0x7f_7f7f, spin);
}

/// `0x3eecd0` (module docs).
pub fn draw(w: &mut MobyWorld, id: usize, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    use crate::fx::FxQuad;
    let m = &mut w.mobys[id];
    let scroll = m.pv_f(0x30);
    let s = |s0: f32| { let v = s0 - scroll; if v <= 7.0 { v + 7.0 } else { v } };
    let st = [[s(0.0), 0.0], [s(0.0), 1.0], [s(1.0), 0.0], [s(1.0), 1.0]];
    let n = scroll + 0.1;
    m.set_pv_f(0x30, if 4.0 <= n { n - 4.0 } else { n });
    let pos = m.pos;
    let a = sub(pos, cam);
    let back = set_len(m.pv_v(0x20), -2.0);
    let tail = add(pos, back);
    let side = set_len(cross_ba(a, back), 0.1);
    let (c0, c1) = (0x4010_8080u32, 0x4010_1080u32);
    out.push(FxQuad { fx: 0x17, additive: false, corners: [add(pos, side), sub(pos, side), add(tail, side), sub(tail, side)], st, rgba: [c0, c0, c1, c1] });
    let far = add(tail, back);
    out.push(FxQuad { fx: 0x17, additive: false, corners: [add(tail, side), sub(tail, side), add(far, side), sub(far, side)], st, rgba: [c1, c1, 0x80_1010, 0x80_1010] });
    let wide = set_len(side, f32::from_bits(0x3f19_999a));
    let end = add(add(pos, back), back);
    let base = [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]];
    out.push(FxQuad { fx: 0xe, additive: true, corners: [add(pos, wide), sub(pos, wide), add(end, wide), sub(end, wide)], st: base, rgba: [0x40ff_0020, 0x40ff_0020, 0xff_0020, 0xff_0020] });
}
