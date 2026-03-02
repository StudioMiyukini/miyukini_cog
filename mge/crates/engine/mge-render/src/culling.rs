// @id: MGE-Render-Culling @do: implement @role: back-end @layer: 2 @human: francois

//! Spatial hashing and frustum culling for instanced sprite rendering.
//!
//! This module provides a uniform-grid spatial hash ([`RenderSpatialGrid`])
//! for O(1) amortised insertion and near-O(1) rectangular queries, plus a
//! high-level [`FrustumCuller`] that rebuilds the grid each frame and
//! returns visible entity IDs sorted by depth.

use std::collections::HashMap;

// ---------------------------------------------------------------------------
// RenderEntity
// ---------------------------------------------------------------------------

/// Lightweight renderable entity descriptor for spatial indexing.
///
/// Contains only the minimum data needed for culling decisions.
#[derive(Debug, Clone, Copy)]
pub struct RenderEntity {
    /// Unique entity identifier (matches the ECS entity id).
    pub id: u32,
    /// World-space position `[x, y]` in pixels.
    pub position: [f32; 2],
    /// Bounding dimensions `[width, height]` in pixels.
    pub size: [f32; 2],
}

// ---------------------------------------------------------------------------
// RenderSpatialGrid
// ---------------------------------------------------------------------------

/// Uniform-grid spatial hash for fast rectangular queries.
///
/// Entities are inserted into cells based on their position and size.
/// Large entities that span multiple cells are inserted into each
/// overlapping cell.
///
/// Default cell size is 256 pixels, which balances memory usage against
/// query granularity for typical isometric maps.
pub struct RenderSpatialGrid {
    /// Size of each cell in pixels (both axes).
    cell_size: f32,
    /// Map from (cell_x, cell_y) to the entities overlapping that cell.
    cells: HashMap<(i32, i32), Vec<RenderEntity>>,
}

impl RenderSpatialGrid {
    /// Create a new spatial grid with the given cell size.
    pub fn new(cell_size: f32) -> Self {
        Self {
            cell_size,
            cells: HashMap::new(),
        }
    }

    /// Remove all entities from the grid.
    pub fn clear(&mut self) {
        for bucket in self.cells.values_mut() {
            bucket.clear();
        }
    }

    /// Insert an entity into every cell it overlaps.
    pub fn insert(&mut self, entity: RenderEntity) {
        let min_cx = (entity.position[0] / self.cell_size).floor() as i32;
        let min_cy = (entity.position[1] / self.cell_size).floor() as i32;
        let max_cx =
            ((entity.position[0] + entity.size[0]) / self.cell_size).floor() as i32;
        let max_cy =
            ((entity.position[1] + entity.size[1]) / self.cell_size).floor() as i32;

        for cx in min_cx..=max_cx {
            for cy in min_cy..=max_cy {
                self.cells.entry((cx, cy)).or_default().push(entity);
            }
        }
    }

    /// Query all entities whose cells intersect the given axis-aligned
    /// rectangle `[min_x, min_y, max_x, max_y]`.
    ///
    /// Entities that span multiple cells may appear in the result more than
    /// once. The caller should deduplicate by `id` if needed.
    pub fn query_rect(
        &self,
        min_x: f32,
        min_y: f32,
        max_x: f32,
        max_y: f32,
    ) -> Vec<&RenderEntity> {
        let min_cx = (min_x / self.cell_size).floor() as i32;
        let min_cy = (min_y / self.cell_size).floor() as i32;
        let max_cx = (max_x / self.cell_size).floor() as i32;
        let max_cy = (max_y / self.cell_size).floor() as i32;

        let mut result = Vec::new();
        for cx in min_cx..=max_cx {
            for cy in min_cy..=max_cy {
                if let Some(bucket) = self.cells.get(&(cx, cy)) {
                    for entity in bucket {
                        result.push(entity);
                    }
                }
            }
        }
        result
    }
}

// ---------------------------------------------------------------------------
// FrustumCuller
// ---------------------------------------------------------------------------

/// High-level frustum culler that rebuilds a spatial grid each frame and
/// returns visible entity IDs sorted by depth (ascending Y for iso sorting).
pub struct FrustumCuller {
    /// Internal spatial grid.
    grid: RenderSpatialGrid,
}

impl FrustumCuller {
    /// Create a new frustum culler with the given cell size.
    pub fn new(cell_size: f32) -> Self {
        Self {
            grid: RenderSpatialGrid::new(cell_size),
        }
    }

    /// Clear and re-populate the spatial grid with the given entities.
    pub fn rebuild(&mut self, entities: &[RenderEntity]) {
        self.grid.clear();
        for entity in entities {
            self.grid.insert(*entity);
        }
    }

