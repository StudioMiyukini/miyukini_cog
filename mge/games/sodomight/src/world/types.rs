// @id: Sodomight-World-Types @do: world-types @role: back-end @layer: 4 @human: miyuk
//! Constants, record types, error enum, difficulty, and town portal.

use mge_arpg_entity::{Health, Level, Position, Team};

/// Milliseconds per game tick at the default 25 Hz tick rate.
pub(crate) const TICK_DELTA_MS: u32 = 40;

/// Default aggro sight range for monsters (world units).
pub(crate) const DEFAULT_SIGHT_RANGE: f32 = 8.0;

/// Default attack range for monsters (world units).
pub(crate) const DEFAULT_ATTACK_RANGE: f32 = 1.5;

/// Maximum melee attack range for the player (world units).
/// Attacks beyond this distance are rejected with `WorldError::TooFar`.
pub(crate) const PLAYER_MELEE_RANGE: f32 = 2.5;

/// Default XP base reward per monster kill.
pub(crate) const BASE_XP_PER_KILL: u64 = 50;

/// Minimum number of ticks between successive monster attacks (25 Hz -> 2.0s).
pub(crate) const ATTACK_COOLDOWN_TICKS: u64 = 50;

/// Monster chase speed in world units per tick.
pub(crate) const MONSTER_MOVE_SPEED: f32 = 0.04;

/// Ticks between life regeneration ticks (25 Hz -> every 4 seconds).
pub(crate) const LIFE_REGEN_INTERVAL: u64 = 100;

/// Ticks between mana regeneration ticks (25 Hz -> every 2 seconds).
pub(crate) const MANA_REGEN_INTERVAL: u64 = 50;

/// Life restored per regen tick.
pub(crate) const LIFE_REGEN_AMOUNT: i32 = 1;

/// Mana restored per regen tick.
pub(crate) const MANA_REGEN_AMOUNT: i32 = 2;

/// Lightweight ECS component bundling all data needed for a monster entity.
#[derive(Debug, Clone)]
pub struct MonsterRecord {
    /// Display name.
    pub name: String,
    /// World position.
    pub position: Position,
    /// Hit points.
    pub health: Health,
    /// Monster level.
    pub level: Level,
    /// Team affiliation.
    pub team: Team,
}

/// Lightweight ECS component for the player entity.
#[derive(Debug, Clone)]
pub struct PlayerRecord {
    /// World position.
    pub position: Position,
    /// Hit points.
    pub health: Health,
    /// Character level.
    pub level: Level,
    /// Team affiliation.
    pub team: Team,
}

/// Errors that can occur during world operations.
#[derive(Debug, thiserror::Error)]
pub enum WorldError {
    /// The target entity was not found or is dead.
    #[error("Entity not found or dead")]
    EntityNotFound,

    /// The player's inventory is full.
    #[error("Inventory is full")]
    InventoryFull,

    /// The specified loot index is invalid.
    #[error("Invalid loot index: {0}")]
    InvalidLootIndex(usize),

    /// The requested skill is not known or cannot be used.
    #[error("Skill error: {0}")]
    SkillError(String),

    /// The player does not have enough mana.
    #[error("Not enough mana (need {need}, have {have})")]
    NotEnoughMana {
        /// Mana required.
        need: i32,
        /// Mana currently available.
        have: i32,
    },

    /// The skill is still on cooldown.
    #[error("Skill '{0}' is on cooldown")]
    OnCooldown(String),

    /// An ECS operation failed.
    #[error("ECS error: {0}")]
    EcsError(String),

    /// The player is too far from the target.
    #[error("Too far to pick up item (distance {distance:.2}, max {max:.2})")]
    TooFar {
        /// Actual distance to the target.
        distance: f32,
        /// Maximum allowed distance.
        max: f32,
    },

    /// The target equipment slot is invalid.
    #[error("Invalid equipment slot")]
    InvalidEquipSlot,

    /// The inventory slot is empty or out of bounds.
    #[error("Inventory slot empty or out of bounds")]
    InventorySlotEmpty,
}

/// Game difficulty, determining XP loss on player death (D2-style).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Difficulty {
    /// No XP loss on death.
    #[default]
    Normal,
    /// 5 % of the current level's XP range is lost on death.
    Nightmare,
    /// 10 % of the current level's XP range is lost on death.
    Hell,
}

/// Compute the XP penalty incurred on death for a given difficulty.
#[must_use]
pub fn xp_death_penalty(
    difficulty: Difficulty,
    current_xp: u64,
    level: u32,
    table: &mge_arpg_stats::ExpTable,
) -> u64 {
    if difficulty == Difficulty::Normal || level == 0 {
        return current_xp;
    }
    let floor_xp = table.xp_for_level(level).unwrap_or(0);
    let ceil_xp = table.xp_for_level(level + 1).unwrap_or(current_xp);
    let range = ceil_xp.saturating_sub(floor_xp);
    let penalty = match difficulty {
        Difficulty::Normal => 0,
        Difficulty::Nightmare => range / 20,
        Difficulty::Hell => range / 10,
    };
    current_xp.saturating_sub(penalty).max(floor_xp)
}

/// A one-use portal that remembers where the player was in the field.
#[derive(Debug, Clone)]
pub struct TownPortal {
    /// The zone the player will return to when re-entering the portal.
    pub return_zone: String,
    /// The `(x, y)` position the player will land at on return.
    pub return_position: (f32, f32),
    /// Whether the portal is still usable.
    pub active: bool,
}

impl TownPortal {
    /// Consume the portal, teleporting the player back to the field.
    pub fn use_portal(&mut self) {
        self.active = false;
    }
}

/// Cast a new Town Portal scroll.
#[must_use]
pub fn cast_town_portal(zone_id: &str, position: (f32, f32)) -> TownPortal {
    TownPortal {
        return_zone: zone_id.to_string(),
        return_position: position,
        active: true,
    }
}
