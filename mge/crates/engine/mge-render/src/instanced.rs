// @id: MGE-Render-Instanced @do: implement @role: back-end @layer: 2 @human: francois

//! Instanced sprite rendering pipeline: storage buffer, texture array, and
//! single-draw-call batching.
//!
//! This module replaces the legacy per-texture render pass approach with a
//! GPU-instanced pipeline that uses:
//!
//! - [`InstanceData`] -- per-sprite data uploaded to a storage buffer.
//! - [`InstancedSpriteBatcher`] -- CPU-side staging for instance data.
//! - [`TextureArray`] -- GPU `texture_2d_array` managing multiple sprite sheets.
//! - [`InstancedSpritePipeline`] -- wgpu render pipeline for instanced draws.

// ---------------------------------------------------------------------------
// InstanceData
// ---------------------------------------------------------------------------

/// Per-instance data for a single sprite, uploaded to a GPU storage buffer.
///
/// The layout is `repr(C)` and exactly 64 bytes to ensure correct alignment
/// in the storage buffer. Fields are ordered to avoid padding.
///
/// The `_pad` field ensures 16-byte alignment of the struct (required by
/// most GPU storage buffer layouts).
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceData {
    /// World-space position `[x, y]` in pixels.
    pub position: [f32; 2],
    /// Sprite dimensions `[width, height]` in pixels.
    pub size: [f32; 2],
    /// UV rectangle `[u_min, v_min, u_max, v_max]` (normalised).
    pub uv_rect: [f32; 4],
    /// RGBA tint colour (pre-multiplied alpha friendly).
    pub tint: [f32; 4],
    /// Index into the texture array (layer).
    pub texture_index: u32,
    /// Depth value for sorting (higher = further back).
    pub z_depth: f32,
    /// Padding to reach 64 bytes (16-byte aligned).
    #[allow(clippy::pub_underscore_fields)]
    pub _pad: [f32; 2],
}

// ---------------------------------------------------------------------------
// InstancedSpriteBatcher
// ---------------------------------------------------------------------------

use crate::errors::RenderError;

/// CPU-side staging buffer for instanced sprite data.
///
/// Collects [`InstanceData`] entries each frame, then exposes them as a
/// contiguous byte slice for upload to a GPU storage buffer. The batcher
/// enforces a maximum capacity to prevent unbounded memory growth.
///
/// # Per-frame usage
///
/// ```text
/// batcher.clear();
/// for sprite in visible_sprites {
///     batcher.push(instance)?;
/// }
/// batcher.sort_by_depth();
/// let bytes = batcher.as_bytes();
/// queue.write_buffer(&storage_buffer, 0, bytes);
/// ```
pub struct InstancedSpriteBatcher {
    /// CPU-side instance data staging area.
    instances: Vec<InstanceData>,
    /// Maximum number of instances this batcher can hold.
    capacity: usize,
}

impl InstancedSpriteBatcher {
    /// Create a new batcher with the given maximum capacity.
    pub fn new(capacity: usize) -> Self {
        Self {
            instances: Vec::with_capacity(capacity),
            capacity,
        }
    }

    /// Remove all instances from the staging buffer.
    pub fn clear(&mut self) {
        self.instances.clear();
    }

    /// Push a sprite instance into the staging buffer.
    ///
    /// Returns `RenderError::BatcherOverflow` if the batcher is full.
    pub fn push(&mut self, instance: InstanceData) -> Result<(), RenderError> {
        if self.instances.len() >= self.capacity {
            return Err(RenderError::BatcherOverflow {
                capacity: self.capacity,
                attempted: self.instances.len() + 1,
            });
        }
        self.instances.push(instance);
        Ok(())
    }

    /// Number of instances currently in the staging buffer.
    pub fn len(&self) -> usize {
        self.instances.len()
    }

    /// Returns `true` when the staging buffer contains no instances.
    pub fn is_empty(&self) -> bool {
        self.instances.is_empty()
    }

    /// Return the instance data as a contiguous byte slice suitable for
    /// GPU buffer upload via `queue.write_buffer`.
    pub fn as_bytes(&self) -> &[u8] {
        bytemuck::cast_slice(&self.instances)
    }

