//! The Chopper (item 0x16, crate::hero::items::ROWS; L00's record `0x263840`: attachment 6, stance 0, use sequence
//! 0x8f, 30 a second, 1.8 a second, damage 1, 35 blades, name 0x32e2): the hand moby **0x0879** (`0x3b7038`, called by
//! the item pass while it is the hand item) and the blade it throws **0x087a** (`0x41e080`, made by `0x41d7d0`).
//!
//! **The hand** (pvars: `+0x00` its trail slot, `+0x0c` the cooldown (seconds), `+0x10` the jump throw's wait, `+0x14`
//! the lock, `+0x18` the last search's pick, `+0x1c` (s16) the ticks it must stay picked, `+0x1e` (s16) the lock's
//! hold). Each tick first its aim ([`aim`], `0x3b7598`), then in first person or the look stance (1 / 0x1d) the
//! screen-centre crosshair (`0x2db498`, FX 0x26: red `0x600000ff` while the cooldown runs with blades left, else green
//! `0x600fff0f`). State **0** takes the class's trail slot ([`Pool::slot`], `0x32b1b8`) and goes to **1**. In 1 and 2,
//! outside first person and the look stance's twin 0x1d `0x18c354` is set to 0x28, else counted down (a throw from
//! there waits it out). **1**: the cooldown runs down; a blade goes
//! * from first person or the look stance (1 / 0x1d): the cooldown out, ○ held (`0x18b050`), in the hand past 0x16
//!   ticks, a blade left: the cooldown 1 / the row's rate;
//! * from the throw 0x83 (crate::hero::melee: ○ on the ground, `0x2c96d0`'s case 0x16), no blend running, its
//!   sequence passing frame 33, a blade left;
//! * in a jump (group 4, not landed), the cooldown at 0, ○ held, past 0x16 ticks, a blade left: the use stance
//!   (`0x2af5e0`: the firing arm's layer, sequence 0x8f), the cooldown, **2** for 8 ticks; then the blade, back to 1.
//!
//! A blade: one ammo, the hand's class sound 2, the blade made (`0x41d7d0(hand, its place, from the look, the trail
//! slot, the lock, 1)`, [`spawn`]), the hand its owner (`+0xb8`).
//!
//! **The aim** (`0x3b7598`): the hold counts down; ready, outside first person, the arm not raised at a wall
//! (`0x18c30c`) and between holds, the search (crate::moby::aim, no crates) from 0.75 up Ratchet's up row along his
//! yaw and pitch, 30 far, 10° (17.5° on Oozla while the lock is a 0x0a8d / 0x0a8e) by 20°, 35° by 45° near him: the
//! same pick 4 ticks running becomes the lock, held 2 ticks; none clears it; in first person there is none. A lock
//! shows its green reticle (FX 0x26, `0xff0fff0f`) on its aim point, `|0.9 − d·0.0067·0.65|` large at the camera's
//! distance d.
//!
//! **The blade** (pvars: `+0x00` its velocity (a tick), `+0x10` (s16) its life (300), `+0x12` (s16) the homing's
//! ramp, `+0x14` its target, `+0x18` (s16) the ticks it flies free after a hit or a pass, `+0x1a` (s16) past its
//! target, `+0x1c` the next bounce's kind (1: the first), `+0x20` the homing's spring, `+0x28` its spin, `+0x2c` (s16)
//! the mobys it hit, `+0x30` its trail (−1 none), `+0x34` (s16) search again, `+0x36` (s16) homing on its home point,
//! `+0x38` its loop sound, `+0x3c` the hand's trail slot, `+0x40` its home point, `+0x50` the hand's class, `+0x54` the
//! trail's points, `+0x58` its tilt (−70° at first), `+0x60` the lightning's refresh, `+0x64` the glow's phase, `+0x68`
//! thrown from the hand (not a split), `+0x70` its place at the throw; from [`TRAIL`] its trail's points, from
//! [`BOLT`] the lightning's).
//!
//! Made ([`spawn`]): 0.3 up, 0.3 left and 0.5 ahead of the hand in Ratchet's frame (a wall on the way from his chest
//! pulls it a quarter of the way back), flying the row's speed along his yaw (the look: the camera's yaw and its pitch
//! + 7°, which the throw's yaw and pitch `0x18aa24` / `0x18aa28` keep) turned as he is (the look: the camera); a
//! moby of class type 5 (or a crate) within 1 is its target at once, the blade on its aim point; else the hand's lock
//! (a split: its own search, 30 far, 10° by 20°); with one it is turned left by up to 25° (all of it at 30 away),
//! without one it goes for a home point 64 ahead of Ratchet (the look: of the camera), turned left 25° unless thrown
//! from the look stance with Ratchet hidden. Its trail (25 a slot, [`Pool`]) when the next is free, with two
//! after-images (crate::moby::afterimage: alpha 0x30 two ticks back, 0x17 three); tilted −70°.
//!
//! Each tick ([`update_blade`]): its home point turned about Ratchet with the platform's turn (`0x189f7c`); its life
//! out → **1**; a search asked for (all round, 12 far, crates too) gives a new target; a live target (drawn, in the
//! target list, health left) is homed on ([`steer`], `0x41f3f0`) unless the free ticks run; without one the home point
//! while it is ahead. Then it moves, out of the world's 2..1021 box it is gone, and the step's line (flags 0x10; 0x12
//! and no damage while it flies free) hits a moby with the row's damage (flags 0x10000, its kind the class's): at the
//! point, ten sparks ([`sparks`], `0x41f668`), the class sound 2 (a moby of health: 3, or 4 for a health block's
//! `+0x09` other than 1); its target killed by it (health at most the damage) has it search again; the third moby
//! (the Mega Chopper's second) ends it. Off the first moby it turns 10..35° about its up row × its way; off a moby
//! again it is reflected off the moby's up row, off a wall off the face (then 0.2 out, and a search when it has no
//! target); after a moby it flies free 10 ticks. Passing its target (it behind) gives 10 free ticks. Its rows face its
//! way (up the up there), tilted toward the turn (`(−turn)·60°`, ±4° with none, 5% a tick) and spun 24° a tick; its
//! speed the row's, down to 0.25 a tick in a hard turn. Its trail's points shift (40 at most) with this tick's start
//! newest; every 4 ticks the lightning's seven points are drawn anew across it, between times mirrored through it.
//! Its glow word pulses (phase 12° a tick) between `0x0a0aff` and `0x7f7fff` (the Mega's: `0xff0a0a` / `0xff7f7f`,
//! a split's `0xff7fff`). The after-images follow; its loop sound 1 plays. **1**: the trail and the sound let go, a
//! smoke burst (`0x3210a8(0.5, 5)`), gone.
//!
//! **The draw** ([`draw`], `0x41f9a8` → `0x41fb48`): from 3 points the trail as a ribbon (FX 0xb, additive) 0.5
//! wide (a split's 0.25) across the blade's up row, `0x202080` at alpha 0x50 (the Mega's `0x802020` / a split's
//! `0x802080` at 0x7f) fading in over its first 20 ticks; the Mega's lightning through its seven points
//! (crate::fx::ribbon: FX 0x17 and 0xe three times as wide, `0xff5050` / `0xff1010` at a random 0.5..1 of 0x7f / 0x30).
//!
//! Not ported: the weapon mods' effects on a hit (`0x3f9f70`), the gravity frames' bends (`0x380e08`, `0x37f538` /
//! `0x37f618`, the velocity flattened along the up), the lock a cheat or the Lock-On mod holds (`0x18b270`), the
//! after-images' one-ghost setting (`0x1adb0c`: two here), the trail slot's other users (the Lava Gun's class
//! `0x41b198`: their eviction frees this slot's after-images, [`Pool::slot`]). The Mega Chopper (row 0x41, class 0x0af6,
//! the same updates) waits for the weapons' upgrade system: [`level`] reads 0.

