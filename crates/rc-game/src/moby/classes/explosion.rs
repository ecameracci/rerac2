//! The big explosion, class **0x0d4e** (L02 start `0x425300`, its blasts `0x425c68`, update `0x4255e8`, draw
//! `0x426410`; the class table's record `0x2a69ec`): what a big machine leaves when it blows up — Maktar's dropship
//! (crate::moby::classes::dropship) and, on other levels, the classes listed in the gap file.
//!
//! **The start** ([`start`] `0x425300(ground, _, pos, dir)`) creates the moby at `pos` (state 0, updated and drawn at
//! any distance), its frame from `dir` (`+0xc0`; `+0xd0` = `dir × up` made unit, `+0xe0` = `+0xd0 × dir`; up
//! (0, 0, 1), `0x32a9b0`), moved half a unit along `dir`; `ground` (0: none, or when 6 or more under it) is where its
//! ring of dust can rise. Six random angles turn its discs. Its pvars: `+0x00..+0x14` the angles, `+0x18` the ground,
//! `+0x1c` the flash's timer A (20) and `+0x1e` the blast's timer B (60), `+0x20` / `+0x24` their inverses, `+0x28`
//! / `+0x2a` where each starts fading (A − 3, B − 20) and `+0x2c` / `+0x30` those inverses; `+0x36` the glare's timer
//! (16, none when the line from it to the camera meets the world, flags 0x12) and `+0x38` its inverse; in view
//! (`0x2f4008(128, a sphere of 1) == 1`) `+0x34` the screen flash's 4 ticks (`+0x3c` 1/4) and `+0x40` the shimmer's
//! 15 + 1 (`+0x44` 1/15). Then the blasts ([`blasts`]).
//!
//! **The update** ([`update`]): the explosions' fog flash runs (crate::fog::FogFlash::tick); the angles turn (3°,
//! 0.2°, ¼°, −¼°, 0.4°, −0.4° a tick); B, the screen flash, the shimmer and A count down. While A or B runs its draw
//! is asked for ([`draw`]), its first 6 ticks of B throw 20 streaks along `dir` (16..32 a second, turned 15°..80°
//! about `+0xd0` then about `dir`, 20..40 ticks), B in 13..39 drops a cinder (crate::particles::cinder) under it,
//! and over the ground with B past 25 a line of dust puffs rises ([`dust`]). Both out, it goes once the fog flash is
//! over.
//!
//! **The draw** ([`draw`], from the frame's second list): two discs across `dir` (FX 0x38, additive, ±angle 0,
//! 2 → 20 across over A, alpha 60·cos² of the view's angle to `dir`, fading over A's last 17 ticks); the blast's
//! three discs across `dir` (angles 1, 2, 3: FX 0x38 at 1.5·s and a fifth of the alpha, FX 0x39 at 1.25·s and
//! 1.5·alpha, FX 0x39 at s; s 1 → 4 over B, alpha 255 fading as t² over B's last 40 ticks, faded out within 11 of
//! the camera) and three facing the camera a quarter unit toward it (angles 4, 5, 1: FX 0x39 at s and 0.75·s, FX
//! 0x38 at 1.5·s and a fifth); while the glare runs, its four-pointed star 2 units in front of the camera (FX 0xb)
//! when it is on the screen ([`glare`]); while the shimmer runs, the last finished frame blended back in through a
//! soft mask in front of the camera ([`shimmer`], a crate::fx::FrameQuad).
//!
//! The particles are the L02 copies of the shared spawners: `0x33c6d0` = crate::particles::puff (L00 `0x335bb8`, its
//! update `0x33c898` = L00 `0x335d80`), `0x345ac0` = crate::particles::sprite (L00 `0x33ef78`, update `0x345c68`),
//! `0x346640` = crate::particles::streak (L00 `0x33f940`, update `0x346758`), `0x3468b8` the cinder (type 0x5b).
//!
//! Not ported: the screen flash (`+0x34`: FX 0xb, 0.075 across, 0.1 in front of the camera, on ALPHA row 4
//! `(Cd − Cs)·FIX + Cs` with FIX 0x80, which leaves the frame's colour as it was: it only writes the frame's alpha,
//! which the shimmer's mask overwrites and nothing else reads); the gravity-boots bends (`0x388638`, `0x388688`).
//! The shimmer's last frame is the port's frame before its HUD (crate::fx::FrameSource::Shown), sampled where it lies
//! on the screen: the game's TEX0 for the buffer on show is 512×512 (TH 9) while its screen places are fractions of
//! the 512×416 screen, so on the PS2 the rows it reads may sit lower than the pixels it covers (not checked against
//! the console); its fog and ZTST ALWAYS are kept, its alpha above 0x80 is clamped (crate::display_blend). The glare's
//! screen test uses the game's 512×448 screen and view tangents (0.63, 0.63·0.775) for `0x2e7c70`'s projection.

