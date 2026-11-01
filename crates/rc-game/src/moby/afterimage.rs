//! The after-images (L00 `0x323d10` / `0x323d38` / `0x323e58` / `0x324140`): ghosts of a moby a few ticks behind it.
//! A record (0x150 bytes in the game: `+0x00` / `+0x80` a ring of the owner's last 8 places and angles, `+0x100`
//! / `+0x110` / `+0x120` / `+0x130` up to 4 ghosts' alphas, delays, mobys and pulls, `+0x140` the ring's next slot,
//! `+0x142` its fill (at most 8), `+0x144` the owner, `+0x148` the ghosts, `+0x14c` on) is the owner's user's to keep.
//!
//! * [`Record::start`] (`0x323d10(owner, rec)`): on, for `owner`, the ring empty, no ghosts — unless it is on already.
//! * [`Record::add`] (`0x323d38(pull, rec, alpha, delay)`): a ghost (at most 4, `delay` at most 8), a moby of the
//!   owner's class (`0x311320`) that takes its piece word `+0x7c`, draw distance `+0x32`, light word `+0x38`, scale
//!   `+0x2c` and glow word `+0x78`; drawn (`+0x31` 1), no update and blended (`+0x34` = 0x80a), out of the update
//!   distance (`+0x30` 0), its alpha `+0x23` = `alpha`.
//! * [`Record::tick`] (`0x323e58(rec, fade)`): the owner gone (state 0xfe / 0xfd) frees the record; else its place
//!   and angles into the ring, and each ghost whose delay the ring's fill reaches is put at the ring's entry `delay`
//!   back (1: this tick's), pushed `pull` times its distance away from the owner, and rebuilt. `fade` ≠ 0 takes that
//!   much off every ghost's alpha and frees the record once all are 0.
//! * [`Record::free`] (`0x324140(rec)`): the ghosts deleted, off.
//!
//! The owners: the Chopper's blades (crate::moby::classes::chopper). Not ported: the ghost's copy of the owner's
//! animation keys (`+0x40..+0x47`, `+0x58` / `+0x5c`, `0x3116e0`) and its rows when the owner is drawn as a break
//! piece (`+0x7c` & 0x8000); the class-0 ghost's `+0x62`.

use super::{MobyHost, MobyWorld};
use crate::hero::physics::{add, scale, sub, V3};

/// One ghost (`+0x100 + 4·k` its alpha, `+0x110` its delay, `+0x120` its moby, `+0x130` its pull).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Ghost {
    pub id: usize,
    pub delay: usize,
    pub pull: f32,
}

/// An after-image record (module docs).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Record {
    pub on: bool,
    pub owner: usize,
    /// `+0x140` / `+0x142`: the ring's next slot and its fill.
    pub head: usize,
    pub count: usize,
    /// `+0x00` / `+0x80`: the owner's places and angles.
    pub ring: [(V3, V3); 8],
    pub ghosts: Vec<Ghost>,
}

/// `0x80a`: the ghost's mode (no update, blended).
const GHOST_MODE: u16 = 0x80a;

impl Record {
    /// `0x323d10(owner, rec)`.
    pub fn start(&mut self, owner: usize) {
        if self.on { return; }
        *self = Record { on: true, owner, ..Default::default() };
    }

    /// `0x323d38(pull, rec, alpha, delay)`.
    pub fn add(&mut self, w: &mut MobyWorld, pull: f32, alpha: u8, delay: usize) {
        if self.ghosts.len() == 4 || 8 < delay { return; }
        let Some(o) = w.mobys.get(self.owner) else { return };
        let (class, piece, bangles, draw_dist, light, ambient, scale_, glow) = (o.o_class, o.piece, o.bangles, o.draw_dist, o.light, o.ambient, o.scale, o.glow);
        if w.class(class).is_none() { return; }
        let id = w.create(class);
        let g = &mut w.mobys[id];
        (g.piece, g.bangles, g.draw_dist, g.light, g.ambient, g.scale, g.glow) = (piece, bangles, draw_dist, light, ambient, scale_, glow);
        g.drawn = 1;
        g.mode = GHOST_MODE;
        g.update_dist = 0;
        g.alpha = alpha;
        // The port's: no update of its own, so the engine follows the places written here.
        g.drive = true;
        self.ghosts.push(Ghost { id, delay, pull });
    }

    /// `0x323e58(rec, fade)`.
    pub fn tick(&mut self, w: &mut MobyWorld, host: &dyn MobyHost, fade: u8) {
        if !self.on { return; }
        let Some((pos, rot)) = w.mobys.get(self.owner).filter(|o| o.state != 0xfe && o.state != 0xfd).map(|o| (o.pos, o.rot)) else {
            self.free(w);
            return;
        };
        self.ring[self.head] = (pos, rot);
        self.head = (self.head + 1) % 8;
        self.count = (self.count + 1).min(8);
        for g in &self.ghosts {
            if self.count < g.delay { continue; }
            let (mut p, r) = self.ring[(self.head + 8 - g.delay) % 8];
            if g.pull != 0.0 { p = add(p, scale(sub(p, pos), g.pull)); }
            let Some(m) = w.mobys.get_mut(g.id) else { continue };
            (m.pos, m.rot) = (p, r);
            w.rebuild(g.id, host);
        }
        if fade != 0 {
            let mut all = true;
            for g in &self.ghosts {
                let Some(m) = w.mobys.get_mut(g.id) else { continue };
                m.alpha -= m.alpha.min(fade);
                if m.alpha != 0 { all = false; }
            }
            if all { self.free(w); }
        }
    }

    /// `0x324140(rec)`.
    pub fn free(&mut self, w: &mut MobyWorld) {
        if !self.on { return; }
        for g in self.ghosts.drain(..) {
            if w.mobys.get(g.id).is_some_and(|m| m.state != 0xfe) { w.delete(g.id); }
        }
        self.on = false;
    }

    /// Every ghost's scale times `k` (the Chopper's split, `0x41e080`).
    pub fn scale_ghosts(&self, w: &mut MobyWorld, k: f32) {
        for g in &self.ghosts {
            if let Some(m) = w.mobys.get_mut(g.id) { m.scale *= k; }
        }
    }
}

