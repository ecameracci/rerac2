//! The Pulse Rifle (item 0x17, crate::hero::items::ROWS; L00's record `0x263920`: attachment 0, stance 2 (both hands),
//! use sequence 0x36, the firing arm's 0x46 crouched and none moving, icon 0xea68, name 0x32e6, shots 400 a second,
//! one a second, damage 10, 8 shots): the hand moby **0x05cb** (`0x3a4310`, called by the item pass while it is the
//! hand item) and its shot **0x05d6** (`0x3a92a8`, made by `0x3a9060`; no model: its smoke and beam are its draw's).
//!
//! **The hand** (pvars: `+0x00` (s16) in first person last tick, `+0x02` (s16) the shot's delay, `+0x04` the cooldown
//! (seconds), `+0x08` the view's angle at the start, `+0x0c` the zoom's angle, `+0x10` the muzzle light
//! (crate::point_lights, −1 none), `+0x14` its ticks, `+0x18` its trail slot, `+0x20` the zoom's loop sound, `+0x28` the
//! shots fired, `+0x2c` (s16) the scope has a target, `+0x2e` (s16) its blink, `+0x30` the target; the port keeps the
//! view turn's globals it saved (`$gp+0x5d4..0x5e0`) at `+0x40..0x48`). Put away (`0x18b064` = 3) once started, and once
//! when Ratchet enters the state 0x6f (to its state 4, back to 1 after), it puts the first-person view's turn back
//! ([`crate::first_person::Tune`]), the field of view back to the play's (`0x2d7a60(1.1243, 0.005, 0.2, 0, 0, 3)`,
//! crate::camera::FovBlend) and lets the light go. Its sequence run out, back to 1 (blend 2).
//!
//! State **0**: the cooldown 0.3667 s, state **3**; the view's angle `2·atan(0x1bd2f0)` kept, the zoom a quarter of the
//! way from it to 10°; the turn's globals saved; the class's trail slot taken (crate::moby::classes::chopper::Pool,
//! `0x32b1b8`) and its smoke and beams cleared (`0x3a9938`, [`Mem`]). **3**: the cooldown runs out → **1**.
//!
//! In first person (`0x18b320`) past state 0, every frame with the counter's bit 4 (`0x1b1948 & 4`) the scope looks 128
//! out along the view (flags 0x10, Ratchet left out): a drawn crate, or a moby of class type 5 with health left, is its
//! target. With the look wait out (`0x18c354`): a target blinks (`+0x2e` 0..20, from 11 on) the lock marker (1.5, FX
//! 0x20, `0x400000ff`) while the rifle is ready with ammo; and the crosshair (2, FX 0x27: red `0x600000ff` while the
//! cooldown runs with ammo, else green `0x600fff0f`) at the screen's centre (`0x2db498`, crate::hud::marker).
//!
//! **1** (ready): from the look stance 1 Ratchet goes to its twin 0x1d. Out of first person the look wait is 0x28 and a
//! zoom left (`+0x00`) blends the field of view back to the start's (0.005, 0.2), the zoom a quarter of the way to 10°
//! again; its sound stops. In first person the look wait counts down (a shot waits it out), and R1 (pad 8) zooms in
//! while the angle is over 10°, R2 (pad 2) out while it is under the start's, 0.01 a tick, each with its class sound (2
//! in, 3 out, looping) — neither, the sound stops; the angle held to 10°..the start's; the field of view blends to it
//! (0.02, 0.2). Every tick the view's turn eases with the zoom: by `f = (angle − 10°) / (start − 10°)` the most a tick
//! from 0.6° to the saved, the input's ease (0.03, 0.4) to the saved. ○ pressed (`0x138324`) with the items not kept
//! away and the look wait out: a shot taken — the use stance (`0x2af5e0`), the delay 2, **2**, the hand's sequence 2 —
//! or, with none left, its class sound 1.
//!
//! **2**: the delay out (in first person at once): the zoom's sound stops, its class sound 0, the shot fired ([`fire`],
//! `0x3a4f88`), the cooldown 1 / the row's rate, **3**; the fifth shot asks for the help text 0x9f3 (slot 0x4a).
//!
//! **The light** (`+0x14` ticks from 30): a point light (`0x2fda20(15, 0, …, 0x7f608060)`) 1.5 ahead of the muzzle along
//! Ratchet's forward row (the arm raised at a wall: along the gun's −row 1), radius 15, green `t` with red and blue
//! `0.75·t` for `t` its ticks / 30; freed when they run out.
//!
//! **The shot** ([`fire`]): its speed the row's / 60 a tick; from the muzzle (joint 0) along Ratchet's forward row (the
//! arm raised: the gun's −row 1) — toward a lock's aim point (`0x18aa18`, `0x32b4d0`), or in first person toward what
//! the view meets 128 out (`0x380ef8`: held within 20° of his forward row); the muzzle light; the shot made
//! ([`spawn`]); a moby of class type 5 or a crate within 1 of the muzzle (`0x2e14b0`, Ratchet left out) is hit at once
//! (the shot's hit line from it); four twinkles at the muzzle (`0x33a6a0`: 7, 5 green `0x4010ff10`, 3.5, 2.5 white
//! `0xc0ffffff` / `0xb0ffffff`, 8 ticks, turning either way).
//!
//! **The shot** 0x5d6 (pvars: `+0x00` its velocity, `+0x10` (s16) its life (`128 / speed` ticks), `+0x12` (s16) 1 when
//! fired in first person, `+0x14` one over its life, `+0x18` (s16) its beam, `+0x1a` (s16) the trail slot, `+0x20` /
//! `+0x30` the view's ray's step and point in first person). Made ([`spawn`]): far 0xff, drawn, its rows along its way
//! (`0x3a9490`), the next of the three beams (`$gp+0x694`) started from the muzzle ([`make_beam`], `0x3aa258`; none in
//! the look stance's twin). Each tick ([`update_shot`]): gone once the slot is not the rifle's; it steps (in first person
//! its ray too: the hits come from the view, the smoke from the ray's start); from the second tick a puff of smoke at
//! the step's start, and every tick one halfway ([`puff`], `0x3a9aa0`); out of the world's 2..1021 box, a hit
//! ([`hit`], `0x3a9528`) or its life out, it ends (state **2**); else the slot's smoke and its beam step ([`tail`],
//! `0x3a99d0`). Ended, they step until none lives and it goes; while any lives its draw is asked for.
//!
//! **The hit** (`0x3a9528`): the step's line (flags 0, Ratchet left out) hits a moby with the row's damage (flags
//! 0x10000, its way the shot's, kind the class's `<< 16 | 0x0101`); at the point eight pairs of twinkles jittered 0.5
//! (`0x3a9668`: 10 green, 2 white, 6..16 ticks).
//!
//! **The smoke** ([`Puff`], `0x3a9aa0`, 80 in the slot): rising 0..0.1 a second along the up, turning 1..2° a tick
//! either way, its nine corners pushed out 0.00075..0.0075 a tick each, 60..90 ticks; stepped by every live shot's
//! tick (`0x3a9c08`: two shots, twice). **The beams** ([`Beam`], 3 in the slot, 60 ticks): from the muzzle and one
//! step on, both sliding 4 a second along the shot's way, to the shot.
//!
//! **The draw** ([`draw`], `0x3a9a48` → `0x3a9d70` / `0x3aa5d0`, each live shot's: all the slot's smoke and beams):
//! each puff a 3 × 3 patch facing the camera (FX 0x36 normal, then again additive 0.075·(1 − share) up), its corners
//! (`0x294a40`) pushed out and turned by its roll, `0.5 → its size` large, `0x408040` .. `0x604040` by `|sin(4π·share)|`
//! at its life's share of 128 (32 in the look stance's twin); each beam three rings of 20 (`0x294b20`, 0.2) joined by
//! FX 0x15 quads (`0x708070` at its share of 64, the muzzle's end none, its texture scrolling 0.05 a tick), over them
//! a strip 1 wide across the view (FX 0xe additive, `0x408040` at its share of 80) and one 0.1 wide (FX 0x17 additive,
//! `0x404080` at its share of 255, scrolling 0.075 a tick), both from none at the muzzle.
//!
//! Its glow in the hand (`0x2a7ed8`'s case 0x17): two green glows at its joints 2 and 3 (crate::hero::items,
//! crate::fx::HeroGlow).
//!
//! Not ported: the pad rumble (`0x2d6d60` / `0x2d6f38`), the lock cheat (`0x18b270`), the weapon mods' effects on a hit
//! (`0x4126b8`), the moving frames (`$gp−0x14e4`: their aim straight at a lock or the scope's target, the shot's mode 2,
//! and the frame's carry `0x380e58` / bends `0x380e08`), the LOD ranges the zoom widens (`0x36a1c0`, `0x1aa100`: the
//! engine's LOD is its own), `0x18ccd0` (set to 0xffff while zooming; no reader on Aranos). The upgraded rows (the
//! hand `0x3e4e98`, the impact's blast `0x320538` and its sphere's damage) are not in crate::hero::items::UPGRADE_ROWS.

