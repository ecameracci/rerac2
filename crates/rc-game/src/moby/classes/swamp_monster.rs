//! Oozla's boss, **Swamp Monster II** (0x0a54, L01 `0x3c1db8`, unit U75), the transporter into its arena (0x0e38, L01
//! `0x401540`, U101; the same code on level 8) and the two classes the boss makes: its spit ball (0x0b24, made by
//! `0x3db7a8`, its update `0x3dc040`) and its reward (0x1250, made by `0x418748`, its update `0x4179c0`).
//!
//! **The transporter 0x0e38.** Its pvars: `[0]` the area that sends Ratchet (−1 none), `+4` (16 bits) level 8's
//! scene switch, `+6` (16 bits) a planet unlocked at the end (−1 none), `[2]` the chunk it switches to (−1 none),
//! `[3]` the cuboid Ratchet is put at (−1: none), `[4]` / `[5]` the music's stinger and track asked for at the end,
//! `[6]` set: Ratchet need not be on the ground. Oozla's: area 66, chunk 1, cuboid 188, music 10 / 8, `[6]` 1.
//! * **0**: with its mission (`+0xb0`) done it goes (`0x307440`); else 1. (The empty pvars only print.)
//! * **1**: poked (`+0xbc`), or Ratchet in its area (`0x320388`; on the ground unless `[6]`): updated at any distance,
//!   the screen faded out over 20 frames with the game stopped (`0x2dd378(0x14)`, [`GameState::blocking_fade`]),
//!   Ratchet put at the cuboid `[3]` (`0x2b9b38(.., −1, 1)`: his state kept, the camera reset), the chunk switch
//!   (`0x2f9bd8([2], 0)`, crate::moby::Ctx::switch_chunk), 3.
//! * **3**: once no scene is asked for or playing (`0x367d78`), the planet `+6` unlocked and its banner
//!   (`0x318628`, `0x31a580`); 4.
//! * **4**: its mission done (`0x309060`) with the checkpoint at its own place (`0x34fea8`) when it was not, Ratchet
//!   idle (`0x2c1ca8(0)`, `0x2beea8(0, 1)`), the music `[5]` / `[4]` asked for (`0x3245e8`), and it goes.
//!
//! **The boss 0x0a54** stands in the arena of chunk 1 at its height `+0x2e8`, on the framework
//! ([`crate::moby::ai`]): health block `+0x20` (its damages: bytes `+0x34` the bites and the spit, `+0x35` the grab's
//! swipe, `+0x36` the throw), its colour flash `+0x50`, reaction `+0x60` (a hurt: sub-state 6, the state kept), target
//! `+0x110`, movement `+0x130` (radius 4, the fence path `+0x2c4`). Every tick ([`checks`], `0x3c15c8`): the damage —
//! its health out (or `+0x2e` = 2, a kill asked from outside: its death reaction, sub 6) pokes the cutaway `+0x2f8`
//! and puts it at the cuboid `+0x2fc`; the target (area `+0x2f0`); its camera tweak (`+0x2e4`: the camera 10 behind
//! Ratchet facing it, field of view 79°; 20 while it spits or holds him); the boss bar (`0x2f20b8`, picture 2) while
//! Ratchet is under 20 above it and it fights (sub-states 2..6). At the end ([`tail`], `0x3c1548`): its flash, its
//! ticks, the waits `+0x2da` / `+0x2dc` / `+0x2de` counted down; and Ratchet drowned (`0x2b3d20`) once more than 1.8
//! under the swamp's water, 1 under the boss and within 10 of it.
//!
//! **Sub-states** (`+0x95`):
//! * **0** set-up ([`setup`], `0x3c1250`; with its mission done the moby `+0x308` is poked and it goes), 1.
//! * **1** once it has its target: the fight's music (`[0x310]` / `[0x318]`) and its camera tweak (priority 5), 2.
//! * **2** the fight ([`fight`]): [`approach`] (`0x3c1810`): farther than `+0x2ec` it walks (sequence `+0x2e1`) to
//!   Ratchet, or round the arena (38 from the cuboid `+0x2cc`, 5° a step) while he is more than 15° round from it;
//!   nearer it turns to him in place (`+0x2e2`). Its mode `+0xbc` picks the attacks by his distance and the angle
//!   to him: the lunge ([`lunge`], state 1, sequence 6 / 7), the spit ([`spit`], 2, 0xd), the swing ([`swing`], 3, 8),
//!   the grab ([`grab`], 4, 0xe), each with its wait; farther than 40 it wades away (sequence 4, faster turns) and
//!   comes back up (0x16) within 15. Under two thirds of its health it roars (0x10) into the second mode set (3..6:
//!   the grab, then swings and lunges), under a third into the third (7..9).
//! * **3** the attack ([`attack`]): the lunge's jaws sweep (frames 44..68: spheres of 0.5 on joints 1, 2 or 5, 6 and
//!   lines between them, jittered 0.3), the swing's arm hits (frames 34..41, joint 8 or 9, 2.2), the spit at frame 16
//!   (a 0x0b24 from joint 10, 14 a second across, solved for 9.8 of gravity, `0x3140e8`), the grab (frames 12..17:
//!   Ratchet within 3.9 of joint 12 (2.4 lower) is caught, `+0x2f4`; frames 17..19 its swipe at joint 12 or 13).
//!   Caught, he is held at joint 10 ([`hold_hero`], `0x3c1ca8`); at the sequence's end he is bitten (`+0x36`; under
//!   2 health dead) and thrown (4, sequence 0xb); else back to 2.
//! * **4** the throw: it turns side-on to the arena's middle; past frame 33 Ratchet is let go (0x16) and, while he is
//!   within 13 of joint 14, pushed along its facing 20 a second, and up by 0.7 of that times `(1 − d/150)²`.
//! * **5** a roar or a wade's end: at the sequence's end back to 2. **6** the knock (the framework's flight): dead → 7.
//! * **7** dying: frozen (its animation stopped, hidden, no target), its reward 0x1250 made at its place (half a unit
//!   lower; [`make_reward`], `0x418748`), Ratchet held (0x6f); the reward's path `+0x2a0` from Ratchet to it, 1 higher,
//!   0.85 of it; 8.
//! * **8** the reward flies to Ratchet over 160 ticks (`+0x2ac` from 1 down by 0.00625, 4 high at the middle), its
//!   show's place 2.5 from Ratchet toward the boss; Ratchet kept at the cutaway's cuboid; the cutaway played while
//!   poked (`0x395620`). Once the reward is gone (taken): its bolts (`0x310b28(m, 0, 3)`), its mission done, then (with
//!   a cuboid `+0x304`) the 16-frame fade, chunk 0 (`0x2f9bd8(0, 0)`), Ratchet at that cuboid idle with the camera
//!   reset and the checkpoint there; the moby `+0x308` poked, the music `+0x314` / `+0x31c`, and it goes.
//!
//! Its sounds are on chunk 1's bank 2 (crate::audio::chunk); the update plays none itself.
//!
//! **Its rewards**: its bolts, its mission, and the reward 0x1250 that gives the item 0x32 (`0x2e99e8(0x32)`, message
//! 0x129a). No skill point.
//!
//! **The spit ball 0x0b24** (`0x3dc040`): it grows to 1.6 its class's scale (0.025 of the way a tick), moves by its
//! velocity `[0]`, its sequence 1 after the first; touching anything (a sphere of 1.6, `0x2d5048`, not its owner
//! `[4]`, not a 0x0660) or under the water it splashes: the general explosion (`0x3165d8`,
//! crate::moby::blast::explode: a sphere of 2 hits for its damage `[6]`, flags 0x810001; its flashes and four embers),
//! two lines 6 down from Ratchet's moby (`0x3132c8`, 0.0001), and it goes. Else gravity 9.8 a second²; out of
//! 2..1021 it goes.
//!
//! **The reward 0x1250** (`0x4179c0`) is the platinum bolt's show (crate::moby::classes::platinum) with its own
//! prize: with the item 0x32 already had it goes; carried by the boss (state 1, turning); Ratchet within 3 across and 2
//! up takes it: with `+0xc` (16 bits) set at once (message 0x3315, the item, its sound 1), else the show
//! (platinum's `0x398500`, the camera `0x4183f8`) and at its end the message 0x129a, the item, Ratchet idle, the
//! camera behind him.
//!
//! Not ported: the blocking fades of the reward's show (`0x2dd378(10)`); the autosaves at the transporter's and the
//! reward's ends (`0x303030`, G-LVL-006); the companion moby 0x0660 the boss makes (`0x3a7680`); its tail's aim
//! (`0x31cfa0`, G-ENM-001); the cutscene lock `0x18c0d5` it sets while it holds Ratchet and Ratchet's `0x18a010`;
//! the path lengths `0x338aa8` its paths are given (the port's paths carry none); the 0x0b24's trail and splash
//! particles (`0x3dbbe8`, `0x3db898`, G-PRT-001); the reward's rings and sparkles (`0x418188`,
//! `0x418610`: their random draws are left out), its vehicle check (`0x18c0d4` 3: `0x2b37f0`); the flag
//! `0x1b22a8` the transporter and the reward hold; level 8's scene switch of the transporter (`+4` = 0 there).
//!
//! The port's own steps: the original stops the game inside the tick for a blocking fade and goes on after it; here
//! the fade is asked for ([`GameState::blocking_fade`]) and the rest waits for the next tick — the transporter's
//! state 2 and the boss's sub-state 9.

