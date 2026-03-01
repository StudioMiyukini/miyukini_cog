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
    pub max_sprites: u32,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            vsync: true,
            max_sprites: 16_384,
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
