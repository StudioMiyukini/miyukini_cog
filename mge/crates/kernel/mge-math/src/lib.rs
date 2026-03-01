// @id: MGE-Math-Lib @do: module-root @role: back-end @layer: 1 @human: miyuk

//! mge-math -- coordonnees isometriques dimetric 2:1, vecteurs, AABB, interpolation.
//!
//! Ce crate fournit les primitives mathematiques de base pour le moteur MGE.
//! Les formules isometriques suivent le standard Sodomight (clone D2) :
//!   - Tile 64x32 pixels (ratio 2:1)
//!   - `screen_x = (tile_x - tile_y) * 32`
//!   - `screen_y = (tile_x + tile_y) * 16`
#![deny(unsafe_code)]

// clippy::all + clippy::pedantic are configured in workspace lints (Cargo.toml)

pub mod iso;
pub mod vec2;
pub mod aabb;
pub mod interp;

pub use iso::{IsoCoord, WorldPos, screen_to_world, TILE_W, TILE_H};
pub use vec2::Vec2;
pub use aabb::Aabb;
pub use interp::{lerp, lerp_f32};

#[cfg(test)]
mod tests {
    use crate::iso::{IsoCoord, WorldPos, screen_to_world};
    use crate::vec2::Vec2;
    use crate::aabb::Aabb;
    use crate::interp::{lerp, smoothstep};

    // =======================================================================
    // Isometric coordinates
    // =======================================================================

    #[test]
    fn test_iso_to_screen_origin() {
        let (sx, sy) = IsoCoord::new(0, 0).to_screen();
        assert!((sx).abs() < f32::EPSILON);
        assert!((sy).abs() < f32::EPSILON);
    }

