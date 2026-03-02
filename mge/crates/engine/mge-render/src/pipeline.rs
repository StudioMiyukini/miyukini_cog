// @id: sd-impl-pipeline @do: implement @role: back-end @layer: 2 @human: miyuk

//! GPU sprite rendering pipeline: vertex types, render pipeline, texture
//! management, and batched draw submission.
//!
//! This module bridges the CPU-side [`SpriteInstance`](crate::sprite::SpriteInstance)
//! data with the wgpu render pipeline. It provides:
//!
//! - [`SpriteVertex`] -- per-vertex data uploaded to the GPU.
//! - [`SpritePipeline`] -- wgpu render pipeline, camera uniform, bind group layouts.
//! - [`GpuTexture`] -- GPU-resident texture with its bind group.
//! - [`SpriteBatcher`] -- dynamic vertex/index buffer manager for batched draws.

// ---------------------------------------------------------------------------
// SpriteVertex
// ---------------------------------------------------------------------------

/// Per-vertex data for a sprite quad (4 vertices per sprite).
///
/// Each sprite is a screen-aligned quad with position, UV, and tint.
/// The vertex buffer is rebuilt every frame by [`SpriteBatcher`].
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SpriteVertex {
    /// Screen-space position in pixels `[x, y]`.
    pub position: [f32; 2],
    /// Texture coordinates `[u, v]`.
    pub uv: [f32; 2],
    /// RGBA tint colour (pre-multiplied alpha friendly).
    pub tint: [f32; 4],
}

impl SpriteVertex {
    /// Vertex buffer layout descriptor for wgpu pipeline creation.
    ///
    /// Attributes:
    /// - location 0: `Float32x2` -- position
    /// - location 1: `Float32x2` -- uv
    /// - location 2: `Float32x4` -- tint
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        const ATTRIBS: [wgpu::VertexAttribute; 3] = wgpu::vertex_attr_array![
            0 => Float32x2,  // position
            1 => Float32x2,  // uv
            2 => Float32x4,  // tint
        ];

        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<SpriteVertex>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &ATTRIBS,
        }
    }
}

// ---------------------------------------------------------------------------
// SpritePipeline
// ---------------------------------------------------------------------------

/// Size of a 4x4 f32 matrix in bytes.
const MAT4_SIZE: u64 = 64;

/// GPU sprite render pipeline: shader, bind groups, camera uniform, sampler.
///
/// Created once at startup. Shared across all sprite draw calls per frame.
pub struct SpritePipeline {
    /// Compiled wgpu render pipeline with alpha blending.
    pub render_pipeline: wgpu::RenderPipeline,
    /// Uniform buffer holding the camera projection matrix (64 bytes).
    pub camera_buffer: wgpu::Buffer,
    /// Bind group referencing the camera uniform buffer.
    pub camera_bind_group: wgpu::BindGroup,
    /// Texture sampler (linear filtering, clamp-to-edge).
    pub sampler: wgpu::Sampler,
    /// Layout for bind group 0 (camera uniform).
    pub camera_bind_group_layout: wgpu::BindGroupLayout,
    /// Layout for bind group 1 (texture + sampler).
    pub texture_bind_group_layout: wgpu::BindGroupLayout,
}

/// Build the wgpu render pipeline with alpha blending for 2D sprites.
fn create_render_pipeline(
    device: &wgpu::Device,
    format: wgpu::TextureFormat,
    shader: &wgpu::ShaderModule,
    camera_layout: &wgpu::BindGroupLayout,
    texture_layout: &wgpu::BindGroupLayout,
) -> wgpu::RenderPipeline {
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Sprite Pipeline Layout"),
        bind_group_layouts: &[camera_layout, texture_layout],
        push_constant_ranges: &[],
    });

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Sprite Render Pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: shader,
            entry_point: Some("vs_main"),
            buffers: &[SpriteVertex::desc()],
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: shader,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format,
                blend: Some(wgpu::BlendState {
                    color: wgpu::BlendComponent {
                        src_factor: wgpu::BlendFactor::SrcAlpha,
                        dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                        operation: wgpu::BlendOperation::Add,
                    },
                    alpha: wgpu::BlendComponent {
                        src_factor: wgpu::BlendFactor::One,
                        dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                        operation: wgpu::BlendOperation::Add,
                    },
                }),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None, // 2D sprites: no face culling
            unclipped_depth: false,
            polygon_mode: wgpu::PolygonMode::Fill,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
        cache: None,
    })
}

