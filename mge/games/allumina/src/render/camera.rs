//! @id     games_allumina_render_camera
//! @do     definir_camera2_d
//! @role   ui
//! @layer  core
//! @human  games allumina — module « render camera ».

//! Render-side camera: turns world tile coordinates into screen pixels and
//! back, applying the follow-camera centre, zoom, and viewport.
//!
//! The *gameplay* camera (follow easing, zoom clamps, pan) is the
//! [`crate::plugins::camera::Camera`] resource updated each tick. This module
//! is the pure projection math the renderer and input picking share, layered
//! on the base isometric transform in [`crate::plugins::isometric`].

use crate::components::Position2D;
use crate::plugins::isometric::{screen_to_world, world_to_screen};

/// A snapshot of the camera used for one frame's projection.
#[derive(Debug, Clone, Copy)]
pub struct Camera2D {
    /// World-space tile the camera is centred on.
    pub center: Position2D,
    /// Zoom multiplier (1.0 = 1:1 pixels).
    pub zoom: f32,
    /// Viewport size in physical pixels.
    pub viewport: [f32; 2],
    /// Free-running render clock (seconds) for cosmetic animation (tile/portal
    /// pulses, item levitation) of things that have no per-entity controller.
    pub time: f32,
}

impl Camera2D {
    pub fn new(center: Position2D, zoom: f32, viewport: [f32; 2]) -> Self {
        Self { center, zoom, viewport, time: 0.0 }
    }

    /// Attach the render clock to this snapshot.
    #[must_use]
    pub fn with_time(mut self, time: f32) -> Self {
        self.time = time;
        self
    }

    /// World tile position → screen pixel (top-left origin).
    #[must_use]
    pub fn world_to_screen(&self, p: Position2D) -> [f32; 2] {
        let (px, py) = world_to_screen(p);
        let (cx, cy) = world_to_screen(self.center);
        [
            (px - cx) * self.zoom + self.viewport[0] * 0.5,
            (py - cy) * self.zoom + self.viewport[1] * 0.5,
        ]
    }

    /// Screen pixel → world tile position (inverse of [`Self::world_to_screen`]).
    #[must_use]
    pub fn screen_to_world(&self, sx: f32, sy: f32) -> Position2D {
        let (cx, cy) = world_to_screen(self.center);
        let rel_x = (sx - self.viewport[0] * 0.5) / self.zoom + cx;
        let rel_y = (sy - self.viewport[1] * 0.5) / self.zoom + cy;
        screen_to_world(rel_x, rel_y)
    }

    /// Whether a screen-space point lies within the viewport (with margin).
    #[must_use]
    pub fn on_screen(&self, p: [f32; 2], margin: f32) -> bool {
        p[0] >= -margin
            && p[1] >= -margin
            && p[0] <= self.viewport[0] + margin
            && p[1] <= self.viewport[1] + margin
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn screen_world_round_trip() {
        let cam = Camera2D::new(Position2D::new(5.0, 7.0), 1.5, [1280.0, 720.0]);
        let p = Position2D::new(8.0, 3.0);
        let s = cam.world_to_screen(p);
        let back = cam.screen_to_world(s[0], s[1]);
        assert!((back.x - p.x).abs() < 1e-2, "x {} vs {}", back.x, p.x);
        assert!((back.y - p.y).abs() < 1e-2, "y {} vs {}", back.y, p.y);
    }

    #[test]
    fn center_maps_to_viewport_centre() {
        let cam = Camera2D::new(Position2D::new(2.0, 2.0), 1.0, [800.0, 600.0]);
        let s = cam.world_to_screen(cam.center);
        assert!((s[0] - 400.0).abs() < 1e-3);
        assert!((s[1] - 300.0).abs() < 1e-3);
    }
}
