//! Endako (level 3)'s classes, set D: the city's breakables and its robots (L03 addresses; the engine functions they
//! call are named by their L03 addresses with the L00 ones the port knows them by where they differ).
//!
//! **The breakables.** Each waits in state 1 for a hit with the flags 0x4010000 (`0x31f0f0`, L00 `0x31dce8`) while
//! `+0x68`[0] = 0 and breaks in state 2: its pieces thrown (`0x32cd30`, L00 `0x32b580`, crate::moby::shards::
//! throw_all, from the `$gp` tables below: every one's skip piece `$gp+0x2a90` / `+0x2ad0` / `+0x2b10` is −1, so the
//! moby is deleted rather than left showing it), its class sound 0.
//! * **0x0f37** ([`update_0f37`], `0x41b360`, 50 placed): a lamp: two lasting glows 20 × its scale along its up row
//!   (crate::particles::glow, `0x340ae0` = L00 `0x33f550`); its break gives the level's bolts (`0x327b28` = L00
//!   `0x327388`, crate::moby::bolts::level_drop, 3 up), a blast of size 0 (`0x403f10`) and the general explosion's 15
//!   embers (`0x3219e0` = L00 `0x320538`) at the lamp's head, and six debris chunks out of each of its four corners
//!   (`0x328028` = L00 `0x327648`, crate::moby::shards::throw_chunks).
//! * **0x0f3e** ([`update_0f3e`], `0x41bc30`, 49 placed): a hanging lamp with a glow 8 × its scale under it; its break
//!   leaves its stub (0x0f40, `0x3285c0` = L00 `0x327898`) and a blast of size 0.75, and the lamp, hidden, pours smoke
//!   for 30 ticks ([`crate::particles::staged_box`], `0x2edd68`: 40 puffs, then 3 a tick out of a box that drifts
//!   aside) before it is deleted.
//! * **0x0f38** ([`update_0f38`], `0x41b750`, 13 placed): a box between two beams; its break gives a blast of size
//!   0.75, two debris chunks at each side and 15 puffs of smoke a side ([`crate::particles::staged_box`]).
//!
//! The breakables' count (`$gp−0x440`, `0x1aebb0`): each one with `+0x68`[0] ≠ 0 takes one off at its set-up, and
//! each break takes one off; no code the port has (nor any absolute read in the overlay) reads it, so it is not kept.
//!
//! Not ported (the breakables): 0x0f38's two beams (class 0x13ba, made by `0x42cdd0` while `+0x68`[4] = 0: from
//! (±12, −10, 20) to (±12, −25, 20) in its frame, colour 0x50285050, widths 0.5 / 1.5, kind 5, `+0x68`[8] ≠ 0 its
//! flag; its update `0x42c920` has no function in Ghidra and it draws itself); 0x0f3e keeps its glow's handle at
//! `+0x68`[4], which nothing reads.
//!
//! **The robots** 0x093c / 0x0940 / 0x0964 / 0x0a59 ([`update_robot`], `0x3c31f8`; mode 0x20, on crate::moby::ai's
//! framework, the movement block at `+0x110`): Clank's helpers. Each sleeps in a cage, its child, until Clank breaks
//! it, then follows him through the network of floor polygons of **0x0f2d** ([`update_0f2d`], `0x41b0b0`: built at
//! its set-up from its list of paths, `0x32b318`, crate::path_network::build_list; the openings between its polygons
//! no walls for the fences afterwards).
//! * **Set-up** (`0x3c4828`, state 0): mirrored at random (mode 0x8000 by `randi(2)`); on the floor under it
//!   (`0x31e228`, kept at `+0x3a4`), 0.089 up (0x0940 also 0.2 back); its child made at its place (`0x312738` +
//!   `0x32be48`: drawn, draw distance 0x30, its light and ambient; 0x0964's is 0x0972, the others' the cage **0x0d75**
//!   ([`update_cage`], `0x4096c8`) with the pieces `$gp+0xed0` (0x3fff) over it, 0x093c's 1.2 times its class's
//!   size); its movement block (radius 0.39 for 0x0940, 0.42 for 0x093c / 0x0a59, 0.15 for 0x0964; height 0.3, steps
//!   0.5, slope 1.588, gravity 1/720, the turn before walking 30°, stop 1.5); sequence 0 at a random frame; 1 (sub 0).
//!   0x0940 closes the records holding the unseen floors 0x097a of its eight groups `+0x200` (`0x41b120`); 0x0a59
//!   plays sequence 6.
//! * **Every tick** (`0x3c4828`): its shadow's slab probed within 24 of the camera (range 0x16); a hit with flag 1
//!   (`0x31f0f0`), or 10 below its start with the floor within 10 under it, and no robot's blast in the last 45 ticks
//!   (`$gp+0xec8` / `+0xecc`, crate::game::GameState::robot_blast_frame): a blast at 0.5 above it (`0x403f10`: size
//!   0.25, sprites 5 / 5 / 5, 10 puffs, 0x14 streaks, its light), 0x0964 breaks (`0x327e78`: pieces 0..3), its
//!   velocity stopped, 4 (sub 1), and it is beamed back to its child (`0x3c6c70` → `0x2ed370`, crate::moby::classes::
//!   teleport::start); the hit record let go.
//! * Sub-state 1, its first tick: the walk (`0x32f550`: turn accel 0.2°, at most 6°; walk accel 6/3600, most 3/60),
//!   accel `+0x24` 0.0017, stop 1.5.
//! * **1**: asleep until its child is hooked (`0x32bf38`): the help for its kind once (0x0940: 0x9cf in slot 0x26,
//!   0x093c: 0x9d1 / 0x28, 0x0a59: 0xa08 / 0x5f, while their counts are 0; 0x0964: 0x9d2 / 0x29 while never shown);
//!   4 (sub 1).
//! * **4**, beside Clank: its first tick sequence 0 (blend 15), the idles' count 2, still; Clank pressing on it
//!   (`0x18a090` / `0x18a08c`) pushes it ([`push`], `0x3c5970`) and it backs off ([`back_off`], `0x3c5848`); farther
//!   than 3 across from him with a way through the network: 3, sequence 2. At a sequence's end: from its idle 1 back
//!   to 0, else one time in its idles' count the idle 1 (the count a step toward 10, `0x31ff78`).
//! * **3**, following: toward Clank through the network ([`move_to`], `0x3c57c0`) fanned out by its place among its
//!   group's robots on the way (`0x3c6ef8`, every 16th frame; `0x3c70c0`); no way, within 1.5, or stuck over 60 ticks
//!   within 3: 4, sequence 0.
//! * **The tail** (`0x3c31f8`, `0x3c5a70`): in the air or on a moby, upright and held (`0x32e698`); its state's ticks
//!   counted; drawn, its glow `+0x78` pulses between its sub-state's colours (`0x3c5b80`, `$gp+0xee0..`); its fence
//!   path the record it is in (`0x41b218`).
//!
//! Not ported (the robots): Clank's commands (`0x18c304`, its D-pad orders 1..8: the hero side has none) and all
//! they lead to — the states 2 and 5..0x15 (going to and using the unseen floors 0x097a, the pieces 0x0e87 they
//! raise, 0x0b10 / 0x0f4d, `0x3c6348` / `0x3c6560` / `0x3c6730` / `0x3c68d8` / `0x3c6a90` and their helps 0x9d0 /
//! 0xa29 / 0xa0a), the command list `0x32c138` and timer `+0x364`, the sub-states 2..8's limits; the fall of state
//! 0x16 (nothing the port has sets it); the head's look at Clank and the joint aims (`0x3c61e0` / `0x3c6018`,
//! `0x325588`: crate::moby::ai's joint aims are not ported); the colour flash `0x321690`; the counts of robots near
//! Clank (`0x1b275c..0x1b2764`); the glow's group stamp and draw hook (`0x2e8310(0x3c5e08)`); `0x3c6c70`'s search for a
//! nearer 0x0972 of its groups to beam back to (`0x2e0558`); the push on a moving floor moby (`0x3234d0`); the light
//! `0x31bcac`. 0x0964's child is the cage 0x0972 ([`update_0972`]).