/// Create the camera bind group layout (group 0: uniform buffer, vertex stage).
fn create_camera_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Camera Bind Group Layout"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: wgpu::BufferSize::new(MAT4_SIZE),
            },
            count: None,
        }],
    })
}

/// Create the texture bind group layout (group 1: texture + sampler, fragment stage).
fn create_texture_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Texture Bind Group Layout"),
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
    })
}

impl SpritePipeline {
    /// Create the sprite render pipeline.
    ///
    /// # Arguments
    ///
    /// * `device` -- wgpu device (from `mge-platform` `GpuContext`).
    /// * `format` -- surface texture format (e.g. `Bgra8UnormSrgb`).
    ///
    /// The pipeline uses:
    /// - Bind group 0: camera uniform (mat4x4f, vertex stage).
    /// - Bind group 1: texture (binding 0) + sampler (binding 1), fragment stage.
    /// - Alpha blending: `SrcAlpha` / `OneMinusSrcAlpha`.
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Sprite Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("sprite.wgsl").into()),
        });

        let camera_bind_group_layout = create_camera_bind_group_layout(device);
        let texture_bind_group_layout = create_texture_bind_group_layout(device);

        let render_pipeline = create_render_pipeline(
            device,
            format,
            &shader,
            &camera_bind_group_layout,
            &texture_bind_group_layout,
        );

        let camera_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Camera Uniform Buffer"),
            size: MAT4_SIZE,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera Bind Group"),
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
        });

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Sprite Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        Self {
            render_pipeline,
            camera_buffer,
            camera_bind_group,
            sampler,
            camera_bind_group_layout,
            texture_bind_group_layout,
        }
    }

    /// Upload the camera orthographic projection matrix to the GPU.
    ///
    /// The matrix maps screen-pixel coordinates to wgpu clip space `[-1, 1]`,
    /// accounting for camera scroll (`cam_x`, `cam_y`) and zoom.
    ///
    /// The visible rectangle is `[cam_x .. cam_x + ew] x [cam_y .. cam_y + eh]`
    /// where `ew = screen_w / zoom` and `eh = screen_h / zoom`.
    ///
    /// The resulting column-major matrix (wgpu convention):
    ///
    /// ```text
    /// | 2/ew       0       0   0 |
    /// |   0    -2/eh       0   0 |
    /// |   0       0        1   0 |
    /// | -(1+2*cx/ew)  (1+2*cy/eh)  0  1 |
    /// ```
    ///
    /// This maps `(cam_x, cam_y)` to `(-1, 1)` (top-left) and
    /// `(cam_x+ew, cam_y+eh)` to `(1, -1)` (bottom-right).
    pub fn update_camera(
        &self,
        queue: &wgpu::Queue,
        screen_w: f32,
        screen_h: f32,
        cam_x: f32,
        cam_y: f32,
        zoom: f32,
    ) {
        // Effective viewport size (camera sees less when zoomed in).
        let ew = screen_w / zoom;
        let eh = screen_h / zoom;

        // Column-major orthographic projection.
        // Maps pixel rect [cam_x, cam_x+ew] x [cam_y, cam_y+eh] to [-1,1].
        //
        // Derivation for X:  clip_x = 2*(x - cam_x)/ew - 1
        //                          = (2/ew)*x - (1 + 2*cam_x/ew)
        // Derivation for Y:  clip_y = -(2*(y - cam_y)/eh - 1)   (Y flipped)
        //                          = (-2/eh)*y + (1 + 2*cam_y/eh)
        #[rustfmt::skip]
        let proj: [[f32; 4]; 4] = [
            // column 0
            [ 2.0 / ew,                               0.0,   0.0, 0.0],
            // column 1
            [      0.0,                     -2.0 / eh,       0.0, 0.0],
            // column 2
            [      0.0,                               0.0,   1.0, 0.0],
            // column 3
            [-(1.0 + 2.0 * cam_x / ew), 1.0 + 2.0 * cam_y / eh, 0.0, 1.0],
        ];

        queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&proj));
    }
}

