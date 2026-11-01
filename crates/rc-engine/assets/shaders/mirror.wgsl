// The mirror cheat, byte 4 of 0x1a7a48 ("mirrored levels"; Rust side: mirror_render.rs).
//
// The camera update (0x2d9f38) rebuilds the camera matrix's second row the other way round, so the world is drawn as
// its mirror image while the 2D layer (the HUD, the menus, the movies) is not. The same picture here: the world's
// frame read back mirrored left to right, after the 3D passes and before the AA blit and the UI pass.

#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var screen_sampler: sampler;

struct MirrorFlip {
    on: u32,
}
@group(0) @binding(2) var<uniform> flip: MirrorFlip;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let size = vec2<i32>(textureDimensions(screen_texture));
    let p = vec2<i32>(floor(in.position.xy));
    let x = select(p.x, size.x - 1 - p.x, flip.on != 0u);
    return textureLoad(screen_texture, vec2<i32>(x, p.y), 0);
}
