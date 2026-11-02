//! The back slot (slot 3 of the item slots `0x18b040 + 0x50·k`): Clank on Ratchet's back and the pack he wears, and
//! Clank's head glow. Its mobys are made by `0x2afc68`, placed at Ratchet's attachment by `0x2b09e0`, run by the slot
//! loop `0x2b2060` and shown or hidden by `0x2d3408` / `0x2d3208`; the hero's code chooses their sequences.
//!
//! **The mobys** (`0x2afc68`, when there are none): the pack `0x18b130` of the back item `0x18b158` (the request
//! `0x18c110`, else the saved back item `0x1a73a4`, else 2 — the Heli-Pack, class 0x25f — or 3 with the Thruster-Pack
//! `0x1a7a11`, class 0x260; their rows' `+0x04`), then Clank `0x18b134` (item 1's row: class 0x259). Each: draw
//! distance 0x20, drawn, lit as Ratchet, the shine 0x10 for a class with metal packets, the slot ready (`0x18b154` =
//! 2), hidden (mode |= 0x41) while Clank is away from the back (`0x18c31c`: from the level start without him,
//! `0x2ab1a8`, and while Endako's rescue plays). The game makes them on every level, Clank had or not; the port
//! does too, so the hero's draws on the random stream below are the game's.
//!
//! **Their place** (`0x2b09e0`, slot 3; not in a scene, game modes 2 / 6): both at the attachment of the back item's
//! row (`+0x00`: 5, Ratchet's joint list 5), out of the moby loop (mode |= 6): the attachment's translation, its
//! rows with their columns made unit (`0x31f858`), the pitch and yaw `+0xf4` / `+0xf8` of the first row, their own
//! animation advanced (`0x313420`). After Clank, his **head glow** `0x18b13c` (class 0x4b4; made once: the glow word
//! `0x801432d7`, draw distance 0x40, its scale × 1.3), while shown at Clank's joint list 8 (`0x312728`; its place is
//! also `0x18be30`, where the Clank Zapper's lines start), lit as Ratchet, its glow word pulsing over 120 ticks:
//! s = sin(2π·(frame mod 120) / 120 − π), red 0xd7 + 90·s (at most 0xff), green 0x32 + 50·s, blue 0x14 + 10·s.
//!
//! **The slot loop** (`0x2b2060`, slot 3): a pack whose sequence 0 has run out goes to sequence 1 (blend 2) when its
//! class has more than one; then the pack's class update — the Heli-Pack's `0x39e7a8` (L00): in the glide 8 its
//! sequence 6 (the rotors), blend 10. The Thruster-Pack (0x260) has none.
//!
//! **The sequences the hero chooses** (`0x2d2140`, tables by the back item: the Heli-Pack's `$gp−0x7290`, the
//! Thruster-Pack's `$gp−0x72a8`, item 4's `$gp−0x7278`; records {Ratchet's sequence s16, the pack's u8, Clank's u8}):
//! of the records for Ratchet's sequence one is drawn (`0x31b9e0(0, n − 1)`, a draw even of one) and its pair given.
//! * Ratchet's SetAnim (`0x2d1fe0`, from `0x2d2508`, no body): with a record for the new sequence (not the idle 0) and
//!   the pack in another sequence, both take the record's from the same frame with SetAnim's blend (a negative blend
//!   its curve's ticks, `$gp−0x71f8`: 11, 16, …); else both go back to their rest 1 (blend 7; 0x13 leaving the glide
//!   with the Heli-Pack) — unless Ratchet goes idle in group 0 with the pack playing one of the table's sequences.
//! * Ratchet's advance (`0x2d2278`, from `0x2d2888`): with a record for his sequence, the pack and Clank play at his
//!   speed `0x18aa40`.
//! * Idle (`0x2cb650` case 0): both back to 1 (blend 7) when their sequence has run out; and to 1 (blend 12) while his
//!   head is turned far (record 3's yaw over 35° or its nod over 25°). Then Clank's fidget: in the idle's 11th tick or
//!   later with Clank at rest, when the timer `0x18c308` runs out and the head is near ahead (yaw under 32°, nod under
//!   20°): the next in 110..270 ticks (`0x31b9e0`) and, with the pack at rest, a record for the idle 0 (`0x2d1e88`,
//!   blend 7).
//! * The run 2 (`0x2b8dc0`, from `0x2ba8e0`): Clank's record 18 (his joint list 0) springs (0.02, 0.35) to the
//!   turn's yaw −1.2 × the yaw residual (within ±1.4, at most ±75°) and up 0.07 (`0.1·|residual| + 0.1`, at most
//!   0.07) in his units.
//!
//! **Clank's records** (L00 `0x25c9f0..0x25ce10`, records 18..23 of `0x25bd90`, `+0xa2` = 1: on the back moby,
//! `0x2a4fe8(1)`): his joint lists 0..5, sprung with Ratchet's (`0x2a5368` → `0x2a5048`) and linked onto his moby;
//! record 18's scale is Clank's head scale with the big-head cheat (`0x2ab498`, `0x2b8dc0`).
//!
//! **Clank's glow and blink** (`0x2a5840`, from Ratchet's update with Clank on the back): his glow word (mode 0x10)
//! pulsing over 110 ticks — s = sin(2π·(frame mod 110) / 110 − π), red base + 24·s + 0xc (base 0x38, 0x88 with one
//! health left), green 0xa0 + 48·s, blue 0x4c + 24·s — and, for 45 ticks after a hurt (`0x18a02e`, set by the hurt
//! entries 0x16 / 0x42, counted down by `0x2c3330`), eased toward red; his blink (not in state 0x7c): when the timer
//! `0x18afac` has run out with Clank at rest, the next in 50..200 ticks (`0x31b9e0`) and four nodes (`0x18aea0`, mode
//! 1, his lists 2..5, `$gp−0x70d0`) linked for 21 ticks, their rotations, scales and translations (× his scale) from
//! `$gp−0x7190` / `−0x7150` / `−0x7110`, their weight from `0x25de60`.
//!
//! Not ported: the slot's item change (`0x2b13c8` for slot 3: the request `0x18c110`, the saved back item
//! `0x1a73a4`), the Thruster-Pack's jets (class 0xa7, `0x391d50(0 / 1)` at the pack's making, their update `0x391db0`
//! and draw `0x391228`), the Hydro-Pack's bubbles in the fast swim (`0x2bb8dc`, class 0x261: no level before 6 has
//! water), the glow's draw callback `0x2a7a88` (`0x2a7ac8`), the body's glow (`0x2a5840` on the body moby, crate::hero::body),
//! Clank's records 19..23 (nothing found that gives them targets).

