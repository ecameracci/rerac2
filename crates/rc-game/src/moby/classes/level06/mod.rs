//! Level 0x06's own classes (level 6; L06 addresses, the engine functions they call named by their L06 addresses with
//! the L00 ones the port knows them by where they differ). Each class's update is registered in [`update_for`] and
//! its draw callback, if any, in [`draw`]; crate::moby::classes::update_for and crate::fx ask these after their own.
//!
//! Not ported (level 6's other classes):
//! * Waiting on a missing system (docs/plan/gaps.md): 0x0fc1 (`0x458888`) and 0x0fc0 (`0x458718`), effect-quad
//!   trails (G-REN-004); the vent 0x0feb (`0x45da00`, particle type 0x53 `0x37fae8`) and 0x0fb2 / 0x0fd6 / 0x0fce /
//!   0x0fcf / 0x0fe4 (`0x4581c8`) (G-PRT-001); 0x0eb1 (`0x451ad0`, items, G-WPN-002); 0x0bc7 (`0x422070`) and 0x0fe6
//!   (`0x45bbc8`), the line test (G-CLS-005); the core 0x0fc2 (`0x458a00`), its beams (class 0x13ba, G-REN-019).
//! * The challenge host 0x0ccf (`0x4318a0`): its talk (speech 0x1320 / 0x1321, the scene `0x3b3068`, the help
//!   messages 0x4a / 0x4b / 0x4c, the bolt counts `0x295950` / `0x295954`; the dialogue system, G-UI-010).
//! * Ratchet's: the ferry 0x0330 (`0x3e5460`, L11 `0x3b7340`: a ride on paths with its own controls), 0x0947
//!   (`0x404c60`: a swimmer that grabs Ratchet in the water, his state 0x12, and throws him out, his state 0x6f
//!   `0x2f96e8`), 0x0b57 (`0x41b200`, Ratchet's state 0x37 and the level's scene functions `0x479xxx`).
//! * The draws: 0x093e (`0x404680`, L11 `0x3d19f8`: a screen layer drawn by `0x403ac0`), 0x08a0 (`0x401b40`:
//!   nine shuffled symbols in a column, its draw `0x401f00` reading the table in its moby `+0x68`[3]'s pvars,
//!   `+0x2c00` / `+0x12de0`).
//! * The hints 0x0cdd (`0x4321e8`: help 0x9df / 0x9e0 / 0x9e1 / 0x9e2 / 0x9db / 0x9b5 by cuboids; it reads Ratchet's
//!   `0x18c2d4`, the byte `0x1a7b27` and the table `0x21ecc4`, none mapped), 0x08a1 (`0x402450`: the table
//!   `0x21ecc4` and the path points' `w`), 0x1240 (`0x465f18`: the level's table `0x1b21c4`, `0x47c358`).

pub mod arc_shot;
pub mod brawler;
pub mod breakables;
pub mod gate;
pub mod glowbug;
pub mod lifts;
pub mod pole;
pub mod props;
pub mod traffic;

use crate::hero::physics::V3;
use crate::moby::MobyWorld;

/// The level's globals its classes share.
#[derive(Clone, Debug, Default)]
pub struct Globals {
    /// `0x1b3760`: the brawlers' arc shots' six slots taken (crate::moby::classes::level06::arc_shot).
    pub shot_slots: [u8; 6],
    /// `0x1b3790` / `0x1b37a0`: the six arcs' timers and alphas (one set for every slot).
    pub arc_timers: [i16; 6],
    pub arc_alpha: [i16; 6],
    /// `0x25ffc0`: each slot's six arcs of four points.
    pub arcs: [[[V3; 4]; 6]; 6],
}

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
        0x0e8e => Some(props::update_0e8e),
        0x08c9 => Some(lifts::update_08c9),
        0x0849 => Some(lifts::update_0849),
        glowbug::CLASS => Some(glowbug::update_0db0),
        brawler::CLASS => Some(brawler::update_0bba),
        brawler::SWITCH_CLASS => Some(brawler::update_0fdc),
        arc_shot::CLASS => Some(arc_shot::update_0fe5),
        pole::CLASS => Some(pole::update_12c4),
        gate::CLASS => Some(gate::update_031a),
        0x0307 => Some(pole::update_0307),
        _ => None,
    }
}

/// This level's draw callbacks (crate::fx's `one`): true when the class has one here.
#[allow(clippy::too_many_arguments, unused_variables)]
pub fn draw(w: &mut MobyWorld, host: &dyn crate::moby::MobyHost, game: &mut crate::game::GameState, hero_state: i32, view: &crate::particles::BSphereView, cam: V3, id: usize, first: bool, out: &mut Vec<crate::fx::FxQuad>, frame: &mut Vec<crate::fx::FrameQuad>) -> bool {
    match w.mobys[id].o_class {
        glowbug::CLASS => { glowbug::draw(w, id, host, cam, out); true }
        arc_shot::CLASS => { arc_shot::draw(w, id, game, cam, out); true }
        _ => false,
    }
}
