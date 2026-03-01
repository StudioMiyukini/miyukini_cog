// @id: MGE-Collision-Collider @do: implement @role: back-end @layer: 2 @human: miyuk

//! Collider -- shape + layer + mask for collision detection.
//!
//! A `Collider` wraps an AABB with layer/mask filtering. Trigger colliders
//! detect overlaps without physics resolution.

use mge_math::{Aabb, Vec2};

use crate::layer::CollisionLayer;

/// A collision shape associated with an entity.
///
/// Combines an axis-aligned bounding box with layer/mask filtering.
/// If `is_trigger` is `true`, intersections are reported but no
/// physics resolution (push-out) should be applied.
#[derive(Debug, Clone, Copy)]
pub struct Collider {
    /// Axis-aligned bounding box in world space.
    pub aabb: Aabb,
    /// Which layer(s) this collider belongs to.
    pub layer: CollisionLayer,
    /// Which layer(s) this collider interacts with.
    pub mask: CollisionLayer,
    /// Trigger-only flag: detect overlap, skip physics resolution.
    pub is_trigger: bool,
}

impl Collider {
    /// Creates a standard (non-trigger) collider.
    pub fn new(aabb: Aabb, layer: CollisionLayer, mask: CollisionLayer) -> Self {
        Self {
            aabb,
            layer,
            mask,
            is_trigger: false,
        }
    }

    /// Creates a trigger collider that only detects overlaps.
    ///
    /// The mask is set to [`CollisionLayer::ALL`] so it detects all layers.
    pub fn trigger(aabb: Aabb, layer: CollisionLayer) -> Self {
        Self {
            aabb,
            layer,
            mask: CollisionLayer::ALL,
            is_trigger: true,
        }
    }

    /// Returns `true` if this collider intersects `other`, considering
    /// both AABB geometry and layer/mask compatibility.
    ///
    /// Two colliders intersect when:
    /// 1. Their AABBs overlap (strict, edges touching do not count).
    /// 2. `self.mask` contains at least one bit from `other.layer` AND
    ///    `other.mask` contains at least one bit from `self.layer`.
    pub fn intersects(&self, other: &Self) -> bool {
        // Layer/mask filter: both must accept each other
        let layer_ok = self.mask.contains(other.layer) && other.mask.contains(self.layer);
        layer_ok && self.aabb.intersects(other.aabb)
    }
}

/// A detected collision pair with a minimal penetration vector.
#[derive(Debug, Clone, Copy)]
pub struct CollisionPair {
    /// First entity ID.
    pub entity_a: u32,
    /// Second entity ID.
    pub entity_b: u32,
    /// Minimal penetration vector to resolve the overlap.
    ///
    /// Pushing `entity_a` by this vector (or `entity_b` by its negation)
    /// separates the two AABBs on the axis of least penetration.
    pub overlap: Vec2,
}
