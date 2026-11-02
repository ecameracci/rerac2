//! The mobys' draw callbacks (`0x2e7920(fn, moby)`: registered during the moby's update, called by the frame's draw
//! after every update): the quads they send to the GS (`0x2eb998`, an FX texture, ALPHA 0x48 additive or 0x44),
//! which the engine draws through its fx_draw. They run after the tick ([`callbacks`]) in the moby order, as the frame
//! calls them, so the random draws some make (a glow's roll) come after the tick's.
//!
//! | Callback | Moby | Draws |
//! |---|---|---|
//! | `0x3d8340` | 0x0b0a | its three glows ([`glow`]: the pilot light at `+0x3a0`, the nozzle `+0x3b0` and the mouth `+0x3c0`); climbing, its wire (`0x3d8498`) |
//! | `0x3dc220` | 0x0b0b | its three glows ([`glow`]: the visor at joint 4, the chest at joint 5, below it joint 6 at (0.25, 0, 0)) |
//! | `0x3f4998` | 0x0ce9 in glow mode | a glow at each tie of its group ([`crate::moby::classes::tie_lights::draw`]) |
//! | `0x3e4050` | the Lancer's shot | its trail's newest point and the trail ([`crate::moby::classes::shot::draw_trail`]) |
//! | `0x3a17f0` | the nanotech cluster 0x326 (the first list) | its sphere, halo and crate glass ([`crate::moby::classes::nanotech::draw`]) |
//! | `0x3efd68` | the Gravity Bomb in the hand 0xce3 | where its throw lands ([`crate::moby::classes::gravity_bomb::draw_marker`]) |
//! | `0x3f0e80` | the Gravity Bomb's blast 0xce5 | its shockwave and flares ([`crate::moby::classes::gravity_bomb::draw_blast`]) |
//! | `0x41f9a8` | the Chopper's blade 0x087a with a trail | its trail and the Mega's lightning ([`crate::moby::classes::chopper::draw`]) |
//! | `0x3a7968` | the Blitz Gun in the hand 0x05d1 after a shot | its muzzle's starburst and flash, the screen flash ([`crate::moby::classes::blitz::draw`]) |
//! | `0x3fd908` | the Blitz Gun's tracer 0x0d5e | its beam ([`crate::moby::classes::blitz::draw_tracer`]) |
//! | `0x3feba0` | the beam 0x0dc1 (the upgrade screen's lightning) | its bolt ([`crate::moby::classes::beam::draw`]) |
//! | `0x3d4320` / `0x3fe250` | the upgraded Blitz Gun 0x0af1 after a shot / its tracer 0x0d9c | its arcs, ring and glare ([`crate::moby::classes::blitz_mega::draw`]); the beam |
//! | `0x3a9a48` | the Pulse Rifle's shot 0x05d6 with smoke or a beam | its slot's smoke and beams ([`crate::moby::classes::pulse_rifle::draw`]) |
//! | `0x3eecd0` | the Synthenoids' shot 0x0cbb | its streaks ([`crate::moby::classes::synthenoid::draw`]) |
//! | `0x41c328` | the Lava Gun in the hand 0x05c9 drawn | its stream, muzzle glows and burning pools ([`crate::moby::classes::lava_gun::draw`]) |
//! | `0x3ec108` | the Mini-Turret Glove in the hand 0x0c5a | where its throw lands ([`crate::moby::classes::turret::draw_marker`]) |
//! | `0x389818` | the Mini-Turret 0x1366 | its muzzle flash and its shots ([`crate::moby::classes::turret::draw`]) |
//! | `0x2a7ac8` (first list) | Ratchet | the hand item's glows ([`HeroGlow`]; its other glows are not ported) |
//! | `0x3c6eb8` (L02) | the Spiderbot Glove in the hand 0x0907 | where its throw lands ([`crate::moby::classes::spiderbot::draw_marker`]) |
//! | `0x3cac30` | Oozla's conveyor 0x0a7b out | its belt of light ([`crate::moby::classes::oozla::draw_0a7b`]) |
//! | `0x3d2ea8` | Oozla's hatch 0x0a9d open | its light ([`crate::moby::classes::oozla::draw_0a9d`]) |
//! | `0x414118` | Maktar's power node 0x0c82 showing | its arc to the lock ([`ribbon`], crate::moby::classes::maktar::draw_0c82) |
//! | `0x413460` | Maktar's lock 0x0c81 showing | its rings of light (crate::moby::classes::maktar::draw_0c81) |
//! | `0x3ec7c8` | Maktar's conveyor bridge 0x0ab0 running | its belt's beam (crate::moby::classes::maktar::draw_0ab0) |
//! | `0x3e90c8` | Maktar's slot machine 0x0a9c drawn | its reels (crate::moby::classes::maktar::draw_0a9c) |
//! | `0x402bc8` | the keeper's shot 0x0b5a flying | its glow and streak ([`halo`], [`streak`]; crate::moby::classes::keeper::draw_shot) |
//! | `0x38e4c8` | the Tractor Beam holding | its beam ([`crate::moby::classes::tractor::draw`]) |
//! | `0x3d7858` / `0x3dc2d0` | Maktar's arena bosses 0x09f8 / 0x0a44 | their lights (crate::moby::classes::bosses) |
//! | `0x423e10` | 0x0a44's shot 0x0d08 (the first list) | its glow and sparks (crate::moby::classes::bosses::draw_shot) |
//! | `0x4137d0` | the space fights' laser 0xffa | a streak ([`crate::ship::draw_laser`]) |
//! | `0x3d20e8` (L03) | the Endako boss's laser 0x09df | its glows and strips ([`sprite`], crate::moby::classes::endako_boss::draw_beam) |
//! | `0x426410` (L02) | the big explosion 0x0d4e | its discs and glare ([`crate::moby::classes::explosion::draw`]); its shimmer ([`FrameQuad`]) |
//! | `0x3fac68` (L02) | the Spiderbot 0x0b58 | its blast's sphere ([`FrameQuad`], [`crate::moby::classes::spiderbot::draw_blast`]) |
//! | `0x4034c8` | the upgraded Spiderbot 0x0e79 | its blast's sphere and its charge's marker ([`crate::moby::classes::spiderbot_plus::draw_marker`]) |
//! | `0x405658` / `0x406038` | its shot 0x0eaa / shell 0x0eab | sprites and streaks ([`crate::moby::classes::spiderbot_plus::draw_shot`], [`crate::moby::classes::spiderbot_plus::draw_lob`]) |
//!
//! **The frame's own image** (`0x2ea6c8(n)` for n < 0: a texture of a GS buffer instead of an FX texture): −1 queues a
//! GS copy of the frame as it stands (the draw buffer `0x1a72dc` scaled by the sprites `0x138ad0` into a 256×256 CT16
//! texture at the next texture page; made once a frame, `gp−0x6520` cleared by the frame start `0x2ea390`), −7 the
//! buffer on show (`0x1a72d8`, DISPFB: the last finished frame) as it is. The quads that sample them are
//! [`FrameQuad`]s ([`Lists::frame`]): the engine draws them after the second list's quads, the copy taken before the
//! first of them (a full-size copy here, not the 256×256 CT16 one).
//!
//! Before them, the frame's blob shadows (`0x2e7ab0`, crate::shadows: FX 0, ALPHA 0x44) go first into the same quads.
//!
//! The frame's first list (`0x2e7718`, run by `0x2e7768` before the particles, `0x3311e0`; [`Lists::first`]) holds on
//! Aranos the arrival ship's draw (`0x362b40`, which the engine's created-moby draw does), the ship's shadow
//! (`0x361af8`), class 0xd0's ribbon (`0x39ac30`: the Swingshot's line, crate::moby::classes::swingshot::draw_line)
//! and the nanotech clusters' glow; its other users are
//! other levels' classes (G-REN-003).
//!
//! Not ported: 0x0b0a's emerge flash
//! (`0x3d8920` → `0x2ee190`, FX 0x3b), 0x0b0b's wire (`0x3db790`), the group branch of `0x3dc220` (a grouped 0x0b0b
//! draws every drawn member of its class in the group: here each draws its own).