use crate::collision::QueryFlags;
use crate::first_person::Tune;
use crate::hero::items::{row, ItemRow, PULSE};
use crate::hero::physics::{add, add_rot, cross, dot, len, scale, set_len, sub, unit, V3};
use crate::moby::hits::HitTemplate;
use crate::moby::{aim, countdown, Ctx, MobyWorld};
use crate::point_lights::PointLight;

/// The hand's class (item 0x17's row `+0x04`) and the shot's (`+0x50`).
pub const HAND_CLASS: i16 = 0x5cb;
pub const SHOT: i16 = 0x5d6;
/// `t[3]` of the shot's hit template (`0x45afdf66`).
const NO_KNOCK_LIMIT: f32 = 5627.925;
/// `0x3e32b8c4`: the zoom's narrowest (10°).
const NARROW: f32 = 0.174_532_95;
/// `0x3c2b92a8`: the view's most turn a tick at the narrowest (0.6°).
const NARROW_TURN: f32 = 0.010_471_977;
/// The puffs and beams of a slot.
pub const PUFFS: usize = 0x50;
pub const BEAMS: usize = 3;
/// The muzzle light's ticks and colour (`0x7f608060`: red and blue 0x60, green 0x80, /128).
const LIGHT_TICKS: i32 = 0x1e;

/// One puff of smoke (`0x70` bytes of the slot's memory: `+0x00` its place, `+0x10` (s16) its life, `+0x12` (s16) in
/// use, `+0x14` its roll, `+0x18` its turn a tick, `+0x1c` its size, `+0x20` one over its life, `+0x24` its rise a tick,
/// `+0x28` its nine corners' push, `+0x4c` their growth a tick). `up`: the up at its place (the port's, for the draw).
#[derive(Clone, Copy, Debug, Default)]
pub struct Puff {
    pub pos: V3,
    pub life: i16,
    pub live: bool,
    pub roll: f32,
    pub spin: f32,
    pub size: f32,
    pub inv: f32,
    pub rise: f32,
    pub push: [f32; 9],
    pub grow: [f32; 9],
    pub up: V3,
}

