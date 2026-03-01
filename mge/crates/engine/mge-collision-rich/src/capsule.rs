// @id: MGE-Collision-Rich-Capsule @do: capsule-collider @role: back-end @layer: 2 @human: miyuk

//! Capsule collider -- a line segment expanded by a radius.

use mge_math::Vec2;

/// A capsule: a line segment (a -> b) expanded by `radius` in all directions.
///
/// Useful for swept collision and elongated characters/projectiles.
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct CapsuleCollider {
    /// Start endpoint of the capsule segment.
    pub a: Vec2,
    /// End endpoint of the capsule segment.
    pub b: Vec2,
    /// Expansion radius around the segment.
    pub radius: f32,
}

impl CapsuleCollider {
    /// Creates a new capsule collider.
    pub fn new(a: Vec2, b: Vec2, radius: f32) -> Self {
        Self { a, b, radius }
    }

    /// Length of the capsule segment (distance from a to b).
    pub fn length(&self) -> f32 {
        let dx = self.b.x - self.a.x;
        let dy = self.b.y - self.a.y;
        (dx * dx + dy * dy).sqrt()
    }

    /// Normalized direction from a to b.
    ///
    /// Returns `Vec2::zero()` if the segment is degenerate (zero length).
    pub fn direction(&self) -> Vec2 {
        let len = self.length();
        if len < f32::EPSILON {
            return Vec2::zero();
        }
        Vec2::new(
            (self.b.x - self.a.x) / len,
            (self.b.y - self.a.y) / len,
        )
    }

    /// Closest point on the segment [a, b] to the given point p.
    ///
    /// The result is clamped to the segment endpoints.
    pub fn closest_point_on_segment(&self, p: Vec2) -> Vec2 {
        let ab_x = self.b.x - self.a.x;
        let ab_y = self.b.y - self.a.y;
        let len_sq = ab_x * ab_x + ab_y * ab_y;
        if len_sq < f32::EPSILON {
            return self.a;
        }
        let t = ((p.x - self.a.x) * ab_x + (p.y - self.a.y) * ab_y) / len_sq;
        let t = t.clamp(0.0, 1.0);
        Vec2::new(
            self.a.x + t * ab_x,
            self.a.y + t * ab_y,
        )
    }

    /// Returns true when point p is inside this capsule.
    ///
    /// A point is inside if its distance to the nearest point on the segment
    /// is less than or equal to the capsule radius.
    pub fn contains_point(&self, p: Vec2) -> bool {
        let closest = self.closest_point_on_segment(p);
        let dx = p.x - closest.x;
        let dy = p.y - closest.y;
        dx * dx + dy * dy <= self.radius * self.radius
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capsule_length() {
        let cap = CapsuleCollider::new(
            Vec2::new(0.0, 0.0),
            Vec2::new(3.0, 4.0),
            1.0,
        );
        assert!((cap.length() - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_closest_point_on_segment_middle() {
        // Point directly above the midpoint of a horizontal segment
        let cap = CapsuleCollider::new(
            Vec2::new(0.0, 0.0),
            Vec2::new(10.0, 0.0),
            1.0,
        );
        let closest = cap.closest_point_on_segment(Vec2::new(5.0, 5.0));
        assert!((closest.x - 5.0).abs() < 0.001);
        assert!((closest.y - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_closest_point_clamped_to_a() {
        // Point far before endpoint a -- should clamp to a.
        let cap = CapsuleCollider::new(
            Vec2::new(0.0, 0.0),
            Vec2::new(10.0, 0.0),
            1.0,
        );
        let closest = cap.closest_point_on_segment(Vec2::new(-100.0, 0.0));
        assert!((closest.x - 0.0).abs() < 0.001);
        assert!((closest.y - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_closest_point_clamped_to_b() {
        // Point far past endpoint b -- should clamp to b.
        let cap = CapsuleCollider::new(
            Vec2::new(0.0, 0.0),
            Vec2::new(10.0, 0.0),
            1.0,
        );
        let closest = cap.closest_point_on_segment(Vec2::new(200.0, 0.0));
        assert!((closest.x - 10.0).abs() < 0.001);
        assert!((closest.y - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_capsule_contains_point() {
        let cap = CapsuleCollider::new(
            Vec2::new(0.0, 0.0),
            Vec2::new(10.0, 0.0),
            2.0,
        );
        // Point on the segment -- inside
        assert!(cap.contains_point(Vec2::new(5.0, 0.0)));
        // Point within radius of the segment -- inside
        assert!(cap.contains_point(Vec2::new(5.0, 1.5)));
        // Point outside the capsule
        assert!(!cap.contains_point(Vec2::new(5.0, 50.0)));
    }
}