use crate::hero::physics::{add, add_rot, atan2, cross, dist, dot, len, scale, set_len, sub, unit, V3};
use crate::moby::{countdown, Ctx, MobyWorld};
use crate::particles::{self, rec};

pub const CLASS: i16 = 0x0d4e;

/// The port keeps the moby's frame (`+0xc0`, `+0xd0`, `+0xe0` past its pvars in the game) in its pvars.
const DIR: usize = 0x50;
const SIDE: usize = 0x60;
const UP: usize = 0x70;

/// Pvar offsets (module docs).
const GROUND: usize = 0x18;
const A: usize = 0x1c;
const B: usize = 0x1e;
const INV_A: usize = 0x20;
const INV_B: usize = 0x24;
const A_FADE: usize = 0x28;
const B_FADE: usize = 0x2a;
const INV_A_FADE: usize = 0x2c;
const INV_B_FADE: usize = 0x30;
const SCREEN_FLASH: usize = 0x34;
const GLARE: usize = 0x36;
const INV_GLARE: usize = 0x38;
const INV_SCREEN_FLASH: usize = 0x3c;
const SHIMMER: usize = 0x40;
const INV_SHIMMER: usize = 0x44;

/// `$gp+0x30b0` / `+0x30b4`: A and how long before its end the flash fades; `$gp+0x30d8` / `+0x30dc` the same for B.
const A_TICKS: i32 = 20;
const A_FADE_BY: i32 = 3;
const B_TICKS: i32 = 60;
const B_FADE_BY: i32 = 20;
/// `$gp+0x3188`, `$gp+0x318c`, `$gp+0x3194` + `$gp+0x3198`: the glare, the screen flash, the shimmer.
const GLARE_TICKS: i32 = 16;
const SCREEN_FLASH_TICKS: i32 = 4;
const SHIMMER_TICKS: i32 = 15;
const SHIMMER_DELAY: i32 = 1;
/// `$gp+0x317c`: the blast disc's full size (and ×1.5 the dust's reach).
const BLAST_SIZE: f32 = 4.0;

/// The angles' turns a tick (`$gp+0x30c0`, `+0x30d4` degrees; `0x3b8efa36` = ¼°; `$gp+0x30cc`, `+0x30d0` degrees).
fn turns() -> [f32; 6] {
    let deg = 0.017_453_294;
    [3.0 * deg, 0.2 * deg, f32::from_bits(0x3b8e_fa36), f32::from_bits(0xbb8e_fa36), 0.4 * deg, -0.4 * deg]
}

/// `14.6 / 3600`: the sprites' and streaks' fall a tick (`$gp+0x30f8` × 1/3600).
const FALL: f32 = 14.6 * 0.000_277_777_78;

/// `0x324290` + `0x325f20`: `v` turned by `a` about `axis` (the quaternion `(axis·sin(a/2), cos(a/2))`, `q v q*`).
fn turn(v: V3, axis: V3, a: f32) -> V3 {
    let (s, w) = (a * 0.5).sin_cos();
    let u = scale(axis, s);
    let t = scale(cross(u, v), 2.0);
    add(add(v, scale(t, w)), cross(u, t))
}