    #[test]
    fn test_iso_to_screen_tx1_ty0() {
        // tx=1, ty=0  ->  sx=32, sy=16
        let (sx, sy) = IsoCoord::new(1, 0).to_screen();
        assert!((sx - 32.0).abs() < f32::EPSILON);
        assert!((sy - 16.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_iso_to_screen_tx0_ty1() {
        // tx=0, ty=1  ->  sx=-32, sy=16
        let (sx, sy) = IsoCoord::new(0, 1).to_screen();
        assert!((sx - (-32.0)).abs() < f32::EPSILON);
        assert!((sy - 16.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_iso_from_screen_roundtrip() {
        let orig = IsoCoord::new(3, 5);
        let (sx, sy) = orig.to_screen();
        let back = IsoCoord::from_screen(sx, sy);
        assert_eq!(orig, back);
    }

    #[test]
    fn test_iso_z_order() {
        let a = IsoCoord::new(2, 3);
        let b = IsoCoord::new(1, 1);
        assert!(a.z_order() > b.z_order());
    }

    // =======================================================================
    // WorldPos
    // =======================================================================

    #[test]
    fn test_world_pos_to_screen_origin() {
        let (sx, sy) = WorldPos::zero().to_screen();
        assert!((sx).abs() < f32::EPSILON);
        assert!((sy).abs() < f32::EPSILON);
    }

    #[test]
    fn test_screen_to_world_roundtrip() {
        let pos = WorldPos::new(3.0, 5.0);
        let (sx, sy) = pos.to_screen();
        let back = screen_to_world(sx, sy);
        assert!((back.x - pos.x).abs() < 0.001);
        assert!((back.y - pos.y).abs() < 0.001);
    }

    #[test]
    fn test_screen_to_world_roundtrip_fractional() {
        let pos = WorldPos::new(3.5, 7.2);
        let (sx, sy) = pos.to_screen();
        let back = screen_to_world(sx, sy);
        assert!((back.x - pos.x).abs() < 0.001);
        assert!((back.y - pos.y).abs() < 0.001);
    }

    #[test]
    fn test_world_pos_distance() {
        let a = WorldPos::new(0.0, 0.0);
        let b = WorldPos::new(3.0, 4.0);
        assert!((a.distance(b) - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_world_pos_lerp() {
        let a = WorldPos::new(0.0, 0.0);
        let b = WorldPos::new(10.0, 20.0);
        let mid = a.lerp(b, 0.5);
        assert!((mid.x - 5.0).abs() < f32::EPSILON);
        assert!((mid.y - 10.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_world_pos_from_iso() {
        let coord = IsoCoord::new(4, 7);
        let pos = WorldPos::from_iso(coord);
        assert!((pos.x - 4.0).abs() < f32::EPSILON);
        assert!((pos.y - 7.0).abs() < f32::EPSILON);
    }

    // =======================================================================
    // Vec2
    // =======================================================================

    #[test]
    fn test_vec2_ops() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(3.0, 4.0);
        let c = a + b;
        assert!((c.x - 4.0).abs() < f32::EPSILON);
        assert!((c.y - 6.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_vec2_sub() {
        let a = Vec2::new(5.0, 7.0);
        let b = Vec2::new(2.0, 3.0);
        let c = a - b;
        assert!((c.x - 3.0).abs() < f32::EPSILON);
        assert!((c.y - 4.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_vec2_mul_scalar() {
        let v = Vec2::new(3.0, 4.0);
        let s = v * 2.0;
        assert!((s.x - 6.0).abs() < f32::EPSILON);
        assert!((s.y - 8.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_vec2_length() {
        let v = Vec2::new(3.0, 4.0);
        assert!((v.length() - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_vec2_length_sq() {
        let v = Vec2::new(3.0, 4.0);
        assert!((v.length_sq() - 25.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_vec2_normalized() {
        let v = Vec2::new(3.0, 4.0).normalized();
        assert!((v.length() - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_vec2_normalized_zero() {
        let v = Vec2::zero().normalized();
        assert!((v.x).abs() < f32::EPSILON);
        assert!((v.y).abs() < f32::EPSILON);
    }

    #[test]
    fn test_vec2_dot() {
        let a = Vec2::new(1.0, 0.0);
        let b = Vec2::new(0.0, 1.0);
        assert!((a.dot(b)).abs() < f32::EPSILON); // perpendicular
    }

    #[test]
    fn test_vec2_lerp() {
        let a = Vec2::new(0.0, 0.0);
        let b = Vec2::new(10.0, 10.0);
        let mid = a.lerp(b, 0.25);
        assert!((mid.x - 2.5).abs() < f32::EPSILON);
        assert!((mid.y - 2.5).abs() < f32::EPSILON);
    }

    // =======================================================================
    // AABB
    // =======================================================================

    #[test]
    fn test_aabb_intersects() {
        let a = Aabb::new(Vec2::new(0.0, 0.0), Vec2::new(2.0, 2.0));
        let b = Aabb::new(Vec2::new(1.0, 1.0), Vec2::new(3.0, 3.0));
        let c = Aabb::new(Vec2::new(5.0, 5.0), Vec2::new(7.0, 7.0));
        assert!(a.intersects(b));
        assert!(!a.intersects(c));
    }

    #[test]
    fn test_aabb_no_intersect_touching_edge() {
        // Touching at edge (max.x == other.min.x) is NOT intersecting (strict < / >)
        let a = Aabb::new(Vec2::new(0.0, 0.0), Vec2::new(2.0, 2.0));
        let b = Aabb::new(Vec2::new(2.0, 0.0), Vec2::new(4.0, 2.0));
        assert!(!a.intersects(b));
    }

    #[test]
    fn test_aabb_contains() {
        let a = Aabb::new(Vec2::new(0.0, 0.0), Vec2::new(10.0, 10.0));
        assert!(a.contains(Vec2::new(5.0, 5.0)));
        assert!(a.contains(Vec2::new(0.0, 0.0))); // boundary inclusive
        assert!(!a.contains(Vec2::new(-1.0, 5.0)));
    }

    #[test]
    fn test_aabb_from_center_half() {
        let a = Aabb::from_center_half(Vec2::new(5.0, 5.0), Vec2::new(2.0, 3.0));
        assert!((a.min.x - 3.0).abs() < f32::EPSILON);
        assert!((a.min.y - 2.0).abs() < f32::EPSILON);
        assert!((a.max.x - 7.0).abs() < f32::EPSILON);
        assert!((a.max.y - 8.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_aabb_dimensions() {
        let a = Aabb::new(Vec2::new(1.0, 2.0), Vec2::new(5.0, 8.0));
        assert!((a.width() - 4.0).abs() < f32::EPSILON);
        assert!((a.height() - 6.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_aabb_center() {
        let a = Aabb::new(Vec2::new(0.0, 0.0), Vec2::new(10.0, 6.0));
        let c = a.center();
        assert!((c.x - 5.0).abs() < f32::EPSILON);
        assert!((c.y - 3.0).abs() < f32::EPSILON);
    }

    // =======================================================================
    // Interpolation
    // =======================================================================

    #[test]
    fn test_lerp_boundaries() {
        assert!((lerp(0.0, 10.0, 0.0) - 0.0).abs() < f32::EPSILON);
        assert!((lerp(0.0, 10.0, 1.0) - 10.0).abs() < f32::EPSILON);
        assert!((lerp(0.0, 10.0, 0.5) - 5.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_smoothstep_boundaries() {
        assert!((smoothstep(0.0, 1.0, -0.5) - 0.0).abs() < f32::EPSILON);
        assert!((smoothstep(0.0, 1.0, 1.5) - 1.0).abs() < f32::EPSILON);
        assert!((smoothstep(0.0, 1.0, 0.5) - 0.5).abs() < f32::EPSILON);
    }
}
