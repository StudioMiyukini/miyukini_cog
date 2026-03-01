// @id: MGE-Studio-Map @do: map-editor @role: back-end @layer: 6 @human: miyuk
//! Map editor state: tile placement on named layers.

use mge_math::Vec2;

use crate::error::StudioError;

/// Map layer kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum LayerKind {
    /// Ground tiles (walkable floor).
    Ground,
    /// Object layer (furniture, props).
    Object,
    /// Overlay layer (roofs, ceilings).
    Overlay,
    /// Collision mask layer.
    Collision,
}

/// A tile placed on the map.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PlacedTile {
    /// Atlas frame id for this tile.
    pub frame_id: String,
    /// Grid position (tile coordinates, not world pixels).
    pub grid_pos: (i32, i32),
    /// Layer this tile belongs to.
    pub layer: LayerKind,
}

impl PlacedTile {
    /// Create a new placed tile.
    pub fn new(frame_id: impl Into<String>, x: i32, y: i32, layer: LayerKind) -> Self {
        Self {
            frame_id: frame_id.into(),
            grid_pos: (x, y),
            layer,
        }
    }

    /// World position assuming `tile_size` pixels per tile.
    pub fn world_pos(&self, tile_size: f32) -> Vec2 {
        Vec2 {
            x: self.grid_pos.0 as f32 * tile_size,
            y: self.grid_pos.1 as f32 * tile_size,
        }
    }
}

/// Map editor state.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct MapEditorState {
    /// Map name.
    pub name: String,
    /// Map width in tiles.
    pub width: u32,
    /// Map height in tiles.
    pub height: u32,
    /// All placed tiles across all layers.
    pub tiles: Vec<PlacedTile>,
    /// Tile size in world pixels.
    pub tile_size: f32,
}

impl MapEditorState {
    /// Create a new map editor state.
    pub fn new(name: impl Into<String>, width: u32, height: u32, tile_size: f32) -> Self {
        Self {
            name: name.into(),
            width,
            height,
            tiles: Vec::new(),
            tile_size,
        }
    }

    /// Place a tile. Returns error if position is out of bounds.
    pub fn place_tile(&mut self, tile: PlacedTile) -> Result<(), StudioError> {
        if tile.grid_pos.0 < 0
            || tile.grid_pos.1 < 0
            || tile.grid_pos.0 >= self.width.cast_signed()
            || tile.grid_pos.1 >= self.height.cast_signed()
        {
            return Err(StudioError::MapError {
                message: format!(
                    "Tile position ({}, {}) is outside map bounds {}x{}",
                    tile.grid_pos.0, tile.grid_pos.1, self.width, self.height
                ),
            });
        }
        self.tiles.push(tile);
        Ok(())
    }

    /// Remove tiles at the given grid position on the given layer.
    pub fn clear_tile(&mut self, x: i32, y: i32, layer: LayerKind) {
        self.tiles
            .retain(|t| !(t.grid_pos == (x, y) && t.layer == layer));
    }

    /// Get all tiles on a specific layer.
    pub fn tiles_on_layer(&self, layer: LayerKind) -> impl Iterator<Item = &PlacedTile> {
        self.tiles.iter().filter(move |t| t.layer == layer)
    }

    /// Total number of placed tiles.
    pub fn tile_count(&self) -> usize {
        self.tiles.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_place_tile_valid() {
        let mut map = MapEditorState::new("test_map", 10, 10, 32.0);
        let tile = PlacedTile::new("grass", 5, 5, LayerKind::Ground);
        assert!(map.place_tile(tile).is_ok());
        assert_eq!(map.tile_count(), 1);
    }

    #[test]
    fn test_place_tile_out_of_bounds() {
        let mut map = MapEditorState::new("test_map", 10, 10, 32.0);
        let tile = PlacedTile::new("grass", 15, 5, LayerKind::Ground);
        let result = map.place_tile(tile);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), StudioError::MapError { .. }));
    }

    #[test]
    fn test_clear_tile() {
        let mut map = MapEditorState::new("test_map", 10, 10, 32.0);
        map.place_tile(PlacedTile::new("grass", 3, 3, LayerKind::Ground))
            .unwrap();
        map.place_tile(PlacedTile::new("tree", 3, 3, LayerKind::Object))
            .unwrap();
        assert_eq!(map.tile_count(), 2);
        map.clear_tile(3, 3, LayerKind::Ground);
        assert_eq!(map.tile_count(), 1);
        // The Object layer tile should remain.
        assert_eq!(map.tiles[0].layer, LayerKind::Object);
    }

    #[test]
    fn test_tiles_on_layer() {
        let mut map = MapEditorState::new("test_map", 10, 10, 32.0);
        map.place_tile(PlacedTile::new("grass", 0, 0, LayerKind::Ground))
            .unwrap();
        map.place_tile(PlacedTile::new("tree", 1, 1, LayerKind::Object))
            .unwrap();
        map.place_tile(PlacedTile::new("dirt", 2, 2, LayerKind::Ground))
            .unwrap();
        let ground_tiles: Vec<_> = map.tiles_on_layer(LayerKind::Ground).collect();
        assert_eq!(ground_tiles.len(), 2);
    }

    #[test]
    fn test_world_pos() {
        let tile = PlacedTile::new("grass", 2, 3, LayerKind::Ground);
        let pos = tile.world_pos(64.0);
        assert!((pos.x - 128.0).abs() < f32::EPSILON);
        assert!((pos.y - 192.0).abs() < f32::EPSILON);
    }
}
