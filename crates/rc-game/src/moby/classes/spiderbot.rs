//! The Spiderbot Glove (item 0x20, crate::hero::items::ROWS; L02's record `0x266680`: attachment 6, use sequence 0x2c,
//! 8 bots, the blast's damage 12, texts 0x32f1 / 0x32f2): the hand moby **0x0907** (L02 `0x3c6ab0`, called by the item
//! pass while it is the hand item), the bot it throws **0x0b58** (`0x3f8ff8`, made by `0x3f99d8`), which Ratchet steers
//! through his held state 0x1c and the bot's camera (camera class **0x1b**, `0x457aa0`), and the ends `0x3fb028(on, n)`
//! ([`end`]).
//!
//! **The hand** (pvars: `+0x00` the landing point, `+0x10` its normal, `+0x20` a timer, `+0x24` a landing found, `+0x28`
//! no bot out, `+0x2c` the throw refused): each tick the throw from its joint 0 (`0x385b58(1.6, reach, 8 a second,
//! 11 a second², ..)`: no target — 4.5 ahead of Ratchet, 11 in the look stance 0x1d, or with L1 the camera's look,
//! [`launch`]; Ratchet in the look stance 1 goes to 0x1d) and the arc followed to where it lands ([`landing`],
//! `0x3861b8`: 300 tests ten ticks apart, flags 0x12, a surface of no kind skipped; the point pulled 6% toward the
//! camera), whose marker it draws ([`draw_marker`], `0x3c6eb8`; red `0x700000f0` / `0x500000f0` when refused). The
//! throw is refused outside play or with a scene asked for (`0x1a8f00` / `0x1a8f08`), with no landing, a bot out, a
//! landing surface of a kind (`0x388be0`: 0 / 0xd / 0xe, 1 / 3 / 0xb / 0xc), a landing less than 0.75 up, onto the
//! classes 0x0b21 / 0x0b5b, from the classes 0x0330 / 0x0b21 under Ratchet, or in his states 0x28, 0x3e, 0x27, 0x29,
//! 0x2a, 6, 0x74, 0x81. In the look stance 0x1d it waits out `0x18c354` (else 0x28 again). States: **0** → 1 (a bot
//! may go, no landing yet); **1** ○ pressed (`0x138324 & 0x18b050`, not kept away `0x18c0dc`): one bot (`0x2d77d0`),
//! else its class sound 0 — the landing lag `0x18a000` 0x78, the prompt lease taken (`0x32cc40(6, 0, 1)`), the use
//! stance (`0x2b2960`), **2** for 12 ticks, then the bot made at its joint 0 with the throw's velocity, the hand its
//! parent, class sound 1, the use over (`0x2b2bc8`), **3** for 5 ticks, then 1.
//!
//! **The bot** (pvars: `+0x00` its velocity, `+0x10` / `+0x14` the stick ahead and across, `+0x18` its speed, `+0x20` a
//! timer, `+0x24` the blast's reach, `+0x2c` the last animation frame, `+0x30` the stick's size, `+0x34` facing the
//! stick, `+0x38` its turn; the globals `0x1b4280` its 30 s, `0x1b4284` on a magnetic surface, `0x1b4288` on surface
//! 6, kept in its pvars here). Made (`0x3f99d8`) update and draw distance 0xff, its rows kept (`+0x34 | 0x100`) from
//! Ratchet's angles. Every tick its down (`0x1b4270`): the gravity's, or with the line from 0.375 above to 1 below
//! along its up row (flags 2) on surface 2 the surface's normal reversed; the prompt lease. Then by state:
//! * **0**: its scale 0.00025, its 30 s (0x708 ticks); **1** (flying): it grows by 0.01 a tick to 0.01811, moves by
//!   its velocity under the gravity 11 a second² and the line from its last place (flags 0) lands it on anything but
//!   Ratchet and the hand: there, still, the lease, Ratchet in state 0x1c, the bot's camera ([`camera_switch`]),
//!   sequence 1 (blend 10, its speed 0), **2**. Ratchet in 0x62, 0x6f, 0x64, 0x63, 0x17, 0x18, 0x81, 0x27 or a scene
//!   ends it: the camera let go (`0x457b48`), **7**, the hand told (`0x3c6e98`).
//! * **2**: class sound 2, **3** (steered): the stick ([`stick`], `0x3f9bd8`), the turn toward it in the camera's
//!   frame ([`turn`], `0x3fa790`), the walk ([`walk`], `0x3f9cd8`: its corners, the push out of walls, the hold on the
//!   surface), its footsteps by its animation frame ([`steps`], `0x3f9ad8`); its 30 s out or ○ pressed → **9** (10
//!   ticks); Ratchet out of state 0x1c or a scene: the camera let go, **9** (0 ticks).
//! * **4** (falling): moved under the gravity 22 a second², a 0.2 sphere (else the line) stops it ([`sweep`],
//!   `0x3865e8`: off a wall or a moby of class type 5 it bounces, 85% of the normal speed); landed on a surface of no
//!   kind and not 0xc it walks again (**2**), else **9** (10 ticks); flying faster than 0.4 a tick → **9** (10 ticks).
//! * **9**: the hand told, **5** (the fuse): its timer out, the big explosion (crate::moby::classes::explosion, the
//!   ground its height, along its first row), class sound 1, hidden (`+0x34 | 0x41`), **6** for 50 ticks with Ratchet in
//!   0x1c (else the camera let go and 0): the blast ([`blast`], `0x3faaa0`) its first 21 ticks, its reach growing 16 a
//!   second (at most 8); then **8**. **7**: its timer out, **8**: Ratchet out of 0x1c to 0, the bot's camera let go when
//!   current, gone.
//!
//! Each tick from state 2 on its blob shadow (0.6, crate::shadows).
//!
//! **The camera** (class 0x1b; activate `0x457a90` never, init `0x457470`, update `0x4575e8`, pre nothing: the L02 class
//! table `0x2a6e0c`): `0x457aa0` sets its record's `+0x20` to the bot and switches it in (`0x2db6b8`). Init: `+0x28` /
//! `+0x2c` (−2.5, 1), `+0x30` 2.5 back, `+0x34` 1 up, `+0x60` the bot's up row; 2.5 behind and 1 above the bot looking
//! along its first row. Update ([`camera_update`]): the up — the bot's up row on a magnetic surface, else the world's —
//! eased toward by an eighth of the angle in `+0x60` (`0x3260a0`); the look at a quarter above the bot (the rows from
//! the place before this tick's move); the place `+0x30` across and `+0x34` along `+0x60` from there, the way it is
//! now; pulled in by 16 spheres from 0.075 to 0.5 along the line ([`camera_pull`], `0x457858`, flags 0x10); a tenth of
//! the way there a tick. The let-go (`0x2db5f0`) marks the current camera releasing: the follow camera comes back (a
//! cut). Run here at the end of the bot's own update (the original runs it after the moby loop) where the collision
//! is at hand; crate::cameras shows it.
//!
//! **The ends** (`0x3fb028(on, n)`): a bot out goes to 5 (on: blows up) or 7 (not), the hand told, its timer `n` (−1:
//! 10). The arena master calls `(1, −1)` at a challenge's end, Ratchet's warp `0x2bd368` `(0, 0)`.
//!
//! Not ported: the upgraded glove (item 0x4e: the hand 0x0e7a `0x42e7a0`, its bot 0x0e79 `0x42b978` with its shells
//! `0x430a10` / `0x431858`, its end `0x42e670`: the same machine with its own globals); the 30 s on the screen
//! (`0x3f9818`, the screen callback `0x311b10`: two HUD sprites 0xc1 / 0xc2 at (454, 380·k) and the clock
//! `"%d:%02d:%02d"` of `0x1b4280` at (405, 372·k)); the blast's sphere (`0x3facb0`: the overlay's mesh `0x298dc0` /
//! `0x2987c0` with its 98 scrolling texture offsets `0x2496d0`, 2·reach across); the bolts it pulls in (`0x3fa9a8`:
//! every other frame the bolts within 5, `0x38a9d8(0, 0, bolt, bot)`: crate::moby::classes::bolt flies only to
//! Ratchet); the weapon mods on the blast's hits (`0x422630`); the gravity frames and moving frames (`0x386d68` /
//! `0x386e48`, `0x388638` / `0x3886f0`, `0x326450`), the level-specific refusal under 462.43 (`0x1a8ff4`) and the
//! landing on the lava level's surface 3 (`0x1a79f8`); the camera's blend in (its record's blend kind 1, `0x2db6b8`:
//! a cut here) and its `+0x28` / `+0x2c` set to (−10.75, 4) while the bot blows up (nothing reads them); `0x189ff2`.
//! The quaternion turns (`0x2f3ba8` of an axis not made unit) are made unit here: the original's rows are set to
//! length where they are read.