use crate::hero::physics::{add, cross, len, scale, sub, V3};
use crate::moby::MobyWorld;

/// One quad (`0x2eb998`): FX texture, additive (0x48) or normal (0x44), corners in GS strip order, ST, RGBA.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FxQuad {
    pub fx: usize,
    pub additive: bool,
    pub corners: [V3; 4],
    pub st: [[f32; 2]; 4],
    pub rgba: [u32; 4],
}

/// One of the hand item's glows (`0x2a7e60(size, push, point, rgba)` into the queue `0x18bea0`, eight at most, emptied by
/// the hero update `0x2a6dd0`): `0x2a7ac8` draws each by `0x326d60` — FX 0xb, additive, a ±1 square × `size` facing the
/// camera, `push` toward it ([`sprite`] without a roll).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HeroGlow {
    pub size: f32,
    pub push: f32,
    pub point: V3,
    pub rgba: u32,
}

/// The frame image a [`FrameQuad`] samples (module docs).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FrameSource {
    /// `0x2ea6c8(−1)`: the frame as it stands when the quad draws (made once a frame, before the first such quad).
    Copy,
    /// `0x2ea6c8(−7)`: the last finished frame (the buffer on show).
    Shown,
}

/// A quad textured with the frame's own image (`0x2eb998` with a [`FrameSource`]'s TEX0): each corner samples the
/// image at its own screen place (`0x2ea5b8`: 0..1 across and down the screen) plus `offset`, held to `0..clamp` (the
/// engine takes the place per pixel and interpolates the offsets, so that a quad reaching past the screen or near the
/// camera does not smear the image across it; the game interpolates the corners' clamped places as ST/Q).
/// `mask`: the FX texture whose alpha (at `st`, MODULATE with the vertex alpha) blends it over the frame (ALPHA 0x44);
/// None: it replaces the frame (`(Cd − Cd)·As + Cs`, ALPHA 0x05). Its colour is MODULATEd by `rgba` and fogged.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FrameQuad {
    pub source: FrameSource,
    pub mask: Option<usize>,
    pub corners: [V3; 4],
    pub st: [[f32; 2]; 4],
    pub offset: [[f32; 2]; 4],
    pub clamp: f32,
    pub rgba: [u32; 4],
}

