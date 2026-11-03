//! Level 0x0c's own classes (level 12; L12 addresses, the engine functions they call named by their L12 addresses with
//! the L00 ones the port knows them by where they differ). Each class's update is registered in [`update_for`] and
//! its draw callback, if any, in [`draw`]; crate::moby::classes::update_for and crate::fx ask these after their own.

use crate::hero::physics::V3;
use crate::moby::MobyWorld;

pub mod breakables;
pub mod props;

/// This level's class updates.
pub fn update_for(o_class: i16) -> Option<super::UpdateFn> {
    match o_class {
        0x110c => Some(breakables::update_110c),
        0x1307 => Some(breakables::update_1307),
        0x1114 => Some(breakables::update_1114),
        0x0852 | 0x0884 => Some(breakables::update_0852),
        0x0f69 => Some(props::update_0f69),
        0x087f => Some(props::update_087f),
        0x115f => Some(props::update_115f),
        // `0x38b400`: `jr ra`, as level 4's U293 classes.
        0x1354 => Some(super::level04::misc::update_empty),
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
