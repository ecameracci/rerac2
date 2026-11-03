//! Level 0x0d's own classes (level 13; L13 addresses, the engine functions they call named by their L13 addresses with
//! the L00 ones the port knows them by where they differ). Each class's update is registered in [`update_for`] and
//! its draw callback, if any, in [`draw`]; crate::moby::classes::update_for and crate::fx ask these after their own.

pub mod breakables;
pub mod movers;
pub mod powerline;
pub mod props;
pub mod targets;

use crate::hero::physics::V3;
use crate::moby::MobyWorld;

/// This level's class updates.
pub fn update_for(o_class: i16) -> Option<super::UpdateFn> {
    match o_class {
        0x0ee7 => Some(powerline::update_0ee7),
        0x1150 => Some(powerline::update_1150),
        0x130b => Some(breakables::update_130b),
        0x130a => Some(breakables::update_130a),
        0x130f => Some(breakables::update_130f),
        0x130d => Some(breakables::update_130d),
        0x0d97 | 0x0d95 => Some(movers::update_0d97),
        0x0d65 => Some(movers::update_0d65),
        0x0cd8 => Some(movers::update_0cd8),
        0x0b5c | 0x0b67 | 0x0b68 | 0x0b5b | 0x0b61 => Some(props::update_0b5c),
        0x0dd1 | 0x0da3 | 0x0da1 => Some(props::update_0dd1),
        0x0ef2 | 0x113c => Some(props::update_0ef2),
        0x12ed => Some(props::update_12ed),
        0x0f20 => Some(props::update_0f20),
        0x040f => Some(props::update_040f),
        0x0efa => Some(props::update_0efa),
        0x0c86 => Some(targets::update_0c86),
        0x0c87 => Some(targets::update_0c87),
        0x0fec => Some(props::update_0fec),
        0x11b0 => Some(props::update_11b0),
        0x1225 => Some(props::update_1225),
        _ => None,
    }
}

/// This level's draw callbacks (crate::fx's `one`): true when the class has one here.
#[allow(clippy::too_many_arguments, unused_variables)]
pub fn draw(w: &mut MobyWorld, host: &dyn crate::moby::MobyHost, game: &mut crate::game::GameState, hero_state: i32, view: &crate::particles::BSphereView, cam: V3, id: usize, first: bool, out: &mut Vec<crate::fx::FxQuad>, frame: &mut Vec<crate::fx::FrameQuad>) -> bool {
    match w.mobys[id].o_class {
        0x0ee7 => powerline::draw_0ee7(w, id, cam, out),
        _ => return false,
    }
    true
}
