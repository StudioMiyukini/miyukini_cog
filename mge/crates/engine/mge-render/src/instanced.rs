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
}
