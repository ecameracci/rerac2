//! The upgraded Spiderbot's guns (the bot 0x0e79 of the upgraded glove, rows 0x4e / 0x69 / 0x6a; its machine is the
//! base bot's, crate::moby::classes::spiderbot; L00's addresses): its turret that looks about, the twin barrels □
//! fires and the charged twin shell ✕ lobs, run by the bot each tick it walks (states 2 and 3, [`gun`] `0x401c08`),
//! and the two shots they make, **0x0eaa** ([`update_shot`] `0x405248`, made by `0x405338`) and **0x0eab**
//! ([`update_lob`] `0x405ad8`, made by `0x405f70`).
//!
//! **Its words** (pvars, [`pv`]): `+0x00` the bot's velocity (the charge's throw while ✕ is held), `+0x10` the
//! turret's look and `+0x20` where it wants to look (both in the bot's frame), `+0x30` / `+0x40` where the charge
//! lands and the surface's normal, `+0x5c` 0x7f000000 (set, never read), `+0x6c` / `+0x70` / `+0x74` its joint nodes
//! (`0x37f060(bot, list)`: the turret on list 0, the barrels on lists 4 and 3; crate::moby::Moby::joint_nodes, in
//! that order), `+0x78` a landing found, `+0x7c` the charge's reach (1.75 → 6). The globals: the barrels' angles
//! `gp+0x1ca4` / `gp+0x1ca8` and the charge's loop sound `gp+0x1c6c` (crate::moby::classes::spiderbot::Blast).
//!
//! **The turret** ([`gun`]): the target is the best of the aim search all round within 10 of its joint 0
//! (crate::moby::aim::search, cone 2π, no crates). Without ✕ held: a target turns its want toward the target's aim
//! point; else with □ held straight ahead; else one tick in 111 (`rand() % 0x6f`, drawn every such tick) a new look,
//! 45..90° either way about the up from where it looks and up to 0.4 up. Its look turns toward the want by a quarter
//! of the angle a tick with □ or ✕ held, else an eighth (`0x321ca8`), and its node (mode 1, weight 1: the joint's
//! pose replaced) takes the frame of the look across the bot's up, at the joint's rest place.
//!
//! **□ held**: the barrels' angles turn 0.3665 a tick (21°); each barrel's node is shortened along its x to
//! `1 − ((cos a + 1)/2)²/2` (the recoil). An angle crossing 0 upward fires its barrel (the first: joint 2, else the
//! second: joint 1): toward the target's aim point, else along the barrel joint's first row, at 0.25 a tick; class
//! sound 6, the muzzle's flash ([`muzzle`], `0x4038b8`) and the shot ([`make_shot`], `0x405338`).
//!
//! **✕ held** (□ not): from between the barrels (joints 2 and 1), the throw (`0x37e328(1.75, reach, 0.1, 3 a second²,
//! .., bot)`: the reach along the bot's facing, or with L1 the camera's look) into `+0x00` and where it lands
//! (`0x37e988`, `+0x30` / `+0x40` / `+0x78`); the reach grows 0.065 a tick to 6; the sparks at the barrels
//! ([`charge_sparks`], `0x403c00`); the loop sound 4 kept up; the want along the throw. **✕ let go**: the loop sound
//! stopped, class sound 8, no landing, the reach back to 1.75, the flash between the barrels ([`lob_flash`],
//! `0x403a60`) and the shell ([`make_lob`], `0x405f70`).
//!
//! **The shot 0x0eaa** (pvars `+0x00` its velocity, `+0x10` its life 0x50, `+0x14` the streaks' phase, `+0x18` how far
//! it went, `+0x20` the bot): its frame along its velocity. **0** → **1**: it moves, the line from its last place
//! (flags 0, [`shot_hit`] `0x405480`) meets something — a moby of class type 5 or one the lines test (mode 0x4000),
//! not Ratchet, takes 1 (on the row of level 2 10, level 3 15: their `−0x10` flag is set) with flags 0x210000 along
//! its way; then class sound 7 of 0x0e79 at it, **2**; its life out, **2**. **2**: gone. Its draw ([`draw_shot`],
//! `0x405658`): three sprites (FX 0x31 0.2 `0x40801010`, FX 0x31 0.1 `0x707f7f7f`, FX 0x2f 0.05 `0x407f7f7f`, random
//! rolls) and, past its first unit, three streaks 1.2 behind it (FX 0x17: 0.03 `0x7f804040` → `0x804040`, twice 0.1
//! `0x7f801010` → `0x801010`), their phase sliding 0.1 a frame.
//!
//! **The shell 0x0eab** (pvars `+0x00` its velocity, `+0x18` its point light (−1 none), `+0x1c` its roll, `+0x20` its
//! twin's offset (the second barrel from the first), `+0x34` the bot): two shells drawn as one. **0**: no light, a
//! random roll, **1**. **1**: it moves, falls 3 a second², out of the world's 2..1021 box gone; the line from its last
//! place (flags 0, `0x4062c8`) meeting anything puts it there, class sound 5 of 0x0e79, **2**. Each tick in 0 and 1 its
//! draw is asked for ([`draw_lob`], `0x406038`), the trails of both shells ([`trail`], `0x4063c8`) and its light
//! (`0x2fdb60(5, 0, 1, 0.8, 0.25)`, kept 1 above it). **2**: the mobys of class type 5 or mode 0x4000 within 3 take 10
//! (20 on the level 3 row) with flags 0x810000 along the way from it; the light let go; gone.
//!
//! Not ported: the explosion mobys these make (`0x416c40`: the class 0x139d at the shell's blast; `0x405858`'s class
//! 0x139f at a shot's hit, and its two sparks `0x33fbb8`): the port has neither class nor that particle; the weapon
//! mods on the hits (`0x3f9f70`); the gravity frames' bends (`0x37f538` / `0x37f618`, `0x380e08` / `0x380ec0`). The
//! aim search's class list (`0x1b0c8c`: 0, 0x0e79) is not passed (neither is of class type 5). With ✕ and □ both held
//! the original aims a shot at a target with a point it did not set this tick (its stack's leftovers): the port
//! takes the target's aim point.

