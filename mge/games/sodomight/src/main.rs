use anyhow::{Context, Result};
use mge_audio::{AudioBus, AudioCue};
use mge_content::{load_bootstrap, GameBootstrap};
use mge_core::{RuntimeConfig, SceneSummary};
use mge_render::GraphicsState;
use mge_replication::ReplicationPlan;
use mge_save::{PlayerProfile, SaveManager};
use mge_server_core::AuthoritativeSim;
use std::{path::PathBuf, sync::Arc};
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowAttributes, WindowId},
};

struct SodomightApp {
    bootstrap: GameBootstrap,
    save_manager: SaveManager,
    profile: PlayerProfile,
    sim: AuthoritativeSim,
    audio: AudioBus,
    replication: ReplicationPlan,
    window: Option<Arc<Window>>,
    window_id: Option<WindowId>,
    renderer: Option<GraphicsState>,
    fatal_error: Option<anyhow::Error>,
}

impl SodomightApp {
    fn new(bootstrap: GameBootstrap, save_manager: SaveManager) -> Result<Self> {
        let profile = save_manager.load_profile()?.unwrap_or(PlayerProfile {
            display_name: "Wanderer".to_owned(),
            last_scene: bootstrap.startup_scene.id.clone(),
            level: 1,
        });

        let scene = SceneSummary {
            id: bootstrap.startup_scene.id.clone(),
            biome: bootstrap.startup_scene.biome.clone(),
            ambient_rgb: [
                bootstrap.startup_scene.ambient_rgb.r,
                bootstrap.startup_scene.ambient_rgb.g,
                bootstrap.startup_scene.ambient_rgb.b,
            ],
        };
        let config = RuntimeConfig { tick_rate_hz: 60, startup_scene: scene.id.clone() };

        Ok(Self {
            replication: ReplicationPlan::bootstrap(scene.id.clone()),
            sim: AuthoritativeSim::new(config, scene),
            audio: AudioBus::new(bootstrap.audio.listener_bus.clone()),
            bootstrap,
            fatal_error: None,
            profile,
            renderer: None,
            save_manager,
            window: None,
            window_id: None,
        })
    }

    fn set_fatal(&mut self, event_loop: &ActiveEventLoop, error: anyhow::Error) {
        self.fatal_error = Some(error);
        event_loop.exit();
    }

    fn save_progress(&self) {
        let _ = self.save_manager.save_profile(&self.profile);
    }
}

impl ApplicationHandler for SodomightApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        event_loop.set_control_flow(ControlFlow::Poll);
        let size = PhysicalSize::new(self.bootstrap.window.width, self.bootstrap.window.height);
        let attributes: WindowAttributes = Window::default_attributes()
            .with_title(self.bootstrap.window.title.clone())
            .with_inner_size(size);

        let window = match event_loop.create_window(attributes) {
            Ok(window) => Arc::new(window),
            Err(error) => {
                self.set_fatal(event_loop, anyhow::anyhow!("failed to create window: {error}"));
                return;
            }
        };

        let renderer =
            match pollster::block_on(GraphicsState::new(window.clone(), &self.bootstrap.render)) {
                Ok(renderer) => renderer,
                Err(error) => {
                    self.set_fatal(event_loop, error);
                    return;
                }
            };

        self.window_id = Some(window.id());
        self.window = Some(window);
        self.renderer = Some(renderer);
        self.audio.push_cue(AudioCue::TownAmbience);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        if Some(window_id) != self.window_id {
            return;
        }

        match event {
            WindowEvent::CloseRequested => {
                self.save_progress();
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                if let Some(renderer) = &mut self.renderer {
                    renderer.resize(size);
                }
            }
            WindowEvent::RedrawRequested => {
                let _ = self.sim.tick();
                let ticks = u32::try_from(self.sim.runtime().tick_index()).unwrap_or(u32::MAX);
                self.profile.level = ticks / 60 + 1;
                self.profile.last_scene = self.sim.runtime().scene().id.clone();

                if let Some(renderer) = &mut self.renderer {
                    if let Err(error) = renderer.render() {
                        self.set_fatal(event_loop, error);
                    }
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}

fn main() -> Result<()> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let bootstrap_path = manifest_dir.join("data").join("bootstrap.ron");
    let bootstrap = load_bootstrap(&bootstrap_path)?;
    let save_manager = SaveManager::new(&bootstrap.game_id)?;
    let mut app = SodomightApp::new(bootstrap, save_manager)?;
    let event_loop = EventLoop::new().context("failed to create winit event loop")?;

    event_loop.run_app(&mut app).context("failed while running Sodomight app")?;

    if let Some(error) = app.fatal_error {
        return Err(error);
    }

    let _ = app.replication;
    Ok(())
}
