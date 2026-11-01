//! Endako (level 3)'s boss, **0x0c8a** (L03 `0x3f5c78`, one placed; unit U201), a flying gunship with its pilot 0x0c49
//! aboard, and the shots it makes: the tracer **0x0a72** (made by `0x3dae28`, its update `0x3dab80`), the sweeping laser
//! **0x09df** (made by `0x3d1fd8`, its update `0x3d1ae8`) and the rocket **0x0b1b** (made by `0x3e2ee8`; its update
//! `0x3e2958` is the keeper's shot's code, crate::moby::classes::keeper::update_shot). L03 addresses throughout; the
//! engine functions are named by their L03 addresses (the port's L00 ones in the docs they link to).
//!
//! **pvars** (Endako's placed block): `[0]` → the health block `+0x20` (health `+0x20`, its most `+0x24`, `+0x2e` 2 a kill
//! asked from outside; the damages: `+0x34` a tracer 1, `+0x35` the laser 2, `+0x36` a rocket 3), `[0xc]` → the target
//! `+0x50`, the colour flash `+0x70`. `[0x80]` / `[0x84]` the paths of its two passes (59, 60), `[0x88]` its fight's
//! path (88); the cuboids: `[0x8c]` / `[0x90]` start a pass (36, 38), `[0x98]` / `[0x9c]` end it (37, 39), `[0x94]` the
//! fight's (40); `[0xa4]` / `[0xa8]` two mobys held (`0x32bf18`, crate::moby::Moby::once bit 2) until its end and its
//! fight; `[0xac]` the group of ammo crates it brings back; the music `[0xb0]` at the fight (18), `[0xb4]` / `[0xb8]` at
//! its end (0, 22); `+0x3d0..+0x404` its fourteen voice lines.
//!
//! **Each tick** ([`update`]): first [`senses`] (`0x3f6bd0`), then its state, then [`tail`] (`0x3f76c0`).
//! * [`senses`]: in state 0 the set-up — with its mission done it goes; its most health by the mission's rank
//!   (`0x2e50d0`: 1 × 0.7, 2 × 0.9); its six armour pieces drawn (`+0x7c` 0x3f); the next quarter `+0x334` (¾ of it);
//!   its place `+0x290` and home `+0x2b0` where it stands; hidden, no collision, not a target, waiting (1, sub 0); the
//!   first attack the laser (`+0x30b`), its side `+0x330` 1; the stops of its fight's path (the points whose `w` ≥ 0,
//!   seven at most, `+0x314..`); `[0xa8]` / `[0xa4]` held; its crates put away (`0x3f7d18` → `0x3a4478`,
//!   crate::moby::classes::crate_::put_away). Every tick: its scale 0.9 of its class's; a hit (flags 0x10000, not from
//!   its own laser 0x09df) of some damage (`0x31f150`, crate::moby::ai::damage_of) or a kill asked flashes it red
//!   (`+0x77` 0x96), plays its sound 1. Before its fight (sub 0) a hit in its first pass sends it away (3). In the
//!   fight the health goes down (a kill: to 0); out of it, dying (0xe, sub 1); under the next quarter its two pieces
//!   of the quarter `+0x308` break off (`0x2dca30` mode 1, crate::moby::shards::spawn_ball: spin 30, gravity 15,
//!   7/60 to either side and 5/60 up in its frame), the quarter counts up, the hit's reaction (0xd, sub 6) and the
//!   next quarter is the one under its health. In states 6..0xd it looks for Ratchet (`0x32c4e8(16, 8)`,
//!   crate::moby::ai::find_target). Then its voice ([`voice`], `0x3f84c8`).
//! * **States**:
//!   * **1** waiting for pass `+0x2d4` (0, 1): Ratchet in its cuboid `[0x8c + 4k]` shows it 7 below the start of the
//!     pass's path `[0x80 + 4k]` facing him, rising to it (2); in the fight's cuboid `[0x94]` it goes to wait (4).
//!   * **2** the pass: Ratchet in `[0x98 + 4k]` marks `0x1395c2` and sends it along its path to the end (3).
//!   * **3** at the end it hides: after the first pass waits for the second (1), after the second for the fight (4).
//!   * **4** Ratchet in the fight's cuboid: the scene 0x36 queued and started once (`$gp+0x203c`), 5.
//!   * **5** after 20 ticks, no scene playing: the fight (6, sub 2) — shown, a target, 7 below its fourth stop rising to
//!     it facing Ratchet, the wait `+0x2e0` 60, `[0xa8]` let go, the music `[0xb0]`.
//!   * **6** the fight (sub 2): on its first tick its sequence 0 and the next stop ([`next_stop`], `0x3f7a98`: an
//!     even one after its guns, else an odd one; never the stop it heads for); at the stop, the wait over, an attack
//!     ([`attack`], `0x3f7908`).
//!   * **7** / **8** the guns (sub 3): its two turrets (joint lists 0, 1) turn to ±40° (`+0x2f8`), then (sequence 1,
//!     2 or 3 by which turn) sweep back to the other side at 20° a second firing each from its muzzle (lists 2, 3)
//!     every eighth frame (staggered by four): a tracer 0x0a72 at 24 a second along the muzzle's row, jittered 0.1
//!     ([`spawn_tracer`]); its sound 3. Both through, 6.
//!   * **9** (sub 4): it hovers (no code runs in its state).
//!   * **0xc** the laser (sub 5): every 90 ticks a laser 0x09df from its side's joint (lists 5 / 6 by turns), moving at
//!     first across its frame at the rank's speed (`$gp+0x2000`: 5.5, 7, 8 a second) and down to 159 over 60 ticks,
//!     then toward Ratchet ([`spawn_beam`]); its wait (330) out, 6.
//!   * **0xd** the hit (sub 6): its sequence 8, at its end 0xa (sub 4) heading for an odd stop, its speed 0.
//!   * **0xa** (sub 4): the rockets to fire (`0x31cda8`: by rank 2..3, 3..5, 4..6); sequence 4, at its end 0xb.
//!   * **0xb** the rockets: sequence 6; at its frame 2 a rocket 0x0b1b from joint list 4 along its facing (18 a second,
//!     flat), its sound 4; the rockets fired, 6.
//!   * **0xe** dying (sub 1): stopped where it is, rolling to 15°; every 45 ticks a blast (crate::moby::blast, size 1)
//!     at a random point of its frame and its sound 2; its bob quickens; after 180 ticks the big explosion
//!     (crate::moby::classes::explosion) facing the camera and it falls (0xf, the path dropped, −5/60 a tick).
//!   * **0xf** the fall: a blast every 15 ticks (no flash), it rolls to 45° and pitches to 90°, falling faster; 1 under
//!     its home its bolts (`0x31c188(m, 0, 3)`); 12 under: its camera tweak let go, `[0xa4]` let go, the music `[0xb4]`
//!     / `[0xb8]`, the pilot deleted, `0x1395c3` marked, **its mission done** (`0x3146c0`), its loop stopped, gone.
//! * [`tail`]: its flash; [`glide`] (`0x3f71b0`); its camera ([`camera`], `0x3f7b28`); the rotor and the turrets
//!   ([`turn_parts`], `0x3f7c50`); the crates back ([`crates_back`], `0x3f7d88`); the pilot ([`pilot`], `0x3f7798`);
//!   from sub 2 the boss bar (`0x2fddb0(health / most, 5, 0)`: picture 5); its draws ([`draw`]: the tracers'
//!   `0x3f7f88` with any kept, the rotor's glow `0x3f81e8` drawn last frame); its ticks.
//!
//! **The laser** 0x09df ([`update_beam`], [`draw_beam`]): its line hits as it sweeps, throwing sparks ([`sparks`],
//! `0x3d25d0`) at the hit and down the line; drawn as four glows and two pulsing strips straight down from it.
//!
//! **The guns' aim**: the turrets' turns are manipulators on the boss's joint lists 0 and 1 (`turn_parts`), drawn by the
//! engine on the placed moby; the muzzles' frames (`0x313b40` → `0x3189c0`, the joints posed with the moby's `+0x54`
//! manipulator list) carry them, as the port's joint queries now do.
//!
//! **Its end**: the bolts, the music, `0x1395c3`, the mission (the level's mission end, crate::game::GameState::
//! mission_done). No scene, item or skill point of its own.
//!
//! **Dev**: `RC_ENDAKO_BOSS=1` puts Ratchet at the fight's cuboid `[0x94]` on the boss's first tick (so the fight starts
//! at once) and logs its state changes on stdout.
//!
//! Not ported:
//! * Its camera's turn toward it (`0x3f7b28` writes the follow camera's yaw rate `+0x1d4` and scripted yaw `+0x1dc`; the
//!   moby update has no camera): only the tweak (distance 8, pivot 3, priority 5) is asked for.
//! * The pilot's second aim (`0x327aa8`), the hero word `0x18a03e`
//!   (0xf0) of its taunt, the line's start held for its stream (`0x330128`: the port's dialogue starts at once; the
//!   pilot's sequence 1 starts with the line).
//! * A laser carried by a moby (`+0x40`: the boss gives none).

use crate::hero::physics::{add, add_rot, approach, approach_angle, atan2, cross, dist, ease_speed, euler_rows, len, len2, scale, set_len, sub, sub_rot, turn_ease, turn_spring, unit, V3};
use crate::moby::{ai, countdown, mode, Ctx, Moby, MobyWorld};
use super::props::{seq, wrapped};

pub const CLASS: i16 = 0x0c8a;
/// Its pilot (made by `0x3f7798`; no update of its own).
pub const PILOT: i16 = 0x0c49;
pub const TRACER: i16 = 0x0a72;
pub const BEAM: i16 = 0x09df;
pub use super::keeper::ROCKET;

/// Degrees to radians; a second's degrees to a tick's radians (`0.00029088825`).
const DEG: f32 = 0.017_453_294;
const DEG_S: f32 = 0.000_290_888_25;

