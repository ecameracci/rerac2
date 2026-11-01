//! The body switch (`0x18c0d4` ≠ 0): the player's control moved onto another moby — Clank's body (1) on Endako —
//! and back to Ratchet (L03 `0x2b8508` / `0x2b8718`, the same code on every level that has a body).
//!
//! **`0x2b8508(body, state, moby)`**: a use of the hand item over (`0x2b5208`), the wrench asked for (`0x18c0d6`), the
//! items' last update (`0x2b7a20`), Ratchet's moby (`0x18aa38`) and his items hidden (`0x2d3d88`), his looping sounds
//! stopped (`0x2acf78`), his posture's records reset (`0x2abc28`); then the body (`0x18c0d4`) and its moby (`0x18aa34`,
//! the hero moby `0x18c0b0` from now on: the hero's animation, write-back and draw are its; mode |= 6, out of the moby
//! loop), the capsule's sizes for the body (`0x2b8f18`: Clank 0.6 high from 0.45 up, 0.3 round; the top and the
//! bottom at once), the animation speed 1 (`0x18aa40`), the state kept (`0x18c312`), Clank's health swapped in
//! (`0x18c2f4` keeps Ratchet's, `0x18c2f0` Clank's — 4 at the level start, `0x2ab1a8` — at most 4), and the state set
//! (`0x2c7288(state, 2)`).
//!
//! **`0x2b8718`**: Clank's health kept and Ratchet's back, the posture reset, the body 0, the body moby back in the
//! moby loop (mode &= ~6), his looping sounds stopped, Ratchet's moby the hero moby again where the player is (the
//! hero's position), the animation speed 1, and idle (`0x2c7288(0, 1)`) unless he is in the scene state 0x61 during a
//! scene (`0x1a8f00` 2 or 6).
//!
//! The moby side (the hero moby, Ratchet's moby and items hidden and shown, the hero's animation moved onto the body's
//! class) is the engine's ([`Change`], `rc_engine`'s play); the hero side is here. While a body plays, the hero tick
//! is `0x2a65d8`: no ledge probe but Clank's, no rail or pole, no item pass (`0x2b21f0`) and no pose layers
//! (`0x2afc38`): [`body_pass`] delivers the body's own hits (Clank's punch) in its place.
//!
//! Not ported: the GUI slot 3's callbacks for Clank (`0x2f8c00(0x23, …)`, only with △ held at the switch, and
//! Ratchet's back at the return), the item records `0x18b700` / `0x18b850` (`0x3253d0`), Ratchet's moby's `+0x9c`, the
//! words `0x18aa44` / `0x18aa60` / `0x18aa68` (`0x2d33d8`), the body's light (`0x31bcac`), Clank's mobys
//! `0x18b680` / `0x18b684` deleted at the return (none made: G-HERO-004), the use's idle sequence of `0x2b5208`.

use super::posture::Posture;
use super::Hero;
use crate::moby::{Ctx, MobyWorld};

/// `0x2b8f18` for Clank's body: the capsule's top `0x18a078` (0x3f199999), bottom `0x18a07c` (0x3ee66667) and radius
/// `0x18a080` (0x3e99999a).
pub const CLANK_CAPSULE: (f32, f32, f32) = (f32::from_bits(0x3f19_9999), f32::from_bits(0x3ee6_6667), f32::from_bits(0x3e99_999a));

/// A switch for the engine to carry out on the mobys and the animation (in order, before the hero's tick).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Change {
    /// `0x2b8508`: the hero moby becomes this moby.
    To(usize),
    /// `0x2b8718`: Ratchet's moby is the hero moby again.
    Back,
}

/// The body block.
#[derive(Clone, Debug)]
pub struct BodyBlock {
    /// `0x18aa34`: the body's moby (the hero moby `0x18c0b0` while it plays).
    pub moby: Option<usize>,
    /// `0x18c2f0`: Clank's health (4 at the level start); `0x18c2f4`: Ratchet's while a body plays.
    pub clank_hp: i32,
    pub ratchet_hp: i16,
    /// `0x18c312`: the state the switch started the body in (the checkpoint keeps it).
    pub state: i16,
    /// `0x18c31c`: Clank away from Ratchet's back (set at the level start without him, `0x2ab1a8`; Endako's rescue
    /// sets it until Clank is back): no packs, no glide.
    pub clank_away: bool,
    /// The switches of this tick for the engine.
    pub changes: Vec<Change>,
}