use crate::collision::QueryFlags;
use crate::hero::items::{row, ItemRow, CHOPPER};
use crate::hero::physics::{add, add_rot, cross, dist, dot, len, scale, set_len, sub, sub_rot, unit, V3};
use crate::moby::afterimage::Record;
use crate::moby::hits::HitTemplate;
use crate::moby::{aim, countdown, mode, Ctx, MobyWorld};

/// The hand's class (item 0x16's row `+0x04`) and the blade's (`+0x50`).
pub const HAND_CLASS: i16 = 0x879;
pub const BLADE: i16 = 0x87a;
/// The trails a slot holds.
pub const TRAILS: usize = 25;
/// The blade's pvars: its trail's points (40 of `{x, y, z, w}`), then the lightning's seven.
pub const TRAIL: usize = 0x80;
pub const TRAIL_POINTS: usize = 40;
pub const BOLT: usize = TRAIL + 0x10 * TRAIL_POINTS;
const BLADE_PVARS: usize = BOLT + 0x10 * 7;
/// `t[3]` of a blade's hit template (`0x45afdf66`).
const NO_KNOCK_LIMIT: f32 = 5627.925;
/// The crosshair's and the reticle's FX texture.
const RETICLE: u16 = 0x26;

/// The trail slots (`0x268da0`, two of 0xa430 bytes: a class's scratch for its projectiles; `0x32b1b8` /
/// `0x32b3b0` / `0x32b3e0`): the class each holds, and per slot the 25 trails' use flags and after-images; the
/// next trail a blade takes (`0x1b2cdc`).
#[derive(Clone, Debug)]
pub struct Pool {
    /// `0x268da4 + 0xa430·k` (−1: free) and `0x1a902c`, the slot looked at first.
    pub owner: [i16; 2],
    pub next: usize,
    pub used: [[bool; TRAILS]; 2],
    pub ghosts: [Vec<Record>; 2],
    pub index: usize,
}

impl Default for Pool {
    fn default() -> Self { Pool { owner: [-1; 2], next: 0, used: [[false; TRAILS]; 2], ghosts: [vec![Record::default(); TRAILS], vec![Record::default(); TRAILS]], index: 0 } }
}

impl Pool {
    /// `0x32b1b8(hand, 0x3b7528, &found)`: the slot for `class` — the next one looked at that is free or already its
    /// own (found: the latter), else the next one taken from its class (`0x3b7528`: its trails' after-images freed).
    /// The next slot looked at moves on.
    pub fn slot(&mut self, w: &mut MobyWorld, class: i16) -> (usize, bool) {
        for k in 0..2 {
            let s = (self.next + k) % 2;
            if self.owner[s] == -1 || self.owner[s] == class {
                let found = self.owner[s] != -1;
                self.owner[s] = class;
                self.next = (s + 1) % 2;
                return (s, found);
            }
        }
        let s = self.next;
        for i in 0..TRAILS {
            if self.used[s][i] { self.ghosts[s][i].free(w); }
        }
        self.owner[s] = class;
        self.next = (s + 1) % 2;
        (s, false)
    }

    /// `0x32b3b0(class, slot)`: the slot is still `class`'s.
    pub fn holds(&self, class: i16, slot: usize) -> bool { self.owner.get(slot) == Some(&class) }
}

/// The Chopper's row as `0x139568[0x16]` gives it (the port has the base row only).
fn cur_row() -> Option<&'static ItemRow> { row(CHOPPER) }

/// The upgrade level of the Chopper's row (`0x2624f4 + 0xe0·0x139568[0x16]`: 0 the Chopper, 1 the Mega Chopper). The
/// port keeps no upgrade rows yet.
pub fn level(_game: &crate::game::GameState) -> u8 { 0 }

/// `0x321c00(a, out, v, axis)`: `v` turned by `a` about `axis` (made unit; under 1e-5 none).
fn turn(v: V3, axis: V3, a: f32) -> V3 {
    if a.abs() < 1e-5 || len(axis) == 0.0 { return v; }
    crate::gravity::rotate(v, unit(axis), a)
}

/// `0x2eef88(out, a, b)` = b × a.
fn cross_ba(a: V3, b: V3) -> V3 { cross(b, a) }