/// The boss's tuning (`$gp+0x2000..+0x2064` on Endako).
mod tune {
    /// `+0x2000`: the laser's speed by the mission's rank (a second).
    pub const BEAM_SPEED: [f32; 3] = [5.5, 7.0, 8.0];
    /// `+0x200c`: the tracers' speed (a second).
    pub const TRACER_SPEED: f32 = 24.0;
    /// `+0x2010` / `+0x2014`: the laser's ticks to turn toward Ratchet, a laser every 90 ticks.
    pub const BEAM_TICKS: i32 = 120;
    pub const BEAM_EVERY: u16 = 90;
    /// `+0x201c`: the scene before the fight on.
    pub const SCENE_ON: bool = true;
    /// `+0x2024`: the guns' sweep (degrees a second).
    pub const SWEEP: f32 = 20.0;
    /// `+0x2028`: the laser attack's ticks.
    pub const BEAM_WAIT: i32 = 330;
    /// `+0x202c`: the height the laser goes to.
    pub const BEAM_Z: f32 = 159.0;
    /// `+0x2034`: a tracer's streak per tick of its trail (a second).
    pub const STREAK: f32 = 24.0;
    /// `+0x2038`: the rockets' speed (a second).
    pub const ROCKET_SPEED: f32 = 18.0;
    /// `+0x2040`: its scale over its class's (0x3f666666).
    pub const SCALE: u32 = 0x3f66_6666;
    /// `+0x2044` / `+0x2048`: the bob's size and height in the other subs.
    pub const BOB: [f32; 2] = [0.3, 0.3];
    /// `+0x204c`: the bob's speed; `+0x2050` its lean and `+0x2054` its veer at full speed; `+0x2058` its glide's
    /// accel and brake (degrees and units a second, a second²).
    pub const BOB_SPEED: f32 = 100.0;
    pub const LEAN: f32 = 20.0;
    pub const VEER: f32 = 60.0;
    pub const GLIDE: f32 = 10.0;
    /// `+0x2060` / `+0x2064`: its camera tweak's distance and pivot height.
    pub const CAM: [f32; 2] = [8.0, 3.0];
    /// The guns' turn (0x3f32b8c4, 40°).
    pub const GUN_TURN: u32 = 0x3f32_b8c4;
}

/// The boss's pvars (module docs).
mod pv {
    pub const HP: usize = 0x20;
    pub const MAX: usize = 0x24;
    pub const KILL: usize = 0x2e;
    pub const DMG_TRACER: usize = 0x34;
    pub const DMG_BEAM: usize = 0x35;
    pub const DMG_ROCKET: usize = 0x36;
    pub const TARGET: usize = 0x50;
    pub const FLASH: usize = 0x70;
    pub const PASS_PATH: usize = 0x80;
    pub const FIGHT_PATH: usize = 0x88;
    pub const PASS_IN: usize = 0x8c;
    pub const FIGHT_IN: usize = 0x94;
    pub const PASS_OUT: usize = 0x98;
    pub const HOLD_END: usize = 0xa4;
    pub const HOLD_FIGHT: usize = 0xa8;
    pub const CRATES: usize = 0xac;
    pub const MUSIC: usize = 0xb0;
    pub const MUSIC_END: usize = 0xb4;
    pub const STINGER_END: usize = 0xb8;
    /// The turrets' and the pilot's aim blocks (`0x325588`): `+0x48` the turn, `+0x58` its speed, `+0x68` the turn
    /// asked for this tick.
    pub const AIM_GUN: [usize; 2] = [0x110, 0x190];
    pub const AIM_PILOT: usize = 0x210;
    pub const PLACE: usize = 0x290;
    pub const ROT: usize = 0x2a0;
    pub const HOME: usize = 0x2b0;
    pub const PHASE: usize = 0x2c0;
    /// The pilot + 1; the camera tweak's slot + 1; the path + 1 (the original's pointers).
    pub const PILOT: usize = 0x2c4;
    pub const TWEAK: usize = 0x2c8;
    pub const PATH: usize = 0x2cc;
    pub const SPEED: usize = 0x2d0;
    pub const PASS: usize = 0x2d4;
    pub const AT: usize = 0x2d8;
    pub const TO: usize = 0x2dc;
    pub const WAIT: usize = 0x2e0;
    pub const ATTACK: usize = 0x2e4;
    pub const SWEEP: usize = 0x2e8;
    pub const GUN_TO: usize = 0x2f8;
    pub const BOB: usize = 0x300;
    pub const BOB_Z: usize = 0x304;
    pub const STAGE: usize = 0x308;
    pub const NEXT_GUNS: usize = 0x309;
    pub const NEXT_STAY: usize = 0x30a;
    pub const NEXT_BEAM: usize = 0x30b;
    pub const STOPS: usize = 0x314;
    pub const SIDE: usize = 0x330;
    pub const NEXT_HP: usize = 0x334;
    pub const ROTOR: usize = 0x338;
    pub const ROCKETS: usize = 0x33c;
    pub const TRACERS: usize = 0x340;
    pub const N_TRACERS: usize = 0x3bc;
    pub const BOLTS: usize = 0x3c0;
    pub const RANK: usize = 0x3c2;
    pub const YAW_V: usize = 0x3c4;
    pub const LEAN_V: usize = 0x3c8;
    pub const LOOP: usize = 0x3cc;
    pub const LINES: usize = 0x3d0;
    pub const LINE_WAIT: usize = 0x408;
    pub const LAST_LINE: usize = 0x40c;
    pub const SAID: usize = 0x410;
    /// The port's: `$gp+0x4770` (the health's share last tick) and `$gp+0x203c` (the scene played).
    pub const LAST_SHARE: usize = 0x420;
    pub const SCENE_DONE: usize = 0x424;
    /// The port's: its rotor's glow registered this tick (drawn last frame, `+0x31`).
    pub const ROTOR_DRAW: usize = 0x428;
    pub const LEN: usize = 0x430;
}

/// `0x1395b8 + k` (crate::game::GameState::flags): `0x1395c2` its first pass seen, `0x1395c3` it is down.
const F_PASSED: usize = 0x0a;
const F_DOWN: usize = 0x0b;

/// `RC_ENDAKO_BOSS=1` (module docs).
fn dev() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| std::env::var("RC_ENDAKO_BOSS").is_ok_and(|v| v != "0"))
}

fn inside(ctx: &Ctx, z: i32) -> bool { ctx.level.in_cuboid(z, ctx.hero.pos) }
fn point(ctx: &Ctx, path: i32, i: i32) -> Option<[f32; 4]> {
    usize::try_from(path).ok().and_then(|p| ctx.level.paths.get(p)).and_then(|p| p.get(usize::try_from(i).ok()?)).copied()
}
fn path_len(ctx: &Ctx, path: i32) -> i32 { usize::try_from(path).ok().and_then(|p| ctx.level.paths.get(p)).map_or(0, |p| p.len() as i32) }
fn v3(p: [f32; 4]) -> V3 { [p[0], p[1], p[2]] }
fn anim(w: &MobyWorld, id: usize, ctx: &mut Ctx, s: u8, blend: u8) { ctx.host.set_anim(&w.mobys[id], s, 0, blend); }
fn anim_unless(w: &MobyWorld, id: usize, ctx: &mut Ctx, s: u8, blend: u8) { if seq(w, id, ctx) != s { anim(w, id, ctx, s, blend); } }
/// `0x2f02d8`: a local vector through the rows.
fn through(rows: &[V3; 3], v: V3) -> V3 { std::array::from_fn(|k| v[0] * rows[0][k] + v[1] * rows[1][k] + v[2] * rows[2][k]) }

/// Shown, a target, its class's collision (`& 0xffbe | 0x1000`, `+0x98` the class's).
fn show(w: &mut MobyWorld, id: usize) {
    let solid = w.class(CLASS).is_some_and(|c| c.has_collision);
    let m = &mut w.mobys[id];
    m.mode = (m.mode & 0xffbe) | mode::TARGET;
    m.has_collision = solid;
    w.changed = true;
}

/// Hidden, no collision, not a target (`& 0xefff | 0x41`, `+0x98` 0).
fn hide(m: &mut Moby) {
    m.mode = (m.mode & !mode::TARGET) | 0x41;
    m.has_collision = false;
}

/// `0x32bf18(moby, on)`: the moby held (`+0xbe` bit 2) or let go.
fn hold(w: &mut MobyWorld, i: i32, on: bool) {
    let Some(m) = usize::try_from(i).ok().and_then(|i| w.mobys.get_mut(i)) else { return };
    if on { m.once |= 4 } else { m.once &= !4 }
}

fn set_manip(m: &mut Moby, list: u8, q: Option<crate::quat::Q>) {
    m.manips.retain(|x| x.0 != list);
    if let Some(q) = q { m.manips.push((list, q)); }
}

/// `0x325588(0.03, 0.3, who, block, list)`: the aim block's turn springs toward the turn asked for (`0x320570`, the
/// angle spring: 0.03, damping 0.3) and turns joint list `list` of `who` about its z (the original's manipulator node);
/// asked for nothing and back within 0.005, the node goes. Only its z is ever asked for here.
fn aim(w: &mut MobyWorld, holder: usize, b: usize, who: usize, list: u8) {
    let m = &mut w.mobys[holder];
    let t = m.pv_f(b + 0x68);
    let (mut a, mut v) = (m.pv_f(b + 0x48), m.pv_f(b + 0x58));
    if t == 0.0 && a.abs() < 0.005 {
        if let Some(x) = w.mobys.get_mut(who) { set_manip(x, list, None); }
        return;
    }
    turn_spring(t, 0.03, 0.3, 0.0, &mut a, &mut v, 0);
    m.set_pv_f(b + 0x48, a);
    m.set_pv_f(b + 0x58, v);
    m.set_pv_f(b + 0x68, 0.0);
    if let Some(x) = w.mobys.get_mut(who) { set_manip(x, list, Some(crate::quat::axis_angle(a, [0.0, 0.0, 1.0]))); }
}

fn log(w: &MobyWorld, id: usize, what: &str) {
    if dev() {
        let m = &w.mobys[id];
        println!("endako boss: {what} (state {} sub {}, place ({:.1}, {:.1}, {:.1}), health {:.1})", m.state, m.sub_state, m.pos[0], m.pos[1], m.pos[2], m.pv_f(pv::HP));
    }
}

// ---- The boss 0x0c8a ----