use crate::hero::physics::{add, add_rot, atan2, set_len, V3};
use crate::moby::blast::{self, Blast, Explosion};
use crate::moby::{bolts, countdown, shards, Ctx, MobyWorld};

/// The hit flags the breakables answer (`0x31f0f0(m, 0x4010000, 0)`).
const BREAK_FLAGS: u32 = 0x0401_0000;
/// The debris chunks' colour (`0x328028`'s `0x50404040`).
const CHUNK_COLOUR: u32 = 0x5040_4040;

/// `0x3265b8(x, y, z, m, out)`: a point in the moby's frame (through its rows, unscaled) plus its place.
fn frame_point(w: &MobyWorld, id: usize, v: V3) -> V3 {
    let m = &w.mobys[id];
    add(through(&m.rows, v), m.pos)
}

/// `0x2f02d8(out, v, m + 0xc0)` (L00 `0x2ef318`): `v` through the moby's rows.
fn through(r: &[V3; 3], v: V3) -> V3 { std::array::from_fn(|k| v[0] * r[0][k] + v[1] * r[1][k] + v[2] * r[2][k]) }

/// State 1 of every breakable: a hit with [`BREAK_FLAGS`] while `+0x68`[0] = 0 breaks it (2); the hit record let go.
fn wait_hit(w: &mut MobyWorld, id: usize) {
    if let Some(h) = w.hit_message(id) {
        if h.tmpl.flags & BREAK_FLAGS != 0 && w.mobys[id].pv_i(0) == 0 {
            w.mobys[id].set_state(2, -1);
        }
    }
    w.clear_hit(id);
}

/// `0x2edd68` on moby `id` (crate::particles::staged_box).
#[allow(clippy::too_many_arguments)]
fn smoke(w: &MobyWorld, id: usize, ctx: &mut Ctx, lo: V3, hi: V3, vlo: V3, vhi: V3, count: i32, kind: u8) {
    let m = &w.mobys[id];
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    crate::particles::staged_box(sys, &mut ctx.game.rng, &m.rows, m.pos, m.scale, lo, hi, vlo, vhi, count, kind);
}

/// 0x0f37 (`0x41b360`). **0**: the glows (`0x340ae0`): at its up row set to 20 × its scale (`0x2f00f8`), 367500 grey
/// 0x28208080 (its def's frame 0) and 325500 0x20208080 (frame 1) pulled 0.75, far 0x60, lasting; 1. **1**: the hit
/// ([`wait_hit`]). **2**: its pieces (`$gp+0x2a70`: x / y −3..3, z 1..7; spin 90..360 (`+0x2a68`), gravity 10..20
/// (`+0x2a60`); pointed 0x7f, flags 1: no bolts of their own), the level's bolts 3 up; at its head (0, 0, 20 × its
/// scale) a blast with every count 0 and size 0, and the explosion (`0x3219e0`: near 9, size 1, 15 embers, flags
/// 0x810001); out of the head's four corners (±1/30, ±1/30, 1/15 through its rows) six debris chunks each (reach 0.25,
/// 5 a second, 0.75..1.25 of their size, pointed); deleted.
pub fn update_0f37(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let m = &w.mobys[id];
            let (pos, up) = (m.pos, set_len(m.rows[2], m.scale * 20.0));
            if let Some(sys) = ctx.parts.as_deref_mut() {
                crate::particles::glow_def(sys, &mut ctx.game.rng, 367_500.0, 0.0, up, id, pos, 0x60, false, 0x2820_8080, -1, 0);
                crate::particles::glow_def(sys, &mut ctx.game.rng, 325_500.0, 0.75, up, id, pos, 0x60, false, 0x2020_8080, -1, 1);
            }
            w.mobys[id].set_state(1, -1);
        }
        1 => wait_hit(w, id),
        2 => {
            shards::throw_all(w, id, ctx, [-3.0, -3.0, 1.0, 0.0, 3.0, 3.0, 7.0, 0.0], (90.0, 360.0), (10.0, 20.0), 0x7f, 0, -1, 1, 0xffff);
            bolts::level_drop(w, id, 3.0, 1, ctx);
            let s = w.mobys[id].scale;
            let head = frame_point(w, id, [0.0, 0.0, s * 20.0]);
            blast::burst(w, ctx, head, &Blast::default());
            let e = Explosion { near: 9.0, size: 1.0, embers: 0xf, sound: -1, lod: -1, flags: 0x81_0001, ..Default::default() };
            blast::explode(w, ctx, Some(id), Some(head), &e);
            let head = frame_point(w, id, [0.0, 0.0, s * 20.0]);
            for i in 0..4 {
                let (x, y) = ((i / 2 * 2 - 1) as f32, (i % 2 * 2 - 1) as f32);
                let v = through(&w.mobys[id].rows, [x * 0.033_333_335, y * 0.033_333_335, 0.066_666_67]);
                shards::throw_chunks(w, ctx, 0.25, 5.0, (0.75, 1.25), id, head, 6, true, v, CHUNK_COLOUR, None);
            }
            w.delete(id);
        }
        _ => {}
    }
}