use crate::cameras::Cameras;
use crate::collision::QueryFlags;
use crate::hero::items::{row, SPIDERBOT};
use crate::hero::physics::{add, cross, dot, len, scale, set_len, sub, unit, V3};
use crate::moby::hits::HitTemplate;
use crate::moby::{aim, countdown, mode, Ctx, MobyWorld};
use crate::quat;

/// The hand's, the bot's and its camera's classes.
pub const HAND_CLASS: i16 = 0x0907;
pub const BOT_CLASS: i16 = 0x0b58;
pub const CAMERA_CLASS: i16 = 0x1b;

/// `0x385b58`'s arguments: the least reach (with L1), the reach ahead (in the look stance 0x1d), the speed across (a
/// tick) and the gravity (a tick²: 11 a second²).
const LEAST: f32 = 1.6;
const REACH: f32 = 4.5;
const REACH_LOOK: f32 = 11.0;
const SPEED: f32 = f32::from_bits(0x3e08_8889);
const GRAVITY: f32 = f32::from_bits(0x3b48_3fb8);
/// The falling bot's gravity (22 a second²).
const FALL_GRAVITY: f32 = f32::from_bits(0x3bc8_3fb8);
/// The marker's colours when the throw is refused (`0x1afe88`).
const MARKER_REFUSED: [u32; 2] = [0x7000_00f0, 0x5000_00f0];
const MARKER_RGBA: [u32; 2] = [0x70f0_f0b0, 0x50f0_f0b0];

/// The hand's pvars.
mod hand {
    pub const LAND: usize = 0x00;
    pub const NORMAL: usize = 0x10;
    pub const TIMER: usize = 0x20;
    pub const FOUND: usize = 0x24;
    pub const READY: usize = 0x28;
    pub const REFUSED: usize = 0x2c;
}

/// The bot's pvars (the port keeps the globals `0x1b4280` / `0x1b4284` / `0x1b4288` and its hand `+0xb8` from 0x40).
mod pv {
    pub const VEL: usize = 0x00;
    pub const AHEAD: usize = 0x10;
    pub const ACROSS: usize = 0x14;
    pub const SPEED: usize = 0x18;
    pub const TIMER: usize = 0x20;
    pub const REACH: usize = 0x24;
    pub const HOLD: usize = 0x28;
    pub const FRAME: usize = 0x2c;
    pub const STICK: usize = 0x30;
    pub const FACING: usize = 0x34;
    pub const TURN: usize = 0x38;
    pub const CLOCK: usize = 0x40;
    pub const MAGNETIC: usize = 0x44;
    pub const SURFACE6: usize = 0x48;
    pub const HAND: usize = 0x4c;
}

/// `gp+0x2090` / `+0x2094`: its height to its middle and its size (the corners' reach, the shadow).
const HALF: f32 = 0.375;
const SIZE: f32 = 0.6;
/// `gp+0x20ac`: the top speed (a second); `+0x20b8` / `+0x20bc`: the hold's pull to the surface and its tilt.
const TOP_SPEED: f32 = 8.0;
const PULL: f32 = 0.5;
const TILT: f32 = 0.25;
/// `0x1b4280`'s start: 30 s.
const CLOCK: i32 = 0x708;
/// The flight's scale: from, growth a tick, at most.
const SCALE_FROM: f32 = f32::from_bits(0x3983_126f);
const SCALE_GROW: f32 = 0.01;
const SCALE_MAX: f32 = 0.018_11;
const SCALE_TO: f32 = f32::from_bits(0x3c94_5b6c);
/// `gp+0x207c` / `+0x2080` / `+0x2088`: the blast's largest reach, its growth a second, its ticks (the hits the first
/// 21).
const BLAST_MAX: f32 = 8.0;
const BLAST_GROWTH: f32 = 16.0;
const BLAST_TICKS: i32 = 50;
/// `t[3]` of the blast's hits (`0x45afdf66`).
const NO_KNOCK_LIMIT: f32 = 5627.987;
/// `gp+0x2038` / `+0x2058`: the animation frames its feet sound at and the class sounds.
const STEPS: [(f32, i32); 8] = [(4.0, 0), (4.5, 3), (6.0, 0), (12.5, 3), (20.0, 3), (21.0, 0), (22.5, 0), (28.5, 3)];
/// `0x1b10b0`: the corners in the order the hold tests them (around the bot).
const ORDER: [usize; 8] = [0, 4, 1, 7, 3, 5, 2, 6];