/// `0x2f4008(far, (c, r)) == 1`: the sphere wholly in the view (in front, within `far` along it, inside the four
/// side planes by its radius), by the game's tangents.
fn wholly_in_view(ctx: &Ctx, c: V3, r: f32, far: f32) -> bool {
    let (eye, rows) = (ctx.host.camera_pos(), ctx.host.camera_rows());
    let d = sub(c, eye);
    let (z, x, y) = (dot(rows[0], d), dot(rows[1], d), dot(rows[2], d));
    let (tx, ty): (f32, f32) = (0.63, 0.63 * 0.775);
    let (kx, ky) = ((1.0 + tx * tx).sqrt(), (1.0 + ty * ty).sqrt());
    0.0 < z - r && z + r <= far && x.abs() + r * kx <= tx * z && y.abs() + r * ky <= ty * z
}

/// `0x425300(ground, _, pos, dir)`: the big explosion at `pos` facing `dir` (module docs); None when the level has no
/// class 0x0d4e.
pub fn start(w: &mut MobyWorld, ctx: &mut Ctx, ground: f32, pos: V3, dir: V3) -> Option<usize> {
    w.class(CLASS)?;
    let id = w.create(CLASS);
    let cam = ctx.host.camera_pos();
    let side = unit(cross(dir, [0.0, 0.0, 1.0]));
    let up = cross(side, dir);
    let at = add(pos, scale(dir, 0.5));
    let ground = if ground != 0.0 && BLAST_SIZE * 1.5 <= at[2] - ground { 0.0 } else { ground };
    let g = &mut ctx.game.rng;
    let angles: [f32; 6] = std::array::from_fn(|_| g.rand_angle());
    let m = &mut w.mobys[id];
    m.update = Some(update);
    m.mode &= !crate::moby::mode::NO_UPDATE;
    m.state = 0;
    m.update_dist = 0xff;
    m.draw_dist = 0xff;
    m.pos = at;
    m.set_pv_v(DIR, dir);
    m.set_pv_v(SIDE, side);
    m.set_pv_v(UP, up);
    m.set_pv_f(GROUND, ground);
    for (k, a) in angles.iter().enumerate() { m.set_pv_f(4 * k, *a); }
    m.set_pv_s(A, A_TICKS as i16);
    m.set_pv_s(B, B_TICKS as i16);
    m.set_pv_f(INV_A, 1.0 / A_TICKS as f32);
    m.set_pv_f(INV_B, 1.0 / B_TICKS as f32);
    m.set_pv_s(A_FADE, (A_TICKS - A_FADE_BY) as i16);
    m.set_pv_f(INV_A_FADE, 1.0 / (A_TICKS - A_FADE_BY) as f32);
    m.set_pv_s(B_FADE, (B_TICKS - B_FADE_BY) as i16);
    m.set_pv_f(INV_B_FADE, 1.0 / (B_TICKS - B_FADE_BY) as f32);
    // `0x2e1e10(pos, camera, 0x12)`: the glare only with nothing between.
    if w.coll_line(ctx, at, cam, 0x12, None).is_some() {
        w.mobys[id].set_pv_s(GLARE, 0);
    } else {
        let full = wholly_in_view(ctx, at, 1.0, 128.0);
        let m = &mut w.mobys[id];
        m.set_pv_s(GLARE, GLARE_TICKS as i16);
        m.set_pv_f(INV_GLARE, 1.0 / GLARE_TICKS as f32);
        if full {
            m.set_pv_s(SCREEN_FLASH, SCREEN_FLASH_TICKS as i16);
            m.set_pv_f(INV_SCREEN_FLASH, 1.0 / SCREEN_FLASH_TICKS as f32);
            m.set_pv_i(SHIMMER, SHIMMER_TICKS + SHIMMER_DELAY);
            m.set_pv_f(INV_SHIMMER, 1.0 / SHIMMER_TICKS as f32);
        } else {
            m.set_pv_i(SHIMMER, 0);
            m.set_pv_s(SCREEN_FLASH, 0);
        }
    }
    w.rebuild(id, &*ctx.host);
    w.changed = true;
    blasts(w, ctx, id);
    Some(id)
}

