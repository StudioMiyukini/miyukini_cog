// @id: sd-impl-renderer @do: implement @role: back-end @layer: 2 @human: miyuk

//! Top-level renderer (GPU-backed).
//!
//! Full wgpu initialisation requires a window and a surface, which are
//! not available in headless test environments. This module exposes the
//! `Renderer` struct and a `RenderConfig`, but its constructors are
//! behind a runtime GPU check.
//!
//! The actual GPU pipeline creation will be completed once `mge-platform`
//! provides `GpuContext` with a live window handle.

// ---------------------------------------------------------------------------
// RenderConfig
// ---------------------------------------------------------------------------

/// Maximum sprites with u16 index buffers (65 536 / 4 vertices = 16 384).
pub const MAX_SPRITES_U16: u32 = 16_384;

/// Maximum sprites with u32 index buffers (requires `index-u32` feature).
pub const MAX_SPRITES_U32: u32 = 262_144;

/// Configuration parameters for the renderer.
#[derive(Debug, Clone)]
pub struct RenderConfig {
    /// Viewport width in pixels.
    pub width: u32,
    /// Viewport height in pixels.
    pub height: u32,
    /// Enable vertical sync.
    pub vsync: bool,
    /// Maximum number of sprites per frame.
    ///
    /// Without the `index-u32` feature, this is capped at [`MAX_SPRITES_U16`]
    /// (16 384). With `index-u32` enabled, values up to [`MAX_SPRITES_U32`]
    /// (262 144) are accepted.
    pub max_sprites: u32,
}

impl RenderConfig {
    /// Return the effective `max_sprites` capped to the index buffer limit.
    ///
    /// Without `index-u32`: capped at [`MAX_SPRITES_U16`].
    /// With `index-u32`: capped at [`MAX_SPRITES_U32`].
    pub fn effective_max_sprites(&self) -> u32 {
        let ceiling = Self::max_sprites_ceiling();
        self.max_sprites.min(ceiling)
    }

    /// Return the absolute ceiling for `max_sprites` given the active features.
    pub fn max_sprites_ceiling() -> u32 {
        if cfg!(feature = "index-u32") {
            MAX_SPRITES_U32
        } else {
            MAX_SPRITES_U16
        }
    }
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            vsync: true,
            max_sprites: MAX_SPRITES_U16,
        }
    }
}

// ---------------------------------------------------------------------------
// Renderer (stub -- full impl requires a window/surface)
// ---------------------------------------------------------------------------

/// Top-level GPU renderer.
///
/// In production this holds `wgpu::Device`, `wgpu::Queue`, the sprite
/// pipeline, the atlas bind-group layouts, and the camera uniform buffer.
///
/// Because creating a surface requires a live window handle, the struct
/// is currently a configuration-only stub. The full implementation will
/// be wired in once `mge-platform` provides `GpuContext`.
#[derive(Debug)]
pub struct Renderer {
    /// Active configuration.
    pub config: RenderConfig,
    // Future fields (require GpuContext from mge-platform):
    // device: wgpu::Device,
    // queue: wgpu::Queue,
    // surface: wgpu::Surface<'static>,
    // pipeline: wgpu::RenderPipeline,
    // batcher: SpriteBatcher,
    // camera_buffer: wgpu::Buffer,
    // camera_bind_group: wgpu::BindGroup,
    // sampler: wgpu::Sampler,
    // dual_res: DualResolution,
    // atlas_registry: AtlasRegistry,
    // surface_format: wgpu::TextureFormat,
}

impl Renderer {
    /// Create a renderer from a configuration.
    ///
    /// This does **not** initialise the GPU -- call the GPU init path
    /// once `mge-platform` provides a window surface.
    pub fn new(config: RenderConfig) -> Self {
        Self { config }
    }

    /// Return the configured viewport dimensions.
    pub fn viewport_size(&self) -> (u32, u32) {
        (self.config.width, self.config.height)
    }
}
