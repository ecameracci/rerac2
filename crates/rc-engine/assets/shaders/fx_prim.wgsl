// GS primitives the game draws straight from a draw callback (crate::fx_draw): `FastDrawQuadReal` quads and
// `DrawEnvOverlayMesh` strips, built on the CPU every tick (the fire / smoke fields 760, the nanotech glow 806, …).
// Per vertex: position (Bevy space), ST, GS vertex RGBA bytes / 128. GS: PRIM 0x7c (Gouraud, TME, FGE, ABE), TEX0
// MODULATE with TCC (C = Ct·Cv >> 7, A = At·Av >> 7), then fog, then ALPHA 0x48 (Cs·As + Cd, params.misc.x = 1) or
// 0x44 ((Cs − Cd)·As + Cd), on the frame's display bytes like the GS (crate::display_blend `gs_add` / `gs_mix`;
// blend One, OneMinusSrcAlpha). The texture holds the raw GS bytes (texel alpha 0..0x80). FX_OPAQUE (FxPrimParams::opaque):
// an opaque world surface in the main pass, no blend, Z written. FX_FRAME (FxPrimParams::frame, rc_game::fx::FrameQuad):
// the colour is the frame image's (crate::frame_copy, display bytes) at the pixel's own screen place plus the UV_1
// offset (fractions of the game's 4:3 screen, interpolated as the GS's ST/Q) held to 0..params.frame.y; the alpha the
// FX texture's (the mask, params.frame.z = 1) or 0x80 (the frame image replaces the frame). The game sends each
// corner's screen place as its ST (`0x2ea5b8`) and clamps it per corner; the place is taken per pixel here, which
// keeps the quads that reach past the screen or near the camera from smearing the image across them.

#import bevy_pbr::view_transformations::{position_world_to_clip, position_world_to_view}
#import rerac::display_blend::{gs_add, gs_mix}
#import bevy_pbr::mesh_view_bindings::view

struct FxFog {
    color: vec4<f32>,
    params: vec4<f32>,
}

struct FxParams {
    misc: vec4<f32>,
    frame: vec4<f32>,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var tex: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var tex_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var<uniform> fog: FxFog;
@group(#{MATERIAL_BIND_GROUP}) @binding(3) var<uniform> params: FxParams;
@group(#{MATERIAL_BIND_GROUP}) @binding(4) var frame_tex: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(5) var frame_sampler: sampler;

struct FxVertex {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) color: vec4<f32>,
#ifdef FX_FRAME
    @location(3) offset: vec2<f32>,
#endif
}

struct FxVertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    // RGBA bytes / 128 and GS F / 255, screen-space linear like the GS.
    @location(1) @interpolate(linear) color: vec4<f32>,
    @location(2) @interpolate(linear) fog: f32,
#ifdef FX_FRAME
    // The offset from the pixel's screen place, in fractions of the view (perspective, as the GS's ST/Q).
    @location(3) offset: vec2<f32>,
#endif
}

@vertex
fn vertex(v: FxVertex) -> FxVertexOutput {
    var out: FxVertexOutput;
    out.position = position_world_to_clip(v.position);
    out.uv = v.uv;
    out.color = v.color;
    let depth = -position_world_to_view(v.position).z * 1024.0;
    let f = max(min(depth * fog.params.x + fog.params.y, fog.params.w), fog.params.z);
    out.fog = trunc(f) / 255.0;
#ifdef FX_FRAME
    // The offset's across part is a fraction of the game's 4:3 screen (the view's height × 4/3).
    let vp = view.viewport;
    out.offset = vec2<f32>(v.offset.x * vp.w * (4.0 / 3.0) / vp.z, v.offset.y);
#endif
    return out;
}

@fragment
fn fragment(in: FxVertexOutput) -> @location(0) vec4<f32> {
    let t = textureSample(tex, tex_sampler, in.uv);
    var ct = round(t * 255.0);
#ifdef FX_FRAME
    // The frame image's colour; the mask's alpha, or 0x80 (with a 0x80 vertex alpha: the frame replaced).
    // `0x2ea5b8`: the place on the screen, 0..1 across and down, moved and held to 0..limit, then into the frame
    // image (the view's target).
    let vp = view.viewport;
    let s = clamp((in.position.xy - vp.xy) / vp.zw + in.offset, vec2<f32>(0.0), vec2<f32>(params.frame.y));
    let f = textureSampleLevel(frame_tex, frame_sampler, (vp.xy + s * vp.zw) / vec2<f32>(textureDimensions(frame_tex)), 0.0);
    ct = vec4<f32>(round(f.rgb * 255.0), select(128.0, ct.a, params.frame.z > 0.5));
#endif
    // MODULATE on the bytes: floor(Ct·Cv / 128) (colour clamped to 255), A likewise.
    var rgb = min(floor(ct.rgb * in.color.rgb + vec3<f32>(1e-3)), vec3<f32>(255.0)) / 255.0;
    let a = min(floor(ct.a * in.color.a + 1e-3), 255.0);
    if (fog.color.w > 0.5) {
        rgb = mix(fog.color.rgb, rgb, in.fog);
    }
#ifdef FX_OPAQUE
    // An opaque world surface (FxPrimParams::opaque): the display bytes as the world shaders write them.
    return vec4<f32>(srgb_to_linear(rgb), 1.0);
#else
    let cs = round(rgb * 255.0);
    // 0x48 adds Cs·As; the subtractive pass (misc.y, a reverse-subtract blend) takes the same term off.
    if (params.misc.x > 0.5 || params.misc.y > 0.5) {
        return gs_add(cs, a);
    }
    return gs_mix(cs, a);
#endif
}

fn srgb_to_linear(c: vec3<f32>) -> vec3<f32> {
    let lo = c / 12.92;
    let hi = pow((c + 0.055) / 1.055, vec3<f32>(2.4));
    return select(hi, lo, c <= vec3<f32>(0.04045));
}
