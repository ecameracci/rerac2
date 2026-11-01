//! The frame's own image for the effects that sample it (rc_game::fx::FrameQuad; the game's `0x2ea6c8(n)` with n < 0:
//! a GS copy of the frame as it stands, −1, or the buffer on show, −7). Two textures the size of the world camera's
//! target, holding the frame's display bytes (crate::display_blend's effect target, `Rgba8Unorm`):
//! * [`FrameImages::copy`] — the frame as it stands when the first effect that samples it draws: crate::display_blend
//!   ends its effect pass before that item (the world, the translucent world, and the effects sorted before it: the
//!   mobys' effects, the list-1 callbacks, the particles, the list-2 callbacks), copies the target and goes on;
//!   one copy a frame, as the game's (`gp−0x6520`, cleared by the frame start `0x2ea390`);
//! * [`FrameImages::shown`] — the last finished frame: the effect target copied at the end of each frame's effect
//!   pass (the port's frame before its HUD and post passes; the game's buffer on show has its HUD).
//!
//! The entities that sample the copy carry [`FrameSample`] (crate::fx_draw), the world camera [`FrameCopyView`].

use bevy::asset::RenderAssetUsages;
use bevy::image::{ImageAddressMode, ImageFilterMode, ImageSampler, ImageSamplerDescriptor};
use bevy::platform::collections::HashSet;
use bevy::prelude::*;
use bevy::render::extract_component::{ExtractComponent, ExtractComponentPlugin};
use bevy::render::extract_resource::{ExtractResource, ExtractResourcePlugin};
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_resource::{Extent3d, Texture, TextureDimension, TextureUsages};
use bevy::render::renderer::RenderContext;
use bevy::render::sync_world::MainEntity;
use bevy::render::texture::GpuImage;
use bevy::render::{Extract, ExtractSchedule, RenderApp};

pub use rc_game::fx::FrameSource;

/// The two frame textures (module docs); their size follows the world camera's target.
#[derive(Resource, Clone, ExtractResource)]
pub struct FrameImages {
    pub copy: Handle<Image>,
    pub shown: Handle<Image>,
    size: UVec2,
}

impl FrameImages {
    pub fn get(&self, source: FrameSource) -> Handle<Image> {
        match source {
            FrameSource::Copy => self.copy.clone(),
            FrameSource::Shown => self.shown.clone(),
        }
    }
}

/// An effect entity whose material samples a frame image.
#[derive(Component, Clone, Copy, Debug)]
pub struct FrameSample(pub FrameSource);

/// The view whose effect pass makes the frame images (the world camera, crate::fly_cam::FlyCam).
#[derive(Component, ExtractComponent, Clone, Copy, Default)]
pub struct FrameCopyView;

/// Render world: the main-world entities that sample [`FrameImages::copy`] (the copy is taken before the first).
#[derive(Resource, Default)]
pub struct CopySamplers(pub HashSet<MainEntity>);

pub struct FrameCopyPlugin;

impl Plugin for FrameCopyPlugin {
    fn build(&self, app: &mut App) {
        let mut images = app.world_mut().resource_mut::<Assets<Image>>();
        let size = UVec2::ONE;
        let (copy, shown) = (images.add(frame_image(size)), images.add(frame_image(size)));
        app.insert_resource(FrameImages { copy, shown, size })
            .add_plugins((ExtractResourcePlugin::<FrameImages>::default(), ExtractComponentPlugin::<FrameCopyView>::default()))
            .add_systems(Update, (tag_view, follow_size));
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else { return };
        render_app.init_resource::<CopySamplers>().add_systems(ExtractSchedule, extract_samplers);
    }
}

/// A frame texture of `size`: display bytes, bilinear, clamped at the edges.
fn frame_image(size: UVec2) -> Image {
    let mut img = Image::new_uninit(
        Extent3d { width: size.x.max(1), height: size.y.max(1), depth_or_array_layers: 1 },
        TextureDimension::D2,
        crate::display_blend::EFFECT_FORMAT,
        RenderAssetUsages::RENDER_WORLD,
    );
    img.texture_descriptor.usage = TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST;
    img.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::ClampToEdge,
        address_mode_v: ImageAddressMode::ClampToEdge,
        mag_filter: ImageFilterMode::Linear,
        min_filter: ImageFilterMode::Linear,
        ..default()
    });
    img
}

fn tag_view(mut commands: Commands, cams: Query<Entity, (With<crate::fly_cam::FlyCam>, Without<FrameCopyView>)>) {
    for e in &cams { commands.entity(e).insert(FrameCopyView); }
}

/// Keeps the frame textures the size of the world camera's target.
fn follow_size(mut frames: ResMut<FrameImages>, mut images: ResMut<Assets<Image>>, cams: Query<&Camera, With<FrameCopyView>>) {
    let Some(size) = cams.iter().find_map(Camera::physical_target_size) else { return };
    if frames.size == size { return; }
    frames.size = size;
    let e = Extent3d { width: size.x.max(1), height: size.y.max(1), depth_or_array_layers: 1 };
    for h in [frames.copy.clone(), frames.shown.clone()] {
        if let Some(mut img) = images.get_mut(&h) { img.resize(e); }
    }
}

fn extract_samplers(mut set: ResMut<CopySamplers>, q: Extract<Query<(Entity, &FrameSample)>>) {
    set.0.clear();
    set.0.extend(q.iter().filter(|(_, s)| s.0 == FrameSource::Copy).map(|(e, _)| MainEntity::from(e)));
}

/// Copies `src` (the effect target, display bytes) into the frame texture `source` (render world; nothing when it is
/// not ready). The copy covers the smaller of the two sizes.
pub fn copy_into(ctx: &mut RenderContext, world: &World, src: &Texture, source: FrameSource) {
    let (Some(frames), Some(gpu)) = (world.get_resource::<FrameImages>(), world.get_resource::<RenderAssets<GpuImage>>()) else { return };
    let Some(dst) = gpu.get(&frames.get(source)) else { return };
    let (a, b) = (src.size(), dst.texture.size());
    if src.format() != dst.texture.format() { return; }
    let size = Extent3d { width: a.width.min(b.width), height: a.height.min(b.height), depth_or_array_layers: 1 };
    ctx.command_encoder().copy_texture_to_texture(src.as_image_copy(), dst.texture.as_image_copy(), size);
}
