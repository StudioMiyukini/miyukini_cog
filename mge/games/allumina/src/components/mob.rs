//! @id     games_allumina_components_mob
//! @do     definir_mob_tag
//! @role   ui
//! @layer  core
//! @human  games allumina — module « components mob ».

//! Monster components.

use serde::{Deserialize, Serialize};

use crate::components::Position2D;

/// Marker component: this entity is a hostile mob.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MobTag;

/// Links a live mob back to its static definition in `data::mobs`.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MobType {
    pub def_id: u32,
}

/// How a mob engages: drives target distance and attack style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AiBehavior {
    Melee,
    Ranged,
    Caster,
}

/// Finite-state-machine state of a mob's brain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AiState {
    Idle,
    Chase,
    Attack,
    Return,
    Dead,
}

/// Aggro/leash geometry plus the mob's spawn anchor (for `Return`).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AggroRange {
    /// Distance (tiles) at which an idle mob notices the player.
    pub aggro: f32,
    /// Distance from `home` past which the mob disengages and returns.
    pub leash: f32,
    pub behavior: AiBehavior,
    pub home: Position2D,
    pub state: AiState,
}

impl AggroRange {
    pub fn new(aggro: f32, leash: f32, behavior: AiBehavior, home: Position2D) -> Self {
        Self { aggro, leash, behavior, home, state: AiState::Idle }
    }
}

/// Per-boss special-attack state: independent slam/charge cooldowns plus a
/// one-way enrage latch that fires when the boss drops to low health.
#[derive(Debug, Clone, Copy)]
pub struct BossState {
    /// Seconds until the next ground slam.
    pub slam_cd: f32,
    /// Seconds until the next charge.
    pub charge_cd: f32,
    /// Set once the boss crosses the enrage threshold; never clears.
    pub enraged: bool,
}

impl BossState {
    /// Fresh state with both specials on their opening cooldown.
    #[must_use]
    pub fn new(slam_cd: f32, charge_cd: f32) -> Self {
        Self { slam_cd, charge_cd, enraged: false }
    }
}