/// One beam (`0x60` bytes past the puffs: `+0x00` / `+0x10` / `+0x20` its three points, `+0x30` its way, `+0x40` the up
/// across it, `+0x50` (s16) live, `+0x52` (s16) its life (60), `+0x54` / `+0x58` its textures' scroll, `+0x5c` one over
/// its life).
#[derive(Clone, Copy, Debug, Default)]
pub struct Beam {
    pub p: [V3; 3],
    pub axis: V3,
    pub side: V3,
    pub live: bool,
    pub life: i16,
    pub scroll: [f32; 2],
    pub inv: f32,
}

/// One trail slot's memory as the rifle uses it.
#[derive(Clone, Debug, Default)]
pub struct Slot {
    pub puffs: Vec<Puff>,
    pub beams: [Beam; BEAMS],
}

/// The rifle's part of the two trail slots (`0x268da0 + 0xa430·k`) and the next beam (`$gp+0x694`).
#[derive(Clone, Debug, Default)]
pub struct Mem {
    pub slots: [Slot; 2],
    pub next_beam: i32,
}

impl Mem {
    /// `0x3a9938(slot)`: every puff out of use; the beams dead, their scroll at 0.
    pub fn clear(&mut self, slot: usize) {
        let Some(s) = self.slots.get_mut(slot) else { return };
        s.puffs = vec![Puff::default(); PUFFS];
        for b in s.beams.iter_mut() {
            b.live = false;
            b.scroll = [0.0; 2];
        }
    }

    fn slot(&mut self, slot: usize) -> &mut Slot {
        let s = &mut self.slots[slot.min(1)];
        if s.puffs.len() != PUFFS { s.puffs = vec![Puff::default(); PUFFS]; }
        s
    }
}

/// The Pulse Rifle's row as `0x139568[0x17]` gives it (crate::upgrades).
fn cur_row(game: &crate::game::GameState) -> Option<&'static ItemRow> { row(game, PULSE) }

/// `0x2eef88(out, a, b)` = b × a.
fn cross_ba(a: V3, b: V3) -> V3 { cross(b, a) }

/// `0x2eeb40(+0x04)`: the cooldown down a frame (1/60 s); true when out.
fn cooldown(w: &mut MobyWorld, id: usize) -> bool {
    let c = w.mobys[id].pv_f(4);
    if 1.0 / 60.0 < c {
        w.mobys[id].set_pv_f(4, c - 1.0 / 60.0);
        false
    } else {
        w.mobys[id].set_pv_f(4, 0.0);
        true
    }
}

/// `0x2eeae0` on the s16 at `off`: true when it was 0 or just reached it.
fn count_s(w: &mut MobyWorld, id: usize, off: usize) -> bool {
    let mut n = w.mobys[id].pv_s(off) as i32;
    let done = countdown(&mut n);
    w.mobys[id].set_pv_s(off, n as i16);
    done
}

/// The muzzle light let go (`0x2fdc48`).
fn free_light(w: &mut MobyWorld, id: usize) {
    let s = w.mobys[id].pv_i(0x10);
    if 0 <= s { w.point_lights.free(s as usize); }
    w.mobys[id].set_pv_i(0x10, -1);
}

/// The zoom's loop sound let go when still playing (`0x3602f0`).
fn stop_zoom_sound(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let s = w.mobys[id].pv_i(0x20);
    if s != -1 && ctx.host.sound_playing(s) { ctx.host.stop_sound(s); }
    w.mobys[id].set_pv_i(0x20, -1);
}

/// The way out (put away, the state 0x6f): the view turn's globals back (when they were saved), the play's field of
/// view, the light let go.
fn put_back(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, saved: bool) {
    let m = &w.mobys[id];
    if saved { ctx.game.cameras.fp.tune = Tune { k: m.pv_f(0x40), d: m.pv_f(0x44), turn: m.pv_f(0x48) }; }
    ctx.game.cam_tweaks.set_fov(crate::camera::PLAY_FOV, 0.005, 0.2, 0.0, 3);
}

/// `0x3a4310` (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].pvars.resize(crate::moby::PVAR_BYTES, 0);
    if ctx.hero.items.ready == 3 {
        if w.mobys[id].state == 0 { return; }
        put_back(w, id, ctx, true);
        free_light(w, id);
        return;
    }
    if ctx.hero.state == 0x6f {
        let was = w.mobys[id].state;
        if was == 4 { return; }
        w.mobys[id].state = 4;
        free_light(w, id);
        put_back(w, id, ctx, was != 0);
        return;
    }
    if w.mobys[id].state == 4 { w.mobys[id].state = 1; }
    // Its sequence run out: back to 1 (`0x31b760(hand, 1, 0, 2, 0)`).
    if let Some(a) = ctx.host.anim(&w.mobys[id]) {
        if a.seq_a == a.seq_b && a.flags & 2 != 0 { ctx.host.set_anim(&w.mobys[id], 1, 0, 2); }
    }
    let fp = ctx.hero.items.fp.on;
    if fp && w.mobys[id].state != 0 { scope(w, id, ctx); }
    match w.mobys[id].state {
        0 => start(w, id, ctx),
        1 => ready(w, id, ctx, fp),
        2 => {
            if count_s(w, id, 2) || w.mobys[id].pv_s(0) != 0 { shoot(w, id, ctx, fp); }
        }
        3 => {
            if cooldown(w, id) { w.mobys[id].state = 1; }
        }
        _ => {}
    }
    light(w, id, ctx);
}