/// 0x0c8a (`0x3f5c78`, module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if !senses(w, id, ctx) { return; }
    let before = w.mobys[id].state;
    match before {
        1 => wait_pass(w, id, ctx),
        2 => {
            let k = w.mobys[id].pv_i(pv::PASS);
            if inside(ctx, w.mobys[id].pv_i(pv::PASS_OUT + 4 * k.clamp(0, 1) as usize)) { pass_out(w, id, ctx); }
        }
        3 => {
            let m = &mut w.mobys[id];
            if m.pv_i(pv::AT) == m.pv_i(pv::TO) {
                hide(m);
                if m.pv_i(pv::PASS) == 0 {
                    m.set_state(1, -1);
                    m.set_pv_i(pv::PASS, 1);
                } else {
                    m.set_state(4, -1);
                }
                w.changed = true;
            }
        }
        4 => {
            if tune::SCENE_ON && inside(ctx, w.mobys[id].pv_i(pv::FIGHT_IN)) {
                if w.mobys[id].pv_i(pv::SCENE_DONE) == 0 {
                    // `0x2f5ee8(0x36)` / `0x2f6030`.
                    if ctx.game.scenes.queue.push(0x36) {
                        w.mobys[id].set_pv_i(pv::SCENE_DONE, 1);
                        ctx.game.scenes.start_queue(ctx.hero.hp);
                        w.mobys[id].set_state(5, -1);
                    }
                } else {
                    w.mobys[id].set_state(5, -1);
                }
            }
        }
        5 => {
            if 0x14 < w.mobys[id].state_ticks as u16 && ctx.game.scenes.mode != crate::scene::SCENE { fight(w, id, ctx); }
        }
        6 => {
            if w.mobys[id].first() {
                anim_unless(w, id, ctx, 0, 5);
                let prev = w.mobys[id].prev_state;
                if prev != 4 { next_stop(w, id, ctx, (prev != 8) as i32); }
            }
            let m = &mut w.mobys[id];
            if m.pv_i(pv::AT) == m.pv_i(pv::TO) {
                let mut t = m.pv_i(pv::WAIT);
                let over = countdown(&mut t);
                m.set_pv_i(pv::WAIT, t);
                if over { attack(w, id, ctx); }
            }
        }
        7 => aim_guns(w, id, ctx),
        8 => sweep(w, id, ctx),
        10 => {
            if w.mobys[id].first() {
                // `0x31cda8`: the rockets by rank.
                let (a, b) = match w.mobys[id].pv_s(pv::RANK) { 2 => (3, 5), 3 => (4, 6), _ => (2, 3) };
                let n = ctx.game.rng.rand_range(a, b);
                w.mobys[id].set_pv_i(pv::ROCKETS, n);
                anim_unless(w, id, ctx, 4, 5);
            }
            if wrapped(w, id, ctx) { w.mobys[id].set_state(0xb, -1); }
        }
        0xb => rockets(w, id, ctx),
        0xc => beams(w, id, ctx),
        0xd => {
            if w.mobys[id].first() { anim_unless(w, id, ctx, 8, 5); }
            if wrapped(w, id, ctx) {
                w.mobys[id].set_state(10, 4);
                next_stop(w, id, ctx, 1);
                w.mobys[id].set_pv_f(pv::SPEED, 0.0);
            }
        }
        0xe => dying(w, id, ctx),
        0xf => {
            if fall(w, id, ctx) { return; }
        }
        _ => {}
    }
    if w.mobys[id].state != before { log(w, id, "state"); }
    tail(w, id, ctx);
}

/// `0x3f6bd0`: the set-up, the hits, the target, the voice (module docs); false when the set-up deleted it.
fn senses(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    if w.mobys[id].state == 0 && !setup(w, id, ctx) { return false; }
    let s = w.class(CLASS).map_or(1.0, |c| c.scale) * f32::from_bits(tune::SCALE);
    w.mobys[id].scale = s;
    // `0x31f0f0(m, 0x10000, 0)`; its own laser's hits let go.
    let hit = w.hit_message(id).filter(|h| h.tmpl.flags & 0x10000 != 0);
    let hit = hit.filter(|h| h.tmpl.source.and_then(|s| w.mobys.get(s)).is_none_or(|s| s.o_class != BEAM));
    let wrench = hit.and_then(|h| h.tmpl.source).and_then(|s| w.mobys.get(s)).is_some_and(|s| s.o_class == crate::hero::items::WRENCH_CLASS);
    let dmg = ai::damage_of(w, id, hit.as_ref(), wrench);
    let kill = w.mobys[id].pv_b(pv::KILL);
    if dmg != 0.0 || kill == 2 {
        w.mobys[id].set_pv_b(pv::FLASH + 7, 0x96);
        ctx.host.play_sound(&w.mobys[id], 1, 0);
        ai::flash_start(&mut w.mobys[id], pv::FLASH);
        if w.mobys[id].sub_state == 0 {
            if w.mobys[id].state == 2 { pass_out(w, id, ctx); }
        } else {
            let m = &mut w.mobys[id];
            let mut hp = m.pv_f(pv::HP);
            if kill == 2 { hp = 0.0 } else { approach(0.0, dmg, &mut hp); }
            m.set_pv_f(pv::HP, hp);
            if hp == 0.0 && m.state != 0xe && m.state != 0xf {
                m.set_state(0xe, 1);
                log(w, id, "down");
            } else if 0.0 < m.pv_f(pv::NEXT_HP) && hp < m.pv_f(pv::NEXT_HP) {
                quarter(w, id, ctx);
            }
        }
    }
    if (6..=0xd).contains(&w.mobys[id].state) { ai::find_target(w, id, ctx, 16.0, 8.0, -1, None, false); }
    w.mobys[id].set_pv_b(pv::KILL, 1);
    voice(w, id, ctx);
    w.clear_hit(id);
    true
}

/// State 0 (module docs).
fn setup(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    let m = &mut w.mobys[id];
    if m.pvars.len() < pv::LEN { m.pvars.resize(pv::LEN, 0); }
    m.set_pv_b(0x48, 30);
    m.set_pv_b(0x4a, 32);
    m.set_pv_b(pv::KILL, 1);
    let held = m.pv_i(pv::HOLD_FIGHT);
    hold(w, held, true);
    let mission = w.mobys[id].mission;
    if mission != 0xff && ctx.game.missions.get(ctx.game.level.max(0) as usize * 0x10 + (mission & 0x7f) as usize).is_some_and(|b| b & 0x80 != 0) {
        w.delete(id);
        return false;
    }
    let rank = ctx.game.mission_rank(mission) as i16;
    let m = &mut w.mobys[id];
    m.set_pv_s(pv::RANK, rank);
    let most = m.pv_s(pv::MAX);
    match rank {
        1 => m.set_pv_s(pv::MAX, (most as f32 * 0.7) as i16),
        2 => m.set_pv_s(pv::MAX, (most as f32 * 0.9) as i16),
        _ => {}
    }
    m.set_pv_i(pv::LOOP, -1);
    m.set_pv_f(pv::HP, m.pv_s(pv::MAX) as f32);
    m.bangles = 0x3f;
    m.update_dist = 0xff;
    m.draw_dist = m.draw_dist.wrapping_add(0x40);
    m.set_pv_b(0x28, 3);
    m.set_pv_f(pv::NEXT_HP, m.pv_f(pv::HP) * 0.75);
    let p = m.pos;
    m.set_pv_v(pv::PLACE, p);
    m.set_pv_v(pv::HOME, p);
    hide(m);
    m.set_state(1, 0);
    if seq(w, id, ctx) != 0 { ctx.host.set_anim(&w.mobys[id], 0, 0, 0); }
    let m = &mut w.mobys[id];
    m.set_pv_b(pv::NEXT_BEAM, 1);
    m.set_pv_i(pv::SIDE, 1);
    // The stops: the fight path's points with `w` ≥ 0, seven at most.
    let path = m.pv_i(pv::FIGHT_PATH);
    let pts = usize::try_from(path).ok().and_then(|p| ctx.level.paths.get(p)).cloned().unwrap_or_default();
    let mut k = 0usize;
    if !pts.is_empty() {
        if 0.0 <= pts[0][3] {
            m.set_pv_i(pv::STOPS, 0);
            k = 1;
        }
        for (i, p) in pts.iter().enumerate().skip(1) {
            if 0.0 <= p[3] {
                m.set_pv_i(pv::STOPS + 4 * k, i as i32);
                k += 1;
                if 6 < k { break; }
            }
        }
    }
    let held = m.pv_i(pv::HOLD_END);
    hold(w, held, true);
    // `0x3f7d18`: its crates put away.
    for c in group(w, w.mobys[id].pv_i(pv::CRATES)) { super::crate_::put_away(w, c, ctx); }
    w.changed = true;
    if dev() {
        if let Some((p, r)) = ctx.level.cuboid_place(w.mobys[id].pv_i(pv::FIGHT_IN)) {
            let m = &w.mobys[id];
            let class = |i: i32| usize::try_from(i).ok().and_then(|i| w.mobys.get(i)).map_or(-1, |x| x.o_class as i32);
            let crates: Vec<String> = group(w, m.pv_i(pv::CRATES)).iter().map(|&c| format!("#{c} {:#x}", w.mobys[c].o_class)).collect();
            println!("endako boss: Ratchet put at the fight's cuboid {} ({:.1}, {:.1}, {:.1}) facing it; held #{} {:#x} / #{} {:#x}; crates [{}]",
                m.pv_i(pv::FIGHT_IN), p[0], p[1], p[2], m.pv_i(pv::HOLD_END), class(m.pv_i(pv::HOLD_END)), m.pv_i(pv::HOLD_FIGHT), class(m.pv_i(pv::HOLD_FIGHT)), crates.join(", "));
            let to = sub(m.pos, p);
            ctx.hero.teleport(p, [r[0], r[1], atan2(to[0], to[1])]);
        }
    }
    true
}

fn group(w: &MobyWorld, g: i32) -> Vec<usize> {
    usize::try_from(g).ok().and_then(|g| w.groups.get(g)).cloned().unwrap_or_default()
}

/// A quarter of its health gone (module docs).
fn quarter(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let stage = w.mobys[id].pv_b(pv::STAGE);
    for k in 0..2u8 {
        let m = &w.mobys[id];
        let across = if k == 0 { f32::from_bits(0x3dee_eef0) } else { -f32::from_bits(0x3dee_eef0) };
        let vel = through(&m.rows, [0.0, across, f32::from_bits(0x3daa_aaab)]);
        let piece = stage.wrapping_mul(2).wrapping_add(k);
        let (pos, rot) = (m.pos, m.rot);
        crate::moby::shards::spawn_ball(w, ctx, CLASS, Some(id), pos, rot, 30.0, 15.0, piece, vel, 0xff);
        let m = &mut w.mobys[id];
        m.bangles &= !(1u16.checked_shl(piece as u32).unwrap_or(0));
    }
    let m = &mut w.mobys[id];
    m.set_pv_b(pv::STAGE, stage.wrapping_add(1));
    m.set_state(0xd, 6);
    let most = m.pv_s(pv::MAX) as i32;
    let q = (m.pv_f(pv::HP) * 4.0 / most as f32) as i32;
    m.set_pv_f(pv::NEXT_HP, (q * most) as f32 * 0.25);
    log(w, id, "a quarter off");
}

