//! Level 0x0c's own classes (level 12; L12 addresses, the engine functions they call named by their L12 addresses with
//! the L00 ones the port knows them by where they differ). Each class's update is registered in [`update_for`] and
//! its draw callback, if any, in [`draw`]; crate::moby::classes::update_for and crate::fx ask these after their own.

use crate::hero::physics::V3;
use crate::moby::MobyWorld;

pub mod breakables;
pub mod field;
pub mod head;
pub mod lifts;
pub mod props;
pub mod tractor;

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
        0x1351 => Some(props::update_1351),
        0x0fc4 | 0x0fc5 => Some(tractor::update_0fc4),
        0x0fbc => Some(lifts::update_0fbc),
        0x0fd8 => Some(lifts::update_0fd8),
        0x0fd9 => Some(lifts::update_0fd9),
        0x0fc6 | 0x0fc7 | 0x0fc8 => Some(tractor::update_0fc6),
        0x0f6a => Some(tractor::update_0f6a),
        0x0fb8 => Some(tractor::update_0fb8),
        0x0fb9 => Some(tractor::update_0fb9),
        0x0f00 => Some(field::update_0f00),
        0x086a => Some(head::update_086a),
        0x117b => Some(props::update_117b),
        0x0ee8 => Some(props::update_0ee8),
        0x0424 => Some(props::update_0424),
        // `0x38b400`: `jr ra`, as level 4's U293 classes.
        0x1354 => Some(super::level04::misc::update_empty),
        _ => None,
    }
}

/// This level's draw callbacks (crate::fx's `one`): true when the class has one here.
#[allow(clippy::too_many_arguments, unused_variables)]
pub fn draw(w: &mut MobyWorld, host: &dyn crate::moby::MobyHost, game: &mut crate::game::GameState, hero_state: i32, view: &crate::particles::BSphereView, cam: V3, id: usize, first: bool, out: &mut Vec<crate::fx::FxQuad>, frame: &mut Vec<crate::fx::FrameQuad>) -> bool {
    match w.mobys[id].o_class {
        0x0424 => {
            props::draw_0424(w, id, game.level, cam, out);
            true
        }
        _ => false,
    }
}
