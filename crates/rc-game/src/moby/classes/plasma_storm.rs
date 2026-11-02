//! The Plasma Storm, the Plasma Coil's upgrade (item 0x1c's rows 0x3e, 0x65, 0x66, crate::hero::items::UPGRADE_ROWS; L00
//! `0x265b40`, `0x267d60`, `0x267e40`: attachment 0, stance 2, use sequence 0 kept, icon 0xea71, the storm's damage 24 /
//! 30 / 50 (`+0x60`) and an arc's 16 / 25 / 40 (`+0x64`), 15 / 25 / 50 ammo, a shot a second (`+0x44`)): the hand moby
//! **0x05ca** (`0x3a2360`) and the storm it fires **0x09a9** (`0x3bc078`, made by `0x3bc4c8`; the classes' table of
//! every level holds it, its look is its draw's).
//!
//! **The hand** (pvars: `+0x00` the cooldown (seconds), `+0x04` its trail slot, `+0x08` the slot's memory, `+0x0c` the
//! coil's spin, `+0x10` its angle, `+0x20` the joint node on list 0 whose rotation `+0x30` spins the coil). Each tick
//! first: in the look stance 0x1d the look wait (`0x18c354`) counts down, else it is 0x28. State **0**: the cooldown 22
//! ticks (0.3667 s), state **1**, the phase (`$gp+0x5a4`) 0 and the first tick's latch (`$gp+0x58c`) cleared; past its
//! draw-out (sequence B not 0, or the sequence wrapped) its sequence 1; the spin and the angle 0; the node (`0x312480`);
//! the class's trail slot (crate::moby::classes::chopper::Pool, `0x32b1b8`); the four arcs drawn about the hand
//! (crate::moby::classes::plasma_coil::HandArc, `0x23b490`) drawn anew (their speeds ±0.05..0.3, amplitudes 0.25..0.75).
//! **1**, each tick: the first one sets the spin 0. Ratchet in the look stance 1 goes to its twin 0x1d; in 0x1d the
//! screen-centre crosshair shows (`0x2db498`, FX 0x26: red `0x600000ff` while the cooldown runs with ammo left, else
//! green `0x600fff0f`). No aim search (unlike the Plasma Coil's). The cooldown runs down; out, the look wait out, the
//! items not kept away, ○ held, fewer than 2 storms out (`0x3bca38`) and in the hand past 0x16 ticks: one ammo — taken,
//! the class sound 2 and the latch (`0x1b27c0`); the phase from 0 (the spin to 0.1) or 2 goes to **spinning up** (1)
//! and the cooldown to 1 / the row's rate — none left: ○ pressed or the latch set, the class sound 6 and the latch
//! cleared. The phases: idle (0) the spin rises 0.1 a tick to 0.1; spinning up (1) it rises 0.1 a tick and past 1 the
//! storm goes — from joint 0 a third a tick along Ratchet's way (the arm raised at a wall: the gun's −row 1), toward a
//! lock's aim point (`0x18aa18`, `0x32b4d0`), through [`spawn`] — and the phase goes to **spinning down** (2): the spin
//! falls 0.01 a tick back to 0.1, then idle. Drawn, the hand's draw is asked for. The coil turns spin·π/10 a tick about
//! its joint's x (the node's rotation `(sin a/2, 0, 0, cos a/2)`).
//!
//! **The storm** (pvars: `+0x00` its velocity, `+0x20` the way it bends back to, `+0x34` its ticks, `+0x40` the hand's
//! trail slot, `+0x44` the hand's class, `+0x4c` its size, `+0x54` its arcs out, `+0x58` its tendrils out, `+0x5c` its
//! light). Made ([`spawn`]): updated and drawn at any distance, a third a tick along the way, 120 ticks, 0.01 large, a
//! random turn; a point light (crate::point_lights, `0x2fdb60`: `0.25, 0.14, 1`). Each tick:
//! * **0**: its size eases an eighth of the way to 0.6; it moves ([`fly`]) — a target it met (mode 0x1000, not Ratchet
//!   nor a 0x05ca) takes the row's damage (flags 0x210000, along its way at 1/30, kind `0x09a9 << 16 | 0x0301`); then
//!   every target within 15 all round (crate::moby::aim::search_list_except: 15 at most, no crates, not the classes 0 /
//!   0x09a9) it has not claimed (crate::moby::claims) while fewer than 15 arcs are out: claimed for 120 ticks, an arc
//!   ([`start_arc`], 10 ticks to reach it), the class sound 0. Its ticks out, **2** for 40.
//! * **2**: it moves (a target met takes nothing); its size `ticks/40·0.59 + 0.01`; it jitters ±0.1; two twinkles on it
//!   (crate::particles::twinkle: 0.15 → 1.5 `0x4fff0000`, 0.125 → 1.25 white, falling 0.00139, 60 ticks). Its ticks out,
//!   **3**.
//! * **3**: with no arcs nor tendrils out, the bursts of 6 blue and 6 red sparks ([`burst`]) along the up there, its
//!   claims let go (`0x37f500`), the count down, its light freed, deleted.
//!
//! Then its arcs ([`step_arcs`], `0x3bcfb0`) and tendrils ([`step_tendrils`], `0x3be058`) step and its draw is asked for.
//!
//! **The flight** ([`fly`], `0x3bc640`): its turn x 3° and y −0.75° a tick (z kept); it steps along its velocity,
//! bent 0.075 of the way back toward `+0x20` (`0x321ca8`); a sphere of its size meeting anything (`0x2df8d8`, flags 0)
//! puts it at the pushed-out centre, its velocity reflected off the face (`0x2ef2a8`) and 0.85 of the normal part kept,
//! `+0x20` 0.95 of itself, bursts of 6 blue and 6 red sparks at the contact along the normal, its class sound 1. Its
//! light follows (radius 4, its colour ±0.1). Out of the world's box (2..1021) it goes back and to **3**. Under 8 tendrils
//! out, 8..20 ticks of a new one ([`start_tendril`], `0x3bdde0`).
//!
//! **An arc** (30, `0x23dd20`, 0xa0 bytes: crate::moby::classes::plasma_coil::Arc's record): from the storm to its moby's
//! aim point, growing to it over 10 ticks while its wobble turns 100 times as fast; reached, the moby (with a collision)
//! takes the row's `+0x64` once (as the storm's hit), its class sound 1 and a burst of 9 red sparks back along it; 20
//! ticks more. Its control points wobble (three angles each, ±π, their speeds ±0.001..0.005, amplitudes 3.25..4.75).
//! **A tendril** (16, `0x23efe0`, 0xb0 bytes): from the storm to a point its size away (`0x325100`: a random yaw and
//! pitch) turned by the storm's frame, its control points wobbling (speeds ±0.1..0.5, amplitudes 3.25..4.75 / 4).
//!
//! **The storm's draw** (`0x3bcab0`): five sprites on it (FX 0x2c 0.385..0.415·size `0x0072ff` at 0x80..200, 0.01 back;
//! FX 0x2d 1.225·size, FX 0x29 0.6·size at 4..0x28 and twice FX 0x2a 0.975..1.025·size at 4..0x2d in `0xddb973`); each
//! arc ([`draw_arc`], `0x3bd670`: 16 points along the cubic jittered 0.075 as a [`strip`], fixed alpha 0..0x40 in its
//! last 10 ticks, else 0x40..0x80; reached, FX 0x31 0.5 `0x103080` and 0.3 white at its end, 0x50..0x60); each tendril
//! ([`draw_tendril`], `0x3be270`: 8 points along its cubic, tapering to its end, fixed alpha 0..0x40 in the second half
//! of its life, else 0x40..0x80). The strips (`0x3bd8b0` / `0x3be3d0`): two of FX 0x2b, one 0.005..0.085 and one 0.1..0.25
//! across (twice that for an arc, a quarter for a tendril), their colours running from `0x0072ff` to `0xddb973` and to
//! `0xff0000` along them, fixed alpha `fix` and `fix / 3`.
//!
//! **The hand's draw** (`0x3a29b0`, [`draw_hand`]): the Plasma Coil's (`0x3e9f18`) from its own points (`0x294770`: the
//! ball point 0.4 out along joint 0's x wobbling, `$gp+0x5b0` / `$gp+0x5b4`), its ball point's glints 0.185625·spin and
//! 0.084375·spin, the points beside it from a circle in the hand's x-z plane (`0x3a3838`), its own arcs (`0x3a3568`).
//!
//! Not ported: the gravity frames' trackers and bends (`0x37f538` / `0x37f618` / `0x37f5f8`, `0x380e08` / `0x380ec0`),
//! the lock cheat (`0x18b270`), the weapon mods' effects on a hit (`0x3f9f70`), the GS test register the draws set
//! (`0x37c948`), the ball point's spring (`0x31ec88`: the update resets it every tick, so the draw always snaps), the
//! words `$gp+0x5a8` / `$gp+0x5ac` the shot writes and nothing reads.