impl Default for BodyBlock {
    fn default() -> Self { BodyBlock { moby: None, clank_hp: 4, ratchet_hp: 0, state: 0, clank_away: false, changes: Vec::new() } }
}

/// `0x2abc28`: every posture record unlinked, its angles and targets cleared, its scale 1. The port's: the blink's
/// nodes and the pose layers go too (they belong to Ratchet's moby, which the body's draw does not take).
fn reset_records(p: &mut Posture) {
    for r in p.recs.iter_mut() {
        r.linked = false;
        r.cur = [0.0; 3];
        r.target = [0.0; 3];
        r.trans_target = [0.0; 3];
        r.scale = 1.0;
    }
    p.blink_linked = false;
}

impl Hero {
    /// `0x2b8508(body, state, moby)`: the player becomes the body `body` on moby `moby`, in state `state` (module
    /// docs).
    pub fn switch_body(&mut self, body: u8, state: i32, moby: usize) {
        // `0x2b5208`: a use of the hand item over; `0x18c0d6`: the wrench asked for.
        self.items.using = false;
        self.items.keep = false;
        self.items.wrench_req = true;
        // `0x2acf78`, `0x2abc28`.
        self.loops_off = true;
        reset_records(&mut self.posture);
        self.layers.hold = [None, None];
        self.layers.face = None;
        self.mode = body;
        self.body.moby = Some(moby);
        self.body.changes.push(Change::To(moby));
        // `0x2b8f18`, then the top and the bottom at their targets.
        let (top, bottom, r) = if body == 1 { CLANK_CAPSULE } else { (0.8, 0.7, 0.45) };
        (self.cap_top_target, self.cap_bottom_target, self.cap_radius_target) = (top, bottom, r);
        (self.cap_top, self.cap_bottom) = (top, bottom);
        self.anim_speed = 1.0;
        self.body.state = state as i16;
        if body == 1 {
            self.body.ratchet_hp = self.hp as i16;
            self.hp = self.body.clank_hp.min(4);
        }
        self.request_state(state, true);
    }

    /// `0x2b8718`: back to Ratchet (module docs). `in_scene`: the game in a scene (`0x1a8f00` 2 or 6).
    pub fn body_back(&mut self, in_scene: bool) {
        if self.mode == 1 {
            self.body.clank_hp = self.hp;
            self.hp = self.body.ratchet_hp as i32;
        }
        reset_records(&mut self.posture);
        self.mode = 0;
        self.body.moby = None;
        self.body.changes.push(Change::Back);
        self.loops_off = true;
        self.anim_speed = 1.0;
        if self.state != 0x61 || !in_scene { self.request_state(0, true); }
    }

    /// `0x2c8e30`: the idle state of the body playing (Ratchet 0, Clank 0x3f; the others none).
    pub fn idle_state(&self) -> Option<i32> {
        match self.mode {
            0 => Some(0),
            1 => Some(0x3f),
            _ => None,
        }
    }
}

/// After the hero's tick while a body plays (`0x2a65d8` has no item pass): the hits the body's states dealt this tick
/// (Clank's punch, `0x2ba8e0` case 0x4d), as the item pass delivers Ratchet's ([`crate::hero::items::item_pass`]).
pub fn body_pass(world: &mut MobyWorld, ctx: &mut Ctx) {
    for h in std::mem::take(&mut ctx.hero.hits_out) {
        let t = crate::moby::hits::HitTemplate::new(h.damage, ctx.hero_moby, h.flags, [h.dir[0], h.dir[1], h.dir[2], 0.0]);
        let mut c = h.centre;
        if let (Some([a, b]), Some(hm)) = (h.joints, ctx.hero_moby) {
            if let (Some(pa), Some(pb)) = (ctx.host.joint(&world.mobys[hm], a), ctx.host.joint(&world.mobys[hm], b)) {
                c = [(pa[0] + pb[0]) * 0.5, (pa[1] + pb[1]) * 0.5, (pa[2] + pb[2]) * 0.5];
            }
        }
        world.sphere_hit(ctx.scene, c, h.r, 0, ctx.hero_moby, Some(&t));
    }
}
