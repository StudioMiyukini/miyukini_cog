// @id: MGE-Collision-Grid @do: implement @role: back-end @layer: 2 @human: miyuk

//! Spatial grid -- uniform-grid broadphase for collision detection.
//!
//! Divides world space into fixed-size cells. Entities are inserted by
//! AABB, potentially spanning multiple cells. Queries return all entity
//! IDs whose cell overlaps the query AABB (candidates, not exact hits).

use std::collections::HashMap;

use mge_math::Aabb;

/// Uniform spatial grid for broadphase collision culling.
///
/// Cell size should match tile size (default 64.0 for Sodomight iso tiles).
/// Entities that span multiple cells are inserted into every overlapping cell.
#[derive(Debug)]
pub struct SpatialGrid {
    /// Width and height of each grid cell in world units.
    cell_size: f32,
    /// Map from cell coordinates to entity IDs occupying that cell.
    cells: HashMap<(i32, i32), Vec<u32>>,
}

impl SpatialGrid {
    /// Creates a new spatial grid with the given cell size.
    ///
    /// # Panics
    ///
    /// Panics if `cell_size` is not positive (debug builds only via `debug_assert`).
    pub fn new(cell_size: f32) -> Self {
        debug_assert!(cell_size > 0.0, "cell_size must be positive");
        Self {
            cell_size,
            cells: HashMap::new(),
        }
    }

    /// Removes all entities from the grid.
    pub fn clear(&mut self) {
        self.cells.clear();
    }

    /// Inserts an entity into every cell overlapped by its AABB.
    pub fn insert(&mut self, entity_id: u32, aabb: &Aabb) {
        for cell in self.cells_for_aabb(aabb) {
            self.cells.entry(cell).or_default().push(entity_id);
        }
    }

    /// Returns all entity IDs in cells that overlap the query AABB.
    ///
    /// The result may contain duplicates if an entity spans multiple cells
    /// within the query area. Callers should de-duplicate if needed.
    pub fn query(&self, aabb: &Aabb) -> Vec<u32> {
        let mut result = Vec::new();
        for cell in self.cells_for_aabb(aabb) {
            if let Some(ids) = self.cells.get(&cell) {
                result.extend(ids);
            }
        }
        result
    }

    /// Converts world coordinates to cell coordinates.
    fn world_to_cell(&self, x: f32, y: f32) -> (i32, i32) {
        // floor-divide to handle negative coordinates correctly
        let cx = (x / self.cell_size).floor() as i32;
        let cy = (y / self.cell_size).floor() as i32;
        (cx, cy)
    }

    /// Returns all cell coordinates that an AABB overlaps.
    fn cells_for_aabb(&self, aabb: &Aabb) -> Vec<(i32, i32)> {
        let (min_cx, min_cy) = self.world_to_cell(aabb.min.x, aabb.min.y);
        let (max_cx, max_cy) = self.world_to_cell(aabb.max.x, aabb.max.y);

        let mut cells = Vec::new();
        for cx in min_cx..=max_cx {
            for cy in min_cy..=max_cy {
                cells.push((cx, cy));
            }
        }
        cells
    }
}