/// `0x425c68(m)`: the blasts.
/// * Wholly in view within 48 (a sphere of 2): the fog flash (crate::fog::FogFlash::start, its distance from the
///   camera) and the up-and-down shake of 0.1 over 40 ticks.
/// * The general explosion `0x324930` with only a light of 20 asked for at lod −1 (crate::moby::blast::explode: it
///   leaves nothing at full frame rate).
/// * The moby half a unit toward the camera; then 20 lilac glows (crate::particles::puff, 1.5·210000, `0x7f60a0ff`,
///   turning one way then the other) a unit about it, drifting 1..2 a second toward the camera's side (within 90° of
///   its yaw, up to 80° up or down), fading in over 1..10 ticks for 40..70;
/// * 32 sets of blast sprites (crate::particles::sprite: two grey `0x7f7f7f` 0.35..0.75·210000 across, the second a
///   hundredth off and turning the other way, and a blue `0x202080` halo six times as big on type 0x35's frame at
///   alpha 0x30, turning twice as fast the other way) about it, thrown 4..8 a second toward the camera's side within
///   80° and 0..12 a second up, falling 14.6/3600 a tick, 50..80 ticks;
/// * a unit nearer the camera, 20 dark smoke puffs (`0x7f202020`, normal blend) rising 0.2..3 a second, 0.5..1.5 ·
///   210000, fading in over 7..15 ticks for 90..180.
pub fn blasts(w: &mut MobyWorld, ctx: &mut Ctx, id: usize) {
    let cam = ctx.host.camera_pos();
    let pos = w.mobys[id].pos;
    if wholly_in_view(ctx, pos, 2.0, 48.0) {
        ctx.game.fog_flash.start(dist(pos, cam));
        ctx.game.shakes.ask(0, 0.1, 0x28);
    }
    let e = crate::moby::blast::Explosion { near: 9.0, light: 20.0, sound: -1, lod: -1, flags: 0x81_0001, ..Default::default() };
    crate::moby::blast::explode(w, ctx, Some(id), Some(pos), &e);
    let at = add(pos, set_len(sub(cam, pos), 0.5));
    w.mobys[id].pos = at;
    let yaw = atan2(cam[0] - at[0], cam[1] - at[1]);
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let g = &mut ctx.game.rng;
    let mut side = 1;
    for _ in 0..20 {
        side = -side;
        let p: V3 = std::array::from_fn(|k| at[k] + g.randf_signed(0.0, 1.0));
        let speed = g.randf(0.016_666_668, 0.033_333_335);
        let pitch = g.randf_signed(0.0, 1.396_263_4);
        let a = add_rot(yaw, g.randf_signed(0.0, 1.570_796_4));
        let v = super::shot::polar(speed, a, pitch);
        if let Some(i) = particles::puff(sys, g, 0.0, 1.0, 1.01, 1.5 * 210_000.0, p, side, v, 0x7f60_a0ff, 1) {
            let life = g.rand_range(0x28, 0x46) as i16;
            let fade = g.rand_range(1, 10) as f32;
            let r = &mut sys.pool.recs[i];
            rec::set_i16(r, 0xa, life);
            r[9] = 4 + 0x40;
            rec::set_u32(r, 0x24, 3);
            rec::set_ff(r, 0x3c, fade);
            r[0x2a] = 0x40;
            r[0x2b] = r[0xa];
        }
    }
    for _ in 0..32 {
        let spin = g.randf_signed(2.0, 5.0) as i32;
        let life = g.rand_range(50, 80);
        let p: V3 = std::array::from_fn(|k| at[k] + g.randf_signed(0.0, 0.1));
        let size = g.randf(0.35, 0.75);
        let speed = g.randf(4.0, 8.0);
        let a = add_rot(yaw, g.randf_signed(0.0, 1.396_263_4));
        let up = g.randf(0.0, 12.0) * 0.016_666_668;
        let v = [a.cos() * 0.016_666_668 * speed, a.sin() * 0.016_666_668 * speed, up];
        particles::sprite(sys, g, size * 210_000.0, 2100.0, FALL, p, v, life, 0x7f_7f7f, spin as u8, 0x7f, 0x7f, 1);
        let q: V3 = std::array::from_fn(|k| p[k] + g.randf_signed(0.0, 0.01));
        particles::sprite(sys, g, size * 210_000.0, 2100.0, FALL, q, v, life, 0x7f_7f7f, (-spin) as u8, 0x7f, 0x7f, 1);
        if let Some(i) = particles::sprite(sys, g, size * 1_260_000.0, 4200.0, FALL, p, v, life, 0x20_2080, (spin * -2) as u8, 0x30, 0x30, 1) {
            sys.pool.recs[i][2] = sys.def_first(0x35);
        }
    }
    let at = add(at, set_len(sub(cam, at), 1.0));
    for _ in 0..20 {
        let rise = g.randf(0.2, 3.0);
        let vel = scale(particles::GRAVITY, -(rise * 0.016_666_668));
        let life = g.rand_range(0x5a, 0xb4) as i16;
        let spin = g.randf_signed(1.0, 2.0) as i32;
        let size = g.randf(0.5, 1.5) * 210_000.0;
        let p: V3 = std::array::from_fn(|k| at[k] + g.randf_signed(0.0, 0.75));
        if let Some(i) = particles::puff(sys, g, 0.0, 1.0, 1.01, size, p, spin, vel, 0x7f20_2020, 1) {
            let r = &mut sys.pool.recs[i];
            rec::set_i16(r, 0xa, life);
            r[3] = 0x44;
            r[9] = 4 + 0x40;
            let fade = g.rand_range(7, 0xf) as f32;
            let r = &mut sys.pool.recs[i];
            rec::set_ff(r, 0x3c, fade);
            rec::set_u32(r, 0x24, 3);
            r[0x2a] = 0x7f;
            r[0x2b] = r[0xa];
        }
    }
}

