// @id: MGE-Collision-Rich @do: rich-collision @role: back-end @layer: 2 @human: miyuk

//! Rich collision shapes: circles, capsules, OBB, and shape intersection tests.
//!
//! Extends `mge-collision` (AABB broadphase) with narrowphase primitives:
//! - [`CircleCollider`] -- circle defined by center + radius
//! - [`CapsuleCollider`] -- segment expanded by a radius
//! - [`ObbCollider`] -- oriented bounding box with SAT overlap test
//! - [`ShapeIntersect`] -- stateless cross-shape intersection queries
//! - [`UnifiedCollider`] -- enum wrapping all shape types for broadphase/narrowphase dispatch
#![deny(unsafe_code)]

use mge_math::{Aabb, Vec2};

pub mod circle;
pub mod capsule;
pub mod obb;
pub mod intersect;
pub mod error;

pub use circle::CircleCollider;
pub use capsule::CapsuleCollider;
pub use obb::ObbCollider;
pub use intersect::ShapeIntersect;
pub use error::CollisionRichError;

/// Unified collider wrapping all shape types for broadphase/narrowphase dispatch.
///
/// Enables a single collision pipeline to handle heterogeneous collider types
/// by providing a common [`bounding_aabb`](UnifiedCollider::bounding_aabb) method
/// for broadphase culling, regardless of the underlying shape.
#[derive(Debug, Clone, Copy)]
pub enum UnifiedCollider {
    /// Axis-aligned bounding box (min/max corners).
    Aabb {
        /// Lower-left corner (minimum coordinates).
        min: Vec2,
        /// Upper-right corner (maximum coordinates).
        max: Vec2,
    },
    /// Circle defined by center + radius.
    Circle(CircleCollider),
    /// Capsule (segment + radius).
    Capsule(CapsuleCollider),
    /// Oriented bounding box.
    Obb(ObbCollider),
}

