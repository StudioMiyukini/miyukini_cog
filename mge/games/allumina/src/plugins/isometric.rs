//! @id     games_allumina_plugins_isometric
//! @do     definir_tile_w
//! @role   orchestration
//! @layer  core
//! @human  games allumina — module « plugins isometric ».

//! Isometric projection. World tile coordinates map to screen pixels via the
//! classic 2:1 diamond transform.

use crate::app::{App, PhaseId, Plugin};
use crate::components::Position2D;
use crate::ecs::World;

/// Tile width in screen pixels.
pub const TILE_W: f32 = 64.0;
/// Tile height in screen pixels.
pub const TILE_H: f32 = 32.0;

/// World → screen: `screen_x = (x - y) * TILE_W/2`, `screen_y = (x + y) * TILE_H/2`.
pub fn world_to_screen(p: Position2D) -> (f32, f32) {
    ((p.x - p.y) * (TILE_W / 2.0), (p.x + p.y) * (TILE_H / 2.0))
}

/// Inverse projection: screen pixels back to world tile coordinates.
pub fn screen_to_world(sx: f32, sy: f32) -> Position2D {
    let hx = sx / (TILE_W / 2.0);
    let hy = sy / (TILE_H / 2.0);
    Position2D::new((hx + hy) / 2.0, (hy - hx) / 2.0)
}

/// Project the camera-relative screen position of an entity tile.
pub fn world_to_screen_camera(p: Position2D, camera_center: Position2D) -> (f32, f32) {
    let (px, py) = world_to_screen(p);
    let (cx, cy) = world_to_screen(camera_center);
    (px - cx, py - cy)
}

/// Headless "render": periodically log the player's projected screen position
/// (stands in for a real wgpu/macroquad draw pass).
pub fn render_system(world: &mut World) {
    if world.tick % 90 != 0 {
        return;
    }
    let Some(player) = world.player_entity else { return };
    let Some(&pos) = world.positions.get(&player) else { return };
    let (sx, sy) = world_to_screen_camera(pos, world.camera.center);
    world.log_event(format!(
        "[render] player tile ({:.1},{:.1}) -> screen ({sx:.0},{sy:.0}) zoom {:.1}",
        pos.x, pos.y, world.camera.zoom
    ));
}

pub struct IsometricPlugin;

impl Plugin for IsometricPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(PhaseId::Render, render_system);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn origin_maps_to_origin() {
        let (x, y) = world_to_screen(Position2D::new(0.0, 0.0));
        assert!((x).abs() < f32::EPSILON && (y).abs() < f32::EPSILON);
    }

    #[test]
    fn round_trip() {
        let p = Position2D::new(3.0, 5.0);
        let (sx, sy) = world_to_screen(p);
        let back = screen_to_world(sx, sy);
        assert!((back.x - p.x).abs() < 1e-3);
        assert!((back.y - p.y).abs() < 1e-3);
    }
}