use super::plasma_coil::{self, Arc, HandArc, Wobble};
use crate::hero::items::{row, ItemRow, PLASMA_COIL};
use crate::hero::physics::{add, add_rot, cross, dot, euler_rows, len, scale, set_len, sub, sub_rot, unit, V3};
use crate::moby::hits::HitTemplate;
use crate::moby::{aim, countdown, mode, Ctx, MobyWorld};
use crate::point_lights::PointLight;
use rc_formats::moby_anim::JointModifier;

/// The hand's class (rows 0x3e / 0x65 / 0x66 `+0x04`) and the storm's.
pub const HAND_CLASS: i16 = 0x5ca;
pub const BALL: i16 = 0x9a9;
/// `0x3bca38`: the storms out at most.
const MAX_BALLS: i32 = 2;
/// The arcs (`0x23dd20`) and tendrils (`0x23efe0`).
const ARCS: usize = 0x1e;
const TENDRILS: usize = 0x10;
/// The crosshair's FX texture.
const RETICLE: u16 = 0x26;
/// `$gp+0x590` .. `$gp+0x5a0`: the spin's rise and fall a tick, its idle value, the idle rise, the shot's spin.
const SPIN_UP: f32 = 0.1;
const SPIN_DOWN: f32 = 0.01;
const SPIN_IDLE: f32 = 0.1;
const SPIN_IDLE_UP: f32 = 0.1;
const SPIN_SHOT: f32 = 1.0;
/// The shot's speed a tick.
const THIRD: f32 = f32::from_bits(0x3eaa_aaab);
/// `0x294770`: the points about joint 0 (the second, the ball point, rewritten by every draw).
const POINTS: [V3; 5] = [
    [f32::from_bits(0x3dcc_cccd), 0.0, 0.0],
    [0.5, 0.0, 0.0],
    [f32::from_bits(0x3ea2_4dd3), 0.0, f32::from_bits(0xbe6b_851f)],
    [f32::from_bits(0x3ea2_4dd3), f32::from_bits(0xbe42_8f5c), f32::from_bits(0x3de3_53f8)],
    [f32::from_bits(0x3ea2_4dd3), f32::from_bits(0x3e42_8f5c), f32::from_bits(0x3de3_53f8)],
];
/// `0x1afd58`: the classes the storm's search leaves out.
const SEARCH_SKIPS: [i16; 2] = [0, BALL];
/// The storm's light (`0x2fdb60`'s colour, its radius once it flies).
const LIGHT: [f32; 3] = [0.25, f32::from_bits(0x3e0f_5c29), 1.0];
const LIGHT_RADIUS: f32 = 4.0;

