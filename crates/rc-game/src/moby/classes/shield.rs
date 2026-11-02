//! The Shield Charger (item 0x2d, crate::hero::items::ROWS; L00's record `0x264c60`: its class 0x0a5c, stance 1, icon
//! 0xea74, name 0x32f9, damage 1 (`+0x60`), 5 charges; its upgrade row 0x4d): not held — asked for (the quick select,
//! `0x18c0e8`), the item change `0x2b13c8` raises the shield ([`deploy`], `0x3c4eb8`) around Ratchet: the moby
//! **0x0a5c** (`0x3c51d8`; no model) and the shield's amount in the hero's damage block (`$gp+0x3640`,
//! crate::hero::damage), which takes hits before his health.
//!
//! **Raised** ([`deploy`], `0x3c4eb8(0)`): none up (`$gp+0xf58`) and a charge taken (`0x2d42d0(0x2d, 1)`): the moby
//! (hidden, updated at any distance), its four comets' heads along its rows 0, 1, 2 and 0 + 2 turning about its rows 2,
//! 0, 1 and 1 × (0 + 2) ([`Comet`]); the shield 40 (`0x37fad8(0)`; the upgrade's 100), its class sound 1.
//!
//! **Each tick** ([`update`], `0x3c51d8`; not outside play): on Ratchet; the shield's hit wait counts down
//! (`0x37faa8`); its loop sound (class sound 3) plays. While it holds — some shield left, Ratchet himself (his moby's
//! class 0, not Clank), not in his state 0x31 nor group 0x14, health left (`0x3c6f60`): a change of the amount since
//! the last tick starts its colour's fade (30 ticks from `0x20207f` back to `0x7f2020`); it rides 0.75 up his up row;
//! eight orbs (`0x33bb60`, kind 2, riding it) each 15..40 ticks on a sphere of 1 about his joint 10, `0x7f000000 | the
//! colour` 0.7 across and a white one 0.3 of it; ready (`+0x10`, seconds), every enemy (drawn, class type 5, health
//! left) within 0.9 takes the row's damage (flags 0x10000 along the way to it, kind `0x0a5c << 16 | 0x0203`) with a
//! burst of twinkles where it touches ([`sparks`], `0x3c6d30`), the wait 0.5; five arcs of 10 points from his joint 10,
//! each 3..45 ticks, drawn anew as they run out (a 0.15 step a random way, each turned 5..75° about the way to the
//! camera, one way and the other). Its draw (the first list) is asked for. Else it falls: the sound stopped, the shield
//! cleared (`0x37fb00`), its class sound 2, gone.
//!
//! **The shield's hits** (`0x37fa58`, crate::hero::damage): with the wait out, the hit's damage comes off the shield —
//! what it could not take goes on to Ratchet — and the wait is 15 ticks; within the wait a hit is taken whole.
//!
//! **The draw** ([`draw`], `0x3c5c98`, the frame's first list; while it holds): at Ratchet's joint 10 (`+0x30`): the
//! arcs (`0x3c5d18`: kept in shape from the joint, crate::fx::ribbon: FX 0x17 0.04 across `0x7f7f7f` and FX 0xe four
//! times as wide `0x7f2040`, alpha 32 · their ticks left); the sphere (`0x3c5fe0`: the nanotech's points ×3 facing the
//! camera, FX 0x15 blended, the colour at alpha 0x18; its halo of 32 quads, FX 0xb added, the colour at 0x40 inside);
//! the four comets (`0x3c63c0`: each head turned 6° about its axis, jittered 0.025 and held 0.9..1.1 out, its last 20
//! places kept; two orbs at the head (`0x20000000 | the colour` 0.6 across and `0x407f7f7f` 0.15) and its trail
//! ([`trail`], `0x3c6758`: FX 0xe 0.02 across fading in to `0x207f7f7f`, and a glow 0.07 across stretched 1.3 of
//! each step either way fading in to `0x20000000 | the colour`).
//!
//! Its glow in the hand: none (it is not held; `0x2a7ed8` has no case 0x2d).
//!
//! Not ported: the upgraded shield 0x1192 (rows 0x4d on: 100 strong, its zaps `0x4144f8` on the enemies about it,
//! `$gp+0xf04`), the statistic of the shield's damage (`0x13991c`), the level sound a raise with none left or one up
//! plays (`0x3603c8`), the GS test register the draws set (`0x37c948`).

