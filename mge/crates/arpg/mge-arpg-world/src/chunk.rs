// @id: MGE-ARPG-World-Chunk @do: chunk @role: back-end @layer: 3 @human: miyuk

//! Chunk: a fixed-size tile grid region.

use crate::error::WorldError;
use crate::tile::Tile;

/// Chunk side length in tiles.
pub const CHUNK_SIZE: usize = 16;

/// A fixed 16x16 grid of tiles.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Chunk {
    tiles: Vec<Tile>,
}

impl Chunk {
    /// Create a chunk filled with floor tiles.
    #[must_use]
    pub fn new_floor(sprite_id: u32) -> Self {
        Self {
            tiles: (0..CHUNK_SIZE * CHUNK_SIZE)
                .map(|_| Tile::floor(sprite_id))
                .collect(),
        }
    }

    /// Create a chunk filled with wall tiles.
    #[must_use]
    pub fn new_wall(sprite_id: u32) -> Self {
        Self {
            tiles: (0..CHUNK_SIZE * CHUNK_SIZE)
                .map(|_| Tile::wall(sprite_id))
                .collect(),
        }
    }

    fn index(x: usize, y: usize) -> Option<usize> {
        if x < CHUNK_SIZE && y < CHUNK_SIZE {
            Some(y * CHUNK_SIZE + x)
        } else {
            None
        }
    }

    /// Get tile at local coordinates.
    ///
    /// # Errors
    ///
    /// Returns `WorldError::InvalidTileCoord` if `x` or `y` is out of bounds.
    pub fn get(&self, x: usize, y: usize) -> Result<&Tile, WorldError> {
        let idx =
            Self::index(x, y).ok_or(WorldError::InvalidTileCoord { x, y })?;
        Ok(&self.tiles[idx])
    }

    /// Get mutable tile at local coordinates.
    ///
    /// # Errors
    ///
    /// Returns `WorldError::InvalidTileCoord` if `x` or `y` is out of bounds.
    pub fn get_mut(
        &mut self,
        x: usize,
        y: usize,
    ) -> Result<&mut Tile, WorldError> {
        let idx =
            Self::index(x, y).ok_or(WorldError::InvalidTileCoord { x, y })?;
        Ok(&mut self.tiles[idx])
    }

    /// Returns `true` if the tile at `(x, y)` is walkable.
    ///
    /// Returns `false` for out-of-bounds coordinates.
    #[must_use]
    pub fn is_walkable(&self, x: usize, y: usize) -> bool {
        self.get(x, y)
            .is_ok_and(Tile::is_walkable)
    }

    /// Total tile count (always `CHUNK_SIZE` squared).
    #[must_use]
    pub fn tile_count(&self) -> usize {
        self.tiles.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tile::TileKind;

    #[test]
    fn test_chunk_new_floor() {
        let chunk = Chunk::new_floor(1);
        assert_eq!(chunk.tile_count(), CHUNK_SIZE * CHUNK_SIZE);
        assert!(chunk.is_walkable(0, 0));
    }

    #[test]
    fn test_chunk_new_wall() {
        let chunk = Chunk::new_wall(2);
        assert!(!chunk.is_walkable(0, 0));
    }

    #[test]
    fn test_chunk_get_valid() {
        let chunk = Chunk::new_floor(1);
        let tile = chunk.get(5, 5);
        assert!(tile.is_ok());
        assert_eq!(tile.unwrap().kind, TileKind::Floor);
    }

    #[test]
    fn test_chunk_get_out_of_bounds() {
        let chunk = Chunk::new_floor(1);
        assert!(chunk.get(CHUNK_SIZE, 0).is_err());
        assert!(chunk.get(0, CHUNK_SIZE).is_err());
        assert!(chunk.get(CHUNK_SIZE, CHUNK_SIZE).is_err());
    }

    #[test]
    fn test_chunk_is_walkable_floor() {
        let chunk = Chunk::new_floor(1);
        assert!(chunk.is_walkable(0, 0));
        assert!(chunk.is_walkable(CHUNK_SIZE - 1, CHUNK_SIZE - 1));
    }

    #[test]
    fn test_chunk_is_walkable_wall() {
        let chunk = Chunk::new_wall(2);
        assert!(!chunk.is_walkable(0, 0));
        assert!(!chunk.is_walkable(CHUNK_SIZE - 1, CHUNK_SIZE - 1));
    }

    #[test]
    fn test_chunk_set_tile() {
        let mut chunk = Chunk::new_floor(1);
        assert!(chunk.is_walkable(3, 3));

        let tile = chunk.get_mut(3, 3).unwrap();
        tile.kind = TileKind::Wall;

        assert!(!chunk.is_walkable(3, 3));
    }
}