/// `0x2eef20`'s rejection of a quad with a corner past the front clip plane, as the game's near distance (`0x18cda0`,
/// 32 / 1024): true when every corner is in front of the camera (`rows[0]` its forward row) by more than it.
pub fn in_front(corners: &[V3; 4], cam: V3, forward: V3) -> bool {
    corners.iter().all(|c| crate::hero::physics::dot(sub(*c, cam), forward) > 32.0 / 1024.0)
}

const ST: [[f32; 2]; 4] = [[0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [1.0, 1.0]];

fn unit(v: V3) -> V3 {
    let l = len(v);
    if 0.0 < l { scale(v, 1.0 / l) } else { v }
}

/// `0x2edd50(a, b, phase, moby, points, n, c0, c1, fx)`: a quad behind each of a moby's `points` (in its frame: `rows`
/// and `pos`), from the point back along its facing by `b` and `a` to either side across the view (`(−forward) ×
/// (camera − pos)`); s runs from `phase` to `phase + 1` along it, t across; colours c0 at the point and c1 at the back;
/// additive (the effect texture's ALPHA row 1, `0x2ecef8(fx, 1, 0x80)`).
#[allow(clippy::too_many_arguments)]
pub fn strip(out: &mut Vec<FxQuad>, a: f32, b: f32, phase: f32, rows: &[V3; 3], pos: V3, cam: V3, points: &[V3], c0: u32, c1: u32, fx: usize) {
    let back = scale(rows[0], -1.0);
    let side = crate::hero::physics::set_len(cross(back, sub(cam, pos)), a);
    let back = crate::hero::physics::set_len(back, b);
    let st = [[phase, 0.0], [phase, 1.0], [phase + 1.0, 0.0], [phase + 1.0, 1.0]];
    for p in points {
        let w = add(add(add(scale(rows[0], p[0]), scale(rows[1], p[1])), scale(rows[2], p[2])), pos);
        let tail = add(w, back);
        out.push(FxQuad { fx, additive: true, corners: [sub(w, side), add(w, side), sub(tail, side), add(tail, side)], st, rgba: [c0, c0, c1, c1] });
    }
}

/// `0x2edf78(size, push, roll, point, fx, rgba, additive)`: a sprite facing the camera, `push` toward it: its rows the
/// way to the camera, that across gravity (`0x18a0e0`, down) and the third, turned by `roll` about the first; the
/// template `0x1aa310` (±1 square) × `size`.
#[allow(clippy::too_many_arguments)]
pub fn sprite(out: &mut Vec<FxQuad>, size: f32, push: f32, roll: f32, point: V3, cam: V3, fx: usize, rgba: u32, additive: bool) {
    let d = unit(sub(cam, point));
    let c = unit(cross(d, [0.0, 0.0, -1.0]));
    let b = unit(cross(c, d));
    let (s, co) = roll.sin_cos();
    let (c, b) = (add(scale(c, co), scale(b, s)), sub(scale(b, co), scale(c, s)));
    let p = add(point, scale(d, push));
    let corner = |y: f32, z: f32| add(p, add(scale(c, y * size), scale(b, z * size)));
    out.push(FxQuad { fx, additive, corners: [corner(-1.0, 1.0), corner(-1.0, -1.0), corner(1.0, 1.0), corner(1.0, -1.0)], st: [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]], rgba: [rgba; 4] });
}

