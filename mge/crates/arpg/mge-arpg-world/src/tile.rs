// @id: MGE-ARPG-World-Tile @do: tile-definition @role: back-end @layer: 3 @human: miyuk

//! Tile type definitions.

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

#[cfg(test)]
mod tests {
    use super::*;

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
}
