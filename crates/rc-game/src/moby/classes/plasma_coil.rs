//! The Plasma Coil (item 0x1c, crate::hero::items::ROWS; L00's record `0x263d80`: attachment 0, stance 1, use
//! sequence 0x38 kept, icon 0xea71, name 0x32f5, the ball's speed 15 (`+0x40`, read but overridden), damage 18 (`+0x60`)
//! and 12 an arc (`+0x64`), 15 ammo, its upgrade row 0x3e): the hand moby **0x0c4a** (`0x3e9608`, called by the item
//! pass while it is the hand item; no Ghidra function there, read from the assembly) and the plasma ball it fires
//! **0x0a9b** (`0x3cbb78`, made by `0x3ccdc8`).
//!
//! **The hand** (pvars: `+0x00` the cooldown (seconds), `+0x04` its trail slot, `+0x08` the slot's memory, `+0x10` the
//! coil's spin, `+0x14` its angle, `+0x20` the joint node on list 0 whose rotation `+0x30` spins the coil). State **0**:
//! past its draw-out (sequence B not 0, or the sequence wrapped) its sequence 1; the node (`0x312480`); the cooldown
//! 22 ticks (0.3667 s), the spin 0, state **1**; the phase (`$gp+0x1690`) 0; the class's trail slot
//! (crate::moby::classes::chopper::Pool, `0x32b1b8`); the four arcs drawn about the hand ([`HandArc`], `0x24f740`)
//! drawn anew; the balls' table cleared (`0x3cd028`). **1**, each tick: Ratchet in the look stance 1 goes to its twin
//! 0x1d; in 0x1d the look wait (`0x18c354`) counts down and the screen-centre crosshair shows (`0x2db498`, FX 0x26: red
//! `0x600000ff` while the cooldown runs with ammo left, else green `0x600fff0f`), else the wait is 0x28. The aim
//! (`0x3260c0`, crate::moby::aim::search, crates too): from 0.75 up Ratchet's up row along his way (`0x18a580`), 20 far,
//! 10° by 10° (45° by 45° near him); ready with ammo left, its pick shows the green reticle (FX 0x26, `0xff0fff0f`,
//! `|0.9 − d·0.0067·0.65|` large at the camera's distance d) on its aim point. The cooldown runs down; out, the items
//! not kept away, ○ held, the wait out, fewer than 4 balls out and in the hand past 0x16 ticks: one ammo — taken, the
//! class sound 2 and the latch (`0x1b2af0`); the phase from 0 (the spin to 0.1) or 2 goes to **spinning up** (1) and
//! the cooldown to 0.5 — none left: ○ pressed or the latch set, the class sound 7 and the latch cleared. The phases:
//! idle (0) the spin rises 0.1 a tick to 0.1; spinning up (1) it rises 0.1 a tick and past 1 the ball goes — from
//! joint 0 along Ratchet's way (the arm raised at a wall: the gun's −row 1), turned toward a moby at the muzzle
//! (`0x2e14b0(1)`: a target, not Ratchet, not 0xc87 / 0xecc) and then toward the aim's pick, through [`spawn`] —
//! and the phase goes to **spinning down** (2): the spin falls 0.01 a tick back to 0.1, then idle. Drawn, the hand's
//! draw is asked for. The coil turns −spin·π/10 a tick about its joint's y (the node's rotation `(0, sin a/2, 0,
//! cos a/2)`).
//!
//! **The ball** (pvars: `+0x00` its velocity, `+0x10` its timer, `+0x14` its index in the table `0x247240`, `+0x18` its
//! size (2.5), `+0x1c` its arcs out, `+0x20` its loop sound, `+0x38` the moby it struck, `+0x3c` the hand's trail slot,
//! `+0x40` the hand's class). Made ([`spawn`], `0x3ccdc8`; none with 4 out): at the muzzle, flying 1 a tick, 160
//! ticks, updated and drawn at any distance; the first ball after a level's load clears the rays and arcs (`$gp+0x10fc`,
//! the trail slot's memory); 20 rays started ([`start_ray`], `0x3cc980`). Each tick ([`update_ball`]):
//! * **1**: it moves; a moby within 0.45 (`0x2e14b0`) that is a target (mode 0x1000), not the hand's class nor Ratchet,
//!   is struck: the ball on its aim point, the row's damage (flags 0x210000, along its way at 1/30, kind `0x0a9b << 16 |
//!   0x0301`), sparks there ([`sparks`], `0x3ce060(10, 1)`), **2** for 15 ticks; else its step meets the world (the line,
//!   flags 2, then a sphere of 0.1): 0.1 out from the face, sparks (`0x3ce060(15, 0)`), **2** for 15 ticks. Its timer
//!   out, **4**.
//! * **2**: it rides the struck moby's aim point; its timer out, **4**, and the search (crate::moby::aim::search_list:
//!   all round, 10 far, no crates, 3 at most, not the struck moby, not the classes 0 / 0x0a9b) — each moby found takes
//!   five arcs ([`start_arc`], `0x3cc1f0`, 10 ticks to reach it) and the class sound 1.
//! * **4**: on a struck moby (its class sound 3 the first time in the level, `$gp+0x10f4`, never cleared), riding it;
//!   with no arcs out it is gone (`0x3cd060`) — over no moby, sparks (`0x3ce060(10, 1)`) and the class sound 2.
//! Its loop sound (class sound 0) plays while not 4. Then its arcs ([`step_arcs`], `0x3cc4c0`) and rays
//! ([`step_rays`], `0x3cca98`) step; its trail slot still its hand's (`0x32b388`), its draw is asked for, else it is
//! deleted (the count left as it is: the hand's next draw-out clears it).
//!
//! **An arc** (12 in the slot's memory, `0xa0` bytes): from the ball to its moby's aim point, growing to it over 10
//! ticks while its wobble turns 100 times as fast; reached, the moby (with a collision) takes the row's `+0x64` once
//! (flags 0x210000, along it at 1/30) and sparks (`0x3ce060(10, 1)`); it lasts 20 ticks more. Its two control points
//! wobble (three angles each, their speeds 0.001..0.05 either way, amplitudes 3.25..6.75 and 1.25..4.75).
//! **A ray** (64, `0x30` bytes): a way from the ball (length 0.7..0.9, a fresh unit way on renewal), 20..50 ticks, its
//! texture scrolling 0.01..0.075 a tick.
//!
//! **The ball's draw** (`0x3cd2d8`): each arc ([`draw_arc`], `0x3cd940`) — 24 points along the cubic between its ends
//! and control points, jittered 0.075, the last extrapolated, as a strip ([`strip`], `0x3cdb88`: FX 0x2b `0xafafaf`
//! 0.02..0.1 across and `0xff0000` 0.15..0.3 across at a third, fixed alpha 0..0x40 in its last 10 ticks, else
//! 0x40..0x80) and, reached, two sprites at its end (FX 0x31: 0.5 `0x103080`, 0.3 white, alpha 0x50..0x60); each ray
//! ([`draw_ray`], `0x3ccc50`: FX 0x2f in `0x800000` and FX 0x30 in white over it, the ball's size / 4 across at the ball
//! and half at its end, alpha pulsing over its life); then four sprites on the ball (`0x3cd478`: FX 0x31 0.5·size
//! `0x103080` and 0.3·size white at 0x50..0x60, FX 0xb 0.7·size `0x803010` at 8..0x16 and 0.45·size at 0x20..0x40).
//!
//! **The hand's draw** (`0x3e9f18`, [`draw_hand`]): its four arcs' wobble steps (`0x3eaae0`); five points about its
//! joints (`0x29ade0` turned by joint 0's frame: the ball point at −0.4 wobbling, `0x3eb560`) — glints at each (`0x3eaf80`:
//! FX 0x2c sprites, 0.08 toward the camera), the ball point's `0x7f00397f` / `0x7f0072ff` sized by the spin; a
//! lightning strip ([`hand_strip`], `0x3eac00` → `0x3eb010`: 10 points along the cubic jittered 0.04, FX 0x2b 0.0125
//! across and FX 0xe 0.053 across, fixed alpha 0x80 / 0x2a) from joint 0's point to the ball point, from each of the
//! three prongs' points to a point beside the ball point (0.45·spin across) and from joints 1..3 to the prongs' points;
//! one time in 11 a pair of twinkles at the points beside the ball (spinning past 0.1) and at one of the four points
//! ([`spark`], `0x3eb608`, still: its velocity is cleared before use).
//!
//! Its glow in the hand (`0x2a7ed8`'s case 0x1c, crate::hero::items): its word pulses `0xff202020` .. `0xffefefef`.
//!
//! Not ported: the gravity frames' trackers and bends (`0x37f538` / `0x37f618` / `0x37f5f8`, `0x380e08`), the lock
//! cheat (`0x18b270`, `0x32b4d0`), the weapon mods' effects on a hit (`0x3f9f70`), the loop sound's random pitch
//! (`0x360a10`; its random draw is kept), the GS test register the draws set (`0x37c948`), the ball point's spring
//! (`0x31ec88`: the update resets it every tick, so the draw always snaps), the words `$gp+0x1694` / `$gp+0x1698` the
//! shot writes and nothing reads, the strips' fixed alpha (ALPHA 0x68) as the vertex alpha of an additive quad (the
//! texture's alpha is not left out). The upgraded Plasma Coil, the Plasma Storm (rows 0x3e / 0x65 / 0x66), is
//! crate::moby::classes::plasma_storm; it shares the hand's twinkles ([`spark`]) and strips.
//! The second strip of `0x3eb010` takes its second corner's colour from the stack (the store meant for it goes to
//! the first strip's): here the first corner's.

