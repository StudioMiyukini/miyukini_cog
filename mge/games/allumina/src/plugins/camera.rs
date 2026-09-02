//! @id     games_allumina_plugins_camera
//! @do     definir_min_zoom
//! @role   orchestration
//! @layer  core
//! @human  games allumina — module « plugins camera ».

//! Camera that follows the player, with zoom and free-scroll support.

use crate::app::{App, PhaseId, Plugin};
use crate::components::Position2D;
use crate::ecs::World;

const MIN_ZOOM: f32 = 0.5;
const MAX_ZOOM: f32 = 3.0;
/// How quickly the camera eases toward the player (fraction per tick).
const FOLLOW_LERP: f32 = 0.15;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    /// World-space point the camera is centred on.
    pub center: Position2D,
    pub zoom: f32,
    /// Manual pan offset (screen space).
    pub scroll: (f32, f32),
}

impl Default for Camera {
    fn default() -> Self {
        Self { center: Position2D::new(0.0, 0.0), zoom: 1.0, scroll: (0.0, 0.0) }
    }
}

impl Camera {
    pub fn zoom_in(&mut self, step: f32) {
        self.zoom = (self.zoom + step).clamp(MIN_ZOOM, MAX_ZOOM);
    }

    pub fn zoom_out(&mut self, step: f32) {
        self.zoom = (self.zoom - step).clamp(MIN_ZOOM, MAX_ZOOM);
    }

    pub fn pan(&mut self, dx: f32, dy: f32) {
        self.scroll.0 += dx;
        self.scroll.1 += dy;
    }
}

/// Ease the camera toward the player each tick.
pub fn camera_system(world: &mut World) {
    let Some(player) = world.player_entity else { return };
    let Some(&pos) = world.positions.get(&player) else { return };
    let cam = &mut world.camera;
    cam.center.x += (pos.x - cam.center.x) * FOLLOW_LERP;
    cam.center.y += (pos.y - cam.center.y) * FOLLOW_LERP;
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(PhaseId::Camera, camera_system);
    }
}