use super::spiderbot::{PLUS, PLUS_BOT};
use crate::collision::QueryFlags;
use crate::hero::items::SPIDERBOT;
use crate::hero::physics::{add, add_rot, cross, dot, len, scale, set_len, sub, unit, V3};
use crate::moby::hits::HitTemplate;
use crate::moby::{aim, countdown, mode, Ctx, Moby, MobyWorld};
use crate::pad::button::{CROSS, L1, SQUARE};
use rc_formats::moby_anim::JointModifier;

/// The shot and the shell.
pub const SHOT: i16 = 0x0eaa;
pub const LOB: i16 = 0x0eab;

/// The bot's own words (module docs; [`PLUS`] holds the machine's).
pub mod pv {
    pub const AIM: usize = 0x10;
    pub const WANT: usize = 0x20;
    pub const LAND: usize = 0x30;
    pub const NORMAL: usize = 0x40;
    pub const MARK: usize = 0x5c;
    pub const FOUND: usize = 0x78;
    pub const CHARGE: usize = 0x7c;
    /// The port's: the buttons held last tick (`0x138328`, let go = last & !now).
    pub const HELD: usize = 0xa4;
}

/// `0x3257a0(2π, 2π, 10, 0, 0, ..)`.
const CONE: aim::Cone = aim::Cone { yaw: std::f32::consts::TAU, pitch: std::f32::consts::TAU, range: 10.0, near_pitch: 0.0, near_yaw: 0.0 };
/// The turret node's rest place (`+0x30`, model units).
const TURRET_AT: V3 = [f32::from_bits(0xc0fa_a3c2), f32::from_bits(0x2c05_e179), f32::from_bits(0x461c_799a)];
/// The barrels' turn a tick (`0x3ebba867`).
const BARREL_STEP: f32 = f32::from_bits(0x3ebb_a867);
/// The shots' speed a tick.
const SHOT_SPEED: f32 = 0.25;
/// The charge: the least reach, the most, its growth a tick, the throw's speed across a tick and its gravity a tick².
const CHARGE_LEAST: f32 = 1.75;
const CHARGE_MOST: f32 = 6.0;
const CHARGE_GROW: f32 = 0.065;
const LOB_SPEED: f32 = f32::from_bits(0x3dcc_ccce);
const LOB_GRAVITY: f32 = f32::from_bits(0x3a5a_740e);
/// The charge marker's size (`0x4034c8` → `0x37dfc0(0.2, ..)`).
const MARK_SIZE: f32 = 0.2;
/// `t[3]` of the shell's hits (`0x45afdf66`).
const NO_KNOCK_LIMIT: f32 = 5627.987;

/// `0x401ab0`'s part for the guns: its mark, the look and the want ahead (1, 0, 0, 0), the three joint nodes
/// (`0x37f060`: `0x312480` leaves each composing, unrotated, at unit scale), the charge's reach 1.75.
pub fn init(m: &mut Moby) {
    m.set_pv_i(pv::MARK, 0x7f00_0000);
    m.set_pv_v(pv::AIM, [1.0, 0.0, 0.0]);
    m.set_pv_v(pv::WANT, [1.0, 0.0, 0.0]);
    m.joint_nodes = [0u8, 4, 3].iter().map(|&l| (l, JointModifier::compose(0))).collect();
    m.set_pv_f(pv::CHARGE, CHARGE_LEAST);
}