use crate::game::GameState;
use crate::hero::physics::*;
use crate::moby::hits::HitTemplate;
use crate::moby::{ai, mode, Ctx, MobyWorld};
use super::leaper::seq_a;
use super::props::{seq, wrapped};

pub const CLASS: i16 = 0x0a54;
pub const TRANSPORTER: i16 = 0x0e38;
pub const BALL: i16 = 0x0b24;
pub const REWARD: i16 = 0x1250;
/// The class whose touch the ball passes through (`0x3dc040`).
const COMPANION: i16 = 0x0660;
/// The reward's item (`0x1a7b32`, `0x2e99e8(0x32)`).
const ITEM: usize = 0x32;

/// Degrees to radians (`0.017453294`); a second's degrees to a tick's radians (`0.00029088825`); a second²'s degrees
/// to a tick²'s radians (`4.8481375e-6`).
const DEG: f32 = 0.017_453_294;
const DEG_S: f32 = 0.000_290_888_25;
const DEG_S2: f32 = 4.848_137_5e-6;

/// The boss's tuning (`$gp+0xe20..+0xecc` on Oozla).
mod tune {
    /// `+0xe20`: the movement block's gravity (a second²).
    pub const GRAVITY: f32 = 9.8;
    /// `+0xe24` / `+0xe28` / `+0xe2c`: the near distances (the grab's, the second mode's, the first's).
    pub const NEAR_GRAB: f32 = 12.0;
    pub const NEAR_2: f32 = 18.0;
    pub const NEAR: f32 = 15.0;
    /// `+0xe30`: the spit's farthest; `+0xe38`: farther it wades away.
    pub const SPIT_FAR: f32 = 40.0;
    pub const FAR: f32 = 40.0;
    /// `+0xe3c`: the grab's reach.
    pub const GRAB_REACH: f32 = 3.9;
    /// `+0xe44` / `+0xe48` / `+0xe4c`: the walk's accel, the turn's accel and most (`0x3235c8`).
    pub const WALK_A: f32 = 8.0;
    pub const TURN_A: f32 = 120.0;
    pub const TURN_MOST: f32 = 120.0;
    /// `+0xe50..+0xe68`: the walk's most at the set-up, near, roaring, the second mode's near, the third's roar and near.
    pub const WALK_SETUP: f32 = 6.0;
    pub const WALK: f32 = 4.0;
    pub const WALK_ROAR: f32 = 7.5;
    pub const WALK_2: f32 = 4.0;
    pub const WALK_ROAR_3: f32 = 7.4;
    pub const WALK_3: f32 = 4.0;
    /// `+0xe6c..+0xe78`: wading (walk accel, most, turn accel, most).
    pub const WADE: [f32; 4] = [20.0, 15.0, 360.0, 240.0];
    /// `+0xe7c` / `+0xe80` / `+0xe84` / `+0xe88`: the facings (degrees) of the lunge, the spit, the grab, the swing.
    pub const FACE: f32 = 15.0;
    pub const FACE_SPIT: f32 = 35.0;
    pub const FACE_GRAB: f32 = 5.0;
    pub const FACE_SWING: f32 = 15.0;
    /// `+0xe90` / `+0xe94`: one in 19 / 99 ticks it lunges / spits (`0x311708(n − 1)` = 0).
    pub const LUNGE_ODDS: i32 = 20;
    pub const SPIT_ODDS: i32 = 100;
    /// `+0xe98`: the second mode's swings before it wades off; `+0xe9c`: the first mode's ticks before it takes its
    /// distance anyway.
    pub const SWINGS: u8 = 3;
    pub const PATIENCE: u16 = 900;
    /// `+0xea0`: within this (degrees) round the arena it walks straight at him.
    pub const ROUND: f32 = 15.0;
    /// `+0xea4` / `+0xea8` / `+0xeac`: the waits after a lunge or swing, a spit, a grab.
    pub const LUNGE_WAIT: i16 = 180;
    pub const SPIT_WAIT: i16 = 360;
    pub const GRAB_WAIT: i16 = 480;
    /// `+0xeb0`: the camera's field of view (degrees); `+0xeb4` / `+0xeb8` its distances.
    pub const FOV: f32 = 79.0;
    pub const CAM_NEAR: f32 = 10.0;
    pub const CAM_FAR: f32 = 20.0;
    /// `+0xebc` / `+0xec0`: the throw's push (a reach, then 20 a second).
    pub const THROW_REACH: f32 = 50.0;
    pub const THROW_SPEED: f32 = 20.0;
    /// `+0xec4` / `+0xec8`: the spit's speed across (a second) and gravity (a second²).
    pub const SPIT_SPEED: f32 = 14.0;
    pub const SPIT_G: f32 = 9.8;
}

/// The boss's pvars.
mod pv {
    pub const HP: usize = 0x20;
    pub const MAX: usize = 0x24;
    /// The health block's `+0xe`: 1; 2 a kill asked from outside; 3 dying of it.
    pub const KILL: usize = 0x2e;
    pub const DMG_BITE: usize = 0x34;
    pub const DMG_SWIPE: usize = 0x35;
    pub const DMG_THROW: usize = 0x36;
    pub const FLASH: usize = 0x50;
    /// The reaction block's fallback sequences (`+0x11` / `+0x12`) and options (`+0x38`).
    pub const R_SEQ_A: usize = 0x71;
    pub const R_SEQ_B: usize = 0x72;
    pub const R_OPTS: usize = 0x98;
    pub const T_MOBY: usize = 0x120;
    pub const T_FOUND: usize = 0x124;
    pub const MV: usize = 0x130;
    /// The goal (`+0x2a0`), later the reward's path from Ratchet; `+0x2ac` its share left.
    pub const GOAL: usize = 0x2a0;
    pub const SHARE: usize = 0x2ac;
    pub const PATHS: [usize; 3] = [0x2c0, 0x2c4, 0x2c8];
    pub const MIDDLE: usize = 0x2cc;
    pub const SPIT_WAIT: usize = 0x2da;
    pub const LUNGE_WAIT: usize = 0x2dc;
    pub const GRAB_WAIT: usize = 0x2de;
    pub const NEAR_FLAG: usize = 0x2e0;
    pub const WALK_SEQ: usize = 0x2e1;
    pub const STAND_SEQ: usize = 0x2e2;
    pub const SWINGS: usize = 0x2e3;
    /// The camera tweak's slot + 1 (the original's pointer).
    pub const TWEAK: usize = 0x2e4;
    pub const HOME_Z: usize = 0x2e8;
    pub const NEAR: usize = 0x2ec;
    pub const AREA: usize = 0x2f0;
    pub const HOLDING: usize = 0x2f4;
    /// The cutaway 0x02e1 played at its death (a moby index) and the cuboid it is put at.
    pub const CUTAWAY: usize = 0x2f8;
    pub const DEATH_AT: usize = 0x2fc;
    /// The reward (a link: index + 1).
    pub const REWARD: usize = 0x300;
    pub const EXIT_AT: usize = 0x304;
    pub const POKE: usize = 0x308;
    pub const MUSIC_FIGHT: usize = 0x310;
    pub const MUSIC_AFTER: usize = 0x314;
    pub const STINGER_FIGHT: usize = 0x318;
    pub const STINGER_AFTER: usize = 0x31c;
}

/// `$gp+0x2a08..+0x2a14` (L01): the reward's turns, degrees a second (x, y, z, the fourth angle).
const REWARD_TURN: [f32; 4] = [120.0, 40.0, 20.0, 90.0];

fn mission_done(g: &GameState, mission: u8) -> bool {
    mission != 0xff && g.missions.get(g.level.max(0) as usize * 0x10 + mission as usize).is_some_and(|b| b & 0x80 != 0)
}