    /// Sort instances by `z_depth` ascending (back-to-front painter order).
    pub fn sort_by_depth(&mut self) {
        self.instances.sort_by(|a, b| {
            a.z_depth
                .partial_cmp(&b.z_depth)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
    }
}

// ---------------------------------------------------------------------------
// TextureArray
// ---------------------------------------------------------------------------

/// GPU-backed 2D texture array for instanced sprite rendering.
///
/// All layers share the same `width x height` dimensions and use
/// `Rgba8UnormSrgb` format. New layers are added via [`add_layer`](Self::add_layer)
/// which writes image data to the next available array slice.
///
/// The texture is created with `max_layers` slices up front. Layers are
/// populated on demand; the GPU texture itself is valid (but uninitialised)
/// for unpopulated layers.
pub struct TextureArray {
    /// The underlying wgpu 2D array texture.
    texture: wgpu::Texture,
    /// Default view spanning all layers.
    view: wgpu::TextureView,
    /// Number of layers currently populated with image data.
    layer_count: u32,
    /// Maximum number of layers this array supports.
    max_layers: u32,
    /// Width of each layer in pixels.
    width: u32,
    /// Height of each layer in pixels.
    height: u32,
}

impl TextureArray {
    /// Create a new texture array on the GPU.
    ///
    /// The texture is immediately usable but layers are empty until
    /// populated via [`add_layer`](Self::add_layer).
    pub fn new(device: &wgpu::Device, width: u32, height: u32, max_layers: u32) -> Self {
        let size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: max_layers,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Instanced Sprite Texture Array"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::D2Array),
            ..Default::default()
        });

        Self {
            texture,
            view,
            layer_count: 0,
            max_layers,
            width,
            height,
        }
    }

    /// Upload RGBA8 image data as the next layer in the array.
    ///
    /// Returns the layer index on success. The `image_data` must contain
    /// exactly `width * height * 4` bytes.
    ///
    /// Returns `RenderError::TextureArrayFull` if all layers are occupied,
    /// or `RenderError::InvalidTextureSize` if the data length is wrong.
    pub fn add_layer(
        &mut self,
        queue: &wgpu::Queue,
        image_data: &[u8],
    ) -> Result<u32, RenderError> {
        if self.layer_count >= self.max_layers {
            return Err(RenderError::TextureArrayFull {
                max_layers: self.max_layers,
            });
        }

        let expected_len = (self.width * self.height * 4) as usize;
        if image_data.len() != expected_len {
            // Infer dimensions from data length for error reporting.
            let got_pixels = image_data.len() / 4;
            return Err(RenderError::InvalidTextureSize {
                expected_w: self.width,
                expected_h: self.height,
                got_w: got_pixels as u32,
                got_h: 1,
            });
        }

        let layer_index = self.layer_count;

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d {
                    x: 0,
                    y: 0,
                    z: layer_index,
                },
                aspect: wgpu::TextureAspect::All,
            },
            image_data,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * self.width),
                rows_per_image: Some(self.height),
            },
            wgpu::Extent3d {
                width: self.width,
                height: self.height,
                depth_or_array_layers: 1,
            },
        );

        self.layer_count += 1;
        Ok(layer_index)
    }

    /// Return a reference to the texture view spanning all layers.
    pub fn view(&self) -> &wgpu::TextureView {
        &self.view
    }

    /// Return the number of populated layers.
    pub fn layer_count(&self) -> u32 {
        self.layer_count
    }
}

// ---------------------------------------------------------------------------
// InstancedSpritePipeline
// ---------------------------------------------------------------------------

/// Vertex for the static unit quad (2 floats: x, y).
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct QuadVertex {
    /// Position in the unit square `[0..1, 0..1]`.
    pub position: [f32; 2],
}

impl QuadVertex {
    /// Vertex buffer layout for the unit quad.
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        const ATTRIBS: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![
            0 => Float32x2,
        ];

        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<QuadVertex>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &ATTRIBS,
        }
    }
}

/// Static unit quad vertices: (0,0), (1,0), (1,1), (0,1).
const QUAD_VERTICES: [QuadVertex; 4] = [
    QuadVertex { position: [0.0, 0.0] },
    QuadVertex { position: [1.0, 0.0] },
    QuadVertex { position: [1.0, 1.0] },
    QuadVertex { position: [0.0, 1.0] },
];

/// Static quad indices: two triangles (0,1,2) and (2,3,0).
const QUAD_INDICES: [u16; 6] = [0, 1, 2, 2, 3, 0];

/// Size of a 4x4 f32 matrix in bytes (used for camera uniform).
const MAT4_SIZE: u64 = 64;