use crate::hero::items::{row, SHIELD};
use crate::hero::physics::{add, cross, scale, set_len, sub, unit, V3};
use crate::moby::hits::HitTemplate;
use crate::moby::{countdown, Ctx, MobyWorld};

/// The shield's class (item 0x2d's row `+0x04`).
pub const CLASS: i16 = 0xa5c;
/// `0x37fad8(0)`: the shield raised.
pub const STRENGTH: f32 = 40.0;
/// `0x37fa58`: the ticks a hit keeps the next off it.
pub const HIT_WAIT: i32 = 0xf;
/// `$gp+0xed0` / `+0xed4`: the colour at rest and just hit.
const COLOURS: [u32; 2] = [0x7f_2020, 0x20_207f];
/// `$gp+0xf50`: the hit's fade.
const FADE: i16 = 0x1e;
/// `$gp+0xeec`: the shield's size (the sphere's scale; its touch 0.3 of it).
const SIZE: f32 = 3.0;
/// `$gp+0xf0c`: the orbs' sphere and the comets' reach.
const REACH: f32 = 1.0;
/// Ratchet's joint the shield centres on.
const JOINT: usize = 10;
/// The arcs' and the comets' counts and points.
const ARCS: usize = 5;
const ARC_POINTS: usize = 10;
const COMETS: usize = 4;
const TRAIL: usize = 20;

/// A comet (`0x23fb00 + 0x140·k` its last 20 places from the centre, newest first; `0x1b2880 + 0x10·k` its axis;
/// `0x1b28c0 + 4·k` how many are kept).
#[derive(Clone, Debug, Default)]
pub struct Comet {
    pub trail: Vec<V3>,
    pub axis: V3,
    pub count: usize,
}

/// An arc (`0x240000 + 0xa0·k` its points, `0x1b2910` / `0x1b2920` / `0x1b2930 + 2·k` its ticks, alpha and life).
#[derive(Clone, Debug, Default)]
pub struct Arc {
    pub points: [V3; ARC_POINTS],
    pub ticks: i16,
    pub alpha: i16,
    pub life: i16,
}

/// The Shield Charger's globals: the shield up (`$gp+0xf58`), its orbs (`0x1b28d0`, their ticks `0x1b28f0`), arcs and
/// comets; the orbs the draw asked for, made by the next update (the draw has no particles).
#[derive(Clone, Debug, Default)]
pub struct Mem {
    pub moby: Option<usize>,
    pub orbs: [bool; 8],
    pub orb_ticks: [i32; 8],
    pub arcs: [Arc; ARCS],
    pub comets: Vec<Comet>,
    pub heads: Vec<(V3, u32)>,
}

/// `0x321b28(out, v, q)`: `v` turned by `q` (`q·v·q*`).
fn qrot(q: crate::quat::Q, v: V3) -> V3 {
    use crate::quat::mul;
    let t = mul(mul(q, [v[0], v[1], v[2], 0.0]), [-q[0], -q[1], -q[2], q[3]]);
    [t[0], t[1], t[2]]
}

/// `0x2eef88(out, a, b)` = b × a.
fn cross_ba(a: V3, b: V3) -> V3 { cross(b, a) }

/// `0x321c00(a, out, v, axis)`: `v` turned by `a` about `axis` (under 1e-5 none).
fn turn(v: V3, axis: V3, a: f32) -> V3 {
    if a.abs() < 1e-5 { return v; }
    crate::gravity::rotate(v, unit(axis), a)
}

/// `0x37fa58(damage)`: what of a hit gets past the shield (module docs).
pub fn absorb(shield: &mut f32, wait: &mut i32, damage: f32) -> f32 {
    if *wait != 0 { return 0.0; }
    *shield -= damage;
    let mut left = 0.0;
    if *shield < 0.0 {
        left = -*shield;
        *shield = 0.0;
    }
    *wait = HIT_WAIT;
    left
}