use super::physics::V3;
use super::posture::PoseRec;
use super::Hero;
use crate::moby::{mode, Ctx, MobyWorld};
use rc_formats::moby_anim::JointModifier;

/// Clank's moby class on the back (item 1's row `+0x04`; also the Clank Zapper upgrade row 0x49's).
pub const CLANK_CLASS: i16 = 0x259;
/// Clank's head glow (`0x18b13c`).
pub const GLOW_CLASS: i16 = 0x4b4;
/// The Heli-Pack (item 2, class 0x25f) and the Thruster-Pack (item 3, class 0x260).
pub const HELI: i32 = 2;
pub const THRUSTER: i32 = 3;
pub const HELI_CLASS: i16 = 0x25f;
/// Clank's joint list the head glow sits at (`0x312728(clank, 8)`).
pub const GLOW_LIST: usize = 8;

/// `0x2d2140`'s tables: {Ratchet's sequence, the pack's, Clank's} (L00 `0x1a7d60` the Heli-Pack's, `0x1a7d48` the
/// Thruster-Pack's, `0x1a7d78` item 4's).
const HELI_TABLE: [(i16, u8, u8); 5] = [(0, 4, 11), (0, 3, 10), (0, 7, 14), (0x15, 5, 12), (0x12, 8, 15)];
const THRUSTER_TABLE: [(i16, u8, u8); 5] = [(0, 3, 3), (0, 4, 4), (0, 8, 5), (0, 5, 6), (0, 6, 7)];
const TABLE_4: [(i16, u8, u8); 2] = [(0x5e, 3, 16), (0x5e, 4, 17)];