/// `0x3b7038`.
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].pvars.resize(crate::moby::PVAR_BYTES, 0);
    aim(w, id, ctx);
    let fp = ctx.hero.items.fp.on;
    let hero_state = ctx.hero.state;
    if fp || matches!(hero_state, 1 | 0x1d) {
        let red = 0.0 < w.mobys[id].pv_f(0xc) && ammo(ctx);
        crate::hud::marker(&mut ctx.game.markers, 1.0, id, if red { 0x6000_00ff } else { 0x600f_ff0f }, None, RETICLE);
    }
    let Some(r) = cur_row().copied() else { return };
    match w.mobys[id].state {
        0 => {
            let class = w.mobys[id].o_class;
            let mut pool = std::mem::take(&mut w.chopper);
            let (slot, found) = pool.slot(w, class);
            if !found { pool.used[slot] = [false; TRAILS]; }
            w.chopper = pool;
            let m = &mut w.mobys[id];
            m.set_pv_i(0, slot as i32);
            m.state = 1;
            m.set_pv_s(0x1c, 4);
            m.set_pv_s(0x1e, 0);
            m.set_pv_f(0xc, 0.0);
            m.set_pv_i(0x14, 0);
            m.set_pv_i(0x18, 0);
        }
        1 | 2 => {
            let ok = if !fp && hero_state != 0x1d {
                ctx.hero.items.look_wait = 0x28;
                true
            } else {
                countdown(&mut ctx.hero.items.look_wait)
            };
            let it = &ctx.hero.items;
            let (held, ticks) = (it.held, it.item_ticks);
            let (mut look, mut throw, mut jump) = (false, false, false);
            if w.mobys[id].state == 1 {
                let out = cooldown(w, id);
                if out && ok && held & it.trigger != 0 && 0x16 < ticks && (fp || matches!(hero_state, 1 | 0x1d)) && ammo(ctx) {
                    look = true;
                    w.mobys[id].set_pv_f(0xc, 1.0 / r.rate);
                }
                let v = &ctx.hero_view;
                throw = hero_state == 0x83 && !v.blending() && crate::hero::melee::passed(v, 33.0) && ammo(ctx);
                let h = &*ctx.hero;
                if h.group == 4 && ok && h.jump.landed == 0 && w.mobys[id].pv_f(0xc) == 0.0 && held & crate::pad::button::CIRCLE != 0 && 0x16 < ticks && ammo(ctx) {
                    // `0x2af5e0`, then the firing arm's layer started over at its frame 0 (`0x3122c8`): a fresh start.
                    ctx.hero.items.use_start(4);
                    let m = &mut w.mobys[id];
                    m.state = 2;
                    m.set_pv_i(0x10, 8);
                    m.set_pv_f(0xc, 1.0 / r.rate);
                }
            } else {
                let mut n = w.mobys[id].pv_i(0x10);
                if countdown(&mut n) {
                    jump = true;
                    w.mobys[id].state = 1;
                }
                w.mobys[id].set_pv_i(0x10, n);
            }
            if look || throw || jump {
                ctx.hero.items.take_ammo(ctx.game, 1);
                ctx.host.play_sound(&w.mobys[id], 2, 0);
                let (at, slot, lock) = (w.mobys[id].pos, w.mobys[id].pv_i(0) as usize, w.mobys[id].pv_link(0x14));
                if let Some(b) = spawn(w, ctx, id, at, look, slot, lock, true) { w.mobys[b].parent = Some(id); }
            }
        }
        _ => {}
    }
}

/// `0x2d43e0(0x16)`: a blade left.
fn ammo(ctx: &Ctx) -> bool { ctx.game.ammo.get(CHOPPER as usize).is_some_and(|&a| a != 0) }

/// `0x2eeb40(+0x0c)`: the cooldown down a frame (1/60 s); true when out.
fn cooldown(w: &mut MobyWorld, id: usize) -> bool {
    let c = w.mobys[id].pv_f(0xc);
    if 1.0 / 60.0 < c {
        w.mobys[id].set_pv_f(0xc, c - 1.0 / 60.0);
        false
    } else {
        w.mobys[id].set_pv_f(0xc, 0.0);
        true
    }
}

/// `0x3b7598` (module docs).
fn aim(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    {
        let m = &mut w.mobys[id];
        let mut hold = m.pv_s(0x1e) as i32;
        countdown(&mut hold);
        m.set_pv_s(0x1e, hold as i16);
    }
    let fp = ctx.hero.items.fp.on;
    if fp {
        w.mobys[id].set_pv_i(0x14, 0);
    } else if w.mobys[id].state == 1 && !ctx.hero.items.raised && w.mobys[id].pv_s(0x1e) == 0 {
        let h = &*ctx.hero;
        let from = add(h.pos, scale(h.rows[2], 0.75));
        let dir = crate::moby::classes::shot::polar(1.0, h.rot[2], h.rot[1]);
        let wide = ctx.game.level == 1 && w.mobys[id].pv_link(0x14).and_then(|t| w.mobys.get(t)).is_some_and(|t| (0xa8d..=0xa8e).contains(&t.o_class));
        let cone = aim::Cone { yaw: if wide { 0.305_432_68 } else { 0.174_532_95 }, pitch: 0.349_065_9, range: 30.0, near_pitch: 0.785_398_2, near_yaw: 0.610_865_3 };
        let found = aim::search(w, ctx, cone, from, dir, false);
        let m = &mut w.mobys[id];
        match found {
            None => {
                m.set_pv_i(0x14, 0);
                m.set_pv_s(0x1c, 4);
            }
            Some(f) if Some(f) != m.pv_link(0x18) => m.set_pv_s(0x1c, 4),
            Some(f) => {
                let mut d = m.pv_s(0x1c) as i32;
                let done = countdown(&mut d);
                m.set_pv_s(0x1c, d as i16);
                if done {
                    m.set_pv_i(0x14, f as i32);
                    m.set_pv_s(0x1e, 2);
                }
            }
        }
        m.set_pv_i(0x18, found.map_or(0, |f| f as i32));
    }
    let Some(t) = w.mobys[id].pv_link(0x14).filter(|&t| w.mobys.get(t).is_some()) else { return };
    let at = aim::aim_point(w, t);
    let d = dist(ctx.host.camera_pos(), at);
    crate::hud::marker(&mut ctx.game.markers, (0.9 - d * 0.006_666_667 * 0.65).abs(), id, 0xff0f_ff0f, Some(at), RETICLE);
}