use crate::hero::items::{row, ItemRow, PLASMA_COIL};
use crate::hero::physics::{add, add_rot, atan2, cross, dist, dist2, scale, set_len, sub, sub_rot, unit, V3};
use crate::moby::hits::HitTemplate;
use crate::moby::{aim, countdown, mode, Ctx, MobyWorld};
use rc_formats::moby_anim::JointModifier;

/// The hand's class (item 0x1c's row `+0x04`) and the ball's.
pub const HAND_CLASS: i16 = 0xc4a;
pub const BALL: i16 = 0xa9b;
/// `0x3cd018`: the balls out at most.
const MAX_BALLS: i32 = 4;
/// The rays and arcs in the trail slot's memory.
const RAYS: usize = 0x40;
const ARCS: usize = 0xc;
/// The crosshair's and the reticle's FX texture.
const RETICLE: u16 = 0x26;
/// `$gp+0x167c` .. `$gp+0x168c`: the spin's rise and fall a tick, its idle value, the idle rise, the shot's spin.
const SPIN_UP: f32 = 0.1;
const SPIN_DOWN: f32 = 0.01;
const SPIN_IDLE: f32 = 0.1;
const SPIN_IDLE_UP: f32 = 0.1;
const SPIN_SHOT: f32 = 1.0;
/// `0x29ade0`: the points about joint 0 (the second, the ball point, rewritten by every draw).
const POINTS: [V3; 5] = [[0.0, -0.1, 0.0], [0.0, -0.5, 0.0], [0.0, -0.28, -0.11], [-0.11, -0.28, 0.03], [0.11, -0.28, 0.03]];
/// `0x1b00f0`: the classes the explosion's search leaves out (−1 ends).
const SEARCH_SKIPS: [i16; 2] = [0, BALL];

/// Three wobbling angles a control point is drawn from (an arc's and a hand arc's halves): `cos a₀·k₀, sin a₁·k₁,
/// −sin a₂·k₂`.
#[derive(Clone, Copy, Debug, Default)]
pub struct Wobble {
    pub angles: [f32; 3],
    pub speeds: [f32; 3],
    pub amps: [f32; 3],
}

impl Wobble {
    pub(super) fn step(&mut self, k: f32) -> V3 {
        for i in 0..3 { self.angles[i] = add_rot(self.angles[i], self.speeds[i] * k); }
        [self.angles[0].cos() * self.amps[0], self.angles[1].sin() * self.amps[1], -self.angles[2].sin() * self.amps[2]]
    }
}

/// One of the four arcs about the hand (`0x24f740 + 0x80·k`: `+0x00` / `+0x24` the halves' angles, then speeds and
/// amplitudes, `+0x50` / `+0x60` the control points).
#[derive(Clone, Copy, Debug, Default)]
pub struct HandArc {
    pub halves: [Wobble; 2],
    pub ctrl: [V3; 2],
}

/// A ray of a ball (`+0x00` its way, `+0x10` its ball, `+0x14` its length, `+0x18` / `+0x1c` its ticks and life,
/// `+0x20` its scroll, `+0x24` its phase, `+0x28` a sign no draw reads).
#[derive(Clone, Copy, Debug, Default)]
pub struct Ray {
    pub dir: V3,
    pub owner: Option<usize>,
    pub len: f32,
    pub ticks: i32,
    pub total: i32,
    pub scroll: f32,
    pub phase: f32,
    pub sign: f32,
}

/// An arc of a ball (`+0x00` / `+0x10` the control offsets, `+0x20` / `+0x30` its ends, `+0x40` / `+0x64` its
/// halves' wobble, `+0x88` its life after the reach, `+0x8c` its ball, `+0x90` its moby, `+0x94` / `+0x98` the reach's
/// ticks and their count, `+0x9c` hit).
#[derive(Clone, Copy, Debug, Default)]
pub struct Arc {
    pub ctrl: [V3; 2],
    pub from: V3,
    pub to: V3,
    pub halves: [Wobble; 2],
    pub life: i32,
    pub owner: Option<usize>,
    pub target: Option<usize>,
    pub grow: i32,
    pub grow_total: i32,
    pub hit: bool,
}

/// The Plasma Coil's globals: the rays and arcs (its trail slot's memory, `$gp+0x3a64` / `$gp+0x3a68`) and their set-up
/// (`$gp+0x10fc`), the balls' table `0x247240` and count `$gp+0x10f8`, the struck sound's flag `$gp+0x10f4`, the hand's
/// arcs `0x24f740`, the phase `$gp+0x1690`, the empty latch `0x1b2af0`, the ball point's wobble angles `$gp+0x169c` /
/// `$gp+0x16a0`; the twinkles the hand's draw asked for, made by the next update (the draw has no particles).
#[derive(Clone, Debug)]
pub struct Mem {
    pub rays: Vec<Ray>,
    pub arcs: Vec<Arc>,
    pub ready: bool,
    pub balls: [Option<usize>; 5],
    pub count: i32,
    pub struck_sound: bool,
    pub hand_arcs: [HandArc; 4],
    pub phase: i32,
    pub latch: bool,
    pub wobble: [f32; 2],
    pub sparks: Vec<Spark>,
    /// The Plasma Storm's (crate::moby::classes::plasma_storm).
    pub storm: super::plasma_storm::Mem,
}

