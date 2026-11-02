//! The upgrade screen (game mode 8): a weapon in Ratchet's hand that has earned its upgrade (crate::upgrades, the tick
//! `0x2d6258` → `0x37fb10(item, next row)`) is shown changing into its next version, away from the world, before it
//! comes back to his hand.
//!
//! **The enter** (`0x37fb50`): the item (`0x1b25f8`) and its next row (`0x1b25fa`); the fade's ticks 20 (`0x1b25f2`,
//! `gp−0x14d8`), the dialogue line 0xdb7 asked for when the dialogue is free (`0x1a63cc`); the moby shown (`0x1b2620`):
//! the hand item's own (its slot `0x18b040 + 0x50·slot`, the record's `−0x04`; `0x1b2625` set), or for the items 0x1f
//! / 0x2d / 9 a preview moby of their row's class (0x259 for 9; `0x37fd78`).
//!
//! **The frame** (`0x380c50` → `0x37fec8`; the world stands: no hero, moby or camera update), by its state
//! `0x1b25f0` and its ticks `0x1b25f2` (`0x2eeae0`: down to 0):
//! * **0**: the screen fades to black (`0x380770`: black at `(1 − t/20)·255`) over the 20 ticks; then the camera at
//!   (25, 25, 900) facing along x (`0x1b8d40`, rows `0x1b8f70` the identity), tan `0.5662` for the view (`0x1bd2f0`),
//!   the moby at rest (its angles 0, the hand item's pitch −90°) and shown, **1** for 25 ticks (`gp−0x14d4`);
//! * **1**: the moby placed ([`Screen::place`], `0x3803e0`): 3 ahead of the camera (2 for the gloves 0x879 / 0xaf6 / 0x907 /
//!   0xe7a, 2.5 for 0xc5a / 0xa65), turning 2° a tick (the hand item about its x, a preview about its z), then
//!   **2** for 40 ticks (`gp−0x14d0`);
//! * **2**: placed; the glows ([`Screen::glows`], `0x3807f0`): two blue (`0xff2020`) and a white half as large, 420000 across
//!   (`gp−0x14b4` 2 × 210000), coming from 9 ahead to 0.1 ahead of the camera over the 40 ticks; then **3** (2
//!   ticks, the white flash full);
//! * **3**: the white flash (`0x380a58`: white at alpha `0x1b2626`); then the moby swapped ([`Screen::swap`], `0x3805e0`) for one
//!   of the next row's class at its place and angles (its sequence 1 when it has two), the hand item's own deleted
//!   with the slot's last held item and creation (`0x1a7398`, `0x18b040`, `0x18c104`), **4** for 12 ticks;
//! * **4**: placed; the flash fading over the 12 ticks; **5** for 80 ticks (`gp−0x14cc`); **5**: placed; then **6** for 3;
//! * **6**: placed, black at `(1 − t/40)·255`; then the camera back (`0x1b25fd`), the landing lag 25 (`0x18a000`),
//!   the shown moby gone, the upgrade (`0x2f4288`) and its message (`0x2f48e0`), the hand item asked for again
//!   (`0x3227d8`), **7**;
//! * **7**: black fading out over 20 ticks, then game mode 0 (`0x32c5f0(0, 2, ..)`).
//!
//! **The lightning** ([`Screen::lightning`], `0x380ac0`, at the end of each placing, states 1..6): six beams a tick
//! (crate::moby::classes::beam) from 9 ahead of the camera to 3 ahead turned by a random 0.1..3 (8..12 ticks; widths
//! `gp−0x14b0..`: 1.25, 0.4, 0.3, 0.8; `0x808080` / `0x808080` / `0xff1010`, the glow `0xff1010` → `0xff1010`). The
//! frame (`0x380c50`) runs the beams' updates before its state, and their draws are the frame's only callbacks.
//!
//! Not ported: the wait for the dialogue's stream in state
//! 0 (`0x1a641a`, up to 0xb5 ticks), the other mobys the frame animates (0xdc1's update, 0xcb2 / 0xb02 / 0x259 / 0xa5c /
//! 0x1192: none on the screen), the items 0x3d / 0x49 / 0x4d / 0x5d / 0x5e / 0x6b / 0x6c's swap to the hidden preview
//! (`0x37fe70`: their weapons are not ported).

use crate::hero::physics::{add, euler_rows, scale, V3};
use crate::moby::{Ctx, MobyWorld};