/// The bot's end (state 8): its joint nodes let go (`0x37f110`), and the charge's loop sound with it (its delete
/// `0x311688` ends the moby's sounds).
pub fn release(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].joint_nodes.clear();
    let s = w.spiderbot.charge_sound;
    if s != -1 && ctx.host.sound_playing(s) { ctx.host.stop_sound(s); }
    w.spiderbot.charge_sound = -1;
}

/// `v` in the frame of `rows` (`0x2ef9b0` the transpose, `0x2ef318`).
fn local(v: V3, rows: &[V3; 3]) -> V3 { [dot(v, rows[0]), dot(v, rows[1]), dot(v, rows[2])] }

/// `0x2eef88(out, a, b)`: `b × a`.
fn cr(a: V3, b: V3) -> V3 { cross(b, a) }

/// `0x321ca8(t, out, v, to, 0)`: `v` turned toward `to` by `t` of the angle between them (as it is when either is 0).
fn turn_toward(t: f32, v: V3, to: V3) -> V3 {
    let (lv, lt) = (len(v), len(to));
    if lv * lt == 0.0 { return v; }
    let axis = unit(cr(to, v));
    let a = (std::f32::consts::FRAC_PI_2 - (dot(v, to) / (lv * lt)).clamp(-1.0, 1.0).asin()) * t * 0.5;
    let (s, c) = a.sin_cos();
    // `q · v · q̄` with q = (axis·sin, cos); `quat::rotate` turns by the conjugate.
    crate::quat::rotate([-axis[0] * s, -axis[1] * s, -axis[2] * s, c], v)
}

/// `0x401c08` (module docs).
pub fn gun(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let held = ctx.hero.items.held;
    let let_go = (w.mobys[id].pv_i(pv::HELD) as u32) & !held;
    w.mobys[id].set_pv_i(pv::HELD, held as i32);
    let pos = w.mobys[id].pos;
    let head = ctx.host.joint(&w.mobys[id], 0).unwrap_or(pos);
    let rows = w.mobys[id].rows;
    let target = aim::search(w, ctx, CONE, head, rows[0], false);
    let aim_at = target.map(|t| aim::aim_point(w, t));
    if held & CROSS == 0 {
        if let Some(p) = aim_at {
            w.mobys[id].set_pv_v(pv::WANT, local(unit(sub(p, head)), &rows));
        } else if held & SQUARE != 0 {
            w.mobys[id].set_pv_v(pv::WANT, [1.0, 0.0, 0.0]);
        } else if ctx.game.rng.rand() % 0x6f == 0 {
            // A new look: 45..90° either way about the up from the look's yaw, up to 0.4 up.
            let f = w.mobys[id].pv_v(pv::AIM);
            let left = unit(cr(f, [0.0, 0.0, 1.0]));
            let up = cr(left, f);
            let yaw = crate::moby::attach::angles_of(&[f, left, up])[2];
            let g = &mut ctx.game.rng;
            let turn = g.randf_signed(std::f32::consts::FRAC_PI_4, std::f32::consts::FRAC_PI_2);
            let yaw = add_rot(yaw, turn);
            let pitch = g.randf(0.0, 0.4);
            w.mobys[id].set_pv_v(pv::WANT, super::shot::polar(1.0, yaw, pitch));
        }
    }
    let t = if held & (CROSS | SQUARE) != 0 { 0.25 } else { 0.125 };
    {
        let m = &mut w.mobys[id];
        let look = turn_toward(t, m.pv_v(pv::AIM), m.pv_v(pv::WANT));
        m.set_pv_v(pv::AIM, look);
        // The turret's node (`+0x6c`): the look's frame across the bot's up (`0x31fe30`).
        let left = unit(cr(look, [0.0, 0.0, 1.0]));
        let up = cr(left, look);
        let q = crate::quat::from_rows(&[look, left, up]);
        if let Some((_, n)) = m.joint_nodes.get_mut(0) {
            *n = JointModifier { mode: 1, weight: 1.0, quat: q, scale: [1.0; 3], trans: TURRET_AT, ..*n };
        }
    }
    if held & SQUARE != 0 {
        barrels(w, id, ctx, aim_at);
    } else if held & CROSS != 0 {
        charge(w, id, ctx);
    }
    if let_go & CROSS != 0 {
        let s = w.spiderbot.charge_sound;
        if s != -1 && ctx.host.sound_playing(s) { ctx.host.stop_sound(s); }
        w.spiderbot.charge_sound = -1;
        ctx.host.play_sound(&w.mobys[id], 8, 0);
        let (a, b) = barrel_points(w, id, ctx);
        let mid = scale(add(a, b), 0.5);
        let vel = w.mobys[id].pv_v(PLUS.vel);
        let m = &mut w.mobys[id];
        m.set_pv_i(pv::FOUND, 0);
        m.set_pv_f(pv::CHARGE, CHARGE_LEAST);
        lob_flash(ctx, mid, vel);
        make_lob(w, ctx, id, a, b, vel);
    }
}