/// A placed moby named by a pvar (−1 none).
fn placed(w: &MobyWorld, i: i32) -> Option<usize> { usize::try_from(i).ok().filter(|&i| i < w.mobys.len()) }

// ---- The transporter 0x0e38 ----

/// 0x0e38 (`0x401540`, module docs).
pub fn update_0e38(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            if mission_done(ctx.game, w.mobys[id].mission) { return w.delete(id); }
            w.mobys[id].set_state(1, -1);
        }
        1 => {
            let m = &w.mobys[id];
            let area = m.pv_i(0);
            let go = m.timer != 0
                || (area != -1 && ctx.level.in_area(area, ctx.hero.pos) && (m.pv_i(0x18) != 0 || ctx.hero.grounded_ticks != 0));
            if !go { return; }
            let m = &mut w.mobys[id];
            m.update_dist = 0xff;
            // `0x2dd378(0x14)`; the rest of the tick waits for its frames (the port's state 2).
            ctx.game.blocking_fade(0x14);
            m.state = 2;
        }
        2 => {
            let m = &w.mobys[id];
            let (at, chunk) = (m.pv_i(0xc), m.pv_i(8));
            if let Some((p, r)) = ctx.level.cuboid_place(at) {
                // `0x2b9b38(.., −1, 1)`: his state kept, the camera reset.
                ctx.hero.teleport(p, r);
                ctx.game.cameras.reset();
            }
            if chunk != -1 { ctx.switch_chunk(chunk, 0); }
            w.mobys[id].set_state(3, -1);
        }
        3 => {
            if ctx.game.scenes.in_scene() { return; }
            let planet = w.mobys[id].pv_s(6) as i32;
            if planet != -1 {
                ctx.game.unlock_planet(planet);
                crate::travel::new_planet_banner(ctx.game, planet);
            }
            w.mobys[id].set_state(4, -1);
        }
        4 => {
            let m = &w.mobys[id];
            let (mission, pos, rot) = (m.mission, m.pos, m.rot);
            if mission != 0xff && !mission_done(ctx.game, mission) {
                ctx.game.mission_done(mission);
                ctx.game.save_checkpoint(pos, rot);
            }
            // `0x2c1ca8(0)` (on foot) and `0x2beea8(0, 1)`.
            ctx.hero.request_state(0, true);
            let m = &w.mobys[id];
            ctx.host.music_request(m.pv_i(0x14), m.pv_i(0x10));
            w.delete(id);
        }
        _ => {}
    }
}

// ---- The boss 0x0a54 ----

/// `0x3c11d0(m, s, 0, blend)`: sequence `s` unless it plays; its speed 2/3 (but for 0xc), 1.25 times that for the
/// lunges and the swing (6..8).
fn anim(w: &MobyWorld, id: usize, ctx: &mut Ctx, s: u8, blend: i32) {
    if seq(w, id, ctx) != s { ctx.host.set_anim(&w.mobys[id], s, 0, blend.clamp(0, 255) as u8); }
    let mut speed = ctx.host.anim(&w.mobys[id]).map_or(1.0, |a| a.speed);
    if s != 0xc { speed = f32::from_bits(0x3f2a_aaab); }
    if (6..=8).contains(&s) { speed *= 1.25; }
    ctx.host.set_anim_speed(&w.mobys[id], speed);
}

/// `0x3235c8(walk accel, walk most, turn accel, turn most, m)` in the tuning's units.
fn limits(w: &mut MobyWorld, id: usize, walk_a: f32, walk_most: f32, turn_a: f32, turn_most: f32) {
    ai::senses(&mut w.mobys[id], walk_a * 0.000_277_777_78, walk_most * 0.016_666_668, turn_a * DEG_S2, turn_most * DEG_S);
}

/// The reaction's fallback sequences (`+0x71` / `+0x72`).
fn react_seqs(w: &mut MobyWorld, id: usize, s: u8) {
    let m = &mut w.mobys[id];
    m.set_pv_b(pv::R_SEQ_B, s);
    m.set_pv_b(pv::R_SEQ_A, s);
}

/// Its target's place (the target block's moby), else Ratchet's.
fn target_pos(w: &MobyWorld, id: usize, ctx: &Ctx) -> V3 {
    ai::link(&w.mobys[id], pv::T_MOBY).and_then(|t| w.mobys.get(t)).map_or(ctx.hero.pos, |t| t.pos)
}

/// `0x323560(yaw, m)`: the facing eased toward `yaw` at the movement block's turn limits (`0x3152d0`).
fn face(w: &mut MobyWorld, id: usize, yaw: f32) {
    let m = &mut w.mobys[id];
    let o = pv::MV;
    let (a, b, most) = (m.pv_f(o + 0x14), m.pv_f(o + 0x18), m.pv_f(o + 0x1c));
    let (mut r, mut v) = (m.rot[2], m.pv_f(o + 0xb4));
    turn_ease(yaw, a, b, most, &mut r, &mut v, 0);
    m.rot[2] = r;
    m.set_pv_f(o + 0xb4, v);
}

fn face_hero(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let (p, h) = (w.mobys[id].pos, ctx.hero.pos);
    face(w, id, atan2(h[0] - p[0], h[1] - p[1]));
}

/// `0x2ccc78`: its camera tweak let go.
fn release_tweak(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let t = w.mobys[id].pv_i(pv::TWEAK);
    if t != 0 {
        ctx.game.cam_tweaks.release((t - 1) as usize);
        w.mobys[id].set_pv_i(pv::TWEAK, 0);
    }
}

/// `0x3c1250`: the set-up (module docs).
fn setup(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    // The reaction block.
    m.set_pv_b(0xd7, 1);
    m.set_pv_b(0x72, 0x13);
    m.set_pv_b(0x76, 0x14);
    m.set_pv_b(0x77, 0xff);
    m.set_pv_i(0x7c, 0x3e6a_aaab);
    m.set_pv_i(0x84, 0x3c61_47ae);
    m.set_pv_i(0x88, 0x3aac_f136);
    m.set_pv_i(0x8c, 0x3b88_8889);
    m.set_pv_f(0x94, 0.5);
    m.set_pv_i(0x9c, 0x3e19_999a);
    m.set_pv_b(0xcc, 6);
    m.set_pv_b(0xcd, 0x19);
    m.set_pv_i(0xdc, m.pv_i(pv::PATHS[0]));
    m.set_pv_b(0x70, 0xff);
    m.set_pv_i(0xa8, 6);
    m.set_pv_b(0x71, 0x13);
    m.set_pv_i(0x80, 0x3e19_999a);
    m.set_pv_f(0x90, 0.5);
    for o in [0xd3, 0xce, 0xcf, 0xd2] { m.set_pv_b(o, 0xff); }
    // The health block (`0x2e52c0`: 40.0 and 28.0 truncated).
    m.set_pv_b(0x48, 40);
    m.set_pv_b(0x28, 4);
    m.set_pv_b(0x4a, 28);
    m.set_pv_b(0x2a, 0x30);
    m.set_pv_b(0x29, 1);
    m.set_pv_b(pv::KILL, 1);
    m.set_pv_f(0x40, 6.0);
    m.set_pv_i(0x44, 0x4053_3333);
    m.set_pv_f(0x30, 2.0);
    m.mode |= mode::TARGET;
    // The movement block.
    let o = pv::MV;
    m.set_pv_i(o + 0x3c, m.pv_i(pv::PATHS[1]));
    m.set_pv_f(o, 4.0);
    m.set_pv_i(o + 4, 0x3eb3_3333);
    m.set_pv_i(o + 0xc, 0x3dcc_cccd);
    m.set_pv_i(o + 0x10, 0x3eb2_b8c4);
    m.set_pv_i(o + 0x30, 0x3f06_0a93);
    m.set_pv_f(o + 0x2c, tune::GRAVITY * 0.000_277_777_78);
    m.set_pv_i(o + 8, 0x3dcc_cccd);
    limits(w, id, tune::WALK_A, tune::WALK_SETUP, tune::TURN_A, tune::TURN_MOST);
    let m = &mut w.mobys[id];
    m.set_pv_b(0x28, 3);
    m.timer = 0;
    m.set_pv_b(pv::WALK_SEQ, 1);
    m.set_pv_b(pv::NEAR_FLAG, 0);
    m.set_pv_b(pv::STAND_SEQ, 0);
    let z = m.pos[2];
    m.set_pv_f(pv::HOME_Z, z);
    m.set_pv_f(pv::NEAR, tune::NEAR);
    m.set_state(0, 1);
    anim(w, id, ctx, 0, 2);
}