/// A tendril of a storm (`+0x00` / `+0x10` its control offsets, `+0x20` / `+0x30` its ends, `+0x40` its end about the
/// storm, `+0x50` / `+0x74` its halves' wobble, `+0x98` / `+0x9c` its ticks and life, `+0xa0` its storm).
#[derive(Clone, Copy, Debug, Default)]
pub struct Tendril {
    pub ctrl: [V3; 2],
    pub from: V3,
    pub to: V3,
    pub local: V3,
    pub halves: [Wobble; 2],
    pub ticks: i32,
    pub total: i32,
    pub owner: Option<usize>,
}

/// The Plasma Storm's globals: the arcs `0x23dd20` and tendrils `0x23efe0`, the hand's arcs `0x23b490`, the phase
/// `$gp+0x5a4`, the first tick's latch `$gp+0x58c`, the empty latch `0x1b27c0`, the storms out `$gp+0xd60`, the ball
/// point's wobble angles `$gp+0x5b0` / `$gp+0x5b4`.
#[derive(Clone, Debug)]
pub struct Mem {
    pub arcs: Vec<Arc>,
    pub tendrils: Vec<Tendril>,
    pub hand_arcs: [HandArc; 4],
    pub phase: i32,
    pub primed: bool,
    pub latch: bool,
    pub count: i32,
    pub wobble: [f32; 2],
}

impl Default for Mem {
    fn default() -> Self {
        Mem {
            arcs: vec![Arc::default(); ARCS],
            tendrils: vec![Tendril::default(); TENDRILS],
            hand_arcs: [HandArc::default(); 4],
            phase: 0,
            primed: false,
            latch: false,
            count: 0,
            // `$gp+0x5b4`'s data: 0.14.
            wobble: [0.0, f32::from_bits(0x3e0f_5c29)],
        }
    }
}

/// The Plasma Coil's row as `0x139568[0x1c]` gives it (crate::upgrades).
fn cur_row(game: &crate::game::GameState) -> Option<&'static ItemRow> { row(game, PLASMA_COIL) }

/// An arc's damage (the row's `+0x64`; crate::hero::items::ItemRow keeps only `+0x60`).
fn arc_damage(game: &crate::game::GameState) -> f32 {
    match cur_row(game).map(|r| r.item) {
        Some(0x65) => 25.0,
        Some(0x66) => 40.0,
        _ => 16.0,
    }
}

/// `0x3a2360` (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].pvars.resize(crate::moby::PVAR_BYTES, 0);
    plasma_coil::spawn_sparks(w, ctx);
    let mut ok = true;
    if ctx.hero.state == 0x1d {
        ok = countdown(&mut ctx.hero.items.look_wait);
    } else {
        ctx.hero.items.look_wait = 0x28;
    }
    match w.mobys[id].state {
        0 => start(w, id, ctx),
        1 => {
            tick(w, id, ctx, ok);
            spin(w, id);
        }
        _ => {}
    }
}

/// The hand's state 0 (module docs).
fn start(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    {
        let m = &mut w.mobys[id];
        m.set_pv_f(0, f32::from_bits(0x3ebb_bbbc));
        m.state = 1;
    }
    w.coil.storm.phase = 0;
    w.coil.storm.primed = false;
    if let Some(a) = ctx.host.anim(&w.mobys[id]) {
        if a.seq_b != 0 || a.flags & 2 != 0 { ctx.host.set_anim(&w.mobys[id], 1, 0, 0); }
    }
    let class = w.mobys[id].o_class;
    {
        let m = &mut w.mobys[id];
        m.set_pv_f(0xc, 0.0);
        m.set_pv_f(0x10, 0.0);
        m.joint_nodes = vec![(0, JointModifier::compose(0))];
    }
    let mut pool = std::mem::take(&mut w.chopper);
    let (slot, _) = pool.slot(w, class);
    w.chopper = pool;
    w.mobys[id].set_pv_i(4, slot as i32);
    let g = &mut ctx.game.rng;
    for a in w.coil.storm.hand_arcs.iter_mut() {
        for h in a.halves.iter_mut() {
            h.angles = [0.0; 3];
            for s in h.speeds.iter_mut() { *s = g.randf_signed(0.05, f32::from_bits(0x3e99_999a)); }
            for k in h.amps.iter_mut() { *k = g.randf(0.25, 0.75); }
        }
        a.ctrl = [[0.0; 3]; 2];
    }
}

/// The hand's state 1 (module docs).
fn tick(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, ok: bool) {
    if !w.coil.storm.primed {
        w.mobys[id].set_pv_f(0xc, 0.0);
        w.coil.storm.primed = true;
    }
    if ctx.hero.state == 1 { ctx.hero.request_state(0x1d, true); }
    if ctx.hero.state == 0x1d {
        let red = w.mobys[id].pv_f(0) != 0.0 && ctx.hero.items.has_ammo(ctx.game);
        crate::hud::marker(&mut ctx.game.markers, 1.0, id, if red { 0x6000_00ff } else { 0x600f_ff0f }, None, RETICLE);
    }
    if plasma_coil::cooldown(w, id) {
        let it = ctx.hero.items;
        if ok && it.no_use == 0 && it.held & it.trigger != 0 && w.coil.storm.count != MAX_BALLS && 0x16 < it.item_ticks {
            if ctx.hero.items.take_ammo(ctx.game, 1) {
                w.coil.storm.latch = true;
                ctx.host.play_sound(&w.mobys[id], 2, 0);
                let wait = 1.0 / cur_row(ctx.game).map_or(1.0, |r| r.rate);
                match w.coil.storm.phase {
                    0 => {
                        w.mobys[id].set_pv_f(0xc, SPIN_IDLE);
                        w.coil.storm.phase = 1;
                        w.mobys[id].set_pv_f(0, wait);
                    }
                    2 => {
                        w.coil.storm.phase = 1;
                        w.mobys[id].set_pv_f(0, wait);
                    }
                    _ => {}
                }
            } else if it.pressed & it.trigger != 0 || w.coil.storm.latch {
                ctx.host.play_sound(&w.mobys[id], 6, 0);
                w.coil.storm.latch = false;
            }
        }
    }
    let s = w.mobys[id].pv_f(0xc);
    match w.coil.storm.phase {
        0 => {
            if s < SPIN_IDLE {
                let n = s + SPIN_IDLE_UP;
                w.mobys[id].set_pv_f(0xc, if SPIN_IDLE < n { SPIN_IDLE } else { n });
            }
        }
        1 => {
            let n = s + SPIN_UP;
            w.mobys[id].set_pv_f(0xc, n);
            if SPIN_SHOT < n { shoot(w, id, ctx); }
        }
        2 => {
            let n = s - SPIN_DOWN;
            if n <= SPIN_IDLE {
                w.mobys[id].set_pv_f(0xc, SPIN_IDLE);
                w.coil.storm.phase = 0;
            } else {
                w.mobys[id].set_pv_f(0xc, n);
            }
        }
        _ => {}
    }
    if w.mobys[id].drawn != 0 { w.register_draw(id); }
}