impl Default for Mem {
    fn default() -> Self {
        Mem {
            rays: vec![Ray::default(); RAYS],
            arcs: vec![Arc::default(); ARCS],
            ready: false,
            balls: [None; 5],
            count: 0,
            struck_sound: false,
            hand_arcs: [HandArc::default(); 4],
            phase: 0,
            latch: false,
            // `$gp+0x16a0`'s data: 0.14.
            wobble: [0.0, 0.14],
            sparks: Vec::new(),
            storm: Default::default(),
        }
    }
}

/// A pair of twinkles `0x3eb608` asked for: its fall, place, size and turn.
#[derive(Clone, Copy, Debug)]
pub struct Spark {
    pub fall: f32,
    pub at: V3,
    pub size: f32,
    pub spin: i32,
}

/// The Plasma Coil's row as `0x139568[0x1c]` gives it (crate::upgrades).
fn cur_row(game: &crate::game::GameState) -> Option<&'static ItemRow> { row(game, PLASMA_COIL) }

/// The arc's damage (the row's `+0x64`; crate::hero::items::ItemRow keeps only `+0x60`).
const ARC_DAMAGE: f32 = 12.0;

/// `0x2eef88(out, a, b)` = b × a.
fn cross_ba(a: V3, b: V3) -> V3 { cross(b, a) }

/// `0x321c00(a, out, v, axis)`: `v` turned by `a` about `axis` (under 1e-5 none).
fn turn(v: V3, axis: V3, a: f32) -> V3 {
    if a.abs() < 1e-5 { return v; }
    crate::gravity::rotate(v, unit(axis), a)
}

/// A moby's aim point (`0x31fa20` + `0x326520(m, out, 1)`): its place raised along the up there by its aim height
/// when the enemy framework runs it.
pub(super) fn aim_at(w: &MobyWorld, ctx: &Ctx, t: usize) -> V3 {
    let p = w.mobys[t].pos;
    match aim::health(w, t) {
        Some((_, h)) => add(p, scale(ctx.hero.gravity.up_at(p), h)),
        None => p,
    }
}

/// `0x3ea9c0` / `0x3cd150(t, out, p₀, p₁, a, b)`: the cubic from `p0` to `p1` (`(f₆·t³ + (f₁ − f₆)·t² + (p₁ − a)·t + p₀`,
/// f₁ = a − p₀, f₆ = (b − p₁) − f₁); exactly the ends at 0 and 1.
pub(super) fn cubic(t: f32, p0: V3, p1: V3, a: V3, b: V3) -> V3 {
    if t == 0.0 { return p0; }
    if t == 1.0 { return p1; }
    let (t2, t3) = (t * t, t * t * t);
    std::array::from_fn(|k| {
        let f1 = a[k] - p0[k];
        let f6 = (b[k] - p1[k]) - f1;
        f6 * t3 + (f1 - f6) * t2 + (p1[k] - a[k]) * t + p0[k]
    })
}

/// `0x3cd018`: four balls out.
fn full(w: &MobyWorld) -> bool { w.coil.count == MAX_BALLS }

/// `0x3e9608` (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].pvars.resize(crate::moby::PVAR_BYTES, 0);
    spawn_sparks(w, ctx);
    let speed = cur_row(ctx.game).map_or(15.0, |r| r.speed) * 0.016_666_668;
    match w.mobys[id].state {
        0 => start(w, id, ctx),
        1 => {
            tick(w, id, ctx, speed);
            spin(w, id);
        }
        _ => {}
    }
}

/// The hand's state 0 (module docs).
fn start(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if let Some(a) = ctx.host.anim(&w.mobys[id]) {
        if a.seq_b != 0 || a.flags & 2 != 0 { ctx.host.set_anim(&w.mobys[id], 1, 0, 0); }
    }
    let class = w.mobys[id].o_class;
    {
        let m = &mut w.mobys[id];
        m.joint_nodes = vec![(0, JointModifier::compose(0))];
        m.set_pv_f(0, f32::from_bits(0x3ebb_bbbc));
        m.set_pv_i(0xc, -1);
        m.set_pv_f(0x10, 0.0);
        m.set_pv_f(0x14, 0.0);
        m.state = 1;
    }
    w.coil.phase = 0;
    let mut pool = std::mem::take(&mut w.chopper);
    let (slot, _) = pool.slot(w, class);
    w.chopper = pool;
    w.mobys[id].set_pv_i(4, slot as i32);
    let g = &mut ctx.game.rng;
    for a in w.coil.hand_arcs.iter_mut() {
        for h in a.halves.iter_mut() {
            h.angles = [0.0; 3];
            for s in h.speeds.iter_mut() { *s = g.randf_signed(0.05, 0.3); }
            for k in h.amps.iter_mut() { *k = g.randf(0.25, 0.75); }
        }
        a.ctrl = [[0.0; 3]; 2];
    }
    // `0x3cd028`: the balls' table cleared, none out.
    w.coil.balls = [None; 5];
    w.coil.count = 0;
}

/// The hand's state 1 (module docs).
fn tick(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, speed: f32) {
    if ctx.hero.state == 1 { ctx.hero.request_state(0x1d, true); }
    let mut ok = true;
    if ctx.hero.state == 0x1d {
        ok = countdown(&mut ctx.hero.items.look_wait);
        let red = w.mobys[id].pv_f(0) != 0.0 && ctx.hero.items.has_ammo(ctx.game);
        crate::hud::marker(&mut ctx.game.markers, 1.0, id, if red { 0x6000_00ff } else { 0x600f_ff0f }, None, RETICLE);
    } else {
        ctx.hero.items.look_wait = 0x28;
    }
    // The aim (the lock cheat's `0x18aa18` path is not ported).
    let pick = if ctx.hero.items.aim_target.is_some() {
        None
    } else {
        let h = &*ctx.hero;
        let from = add(h.pos, scale(h.rows[2], 0.75));
        let cone = aim::Cone { yaw: f32::from_bits(0x3e32_b8c4), pitch: f32::from_bits(0x3e32_b8c4), range: 20.0, near_pitch: f32::from_bits(0x3f49_0fdc), near_yaw: f32::from_bits(0x3f49_0fdc) };
        aim::search(w, ctx, cone, from, h.rows[0], true)
    };
    if w.mobys[id].pv_f(0) == 0.0 && ctx.hero.items.has_ammo(ctx.game) {
        if let Some(t) = pick {
            let at = aim_at(w, ctx, t);
            let d = dist(ctx.host.camera_pos(), at);
            let size = (0.9 - d * f32::from_bits(0x3bda_740e) * f32::from_bits(0x3f26_6666)).abs();
            crate::hud::marker(&mut ctx.game.markers, size, id, 0xff0f_ff0f, Some(at), RETICLE);
        }
    }
    if cooldown(w, id) {
        let it = ctx.hero.items;
        if it.no_use == 0 && it.held & it.trigger != 0 && ok && !full(w) && 0x16 < it.item_ticks {
            if ctx.hero.items.take_ammo(ctx.game, 1) {
                w.coil.latch = true;
                ctx.host.play_sound(&w.mobys[id], 2, 0);
                match w.coil.phase {
                    0 => {
                        w.coil.phase = 1;
                        let m = &mut w.mobys[id];
                        m.set_pv_f(0x10, SPIN_IDLE);
                        m.set_pv_f(0, 0.5);
                    }
                    2 => {
                        w.coil.phase = 1;
                        w.mobys[id].set_pv_f(0, 0.5);
                    }
                    _ => {}
                }
            } else if it.pressed & it.trigger != 0 || w.coil.latch {
                ctx.host.play_sound(&w.mobys[id], 7, 0);
                w.coil.latch = false;
            }
        }
    }
    let s = w.mobys[id].pv_f(0x10);
    match w.coil.phase {
        0 => {
            if s < SPIN_IDLE {
                let n = s + SPIN_IDLE_UP;
                w.mobys[id].set_pv_f(0x10, if SPIN_IDLE < n { SPIN_IDLE } else { n });
            }
        }
        1 => {
            let n = s + SPIN_UP;
            w.mobys[id].set_pv_f(0x10, n);
            if SPIN_SHOT < n { shoot(w, id, ctx, speed, pick); }
        }
        2 => {
            let n = s - SPIN_DOWN;
            if n <= SPIN_IDLE {
                w.mobys[id].set_pv_f(0x10, SPIN_IDLE);
                w.coil.phase = 0;
            } else {
                w.mobys[id].set_pv_f(0x10, n);
            }
        }
        _ => {}
    }
    if w.mobys[id].drawn != 0 { w.register_draw(id); }
}