/// `0x3c6f60(shield)` (module docs).
fn holds(w: &MobyWorld, ctx: &Ctx) -> bool {
    let h = &*ctx.hero;
    0.0 < h.damage.shield && ctx.hero_moby.and_then(|m| w.mobys.get(m)).is_none_or(|m| m.o_class == 0) && h.mode == 0 && h.state != 0x31 && h.group != 0x14 && 0 < h.hp
}

/// `0x3c4eb8(0)` (module docs): from the item change (crate::hero::items).
pub fn deploy(w: &mut MobyWorld, ctx: &mut Ctx) {
    if w.shield.moby.is_some() { return; }
    let Some(a) = ctx.game.ammo.get_mut(SHIELD as usize) else { return };
    if *a < 1 { return; }
    if w.class(CLASS).is_none() {
        eprintln!("rc_game::shield: class {CLASS:#x} is not loaded: no shield");
        return;
    }
    *a -= 1;
    let id = w.create(CLASS);
    let pos = ctx.hero.pos;
    let rows = {
        let m = &mut w.mobys[id];
        m.pvars.resize(crate::moby::PVAR_BYTES, 0);
        m.mode |= 0x41;
        m.parent = ctx.hero_moby;
        m.update_dist = 0xff;
        m.state = 1;
        m.rows = crate::hero::physics::euler_rows(m.rot);
        m.pos = pos;
        m.rows
    };
    w.shield.moby = Some(id);
    let heads = [rows[0], rows[1], rows[2], add(rows[0], rows[2])].map(|v| set_len(v, REACH));
    let axes = [rows[2], rows[0], rows[1], cross_ba(heads[3], rows[1])];
    w.shield.comets = (0..COMETS).map(|k| {
        let mut trail = vec![[0.0; 3]; TRAIL];
        trail[0] = heads[k];
        Comet { trail, axis: axes[k], count: 0 }
    }).collect();
    w.shield.orbs = [false; 8];
    for a in w.shield.arcs.iter_mut() { a.ticks = 0; }
    ctx.hero.damage.shield = STRENGTH;
    ctx.hero.damage.shield_wait = 0;
    let m = &mut w.mobys[id];
    m.set_pv_f(0, STRENGTH);
    m.set_pv_i(0x14, 0x1e);
    m.set_pv_i(0xc, -1);
    m.set_pv_f(0x10, 0.0);
    for o in [0x18, 0x1a, 0x1c, 0x1e] { m.set_pv_s(o, 0); }
    for o in [0x20, 0x24, 0x28, 0x2c] { m.set_pv_i(o, 0); }
    ctx.host.play_sound(&w.mobys[id], 1, 0);
}

/// The loop sound let go when still playing (`0x3602f0`).
fn stop_sound(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let s = w.mobys[id].pv_i(0xc);
    if s != -1 && ctx.host.sound_playing(s) { ctx.host.stop_sound(s); }
    w.mobys[id].set_pv_i(0xc, -1);
}