/// The spin's end of `0x3a2360`: the angle turns spin·π/10 and the node takes its half-angle rotation about x.
fn spin(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    let a = add_rot(m.pv_f(0x10), m.pv_f(0xc) * f32::from_bits(0x3ea0_d97d));
    m.set_pv_f(0x10, a);
    let (s, c) = ((a * 0.5).sin(), (a * 0.5).cos());
    m.set_pv_f(0x30, s);
    m.set_pv_f(0x3c, c);
    if let Some((_, n)) = m.joint_nodes.iter_mut().find(|(l, _)| *l == 0) { n.quat = [s, 0.0, 0.0, c]; }
}

/// The shot of the spin-up phase (module docs).
fn shoot(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let from = ctx.host.joint(&w.mobys[id], 0).unwrap_or(w.mobys[id].pos);
    let mut dir = if ctx.hero.items.raised { set_len(w.mobys[id].rows[1], -THIRD) } else { set_len(ctx.hero.rows[0], THIRD) };
    if let Some((t, _)) = ctx.hero.items.aim_target.filter(|&(t, _)| w.mobys.get(t).is_some()) {
        dir = set_len(sub(aim::aim_point(w, t), from), THIRD);
    }
    let slot = w.mobys[id].pv_i(4);
    if let Some(b) = spawn(w, ctx, id, slot, from, dir) { w.mobys[b].parent = Some(id); }
    w.coil.storm.phase = 2;
}

/// `0x3bc4c8(hand, slot, at, dir)` (module docs).
pub fn spawn(w: &mut MobyWorld, ctx: &mut Ctx, hand: usize, slot: i32, at: V3, dir: V3) -> Option<usize> {
    w.class(BALL)?;
    let hand_class = w.mobys[hand].o_class;
    let id = w.create(BALL);
    {
        let m = &mut w.mobys[id];
        m.update_dist = 0xff;
        m.draw_dist = 0x1ff;
        m.mode |= 0x100;
        m.set_pv_i(0x44, hand_class as i32);
        m.set_pv_i(0x34, 0x78);
        m.set_pv_f(0x4c, f32::from_bits(0x3c23_d70a));
        m.set_pv_i(0x40, slot);
        m.pos = at;
        m.set_pv_v(0, dir);
        m.set_pv_v(0x20, dir);
        m.set_pv_i(0x54, 0);
        m.set_pv_i(0x58, 0);
        m.state = 0;
    }
    w.coil.storm.count += 1;
    let light = w.point_lights.alloc(PointLight { color: LIGHT, intensity: 0.0, pos: at, radius: 0.0 }).map_or(-1, |s| s as i32);
    w.mobys[id].set_pv_i(0x5c, light);
    let g = &mut ctx.game.rng;
    let rot = [g.rand_angle(), g.rand_angle(), g.rand_angle()];
    let m = &mut w.mobys[id];
    m.rot = rot;
    m.rows = euler_rows(rot);
    Some(id)
}

