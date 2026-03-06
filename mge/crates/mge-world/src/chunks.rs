use serde::{Deserialize, Serialize};

use crate::zone::Biome;

/// Individual tile type in a chunk.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TileKind {
    Ground,
    Wall,
    Water,
    Road,
    Door,
    Pillar,
    Chasm,
}

impl TileKind {
    pub fn is_walkable(self) -> bool {
        matches!(self, Self::Ground | Self::Road | Self::Door)
    }
}

/// A rectangular tile chunk (template piece for procedural assembly).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(clippy::struct_excessive_bools)]
pub struct Chunk {
    pub id: u32,
    pub biome: Biome,
    /// Chunk width in tiles.
    pub width: u8,
    /// Chunk height in tiles.
    pub height: u8,
    /// Row-major tile data (width × height entries).
    pub tiles: Vec<TileKind>,
    /// Entry edge connectors: N/S/E/W = true means this chunk can connect there.
    pub connects_north: bool,
    pub connects_south: bool,
    pub connects_east: bool,
    pub connects_west: bool,
}

impl Chunk {
    pub fn tile_at(&self, x: u8, y: u8) -> Option<TileKind> {
        if x >= self.width || y >= self.height {
            return None;
        }
        self.tiles.get(usize::from(y) * usize::from(self.width) + usize::from(x)).copied()
    }

    pub fn walkable_count(&self) -> usize {
        self.tiles.iter().filter(|t| t.is_walkable()).count()
    }
}

/// Chunk library keyed by biome.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChunkLibrary {
    pub chunks: Vec<Chunk>,
}

impl ChunkLibrary {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, chunk: Chunk) {
        self.chunks.push(chunk);
    }

    pub fn for_biome(&self, biome: Biome) -> Vec<&Chunk> {
        self.chunks.iter().filter(|c| c.biome == biome).collect()
    }
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

#[allow(clippy::fn_params_excessive_bools)]
fn make_chunk(
    id: u32,
    biome: Biome,
    tiles: Vec<TileKind>,
    n: bool,
    s: bool,
    e: bool,
    w: bool,
) -> Chunk {
    Chunk {
        id,
        biome,
        width: 8,
        height: 8,
        tiles,
        connects_north: n,
        connects_south: s,
        connects_east: e,
        connects_west: w,
    }
}

fn open_field(biome: Biome, id: u32) -> Chunk {
    let tiles = vec![TileKind::Ground; 64];
    make_chunk(id, biome, tiles, true, true, true, true)
}

fn corridor_ns(biome: Biome, id: u32) -> Chunk {
    let mut tiles = vec![TileKind::Wall; 64];
    for row in 0..8usize {
        tiles[row * 8 + 3] = TileKind::Ground;
        tiles[row * 8 + 4] = TileKind::Ground;
    }
    make_chunk(id, biome, tiles, true, true, false, false)
}

fn room_with_pillars(biome: Biome, id: u32) -> Chunk {
    let mut tiles = vec![TileKind::Ground; 64];
    // Wall border
    for i in 0..8usize {
        tiles[i] = TileKind::Wall;
        tiles[56 + i] = TileKind::Wall;
        tiles[i * 8] = TileKind::Wall;
        tiles[i * 8 + 7] = TileKind::Wall;
    }
    // Interior pillars
    tiles[2 * 8 + 2] = TileKind::Pillar;
    tiles[2 * 8 + 5] = TileKind::Pillar;
    tiles[5 * 8 + 2] = TileKind::Pillar;
    tiles[5 * 8 + 5] = TileKind::Pillar;
    make_chunk(id, biome, tiles, true, true, true, true)
}

/// Build the MVP chunk library for all Act 1 biomes.
pub fn act1_chunk_library() -> ChunkLibrary {
    let mut lib = ChunkLibrary::new();
    // Wilderness
    lib.add(open_field(Biome::Wilderness, 1));
    lib.add(corridor_ns(Biome::Wilderness, 2));
    // Forest
    lib.add(open_field(Biome::Forest, 10));
    // Cave
    lib.add(corridor_ns(Biome::Cave, 20));
    lib.add(room_with_pillars(Biome::Cave, 21));
    // Dungeon
    lib.add(room_with_pillars(Biome::Dungeon, 30));
    lib.add(corridor_ns(Biome::Dungeon, 31));
    // Catacomb
    lib.add(room_with_pillars(Biome::Catacomb, 40));
    lib.add(corridor_ns(Biome::Catacomb, 41));
    // Cathedral
    lib.add(room_with_pillars(Biome::Cathedral, 50));
    lib
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_field_all_ground() {
        let chunk = open_field(Biome::Wilderness, 99);
        assert_eq!(chunk.walkable_count(), 64);
    }

    #[test]
    fn corridor_has_two_walkable_columns() {
        let chunk = corridor_ns(Biome::Cave, 99);
        assert_eq!(chunk.walkable_count(), 16); // 2 cols × 8 rows
    }

    #[test]
    fn tile_at_out_of_bounds_returns_none() {
        let chunk = open_field(Biome::Wilderness, 99);
        assert!(chunk.tile_at(8, 0).is_none());
    }

    #[test]
    fn chunk_library_for_biome_filters_correctly() {
        let lib = act1_chunk_library();
        let caves = lib.for_biome(Biome::Cave);
        assert!(!caves.is_empty());
        assert!(caves.iter().all(|c| c.biome == Biome::Cave));
    }
}
