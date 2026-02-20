//! Collision AABB-AABB — MGE Démo Pathfinding
//! Hitbox rectangulaire (cube 2D), détection et résolution MTV (Minimum Translation Vector)

use crate::grid::Vec2;

/// Hitbox AABB : centre + demi-taille (half_size)
#[derive(Debug, Clone, Copy)]
pub struct HitboxAABB {
    pub center: Vec2,
    pub half_size: f32,
}

impl HitboxAABB {
    pub fn new(center: Vec2, half_size: f32) -> Self {
        Self { center, half_size }
    }

    fn left(&self) -> f32 {
        self.center.x - self.half_size
    }
    fn right(&self) -> f32 {
        self.center.x + self.half_size
    }
    fn top(&self) -> f32 {
        self.center.y - self.half_size
    }
    fn bottom(&self) -> f32 {
        self.center.y + self.half_size
    }

    /// Teste si un point est à l'intérieur de la hitbox
    pub fn contains_point(&self, point: Vec2) -> bool {
        point.x >= self.left()
            && point.x <= self.right()
            && point.y >= self.top()
            && point.y <= self.bottom()
    }

    /// Teste l'intersection AABB-AABB
    pub fn intersects(&self, other: &HitboxAABB) -> bool {
        self.left() < other.right()
            && other.left() < self.right()
            && self.top() < other.bottom()
            && other.top() < self.bottom()
    }

    /// Calcule le MTV pour séparer self et other.
    /// Retourne le vecteur à appliquer à other pour le pousser hors de self.
    pub fn mtv_toward_other(&self, other: &HitboxAABB) -> Option<Vec2> {
        let overlap_left = self.right() - other.left();
        let overlap_right = other.right() - self.left();
        let overlap_top = self.bottom() - other.top();
        let overlap_bottom = other.bottom() - self.top();
        if overlap_left <= 0.0 || overlap_right <= 0.0 || overlap_top <= 0.0 || overlap_bottom <= 0.0 {
            return None;
        }
        let pen_x = overlap_left.min(overlap_right);
        let pen_y = overlap_top.min(overlap_bottom);
        let (mtv_x, mtv_y) = if pen_x < pen_y {
            let sign = if other.center.x > self.center.x { 1.0 } else { -1.0 };
            (pen_x * sign, 0.0)
        } else {
            let sign = if other.center.y > self.center.y { 1.0 } else { -1.0 };
            (0.0, pen_y * sign)
        };
        Some(Vec2::new(mtv_x, mtv_y))
    }
}

/// Résout la collision AABB entre deux entités en repoussant `follower_pos` hors de `player_pos`.
/// Réponse Block : le suiveur est déplacé pour ne plus chevaucher le joueur.
pub fn resolve_aabb_collision(
    player_pos: &Vec2,
    follower_pos: &mut Vec2,
    half_size: f32,
) -> bool {
    let a = HitboxAABB::new(*player_pos, half_size);
    let b = HitboxAABB::new(*follower_pos, half_size);
    if let Some(mtv) = a.mtv_toward_other(&b) {
        follower_pos.x += mtv.x;
        follower_pos.y += mtv.y;
        true
    } else {
        false
    }
}