/// `0x2f2e88(out, a, b)`: `b × a` (the VU's `vopmula b, a; vopmsub a, b`).
fn cr(a: V3, b: V3) -> V3 { cross(b, a) }

/// `0x388be0(surface)`: 2 for 0 / 0xd / 0xe, 1 for 1 / 3 / 0xb / 0xc, else 0 (a surface the bot may be on).
fn surface_kind(s: i32) -> i32 {
    match s {
        0 | 0xe | 0xd => 2,
        1 | 3 | 0xb | 0xc => 1,
        _ => 0,
    }
}

/// The rows turned by the quaternion `(axis·sin a, cos a)` made unit (`0x2f3ba8` + `0x2f3920(rows, R, rows)`).
fn turn_rows(rows: [V3; 3], axis: V3, a: f32) -> [V3; 3] {
    let (s, c) = a.sin_cos();
    let q = [axis[0] * s, axis[1] * s, axis[2] * s, c];
    let l = q.iter().map(|x| x * x).sum::<f32>().sqrt();
    if l <= 0.0 { return rows; }
    quat::turn_rows(&rows, q.map(|x| x / l))
}

/// `0x385b58(1.6, reach, 8/60, 11/3600, out, from, 0, 0)` under plain gravity: ahead of Ratchet by `reach` (with L1
/// held and the item not kept away, along the camera's look, crate::moby::classes::gravity_bomb::look_aim_with), the
/// speed across 8 a second, the rise for the fall.
fn launch(ctx: &Ctx, from: V3, reach: f32) -> V3 {
    let it = ctx.hero.items;
    let (way, reach) = if it.held & crate::pad::button::L1 != 0 && it.no_use == 0 { super::gravity_bomb::look_aim_with(ctx, LEAST, reach) } else { (ctx.hero.rows[0], reach) };
    let to = add(ctx.hero.pos, set_len(way, reach));
    let d = sub(to, from);
    let across = [d[0], d[1], 0.0];
    let mut v = set_len(across, SPEED);
    let t = len(across) / SPEED;
    if 0.0 < t { v[2] = (d[2] + GRAVITY * t * t * 0.5) / t; }
    v
}

/// Where a throw lands: the point, the surface's normal, the moby hit and the surface.
struct Landing {
    point: V3,
    normal: V3,
    moby: Option<usize>,
    surface: i32,
}

/// `0x3861b8(gravity, from, vel, ..)`: the arc from `from` at `vel`, ten ticks between line tests (flags 0x12, not
/// the hand), 299 of them, a hit on a surface of no kind (−1, its kind word not positive) skipped; the point pulled
/// 6% toward the camera. Nothing while Ratchet dies (0x6f) or outside play.
fn landing(w: &MobyWorld, ctx: &Ctx, id: usize, from: V3, vel: V3) -> Option<Landing> {
    if ctx.hero.state == 0x6f || ctx.game.scenes.mode != crate::scene::PLAY { return None; }
    let (mut p, mut v) = (from, vel);
    for _ in 0..299 {
        let a = p;
        for _ in 0..10 {
            p = add(p, v);
            v[2] -= GRAVITY;
        }
        let Some(h) = w.coll_line(ctx, a, p, 0x12, Some(id)) else { continue };
        if h.surface_id() == -1 && h.kind < 1 { continue; }
        let cam = ctx.host.camera_pos();
        return Some(Landing { point: add(cam, scale(sub(h.point, cam), 0.94)), normal: unit(h.normal), moby: h.moby, surface: h.surface_id() });
    }
    None
}

/// `0x3c6ab0`.
pub fn update_hand(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].pvars.resize(crate::moby::PVAR_BYTES, 0);
    let from = ctx.host.joint(&w.mobys[id], 0).unwrap_or(w.mobys[id].pos);
    let hero_state = ctx.hero.state;
    // `0x385b58`: no target; Ratchet's look stance 1 goes to its scripted twin.
    if hero_state == 1 { ctx.hero.request_state(0x1d, true); }
    let vel = launch(ctx, from, if hero_state == 0x1d { REACH_LOOK } else { REACH });
    let found = landing(w, ctx, id, from, vel);
    let mut refused = ctx.game.scenes.mode != crate::scene::PLAY || ctx.game.scenes.pending != crate::scene::NONE;
    {
        let m = &mut w.mobys[id];
        if let Some(l) = &found {
            m.set_pv_v(hand::LAND, l.point);
            m.set_pv_v(hand::NORMAL, l.normal);
        }
        m.set_pv_i(hand::FOUND, found.is_some() as i32);
    }
    let class_of = |t: Option<usize>| t.and_then(|t| w.mobys.get(t)).map(|x| x.o_class);
    let ok = found.as_ref().is_some_and(|l| {
        surface_kind(l.surface) == 0
            && w.mobys[id].pv_i(hand::READY) != 0
            && !matches!(hero_state, 0x28 | 0x3e | 0x27 | 0x29 | 0x2a | 6 | 0x74 | 0x81)
            && !(ctx.hero.ground_moby.is_some() && matches!(ctx.hero.ground_class, 0x330 | 0xb21))
            && !matches!(class_of(l.moby), Some(0xb21 | 0xb5b))
            && 0.75 <= dot(l.normal, [0.0, 0.0, 1.0])
    });
    if !ok { refused = true; }
    w.mobys[id].set_pv_i(hand::REFUSED, refused as i32);
    let look_ok = if hero_state == 0x1d {
        countdown(&mut ctx.hero.items.look_wait)
    } else {
        ctx.hero.items.look_wait = 0x28;
        true
    };
    let mut test = false;
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.state = 1;
            m.set_pv_i(hand::READY, 1);
            m.set_pv_i(hand::FOUND, 0);
            test = true;
        }
        1 => test = true,
        2 => {
            let mut t = w.mobys[id].pv_i(hand::TIMER);
            if countdown(&mut t) {
                if let Some(b) = make_bot(w, ctx, from, vel) { w.mobys[b].set_pv_i(pv::HAND, id as i32); }
                ctx.host.play_sound(&w.mobys[id], 1, 0);
                let idle = ctx.hero.idle_seq();
                ctx.hero.items.use_end(hero_state, ctx.hero_view.seq_b, idle);
                w.mobys[id].state = 3;
                t = 5;
            }
            w.mobys[id].set_pv_i(hand::TIMER, t);
        }
        3 => {
            let mut t = w.mobys[id].pv_i(hand::TIMER);
            if countdown(&mut t) { w.mobys[id].state = 1; }
            w.mobys[id].set_pv_i(hand::TIMER, t);
        }
        _ => {}
    }
    let it = ctx.hero.items;
    if test && it.pressed & it.trigger != 0 && it.no_use == 0 && !refused && look_ok {
        if ctx.hero.items.take_ammo(ctx.game, 1) {
            ctx.hero.land_lag = 0x78;
            ctx.game.prompt.force(6, 0, 1);
            ctx.hero.items.use_start(ctx.hero.group);
            let m = &mut w.mobys[id];
            m.state = 2;
            m.set_pv_i(hand::TIMER, 0xc);
            m.set_pv_i(hand::READY, 0);
        } else {
            ctx.host.play_sound(&w.mobys[id], 0, 0);
        }
    }
    w.register_draw(id);
}