impl UnifiedCollider {
    /// Returns the axis-aligned bounding box that tightly encloses this collider.
    ///
    /// Used for broadphase culling before narrowphase intersection tests.
    pub fn bounding_aabb(&self) -> Aabb {
        match self {
            Self::Aabb { min, max } => Aabb::new(*min, *max),

            Self::Circle(c) => c.bounding_aabb(),

            Self::Capsule(cap) => {
                let min_x = cap.a.x.min(cap.b.x) - cap.radius;
                let min_y = cap.a.y.min(cap.b.y) - cap.radius;
                let max_x = cap.a.x.max(cap.b.x) + cap.radius;
                let max_y = cap.a.y.max(cap.b.y) + cap.radius;
                Aabb::new(Vec2::new(min_x, min_y), Vec2::new(max_x, max_y))
            }

            Self::Obb(obb) => {
                let verts = obb.to_vertices();
                let mut min_x = verts[0].x;
                let mut min_y = verts[0].y;
                let mut max_x = verts[0].x;
                let mut max_y = verts[0].y;
                for v in &verts[1..] {
                    min_x = min_x.min(v.x);
                    min_y = min_y.min(v.y);
                    max_x = max_x.max(v.x);
                    max_y = max_y.max(v.y);
                }
                Aabb::new(Vec2::new(min_x, min_y), Vec2::new(max_x, max_y))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f32 = 0.001;

    fn approx_eq(a: f32, b: f32) -> bool {
        (a - b).abs() < EPSILON
    }

    #[test]
    fn unified_aabb_bounding_box() {
        let collider = UnifiedCollider::Aabb {
            min: Vec2::new(0.0, 0.0),
            max: Vec2::new(10.0, 10.0),
        };
        let bb = collider.bounding_aabb();
        assert!(approx_eq(bb.min.x, 0.0));
        assert!(approx_eq(bb.min.y, 0.0));
        assert!(approx_eq(bb.max.x, 10.0));
        assert!(approx_eq(bb.max.y, 10.0));
    }

    #[test]
    fn unified_circle_bounding_box() {
        let collider = UnifiedCollider::Circle(
            CircleCollider::new(Vec2::new(5.0, 5.0), 3.0),
        );
        let bb = collider.bounding_aabb();
        // center (5,5), radius 3 -> AABB (2,2)-(8,8)
        assert!(approx_eq(bb.min.x, 2.0));
        assert!(approx_eq(bb.min.y, 2.0));
        assert!(approx_eq(bb.max.x, 8.0));
        assert!(approx_eq(bb.max.y, 8.0));
    }

    #[test]
    fn unified_capsule_bounding_box_horizontal() {
        // Horizontal capsule from (0,0) to (10,0) with radius 2
        let collider = UnifiedCollider::Capsule(
            CapsuleCollider::new(Vec2::new(0.0, 0.0), Vec2::new(10.0, 0.0), 2.0),
        );
        let bb = collider.bounding_aabb();
        // Expected: min (-2,-2), max (12, 2)
        assert!(approx_eq(bb.min.x, -2.0));
        assert!(approx_eq(bb.min.y, -2.0));
        assert!(approx_eq(bb.max.x, 12.0));
        assert!(approx_eq(bb.max.y, 2.0));
    }

    #[test]
    fn unified_capsule_bounding_box_diagonal() {
        // Diagonal capsule from (1,1) to (4,5) with radius 1
        let collider = UnifiedCollider::Capsule(
            CapsuleCollider::new(Vec2::new(1.0, 1.0), Vec2::new(4.0, 5.0), 1.0),
        );
        let bb = collider.bounding_aabb();
        // min.x = min(1,4) - 1 = 0, min.y = min(1,5) - 1 = 0
        // max.x = max(1,4) + 1 = 5, max.y = max(1,5) + 1 = 6
        assert!(approx_eq(bb.min.x, 0.0));
        assert!(approx_eq(bb.min.y, 0.0));
        assert!(approx_eq(bb.max.x, 5.0));
        assert!(approx_eq(bb.max.y, 6.0));
    }

    #[test]
    fn unified_obb_bounding_box_no_rotation() {
        // OBB at (5,5) with half-extents (3,2), no rotation
        // Corners: (2,3), (8,3), (8,7), (2,7)
        let collider = UnifiedCollider::Obb(
            ObbCollider::new(Vec2::new(5.0, 5.0), Vec2::new(3.0, 2.0), 0.0),
        );
        let bb = collider.bounding_aabb();
        assert!(approx_eq(bb.min.x, 2.0));
        assert!(approx_eq(bb.min.y, 3.0));
        assert!(approx_eq(bb.max.x, 8.0));
        assert!(approx_eq(bb.max.y, 7.0));
    }

    #[test]
    fn unified_obb_bounding_box_rotated_45_degrees() {
        // OBB at origin, half-extents (1,1), rotated 45 degrees
        // The rotated square should have AABB bounds of approximately
        // +/-sqrt(2) on each axis.
        let angle = std::f32::consts::FRAC_PI_4; // 45 degrees
        let collider = UnifiedCollider::Obb(
            ObbCollider::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0), angle),
        );
        let bb = collider.bounding_aabb();
        let sqrt2 = std::f32::consts::SQRT_2;
        assert!(approx_eq(bb.min.x, -sqrt2));
        assert!(approx_eq(bb.min.y, -sqrt2));
        assert!(approx_eq(bb.max.x, sqrt2));
        assert!(approx_eq(bb.max.y, sqrt2));
    }

    #[test]
    fn unified_collider_is_copy() {
        // Verify that UnifiedCollider is Copy (compile-time check).
        let a = UnifiedCollider::Circle(CircleCollider::new(Vec2::new(0.0, 0.0), 1.0));
        let b = a; // Copy
        let _ = a; // Original still usable
        let _ = b;
    }
}