/// The barrels' joints 2 and 1 in the world (`0x3127c8`).
fn barrel_points(w: &MobyWorld, id: usize, ctx: &Ctx) -> (V3, V3) {
    let m = &w.mobys[id];
    (ctx.host.joint(m, 2).unwrap_or(m.pos), ctx.host.joint(m, 1).unwrap_or(m.pos))
}

/// □ held (module docs).
fn barrels(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, aim_at: Option<V3>) {
    let [a0, b0] = w.spiderbot.barrels;
    let recoil = |x: f32| {
        let e = (x.cos() + 1.0) * 0.5;
        1.0 - e * e * 0.5
    };
    let (sa, sb) = (recoil(a0), recoil(b0));
    let m = &mut w.mobys[id];
    if let Some((_, n)) = m.joint_nodes.get_mut(1) { n.scale[0] = sa; }
    if let Some((_, n)) = m.joint_nodes.get_mut(2) { n.scale[0] = sb; }
    let (a, b) = (add_rot(a0, BARREL_STEP), add_rot(b0, BARREL_STEP));
    w.spiderbot.barrels = [a, b];
    let j = if a0 < 0.0 && 0.0 <= a {
        2
    } else if b0 < 0.0 && 0.0 <= b {
        1
    } else {
        return;
    };
    let m = &w.mobys[id];
    let muzzle = ctx.host.joint(m, j).unwrap_or(m.pos);
    let way = ctx.host.joint_matrix(m, j).map_or(m.rows[0], |(r, _)| r[0]);
    let dir = match aim_at {
        Some(p) => set_len(sub(p, muzzle), SHOT_SPEED),
        None => set_len(way, SHOT_SPEED),
    };
    ctx.host.play_sound(&w.mobys[id], 6, 0);
    muzzle_flash(ctx, muzzle, dir);
    make_shot(w, ctx, id, muzzle, dir);
}

/// ✕ held, □ not (module docs).
fn charge(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (a, b) = barrel_points(w, id, ctx);
    let mid = scale(add(a, b), 0.5);
    let reach = w.mobys[id].pv_f(pv::CHARGE);
    let vel = throw(w, id, ctx, mid, reach);
    let found = super::gravity_bomb::landing_with(w, ctx, id, mid, vel, LOB_GRAVITY);
    let charge = (reach + CHARGE_GROW).min(CHARGE_MOST);
    {
        let m = &mut w.mobys[id];
        m.set_pv_v(PLUS.vel, vel);
        if let Some((p, n)) = found {
            m.set_pv_v(pv::LAND, p);
            m.set_pv_v(pv::NORMAL, n);
        }
        m.set_pv_i(pv::FOUND, found.is_some() as i32);
        m.set_pv_f(pv::CHARGE, charge);
    }
    charge_sparks(ctx, charge, mid);
    let s = w.spiderbot.charge_sound;
    if s == -1 || !ctx.host.sound_playing(s) { w.spiderbot.charge_sound = ctx.host.play_sound(&w.mobys[id], 4, 4); }
    let rows = w.mobys[id].rows;
    w.mobys[id].set_pv_v(pv::WANT, local(unit(vel), &rows));
}

/// `0x37e328(1.75, reach, 0.1, 3/3600, .., from, 0, bot)` under plain gravity: `reach` along the bot's facing (with
/// L1 held and the items not kept away, the camera's look, its reach eased from the bot's place:
/// crate::moby::classes::gravity_bomb::look_aim_from), 0.1 a tick across, the rise for the fall.
fn throw(w: &MobyWorld, id: usize, ctx: &Ctx, from: V3, reach: f32) -> V3 {
    let m = &w.mobys[id];
    let it = ctx.hero.items;
    let (way, reach) = if it.held & L1 != 0 && it.no_use == 0 { super::gravity_bomb::look_aim_from(ctx, CHARGE_LEAST, reach, m.pos) } else { (m.rows[0], reach) };
    let to = add(m.pos, set_len(way, reach));
    let d = sub(to, from);
    let across = [d[0], d[1], 0.0];
    let mut v = set_len(across, LOB_SPEED);
    let t = len(across) / LOB_SPEED;
    if 0.0 < t { v[2] = (d[2] + LOB_GRAVITY * t * t * 0.5) / t; }
    v
}