/// State 0 (module docs).
fn start(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let begin = ctx.game.cam_tweaks.view_tan().atan() * 2.0;
    let tune = ctx.game.cameras.fp.tune;
    {
        let m = &mut w.mobys[id];
        m.set_pv_s(0, 0);
        m.set_pv_f(4, 0.366_666_67);
        m.state = 3;
        m.set_pv_f(8, begin);
        m.set_pv_f(0xc, begin + (NARROW - begin) * 0.25);
        m.set_pv_f(0x40, tune.k);
        m.set_pv_f(0x44, tune.d);
        m.set_pv_f(0x48, tune.turn);
        m.set_pv_i(0x14, 0);
        m.set_pv_s(0x2c, 0);
        m.set_pv_s(0x2e, 0);
        m.set_pv_i(0x30, 0);
        m.set_pv_i(0x24, -1);
        m.set_pv_i(0x10, -1);
        m.set_pv_i(0x20, -1);
    }
    let class = w.mobys[id].o_class;
    let mut pool = std::mem::take(&mut w.chopper);
    let (slot, _) = pool.slot(w, class);
    w.chopper = pool;
    w.mobys[id].set_pv_i(0x18, slot as i32);
    w.pulse.clear(slot);
}

/// The scope in first person (module docs).
fn scope(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if ctx.game.frame & 4 != 0 {
        let m = &mut w.mobys[id];
        m.set_pv_i(0x30, 0);
        m.set_pv_s(0x2c, 0);
        let (cam, rows) = (ctx.host.camera_pos(), ctx.host.camera_rows());
        let far = add(cam, scale(rows[0], 128.0));
        let met = ctx.coll.and_then(|c| crate::collision::coll_line_m(c, ctx.scene, cam, far, QueryFlags(0x10), ctx.hero_moby)).and_then(|o| o.moby);
        if let Some(t) = met.filter(|&t| w.mobys.get(t).is_some_and(|x| x.drawn != 0)) {
            let kind5 = w.class(w.mobys[t].o_class).is_some_and(|c| c.kind == 5);
            if aim::is_crate(w, t) || (kind5 && aim::health(w, t).is_some_and(|(hp, _)| 0.0 < hp)) {
                let m = &mut w.mobys[id];
                m.set_pv_i(0x30, t as i32);
                m.set_pv_s(0x2c, 1);
            }
        }
    }
    if ctx.hero.items.look_wait != 0 { return; }
    let ammo = ctx.hero.items.has_ammo(ctx.game);
    let cool = w.mobys[id].pv_f(4);
    if w.mobys[id].pv_s(0x2c) != 0 {
        let mut b = w.mobys[id].pv_s(0x2e) + 1;
        if 0x14 < b { b = 0; }
        w.mobys[id].set_pv_s(0x2e, b);
        if cool <= 0.0 && ammo && 0xb <= b { crate::hud::marker(&mut ctx.game.markers, 1.5, id, 0x4000_00ff, None, 0x20); }
    }
    let colour = if 0.0 < cool && ammo { 0x6000_00ff } else { 0x600f_ff0f };
    crate::hud::marker(&mut ctx.game.markers, 2.0, id, colour, None, 0x27);
}

/// State 1 (module docs).
fn ready(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, fp: bool) {
    if ctx.hero.state == 1 { ctx.hero.request_state(0x1d, true); }
    let ok = if fp {
        let ok = countdown(&mut ctx.hero.items.look_wait);
        w.mobys[id].set_pv_s(0, 1);
        ok
    } else {
        ctx.hero.items.look_wait = 0x28;
        true
    };
    let begin = w.mobys[id].pv_f(8);
    if fp {
        let held = ctx.hero.items.held;
        let mut a = w.mobys[id].pv_f(0xc);
        let zoom = if held & crate::pad::button::R1 != 0 && NARROW < a {
            Some((2, -0.01))
        } else if held & crate::pad::button::R2 != 0 && a < begin {
            Some((3, 0.01))
        } else {
            None
        };
        match zoom {
            Some((sound, step)) => {
                let s = w.mobys[id].pv_i(0x20);
                if s == -1 || !ctx.host.sound_playing(s) {
                    let h = ctx.host.play_sound(&w.mobys[id], sound, 4);
                    w.mobys[id].set_pv_i(0x20, h);
                }
                a += step;
            }
            None => stop_zoom_sound(w, id, ctx),
        }
        if begin < a {
            a = begin;
        } else if a < NARROW {
            a = NARROW;
        }
        w.mobys[id].set_pv_f(0xc, a);
        ctx.game.cam_tweaks.set_fov(a, 0.02, 0.2, 0.0, 3);
    } else {
        stop_zoom_sound(w, id, ctx);
        if w.mobys[id].pv_s(0) != 0 {
            w.mobys[id].set_pv_f(0xc, begin + (NARROW - begin) * 0.25);
            ctx.game.cam_tweaks.set_fov(begin, 0.005, 0.2, 0.0, 3);
            w.mobys[id].set_pv_s(0, 0);
        }
    }
    // The view's turn eases with the zoom.
    {
        let m = &w.mobys[id];
        let f = (m.pv_f(0xc) - NARROW) / (begin - NARROW);
        let t = &mut ctx.game.cameras.fp.tune;
        t.k = (m.pv_f(0x40) - 0.03) * f + 0.03;
        t.turn = (m.pv_f(0x48) - NARROW_TURN) * f + NARROW_TURN;
        t.d = (m.pv_f(0x44) - 0.4) * f + 0.4;
    }
    let it = ctx.hero.items;
    if it.no_use == 0 && it.pressed & it.trigger != 0 && ok {
        if ctx.hero.items.take_ammo(ctx.game, 1) {
            ctx.hero.items.use_start(ctx.hero.group);
            let m = &mut w.mobys[id];
            m.set_pv_s(2, 2);
            m.state = 2;
            ctx.host.set_anim(&w.mobys[id], 2, 0, 0);
        } else {
            ctx.host.play_sound(&w.mobys[id], 1, 0);
        }
    }
}

