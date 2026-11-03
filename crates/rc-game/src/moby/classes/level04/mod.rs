//! Level 0x04's own classes (level 4; L04 addresses, the engine functions they call named by their L04 addresses with
//! the L00 ones the port knows them by where they differ). Each class's update is registered in [`update_for`] and
//! its draw callback, if any, in [`draw`]; crate::moby::classes::update_for and crate::fx ask these after their own.

use crate::hero::physics::V3;
use crate::moby::MobyWorld;

pub mod beast;
pub mod misc;
pub mod platforms;
pub mod rider;

/// This level's class updates.
pub fn update_for(o_class: i16) -> Option<super::UpdateFn> {
    match o_class {
        rider::CLASS => Some(rider::update),
        beast::CLASS => Some(beast::update),
        0x0969 | 0x0e1b | 0x0e1c => Some(misc::update_empty),
        0x1293 => Some(misc::update_1293),
        0x0a99 => Some(misc::update_0a99),
        0x0ab8 => Some(misc::update_0ab8),
        0x0cfd => Some(misc::update_0cfd),
        0x0e8b | 0x0e8f | 0x0e90 => Some(misc::update_0e8b),
        0x0e8d => Some(misc::update_0e8d),
        0x1157 => Some(misc::update_1157),
        0x123f => Some(misc::update_123f),
        0x0d8f => Some(misc::update_0d8f),
        0x0f9c => Some(misc::update_0f9c),
        0x0f61 => Some(misc::update_0f61),
        0x0968 => Some(platforms::update_0968),
        0x09eb => Some(platforms::update_09eb),
        _ => None,
    }
}

/// This level's draw callbacks (crate::fx's `one`): true when the class has one here.
#[allow(clippy::too_many_arguments, unused_variables)]
pub fn draw(w: &mut MobyWorld, host: &dyn crate::moby::MobyHost, game: &mut crate::game::GameState, hero_state: i32, view: &crate::particles::BSphereView, cam: V3, id: usize, first: bool, out: &mut Vec<crate::fx::FxQuad>, frame: &mut Vec<crate::fx::FrameQuad>) -> bool {
    match w.mobys[id].o_class {
        beast::CLASS => { beast::draw(w, id, cam, out); true }
        _ => false,
    }
}