/// The spin's end of `0x3e9608`: the angle turns −spin·π/10 and the node takes its half-angle rotation about y.
fn spin(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    let a = sub_rot(m.pv_f(0x14), m.pv_f(0x10) * f32::from_bits(0x3ea0_d97d));
    m.set_pv_f(0x14, a);
    let (s, c) = ((a * 0.5).sin(), (a * 0.5).cos());
    m.set_pv_f(0x34, s);
    m.set_pv_f(0x3c, c);
    if let Some((_, n)) = m.joint_nodes.iter_mut().find(|(l, _)| *l == 0) { n.quat = [0.0, s, 0.0, c]; }
}

/// `0x2eeb40(+0x00)`: the cooldown down a frame (1/60 s); true when out.
pub(super) fn cooldown(w: &mut MobyWorld, id: usize) -> bool {
    let c = w.mobys[id].pv_f(0);
    if 1.0 / 60.0 < c {
        w.mobys[id].set_pv_f(0, c - 1.0 / 60.0);
        false
    } else {
        w.mobys[id].set_pv_f(0, 0.0);
        true
    }
}

/// The shot of the spin-up phase (module docs).
fn shoot(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, speed: f32, pick: Option<usize>) {
    let muzzle = ctx.host.joint(&w.mobys[id], 0).unwrap_or(w.mobys[id].pos);
    let mut dir = if ctx.hero.items.raised { set_len(w.mobys[id].rows[1], -speed) } else { set_len(ctx.hero.rows[0], speed) };
    let near = ctx.scene.map(|sc| crate::collision::coll_sphere_mobys(sc, muzzle, 1.0, crate::collision::QueryFlags(0), Some(id))).unwrap_or_default();
    for t in near {
        if Some(t) == ctx.hero_moby { continue; }
        let m = &w.mobys[t];
        if m.mode & mode::TARGET == 0 || m.o_class == 0xc87 || m.o_class == 0xecc { continue; }
        dir = set_len(sub(aim_at(w, ctx, t), muzzle), speed);
        break;
    }
    if let Some(t) = pick.filter(|&t| w.mobys.get(t).is_some()) { dir = set_len(sub(aim_at(w, ctx, t), muzzle), speed); }
    let slot = w.mobys[id].pv_i(4) as usize;
    if let Some(b) = spawn(w, ctx, id, slot, muzzle, dir) { w.mobys[b].parent = Some(id); }
    w.coil.phase = 2;
}

/// `0x3ccdc8(hand, slot, at, dir)` (module docs): the ball, unless four are out.
pub fn spawn(w: &mut MobyWorld, ctx: &mut Ctx, hand: usize, slot: usize, at: V3, dir: V3) -> Option<usize> {
    if full(w) || w.class(BALL).is_none() { return None; }
    let index = w.coil.balls.iter().position(|b| b.is_none()).unwrap_or(4);
    let hand_class = w.mobys[hand].o_class;
    let id = w.create(BALL);
    {
        let m = &mut w.mobys[id];
        m.update_dist = 0xff;
        m.draw_dist = 0xff;
        m.state = 0;
        m.pos = at;
        m.set_pv_v(0, set_len(dir, 1.0));
        m.set_pv_i(0x10, 0xa0);
        m.set_pv_i(0x30, 0x14);
        m.set_pv_i(0x14, index as i32);
        m.set_pv_f(0x18, 2.5);
        m.set_pv_i(0x3c, slot as i32);
        m.set_pv_i(0x1c, 0);
        m.set_pv_i(0x34, 0);
        m.set_pv_i(0x38, 0);
        m.set_pv_i(0x40, hand_class as i32);
    }
    w.coil.count += 1;
    if let Some(b) = w.coil.balls.get_mut(index) { *b = Some(id); }
    if !w.coil.ready {
        w.coil.ready = true;
        for r in w.coil.rays.iter_mut() { r.owner = None; }
        for a in w.coil.arcs.iter_mut() { a.owner = None; }
    }
    for _ in 0..20 { start_ray(w, ctx, id); }
    Some(id)
}

/// `0x3cc980(ball)`: the first free ray taken — a random way (−1..1 each) at the length the record last had, 20..50
/// ticks, a scroll −1..1, a sign; then its length for next time 0.7..0.9.
fn start_ray(w: &mut MobyWorld, ctx: &mut Ctx, ball: usize) {
    let Some(r) = w.coil.rays.iter_mut().find(|r| r.owner.is_none()) else { return };
    let g = &mut ctx.game.rng;
    let v = [g.randf(-1.0, 1.0), g.randf(-1.0, 1.0), g.randf(-1.0, 1.0)];
    r.dir = set_len(v, r.len);
    let t = g.rand_range(0x14, 0x32);
    r.ticks = t;
    r.total = t;
    let s = g.randf(-1.0, 1.0);
    r.owner = Some(ball);
    r.phase = 0.0;
    r.scroll = s;
    r.sign = if g.rand() & 1 != 0 { -1.0 } else { 1.0 };
    r.len = g.randf(0.7, 0.9);
}

/// `0x3cca98(ball)` (module docs): each ray of the ball scrolls; its ticks out, a fresh unit way and life.
fn step_rays(w: &mut MobyWorld, ctx: &mut Ctx, ball: usize) {
    let g = &mut ctx.game.rng;
    for r in w.coil.rays.iter_mut().filter(|r| r.owner == Some(ball)) {
        let s = r.scroll - g.randf(0.01, f32::from_bits(0x3d99_999a));
        r.scroll = if s <= -8.0 { s + 8.0 } else { s };
        if countdown(&mut r.ticks) {
            let v = [g.randf(-1.0, 1.0), g.randf(-1.0, 1.0), g.randf(-1.0, 1.0)];
            r.phase = 0.0;
            let t = g.rand_range(0x14, 0x32);
            r.ticks = t;
            r.total = t;
            r.sign = if g.rand() & 1 != 0 { -1.0 } else { 1.0 };
            r.dir = set_len(v, 1.0);
            r.len = g.randf(0.7, 0.9);
        }
    }
}