/// GPU render pipeline for instanced sprite drawing.
///
/// Uses a single draw call with GPU instancing: one static unit quad is
/// drawn `N` times, with per-instance data read from a storage buffer.
///
/// Bind group layout:
/// - Group 0: camera uniform (mat4x4, vertex stage)
/// - Group 1: texture_2d_array + sampler (fragment stage)
/// - Group 2: storage buffer of [`InstanceData`] (vertex stage, read-only)
pub struct InstancedSpritePipeline {
    /// Compiled wgpu render pipeline.
    pub render_pipeline: wgpu::RenderPipeline,
    /// Layout for bind group 0 (camera uniform).
    pub camera_bind_group_layout: wgpu::BindGroupLayout,
    /// Layout for bind group 1 (texture array + sampler).
    pub texture_bind_group_layout: wgpu::BindGroupLayout,
    /// Layout for bind group 2 (instance storage buffer).
    pub storage_bind_group_layout: wgpu::BindGroupLayout,
    /// Static vertex buffer (4 vertices of the unit quad).
    pub vertex_buffer: wgpu::Buffer,
    /// Static index buffer (6 indices for 2 triangles).
    pub index_buffer: wgpu::Buffer,
}

/// Create the camera bind group layout (group 0: uniform, vertex stage).
fn create_instanced_camera_bgl(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Instanced Camera BGL"),
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

/// Create the texture array bind group layout (group 1: texture_2d_array + sampler).
fn create_instanced_texture_bgl(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Instanced Texture Array BGL"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2Array,
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

/// Create the instance storage bind group layout (group 2: read-only storage).
fn create_instanced_storage_bgl(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Instanced Storage BGL"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
    })
}

/// Build the wgpu render pipeline for instanced sprite drawing.
fn create_instanced_render_pipeline(
    device: &wgpu::Device,
    format: wgpu::TextureFormat,
    shader: &wgpu::ShaderModule,
    camera_layout: &wgpu::BindGroupLayout,
    texture_layout: &wgpu::BindGroupLayout,
    storage_layout: &wgpu::BindGroupLayout,
) -> wgpu::RenderPipeline {
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Instanced Sprite Pipeline Layout"),
        bind_group_layouts: &[camera_layout, texture_layout, storage_layout],
        push_constant_ranges: &[],
    });

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Instanced Sprite Render Pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: shader,
            entry_point: Some("vs_main"),
            buffers: &[QuadVertex::desc()],
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
            cull_mode: None,
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

/// Create the static unit-quad vertex buffer (mapped at creation).
fn create_quad_vertex_buffer(device: &wgpu::Device) -> wgpu::Buffer {
    let buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Instanced Quad Vertex Buffer"),
        size: (std::mem::size_of::<QuadVertex>() * QUAD_VERTICES.len()) as u64,
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: true,
    });
    buffer
        .slice(..)
        .get_mapped_range_mut()
        .copy_from_slice(bytemuck::cast_slice(&QUAD_VERTICES));
    buffer.unmap();
    buffer
}

/// Create the static unit-quad index buffer (mapped at creation).
fn create_quad_index_buffer(device: &wgpu::Device) -> wgpu::Buffer {
    let buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Instanced Quad Index Buffer"),
        size: (std::mem::size_of::<u16>() * QUAD_INDICES.len()) as u64,
        usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: true,
    });
    buffer
        .slice(..)
        .get_mapped_range_mut()
        .copy_from_slice(bytemuck::cast_slice(&QUAD_INDICES));
    buffer.unmap();
    buffer
}

impl InstancedSpritePipeline {
    /// Create the instanced sprite pipeline.
    ///
    /// # Arguments
    ///
    /// * `device` -- wgpu device.
    /// * `format` -- surface texture format (e.g. `Bgra8UnormSrgb`).
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Instanced Sprite Shader"),
            source: wgpu::ShaderSource::Wgsl(
                include_str!("sprite_instanced.wgsl").into(),
            ),
        });

        let camera_bind_group_layout = create_instanced_camera_bgl(device);
        let texture_bind_group_layout = create_instanced_texture_bgl(device);
        let storage_bind_group_layout = create_instanced_storage_bgl(device);

        let render_pipeline = create_instanced_render_pipeline(
            device,
            format,
            &shader,
            &camera_bind_group_layout,
            &texture_bind_group_layout,
            &storage_bind_group_layout,
        );

        let vertex_buffer = create_quad_vertex_buffer(device);
        let index_buffer = create_quad_index_buffer(device);

        Self {
            render_pipeline,
            camera_bind_group_layout,
            texture_bind_group_layout,
            storage_bind_group_layout,
            vertex_buffer,
            index_buffer,
        }
    }

    /// Execute the instanced draw call.
    ///
    /// # Arguments
    ///
    /// * `encoder` -- command encoder for the current frame.
    /// * `view` -- texture view of the render target.
    /// * `camera_bind_group` -- bind group 0 (camera uniform).
    /// * `texture_bind_group` -- bind group 1 (texture array + sampler).
    /// * `storage_bind_group` -- bind group 2 (instance storage buffer).
    /// * `instance_count` -- number of sprite instances to draw.
    pub fn render(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        camera_bind_group: &wgpu::BindGroup,
        texture_bind_group: &wgpu::BindGroup,
        storage_bind_group: &wgpu::BindGroup,
        instance_count: u32,
    ) {
        if instance_count == 0 {
            return;
        }

        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Instanced Sprite Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        pass.set_pipeline(&self.render_pipeline);
        pass.set_bind_group(0, camera_bind_group, &[]);
        pass.set_bind_group(1, texture_bind_group, &[]);
        pass.set_bind_group(2, storage_bind_group, &[]);
        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        pass.draw_indexed(0..6, 0, 0..instance_count);
    }
}

