// @id: MGE-ARPG-World-Tile @do: tile-definition @role: back-end @layer: 3 @human: miyuk

//! Tile type definitions and procedural zone generation.
//!
//! This module provides:
//! - [`TileKind`] — terrain type enumeration
//! - [`Tile`] — a single tile with kind + sprite reference
//! - [`TileGrid`] — a 2D grid of `TileKind` values for procedural generation
//! - [`generate_zone`] — deterministic random-walk zone generation
//! - [`place_spawn_points`] — random walkable tile selection for monster spawns

/// The kind of terrain a tile represents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TileKind {
    /// Walkable floor.
    Floor,
    /// Solid wall (blocks movement and sight).
    Wall,
    /// Water (blocks movement, doesn't block sight).
    Water,
    /// A door (blocks movement until opened).
    Door,
    /// Stairs or ladder to another area.
    Stairs,
    /// A deadly pit.
    Pit,
    /// A portal tile (triggers zone transition).
    Portal,
}

impl TileKind {
    /// Returns `true` if this tile kind allows movement.
    #[must_use]
    pub fn is_walkable(self) -> bool {
        matches!(self, Self::Floor | Self::Door | Self::Stairs | Self::Portal)
    }

    /// Returns `true` if this tile kind blocks line of sight.
    #[must_use]
    pub fn blocks_sight(self) -> bool {
        matches!(self, Self::Wall | Self::Door)
    }
}

/// A single tile in the world grid.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Tile {
    /// The terrain type of this tile.
    pub kind: TileKind,
    /// Sprite sheet ID for rendering.
    pub sprite_id: u32,
}

impl Tile {
    /// Create a new tile with the given kind and sprite ID.
    #[must_use]
    pub fn new(kind: TileKind, sprite_id: u32) -> Self {
        Self { kind, sprite_id }
    }

    /// Create a walkable floor tile.
    #[must_use]
    pub fn floor(sprite_id: u32) -> Self {
        Self::new(TileKind::Floor, sprite_id)
    }

    /// Create a solid wall tile.
    #[must_use]
    pub fn wall(sprite_id: u32) -> Self {
        Self::new(TileKind::Wall, sprite_id)
    }

    /// Returns `true` if this tile allows movement.
    #[must_use]
    pub fn is_walkable(&self) -> bool {
        self.kind.is_walkable()
    }

    /// Returns `true` if this tile blocks line of sight.
    #[must_use]
    pub fn blocks_sight(&self) -> bool {
        self.kind.blocks_sight()
    }
}

// ---------------------------------------------------------------------------
// TileGrid — 2D grid of TileKind for procedural generation
// @id: MGE-ARPG-World-TileGrid @do: procgen-grid @role: back-end @layer: 3 @human: miyuk
// ---------------------------------------------------------------------------

/// A 2D grid of [`TileKind`] values used during procedural zone generation.
///
/// The grid stores tiles in row-major order: index = `y * width + x`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TileGrid {
    /// Grid width in tiles.
    pub width: usize,
    /// Grid height in tiles.
    pub height: usize,
    tiles: Vec<TileKind>,
}

impl TileGrid {
    /// Create a new grid filled with `fill` tile kind.
    #[must_use]
    pub fn new(width: usize, height: usize, fill: TileKind) -> Self {
        Self {
            width,
            height,
            tiles: vec![fill; width * height],
        }
    }

    /// Returns the tile kind at `(x, y)`, or `None` if out of bounds.
    #[must_use]
    pub fn get(&self, x: usize, y: usize) -> Option<TileKind> {
        if x < self.width && y < self.height {
            Some(self.tiles[y * self.width + x])
        } else {
            None
        }
    }

    /// Set the tile kind at `(x, y)`. Does nothing if out of bounds.
    pub fn set(&mut self, x: usize, y: usize, kind: TileKind) {
        if x < self.width && y < self.height {
            self.tiles[y * self.width + x] = kind;
        }
    }

    /// Returns `true` if the tile at `(x, y)` is walkable.
    ///
    /// Returns `false` for out-of-bounds coordinates.
    #[must_use]
    pub fn is_walkable(&self, x: usize, y: usize) -> bool {
        self.get(x, y).is_some_and(TileKind::is_walkable)
    }

    /// Count the total number of tiles matching `kind`.
    #[must_use]
    pub fn count(&self, kind: TileKind) -> usize {
        self.tiles.iter().filter(|&&t| t == kind).count()
    }
}

// ---------------------------------------------------------------------------
// Deterministic RNG (xorshift64)
// ---------------------------------------------------------------------------

