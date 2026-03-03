// @id: MGE-Platform-App @do: app-runner @role: back-end @layer: 1 @human: miyuk
//! Application runner: `GameApp` trait + winit event loop integration.

use std::sync::Arc;

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::PhysicalKey;
use winit::window::{Window, WindowId};

use crate::gpu::GpuContext;
use crate::input::InputEvent;
use crate::input_map;
use crate::{PlatformConfig, PlatformError};

/// Trait implemented by the game application.
pub trait GameApp {
    /// Called once after GPU initialisation.
    fn on_init(&mut self, gpu: &GpuContext);
    /// Called each frame (handle ticks + render).
    fn on_frame(&mut self, gpu: &mut GpuContext);
    /// Called for each mapped input event.
    fn on_input(&mut self, event: InputEvent);
}

/// Launch the application event loop. This blocks until exit.
///
/// # Errors
///
/// Returns `PlatformError::EventLoop` if the event loop cannot be created
/// or encounters a fatal error during execution.
pub fn run(config: PlatformConfig, app: impl GameApp + 'static) -> Result<(), PlatformError> {
    let event_loop = EventLoop::new().map_err(|e| PlatformError::EventLoop {
        message: e.to_string(),
    })?;
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut runner = AppRunner {
        config,
        game: app,
        window: None,
        gpu: None,
    };

    event_loop
        .run_app(&mut runner)
        .map_err(|e| PlatformError::EventLoop {
            message: e.to_string(),
        })
}

/// Internal winit `ApplicationHandler` that bridges winit events to `GameApp`.
struct AppRunner<G> {
    /// Platform configuration for window and GPU setup.
    config: PlatformConfig,
    /// The user-provided game application.
    game: G,
    /// The window handle (created on `resumed`).
    window: Option<Arc<Window>>,
    /// The GPU context (created on `resumed`).
    gpu: Option<GpuContext>,
}

impl<G: GameApp> ApplicationHandler for AppRunner<G> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return; // Already initialised (can happen on some platforms).
        }

        let attrs = Window::default_attributes()
            .with_title(&self.config.title)
            .with_inner_size(winit::dpi::LogicalSize::new(
                self.config.width,
                self.config.height,
            ));

        let window = match event_loop.create_window(attrs) {
            Ok(w) => Arc::new(w),
            Err(e) => {
                tracing::error!("Window creation failed: {e}");
                event_loop.exit();
                return;
            }
        };

        match GpuContext::new(Arc::clone(&window), &self.config) {
            Ok(gpu) => {
                self.game.on_init(&gpu);
                self.gpu = Some(gpu);
            }
            Err(e) => {
                tracing::error!("GPU init failed: {e}");
                event_loop.exit();
                return;
            }
        }

        self.window = Some(window);
    }

    #[allow(clippy::too_many_lines)]
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                tracing::info!("Window close requested");
                event_loop.exit();
            }

            WindowEvent::Resized(size) => {
                if let Some(ref mut gpu) = self.gpu {
                    gpu.resize(size.width, size.height);
                }
            }

            WindowEvent::RedrawRequested => {
                if let Some(ref mut gpu) = self.gpu {
                    // Reconfigure surface if window size changed (prevents
                    // "Suboptimal present" spam from Vulkan HAL).
                    gpu.ensure_surface_current();
                    self.game.on_frame(gpu);
                }
            }

            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(code) = event.physical_key {
                    let key = input_map::map_key_code(code);
                    let input_event = if event.state.is_pressed() {
                        InputEvent::KeyDown { key }
                    } else {
                        InputEvent::KeyUp { key }
                    };
                    self.game.on_input(input_event);
                }
            }

            WindowEvent::CursorMoved { position, .. } => {
                self.game.on_input(InputEvent::MouseMove {
                    x: position.x,
                    y: position.y,
                });
            }

            WindowEvent::MouseInput { state, button, .. } => {
                let btn = input_map::map_mouse_button(button);
                self.game.on_input(InputEvent::MouseButtonEvent {
                    button: btn,
                    pressed: state.is_pressed(),
                });
            }

            WindowEvent::MouseWheel { delta, .. } => {
                #[allow(clippy::cast_possible_truncation)]
                let d = match delta {
                    winit::event::MouseScrollDelta::LineDelta(_, y) => y,
                    winit::event::MouseScrollDelta::PixelDelta(pos) => pos.y as f32,
                };
                self.game.on_input(InputEvent::MouseWheel { delta: d });
            }

            WindowEvent::Focused(focused) => {
                self.game.on_input(InputEvent::WindowFocus { focused });
            }

            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(ref window) = self.window {
            window.request_redraw();
        }
    }
}
