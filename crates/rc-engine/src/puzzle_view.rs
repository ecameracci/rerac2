//! The world hidden while the game is in the Electrolyzer's mode 7 (rc_game::moby::classes::electrolyzer): the mode's
//! draw (`0x2ea328`) runs with the draw flags 0xf0, so no sky (bit 1), tfrags (2), ties (4) or shrubs (8) — only the
//! mobys, the effects and the HUD — over the background `0x2f5120(0, 0, 0)` set black (`0x3609b0` and each frame of
//! `0x3784e8`; `0x360a50` puts the level's back). Here the world's tfrag, tie, shrub and billboard entities go to a
//! layer no camera draws, and the sky camera draws nothing (no layer at all: on the hidden layer it drew the hidden
//! world itself, the skyline seen at the top left of Endako's Hacker screen) and clears to black; leaving the mode
//! puts them back.

use bevy::camera::visibility::RenderLayers;
use bevy::camera::ClearColorConfig;
use bevy::prelude::*;

/// A layer no camera draws.
pub const HIDDEN_LAYER: usize = 27;

/// What the mode moved, to put back.
#[derive(Default)]
pub struct Hidden {
    on: bool,
    entities: Vec<Entity>,
    sky: Option<(ClearColorConfig, RenderLayers)>,
}

pub fn apply(
    mut commands: Commands,
    play: Option<Res<crate::play::Play>>,
    world: Query<Entity, (With<crate::pre_shadow::WorldDrawOrder>, Without<RenderLayers>)>,
    mut sky: Query<(&mut Camera, &mut RenderLayers), With<crate::sky_render::SkyCamera>>,
    mut state: Local<Hidden>,
) {
    let on = play.as_ref().is_some_and(|p| p.game.scenes.mode == rc_game::moby::classes::electrolyzer::MODE);
    if on {
        // Entities spawned while it lasts go too.
        for e in &world {
            commands.entity(e).insert(RenderLayers::layer(HIDDEN_LAYER));
            state.entities.push(e);
        }
        if !state.on {
            for (mut cam, mut layers) in &mut sky {
                state.sky = Some((cam.clear_color.clone(), layers.clone()));
                cam.clear_color = ClearColorConfig::Custom(Color::BLACK);
                *layers = RenderLayers::none();
            }
        }
    } else if state.on {
        for e in state.entities.drain(..) {
            if let Ok(mut c) = commands.get_entity(e) { c.remove::<RenderLayers>(); }
        }
        if let Some((clear, l)) = state.sky.take() {
            for (mut cam, mut layers) in &mut sky {
                cam.clear_color = clear.clone();
                *layers = l.clone();
            }
        }
    }
    state.on = on;
}