/// `0x3c6eb8` → `0x3857f0(1, landing, normal, colours)`: the marker where the throw lands (refused: red); not in
/// Ratchet's states 0x17..0x1b, 0x6f, 0x62 or in a scene.
pub fn draw_marker(w: &MobyWorld, id: usize, hero_state: i32, mode: i32, frame: i32, out: &mut Vec<crate::fx::FxQuad>) {
    let m = &w.mobys[id];
    if m.pv_i(hand::FOUND) == 0 || matches!(hero_state, 0x17..=0x1b | 0x6f | 0x62) || mode == crate::scene::SCENE { return; }
    let rgba = if m.pv_i(hand::REFUSED) != 0 { MARKER_REFUSED } else { MARKER_RGBA };
    super::gravity_bomb::marker_quads(m.pv_v(hand::LAND), m.pv_v(hand::NORMAL), rgba, frame, out);
}

/// `0x3c6e98`: the hand may throw again (`+0x28`).
fn tell_hand(w: &mut MobyWorld) {
    if let Some(h) = w.mobys.iter().position(|m| m.o_class == HAND_CLASS && m.state < 0x80) { w.mobys[h].set_pv_i(hand::READY, 1); }
}

/// The bot out (`gp+0x2084`).
fn live_bot(w: &MobyWorld) -> Option<usize> { w.mobys.iter().position(|m| m.o_class == BOT_CLASS && m.state < 0x80) }

/// `0x3fb028(on, n)`: a bot out goes to 5 (blows up) or 7 (goes), the hand told, its timer `n` (−1: 10).
pub fn end(w: &mut MobyWorld, on: bool, n: i32) {
    let Some(b) = live_bot(w) else { return };
    let m = &mut w.mobys[b];
    m.state = if on { 5 } else { 7 };
    m.set_pv_i(pv::TIMER, if n == -1 { 10 } else { n });
    tell_hand(w);
}

/// `0x3f99d8(pos, vel)`: the bot (class 0x0b58) at `pos` with `vel`, update and draw distance 0xff, its rows kept
/// from Ratchet's angles.
fn make_bot(w: &mut MobyWorld, ctx: &mut Ctx, pos: V3, vel: V3) -> Option<usize> {
    w.class(BOT_CLASS)?;
    let id = w.create(BOT_CLASS);
    let rot = ctx.hero.rot;
    let m = &mut w.mobys[id];
    m.update = Some(update_bot);
    m.mode = (m.mode & !mode::NO_UPDATE) | mode::KEEP_ROWS;
    m.pvars.resize(crate::moby::PVAR_BYTES, 0);
    m.update_dist = 0xff;
    m.draw_dist = 0xff;
    m.drawn = 1;
    m.state = 0;
    m.pos = pos;
    m.scale *= 0.25;
    m.rot = rot;
    m.rows = crate::hero::physics::euler_rows(rot);
    m.set_pv_v(pv::VEL, vel);
    w.rebuild(id, &*ctx.host);
    w.changed = true;
    Some(id)
}

/// The hero states and the scene that take the flying bot back (`0x3f8ff8` state 1).
fn flight_over(ctx: &Ctx) -> bool {
    matches!(ctx.hero.state, 0x62 | 0x6f | 0x64 | 0x63 | 0x17 | 0x18 | 0x81 | 0x27) || ctx.game.scenes.mode == crate::scene::SCENE
}

/// Ratchet no longer steering (out of 0x1c, or a scene).
fn steer_over(ctx: &Ctx) -> bool { ctx.hero.state != 0x1c || ctx.game.scenes.mode == crate::scene::SCENE }