/// `0x3c15c8`: the damage, the target, the camera tweak and the boss bar (module docs).
fn checks(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let old = w.mobys[id].state;
    let kind = ai::take_damage(w, id, ctx, false);
    let m = &w.mobys[id];
    let kill = m.pv_b(pv::KILL);
    if (kind != 0 || kill == 2) && !(0.0 < m.pv_f(pv::HP) && kill != 2) {
        w.mobys[id].set_pv_f(pv::HP, 0.0);
        if kill == 2 && w.mobys[id].pv_s(pv::R_OPTS) & 1 == 0 {
            // `0x31df48(m, react, dir, 4)`: its death's reaction (the direction an unset stack vector: straight back).
            let f = w.mobys[id].rows[0];
            ai::react_start(w, id, ctx, [-f[0], -f[1], 1.0, 5_627.987], 4);
            let m = &mut w.mobys[id];
            m.set_pv_b(pv::KILL, 3);
            m.sub_state = 6;
            m.state = 99;
            m.timer = 99;
        }
        let m = &w.mobys[id];
        let (cut, at) = (m.pv_i(pv::CUTAWAY), m.pv_i(pv::DEATH_AT));
        if let Some(c) = placed(w, cut) {
            w.mobys[c].timer = 1;
            if let Some((p, r)) = ctx.level.cuboid_place(at) {
                let m = &mut w.mobys[id];
                m.pos = p;
                m.rot = r;
            }
        }
    }
    if w.mobys[id].state == 0xff { w.mobys[id].state = old; }
    let area = w.mobys[id].pv_i(pv::AREA);
    ai::find_target(w, id, ctx, 0.0, 0.0, area, None, false);
    let m = &w.mobys[id];
    if let Some(slot) = usize::try_from(m.pv_i(pv::TWEAK) - 1).ok() {
        let wide = m.pv_b(pv::HOLDING) != 0 || (m.sub_state == 3 && m.state == 2);
        let dir = sub(m.pos, ctx.hero.pos);
        if let Some(t) = ctx.game.cam_tweaks.slots.get_mut(slot) {
            t.dir = dir;
            t.face_rest = true;
            t.fov = tune::FOV * DEG;
            (t.dist, t.rate_deg) = if wide { (tune::CAM_FAR, 6) } else { (tune::CAM_NEAR, 3) };
        }
    }
    let m = &w.mobys[id];
    if ctx.hero.pos[2] < m.pos[2] + 20.0 && (2..=6).contains(&m.sub_state) {
        // `0x2f20b8(health / most, 2, 0)`.
        let most = m.pv_s(pv::MAX) as f32;
        ctx.game.boss_bar = Some(((m.pv_f(pv::HP) / most).clamp(0.0, 1.0), 5));
        ctx.game.boss_icon = 2;
    }
}

/// `0x3c1548`: its flash, its ticks, its waits.
fn tail(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    ai::flash(m, pv::FLASH);
    if m.state_ticks != -1 { m.state_ticks = m.state_ticks.wrapping_add(1); }
    for o in [pv::GRAB_WAIT, pv::SPIT_WAIT, pv::LUNGE_WAIT] {
        let mut t = m.pv_s(o) as i32;
        crate::moby::countdown(&mut t);
        m.set_pv_s(o, t as i16);
    }
}

/// `0x3c1af0`: a lunge (state 1, sequence 6 or 7).
fn lunge(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let s = (ctx.game.rng.rand() & 1) as u8 + 6;
    let b = ctx.game.rng.rand_range(10, 15);
    anim(w, id, ctx, s, b);
    let m = &mut w.mobys[id];
    m.set_state(1, 3);
    m.set_pv_s(pv::LUNGE_WAIT, tune::LUNGE_WAIT);
}

/// `0x3c1b68`: a swing (state 3, sequence 8).
fn swing(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let b = ctx.game.rng.rand_range(7, 12);
    anim(w, id, ctx, 8, b);
    let m = &mut w.mobys[id];
    m.set_state(3, 3);
    m.set_pv_s(pv::LUNGE_WAIT, tune::LUNGE_WAIT);
}

/// `0x3c1bd0`: a spit (state 2, sequence 0xd).
fn spit(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let b = ctx.game.rng.rand_range(9, 11);
    anim(w, id, ctx, 0xd, b);
    let m = &mut w.mobys[id];
    m.set_state(2, 3);
    m.set_pv_s(pv::SPIT_WAIT, tune::SPIT_WAIT);
}

/// `0x3c1c38`: a grab (state 4, sequence 0xe; out of the collision's 0x30).
fn grab(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let b = ctx.game.rng.rand_range(7, 11);
    anim(w, id, ctx, 0xe, b);
    let m = &mut w.mobys[id];
    m.set_state(4, 3);
    m.coll_disable |= 0x30;
    m.set_pv_s(pv::GRAB_WAIT, tune::GRAB_WAIT);
}

/// The fight's mode near (`+0x2e1` = 1, `+0x2e2` = 0) at the walk's most `walk`, near within `near`.
fn near_mode(w: &mut MobyWorld, id: usize, walk: f32, near: f32, mode: u8) {
    let m = &mut w.mobys[id];
    m.set_pv_b(pv::WALK_SEQ, 1);
    m.set_pv_b(pv::NEAR_FLAG, 1);
    m.set_pv_b(pv::STAND_SEQ, 0);
    react_seqs(w, id, 0x13);
    limits(w, id, tune::WALK_A, walk, tune::TURN_A, tune::TURN_MOST);
    let m = &mut w.mobys[id];
    m.set_pv_f(pv::NEAR, near);
    m.timer = mode;
}

/// Wading off (`+0x2e1` = 4, `+0x2e2` = 0x16) into mode `mode`.
fn wade_mode(w: &mut MobyWorld, id: usize, mode: u8) {
    let m = &mut w.mobys[id];
    m.set_pv_b(pv::WALK_SEQ, 4);
    m.set_pv_b(pv::STAND_SEQ, 0x16);
    m.set_pv_b(pv::NEAR_FLAG, 0);
    let [a, b, c, d] = tune::WADE;
    limits(w, id, a, b, c, d);
    let m = &mut w.mobys[id];
    m.set_pv_f(pv::NEAR, tune::NEAR);
    m.timer = mode;
    react_seqs(w, id, 0xf);
}

/// The roar into the next mode set (sequence 0x10 blended over `blend`, sub-state 5).
fn roar(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, walk: f32, mode: u8, blend: i32) {
    let m = &mut w.mobys[id];
    m.set_pv_b(pv::NEAR_FLAG, 1);
    m.set_pv_b(pv::WALK_SEQ, 5);
    m.set_pv_b(pv::STAND_SEQ, 0xe);
    react_seqs(w, id, 0x13);
    limits(w, id, tune::WALK_A, walk, tune::TURN_A, tune::TURN_MOST);
    let m = &mut w.mobys[id];
    m.set_pv_f(pv::NEAR, tune::NEAR_GRAB);
    m.timer = mode;
    anim(w, id, ctx, 0x10, blend);
    w.mobys[id].set_state(0, 5);
}

/// Back up out of the wade (sequence 0x16, sub-state 5), solid again.
fn surface(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, near: f32, mode: u8) {
    near_mode(w, id, tune::WALK, near, mode);
    w.mobys[id].has_collision = solid(w);
    anim(w, id, ctx, 0x16, 0x14);
    w.mobys[id].set_state(0, 5);
}

/// The class's collision (`+0x98` from the class header).
fn solid(w: &MobyWorld) -> bool { w.class(CLASS).is_some_and(|c| c.has_collision) }