/// `0x41d7d0(hand, at, look, slot, lock, first)` (module docs): the blade, if its class and the hand's trail slot are
/// there.
#[allow(clippy::too_many_arguments)]
pub fn spawn(w: &mut MobyWorld, ctx: &mut Ctx, hand: usize, at: V3, look: bool, slot: usize, lock: Option<usize>, first: bool) -> Option<usize> {
    if w.class(BLADE).is_none() { return None; }
    let r = *cur_row()?;
    let hand_class = w.mobys.get(hand)?.o_class;
    let id = w.create(BLADE);
    let lv = level(ctx.game);
    {
        let (hp, hrot, hlight) = (w.mobys[hand].pos, w.mobys[hand].rot, w.mobys[hand].light);
        let m = &mut w.mobys[id];
        m.pvars.resize(BLADE_PVARS, 0);
        m.set_pv_s(0x12, 10);
        m.set_pv_s(0x2e, 2);
        m.set_pv_s(0x10, 300);
        m.set_pv_i(0x38, -1);
        m.set_pv_i(0x3c, slot as i32);
        m.set_pv_i(0x68, first as i32);
        m.set_pv_i(0x50, hand_class as i32);
        m.set_pv_f(0x58, -1.221_730_6);
        // `0x32a748(blade, hand)`: update and draw distance 0xff, drawn, the owner, its place, angles and light word.
        m.update_dist = 0xff;
        m.draw_dist = 0xff;
        m.drawn = 1;
        m.parent = Some(hand);
        (m.pos, m.rot, m.light) = (hp, hrot, hlight);
        m.glow = if lv == 0 { 0xff0a_0aff } else { 0xffff_0a0a };
    }
    let h = &*ctx.hero;
    let (hero_pos, hero_rows) = (h.pos, h.rows);
    if !first {
        w.mobys[id].pos = at;
    } else {
        let mut p = w.mobys[id].pos;
        p = add(p, scale(hero_rows[2], 0.3));
        p = add(p, scale(hero_rows[1], 0.3));
        p = add(p, scale(hero_rows[0], 0.5));
        let from = add(hero_pos, scale(hero_rows[2], 0.3));
        if ctx.coll.and_then(|c| crate::collision::coll_line_m(c, ctx.scene, from, p, QueryFlags(0x10), ctx.hero_moby)).is_some() {
            p = add(p, scale(sub(from, p), 0.75));
        }
        w.mobys[id].pos = p;
    }
    let pos = w.mobys[id].pos;
    let speed = r.speed * (1.0 / 60.0);
    let (vel, rows) = if !look {
        // `0x2d3b38`: along his yaw (in the gravity frames his forward row).
        let v = if ctx.hero.gravity.mode != 0 { scale(hero_rows[0], speed) } else { [ctx.hero.rot[2].cos() * speed, ctx.hero.rot[2].sin() * speed, 0.0] };
        (v, hero_rows)
    } else {
        let cr = ctx.host.camera_rows();
        let f = cr[0];
        let (yaw, up) = (f[1].atan2(f[0]), f[2].atan2((f[0] * f[0] + f[1] * f[1]).sqrt()));
        let pitch = add_rot(up, 0.122_173_06);
        ctx.hero.melee.throw_yaw = yaw;
        ctx.hero.melee.throw_pitch = pitch;
        (crate::moby::classes::shot::polar(speed, yaw, pitch), cr)
    };
    {
        let m = &mut w.mobys[id];
        m.set_pv_v(0, vel);
        m.rot = crate::moby::attach::angles_of(&rows);
        m.rows = rows;
        m.mode |= mode::KEEP_ROWS;
        m.set_pv_f(0x1c, 1.0);
    }
    // `0x2e14b0(1, place, 1, Ratchet)`: a moby of class type 5 (or a crate) at the throw is its target, the blade on
    // its aim point.
    let near = ctx.scene.map(|sc| crate::collision::coll_sphere_mobys(sc, pos, 1.0, QueryFlags(1), ctx.hero_moby)).and_then(|v| v.first().copied());
    let near = near.filter(|&n| w.class(w.mobys[n].o_class).is_some_and(|c| c.kind == 5) || aim::is_crate(w, n));
    let up = ctx.hero.gravity.up_at(pos);
    let target = match near {
        Some(n) => {
            if let Some((_, a)) = aim::health(w, n) { w.mobys[id].pos = add(w.mobys[n].pos, scale(up, a)); }
            Some(n)
        }
        None if !first => {
            let cone = aim::Cone { yaw: 0.174_532_95, pitch: 0.349_065_9, range: 30.0, near_pitch: 0.0, near_yaw: 0.0 };
            aim::search(w, ctx, cone, pos, rows[0], false)
        }
        None => lock,
    };
    w.mobys[id].set_pv_i(0x14, target.map_or(0, |t| t as i32));
    let pos = w.mobys[id].pos;
    match target {
        None => {
            let home = if !look { add(pos, scale(hero_rows[0], 64.0)) } else { add(ctx.host.camera_pos(), scale(ctx.host.camera_rows()[0], 64.0)) };
            w.mobys[id].set_pv_v(0x40, home);
            if !ctx.hero.gravity.on {
                let hidden = ctx.hero_moby.and_then(|h| w.mobys.get(h)).is_some_and(|h| h.mode & 1 != 0);
                if (!ctx.hero.items.fp.on && !matches!(ctx.hero.state, 0x1d | 1)) || !hidden {
                    let v = turn(w.mobys[id].pv_v(0), up, -0.436_332_35);
                    w.mobys[id].set_pv_v(0, v);
                }
                w.mobys[id].set_pv_s(0x36, 1);
            } else {
                w.mobys[id].set_pv_s(0x36, 0);
            }
        }
        Some(t) => {
            let f = (dist(pos, w.mobys[t].pos) * 0.033_333_335).min(1.0);
            let v = turn(w.mobys[id].pv_v(0), up, -(f * 0.436_332_35));
            w.mobys[id].set_pv_v(0, v);
            w.mobys[id].set_pv_s(0x36, 0);
        }
    }
    // The trail: the next of the slot's 25 when free (its points from none, two after-images), else none.
    let mut pool = std::mem::take(&mut w.chopper);
    if 24 < pool.index { pool.index = 0; }
    let k = pool.index;
    if !pool.holds(hand_class, slot) {
        w.chopper = pool;
        w.delete(id);
        return None;
    }
    if !pool.used[slot][k] {
        pool.index = (k + 1) % TRAILS;
        pool.used[slot][k] = true;
        w.mobys[id].set_pv_i(0x30, k as i32);
        w.mobys[id].set_pv_i(0x54, 0);
        let rec = &mut pool.ghosts[slot][k];
        rec.start(id);
        rec.add(w, 0.0, 0x30, 2);
        rec.add(w, 0.0, 0x17, 3);
    } else {
        w.mobys[id].set_pv_i(0x30, -1);
    }
    w.chopper = pool;
    // The tilt.
    {
        let m = &mut w.mobys[id];
        let (a, r0) = (m.pv_f(0x58), m.rows[0]);
        m.rows[1] = turn(m.rows[1], r0, a);
        m.rows[2] = turn(m.rows[2], r0, a);
        let p = m.pos;
        m.set_pv_v(0x70, p);
    }
    w.rebuild(id, &*ctx.host);
    Some(id)
}

