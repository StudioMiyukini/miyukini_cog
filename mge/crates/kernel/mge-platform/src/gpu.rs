// @id: MGE-Platform-Gpu @do: gpu-context @role: back-end @layer: 1 @human: miyuk
//! GPU context: wgpu device, queue, surface initialisation.

use std::sync::Arc;

use winit::window::Window;

use crate::PlatformConfig;
use crate::PlatformError;

/// Holds all wgpu GPU state: device, queue, surface, config.
pub struct GpuContext {
    /// The wgpu device.
    pub device: wgpu::Device,
    /// The wgpu queue.
    pub queue: wgpu::Queue,
    /// The rendering surface.
    surface: wgpu::Surface<'static>,
    /// Current surface configuration.
    surface_config: wgpu::SurfaceConfiguration,
    /// The window (Arc for 'static lifetime on surface).
    window: Arc<Window>,
}

impl std::fmt::Debug for GpuContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GpuContext")
            .field("surface_size", &self.surface_size())
            .finish_non_exhaustive()
    }
}

impl GpuContext {
    /// Create a new GPU context from a window.
    /// Uses `pollster::block_on` for the async wgpu init.
    #[allow(clippy::too_many_lines)]
    pub fn new(window: Arc<Window>, config: &PlatformConfig) -> Result<Self, PlatformError> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());

        let surface = instance
            .create_surface(Arc::clone(&window))
            .map_err(|e| PlatformError::GpuInit {
                message: format!("Surface creation failed: {e}"),
            })?;

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..Default::default()
        }))
        .ok_or_else(|| PlatformError::GpuInit {
            message: "No compatible GPU adapter found".to_string(),
        })?;

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("mge-device"),
                ..Default::default()
            },
            None,
        ))
        .map_err(|e| PlatformError::GpuInit {
            message: format!("Device request failed: {e}"),
        })?;

        let size = window.inner_size();
        let width = size.width.max(1);
        let height = size.height.max(1);

        let surface_caps = surface.get_capabilities(&adapter);

        let format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .or(surface_caps.formats.first())
            .copied()
            .ok_or_else(|| PlatformError::GpuInit {
                message: "No surface formats available".to_string(),
            })?;

        let alpha_mode = surface_caps
            .alpha_modes
            .first()
            .copied()
            .unwrap_or(wgpu::CompositeAlphaMode::Auto);

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width,
            height,
            present_mode: if config.vsync {
                wgpu::PresentMode::AutoVsync
            } else {
                wgpu::PresentMode::AutoNoVsync
            },
            desired_maximum_frame_latency: 2,
            alpha_mode,
            view_formats: vec![],
        };

        surface.configure(&device, &surface_config);

        tracing::info!(
            "GPU initialised: {:?}, {}x{}, format={:?}",
            adapter.get_info().name,
            width,
            height,
            format,
        );

        Ok(Self {
            device,
            queue,
            surface,
            surface_config,
            window,
        })
    }

    /// Resize the surface (called on `WindowEvent::Resized`).
    pub fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }
        self.surface_config.width = width;
        self.surface_config.height = height;
        self.surface.configure(&self.device, &self.surface_config);
    }

    /// Ensure the surface matches the current window size.
    ///
    /// Prevents "Suboptimal present" warnings from the Vulkan HAL
    /// when the window has been resized or DPI-changed between frames.
    pub fn ensure_surface_current(&mut self) {
        let size = self.window.inner_size();
        if size.width > 0
            && size.height > 0
            && (size.width != self.surface_config.width
                || size.height != self.surface_config.height)
        {
            self.resize(size.width, size.height);
        }
    }

    /// Get a frame to render into.
    pub fn begin_frame(&self) -> Result<GpuFrame, PlatformError> {
        let output = self
            .surface
            .get_current_texture()
            .map_err(|e| PlatformError::GpuInit {
                message: format!("Failed to get surface texture: {e}"),
            })?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("frame_encoder"),
            });
        Ok(GpuFrame {
            output,
            view,
            encoder,
        })
    }

    /// Current surface dimensions.
    #[must_use]
    pub fn surface_size(&self) -> (u32, u32) {
        (self.surface_config.width, self.surface_config.height)
    }

    /// Reference to the underlying window.
    #[must_use]
    pub fn window(&self) -> &Window {
        &self.window
    }

    /// The texture format used by the surface.
    #[must_use]
    pub fn surface_format(&self) -> wgpu::TextureFormat {
        self.surface_config.format
    }
}

/// A single frame being rendered.
pub struct GpuFrame {
    /// The surface texture output for this frame.
    output: wgpu::SurfaceTexture,
    /// The texture view for rendering.
    view: wgpu::TextureView,
    /// The command encoder accumulating draw commands.
    encoder: wgpu::CommandEncoder,
}

impl GpuFrame {
    /// Clear the frame with a solid color and present.
    pub fn clear_and_present(self, gpu: &GpuContext, r: f64, g: f64, b: f64) {
        let mut encoder = self.encoder;
        {
            let _pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("clear_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color { r, g, b, a: 1.0 }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
        }
        gpu.queue.submit(std::iter::once(encoder.finish()));
        self.output.present();
    }

    /// Submit commands and present.
    pub fn finish(self, gpu: &GpuContext) {
        gpu.queue.submit(std::iter::once(self.encoder.finish()));
        self.output.present();
    }

    /// Access the texture view for custom render passes.
    #[must_use]
    pub fn view(&self) -> &wgpu::TextureView {
        &self.view
    }

    /// Access the command encoder for custom render passes.
    pub fn encoder_mut(&mut self) -> &mut wgpu::CommandEncoder {
        &mut self.encoder
    }

    /// Decompose the frame into its raw parts for custom render passes.
    ///
    /// Returns `(encoder, view, output)`. The caller is responsible for
    /// submitting the encoder and presenting the output.
    pub fn into_parts(
        self,
    ) -> (
        wgpu::CommandEncoder,
        wgpu::TextureView,
        wgpu::SurfaceTexture,
    ) {
        (self.encoder, self.view, self.output)
    }
}