/// 0x0f3e (`0x41bc30`). Its pvars: `[0]` unbreakable, `[4]` the glow, `[8]` the smoke's timer. **0**: the glow
/// (`0x340ae0`: 4.2e6 × its scale, at (0, 0, −8) × its scale, pulled 1.5, 0x40202080, far its draw distance); 1.
/// **1**: the hit ([`wait_hit`]). **2**: its pieces (`$gp+0x2af0`: x / y −3..3, z −1..−7; spin 90..360, gravity
/// 10..20; pointed 0x3f, the level's bolts), its class sound 0 again (`0x361dc8`), the blast (`0x403f10`: size 0.75,
/// sprites 0x19 / 0x14 / 0x14, 0x14 puffs, 0x2d streaks, its light); hidden (mode 0x41), its stub 0x0f40 in its
/// place, 30 ticks; 3. **3**: smoke ([`smoke`], kind 1): 40 puffs on its first tick out of (−1, −1, −7)..(1, 1, −5)
/// at (−5, −5, −25)..(5, 5, −7) a second, then three a tick from a box and velocities eased by `1 − ticks/30` toward
/// (−3, −1, −6.5)..(−1, 1, −4.5) and (−15, −5, −10)..(−7, 5, 0) (`0x2efe58`); deleted when the ticks run out.
pub fn update_0f3e(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    const LO: V3 = [-1.0, -1.0, -7.0];
    const HI: V3 = [1.0, 1.0, -5.0];
    const LO2: V3 = [-3.0, -1.0, -6.5];
    const HI2: V3 = [-1.0, 1.0, -4.5];
    const VLO: V3 = [-5.0, -5.0, -25.0];
    const VHI: V3 = [5.0, 5.0, -7.0];
    const VLO2: V3 = [-15.0, -5.0, -10.0];
    const VHI2: V3 = [-7.0, 5.0, 0.0];
    match w.mobys[id].state {
        0 => {
            let m = &w.mobys[id];
            let (s, pos, far) = (m.scale, m.pos, m.draw_dist);
            if let Some(sys) = ctx.parts.as_deref_mut() {
                crate::particles::glow(sys, &mut ctx.game.rng, s * 4.2e6, 1.5, [0.0, 0.0, -8.0 * s], id, pos, far, false, 0x4020_2080, -1);
            }
            w.mobys[id].set_state(1, -1);
        }
        1 => wait_hit(w, id),
        2 => {
            shards::throw_all(w, id, ctx, [-3.0, -3.0, -1.0, 0.0, 3.0, 3.0, -7.0, 0.0], (90.0, 360.0), (10.0, 20.0), 0x3f, 0, -1, 0, 0xffff);
            ctx.host.play_sound(&w.mobys[id], 0, 0);
            let at = w.mobys[id].pos;
            blast::burst(w, ctx, at, &Blast { size: 0.75, bursts: [0x19, 0x14, 0x14], puffs: 0x14, streaks: 0x2d, light: true, shake: false });
            w.mobys[id].mode |= 0x41;
            w.mobys[id].set_pv_i(8, 0x1e);
            super::props::spawn_in_place(w, id, 0x0f40, 0, &*ctx.host);
            w.mobys[id].set_state(3, -1);
        }
        3 => {
            if w.mobys[id].first() {
                smoke(w, id, ctx, LO, HI, VLO, VHI, 0x28, 1);
            }
            let t = 1.0 - w.mobys[id].pv_i(8) as f32 * 0.033_333_335;
            let lerp = |a: V3, b: V3| -> V3 { std::array::from_fn(|k| a[k] + (b[k] - a[k]) * t) };
            let (vlo, vhi, lo, hi) = (lerp(VLO, VLO2), lerp(VHI, VHI2), lerp(LO, LO2), lerp(HI, HI2));
            smoke(w, id, ctx, lo, hi, vlo, vhi, 3, 1);
            let mut n = w.mobys[id].pv_i(8);
            let out = countdown(&mut n);
            w.mobys[id].set_pv_i(8, n);
            if out { w.delete(id); }
        }
        _ => {}
    }
}

/// 0x0f38 (`0x41b750`). **0**: its beams (not ported); 1. **1**: the hit ([`wait_hit`]). **2**: its pieces
/// (`$gp+0x2ab0`: x −4.5..4.5, y 2..4, z 2..6; spin 90..480, gravity 10..20; pointed 0x3f, the level's bolts), the
/// blast (as 0x0f3e's), its class sound 0; at (±12, −15, 20) × its scale two debris chunks each (reach 0.15, 2 a
/// second, 0.5..1 of their size, pointed, moving (0, −0.05, 0) through its rows a tick); smoke (kind 0) 15 puffs at
/// (±12, −12, 20) in its frame moving (−1.5, −5, −1.5)..(1.5, −15, 1.5) a second, the right side first; deleted.
pub fn update_0f38(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => w.mobys[id].set_state(1, -1),
        1 => wait_hit(w, id),
        2 => {
            shards::throw_all(w, id, ctx, [-4.5, 2.0, 2.0, 0.0, 4.5, 4.0, 6.0, 0.0], (90.0, 480.0), (10.0, 20.0), 0x3f, 0, -1, 0, 0xffff);
            let at = w.mobys[id].pos;
            blast::burst(w, ctx, at, &Blast { size: 0.75, bursts: [0x19, 0x14, 0x14], puffs: 0x14, streaks: 0x2d, light: true, shake: false });
            ctx.host.play_sound(&w.mobys[id], 0, 0);
            let s = w.mobys[id].scale;
            let v = through(&w.mobys[id].rows, [0.0, f32::from_bits(0xbd4c_ccce), 0.0]);
            for x in [12.0, -12.0] {
                let p = frame_point(w, id, [s * x, s * -15.0, s * 20.0]);
                shards::throw_chunks(w, ctx, 0.15, 2.0, (0.5, 1.0), id, p, 2, true, v, CHUNK_COLOUR, None);
            }
            for x in [12.0, -12.0] {
                let p = [x, -12.0, 20.0];
                smoke(w, id, ctx, p, p, [-1.5, -5.0, -1.5], [1.5, -15.0, 1.5], 0xf, 0);
            }
            w.delete(id);
        }
        _ => {}
    }
}