/// `$gp−0x71f8`: a negative SetAnim blend's ticks (`curves[−blend − 1]`), as the back takes them (`(int)` of the
/// float: −3's word is 0x42440000 read as an integer).
const CURVE_TICKS: [i32; 4] = [11, 16, 0x4244_0000, 0];

/// Clank's records (`0x25c9f0 + 0xb0·i`, i = 0..5): `{list, k, d}`.
const RECS: [(u8, f32, f32); 6] = [(0, 0.0, 0.0), (1, 0.0, 0.0), (2, 0.04, 0.3), (3, 0.04, 0.3), (4, 0.04, 0.3), (5, 0.04, 0.3)];

/// Clank's blink (`0x18aea0`, four nodes of mode 1): his lists (`$gp−0x70d0`), rotations (x, y, z, w; `$gp−0x7190`),
/// translations (`$gp−0x7110`, × his scale; the scales `$gp−0x7150` are 1) and the weight by the blink's tick
/// (`0x25de60`).
const BLINK_LISTS: [u8; 4] = [2, 3, 4, 5];
const BLINK_QUATS: [[u32; 4]; 4] = [
    [0x3e18_f712, 0xbd4b_48d4, 0x3ea1_a60d, 0x3f6f_88b9],
    [0x3e25_f84d, 0xbd63_5e74, 0x3ea1_0e02, 0x3f6f_0111],
    [0x3a12_ccf7, 0x3b3c_be62, 0x3ee1_0a13, 0x3f65_efc8],
    [0xba6b_edfa, 0xbb91_2989, 0x3edf_a2f0, 0x3f66_464a],
];
const BLINK_TRANS: [[u32; 3]; 4] = [
    [0x44e2_1948, 0x43d5_7c29, 0x44be_451f],
    [0x44e3_a51f, 0x43cd_63d7, 0x450f_5852],
    [0x44e2_1948, 0xc3d5_7c29, 0x44be_451f],
    [0x44e3_a51f, 0xc3cd_63d7, 0x450f_5852],
];
const BLINK_WEIGHTS: [f32; 22] = [0.0, 0.2, 0.4, 0.6, 0.7, 0.8, 0.9, 1.0, 1.0, 1.0, 1.0, 1.0, 0.9, 0.8, 0.7, 0.6, 0.5, 0.4, 0.3, 0.2, 0.1, 0.0];

/// A request of the hero's for a back moby (0 the pack, 1 Clank), carried out by [`back_pass`] in order.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BackReq {
    /// `0x31b5f0(moby, seq, frame, blend)`.
    Anim { which: usize, seq: u8, frame: i32, blend: i32 },
    /// The animation speed `+0x48`.
    Speed { which: usize, speed: f32 },
}

/// The back slot's block (`0x18b130..0x18b180`) and what the hero keeps of it.
#[derive(Clone, Debug)]
pub struct BackSlot {
    /// `0x18b130` the pack, `0x18b134` Clank, `0x18b13c` his head glow.
    pub pack: Option<usize>,
    pub clank: Option<usize>,
    pub glow: Option<usize>,
    /// The port's: their mobys' creation serials (a level's new world reuses the slots).
    pub serials: [u32; 3],
    /// `0x18b158`: the back item; `0x18b154` = 2: the slot ready.
    pub item: i32,
    pub ready: bool,
    /// The pack's and Clank's sequence (`+0x43`) and run-out flag (`+0x60` & 2) as the hero sees them: the last pass's,
    /// with the hero's own requests since.
    pub seq: [u8; 2],
    pub ended: [bool; 2],
    /// Clank's scale (`+0x2c`).
    pub clank_scale: f32,
    /// The hero's requests of this tick.
    pub reqs: Vec<BackReq>,
    /// `0x18c308`: the next Clank fidget's timer.
    pub fidget_timer: i32,
    /// `0x18afac` (s16): the next blink's timer; `0x18afae`: the blink's tick (0 none); its nodes linked.
    pub blink_timer: i32,
    pub blink: i16,
    pub blink_linked: bool,
    /// `0x18a02e`: the hurt's ticks Clank's glow turns red.
    pub flash: i16,
    /// Clank's records (records 18..23 of `0x25bd90`).
    pub recs: [PoseRec; 6],
    /// `0x18be30`: the head glow's place (Clank's joint list 8).
    pub head_point: V3,
}

