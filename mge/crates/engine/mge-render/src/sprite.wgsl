// ---------------------------------------------------------------------------
// sprite.wgsl -- MGE 2D Sprite Shader
// ---------------------------------------------------------------------------
//
// @id   sd-shader-sprite
// @do   render
// @role back-end
// @layer 2
// @human miyuk
//
// Renders textured quads for isometric tiles and sprites.
//
// Each sprite is a quad (4 vertices, 6 indices forming two triangles).
// Vertex positions arrive in screen-pixel space; the camera uniform
// (an orthographic projection incorporating offset and zoom) maps them
// to NDC.
//
// Bind group layout:
//   group(0) binding(0) -- CameraUniform (mat4x4<f32> view_proj)
//   group(1) binding(0) -- sprite texture (texture_2d<f32>)
//   group(1) binding(1) -- sprite sampler (sampler)
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Camera uniform -- orthographic projection with camera offset and zoom.
// ---------------------------------------------------------------------------

struct CameraUniform {
    /// Orthographic view-projection matrix.
    /// Built on the CPU as:
    ///   ortho(0, screen_w, screen_h, 0, -1, 1) * translate(-cam_x, -cam_y) * scale(zoom)
    /// This maps pixel coordinates to clip space [-1, 1].
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

// ---------------------------------------------------------------------------
// Texture and sampler -- bound per draw call (one atlas at a time).
// ---------------------------------------------------------------------------

@group(1) @binding(0)
var sprite_texture: texture_2d<f32>;

@group(1) @binding(1)
var sprite_sampler: sampler;

// ---------------------------------------------------------------------------
// Vertex input -- matches Rust-side SpriteVertex layout.
// ---------------------------------------------------------------------------
//
// SpriteVertex {
//     position: [f32; 2],   // screen-space pixel position (post iso transform)
//     uv:       [f32; 2],   // normalised texture coordinates
//     tint:     [f32; 4],   // RGBA tint colour
// }
//
// Vertex buffer layout (stride = 32 bytes):
//   @location(0) position : vec2<f32>  offset  0  (8 bytes)
//   @location(1) uv       : vec2<f32>  offset  8  (8 bytes)
//   @location(2) tint     : vec4<f32>  offset 16  (16 bytes)

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) uv:       vec2<f32>,
    @location(2) tint:     vec4<f32>,
};

// ---------------------------------------------------------------------------
// Vertex output / fragment input.
// ---------------------------------------------------------------------------

struct VertexOutput {
    /// Clip-space position (written by the vertex stage).
    @builtin(position) clip_position: vec4<f32>,
    /// Interpolated texture coordinates.
    @location(0) uv:   vec2<f32>,
    /// Interpolated RGBA tint colour.
    @location(1) tint: vec4<f32>,
};

// ---------------------------------------------------------------------------
// Vertex shader.
// ---------------------------------------------------------------------------
//
// The input position is already in screen-pixel space (the CPU performed
// the isometric world-to-screen conversion). We just need to project it
// into clip space via the camera's orthographic matrix.
//
// Z is fixed at 0.0 because depth ordering is handled by draw-call order
// (painter's algorithm: layers drawn back-to-front, sprites sorted within
// each layer by their depth key on the CPU side).

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    // Project pixel position to clip space through the camera matrix.
    // W = 1.0 for a standard affine transform.
    out.clip_position = camera.view_proj * vec4<f32>(in.position, 0.0, 1.0);

    // Pass UV and tint through to the fragment stage.
    out.uv   = in.uv;
    out.tint = in.tint;

    return out;
}

// ---------------------------------------------------------------------------
// Fragment shader.
// ---------------------------------------------------------------------------
//
// Samples the sprite atlas texture at the interpolated UV, multiplies by
// the vertex tint colour, and discards fully-transparent fragments so
// they do not write to the depth buffer or cause overdraw artifacts.

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Sample the atlas texture.
    let tex_color: vec4<f32> = textureSample(sprite_texture, sprite_sampler, in.uv);

    // Apply per-vertex tint (component-wise multiply).
    let final_color: vec4<f32> = tex_color * in.tint;

    // Discard nearly-transparent fragments to avoid ghost pixels and
    // unnecessary blending work. The threshold 0.01 catches sub-pixel
    // alpha from bilinear filtering while still rejecting clear areas.
    if final_color.a < 0.01 {
        discard;
    }

    return final_color;
}
