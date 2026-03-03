// @id: Sodomight-World @do: gameplay-world @role: back-end @layer: 4 @human: miyuk
//! Sodomight game world -- connects all gameplay systems (ECS, stats, combat,
//! items, skills, loot, quests, AI) into a single cohesive runtime.
//!
//! The [`SodomightWorld`] struct owns the full game state and provides the
//! high-level API for the game loop: spawning, ticking, combat, inventory,
//! skill usage, and XP/levelling.

pub mod types;

mod ai_tick;
mod combat;
mod inventory;
mod player;
mod spawn;
mod state;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod tests_advanced;

// Re-export public types at the `world` module level for API compatibility.
pub use state::SodomightWorld;
pub use types::{
    Difficulty, MonsterRecord, PlayerRecord, TownPortal, WorldError,
    cast_town_portal, xp_death_penalty,
};