/// `0x3f8ff8`.
pub fn update_bot(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].pvars.resize(crate::moby::PVAR_BYTES, 0);
    // Its down: the gravity's (`0x3864f0` / `0x32aa88`: up (0, 0, 1)), or on surface 2 the surface's (`0x1b4284`).
    let mut down = [0.0, 0.0, -1.0];
    {
        let (p, up) = (w.mobys[id].pos, w.mobys[id].rows[2]);
        let hit = w.coll_line(ctx, add(p, scale(up, HALF)), sub(p, up), 2, Some(id));
        let m = &mut w.mobys[id];
        match hit {
            None => {
                m.set_pv_i(pv::MAGNETIC, 0);
                m.set_pv_i(pv::SURFACE6, 0);
            }
            Some(h) => {
                let s = h.surface_id();
                if s == 2 {
                    down = scale(unit(h.normal), -1.0);
                    m.set_pv_i(pv::MAGNETIC, 1);
                } else {
                    m.set_pv_i(pv::MAGNETIC, 0);
                }
                m.set_pv_i(pv::SURFACE6, (s == 6) as i32);
            }
        }
    }
    ctx.game.prompt.force(6, 0, 1);
    let mut fuse = false;
    match w.mobys[id].state {
        0 | 1 => {
            if w.mobys[id].state == 0 {
                let m = &mut w.mobys[id];
                m.state = 1;
                m.set_pv_i(pv::TIMER, 0);
                m.scale = SCALE_FROM;
                m.set_pv_i(pv::CLOCK, CLOCK);
            }
            if flight_over(ctx) {
                camera_release(&mut ctx.game.cameras);
                w.mobys[id].state = 7;
                tell_hand(w);
                w.mobys[id].set_pv_i(pv::TIMER, 0);
            } else if fly(w, id, ctx) {
                return;
            }
        }
        2 => {
            w.mobys[id].state = 3;
            ctx.host.play_sound(&w.mobys[id], 2, 0);
            if steer_over(ctx) { give_up(w, id, ctx); }
        }
        3 => {
            stick(w, id, ctx);
            turn(w, id, ctx);
            walk(w, id, ctx, down);
            steps(w, id, ctx);
            let mut c = w.mobys[id].pv_i(pv::CLOCK);
            let out = countdown(&mut c);
            w.mobys[id].set_pv_i(pv::CLOCK, c);
            if out || ctx.hero.items.pressed & crate::pad::button::CIRCLE != 0 {
                w.mobys[id].state = 9;
                w.mobys[id].set_pv_i(pv::TIMER, 10);
            }
            if steer_over(ctx) { give_up(w, id, ctx); }
        }
        4 => fall(w, id, ctx),
        5 => fuse = true,
        6 => {
            blast(w, id, ctx);
            let m = &mut w.mobys[id];
            m.set_pv_f(pv::REACH, m.pv_f(pv::REACH) + BLAST_GROWTH * 0.016_666_668);
            let mut t = m.pv_i(pv::TIMER);
            if countdown(&mut t) && m.pv_i(pv::HOLD) == 0 { m.state = 8; }
            m.set_pv_i(pv::TIMER, t);
        }
        7 => {
            let m = &mut w.mobys[id];
            let mut t = m.pv_i(pv::TIMER);
            if countdown(&mut t) { m.state = 8; }
            m.set_pv_i(pv::TIMER, t);
        }
        8 => {
            if ctx.hero.state == 0x1c { ctx.hero.request_state(0, true); }
            if ctx.game.cameras.current_class() == CAMERA_CLASS { camera_release(&mut ctx.game.cameras); }
            w.delete(id);
            return;
        }
        9 => {
            w.mobys[id].state = 5;
            tell_hand(w);
            fuse = true;
        }
        _ => {}
    }
    if fuse {
        let m = &mut w.mobys[id];
        let mut t = m.pv_i(pv::TIMER);
        let lit = countdown(&mut t);
        m.set_pv_i(pv::TIMER, t);
        if lit {
            let (p, dir) = (m.pos, unit(m.rows[0]));
            super::explosion::start(w, ctx, p[2], p, dir);
            ctx.host.play_sound(&w.mobys[id], 1, 0);
            let steering = ctx.hero.state == 0x1c && ctx.game.scenes.mode != crate::scene::SCENE;
            if !steering { camera_release(&mut ctx.game.cameras); }
            let m = &mut w.mobys[id];
            m.state = 6;
            m.mode |= 0x41;
            m.set_pv_i(pv::TIMER, if steering { BLAST_TICKS } else { 0 });
            m.set_pv_f(pv::REACH, 0.0);
        }
    }
    // `0x321c88(0.6, bot)`: its blob shadow (from state 2 on the moving frame's carry, not ported, comes first).
    crate::shadows::request(w, ctx, SIZE, id);
    w.rebuild(id, &*ctx.host);
    w.changed = true;
    if ctx.game.cameras.current_class() == CAMERA_CLASS && ctx.game.cameras.bot.bot == Some(id) { camera_update(w, ctx, id); }
}

/// The steering over (Ratchet out of 0x1c or a scene): the camera let go, **9** at once.
fn give_up(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    camera_release(&mut ctx.game.cameras);
    w.mobys[id].state = 9;
    w.mobys[id].set_pv_i(pv::TIMER, 0);
}

/// State 1's flight: true when it landed (the update ends there).
fn fly(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    let m = &mut w.mobys[id];
    m.scale += SCALE_GROW;
    if SCALE_MAX < m.scale { m.scale = SCALE_TO; }
    let from = m.pos;
    let mut vel = m.pv_v(pv::VEL);
    m.pos = add(m.pos, vel);
    vel[2] -= GRAVITY;
    m.set_pv_v(pv::VEL, vel);
    let to = m.pos;
    let hit = w.coll_line(ctx, from, to, 0, Some(id));
    let Some(h) = hit else { return false };
    if h.moby.is_some_and(|t| Some(t) == ctx.hero_moby || w.mobys.get(t).is_some_and(|x| x.o_class == HAND_CLASS)) { return false; }
    {
        let m = &mut w.mobys[id];
        m.pos = h.point;
        m.set_pv_v(pv::VEL, [0.0; 3]);
    }
    ctx.game.prompt.force(6, 0, 1);
    ctx.hero.request_state(0x1c, true);
    let (p, rows) = (w.mobys[id].pos, w.mobys[id].rows);
    camera_switch(&mut ctx.game.cameras, id, p, rows);
    w.mobys[id].state = 2;
    ctx.host.set_anim(&w.mobys[id], 1, 0, 10);
    ctx.host.set_anim_speed(&w.mobys[id], 0.0);
    w.rebuild(id, &*ctx.host);
    w.changed = true;
    true
}

/// `0x3f9bd8`: the stick (`0x18bdd0`; its x reversed unless the controls are mirrored, `0x1a7a4c`: never here) as
/// ahead / across, each held to ±1, and its size.
fn stick(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let s = ctx.hero.stick;
    let across = (-s[0]).clamp(-1.0, 1.0);
    let ahead = (-s[1]).clamp(-1.0, 1.0);
    let m = &mut w.mobys[id];
    m.set_pv_f(pv::ACROSS, across);
    m.set_pv_f(pv::AHEAD, ahead);
    m.set_pv_f(pv::STICK, (ahead * ahead + across * across).sqrt());
}