/// `0x3c51d8` (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].pvars.resize(crate::moby::PVAR_BYTES, 0);
    if ctx.game.scenes.mode != crate::scene::PLAY { return; }
    w.mobys[id].pos = ctx.hero.pos;
    countdown(&mut ctx.hero.damage.shield_wait);
    let s = w.mobys[id].pv_i(0xc);
    if s == -1 || !ctx.host.sound_playing(s) {
        let h = ctx.host.play_sound(&w.mobys[id], 3, 4);
        w.mobys[id].set_pv_i(0xc, h);
    }
    spawn_heads(w, ctx);
    if !holds(w, ctx) {
        stop_sound(w, id, ctx);
        ctx.hero.damage.shield = 0.0;
        ctx.hero.damage.shield_wait = 0;
        ctx.host.play_sound(&w.mobys[id], 2, 0);
        w.shield.moby = None;
        w.delete(id);
        return;
    }
    let amount = ctx.hero.damage.shield;
    {
        let m = &mut w.mobys[id];
        if amount != m.pv_f(0) {
            m.set_pv_f(0, amount);
            m.set_pv_s(4, FADE);
        }
        let t = m.pv_s(4) as f32 / FADE as f32;
        m.set_pv_i(8, crate::hud::lerp_rgba(t, COLOURS[0], COLOURS[1]) as i32);
        let mut n = m.pv_s(4) as i32;
        countdown(&mut n);
        m.set_pv_s(4, n as i16);
    }
    let colour = w.mobys[id].pv_i(8) as u32;
    let hero_up = ctx.hero_moby.and_then(|h| w.mobys.get(h)).map_or([0.0, 0.0, 1.0], |h| h.rows[2]);
    let pos = add(ctx.hero.pos, scale(hero_up, 0.75));
    w.mobys[id].pos = pos;
    let joint = ctx.hero_moby.and_then(|h| ctx.host.joint(&w.mobys[h], JOINT)).unwrap_or(pos);
    orbs(w, id, ctx, joint, colour);
    // `+0x14`: the upgrade's zaps' wait; `+0x18..`: their ticks (the upgrade's).
    {
        let m = &mut w.mobys[id];
        let mut n = m.pv_i(0x14);
        countdown(&mut n);
        m.set_pv_i(0x14, n);
    }
    let ready = {
        let m = &mut w.mobys[id];
        let c = m.pv_f(0x10);
        if 1.0 / 60.0 < c { m.set_pv_f(0x10, c - 1.0 / 60.0); false } else { m.set_pv_f(0x10, 0.0); true }
    };
    if ready { touch(w, id, ctx, pos, colour); }
    arcs(w, ctx, joint);
    w.register_draw_first(id);
}

/// The eight orbs of the update (module docs).
fn orbs(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, joint: V3, colour: u32) {
    let pos = w.mobys[id].pos;
    for k in 0..8 {
        if !w.shield.orbs[k] {
            let g = &mut ctx.game.rng;
            let life = g.rand_range(0xf, 0x28);
            let yaw = g.rand_angle();
            let pitch = g.rand_angle();
            let at = add(super::shot::polar(REACH, yaw, pitch), joint);
            w.shield.orbs[k] = true;
            w.shield.orb_ticks[k] = life;
            let Some(sys) = ctx.parts.as_deref_mut() else { continue };
            let frame = sys.def_frame(crate::particles::ORB, 1);
            for (size, rgba) in [(0.7 * 210_000.0, 0x7f00_0000 | colour), (0.7 * f32::from_bits(0x4776_1801), 0x307f_7f7f)] {
                if let Some(i) = crate::particles::orb(sys, g, at, life as i16, 2) {
                    crate::particles::orb_attach(sys, i, id, pos);
                    let r = &mut sys.pool.recs[i];
                    crate::particles::rec::set_u32(r, 4, rgba);
                    crate::particles::rec::set_ff(r, 0xc, size);
                    r[2] = frame;
                }
            }
        } else if countdown(&mut w.shield.orb_ticks[k]) {
            w.shield.orbs[k] = false;
        }
    }
}

/// The touch of the update (module docs).
fn touch(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, pos: V3, colour: u32) {
    let r = SIZE * 0.3;
    let near = ctx.scene.map(|sc| crate::collision::coll_sphere_mobys(sc, pos, r, crate::collision::QueryFlags(1), ctx.hero_moby)).unwrap_or_default();
    let damage = row(ctx.game, SHIELD).map_or(1.0, |r| r.damage);
    let up = w.mobys[id].rows[2];
    for t in near {
        let Some(m) = w.mobys.get(t) else { continue };
        if m.drawn == 0 || m.state == 0xfe || m.state == 0xfd || !w.class(m.o_class).is_some_and(|c| c.kind == 5) { continue; }
        let at = match crate::moby::aim::health(w, t) {
            None => m.pos,
            Some((hp, _)) if hp <= 0.0 => continue,
            Some((_, h)) => add(m.pos, scale(up, h)),
        };
        let d = set_len(sub(at, pos), 1.0);
        let mut tm = HitTemplate::new(damage, Some(id), 0x1_0000, [d[0], d[1], d[2], 5627.925]);
        tm.kind = (CLASS as u16 as u32) << 16 | 0x0203;
        w.deliver_hit_any(t, &tm, [0.0; 4]);
        sparks(ctx, add(pos, scale(d, r)), colour);
        w.mobys[id].set_pv_f(0x10, 0.5);
    }
}

