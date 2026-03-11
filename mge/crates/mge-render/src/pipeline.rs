use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

use crate::batch::SpriteBatch;

/// Maximum number of sprite instances per draw call.
/// Clamped before GPU upload (Victor rec. #1).
pub const MAX_INSTANCES: usize = 16_384;

/// GPU-side layout for one sprite instance — 48 bytes, std430-compatible.
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct SpriteInstanceGpu {
    pub screen_pos: [f32; 2],
    pub uv: [f32; 4],
    pub tint: [f32; 4],
    pub scale: [f32; 2],
}

// Compile-time size check (Victor rec. #3)
const _: () = assert!(std::mem::size_of::<SpriteInstanceGpu>() == 48);

/// Unit quad: 6 vertices forming 2 triangles covering [0,0]-[1,1].
const QUAD_VERTICES: [[f32; 2]; 6] = [
    [0.0, 0.0],
    [1.0, 0.0],
    [1.0, 1.0],
    [0.0, 0.0],
    [1.0, 1.0],
    [0.0, 1.0],
];

/// Sprite instanced render pipeline.
///
/// Owns the wgpu pipeline, bind groups, and buffers needed to draw
/// a `SpriteBatch` in a single draw call using a 1×1 white sentinel texture.
pub struct SpritePipeline {
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    instance_buffer: wgpu::Buffer,
    tex_bind_group: wgpu::BindGroup,
    vp_bind_group: wgpu::BindGroup,
    vp_buffer: wgpu::Buffer,
}

impl SpritePipeline {
    /// Create the sprite pipeline, sentinel texture, and GPU buffers.
    #[allow(clippy::too_many_lines)]
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
    ) -> Self {
        // --- Shader module (static, never generated dynamically — Victor rec. #5) ---
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("mge.sprite.shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        // --- 1×1 white sentinel texture (Victor rec. #4) ---
        let white_pixel: [u8; 4] = [255, 255, 255, 255];
        assert!(white_pixel.len() == 4, "sentinel texture must be exactly 4 bytes");

        let tex = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("mge.sprite.tex1x1"),
            size: wgpu::Extent3d { width: 1, height: 1, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        queue.write_texture(
            tex.as_image_copy(),
            &white_pixel,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4),
                rows_per_image: None,
            },
            wgpu::Extent3d { width: 1, height: 1, depth_or_array_layers: 1 },
        );
        let tex_view = tex.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("mge.sprite.sampler"),
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        // --- Bind group layouts ---
        let vp_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("mge.sprite.vp_bgl"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
        let tex_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("mge.sprite.tex_bgl"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        // --- Viewport uniform buffer ---
        let vp_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("mge.sprite.vp_buf"),
            contents: bytemuck::cast_slice(&[1280.0_f32, 720.0_f32]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let vp_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("mge.sprite.vp_bg"),
            layout: &vp_bgl,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: vp_buffer.as_entire_binding(),
            }],
        });

        // --- Texture bind group ---
        let tex_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("mge.sprite.tex_bg"),
            layout: &tex_bgl,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&tex_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        // --- Vertex buffer (static quad) ---
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("mge.sprite.quad_vb"),
            contents: bytemuck::cast_slice(&QUAD_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        // --- Instance buffer (dynamic, MAX_INSTANCES) ---
        let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("mge.sprite.instance_buf"),
            size: (MAX_INSTANCES * std::mem::size_of::<SpriteInstanceGpu>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // --- Render pipeline ---
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("mge.sprite.pl_layout"),
            bind_group_layouts: &[&vp_bgl, &tex_bgl],
            immediate_size: 0,
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("mge.sprite.pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[
                    // Slot 0: quad vertices (per-vertex)
                    wgpu::VertexBufferLayout {
                        array_stride: 8, // 2 × f32
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &[wgpu::VertexAttribute {
                            shader_location: 0,
                            offset: 0,
                            format: wgpu::VertexFormat::Float32x2,
                        }],
                    },
                    // Slot 1: instance data (per-instance)
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<SpriteInstanceGpu>() as u64,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &[
                            // screen_pos
                            wgpu::VertexAttribute {
                                shader_location: 1,
                                offset: 0,
                                format: wgpu::VertexFormat::Float32x2,
                            },
                            // uv
                            wgpu::VertexAttribute {
                                shader_location: 2,
                                offset: 8,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                            // tint
                            wgpu::VertexAttribute {
                                shader_location: 3,
                                offset: 24,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                            // scale
                            wgpu::VertexAttribute {
                                shader_location: 4,
                                offset: 40,
                                format: wgpu::VertexFormat::Float32x2,
                            },
                        ],
                    },
                ],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });

        Self {
            pipeline,
            vertex_buffer,
            instance_buffer,
            tex_bind_group,
            vp_bind_group,
            vp_buffer,
        }
    }

    /// Draw the sprite batch into the given render target.
    ///
    /// Uses `LoadOp::Load` so this must be called *after* the clear pass.
    pub fn render(
        &self,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        batch: &SpriteBatch,
        viewport: [f32; 2],
    ) {
        let instances = batch.instances();
        let count = instances.len().min(MAX_INSTANCES);
        if count == 0 {
            return;
        }

        // Update viewport uniform
        queue.write_buffer(&self.vp_buffer, 0, bytemuck::cast_slice(&viewport));

        // Convert CPU instances → GPU layout (UV = [0,0,1,1] for 1×1 sentinel)
        let gpu_data: Vec<SpriteInstanceGpu> = instances[..count]
            .iter()
            .map(|i| SpriteInstanceGpu {
                screen_pos: i.screen_pos,
                uv: [0.0, 0.0, 1.0, 1.0],
                tint: i.tint,
                scale: i.scale,
            })
            .collect();
        queue.write_buffer(
            &self.instance_buffer,
            0,
            bytemuck::cast_slice(&gpu_data),
        );

        // Sprite render pass (LoadOp::Load — preserves the clear color)
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("mge.sprite.pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });

            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.vp_bind_group, &[]);
            pass.set_bind_group(1, &self.tex_bind_group, &[]);
            pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            pass.set_vertex_buffer(1, self.instance_buffer.slice(..));

            #[allow(clippy::cast_possible_truncation)]
            pass.draw(0..6, 0..count as u32);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sprite_instance_gpu_size() {
        assert_eq!(std::mem::size_of::<SpriteInstanceGpu>(), 48);
    }

    #[test]
    fn sprite_instance_gpu_alignment() {
        assert_eq!(std::mem::align_of::<SpriteInstanceGpu>(), 4);
    }

    #[test]
    fn max_instances_invariant() {
        assert!(MAX_INSTANCES <= 65_536);
        assert!(MAX_INSTANCES > 0);
    }

    #[test]
    fn gpu_instance_fields() {
        let inst = SpriteInstanceGpu {
            screen_pos: [100.0, 200.0],
            uv: [0.0, 0.0, 1.0, 1.0],
            tint: [1.0, 0.5, 0.0, 1.0],
            scale: [64.0, 32.0],
        };
        assert!((inst.screen_pos[0] - 100.0).abs() < f32::EPSILON);
        assert!((inst.tint[1] - 0.5).abs() < f32::EPSILON);
        assert!((inst.scale[0] - 64.0).abs() < f32::EPSILON);
    }

    #[test]
    fn gpu_instance_is_pod() {
        // bytemuck::cast_slice should work without panic
        let data = [SpriteInstanceGpu::zeroed(); 2];
        let bytes: &[u8] = bytemuck::cast_slice(&data);
        assert_eq!(bytes.len(), 96);
    }
}