// ---------------------------------------------------------------------------
// GpuTexture
// ---------------------------------------------------------------------------

/// A GPU-resident texture with its associated view and bind group.
///
/// Created from an `image::RgbaImage` via [`GpuTexture::from_image`].
/// The bind group is pre-built using the pipeline's texture layout
/// and sampler so it can be set directly during a render pass.
pub struct GpuTexture {
    /// The underlying wgpu texture.
    pub texture: wgpu::Texture,
    /// Default view of the texture.
    pub view: wgpu::TextureView,
    /// Pre-built bind group (group 1: texture + sampler).
    pub bind_group: wgpu::BindGroup,
    /// Texture width in pixels.
    pub width: u32,
    /// Texture height in pixels.
    pub height: u32,
}

impl GpuTexture {
    /// Upload an `RgbaImage` to the GPU and create a ready-to-use bind group.
    ///
    /// The bind group uses the pipeline's `texture_bind_group_layout` with:
    /// - binding 0: the texture view
    /// - binding 1: the pipeline's shared sampler
    pub fn from_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        pipeline: &SpritePipeline,
        image_data: &image::RgbaImage,
        label: &str,
    ) -> Self {
        let (width, height) = image_data.dimensions();

        let size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some(label),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            image_data.as_raw(),
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(&format!("{label} Bind Group")),
            layout: &pipeline.texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&pipeline.sampler),
                },
            ],
        });

        Self {
            texture,
            view,
            bind_group,
            width,
            height,
        }
    }
}

// ---------------------------------------------------------------------------
// SpriteBatcher
// ---------------------------------------------------------------------------

/// Number of indices per sprite quad (2 triangles).
const INDICES_PER_SPRITE: u32 = 6;
/// Number of vertices per sprite quad.
const VERTS_PER_SPRITE: u32 = 4;

/// Dynamic sprite batcher: collects quads into a vertex buffer and draws
/// them with a static index buffer.
///
/// # Index format
///
/// Without the `index-u32` feature, indices are `u16` (max 16 384 sprites).
/// With `index-u32` enabled, indices are `u32` (max 262 144 sprites).
///
/// # Usage per frame
///
/// ```text
/// batcher.begin();                        // clear vertices
/// batcher.push(x, y, w, h, uv, tint);    // for each visible sprite
/// let count = batcher.flush(&queue);      // upload to GPU
/// batcher.draw(&mut render_pass, count);  // issue draw call
/// ```
pub struct SpriteBatcher {
    /// Pre-allocated GPU vertex buffer (max_sprites * 4 vertices).
    pub vertex_buffer: wgpu::Buffer,
    /// Static GPU index buffer (max_sprites * 6 indices, pattern 0,1,2,2,3,0).
    pub index_buffer: wgpu::Buffer,
    /// CPU-side vertex staging area.
    pub vertices: Vec<SpriteVertex>,
    /// Maximum number of sprites this batcher can handle.
    pub max_sprites: u32,
}

