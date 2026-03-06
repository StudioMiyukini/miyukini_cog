use serde::{Deserialize, Serialize};

use crate::chunks::{Chunk, ChunkLibrary, TileKind};
use crate::zone::Biome;

/// Rules controlling procedural layout generation for a zone.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneGenRules {
    pub biome: Biome,
    /// Number of chunk columns in the generated grid.
    pub grid_cols: u8,
    /// Number of chunk rows in the generated grid.
    pub grid_rows: u8,
    /// 0–100: probability that a branch corridor extends from the main path.
    pub branch_prob: u8,
    /// 0–100: probability that a branch ends as a dead end (vs looping back).
    pub dead_end_prob: u8,
    /// 0–100: desired fraction of grid cells filled with walkable chunks.
    pub density: u8,
}

impl ZoneGenRules {
    #[must_use]
    pub fn new(biome: Biome, grid_cols: u8, grid_rows: u8) -> Self {
        Self {
            biome,
            grid_cols,
            grid_rows,
            branch_prob: 30,
            dead_end_prob: 40,
            density: 70,
        }
    }
}

/// A single placed chunk in the generated layout.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlacedChunk {
    pub col: u8,
    pub row: u8,
    pub chunk_id: u32,
}

/// The result of procedural layout generation for a zone.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedLayout {
    pub biome: Biome,
    pub grid_cols: u8,
    pub grid_rows: u8,
    pub placed: Vec<PlacedChunk>,
}

impl GeneratedLayout {
    /// Returns the placed chunk at (col, row) if any.
    #[must_use]
    pub fn chunk_at(&self, col: u8, row: u8) -> Option<&PlacedChunk> {
        self.placed.iter().find(|p| p.col == col && p.row == row)
    }

    /// Total number of tiles (walkable + non-walkable) across all placed chunks.
    #[must_use]
    pub fn total_tiles(&self) -> usize {
        usize::from(self.grid_cols) * usize::from(self.grid_rows) * 64
    }
}

/// Deterministic LCG (same as simulator — Knuth multiplicative).
fn lcg_next(state: u64) -> u64 {
    state
        .wrapping_mul(6_364_136_223_846_793_005)
        .wrapping_add(1_442_695_040_888_963_407)
}

/// Generate a `GeneratedLayout` from `rules` and `library` using `seed`.
///
/// Algorithm:
/// 1. Walk a main path from (0,0) south-east.
/// 2. At each step, roll the branch probability to optionally extend a side corridor.
/// 3. Fill remaining empty cells up to `density` percent using random eligible chunks.
#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn generate_layout(rules: &ZoneGenRules, library: &ChunkLibrary, seed: u64) -> GeneratedLayout {
    let eligible: Vec<&Chunk> = library.for_biome(rules.biome);
    let mut placed: Vec<PlacedChunk> = Vec::new();
    let mut rng = seed;

    let cols = usize::from(rules.grid_cols);
    let rows = usize::from(rules.grid_rows);
    let capacity = cols * rows;

    // occupied[row * cols + col] = true if the cell is taken
    let mut occupied = vec![false; capacity];

    // Helper: pick a chunk id deterministically
    let pick_id = |rng: &mut u64| -> u32 {
        if eligible.is_empty() {
            return 0;
        }
        *rng = lcg_next(*rng);
        eligible[(*rng as usize) % eligible.len()].id
    };

    // --- Main path: walk south then east, bounding to grid ---
    let mut col: usize = 0;
    let mut row: usize = 0;
    loop {
        let idx = row * cols + col;
        if !occupied[idx] {
            let id = pick_id(&mut rng);
            placed.push(PlacedChunk {
                col: col as u8,
                row: row as u8,
                chunk_id: id,
            });
            occupied[idx] = true;
        }

        // Branch attempt
        rng = lcg_next(rng);
        let branch_roll = (rng % 100) as u8;
        if branch_roll < rules.branch_prob {
            // pick a neighbour cell
            rng = lcg_next(rng);
            let dir = rng % 4;
            let (bcol, brow) = match dir {
                0 if row > 0 => (col, row - 1),
                1 if row + 1 < rows => (col, row + 1),
                2 if col > 0 => (col - 1, row),
                3 if col + 1 < cols => (col + 1, row),
                _ => (col, row),
            };
            let bidx = brow * cols + bcol;
            if !occupied[bidx] {
                let id = pick_id(&mut rng);
                placed.push(PlacedChunk {
                    col: bcol as u8,
                    row: brow as u8,
                    chunk_id: id,
                });
                occupied[bidx] = true;
            }
        }

        // Advance main path
        if col + 1 < cols {
            col += 1;
        } else if row + 1 < rows {
            row += 1;
            col = 0;
        } else {
            break;
        }
    }

    // --- Density fill ---
    let target = (capacity * usize::from(rules.density)) / 100;
    if placed.len() < target {
        for r in 0..rows {
            for c in 0..cols {
                if placed.len() >= target {
                    break;
                }
                let idx = r * cols + c;
                if !occupied[idx] {
                    let id = pick_id(&mut rng);
                    placed.push(PlacedChunk {
                        col: c as u8,
                        row: r as u8,
                        chunk_id: id,
                    });
                    occupied[idx] = true;
                }
            }
        }
    }

    GeneratedLayout {
        biome: rules.biome,
        grid_cols: rules.grid_cols,
        grid_rows: rules.grid_rows,
        placed,
    }
}