    /// Return the IDs of entities visible within the given rectangle.
    ///
    /// The result is deduplicated and sorted by `position.y` ascending
    /// (painter's-algorithm depth ordering for isometric projection).
    pub fn cull(&self, visible_rect: [f32; 4]) -> Vec<u32> {
        let candidates = self.grid.query_rect(
            visible_rect[0],
            visible_rect[1],
            visible_rect[2],
            visible_rect[3],
        );

        // Deduplicate by entity id using a small vec (faster than HashSet
        // for typical visible-set sizes of a few hundred entities).
        let mut seen = Vec::new();
        let mut result: Vec<(u32, f32)> = Vec::new();
        for entity in candidates {
            if !seen.contains(&entity.id) {
                seen.push(entity.id);
                result.push((entity.id, entity.position[1]));
            }
        }

        // Sort by Y position ascending (back-to-front in iso space).
        result.sort_by(|a, b| {
            a.1.partial_cmp(&b.1)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        result.into_iter().map(|(id, _)| id).collect()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -- RenderSpatialGrid tests -------------------------------------------

    #[test]
    fn spatial_grid_insert_and_query() {
        let mut grid = RenderSpatialGrid::new(256.0);
        for i in 0..5 {
            grid.insert(RenderEntity {
                id: i,
                position: [i as f32 * 100.0, 0.0],
                size: [32.0, 32.0],
            });
        }
        // Query a rect that covers entities at x=0..300 (should catch ids 0,1,2)
        let result = grid.query_rect(0.0, 0.0, 300.0, 100.0);
        // At least 3 entities (0, 100, 200 are all in range)
        assert!(
            result.len() >= 3,
            "expected at least 3 entities, got {}",
            result.len()
        );
    }

    #[test]
    fn spatial_grid_query_empty() {
        let grid = RenderSpatialGrid::new(256.0);
        let result = grid.query_rect(0.0, 0.0, 100.0, 100.0);
        assert!(result.is_empty());
    }

    #[test]
    fn spatial_grid_clear() {
        let mut grid = RenderSpatialGrid::new(256.0);
        grid.insert(RenderEntity {
            id: 0,
            position: [0.0, 0.0],
            size: [32.0, 32.0],
        });
        grid.clear();
        let result = grid.query_rect(-1000.0, -1000.0, 1000.0, 1000.0);
        assert!(result.is_empty());
    }

    #[test]
    fn spatial_grid_large_entity_spans_cells() {
        let mut grid = RenderSpatialGrid::new(256.0);
        // Entity at (0,0) with size 512x512 should span 4 cells:
        // cell (0,0), (0,1), (1,0), (1,1) -- or more depending on float edge.
        grid.insert(RenderEntity {
            id: 42,
            position: [0.0, 0.0],
            size: [512.0, 512.0],
        });

        // Query each quadrant individually to verify the entity is present.
        let q1 = grid.query_rect(0.0, 0.0, 1.0, 1.0);
        let q2 = grid.query_rect(300.0, 0.0, 301.0, 1.0);
        let q3 = grid.query_rect(0.0, 300.0, 1.0, 301.0);
        let q4 = grid.query_rect(300.0, 300.0, 301.0, 301.0);

        assert!(!q1.is_empty(), "entity should be in quadrant 1");
        assert!(!q2.is_empty(), "entity should be in quadrant 2");
        assert!(!q3.is_empty(), "entity should be in quadrant 3");
        assert!(!q4.is_empty(), "entity should be in quadrant 4");
    }

    // -- FrustumCuller tests -----------------------------------------------

    #[test]
    fn frustum_culler_filters_offscreen() {
        let mut culler = FrustumCuller::new(256.0);

        // 10 entities: 3 inside the visible rect, 7 outside.
        let mut entities = Vec::new();
        // Inside: y=0..200
        for i in 0..3 {
            entities.push(RenderEntity {
                id: i,
                position: [i as f32 * 50.0, i as f32 * 50.0],
                size: [32.0, 32.0],
            });
        }
        // Outside: x=5000+
        for i in 3..10 {
            entities.push(RenderEntity {
                id: i,
                position: [5000.0 + i as f32 * 100.0, 5000.0],
                size: [32.0, 32.0],
            });
        }

        culler.rebuild(&entities);
        let visible = culler.cull([0.0, 0.0, 300.0, 300.0]);
        assert_eq!(
            visible.len(),
            3,
            "expected 3 visible entities, got {}: {:?}",
            visible.len(),
            visible
        );
    }

    #[test]
    fn frustum_culler_returns_sorted() {
        let mut culler = FrustumCuller::new(256.0);

        let entities = vec![
            RenderEntity {
                id: 10,
                position: [0.0, 200.0],
                size: [32.0, 32.0],
            },
            RenderEntity {
                id: 20,
                position: [0.0, 50.0],
                size: [32.0, 32.0],
            },
            RenderEntity {
                id: 30,
                position: [0.0, 150.0],
                size: [32.0, 32.0],
            },
        ];

        culler.rebuild(&entities);
        let visible = culler.cull([-100.0, -100.0, 500.0, 500.0]);

        // Should be sorted by Y: 50 (id 20), 150 (id 30), 200 (id 10)
        assert_eq!(visible.len(), 3);
        assert_eq!(visible[0], 20, "first should be id 20 (y=50)");
        assert_eq!(visible[1], 30, "second should be id 30 (y=150)");
        assert_eq!(visible[2], 10, "third should be id 10 (y=200)");
    }
}