/// CPU-side texture array layer tracker for unit testing without GPU.
///
/// This mirrors the layer counting logic of [`TextureArray`] but does not
/// require a wgpu device. Used exclusively in tests.
#[cfg(test)]
struct MockTextureArrayTracker {
    layer_count: u32,
    max_layers: u32,
}

#[cfg(test)]
impl MockTextureArrayTracker {
    fn new(max_layers: u32) -> Self {
        Self {
            layer_count: 0,
            max_layers,
        }
    }

    fn add_layer(&mut self) -> Result<u32, RenderError> {
        if self.layer_count >= self.max_layers {
            return Err(RenderError::TextureArrayFull {
                max_layers: self.max_layers,
            });
        }
        let index = self.layer_count;
        self.layer_count += 1;
        Ok(index)
    }

    fn layer_count(&self) -> u32 {
        self.layer_count
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use bytemuck::Zeroable;

    #[test]
    fn instance_data_size_is_64() {
        assert_eq!(std::mem::size_of::<InstanceData>(), 64);
    }

    #[test]
    fn instance_data_is_pod() {
        // bytemuck::bytes_of should not panic for a valid Pod type.
        let data = InstanceData::zeroed();
        let bytes = bytemuck::bytes_of(&data);
        assert_eq!(bytes.len(), 64);
    }

    #[test]
    fn instance_data_default_values() {
        let data = InstanceData::zeroed();
        assert_eq!(data.position, [0.0, 0.0]);
        assert_eq!(data.size, [0.0, 0.0]);
        assert_eq!(data.uv_rect, [0.0, 0.0, 0.0, 0.0]);
        assert_eq!(data.tint, [0.0, 0.0, 0.0, 0.0]);
        assert_eq!(data.texture_index, 0);
        assert!((data.z_depth - 0.0).abs() < f32::EPSILON);
        assert_eq!(data._pad, [0.0, 0.0]);
    }

    // -- InstancedSpriteBatcher tests --------------------------------------

    fn make_instance(z: f32) -> InstanceData {
        InstanceData {
            position: [0.0, 0.0],
            size: [64.0, 32.0],
            uv_rect: [0.0, 0.0, 1.0, 1.0],
            tint: [1.0, 1.0, 1.0, 1.0],
            texture_index: 0,
            z_depth: z,
            _pad: [0.0, 0.0],
        }
    }

    #[test]
    fn batcher_push_and_len() {
        let mut batcher = InstancedSpriteBatcher::new(100);
        for i in 0..10 {
            batcher.push(make_instance(i as f32)).unwrap();
        }
        assert_eq!(batcher.len(), 10);
    }

    #[test]
    fn batcher_clear() {
        let mut batcher = InstancedSpriteBatcher::new(100);
        for i in 0..5 {
            batcher.push(make_instance(i as f32)).unwrap();
        }
        assert!(!batcher.is_empty());
        batcher.clear();
        assert!(batcher.is_empty());
        assert_eq!(batcher.len(), 0);
    }

    #[test]
    fn batcher_overflow() {
        let mut batcher = InstancedSpriteBatcher::new(5);
        for i in 0..5 {
            batcher.push(make_instance(i as f32)).unwrap();
        }
        // The 6th push should fail.
        let result = batcher.push(make_instance(5.0));
        assert!(result.is_err());
        match result {
            Err(RenderError::BatcherOverflow { capacity, attempted }) => {
                assert_eq!(capacity, 5);
                assert_eq!(attempted, 6);
            }
            _ => panic!("expected BatcherOverflow error"),
        }
    }

    #[test]
    fn batcher_sort_by_depth() {
        let mut batcher = InstancedSpriteBatcher::new(10);
        batcher.push(make_instance(3.0)).unwrap();
        batcher.push(make_instance(1.0)).unwrap();
        batcher.push(make_instance(2.0)).unwrap();
        batcher.sort_by_depth();

        let bytes = batcher.as_bytes();
        let sorted: &[InstanceData] = bytemuck::cast_slice(bytes);
        assert!((sorted[0].z_depth - 1.0).abs() < f32::EPSILON);
        assert!((sorted[1].z_depth - 2.0).abs() < f32::EPSILON);
        assert!((sorted[2].z_depth - 3.0).abs() < f32::EPSILON);
    }

    // -- TextureArray (mock tracker) tests ---------------------------------

    #[test]
    fn texture_array_layer_count_increments() {
        let mut tracker = MockTextureArrayTracker::new(8);
        assert_eq!(tracker.layer_count(), 0);

        let idx0 = tracker.add_layer().unwrap();
        assert_eq!(idx0, 0);
        assert_eq!(tracker.layer_count(), 1);

        let idx1 = tracker.add_layer().unwrap();
        assert_eq!(idx1, 1);
        assert_eq!(tracker.layer_count(), 2);

        let idx2 = tracker.add_layer().unwrap();
        assert_eq!(idx2, 2);
        assert_eq!(tracker.layer_count(), 3);
    }

    #[test]
    fn texture_array_exceeds_max_layers() {
        let mut tracker = MockTextureArrayTracker::new(2);
        tracker.add_layer().unwrap();
        tracker.add_layer().unwrap();

        // Third layer should fail.
        let result = tracker.add_layer();
        assert!(result.is_err());
        match result {
            Err(RenderError::TextureArrayFull { max_layers }) => {
                assert_eq!(max_layers, 2);
            }
            _ => panic!("expected TextureArrayFull error"),
        }
    }

    // -- Integration tests (CPU-side, no GPU) ------------------------------

    #[test]
    fn integration_instanced_pipeline_100_instances() {
        let mut batcher = InstancedSpriteBatcher::new(1024);
        for i in 0..100 {
            batcher
                .push(InstanceData {
                    position: [i as f32 * 10.0, i as f32 * 5.0],
                    size: [64.0, 32.0],
                    uv_rect: [0.0, 0.0, 1.0, 1.0],
                    tint: [1.0, 1.0, 1.0, 1.0],
                    texture_index: i % 4,
                    z_depth: i as f32,
                    _pad: [0.0, 0.0],
                })
                .unwrap();
        }

        batcher.sort_by_depth();

        let bytes = batcher.as_bytes();
        // 100 instances * 64 bytes each = 6400 bytes
        assert_eq!(bytes.len(), 6400);
        assert_eq!(batcher.len(), 100);

        // Verify sort order: z_depth should be ascending.
        let data: &[InstanceData] = bytemuck::cast_slice(bytes);
        for i in 1..data.len() {
            assert!(
                data[i].z_depth >= data[i - 1].z_depth,
                "sort broken at index {}: {} < {}",
                i,
                data[i].z_depth,
                data[i - 1].z_depth
            );
        }
    }

    #[test]
    fn integration_frustum_cull_and_batch() {
        use crate::culling::{FrustumCuller, RenderEntity};

        // Create 50 entities spread across a large area.
        let mut entities = Vec::new();
        for i in 0..50 {
            entities.push(RenderEntity {
                id: i,
                position: [i as f32 * 100.0, i as f32 * 50.0],
                size: [32.0, 32.0],
            });
        }

        let mut culler = FrustumCuller::new(256.0);
        culler.rebuild(&entities);

        // Visible rect covers roughly entities at positions (0,0) to (2000, 1000).
        // That's entities with x=0..2000 and y=0..1000, i.e. indices 0..20
        // (since entity i is at x=100*i, y=50*i; y=1000 => i=20).
        let visible = culler.cull([0.0, 0.0, 2000.0, 1000.0]);

        // We expect approximately 20 entities (indices 0 through ~20).
        assert!(
            visible.len() >= 18 && visible.len() <= 25,
            "expected ~20 visible entities, got {}",
            visible.len()
        );

        // Verify they are sorted by Y (position[1]).
        // Since entities are indexed by culler using ID, verify the IDs
        // correspond to ascending Y positions.
        for i in 1..visible.len() {
            let prev_y = entities[visible[i - 1] as usize].position[1];
            let curr_y = entities[visible[i] as usize].position[1];
            assert!(
                curr_y >= prev_y,
                "cull result not sorted: y[{}]={} > y[{}]={}",
                i - 1,
                prev_y,
                i,
                curr_y
            );
        }
    }
}