impl Default for BackSlot {
    fn default() -> Self {
        BackSlot {
            pack: None,
            clank: None,
            glow: None,
            serials: [0; 3],
            item: 0,
            ready: false,
            seq: [0; 2],
            ended: [false; 2],
            clank_scale: 1.0,
            reqs: Vec::new(),
            fidget_timer: 0,
            blink_timer: 0,
            blink: 0,
            blink_linked: false,
            flash: 0,
            recs: RECS.map(|(l, k, d)| PoseRec::new(l, k, d)),
            head_point: [0.0; 3],
        }
    }
}

impl BackSlot {
    /// The slot's mobys (the hide passes: `0x2d3208`, the body switch's `0x2d3d88`).
    pub fn mobys(&self) -> impl Iterator<Item = usize> + '_ { [self.pack, self.clank, self.glow].into_iter().flatten() }

    /// `0x2adf18(3)`: the back item while the slot is ready, else −1.
    pub fn item_ready(&self) -> i32 { if self.ready { self.item } else { -1 } }

    fn table(&self) -> &'static [(i16, u8, u8)] {
        match self.item_ready() {
            HELI => &HELI_TABLE,
            THRUSTER => &THRUSTER_TABLE,
            4 => &TABLE_4,
            _ => &[],
        }
    }

    fn anim(&mut self, which: usize, seq: u8, frame: i32, blend: i32) {
        self.reqs.push(BackReq::Anim { which, seq, frame, blend });
        self.seq[which] = seq;
        self.ended[which] = false;
    }

    /// The modifier nodes on Clank's moby: the blink's, then his linked records (as Ratchet's, crate::hero::posture).
    pub fn clank_nodes(&self) -> Vec<(u8, JointModifier)> {
        let mut out = Vec::new();
        if self.blink_linked {
            let w = BLINK_WEIGHTS[self.blink.clamp(0, 21) as usize];
            let s = self.clank_scale;
            for (i, &l) in BLINK_LISTS.iter().enumerate() {
                let quat = BLINK_QUATS[i].map(f32::from_bits);
                let trans = BLINK_TRANS[i].map(|t| f32::from_bits(t) * s);
                out.push((l, JointModifier { mode: 1, weight: w, quat, trans, ..JointModifier::compose(0) }));
            }
        }
        for r in self.recs.iter().filter(|r| r.linked) {
            let s = r.node_scale;
            out.push((r.list, JointModifier { quat: crate::hero::quat::euler_quat(r.cur), scale: [s, s, s], trans: r.trans, ..JointModifier::compose(0) }));
        }
        out
    }
}

impl Hero {
    /// `0x2d2140(seq)`: one of the back table's records for Ratchet's sequence `seq` (a draw `0x31b9e0(0, n − 1)`
    /// when there are any): the pack's and Clank's sequences.
    fn back_record(&mut self, seq: u8) -> Option<(u8, u8)> {
        let t = self.back.table();
        let hits: Vec<usize> = (0..t.len()).filter(|&i| t[i].0 == seq as i16).collect();
        if hits.is_empty() { return None; }
        let i = hits[self.rng.rand_range(0, hits.len() as i32 - 1) as usize];
        Some((t[i].1, t[i].2))
    }

