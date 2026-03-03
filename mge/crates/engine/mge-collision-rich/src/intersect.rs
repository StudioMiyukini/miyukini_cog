// @id: MGE-Collision-Rich-Intersect @do: shape-intersection @role: back-end @layer: 2 @human: miyuk

//! Stateless shape intersection tests between different collider types.

use mge_math::Vec2;

use crate::UnifiedCollider;
use crate::capsule::{self, CapsuleCollider};
use crate::circle::{self, CircleCollider};

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

/// Narrowphase intersection dispatch for [`UnifiedCollider`] pairs.
///
/// Matches on `(a, b)` shape variants and dispatches to the most specific
/// intersection test available. For pairs without a dedicated test, falls
/// back to AABB-vs-AABB overlap using each collider's bounding AABB.
///
/// # AABB overlap semantics
///
/// The AABB fallback uses `<=` (inclusive boundary) to stay consistent with
/// the circle and capsule tests that also treat touching as overlapping.
pub fn narrowphase_intersect(a: &UnifiedCollider, b: &UnifiedCollider) -> bool {
    match (a, b) {
        // --- Circle-Circle ---
        (UnifiedCollider::Circle(ca), UnifiedCollider::Circle(cb)) => {
            circle::circle_circle_intersect(ca, cb)
        }

        // --- Circle-AABB / AABB-Circle ---
        (UnifiedCollider::Circle(c), UnifiedCollider::Aabb { min, max })
        | (UnifiedCollider::Aabb { min, max }, UnifiedCollider::Circle(c)) => {
            c.overlaps_aabb(&mge_math::Aabb::new(*min, *max))
        }

        // --- Circle-Capsule / Capsule-Circle ---
        (UnifiedCollider::Circle(c), UnifiedCollider::Capsule(cap))
        | (UnifiedCollider::Capsule(cap), UnifiedCollider::Circle(c)) => {
            ShapeIntersect::circle_capsule(c, cap)
        }

        // --- Capsule-AABB / AABB-Capsule ---
        (UnifiedCollider::Capsule(cap), UnifiedCollider::Aabb { min, max })
        | (UnifiedCollider::Aabb { min, max }, UnifiedCollider::Capsule(cap)) => {
            capsule::capsule_aabb_intersect(cap, *min, *max)
        }

        // --- Capsule-Capsule ---
        (UnifiedCollider::Capsule(ca), UnifiedCollider::Capsule(cb)) => {
            ShapeIntersect::capsule_capsule(ca, cb)
        }

        // --- AABB-AABB ---
        (UnifiedCollider::Aabb { min: min_a, max: max_a },
         UnifiedCollider::Aabb { min: min_b, max: max_b }) => {
            aabb_overlap_inclusive(*min_a, *max_a, *min_b, *max_b)
        }

        // --- OBB-OBB ---
        (UnifiedCollider::Obb(oa), UnifiedCollider::Obb(ob)) => {
            oa.overlaps_obb(ob)
        }

        // --- Fallback: bounding AABB overlap for any unhandled pair ---
        _ => {
            let bb_a = a.bounding_aabb();
            let bb_b = b.bounding_aabb();
            aabb_overlap_inclusive(bb_a.min, bb_a.max, bb_b.min, bb_b.max)
        }
    }
}