// ---------------------------------------------------------------------------------------------------------------
// The robots.

/// The robots' classes and their cage.
pub const ROBOT_93C: i16 = 0x093c;
pub const ROBOT_940: i16 = 0x0940;
pub const ROBOT_964: i16 = 0x0964;
pub const ROBOT_A59: i16 = 0x0a59;
pub const CAGE: i16 = 0x0d75;
/// The path network's class, and the robots' unseen floor (crate::moby::classes::endako_b::update_097a).
pub const NETWORK: i16 = 0x0f2d;
const FLOOR_97A: i16 = 0x097a;
/// 0x0964's child instead of a cage (crate::moby::classes::endako_c's).
const CHILD_964: i16 = 0x0972;

/// The network moby's pvars: the list of paths (`[0..0x20]` words, −1 ends it), the records (0x20 of them), their
/// count.
const NET_LIST: usize = 0;
const NET_NODES: usize = 0x80;
const NET_COUNT: usize = 0x480;
const NET_MAX: usize = 0x20;

/// The robot's pvars: its movement block (crate::moby::ai, `[0x18]`), its eight groups, its network moby (an index),
/// its child (a link), the glow's phase, its place in its group's spread, the records it and its goal are in, the
/// idles' count, the floor under its start.
const MOVE: usize = 0x110;
const GROUPS: usize = 0x200;
const NET: usize = 0x220;
const CHILD: usize = 0x354;
const PHASE: usize = 0x35c;
const SPREAD: usize = 0x360;
const AT: usize = 0x368;
const TO: usize = 0x36c;
const IDLES: usize = 0x380;
const START_Z: usize = 0x3a4;
/// The movement block's fields: radius, walk accel / most, stop distance, options, fence path, push moby, floor moby,
/// floor height, stuck ticks, velocity, goal distance, turn limits and speed.
const RADIUS: usize = MOVE;
const WALK_ACCEL: usize = MOVE + 0x20;
const WALK_MOST: usize = MOVE + 0x28;
const STOP: usize = MOVE + 0x34;
const OPTS: usize = MOVE + 0x38;
const FENCE: usize = MOVE + 0x3c;
const STUCK: usize = MOVE + 0x80;
const PUSHER: usize = MOVE + 0x84;
const FLOOR_MOBY: usize = MOVE + 0x88;
const FLOOR_Z: usize = MOVE + 0x90;
const VEL: usize = MOVE + 0xa0;
const GOAL_DIST: usize = MOVE + 0xcc;
const TURN_ACCEL: usize = MOVE + 0x14;
const TURN_BRAKE: usize = MOVE + 0x18;
const TURN_MOST: usize = MOVE + 0x1c;
const TURN_SPEED: usize = MOVE + 0xb4;

/// L03 `$gp+0xeb0..`: the stop distance 1.5 and the follow distance 3; the walk (accel 6 a second², most 3 a second,
/// `$gp+0xec0` / `+0xec4`); the pushes (most 2 a second, accel 8, `+0xed4` / `+0xed8`); the blasts' least spacing 45
/// ticks (`+0xecc`); the cage's pieces drawn (`+0xed0`).
const NEAR: f32 = 1.5;
const FOLLOW: f32 = 3.0;
const WALK: (f32, f32) = (6.0, 3.0);
const PUSH: (f32, f32) = (2.0, 8.0);
const BLAST_GAP: i32 = 0x2d;
const CAGE_PIECES: u16 = 0x3fff;
/// Degrees a second to radians a tick (`0x3998825c`).
const DEG_TICK: f32 = 0.000_290_888_25;

/// `+0x7c`: with 0x8000 one break piece drawn alone (Moby::piece), else the pieces drawn over the whole (bangles).
fn set_7c(m: &mut crate::moby::Moby, v: u16) {
    if v & 0x8000 != 0 {
        m.piece = Some((v & 0x7fff).trailing_zeros() as u8);
        m.bangles = 0;
    } else {
        m.piece = None;
        m.bangles = v;
    }
}

fn link(w: &MobyWorld, id: usize, off: usize) -> Option<usize> { crate::moby::ai::link(&w.mobys[id], off).filter(|&i| i < w.mobys.len()) }

/// The members of group `g` (none for 0xff or past the groups).
fn members(w: &MobyWorld, g: i32) -> Vec<usize> {
    if g & 0xff == 0xff { return Vec::new(); }
    usize::try_from(g).ok().and_then(|g| w.groups.get(g)).cloned().unwrap_or_default()
}

fn live(w: &MobyWorld, i: usize) -> bool { w.mobys.get(i).is_some_and(|m| m.state != 0xfe && m.state != 0xfd) }

/// The class's sequence `seq` from a random frame (`0x31b5f0(m, seq, randi(frames), blend)`) unless it plays.
fn play_rand(w: &MobyWorld, id: usize, ctx: &mut Ctx, seq: u8, blend: u8) {
    if super::props::seq(w, id, ctx) == seq { return; }
    let n = ctx.host.seq_frames(&w.mobys[id], seq).unwrap_or(0) as i32;
    let f = ctx.game.rng.randi(n) as u8;
    ctx.host.set_anim(&w.mobys[id], seq, f, blend);
}