/// `0x3c1810`: the move of the fight (module docs); the distance across to its target, −1000 without one.
fn approach(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> f32 {
    fn stand(w: &MobyWorld, id: usize, ctx: &mut Ctx, s: u8) {
        if seq(w, id, ctx) != s && w.mobys[id].sub_state != 3 { anim(w, id, ctx, s, 0x14); }
    }
    let m = &w.mobys[id];
    let (walk_s, stand_s) = (m.pv_b(pv::WALK_SEQ), m.pv_b(pv::STAND_SEQ));
    let home_z = m.pv_f(pv::HOME_Z);
    if m.pv_i(pv::T_FOUND) != 1 {
        stand(w, id, ctx, stand_s);
        let (p, h) = (w.mobys[id].pos, ctx.hero.pos);
        ai::move_step(w, id, ctx, atan2(h[0] - p[0], h[1] - p[1]), p);
        w.mobys[id].pos[2] = home_z;
        return -1000.0;
    }
    let tp = target_pos(w, id, ctx);
    let p = m.pos;
    let d = dist2(p, tp);
    if m.pv_f(pv::NEAR) < d {
        if seq(w, id, ctx) != walk_s { anim(w, id, ctx, walk_s, 0x14); }
        let c = ctx.level.cuboid_place(w.mobys[id].pv_i(pv::MIDDLE)).map_or(p, |c| c.0);
        let a_m = atan2(p[0] - c[0], p[1] - c[1]);
        let a_t = atan2(tp[0] - c[0], tp[1] - c[1]);
        let delta = sub_rot(a_t, a_m);
        let off = delta.abs();
        let goal = if off < tune::ROUND * DEG {
            tp
        } else {
            let a = add_rot(a_m, (delta / off) * 0.087_266_475);
            [c[0] + a.cos() * 38.0, c[1] + a.sin() * 38.0, c[2]]
        };
        w.mobys[id].set_pv_v(pv::GOAL, goal);
        // `0x322450(m, goal)`.
        ai::move_step(w, id, ctx, atan2(goal[0] - p[0], goal[1] - p[1]), goal);
    } else {
        stand(w, id, ctx, stand_s);
        // `0x322730(yaw, m)`.
        ai::move_step(w, id, ctx, atan2(tp[0] - p[0], tp[1] - p[1]), p);
    }
    w.mobys[id].pos[2] = home_z;
    d
}

/// Sub-state 2: the fight (module docs; `0x3c1db8`'s case 2).
fn fight(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let d = approach(w, id, ctx);
    if d == -1000.0 {
        release_tweak(w, id, ctx);
        w.mobys[id].set_state(0, 1);
        anim(w, id, ctx, 0, 10);
        return;
    }
    let tp = target_pos(w, id, ctx);
    let m = &w.mobys[id];
    let off = rot_dist(m.rot[2], atan2(tp[0] - m.pos[0], tp[1] - m.pos[1]));
    let most = m.pv_s(pv::MAX) as f32;
    let hp = |w: &MobyWorld| w.mobys[id].pv_f(pv::HP);
    let wait = |w: &MobyWorld, o: usize| w.mobys[id].pv_s(o) != 0;
    let solid = solid(w);
    let (timer, ticks) = (m.timer, m.state_ticks as u16);
    match timer {
        0 => {
            let mut ready = tune::PATIENCE < ticks;
            if d <= tune::NEAR && off < tune::FACE * DEG && !wait(w, pv::LUNGE_WAIT) {
                ready = true;
                lunge(w, id, ctx);
            }
            if ready {
                limits(w, id, tune::WALK_A, tune::WALK, tune::TURN_A, tune::TURN_MOST);
                let m = &mut w.mobys[id];
                m.set_pv_f(pv::NEAR, tune::NEAR);
                m.timer = 1;
            }
            if hp(w) < (most + most) * 0.333_333_34 { roar(w, id, ctx, tune::WALK_ROAR, 3, 0xd); }
        }
        1 | 2 => {
            let mut blend = 0;
            if timer == 1 {
                if !(tune::NEAR < d || tune::FACE * DEG <= off || ctx.game.rng.randi(tune::LUNGE_ODDS - 1) != 0 || wait(w, pv::LUNGE_WAIT)) {
                    lunge(w, id, ctx);
                } else if !(d <= tune::NEAR || tune::SPIT_FAR < d || tune::FACE_SPIT * DEG <= off || ctx.game.rng.randi(tune::SPIT_ODDS - 1) != 0 || wait(w, pv::SPIT_WAIT)) {
                    spit(w, id, ctx);
                } else if tune::FAR < d {
                    wade_mode(w, id, 2);
                    anim(w, id, ctx, 3, 0xd);
                    w.mobys[id].set_state(0, 5);
                }
            } else {
                blend = 0x3c;
                w.mobys[id].has_collision = false;
                let s = seq_a(w, id, ctx);
                if d <= tune::NEAR && (s == 0x16 || s == 0) { surface(w, id, ctx, tune::NEAR, 1); }
            }
            if hp(w) < (most + most) * 0.333_333_34 { roar(w, id, ctx, tune::WALK_ROAR, 3, blend + 0xd); }
        }
        3..=6 => {
            match w.mobys[id].timer {
                4 => {
                    if d <= tune::NEAR || tune::NEAR_2 < d || tune::FACE_SWING * DEG <= off || wait(w, pv::LUNGE_WAIT) {
                        if d <= tune::NEAR && off < tune::FACE_SWING * DEG && !wait(w, pv::LUNGE_WAIT) { lunge(w, id, ctx); }
                    } else {
                        // `0x2e3e48`: the swings counted out.
                        let m = &mut w.mobys[id];
                        let n = m.pv_b(pv::SWINGS);
                        let out = if n == 0 { true } else { m.set_pv_b(pv::SWINGS, n - 1); n - 1 == 0 };
                        if out {
                            limits(w, id, tune::WALK_A, tune::WADE[1], tune::TURN_A, tune::TURN_MOST);
                            let m = &mut w.mobys[id];
                            m.timer = 5;
                            m.set_pv_b(pv::NEAR_FLAG, 0);
                            m.set_pv_b(pv::WALK_SEQ, 4);
                            m.set_pv_f(pv::NEAR, tune::NEAR);
                            m.set_pv_b(pv::STAND_SEQ, 0x16);
                            react_seqs(w, id, 0xf);
                            anim(w, id, ctx, 3, 0xd);
                            w.mobys[id].set_state(0, 5);
                        }
                        swing(w, id, ctx);
                    }
                }
                3 => {
                    if d <= tune::NEAR_GRAB && off < tune::FACE_GRAB * DEG && !wait(w, pv::GRAB_WAIT) {
                        near_mode(w, id, tune::WALK_2, tune::NEAR_2, 4);
                        w.mobys[id].set_pv_b(pv::SWINGS, tune::SWINGS);
                        grab(w, id, ctx);
                    }
                }
                5 => {
                    w.mobys[id].has_collision = false;
                    let s = seq_a(w, id, ctx);
                    if d <= tune::NEAR && (s == 0x16 || s == 0) { surface(w, id, ctx, tune::NEAR_2, 6); }
                }
                6 => {
                    w.mobys[id].has_collision = solid;
                    if d <= tune::NEAR_2 && off < tune::FACE * DEG && ctx.game.rng.randi(tune::LUNGE_ODDS - 1) == 0 && !wait(w, pv::LUNGE_WAIT) {
                        if tune::NEAR < d && ctx.game.rng.rand() & 1 != 0 { swing(w, id, ctx) } else { lunge(w, id, ctx) }
                    }
                }
                _ => {}
            }
            let t = w.mobys[id].timer;
            if t != 5 && t < 7 && tune::FAR < d { wade_mode(w, id, 5); }
            if hp(w) < most * 0.333_333_34 { roar(w, id, ctx, tune::WALK_ROAR_3, 7, 0xd); }
        }
        7 => {
            if d <= tune::NEAR_GRAB && off < tune::FACE_GRAB * DEG && !wait(w, pv::GRAB_WAIT) {
                near_mode(w, id, tune::WALK_3, tune::NEAR, 9);
                grab(w, id, ctx);
            }
        }
        8 => {
            w.mobys[id].has_collision = false;
            let s = seq_a(w, id, ctx);
            if d <= tune::NEAR && (s == 0x16 || s == 0) { surface(w, id, ctx, tune::NEAR, 9); }
        }
        9 => {
            w.mobys[id].has_collision = solid;
            if tune::NEAR < d || tune::FACE * DEG <= off || ctx.game.rng.randi(tune::LUNGE_ODDS - 1) != 0 || wait(w, pv::LUNGE_WAIT) {
                if !(d <= tune::NEAR || tune::SPIT_FAR < d || tune::FACE_SPIT * DEG <= off || ctx.game.rng.randi(tune::SPIT_ODDS - 1) != 0 || wait(w, pv::SPIT_WAIT)) {
                    spit(w, id, ctx);
                } else if tune::FAR < d {
                    wade_mode(w, id, 8);
                    anim(w, id, ctx, 3, 0xd);
                    w.mobys[id].set_state(0, 5);
                }
            } else if ctx.game.rng.rand() & 1 == 0 {
                swing(w, id, ctx);
            } else {
                lunge(w, id, ctx);
            }
        }
        _ => {}
    }
    if seq_a(w, id, ctx) == 0x16 && wrapped(w, id, ctx) {
        w.mobys[id].set_pv_b(pv::STAND_SEQ, 0);
        anim(w, id, ctx, 0, 10);
    }
}

/// `0x313080(damage, &t, m, 1)`: a plain hit of the boss's (flags 1, no direction).
fn plain_hit(id: usize, damage: f32) -> HitTemplate {
    let mut t = HitTemplate::new(damage, Some(id), 1, [0.0; 4]);
    t.count = 0;
    t
}

/// `0x3c1ca8`: Ratchet held at its joint 10, 1 lower (1/15 a tick at most), still.
fn hold_hero(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if ctx.hero.state != 0x6f {
        w.mobys[id].has_collision = false;
        ctx.hero.request_state(0x6f, true);
        ctx.hero.anim_req = Some((10.0, 10, 0));
    }
    let Some(j) = ctx.host.joint(&w.mobys[id], 10) else { return };
    let mut d = sub([j[0], j[1], j[2] - 1.0], ctx.hero.pos);
    // `0x317418(1/15, d)`.
    let most = f32::from_bits(0x3d88_8889);
    if most < len(d) { d = set_len(d, most); }
    let h = &mut *ctx.hero;
    h.pos = add(h.pos, d);
    h.vel = [0.0; 3];
    h.disp = [0.0; 3];
    h.eff = [0.0; 3];
    h.eff_v = [0.0; 3];
    h.hot_hops = 0;
    h.surf.pit = false;
    h.surf.sand = false;
    h.timer = 0;
}

/// Sub-state 3: the attack (module docs; `0x3c1db8`'s case 3).
fn attack(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let f = ctx.host.frame_time(&w.mobys[id]);
    match w.mobys[id].state {
        2 => {
            face_hero(w, id, ctx);
            if ai::reached(w, id, ctx, 16.0) {
                if let Some(from) = ctx.host.joint(&w.mobys[id], 10) {
                    let tp = target_pos(w, id, ctx);
                    let speed = tune::SPIT_SPEED * 0.016_666_668;
                    let mut v = set_len(sub(tp, from), speed);
                    // `0x3140e8(speed, g, from, to, 0)`.
                    let t = dist2(tp, from) / speed;
                    v[2] = if t != 0.0 { -((from[2] - tp[2]) + -(tune::SPIT_G * 0.000_277_777_78) * t * t * 0.5) / t } else { 0.0 };
                    let dmg = w.mobys[id].pv_b(pv::DMG_BITE) as f32;
                    make_ball(w, ctx, dmg, from, v, id);
                }
            }
        }
        1 => {
            face_hero(w, id, ctx);
            if 44.0 < f && f < 68.0 {
                let base = match seq_a(w, id, ctx) { 6 => Some(0), 7 => Some(4), _ => None };
                if let Some(base) = base {
                    let dmg = w.mobys[id].pv_b(pv::DMG_BITE) as f32;
                    let mut a = ctx.host.joint(&w.mobys[id], base).unwrap_or(w.mobys[id].pos);
                    for k in 1..=2 {
                        let t = plain_hit(id, dmg);
                        let b = a;
                        a = ctx.host.joint(&w.mobys[id], base + k).unwrap_or(a);
                        // `0x2d6c20(0.5, a, 0, m, &t)`.
                        w.sphere_hit(ctx.scene, a, 0.5, 0, Some(id), Some(&t));
                        for x in a.iter_mut() { *x += ctx.game.rng.randf(-0.3, 0.3); }
                        // `0x2d3fe8(b, a, 9, m, &t)`.
                        let hit = ctx.coll.and_then(|c| crate::collision::coll_line_m(c, ctx.scene, b, a, crate::collision::QueryFlags(9), Some(id)));
                        if let Some(o) = hit {
                            if let Some(target) = o.moby { w.deliver_hit_any(target, &t, [o.point[0], o.point[1], o.point[2], 1.0]); }
                        }
                    }
                }
            }
        }
        3 => {
            if 34.0 < f && f < 41.0 {
                let j = (ctx.game.rng.rand() & 1) as usize | 8;
                let dmg = w.mobys[id].pv_b(pv::DMG_BITE) as f32;
                ai::attack_joint(w, id, ctx, f32::from_bits(0x400c_cccd), dmg, 1.0, j, 1, 0, 1, 0);
            }
        }
        4 => {
            face_hero(w, id, ctx);
            if w.mobys[id].pv_b(pv::HOLDING) == 0 {
                if 12.0 < f && f <= 17.0 {
                    if let Some(j) = ctx.host.joint(&w.mobys[id], 0xc) {
                        let j = [j[0], j[1], j[2] - 2.4];
                        if dist(j, ctx.hero.pos) < tune::GRAB_REACH && ctx.hero.state != 0x6f { w.mobys[id].set_pv_b(pv::HOLDING, 1); }
                    }
                } else if 17.0 < f && f < 19.0 {
                    w.mobys[id].has_collision = solid(w);
                    let j = (ctx.game.rng.rand() & 1) as usize | 0xc;
                    let dmg = w.mobys[id].pv_b(pv::DMG_SWIPE) as f32;
                    ai::attack_joint(w, id, ctx, 2.0, dmg, 1.0, j, 1, 0, 1, 0);
                }
            }
        }
        _ => {}
    }
    if w.mobys[id].pv_b(pv::HOLDING) == 0 {
        if wrapped(w, id, ctx) {
            w.mobys[id].coll_disable &= !0x30;
            w.mobys[id].set_state(0, 2);
            anim(w, id, ctx, 0, 0x14);
        }
        return;
    }
    hold_hero(w, id, ctx);
    if !wrapped(w, id, ctx) { return; }
    if ctx.hero.hp < 2 {
        ctx.hero.die();
    } else if let Some(h) = ctx.hero_moby {
        // `0x313400(Ratchet, &t)`.
        let t = plain_hit(id, w.mobys[id].pv_b(pv::DMG_THROW) as f32);
        let p = w.mobys[id].pos;
        w.deliver_hit_any(h, &t, [p[0], p[1], p[2], 1.0]);
    }
    anim(w, id, ctx, 0xb, 0x14);
    w.mobys[id].set_state(0, 4);
}

/// Sub-state 4: the throw (module docs; `0x3c1db8`'s case 4).
fn throw(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let f = ctx.host.frame_time(&w.mobys[id]);
    let m = &w.mobys[id];
    let c = ctx.level.cuboid_place(m.pv_i(pv::MIDDLE)).map_or(m.pos, |c| c.0);
    let a = atan2(c[0] - m.pos[0], c[1] - m.pos[1]);
    let half = f32::from_bits(0x3fc9_0fdc);
    let (y1, y2) = (add_rot(a, half), sub_rot(a, half));
    let yaw = if rot_dist(m.rot[2], y2) <= rot_dist(m.rot[2], y1) { y2 } else { y1 };
    face(w, id, yaw);
    if seq_a(w, id, ctx) != 0xb { return; }
    let Some(j) = ctx.host.joint(&w.mobys[id], 0xe) else { return };
    let dj = dist(j, ctx.hero.pos);
    let far = 13.0 < dj;
    if w.mobys[id].pv_b(pv::HOLDING) != 0 {
        if 33.0 < f {
            // Let go: `0x2beea8(0x16, 1)` (the original then writes 0x6f / 9 into the state words itself).
            ctx.hero.request_state(0x16, true);
            w.mobys[id].set_pv_b(pv::HOLDING, 0);
        } else {
            hold_hero(w, id, ctx);
        }
    }
    let solid = solid(w);
    if w.mobys[id].pv_b(pv::HOLDING) == 0 && !far {
        let fwd = w.mobys[id].rows[0];
        let v = set_len(set_len(fwd, tune::THROW_REACH), tune::THROW_SPEED * 0.016_666_668);
        let s = 1.0 - dj * 0.006_666_667;
        let v = [v[0], v[1], len2(v) * (s * s) * 0.7];
        let h = &mut *ctx.hero;
        h.vel = v;
        h.disp = v;
        h.pos = add(h.pos, v);
    } else if far {
        let m = &mut w.mobys[id];
        m.coll_disable &= !0x30;
        m.has_collision = solid;
        if ctx.hero.state == 0x6f { ctx.hero.request_state(0x16, true); }
    }
    if wrapped(w, id, ctx) {
        let m = &mut w.mobys[id];
        m.coll_disable &= !0x30;
        m.has_collision = solid;
        anim(w, id, ctx, 0xc, 0x14);
        w.mobys[id].set_state(0, 5);
        if ctx.hero.state == 0x6f { ctx.hero.request_state(0x16, true); }
    }
}

/// `0x418748(pos, 1, −1, −1)`: the reward made at `at`: drawn (0x5a, updated within 0x40), its show's place and yaw,
/// four random angles, its turns, state 1 (`0x4180b0`: its rings' four random phases).
fn make_reward(w: &mut MobyWorld, ctx: &mut Ctx, at: V3) -> usize {
    let r = w.create(REWARD);
    let ambient = ctx.hero_moby.and_then(|h| w.mobys.get(h)).map(|h| (h.light, h.ambient));
    let m = &mut w.mobys[r];
    if let Some((light, amb)) = ambient { (m.light, m.ambient) = (light, amb); }
    m.pos = at;
    m.draw_dist = 0x5a;
    m.update_dist = 0x40;
    m.drawn = 1;
    m.set_pv_i(0, 1);
    m.set_pv_v(0x10, at);
    m.set_pv_f(0x1c, m.rot[2]);
    let rng = &mut ctx.game.rng;
    let a: [f32; 4] = std::array::from_fn(|_| rng.rand_angle());
    m.rot = [a[0], a[1], a[2]];
    m.set_pv_f(0x7c, a[3]);
    m.set_pv_i(8, -1);
    m.set_pv_i(4, -1);
    m.set_pv_f(0x60, REWARD_TURN[2] * DEG_S);
    m.set_pv_f(0x68, REWARD_TURN[0] * DEG_S);
    m.set_pv_f(0x64, REWARD_TURN[1] * DEG_S);
    m.state = 1;
    for k in 0..4 {
        let phase = ctx.game.rng.randi(0xff) as f32;
        w.mobys[r].set_pv_f(0x20 + 4 * k, phase);
    }
    r
}

/// Sub-states 7..9: the death (module docs).
fn death(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    match w.mobys[id].sub_state {
        7 => {
            ctx.host.set_anim_speed(&w.mobys[id], 0.0);
            w.mobys[id].mode |= 0x41;
            release_tweak(w, id, ctx);
            // The port's guard: a level without the reward's class finishes at once (the original would wait).
            if w.class(REWARD).is_none() { return finish(w, id, ctx); }
            let at = w.mobys[id].pos;
            let r = make_reward(w, ctx, at);
            ai::set_link(&mut w.mobys[id], pv::REWARD, Some(r));
            ctx.hero.request_state(0x6f, true);
            w.mobys[r].pos[2] -= 0.5;
            let h = ctx.hero.pos;
            let m = &mut w.mobys[id];
            let mut g = sub(m.pos, h);
            g[2] += 1.0;
            m.set_pv_v(pv::GOAL, scale(g, f32::from_bits(0x3f59_999a)));
            m.set_pv_f(pv::SHARE, 1.0);
            m.set_state(0, 8);
            false
        }
        8 => {
            let t = w.mobys[id].pv_f(pv::SHARE) - 0.006_25;
            w.mobys[id].set_pv_f(pv::SHARE, t);
            let cut = placed(w, w.mobys[id].pv_i(pv::CUTAWAY));
            if let Some((p, r)) = cut.and_then(|c| ctx.level.cuboid_place(w.mobys[c].pv_i(0x28))) {
                ctx.hero.pos = p;
                ctx.hero.rot = r;
            }
            let reward = ai::link(&w.mobys[id], pv::REWARD).filter(|&r| w.mobys.get(r).is_some_and(|x| x.o_class == REWARD));
            if let Some(r) = reward.filter(|&r| w.mobys[r].state < 2) {
                if let Some(c) = cut.filter(|&c| w.mobys[c].timer != 0) {
                    w.mobys[c].timer = 0;
                    super::oozla::update_02e1(w, c, ctx);
                }
                let path = w.mobys[id].pv_v(pv::GOAL);
                let h = ctx.hero.pos;
                let mut p = set_len(path, t * len(path));
                p[2] += (t * std::f32::consts::PI).sin() * 4.0;
                let me = w.mobys[id].pos;
                let rm = &mut w.mobys[r];
                rm.pos = add(p, h);
                let mut home = set_len(sub(me, h), 2.5);
                let yaw = add_rot(atan2(home[0], home[1]), std::f32::consts::PI);
                home = add(home, h);
                rm.set_pv_v(0x10, home);
                rm.set_pv_f(0x1c, yaw);
                w.changed = true;
            }
            let gone = reward.is_none_or(|r| matches!(w.mobys[r].state, 0xfe | 0xfd));
            if gone { return finish(w, id, ctx); }
            false
        }
        9 => leave(w, id, ctx),
        _ => false,
    }
}

/// The reward taken: its bolts and mission, then out of the arena (behind the fade, the port's sub-state 9) or
/// straight to the end. Returns true when it has gone.
fn finish(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    crate::moby::bolts::drop(w, id, 0, 3, ctx);
    let mission = w.mobys[id].mission;
    ctx.game.mission_done(mission);
    if w.mobys[id].pv_i(pv::EXIT_AT) != -1 {
        // `0x2dd378(0x10)`: the rest after its frames.
        ctx.game.blocking_fade(0x10);
        w.mobys[id].set_state(0, 9);
        return false;
    }
    end(w, id, ctx)
}

/// Sub-state 9 (the port's): chunk 0, Ratchet at the cuboid `+0x304` idle with the camera reset, the checkpoint.
fn leave(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    ctx.switch_chunk(0, 0);
    if let Some((p, r)) = ctx.level.cuboid_place(w.mobys[id].pv_i(pv::EXIT_AT)) {
        // `0x2b9b38(.., 0, 1)`.
        ctx.hero.teleport(p, r);
        ctx.hero.request_state(0, false);
        ctx.game.cameras.reset();
    }
    let (p, r) = (ctx.hero.pos, ctx.hero.rot);
    ctx.game.save_checkpoint(p, r);
    end(w, id, ctx)
}

/// The end: the moby `+0x308` poked, the music after the fight, gone.
fn end(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    let m = &w.mobys[id];
    if let Some(p) = placed(w, m.pv_i(pv::POKE)) { w.mobys[p].timer = 1; }
    let m = &w.mobys[id];
    ctx.host.music_request(m.pv_i(pv::MUSIC_AFTER), m.pv_i(pv::STINGER_AFTER));
    w.delete(id);
    true
}

/// 0x0a54 (`0x3c1db8`, module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    checks(w, id, ctx);
    match w.mobys[id].sub_state {
        0 => {
            let m = &w.mobys[id];
            if mission_done(ctx.game, m.mission) {
                if let Some(p) = placed(w, m.pv_i(pv::POKE)) { w.mobys[p].timer = 1; }
                return w.delete(id);
            }
            setup(w, id, ctx);
        }
        1 => {
            let m = &w.mobys[id];
            if m.pv_i(pv::T_FOUND) == 1 {
                if m.pv_i(pv::TWEAK) == 0 {
                    ctx.host.music_request(m.pv_i(pv::MUSIC_FIGHT), m.pv_i(pv::STINGER_FIGHT));
                    if let Some(s) = ctx.game.cam_tweaks.request(5) { w.mobys[id].set_pv_i(pv::TWEAK, s as i32 + 1); }
                }
                w.mobys[id].set_state(0, 2);
            }
        }
        2 => fight(w, id, ctx),
        3 => attack(w, id, ctx),
        4 => throw(w, id, ctx),
        5 => {
            if wrapped(w, id, ctx) {
                w.mobys[id].set_state(0, 2);
                let s = w.mobys[id].pv_b(pv::WALK_SEQ);
                anim(w, id, ctx, s, 0x14);
                w.clear_hit(id);
            }
        }
        6 => {
            let res = ai::flight(w, id, ctx);
            if res & 1 != 0 {
                w.mobys[id].set_state(0, 7);
            } else if res & 2 != 0 {
                let s = w.mobys[id].pv_b(pv::WALK_SEQ);
                anim(w, id, ctx, s, 0xc);
                w.mobys[id].set_state(0, 2);
            }
        }
        7..=9 => if death(w, id, ctx) { return; },
        _ => {}
    }
    // Ratchet drowned near it (`0x3bc3b8`: the swamp's water).
    let m = &w.mobys[id];
    let h = ctx.hero.pos;
    if m.sub_state != 0 && h[2] < ctx.game.water_z - 1.8 && h[2] < m.pos[2] - 1.0 && dist2(m.pos, h) < 10.0 { ctx.hero.die(); }
    tail(w, id);
}