/// `0x3c6d30(at, rgb)`: eight pairs of twinkles (crate::particles::twinkle) jittered 0.1, flying 0.5..6 a second a
/// random way, 20 ticks: 1..6 large in `0x7f000000 | rgb`, and a white one 0.4 of it turning the other way; their
/// `+0x09` 0x74.
fn sparks(ctx: &mut Ctx, at: V3, rgb: u32) {
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let g = &mut ctx.game.rng;
    for _ in 0..8 {
        let size = g.randf(1.0, 6.0);
        let r = g.randi(2);
        let s = if r == 0 { -1 } else { r };
        let f = g.randf(0.5, 6.0) * 0.016_666_668;
        let yaw = g.rand_angle();
        let pitch = g.rand_angle();
        let v = super::shot::polar(f, yaw, pitch);
        let p: V3 = std::array::from_fn(|k| at[k] + g.randf_signed(0.0, 0.1));
        if let Some(i) = crate::particles::twinkle(sys, g, size * 0.1, size, 0.0, p, 0x14, rgb | 0x7f00_0000, 0, s as i8, v, 1) { sys.pool.recs[i][9] = 0x74; }
        if let Some(i) = crate::particles::twinkle(sys, g, size * 0.04, size * 0.4, 0.0, p, 0x14, 0x7f7f_7f7f, 1, -s as i8, v, 1) { sys.pool.recs[i][9] = 0x74; }
    }
}

/// The arcs of the update (module docs).
fn arcs(w: &mut MobyWorld, ctx: &mut Ctx, joint: V3) {
    let cam = ctx.host.camera_pos();
    for a in w.shield.arcs.iter_mut() {
        let mut t = a.ticks as i32;
        if countdown(&mut t) { t = 0; }
        a.ticks = t as i16;
        if a.ticks != 0 { continue; }
        let g = &mut ctx.game.rng;
        let life = g.rand_range(3, 0x2d) as i16;
        a.life = life;
        a.alpha = 0x40;
        a.ticks = life;
        // `0x31bb48(0.15, 0.15)`.
        let yaw = g.rand_angle();
        let pitch = g.rand_angle();
        let mut d = super::shot::polar(g.randf(f32::from_bits(0x3e19_999a), f32::from_bits(0x3e19_999a)), yaw, pitch);
        a.points[0] = joint;
        let mut sign = 1.0f32;
        for i in 0..ARC_POINTS - 1 {
            let v = sub(a.points[i], cam);
            let ang = g.randf(f32::from_bits(0x3db2_b8c4), f32::from_bits(0x3fa7_8d37)) * sign;
            sign = -sign;
            d = turn(d, v, ang);
            a.points[i + 1] = add(a.points[i], d);
        }
    }
}

/// The comets' orbs the last draw asked for (`0x33bb60(head, 2, 0, 1)`: kind 0, `0x20000000 | the colour` 0.6 across
/// and `0x407f7f7f` 0.15).
fn spawn_heads(w: &mut MobyWorld, ctx: &mut Ctx) {
    let heads = std::mem::take(&mut w.shield.heads);
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let g = &mut ctx.game.rng;
    let frame = sys.def_frame(crate::particles::ORB, 1);
    for (at, colour) in heads {
        if let Some(i) = crate::particles::orb(sys, g, at, 2, 0) {
            let r = &mut sys.pool.recs[i];
            crate::particles::rec::set_u32(r, 4, 0x2000_0000 | colour);
            crate::particles::rec::set_ff(r, 0xc, 0.6 * 210_000.0);
            r[2] = frame;
        }
        if let Some(i) = crate::particles::orb(sys, g, at, 2, 0) {
            let r = &mut sys.pool.recs[i];
            crate::particles::rec::set_u32(r, 4, 0x407f_7f7f);
            crate::particles::rec::set_ff(r, 0xc, 0.6 * f32::from_bits(0x46f6_1801));
        }
    }
}