/// The sprite `0x33ef78` with its frame from def list `list`'s entry `k` (`*(0x1fe8c0[list] + k)`).
#[allow(clippy::too_many_arguments)]
fn flash_sprite(sys: &mut crate::particles::Particles, g: &mut crate::rng::Rng, size: f32, pos: V3, life: i32, rgb: u32, spin: u8, alpha: [i32; 2], def: (u8, usize)) {
    if let Some(i) = crate::particles::sprite(sys, g, size, size, 0.0, pos, [0.0; 3], life, rgb, spin, alpha[0], alpha[1], 1) {
        let d = sys.def_frame(def.0, def.1);
        sys.pool.recs[i][2] = d;
    }
}

/// `0x4038b8(muzzle, dir)`: nine times, half the shot's step ahead of the muzzle jittered 0.005..0.035 either way, a
/// random spin: a sprite (35700, 4 ticks, `0x808080`, alpha 0x7f → 0x3f, frame `0x5b`[1]) and another (`0x406080`,
/// 0x10 → 8, frame `0x35`[0]).
fn muzzle_flash(ctx: &mut Ctx, muzzle: V3, dir: V3) {
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let g = &mut ctx.game.rng;
    let base = add(muzzle, scale(dir, 0.5));
    for _ in 0..9 {
        let p: V3 = std::array::from_fn(|k| base[k] + g.randf_signed(0.005, 0.035));
        let spin = g.randi(0xff) as u8;
        flash_sprite(sys, g, 35_700.0, p, 4, 0x80_8080, spin, [0x7f, 0x3f], (0x5b, 1));
        flash_sprite(sys, g, 35_700.0, p, 4, 0x40_6080, spin, [0x10, 8], (0x35, 0));
    }
}

/// `0x403a60(mid, vel)`: nine times, half the throw's step ahead jittered 0.1 either way, a random spin: a sprite
/// (84000, 10 ticks, `0x407fff`, alpha 0x7f → 0, frame `0x5c`[5]) and another (`0x406080`, frame `0x35`[0]).
fn lob_flash(ctx: &mut Ctx, mid: V3, vel: V3) {
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let g = &mut ctx.game.rng;
    let base = add(mid, scale(vel, 0.5));
    for _ in 0..9 {
        let p: V3 = std::array::from_fn(|k| base[k] + g.randf_signed(0.0, 0.1));
        let spin = g.randi(0xff) as u8;
        flash_sprite(sys, g, 84_000.0, p, 10, 0x40_7fff, spin, [0x7f, 0], (0x5c, 5));
        flash_sprite(sys, g, 84_000.0, p, 10, 0x40_6080, spin, [0x7f, 0], (0x35, 0));
    }
}

/// `0x403c00(reach, mid)`: twice a grey spark (`0x339848`: reach·6300, growing 500, 15 ticks, alpha 0x5f,
/// `0x7f7f7f`) jittered 0.04 about `mid`, then at the last place a blue one (0.8 of the size, alpha 0x7f, `0x407fff`,
/// frame `0x5c`[5]).
fn charge_sparks(ctx: &mut Ctx, reach: f32, mid: V3) {
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let g = &mut ctx.game.rng;
    let mut p = mid;
    for _ in 0..2 {
        p = std::array::from_fn(|k| mid[k] + g.randf_signed(0.0, 0.04));
        let spin = g.randf_signed(1.0, 3.0) as i32;
        crate::particles::spark(sys, g, reach * 6300.0, 500.0, 1.0, 0.0, 0.0, p, [0.0; 3], 0xf, 0x5f, 0x7f_7f7f, spin);
    }
    let spin = g.randf_signed(1.0, -1.0) as i32;
    if let Some(i) = crate::particles::spark(sys, g, reach * 6300.0 * 0.8, 500.0, 1.0, 0.0, 0.0, p, [0.0; 3], 0xf, 0x7f, 0x40_7fff, spin) {
        let d = sys.def_frame(0x5c, 5);
        sys.pool.recs[i][2] = d;
    }
}

/// The bot's draw (`0x4034c8`, after the sphere): the charge's landing marker, 0.2 across.
pub fn draw_marker(w: &MobyWorld, id: usize, hero_state: i32, mode: i32, frame: i32, out: &mut Vec<crate::fx::FxQuad>) {
    let m = &w.mobys[id];
    if m.pv_i(pv::FOUND) == 0 || matches!(hero_state, 0x17..=0x1b | 0x6f | 0x62) || mode == crate::scene::SCENE { return; }
    super::gravity_bomb::marker_quads_sized(MARK_SIZE, m.pv_v(pv::LAND), m.pv_v(pv::NORMAL), super::gravity_bomb::MARKER_RGBA, frame, out);
}