// ---- The spit ball 0x0b24 ----

/// `0x3db7a8(damage, from, velocity, owner)`: a ball at `from` (updated and drawn at any distance, three random angles,
/// 0.2 its class's scale), its velocity `[0]`, its owner `[4]` (a link), its damage `[6]`.
fn make_ball(w: &mut MobyWorld, ctx: &mut Ctx, damage: f32, from: V3, v: V3, owner: usize) {
    let b = w.create(BALL);
    let class_scale = w.class(BALL).map_or(1.0, |c| c.scale);
    let rng = &mut ctx.game.rng;
    let (z, y, x) = (rng.rand_angle(), rng.rand_angle(), rng.rand_angle());
    let m = &mut w.mobys[b];
    m.state = 0;
    m.update_dist = 0xff;
    m.draw_dist = 0xff;
    m.drawn = 1;
    m.rot = [x, y, z];
    m.scale = class_scale * f32::from_bits(0x3e4c_cccd);
    ai::set_link(m, 0x10, Some(owner));
    m.set_pv_f(0x18, damage);
    m.pos = from;
    m.set_pv_v(0, v);
    if seq(w, b, ctx) != 0 { ctx.host.set_anim(&w.mobys[b], 0, 0, 10); }
}

/// `0x3132c8(damage, push, src, from, to, 1, 0, 1, 0)`: a hit along the line, from moby `src` (its facing × `push`,
/// up 1), to the first moby it meets (flags 9, not `src`).
fn line_hit(w: &mut MobyWorld, ctx: &Ctx, src: usize, damage: f32, push: f32, from: V3, to: V3) {
    let Some(m) = w.mobys.get(src) else { return };
    let d = if m.mode & mode::KEEP_ROWS == 0 { [m.rot[2].cos(), m.rot[2].sin(), 0.0] } else { m.rows[0] };
    let mut t = HitTemplate::new(damage, Some(src), 1, [d[0] * push, d[1] * push, 1.0, 5_627.987]);
    t.kind = (m.o_class as u16 as u32) << 16 | 1 << 8;
    let hit = ctx.coll.and_then(|c| crate::collision::coll_line_m(c, ctx.scene, from, to, crate::collision::QueryFlags(9), Some(src)));
    if let Some(o) = hit {
        if let Some(target) = o.moby { w.deliver_hit_any(target, &t, [o.point[0], o.point[1], o.point[2], 1.0]); }
    }
}

