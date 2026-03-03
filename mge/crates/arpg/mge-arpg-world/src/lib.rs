// @id: MGE-ARPG-World @do: world-zones @role: back-end @layer: 3 @human: miyuk

//! ARPG world system: zones, tiles, chunks, waypoints, portals.
#![deny(unsafe_code)]

pub mod chunk;
pub mod dungeon;
pub mod error;
pub mod portal;
pub mod tile;
pub mod waypoint;
pub mod world_map;
pub mod zone;

pub use chunk::{Chunk, CHUNK_SIZE};
pub use dungeon::{DungeonDef, act1_dungeon_zones, act1_dungeons};
pub use error::WorldError;
pub use portal::Portal;
pub use tile::{Tile, TileKind};
pub use waypoint::{Waypoint, WaypointRegistry, act1_waypoints};
pub use world_map::{WorldMap, act1_monastery_zones};
pub use zone::{Zone, ZoneDef, ZoneId};