/// `0x3fa790`: the turn toward the stick. The stick's way in the camera's rows (`0x1ba730`, the last view), flattened
/// on the bot's up row; the bot faces it within 0.75; the turn's size `1 − cos`; the rows turned by a tenth of the
/// angle about `way × forward` (none: about the forward row) (`0x2f3ba8` / `0x2f3920`). The turn eases by 0.7 a tick.
fn turn(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let cam = ctx.game.cameras.last_view.1;
    let m = &mut w.mobys[id];
    let v = [m.pv_f(pv::AHEAD), m.pv_f(pv::ACROSS), 0.0];
    m.set_pv_i(pv::FACING, 0);
    m.set_pv_f(pv::TURN, m.pv_f(pv::TURN) * 0.7);
    if len(v) == 0.0 { return; }
    let v = unit(v);
    let v: V3 = std::array::from_fn(|k| v[0] * cam[0][k] + v[1] * cam[1][k] + v[2] * cam[2][k]);
    let up = m.rows[2];
    let v = unit(sub(v, scale(up, dot(v, up))));
    let mut axis = cr(m.rows[0], v);
    if len(axis) == 0.0 { axis = m.rows[0]; }
    let d = dot(m.rows[0], v);
    m.set_pv_f(pv::TURN, 1.0 - d);
    if 0.75 < d { m.set_pv_i(pv::FACING, 1); }
    let a = (std::f32::consts::FRAC_PI_2 - d.clamp(-1.0, 1.0).asin()) * 0.1;
    m.rows = turn_rows(m.rows, axis, a);
}

/// `0x3f9cd8`: the speed — not pushed or not facing the stick, 0.85 of itself; else up by the stick's size / 120 a
/// tick, past 8 a second held at 8 · the stick's size — its animation speed (25 · speed, plus twice the turn when not
/// facing), the move along the forward row; then its corners (`0x3f9e08`), the push out of walls (`0x3f9f80`) and
/// the hold on the surface (`0x3fa2c8`).
fn walk(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, down: V3) {
    let m = &mut w.mobys[id];
    let (mag, facing) = (m.pv_f(pv::STICK), m.pv_i(pv::FACING) != 0);
    let mut speed = m.pv_f(pv::SPEED);
    if mag == 0.0 || !facing {
        speed *= 0.85;
    } else {
        speed += mag * 0.008_333_334;
        if TOP_SPEED * 0.016_666_668 < speed.abs() { speed = TOP_SPEED * mag * 0.016_666_668; }
    }
    m.set_pv_f(pv::SPEED, speed);
    let mut anim = speed * 25.0;
    if !facing { anim += m.pv_f(pv::TURN) + m.pv_f(pv::TURN); }
    m.pos = add(m.pos, set_len(m.rows[0], speed));
    ctx.host.set_anim_speed(&w.mobys[id], anim);
    let corners = corners(w, id, down);
    push_out(w, id, ctx, down);
    hold(w, id, ctx, down, &corners);
}

/// `0x3f9e08`: the eight points the hold tests, 0.6 from the bot (corners: forward or back plus across; then forward,
/// back, across right and left), raised to its middle (0.375 against `down`).
fn corners(w: &MobyWorld, id: usize, down: V3) -> [V3; 8] {
    let m = &w.mobys[id];
    let a = [m.rows[0], scale(m.rows[0], -1.0)];
    let b = [scale(m.rows[1], -1.0), m.rows[1]];
    let raw = [add(a[0], b[0]), add(a[0], b[1]), add(a[1], b[0]), add(a[1], b[1]), a[0], a[1], b[0], b[1]];
    let lift = scale(down, -HALF);
    raw.map(|v| add(add(set_len(v, SIZE), m.pos), lift))
}

/// `0x3f9f80`: moving, out of walls — off surface 6 a 0.6 sphere at its middle against the world moves it by the push
/// out (`0x2e5370`); then 32 lines from its middle, 0.6 long, swept from across through the way it moves (back when
/// its speed is not above 0) to the other side (the across row turned about the up row, `(up·sin a, cos a)` for a in
/// 0..π/2), flags 0: each hit adds how far it went in (a moby of class type 5 blows the bot up: **9**, 10 ticks); it
/// moves back by their mean.
fn push_out(w: &mut MobyWorld, id: usize, ctx: &Ctx, down: V3) {
    let speed = w.mobys[id].pv_f(pv::SPEED);
    if speed.abs() < 0.0001 { return; }
    let back = speed <= 0.0;
    let c = add(w.mobys[id].pos, scale(down, -HALF));
    if w.mobys[id].pv_i(pv::SURFACE6) == 0 {
        if let Some(h) = ctx.coll.and_then(|coll| crate::collision::coll_sphere_m(coll, None, c, SIZE, QueryFlags(0), None)) {
            let p = h.pushed_centre.unwrap_or(c);
            let m = &mut w.mobys[id];
            m.pos = add(m.pos, sub(p, c));
        }
    }
    let (up, across) = (w.mobys[id].rows[2], set_len(w.mobys[id].rows[1], SIZE));
    let mut sum = [0.0; 3];
    let mut n = 0;
    for i in 0..32 {
        let a = i as f32 / 31.0 * std::f32::consts::FRAC_PI_2;
        let s = if back { a.sin() } else { -a.sin() };
        // `q · v · q̄` with q = (up·s, cos a): `quat::rotate` turns by the conjugate.
        let q = [-up[0] * s, -up[1] * s, -up[2] * s, a.cos()];
        let end = add(quat::rotate(q, across), c);
        if let Some(h) = w.coll_line(ctx, c, end, 0, Some(id)) {
            n += 1;
            sum = add(sum, sub(h.point, end));
            let kind = h.moby.and_then(|t| w.mobys.get(t)).and_then(|x| w.class(x.o_class)).map_or(0, |c| c.kind);
            if kind == 5 {
                w.mobys[id].state = 9;
                w.mobys[id].set_pv_i(pv::TIMER, 10);
            }
        }
    }
    if n != 0 {
        let m = &mut w.mobys[id];
        m.pos = add(m.pos, scale(sum, 1.0 / n as f32));
    }
}