/// State 1: waiting for its pass.
fn wait_pass(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let k = w.mobys[id].pv_i(pv::PASS).clamp(0, 1) as usize;
    if inside(ctx, w.mobys[id].pv_i(pv::PASS_IN + 4 * k)) {
        w.mobys[id].set_state(2, -1);
        show(w, id);
        let path = w.mobys[id].pv_i(pv::PASS_PATH + 4 * k);
        let start = point(ctx, path, 0).map_or(w.mobys[id].pos, v3);
        let hero = ctx.hero.pos;
        let m = &mut w.mobys[id];
        m.set_pv_i(pv::PATH, path + 1);
        m.set_pv_i(pv::AT, 0);
        m.set_pv_i(pv::TO, 0);
        let place = [start[0], start[1], start[2] - 7.0];
        m.set_pv_v(pv::PLACE, place);
        m.pos = place;
        m.set_pv_v(pv::ROT, [0.0; 3]);
        m.set_pv_f(pv::ROT + 8, atan2(hero[0] - place[0], hero[1] - place[1]));
        m.set_pv_f(pv::SPEED, f32::from_bits(0x3e2a_aaab));
    }
    if inside(ctx, w.mobys[id].pv_i(pv::FIGHT_IN)) { w.mobys[id].set_state(4, -1); }
}

/// The end of a pass: `0x1395c2`, along the path to its end (3).
fn pass_out(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if let Some(b) = ctx.game.flags.get_mut(F_PASSED) { *b = 1; }
    let path = w.mobys[id].pv_i(pv::PATH) - 1;
    let n = path_len(ctx, path);
    let m = &mut w.mobys[id];
    m.set_state(3, -1);
    m.set_pv_i(pv::TO, n - 1);
}

/// State 5's end: the fight (module docs).
fn fight(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].set_state(6, 2);
    show(w, id);
    let path = w.mobys[id].pv_i(pv::FIGHT_PATH);
    let i = w.mobys[id].pv_i(pv::STOPS + 12);
    let at = point(ctx, path, i).map_or(w.mobys[id].pos, v3);
    let hero = ctx.hero.pos;
    let m = &mut w.mobys[id];
    m.set_pv_i(pv::PATH, path + 1);
    m.set_pv_i(pv::TO, i);
    m.set_pv_i(pv::AT, i);
    m.pos = at;
    let yaw = atan2(hero[0] - at[0], hero[1] - at[1]);
    m.set_pv_f(pv::ROT + 8, yaw);
    m.rot[2] = yaw;
    m.set_pv_i(pv::WAIT, 0x3c);
    m.set_pv_f(pv::SPEED, f32::from_bits(0x3e2a_aaab));
    m.set_pv_v(pv::PLACE, [at[0], at[1], at[2] - 7.0]);
    let held = m.pv_i(pv::HOLD_FIGHT);
    let track = m.pv_i(pv::MUSIC);
    hold(w, held, false);
    ctx.host.music_request(track, -1);
}

/// `0x3f7a98(m, kind)`: the next stop — an odd one (1, 3, 5) for kind 1, an even one (0, 2, 4, 6) for 0, any of the
/// seven else; drawn again while the stop's number equals the point it heads for (`+0x2dc`, as the original compares
/// them).
fn next_stop(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, kind: i32) {
    let to = w.mobys[id].pv_i(pv::TO);
    let k = loop {
        let k = match kind {
            1 => ctx.game.rng.randi(3) * 2 + 1,
            0 => ctx.game.rng.randi(4) * 2,
            _ => ctx.game.rng.randi(7),
        };
        if k != to { break k; }
    };
    let m = &mut w.mobys[id];
    m.set_pv_i(pv::TO, m.pv_i(pv::STOPS + 4 * k as usize));
}

/// `0x3f7908`: the attack at a stop (`$gp+0x205c` −1: none forced) — the next asked for (`+0x30a` the hover, `+0x309`
/// the guns, `+0x30b` the laser) or one drawn among the quarters gone + 1: the hover (9, sub 4; its wait 300; the guns
/// after it 1..2 times), the guns (7, sub 3: under two quarters gone one turret at random, else both; the laser next),
/// the laser (0xc, sub 5; its wait 330; the guns next).
fn attack(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    let k = if m.pv_b(pv::NEXT_STAY) != 0 {
        m.set_pv_b(pv::NEXT_STAY, m.pv_b(pv::NEXT_STAY) - 1);
        1
    } else if m.pv_b(pv::NEXT_GUNS) != 0 {
        m.set_pv_b(pv::NEXT_GUNS, m.pv_b(pv::NEXT_GUNS) - 1);
        0
    } else if m.pv_b(pv::NEXT_BEAM) != 0 {
        m.set_pv_b(pv::NEXT_BEAM, m.pv_b(pv::NEXT_BEAM) - 1);
        2
    } else {
        ctx.game.rng.randi(m.pv_b(pv::STAGE) as i32 + 1)
    };
    let turn = f32::from_bits(tune::GUN_TURN);
    match k {
        1 => {
            m.set_pv_i(pv::ATTACK, 300);
            m.set_state(9, 4);
            let n = ctx.game.rng.randi(2) as u8 + 1;
            w.mobys[id].set_pv_b(pv::NEXT_GUNS, n);
        }
        0 => {
            m.set_state(7, 3);
            if m.pv_b(pv::STAGE) < 2 {
                if ctx.game.rng.randi(2) == 0 { m.set_pv_f(pv::GUN_TO + 4, -turn) } else { m.set_pv_f(pv::GUN_TO, turn) }
            } else {
                m.set_pv_f(pv::GUN_TO, turn);
                m.set_pv_f(pv::GUN_TO + 4, -turn);
            }
            m.set_pv_b(pv::NEXT_BEAM, 1);
        }
        2 => {
            m.set_pv_i(pv::ATTACK, tune::BEAM_WAIT);
            m.set_state(0xc, 5);
            m.set_pv_b(pv::NEXT_GUNS, 1);
        }
        _ => {}
    }
}

/// State 7: the turrets turn; there, the sweep's sequence (1: the first alone, 2: the second alone, 3: both) and 8.
fn aim_guns(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    let to = [m.pv_f(pv::GUN_TO), m.pv_f(pv::GUN_TO + 4)];
    for g in 0..2 { m.set_pv_f(pv::AIM_GUN[g] + 0x68, to[g]); }
    let there = (0..2).all(|g| (m.pv_f(pv::AIM_GUN[g] + 0x48) - to[g]).abs() <= 0.01);
    if !there { return; }
    let s = if to[0] == 0.0 || to[1] == 0.0 { if to[0] == 0.0 { 2 } else { 1 } } else { 3 };
    if seq(w, id, ctx) != s { anim(w, id, ctx, s, 5); }
    let m = &mut w.mobys[id];
    m.set_state(8, -1);
    m.set_pv_i(0x2f0, 0);
    m.set_pv_f(pv::SWEEP, to[0]);
    m.set_pv_f(pv::SWEEP + 4, to[1]);
    m.set_pv_i(0x2f4, 0);
}

/// State 8: the sweep and the tracers (module docs).
fn sweep(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let mut through_all = true;
    for g in 0..2 {
        let m = &mut w.mobys[id];
        let to = m.pv_f(pv::GUN_TO + 4 * g);
        if to == 0.0 { continue; }
        let mut s = m.pv_f(pv::SWEEP + 4 * g);
        approach(-to, tune::SWEEP * DEG_S, &mut s);
        m.set_pv_f(pv::SWEEP + 4 * g, s);
        m.set_pv_f(pv::AIM_GUN[g] + 0x68, s);
        if 0.01 < (s + to).abs() { through_all = false; }
        if (ctx.game.frame + 4 * g as i32) & 7 == 0 { fire(w, id, ctx, g); }
    }
    if through_all {
        let m = &mut w.mobys[id];
        m.set_pv_f(pv::GUN_TO, 0.0);
        m.set_pv_f(pv::GUN_TO + 4, 0.0);
        m.set_state(6, 2);
    }
}

/// A tracer from turret `g`'s muzzle (joint list `g + 2`): `0x313b40`'s frame (the pose with the moby's
/// manipulators, so the turret's turn `turn_parts` left last tick), the point (0, ±0.1, ±0.1) in it, its first row at
/// 24 a second; its sound 3.
fn fire(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, g: usize) {
    let r1 = ctx.game.rng.randf_signed(0.0, 0.1);
    let r2 = ctx.game.rng.randf_signed(0.0, 0.1);
    let m = &w.mobys[id];
    let (rows, at) = ctx.host.joint_matrix(m, g + 2).unwrap_or((m.rows, m.pos));
    let from = add(at, through(&rows, [0.0, r1, r2]));
    let vel = set_len(rows[0], tune::TRACER_SPEED / 60.0);
    ctx.host.play_sound(&w.mobys[id], 3, 0);
    let dmg = w.mobys[id].pv_b(pv::DMG_TRACER) as i32;
    if let Some(s) = spawn_tracer(w, id, from, vel, dmg) {
        // `0x3f7f28`: kept for the streaks, thirty at most.
        let m = &mut w.mobys[id];
        let n = m.pv_i(pv::N_TRACERS);
        if n < 0x1e {
            m.set_pv_i(pv::TRACERS + 4 * n as usize, s as i32 + 1);
            m.set_pv_i(pv::N_TRACERS, n + 1);
        }
    }
}

/// State 0xb: the rockets (module docs).
fn rockets(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].first() && seq(w, id, ctx) != 6 { anim(w, id, ctx, 6, 5); }
    if ai::reached(w, id, ctx, 2.0) {
        let m = &w.mobys[id];
        let p = ctx.host.joint(m, 4).unwrap_or(m.pos);
        let v = set_len([m.rows[0][0], m.rows[0][1], 0.0], tune::ROCKET_SPEED / 60.0);
        let dmg = m.pv_b(pv::DMG_ROCKET) as f32;
        ctx.host.play_sound(&w.mobys[id], 4, 0);
        super::keeper::spawn_shot_of(w, ROCKET, id, dmg, dmg, 1.0, p, v, v, (0x1e, 0x50, 1));
    }
    if wrapped(w, id, ctx) {
        let m = &mut w.mobys[id];
        let mut n = m.pv_i(pv::ROCKETS);
        let over = countdown(&mut n);
        m.set_pv_i(pv::ROCKETS, n);
        if over { m.set_state(6, 2); }
    }
}