/// Inclusive AABB overlap test (touching edges count as overlapping).
///
/// Differs from [`mge_math::Aabb::intersects`] which uses strict `<`/`>`
/// comparisons. This version uses `<=` for consistency with the circle and
/// capsule overlap semantics where touching = overlapping.
fn aabb_overlap_inclusive(min_a: mge_math::Vec2, max_a: mge_math::Vec2, min_b: mge_math::Vec2, max_b: mge_math::Vec2) -> bool {
    min_a.x <= max_b.x
        && max_a.x >= min_b.x
        && min_a.y <= max_b.y
        && max_a.y >= min_b.y
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

    // --- TASK-150: narrowphase dispatch tests ---

    #[test]
    fn broadphase_narrowphase_circle_aabb() {
        // Circle at (5,5) r=3 vs AABB (3,3)-(7,7) -- circle center inside AABB.
        let circle = UnifiedCollider::Circle(
            CircleCollider::new(Vec2::new(5.0, 5.0), 3.0),
        );
        let aabb = UnifiedCollider::Aabb {
            min: Vec2::new(3.0, 3.0),
            max: Vec2::new(7.0, 7.0),
        };
        assert!(narrowphase_intersect(&circle, &aabb));
        // Symmetric: AABB vs Circle should also work.
        assert!(narrowphase_intersect(&aabb, &circle));
    }

    #[test]
    fn broadphase_no_overlap() {
        // Circle at (0,0) r=1 vs AABB (100,100)-(200,200) -- far apart.
        let circle = UnifiedCollider::Circle(
            CircleCollider::new(Vec2::new(0.0, 0.0), 1.0),
        );
        let aabb = UnifiedCollider::Aabb {
            min: Vec2::new(100.0, 100.0),
            max: Vec2::new(200.0, 200.0),
        };
        assert!(!narrowphase_intersect(&circle, &aabb));
    }

    #[test]
    fn dispatch_circle_circle() {
        // Two circles overlapping: distance=4, sum_radii=6 => overlap.
        let a = UnifiedCollider::Circle(
            CircleCollider::new(Vec2::new(0.0, 0.0), 3.0),
        );
        let b = UnifiedCollider::Circle(
            CircleCollider::new(Vec2::new(4.0, 0.0), 3.0),
        );
        assert!(narrowphase_intersect(&a, &b));
    }

    #[test]
    fn dispatch_aabb_aabb_overlap() {
        let a = UnifiedCollider::Aabb {
            min: Vec2::new(0.0, 0.0),
            max: Vec2::new(5.0, 5.0),
        };
        let b = UnifiedCollider::Aabb {
            min: Vec2::new(3.0, 3.0),
            max: Vec2::new(8.0, 8.0),
        };
        assert!(narrowphase_intersect(&a, &b));
    }

    #[test]
    fn dispatch_aabb_aabb_no_overlap() {
        let a = UnifiedCollider::Aabb {
            min: Vec2::new(0.0, 0.0),
            max: Vec2::new(2.0, 2.0),
        };
        let b = UnifiedCollider::Aabb {
            min: Vec2::new(10.0, 10.0),
            max: Vec2::new(12.0, 12.0),
        };
        assert!(!narrowphase_intersect(&a, &b));
    }

    #[test]
    fn dispatch_capsule_aabb() {
        // Capsule (0,0)-(10,0) r=2 vs AABB (4,-1)-(6,1) -- overlaps.
        let cap = UnifiedCollider::Capsule(
            CapsuleCollider::new(Vec2::new(0.0, 0.0), Vec2::new(10.0, 0.0), 2.0),
        );
        let aabb = UnifiedCollider::Aabb {
            min: Vec2::new(4.0, -1.0),
            max: Vec2::new(6.0, 1.0),
        };
        assert!(narrowphase_intersect(&cap, &aabb));
        // Symmetric
        assert!(narrowphase_intersect(&aabb, &cap));
    }

    #[test]
    fn dispatch_obb_fallback() {
        // OBB vs Circle: should fallback to AABB overlap.
        // OBB at (0,0) half-extents (5,5) no rotation -> AABB (-5,-5)-(5,5)
        // Circle at (3,3) r=1 -> AABB (2,2)-(4,4) -- overlaps.
        let obb = UnifiedCollider::Obb(
            crate::obb::ObbCollider::new(
                Vec2::new(0.0, 0.0),
                Vec2::new(5.0, 5.0),
                0.0,
            ),
        );
        let circle = UnifiedCollider::Circle(
            CircleCollider::new(Vec2::new(3.0, 3.0), 1.0),
        );
        assert!(narrowphase_intersect(&obb, &circle));
    }
}
