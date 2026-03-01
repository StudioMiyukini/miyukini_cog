// @id: MGE-Collision-Rich-Obb @do: obb-collider @role: back-end @layer: 2 @human: miyuk

//! Oriented Bounding Box (OBB) collider with SAT overlap test.

use mge_math::Vec2;

/// An oriented bounding box defined by center, half-extents, and rotation angle.
///
/// The angle is in radians and rotates the box around its center.
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct ObbCollider {
    /// Center of the OBB in world space.
    pub center: Vec2,
    /// Half-extents along the local X and Y axes (before rotation).
    pub half_extents: Vec2,
    /// Rotation angle in radians.
    pub angle_rad: f32,
}

impl ObbCollider {
    /// Creates a new OBB collider.
    pub fn new(center: Vec2, half_extents: Vec2, angle_rad: f32) -> Self {
        Self {
            center,
            half_extents,
            angle_rad,
        }
    }

    /// Computes the 4 corners of the OBB in world space.
    ///
    /// Returns corners in order: bottom-left, bottom-right, top-right, top-left
    /// (relative to the local frame before rotation).
    pub fn to_vertices(&self) -> [Vec2; 4] {
        let cos = self.angle_rad.cos();
        let sin = self.angle_rad.sin();
        let hw = self.half_extents.x;
        let hh = self.half_extents.y;

        // Local corners: (-hw,-hh), (+hw,-hh), (+hw,+hh), (-hw,+hh)
        let corners_local = [
            Vec2::new(-hw, -hh),
            Vec2::new(hw, -hh),
            Vec2::new(hw, hh),
            Vec2::new(-hw, hh),
        ];

        // Rotate and translate each corner
        corners_local.map(|c| Vec2::new(
            self.center.x + c.x * cos - c.y * sin,
            self.center.y + c.x * sin + c.y * cos,
        ))
    }

    /// Projects all 4 vertices onto the axis `(nx, ny)` and returns `(min, max)`.
    pub fn project_onto_axis(&self, nx: f32, ny: f32) -> (f32, f32) {
        let verts = self.to_vertices();
        let mut min_proj = f32::MAX;
        let mut max_proj = f32::MIN;
        for v in &verts {
            let proj = v.x * nx + v.y * ny;
            if proj < min_proj {
                min_proj = proj;
            }
            if proj > max_proj {
                max_proj = proj;
            }
        }
        (min_proj, max_proj)
    }

    /// SAT (Separating Axis Theorem) test against another OBB.
    ///
    /// Returns true if the two OBBs overlap.
    pub fn overlaps_obb(&self, other: &ObbCollider) -> bool {
        // 4 potential separating axes: 2 from each OBB's local frame
        let axes = [
            Vec2::new(self.angle_rad.cos(), self.angle_rad.sin()),
            Vec2::new(-self.angle_rad.sin(), self.angle_rad.cos()),
            Vec2::new(other.angle_rad.cos(), other.angle_rad.sin()),
            Vec2::new(-other.angle_rad.sin(), other.angle_rad.cos()),
        ];

        for axis in &axes {
            let (min_a, max_a) = self.project_onto_axis(axis.x, axis.y);
            let (min_b, max_b) = other.project_onto_axis(axis.x, axis.y);
            if max_a < min_b || max_b < min_a {
                return false; // Separating axis found
            }
        }
        true // No separating axis found -- overlap
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_obb_vertices_no_rotation() {
        // OBB at origin, half-extents (2, 3), no rotation.
        let obb = ObbCollider::new(Vec2::new(0.0, 0.0), Vec2::new(2.0, 3.0), 0.0);
        let verts = obb.to_vertices();

        // Expected: (-2,-3), (2,-3), (2,3), (-2,3)
        assert!((verts[0].x - (-2.0)).abs() < 0.001);
        assert!((verts[0].y - (-3.0)).abs() < 0.001);
        assert!((verts[1].x - 2.0).abs() < 0.001);
        assert!((verts[1].y - (-3.0)).abs() < 0.001);
        assert!((verts[2].x - 2.0).abs() < 0.001);
        assert!((verts[2].y - 3.0).abs() < 0.001);
        assert!((verts[3].x - (-2.0)).abs() < 0.001);
        assert!((verts[3].y - 3.0).abs() < 0.001);
    }

    #[test]
    fn test_obb_overlaps_same() {
        // An OBB overlaps with an identical copy of itself.
        let obb = ObbCollider::new(Vec2::new(5.0, 5.0), Vec2::new(2.0, 3.0), 0.5);
        assert!(obb.overlaps_obb(&obb));
    }
}