/// The network's moby (`+0x220`, a moby index), when it is one.
fn net_of(w: &MobyWorld, id: usize) -> Option<usize> {
    usize::try_from(w.mobys[id].pv_i(NET)).ok().filter(|&n| w.mobys.get(n).is_some_and(|m| m.o_class == NETWORK))
}

/// 0x0f2d's set-up (`0x41b0b0` state 0): without pvars deleted; else its network built from its list (`0x32b318`,
/// crate::path_network::build_list, 32 records at most), the count at `+0x480`, its openings kept; 1.
fn net_build(w: &mut MobyWorld, net: usize, ctx: &mut Ctx) {
    if w.mobys[net].pvars.is_empty() {
        w.delete(net);
        return;
    }
    let m = &mut w.mobys[net];
    if m.pvars.len() < NET_COUNT + 4 { m.pvars.resize(NET_COUNT + 4, 0); }
    let list: Vec<i32> = (0..NET_MAX).map(|k| m.pv_i(NET_LIST + 4 * k)).collect();
    let mut nodes = crate::path_network::read_nodes(&m.pvars, NET_NODES, NET_MAX);
    let n = crate::path_network::build_list(ctx.level, &list, &mut nodes);
    crate::path_network::write_nodes(&mut m.pvars, NET_NODES, &nodes[..n]);
    m.set_pv_i(NET_COUNT, n as i32);
    m.set_state(1, -1);
    crate::path_network::open(&mut ctx.game.path_openings, ctx.level, &nodes[..n]);
}

/// 0x0f2d (`0x41b0b0`, five placed): the robots' path network; nothing after its set-up ([`net_build`]).
pub fn update_0f2d(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 0 { net_build(w, id, ctx); }
}

/// The network's records.
fn net_nodes(w: &MobyWorld, net: Option<usize>) -> Vec<crate::path_network::Node> {
    let Some(m) = net.map(|n| &w.mobys[n]) else { return Vec::new() };
    let n = (m.pv_i(NET_COUNT).max(0) as usize).min(NET_MAX);
    crate::path_network::read_nodes(&m.pvars, NET_NODES, n)
}

/// `0x41b120(net, p, open)`: the record that holds `p` closed to walking (flag 1) or opened; the network built
/// first if it is not yet.
fn net_mark(w: &mut MobyWorld, net: usize, ctx: &mut Ctx, p: V3, open: bool) {
    if w.mobys[net].state == 0 { net_build(w, net, ctx); }
    let mut nodes = net_nodes(w, Some(net));
    let mut k = -1;
    if !crate::path_network::locate(ctx.level, &nodes, p, &mut k) { return; }
    let n = &mut nodes[k as usize];
    if open { n.flags &= !1 } else { n.flags |= 1 }
    crate::path_network::write_nodes(&mut w.mobys[net].pvars, NET_NODES, &nodes);
}

/// `0x41b298(net, from, to)` → `0x41b2c0`: a way from `from` to `to` through the network (crate::path_network::
/// steer, 0.1 narrowing the openings, fresh records).
fn reachable(w: &MobyWorld, net: Option<usize>, ctx: &Ctx, from: V3, to: V3) -> bool {
    let nodes = net_nodes(w, net);
    let (mut yaw, mut at, mut goal) = (0.0, -1, -1);
    crate::path_network::steer(ctx.level, &nodes, 0.1, from, to, &mut yaw, &mut at, &mut goal)
}

/// `0x3c70c0`: the yaw offset of its place in the spread: the angle 1.5 subtends at the goal's distance (at most
/// 85.7°) × 0.2 × its place (`+0x360`).
fn spread_offset(m: &crate::moby::Moby) -> f32 {
    let a = atan2(m.pv_f(GOAL_DIST), 1.5).min(1.495_996_6);
    a * 0.2 * m.pv_i(SPREAD) as f32
}

/// `0x3c57c0(m, goal, spread)`: the heading through the network toward `goal` (`0x41b1d0` → crate::path_network::
/// steer_offset with its radius `+0x110`, the records kept in `+0x368` / `+0x36c`; with `spread` its spread's
/// offset), then the framework's move (`0x32e520`, crate::moby::ai::move_step). Whether the goal is reachable.
fn move_to(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, goal: V3, spread: bool) -> bool {
    let nodes = net_nodes(w, net_of(w, id));
    let m = &mut w.mobys[id];
    let off = if spread { spread_offset(m) } else { 0.0 };
    let (mut at, mut to, mut yaw) = (m.pv_i(AT), m.pv_i(TO), 0.0);
    let ok = crate::path_network::steer_offset(ctx.level, &nodes, off, m.pv_f(RADIUS), m.pos, goal, &mut yaw, &mut at, &mut to);
    m.set_pv_i(AT, at);
    m.set_pv_i(TO, to);
    crate::moby::ai::move_step(w, id, ctx, yaw, goal);
    ok
}

/// `0x3c6ef8(m, p)`: its place in the spread toward `p`: over the live members of its group (`+0x21`) in its
/// sub-state but itself, −1 for each on its left of the line to `p` and +1 for each on its right, held to ±5.
fn spread(w: &MobyWorld, id: usize, p: V3) -> i32 {
    let m = &w.mobys[id];
    if m.group as u8 == 0xff { return 0; }
    let rows = crate::hero::physics::euler_rows([0.0, 0.0, atan2(p[0] - m.pos[0], p[1] - m.pos[1])]);
    let mut n = 0;
    for o in members(w, m.group as u8 as i32) {
        if o == id || !live(w, o) || w.mobys[o].sub_state != m.sub_state { continue; }
        let d = crate::hero::physics::sub(w.mobys[o].pos, m.pos);
        n += if 0.0 < crate::hero::physics::dot(d, rows[1]) { -1 } else { 1 };
    }
    n.clamp(-5, 5)
}

