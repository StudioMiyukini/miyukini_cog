//! @id     games_allumina_components_projectile
//! @do     definir_projectile
//! @role   ui
//! @layer  core
//! @human  games allumina — module « components projectile ».

//! In-flight projectiles spawned by ranged auto-attacks. Stored on the
//! [`crate::ecs::World`] as a flat `Vec`: they aren't ECS entities (no id,
//! no components — they're short-lived and homing).

use crate::ecs::Entity;

/// A single homing projectile travelling from `owner` toward `target`.
///
/// Damage is rolled at spawn time (so the projectile carries a fixed payload),
/// then mitigated by the target's defence at impact. Magic projectiles bypass
/// physical defence; physical projectiles run the standard defence reduction.
#[derive(Debug, Clone, Copy)]
pub struct Projectile {
    pub owner: Entity,
    pub target: Entity,
    pub x: f32,
    pub y: f32,
    /// Tiles per second (15.0 for arrows, 12.0 for magic foci).
    pub speed: f32,
    /// Damage rolled at spawn (pre-mitigation, post-crit).
    pub damage: f32,
    /// `true` → bypasses physical defence (magic foci).
    pub is_magic: bool,
    /// Whether the spawn-time crit roll succeeded — surfaced to the log.
    pub crit: bool,
    /// Tiles travelled so far. Projectiles disappear past `MAX_RANGE_TILES`.
    pub travelled: f32,
}

/// Maximum total flight distance (tiles) before a projectile fizzles.
pub const MAX_RANGE_TILES: f32 = 15.0;

/// Impact distance (tiles): below this the projectile resolves on the target.
pub const HIT_RADIUS: f32 = 0.3;