/// `0x3fa2c8`: the hold on the surface. From each corner ([`ORDER`]) a line 0.75 along `down` (flags 0x12): a hit on a
/// surface of a kind blows the bot up (**9**, 10 ticks); one at least half up against `down` (any, on a magnetic
/// surface) is kept. With some kept, the mean of the normals of the fans between neighbours (their ways from the bot)
/// tilts the rows toward it (`(n × up)·sin(up·n · 0.125)`), and the bot moves half way to their mean height along
/// `down`. With no hit at all it falls (**4**, its velocity its speed along its forward row).
fn hold(w: &mut MobyWorld, id: usize, ctx: &Ctx, down: V3, corners: &[V3; 8]) {
    let mut kept: Vec<V3> = Vec::new();
    let mut any = false;
    for &k in &ORDER {
        let p = corners[k];
        let Some(h) = w.coll_line(ctx, p, add(p, scale(down, HALF + HALF)), 0x12, Some(id)) else { continue };
        any = true;
        if surface_kind(h.surface_id()) == 0 {
            let n = unit(h.normal);
            if 0.5 < dot(n, scale(down, -1.0)) || w.mobys[id].pv_i(pv::MAGNETIC) != 0 { kept.push(h.point); }
        } else {
            w.mobys[id].state = 9;
            w.mobys[id].set_pv_i(pv::TIMER, 10);
        }
    }
    let m = &mut w.mobys[id];
    if !kept.is_empty() {
        let k = kept.len();
        let inv = 1.0 / k as f32;
        let mut normals = [0.0; 3];
        let mut mean = [0.0; 3];
        for i in 0..k {
            let a = unit(sub(kept[i], m.pos));
            let b = unit(sub(kept[(i + 1) % k], m.pos));
            normals = add(normals, unit(cr(b, a)));
            mean = add(mean, kept[i]);
        }
        let n = unit(scale(normals, inv));
        let up = unit(m.rows[2]);
        let axis = cr(up, n);
        let a = dot(up, n) * TILT * 0.5;
        m.rows = turn_rows(m.rows, axis, a);
        let v = sub(scale(mean, inv), m.pos);
        let target = add(scale(down, dot(v, down)), m.pos);
        m.pos = add(m.pos, scale(sub(target, m.pos), PULL));
    }
    if !any {
        m.state = 4;
        let s = m.pv_f(pv::SPEED);
        m.set_pv_v(pv::VEL, set_len(m.rows[0], s));
    }
}

/// `0x3f9ad8`: its feet's sounds — the animation frame (`0x315998`) crossing one of [`STEPS`] since the last tick
/// plays that class sound.
fn steps(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let f = ctx.host.frame_time(&w.mobys[id]);
    let last = w.mobys[id].pv_f(pv::FRAME);
    for &(t, sound) in &STEPS {
        if (0.0 < f - t || 0.0 < last - t) && (f - t < 0.0 || last - t < 0.0) {
            ctx.host.play_sound(&w.mobys[id], sound, 0);
            break;
        }
    }
    w.mobys[id].set_pv_f(pv::FRAME, f);
}

/// State 4: the fall under 22 a second², then [`sweep`].
fn fall(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let m = &mut w.mobys[id];
    let from = m.pos;
    let mut vel = m.pv_v(pv::VEL);
    let to = add(from, vel);
    vel[2] -= FALL_GRAVITY;
    let (r, at, surface) = sweep(w, ctx, id, from, to, &mut vel);
    let m = &mut w.mobys[id];
    m.pos = at;
    m.set_pv_v(pv::VEL, vel);
    if r == 2 {
        if surface_kind(surface) == 0 && surface != 0xc {
            m.state = 2;
            m.set_pv_v(pv::VEL, [0.0; 3]);
        } else {
            m.state = 9;
            m.set_pv_i(pv::TIMER, 10);
        }
    } else if 0.4 < len(vel) {
        m.state = 9;
        m.set_pv_i(pv::TIMER, 10);
    }
}

/// `0x3865e8` with the bot's block (a 0.2 sphere, flags 0, restitution 0.85, floors 0.75 up, bouncing off class type
/// 5): the sphere at `to` (its pushed-out centre, 1), else the line from `from` (its point, 2). A hit under 0.75 up or
/// a moby of class type 5 bounces `vel` (reflected, 85% of the normal speed kept: 1); else it lands at the point (2).
/// Returns the result, the place and the surface.
fn sweep(w: &MobyWorld, ctx: &Ctx, id: usize, from: V3, to: V3, vel: &mut V3) -> (i32, V3, i32) {
    let Some(c) = ctx.coll else { return (0, to, -1) };
    let hit = crate::collision::coll_sphere_m(c, ctx.scene, to, 0.2, QueryFlags(0), Some(id)).map(|h| (h.pushed_centre.unwrap_or(h.point), h))
        .or_else(|| w.coll_line(ctx, from, to, 0, Some(id)).map(|h| (h.point, h)));
    let Some((at, h)) = hit else { return (0, to, -1) };
    let n = unit(h.normal);
    let kind = h.moby.and_then(|t| w.mobys.get(t)).and_then(|x| w.class(x.o_class)).map_or(0, |c| c.kind);
    if dot([0.0, 0.0, 1.0], n) < 0.75 || kind == 5 {
        let r = sub(*vel, scale(n, 2.0 * dot(*vel, n)));
        let along = scale(n, dot(r, n));
        *vel = add(sub(r, along), scale(along, 0.85));
        return (1, at, h.surface_id());
    }
    (2, h.point, h.surface_id())
}

/// `0x3faaa0`: the blast's hits, its first 21 ticks — every moby with health or one the lines test (mode 0x4000), not
/// Ratchet, within the reach (at most 8) takes the record's damage (`+0x60`: 12) with flags 0x830000 along the line
/// from the bot.
fn blast(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pv_i(pv::TIMER) < BLAST_TICKS - 0x14 { return; }
    let Some(sc) = ctx.scene else { return };
    let (at, r) = (w.mobys[id].pos, w.mobys[id].pv_f(pv::REACH).min(BLAST_MAX));
    let damage = row(SPIDERBOT).map_or(0.0, |r| r.damage);
    for t in crate::collision::coll_sphere_mobys(sc, at, r, QueryFlags(0), Some(id)) {
        let Some(x) = w.mobys.get(t) else { continue };
        if !(aim::health(w, t).is_some() || x.mode & mode::LINE != 0) || Some(t) == ctx.hero_moby { continue; }
        let d = unit(sub(x.pos, at));
        let mut tm = HitTemplate::new(damage, Some(id), 0x0083_0000, [d[0], d[1], d[2], NO_KNOCK_LIMIT]);
        tm.kind = (x.o_class as u16 as u32) << 16 | 0x0305;
        let p = x.pos;
        w.deliver_hit_any(t, &tm, [p[0], p[1], p[2], 1.0]);
    }
}