/// `0x32f4e8(yaw, m)`: its yaw eased toward `to` by its block's turn limits (`0x3206d8`).
fn turn_to(m: &mut crate::moby::Moby, to: f32) {
    let (a, b, c) = (m.pv_f(TURN_ACCEL), m.pv_f(TURN_BRAKE), m.pv_f(TURN_MOST));
    let (mut yaw, mut v) = (m.rot[2], m.pv_f(TURN_SPEED));
    crate::hero::physics::turn_ease(to, a, b, c, &mut yaw, &mut v, 0);
    m.rot[2] = yaw;
    m.set_pv_f(TURN_SPEED, v);
}

/// `0x3c5970(r, p)`: a robot in state 4 (or 2) with no floor moby pushed away from `p` across: 16/3600 added to its
/// velocity, which is held to 2/60.
fn push(w: &mut MobyWorld, r: usize, p: V3) {
    let m = &mut w.mobys[r];
    if !matches!(m.state, 2 | 4) || m.pv_i(FLOOR_MOBY) != 0 { return; }
    let mut d = crate::hero::physics::sub(m.pos, p);
    d[2] = 0.0;
    let d = set_len(d, (PUSH.1 + PUSH.1) * 0.000_277_777_78);
    let mut v = add(m.pv_v(VEL), d);
    let most = PUSH.0 * 0.016_666_668;
    let l = crate::hero::physics::len(v);
    if most < l { v = crate::hero::physics::scale(v, most / l); }
    m.set_pv_v(VEL, v);
}

/// `0x3c5848(m)`: while its velocity lasts it backs off: past 1/60 it turns to face against it, then the
/// framework's move in place with the push's limits (`0x32e328`: most 2 a second, accel 8, options 0x10), its record
/// found again (`0x41b250`), and a robot it pushes on is pushed in turn.
fn back_off(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let v = w.mobys[id].pv_v(VEL);
    let l = crate::hero::physics::len(v);
    if l <= 0.0 { return; }
    if 0.016_666_668 < l {
        let to = add_rot(atan2(v[0], v[1]), std::f32::consts::PI);
        turn_to(&mut w.mobys[id], to);
    }
    let m = &mut w.mobys[id];
    let saved = (m.pv_i(OPTS), m.pv_f(WALK_MOST), m.pv_f(WALK_ACCEL));
    m.set_pv_i(OPTS, 0x10);
    m.set_pv_f(WALK_MOST, PUSH.0 * 0.016_666_668);
    m.set_pv_f(WALK_ACCEL, PUSH.1 * 0.000_277_777_78);
    let (yaw, pos) = (m.rot[2], m.pos);
    crate::moby::ai::move_step(w, id, ctx, yaw, pos);
    let m = &mut w.mobys[id];
    m.set_pv_i(OPTS, saved.0);
    m.set_pv_f(WALK_MOST, saved.1);
    m.set_pv_f(WALK_ACCEL, saved.2);
    let nodes = net_nodes(w, net_of(w, id));
    let mut at = w.mobys[id].pv_i(AT);
    crate::path_network::locate(ctx.level, &nodes, w.mobys[id].pos, &mut at);
    w.mobys[id].set_pv_i(AT, at);
    if let Some(o) = link(w, id, PUSHER) {
        if matches!(w.mobys[o].o_class, ROBOT_940 | ROBOT_93C | ROBOT_964) {
            let p = w.mobys[id].pos;
            push(w, o, p);
        }
    }
}

/// The robot's set-up (`0x3c4828` in state 0); see the module.
fn robot_init(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let mirror = ctx.game.rng.randi(2) != 0;
    let m = &mut w.mobys[id];
    if mirror { m.mode |= 0x8000 } else { m.mode &= 0x7fff }
    if let Some(z) = crate::moby::ground_under(ctx, m.pos) {
        m.pos[2] = z;
        m.set_pv_f(START_Z, z);
    }
    let class = m.o_class;
    let child_class = if class == ROBOT_964 { CHILD_964 } else { CAGE };
    if w.class(child_class).is_some() {
        let c = w.create(child_class);
        let (pos, rot, light, ambient) = { let m = &w.mobys[id]; (m.pos, m.rot, m.light, m.ambient) };
        let x = &mut w.mobys[c];
        x.update_dist = 0xff;
        x.draw_dist = 0xff;
        x.drawn = 1;
        x.parent = Some(id);
        x.pos = pos;
        x.rot = rot;
        x.light = light;
        x.ambient = ambient;
        w.rebuild(c, &*ctx.host);
        let base = w.class(child_class).map_or(1.0, |k| k.scale);
        let x = &mut w.mobys[c];
        x.mode &= 0xfff8;
        if child_class == CAGE { set_7c(x, CAGE_PIECES); }
        x.draw_dist = 0x30;
        if class == ROBOT_93C { x.scale = base * 1.2; }
        w.changed = true;
        crate::moby::ai::set_link(&mut w.mobys[id], CHILD, Some(c));
    }
    let m = &mut w.mobys[id];
    m.set_pv_b(0x28, 3);
    m.set_pv_f(0x20, 1.0);
    m.set_pv_s(0x24, 1);
    m.set_pv_f(STOP, NEAR);
    m.set_pv_f(MOVE + 4, f32::from_bits(0x3e99_999a));
    let r = match class {
        ROBOT_940 => 0x3ec7_ae14,
        ROBOT_93C | ROBOT_A59 => 0x3ed7_0a3d,
        _ => 0x3e19_999a,
    };
    m.set_pv_f(RADIUS, f32::from_bits(r));
    m.pos[2] += 0.089;
    if class == ROBOT_940 {
        // `0x2f00f8(−0.2, d, rows[0])`: 0.2 back along its facing.
        let d = set_len(m.rows[0], -0.2);
        m.pos = add(m.pos, d);
    }
    m.set_pv_f(MOVE + 0xc, 0.5);
    m.set_pv_f(MOVE + 0x10, f32::from_bits(0x3fcb_4bc5));
    m.set_pv_f(MOVE + 0x30, f32::from_bits(0x3f06_0a93));
    m.set_pv_f(MOVE + 0x2c, f32::from_bits(0x3bb6_0b61));
    m.set_pv_f(MOVE + 8, 0.5);
    m.set_state(1, 0);
    play_rand(w, id, ctx, 0, 0);
    if class == ROBOT_940 {
        let net = net_of(w, id);
        for k in 0..8 {
            let g = w.mobys[id].pv_i(GROUPS + 4 * k);
            for o in members(w, g) {
                if w.mobys.get(o).is_some_and(|x| x.o_class == FLOOR_97A) {
                    let p = w.mobys[o].pos;
                    if let Some(n) = net { net_mark(w, n, ctx, p, false); }
                }
            }
        }
    } else if class == ROBOT_A59 {
        super::props::set_seq(w, id, ctx, 6, 0, 0);
    }
}

