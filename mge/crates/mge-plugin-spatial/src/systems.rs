//! @id mge.plugin.spatial.v1.systems
//! @role system
//! @layer plugin
//! @domain spatial
//! @do provide_spatial_grid_index_and_update_system

use std::collections::HashMap;

use mge_ecs::{Component, EntityId, World};

use crate::components::Position2D;

/// Grille spatiale pour requêtes de voisinage en O(k) au lieu de O(n).
#[derive(Debug, Clone)]
pub struct SpatialGrid {
    pub cell_size: f32,
    cells: HashMap<(i32, i32), Vec<EntityId>>,
}

impl SpatialGrid {
    pub fn new(cell_size: f32) -> Self {
        Self {
            cell_size: cell_size.max(0.5),
            cells: HashMap::new(),
        }
    }

    fn cell_key(&self, x: f32, y: f32) -> (i32, i32) {
        ((x / self.cell_size).floor() as i32, (y / self.cell_size).floor() as i32)
    }

    pub fn clear(&mut self) {
        for v in self.cells.values_mut() {
            v.clear();
        }
    }

    pub fn insert(&mut self, id: EntityId, x: f32, y: f32) {
        let key = self.cell_key(x, y);
        self.cells.entry(key).or_default().push(id);
    }

    /// Retourne les entités dans un rayon autour de (cx, cy).
    pub fn query_radius(&self, cx: f32, cy: f32, radius: f32) -> Vec<EntityId> {
        let min_key = self.cell_key(cx - radius, cy - radius);
        let max_key = self.cell_key(cx + radius, cy + radius);
        let mut result = Vec::new();
        for ix in min_key.0..=max_key.0 {
            for iy in min_key.1..=max_key.1 {
                if let Some(ids) = self.cells.get(&(ix, iy)) {
                    result.extend(ids.iter().copied());
                }
            }
        }
        result.retain(|_| true); // placeholder — filtered by caller with actual positions
        result
    }

    /// Reconstruit entièrement la grille depuis le World.
    pub fn rebuild(&mut self, world: &World) {
        self.clear();
        for (id, pos) in world.iter1::<Position2D>() {
            self.insert(id, pos.x, pos.y);
        }
    }
}

impl Component for SpatialGrid {}

impl Default for SpatialGrid {
    fn default() -> Self {
        Self::new(8.0)
    }
}