/// `0x3d86a0(size, m, point, rgb, alpha, alpha2)`: a soft glow (FX 0xb, additive) at `point` pulled 0.5 toward the
/// camera, facing it with a random roll (`0x31bb00`): the template's ±2 square × `size` in `rgb` at `alpha`, then half
/// of it at `alpha2`.
pub fn glow(out: &mut Vec<FxQuad>, rng: &mut crate::rng::Rng, size: f32, point: V3, rgb: u32, alpha: u32, alpha2: u32, cam: V3) {
    let roll = rng.rand_angle();
    let f = unit(sub(cam, point));
    let p = add(point, scale(f, 0.5));
    // A basis across the view, turned by the roll.
    let up = if f[2].abs() < 0.99 { [0.0, 0.0, 1.0] } else { [1.0, 0.0, 0.0] };
    let r0 = unit(cross(up, f));
    let u0 = cross(f, r0);
    let (s, c) = roll.sin_cos();
    let r = add(scale(r0, c), scale(u0, s));
    let u = sub(scale(u0, c), scale(r0, s));
    for (k, a) in [(1.0, alpha), (0.5, alpha2)] {
        let h = 2.0 * size * k;
        let corner = |y: f32, z: f32| add(p, add(scale(r, y * h), scale(u, z * h)));
        let rgba = (rgb & 0xff_ffff) | a << 24;
        out.push(FxQuad { fx: 0xb, additive: true, corners: [corner(-1.0, -1.0), corner(1.0, -1.0), corner(-1.0, 1.0), corner(1.0, 1.0)], st: ST, rgba: [rgba; 4] });
    }
}

/// `0x2f10c8(point, rgb)` (L02): a glow of three quads of FX 0x2c (additive) facing the camera at `point` pulled 0.25
/// toward it, rolled a random way: ±2·k across (k `randf(0.3, 0.4)`) at half `rgb` and alpha `rand_range(0x30, 0x40)`,
/// then half that size at `rgb` and then a quarter, white (`0x7f7f7f`), each at alpha `rand_range(0x40, 0x7f)`.
pub fn halo(out: &mut Vec<FxQuad>, rng: &mut crate::rng::Rng, point: V3, rgb: u32, cam: V3) {
    let a = rng.rand_range(0x30, 0x40) as u32;
    let half = (rgb >> 17) << 16 | (rgb >> 9 & 0xff00) | (rgb >> 1 & 0xff);
    let d = sub(cam, point);
    let yaw = crate::hero::physics::atan2(d[0], d[1]);
    let pitch = -crate::hero::physics::atan2((d[0] * d[0] + d[1] * d[1]).sqrt(), d[2]);
    let roll = rng.rand_angle();
    let rows = crate::hero::physics::euler_rows([roll, pitch, yaw]);
    let at = add(point, scale(unit(d), 0.25));
    let k = rng.randf(0.3, 0.4);
    let corner = |y: f32, z: f32, k: f32| add(at, add(scale(rows[1], y * k), scale(rows[2], z * k)));
    let quad = |k: f32| [corner(-2.0, -2.0, k), corner(-2.0, 2.0, k), corner(2.0, -2.0, k), corner(2.0, 2.0, k)];
    let st = [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]];
    out.push(FxQuad { fx: 0x2c, additive: true, corners: quad(k), st, rgba: [a << 24 | half; 4] });
    let c = rgb.wrapping_add((rng.rand_range(0x40, 0x7f) as u32) << 24);
    out.push(FxQuad { fx: 0x2c, additive: true, corners: quad(k * 0.5), st, rgba: [c; 4] });
    let c = (rng.rand_range(0x40, 0x7f) as u32) << 24 | 0x7f_7f7f;
    out.push(FxQuad { fx: 0x2c, additive: true, corners: quad(k * 0.25), st, rgba: [c; 4] });
}