/// `0x3c6c70` (the cage's place only) and `0x2ed370`: the robot beamed back to its child (cage), on the floor
/// under it, facing its way (crate::moby::classes::teleport::start).
fn respawn(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let Some(c) = link(w, id, CHILD) else { return };
    let (mut p, rot) = (w.mobys[c].pos, w.mobys[c].rot);
    if let Some(z) = crate::moby::ground_under(ctx, p) { p[2] = z; }
    super::teleport::start(w, id, p, rot, &[]);
}

/// The robots' every-tick part before their state (`0x3c4828` past the set-up); see the module.
fn robot_pre(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 0 { robot_init(w, id, ctx); }
    crate::shadow_volumes::probe_near(w, id, ctx, 24.0, 0x16);
    let hit = w.hit_message(id).is_some_and(|h| h.tmpl.flags & 1 != 0);
    let m = &w.mobys[id];
    let z = m.pos[2];
    let fell = z < m.pv_f(START_Z) - 10.0 && z - 10.0 < m.pv_f(FLOOR_Z);
    if !hit && !fell { return; }
    if BLAST_GAP < (ctx.game.robot_blast_frame - ctx.game.frame).abs() {
        ctx.game.robot_blast_frame = ctx.game.frame;
        let at = add(w.mobys[id].pos, [0.0, 0.0, 0.5]);
        blast::burst(w, ctx, at, &Blast { size: 0.25, bursts: [5, 5, 5], puffs: 10, streaks: 0x14, light: true, shake: false });
        if w.mobys[id].o_class == ROBOT_964 {
            let v = w.mobys[id].pv_v(VEL);
            shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 6.0), 0xf, v, 0xff);
        }
        let m = &mut w.mobys[id];
        m.set_state(4, 1);
        m.set_pv_v(VEL, [0.0; 3]);
        m.set_pv_i(MOVE + 0xd8, -1);
        respawn(w, id, ctx);
    }
    w.clear_hit(id);
}

/// The glow (`0x3c5b80`), drawn: its phase steps by the sub-state's speed and the glow word goes between the
/// sub-state's two colours by `sin(phase)·0.5 + 0.5` (`$gp+0xee0..0xf2c`).
fn glow(m: &mut crate::moby::Moby) {
    if m.drawn == 0 { return; }
    let (a, b, speed) = match m.sub_state {
        1 | 7 => (0x4000_8000, 0x2000_4000, 240.0),
        2 | 8 => (0x4000_0080, 0x2000_0040, 360.0),
        3 => (0x40c0_2020, 0x2080_2020, 360.0),
        4 => (0x4080_0080, 0x2060_0060, 360.0),
        5 => (0x4000_4080, 0x2000_6080, 360.0),
        6 => (0x4080_6000, 0x2040_3000, 360.0),
        _ => (0x4000_8080, 0x2000_4040, 120.0),
    };
    let ph = add_rot(m.pv_f(PHASE), speed * DEG_TICK);
    m.set_pv_f(PHASE, ph);
    m.glow = crate::hud::lerp_rgba(ph.sin() * 0.5 + 0.5, a, b);
}

/// 0x093c / 0x0940 / 0x0964 / 0x0a59 (`0x3c31f8`): Endako's robots; see the module.
pub fn update_robot(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    robot_pre(w, id, ctx);
    if w.mobys[id].sub_state == 1 && w.mobys[id].once & 2 == 0 {
        let m = &mut w.mobys[id];
        m.once |= 2;
        crate::moby::ai::senses(m, WALK.0 * 0.000_277_777_78, WALK.1 * 0.016_666_668, f32::from_bits(0x3b64_c38a), f32::from_bits(0x3dd6_7752));
        m.set_pv_f(STOP, NEAR);
        m.set_pv_f(MOVE + 0x24, f32::from_bits(0x3ada_740e));
    }
    let hero = ctx.hero.pos;
    match w.mobys[id].state {
        1 => {
            let woke = link(w, id, CHILD).is_some_and(|c| w.mobys[c].once & 4 != 0);
            if woke {
                let (text, slot) = match w.mobys[id].o_class {
                    ROBOT_940 => (0x9cf, 0x26),
                    ROBOT_93C => (0x9d1, 0x28),
                    ROBOT_A59 => (0xa08, 0x5f),
                    _ => (0x9d2, 0x29),
                };
                let k = slot as usize;
                let ask = if slot == 0x29 { (ctx.game.help_levels[k] as i32) >= 0 } else { ctx.game.help_counts[k] == 0 };
                if ask { ctx.game.help(text, slot); }
                w.mobys[id].set_state(4, 1);
            }
        }
        3 => {
            let ok = move_to(w, id, ctx, hero, true);
            let m = &w.mobys[id];
            let d = m.pv_f(GOAL_DIST);
            if !ok || d < NEAR || (0x3c < m.pv_i(STUCK) && d < FOLLOW) {
                w.mobys[id].set_state(4, -1);
                play_rand(w, id, ctx, 0, 10);
            }
            if ctx.game.frame & 0xf == id as i32 & 0xf {
                let n = spread(w, id, hero);
                w.mobys[id].set_pv_i(SPREAD, n);
            }
        }
        4 => {
            if w.mobys[id].first() {
                play_rand(w, id, ctx, 0, 0xf);
                let m = &mut w.mobys[id];
                m.set_pv_i(IDLES, 2);
                m.set_pv_v(VEL, [0.0; 3]);
            }
            if ctx.hero.wall_moby == Some(id) || ctx.hero.push_moby == Some(id) { push(w, id, hero); }
            back_off(w, id, ctx);
            let net = net_of(w, id);
            let pos = w.mobys[id].pos;
            if FOLLOW < crate::hero::physics::dist2(pos, hero) && reachable(w, net, ctx, pos, hero) {
                w.mobys[id].set_state(3, -1);
                play_rand(w, id, ctx, 2, 10);
            }
            let a = ctx.host.anim(&w.mobys[id]);
            if super::props::wrapped(w, id, ctx) && a.is_some_and(|a| a.seq_a == a.seq_b) {
                if a.is_some_and(|a| a.seq_a == 1) {
                    let n = ctx.host.seq_frames(&w.mobys[id], 0).unwrap_or(0) as i32;
                    let f = ctx.game.rng.randi(n) as u8;
                    ctx.host.set_anim(&w.mobys[id], 0, f, 0xf);
                } else {
                    let n = w.mobys[id].pv_i(IDLES);
                    if ctx.game.rng.randi(n) == 0 {
                        // `0x31ff78(&n, 10, 1)`: a step toward 10.
                        let m = &mut w.mobys[id];
                        m.set_pv_i(IDLES, n + (10 - n).clamp(-1, 1));
                        super::props::set_seq(w, id, ctx, 1, 0, 0xf);
                    }
                }
            }
        }
        _ => {}
    }
    let m = &mut w.mobys[id];
    if !matches!(m.state, 0x16 | 1 | 0xf | 0x10) && (m.pv_f(FLOOR_Z) < m.pos[2] - 0.05 || m.pv_i(FLOOR_MOBY) != 0) {
        m.rot[0] = 0.0;
        m.rot[1] = 0.0;
        crate::moby::ai::hold(w, id, ctx);
    }
    let m = &mut w.mobys[id];
    if m.state_ticks != -1 { m.state_ticks += 1; }
    glow(m);
    if m.sub_state != 7 && m.state != 1 {
        let at = m.pv_i(AT);
        let nodes = net_nodes(w, net_of(w, id));
        let path = usize::try_from(at).ok().and_then(|k| nodes.get(k)).map_or(-1, |n| n.path);
        w.mobys[id].set_pv_i(FENCE, path);
    }
}