/// `0x3bc078` (module docs).
pub fn update_ball(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].pvars.resize(crate::moby::PVAR_BYTES, 0);
    match w.mobys[id].state {
        2 => {
            fly(w, id, ctx);
            let n = w.mobys[id].pv_i(0x34);
            w.mobys[id].set_pv_f(0x4c, n as f32 / 40.0 * f32::from_bits(0x3f17_0a3e) + 0.01);
            let g = &mut ctx.game.rng;
            let j = [g.randf(-0.1, 0.1), g.randf(-0.1, 0.1), g.randf(-0.1, 0.1)];
            let pos = add(w.mobys[id].pos, j);
            w.mobys[id].pos = pos;
            if let Some(sys) = ctx.parts.as_deref_mut() {
                let g = &mut ctx.game.rng;
                let fall = f32::from_bits(0x3ab6_0b61);
                crate::particles::twinkle(sys, g, f32::from_bits(0x3e19_999a), 1.5, fall, pos, 0x3c, 0x4fff_0000, 0, 2, [0.0; 3], 1);
                crate::particles::twinkle(sys, g, 0.125, 1.25, fall, pos, 0x3c, 0x7fff_ffff, 0, 2, [0.0; 3], 1);
            }
            let mut n = n;
            if countdown(&mut n) { w.mobys[id].state = 3; }
            w.mobys[id].set_pv_i(0x34, n);
        }
        0 | 1 => {
            let s = w.mobys[id].pv_f(0x4c);
            w.mobys[id].set_pv_f(0x4c, s + (0.6 - s) * 0.125);
            if let Some(t) = fly(w, id, ctx) {
                let d = set_len(w.mobys[id].pv_v(0), f32::from_bits(0x3d08_8889));
                let damage = cur_row(ctx.game).map_or(0.0, |r| r.damage);
                let mut tm = HitTemplate::new(damage, Some(id), 0x21_0000, [d[0], d[1], d[2], 0.0]);
                tm.kind = (BALL as u16 as u32) << 16 | 0x0301;
                w.deliver_hit_any(t, &tm, [0.0; 4]);
            }
            let (pos, fwd) = (w.mobys[id].pos, w.mobys[id].rows[0]);
            let tau = f32::from_bits(0x40c9_0fdb);
            let cone = aim::Cone { yaw: tau, pitch: tau, range: 15.0, near_pitch: 0.0, near_yaw: 0.0 };
            let found = aim::search_list_except(w, ctx, cone, pos, fwd, false, 0xf, &[], &SEARCH_SKIPS);
            for t in found {
                if w.mobys[id].pv_i(0x54) < 0xf && w.claims.free(id, t) {
                    w.claims.claim(id, t, 0x78);
                    start_arc(w, ctx, id, t, 10);
                    ctx.host.play_sound(&w.mobys[id], 0, 0);
                }
            }
            let mut n = w.mobys[id].pv_i(0x34);
            if countdown(&mut n) {
                w.mobys[id].state = 2;
                n = 0x28;
            }
            w.mobys[id].set_pv_i(0x34, n);
        }
        3 if w.mobys[id].pv_i(0x54) == 0 && w.mobys[id].pv_i(0x58) == 0 => {
            let mut at = w.mobys[id].pos;
            let up = ctx.hero.gravity.up_at(at);
            burst(ctx, &mut at, up, 6, 0x72ff);
            burst(ctx, &mut at, up, 6, 0xff_0000);
            w.claims.release(id);
            w.coil.storm.count -= 1;
            let light = w.mobys[id].pv_i(0x5c);
            if light != -1 { w.point_lights.free(light as usize); }
            w.delete(id);
            return;
        }
        _ => {}
    }
    step_arcs(w, ctx, id);
    step_tendrils(w, ctx, id);
    w.register_draw(id);
}