/// A created shot's common setup (update and draw distance 0xff, state 0).
fn spawn(w: &mut MobyWorld, class: i16, update: crate::moby::UpdateFn, at: V3) -> Option<usize> {
    w.class(class)?;
    let id = w.create(class);
    let m = &mut w.mobys[id];
    m.update = Some(update);
    m.mode &= !mode::NO_UPDATE;
    m.pvars.resize(crate::moby::PVAR_BYTES, 0);
    m.update_dist = 0xff;
    m.draw_dist = 0xff;
    m.drawn = 1;
    m.state = 0;
    m.pos = at;
    w.changed = true;
    Some(id)
}

/// `0x405338(bot, at, vel)`: the shot 0x0eaa at `at` moving `vel` a tick, its life 0x50, its frame along `vel`
/// across the gravity's up (`0x326448`).
fn make_shot(w: &mut MobyWorld, ctx: &mut Ctx, bot: usize, at: V3, vel: V3) {
    let up = ctx.hero.gravity.up_at(at);
    let Some(id) = spawn(w, SHOT, update_shot, at) else { return };
    let m = &mut w.mobys[id];
    m.set_pv_v(0, vel);
    m.set_pv_i(0x10, 0x50);
    m.set_pv_f(0x14, 0.0);
    m.set_pv_f(0x18, 0.0);
    m.set_pv_i(0x20, bot as i32);
    let f = unit(vel);
    let l = unit(cr(f, up));
    m.rows = [f, l, cr(l, f)];
}

/// `0x405248` (module docs).
pub fn update_shot(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 | 1 => {
            w.mobys[id].state = 1;
            let m = &mut w.mobys[id];
            let from = m.pos;
            m.pos = add(m.pos, m.pv_v(0));
            shot_hit(w, id, ctx, from);
            let m = &mut w.mobys[id];
            let mut t = m.pv_i(0x10);
            if countdown(&mut t) { m.state = 2; }
            m.set_pv_i(0x10, t);
        }
        2 => {
            w.delete(id);
            return;
        }
        _ => {}
    }
    w.register_draw(id);
}

/// `0x405480(shot, from)` (module docs).
fn shot_hit(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, from: V3) {
    let to = w.mobys[id].pos;
    let Some(h) = w.coll_line(ctx, from, to, 0, Some(id)) else { return };
    if let Some(t) = h.moby.filter(|&t| Some(t) != ctx.hero_moby) {
        let x = &w.mobys[t];
        let kind = w.class(x.o_class).map_or(0, |c| c.kind);
        if kind == 5 || x.mode & mode::LINE != 0 {
            let damage = match crate::upgrades::level_of(ctx.game.item_row(SPIDERBOT)) {
                2 => 10.0,
                3 => 15.0,
                _ => 1.0,
            };
            let d = unit(w.mobys[id].pv_v(0));
            let mut tm = HitTemplate::new(damage, Some(id), 0x0021_0000, [d[0], d[1], d[2], 0.0]);
            tm.kind = (SHOT as u16 as u32) << 16 | 0x0305;
            w.deliver_hit_any(t, &tm, [0.0; 4]);
        }
    }
    ctx.host.play_sound_of(&w.mobys[id], PLUS_BOT, 7, 0);
    w.mobys[id].state = 2;
}

/// `0x405658` (module docs).
pub fn draw_shot(w: &mut MobyWorld, id: usize, g: &mut crate::rng::Rng, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let m = &mut w.mobys[id];
    let mut phase = m.pv_f(0x14) - 0.1;
    if phase <= -8.0 { phase += 8.0; }
    m.set_pv_f(0x14, phase);
    let went = m.pv_f(0x18) + len(m.pv_v(0));
    m.set_pv_f(0x18, went);
    let (pos, rows) = (m.pos, m.rows);
    crate::fx::sprite(out, 0.2, 0.0, g.rand_angle(), pos, cam, 0x31, 0x4080_1010, true);
    crate::fx::sprite(out, 0.1, 0.0, g.rand_angle(), pos, cam, 0x31, 0x707f_7f7f, true);
    crate::fx::sprite(out, 0.05, 0.0, g.rand_angle(), pos, cam, 0x2f, 0x407f_7f7f, true);
    if 1.0 < went {
        let at = [[0.0; 3]];
        crate::fx::strip(out, 0.03, 1.2, phase, &rows, pos, cam, &at, 0x7f80_4040, 0x80_4040, 0x17);
        crate::fx::strip(out, 0.1, 1.2, phase, &rows, pos, cam, &at, 0x7f80_1010, 0x80_1010, 0x17);
        crate::fx::strip(out, 0.1, 1.2, phase, &rows, pos, cam, &at, 0x7f80_1010, 0x80_1010, 0x17);
    }
}