/// `0x3cc1f0(ball, moby, ticks)`: the first free arc taken from the ball to the moby's aim point.
fn start_arc(w: &mut MobyWorld, ctx: &mut Ctx, ball: usize, target: usize, ticks: i32) {
    let Some(k) = w.coil.arcs.iter().position(|a| a.owner.is_none()) else { return };
    let to = if aim::health(w, target).is_some() { aim_at(w, ctx, target) } else { w.mobys[target].pos };
    let from = w.mobys[ball].pos;
    let n = w.mobys[ball].pv_i(0x1c);
    w.mobys[ball].set_pv_i(0x1c, n + 1);
    let g = &mut ctx.game.rng;
    let pi = std::f32::consts::PI;
    let mut halves = [Wobble::default(); 2];
    for h in halves.iter_mut() { for x in h.angles.iter_mut() { *x = g.randf(-pi, pi); } }
    for h in halves.iter_mut() { for x in h.speeds.iter_mut() { *x = g.randf_signed(0.001, 0.05); } }
    for x in halves[0].amps.iter_mut() { *x = g.randf(3.25, 6.75); }
    for x in halves[1].amps.iter_mut() { *x = g.randf(1.25, 4.75); }
    w.coil.arcs[k] = Arc { ctrl: [[0.0; 3]; 2], from, to, halves, life: 0x14, owner: Some(ball), target: Some(target), grow: ticks, grow_total: ticks, hit: false };
}

/// `0x3cc4c0(ball)` (module docs).
fn step_arcs(w: &mut MobyWorld, ctx: &mut Ctx, ball: usize) {
    let pos = w.mobys[ball].pos;
    for k in 0..ARCS {
        if w.coil.arcs[k].owner != Some(ball) { continue; }
        let mut a = w.coil.arcs[k];
        a.from = pos;
        let f = if a.grow == 0 { 1.0 } else { 100.0 };
        a.ctrl = [a.halves[0].step(f), a.halves[1].step(f)];
        if let Some(t) = a.target.filter(|&t| w.mobys.get(t).is_some()) {
            let p = aim_at(w, ctx, t);
            let s = 1.0 - a.grow as f32 / a.grow_total as f32;
            a.to = std::array::from_fn(|i| a.from[i] + (p[i] - a.from[i]) * s);
        }
        if countdown(&mut a.grow) {
            if let Some(t) = a.target.filter(|&t| w.mobys.get(t).is_some_and(|m| m.has_collision)) {
                if !a.hit {
                    let d = set_len(sub(a.to, a.from), f32::from_bits(0x3d08_8889));
                    let mut tm = HitTemplate::new(ARC_DAMAGE, Some(ball), 0x21_0000, [d[0], d[1], d[2], 0.0]);
                    tm.kind = (BALL as u16 as u32) << 16 | 0x0301;
                    w.deliver_hit_any(t, &tm, [0.0; 4]);
                    a.hit = true;
                    sparks(ctx, a.to, 10, 1);
                }
            }
            if countdown(&mut a.life) {
                a.owner = None;
                let n = w.mobys[ball].pv_i(0x1c);
                w.mobys[ball].set_pv_i(0x1c, n - 1);
            }
        }
        w.coil.arcs[k] = a;
    }
}

/// A hit on `target` from the ball (`0x31d158` + `0x31d378`): the row's damage, flags 0x210000, along `vel` at 1/30,
/// kind `0x0a9b << 16 | 0x0301`.
fn strike(w: &mut MobyWorld, ctx: &Ctx, ball: usize, target: usize, vel: V3) {
    let damage = cur_row(ctx.game).map_or(0.0, |r| r.damage);
    let d = set_len(vel, f32::from_bits(0x3d08_8889));
    let mut t = HitTemplate::new(damage, Some(ball), 0x21_0000, [d[0], d[1], d[2], 0.0]);
    t.kind = (BALL as u16 as u32) << 16 | 0x0301;
    w.deliver_hit_any(target, &t, [0.0; 4]);
}

/// The ball rides the moby it struck (its aim point with a health block, else its place).
fn ride(w: &mut MobyWorld, ctx: &Ctx, id: usize) {
    let Some(t) = w.mobys[id].pv_link(0x38).filter(|&t| w.mobys.get(t).is_some()) else { return };
    if aim::health(w, t).is_some() { w.mobys[id].pos = aim_at(w, ctx, t); }
}

/// The loop sound let go when still playing (`0x3602f0`).
fn stop_sound(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let s = w.mobys[id].pv_i(0x20);
    if s != -1 && ctx.host.sound_playing(s) { ctx.host.stop_sound(s); }
    w.mobys[id].set_pv_i(0x20, -1);
}

/// `0x3cd060(ball)`: gone — the count down, its table entry and its rays freed.
fn gone(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    stop_sound(w, id, ctx);
    w.coil.count -= 1;
    let index = w.mobys[id].pv_i(0x14);
    if let Some(b) = usize::try_from(index).ok().and_then(|i| w.coil.balls.get_mut(i)) { *b = None; }
    for r in w.coil.rays.iter_mut().filter(|r| r.owner == Some(id)) { r.owner = None; }
    w.delete(id);
}

