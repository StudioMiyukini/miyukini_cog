// @id: MGE-Collision-Rich-Circle @do: circle-collider @role: back-end @layer: 2 @human: miyuk

//! Circle collider -- defined by center and radius.

use mge_math::{Aabb, Vec2};

/// A circle defined by center and radius.
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct CircleCollider {
    /// Center of the circle in world space.
    pub center: Vec2,
    /// Radius of the circle (must be >= 0).
    pub radius: f32,
}

impl CircleCollider {
    /// Creates a new circle collider.
    pub fn new(center: Vec2, radius: f32) -> Self {
        Self { center, radius }
    }

    /// Returns true when this circle overlaps the given AABB.
    ///
    /// Uses the clamped nearest-point test: find the closest point inside the
    /// AABB to the circle center, then check if the distance is within radius.
    pub fn overlaps_aabb(&self, aabb: &Aabb) -> bool {
        let nearest_x = self.center.x.clamp(aabb.min.x, aabb.max.x);
        let nearest_y = self.center.y.clamp(aabb.min.y, aabb.max.y);
        let dx = self.center.x - nearest_x;
        let dy = self.center.y - nearest_y;
        dx * dx + dy * dy <= self.radius * self.radius
    }

    /// Returns true when the point is inside this circle.
    pub fn contains_point(&self, p: Vec2) -> bool {
        let dx = p.x - self.center.x;
        let dy = p.y - self.center.y;
        dx * dx + dy * dy <= self.radius * self.radius
    }

    /// Returns the AABB that tightly bounds this circle.
    pub fn bounding_aabb(&self) -> Aabb {
        Aabb::new(
            Vec2::new(self.center.x - self.radius, self.center.y - self.radius),
            Vec2::new(self.center.x + self.radius, self.center.y + self.radius),
        )
    }
}

/// Circle vs circle overlap test (free function).
///
/// Returns true if the distance between centers is less than or equal to
/// the sum of the radii. Handles zero-radius circles correctly (degenerates
/// to point-in-circle or point-point coincidence).
pub fn circle_circle_intersect(a: &CircleCollider, b: &CircleCollider) -> bool {
    let dx = a.center.x - b.center.x;
    let dy = a.center.y - b.center.y;
    let dist_sq = dx * dx + dy * dy;
    let sum_r = a.radius + b.radius;
    dist_sq <= sum_r * sum_r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_overlaps_aabb_inside() {
        // Circle centered inside the AABB -- must overlap.
        let circle = CircleCollider::new(Vec2::new(5.0, 5.0), 1.0);
        let aabb = Aabb::new(Vec2::new(0.0, 0.0), Vec2::new(10.0, 10.0));
        assert!(circle.overlaps_aabb(&aabb));
    }

    #[test]
    fn test_circle_not_overlaps_aabb() {
        // Circle far away from the AABB -- no overlap.
        let circle = CircleCollider::new(Vec2::new(100.0, 100.0), 1.0);
        let aabb = Aabb::new(Vec2::new(0.0, 0.0), Vec2::new(10.0, 10.0));
        assert!(!circle.overlaps_aabb(&aabb));
    }

    #[test]
    fn test_circle_overlaps_aabb_edge() {
        // Circle just touching the corner of the AABB.
        // AABB corner is at (10, 10). Circle center at (11, 10) with radius 1
        // => distance to corner = 1.0, which equals radius => overlap (<=).
        let circle = CircleCollider::new(Vec2::new(11.0, 10.0), 1.0);
        let aabb = Aabb::new(Vec2::new(0.0, 0.0), Vec2::new(10.0, 10.0));
        assert!(circle.overlaps_aabb(&aabb));
    }

    #[test]
    fn test_circle_contains_point() {
        let circle = CircleCollider::new(Vec2::new(5.0, 5.0), 3.0);
        // Point inside
        assert!(circle.contains_point(Vec2::new(5.0, 5.0)));
        assert!(circle.contains_point(Vec2::new(6.0, 5.0)));
        // Point outside
        assert!(!circle.contains_point(Vec2::new(50.0, 50.0)));
    }

    #[test]
    fn test_bounding_aabb() {
        let circle = CircleCollider::new(Vec2::new(5.0, 5.0), 2.0);
        let bb = circle.bounding_aabb();
        assert!((bb.min.x - 3.0).abs() < f32::EPSILON);
        assert!((bb.min.y - 3.0).abs() < f32::EPSILON);
        assert!((bb.max.x - 7.0).abs() < f32::EPSILON);
        assert!((bb.max.y - 7.0).abs() < f32::EPSILON);
    }

    #[test]
    fn circle_circle_overlap() {
        // Close centers: distance=3, sum_radii=7 => overlap.
        let a = CircleCollider::new(Vec2::new(0.0, 0.0), 5.0);
        let b = CircleCollider::new(Vec2::new(3.0, 0.0), 2.0);
        assert!(circle_circle_intersect(&a, &b));
    }

    #[test]
    fn circle_circle_apart() {
        // Far apart: distance=100, sum_radii=3 => no overlap.
        let a = CircleCollider::new(Vec2::new(0.0, 0.0), 1.0);
        let b = CircleCollider::new(Vec2::new(100.0, 0.0), 2.0);
        assert!(!circle_circle_intersect(&a, &b));
    }

    #[test]
    fn circle_circle_zero_radius() {
        // Both radii zero, same center => overlap (point coincidence).
        let a = CircleCollider::new(Vec2::new(5.0, 5.0), 0.0);
        let b = CircleCollider::new(Vec2::new(5.0, 5.0), 0.0);
        assert!(circle_circle_intersect(&a, &b));

        // Both radii zero, different centers => no overlap.
        let c = CircleCollider::new(Vec2::new(0.0, 0.0), 0.0);
        let d = CircleCollider::new(Vec2::new(1.0, 0.0), 0.0);
        assert!(!circle_circle_intersect(&c, &d));
    }

    #[test]
    fn circle_circle_touching() {
        // Exactly touching: distance=5, sum_radii=5 => overlap (<= boundary).
        let a = CircleCollider::new(Vec2::new(0.0, 0.0), 3.0);
        let b = CircleCollider::new(Vec2::new(5.0, 0.0), 2.0);
        assert!(circle_circle_intersect(&a, &b));
    }
}