/// `0x405f70(bot, at, twin, vel)`: the shell 0x0eab at `at` moving `vel`, its twin `twin − at` from it.
fn make_lob(w: &mut MobyWorld, _ctx: &mut Ctx, bot: usize, at: V3, twin: V3, vel: V3) {
    let Some(id) = spawn(w, LOB, update_lob, at) else { return };
    let m = &mut w.mobys[id];
    m.set_pv_v(0, vel);
    m.set_pv_v(0x20, sub(twin, at));
    m.set_pv_i(0x34, bot as i32);
}

/// Outside the world's 2..1021 box.
fn outside(p: V3) -> bool { p.iter().any(|&x| !(2.0..=1021.0).contains(&x)) }

/// `0x405ad8` (module docs).
pub fn update_lob(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let mut from = w.mobys[id].pos;
    match w.mobys[id].state {
        0 => {
            let roll = ctx.game.rng.rand_angle();
            let m = &mut w.mobys[id];
            m.set_pv_i(0x18, -1);
            m.state = 1;
            m.set_pv_f(0x1c, roll);
        }
        1 => {
            let m = &mut w.mobys[id];
            from = m.pos;
            let mut vel = m.pv_v(0);
            m.pos = add(m.pos, vel);
            vel[2] -= LOB_GRAVITY;
            m.set_pv_v(0, vel);
            // `0x4062c8`.
            if outside(w.mobys[id].pos) {
                lob_gone(w, id);
                return;
            }
            let to = w.mobys[id].pos;
            if let Some(h) = w.coll_line(ctx, from, to, 0, Some(id)) {
                w.mobys[id].pos = h.point;
                w.mobys[id].state = 2;
                ctx.host.play_sound_of(&w.mobys[id], PLUS_BOT, 5, 0);
            }
            if outside(w.mobys[id].pos) {
                lob_gone(w, id);
                return;
            }
        }
        2 => {
            lob_blast(w, id, ctx);
            lob_gone(w, id);
            return;
        }
        _ => {
            lob_gone(w, id);
            return;
        }
    }
    w.register_draw(id);
    let (pos, twin) = (w.mobys[id].pos, w.mobys[id].pv_v(0x20));
    trail(w, id, ctx, pos, from);
    trail(w, id, ctx, add(pos, twin), add(from, twin));
    let light = crate::point_lights::PointLight { color: [1.0, 0.8, 0.25], intensity: 0.0, pos: add(pos, [0.0, 0.0, 1.0]), radius: 5.0 };
    let slot = w.mobys[id].pv_i(0x18);
    if slot == -1 {
        // `0x2fdb60(5, 0, 1, 0.8, 0.25, pos)`: taken at its place, raised from the next tick on.
        let s = w.point_lights.alloc(crate::point_lights::PointLight { pos, ..light }).map_or(-1, |s| s as i32);
        w.mobys[id].set_pv_i(0x18, s);
    } else {
        w.point_lights.set(slot as usize, light);
    }
}

/// The shell gone: its light let go (`0x2fdc48`), deleted.
fn lob_gone(w: &mut MobyWorld, id: usize) {
    let s = w.mobys[id].pv_i(0x18);
    if 0 <= s { w.point_lights.free(s as usize); }
    w.mobys[id].set_pv_i(0x18, -1);
    w.delete(id);
}

/// State 2's hits (module docs).
fn lob_blast(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let Some(sc) = ctx.scene else { return };
    let at = w.mobys[id].pos;
    let damage = if crate::upgrades::level_of(ctx.game.item_row(SPIDERBOT)) == 3 { 20.0 } else { 10.0 };
    for t in crate::collision::coll_sphere_mobys(sc, at, 3.0, QueryFlags(0), Some(id)) {
        let Some(x) = w.mobys.get(t) else { continue };
        let kind = w.class(x.o_class).map_or(0, |c| c.kind);
        if !(kind == 5 || x.mode & mode::LINE != 0) { continue; }
        let d = unit(sub(x.pos, at));
        let mut tm = HitTemplate::new(damage, Some(id), 0x0081_0000, [d[0], d[1], d[2], NO_KNOCK_LIMIT]);
        tm.kind = (x.o_class as u16 as u32) << 16 | 0x0305;
        let p = x.pos;
        w.deliver_hit_any(t, &tm, [p[0], p[1], p[2], 1.0]);
    }
}

