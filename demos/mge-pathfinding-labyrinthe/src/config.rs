//! Configuration centralisée — MGE Démo Pathfinding

pub use crate::chunk::TILE_SIZE;

pub const WIDTH: usize = 800;
pub const HEIGHT: usize = 600;

pub const RESPAWN_DELAY: f32 = 6.0;
pub const PLAYER_SPEED: f32 = 120.0;

pub const MONSTER_VISION: f32 = 150.0;
pub const MONSTER_RECALC: f32 = 0.5;
pub const MONSTER_SPEED_MULTIPLIER: f32 = 0.7;
pub const MONSTER_WANDER_RADIUS: f32 = 5.0 * TILE_SIZE;
pub const MONSTER_WANDER_INTERVAL: f32 = 2.0;

pub const PLAYER_AOE_RADIUS: f32 = 20.0;
pub const PLAYER_AOE_COOLDOWN: f32 = 1.0;
pub const PLAYER_AOE_DMG_MIN: i32 = 1;
pub const PLAYER_AOE_DMG_MAX: i32 = 4;
