// @id: MGE-Collision-World @do: implement @role: back-end @layer: 2 @human: miyuk

//! Collision world -- top-level container for broadphase + narrowphase.
//!
//! Manages all colliders, delegates broadphase to `SpatialGrid`, and
//! performs narrowphase AABB intersection + penetration computation.

use std::collections::HashMap;

use mge_math::{Aabb, Vec2};

use crate::collider::{Collider, CollisionPair};
use crate::grid::SpatialGrid;
use crate::CollisionError;

/// Default cell size matching one Sodomight isometric tile width (64 px).
const DEFAULT_CELL_SIZE: f32 = 64.0;

/// Top-level collision detection container.
///
/// Owns all colliders and rebuilds the spatial grid on each detection pass.
/// Call [`detect_all`](Self::detect_all) each tick to obtain every collision pair.
#[derive(Debug)]
pub struct CollisionWorld {
    /// Entity ID -> Collider mapping.
    colliders: HashMap<u32, Collider>,
}

impl CollisionWorld {
    /// Creates an empty collision world with the default cell size (64.0).
    pub fn new() -> Self {
        Self {
            colliders: HashMap::new(),
        }
    }

    /// Inserts a collider for the given entity.
    ///
    /// If the entity already exists, its collider is replaced.
    pub fn insert(&mut self, entity_id: u32, collider: Collider) {
        self.colliders.insert(entity_id, collider);
    }

    /// Removes a collider from the world.
    ///
    /// Returns `Ok(())` if the entity was found, or an error if it did not exist.
    pub fn remove(&mut self, entity_id: u32) -> Result<(), CollisionError> {
        self.colliders
            .remove(&entity_id)
            .map(|_| ())
            .ok_or(CollisionError::EntityNotFound(entity_id))
    }

    /// Updates the position of an existing collider by translating its AABB
    /// so that its center moves to the given world position.
    ///
    /// Returns an error if the entity does not exist.
    pub fn update_position(&mut self, entity_id: u32, pos: Vec2) -> Result<(), CollisionError> {
        let collider = self
            .colliders
            .get_mut(&entity_id)
            .ok_or(CollisionError::EntityNotFound(entity_id))?;

        let half_w = collider.aabb.width() / 2.0;
        let half_h = collider.aabb.height() / 2.0;
        collider.aabb = Aabb::new(
            Vec2::new(pos.x - half_w, pos.y - half_h),
            Vec2::new(pos.x + half_w, pos.y + half_h),
        );
        Ok(())
    }

    /// Runs broadphase + narrowphase and returns all collision pairs.
    ///
    /// Each pair is reported exactly once (`entity_a < entity_b`).
    /// The overlap vector points from A toward B on the axis of least
    /// penetration.
    pub fn detect_all(&self) -> Vec<CollisionPair> {
        // Rebuild the grid each tick (simple and correct).
        let mut grid = SpatialGrid::new(DEFAULT_CELL_SIZE);
        for (&id, collider) in &self.colliders {
            grid.insert(id, &collider.aabb);
        }

        let mut pairs = Vec::new();
        let mut seen = std::collections::HashSet::new();

        for (&id_a, collider_a) in &self.colliders {
            let candidates = grid.query(&collider_a.aabb);
            for id_b in candidates {
                if id_a >= id_b {
                    continue;
                }
                // De-duplicate: only process each pair once
                let key = (id_a, id_b);
                if !seen.insert(key) {
                    continue;
                }

                if let Some(collider_b) = self.colliders.get(&id_b) {
                    if collider_a.intersects(collider_b) {
                        let overlap = compute_penetration(&collider_a.aabb, &collider_b.aabb);
                        pairs.push(CollisionPair {
                            entity_a: id_a,
                            entity_b: id_b,
                            overlap,
                        });
                    }
                }
            }
        }
        pairs
    }

    /// Queries the broadphase grid for entity IDs whose AABB overlaps the
    /// given area. Does not perform narrowphase filtering.
    pub fn query_area(&self, aabb: &Aabb) -> Vec<u32> {
        // Rebuild temporary grid for the query
        let mut grid = SpatialGrid::new(DEFAULT_CELL_SIZE);
        for (&id, collider) in &self.colliders {
            grid.insert(id, &collider.aabb);
        }

        let candidates = grid.query(aabb);
        // De-duplicate and filter by actual AABB intersection
        let mut result = Vec::new();
        let mut seen = std::collections::HashSet::new();
        for id in candidates {
            if !seen.insert(id) {
                continue;
            }
            if let Some(c) = self.colliders.get(&id) {
                if c.aabb.intersects(*aabb) {
                    result.push(id);
                }
            }
        }
        result
    }

    /// Returns the number of colliders in the world.
    pub fn len(&self) -> usize {
        self.colliders.len()
    }

    /// Returns `true` if the world contains no colliders.
    pub fn is_empty(&self) -> bool {
        self.colliders.is_empty()
    }
}

impl Default for CollisionWorld {
    fn default() -> Self {
        Self::new()
    }
}

/// Computes the minimum penetration vector between two intersecting AABBs.
///
/// The vector is along the axis with the smallest overlap, oriented from
/// `a` toward `b`.
fn compute_penetration(a: &Aabb, b: &Aabb) -> Vec2 {
    let overlap_x_right = a.max.x - b.min.x; // a pushes left
    let overlap_x_left = b.max.x - a.min.x; // a pushes right
    let overlap_y_down = a.max.y - b.min.y; // a pushes up
    let overlap_y_up = b.max.y - a.min.y; // a pushes down

    // Pick the axis and direction with least penetration
    let (pen_x, sign_x) = if overlap_x_right < overlap_x_left {
        (overlap_x_right, -1.0_f32)
    } else {
        (overlap_x_left, 1.0)
    };

    let (pen_y, sign_y) = if overlap_y_down < overlap_y_up {
        (overlap_y_down, -1.0_f32)
    } else {
        (overlap_y_up, 1.0)
    };

    if pen_x < pen_y {
        Vec2::new(sign_x * pen_x, 0.0)
    } else {
        Vec2::new(0.0, sign_y * pen_y)
    }
}