/// State 0xc: the lasers (module docs).
fn beams(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    let mut t = m.pv_i(pv::ATTACK);
    let over = countdown(&mut t);
    m.set_pv_i(pv::ATTACK, t);
    if over {
        m.set_state(6, 2);
        return;
    }
    let ticks = m.state_ticks as u16;
    if ticks == 0 || ticks % tune::BEAM_EVERY != 0 { return; }
    let side = m.pv_i(pv::SIDE);
    let p = ctx.host.joint(m, if side < 1 { 5 } else { 6 }).unwrap_or(m.pos);
    let m = &mut w.mobys[id];
    let side = -side;
    m.set_pv_i(pv::SIDE, side);
    let n = tune::BEAM_TICKS;
    let speed = tune::BEAM_SPEED[(m.pv_s(pv::RANK) as i32 - 1).clamp(0, 2) as usize];
    let c0 = through(&m.rows, [1.0 / 60.0, speed * side as f32 * (1.0 / 60.0), ((tune::BEAM_Z - p[2]) + (tune::BEAM_Z - p[2])) / n as f32]);
    let mid = add(scale(c0, (n >> 1) as f32), p);
    let mut b0 = sub(m.pv_v(pv::TARGET), mid);
    b0[2] = 0.0;
    let b0 = set_len(b0, speed * (1.0 / 60.0));
    let dmg = m.pv_b(pv::DMG_BEAM) as i32;
    spawn_beam(w, id, p, c0, b0, n, 300, dmg);
}

/// State 0xe (module docs).
fn dying(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].first() {
        anim_unless(w, id, ctx, 8, 5);
    } else {
        anim_unless(w, id, ctx, 0, 5);
    }
    if w.mobys[id].state_ticks as u16 % 0x2d == 0 {
        let g = &mut ctx.game.rng;
        let v = [g.randf(-2.0, 2.0), g.randf(-4.0, 4.0), g.randf(-1.0, 3.0)];
        boom(w, id, ctx, v, true);
    }
    let m = &mut w.mobys[id];
    let mut r = m.pv_f(pv::ROT);
    approach_angle(f32::from_bits(0x3e86_0a93), f32::from_bits(0x3abe_a2f4), &mut r, 0);
    m.set_pv_f(pv::ROT, r);
    m.set_pv_f(pv::ROT + 4, r);
    m.set_pv_i(pv::TO, m.pv_i(pv::AT));
    let ticks = m.state_ticks as u16;
    if ticks < 0xb4 {
        m.set_pv_f(pv::PHASE, add_rot(m.pv_f(pv::PHASE), ticks as f32 * 0.002_424_068_7));
    } else {
        let pos = m.pos;
        let d = set_len(sub(ctx.host.camera_pos(), pos), 1.0);
        super::explosion::start(w, ctx, 0.0, pos, d);
        let m = &mut w.mobys[id];
        m.set_state(0xf, -1);
        m.set_pv_i(pv::PATH, 0);
        m.set_pv_f(pv::SPEED, f32::from_bits(0xbdaa_aaab));
    }
}

/// A blast at `v` in its frame and its sound 2 (`0x403f10(1, 20, at, 0, 0x19, 0x14, 0x14, 0x14, 0x2d, light)`).
fn boom(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, v: V3, light: bool) {
    let m = &w.mobys[id];
    let at = add(through(&m.rows, v), m.pos);
    ctx.host.play_sound(m, 2, 0);
    crate::moby::blast::burst(w, ctx, at, &crate::moby::blast::Blast { size: 1.0, bursts: [0x19, 0x14, 0x14], puffs: 0x14, streaks: 0x2d, light, shake: false });
}

/// State 0xf: the fall and its end (module docs); true once it is gone.
fn fall(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    if w.mobys[id].state_ticks as u16 % 0xf == 0 {
        let g = &mut ctx.game.rng;
        let v = [g.randf(-2.0, 2.0), g.randf(-2.0, 2.0), g.randf(0.0, 3.0)];
        boom(w, id, ctx, v, false);
    }
    let m = &mut w.mobys[id];
    let (mut r, mut p) = (m.pv_f(pv::ROT), m.pv_f(pv::ROT + 4));
    approach_angle(f32::from_bits(0x3f49_0fdc), f32::from_bits(0x3cd6_7752), &mut r, 0);
    approach_angle(f32::from_bits(0x3fc9_0fdc), f32::from_bits(0x3cd6_7752), &mut p, 0);
    m.set_pv_f(pv::ROT, r);
    m.set_pv_f(pv::ROT + 4, p);
    let v = m.pv_f(pv::SPEED) - m.state_ticks as u16 as f32 * 9.259_26e-5;
    m.set_pv_f(pv::SPEED, v);
    let z = m.pv_f(pv::PLACE + 8) + v;
    m.set_pv_f(pv::PLACE + 8, z);
    let home = m.pv_f(pv::HOME + 8);
    if m.pv_s(pv::BOLTS) == 0 && z < home - 1.0 {
        m.set_pv_s(pv::BOLTS, 1);
        crate::moby::bolts::drop(w, id, 0, 3, ctx);
    }
    if home - 12.0 <= z { return false; }
    let m = &w.mobys[id];
    let (tweak, held, track, stinger, pilot, mission, h) =
        (m.pv_i(pv::TWEAK), m.pv_i(pv::HOLD_END), m.pv_i(pv::MUSIC_END), m.pv_i(pv::STINGER_END), ai::link(m, pv::PILOT), m.mission, m.pv_i(pv::LOOP));
    if let Ok(s) = usize::try_from(tweak - 1) { ctx.game.cam_tweaks.release(s); }
    hold(w, held, false);
    ctx.host.music_request(track, stinger);
    if let Some(p) = pilot { w.delete(p); }
    if let Some(b) = ctx.game.flags.get_mut(F_DOWN) { *b = 1; }
    ctx.game.mission_done(mission);
    if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
    w.mobys[id].set_pv_i(pv::LOOP, -1);
    log(w, id, "gone, its mission done");
    w.delete(id);
    true
}

/// `0x3f76c0` (module docs).
fn tail(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    ai::flash(&mut w.mobys[id], pv::FLASH);
    glide(w, id, ctx);
    camera(w, id, ctx);
    turn_parts(w, id);
    crates_back(w, id, ctx);
    pilot(w, id, ctx);
    let m = &w.mobys[id];
    if 2 <= m.sub_state {
        // `0x2fddb0(health / most, 5, 0)`.
        let most = m.pv_s(pv::MAX) as f32;
        ctx.game.boss_bar = Some(((m.pv_f(pv::HP) / most).clamp(0.0, 1.0), 5));
        ctx.game.boss_icon = 5;
    }
    // The tracers' draw (`0x3f7f88`), the list's dead let go first (as the draw does).
    let m = &mut w.mobys[id];
    let mut n = m.pv_i(pv::N_TRACERS).clamp(0, 0x1e);
    let mut k = 0;
    while k < n {
        let t = w.mobys[id].pv_i(pv::TRACERS + 4 * k as usize) - 1;
        let live = usize::try_from(t).ok().and_then(|t| w.mobys.get(t)).is_some_and(|x| x.o_class == TRACER && x.state != 0xfe && x.state != 0xfd);
        if !live {
            // `0x3f7f58`: the last one takes its place.
            n -= 1;
            let m = &mut w.mobys[id];
            let last = m.pv_i(pv::TRACERS + 4 * n as usize);
            m.set_pv_i(pv::TRACERS + 4 * k as usize, last);
        }
        k += 1;
    }
    let m = &mut w.mobys[id];
    m.set_pv_i(pv::N_TRACERS, n);
    // Its draws ([`draw`]): the tracers' with any kept, the rotor's glow drawn last frame.
    let rotor = m.drawn != 0;
    m.set_pv_b(pv::ROTOR_DRAW, rotor as u8);
    if n != 0 || rotor { w.register_draw(id); }
    let m = &mut w.mobys[id];
    if m.state_ticks != -1 { m.state_ticks = m.state_ticks.wrapping_add(1); }
}