/// `0x41f3f0(blade, point, up)`: the way turned toward `point` — within 1.5 half of the angle at once; else by a
/// spring on half the angle (`0x2d7f80`: 0.025 × the ramp `+0x12` / 30 (it grows to 30), 0.3, `+0x20` its speed) as a
/// quaternion half-angle about way × to-it (over 165° off: way × (up × way)). Returns the spring's speed in degrees,
/// at most 1 (the turn).
fn steer(w: &mut MobyWorld, id: usize, point: V3, up: V3) -> f32 {
    let m = &mut w.mobys[id];
    let vel = m.pv_v(0);
    let d = sub(point, m.pos);
    let l = len(d);
    if l == 0.0 { return 0.0; }
    if l < 1.5 {
        // `0x321ca8(0.5, vel, vel, d, 0)`: half of the angle between them.
        let (lv, ld) = (len(vel), l);
        if lv * ld == 0.0 { return 0.0; }
        let a = (std::f32::consts::FRAC_PI_2 - (dot(vel, d) / (lv * ld)).clamp(-1.0, 1.0).asin()) * 0.5;
        m.set_pv_v(0, turn(vel, cross(vel, d), a));
        return 0.0;
    }
    let ramp = (m.pv_s(0x12) + 1).min(0x1e);
    m.set_pv_s(0x12, ramp);
    let k = ramp as f32 / 30.0;
    let mut axis = cross(vel, d);
    let (lv, ld) = (len(vel), len(d));
    if lv * ld == 0.0 { return 0.0; }
    let off = std::f32::consts::FRAC_PI_2 - (dot(vel, d) / (lv * ld)).clamp(-1.0, 1.0).asin();
    if 2.879_793_5 < off.abs() { axis = cross(vel, cross(up, vel)); }
    // `0x2d7f80(0, off/2, 0.025k, 0.3, 0, +0x20)`.
    let e = sub_rot(off * 0.5, 0.0);
    let mut v = m.pv_f(0x20) + (k * 0.025 * e - 0.3 * m.pv_f(0x20));
    v = v.clamp(-e.abs(), e.abs());
    m.set_pv_f(0x20, v);
    let half = add_rot(0.0, v);
    m.set_pv_v(0, turn(vel, axis, half * 2.0));
    (v * 57.295_773).min(1.0)
}

/// `0x41f668(at, n, split)`: `n` times two twinkles (crate::particles::twinkle) from the hit, jittered 0.15 a time,
/// flying toward the camera within ±60° at 0.05..3 a second and up 1..5 a second, falling 0.002, 60 ticks, turning one
/// way and the other: one 0.5..2 large in `0x202080` (the Mega's `0x802020`, a split's `0x802080`), one white 0.7 of
/// it; their `+0x09` 0x74.
fn sparks(ctx: &mut Ctx, at: V3, n: i32, split: bool) {
    let lv = level(ctx.game);
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let g = &mut ctx.game.rng;
    let to_cam = sub(ctx.host.camera_pos(), at);
    let up = [0.0, 0.0, 1.0];
    let mut p = at;
    for _ in 0..n {
        let size = g.randf(0.5, 2.0);
        let r = g.randi(2);
        let s = if r == 0 { -1 } else { r };
        let a = g.randf(-1.047_197_7, 1.047_197_7);
        let mut v = set_len(turn(to_cam, up, a), g.randf(0.05, 3.0) * (1.0 / 60.0));
        v[2] = g.randf(1.0, 5.0) * (1.0 / 60.0);
        for k in 0..3 { p[k] += g.randf_signed(0.0, 0.15); }
        let c = if lv == 0 { 0x20_2080 } else if split { 0x80_2080 } else { 0x80_2020 };
        if let Some(i) = crate::particles::twinkle(sys, g, size * 0.1, size, 0.002, p, 0x3c, c | 0x7f00_0000, 0, s as i8, v, 1) { sys.pool.recs[i][9] = 0x74; }
        if let Some(i) = crate::particles::twinkle(sys, g, size * 0.07, size * 0.7, 0.002, p, 0x3c, 0x7f7f_7f7f, 1, -s as i8, v, 1) { sys.pool.recs[i][9] = 0x74; }
    }
}

/// The blade's trail let go (its flag cleared, its after-images freed).
fn drop_trail(w: &mut MobyWorld, id: usize) {
    let (k, slot) = (w.mobys[id].pv_i(0x30), w.mobys[id].pv_i(0x3c));
    if k < 0 { return; }
    let mut pool = std::mem::take(&mut w.chopper);
    if let (Some(u), Some(g)) = (pool.used.get_mut(slot as usize).and_then(|u| u.get_mut(k as usize)), pool.ghosts.get_mut(slot as usize).and_then(|g| g.get_mut(k as usize))) {
        *u = false;
        g.free(w);
    }
    w.chopper = pool;
    w.mobys[id].set_pv_i(0x30, -1);
}

