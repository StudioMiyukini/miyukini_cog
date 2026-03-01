// @id: MGE-Math-Aabb @do: implement @role: back-end @layer: 1 @human: miyuk

//! AABB -- Axis-Aligned Bounding Box.
//!
//! Utilise pour les collisions broad-phase et le frustum culling du renderer.
//! Les axes sont alignes sur l'espace ecran (pas sur l'espace isometrique).

use crate::vec2::Vec2;

/// Boite englobante alignee sur les axes.
///
/// `min` est le coin inferieur gauche, `max` est le coin superieur droit.
/// L'intersection est stricte (les bords qui se touchent ne comptent pas).
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct Aabb {
    /// Coin inferieur gauche (coordonnees minimales).
    pub min: Vec2,
    /// Coin superieur droit (coordonnees maximales).
    pub max: Vec2,
}

impl Aabb {
    /// Cree une AABB a partir de ses coins min et max.
    pub fn new(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }

    /// Cree une AABB a partir d'un centre et de demi-extensions.
    ///
    /// `half` represente la moitie de la largeur et de la hauteur.
    pub fn from_center_half(center: Vec2, half: Vec2) -> Self {
        Self {
            min: Vec2::new(center.x - half.x, center.y - half.y),
            max: Vec2::new(center.x + half.x, center.y + half.y),
        }
    }

    /// Largeur de la boite.
    pub fn width(self) -> f32 {
        self.max.x - self.min.x
    }

    /// Hauteur de la boite.
    pub fn height(self) -> f32 {
        self.max.y - self.min.y
    }

    /// Centre de la boite.
    pub fn center(self) -> Vec2 {
        Vec2::new(
            (self.min.x + self.max.x) / 2.0,
            (self.min.y + self.max.y) / 2.0,
        )
    }

    /// Test d'intersection AABB vs AABB (strict : les bords qui se touchent ne comptent pas).
    pub fn intersects(self, other: Self) -> bool {
        self.min.x < other.max.x
            && self.max.x > other.min.x
            && self.min.y < other.max.y
            && self.max.y > other.min.y
    }

    /// Teste si un point est contenu dans la boite (inclusif sur les bords).
    pub fn contains(self, p: Vec2) -> bool {
        p.x >= self.min.x && p.x <= self.max.x && p.y >= self.min.y && p.y <= self.max.y
    }

    /// Fusionne deux AABB en une AABB englobante.
    pub fn merge(self, other: Self) -> Self {
        Self {
            min: Vec2::new(self.min.x.min(other.min.x), self.min.y.min(other.min.y)),
            max: Vec2::new(self.max.x.max(other.max.x), self.max.y.max(other.max.y)),
        }
    }
}
