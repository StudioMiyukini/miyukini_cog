// ---------------------------------------------------------------------------
// sprite_instanced.wgsl -- MGE Instanced 2D Sprite Shader
// ---------------------------------------------------------------------------
//
// @id   MGE-Shader-SpriteInstanced
// @do   render
// @role back-end
// @layer 2
// @human francois
//
// Renders textured quads via GPU instancing. Each sprite's data is read from
// a storage buffer indexed by `instance_index`. A static unit quad (4 vertices,
// 6 indices) is scaled/positioned per instance.
//
// Bind group layout:
//   group(0) binding(0) -- CameraUniform (mat4x4<f32> view_proj)
//   group(1) binding(0) -- sprite texture array (texture_2d_array<f32>)
//   group(1) binding(1) -- sprite sampler (sampler)
//   group(2) binding(0) -- instance storage buffer (read-only)
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Camera uniform -- orthographic projection with camera offset and zoom.
// ---------------------------------------------------------------------------

struct CameraUniform {
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

// ---------------------------------------------------------------------------
// Texture array and sampler.
// ---------------------------------------------------------------------------

@group(1) @binding(0)
var sprite_textures: texture_2d_array<f32>;

@group(1) @binding(1)
var sprite_sampler: sampler;

// ---------------------------------------------------------------------------
// Instance data -- matches Rust-side InstanceData layout (64 bytes).
// ---------------------------------------------------------------------------
//
// InstanceData {
//     position:      vec2<f32>,   //  0 ..  8
//     size:          vec2<f32>,   //  8 .. 16
//     uv_rect:       vec4<f32>,   // 16 .. 32
//     tint:          vec4<f32>,   // 32 .. 48
//     texture_index: u32,         // 48 .. 52
//     z_depth:       f32,         // 52 .. 56
//     _pad:          vec2<f32>,   // 56 .. 64
// }

struct InstanceData {
    position: vec2<f32>,
    size: vec2<f32>,
    uv_rect: vec4<f32>,
    tint: vec4<f32>,
    texture_index: u32,
    z_depth: f32,
    _pad: vec2<f32>,
};

@group(2) @binding(0)
var<storage, read> instances: array<InstanceData>;

// ---------------------------------------------------------------------------
// Vertex input -- unit quad positions passed via vertex buffer.
// ---------------------------------------------------------------------------

struct VertexInput {
    @location(0) vertex_pos: vec2<f32>,
};

// ---------------------------------------------------------------------------
// Vertex output / fragment input.
// ---------------------------------------------------------------------------

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) tint: vec4<f32>,
    @location(2) @interpolate(flat) texture_index: u32,
};

// ---------------------------------------------------------------------------
// Vertex shader.
// ---------------------------------------------------------------------------
//
// The unit quad has vertices (0,0), (1,0), (1,1), (0,1).
// For each instance:
//   world_pos = instance.position + vertex_pos * instance.size
//   uv = mix(uv_rect.xy, uv_rect.zw, vertex_pos)

@vertex
fn vs_main(in: VertexInput, @builtin(instance_index) instance_idx: u32) -> VertexOutput {
    let inst = instances[instance_idx];

    // Scale and translate the unit quad to the sprite's world position.
    let world_pos = inst.position + in.vertex_pos * inst.size;

    var out: VertexOutput;
    out.clip_position = camera.view_proj * vec4<f32>(world_pos, inst.z_depth, 1.0);
    out.uv = mix(inst.uv_rect.xy, inst.uv_rect.zw, in.vertex_pos);
    out.tint = inst.tint;
    out.texture_index = inst.texture_index;

    return out;
}

// ---------------------------------------------------------------------------
// Fragment shader.
// ---------------------------------------------------------------------------
//
// Samples the texture array at the interpolated UV with the instance's
// texture layer index, applies tint, and discards transparent fragments.

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let tex_color = textureSample(
        sprite_textures,
        sprite_sampler,
        in.uv,
        i32(in.texture_index)
    );

    let final_color = tex_color * in.tint;

    if final_color.a < 0.01 {
        discard;
    }

    return final_color;
}
