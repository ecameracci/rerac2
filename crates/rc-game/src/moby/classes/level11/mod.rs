//! Level 0x0b's own classes (level 11; L11 addresses, the engine functions they call named by their L11 addresses with
//! the L00 ones the port knows them by where they differ). Each class's update is registered in [`update_for`] and
//! its draw callback, if any, in [`draw`]; crate::moby::classes::update_for and crate::fx ask these after their own.
//!
//! * [`movers`]: the floors, columns and blocks the Dynamo's targets and the hologram lift move.
//! * [`beams`]: the electric fence 0x0ee0.
//! * [`critters`]: the birds 0x1105.
//! * [`stalks`]: the sparked stalks 0x10ea / 0x118c.
//! * [`props`]: the small objects (the Hacker's gate, the help texts, the sparks, ...).

pub mod beams;
pub mod critters;
pub mod movers;
pub mod props;
pub mod stalks;

use crate::hero::physics::V3;
use crate::moby::{Ctx, MobyWorld};

/// The level's globals its classes share.
#[derive(Clone, Debug, Default)]
pub struct Globals {
    /// `$gp+0x17c8`: the wave floor's tick (crate::moby::classes::level11::movers::update_0b6b).
    pub wave: i32,
    /// `$gp+0x1d78`: cleared by the sinking floor's set-up (nothing here reads it).
    pub g1d78: i32,
    /// `$gp−0x334` (`0x1aecbc`): the birds 0x1105 killed (the skill point 0x14, props::update_12c7).
    pub bird_kills: i32,
    /// `0x1a904c` (read by props::update_12c7: its enemy count runs while it is set; no writer in the port),
    /// `0x1a9050` and `$gp−0x338` (written by it, nothing here reads them).
    pub g904c: i32,
    pub g9050: i32,
    pub g338: i32,
    /// The electric fence's state (`$gp+0x3600..`, beams::Fence).
    pub fence: beams::Fence,
    /// `$gp+0x1764`: the frame the light shafts' draw was last asked for (props::update_0e49).
    pub shafts_frame: i32,
}

/// The level's mission `mission` done (`0x19b2e8 + level·16`, bit 7).
pub fn mission_done(ctx: &Ctx, mission: i32) -> bool {
    let g = &*ctx.game;
    usize::try_from(mission).ok().and_then(|k| g.missions.get(g.level.max(0) as usize * 0x10 + k)).is_some_and(|b| b & 0x80 != 0)
}

/// This level's class updates.
pub fn update_for(o_class: i16) -> Option<super::UpdateFn> {
    match o_class {
        0x0b6b => Some(movers::update_0b6b),
        0x0d32 => Some(movers::update_0d32),
        0x0c7b => Some(movers::update_0c7b),
        0x0b6f => Some(movers::update_0b6f),
        0x0c7a => Some(movers::update_0c7a),
        0x0c78 | 0x0b71 | 0x0b72 | 0x0b73 | 0x0b74 => Some(movers::update_slider),
        0x0a1f => Some(movers::update_0a1f),
        0x0c7c => Some(movers::update_0c7c),
        0x117a => Some(props::update_117a),
        0x117e => Some(props::update_117e),
        0x123a => Some(props::update_123a),
        0x0c2d => Some(props::update_0c2d),
        0x0dfd => Some(props::update_0dfd),
        0x1105 => Some(critters::update_1105),
        0x12c7 => Some(props::update_12c7),
        0x08e2 => Some(props::update_08e2),
        0x10ea | 0x118c => Some(stalks::update),
        0x0ee0 => Some(beams::update),
        0x0e49 => Some(props::update_0e49),
        _ => None,
    }
}

/// This level's draw callbacks (crate::fx's `one`): true when the class has one here.
#[allow(clippy::too_many_arguments, unused_variables)]
pub fn draw(w: &mut MobyWorld, host: &dyn crate::moby::MobyHost, game: &mut crate::game::GameState, hero_state: i32, view: &crate::particles::BSphereView, cam: V3, id: usize, first: bool, out: &mut Vec<crate::fx::FxQuad>, frame: &mut Vec<crate::fx::FrameQuad>) -> bool {
    match w.mobys[id].o_class {
        0x0dfd => props::draw_0dfd(w, id, &mut game.rng, cam, out),
        0x10ea | 0x118c => stalks::draw(w, id, &mut game.rng, cam, out),
        0x0ee0 => beams::draw(w, id, cam, out),
        0x0e49 => props::draw_0e49(w, id, game.frame, view, out),
        _ => return false,
    }
    true
}