/// The loop sound let go when it is still its own and playing (`0x3602f0`).
fn stop_sound(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let s = w.mobys[id].pv_i(0x38);
    if s != -1 && ctx.host.sound_playing(s) { ctx.host.stop_sound(s); }
    w.mobys[id].set_pv_i(0x38, -1);
}

/// `0x41e080` (module docs).
pub fn update_blade(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (class, slot) = (w.mobys[id].pv_i(0x50) as i16, w.mobys[id].pv_i(0x3c) as usize);
    if !w.chopper.holds(class, slot) {
        w.delete(id);
        return;
    }
    let Some(r) = cur_row().copied() else { return };
    let lv = level(ctx.game);
    let start = w.mobys[id].pos;
    let up = ctx.hero.gravity.up_at(start);
    // The home point turned about Ratchet with the platform's turn.
    {
        let hp = ctx.hero.pos;
        let down = ctx.hero.gravity.down;
        let m = &mut w.mobys[id];
        let h = sub(m.pv_v(0x40), hp);
        m.set_pv_v(0x40, add(turn(h, down, -ctx.hero.carry_yaw), hp));
    }
    match w.mobys[id].state {
        0 => {}
        1 => {
            drop_trail(w, id);
            stop_sound(w, id, ctx);
            crate::moby::blast::smoke_burst(w, ctx, 0.5, 5.0, Some(id), start, -1);
            w.delete(id);
            return;
        }
        _ => return,
    }
    {
        let m = &mut w.mobys[id];
        let mut life = m.pv_s(0x10) as i32;
        if countdown(&mut life) { m.state = 1; }
        m.set_pv_s(0x10, life as i16);
        let mut free = m.pv_s(0x18) as i32;
        countdown(&mut free);
        m.set_pv_s(0x18, free as i16);
    }
    if w.mobys[id].pv_s(0x34) != 0 {
        w.mobys[id].set_pv_s(0x34, 0);
        // `0x40490fdc`: just past π (all round, crate::moby::aim).
        let pi = f32::from_bits(0x4049_0fdc);
        let cone = aim::Cone { yaw: pi, pitch: pi, range: 12.0, near_pitch: pi, near_yaw: pi };
        let t = aim::search(w, ctx, cone, start, w.mobys[id].pv_v(0), true);
        let m = &mut w.mobys[id];
        m.set_pv_i(0x14, t.map_or(0, |t| t as i32));
        if t.is_some() {
            m.set_pv_s(0x1a, 0);
            m.set_pv_s(0x12, 10);
            m.set_pv_s(0x18, 0);
        }
    }
    let tgt = w.mobys[id].pv_link(0x14);
    let live = tgt.and_then(|t| w.mobys.get(t)).is_some_and(|t| t.state != 0xfe && t.state != 0xfd && t.draw_dist != 0 && t.mode & mode::TARGET != 0) && tgt.is_some_and(|t| aim::health(w, t).is_none_or(|(hp, _)| 0.0 < hp));
    let mut bend = 0.0;
    if !live {
        w.mobys[id].set_pv_i(0x14, 0);
        if w.mobys[id].pv_s(0x36) != 0 {
            let home = w.mobys[id].pv_v(0x40);
            bend = steer(w, id, home, up);
            let m = &mut w.mobys[id];
            if dot(sub(home, add(m.pos, m.pv_v(0))), m.pv_v(0)) < 0.0 { m.set_pv_s(0x36, 0); }
        }
    } else if w.mobys[id].pv_s(0x18) == 0 {
        let t = tgt.unwrap();
        let p = match aim::health(w, t) {
            Some((_, a)) => add(w.mobys[t].pos, scale(up, a)),
            None => w.mobys[t].pos,
        };
        bend = steer(w, id, p, up);
    }
    let vel = w.mobys[id].pv_v(0);
    let d = unit(vel);
    let mut tmpl = HitTemplate::new(r.damage, Some(id), 0x10000, [d[0], d[1], d[2], NO_KNOCK_LIMIT]);
    tmpl.kind = (BLADE as u16 as u32) << 16 | 0x101;
    let to = add(start, vel);
    w.mobys[id].pos = to;
    if to.iter().any(|&c| !(2.0..=1021.0).contains(&c)) {
        drop_trail(w, id);
        stop_sound(w, id, ctx);
        w.delete(id);
        return;
    }
    let free = w.mobys[id].pv_s(0x18) != 0;
    let flags = if free { 0x12 } else { 0x10 };
    let hit = ctx.coll.and_then(|c| crate::collision::coll_line_m(c, ctx.scene, start, to, QueryFlags(flags), ctx.hero_moby));
    if let Some(o) = hit {
        if !free {
            if let Some(m) = o.moby { w.deliver_hit_any(m, &tmpl, [o.point[0], o.point[1], o.point[2], 1.0]); }
        }
        w.mobys[id].pos = o.point;
        sparks(ctx, o.point, 10, w.mobys[id].pv_i(0x68) == 0);
        w.mobys[id].set_pv_s(0x36, 0);
        let hit_moby = o.moby.filter(|_| o.kind < 1);
        match hit_moby {
            Some(hm) => {
                let hb = aim::health(w, hm);
                let snd = match hb {
                    None => 2,
                    Some(_) => if w.mobys[hm].pv_b(crate::moby::ai::health(&w.mobys[hm]) + 9) == 1 { 3 } else { 4 },
                };
                ctx.host.play_sound(&w.mobys[id], snd, 0);
                if Some(hm) == w.mobys[id].pv_link(0x14) && hb.is_some_and(|(hp, _)| hp <= r.damage) {
                    w.mobys[id].set_pv_i(0x14, 0);
                    w.mobys[id].set_pv_s(0x34, 1);
                }
                let n = w.mobys[id].pv_s(0x2c) + 1;
                w.mobys[id].set_pv_s(0x2c, n);
                if (if lv != 0 { 2 } else { 3 }) <= n { w.mobys[id].state = 1; }
            }
            None => { ctx.host.play_sound(&w.mobys[id], 2, 0); }
        }
        let first_bounce = w.mobys[id].pv_f(0x1c) == 1.0;
        if first_bounce && hit_moby.is_some() {
            // Off the first moby: 10..35° about its up row × its way.
            let m = &w.mobys[id];
            let axis = cross_ba(m.pv_v(0), m.rows[2]);
            let a = ctx.game.rng.randf(0.174_532_95, 0.610_865_3);
            let v = turn(m.pv_v(0), axis, -a);
            w.mobys[id].set_pv_v(0, v);
            w.mobys[id].set_pv_f(0x1c, 2.0);
        } else {
            let n = match hit_moby {
                Some(hm) => w.mobys[hm].rows[2],
                None => unit(scale(o.normal, 1.0 / 1024.0)),
            };
            let m = &mut w.mobys[id];
            let v = m.pv_v(0);
            m.set_pv_v(0, sub(v, scale(n, 2.0 * dot(v, n))));
            m.pos = add(m.pos, scale(n, 0.2));
            if m.pv_i(0x14) == 0 { m.set_pv_s(0x34, 1); }
            m.set_pv_f(0x1c, 2.0);
        }
        if hit_moby.is_some() {
            let m = &mut w.mobys[id];
            m.set_pv_s(0x18, 10);
            m.set_pv_s(0x1a, 1);
            m.set_pv_s(0x12, 0);
            if lv != 0 && m.pv_i(0x68) != 0 {
                m.set_pv_i(0x68, 0);
                split(w, id, ctx);
            }
        }
    }
    // Past the target: free ticks.
    {
        let tgt = w.mobys[id].pv_link(0x14).filter(|&t| w.mobys.get(t).is_some());
        let behind = {
            let m = &w.mobys[id];
            tgt.is_some_and(|t| dot(m.pv_v(0), sub(w.mobys[t].pos, m.pos)) < 0.0)
        };
        let m = &mut w.mobys[id];
        if m.pv_s(0x18) == 0 && m.pv_s(0x1a) == 1 && !behind { m.set_pv_s(0x1a, 0); }
        if behind && m.pv_s(0x18) == 0 && m.pv_s(0x1a) == 0 {
            m.set_pv_s(0x1a, 1);
            m.set_pv_s(0x18, 10);
        }
    }
    // The rows: along the way, up the up; tilted into the turn; spun.
    {
        let m = &mut w.mobys[id];
        let r0 = unit(m.pv_v(0));
        let r1 = unit(cross_ba(r0, up));
        let r2 = cross_ba(r1, r0);
        let mut rows = [r0, r1, r2];
        let mut t = -bend * 1.047_197_6;
        if t == 0.0 { t = if 0.0 < m.pv_f(0x58) { 0.069_813_18 } else { -0.069_813_18 }; }
        let tilt = add_rot(m.pv_f(0x58), sub_rot(t, m.pv_f(0x58)) * 0.05);
        m.set_pv_f(0x58, tilt);
        if 1e-5 < tilt.abs() {
            rows[1] = turn(rows[1], rows[0], tilt);
            rows[2] = turn(rows[2], rows[0], tilt);
        }
        let spin = add_rot(m.pv_f(0x28), 0.418_879_06);
        m.set_pv_f(0x28, spin);
        if 1e-5 < spin.abs() {
            rows[0] = turn(rows[0], rows[2], spin);
            rows[1] = turn(rows[1], rows[2], spin);
        }
        m.rows = rows;
        let s = r.speed * (1.0 / 60.0);
        let v = set_len(m.pv_v(0), s + (0.25 - s) * bend);
        m.set_pv_v(0, v);
    }
    // The trail's points and the lightning's.
    if 0 <= w.mobys[id].pv_i(0x30) {
        let m = &mut w.mobys[id];
        let n = m.pv_i(0x54).clamp(0, TRAIL_POINTS as i32) as usize;
        for k in (1..n).rev() {
            let v = m.pv_v(TRAIL + 0x10 * (k - 1));
            m.set_pv_v(TRAIL + 0x10 * k, v);
        }
        m.set_pv_v(TRAIL, start);
        if n < TRAIL_POINTS {
            // The count grows past the points written: the game's last point is left from the trail's last user
            // (its buffer is not cleared); the port repeats the one before it.
            let last = m.pv_v(TRAIL + 0x10 * n.saturating_sub(1));
            m.set_pv_v(TRAIL + 0x10 * n, last);
            m.set_pv_i(0x54, n as i32 + 1);
        }
        let mut c = m.pv_i(0x60);
        let now = m.pos;
        if !countdown(&mut c) {
            for k in 0..7 {
                let o = BOLT + 0x10 * k;
                let p = add(sub(start, m.pv_v(o)), now);
                m.set_pv_v(o, p);
                m.set_pv_f(o + 0xc, 0.15);
            }
        } else {
            let half = if m.pv_i(0x68) == 0 { 0.25 } else { 0.5 };
            let row2 = m.rows[2];
            let side = set_len(cross_ba(sub(now, ctx.host.camera_pos()), row2), half);
            let g = &mut ctx.game.rng;
            let mut j = g.randf_signed(0.0, 0.1);
            let lift = |j: f32| scale(row2, j);
            let mut pts = [[0.0f32; 3]; 7];
            pts[0] = add(sub(now, side), lift(j));
            pts[6] = add(add(now, side), lift(j));
            let e = scale(side, 0.666);
            j += g.randf_signed(0.0, 0.1);
            pts[1] = add(sub(now, e), lift(j));
            pts[5] = add(add(now, e), lift(j));
            let e = scale(side, 0.333);
            j += g.randf_signed(0.0, 0.1);
            pts[2] = add(sub(now, e), lift(j));
            pts[4] = add(add(now, e), lift(j));
            pts[3] = add(now, lift(j + g.randf_signed(0.0, 0.1)));
            for (k, p) in pts.iter().enumerate() {
                m.set_pv_v(BOLT + 0x10 * k, *p);
                m.set_pv_f(BOLT + 0x10 * k + 0xc, 0.15);
            }
            c = 4;
        }
        m.set_pv_i(0x60, c);
        // `0x2e7920(0x41f9a8)`: its draw (crate::fx).
        w.register_draw(id);
    }
    // The glow.
    {
        let split = w.mobys[id].pv_i(0x68) == 0;
        let m = &mut w.mobys[id];
        let ph = add_rot(m.pv_f(0x64), 720.0 * 0.000_290_888_25);
        m.set_pv_f(0x64, ph);
        let (a, b) = match (lv, split) {
            (0, _) => (0x0a_0aff, 0x7f_7fff),
            (_, false) => (0xff_0a0a, 0xff_7f7f),
            (_, true) => (0xff_7fff, 0xff_7fff),
        };
        m.glow = crate::hud::lerp_rgba(ph.sin() * 0.5 + 0.5, a, b) | 0xff00_0000;
    }
    w.rebuild(id, &*ctx.host);
    if 0 <= w.mobys[id].pv_i(0x30) {
        let m = &mut w.mobys[id];
        m.rot = crate::moby::attach::angles_of(&m.rows);
        let (k, s) = (m.pv_i(0x30) as usize, slot);
        let mut pool = std::mem::take(&mut w.chopper);
        if let Some(rec) = pool.ghosts.get_mut(s).and_then(|g| g.get_mut(k)) { rec.tick(w, &*ctx.host, 0); }
        w.chopper = pool;
    }
    let s = w.mobys[id].pv_i(0x38);
    if !(s != -1 && ctx.host.sound_playing(s)) {
        let h = ctx.host.play_sound(&w.mobys[id], 1, 4);
        w.mobys[id].set_pv_i(0x38, h);
    }
}