/// State 2's shot (module docs).
fn shoot(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, fp: bool) {
    let Some(r) = cur_row(ctx.game).copied() else { return };
    stop_zoom_sound(w, id, ctx);
    ctx.host.play_sound(&w.mobys[id], 0, 0);
    fire(w, id, ctx, fp, r.speed);
    let m = &mut w.mobys[id];
    m.set_pv_f(4, 1.0 / r.rate);
    m.state = 3;
    let n = m.pv_i(0x28) + 1;
    m.set_pv_i(0x28, n);
    if n == 5 { ctx.game.help(0x9f3, 0x4a); }
}

/// The light's tail of `0x3a4310` (module docs).
fn light(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let mut n = w.mobys[id].pv_i(0x14);
    if n == 0 { return; }
    let out = countdown(&mut n);
    w.mobys[id].set_pv_i(0x14, n);
    if out {
        free_light(w, id);
        return;
    }
    if w.mobys[id].pv_i(0x10) == -1 {
        let l = PointLight { color: [0.75, 1.0, 0.75], intensity: 0.0, pos: w.mobys[id].pos, radius: 15.0 };
        let s = w.point_lights.alloc(l).map_or(-1, |s| s as i32);
        w.mobys[id].set_pv_i(0x10, s);
    }
    let s = w.mobys[id].pv_i(0x10);
    if s < 0 { return; }
    let muzzle = ctx.host.joint(&w.mobys[id], 0).unwrap_or(w.mobys[id].pos);
    let off = if ctx.hero.items.raised { set_len(w.mobys[id].rows[1], -1.5) } else { set_len(ctx.hero.rows[0], 1.5) };
    let t = n as f32 / LIGHT_TICKS as f32;
    w.point_lights.set(s as usize, PointLight { color: [t * 0.75, t, t * 0.75], intensity: 0.0, pos: add(muzzle, off), radius: 15.0 });
}

/// `0x380ef8(speed, 128, 20°, …)`: the first-person aim — from the muzzle toward what the view meets 128 out (flags 0,
/// Ratchet left out; else its end), turned back to 20° off Ratchet's forward row when further; at `speed`.
fn view_aim(ctx: &Ctx, muzzle: V3, speed: f32) -> V3 {
    let (cam, rows) = (ctx.host.camera_pos(), ctx.host.camera_rows());
    let mut end = add(cam, scale(rows[0], 128.0));
    if let Some(o) = ctx.coll.and_then(|c| crate::collision::coll_line_m(c, ctx.scene, cam, end, QueryFlags(0), ctx.hero_moby)) { end = o.point; }
    let d = unit(sub(end, muzzle));
    let fwd = ctx.hero.rows[0];
    let most = 0.349_065_9f32;
    if dot(d, fwd) < most.cos() {
        set_len(crate::camera::rot(fwd, most, cross_ba(d, fwd)), speed)
    } else {
        scale(d, speed)
    }
}

/// `0x3a4f88` (module docs).
pub fn fire(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, fp: bool, speed: f32) {
    let f = speed * 0.016_666_668;
    let muzzle = ctx.host.joint(&w.mobys[id], 0).unwrap_or(w.mobys[id].pos);
    let mut dir = if ctx.hero.items.raised { set_len(w.mobys[id].rows[1], -f) } else { set_len(ctx.hero.rows[0], f) };
    // `0x2fda20(15, 0, muzzle, 0x7f608060)`, 30 ticks (the last one is out by now: the cooldown is longer).
    free_light(w, id);
    let l = PointLight { color: [0.75, 1.0, 0.75], intensity: 0.0, pos: muzzle, radius: 15.0 };
    let s = w.point_lights.alloc(l).map_or(-1, |s| s as i32);
    w.mobys[id].set_pv_i(0x10, s);
    w.mobys[id].set_pv_i(0x14, LIGHT_TICKS);
    let mut mode = 0;
    if !fp {
        // `0x32b4d0`: toward the lock's aim point.
        if let Some((t, _)) = ctx.hero.items.aim_target.filter(|&(t, _)| w.mobys.get(t).is_some()) { dir = set_len(sub(aim::aim_point(w, t), muzzle), f); }
    } else {
        mode = 1;
        dir = view_aim(ctx, muzzle, f);
    }
    let slot = w.mobys[id].pv_i(0x18);
    let shot = spawn(w, ctx, muzzle, dir, slot, mode, speed);
    if let Some(s) = shot { w.mobys[s].parent = Some(id); }
    // `0x2e14b0(1, muzzle, 1, Ratchet)`: a moby of class type 5 or a crate at the muzzle is hit from where it stands.
    let near = ctx.scene.map(|sc| crate::collision::coll_sphere_mobys(sc, muzzle, 1.0, QueryFlags(1), ctx.hero_moby)).and_then(|v| v.first().copied());
    if let (Some(n), Some(s)) = (near, shot) {
        if w.class(w.mobys[n].o_class).is_some_and(|c| c.kind == 5) || aim::is_crate(w, n) {
            let from = w.mobys[n].pos;
            hit(w, s, ctx, from);
        }
    }
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let g = &mut ctx.game.rng;
    for (size, rgba, spin) in [(7.0, 0x4010_ff10u32, 1i8), (5.0, 0x4010_ff10, -1), (3.5, 0xc0ff_ffff, 1), (2.5, 0xb0ff_ffff, -1)] {
        crate::particles::twinkle(sys, g, size, 0.1, 0.0, muzzle, 8, rgba, 2, spin, [0.0; 3], 1);
    }
}