/// `0x2f1430(scroll, step, head, points, rgb, n)` (L02): a streak of FX 0x17 (additive) from `head` through `points`,
/// facing the camera: a narrow strip (half-width `randf(0.2, 0.45)`, alpha 0x7f) and one 1.5 times as wide (alpha
/// 0x3f), each edge after the first ¾ of the one before; quad k (1..n) takes s from `step·k` (less 1 past 1) plus
/// `scroll` to `step` more, t across (flipped one time in two); the last edge `rgb`'s own alpha.
#[allow(clippy::too_many_arguments)]
pub fn streak(out: &mut Vec<FxQuad>, rng: &mut crate::rng::Rng, scroll: f32, step: f32, head: V3, points: &[V3], rgb: u32, cam: V3) {
    let Some(&first) = points.first() else { return };
    let mut w = rng.randf(0.2, 0.45);
    let (t0, t1) = if rng.rand() & 1 == 0 { (0.0, 1.0) } else { (1.0, 0.0) };
    let side = |p: V3, next: V3| unit(cross(sub(cam, p), unit(sub(next, p))));
    let (ca, cb) = (rgb | 0x7f00_0000, rgb | 0x3f00_0000);
    let mut s = side(head, first);
    let mut prev = (head, s, w);
    let n = points.len();
    for k in 1..=n {
        let p = points[k - 1];
        if k < n { s = side(p, points[k]); }
        w *= 0.75;
        let mut s0 = step * k as f32;
        let mut s1 = s0 + step;
        if 1.0 <= s0 { s0 -= 1.0; s1 -= 1.0; }
        let (s0, s1) = (s0 + scroll, s1 + scroll);
        let end = if k == n { (rgb, rgb) } else { (ca, cb) };
        for (wide, start, stop) in [(1.0, ca, end.0), (1.5, cb, end.1)] {
            let (a, aw, b, bw) = (prev.0, prev.2 * wide, p, w * wide);
            let corners = [add(a, scale(prev.1, aw)), sub(a, scale(prev.1, aw)), add(b, scale(s, bw)), sub(b, scale(s, bw))];
            out.push(FxQuad { fx: 0x17, additive: true, corners, st: [[s0, t0], [s0, t1], [s1, t0], [s1, t1]], rgba: [start, start, stop, stop] });
        }
        prev = (p, s, w);
    }
}

/// `0x2f03a8(_, k, v, points, fx₀, fx₁, n, c₀, c₁)` (L02): a ribbon through `points` (`{x, y, z, half-width}`)
/// facing the camera — at each point its half-width across `dir × (point − camera)` (`dir` from the last point, the
/// first's toward the second) — drawn twice per segment: `fx₀` in `c₀`, then `fx₁` in `c₁` `k` times as wide; s runs
/// `v..v + 1` along each segment, t across; additive.
pub fn ribbon(out: &mut Vec<FxQuad>, k: f32, v: f32, points: &[[f32; 4]], cam: V3, fx: [usize; 2], c: [u32; 2]) {
    let n = points.len();
    if n < 2 { return; }
    let p = |i: usize| -> V3 { [points[i][0], points[i][1], points[i][2]] };
    let side = |i: usize| -> V3 {
        let dir = if i == 0 { sub(p(1), p(0)) } else { sub(p(i), p(i - 1)) };
        scale(unit(cross(dir, sub(p(i), cam))), points[i][3])
    };
    let st = [[v, 0.0], [v, 1.0], [v + 1.0, 0.0], [v + 1.0, 1.0]];
    for i in 1..n {
        let (a, b) = (side(i - 1), side(i));
        let (pa, pb) = (p(i - 1), p(i));
        out.push(FxQuad { fx: fx[0], additive: true, corners: [add(pa, a), sub(pa, a), add(pb, b), sub(pb, b)], st, rgba: [c[0]; 4] });
        let (a, b) = (scale(a, k), scale(b, k));
        out.push(FxQuad { fx: fx[1], additive: true, corners: [add(pa, a), sub(pa, a), add(pb, b), sub(pb, b)], st, rgba: [c[1]; 4] });
    }
}