/// The game mode.
pub const MODE: i32 = 8;
/// `0x1b8d40`: the camera's place; its rows the identity (`0x1b8f70`).
pub const EYE: V3 = [25.0, 25.0, 900.0];
pub const ROWS: [V3; 3] = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
/// `0x1bd2f0` = sin / cos of `0x3f0fea69` (0.5662): the view's half-angle tangent.
pub fn tan() -> f32 { let a = f32::from_bits(0x3f0f_ea69); a.sin() / a.cos() }
/// `gp−0x14d8..−0x14cc` (`0x1adb18..`): the fades' ticks, state 1's, state 2's and state 5's.
const FADE: i16 = 20;
const SHOW: i16 = 25;
const GROW: i16 = 40;
const HOLD: i16 = 80;
/// `0x3d0efa36`: the turn a tick (2°).
const TURN: f32 = 0.034_906_59;
/// `gp−0x14c8..−0x14b8`: the hand item's offsets along its rows 1 and 0 (gloves 0.2 / 0, else 0.3 / −0.1), a
/// preview's along its row 2 (−0.2).
const HAND_ACROSS: [f32; 2] = [0.2, 0.3];
const HAND_ALONG: [f32; 2] = [0.0, -0.1];
const PREVIEW_DOWN: f32 = -0.2;
/// `gp−0x14b4`: the glows' size factor (× 210000).
const GLOW_SIZE: f32 = 2.0;
/// The glows' particle definition (`*0x1fe994`: the table `0x1fe8c0`'s 0x35).
const GLOW_DEF: u8 = 0x35;
/// The dialogue line the screen asks for (`0x1a63cc`).
pub const LINE: i32 = 0xdb7;
/// `0x37fe70`'s hidden preview class.
const PREVIEW_CLASS: i16 = 0x3ef;

/// The screen while it runs (module docs).
#[derive(Clone, Debug, Default)]
pub struct Screen {
    /// `0x1b25f8` / `0x1b25fa`.
    pub item: i32,
    pub next: i32,
    /// `0x1b25f0`, `0x1b25f2`, `0x1b25f4` (1 / the ticks of the fade or the glows).
    pub state: u8,
    pub ticks: i16,
    pub inv: f32,
    /// `0x1b2620`: the moby shown; `0x1b2625`: it is the hand item's own.
    pub moby: Option<usize>,
    pub hand: bool,
    /// `0x1b2624`: the glows' spin; `0x1b2626`: the flash's alpha.
    pub spin: u8,
    pub flash: i32,
    /// `0x1b25fd`: the camera back.
    pub camera_on: bool,
    /// This frame's view `(eye, rows, tan)` (none: the follow camera as it stands) and full-screen fills (PS2 alpha).
    pub view: Option<(V3, [V3; 3], f32)>,
    pub fills: Vec<[u8; 4]>,
}

/// `0x2eeae0(&t)`: the ticks down by one; true once they are out.
fn tick_out(t: &mut i16) -> bool {
    if *t == 0 { return true; }
    *t = (*t).max(1) - 1;
    *t < 1
}

/// The black fade at `(1 − t·inv)·255` (`0x380770`; past 0x80 it is black).
fn black(ticks: i16, inv: f32) -> [u8; 4] { [0, 0, 0, ((1.0 - ticks as f32 * inv) * 255.0).clamp(0.0, 128.0) as u8] }

impl Screen {
    /// `0x37fb50`: the screen for `item` going to row `next` (module docs).
    pub fn open(w: &mut MobyWorld, ctx: &mut Ctx, item: i32, next: i32) -> Screen {
        let mut s = Screen { item, next, ticks: FADE, inv: 1.0 / FADE as f32, ..Default::default() };
        if ctx.host.speech_free() { ctx.host.request_speech(LINE); }
        let preview = match item {
            0x1f | 0x2d => crate::hero::items::row(ctx.game, item).map(|r| r.class),
            9 => Some(0x259),
            _ => None,
        };
        match preview {
            Some(class) => s.moby = preview_moby(w, ctx, class, if item == 0x1f { 1 } else { -1 }),
            None => {
                s.moby = ctx.hero.items.hand;
                s.hand = true;
            }
        }
        s
    }

