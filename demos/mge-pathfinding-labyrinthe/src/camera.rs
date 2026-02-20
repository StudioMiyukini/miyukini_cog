//! Caméra centrée sur le joueur — MGE Démo Pathfinding

use crate::grid::Vec2;

pub struct Camera {
    pub center: Vec2,
}

impl Camera {
    pub fn new(center: Vec2) -> Self {
        Self { center }
    }

    /// Convertit une position monde en position écran
    pub fn world_to_screen(&self, world_pos: Vec2, screen_w: usize, screen_h: usize) -> (i32, i32) {
        let sx = world_pos.x - self.center.x + (screen_w as f32 / 2.0);
        let sy = world_pos.y - self.center.y + (screen_h as f32 / 2.0);
        (sx as i32, sy as i32)
    }

    /// Convertit une position écran en position monde
    pub fn screen_to_world(&self, screen_x: f32, screen_y: f32, screen_w: usize, screen_h: usize) -> Vec2 {
        Vec2::new(
            screen_x - (screen_w as f32 / 2.0) + self.center.x,
            screen_y - (screen_h as f32 / 2.0) + self.center.y,
        )
    }

    /// Limites visibles en coordonnées monde (min_x, min_y, max_x, max_y)
    pub fn visible_bounds(&self, screen_w: usize, screen_h: usize) -> (f32, f32, f32, f32) {
        let half_w = screen_w as f32 / 2.0;
        let half_h = screen_h as f32 / 2.0;
        (
            self.center.x - half_w,
            self.center.y - half_h,
            self.center.x + half_w,
            self.center.y + half_h,
        )
    }
}