/// The Mega Chopper's split off its first moby (`0x41e080`): its way turned 45..80° either way about the up there, a
/// second blade (`0x41d7d0(hand, its place, 0, its slot, none, 0)`) at its place, rows and angles, flying its old way;
/// both, and their after-images, half the size.
fn split(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (pos, hand, slot) = (w.mobys[id].pos, w.mobys[id].parent, w.mobys[id].pv_i(0x3c) as usize);
    let up = ctx.hero.gravity.up_at(pos);
    let a = ctx.game.rng.randf_signed(0.785_398_2, 1.396_263_6);
    let v = turn(w.mobys[id].pv_v(0), up, a);
    w.mobys[id].set_pv_v(0, v);
    let Some(hand) = hand else { return };
    let Some(c) = spawn(w, ctx, hand, pos, false, slot, None, false) else { return };
    w.mobys[id].scale *= 0.5;
    w.mobys[c].scale *= 0.5;
    for b in [id, c] {
        let k = w.mobys[b].pv_i(0x30);
        if k < 0 { continue; }
        let pool = std::mem::take(&mut w.chopper);
        if let Some(rec) = pool.ghosts.get(slot).and_then(|g| g.get(k as usize)) { rec.scale_ghosts(w, 0.5); }
        w.chopper = pool;
    }
    let (rows, rot) = (w.mobys[id].rows, w.mobys[id].rot);
    let m = &mut w.mobys[c];
    (m.pos, m.rows, m.rot) = (pos, rows, rot);
    m.set_pv_v(0, turn(v, up, -a));
    m.parent = Some(hand);
}