/// Count walkable tiles in a layout given the chunk library.
#[must_use]
pub fn walkable_tile_count(layout: &GeneratedLayout, library: &ChunkLibrary) -> usize {
    layout
        .placed
        .iter()
        .filter_map(|p| library.chunks.iter().find(|c| c.id == p.chunk_id))
        .map(|c| c.tiles.iter().filter(|t| t.is_walkable()).count())
        .sum()
}

/// Returns every distinct `TileKind` present in the layout.
#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn distinct_tile_kinds(layout: &GeneratedLayout, library: &ChunkLibrary) -> Vec<TileKind> {
    let mut kinds: Vec<TileKind> = library
        .chunks
        .iter()
        .filter(|c| layout.placed.iter().any(|p| p.chunk_id == c.id))
        .flat_map(|c| c.tiles.iter().copied())
        .collect();
    kinds.sort_unstable_by_key(|k| *k as u8);
    kinds.dedup();
    kinds
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunks::act1_chunk_library;

    fn wilderness_rules() -> ZoneGenRules {
        ZoneGenRules::new(Biome::Wilderness, 4, 4)
    }

    #[test]
    fn layout_has_placed_chunks() {
        let lib = act1_chunk_library();
        let rules = wilderness_rules();
        let layout = generate_layout(&rules, &lib, 42);
        assert!(!layout.placed.is_empty());
    }

    #[test]
    fn layout_chunks_within_grid_bounds() {
        let lib = act1_chunk_library();
        let rules = wilderness_rules();
        let layout = generate_layout(&rules, &lib, 7);
        for p in &layout.placed {
            assert!(p.col < rules.grid_cols, "col out of bounds");
            assert!(p.row < rules.grid_rows, "row out of bounds");
        }
    }

    #[test]
    fn no_duplicate_grid_cells() {
        let lib = act1_chunk_library();
        let rules = wilderness_rules();
        let layout = generate_layout(&rules, &lib, 13);
        let mut positions: Vec<(u8, u8)> = layout.placed.iter().map(|p| (p.col, p.row)).collect();
        positions.sort_unstable();
        let len_before = positions.len();
        positions.dedup();
        assert_eq!(len_before, positions.len(), "duplicate grid cells found");
    }

    #[test]
    fn different_seeds_produce_different_layouts() {
        let lib = act1_chunk_library();
        let rules = wilderness_rules();
        let l1 = generate_layout(&rules, &lib, 1);
        let l2 = generate_layout(&rules, &lib, 999_999);
        // Very unlikely both produce identical placed sequences
        let ids1: Vec<u32> = l1.placed.iter().map(|p| p.chunk_id).collect();
        let ids2: Vec<u32> = l2.placed.iter().map(|p| p.chunk_id).collect();
        // At minimum the chunk ids should differ when there is more than one chunk type
        if lib.for_biome(Biome::Wilderness).len() > 1 {
            assert_ne!(ids1, ids2);
        } else {
            // Only one chunk type — just check non-empty
            assert!(!ids1.is_empty());
        }
    }

    #[test]
    fn walkable_tile_count_positive() {
        let lib = act1_chunk_library();
        let rules = wilderness_rules();
        let layout = generate_layout(&rules, &lib, 42);
        assert!(walkable_tile_count(&layout, &lib) > 0);
    }
}