/// `0x4063c8(shell, to, from)`: `|vel|·10` blue sparks (`0x339848`: 42000 shrinking 500..1000 a tick, 30..45 ticks,
/// `0x407fff` at alpha 0x60 (normal blend) or 0x40 (additive), frame `0x17`[0]) stepping from `from` toward `to`,
/// rising 0.1..0.2 a second; one time in four an ember (`0x3348c0`: 31500, `0x7f407fff` → `0xff`, 30..45 ticks,
/// falling 0.1 a second², frame `0x35`[0]) at the shell drifting back against its way, jittered 1/60, up 0.025.
fn trail(w: &MobyWorld, id: usize, ctx: &mut Ctx, to: V3, from: V3) {
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let g = &mut ctx.game.rng;
    let vel = w.mobys[id].pv_v(0);
    let speed = len(vel);
    let n = (speed * 10.0) as i32;
    let step = set_len(sub(to, from), speed / n as f32);
    let up = scale(ctx.hero.gravity.down, -1.0);
    let mut p = from;
    for _ in 0..n.max(0) {
        let spin = g.randf_signed(1.0, 3.0) as i32;
        let rise = scale(up, g.randf(0.1, 0.2) * 0.016_666_668);
        let life = g.rand_range(0x1e, 0x2d) as i16;
        let grow = -g.randf(500.0, 1000.0);
        let alpha = if g.rand() & 2 != 0 { 0x40 } else { 0x60 };
        let i = crate::particles::spark(sys, g, 42_000.0, grow, 1.0, 0.0, 0.0, p, rise, life, alpha, 0x40_7fff, spin);
        p = add(p, step);
        if let Some(i) = i {
            let d = sys.def_frame(0x17, 0);
            let r = &mut sys.pool.recs[i];
            r[2] = d;
            r[3] = if alpha == 0x40 { 0x48 } else { 0x44 };
        }
    }
    if g.randi(4) == 0 {
        let mut v = scale(vel, f32::from_bits(0xbd08_8889));
        for x in &mut v { *x += g.randf(-0.016_666_668, 0.016_666_668); }
        v[2] += 0.025;
        let life = g.rand_range(0x1e, 0x2d);
        if let Some(i) = crate::particles::ember(sys, g, f32::from_bits(0x46f6_1801), w.mobys[id].pos, v, 0x7f40_7fff, 0xff, life, 0, None, None, 0) {
            let d = sys.def_frame(0x35, 0);
            let r = &mut sys.pool.recs[i];
            r[0x2c..0x30].copy_from_slice(&f32::from_bits(0x3ada_740e).to_le_bytes());
            r[2] = d;
        }
    }
}

/// `0x406038` (`gp+0x1db8..0x1dd8`): its roll up 1° a frame; three sprites FX 0x2a 0.2 across (`0x2040ff` at alpha
/// 10..16) at −roll, roll, 2·roll; three FX 0x39 0.3 (`0x207f7f7f`) at −1.5·roll, roll/2, 2.5·roll; two FX 0x31 0.35
/// (`0x10007fff`) at ±roll/2; one FX 0x2f 0.1 (`0x507f7f7f`, a random roll); and at the twin one FX 0x2f 0.09. Each
/// 0.08 toward the camera, additive.
pub fn draw_lob(w: &mut MobyWorld, id: usize, g: &mut crate::rng::Rng, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let m = &mut w.mobys[id];
    let roll = add_rot(m.pv_f(0x1c), 1.0 * 0.017_453_294);
    m.set_pv_f(0x1c, roll);
    let (pos, twin) = (m.pos, add(m.pos, m.pv_v(0x20)));
    const PUSH: f32 = 0.08;
    let c = (g.rand_range(10, 0x10) as u32) << 24 | 0x20_40ff;
    for r in [-roll, roll, add_rot(roll + roll, 0.0)] { crate::fx::sprite(out, 0.2, PUSH, r, pos, cam, 0x2a, c, true); }
    for r in [add_rot(-roll * 1.5, 0.0), roll * 0.5, add_rot(roll * 2.5, 0.0)] { crate::fx::sprite(out, 0.3, PUSH, r, pos, cam, 0x39, 0x207f_7f7f, true); }
    for r in [roll * 0.5, roll * -0.5] { crate::fx::sprite(out, 0.35, PUSH, r, pos, cam, 0x31, 0x1000_7fff, true); }
    crate::fx::sprite(out, 0.1, PUSH, g.rand_angle(), pos, cam, 0x2f, 0x507f_7f7f, true);
    crate::fx::sprite(out, 0.09, PUSH, g.rand_angle(), twin, cam, 0x2f, 0x507f_7f7f, true);
}
