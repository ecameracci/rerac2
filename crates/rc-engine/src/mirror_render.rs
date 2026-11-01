//! The mirror cheat (byte 4 of `0x1a7a48`, rc_game::cheats) on the screen: the camera update (`0x2d9f38`) rebuilds the
//! camera matrix's second row from the other two the other way round (the row negated), so the world, the scenes
//! and the ship's views are drawn as their mirror image, while the 2D layer (the HUD, the pause, the movies) is not.
//! The port's cameras keep their rows (a Bevy transform has no reflection), so the world's frame is mirrored left to
//! right by a full-screen pass after the 3D passes, before the AA blit (crate::aa_blit) and the UI pass that composites
//! the HUD ([`MirrorFlip`], `assets/shaders/mirror.wgsl`). The pad's swap is the game's (rc_game::pad, crate::play).
//!
//! The pass mirrors the whole main target about its centre: the game's frame is centred in it, so the picture is
//! mirrored about its own centre.
//!
//! Not ported: what reads the camera's rows besides the picture: the sound's listener (left and right not swapped),
//! the HUD's markers projected through the camera (drawn where the unmirrored world has their mobys), and the
//! first-person weapon's offset (`0x2d5ce0`).

use crate::fly_cam::FlyCam;
use bevy::core_pipeline::fullscreen_material::FullscreenMaterial;
use bevy::ecs::schedule::ScheduleConfigs;
use bevy::ecs::system::BoxedSystem;
use bevy::prelude::*;
use bevy::render::extract_component::ExtractComponent;
use bevy::render::render_resource::ShaderType;
use bevy::shader::ShaderRef;

/// The mirror pass on the main camera (present only while the cheat is on in a running game).
#[derive(Component, ExtractComponent, Clone, Copy, Default, PartialEq, ShaderType)]
pub struct MirrorFlip {
    pub on: u32,
}

impl FullscreenMaterial for MirrorFlip {
    fn fragment_shader() -> ShaderRef { "shaders/mirror.wgsl".into() }

    /// After the 3D passes, before the AA blit and the UI pass (the HUD and the menus stay as they are).
    fn schedule_configs(system: ScheduleConfigs<BoxedSystem>) -> ScheduleConfigs<BoxedSystem> {
        system
            .after(bevy::core_pipeline::Core3dSystems::PostProcess)
            .before(crate::gs_post::pass::<crate::aa_blit::BlitAcross>)
            .before(bevy::ui_render::ui_pass)
    }
}

pub struct MirrorRenderPlugin;

impl Plugin for MirrorRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(crate::gs_post::GsPostPlugin::<MirrorFlip>::default()).add_systems(PostUpdate, follow_cheat);
    }
}

/// The pass on the main camera while the running game has the cheat on.
fn follow_cheat(mut commands: Commands, play: Option<Res<crate::play::Play>>, cams: Query<(Entity, Has<MirrorFlip>), With<FlyCam>>) {
    let on = play.is_some_and(|p| p.game.cheat(rc_game::cheats::cheat::MIRROR));
    for (e, have) in &cams {
        match (on, have) {
            (true, false) => { commands.entity(e).insert(MirrorFlip { on: 1 }); }
            (false, true) => { commands.entity(e).remove::<MirrorFlip>(); }
            _ => {}
        }
    }
}