/// `0x3f71b0`: along its path, its bob, its facing (module docs: the glide eases its speed toward the distance left
/// at 10 a second² up to 10 a second; the bob by sub — 3: 0.1 and Ratchet's ground − 155 held to 0..1.5, 1: 0.1 / 1.5, 4:
/// 0.3 / 1.5, else 0.3 / 0.3, at 0.5 and 1.5 a second; its phase 100° a second; its yaw along its way at 10° lean in
/// its passes, else toward Ratchet, veered up to 60° toward its way and leaning up to 20° by its speed; the turns
/// eased at 0.1° a tick² up to 12° a tick, not sped up in sub 3).
fn glide(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let path = w.mobys[id].pv_i(pv::PATH) - 1;
    let mut step = [0.0f32; 3];
    if 0 <= path {
        let m = &w.mobys[id];
        let (at, to) = (m.pv_i(pv::AT), m.pv_i(pv::TO));
        let next = if to < at { at - 1 } else { at + (at < to) as i32 };
        let place = m.pv_v(pv::PLACE);
        let p_next = point(ctx, path, next).map_or(place, v3);
        let p_to = point(ctx, path, to).map_or(place, v3);
        let d = sub(p_next, place);
        let dl = len(d);
        let k = tune::GLIDE * (1.0 / 3600.0);
        let mut v = m.pv_f(pv::SPEED);
        ease_speed(dist(place, p_to), k, k, f32::from_bits(0x3e2a_aaab), &mut v);
        step = set_len(d, v);
        let m = &mut w.mobys[id];
        m.set_pv_f(pv::SPEED, v);
        m.set_pv_v(pv::PLACE, add(place, step));
        if dl <= v { m.set_pv_i(pv::AT, next); }
        let h = m.pv_i(pv::LOOP);
        if h == -1 || !ctx.host.sound_playing(h) {
            let h = ctx.host.play_sound(&w.mobys[id], 0, 4);
            w.mobys[id].set_pv_i(pv::LOOP, h);
        }
    }
    let sub_state = w.mobys[id].sub_state;
    let (size, height) = match sub_state {
        3 => (0.1, (ctx.hero.ground_z - 155.0).clamp(0.0, 1.5)),
        1 => (0.1, 1.5),
        4 => (0.3, 1.5),
        _ => (tune::BOB[0], tune::BOB[1]),
    };
    let m = &mut w.mobys[id];
    let (mut a, mut b) = (m.pv_f(pv::BOB), m.pv_f(pv::BOB_Z));
    approach(size, f32::from_bits(0x3c08_8889), &mut a);
    approach(height, f32::from_bits(0x3ccc_ccce), &mut b);
    m.set_pv_f(pv::BOB, a);
    m.set_pv_f(pv::BOB_Z, b);
    let place = m.pv_v(pv::PLACE);
    let phase = m.pv_f(pv::PHASE);
    m.pos = [place[0], place[1], place[2] + a * phase.sin() + b];
    m.set_pv_f(pv::PHASE, add_rot(phase, tune::BOB_SPEED * DEG_S));
    let (lean, yaw) = if sub_state == 0 && 1.0 / 60.0 < len2(step) {
        (0.174_532_95, atan2(step[0], step[1]))
    } else {
        let t = m.pv_v(pv::TARGET);
        let mut yaw = atan2(t[0] - m.pos[0], t[1] - m.pos[1]);
        let f = (m.pv_f(pv::SPEED) * 6.0 - 0.1).clamp(0.0, 1.0);
        if f == 0.0 {
            (0.0, yaw)
        } else {
            if sub_state != 4 {
                let veer = if sub_rot(atan2(step[0], step[1]), m.rot[2]) < 0.0 { -(tune::VEER * DEG) } else { tune::VEER * DEG };
                yaw = add_rot(yaw, veer * f);
            }
            (tune::LEAN * f * DEG, yaw)
        }
    };
    let accel = if sub_state == 3 { 0.0 } else { f32::from_bits(0x3ae4_c38a) };
    let (brake, most) = (f32::from_bits(0x3ae4_c38a), f32::from_bits(0x3e56_7752));
    let (mut p, mut pv_) = (m.pv_f(pv::ROT + 4), m.pv_f(pv::LEAN_V));
    turn_ease(lean, accel, brake, most, &mut p, &mut pv_, 0);
    let (mut y, mut yv) = (m.pv_f(pv::ROT + 8), m.pv_f(pv::YAW_V));
    turn_ease(yaw, accel, brake, most, &mut y, &mut yv, 0);
    m.set_pv_f(pv::ROT + 4, p);
    m.set_pv_f(pv::LEAN_V, pv_);
    m.set_pv_f(pv::ROT + 8, y);
    m.set_pv_f(pv::YAW_V, yv);
    m.rot = m.pv_v(pv::ROT);
}

/// `0x3f7b28`: from sub 2 its camera tweak (priority 5, `0x2d8038`): distance 8, pivot 3 (module docs).
fn camera(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    if m.sub_state < 2 { return; }
    let mut slot = usize::try_from(m.pv_i(pv::TWEAK) - 1).ok();
    if slot.is_none() {
        slot = ctx.game.cam_tweaks.request(5);
        m.set_pv_i(pv::TWEAK, slot.map_or(0, |s| s as i32 + 1));
    }
    if let Some(t) = slot.and_then(|s| ctx.game.cam_tweaks.slots.get_mut(s)) {
        t.dist = tune::CAM[0];
        t.pivot_h = tune::CAM[1];
    }
}

/// `0x3f7c50`: the rotor (joint list 7, −0.416 a tick: `0x2f0b60(angle, q, 2)`) and the turrets' aims (lists 0, 1).
fn turn_parts(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    let a = m.pv_f(pv::ROTOR);
    set_manip(m, 7, Some(crate::quat::axis_angle(a, [0.0, 0.0, 1.0])));
    m.set_pv_f(pv::ROTOR, sub_rot(a, f32::from_bits(0x3ed4_fa0b)));
    for g in 0..2 { aim(w, id, pv::AIM_GUN[g], id, g as u8); }
}

/// `0x3f7d88`: every 64th frame, with any item had below a tenth of its most ammo (`0x139688`, its row's `+0x7e`),
/// each of its crates put away for 61 ticks and more than 3 from Ratchet comes back (`0x3a44e8`, crate::moby::classes::
/// crate_::restore) in a teleport (`0x2ed370`, crate::moby::classes::teleport).
fn crates_back(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if ctx.game.frame & 0x3f != 0 { return; }
    let low = (0..0x38).any(|k| {
        let had = ctx.game.items.get(k).is_some_and(|&b| b != 0);
        let most = ctx.level.items.get(k).map_or(0, |r| r.max_ammo as i16);
        had && most != 0 && (ctx.game.ammo.get(k).copied().unwrap_or(0) as f32) < most as f32 * 0.1
    });
    if !low { return; }
    for c in group(w, w.mobys[id].pv_i(pv::CRATES)) {
        let x = &w.mobys[c];
        if x.state != 6 || x.pv_i(0x100) < 0x3d { continue; }
        if dist(ctx.hero.pos, x.pos) <= 3.0 { continue; }
        let (pos, rot) = (x.pos, x.rot);
        super::crate_::restore(w, c);
        super::teleport::start(w, c, pos, rot, &[]);
    }
}

/// `0x3f7798`: its pilot (made on its first tick, `0x32be48`): at its place and angles; while it is shown, shown and
/// turning its head (list 0) toward Ratchet; hidden with it; its talk (sequence 1) over, back to 0.
fn pilot(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let Some(p) = ai::link(&w.mobys[id], pv::PILOT) else {
        let p = w.create(PILOT);
        let x = &mut w.mobys[p];
        (x.update_dist, x.draw_dist, x.drawn) = (0xff, 0xff, 1);
        x.mode &= 0xfff8;
        ai::set_link(&mut w.mobys[id], pv::PILOT, Some(p));
        w.changed = true;
        return;
    };
    let (pos, rot, hidden) = { let m = &w.mobys[id]; (m.pos, m.rot, m.mode & 1 != 0) };
    let x = &mut w.mobys[p];
    x.pos = pos;
    x.rot = rot;
    x.rows = euler_rows(rot);
    if hidden {
        x.mode |= 1;
        x.drawn = 0;
    } else {
        x.mode &= !1;
        x.drawn = 1;
        let hero = ctx.hero.pos;
        let yaw = sub_rot(atan2(hero[0] - pos[0], hero[1] - pos[1]), rot[2]);
        w.mobys[id].set_pv_f(pv::AIM_PILOT + 0x68, yaw);
        aim(w, id, pv::AIM_PILOT, p, 0);
    }
    w.changed = true;
    if wrapped(w, p, ctx) && seq(w, p, ctx) == 1 { ctx.host.set_anim(&w.mobys[p], 0, 0, 10); }
}

/// `0x3f84c8`: its voice (the dialogue player's lines `+0x3d0 + 4n`) — in its first pass line 0xd once; in the fight
/// line 1 (7 in 10) or 0 while neither was said; from state 6: falling, line 3 once; Ratchet in his state 0x39 on its
/// first tick, line 8 or 9 (two in three); hurt (group 7) on its first tick, one in five line 10 once; its health
/// passing four fifths, three, two and one, one in two lines 2, 5, 6, 4. Its wait (`+0x408`) counts down.
fn voice(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let share = m.pv_f(pv::HP) / m.pv_s(pv::MAX) as f32;
    let state = m.state;
    let said = |w: &MobyWorld, n: usize| w.mobys[id].pv_b(pv::SAID + n) != 0;
    if state == 2 && !said(w, 0xd) { say(w, id, ctx, 0xd); }
    if state == 6 && w.mobys[id].pv_s(pv::SAID) == 0 {
        let n = if 0.3 < ctx.game.rng.randf(0.0, 1.0) { 1 } else { 0 };
        say(w, id, ctx, n);
    }
    let mut last = w.mobys[id].pv_f(pv::LAST_SHARE);
    if 5 < state {
        if state == 0xf && !said(w, 3) { say(w, id, ctx, 3); }
        if ctx.hero.state == 0x39 && ctx.hero.timer == 0 {
            match ctx.game.rng.randi(3) {
                0 => say(w, id, ctx, 8),
                1 => say(w, id, ctx, 9),
                _ => {}
            }
        }
        if ctx.hero.group == 7 && ctx.hero.timer == 0 && ctx.game.rng.randi(5) == 0 && !said(w, 10) { say(w, id, ctx, 10); }
        let rng = &mut ctx.game.rng;
        let mut pass = |at: f32| share < at && at <= last && rng.randi(2) == 0;
        let n = if pass(0.8) { Some(2) } else if pass(0.6) { Some(5) } else if pass(0.4) { Some(6) } else if pass(0.2) { Some(4) } else { None };
        if let Some(n) = n { say(w, id, ctx, n); }
        last = share;
    }
    let m = &mut w.mobys[id];
    m.set_pv_f(pv::LAST_SHARE, last);
    let mut t = m.pv_i(pv::LINE_WAIT);
    countdown(&mut t);
    m.set_pv_i(pv::LINE_WAIT, t);
}

/// `0x3f8820(m, n)`: line `n` (not the last one said) asked for when the dialogue is free — its id's low 14 bits +
/// 2000 (crate::audio::speech); marked said, the wait 600; the pilot talks (its sequence 1, the port's start of the
/// line: module docs).
fn say(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, n: usize) {
    let m = &w.mobys[id];
    if m.pv_i(pv::LAST_LINE) == n as i32 { return; }
    let line = (m.pv_i(pv::LINES + 4 * n) & 0x3fff) + 2000;
    if !ctx.host.request_speech(line) { return; }
    let m = &mut w.mobys[id];
    m.set_pv_b(pv::SAID + n, 1);
    m.set_pv_i(pv::LAST_LINE, n as i32);
    m.set_pv_i(pv::LINE_WAIT, 600);
    if let Some(p) = ai::link(&w.mobys[id], pv::PILOT) { ctx.host.set_anim(&w.mobys[p], 1, 0, 10); }
}

/// The boss's draw callbacks, as its tail registers them: its tracers' streaks ([`draw_tracers`], `0x3f7f88`), then its
/// rotor's glow ([`draw_rotor`], `0x3f81e8`).
pub fn draw(w: &MobyWorld, host: &dyn crate::moby::MobyHost, id: usize, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    draw_tracers(w, id, cam, out);
    if w.mobys[id].pv_b(pv::ROTOR_DRAW) != 0 { draw_rotor(w, host, id, out); }
}