/// `0x3c5c98` (module docs): the shield's look, while it holds.
pub fn draw(w: &mut MobyWorld, host: &dyn crate::moby::MobyHost, id: usize, holds: bool, rng: &mut crate::rng::Rng, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    if !holds { return; }
    let hero = w.mobys[id].parent;
    let centre = hero.and_then(|h| w.mobys.get(h)).and_then(|h| host.joint(h, JOINT)).unwrap_or(w.mobys[id].pos);
    w.mobys[id].set_pv_v(0x30, centre);
    let colour = w.mobys[id].pv_i(8) as u32;
    draw_arcs(w, centre, cam, out);
    draw_sphere(w, centre, colour, cam, out);
    draw_comets(w, centre, colour, rng, cam, out);
}

/// `0x3c5d18` (module docs).
fn draw_arcs(w: &mut MobyWorld, centre: V3, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    for a in w.shield.arcs.iter_mut().filter(|a| a.ticks != 0) {
        let steps: Vec<V3> = (0..ARC_POINTS - 1).map(|i| sub(a.points[i + 1], a.points[i])).collect();
        a.points[0] = centre;
        for i in 0..ARC_POINTS - 1 { a.points[i + 1] = add(a.points[i], steps[i]); }
        a.alpha = if a.ticks < 1 { 0 } else { (32.0 * (a.ticks as f32 / a.life as f32)) as i32 as i16 };
    }
    for a in w.shield.arcs.iter().filter(|a| a.ticks != 0) {
        let k = (a.alpha as u32) << 24;
        let pts: Vec<[f32; 4]> = a.points.iter().map(|p| [p[0], p[1], p[2], f32::from_bits(0x3d23_d70a)]).collect();
        crate::fx::ribbon(out, 4.0, 0.0, &pts, cam, [0x17, 0xe], [k | 0x7f_7f7f, k | 0x7f_2040]);
    }
}

/// `0x3c5fe0` (module docs): the nanotech's sphere and halo (crate::moby::classes::nanotech::GlowTables).
fn draw_sphere(w: &MobyWorld, centre: V3, colour: u32, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let Some(t) = w.nanotech.tables.as_ref() else { return };
    let f = set_len(sub(cam, centre), 1.0);
    let r = set_len(cross([0.0, 0.0, 1.0], f), 1.0);
    let u = cross(f, r);
    let rows = [f, r, u].map(|v| scale(v, SIZE));
    let xform = |v: V3| add(centre, crate::gravity::to_world(v, &rows));
    let rgba = 0x1800_0000 | colour;
    for (a, n) in [(0usize, 0x94usize), (146, 0x6d), (254, 0x24)] {
        let pts: Vec<(V3, [f32; 2], u32)> = (a..(a + n).min(t.verts.len())).map(|i| (xform(t.verts[i]), t.st[i], rgba)).collect();
        super::nanotech::strip(out, 0x15, false, &pts);
    }
    let c = colour | 0x4000_0000;
    for q in &t.halo_quads {
        let corners: [V3; 4] = std::array::from_fn(|k| {
            let s = if k < 2 { 0.3 } else { 0.3 * 1.125 };
            xform(scale(t.halo_pts.get(q[k] as usize).copied().unwrap_or_default(), s))
        });
        out.push(crate::fx::FxQuad { fx: 0xb, additive: true, corners, st: [[0.5, 0.5], [0.5, 0.5], [0.5, 1.0], [0.5, 1.0]], rgba: [c, c, 0, 0] });
    }
}

