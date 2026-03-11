use std::collections::HashMap;

/// Uniform-grid spatial index for fast proximity queries.
///
/// Entities are bucketed into square cells of `cell_size` tiles.
/// Queries return all entity indices within a radius by checking
/// only the overlapping cells.
pub struct SpatialGrid {
    _cell_size: f32,
    inv_cell: f32,
    cells: HashMap<(i32, i32), Vec<usize>>,
}

impl SpatialGrid {
    pub fn new(cell_size: f32) -> Self {
        Self {
            _cell_size: cell_size,
            inv_cell: 1.0 / cell_size,
            cells: HashMap::new(),
        }
    }

    /// Clear and rebuild from a slice of positions.
    /// `positions` is an iterator of (index, [x, y]).
    pub fn rebuild<I>(&mut self, positions: I)
    where
        I: Iterator<Item = (usize, [f32; 2])>,
    {
        self.cells.clear();
        for (idx, pos) in positions {
            let key = self.cell_key(pos);
            self.cells.entry(key).or_default().push(idx);
        }
    }

    /// Query all indices within `radius` of `center`.
    #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
    pub fn query_radius(&self, center: [f32; 2], radius: f32) -> Vec<usize> {
        let mut result = Vec::new();
        let r_cells = (radius * self.inv_cell).ceil() as i32;
        let cx = (center[0] * self.inv_cell).floor() as i32;
        let cy = (center[1] * self.inv_cell).floor() as i32;
        for dx in -r_cells..=r_cells {
            for dy in -r_cells..=r_cells {
                if let Some(bucket) = self.cells.get(&(cx + dx, cy + dy)) {
                    result.extend_from_slice(bucket);
                }
            }
        }
        result
    }

    #[allow(clippy::cast_possible_truncation)]
    fn cell_key(&self, pos: [f32; 2]) -> (i32, i32) {
        (
            (pos[0] * self.inv_cell).floor() as i32,
            (pos[1] * self.inv_cell).floor() as i32,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_query() {
        let mut grid = SpatialGrid::new(5.0);
        let positions = vec![
            (0, [2.0, 2.0]),
            (1, [10.0, 10.0]),
            (2, [3.0, 3.0]),
            (3, [20.0, 20.0]),
        ];
        grid.rebuild(positions.into_iter());

        let near = grid.query_radius([2.5, 2.5], 2.0);
        assert!(near.contains(&0));
        assert!(near.contains(&2));
        assert!(!near.contains(&1));
        assert!(!near.contains(&3));
    }

    #[test]
    fn empty_grid() {
        let grid = SpatialGrid::new(5.0);
        let result = grid.query_radius([0.0, 0.0], 10.0);
        assert!(result.is_empty());
    }
}
