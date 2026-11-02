//! Level 0x06's own classes (level 6; L06 addresses, the engine functions they call named by their L06 addresses with
//! the L00 ones the port knows them by where they differ). Each class's update is registered in [`update_for`] and
//! its draw callback, if any, in [`draw`]; crate::moby::classes::update_for and crate::fx ask these after their own.

pub mod breakables;
pub mod props;
pub mod traffic;

use crate::hero::physics::V3;
use crate::moby::MobyWorld;

/// This level's class updates.
pub fn update_for(o_class: i16) -> Option<super::UpdateFn> {
    match o_class {
        0x12c8 => Some(breakables::update_12c8),
        0x1183 => Some(breakables::update_1183),
        0x11b1 => Some(breakables::update_11b1),
        0x0fe8 => Some(breakables::update_0fe8),
        0x0fb5 => Some(breakables::update_0fb5),
        0x0fea => Some(breakables::update_0fea),
        0x0912 => Some(breakables::update_0912),
        0x0faa => Some(breakables::update_0faa),
        0x0fd2 | 0x0fd4 | 0x0fe9 => Some(breakables::update_0fd2),
        0x0dda => Some(props::update_0dda),
        0x086b | 0x086d => Some(traffic::update_086b),
        0x089d => Some(props::update_089d),
        0x0948 => Some(props::update_0948),
        0x0fe2 => Some(props::update_0fe2),
        0x02ee => Some(props::update_02ee),
        0x07ef => Some(props::update_07ef),
        0x0c4b => Some(props::update_0c4b),
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
