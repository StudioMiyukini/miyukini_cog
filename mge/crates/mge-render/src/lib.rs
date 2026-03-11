pub mod animation;
pub mod pipeline;
pub mod atlas;
pub mod batch;
pub mod camera;
pub mod lighting;
pub mod pass;
pub mod quality;
pub mod telemetry;
pub mod vfx;

use anyhow::{anyhow, Context, Result};
use mge_content::RenderProfile;
use std::sync::Arc;
use wgpu::{Color, SurfaceError};
use winit::{dpi::PhysicalSize, window::Window};

#[derive(Debug, Clone, Copy)]
pub struct FrameStats {
    pub width: u32,
    pub height: u32,
    pub frames_rendered: u64,
}

pub struct GraphicsState {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    clear_color: Color,
    frames_rendered: u64,
    sprite_pipeline: pipeline::SpritePipeline,
}

impl GraphicsState {
    pub async fn new(window: Arc<Window>, profile: &RenderProfile) -> Result<Self> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let surface = instance.create_surface(window).context("failed to create wgpu surface")?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .context("failed to request adapter")?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("mge.device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                memory_hints: wgpu::MemoryHints::Performance,
                trace: wgpu::Trace::Off,
            })
            .await
            .context("failed to request device")?;

        let caps = surface.get_capabilities(&adapter);
        let format = caps
            .formats
            .iter()
            .copied()
            .find(wgpu::TextureFormat::is_srgb)
            .or_else(|| caps.formats.first().copied())
            .ok_or_else(|| anyhow!("surface exposes no compatible format"))?;
        let present_mode = caps
            .present_modes
            .iter()
            .copied()
            .find(|mode| matches!(mode, wgpu::PresentMode::AutoVsync))
            .unwrap_or(wgpu::PresentMode::Fifo);
        let alpha_mode =
            caps.alpha_modes.first().copied().unwrap_or(wgpu::CompositeAlphaMode::Opaque);

        let size = PhysicalSize::new(1280, 720);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width,
            height: size.height,
            present_mode,
            alpha_mode,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        let clear_color = Color {
            r: profile.clear_rgba.r,
            g: profile.clear_rgba.g,
            b: profile.clear_rgba.b,
            a: profile.clear_rgba.a,
        };

        let sprite_pipeline = pipeline::SpritePipeline::new(&device, &queue, format);

        Ok(Self { surface, device, queue, config, clear_color, frames_rendered: 0, sprite_pipeline })
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        if size.width == 0 || size.height == 0 {
            return;
        }

        self.config.width = size.width;
        self.config.height = size.height;
        self.surface.configure(&self.device, &self.config);
    }

    pub fn render(&mut self, batch: &batch::SpriteBatch) -> Result<()> {
        let output = match self.surface.get_current_texture() {
            Ok(frame) => frame,
            Err(SurfaceError::Lost | SurfaceError::Outdated) => {
                self.surface.configure(&self.device, &self.config);
                return Ok(());
            }
            Err(SurfaceError::Timeout | SurfaceError::Other) => return Ok(()),
            Err(SurfaceError::OutOfMemory) => return Err(anyhow!("surface out of memory")),
        };

        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("mge.frame_encoder"),
        });

        // Clear pass
        {
            let _pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("mge.clear_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.clear_color),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
        }

        // Sprite pass (after clear — uses LoadOp::Load)
        #[allow(clippy::cast_precision_loss)]
        let viewport = [self.config.width as f32, self.config.height as f32];
        self.sprite_pipeline.render(&self.queue, &mut encoder, &view, batch, viewport);

        self.queue.submit([encoder.finish()]);
        output.present();
        self.frames_rendered += 1;
        Ok(())
    }

    pub fn frame_stats(&self) -> FrameStats {
        FrameStats {
            width: self.config.width,
            height: self.config.height,
            frames_rendered: self.frames_rendered,
        }
    }
}