/// `0x3cbb78` (module docs).
pub fn update_ball(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].pvars.resize(crate::moby::PVAR_BYTES, 0);
    if w.mobys[id].state == 0 {
        let m = &mut w.mobys[id];
        m.state = 1;
        m.set_pv_i(0x24, -1);
        m.set_pv_i(0x20, -1);
    }
    match w.mobys[id].state {
        1 => {
            let start = w.mobys[id].pos;
            let vel = w.mobys[id].pv_v(0);
            let pos = add(start, vel);
            w.mobys[id].pos = pos;
            // `0x360a10`: the loop sound's pitch moved by −200..200 (not ported; the draw kept).
            ctx.game.rng.rand_range(-200, 200);
            let near = ctx.scene.map(|sc| crate::collision::coll_sphere_mobys(sc, pos, f32::from_bits(0x3ee6_6666), crate::collision::QueryFlags(0), Some(id))).unwrap_or_default();
            for t in near {
                let m = &w.mobys[t];
                if m.o_class == HAND_CLASS || Some(t) == ctx.hero_moby || m.mode & mode::TARGET == 0 { continue; }
                if aim::health(w, t).is_some() {
                    let p = aim_at(w, ctx, t);
                    w.mobys[id].pos = p;
                }
                {
                    let m = &mut w.mobys[id];
                    m.set_pv_i(0x38, t as i32);
                    m.set_pv_i(0x10, 0xf);
                    m.state = 2;
                }
                strike(w, ctx, id, t, vel);
                let at = w.mobys[id].pos;
                sparks(ctx, at, 10, 1);
                break;
            }
            if w.mobys[id].pv_link(0x38).is_none() {
                let met = w.coll_line(ctx, start, pos, 2, Some(id)).or_else(|| ctx.coll.and_then(|c| crate::collision::coll_sphere_m(c, ctx.scene, pos, 0.1, crate::collision::QueryFlags(2), Some(id))));
                if let Some(o) = met {
                    let at = add(o.point, set_len(o.normal, 0.1));
                    let m = &mut w.mobys[id];
                    m.pos = at;
                    m.set_pv_i(0x10, 0xf);
                    m.state = 2;
                    sparks(ctx, at, 0xf, 0);
                }
            }
            let mut n = w.mobys[id].pv_i(0x10);
            if countdown(&mut n) {
                w.mobys[id].state = 4;
                n = 0xa1;
            }
            w.mobys[id].set_pv_i(0x10, n);
        }
        2 => {
            ride(w, ctx, id);
            let mut n = w.mobys[id].pv_i(0x10);
            let out = countdown(&mut n);
            w.mobys[id].set_pv_i(0x10, n);
            if out {
                w.mobys[id].state = 4;
                let (pos, fwd) = (w.mobys[id].pos, w.mobys[id].rows[0]);
                let struck: Vec<usize> = w.mobys[id].pv_link(0x38).into_iter().collect();
                let tau = f32::from_bits(0x40c9_0fdb);
                let cone = aim::Cone { yaw: tau, pitch: tau, range: 10.0, near_pitch: 0.0, near_yaw: 0.0 };
                let found = aim::search_list_except(w, ctx, cone, pos, fwd, false, 3, &struck, &SEARCH_SKIPS);
                for t in found {
                    for _ in 0..5 { start_arc(w, ctx, id, t, 10); }
                    ctx.host.play_sound(&w.mobys[id], 1, 0);
                }
            }
        }
        4 => {
            let struck = w.mobys[id].pv_link(0x38);
            if struck.is_some() {
                if !w.coil.struck_sound {
                    w.coil.struck_sound = true;
                    ctx.host.play_sound(&w.mobys[id], 3, 0);
                }
                ride(w, ctx, id);
            }
            if w.mobys[id].pv_i(0x1c) == 0 {
                let at = w.mobys[id].pos;
                if struck.is_none() {
                    sparks(ctx, at, 10, 1);
                    ctx.host.play_sound(&w.mobys[id], 2, 0);
                }
                // `0x3cd060`'s arcs are none; its rays freed (`0x3ccc18`).
                gone(w, id, ctx);
                return;
            }
        }
        _ => {}
    }
    if w.mobys[id].state == 4 {
        stop_sound(w, id, ctx);
    } else {
        let s = w.mobys[id].pv_i(0x20);
        if s == -1 || !ctx.host.sound_playing(s) {
            let h = ctx.host.play_sound(&w.mobys[id], 0, 4);
            w.mobys[id].set_pv_i(0x20, h);
        }
    }
    step_arcs(w, ctx, id);
    step_rays(w, ctx, id);
    let (class, slot) = (w.mobys[id].pv_i(0x40) as i16, w.mobys[id].pv_i(0x3c) as usize);
    if w.chopper.holds(class, slot) {
        w.register_draw(id);
    } else {
        stop_sound(w, id, ctx);
        for r in w.coil.rays.iter_mut().filter(|r| r.owner == Some(id)) { r.owner = None; }
        w.delete(id);
    }
}

/// `0x3ce060(at, n, kind)`: `n` pairs of twinkles (crate::particles::twinkle) jittered 0.15 about `at`, flying toward
/// the camera within ±60° at 0.05..3 a second and up 1..5 a second, falling 0.002, 0.5..2 large: kind 0 (on the world)
/// one time in two 1.5 times as fast — 0.35 of the size (60 ticks), else drifting toward the camera at their speed —
/// 2.05 (40 ticks); kind 1 (on a moby) the other way about, 0.25 (40) and 0.3 (60); each in `0x7f800000` and a white
/// one 0.7 / 0.245 / 1.435 / 0.175 / 0.21 of the size turning the other way; their `+0x09` 0x74.
fn sparks(ctx: &mut Ctx, at: V3, n: i32, kind: i32) {
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let g = &mut ctx.game.rng;
    let cam = ctx.host.camera_pos();
    let v0 = sub(cam, at);
    let up = [0.0, 0.0, 1.0];
    let fall = f32::from_bits(0x3b03_126f);
    for _ in 0..n {
        let size = g.randf(0.5, 2.0);
        let r = g.randi(2);
        let s = if r == 0 { -1 } else { r };
        let a = g.randf(f32::from_bits(0xbf86_0a93), f32::from_bits(0x3f86_0a93));
        let f = g.randf(f32::from_bits(0x3d4c_cccd), 3.0) * 0.016_666_668;
        let mut v = set_len(turn(v0, up, a), f);
        v[2] = g.randf(1.0, 5.0) * 0.016_666_668;
        let p: V3 = std::array::from_fn(|k| at[k] + g.randf_signed(0.0, f32::from_bits(0x3e19_999a)));
        let odd = g.rand() & 1 != 0;
        let toward = |v: V3| add(v, set_len(sub(cam, p), f));
        let (first, second, life, v) = match (kind, odd) {
            (0, false) => (0.35, 0.245, 0x3c, scale(v, 1.5)),
            (0, true) => (2.05, 1.435, 0x28, toward(v)),
            (_, true) => (0.25, 0.175, 0x28, toward(v)),
            (_, false) => (0.3, 0.21, 0x3c, scale(v, 1.5)),
        };
        if let Some(i) = crate::particles::twinkle(sys, g, size * first, size, fall, p, life, 0x7f80_0000, 0, s as i8, v, 1) { sys.pool.recs[i][9] = 0x74; }
        if let Some(i) = crate::particles::twinkle(sys, g, size * second, size * 0.7, fall, p, life, 0x7f7f_7f7f, 1, -s as i8, v, 1) { sys.pool.recs[i][9] = 0x74; }
    }
}

/// The twinkles the hand's last draw asked for (`0x3eb608`, [`spark`]): still, 0.5..0.75 large, 60 ticks, `0x7f7f2020`
/// and a `0x7fff4040` one 0.7 of it turning the other way; their `+0x09` 0x74.
pub(super) fn spawn_sparks(w: &mut MobyWorld, ctx: &mut Ctx) {
    let list = std::mem::take(&mut w.coil.sparks);
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let g = &mut ctx.game.rng;
    for s in list {
        if let Some(i) = crate::particles::twinkle(sys, g, s.size * 0.1, s.size, s.fall, s.at, 0x3c, 0x7f7f_2020, 0, s.spin as i8, [0.0; 3], 1) { sys.pool.recs[i][9] = 0x74; }
        if let Some(i) = crate::particles::twinkle(sys, g, s.size * 0.07, s.size * 0.7, s.fall, s.at, 0x3c, 0x7fff_4040, 1, -s.spin as i8, [0.0; 3], 1) { sys.pool.recs[i][9] = 0x74; }
    }
}

/// `0x3eb608(fall, at, 1)`: the twinkles' random draws (their size, turn, way: the way then cleared), queued for the
/// next update ([`spawn_sparks`]).
pub(super) fn spark(w: &mut MobyWorld, g: &mut crate::rng::Rng, fall: f32, at: V3) {
    let size = g.randf(0.5, 0.75);
    let r = g.randi(2);
    let spin = if r == 0 { -1 } else { r };
    g.randf(f32::from_bits(0xbf86_0a93), f32::from_bits(0x3f86_0a93));
    g.randf(f32::from_bits(0x3ba3_d70a), f32::from_bits(0x3e99_999a));
    g.randf(0.01, 0.5);
    w.coil.sparks.push(Spark { fall, at, size, spin });
}