/// `0x2f29c8` on a short pvar: one down; true when it was or reaches 0.
fn count_s(m: &mut crate::moby::Moby, o: usize) -> bool {
    let mut t = m.pv_s(o) as i32;
    let done = countdown(&mut t);
    m.set_pv_s(o, t as i16);
    done
}

/// `0x4255e8` (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let level = ctx.game.fog;
    ctx.game.fog_flash.tick(&level);
    let m = &mut w.mobys[id];
    for (k, t) in turns().iter().enumerate() { m.set_pv_f(4 * k, add_rot(m.pv_f(4 * k), *t)); }
    count_s(m, B);
    count_s(m, SCREEN_FLASH);
    let s = m.pv_i(SHIMMER);
    if 0 < s { m.set_pv_i(SHIMMER, s - 1); }
    let a_done = count_s(m, A);
    let b = m.pv_s(B) as i32;
    if a_done && b == 0 {
        if ctx.game.fog_flash.timer == 0 { w.delete(id); }
        return;
    }
    let (pos, dir, side) = (m.pos, m.pv_v(DIR), m.pv_v(SIDE));
    w.register_draw(id);
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let g = &mut ctx.game.rng;
    if B_TICKS - 6 <= b {
        for _ in 0..20 {
            let speed = g.randf(16.0, 32.0) * 0.016_666_668;
            let v = scale(dir, speed);
            let a1 = g.randf_signed(0.261_799_4, 1.396_263_4);
            let a2 = g.randf_signed(0.261_799_4, 1.396_263_4);
            let v = turn(turn(v, side, a1), dir, a2);
            let life = g.rand_range(20, 40) as i16;
            particles::streak(sys, FALL, pos, v, life);
        }
    }
    if (13..=39).contains(&b) {
        let pitch = g.randf(f32::from_bits(0xbf75_be0d), f32::from_bits(0xbfb2_b8c4));
        let yaw = g.rand_angle();
        let r = g.randf(0.0, 0.5) * 0.016_666_668;
        let v = super::shot::polar(r, yaw, pitch);
        let size = g.randf(0.25, 0.75);
        let c = add(pos, scale(dir, -0.45));
        let p: V3 = std::array::from_fn(|k| c[k] + g.randf_signed(0.0, 0.25));
        let life = g.rand_range(0x1e, 0x2d);
        particles::cinder::cinder(sys, g, size * 210_000.0, 0.5, p, v, 0x4f93_e5ff, 0x1024_34ee, life, 1, None, None, 0);
    }
    dust(&w.mobys[id], sys, g);
}