/// `$gp+0x12f0` / `+0x12f4`: 0x0b0a's glow colours.
const GLOW_0B0A: [u32; 2] = [0x0000_36f6, 0x0010_50b0];

/// `0x3d8340` (0x0b0a, every tick): climbing (states 4, 5, 10, or sub-state 2), its wire (`0x3d8498`: FX 0xe, 0.05
/// wide, white at alpha 0x1f, from `+0x380` to `+0x390`); then the glows.
fn draw_0b0a(w: &MobyWorld, id: usize, rng: &mut crate::rng::Rng, cam: V3, out: &mut Vec<FxQuad>) {
    let m = &w.mobys[id];
    if matches!(m.state, 4 | 5 | 10) || m.sub_state == 2 {
        let (a, b) = (m.pv_v(0x380), m.pv_v(0x390));
        let side = scale(unit(cross(sub(cam, a), unit(sub(b, a)))), 0.05);
        let c = 0x1f7f_7f7f;
        out.push(FxQuad { fx: 0xe, additive: true, corners: [add(a, side), sub(a, side), add(b, side), sub(b, side)], st: ST, rgba: [c; 4] });
    }
    glow(out, rng, 0.125, m.pv_v(0x3a0), GLOW_0B0A[0], 0x20, 0x20, cam);
    glow(out, rng, 0.15, m.pv_v(0x3b0), GLOW_0B0A[1], 0x30, 0x20, cam);
    glow(out, rng, 0.15, m.pv_v(0x3c0), GLOW_0B0A[1], 0x10, 0x30, cam);
}

/// 0x0b0b's glow colours (`0x3dc220`'s words): the visor orange, the chest blue.
const GLOW_0B0B: [u32; 2] = [0x0000_36f6, 0x00ff_3e00];

/// `0x3dc220` (0x0b0b, drawn): `0x3db998` (= [`glow`]) at joint 4 (0.125, orange, 0x20 / 0x20), joint 5 (0.15, blue,
/// 0x30 / 0x20) and joint 6's matrix × (0.25, 0, 0) (0.15, blue, 0x10 / 0x30).
fn draw_0b0b(w: &MobyWorld, host: &dyn crate::moby::MobyHost, id: usize, rng: &mut crate::rng::Rng, cam: V3, out: &mut Vec<FxQuad>) {
    let m = &w.mobys[id];
    if m.drawn == 0 { return; }
    let (Some(a), Some(b), Some((rows, p))) = (host.joint(m, 4), host.joint(m, 5), host.joint_matrix(m, 6)) else { return };
    let c = add(p, scale(rows[0], 0.25));
    glow(out, rng, 0.125, a, GLOW_0B0B[0], 0x20, 0x20, cam);
    glow(out, rng, 0.15, b, GLOW_0B0B[1], 0x30, 0x20, cam);
    glow(out, rng, 0.15, c, GLOW_0B0B[1], 0x10, 0x30, cam);
}

/// The frame's callbacks after the tick, by list: the first (`0x2e7718`'s, drawn before the particles) and the second
/// (`0x2e7920`'s, after them; the blob shadows and the ship's shadow first), each in the order the mobys registered
/// ([`MobyWorld::draws_first`] / [`MobyWorld::draws`]), the first list's run first. `hero_state`: Ratchet's
/// (`0x18c0b4`). `view`: the frame's camera for the callbacks' sphere checks (`0x2f0080`).
#[derive(Clone, Debug, Default)]
pub struct Lists {
    pub first: Vec<FxQuad>,
    pub second: Vec<FxQuad>,
    /// The quads of either list that sample the frame's own image (module docs, [`FrameQuad`]), in call order.
    pub frame: Vec<FrameQuad>,
}

