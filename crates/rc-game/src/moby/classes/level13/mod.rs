//! Level 0x0d's own classes (level 13; L13 addresses, the engine functions they call named by their L13 addresses with
//! the L00 ones the port knows them by where they differ). Each class's update is registered in [`update_for`] and
//! its draw callback, if any, in [`draw`]; crate::moby::classes::update_for and crate::fx ask these after their own.

pub mod breakables;
pub mod movers;

use crate::hero::physics::V3;
use crate::moby::MobyWorld;

/// This level's class updates.
pub fn update_for(o_class: i16) -> Option<super::UpdateFn> {
    match o_class {
        0x130b => Some(breakables::update_130b),
        0x130a => Some(breakables::update_130a),
        0x130f => Some(breakables::update_130f),
        0x130d => Some(breakables::update_130d),
        0x0d97 | 0x0d95 => Some(movers::update_0d97),
        0x0d65 => Some(movers::update_0d65),
        0x0cd8 => Some(movers::update_0cd8),
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
