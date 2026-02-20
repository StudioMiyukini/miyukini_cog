//! Balise de ralliement — Clic droit place la balise, followers s'y dirigent pendant 2.5s

use crate::grid::Vec2;

/// Durée de la balise en secondes
pub const RALLY_DURATION: f32 = 2.5;

#[derive(Debug, Clone)]
pub struct RallyPoint {
    /// Position monde de la balise
    pub position: Vec2,
    /// Temps restant en secondes (0 = inactive)
    pub timer: f32,
}

impl RallyPoint {
    pub fn new(position: Vec2) -> Self {
        Self {
            position,
            timer: RALLY_DURATION,
        }
    }

    /// Retourne true si la balise est active
    pub fn is_active(&self) -> bool {
        self.timer > 0.0
    }

    /// Met à jour le timer. Retourne true si toujours active après la mise à jour.
    pub fn update(&mut self, dt: f32) -> bool {
        if self.timer <= 0.0 {
            return false;
        }
        self.timer -= dt;
        self.timer > 0.0
    }
}
