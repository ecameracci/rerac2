//! Level 0x04's own classes (level 4; L04 addresses, the engine functions they call named by their L04 addresses with
//! the L00 ones the port knows them by where they differ). Each class's update is registered in [`update_for`] and
//! its draw callback, if any, in [`draw`]; crate::moby::classes::update_for and crate::fx ask these after their own.
//!
//! Not ported yet (level 4's own list; docs/plan/gaps.md has the systems):
//! * **0x0fb6** (`0x44dd48`, 118 placed): a critter riding floor mobys with its own collision sphere (G-CLS-005).
//! * **0x0b2d** (`0x405aa0`), **0x0474** / **0x0cef** (`0x3c9760`), **0x0bbc** (`0x41b020`): the level chunks (G-LVL-007).
//! * **0x0960** (`0x3e9320`): its particles (G-PRT-001). **0x0d61** (`0x43a100`): the reward items (G-WPN-002).
//! * **0x0c38** (`0x424978`), **0x0d3e** (`0x4362f0`), **0x0da8** (`0x43b090`): NPCs on the talk prompt (G-UI-010).
//! * **0x0c26** (`0x423a38`): the level's help-text controller (help texts 0x9b5..0x9de and 0xa3c on its cuboids,
//!   over many of Ratchet's and the game's words not yet named).
//! * **0x0a58** (`0x3ef548`; level 6's 0x0a50 in the same unit): a spawner that lobs its mobys out on arcs.
//! * **0x02f8** (`0x3c4f80`), **0x0b37** (`0x4084c0`), **0x0b54** (`0x409440`): draw-callback effects (strips and
//!   particle fountains from the level's `$gp` tables).
//! * **0x0329** (`0x3c83c8`): it only sets level words (`0x1b2e70..`, `$gp+0x518` scrolled) that nothing ported reads.

use crate::hero::physics::V3;
use crate::moby::MobyWorld;

pub mod ambush;
pub mod beast;
pub mod misc;
pub mod platforms;
pub mod rider;
pub mod tie_wave;

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
        0x0f71 => Some(misc::update_0f71),
        0x0c17 => Some(misc::update_0c17),
        0x09f0 => Some(ambush::update_09f0),
        0x0ce6 => Some(ambush::update_0ce6),
        0x1362 => Some(ambush::update_1362),
        0x1137 => Some(tie_wave::update),
        0x0f61 => Some(misc::update_0f61),
        0x0968 => Some(platforms::update_0968),
        0x09eb => Some(platforms::update_09eb),
        0x09ea => Some(platforms::update_09ea),
        0x09bd => Some(platforms::update_09bd),
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