/// The rotor's glow (`$gp+0x20d8..+0x21a0` on Endako): the short blades', then the long blades'.
mod rotor {
    use crate::hero::physics::V3;
    /// `+0x20d8` / `+0x20dc`: the FX textures (ST `+0x20e8`: the middle, 0.5, at every corner; additive).
    pub const FX: [usize; 2] = [5, 5];
    /// `+0x2108` / `+0x210c`, `+0x2110` / `+0x2114`: the colours at the hub and at the tip.
    pub const RGBA: [[u32; 2]; 2] = [[0x6020_3040, 0x3020_3040], [0x6020_3040, 0x3020_3040]];
    /// `+0x2120` / `+0x2160`: a blade's hub and tip in the rotor's frame (joint list 7): 3 along x, 7 along y, 1.2 up.
    const UP: f32 = f32::from_bits(0x3f99_999a);
    pub const BLADE: [[V3; 2]; 2] = [[[0.0, 0.0, UP], [3.0, 0.0, UP]], [[0.0, 0.0, UP], [0.0, 7.0, UP]]];
    /// `+0x21a0`: the sweep's fade (an eighth of the alpha a step).
    pub const FADE: f32 = 8.0;
    /// The sweep's step (π/32, 0x3dc90fdb): each edge turns two of them at a time.
    pub const STEP: f32 = f32::from_bits(0x3dc9_0fdb);
}

/// `0x3f81e8`: the blur behind each of its four blades (the short pair along the rotor's x, the long along its y; each
/// pair both ways) in the rotor's frame (joint list 7, `0x313b40`): a quad between two edges from the hub to the tip,
/// the short blades' second edge π/32 on; thirty-two times the two edges take turns stepping π/16 on (the first edge
/// first), their alpha down an eighth, and each blade's quad is drawn (FX 5, flat, additive), so the four sweep half a
/// turn each, fading.
pub fn draw_rotor(w: &MobyWorld, host: &dyn crate::moby::MobyHost, id: usize, out: &mut Vec<crate::fx::FxQuad>) {
    use crate::hud::lerp_rgba;
    let m = &w.mobys[id];
    let Some((rows, at)) = host.joint_matrix(m, 7) else { return };
    let r = euler_rows([0.0, 0.0, rotor::STEP]);
    let r2: [V3; 3] = std::array::from_fn(|i| through(&r, r[i]));
    let mut quads: [([V3; 4], [u32; 4], usize); 4] = std::array::from_fn(|k| {
        let (b, s) = (k / 2, if k % 2 == 1 { -1.0 } else { 1.0 });
        let [hub, tip] = rotor::BLADE[b].map(|v| [v[0] * s, v[1] * s, v[2]]);
        let [ch, ct] = rotor::RGBA[b];
        ([hub, tip, hub, tip], [ch, ct, ch, ct], rotor::FX[b])
    });
    for q in &mut quads[..2] {
        q.0[2] = through(&r, q.0[2]);
        q.0[3] = through(&r, q.0[3]);
    }
    let t = 1.0 / rotor::FADE;
    for i in 0..32 {
        let e = if i & 1 == 0 { 0 } else { 2 };
        for (v, c, fx) in &mut quads {
            for j in e..e + 2 {
                v[j] = through(&r2, v[j]);
                c[j] = lerp_rgba(t, c[j], c[j] & 0xff_ffff);
            }
            let corners = (*v).map(|p| add(through(&rows, p), at));
            out.push(crate::fx::FxQuad { fx: *fx, additive: true, corners, st: [[0.5, 0.5]; 4], rgba: *c });
        }
    }
}

/// `0x3f7f88`: each kept tracer's streak — a quad of FX 5 (additive) from 0.1 to either side of it back along its
/// velocity by 24/60 a tick of its trail to 0.05 to either side, across the view (`(camera − it) × velocity`); red
/// (0x600000ff) at its head fading to nothing (0x0000ff80) at its tail.
pub fn draw_tracers(w: &MobyWorld, id: usize, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let m = &w.mobys[id];
    let n = m.pv_i(pv::N_TRACERS).clamp(0, 0x1e) as usize;
    for k in 0..n {
        let Some(t) = usize::try_from(m.pv_i(pv::TRACERS + 4 * k) - 1).ok().and_then(|t| w.mobys.get(t)) else { continue };
        let p = t.pos;
        let back = set_len(t.pv_v(0), t.pv_s(0x12) as f32 * (tune::STREAK / 60.0));
        // `0x2eff28(out, a, b)` = b × a.
        let side = unit(cross(sub(cam, p), back));
        let r0 = cross(side, back);
        let c = |x: f32, y: f32, z: f32| add(add(add(scale(r0, x), scale(back, y)), scale(side, z)), p);
        out.push(crate::fx::FxQuad {
            fx: 5,
            additive: true,
            corners: [c(0.0, 0.0, 0.1), c(0.0, 0.0, -0.1), c(0.0, -1.0, 0.05), c(0.0, -1.0, -0.05)],
            st: [[0.5, 0.0], [0.5, 1.0], [1.0, 0.0], [1.0, 1.0]],
            rgba: [0x6000_00ff, 0x6000_00ff, 0x0000_ff80, 0x0000_ff80],
        });
    }
}

// ---- The tracer 0x0a72 ----

/// `0x3dae28(owner, pos, vel, damage)`: a tracer at `pos` with the owner's angles and light (`0x32be48`), facing
/// `vel`; 90 ticks, its damage, scale twice its class's. pvars: `+0x00` velocity, `+0x10` ticks left (16 bits), `+0x12`
/// its trail (16 bits, to 12), `+0x14` the owner + 1, `+0x18` the damage.
pub fn spawn_tracer(w: &mut MobyWorld, owner: usize, pos: V3, vel: V3, damage: i32) -> Option<usize> {
    let scale = w.class(TRACER).map_or(1.0, |c| c.scale) * 2.0;
    let s = w.create(TRACER);
    let (rot, light) = (w.mobys[owner].rot, w.mobys[owner].light);
    let x = &mut w.mobys[s];
    (x.update_dist, x.draw_dist, x.drawn) = (0xff, 0xff, 1);
    x.parent = Some(owner);
    x.light = light;
    x.mode &= 0xfff8;
    x.pos = pos;
    x.rot = [rot[0], -atan2(len2(vel), vel[2]), atan2(vel[0], vel[1])];
    x.set_pv_v(0, vel);
    x.set_pv_i(0x18, damage);
    x.set_pv_s(0x10, 0x5a);
    ai::set_link(x, 0x14, Some(owner));
    x.set_pv_s(0x12, 0);
    x.scale = scale;
    w.changed = true;
    Some(s)
}

/// 0x0a72 (`0x3dab80`): in flight (0) its trail grows (to 12), the line of its step (its owner ignored) hits for its
/// damage (flags 0x10001): hit, it hides (1) with a hit's puffs and sparks (`0x3edaf8`, crate::moby::classes::shot::
/// impact); its time out, it hides; else five grey sprites along the step (`0x340508`: 0.1..0.3 of 210000 → 42000,
/// alpha 0x20 → 0, 15 ticks, jittered 0.1). Hidden, its trail shrinks a tick at a time and it goes.
pub fn update_tracer(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            let (pos, vel) = (m.pos, m.pv_v(0));
            let next = add(pos, vel);
            if m.pv_s(0x12) < 0xc { m.set_pv_s(0x12, m.pv_s(0x12) + 1); }
            let owner = ai::link(m, 0x14);
            let t = crate::moby::hits::HitTemplate::new(m.pv_i(0x18) as f32, Some(id), 0x1_0001, [0.0; 4]);
            match w.coll_line(ctx, pos, next, 0, owner) {
                Some(o) => {
                    if let Some(t_id) = o.moby { w.deliver_hit_any(t_id, &t, [o.point[0], o.point[1], o.point[2], 1.0]); }
                    let m = &mut w.mobys[id];
                    hide(m);
                    m.set_state(1, -1);
                    let cam = ctx.host.camera_pos();
                    if let Some(sys) = ctx.parts.as_deref_mut() { super::shot::impact(sys, &mut ctx.game.rng, o.point, cam, true); }
                }
                None => {
                    let m = &mut w.mobys[id];
                    let mut life = m.pv_s(0x10) as i32;
                    let over = countdown(&mut life);
                    m.set_pv_s(0x10, life as i16);
                    if over {
                        hide(m);
                        m.set_state(1, -1);
                    } else {
                        let step = scale(vel, 0.2);
                        let mut at = pos;
                        for _ in 0..5 {
                            let g = &mut ctx.game.rng;
                            let r = g.randf(0.1, 0.3);
                            let spin = g.randi(0xff) as u8;
                            let p = add(g.rand_vec(0.0, 0.1), at);
                            if let Some(sys) = ctx.parts.as_deref_mut() {
                                crate::particles::sprite(sys, &mut ctx.game.rng, r * 210_000.0, r * 42_000.0, 0.0, p, [0.0; 3], 0xf, 0x80_8080, spin, 0x20, 0, 0);
                            }
                            at = add(at, step);
                        }
                    }
                }
            }
            w.mobys[id].pos = next;
        }
        1 => {
            let m = &mut w.mobys[id];
            let n = m.pv_s(0x12) - 1;
            m.set_pv_s(0x12, n);
            if n == -1 { w.delete(id); }
        }
        _ => {}
    }
}

// ---- The laser 0x09df ----

/// `0x3d1fd8(owner, carrier, pos, v0, v1, n, life, damage)`: a laser at `pos` (update and draw distance 0xff), its
/// velocity easing from `v0` to `v1` over `n` ticks, `life` ticks of beam. pvars: `+0x00` / `+0x10` the two
/// velocities, `+0x20` the ease (0..1), `+0x24` 1/n, `+0x28` the life, `+0x34` its brightness, `+0x38` the damage,
/// `+0x3c` its loop, `+0x40` a carrier (none from the boss).
#[allow(clippy::too_many_arguments)]
pub fn spawn_beam(w: &mut MobyWorld, owner: usize, pos: V3, v0: V3, v1: V3, n: i32, life: i32, damage: i32) -> Option<usize> {
    let s = w.create(BEAM);
    let x = &mut w.mobys[s];
    x.parent = Some(owner);
    (x.update_dist, x.draw_dist) = (0xff, 0xff);
    x.rows = euler_rows(x.rot);
    x.pos = pos;
    x.set_pv_v(0, v0);
    x.set_pv_v(0x10, v1);
    x.set_pv_i(0x28, life);
    x.set_pv_i(0x38, damage);
    x.set_pv_f(0x20, 0.0);
    x.set_pv_f(0x34, 0.0);
    x.set_pv_f(0x24, 1.0 / n as f32);
    x.set_pv_i(0x40, 0);
    w.changed = true;
    Some(s)
}

