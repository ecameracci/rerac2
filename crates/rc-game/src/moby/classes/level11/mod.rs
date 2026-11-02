//! Level 0x0b's own classes (level 11; L11 addresses, the engine functions they call named by their L11 addresses with
//! the L00 ones the port knows them by where they differ). Each class's update is registered in [`update_for`] and
//! its draw callback, if any, in [`draw`]; crate::moby::classes::update_for and crate::fx ask these after their own.
//!
//! * [`movers`]: the floors, columns and blocks the Dynamo's targets and the hologram lift move.

pub mod movers;

use crate::hero::physics::V3;
use crate::moby::{Ctx, MobyWorld};

/// The level's globals its classes share.
#[derive(Clone, Debug, Default)]
pub struct Globals {
    /// `$gp+0x17c8`: the wave floor's tick (crate::moby::classes::level11::movers::update_0b6b).
    pub wave: i32,
    /// `$gp+0x1d78`: cleared by the sinking floor's set-up (nothing here reads it).
    pub g1d78: i32,
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
        _ => None,
    }
}

/// This level's draw callbacks (crate::fx's `one`): true when the class has one here.
#[allow(clippy::too_many_arguments, unused_variables)]
pub fn draw(w: &mut MobyWorld, host: &dyn crate::moby::MobyHost, game: &mut crate::game::GameState, hero_state: i32, view: &crate::particles::BSphereView, cam: V3, id: usize, first: bool, out: &mut Vec<crate::fx::FxQuad>, frame: &mut Vec<crate::fx::FrameQuad>) -> bool {
    match w.mobys[id].o_class {
        _ => false,
    }
}