/// 0x0d75 (`0x4096c8`): a robot's cage. **0**: its pieces 10..14 drawn over it (`+0x7c` = 0x7c00, `$gp+0x25dc`); 1.
/// **1**: a hit with the flag 0x10000 breaks it: its class sound 0, the pieces 0x3cff thrown (`0x327e78` = L00
/// `0x327498`, crate::moby::shards::break_into: 1.5..3 a second across and up), 2. **2**, its first tick: piece 14
/// alone (`+0x7c` = 0xc000), its collision primitives 0..3 (the bars) off (`+0x9c` = 0xf), hooked — its robot wakes.
pub fn update_cage(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            set_7c(&mut w.mobys[id], 0x7c00);
            w.mobys[id].set_state(1, -1);
        }
        1 => {
            if w.hit_message(id).is_some_and(|h| h.tmpl.flags & 0x10000 != 0) {
                ctx.host.play_sound(&w.mobys[id], 0, 0);
                shards::break_into(w, id, ctx, (1.5, 3.0), (1.5, 3.0), 0x3cff, [0.0; 3], 0xff);
                w.mobys[id].set_state(2, -1);
            }
        }
        2 => {
            if w.mobys[id].first() {
                let m = &mut w.mobys[id];
                set_7c(m, 0xc000);
                m.once |= 4;
                m.coll_disable = 0xf;
            }
        }
        _ => {}
    }
}

/// 0x0972 (`0x3cb898`): 0x0964's cage, a glass dome. **0**: a placed one (below `0x1b2a20`, the made mobys' first
/// slot: no robot made it) has its collision primitives off (`+0x9c` = 0xf) and goes to 3, nothing more; one its robot
/// made has its pieces 0, 2, 4, 5 drawn over it (`+0x7c` = `$gp+0x12d0`, 0x35), 1. **1**: drawn last frame, its glow
/// (`+0x78`) pulses `$gp+0x12c8` (`0x80363636`) .. `$gp+0x12cc` (`0x80828282`) on the game's frame mod 150; a hit with
/// the flag 0x10000 breaks it: 5 debris chunks half a unit up (`0x328028`: reach 1/3, speed 3, 0.5..1 of the scale,
/// 0.05 up a tick, `0x504040c0`), its pieces 0..5 thrown (`0x327e78`, `$gp+0x12d8..`: 1.5..3 a second across and up),
/// its class sound 0, `+0x7c` 0, hooked (its robot wakes), its primitives off, no update (`+0x34` |= 2); 2.
/// Not ported: the dome's draw `0x3cbab8` (registered while drawn: a 57-point mesh of 42 quads mirroring the camera
/// within 16 across, through `0x2ec4f0`); the class's model is drawn as for any moby.
pub fn update_0972(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            if m.parent.is_none() {
                m.coll_disable = 0xf;
                m.set_state(3, -1);
            } else {
                set_7c(m, 0x35);
                m.set_state(1, -1);
            }
        }
        1 => {
            if w.mobys[id].drawn != 0 {
                let t = ((ctx.game.frame % 150) as f32 * 0.041_887_87 - std::f32::consts::PI).sin();
                w.mobys[id].glow = crate::hud::lerp_rgba((t + 1.0) * 0.5, 0x8036_3636, 0x8082_8282);
            }
            if w.hit_message(id).is_some_and(|h| h.tmpl.flags & 0x10000 != 0) {
                let at = add(w.mobys[id].pos, [0.0, 0.0, 0.5]);
                let reach = f32::from_bits(0x3eaa_7efa);
                shards::throw_chunks(w, ctx, reach, 3.0, (0.5, 1.0), id, at, 5, true, [0.0, 0.0, f32::from_bits(0x3d4c_ccce)], 0x5040_40c0, None);
                shards::break_into(w, id, ctx, (1.5, 3.0), (1.5, 3.0), 0x3f, [0.0; 3], 0xff);
                ctx.host.play_sound(&w.mobys[id], 0, 0);
                let m = &mut w.mobys[id];
                set_7c(m, 0);
                m.once |= 4;
                m.coll_disable = 0xf;
                m.mode |= crate::moby::mode::NO_UPDATE;
                m.set_state(2, -1);
            }
        }
        _ => {}
    }
}