/// The update's dust (`0x4255e8`): with the explosion `h` over its ground and `R` 1.5 times its blast disc's size,
/// for `0 < h < R` and B past 25, `max(1, (1 − h/R)·30)` dust puffs (crate::particles::puff: 0.3 jitter, shrinking
/// to 0.9 a tick, 0.2..1.5 · 210000, `0x7f204080`, turning up to 16 either way) along a line a quarter unit over the
/// ground under it, across `up × dir` and `R·(0.75 − 0.65·h/R)` to either side; each fades from 0x50..0x7f over
/// 8..25 ticks.
fn dust(m: &crate::moby::Moby, sys: &mut particles::Particles, g: &mut crate::rng::Rng) {
    let b = m.pv_s(B) as i32;
    let r = (BLAST_SIZE + (1.0 - BLAST_SIZE) * b as f32 * m.pv_f(INV_B)) * 1.5;
    let ground = m.pv_f(GROUND);
    let h = m.pos[2] - ground;
    if !(0.0 < h && h < r && 0x19 < b) { return; }
    let n = (((1.0 - h / r) * 30.0) as i32).max(1);
    let d = set_len(cross([0.0, 0.0, 1.0], m.pv_v(DIR)), r * ((h / r) * -0.65 + 0.75));
    let c = [m.pos[0], m.pos[1], ground + 0.25];
    let mut at = add(c, d);
    let step = scale(sub(sub(c, d), at), 1.0 / n as f32);
    for _ in 0..n {
        let mut spin = g.randi(0x10);
        if g.randi(2) != 0 { spin = -spin; }
        let size = g.randf(0.2, 1.5) * 210_000.0;
        let i = particles::puff(sys, g, 0.3, 1.0, 0.9, size, at, spin, [0.0; 3], 0x7f20_4080, 1);
        at = add(at, step);
        if let Some(i) = i {
            let life = g.rand_range(8, 0x19) as i16;
            let r = &mut sys.pool.recs[i];
            rec::set_i16(r, 0xa, life);
            r[9] = 4 + 0x40;
            rec::set_u32(r, 0x24, 2);
            let a = g.rand_range(0x50, 0x7f) as u8;
            let r = &mut sys.pool.recs[i];
            r[0x2a] = a;
            r[0x2b] = r[0xa];
        }
    }
}

/// The template `0x1aa310` (a ±1 square in y and z) as GS strip corners, and its ST.
const TEMPLATE: [[f32; 2]; 4] = [[-1.0, 1.0], [-1.0, -1.0], [1.0, 1.0], [1.0, -1.0]];
const TEMPLATE_ST: [[f32; 2]; 4] = [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]];

/// One disc (`0x2f3260` on the template, `0x2eef20`): the template on the rows `b`, `c` (turned by `a` about `axis`
/// when `|a| > 1e-5`) × `s` at `at`; FX `fx` additive.
#[allow(clippy::too_many_arguments)]
fn disc(out: &mut Vec<crate::fx::FxQuad>, fx: usize, axis: V3, b: V3, c: V3, a: f32, s: f32, at: V3, rgba: u32) {
    let (b, c) = if 1e-5 < a.abs() { (turn(b, axis, a), turn(c, axis, a)) } else { (b, c) };
    let corner = |t: [f32; 2]| add(at, add(scale(b, t[0] * s), scale(c, t[1] * s)));
    out.push(crate::fx::FxQuad { fx, additive: true, corners: TEMPLATE.map(corner), st: TEMPLATE_ST, rgba: [rgba; 4] });
}

/// `$gp+0x319c` / `+0x31a0` / `+0x31a4`: the shimmer's half-size at its end and at its start, its mask's FX texture;
/// 0.3 (`0x3e99999a`): how far in front of the camera it hangs; 0.9 (`0x3f666666`): its screen places' limit.
const SHIMMER_END: f32 = 0.0;
const SHIMMER_START: f32 = 0.5;
const SHIMMER_FX: usize = 0x36;
const SHIMMER_AT: f32 = f32::from_bits(0x3e99_999a);
const SHIMMER_CLAMP: f32 = f32::from_bits(0x3f66_6666);

