// Sprite instanced shader — mge-render
// Group 0: viewport uniform
// Group 1: texture + sampler

struct Viewport {
    size: vec2<f32>,
}

@group(0) @binding(0)
var<uniform> vp: Viewport;

@group(1) @binding(0)
var t_sprite: texture_2d<f32>;

@group(1) @binding(1)
var s_sprite: sampler;

struct VertIn {
    // Per-vertex (quad corner)
    @location(0) quad_pos: vec2<f32>,
    // Per-instance
    @location(1) screen_pos: vec2<f32>,
    @location(2) uv: vec4<f32>,
    @location(3) tint: vec4<f32>,
    @location(4) scale: vec2<f32>,
}

struct VertOut {
    @builtin(position) clip: vec4<f32>,
    @location(0) tex_uv: vec2<f32>,
    @location(1) tint: vec4<f32>,
}

@vertex
fn vs_main(in: VertIn) -> VertOut {
    // Pixel position of this vertex
    let px = in.screen_pos + in.quad_pos * in.scale;
    // Convert pixel coords to NDC [-1, 1], Y flipped (screen Y down, NDC Y up)
    let ndc = vec2<f32>(
        px.x / vp.size.x * 2.0 - 1.0,
        1.0 - px.y / vp.size.y * 2.0,
    );
    // UV interpolation within atlas rect
    let uv = vec2<f32>(
        mix(in.uv.x, in.uv.z, in.quad_pos.x),
        mix(in.uv.y, in.uv.w, in.quad_pos.y),
    );
    var out: VertOut;
    out.clip = vec4<f32>(ndc, 0.0, 1.0);
    out.tex_uv = uv;
    out.tint = in.tint;
    return out;
}

@fragment
fn fs_main(in: VertOut) -> @location(0) vec4<f32> {
    let col = textureSample(t_sprite, s_sprite, in.tex_uv);
    let result = col * in.tint;
    // Discard fully transparent fragments
    if result.a < 0.01 {
        discard;
    }
    return result;
}