    /// `0x2d1f10(seq)`: the pack's sequence `seq` is one of the back table's.
    fn back_table_seq(&self, seq: u8) -> bool { self.back.table().iter().any(|r| r.1 == seq) }

    /// `0x2d1fe0(blend, seq, frame)`, from SetAnim (module docs). `blend` is SetAnim's.
    pub(super) fn back_set_anim(&mut self, blend: f32, seq: u8, frame: i32) {
        let b = &self.back;
        if !b.ready || b.pack.is_none() || b.clank.is_none() { return; }
        let hit = self.back_record(seq);
        match hit {
            Some((p, c)) if seq != 0 => {
                if self.back.seq[0] != p {
                    let ticks = if blend < 0.0 { ((-blend) as usize).checked_sub(1).and_then(|i| CURVE_TICKS.get(i)).copied().unwrap_or(0) } else { blend as i32 };
                    self.back.anim(0, p, frame, ticks);
                    self.back.anim(1, c, frame, ticks);
                }
            }
            _ => {
                if self.group == 0 && seq == 0 && self.back_table_seq(self.back.seq[0]) { return; }
                let blend = if self.prev_state == 8 && self.back.item_ready() == HELI { 0x13 } else { 7 };
                for k in 0..2 {
                    if self.back.seq[k] != 1 { self.back.anim(k, 1, 0, blend); }
                }
            }
        }
    }

    /// `0x2d2278` (from Ratchet's advance `0x2d2888`, no body): with a record for his sequence `seq`, the pack and
    /// Clank play at his speed.
    pub(super) fn back_advance(&mut self, seq: u8) {
        if self.mode != 0 || !self.back.ready { return; }
        if self.back_record(seq).is_none() { return; }
        let speed = self.anim_speed;
        for (k, m) in [self.back.pack, self.back.clank].into_iter().enumerate() {
            if m.is_some() { self.back.reqs.push(BackReq::Speed { which: k, speed }); }
        }
    }

    /// The idle's (`0x2cb650` case 0) first part for the back: the run-out sequences and the turned head back to rest.
    pub(super) fn back_idle_rest(&mut self) {
        if self.back.ready {
            for k in 0..2 {
                let m = if k == 0 { self.back.pack } else { self.back.clank };
                if m.is_some() && self.back.ended[k] && self.back.seq[k] != 1 { self.back.anim(k, 1, 0, 7); }
            }
        }
        let head = self.posture.recs[super::posture::rec::HEAD].cur;
        if 0.610_865_3 < head[2] || 0.436_332_35 < head[1] {
            for k in 0..2 {
                let m = if k == 0 { self.back.pack } else { self.back.clank };
                if m.is_some() && self.back.seq[k] != 1 { self.back.anim(k, 1, 0, 0xc); }
            }
        }
    }

    /// The idle's Clank fidget (`0x2cbbf0`, after Ratchet's fidgets; module docs).
    pub(super) fn back_idle_fidget(&mut self) {
        if self.timer < 0xb || self.back.clank.is_none() || self.back.seq[1] != 1 { return; }
        if !crate::moby::countdown(&mut self.back.fidget_timer) { return; }
        let head = self.posture.recs[super::posture::rec::HEAD].cur;
        if !(head[2] < 0.558_505_4) || !(head[1] < 0.349_065_9) || !self.back.ready { return; }
        self.back.fidget_timer = self.rng.rand_range(0x6e, 0x10e);
        // `0x2d1e88`.
        if self.back.pack.is_none() || self.back.seq[0] != 1 { return; }
        if let Some((p, c)) = self.back_record(0) {
            self.back.anim(0, p, 0, 7);
            self.back.anim(1, c, 0, 7);
        }
    }