/// The heat shimmer (`0x426410` while `+0x40` is 1..=15, k = `+0x40`·`+0x44` from 1 to 1/15): a square 0.3 in front
/// of the camera toward the explosion, across the view (`up × v` and `v × that`, up (0, 0, 1) as at the start),
/// `0.5 + (0 − 0.5)·k` to each side. The game draws it twice: FX 0x36 at ALPHA 0x4a (`(0 − 0)·As + Cd`: the colour
/// kept) under TEST 0x31001 (ATST NEVER, AFAIL FB_ONLY, ZTST ALWAYS), which writes `At·Av` (Av = 255·k) into the
/// frame's alpha only; then under TEST 0x33001 (RGB only) the same corners textured with the last finished frame
/// (`0x2ea6c8(−7)`, TEX1 0xff9000000260) at their own screen places (`0x2ea5b8`, held to 0..0.9) and ALPHA 0x54
/// (`(Cs − Cd)·Ad + Cd`): the last frame blended in by that mask. Here one [`crate::fx::FrameQuad`] does both (the
/// mask's alpha as the blend's); the mask's untouched corners never reach the frame.
fn shimmer(m: &crate::moby::Moby, cam: V3, frame: &mut Vec<crate::fx::FrameQuad>) {
    let s = m.pv_i(SHIMMER);
    if s == 0 || SHIMMER_TICKS < s { return; }
    let k = s as f32 * m.pv_f(INV_SHIMMER);
    let size = SHIMMER_START + (SHIMMER_END - SHIMMER_START) * k;
    let v = set_len(sub(m.pos, cam), SHIMMER_AT);
    let at = add(cam, v);
    // `0x2f2e88(out, a, b)` is `b × a`.
    let f = set_len(cross([0.0, 0.0, 1.0], v), size);
    let e = set_len(cross(v, f), size);
    let corners = [add(add(at, e), f), sub(add(at, e), f), add(sub(at, e), f), sub(sub(at, e), f)];
    let rgba = ((255.0 * k) as i32 as u32) << 24 | 0x80_8080;
    frame.push(crate::fx::FrameQuad {
        source: crate::fx::FrameSource::Shown,
        mask: Some(SHIMMER_FX),
        corners,
        st: [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]],
        offset: [[0.0; 2]; 4],
        clamp: SHIMMER_CLAMP,
        rgba: [rgba; 4],
    });
}

/// `0x426410`, the draw (module docs). The glare's timer counts down here, as the game's.
pub fn draw(w: &mut MobyWorld, id: usize, cam: V3, cam_rows: [V3; 3], out: &mut Vec<crate::fx::FxQuad>, frame: &mut Vec<crate::fx::FrameQuad>) {
    let m = &mut w.mobys[id];
    let (pos, dir, side, up) = (m.pos, m.pv_v(DIR), m.pv_v(SIDE), m.pv_v(UP));
    let ang: [f32; 6] = std::array::from_fn(|k| m.pv_f(4 * k));
    let mut u = sub(pos, cam);
    let d = len(u);
    if d != 0.0 { u = scale(u, 1.0 / d); }
    let near = if d < 11.0 { if 7.0 < d { (d - 7.0) * 0.25 } else { 0.0 } } else { 1.0 };
    let facing = dot(u, dir);
    let a = m.pv_s(A) as i32;
    let s = 20.0 + (2.0 - 20.0) * a as f32 * m.pv_f(INV_A);
    let k = if a < m.pv_s(A_FADE) as i32 { a as f32 * m.pv_f(INV_A_FADE) * 60.0 } else { 60.0 };
    let alpha = (k * facing * facing) as i32 as u32;
    let c = alpha << 24 | 0x7f_7f7f;
    disc(out, 0x38, dir, side, up, ang[0], s, pos, c);
    disc(out, 0x38, dir, side, up, -ang[0], s, pos, c);
    let b = m.pv_s(B) as i32;
    let s = BLAST_SIZE + (1.0 - BLAST_SIZE) * b as f32 * m.pv_f(INV_B);
    let f = if b < m.pv_s(B_FADE) as i32 { let t = b as f32 * m.pv_f(INV_B_FADE); t * t } else { 1.0 };
    let full = (255.0 * f * near) as i32;
    let fifth = |a: i32| ((a as f32 * 0.2) as i32 as u32) << 24 | 0xff_ffff;
    disc(out, 0x38, dir, side, up, ang[1], s * 1.5, pos, fifth(full));
    let a2 = ((255.0 * f * near * 1.5) as i32).min(0xff) as u32;
    disc(out, 0x39, dir, side, up, ang[2], s * 1.25, pos, a2 << 24 | 0xff_ffff);
    disc(out, 0x39, dir, side, up, ang[3], s, pos, a2 << 24 | 0xff_ffff);
    let at = add(pos, set_len(sub(cam, pos), 0.25));
    let fwd = unit(sub(cam, at));
    let across = unit(cross(fwd, [0.0, 0.0, 1.0]));
    let over = cross(across, fwd);
    let c = (full as u32) << 24 | 0xff_ffff;
    disc(out, 0x39, fwd, across, over, ang[4], s, at, c);
    disc(out, 0x39, fwd, across, over, ang[5], s * 0.75, at, c);
    disc(out, 0x38, fwd, across, over, ang[1], s * 1.5, at, fifth(full & 0xff));
    if !count_s(m, GLARE) {
        let k = m.pv_s(GLARE) as f32 * m.pv_f(INV_GLARE);
        glare(out, pos, u, cam, cam_rows, (64.0 * k) as i32 as u32);
    }
    shimmer(&w.mobys[id], cam, frame);
}