const ST: [[f32; 2]; 4] = [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]];

/// `0x3cd478(size, roll, at, rgb, alpha, fx, 2)`: the ±1 square × `size` facing the camera (yaw and pitch toward it,
/// `roll`), additive.
fn facing(out: &mut Vec<crate::fx::FxQuad>, size: f32, roll: f32, at: V3, rgb: u32, alpha: i32, fx: usize, cam: V3) {
    let d = sub(cam, at);
    let yaw = atan2(d[0], d[1]);
    let pitch = -atan2(dist2(cam, at), d[2]);
    let rows = crate::hero::physics::euler_rows([roll, pitch, yaw]);
    let sq: [V3; 4] = [[0.0, -1.0, -1.0], [0.0, -1.0, 1.0], [0.0, 1.0, -1.0], [0.0, 1.0, 1.0]];
    let corners = sq.map(|c| add(at, crate::gravity::to_world(scale(c, size), &rows)));
    let c = rgb.wrapping_add((alpha as u32) << 24);
    out.push(crate::fx::FxQuad { fx, additive: true, corners, st: ST, rgba: [c; 4] });
}

/// `0x3cdb88` / `0x3eb010(_, fix, points, n)`: a strip through `points` facing the camera (at each its half-width across
/// `way × (camera − point)`, the way to the next point), two quads a segment: FX `fx[0]` `wide[0]` across in `ca`, FX
/// `fx[1]` `wide[1]` across in `cb` with s stepping a third a segment; their fixed alpha `fix` and `fix / 3`.
#[allow(clippy::too_many_arguments)]
fn strip(out: &mut Vec<crate::fx::FxQuad>, points: &[V3], cam: V3, fx: [usize; 2], wide: [f32; 2], ca: [u32; 4], cb: [u32; 4], fix: i32) {
    let n = points.len();
    if n < 3 { return; }
    let across = |i: usize| {
        let d = unit(sub(points[i + 1], points[i]));
        unit(cross_ba(sub(cam, points[i]), d))
    };
    let edge = |i: usize, w: f32| {
        let c = across(i);
        (add(points[i], scale(c, w)), sub(points[i], scale(c, w)))
    };
    let alpha = |c: [u32; 4], a: i32| c.map(|x| x & 0xff_ffff | ((a as u32) & 0xff) << 24);
    let (ca, cb) = (alpha(ca, fix), alpha(cb, fix / 3));
    let k = f32::from_bits(0x3ea8_f5c3);
    let (mut a, mut b) = (edge(0, wide[0]), edge(0, wide[1]));
    let mut s = 0.0f32;
    for i in 1..n - 1 {
        let (na, nb) = (edge(i, wide[0]), edge(i, wide[1]));
        out.push(crate::fx::FxQuad { fx: fx[0], additive: true, corners: [a.0, a.1, na.0, na.1], st: ST, rgba: ca });
        out.push(crate::fx::FxQuad { fx: fx[1], additive: true, corners: [b.0, b.1, nb.0, nb.1], st: [[s, 0.0], [s, 1.0], [s + k, 0.0], [s + k, 1.0]], rgba: cb });
        s = i as f32 * k;
        if 1.0 <= s { s -= 1.0; }
        (a, b) = (na, nb);
    }
}

/// `0x3cdb20(points, n)`: the last point the one before plus the last step.
pub(super) fn extrapolate(points: &mut Vec<V3>) {
    let n = points.len();
    if n < 2 { return; }
    points.push(add(points[n - 1], sub(points[n - 1], points[n - 2])));
}

/// `0x3cd2d8` (module docs).
pub fn draw_ball(w: &MobyWorld, id: usize, g: &mut crate::rng::Rng, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    // `0x3cc878`: the arcs.
    for a in w.coil.arcs.iter().filter(|a| a.owner == Some(id)) {
        draw_arc(a, g, cam, out);
        if a.grow == 0 {
            for (size, rgb) in [(0.5, 0x10_3080u32), (0.3, 0xff_ffff)] {
                let alpha = g.rand_range(0x50, 0x60);
                let roll = g.rand_angle();
                facing(out, size, roll, a.to, rgb, alpha, 0x31, cam);
            }
        }
    }
    let m = &w.mobys[id];
    let (pos, size) = (m.pos, m.pv_f(0x18));
    for r in w.coil.rays.iter().filter(|r| r.owner == Some(id)) { draw_ray(r, g, pos, size, cam, out); }
    for (k, rgb, lo, hi, fx) in [(0.5, 0x10_3080u32, 0x50, 0x60, 0x31), (0.3, 0xff_ffff, 0x50, 0x60, 0x31), (0.7, 0x80_3010, 8, 0x16, 0xb), (0.45, 0x80_3010, 0x20, 0x40, 0xb)] {
        let alpha = g.rand_range(lo, hi);
        let roll = g.rand_angle();
        facing(out, size * k, roll, pos, rgb, alpha, fx, cam);
    }
}

/// `0x3cd940(ball, arc)`: 24 points along the arc's cubic jittered 0.075, the last extrapolated, as a [`strip`]
/// (FX 0x2b, fixed alpha 0..0x40 in its last 10 ticks, else 0x40..0x80).
fn draw_arc(a: &Arc, g: &mut crate::rng::Rng, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let (ca, cb) = (add(a.ctrl[0], a.from), add(a.ctrl[1], a.to));
    let mut points: Vec<V3> = (0..0x18)
        .map(|i| {
            let p = cubic(i as f32 / 23.0, a.from, a.to, ca, cb);
            let v = [g.randf(-0.1, 0.1), g.randf(-0.1, 0.1), g.randf(-0.1, 0.1)];
            add(p, set_len(v, f32::from_bits(0x3d99_999a)))
        })
        .collect();
    let fix = if a.life < 10 { g.rand_range(0, 0x40) } else { g.rand_range(0x40, 0x80) };
    let wa = g.randf(f32::from_bits(0x3ca3_d70a), 0.1);
    let wb = g.randf(0.15, 0.3);
    extrapolate(&mut points);
    strip(out, &points, cam, [0x2b, 0x2b], [wa, wb], [0x7faf_afaf; 4], [0x80ff_0000; 4], fix);
}