pub fn callbacks(w: &mut MobyWorld, host: &dyn crate::moby::MobyHost, game: &mut crate::game::GameState, hero_state: i32, view: &crate::particles::BSphereView, cam: V3) -> Lists {
    let mut first = Vec::new();
    let mut frame = Vec::new();
    w.screen_fills.clear();
    // The ship's shadow (`0x361af8`, a first-list callback; crate::travel).
    crate::travel::draw_ship_shadow(game, view, &mut first);
    // Ratchet's hand-item glows (`0x2a7ac8`, a first-list callback of his).
    for g in &w.hero_glows { sprite(&mut first, g.size, g.push, 0.0, g.point, cam, 0xb, g.rgba, true); }
    for id in w.draws_first.clone() { one(w, host, game, hero_state, view, cam, id, true, &mut first, &mut frame); }
    let mut out = Vec::new();
    // The blob shadows first (`0x2e7ab0` runs before the callbacks' list, `0x2e7970`).
    crate::shadows::draw(&w.shadows, &mut out);
    for id in w.draws.clone() { one(w, host, game, hero_state, view, cam, id, false, &mut out, &mut frame); }
    Lists { first, second: out, frame }
}

/// One registered callback (the class's).
#[allow(clippy::too_many_arguments)]
fn one(w: &mut MobyWorld, host: &dyn crate::moby::MobyHost, game: &mut crate::game::GameState, hero_state: i32, view: &crate::particles::BSphereView, cam: V3, id: usize, first: bool, out: &mut Vec<FxQuad>, frame: &mut Vec<FrameQuad>) {
    use crate::moby::classes::tie_lights;
    if w.mobys.get(id).is_none_or(|m| 0x80 <= m.state) { return; }
    match w.mobys[id].o_class {
        crate::moby::classes::spiderbot::BOT_CLASS => crate::moby::classes::spiderbot::draw_blast(w, id, cam, host.camera_rows(), frame),
        crate::moby::classes::spiderbot::PLUS_BOT => {
            crate::moby::classes::spiderbot::draw_blast(w, id, cam, host.camera_rows(), frame);
            crate::moby::classes::spiderbot_plus::draw_marker(w, id, hero_state, game.scenes.mode, game.frame, out);
        }
        crate::moby::classes::spiderbot_plus::SHOT => crate::moby::classes::spiderbot_plus::draw_shot(w, id, &mut game.rng, cam, out),
        crate::moby::classes::spiderbot_plus::LOB => crate::moby::classes::spiderbot_plus::draw_lob(w, id, &mut game.rng, cam, out),
        0x0ce9 => tie_lights::draw(w, id, view, cam, out),
        0x0b0a => draw_0b0a(w, id, &mut game.rng, cam, out),
        0x0b0b | 0x0b22 => draw_0b0b(w, host, id, &mut game.rng, cam, out),
        crate::moby::classes::shot::SHOT_CLASS => crate::moby::classes::shot::draw_trail(&mut w.mobys[id], cam, &mut game.trail_phase, out),
        crate::moby::classes::nanotech::CLASS => crate::moby::classes::nanotech::draw(w, id, cam, out),
        crate::moby::classes::gravity_bomb::HAND_CLASS => crate::moby::classes::gravity_bomb::draw_marker(w, id, hero_state, game.frame, out),
        crate::moby::classes::gravity_bomb::BLAST_CLASS => crate::moby::classes::gravity_bomb::draw_blast(w, id, view, cam, out),
        crate::moby::classes::chopper::BLADE => crate::moby::classes::chopper::draw(w, id, game, cam, out),
        crate::moby::classes::blitz::HAND_CLASS => crate::moby::classes::blitz::draw(w, id, cam, out),
        crate::moby::classes::blitz::TRACER | crate::moby::classes::blitz::MEGA_TRACER => crate::moby::classes::blitz::draw_tracer(w, id, cam, out),
        crate::moby::classes::blitz_mega::HAND_CLASS => crate::moby::classes::blitz_mega::draw(w, id, &mut game.rng, cam, out),
        crate::moby::classes::beam::CLASS => crate::moby::classes::beam::draw(w, id, cam, out),
        crate::moby::classes::pulse_rifle::SHOT => crate::moby::classes::pulse_rifle::draw(w, id, hero_state, cam, out),
        crate::moby::classes::synthenoid::SHOT => crate::moby::classes::synthenoid::draw(w, id, cam, out),
        crate::moby::classes::lava_gun::HAND_CLASS => crate::moby::classes::lava_gun::draw(w, id, &mut game.rng, cam, out),
        crate::moby::classes::turret::HAND_CLASS => crate::moby::classes::turret::draw_marker(w, id, hero_state, game.scenes.mode, game.frame, out),
        crate::moby::classes::turret::TURRET => crate::moby::classes::turret::draw(w, host, id, &mut game.rng, cam, out),
        crate::moby::classes::spiderbot::HAND_CLASS | crate::moby::classes::spiderbot::PLUS_HAND => crate::moby::classes::spiderbot::draw_marker(w, id, hero_state, game.scenes.mode, game.frame, out),
        0x0a7b => crate::moby::classes::oozla::draw_0a7b(w, id, out),
        0x0a9d => crate::moby::classes::oozla::draw_0a9d(w, id, out),
        0x0a17 => crate::moby::classes::oozla::draw_0a17(w, id, game, cam, host.camera_rows(), out),
        crate::vendor::CLASS => crate::moby::classes::vendor::draw(w, id, game, host.camera_rows(), out),
        c if crate::moby::classes::flock::CLASSES.contains(&c) => crate::moby::classes::flock::draw(w, id, view, cam, out),
        crate::moby::classes::critter::CLASS => crate::moby::classes::critter::draw(w, id, cam, out),
        0x0ab0 => crate::moby::classes::maktar::draw_0ab0(w, id, out),
        crate::moby::classes::maktar::NODE => crate::moby::classes::maktar::draw_0c82(w, id, cam, out),
        0x0c81 => crate::moby::classes::maktar::draw_0c81(w, id, cam, out),
        0x0a9c => crate::moby::classes::maktar::draw_0a9c(w, id, out),
        crate::moby::classes::fighters::MASTER if first => {
            let mid = w.mobys[id].pv_v(crate::moby::classes::arena::MIDDLE_AT);
            crate::moby::classes::arena::draw_moat(w, id, mid, &mut game.rng, cam, out);
        }
        crate::moby::classes::fighters::MASTER => crate::moby::classes::arena::draw_lamps(w, id, cam, out),
        crate::moby::classes::arena::PROPS => {
            let tick = game.scenes.player.as_ref().map(|p| p.tick);
            crate::moby::classes::arena::draw_props(w, id, &mut game.rng, tick, cam, out);
        }
        crate::moby::classes::arena::COLUMN => crate::moby::classes::arena::draw_column(w, id, cam, out),
        crate::moby::classes::fighters::C3B => crate::moby::classes::fighters::draw_0c3b(w, id, &mut game.rng, cam, out),
        crate::moby::classes::bosses::C9F8 => crate::moby::classes::bosses::draw_09f8(w, host, id, cam, out),
        crate::moby::classes::bosses::CA44 => crate::moby::classes::bosses::draw_0a44(w, host, id, cam, out),
        crate::moby::classes::bosses::SHOT => crate::moby::classes::bosses::draw_shot(w, id, &mut game.rng, cam, out),
        crate::moby::classes::keeper::SHOT | crate::moby::classes::keeper::ROCKET => crate::moby::classes::keeper::draw_shot(w, id, &mut game.rng, cam, out),
        crate::moby::classes::endako_boss::CLASS => crate::moby::classes::endako_boss::draw(w, host, id, cam, out),
        crate::moby::classes::endako_boss::BEAM => crate::moby::classes::endako_boss::draw_beam(w, id, &mut game.rng, cam, out),
        crate::moby::classes::teleport::CLASS => crate::moby::classes::teleport::draw(w, id, &host.camera_rows(), out),
        crate::moby::classes::explosion::CLASS => crate::moby::classes::explosion::draw(w, id, cam, host.camera_rows(), out, frame),
        0x00bc => crate::moby::classes::tractor::draw(w, id, cam, -0.3 * (game.frame % 27) as f32, out),
        crate::moby::classes::swingshot::HAND_CLASS => crate::moby::classes::swingshot::draw_line(w, host, id, cam, out),
        crate::ship::LASER_CLASS => crate::ship::draw_laser(w, id, cam, out),
        crate::moby::classes::endako_c::SENTRY => crate::moby::classes::endako_c::draw_beam(w, id, game.frame, cam, out),
        _ => {}
    }
}
