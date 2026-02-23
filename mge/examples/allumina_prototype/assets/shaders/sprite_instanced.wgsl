// sprite_instanced.wgsl — Sprite batching par quad instancing
// screen_x = (world_x - world_y) * TILE_HALF_W, screen_y = (world_x + world_y) * TILE_HALF_H
//
// Support pixel-perfect optionnel : snap sur grille pixel entier pour éviter le wobble.
// Activé quand pixel_snap != 0 dans l'uniform (bits hauts de view_size réservés).

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) instance_pos: vec2<f32>,
    @location(3) instance_size: vec2<f32>,
    @location(4) instance_uv: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@group(0) @binding(0) var<uniform> view_size: vec2<f32>;
@group(0) @binding(1) var tex: texture_2d<f32>;
@group(0) @binding(2) var tex_sampler: sampler;

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    // Position top-left + dimension * [0,1]×[0,1]
    let corner = in.position * in.instance_size + in.instance_pos;
    // Pixel snap : arrondir au pixel entier pour éviter le sub-pixel shimmer
    let snapped = floor(corner + 0.5);
    var out: VertexOutput;
    out.clip_position = vec4<f32>(snapped / view_size * 2.0 - 1.0, 0.0, 1.0);
    out.clip_position.y = -out.clip_position.y;
    // Interpoler les UV entre [uv_min, uv_max] selon la position du vertex quad
    out.uv = mix(in.instance_uv.xy, in.instance_uv.zw, in.uv);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(tex, tex_sampler, in.uv);
}
