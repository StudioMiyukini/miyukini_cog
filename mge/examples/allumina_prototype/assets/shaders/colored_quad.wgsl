// Quads colorés (pas de texture) pour overlay UI Dev
struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) instance_pos: vec2<f32>,
    @location(3) instance_size: vec2<f32>,
    @location(4) instance_color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
}

@group(0) @binding(0) var<uniform> view_size: vec2<f32>;

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    let corner = in.position * in.instance_size + in.instance_pos;
    var out: VertexOutput;
    out.clip_position = vec4<f32>(corner / view_size * 2.0 - 1.0, 0.0, 1.0);
    out.clip_position.y = -out.clip_position.y;
    out.color = in.instance_color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