/// `0x41f9a8` → `0x41fb48` (module docs): the blade's trail and, for the Mega, its lightning.
pub fn draw(w: &MobyWorld, id: usize, game: &mut crate::game::GameState, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let m = &w.mobys[id];
    if !w.chopper.holds(m.pv_i(0x50) as i16, m.pv_i(0x3c) as usize) { return; }
    let lv = level(game);
    let split = m.pv_i(0x68) == 0;
    let (colour, alpha) = match (lv, split) {
        (0, _) => (0x20_2080u32, 0x50i32),
        (_, false) => (0x80_2020, 0x7f),
        (_, true) => (0x80_2080, 0x7f),
    };
    let n = m.pv_i(0x54).clamp(0, TRAIL_POINTS as i32) as usize;
    if 2 < n {
        let life = m.pv_s(0x10) as i32;
        let a = if 0x118 < life { (alpha as f32 * ((300 - life) as f32 / 20.0)) as i32 } else { alpha };
        let c = colour | (a as u32) << 24;
        let half = if split { 0.25 } else { 0.5 };
        let up = m.rows[2];
        let pt = |k: usize| m.pv_v(TRAIL + 0x10 * k);
        let side = |d: V3| set_len(cross_ba(d, up), half);
        let mut d = sub(m.pos, pt(0));
        let mut s = side(d);
        let (mut a0, mut b0) = (add(m.pos, s), sub(m.pos, s));
        let step = 0.5 / (n as f32 - 2.0);
        let (mut s0, mut s1) = (0.0f32, 0.5f32);
        for k in 0..n {
            if 0 < k { d = sub(pt(k - 1), pt(k)); }
            s = side(d);
            let (a1, b1) = (add(pt(k), s), sub(pt(k), s));
            out.push(crate::fx::FxQuad { fx: 0xb, additive: true, corners: [a0, b0, a1, b1], st: [[s0, 0.0], [s0, 1.0], [s1, 0.0], [s1, 1.0]], rgba: [c; 4] });
            (a0, b0) = (a1, b1);
            s0 = s1;
            s1 += step;
        }
    }
    if lv != 0 {
        let f = game.rng.randf(0.5, 1.0);
        let (ca, cb) = (((f * 127.0) as u32) << 24 | 0xff_5050, ((f * 48.0) as u32) << 24 | 0xff_1010);
        let pts: Vec<[f32; 4]> = (0..7).map(|k| { let v = m.pv_v(BOLT + 0x10 * k); [v[0], v[1], v[2], m.pv_f(BOLT + 0x10 * k + 0xc)] }).collect();
        crate::fx::ribbon(out, 3.0, 0.0, &pts, cam, [0x17, 0xe], [ca, cb]);
    }
}