/// `0x3a9060(pos, dir, slot, mode)`: the shot (module docs), when the slot is still the rifle's.
pub fn spawn(w: &mut MobyWorld, ctx: &mut Ctx, pos: V3, dir: V3, slot: i32, mode: i16, speed: f32) -> Option<usize> {
    if slot < 0 || !w.chopper.holds(HAND_CLASS, slot as usize) || w.class(SHOT).is_none() { return None; }
    let id = w.create(SHOT);
    let life = (128.0 / (speed * 0.016_666_668)) as i32 as i16;
    let up = ctx.hero.gravity.up_at(pos);
    let r0 = unit(dir);
    let r1 = unit(cross_ba(r0, scale(up, -1.0)));
    let r2 = cross_ba(r1, r0);
    let mut b = w.pulse.next_beam;
    if 3 < b { b = 0; }
    w.pulse.next_beam = (b + 1) % 3;
    {
        let m = &mut w.mobys[id];
        m.update_dist = 0xff;
        m.draw_dist = 0xff;
        m.drawn = 1;
        m.mode |= 0x41;
        m.pos = pos;
        m.state = 0;
        m.set_pv_s(0x12, mode);
        m.set_pv_v(0, dir);
        m.set_pv_s(0x10, life);
        m.set_pv_s(0x1a, slot as i16);
        m.set_pv_f(0x14, 1.0 / life as f32);
        m.set_pv_s(0x18, b as i16);
        m.rows = [r0, r1, r2];
        m.mode |= crate::moby::mode::KEEP_ROWS;
    }
    if ctx.hero.state != 0x1d { make_beam(w, id, ctx); }
    if mode == 1 {
        let (cam, rows) = (ctx.host.camera_pos(), ctx.host.camera_rows());
        let m = &mut w.mobys[id];
        m.set_pv_v(0x30, cam);
        m.set_pv_v(0x20, set_len(rows[0], len(dir)));
    }
    Some(id)
}

/// `0x3aa258`: the shot's beam started at it — its way, the up across it, from its place to one step on.
fn make_beam(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let m = &w.mobys[id];
    let (pos, vel, slot, b) = (m.pos, m.pv_v(0), m.pv_s(0x1a) as usize, m.pv_s(0x18) as usize);
    let axis = unit(vel);
    let s = unit(cross_ba(axis, ctx.hero.gravity.up_at(pos)));
    let bm = &mut w.pulse.slot(slot).beams[b.min(BEAMS - 1)];
    bm.axis = axis;
    bm.side = cross_ba(s, axis);
    bm.p[0] = pos;
    bm.p[1] = add(pos, vel);
    bm.p[2] = bm.p[1];
    bm.life = 0x3c;
    bm.inv = 1.0 / 60.0;
    bm.scroll = [0.0; 2];
    bm.live = true;
}

/// `0x3a92a8` (module docs).
pub fn update_shot(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].pvars.resize(crate::moby::PVAR_BYTES, 0);
    let slot = w.mobys[id].pv_s(0x1a);
    if slot < 0 || !w.chopper.holds(HAND_CLASS, slot as usize) {
        w.delete(id);
        return;
    }
    if w.mobys[id].state == 2 {
        tail(w, id, ctx);
        return;
    }
    let first = w.mobys[id].pv_s(0x12) == 1;
    let start = {
        let m = &mut w.mobys[id];
        let start = if first { m.pv_v(0x30) } else { m.pos };
        m.pos = add(m.pos, m.pv_v(0));
        if first {
            let p = add(m.pv_v(0x30), m.pv_v(0x20));
            m.set_pv_v(0x30, p);
        }
        start
    };
    if w.mobys[id].state == 0 {
        w.mobys[id].state = 1;
    } else {
        puff(w, ctx, slot as usize, start);
    }
    let pos = w.mobys[id].pos;
    puff(w, ctx, slot as usize, scale(add(start, pos), 0.5));
    if pos.iter().any(|&c| !(2.0..=1021.0).contains(&c)) {
        w.mobys[id].state = 2;
        return;
    }
    if !hit(w, id, ctx, start) && !count_s(w, id, 0x10) {
        tail(w, id, ctx);
        return;
    }
    w.mobys[id].state = 2;
}

/// `0x3a99d0`: the slot's smoke and the shot's beam step; while either lives its draw is asked for, else an ended shot
/// goes.
fn tail(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let slot = w.mobys[id].pv_s(0x1a) as usize;
    let n = step_puffs(w, ctx, slot);
    let b = step_beam(w, id);
    if n == 0 && !b {
        if w.mobys[id].state == 2 { w.delete(id); }
    } else {
        w.register_draw(id);
    }
}

/// `0x3a9528(shot, from)` (module docs): the line from `from` to the shot (in first person its ray's point); true when
/// it met something.
pub fn hit(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, from: V3) -> bool {
    let m = &w.mobys[id];
    let to = if m.pv_s(0x12) == 1 { m.pv_v(0x30) } else { m.pos };
    let d = unit(m.pv_v(0));
    let damage = cur_row(ctx.game).map_or(0.0, |r| r.damage);
    let mut t = HitTemplate::new(damage, Some(id), 0x10000, [d[0], d[1], d[2], NO_KNOCK_LIMIT]);
    t.kind = (m.o_class as u16 as u32) << 16 | 0x0101;
    let Some(o) = ctx.coll.and_then(|c| crate::collision::coll_line_m(c, ctx.scene, from, to, QueryFlags(0), ctx.hero_moby)) else { return false };
    if let Some(target) = o.moby { w.deliver_hit_any(target, &t, [o.point[0], o.point[1], o.point[2], 1.0]); }
    impact(ctx, o.point);
    true
}