/// `0x3bc640(storm)` (module docs): the target it met, if any.
fn fly(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> Option<usize> {
    {
        let m = &mut w.mobys[id];
        m.rot[0] = add_rot(m.rot[0], f32::from_bits(0x3d56_7752));
        m.rot[1] = sub_rot(m.rot[1], f32::from_bits(0x3c56_7752));
        m.rows = euler_rows(m.rot);
    }
    let was = w.mobys[id].pos;
    let mut vel = w.mobys[id].pv_v(0);
    let mut bend = w.mobys[id].pv_v(0x20);
    let mut pos = add(was, vel);
    if len(vel) != 0.0 && len(bend) != 0.0 { vel = crate::quat::turn_toward(f32::from_bits(0x3d99_999a), vel, bend); }
    let size = w.mobys[id].pv_f(0x4c);
    let hit = ctx.coll.and_then(|c| crate::collision::coll_sphere_m(c, ctx.scene, pos, size, crate::collision::QueryFlags(0), Some(id)));
    let mut met = None;
    if let Some(h) = hit {
        if let Some(t) = h.moby.filter(|&t| Some(t) != ctx.hero_moby && w.mobys.get(t).is_some_and(|m| m.o_class != HAND_CLASS && m.mode & mode::TARGET != 0)) { met = Some(t); }
        pos = h.pushed_centre.unwrap_or(h.point);
        let n = unit(h.normal);
        // `0x2ef2a8`: reflected when it runs into the face.
        let d = dot(vel, n);
        if d < 0.0 { vel = sub(vel, scale(n, 2.0 * d)); }
        let along = scale(n, dot(vel, n));
        vel = add(sub(vel, along), scale(along, f32::from_bits(0x3f59_999a)));
        bend = scale(bend, f32::from_bits(0x3f73_3333));
        let mut at = h.point;
        burst(ctx, &mut at, n, 6, 0x72ff);
        burst(ctx, &mut at, n, 6, 0xff_0000);
        ctx.host.play_sound(&w.mobys[id], 1, 0);
    }
    let light = w.mobys[id].pv_i(0x5c);
    if light != -1 {
        let k = ctx.game.rng.randf(-0.1, 0.1);
        w.point_lights.set(light as usize, PointLight { color: LIGHT.map(|c| c + k), intensity: 0.0, pos, radius: LIGHT_RADIUS });
    }
    if pos.iter().any(|&x| !(2.0..=1021.0).contains(&x)) {
        pos = was;
        w.mobys[id].state = 3;
    }
    {
        let m = &mut w.mobys[id];
        m.pos = pos;
        m.set_pv_v(0, vel);
        m.set_pv_v(0x20, bend);
    }
    if w.mobys[id].pv_i(0x58) < 8 {
        let n = ctx.game.rng.rand_range(8, 0x14);
        start_tendril(w, ctx, id, n);
    }
    met
}

/// `0x3be958(at, way, n, rgb)`: `n` pairs of twinkles (crate::particles::twinkle), 1.75..2 large, falling 0.002, 60
/// ticks, flying 0.05 a tick along `way` turned at random (each angle ±90°) and 0.05 up, `at` itself jittered 0..0.15
/// further for each: `rgb` at 0x4f and a white one 0.7 of the size turning the other way; their `+0x09` 0x74.
fn burst(ctx: &mut Ctx, at: &mut V3, way: V3, n: i32, rgb: u32) {
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let g = &mut ctx.game.rng;
    let q = f32::from_bits(0x3fc9_0fdb);
    for _ in 0..n {
        let size = g.randf(1.75, 2.0);
        let r = g.randi(2);
        let s = if r == 0 { -1 } else { r };
        let ey = g.randf(-q, q);
        let ex = g.randf(-q, q);
        let ez = g.randf(-q, q);
        let rows = euler_rows([ex, ey, ez]);
        let mut v = set_len(crate::gravity::to_world(way, &rows), 0.05);
        v[2] += 0.05;
        for k in 0..3 { at[k] += g.randf_signed(0.0, f32::from_bits(0x3e19_999a)); }
        g.rand();
        let fall = f32::from_bits(0x3b03_126f);
        if let Some(i) = crate::particles::twinkle(sys, g, size * 0.1, size, fall, *at, 0x3c, rgb | 0x4f00_0000, 0, s as i8, v, 1) { sys.pool.recs[i][9] = 0x74; }
        if let Some(i) = crate::particles::twinkle(sys, g, size * 0.07, size * 0.7, fall, *at, 0x3c, 0x7fff_ffff, 1, -s as i8, v, 1) { sys.pool.recs[i][9] = 0x74; }
    }
}

/// `0x3bcd00(storm, moby, ticks)`: the first free arc taken from the storm to the moby's aim point.
fn start_arc(w: &mut MobyWorld, ctx: &mut Ctx, ball: usize, target: usize, ticks: i32) {
    let Some(k) = w.coil.storm.arcs.iter().position(|a| a.owner.is_none()) else { return };
    let n = w.mobys[ball].pv_i(0x54);
    w.mobys[ball].set_pv_i(0x54, n + 1);
    let from = w.mobys[ball].pos;
    let g = &mut ctx.game.rng;
    let pi = std::f32::consts::PI;
    let mut halves = [Wobble::default(); 2];
    for h in halves.iter_mut() { for x in h.angles.iter_mut() { *x = g.randf(-pi, pi); } }
    for h in halves.iter_mut() { for x in h.speeds.iter_mut() { *x = g.randf_signed(f32::from_bits(0x3a83_126f), f32::from_bits(0x3ba3_d70a)); } }
    for h in halves.iter_mut() { for x in h.amps.iter_mut() { *x = g.randf(3.25, 4.75); } }
    let to = if aim::health(w, target).is_some() { plasma_coil::aim_at(w, ctx, target) } else { w.mobys[target].pos };
    w.coil.storm.arcs[k] = Arc { ctrl: [[0.0; 3]; 2], from, to, halves, life: 0x14, owner: Some(ball), target: Some(target), grow: ticks, grow_total: ticks, hit: false };
}

/// `0x3bcfb0(storm)` (module docs).
fn step_arcs(w: &mut MobyWorld, ctx: &mut Ctx, ball: usize) {
    let pos = w.mobys[ball].pos;
    for k in 0..ARCS {
        if w.coil.storm.arcs[k].owner != Some(ball) { continue; }
        let mut a = w.coil.storm.arcs[k];
        a.from = pos;
        let f = if a.grow == 0 { 1.0 } else { 100.0 };
        a.ctrl = [a.halves[0].step(f), a.halves[1].step(f)];
        let target = a.target.filter(|&t| w.mobys.get(t).is_some());
        if let Some(t) = target {
            let p = if aim::health(w, t).is_some() { plasma_coil::aim_at(w, ctx, t) } else { w.mobys[t].pos };
            let s = 1.0 - a.grow as f32 / a.grow_total as f32;
            a.to = std::array::from_fn(|i| a.from[i] + (p[i] - a.from[i]) * s);
        }
        if countdown(&mut a.grow) {
            if let Some(t) = target.filter(|&t| w.mobys[t].has_collision) {
                if !a.hit {
                    let d = set_len(sub(a.to, a.from), f32::from_bits(0x3d08_8889));
                    let mut tm = HitTemplate::new(arc_damage(ctx.game), Some(ball), 0x21_0000, [d[0], d[1], d[2], 0.0]);
                    tm.kind = (BALL as u16 as u32) << 16 | 0x0301;
                    w.deliver_hit_any(t, &tm, [0.0; 4]);
                    ctx.host.play_sound(&w.mobys[ball], 1, 0);
                    let back = set_len(d, -1.0);
                    burst(ctx, &mut a.to, back, 9, 0xff_0000);
                    a.hit = true;
                }
            }
            if countdown(&mut a.life) {
                a.owner = None;
                let n = w.mobys[ball].pv_i(0x54);
                w.mobys[ball].set_pv_i(0x54, n - 1);
            }
        }
        w.coil.storm.arcs[k] = a;
    }
}

/// `0x3bdde0(storm, ticks)`: the first free tendril taken for `ticks`.
fn start_tendril(w: &mut MobyWorld, ctx: &mut Ctx, ball: usize, ticks: i32) {
    let Some(k) = w.coil.storm.tendrils.iter().position(|t| t.owner.is_none()) else { return };
    let n = w.mobys[ball].pv_i(0x58);
    w.mobys[ball].set_pv_i(0x58, n + 1);
    let size = w.mobys[ball].pv_f(0x4c);
    let g = &mut ctx.game.rng;
    let pi = std::f32::consts::PI;
    let mut halves = [Wobble::default(); 2];
    for h in halves.iter_mut() { for x in h.angles.iter_mut() { *x = g.randf(-pi, pi); } }
    for h in halves.iter_mut() { for x in h.speeds.iter_mut() { *x = g.randf_signed(0.1, 0.5); } }
    for h in halves.iter_mut() { for x in h.amps.iter_mut() { *x = g.randf(3.25, 4.75) * 0.25; } }
    let (yaw, pitch) = (g.rand_angle(), g.rand_angle());
    // `0x325100(r, yaw, pitch)`.
    let local = [yaw.cos() * size * pitch.cos(), yaw.sin() * size * pitch.cos(), pitch.sin() * size];
    let t = &mut w.coil.storm.tendrils[k];
    *t = Tendril { ctrl: [[0.0; 3]; 2], from: t.from, to: t.to, local, halves, ticks, total: ticks, owner: Some(ball) };
}

/// `0x3be058(storm)` (module docs).
fn step_tendrils(w: &mut MobyWorld, _ctx: &mut Ctx, ball: usize) {
    let (pos, rows) = (w.mobys[ball].pos, w.mobys[ball].rows);
    for k in 0..TENDRILS {
        let t = &mut w.coil.storm.tendrils[k];
        if t.owner != Some(ball) { continue; }
        t.ctrl = [t.halves[0].step(1.0), t.halves[1].step(1.0)];
        t.from = pos;
        t.to = add(crate::gravity::to_world(t.local, &rows), pos);
        if countdown(&mut t.ticks) {
            t.owner = None;
            let n = w.mobys[ball].pv_i(0x58);
            w.mobys[ball].set_pv_i(0x58, n - 1);
        }
    }
}

/// `0x3bcab0` (module docs).
pub fn draw_ball(w: &MobyWorld, id: usize, g: &mut crate::rng::Rng, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let m = &w.mobys[id];
    let (pos, size) = (m.pos, m.pv_f(0x4c));
    let a = g.rand_range(0x80, 200);
    let k = g.randf(f32::from_bits(0x3ec5_1eb9), f32::from_bits(0x3ed4_7ae1));
    let roll = g.rand_angle();
    crate::fx::sprite(out, k * size, f32::from_bits(0xbc23_d70a), roll, pos, cam, 0x2c, (a as u32) << 24 | 0x72ff, true);
    for (k, fx) in [(f32::from_bits(0x3f9c_cccd), 0x2d), (f32::from_bits(0x3f19_999a), 0x29)] {
        let a = g.rand_range(4, 0x28);
        let roll = g.rand_angle();
        crate::fx::sprite(out, size * k, 0.0, roll, pos, cam, fx, (a as u32) << 24 | 0xdd_b973, true);
    }
    for _ in 0..2 {
        let a = g.rand_range(4, 0x2d);
        let k = g.randf(f32::from_bits(0x3f79_999a), f32::from_bits(0x3f83_3333));
        let roll = g.rand_angle();
        crate::fx::sprite(out, k * size, 0.0, roll, pos, cam, 0x2a, (a as u32) << 24 | 0xdd_b973, true);
    }
    // `0x3bd390`: the arcs.
    for a in w.coil.storm.arcs.iter().filter(|a| a.owner == Some(id)) {
        draw_arc(a, g, cam, out);
        if a.grow == 0 {
            let alpha = g.rand_range(0x50, 0x60);
            let roll = g.rand_angle();
            crate::fx::sprite(out, 0.5, f32::from_bits(0x3e19_999a), roll, a.to, cam, 0x31, (alpha as u32) << 24 | 0x10_3080, true);
            let alpha = g.rand_range(0x50, 0x60);
            let roll = g.rand_angle();
            crate::fx::sprite(out, f32::from_bits(0x3e99_999a), f32::from_bits(0x3e1a_9fbe), roll, a.to, cam, 0x31, (alpha as u32) << 24 | 0xff_ffff, true);
        }
    }
    // `0x3be208`: the tendrils.
    for t in w.coil.storm.tendrils.iter().filter(|t| t.owner == Some(id)) { draw_tendril(t, g, cam, out); }
}

/// `0x3bd670(storm, arc)`: 16 points along the arc's cubic jittered 0.075, as a [`strip`] (fixed alpha 0..0x40 in its
/// last 10 ticks, else 0x40..0x80; twice the widths).
fn draw_arc(a: &Arc, g: &mut crate::rng::Rng, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let (ca, cb) = (add(a.ctrl[0], a.from), add(a.ctrl[1], a.to));
    let points: Vec<V3> = (0..0x10)
        .map(|i| {
            let p = plasma_coil::cubic(i as f32 / 15.0, a.from, a.to, ca, cb);
            let v = [g.randf(-0.5, 0.5), g.randf(-0.5, 0.5), g.randf(-0.5, 0.5)];
            add(p, set_len(v, f32::from_bits(0x3d99_999a)))
        })
        .collect();
    let fix = if a.life < 10 { g.rand_range(0, 0x40) } else { g.rand_range(0x40, 0x80) };
    strip(out, g, points, cam, fix, 2.0, false);
}

/// `0x3be270(storm, tendril)`: 8 points along its cubic as a [`strip`] tapering to its end (fixed alpha 0..0x40 in the
/// second half of its life, else 0x40..0x80; a quarter of the widths).
fn draw_tendril(t: &Tendril, g: &mut crate::rng::Rng, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let (ca, cb) = (add(t.ctrl[0], t.from), add(t.ctrl[1], t.to));
    let points: Vec<V3> = (0..8).map(|i| plasma_coil::cubic(i as f32 / 7.0, t.from, t.to, ca, cb)).collect();
    let fix = if t.ticks < t.total >> 1 { g.rand_range(0, 0x40) } else { g.rand_range(0x40, 0x80) };
    strip(out, g, points, cam, fix, 0.25, true);
}

/// `0x3bd8b0` / `0x3be3d0(_, fix, points, n)`: the last point extrapolated (`0x3bd848`), then a strip through the points
/// facing the camera (at each its half-width across `way × (camera − point)`, the way to the next point), two FX 0x2b
/// quads a segment: 0.005..0.085·`k` across from `0x7f0072ff` to `0x7fddb973` and 0.1..0.25·`k` across from
/// `0x400072ff` to `0x40ff0000` (the segment's end colours `0x2f0000(i / (n − 2), …)`) with s stepping a third a
/// segment; their fixed alpha `fix` and `fix / 3`; `taper`: the last segment's end at its point.
fn strip(out: &mut Vec<crate::fx::FxQuad>, g: &mut crate::rng::Rng, mut points: Vec<V3>, cam: V3, fix: i32, k: f32, taper: bool) {
    let wa = g.randf(f32::from_bits(0x3ba3_d70a), f32::from_bits(0x3dae_147b)) * k;
    let wb = g.randf(f32::from_bits(0x3dcc_cccd), 0.25) * k;
    plasma_coil::extrapolate(&mut points);
    let n = points.len();
    if n < 3 { return; }
    let across = |i: usize| {
        let d = unit(sub(points[i + 1], points[i]));
        unit(cross(d, sub(cam, points[i])))
    };
    let edge = |i: usize, w: f32| {
        let c = across(i);
        (add(points[i], scale(c, w)), sub(points[i], scale(c, w)))
    };
    let alpha = |c: u32, a: i32| c & 0xff_ffff | ((a as u32) & 0xff) << 24;
    let (fa, fb) = (fix, fix / 3);
    let mut ca = 0x7f00_72ffu32;
    let mut cb = 0x4000_72ffu32;
    let third = f32::from_bits(0x3ea8_f5c3);
    let (mut a, mut b) = (edge(0, wa), edge(0, wb));
    let mut s = 0.0f32;
    for i in 1..n - 1 {
        let (mut na, mut nb) = (edge(i, wa), edge(i, wb));
        let t = i as f32 / (n as f32 - 2.0);
        let (ea, eb) = (crate::hud::lerp_rgba(t, 0x7f00_72ff, 0x7fdd_b973), crate::hud::lerp_rgba(t, 0x7f00_72ff, 0x40ff_0000));
        if taper && i == n - 2 {
            na = (points[i], points[i]);
            nb = (points[i], points[i]);
        }
        let st = [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]];
        out.push(crate::fx::FxQuad { fx: 0x2b, additive: true, corners: [a.0, a.1, na.0, na.1], st, rgba: [alpha(ca, fa), alpha(ca, fa), alpha(ea, fa), alpha(ea, fa)] });
        out.push(crate::fx::FxQuad { fx: 0x2b, additive: true, corners: [b.0, b.1, nb.0, nb.1], st: [[s, 0.0], [s, 1.0], [s + third, 0.0], [s + third, 1.0]], rgba: [alpha(cb, fb), alpha(cb, fb), alpha(eb, fb), alpha(eb, fb)] });
        s = i as f32 * third;
        if 1.0 <= s { s -= 1.0; }
        (ca, cb) = (ea, eb);
        (a, b) = (na, nb);
    }
}