/// 0x0b24 (`0x3dc040`, module docs).
pub fn update_ball(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let class_scale = w.class(BALL).map_or(1.0, |c| c.scale);
    let m = &mut w.mobys[id];
    m.scale += (class_scale * f32::from_bits(0x3fcc_cccd) - m.scale) * f32::from_bits(0x3ccc_cccd);
    let v = m.pv_v(0);
    m.pos = add(m.pos, v);
    w.changed = true;
    if wrapped(w, id, ctx) && seq(w, id, ctx) != 1 { ctx.host.set_anim(&w.mobys[id], 1, 0, 10); }
    let water = ctx.game.water_z;
    let m = &w.mobys[id];
    let owner = ai::link(m, 0x10);
    let pos = m.pos;
    let r = f32::from_bits(0x3fcc_cccd);
    let touch = ctx.coll.and_then(|c| crate::collision::coll_sphere_m(c, ctx.scene, pos, r, crate::collision::QueryFlags(0), owner));
    let touched = touch.is_some_and(|o| o.moby.and_then(|t| w.mobys.get(t)).is_none_or(|t| t.o_class != COMPANION));
    if touched || pos[2] < water {
        // `0x3db898`: its spray (not ported) and `0x3165d8(2, damage, 4, 2, 9, 1.2, 0, m, …, 4, 0, 0, −1, 0, 0, −1, 1,
        // 0x810001)`, the general explosion (crate::moby::blast::explode): a sphere of 2 hits for its damage.
        let e = crate::moby::blast::Explosion {
            radius: 2.0, damage: m.pv_f(0x18), flags: 0x81_0001, flash: 4.0, flash2: 2.0, near: 9.0, size: 1.2,
            embers: 4, sound: -1, lod: -1, colour: 1, ..Default::default()
        };
        crate::moby::blast::explode(w, ctx, Some(id), None, &e);
        if let Some(h) = ctx.hero_moby {
            let tiny = f32::from_bits(0x38d1_b717);
            line_hit(w, ctx, h, tiny, 1.0, pos, [pos[0], pos[1], pos[2] - 6.0]);
            let mut v = set_len(w.mobys[id].pv_v(0), r);
            v[2] = 0.0;
            let (a, b) = (add(pos, v), add([pos[0], pos[1], pos[2] - 6.0], v));
            w.mobys[id].pos = a;
            line_hit(w, ctx, h, tiny, 1.0, a, b);
        }
        return w.delete(id);
    }
    let m = &mut w.mobys[id];
    let vz = m.pv_f(8) - f32::from_bits(0x411c_cccd) * 0.000_277_777_78;
    m.set_pv_f(8, vz);
    if pos.iter().all(|&x| (2.0..=1021.0).contains(&x)) {
        // `0x3139b8(scale / class scale, m)`: its blob shadow.
        let size = w.mobys[id].scale / class_scale;
        crate::shadows::request(w, ctx, size, id);
        return;
    }
    w.delete(id);
}