/// `0x3a9668(shot, at)` of the base rows: eight times a green twinkle (10) and a white one (2) jittered 0.5 about `at`,
/// 6..16 ticks, turning either way.
fn impact(ctx: &mut Ctx, at: V3) {
    if ctx.game.upgrade_level(PULSE) != 0 { return; }
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let g = &mut ctx.game.rng;
    for _ in 0..8 {
        let life = g.rand_range(6, 0x10) as i16;
        let spin = if g.randi(2) != 0 { -1 } else { 1 };
        let p: V3 = std::array::from_fn(|k| at[k] + g.randf_signed(0.0, 0.5));
        crate::particles::twinkle(sys, g, 10.0, 0.1, 0.0, p, life, 0x4010_ff10, 2, spin, [0.0; 3], 1);
        crate::particles::twinkle(sys, g, 2.0, 0.1, 0.0, p, life, 0xc0ff_ffff, 2, spin, [0.0; 3], 1);
    }
}

/// `0x3a9aa0(at, slot)`: a puff of smoke in the slot's first free record (module docs); none when all 80 are in use.
fn puff(w: &mut MobyWorld, ctx: &mut Ctx, slot: usize, at: V3) {
    let s = w.pulse.slot(slot);
    let Some(p) = s.puffs.iter_mut().find(|p| !p.live) else { return };
    let g = &mut ctx.game.rng;
    p.live = true;
    p.pos = at;
    p.roll = g.rand_angle();
    p.spin = g.randf_signed(1.0, 2.0) * 0.017_453_294;
    p.size = g.randf(1.0, 0.5);
    p.life = g.rand_range(0x3c, 0x5a) as i16;
    p.inv = 1.0 / p.life as f32;
    // `$gp+0x700`, in the look stance's twin `+0x704`: both 0.1.
    p.rise = g.randf(0.0, 0.1) * 0.016_666_668;
    p.push = [0.0; 9];
    for k in 0..9 { p.grow[k] = g.randf(0.000_75, 0.0075); }
    p.up = ctx.hero.gravity.up_at(at);
}

/// `0x3a9c08`: every live puff of the slot ages; the ones still alive turn, grow and rise. Their count.
fn step_puffs(w: &mut MobyWorld, ctx: &Ctx, slot: usize) -> usize {
    let mut n = 0;
    for p in w.pulse.slot(slot).puffs.iter_mut().filter(|p| p.live) {
        let mut life = p.life as i32;
        let out = countdown(&mut life);
        p.life = life as i16;
        if out {
            p.live = false;
            continue;
        }
        n += 1;
        p.roll = add_rot(p.roll, p.spin);
        for k in 0..9 { p.push[k] += p.grow[k]; }
        p.up = ctx.hero.gravity.up_at(p.pos);
        p.pos = add(p.pos, scale(p.up, p.rise));
    }
    n
}

/// `0x3aa430`: the shot's beam ages; alive, its far end on the shot, its textures scroll and its near points slide 4 a
/// second along the shot's way. Whether it lives.
fn step_beam(w: &mut MobyWorld, id: usize) -> bool {
    let m = &w.mobys[id];
    let (pos, vel, slot, b) = (m.pos, m.pv_v(0), m.pv_s(0x1a) as usize, m.pv_s(0x18) as usize);
    let bm = &mut w.pulse.slot(slot).beams[b.min(BEAMS - 1)];
    if !bm.live { return false; }
    let mut life = bm.life as i32;
    let out = countdown(&mut life);
    bm.life = life as i16;
    if out {
        bm.live = false;
        return false;
    }
    bm.p[2] = pos;
    for (k, step) in [0.05f32, 0.075].into_iter().enumerate() {
        let s = bm.scroll[k] + step;
        bm.scroll[k] = if 4.0 <= s { s - 4.0 } else { s };
    }
    let mv = set_len(vel, 4.0 * 0.016_666_668);
    bm.p[0] = add(bm.p[0], mv);
    bm.p[1] = add(bm.p[1], mv);
    true
}

/// `0x294a40` (x, y, z) and `0x294ad0` (s, t): the puff's 3 × 3 corners; `$gp+0x698`: its four quads' corners.
const CORNERS: [V3; 9] = [[0.0, -0.5, 0.5], [0.0, -0.5, 0.0], [0.0, -0.5, -0.5], [0.0, 0.0, 0.5], [0.0, 0.0, 0.0], [0.0, 0.0, -0.5], [0.0, 0.5, 0.5], [0.0, 0.5, 0.0], [0.0, 0.5, -0.5]];
const CORNER_ST: [[f32; 2]; 9] = [[0.0, 0.0], [0.5, 0.0], [1.0, 0.0], [0.0, 0.5], [0.5, 0.5], [1.0, 0.5], [0.0, 1.0], [0.5, 1.0], [1.0, 1.0]];
const QUADS: [[usize; 4]; 4] = [[0, 1, 3, 4], [1, 2, 4, 5], [3, 4, 6, 7], [4, 5, 7, 8]];
/// `0x294b20`: the beams' ring (y, z of a unit circle in 20 steps).
const RING: [[f32; 2]; 20] = [
    [-0.951_057, -0.309_017], [-0.809_017, -0.587_786], [-0.587_785, -0.809_017], [-0.309_017, -0.951_057], [-0.0, -1.0],
    [0.309_017, -0.951_057], [0.587_785, -0.809_017], [0.809_017, -0.587_785], [0.951_057, -0.309_017], [1.0, -0.0],
    [0.951_057, 0.309_017], [0.809_017, 0.587_785], [0.587_785, 0.809_017], [0.309_017, 0.951_057], [-0.0, 1.0],
    [-0.309_017, 0.951_057], [-0.587_785, 0.809_017], [-0.809_017, 0.587_785], [-0.951_056, 0.309_017], [-1.0, 0.0],
];