    /// `0x37fec8` (module docs). True when the screen is over (game mode 0 asked for).
    pub fn frame(&mut self, w: &mut MobyWorld, ctx: &mut Ctx) -> bool {
        self.fills.clear();
        // `0x380c50`: the beams' updates (the frame's draw lists start empty: they are its only callbacks).
        w.draws.clear();
        w.draws_first.clear();
        for id in 0..w.mobys.len() {
            let m = &w.mobys[id];
            if m.o_class != crate::moby::classes::beam::CLASS || 0x80 <= m.state { continue; }
            if let Some(f) = m.update { f(w, id, ctx); }
        }
        let shown = |s: &Screen| Some((EYE, ROWS, tan())).filter(|_| !s.camera_on);
        match self.state {
            0 => {
                if !tick_out(&mut self.ticks) {
                    self.fills.push(black(self.ticks, self.inv));
                    self.view = None;
                    return false;
                }
                self.state = 1;
                self.ticks = SHOW;
                if let Some(m) = self.moby.and_then(|id| w.mobys.get_mut(id)) {
                    m.rot = if self.hand { [0.0, -std::f32::consts::FRAC_PI_2, 0.0] } else { [0.0; 3] };
                    m.mode &= !0x41;
                }
            }
            1 => {
                self.place(w, ctx);
                if tick_out(&mut self.ticks) {
                    self.ticks = GROW;
                    self.state = 2;
                    self.inv = 1.0 / GROW as f32;
                    self.spin = 0;
                }
            }
            2 => {
                self.place(w, ctx);
                self.glows(ctx);
                if tick_out(&mut self.ticks) {
                    self.ticks = 2;
                    self.state = 3;
                    self.flash = 0x80;
                    self.fills.push([0xff, 0xff, 0xff, 0x80]);
                }
            }
            3 => {
                self.fills.push([0xff, 0xff, 0xff, self.flash.clamp(0, 0x80) as u8]);
                if tick_out(&mut self.ticks) {
                    self.ticks = 0xc;
                    self.state = 4;
                    self.swap(w, ctx);
                }
            }
            4 => {
                self.place(w, ctx);
                self.flash = (self.ticks as f32 * 0.083_333_336 * 128.0) as i32;
                self.fills.push([0xff, 0xff, 0xff, self.flash.clamp(0, 0x80) as u8]);
                if tick_out(&mut self.ticks) {
                    self.state = 5;
                    self.ticks = HOLD;
                    self.flash = 0;
                }
            }
            5 => {
                self.place(w, ctx);
                if tick_out(&mut self.ticks) {
                    self.ticks = 3;
                    self.state = 6;
                }
            }
            6 => {
                self.place(w, ctx);
                self.fills.push(black(self.ticks, self.inv));
                if tick_out(&mut self.ticks) {
                    self.ticks = 0;
                    self.inv = 1.0 / FADE as f32;
                    self.camera_on = true;
                    ctx.hero.land_lag = FADE + 5;
                    if let Some(id) = self.moby.take() {
                        if w.mobys.get(id).is_some_and(|m| m.state < 0x80) { w.delete(id); }
                    }
                    let records = &ctx.level.items;
                    ctx.game.upgrade(records, self.item);
                    ctx.game.upgrade_message(self.item);
                    if self.hand {
                        let held = ctx.game.held;
                        ctx.hero.items.ask(held, self.item);
                    }
                    self.state = 7;
                }
            }
            _ => {
                let t = self.ticks;
                self.ticks += 1;
                if t < FADE {
                    self.fills.push(black(t, self.inv));
                } else {
                    let hp = ctx.hero.hp;
                    ctx.game.scenes.request(0, false, 0, 0, hp);
                    self.view = None;
                    return true;
                }
            }
        }
        self.view = shown(self);
        false
    }

    /// `0x3803e0(moby)`: the shown moby before the camera (module docs), turning; its rows from its angles.
    fn place(&mut self, w: &mut MobyWorld, ctx: &mut Ctx) {
        let Some(id) = self.moby.filter(|&id| w.mobys.get(id).is_some_and(|m| m.state < 0x80)) else { return };
        let class = w.mobys[id].o_class;
        let wide = matches!(class, 0xc5a | 0xa65);
        let glove = wide || matches!(class, 0x879 | 0xaf6 | 0x907 | 0xe7a);
        let d = if wide { 2.5 } else if glove { 2.0 } else { 3.0 };
        let m = &mut w.mobys[id];
        let mut pos = add(EYE, scale(ROWS[0], d));
        let k = if self.hand { 0 } else { 2 };
        m.rot[k] = crate::hero::physics::add_rot(m.rot[k], TURN);
        let rows = euler_rows(m.rot);
        if !self.hand {
            pos = add(pos, scale(rows[2], PREVIEW_DOWN));
        } else {
            let g = (!glove) as usize;
            pos = add(pos, scale(rows[1], HAND_ACROSS[g]));
            pos = add(pos, scale(rows[0], HAND_ALONG[g]));
        }
        m.pos = pos;
        m.rows = rows;
        w.rebuild(id, &*ctx.host);
        self.lightning(w, ctx);
    }