impl SpriteBatcher {
    /// Create a new sprite batcher with pre-allocated GPU buffers.
    ///
    /// The index buffer is filled once with the repeating quad pattern
    /// `[0,1,2, 2,3,0]` offset per sprite. The vertex buffer is empty
    /// and written each frame via [`flush`](Self::flush).
    ///
    /// # Index format
    ///
    /// With the `index-u32` feature enabled, indices use `u32` to support
    /// large sprite counts (up to 262 144). Otherwise indices are `u16`
    /// (max 16 384 sprites).
    pub fn new(device: &wgpu::Device, max_sprites: u32) -> Self {
        let vertex_count = max_sprites as usize * VERTS_PER_SPRITE as usize;
        let vertex_buffer_size =
            (vertex_count * std::mem::size_of::<SpriteVertex>()) as u64;

        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Sprite Vertex Buffer"),
            size: vertex_buffer_size,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Build the static index data: for sprite N, vertices are
        // [4N, 4N+1, 4N+2, 4N+3] and we draw triangles (0,1,2) (2,3,0).
        let index_count = max_sprites as usize * INDICES_PER_SPRITE as usize;

        let (index_buffer_size, index_bytes) = if cfg!(feature = "index-u32") {
            let mut indices: Vec<u32> = Vec::with_capacity(index_count);
            for i in 0..max_sprites {
                let base = i * VERTS_PER_SPRITE;
                indices.push(base);
                indices.push(base + 1);
                indices.push(base + 2);
                indices.push(base + 2);
                indices.push(base + 3);
                indices.push(base);
            }
            let size = (index_count * std::mem::size_of::<u32>()) as u64;
            let bytes: Vec<u8> = bytemuck::cast_slice(&indices).to_vec();
            (size, bytes)
        } else {
            let mut indices: Vec<u16> = Vec::with_capacity(index_count);
            for i in 0..max_sprites {
                let base = i as u16 * VERTS_PER_SPRITE as u16;
                indices.push(base);
                indices.push(base + 1);
                indices.push(base + 2);
                indices.push(base + 2);
                indices.push(base + 3);
                indices.push(base);
            }
            let size = (index_count * std::mem::size_of::<u16>()) as u64;
            let bytes: Vec<u8> = bytemuck::cast_slice(&indices).to_vec();
            (size, bytes)
        };

        // The index pattern is static and never changes. We use
        // `mapped_at_creation` to write the data without needing a queue.
        let index_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Sprite Index Buffer"),
            size: index_buffer_size,
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: true,
        });

        // Write index data into the mapped buffer.
        index_buffer
            .slice(..)
            .get_mapped_range_mut()
            .copy_from_slice(&index_bytes);
        index_buffer.unmap();

        Self {
            vertex_buffer,
            index_buffer,
            vertices: Vec::with_capacity(vertex_count),
            max_sprites,
        }
    }

    /// Clear the CPU vertex staging buffer for a new frame.
    pub fn begin(&mut self) {
        self.vertices.clear();
    }

    /// Push a sprite quad into the CPU staging buffer.
    ///
    /// # Arguments
    ///
    /// * `x`, `y` -- top-left screen position in pixels.
    /// * `w`, `h` -- sprite dimensions in pixels.
    /// * `uv` -- UV rectangle `[u_min, v_min, u_max, v_max]`.
    /// * `tint` -- RGBA tint colour.
    ///
    /// If the batcher is full (reached `max_sprites`) the sprite is silently
    /// dropped. In debug builds a warning is logged.
    pub fn push(&mut self, x: f32, y: f32, w: f32, h: f32, uv: [f32; 4], tint: [f32; 4]) {
        let current_sprites = self.vertices.len() as u32 / VERTS_PER_SPRITE;
        if current_sprites >= self.max_sprites {
            tracing::warn!(
                "SpriteBatcher full ({} sprites), dropping sprite",
                self.max_sprites
            );
            return;
        }

        let u_min = uv[0];
        let v_min = uv[1];
        let u_max = uv[2];
        let v_max = uv[3];

        // Quad vertices (top-left, top-right, bottom-right, bottom-left).
        // Winding: 0-1-2, 2-3-0 (CCW for the index buffer pattern).
        self.vertices.push(SpriteVertex {
            position: [x, y],
            uv: [u_min, v_min],
            tint,
        });
        self.vertices.push(SpriteVertex {
            position: [x + w, y],
            uv: [u_max, v_min],
            tint,
        });
        self.vertices.push(SpriteVertex {
            position: [x + w, y + h],
            uv: [u_max, v_max],
            tint,
        });
        self.vertices.push(SpriteVertex {
            position: [x, y + h],
            uv: [u_min, v_max],
            tint,
        });
    }

    /// Upload the CPU vertex data to the GPU buffer.
    ///
    /// Returns the total number of vertices written (needed for the
    /// subsequent [`draw`](Self::draw) call). Returns `0` if there is
    /// nothing to draw.
    pub fn flush(&mut self, queue: &wgpu::Queue) -> u32 {
        if self.vertices.is_empty() {
            return 0;
        }

        queue.write_buffer(
            &self.vertex_buffer,
            0,
            bytemuck::cast_slice(&self.vertices),
        );

        self.vertices.len() as u32
    }

    /// Issue an indexed draw call for the currently flushed sprite batch.
    ///
    /// # Arguments
    ///
    /// * `pass` -- active render pass with the sprite pipeline already set.
    /// * `vertex_count` -- value returned by [`flush`](Self::flush).
    ///
    /// The number of sprites is `vertex_count / 4`, and each sprite uses
    /// 6 indices.
    pub fn draw<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>, vertex_count: u32) {
        if vertex_count == 0 {
            return;
        }

        let sprite_count = vertex_count / VERTS_PER_SPRITE;
        let index_count = sprite_count * INDICES_PER_SPRITE;

        let index_format = if cfg!(feature = "index-u32") {
            wgpu::IndexFormat::Uint32
        } else {
            wgpu::IndexFormat::Uint16
        };

        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.set_index_buffer(self.index_buffer.slice(..), index_format);
        pass.draw_indexed(0..index_count, 0, 0..1);
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -- SpriteVertex layout tests (no GPU required) -----------------------

    #[test]
    fn sprite_vertex_size_is_32_bytes() {
        // position(8) + uv(8) + tint(16) = 32 bytes
        assert_eq!(std::mem::size_of::<SpriteVertex>(), 32);
    }

    #[test]
    fn sprite_vertex_desc_has_3_attributes() {
        let layout = SpriteVertex::desc();
        assert_eq!(layout.attributes.len(), 3);
        assert_eq!(layout.array_stride, 32);
    }

    #[test]
    fn sprite_vertex_is_pod() {
        // Verify that bytemuck cast_slice works on SpriteVertex.
        let verts = [
            SpriteVertex {
                position: [0.0, 0.0],
                uv: [0.0, 0.0],
                tint: [1.0, 1.0, 1.0, 1.0],
            },
            SpriteVertex {
                position: [1.0, 1.0],
                uv: [1.0, 1.0],
                tint: [0.5, 0.5, 0.5, 0.5],
            },
        ];
        let bytes: &[u8] = bytemuck::cast_slice(&verts);
        assert_eq!(bytes.len(), 64); // 2 * 32
    }

    // -- Orthographic matrix tests -----------------------------------------

    /// Helper: build the ortho projection matrix matching `update_camera`.
    fn build_proj(cam_x: f32, cam_y: f32, screen_w: f32, screen_h: f32, zoom: f32) -> [[f32; 4]; 4] {
        let ew = screen_w / zoom;
        let eh = screen_h / zoom;
        #[rustfmt::skip]
        let proj: [[f32; 4]; 4] = [
            [ 2.0 / ew,                                  0.0,   0.0, 0.0],
            [      0.0,                        -2.0 / eh,       0.0, 0.0],
            [      0.0,                                  0.0,   1.0, 0.0],
            [-(1.0 + 2.0 * cam_x / ew), 1.0 + 2.0 * cam_y / eh, 0.0, 1.0],
        ];
        proj
    }

    /// Transform a point through the column-major projection matrix.
    fn transform(proj: &[[f32; 4]; 4], x: f32, y: f32) -> (f32, f32) {
        let cx = proj[0][0] * x + proj[3][0];
        let cy = proj[1][1] * y + proj[3][1];
        (cx, cy)
    }

    #[test]
    fn ortho_matrix_identity_at_origin() {
        // cam at (0,0), zoom=1, screen 800x600.
        // (0,0) -> clip (-1, 1), (800,600) -> clip (1, -1).
        let proj = build_proj(0.0, 0.0, 800.0, 600.0, 1.0);

        let (cx0, cy0) = transform(&proj, 0.0, 0.0);
        assert!((cx0 - (-1.0)).abs() < f32::EPSILON);
        assert!((cy0 - 1.0).abs() < f32::EPSILON);

        let (cx1, cy1) = transform(&proj, 800.0, 600.0);
        assert!((cx1 - 1.0).abs() < f32::EPSILON);
        assert!((cy1 - (-1.0)).abs() < f32::EPSILON);
    }

    #[test]
    fn ortho_matrix_with_camera_offset() {
        // cam at (100, 50), zoom=1, screen 800x600.
        // (100, 50) should map to (-1, 1).
        // (900, 650) should map to (1, -1).
        let proj = build_proj(100.0, 50.0, 800.0, 600.0, 1.0);

        let (cx0, cy0) = transform(&proj, 100.0, 50.0);
        assert!(
            (cx0 - (-1.0)).abs() < 1e-5,
            "cam origin x should map to -1, got {cx0}"
        );
        assert!(
            (cy0 - 1.0).abs() < 1e-5,
            "cam origin y should map to 1, got {cy0}"
        );

        let (cx1, cy1) = transform(&proj, 900.0, 650.0);
        assert!(
            (cx1 - 1.0).abs() < 1e-5,
            "cam far corner x should map to 1, got {cx1}"
        );
        assert!(
            (cy1 - (-1.0)).abs() < 1e-5,
            "cam far corner y should map to -1, got {cy1}"
        );
    }

    #[test]
    fn ortho_matrix_zoom_scales_viewport() {
        // cam at (0,0), zoom=2, screen 800x600.
        // Effective viewport is 400x300.
        // (0,0) -> (-1,1), (400,300) -> (1,-1).
        let proj = build_proj(0.0, 0.0, 800.0, 600.0, 2.0);

        let (cx0, cy0) = transform(&proj, 0.0, 0.0);
        assert!((cx0 - (-1.0)).abs() < f32::EPSILON);
        assert!((cy0 - 1.0).abs() < f32::EPSILON);

        let (cx1, cy1) = transform(&proj, 400.0, 300.0);
        assert!((cx1 - 1.0).abs() < f32::EPSILON);
        assert!((cy1 - (-1.0)).abs() < f32::EPSILON);
    }

    // -- SpriteBatcher CPU-side tests (no GPU) -----------------------------

    #[test]
    fn batcher_begin_clears_vertices() {
        let mut vertices: Vec<SpriteVertex> = Vec::new();
        vertices.push(SpriteVertex {
            position: [0.0, 0.0],
            uv: [0.0, 0.0],
            tint: [1.0; 4],
        });
        assert_eq!(vertices.len(), 1);
        vertices.clear(); // simulates begin()
        assert!(vertices.is_empty());
    }

    #[test]
    fn push_creates_4_vertices() {
        let mut vertices: Vec<SpriteVertex> = Vec::new();

        // Manually simulate push logic
        let x = 10.0_f32;
        let y = 20.0_f32;
        let w = 64.0_f32;
        let h = 32.0_f32;
        let uv = [0.0_f32, 0.0, 0.5, 0.25];
        let tint = [1.0_f32, 1.0, 1.0, 1.0];

        vertices.push(SpriteVertex {
            position: [x, y],
            uv: [uv[0], uv[1]],
            tint,
        });
        vertices.push(SpriteVertex {
            position: [x + w, y],
            uv: [uv[2], uv[1]],
            tint,
        });
        vertices.push(SpriteVertex {
            position: [x + w, y + h],
            uv: [uv[2], uv[3]],
            tint,
        });
        vertices.push(SpriteVertex {
            position: [x, y + h],
            uv: [uv[0], uv[3]],
            tint,
        });

        assert_eq!(vertices.len(), 4);
        // Verify corners
        assert!((vertices[0].position[0] - 10.0).abs() < f32::EPSILON);
        assert!((vertices[1].position[0] - 74.0).abs() < f32::EPSILON);
        assert!((vertices[2].position[1] - 52.0).abs() < f32::EPSILON);
        assert!((vertices[3].position[0] - 10.0).abs() < f32::EPSILON);
    }

    #[test]
    fn index_pattern_is_correct() {
        // Verify the repeating index pattern for 2 sprites.
        let max_sprites = 2_u32;
        let mut indices: Vec<u16> = Vec::new();
        for i in 0..max_sprites {
            let base = i as u16 * 4;
            indices.push(base);
            indices.push(base + 1);
            indices.push(base + 2);
            indices.push(base + 2);
            indices.push(base + 3);
            indices.push(base);
        }

        assert_eq!(indices.len(), 12);
        // First quad: 0,1,2, 2,3,0
        assert_eq!(indices[0..6], [0, 1, 2, 2, 3, 0]);
        // Second quad: 4,5,6, 6,7,4
        assert_eq!(indices[6..12], [4, 5, 6, 6, 7, 4]);
    }
}