/// `0x3a9a48` (module docs): gone once the slot is not the rifle's; else all the slot's smoke (`0x3a9d70`) and beams
/// (`0x3aa5d0`). `hero_state`: Ratchet's (`0x18c0b4`).
pub fn draw(w: &mut MobyWorld, id: usize, hero_state: i32, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let slot = w.mobys[id].pv_s(0x1a);
    if slot < 0 || !w.chopper.holds(HAND_CLASS, slot as usize) {
        w.delete(id);
        return;
    }
    let s = w.pulse.slot(slot as usize);
    draw_puffs(s, hero_state, cam, out);
    draw_beams(s, cam, out);
}

/// `0x3a9d70`: each puff's patch (module docs).
fn draw_puffs(s: &Slot, hero_state: i32, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    use crate::fx::FxQuad;
    let most = if hero_state == 0x1d { 32.0 } else { 128.0 };
    for p in s.puffs.iter().filter(|p| p.live) {
        let f = p.life as f32 * p.inv;
        let a = (most * f) as i32 as u32;
        let t = (f * std::f32::consts::PI * 4.0).sin().abs();
        let c = a << 24 | crate::hud::lerp_rgba(t, 0x60_4040, 0x40_8040);
        let size = p.size + (0.5 - p.size) * f;
        let to_cam = unit(sub(cam, p.pos));
        let side = unit(cross_ba(to_cam, p.up));
        let up2 = cross_ba(side, to_cam);
        let (sn, cs) = p.roll.sin_cos();
        let lift = (1.0 - f) * 0.075;
        let world = |l: V3| add(p.pos, add(add(scale(to_cam, l[0]), scale(side, l[1])), scale(up2, l[2])));
        for q in QUADS {
            let mut a4 = [[0.0; 3]; 4];
            let mut b4 = [[0.0; 3]; 4];
            let mut st = [[0.0; 2]; 4];
            for (j, &i) in q.iter().enumerate() {
                let c0 = CORNERS[i];
                let d = if p.push[i] == 0.0 { [0.0; 3] } else { set_len(c0, p.push[i]) };
                let e = add(c0, d);
                let l = scale([e[0], e[1] * cs - e[2] * sn, e[1] * sn + e[2] * cs], size);
                a4[j] = world(l);
                b4[j] = world(add(l, scale(up2, lift)));
                st[j] = CORNER_ST[i];
            }
            out.push(FxQuad { fx: 0x36, additive: false, corners: a4, st, rgba: [c; 4] });
            out.push(FxQuad { fx: 0x36, additive: true, corners: b4, st, rgba: [c; 4] });
        }
    }
}

/// `0x3aa5d0`: each beam's rings and strips (module docs).
fn draw_beams(s: &Slot, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    use crate::fx::FxQuad;
    for b in s.beams.iter().filter(|b| b.live) {
        let f = b.life as f32 * b.inv;
        let across = cross_ba(b.axis, b.side);
        let rings: [[V3; 20]; 3] = std::array::from_fn(|k| std::array::from_fn(|i| add(b.p[k], add(scale(across, RING[i][0] * 0.2), scale(b.side, RING[i][1] * 0.2)))));
        let far = ((f * 64.0) as i32 as u32) << 24 | 0x70_8070;
        let sc = b.scroll[0];
        for k in 0..2 {
            let near = if k == 0 { 0x80_6060 } else { far };
            let mut t0 = 0.0f32;
            for i in 0..20 {
                let t1 = t0 + 0.05;
                let j = (i + 1) % 20;
                out.push(FxQuad {
                    fx: 0x15,
                    additive: false,
                    corners: [rings[k + 1][i], rings[k + 1][j], rings[k][i], rings[k][j]],
                    st: [[sc, t0], [sc, t1], [sc + 4.0, t0], [sc + 4.0, t1]],
                    rgba: [far, far, near, near],
                });
                t0 = t1;
            }
        }
        let to_cam = sub(cam, b.p[0]);
        let strip = |out: &mut Vec<FxQuad>, fx: usize, wide: f32, c: u32, s: [[f32; 2]; 2]| {
            let wv = set_len(cross_ba(b.axis, to_cam), wide);
            for k in 0..2 {
                let near = if k == 0 { 0x80_4040 } else { c };
                let (p, q) = (b.p[k + 1], b.p[k]);
                out.push(FxQuad { fx, additive: true, corners: [add(p, wv), sub(p, wv), add(q, wv), sub(q, wv)], st: [[s[k][0], 0.0], [s[k][0], 1.0], [s[k][1], 0.0], [s[k][1], 1.0]], rgba: [c, c, near, near] });
            }
        };
        strip(out, 0xe, 1.0, ((f * 80.0) as i32 as u32) << 24 | 0x40_8040, [[0.0, 0.5], [0.5, 3.0]]);
        let s2 = b.scroll[1];
        strip(out, 0x17, 0.1, ((f * 255.0) as i32 as u32) << 24 | 0x40_4080, [[0.0, s2 + 0.5], [s2 + 0.5, s2 + 3.0]]);
    }
}
