// @id: MGE-Collision-Rich-Intersect @do: shape-intersection @role: back-end @layer: 2 @human: miyuk

//! Stateless shape intersection tests between different collider types.

use mge_math::Vec2;

use crate::capsule::CapsuleCollider;
use crate::circle::CircleCollider;

/// Stateless shape intersection tests.
///
/// All methods are pure functions that take shape references and return a bool.
pub struct ShapeIntersect;

impl ShapeIntersect {
    /// Circle vs circle overlap test.
    ///
    /// Returns true if the distance between centers is less than or equal to
    /// the sum of the radii.
    pub fn circle_circle(a: &CircleCollider, b: &CircleCollider) -> bool {
        let dx = a.center.x - b.center.x;
        let dy = a.center.y - b.center.y;
        let dist_sq = dx * dx + dy * dy;
        let sum_r = a.radius + b.radius;
        dist_sq <= sum_r * sum_r
    }

    /// Circle vs capsule overlap test.
    ///
    /// Returns true if the distance from the circle center to the capsule
    /// segment is less than or equal to the sum of the radii.
    pub fn circle_capsule(c: &CircleCollider, cap: &CapsuleCollider) -> bool {
        let closest = cap.closest_point_on_segment(c.center);
        let dx = c.center.x - closest.x;
        let dy = c.center.y - closest.y;
        let dist_sq = dx * dx + dy * dy;
        let sum_r = c.radius + cap.radius;
        dist_sq <= sum_r * sum_r
    }

    /// Capsule vs capsule overlap test.
    ///
    /// Uses closest-point sampling between the two segments. The test finds the
    /// pair of closest points and checks if their distance is within the sum of
    /// both capsule radii.
    pub fn capsule_capsule(a: &CapsuleCollider, b: &CapsuleCollider) -> bool {
        let (cp_a, cp_b) = closest_points_segment_segment(a, b);
        let dx = cp_a.x - cp_b.x;
        let dy = cp_a.y - cp_b.y;
        let dist_sq = dx * dx + dy * dy;
        let sum_r = a.radius + b.radius;
        dist_sq <= sum_r * sum_r
    }
}

/// Finds the closest pair of points between two capsule segments.
///
/// Samples endpoints and midpoint of each segment, projects onto the other,
/// and returns the pair with minimal distance.
fn closest_points_segment_segment(
    a: &CapsuleCollider,
    b: &CapsuleCollider,
) -> (Vec2, Vec2) {
    let mid_a = Vec2::new(
        (a.a.x + a.b.x) * 0.5,
        (a.a.y + a.b.y) * 0.5,
    );
    let candidates_a = [a.a, a.b, mid_a];

    let mut best_dist_sq = f32::MAX;
    let mut best_pa = a.a;
    let mut best_pb = a.a;

    for &pa in &candidates_a {
        let pb = b.closest_point_on_segment(pa);
        let dx = pa.x - pb.x;
        let dy = pa.y - pb.y;
        let dist_sq = dx * dx + dy * dy;
        if dist_sq < best_dist_sq {
            best_dist_sq = dist_sq;
            best_pa = pa;
            best_pb = pb;
        }
    }

    // Also sample from segment B
    let mid_b = Vec2::new(
        (b.a.x + b.b.x) * 0.5,
        (b.a.y + b.b.y) * 0.5,
    );
    let candidates_b = [b.a, b.b, mid_b];

    for &pb in &candidates_b {
        let pa = a.closest_point_on_segment(pb);
        let dx = pa.x - pb.x;
        let dy = pa.y - pb.y;
        let dist_sq = dx * dx + dy * dy;
        if dist_sq < best_dist_sq {
            best_dist_sq = dist_sq;
            best_pa = pa;
            best_pb = pb;
        }
    }

    (best_pa, best_pb)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_circle_overlap() {
        let a = CircleCollider::new(Vec2::new(0.0, 0.0), 5.0);
        let b = CircleCollider::new(Vec2::new(8.0, 0.0), 5.0);
        // Distance = 8, sum of radii = 10 => overlap.
        assert!(ShapeIntersect::circle_circle(&a, &b));
    }

    #[test]
    fn test_circle_circle_no_overlap() {
        let a = CircleCollider::new(Vec2::new(0.0, 0.0), 2.0);
        let b = CircleCollider::new(Vec2::new(100.0, 0.0), 2.0);
        // Distance = 100, sum of radii = 4 => no overlap.
        assert!(!ShapeIntersect::circle_circle(&a, &b));
    }

    #[test]
    fn test_circle_capsule_overlap() {
        let circle = CircleCollider::new(Vec2::new(5.0, 3.0), 2.0);
        let capsule = CapsuleCollider::new(
            Vec2::new(0.0, 0.0),
            Vec2::new(10.0, 0.0),
            2.0,
        );
        // Closest point on segment to circle center (5,3) is (5,0).
        // Distance = 3, sum of radii = 4 => overlap.
        assert!(ShapeIntersect::circle_capsule(&circle, &capsule));
    }

    #[test]
    fn test_capsule_capsule_overlap() {
        let a = CapsuleCollider::new(
            Vec2::new(0.0, 0.0),
            Vec2::new(10.0, 0.0),
            1.0,
        );
        let b = CapsuleCollider::new(
            Vec2::new(5.0, 1.5),
            Vec2::new(5.0, 10.0),
            1.0,
        );
        // Closest points: (5,0) on a and (5,1.5) on b.
        // Distance = 1.5, sum of radii = 2 => overlap.
        assert!(ShapeIntersect::capsule_capsule(&a, &b));
    }
}