/// 0x09df (`0x3d1ae8`): gone in a scene (`0x1a8f00` = 2). It moves by its velocity (`0x2efe58`: `v0` → `v1` by the
/// ease), the ease toward 1 by 1/n. 0: past half the ease it beams (1). 1: a line from it straight down 5 × the ease
/// hits for its damage (flags 0x10001, itself ignored); its sparks' places (missing: one in two times down the beam,
/// 0..5; meeting: at the hit, then one in two down the beam); its loop sound 0; its brightness up to 1 at 1/15; its
/// life out, 2. 2: its brightness down at 1/30; out, its loop stopped and it goes.
pub fn update_beam(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if ctx.game.scenes.mode == crate::scene::SCENE {
        w.delete(id);
        return;
    }
    let m = &mut w.mobys[id];
    let t = m.pv_f(0x20);
    let (a, b) = (m.pv_v(0), m.pv_v(0x10));
    let vel = add(a, scale(sub(b, a), t));
    m.pos = add(m.pos, vel);
    let mut t = t;
    approach(1.0, m.pv_f(0x24), &mut t);
    m.set_pv_f(0x20, t);
    match m.state {
        0 => {
            m.set_pv_i(0x3c, -1);
            if 0.5 < t { m.set_state(1, -1); }
        }
        1 => {
            let pos = m.pos;
            let end = [pos[0], pos[1], pos[2] - t * 5.0];
            let tmpl = crate::moby::hits::HitTemplate::new(m.pv_i(0x38) as f32, Some(id), 0x1_0001, [0.0; 4]);
            let hit = w.coll_line(ctx, pos, end, 0, Some(id));
            if let Some(o) = hit {
                if let Some(t_id) = o.moby { w.deliver_hit_any(t_id, &tmpl, [o.point[0], o.point[1], o.point[2], 1.0]); }
                // Sparks at the hit (`0x1bfde0`, the query's point) thrown out flat a random way at up to 0.5 a
                // second and up at 1..2 a second.
                let g = &mut ctx.game.rng;
                let a = g.rand_angle();
                let r = g.randf(0.0, 0.5) * SIXTIETH;
                let up = g.randf(1.0, 2.0) * SIXTIETH;
                sparks(ctx, [o.point[0], o.point[1], o.point[2]], [a.cos() * r, a.sin() * r, up]);
            }
            // One time in two, sparks somewhere down the first 5 under it, drifting a random way up to 2 a second.
            let g = &mut ctx.game.rng;
            if g.randi(2) == 0 {
                let at = [pos[0], pos[1], pos[2] - g.randf(0.0, 5.0)];
                let v = g.rand_vec(0.0, f32::from_bits(0x3d08_8889));
                sparks(ctx, at, v);
            }
            let h = w.mobys[id].pv_i(0x3c);
            if h == -1 || !ctx.host.sound_playing(h) {
                let h = ctx.host.play_sound(&w.mobys[id], 0, 4);
                w.mobys[id].set_pv_i(0x3c, h);
            }
            let m = &mut w.mobys[id];
            let mut bright = m.pv_f(0x34);
            if bright < 1.0 { approach(1.0, f32::from_bits(0x3d88_8889), &mut bright); }
            m.set_pv_f(0x34, bright);
            let mut life = m.pv_i(0x28);
            let over = countdown(&mut life);
            m.set_pv_i(0x28, life);
            if over { m.set_state(2, -1); }
        }
        2 => {
            let mut bright = m.pv_f(0x34);
            approach(0.0, f32::from_bits(0x3d08_8889), &mut bright);
            m.set_pv_f(0x34, bright);
            if bright <= 0.0 {
                let h = m.pv_i(0x3c);
                if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
                w.mobys[id].set_pv_i(0x3c, -1);
                w.delete(id);
                return;
            }
        }
        _ => {}
    }
    // The strips' scroll `+0x2c` and pulse `+0x30` (the scroll as the original steps it: above 1 less 1, else below 1
    // plus 1, so it takes turns between 0..1 and 1..2, the same on the texture); its draw.
    let m = &mut w.mobys[id];
    let mut s = m.pv_f(0x2c) + beam::RATES[0] * SIXTIETH;
    if 1.0 < s {
        s -= 1.0;
    } else if s < 1.0 {
        s += 1.0;
    }
    m.set_pv_f(0x2c, s);
    let mut t = m.pv_f(0x30) + beam::RATES[1] * SIXTIETH;
    if 1.0 < t { t -= 1.0; }
    m.set_pv_f(0x30, t);
    w.register_draw(id);
}

/// 1/60 (0x3c888889).
const SIXTIETH: f32 = f32::from_bits(0x3c88_8889);

/// The laser's look (`$gp+0x1560..+0x15b4` on Endako).
mod beam {
    /// `+0x1560` / `+0x1564`: the strips' scroll along and their pulse (a second).
    pub const RATES: [f32; 2] = [-2.0, 2.0];
    /// `+0x1580..+0x159c`: the strips' corners' ST before the scroll.
    pub const ST: [[f32; 2]; 4] = [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]];
    /// `+0x15a0` / `+0x15a4` / `+0x15a8`: the strips' segments, a segment's length, their widest half-width.
    pub const SEGS: i32 = 6;
    pub const SEG: f32 = 1.0;
    pub const HALF: f32 = f32::from_bits(0x3f19_999a);
    /// `+0x15b0` / `+0x15b4`: the strips' colours by their pulse, narrowest to widest.
    pub const RGBA: [u32; 2] = [0xff80_4040, 0x0040_2020];
    /// The strips' FX texture (both additive).
    pub const FX: usize = 0x2f;
    /// The four glows: size, FX texture, the colours a random draw picks between.
    pub const GLOWS: [(f32, usize, u32, u32); 4] = [
        (1.0, 0x31, 0x5010_3080, 0x6010_3080),
        (f32::from_bits(0x3f19_999a), 0x31, 0x50ff_ffff, 0x60ff_ffff),
        (f32::from_bits(0x3fb3_3333), 0xb, 0x0880_3010, 0x1880_3010),
        (f32::from_bits(0x3f66_6666), 0xb, 0x2080_3010, 0x4080_3010),
    ];
}

/// `0x3d20e8` (registered every tick by [`update_beam`]): four glows at it (`0x2eef08`, crate::fx::sprite: each its
/// colour a random way between its two and a random roll; additive), then two strips of FX 0x2f (additive) straight
/// down from it in six segments of 1, across the view on the flat (`(0, 0, 1) × the flat way to the camera`): each
/// strip ±0.6 × its pulse wide (the second half a turn behind the first), its colour by its pulse (its alpha × the
/// brightness `+0x34` while under 1), fading out down the segments; s along a segment from the scroll (+0.5 for the
/// second strip), t across.
pub fn draw_beam(w: &MobyWorld, id: usize, rng: &mut crate::rng::Rng, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    use crate::hud::lerp_rgba;
    let m = &w.mobys[id];
    let p = m.pos;
    for (size, fx, c0, c1) in beam::GLOWS {
        let c = lerp_rgba(rng.randf(0.0, 1.0), c0, c1);
        let roll = rng.rand_angle();
        crate::fx::sprite(out, size, 0.0, roll, p, cam, fx, c, true);
    }
    // `0x2eff28(side, flat, up)` = up × flat.
    let mut flat = sub(cam, p);
    flat[2] = 0.0;
    let side = cross([0.0, 0.0, 1.0], unit(flat));
    let pulse0 = m.pv_f(0x30);
    let mut pulse1 = pulse0 + 0.5;
    if 1.0 < pulse1 { pulse1 -= 1.0; }
    let pulses = [pulse0, pulse1];
    let mut cols = pulses.map(|t| lerp_rgba(t, beam::RGBA[0], beam::RGBA[1]));
    let bright = m.pv_f(0x34);
    if bright < 1.0 { cols = cols.map(|c| lerp_rgba(bright, c & 0xff_ffff, c)); }
    let scroll = m.pv_f(0x2c);
    let st: [[[f32; 2]; 4]; 2] = std::array::from_fn(|k| beam::ST.map(|[s, t]| [s + (scroll + k as f32 * 0.5), t]));
    let step = 1.0 / beam::SEGS as f32;
    let (mut top, mut z0, mut z1) = (1.0f32, 0.0f32, 0.0f32);
    for _ in 0..beam::SEGS {
        let bottom = top - step;
        z1 -= beam::SEG;
        for k in 0..2 {
            let half = beam::HALF * pulses[k];
            let c = |x: f32, z: f32| add(add(scale(side, x), [0.0, 0.0, z]), p);
            let (a, b) = (lerp_rgba(top, cols[k] & 0xff_ffff, cols[k]), lerp_rgba(bottom, cols[k] & 0xff_ffff, cols[k]));
            out.push(crate::fx::FxQuad { fx: beam::FX, additive: true, corners: [c(half, z0), c(-half, z0), c(half, z1), c(-half, z1)], st: st[k], rgba: [a, a, b, b] });
        }
        z0 -= beam::SEG;
        top = bottom;
    }
}

/// `0x3d25d0(m, pos, vel)`: two twinkles (`0x33bc30`, crate::particles::twinkle) 0.1 about `pos`, moving `vel` a tick
/// and falling `randf(3, 7)` / 3600 a tick, by a size `k` (`randf(0.25, 1)`): blue (0x7f800000) from k to 0.2·k over 40
/// ticks, then grey (0x7f7f7f7f) from 0.7·k to 0.14·k over 60, set at 0x20; turning one way and the other by
/// `randi(2)`.
fn sparks(ctx: &mut Ctx, pos: V3, vel: V3) {
    let g = &mut ctx.game.rng;
    let k = g.randf(0.25, 1.0);
    let spin = g.randi(2) as i8;
    let fall = g.randf(3.0, 7.0) * f32::from_bits(0x3991_a2b4);
    let at = add(g.rand_vec(0.0, f32::from_bits(0x3dcc_cccd)), pos);
    if let Some(sys) = ctx.parts.as_deref_mut() {
        let rng = &mut ctx.game.rng;
        crate::particles::twinkle(sys, rng, k * f32::from_bits(0x3e4c_cccd), k, fall, at, 0x28, 0x7f80_0000, 0, spin, vel, 0);
        crate::particles::twinkle(sys, rng, k * f32::from_bits(0x3e0f_5c29), k * f32::from_bits(0x3f33_3333), fall, at, 0x3c, 0x7f7f_7f7f, 1, -spin, vel, 0);
    }
}