/// The glare's star (`0x426410` while `+0x36` runs): `0x2e7c70`'s screen position of the explosion (GS units, 1/16 of a
/// pixel about 2048·16) inside 28492..37082 on both axes; four streaks (FX 0xb, additive) from 2 units in front of the
/// camera toward it, out along the camera's up turned 45° about the view line and then each a quarter turn on, 0.01
/// wide at the root to 0.05 at the tip and `0x104050` from nothing to `alpha`; each as long as `f(u)` with u the
/// box-relative distance (whole units, /8590) to the nearer of the two box edges it points between, f(u) = 2u under
/// ½ and 1 + 5·(2u − 1) past.
fn glare(out: &mut Vec<crate::fx::FxQuad>, pos: V3, u: V3, cam: V3, rows: [V3; 3], alpha: u32) {
    let d = sub(pos, cam);
    let z = dot(rows[0], d);
    if z <= 0.0 { return; }
    let sx = (-dot(rows[1], d) / z / 0.63 * 256.0 + 2048.0) * 16.0;
    let sy = (-dot(rows[2], d) / z / (0.63 * 0.775) * 224.0 + 2048.0) * 16.0;
    let (lo, hi) = (28492.0, 37082.0);
    if !(lo <= sx && sx <= hi && lo <= sy && sy <= hi) { return; }
    let (x, y) = (sx - lo, sy - lo);
    let span = 8590.0;
    let f = |a: f32, b: f32| {
        let t = (a.min(b) as i32) as f32 * 0.000_116_414_434;
        if t < 0.5 { t + t } else { 1.0 + 5.0 * ((t - 0.5) + (t - 0.5)) }
    };
    let lens = [f(y, span - x), f(span - y, span - x), f(span - y, x), f(y, x)];
    // The way out (kept unit here: the game scales one vector to each length in turn).
    let mut way = turn([0.0, 0.0, 1.0], u, 0.785_398_2);
    let root = add(cam, set_len(u, 2.0));
    let (c0, c1) = (0x0010_4050, alpha << 24 | 0x0010_4050);
    for (i, l) in lens.iter().enumerate() {
        let v = scale(way, *l);
        let e = set_len(cross(u, way), 0.01);
        let tip = add(root, v);
        let e5 = scale(e, 5.0);
        out.push(crate::fx::FxQuad {
            fx: 0xb,
            additive: true,
            corners: [add(root, e), sub(root, e), add(tip, e5), sub(tip, e5)],
            st: [[0.0, 1.0], [1.0, 1.0], [0.0, 0.0], [1.0, 0.0]],
            rgba: [c0, c0, c1, c1],
        });
        if i < 3 { way = turn(way, u, std::f32::consts::FRAC_PI_2); }
    }
}
