// @id: MGE-ARPG-World-Zone @do: zone @role: back-end @layer: 3 @human: miyuk

//! Zone definition and runtime zone instance.

use std::collections::HashMap;

use crate::chunk::{Chunk, CHUNK_SIZE};
use crate::portal::Portal;
use crate::waypoint::Waypoint;

/// Unique zone identifier (e.g. "act1_cold_plains").
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct ZoneId(pub String);

impl ZoneId {
    /// Create a new zone identifier.
    #[must_use]
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Return the zone identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Static zone configuration (loaded from TOML).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ZoneDef {
    /// Unique identifier for this zone.
    pub id: ZoneId,
    /// Display name for the zone.
    pub name: String,
    /// Act number (1..=5).
    pub act: u8,
    /// Recommended player level (1..=99).
    pub difficulty: u8,
    /// Monster level for loot and XP calculations.
    pub monster_level: u8,
    /// Zone width in chunks.
    pub width_chunks: u32,
    /// Zone height in chunks.
    pub height_chunks: u32,
    /// BGM audio track identifier.
    pub ambient_id: String,
}

impl ZoneDef {
    /// Width in tiles.
    #[must_use]
    pub fn width_tiles(&self) -> u32 {
        self.width_chunks * CHUNK_SIZE as u32
    }

    /// Height in tiles.
    #[must_use]
    pub fn height_tiles(&self) -> u32 {
        self.height_chunks * CHUNK_SIZE as u32
    }
}

/// Tile size in world units (pixels).
pub const TILE_SIZE: f32 = 64.0;

/// Runtime zone instance with loaded chunks, waypoints, and portals.
#[derive(Debug)]
pub struct Zone {
    /// Static zone definition.
    pub def: ZoneDef,
    chunks: HashMap<(u32, u32), Chunk>,
    /// Waypoints within this zone.
    pub waypoints: Vec<Waypoint>,
    /// Portals within this zone.
    pub portals: Vec<Portal>,
}

impl Zone {
    /// Create a new zone instance from a definition.
    #[must_use]
    pub fn new(def: ZoneDef) -> Self {
        Self {
            def,
            chunks: HashMap::new(),
            waypoints: Vec::new(),
            portals: Vec::new(),
        }
    }

    /// Insert a chunk at chunk grid coordinates `(cx, cy)`.
    pub fn set_chunk(&mut self, cx: u32, cy: u32, chunk: Chunk) {
        self.chunks.insert((cx, cy), chunk);
    }

    /// Get the chunk at chunk coordinates.
    #[must_use]
    pub fn get_chunk(&self, cx: u32, cy: u32) -> Option<&Chunk> {
        self.chunks.get(&(cx, cy))
    }

    /// Returns `true` if the world-space position is on a walkable tile.
    #[must_use]
    pub fn is_walkable_world(&self, wx: f32, wy: f32) -> bool {
        if wx < 0.0 || wy < 0.0 {
            return false;
        }
        let tile_x = (wx / TILE_SIZE) as u32;
        let tile_y = (wy / TILE_SIZE) as u32;
        let cx = tile_x / CHUNK_SIZE as u32;
        let cy = tile_y / CHUNK_SIZE as u32;
        let local_x = (tile_x % CHUNK_SIZE as u32) as usize;
        let local_y = (tile_y % CHUNK_SIZE as u32) as usize;
        self.chunks
            .get(&(cx, cy))
            .is_some_and(|c| c.is_walkable(local_x, local_y))
    }

    /// Add a waypoint to this zone.
    pub fn add_waypoint(&mut self, wp: Waypoint) {
        self.waypoints.push(wp);
    }

    /// Add a portal to this zone.
    pub fn add_portal(&mut self, portal: Portal) {
        self.portals.push(portal);
    }

    /// Find a waypoint by its identifier.
    #[must_use]
    pub fn find_waypoint(&self, id: &str) -> Option<&Waypoint> {
        self.waypoints.iter().find(|w| w.id == id)
    }

    /// Find a mutable waypoint by its identifier.
    #[must_use]
    pub fn find_waypoint_mut(&mut self, id: &str) -> Option<&mut Waypoint> {
        self.waypoints.iter_mut().find(|w| w.id == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mge_math::Vec2;

    fn make_zone_def() -> ZoneDef {
        ZoneDef {
            id: ZoneId::new("test_zone"),
            name: "Test Zone".to_string(),
            act: 1,
            difficulty: 10,
            monster_level: 12,
            width_chunks: 2,
            height_chunks: 3,
            ambient_id: "bgm_test".to_string(),
        }
    }

    #[test]
    fn test_zone_new_empty() {
        let zone = Zone::new(make_zone_def());
        assert!(zone.waypoints.is_empty());
        assert!(zone.portals.is_empty());
        assert!(zone.get_chunk(0, 0).is_none());
    }

    #[test]
    fn test_zone_set_and_get_chunk() {
        let mut zone = Zone::new(make_zone_def());
        let chunk = Chunk::new_floor(1);
        zone.set_chunk(0, 0, chunk);
        assert!(zone.get_chunk(0, 0).is_some());
        assert!(zone.get_chunk(1, 1).is_none());
    }

    #[test]
    fn test_zone_walkable_world_no_chunk() {
        let zone = Zone::new(make_zone_def());
        // No chunks loaded, so nothing is walkable.
        assert!(!zone.is_walkable_world(32.0, 32.0));
        // Negative coordinates are also not walkable.
        assert!(!zone.is_walkable_world(-1.0, 0.0));
    }

    #[test]
    fn test_zone_walkable_world_with_chunk() {
        let mut zone = Zone::new(make_zone_def());
        zone.set_chunk(0, 0, Chunk::new_floor(1));
        // Position (32.0, 32.0) => tile (0, 0) => chunk (0, 0) local (0, 0) => floor => true
        assert!(zone.is_walkable_world(32.0, 32.0));
    }

    #[test]
    fn test_zone_def_width_tiles() {
        let def = make_zone_def();
        // width_chunks=2, CHUNK_SIZE=16 => 32
        assert_eq!(def.width_tiles(), 32);
        // height_chunks=3 => 48
        assert_eq!(def.height_tiles(), 48);
    }

    #[test]
    fn test_zone_add_and_find_waypoint() {
        let mut zone = Zone::new(make_zone_def());
        let wp = Waypoint::new(
            "wp1",
            ZoneId::new("test_zone"),
            Vec2::new(100.0, 200.0),
        );
        zone.add_waypoint(wp);
        assert!(zone.find_waypoint("wp1").is_some());
        assert!(zone.find_waypoint("nonexistent").is_none());
    }
}