/// `0x3ccc50`'s ray: from the ball out along its way at its length — FX 0x2f in `0x800000` and FX 0x30 in white over
/// it (additive), `size / 4` across at the ball and half that at the end (across `way × (camera − ball)`), the end's
/// alpha 0; the alpha `k·(cos(phase − π) + 1)/2` (k 0x30..0x50; phase 2π back to 0 over its life).
fn draw_ray(r: &Ray, g: &mut crate::rng::Rng, pos: V3, size: f32, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let tau = std::f32::consts::TAU;
    let phase = tau - r.ticks as f32 * tau / (r.total - 1) as f32;
    let to = add(set_len(r.dir, r.len), pos);
    let k = g.rand_range(0x30, 0x50) as f32;
    let alpha = ((phase - std::f32::consts::PI).cos() * k * 0.5 + k * 0.5) as i32 as u32;
    let wide = size * 0.25;
    let d = unit(sub(to, pos));
    let c = unit(cross_ba(sub(cam, pos), d));
    let corners = [add(pos, scale(c, wide)), sub(pos, scale(c, wide)), add(to, scale(c, wide * 0.5)), sub(to, scale(c, wide * 0.5))];
    let s = r.scroll;
    let st = [[s, 0.0], [s, 1.0], [s + 1.0, 0.0], [s + 1.0, 1.0]];
    let dark = 0x80_0000u32;
    out.push(crate::fx::FxQuad { fx: 0x2f, additive: true, corners, st, rgba: [alpha << 24 | dark, alpha << 24 | dark, dark, dark] });
    out.push(crate::fx::FxQuad { fx: 0x30, additive: true, corners, st, rgba: [alpha << 24 | 0xff_ffff, alpha << 24 | 0xff_ffff, dark, dark] });
}

/// `0x3eaf80(size, hand, at, rgb, most)`: a glint — FX 0x2c, 1.5..2·size across, 0.08 toward the camera, a random roll,
/// its alpha (`most`/2..`most`) put over `rgb`'s.
pub(super) fn glint(out: &mut Vec<crate::fx::FxQuad>, g: &mut crate::rng::Rng, size: f32, at: V3, rgba: u32, most: i32, cam: V3) {
    let a = g.rand_range(most >> 1, most);
    let s = g.randf(size * 0.75, size);
    let roll = g.rand_angle();
    crate::fx::sprite(out, s + s, f32::from_bits(0x3da3_d70a), roll, at, cam, 0x2c, rgba | (a as u32) << 24, true);
}

/// The prongs' and joints' pair of glints.
pub(super) fn glints(out: &mut Vec<crate::fx::FxQuad>, g: &mut crate::rng::Rng, at: V3, small: bool, cam: V3) {
    glint(out, g, if small { f32::from_bits(0x3d61_47ae) } else { f32::from_bits(0x3de1_47ae) }, at, 0x7f7f_2020, 0x40, cam);
    glint(out, g, if small { f32::from_bits(0x3ccc_cccd) } else { 0.05 }, at, 0x7fff_4040, 0x78, cam);
}

/// `0x3eac00(hand, from, to, arc)` → `0x3eb010(hand, 0x80, points, 11)`: 10 points along the cubic between `from`
/// and `to` (control points the arc's offsets from them) jittered 0.04, the last extrapolated, as a [`strip`] (FX 0x2b
/// 0.0125 across `0x7f7f7f` / `0xff0000` at its second corners, FX 0xe 0.053 across `0xff0000`; fixed alpha 0x80).
pub(super) fn hand_strip(out: &mut Vec<crate::fx::FxQuad>, g: &mut crate::rng::Rng, from: V3, to: V3, arc: &HandArc, cam: V3) {
    let (a, b) = (add(arc.ctrl[0], from), add(arc.ctrl[1], to));
    let mut points: Vec<V3> = (0..10)
        .map(|i| {
            let p = cubic(i as f32 / 9.0, from, to, a, b);
            let v = [g.randf(-1.0, 1.0), g.randf(-1.0, 1.0), g.randf(-1.0, 1.0)];
            add(p, set_len(v, f32::from_bits(0x3d23_d70a)))
        })
        .collect();
    extrapolate(&mut points);
    let grey = 0x7f7f_7f7f;
    let blue = 0x7fff_0000;
    strip(out, &points, cam, [0x2b, 0xe], [f32::from_bits(0x3c4c_cccd), f32::from_bits(0x3d59_999a)], [grey, blue, grey, grey], [blue; 4], 0x80);
}

/// `0x3e9f18` (module docs).
pub fn draw_hand(w: &mut MobyWorld, host: &dyn crate::moby::MobyHost, id: usize, g: &mut crate::rng::Rng, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    // `0x3eaae0`: the four arcs' wobble.
    for a in w.coil.hand_arcs.iter_mut() {
        a.ctrl = [a.halves[0].step(1.0), a.halves[1].step(1.0)];
    }
    // The ball point's wobble (`0x29adf0`), then its angles step.
    let [a1, a2] = w.coil.wobble;
    w.coil.wobble = [add_rot(a1, f32::from_bits(0x3ed6_7752)), add_rot(a2, f32::from_bits(0x3ed3_7cc6))];
    let mut pts = POINTS;
    pts[1] = [-a2.cos() * f32::from_bits(0x3c77_65fe), -(a1.sin() * f32::from_bits(0x3c75_c28f) + f32::from_bits(0x3ecc_cccd)), a1.cos() * f32::from_bits(0x3c74_1f21)];
    let (spin, rows, p0, ball, prongs, joints) = {
        let m = &w.mobys[id];
        let (rows0, _) = host.joint_matrix(m, 0).unwrap_or((m.rows, m.pos));
        let joint = |j: usize| host.joint_matrix(m, j).map_or(m.pos, |(_, p)| p);
        // `0x3eb560(hand, j, out, p)`: joint j's place plus the point turned by joint 0's frame.
        let local = pts.map(|p| crate::gravity::to_world(p, &rows0));
        let at = |j: usize, k: usize| add(joint(j), local[k]);
        (m.pv_f(0x10), m.rows, at(0, 0), at(0, 1), [at(1, 2), at(2, 3), at(3, 4)], [joint(1), joint(2), joint(3)])
    };
    glints(out, g, p0, false, cam);
    glint(out, g, (spin + spin) * f32::from_bits(0x3da8_f5c2), ball, 0x7f00_397f, 0x40, cam);
    glint(out, g, (spin + spin) * f32::from_bits(0x3d19_999a), ball, 0x7f00_72ff, 0x78, cam);
    for p in prongs { glints(out, g, p, true, cam); }
    let arcs = w.coil.hand_arcs;
    hand_strip(out, g, p0, ball, &arcs[0], cam);
    // `0x3eadb0(0.35·spin, hand, ball)`: the normal of two points on a circle about the ball in the hand's y-z plane.
    let r = spin * 0.35;
    let step = std::f32::consts::TAU / 11.0;
    let c = [step, add_rot(step, step)].map(|a| crate::gravity::to_world([0.0, a.cos() * r, -a.sin() * r], &rows));
    let normal = unit(cross_ba(c[0], c[1]));
    for (k, p) in prongs.into_iter().enumerate() {
        let d = unit(sub(ball, p));
        let d = unit(cross_ba(d, normal));
        let d = unit(cross_ba(d, normal));
        let beside = add(scale(d, spin * 0.45), ball);
        hand_strip(out, g, p, beside, &arcs[k + 1], cam);
        glints(out, g, beside, true, cam);
        if g.rand() % 11 == 0 && SPIN_IDLE < spin { spark(w, g, 0.0, beside); }
    }
    for (k, j) in joints.into_iter().enumerate() {
        hand_strip(out, g, j, prongs[k], &arcs[3 - k], cam);
        glints(out, g, j, true, cam);
    }
    if g.rand() % 11 == 0 {
        let i = (g.rand() % 4) as usize;
        let pick = [ball, prongs[0], prongs[1], prongs[2]][i];
        spark(w, g, f32::from_bits(0x3a83_126f), pick);
    }
}