    /// `0x2b8dc0` (the run's physics, after its lean; module docs).
    pub(super) fn back_run_sway(&mut self, big_head: bool) {
        if self.back.clank.is_none() { return; }
        let r = &mut self.back.recs[0];
        if big_head { r.scale = self.posture.clank_head; }
        let units = 1024.0 / self.back.clank_scale;
        if self.state != 2 { return; }
        r.set_spring(0.02, f32::from_bits(0x3eb3_3333));
        let res = self.yaw_residual.clamp(-1.4, 1.4);
        r.target[2] = (-res * 1.2).clamp(-1.308_997, 1.308_997);
        r.trans_target[2] = (res.abs() * 0.1 + 0.1).min(0.07) * units;
    }

    /// `0x2ab498`'s head: Clank's head scale on his record 18 with the big-head cheat.
    pub(super) fn back_head_scale(&mut self, big_head: bool) {
        if big_head && self.back.clank.is_some() { self.back.recs[0].scale = self.posture.clank_head; }
    }

    /// `0x2a5840` with Clank on the back: his blink (the glow word is the pass's, [`clank_glow`]).
    pub(super) fn back_blink(&mut self) {
        if self.back.clank.is_none() || self.state == 0x7c { return; }
        let mut t = self.back.blink_timer;
        let fired = crate::moby::countdown(&mut t);
        self.back.blink_timer = t;
        if fired && self.back.seq[1] == 1 {
            self.back.blink_timer = self.rng.rand_range(0x32, 200);
            self.back.blink = 1;
        }
        let b = &mut self.back;
        if b.blink != 0 {
            b.blink += 1;
            if 0x15 < b.blink { b.blink = 0; }
            b.blink_linked = b.blink != 0;
        }
    }

    /// `0x2a5368`'s Clank records (after Ratchet's).
    pub(super) fn back_springs(&mut self) {
        if self.back.clank.is_none() { return; }
        for r in self.back.recs.iter_mut() { r.update(); }
    }
}

/// `0x2a5840`'s glow word for Clank on the back at frame `frame` (module docs).
pub fn clank_glow(frame: i32, hp: i32, flash: i16) -> u32 {
    let ftoi = |x: f32| x as i32;
    let base = if hp == 1 { 0x88 } else { 0x38 };
    let s = ((frame.rem_euclid(0x6e) as f32 / 110.0) * 2.0 * std::f32::consts::PI - std::f32::consts::PI).sin();
    let s24 = s * 24.0;
    let mut i2 = ftoi(s24);
    let mut r = base + i2 + 0xc;
    let mut i3 = ftoi(s * 48.0);
    let mut i4 = ftoi(s24);
    let mut g = i3 + 0xa0;
    let mut b = i4 + 0x4c;
    if flash != 0 {
        let mut k = 1.0;
        if 0x28 < flash { k = (0x2d - flash) as f32 / 5.0; }
        if flash < 0x14 { k = flash as f32 / 20.0; }
        i2 = ftoi((i2 + 0xc) as f32 * k);
        i3 = ftoi((i3 + 0x18) as f32 * k);
        i4 = ftoi((i4 + 0xc) as f32 * k);
        b -= i4;
        r = r - i2 + ftoi(k * 112.0);
        g = g - i3 - ftoi(k * 72.0);
    }
    (b as u32) << 16 | 0x8000_0000 | (g as u32) << 8 | r as u32
}

/// The head glow's word at frame `frame` (`0x2b09e0`, module docs).
fn head_glow(frame: i32) -> u32 {
    let s = ((frame.rem_euclid(0x78) as f32 / 120.0) * 2.0 * std::f32::consts::PI - std::f32::consts::PI).sin();
    let r = ((s * 90.0) as i32 + 0xd7).min(0xff);
    let g = (s * 50.0) as i32 + 0x32;
    let b = (s * 10.0) as i32 + 0x14;
    (b as u32) << 16 | 0x8000_0000 | (g as u32) << 8 | r as u32
}