// ---- The reward 0x1250 ----

/// 0x1250 (`0x4179c0`, module docs).
pub fn update_reward(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            if ctx.game.items.get(ITEM).is_some_and(|&b| b != 0) { return w.delete(id); }
            let rng = &mut ctx.game.rng;
            let a: [f32; 4] = std::array::from_fn(|_| rng.rand_angle());
            let m = &mut w.mobys[id];
            let (pos, yaw) = (m.pos, m.rot[2]);
            m.set_pv_v(0x10, pos);
            m.set_pv_f(0x1c, yaw);
            m.rot = [a[0], a[1], a[2]];
            m.set_pv_f(0x7c, a[3]);
            m.set_pv_f(0x68, REWARD_TURN[0] * DEG_S);
            m.set_pv_f(0x64, REWARD_TURN[1] * DEG_S);
            m.set_pv_f(0x60, REWARD_TURN[2] * DEG_S);
            if m.draw_dist < 0x40 { m.draw_dist = 0x40; }
            m.state = 1;
            for k in 0..4 {
                let phase = ctx.game.rng.randi(0xff) as f32;
                w.mobys[id].set_pv_f(0x20 + 4 * k, phase);
            }
        }
        1 => {
            let class_scale = w.class(REWARD).map_or(1.0, |c| c.scale);
            let m = &mut w.mobys[id];
            m.scale = class_scale;
            for (r, t) in m.rot.iter_mut().zip(REWARD_TURN) { *r = add_rot(*r, t * DEG_S); }
            let fourth = add_rot(m.pv_f(0x7c), REWARD_TURN[3] * DEG_S);
            m.set_pv_f(0x7c, fourth);
            let h = &*ctx.hero;
            if 3.0 <= dist2(m.pos, h.pos) || 2.0 <= (m.pos[2] - h.pos[2]).abs() || h.hp == 0 || !matches!(h.mode, 0 | 3) { return; }
            if m.pv_s(0xc) == 0 { return super::platinum::show(w, id, ctx); }
            ctx.game.messages.push(crate::msgbox::Request::Text { text: 0x3315, ticks: None });
            ctx.game.get_item(ITEM as i32);
            ctx.host.play_sound(&w.mobys[id], 1, 0);
            w.delete(id);
        }
        2 => {
            let (hp, hr) = (ctx.hero.pos, ctx.hero.rot);
            let a = ctx.host.anim(&w.mobys[id]);
            let playing = a.is_some_and(|a| a.seq_a == 1);
            super::platinum::camera(w, id, ctx, playing);
            let m = &mut w.mobys[id];
            (m.pos, m.rot) = (hp, hr);
            if playing {
                let class_scale = w.class(REWARD).map_or(1.0, |c| c.scale);
                w.mobys[id].scale = class_scale;
                if a.is_some_and(|a| a.flags & 2 != 0) {
                    if a.is_some_and(|a| a.seq_b != 0) { ctx.host.set_anim(&w.mobys[id], 0, 0, 0); }
                    let m = &mut w.mobys[id];
                    m.drawn = 0;
                    m.mode |= 1;
                }
            }
            if ctx.hero_view.flags & 2 != 0 { ctx.hero.anim_req = Some((30.0, 0, 0)); }
            if a.is_some_and(|a| a.seq_a != 0) { return; }
            let mut t = w.mobys[id].pv_i(0x6c);
            let out = crate::moby::countdown(&mut t);
            w.mobys[id].set_pv_i(0x6c, t);
            if !out { return; }
            ctx.game.messages.push(crate::msgbox::Request::Text { text: 0x129a, ticks: None });
            ctx.game.get_item(ITEM as i32);
            // `0x2b9b38(Ratchet, .., 0, 0)`: idle where he is; `0x42be78(0)`: the camera behind him.
            let (p, r) = (ctx.hero.pos, ctx.hero.rot);
            ctx.hero.teleport(p, r);
            ctx.hero.request_state(0, false);
            ctx.game.cameras.camera_script2(crate::script_camera::RELEASE_CUT_BEHIND, ctx.hero);
            w.delete(id);
        }
        _ => {}
    }
}