/// The bot's camera (class 0x1b): its record's words the port keeps here (module docs).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BotCamera {
    /// `+0x20`: the bot it follows.
    pub bot: Option<usize>,
    /// The slot's place and rows.
    pub pos: V3,
    pub rows: [V3; 3],
    /// `+0x24`, `+0x28` / `+0x2c`: set, never read.
    pub ticks: i32,
    pub aim: [f32; 2],
    /// `+0x30` / `+0x34`: across and along its up from the look point.
    pub back: f32,
    pub height: f32,
    /// `+0x60`: its up, eased toward the bot's.
    pub up: V3,
}

/// `0x457aa0(pos, rot, bot)`: the class-0x1b slot follows `bot` and is switched in (`0x2db6b8`: the old one's flags
/// cleared; its blend kind 1 a cut here), then its init (`0x457470`). False without a slot of that class.
pub fn camera_switch(cams: &mut Cameras, bot: usize, bot_pos: V3, bot_rows: [V3; 3]) -> bool {
    let Some(slot) = cams.slots.iter().position(|s| s.class == CAMERA_CLASS) else { return false };
    if let Some(old) = cams.current.and_then(|i| cams.slots.get_mut(i)) {
        old.release = 0;
        old.entered = 0;
    }
    cams.current = Some(slot);
    // `0x457470`.
    let aim = [-2.5, 1.0];
    let behind = set_len(bot_rows[0], aim[0]);
    let pos = add(add(set_len(bot_rows[2], aim[1]), behind), bot_pos);
    let fwd = unit(sub(pos, add(pos, behind)));
    let left = unit(cr(fwd, [0.0, 0.0, 1.0]));
    let up = cr(left, fwd);
    cams.bot = BotCamera { bot: Some(bot), pos, rows: [fwd, left, up], ticks: 0x50, aim, back: aim[0].abs(), height: aim[1], up: bot_rows[2] };
    true
}

/// `0x457b48` → `0x2db5f0`: the current camera releasing (`+0x7e` = 1, a cut back), not entered.
pub fn camera_release(cams: &mut Cameras) {
    if let Some(s) = cams.current.and_then(|i| cams.slots.get_mut(i)) {
        s.release = 1;
        s.entered = 0;
    }
}

/// `0x4575e8`: the bot's camera's update (module docs).
fn camera_update(w: &MobyWorld, ctx: &mut Ctx, id: usize) {
    let (bpos, brows) = (w.mobys[id].pos, w.mobys[id].rows);
    let magnetic = w.mobys[id].pv_i(pv::MAGNETIC) != 0;
    let c = ctx.game.cameras.bot;
    let up = if magnetic { brows[2] } else { [0.0, 0.0, 1.0] };
    let look = add(set_len(brows[2], 0.25), bpos);
    let d = sub(look, c.pos);
    let fwd = unit(d);
    let left = unit(cr(fwd, up));
    let rows = [fwd, left, cr(left, fwd)];
    let eased = ease_toward(0.125, c.up, brows[2]);
    let away = scale(d, -1.0);
    let along = scale(eased, dot(away, eased));
    let across = sub(away, along);
    let wanted = add(look, add(set_len(along, c.height), set_len(across, c.back)));
    let at = camera_pull(ctx, id, &c, look, wanted);
    let b = &mut ctx.game.cameras.bot;
    b.rows = rows;
    b.up = eased;
    b.pos = add(b.pos, scale(sub(at, b.pos), 0.1));
}

/// `0x3260a0(t, out, v, to, 0)`: `v` turned toward `to` by `t` of the angle between them (none when either is 0).
fn ease_toward(t: f32, v: V3, to: V3) -> V3 {
    let (lv, lt) = (len(v), len(to));
    if lv * lt == 0.0 { return v; }
    let axis = unit(cr(to, v));
    let a = (dot(v, to) / (lv * lt)).clamp(-1.0, 1.0).acos() * t;
    // `0x325f20`: `q · v · q̄` with q = (axis·sin(a/2), cos(a/2)); `quat::rotate` turns by the conjugate.
    let (s, co) = (a * 0.5).sin_cos();
    quat::rotate([-axis[0] * s, -axis[1] * s, -axis[2] * s, co], v)
}

/// `0x457858`: 16 spheres along the line from `look` to `wanted` (at a tenth .. the whole of it, 0.075 .. 0.5 across,
/// flags 0x10, not the bot): where one touches within its size of the line (`0x321390`: the line's nearest point),
/// the place moves out to that size from the touch and back to the camera's distance from `look`.
fn camera_pull(ctx: &Ctx, id: usize, c: &BotCamera, look: V3, mut wanted: V3) -> V3 {
    const NEAR: f32 = 0.075;
    const FAR: f32 = 0.5;
    const STEPS: usize = 16;
    let Some(coll) = ctx.coll else { return wanted };
    let dist = (c.back * c.back + c.height * c.height).sqrt();
    let mut out = wanted;
    for i in 0..STEPS {
        let t = i as f32 / (STEPS - 1) as f32 * 0.9 + 0.1;
        let p = add(look, scale(sub(wanted, look), t));
        let Some(h) = crate::collision::coll_sphere_m(coll, ctx.scene, p, NEAR + (FAR - NEAR) * t, QueryFlags(0x10), Some(id)) else { continue };
        let ab = sub(wanted, look);
        let l2 = dot(ab, ab);
        let f = if l2 <= 0.0 { 0.0 } else { (dot(sub(h.point, look), ab) / l2).clamp(0.0, 1.0) };
        let near = add(look, scale(ab, f));
        let dv = sub(near, h.point);
        let r = NEAR + (FAR - NEAR) * f;
        if len(dv) <= r {
            let a = add(h.point, set_len(dv, r));
            wanted = add(look, set_len(sub(a, look), dist));
            out = wanted;
        }
    }
    out
}