/// `0x3c63c0` (module docs).
fn draw_comets(w: &mut MobyWorld, centre: V3, colour: u32, g: &mut crate::rng::Rng, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let mut heads = Vec::new();
    for c in w.shield.comets.iter_mut() {
        c.count = (c.count + 1).min(TRAIL);
        let n = c.count - 1;
        let mut pts: Vec<V3> = Vec::with_capacity(c.count);
        for i in (1..=n).rev() {
            c.trail[i] = c.trail[i - 1];
            pts.push(add(centre, c.trail[i]));
        }
        let q = crate::quat::axis_angle(f32::from_bits(0x3dd6_7751), c.axis);
        let mut h = qrot(q, c.trail[0]);
        for x in h.iter_mut() { *x += g.randf(-0.025, 0.025); }
        let l = crate::hero::physics::len(h);
        if REACH * 1.1 < l { h = scale(h, REACH * 1.1 / l) } else if l < REACH * 0.9 { h = scale(h, REACH * 0.9 / l) }
        c.trail[0] = h;
        pts.push(add(centre, h));
        heads.push((add(centre, h), colour));
        trail(out, &pts, cam, colour);
    }
    w.shield.heads.extend(heads);
}

/// `0x3c6758(0.02, 0.07, 1.3, points, 20, 0xe, 0xe, 0x207f7f7f, 0x7f7f7f, 0x20000000 | rgb, 0x20000000, 0, n − 1, n)`: the
/// comet's trail from its oldest place to the one before its head — FX 0xe 0.02 across fading from `0x7f7f7f` to
/// `0x207f7f7f` — then over the same steps a glow (FX 0xe, 0.07 across) whose ends are taken to the same distance from
/// the camera and pushed out 1.3 of the step each way, fading from `0x20000000` to `0x20000000 | rgb`; additive.
fn trail(out: &mut Vec<crate::fx::FxQuad>, pts: &[V3], cam: V3, rgb: u32) {
    const ST: [[f32; 2]; 4] = [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]];
    let n = pts.len();
    if n < 3 { return; }
    let lerp = crate::hud::lerp_rgba;
    let step = 1.0 / (n - 1) as f32;
    let side = |p: V3, prev: V3| set_len(cross_ba(sub(p, prev), sub(p, cam)), 0.02);
    let s0 = set_len(cross_ba(sub(pts[1], pts[0]), sub(pts[0], cam)), 0.02);
    let (mut a, mut b) = (add(pts[0], s0), sub(pts[0], s0));
    let mut c0 = 0x7f_7f7fu32;
    let mut t = step;
    for i in 1..n - 1 {
        let s = side(pts[i], pts[i - 1]);
        let (c, d) = (add(pts[i], s), sub(pts[i], s));
        let c1 = lerp(t, 0x7f_7f7f, 0x207f_7f7f);
        out.push(crate::fx::FxQuad { fx: 0xe, additive: true, corners: [a, b, c, d], st: ST, rgba: [c0, c0, c1, c1] });
        (a, b, c0) = (c, d, c1);
        t += step;
    }
    let mut prev = sub(pts[0], cam);
    let mut far = crate::hero::physics::len(prev);
    let mut c0 = 0x2000_0000u32;
    let mut t = step;
    for p in pts.iter().take(n - 1).skip(1) {
        let e = sub(*p, cam);
        let l = crate::hero::physics::len(e);
        let (mut p0, mut p1) = if far < l {
            let q = if far == 0.0 { e } else { scale(prev, l / far) };
            (add(cam, e), add(cam, q))
        } else {
            let q = if l == 0.0 { prev } else { scale(e, far / l) };
            (add(cam, q), add(cam, prev))
        };
        let d = sub(p1, p0);
        let k = scale(d, 1.3);
        p0 = sub(p0, k);
        p1 = add(p1, k);
        let s = set_len(cross_ba(sub(cam, p0), d), 0.07);
        let c1 = lerp(t, 0x2000_0000, 0x2000_0000 | rgb);
        out.push(crate::fx::FxQuad { fx: 0xe, additive: true, corners: [add(p0, s), sub(p0, s), add(p1, s), sub(p1, s)], st: ST, rgba: [c0, c0, c1, c1] });
        c0 = c1;
        prev = e;
        far = l;
        t += step;
    }
}
