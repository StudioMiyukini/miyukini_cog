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
}