/// Advance a xorshift64 PRNG state and return the new value.
///
/// This is a self-contained deterministic RNG — no external crate needed.
fn next_rng(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

// ---------------------------------------------------------------------------
// Procedural zone generation
// @id: MGE-ARPG-World-ProcGen @do: zone-generation @role: back-end @layer: 3 @human: miyuk
// ---------------------------------------------------------------------------

/// Number of branch paths to carve after the main path.
const BRANCH_COUNT: usize = 6;

/// Maximum steps per branch path.
const BRANCH_MAX_STEPS: usize = 30;

/// Carve a corridor of floor tiles around `(cx, cy)` with a given `radius`.
fn carve(grid: &mut TileGrid, cx: usize, cy: usize, radius: usize) {
    let r = radius;
    let x_start = cx.saturating_sub(r);
    let y_start = cy.saturating_sub(r);
    let x_end = (cx + r + 1).min(grid.width);
    let y_end = (cy + r + 1).min(grid.height);
    for yy in y_start..y_end {
        for xx in x_start..x_end {
            grid.set(xx, yy, TileKind::Floor);
        }
    }
}

/// Generate a procedural zone of `width` x `height` tiles using a deterministic
/// random walk algorithm.
///
/// The algorithm:
/// 1. Fills the entire grid with [`TileKind::Wall`].
/// 2. Carves a **main path** from the left edge `(0, height/2)` toward the right
///    edge using a random walk weighted toward the right direction.
/// 3. Carves several **branch paths** starting from random points along the main
///    path to add exploration variety.
///
/// The `seed` ensures deterministic output: the same seed always produces the
/// same map layout.
///
/// # Panics
///
/// Does not panic. A zero-sized grid will produce an empty `TileGrid`.
#[must_use]
pub fn generate_zone(width: usize, height: usize, seed: u64) -> TileGrid {
    let mut grid = TileGrid::new(width, height, TileKind::Wall);

    if width == 0 || height == 0 {
        return grid;
    }

    // Ensure seed is non-zero for xorshift (zero would stay zero forever).
    let mut rng = if seed == 0 { 1 } else { seed };

    // -- Main path: random walk from left to right --
    let mut x: usize = 0;
    let mut y: usize = height / 2;

    // Store main path coordinates for branch starting points.
    let mut main_path: Vec<(usize, usize)> = Vec::new();

    while x < width {
        carve(&mut grid, x, y, 1);
        main_path.push((x, y));

        let roll = next_rng(&mut rng) % 100;

        // Weighted direction: ~50% right, ~20% right-up, ~20% right-down, ~5% up, ~5% down
        if roll < 50 {
            // Move right
            x += 1;
        } else if roll < 70 {
            // Move right and up
            x += 1;
            y = y.saturating_sub(1);
        } else if roll < 90 {
            // Move right and down
            x += 1;
            y = (y + 1).min(height.saturating_sub(1));
        } else if roll < 95 {
            // Move up only
            y = y.saturating_sub(1);
        } else {
            // Move down only
            y = (y + 1).min(height.saturating_sub(1));
        }
    }

    // -- Branch paths --
    if !main_path.is_empty() {
        for branch_idx in 0..BRANCH_COUNT {
            // Pick a starting point from the main path.
            let path_idx = next_rng(&mut rng) as usize % main_path.len();
            let (mut bx, mut by) = main_path[path_idx];

            let branch_steps =
                (next_rng(&mut rng) as usize % BRANCH_MAX_STEPS).max(10);

            // Determine the dominant direction for this branch based on index.
            // Even branches tend upward, odd branches tend downward.
            let tends_up = branch_idx % 2 == 0;

            for _ in 0..branch_steps {
                carve(&mut grid, bx, by, 0);

                let roll = next_rng(&mut rng) % 100;

                if roll < 30 {
                    bx = if roll < 15 {
                        bx.saturating_sub(1)
                    } else {
                        (bx + 1).min(width.saturating_sub(1))
                    };
                } else if tends_up {
                    by = by.saturating_sub(1);
                } else {
                    by = (by + 1).min(height.saturating_sub(1));
                }
            }
        }
    }

    grid
}

/// Find `count` random walkable positions in the grid suitable for monster spawn
/// points.
///
/// Returns up to `count` unique `(x, y)` coordinates. If the grid has fewer
/// walkable tiles than `count`, all walkable positions are returned.
///
/// The `seed` ensures deterministic selection.
#[must_use]
pub fn place_spawn_points(
    grid: &TileGrid,
    seed: u64,
    count: usize,
) -> Vec<(usize, usize)> {
    // Collect all walkable positions.
    let walkable: Vec<(usize, usize)> = (0..grid.height)
        .flat_map(|y| (0..grid.width).map(move |x| (x, y)))
        .filter(|&(x, y)| grid.is_walkable(x, y))
        .collect();

    if walkable.is_empty() || count == 0 {
        return Vec::new();
    }

    let mut rng = if seed == 0 { 1 } else { seed };
    let mut selected: Vec<(usize, usize)> = Vec::with_capacity(count.min(walkable.len()));

    // Fisher-Yates-style selection on a shuffled index list.
    let mut indices: Vec<usize> = (0..walkable.len()).collect();
    let take = count.min(indices.len());

    for i in 0..take {
        let j = i + (next_rng(&mut rng) as usize % (indices.len() - i));
        indices.swap(i, j);
        selected.push(walkable[indices[i]]);
    }

    selected
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    #[test]
    fn test_tile_kind_walkable() {
        assert!(TileKind::Floor.is_walkable());
        assert!(!TileKind::Wall.is_walkable());
        assert!(!TileKind::Water.is_walkable());
        assert!(TileKind::Door.is_walkable());
        assert!(TileKind::Stairs.is_walkable());
        assert!(!TileKind::Pit.is_walkable());
        assert!(TileKind::Portal.is_walkable());
    }

    #[test]
    fn test_tile_kind_blocks_sight() {
        assert!(TileKind::Wall.blocks_sight());
        assert!(TileKind::Door.blocks_sight());
        assert!(!TileKind::Floor.blocks_sight());
        assert!(!TileKind::Water.blocks_sight());
        assert!(!TileKind::Stairs.blocks_sight());
        assert!(!TileKind::Pit.blocks_sight());
        assert!(!TileKind::Portal.blocks_sight());
    }

    #[test]
    fn test_tile_floor_constructor() {
        let tile = Tile::floor(42);
        assert_eq!(tile.kind, TileKind::Floor);
        assert_eq!(tile.sprite_id, 42);
        assert!(tile.is_walkable());
        assert!(!tile.blocks_sight());
    }

    #[test]
    fn test_tile_wall_constructor() {
        let tile = Tile::wall(99);
        assert_eq!(tile.kind, TileKind::Wall);
        assert_eq!(tile.sprite_id, 99);
        assert!(!tile.is_walkable());
        assert!(tile.blocks_sight());
    }

    // -- TileGrid tests --

    #[test]
    fn test_tile_grid_new() {
        let grid = TileGrid::new(10, 8, TileKind::Wall);
        assert_eq!(grid.width, 10);
        assert_eq!(grid.height, 8);
        assert_eq!(grid.get(0, 0), Some(TileKind::Wall));
        assert_eq!(grid.get(9, 7), Some(TileKind::Wall));
        assert_eq!(grid.get(10, 0), None);
        assert_eq!(grid.get(0, 8), None);
    }

    #[test]
    fn test_tile_grid_set_and_get() {
        let mut grid = TileGrid::new(5, 5, TileKind::Wall);
        grid.set(2, 3, TileKind::Floor);
        assert_eq!(grid.get(2, 3), Some(TileKind::Floor));
        assert_eq!(grid.get(0, 0), Some(TileKind::Wall));
    }

    #[test]
    fn test_tile_grid_set_out_of_bounds_noop() {
        let mut grid = TileGrid::new(5, 5, TileKind::Wall);
        // Should not panic or change anything.
        grid.set(100, 100, TileKind::Floor);
        assert_eq!(grid.count(TileKind::Floor), 0);
    }

    #[test]
    fn test_tile_grid_is_walkable() {
        let mut grid = TileGrid::new(5, 5, TileKind::Wall);
        assert!(!grid.is_walkable(2, 2));
        grid.set(2, 2, TileKind::Floor);
        assert!(grid.is_walkable(2, 2));
        // Out of bounds => false
        assert!(!grid.is_walkable(99, 99));
    }

    #[test]
    fn test_tile_grid_count() {
        let mut grid = TileGrid::new(4, 4, TileKind::Wall);
        assert_eq!(grid.count(TileKind::Wall), 16);
        assert_eq!(grid.count(TileKind::Floor), 0);
        grid.set(0, 0, TileKind::Floor);
        grid.set(1, 1, TileKind::Floor);
        assert_eq!(grid.count(TileKind::Floor), 2);
        assert_eq!(grid.count(TileKind::Wall), 14);
    }

    // -- Procgen tests --

    /// BFS flood fill from a starting position. Returns the number of reachable
    /// floor tiles.
    fn bfs_floor_count(grid: &TileGrid, start_x: usize, start_y: usize) -> usize {
        if !grid.is_walkable(start_x, start_y) {
            return 0;
        }

        let mut visited = vec![false; grid.width * grid.height];
        let mut queue = VecDeque::new();
        queue.push_back((start_x, start_y));
        visited[start_y * grid.width + start_x] = true;
        let mut count = 0;

        while let Some((x, y)) = queue.pop_front() {
            count += 1;

            // 4-directional neighbors.
            let neighbors: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            for (dx, dy) in neighbors {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx >= 0 && ny >= 0 {
                    let nx = nx as usize;
                    let ny = ny as usize;
                    if nx < grid.width
                        && ny < grid.height
                        && !visited[ny * grid.width + nx]
                        && grid.is_walkable(nx, ny)
                    {
                        visited[ny * grid.width + nx] = true;
                        queue.push_back((nx, ny));
                    }
                }
            }
        }

        count
    }

    #[test]
    fn procgen_path_connected() {
        let grid = generate_zone(64, 32, 42);

        // The main path starts at y = height/2 on the left edge.
        // BFS from the first walkable tile in column 0 should find a significant
        // connected floor area.

        // The carve radius is 1, so the start column has carved tiles around
        // (0, height/2). Search for the first walkable tile in the start column.
        let start_tile = (0..grid.height).find(|&y| grid.is_walkable(0, y));
        assert!(
            start_tile.is_some(),
            "Expected at least one walkable tile in the first column (main path entrance)"
        );

        let reachable = bfs_floor_count(&grid, 0, start_tile.unwrap());
        assert!(
            reachable > 10,
            "Expected >10 reachable floor tiles from start, found {reachable}"
        );

        // Also verify the grid has a healthy ratio of floor tiles.
        let total_floor = grid.count(TileKind::Floor);
        assert!(
            total_floor > 50,
            "Expected >50 total floor tiles in a 64x32 zone, found {total_floor}"
        );

        // Verify walkable tiles exist near the right edge too (main path exits).
        let right_walkable = (0..grid.height)
            .any(|y| grid.is_walkable(grid.width - 1, y) || grid.is_walkable(grid.width.saturating_sub(2), y));
        assert!(
            right_walkable,
            "Expected walkable tiles near the right edge (main path should reach it)"
        );
    }

    #[test]
    fn procgen_deterministic() {
        let grid_a = generate_zone(48, 24, 12_345);
        let grid_b = generate_zone(48, 24, 12_345);

        assert_eq!(
            grid_a, grid_b,
            "Same seed must produce identical grids"
        );
    }

    #[test]
    fn procgen_different_seed() {
        let grid_a = generate_zone(48, 24, 111);
        let grid_b = generate_zone(48, 24, 222);

        assert_ne!(
            grid_a, grid_b,
            "Different seeds should produce different grids"
        );
    }

    #[test]
    fn procgen_zero_seed_handled() {
        // Seed 0 is edge-case for xorshift — should not hang or produce all-wall.
        let grid = generate_zone(32, 16, 0);
        let floor_count = grid.count(TileKind::Floor);
        assert!(
            floor_count > 0,
            "Seed 0 should still produce floor tiles, found {floor_count}"
        );
    }

    #[test]
    fn procgen_zero_size() {
        let grid = generate_zone(0, 0, 42);
        assert_eq!(grid.width, 0);
        assert_eq!(grid.height, 0);
        assert_eq!(grid.count(TileKind::Wall), 0);
    }

    // -- Spawn point tests --

    #[test]
    fn spawn_points_on_walkable() {
        let grid = generate_zone(48, 24, 99);
        let spawns = place_spawn_points(&grid, 99, 5);

        assert!(
            !spawns.is_empty(),
            "Should find at least some spawn points"
        );
        assert!(
            spawns.len() <= 5,
            "Should not exceed requested count"
        );

        for &(x, y) in &spawns {
            assert!(
                grid.is_walkable(x, y),
                "Spawn point ({x}, {y}) must be on a walkable tile"
            );
        }
    }

    #[test]
    fn spawn_points_deterministic() {
        let grid = generate_zone(48, 24, 77);
        let spawns_a = place_spawn_points(&grid, 42, 8);
        let spawns_b = place_spawn_points(&grid, 42, 8);
        assert_eq!(
            spawns_a, spawns_b,
            "Same seed must produce identical spawn selections"
        );
    }

    #[test]
    fn spawn_points_unique() {
        let grid = generate_zone(48, 24, 55);
        let spawns = place_spawn_points(&grid, 55, 10);

        let mut sorted = spawns.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(
            sorted.len(),
            spawns.len(),
            "All spawn points must be unique"
        );
    }

    #[test]
    fn spawn_points_empty_grid() {
        let grid = TileGrid::new(10, 10, TileKind::Wall);
        let spawns = place_spawn_points(&grid, 42, 5);
        assert!(
            spawns.is_empty(),
            "No walkable tiles => no spawn points"
        );
    }

    #[test]
    fn spawn_points_zero_count() {
        let grid = generate_zone(32, 16, 42);
        let spawns = place_spawn_points(&grid, 42, 0);
        assert!(
            spawns.is_empty(),
            "Requesting 0 spawn points should return empty"
        );
    }
}
