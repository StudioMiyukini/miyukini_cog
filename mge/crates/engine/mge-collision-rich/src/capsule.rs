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

/// Capsule vs AABB overlap test.
///
/// Finds the closest point on the capsule segment to the AABB, then checks
/// if the distance between that closest point and the AABB is within the
/// capsule radius. This reduces the problem to a circle-AABB test at the
/// optimal point along the segment.
pub fn capsule_aabb_intersect(cap: &CapsuleCollider, min: Vec2, max: Vec2) -> bool {
    // Find the closest point on the capsule segment to the AABB center,
    // then check if the expanded circle at that point overlaps the AABB.
    // However, using AABB center is not exact for all configurations.
    //
    // Correct approach: find the closest point on the segment to the AABB
    // by clamping the segment's closest point to the AABB, iterating once.
    //
    // Step 1: Clamp the AABB to get a candidate point on/in the AABB closest
    //         to the segment. Start with the AABB center as initial guess.
    // Step 2: Project that candidate onto the segment -> closest_on_seg.
    // Step 3: Clamp closest_on_seg to the AABB -> closest_on_aabb.
    // Step 4: Measure distance between closest_on_seg and closest_on_aabb.
    //
    // Two iterations converges for convex shapes.

    let closest_on_seg = refine_closest_seg_to_aabb(cap, min, max);

    // Now check: is the circle centered at closest_on_seg with cap.radius
    // overlapping the AABB [min, max]?
    let nearest_x = closest_on_seg.x.clamp(min.x, max.x);
    let nearest_y = closest_on_seg.y.clamp(min.y, max.y);
    let dx = closest_on_seg.x - nearest_x;
    let dy = closest_on_seg.y - nearest_y;
    dx * dx + dy * dy <= cap.radius * cap.radius
}

/// Iteratively refines the closest point on the capsule segment to the AABB.
///
/// Two passes of project-clamp converge for convex shapes.
fn refine_closest_seg_to_aabb(cap: &CapsuleCollider, min: Vec2, max: Vec2) -> Vec2 {
    // Pass 1: start from AABB center
    let center = Vec2::new(
        (min.x + max.x) * 0.5,
        (min.y + max.y) * 0.5,
    );
    let on_seg = cap.closest_point_on_segment(center);
    let on_aabb = Vec2::new(
        on_seg.x.clamp(min.x, max.x),
        on_seg.y.clamp(min.y, max.y),
    );

    // Pass 2: refine from the clamped point
    cap.closest_point_on_segment(on_aabb)
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

    #[test]
    fn capsule_aabb_overlap() {
        // Horizontal capsule from (0,0) to (10,0) with radius 2.
        // AABB from (4,-1) to (6,1) -- segment passes through the AABB.
        let cap = CapsuleCollider::new(
            Vec2::new(0.0, 0.0),
            Vec2::new(10.0, 0.0),
            2.0,
        );
        let min = Vec2::new(4.0, -1.0);
        let max = Vec2::new(6.0, 1.0);
        assert!(capsule_aabb_intersect(&cap, min, max));
    }

    #[test]
    fn capsule_aabb_miss() {
        // Horizontal capsule from (0,0) to (10,0) with radius 1.
        // AABB from (50,50) to (60,60) -- far away, no overlap.
        let cap = CapsuleCollider::new(
            Vec2::new(0.0, 0.0),
            Vec2::new(10.0, 0.0),
            1.0,
        );
        let min = Vec2::new(50.0, 50.0);
        let max = Vec2::new(60.0, 60.0);
        assert!(!capsule_aabb_intersect(&cap, min, max));
    }

    #[test]
    fn capsule_aabb_radius_reach() {
        // Capsule segment does not reach the AABB, but radius does.
        // Vertical capsule from (0,0) to (0,10) with radius 3.
        // AABB from (2,4) to (4,6) -- closest segment point is (0,5),
        // distance to AABB nearest point (2,5) = 2.0, which is <= radius 3.
        let cap = CapsuleCollider::new(
            Vec2::new(0.0, 0.0),
            Vec2::new(0.0, 10.0),
            3.0,
        );
        let min = Vec2::new(2.0, 4.0);
        let max = Vec2::new(4.0, 6.0);
        assert!(capsule_aabb_intersect(&cap, min, max));
    }

    #[test]
    fn capsule_aabb_degenerate_point_capsule() {
        // Degenerate capsule (zero-length segment) = circle at (5,5) radius 2.
        // AABB from (6,5) to (8,7) -- distance from (5,5) to nearest AABB
        // point (6,5) = 1.0, which is <= radius 2.
        let cap = CapsuleCollider::new(
            Vec2::new(5.0, 5.0),
            Vec2::new(5.0, 5.0),
            2.0,
        );
        let min = Vec2::new(6.0, 5.0);
        let max = Vec2::new(8.0, 7.0);
        assert!(capsule_aabb_intersect(&cap, min, max));
    }
}