/// `0x3a29b0` (module docs; the Plasma Coil's `0x3e9f18` with the storm's points, glints and plane).
pub fn draw_hand(w: &mut MobyWorld, host: &dyn crate::moby::MobyHost, id: usize, g: &mut crate::rng::Rng, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    // `0x3a3568`: the four arcs' wobble.
    for a in w.coil.storm.hand_arcs.iter_mut() {
        a.ctrl = [a.halves[0].step(1.0), a.halves[1].step(1.0)];
    }
    // The ball point's wobble (`0x294780`), then its angles step.
    let [a1, a2] = w.coil.storm.wobble;
    w.coil.storm.wobble = [add_rot(a1, f32::from_bits(0x3ed6_7752)), add_rot(a2, f32::from_bits(0x3ed3_7cc6))];
    let mut pts = POINTS;
    pts[1] = [a1.sin() * f32::from_bits(0x3c75_c28f) + f32::from_bits(0x3ecc_cccd), -a2.cos() * f32::from_bits(0x3c77_65fe), a1.cos() * f32::from_bits(0x3c74_1f21)];
    let (spin, rows, p0, ball, prongs, joints) = {
        let m = &w.mobys[id];
        let (rows0, _) = host.joint_matrix(m, 0).unwrap_or((m.rows, m.pos));
        let joint = |j: usize| host.joint_matrix(m, j).map_or(m.pos, |(_, p)| p);
        // `0x3a3fe8(hand, j, out, p)`: joint j's place plus the point turned by joint 0's frame.
        let local = pts.map(|p| crate::gravity::to_world(p, &rows0));
        let at = |j: usize, k: usize| add(joint(j), local[k]);
        (m.pv_f(0xc), m.rows, at(0, 0), at(0, 1), [at(1, 2), at(2, 3), at(3, 4)], [joint(1), joint(2), joint(3)])
    };
    plasma_coil::glints(out, g, p0, false, cam);
    plasma_coil::glint(out, g, spin * f32::from_bits(0x3e3e_147a), ball, 0x7f00_397f, 0x40, cam);
    plasma_coil::glint(out, g, spin * f32::from_bits(0x3dac_cccd), ball, 0x7f00_72ff, 0x78, cam);
    for p in prongs { plasma_coil::glints(out, g, p, true, cam); }
    let arcs = w.coil.storm.hand_arcs;
    plasma_coil::hand_strip(out, g, p0, ball, &arcs[0], cam);
    // `0x3a3838(0.35·spin, hand, ball)`: the normal of two points on a circle about the ball in the hand's x-z plane.
    let r = spin * 0.35;
    let step = std::f32::consts::TAU / 11.0;
    let c = [step, add_rot(step, step)].map(|a| crate::gravity::to_world([a.cos() * r, 0.0, -a.sin() * r], &rows));
    let normal = unit(cross(c[1], c[0]));
    for (k, p) in prongs.into_iter().enumerate() {
        let d = unit(sub(ball, p));
        let d = unit(cross(normal, d));
        let d = unit(cross(normal, d));
        let beside = add(scale(d, spin * 0.45), ball);
        plasma_coil::hand_strip(out, g, p, beside, &arcs[k + 1], cam);
        plasma_coil::glints(out, g, beside, true, cam);
        if g.rand() % 11 == 0 && SPIN_IDLE < spin { plasma_coil::spark(w, g, 0.0, beside); }
    }
    for (k, j) in joints.into_iter().enumerate() {
        plasma_coil::hand_strip(out, g, j, prongs[k], &arcs[3 - k], cam);
        plasma_coil::glints(out, g, j, true, cam);
    }
    if g.rand() % 11 == 0 {
        let i = (g.rand() % 4) as usize;
        let pick = [ball, prongs[0], prongs[1], prongs[2]][i];
        plasma_coil::spark(w, g, f32::from_bits(0x3a83_126f), pick);
    }
}