/// The back slot's part of the item pass (`0x2b21f0`): made (`0x2afc68`), the hero's requests carried out, placed
/// with the head glow (`0x2b09e0`), the slot loop (`0x2b2060`), shown or hidden (`0x2d3408`), and what the hero reads
/// of it kept (module docs).
pub fn back_pass(world: &mut MobyWorld, ctx: &mut Ctx) {
    let away = !ctx.game.clank || ctx.hero.body.clank_away;
    let light = ctx.hero_moby.and_then(|h| world.mobys.get(h)).map(|h| (h.light, h.ambient));
    let b = &mut ctx.hero.back;
    for (k, slot) in [&mut b.pack, &mut b.clank, &mut b.glow].into_iter().enumerate() {
        if slot.is_some_and(|id| world.mobys.get(id).is_none_or(|m| m.state >= 0xfe || m.serial != b.serials[k])) { *slot = None; }
    }
    // A new level's world: the slot is made again.
    if b.pack.is_none() && b.clank.is_none() { b.ready = false; }
    // `0x2afc68`.
    let make = |world: &mut MobyWorld, class: i16| -> Option<usize> {
        if class <= 0 || world.class(class).is_none() { return None; }
        let id = world.create(class);
        let m = &mut world.mobys[id];
        m.draw_dist = 0x20;
        m.drawn = 1;
        if let Some((l, a)) = light { (m.light, m.ambient) = (l, a); }
        if m.shine != 0 { m.shine = 0x10; }
        if away { m.mode |= 0x41; }
        Some(id)
    };
    if b.pack.is_none() {
        b.item = if ctx.game.thruster { THRUSTER } else { HELI };
        let class = ctx.level.items.get(b.item as usize).map_or(0, |r| r.classes[0] as i16);
        if let Some(id) = make(world, class) {
            b.pack = Some(id);
            b.serials[0] = world.mobys[id].serial;
            b.ready = true;
            b.seq[0] = 0;
            b.ended[0] = false;
        }
    }
    if b.clank.is_none() {
        let class = ctx.level.items.get(1).map_or(CLANK_CLASS, |r| r.classes[0] as i16);
        if let Some(id) = make(world, class) {
            b.clank = Some(id);
            b.serials[1] = world.mobys[id].serial;
            b.ready = true;
            b.seq[1] = 0;
            b.ended[1] = false;
            b.clank_scale = world.mobys[id].scale;
        }
    }
    // The hero's requests, in his order.
    for r in std::mem::take(&mut b.reqs) {
        let which = match r { BackReq::Anim { which, .. } | BackReq::Speed { which, .. } => which };
        let Some(id) = (if which == 0 { b.pack } else { b.clank }) else { continue };
        match r {
            BackReq::Anim { seq, frame, blend, .. } => {
                // `0x31b5f0`: the frame within the sequence; a blend under 1 cuts (`0x31b518`).
                let m = &world.mobys[id];
                let n = ctx.host.seq_frames(m, seq).unwrap_or(1) as i32;
                let f = if frame < n { frame } else { n - 1 }.max(0);
                ctx.host.set_anim(m, seq, f as u8, blend.clamp(0, 0xff) as u8);
            }
            BackReq::Speed { speed, .. } => ctx.host.set_anim_speed(&world.mobys[id], speed),
        }
    }
    // `0x2b09e0`, slot 3: not in a scene.
    let attach = ctx.level.items.get(b.item.max(0) as usize).map_or(5, |r| r.attach.max(0) as usize);
    let at = ctx.host.hero_attach(attach);
    let clank_nodes = b.clank_nodes();
    for (k, id) in [b.pack, b.clank].into_iter().enumerate() {
        let Some(id) = id else { continue };
        let Some((rows, pos)) = at else { continue };
        let rows = unit_columns(rows);
        ctx.host.advance(&world.mobys[id]);
        {
            let m = &mut world.mobys[id];
            if let Some((l, a)) = light { (m.light, m.ambient) = (l, a); }
            m.parent = ctx.hero_moby;
            (m.pos, m.rows) = (pos, rows);
            m.mode |= mode::NO_UPDATE | mode::NO_MATRIX | mode::KEEP_ROWS;
            let f = rows[0];
            m.rot = [0.0, -f[2].atan2((f[0] * f[0] + f[1] * f[1]).sqrt()), f[1].atan2(f[0])];
            if k == 1 { m.joint_nodes = clank_nodes.clone(); }
        }
        world.rebuild(id, &*ctx.host);
    }
    // After Clank: his head glow.
    if let Some(cid) = b.clank.filter(|_| at.is_some()) {
        if b.glow.is_none() && world.class(GLOW_CLASS).is_some() {
            let id = world.create(GLOW_CLASS);
            let (hp, hr) = (ctx.hero.pos, ctx.hero.rot);
            let m = &mut world.mobys[id];
            m.glow = 0x8014_32d7;
            (m.pos, m.rot) = (hp, hr);
            m.draw_dist = 0x40;
            m.drawn = 1;
            m.update_dist = 0;
            m.mode = 0;
            m.scale *= 1.3;
            world.rebuild(id, &*ctx.host);
            ctx.hero.back.glow = Some(id);
            ctx.hero.back.serials[2] = world.mobys[id].serial;
        }
        let b = &mut ctx.hero.back;
        if let Some(gid) = b.glow.filter(|&g| world.mobys[g].mode & 1 == 0) {
            if let Some((rows, pos)) = ctx.host.joint_matrix(&world.mobys[cid], GLOW_LIST) {
                b.head_point = pos;
                let frame = world.frame;
                let m = &mut world.mobys[gid];
                (m.pos, m.rows) = (pos, rows);
                m.mode |= mode::KEEP_ROWS;
                if let Some((l, a)) = light { (m.light, m.ambient) = (l, a); }
                m.glow = head_glow(frame);
                m.mode |= mode::GLOW;
                world.rebuild(gid, &*ctx.host);
            }
        }
    }
    let b = &mut ctx.hero.back;
    // `0x2b2060`, slot 3: the pack's sequence 0 run out goes to 1; then its class update (the Heli-Pack's `0x39e7a8`).
    if let Some(id) = b.pack {
        let m = &world.mobys[id];
        let a = ctx.host.anim(m);
        if a.is_some_and(|a| a.flags & 2 != 0 && a.seq_b == 0) && ctx.host.seq_frames(m, 1).is_some_and(|n| n != 0) {
            ctx.host.set_anim(m, 1, 0, 2);
        }
        let m = &world.mobys[id];
        if m.o_class == HELI_CLASS && ctx.hero.state == 8 && ctx.host.anim(m).is_some_and(|a| a.seq_b != 6) {
            ctx.host.set_anim(m, 6, 0, 0xa);
        }
    }
    // `0x2a5840`: Clank's glow word while on the back.
    let b = &mut ctx.hero.back;
    if let Some(id) = b.clank.filter(|_| !away) {
        let m = &mut world.mobys[id];
        m.mode |= mode::GLOW;
        m.glow = clank_glow(world.frame, ctx.hero.hp, b.flash);
    }
    // `0x2d3408`: shown, unless Clank is away from the back or in group 0x1f.
    let hide = away || ctx.hero.group == 0x1f;
    for id in b.mobys() {
        let m = &mut world.mobys[id];
        if hide { m.mode |= 0x41 } else { m.mode &= !0x41 }
    }
    // What the hero reads next tick.
    for (k, id) in [b.pack, b.clank].into_iter().enumerate() {
        let Some(a) = id.and_then(|id| ctx.host.anim(&world.mobys[id])) else { continue };
        b.seq[k] = a.seq_b;
        b.ended[k] = a.flags & 2 != 0;
    }
}

/// `0x31f858`: each column of the 3×3 made unit.
fn unit_columns(r: [V3; 3]) -> [V3; 3] {
    let mut out = r;
    for c in 0..3 {
        let l = (r[0][c] * r[0][c] + r[1][c] * r[1][c] + r[2][c] * r[2][c]).sqrt();
        if 0.0 < l {
            for row in out.iter_mut() { row[c] /= l; }
        }
    }
    out
}