    /// `0x380ac0` (module docs, "The lightning").
    fn lightning(&mut self, w: &mut MobyWorld, ctx: &mut Ctx) {
        const WIDTHS: [f32; 4] = [1.25, 0.4, 0.3, 0.8];
        for _ in 0..6 {
            let from = add(EYE, scale(ROWS[0], 9.0));
            let r = ctx.game.rng.rand_vec(0.1, 3.0);
            let to = add(add(EYE, scale(ROWS[0], 3.0)), r);
            let life = ctx.game.rng.rand_range(8, 0xc);
            crate::moby::classes::beam::spawn(w, WIDTHS, from, to, life, [0x80_8080, 0x80_8080, 0xff_1010, 0xff_1010, 0xff_1010]);
        }
    }

    /// `0x3807f0`: the glows of a tick (module docs).
    fn glows(&mut self, ctx: &mut Ctx) {
        let n = self.spin;
        self.spin = self.spin.wrapping_add(1);
        let Some(sys) = ctx.parts.as_deref_mut() else { return };
        let (far, near) = (add(EYE, scale(ROWS[0], 9.0)), add(EYE, scale(ROWS[0], 0.1)));
        let f = self.ticks as f32 * self.inv;
        let at = add(near, scale(crate::hero::physics::sub(far, near), f));
        let size = GLOW_SIZE * 210_000.0;
        let def = sys.def_first(GLOW_DEF);
        let other = ((n as u32 + 0x80) % 0xff) as u8;
        for (sz, rgb, spin) in [(size, 0xff_2020, n), (size, 0xff_2020, other), (size * 0.5, 0xff_ffff, other)] {
            if let Some(i) = crate::particles::sprite(sys, &mut ctx.game.rng, sz, sz, 0.0, at, [0.0; 3], 4, rgb, spin, 0x7f, 0x7f, 0) {
                sys.pool.recs[i][2] = def;
                sys.pool.recs[i][9] = 0x40;
            }
        }
    }

    /// `0x3805e0(moby)`: the moby swapped for one of the next row's class at its place and angles (module docs).
    fn swap(&mut self, w: &mut MobyWorld, ctx: &mut Ctx) {
        let Some(old) = self.moby else { return };
        let Some((pos, rot)) = w.mobys.get(old).map(|m| (m.pos, m.rot)) else { return };
        let class = crate::hero::items::row_at(self.next.max(0) as usize).map(|r| r.class);
        w.delete(old);
        if self.hand {
            let it = &mut ctx.hero.items;
            it.hand = None;
            it.ready = 0;
            it.item = 0;
            it.selected = 0;
            it.next_create = ctx.game.frame;
            ctx.game.held = 0;
        }
        self.moby = class.and_then(|c| preview_moby(w, ctx, c, -1));
        if let Some(m) = self.moby.and_then(|id| w.mobys.get_mut(id)) {
            m.pos = pos;
            m.rot = rot;
        }
        if self.moby.is_none() { eprintln!("rc_game::menus::upgrade: row {:#x}'s class is not loaded", self.next); }
    }
}

/// `0x37fd78(class, seq)`: a preview moby of `class` — drawn (draw distance 0x20, `+0x30` 0xff), its rows kept, at the
/// screen; its sequence `seq` (−1: 1 when the class has two or more).
fn preview_moby(w: &mut MobyWorld, ctx: &mut Ctx, class: i16, seq: i32) -> Option<usize> {
    if class == PREVIEW_CLASS || w.class(class).is_none() { return None; }
    // The class header's `+0x0c`: its sequences (one reaction slot each, crate::moby::ClassInfo::reactions).
    let seqs = w.class(class).map_or(0, |c| c.reactions.len());
    let id = w.create(class);
    let m = &mut w.mobys[id];
    m.mode = (m.mode | crate::moby::mode::KEEP_ROWS) & !0x43;
    m.update_dist = 0xff;
    m.draw_dist = 0x20;
    m.drawn = 1;
    m.pos = [5.0, 5.0, 900.0];
    let s = if 0 <= seq { Some(seq as u8) } else { (2 <= seqs).then_some(1) };
    if let Some(s) = s { ctx.host.set_anim(&w.mobys[id], s, 0, 0); }
    Some(id)
}
