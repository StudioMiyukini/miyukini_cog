// @id: MGE-ARPG-Skills @do: arpg-skills @role: back-end @layer: 3 @human: miyuk
//! ARPG skill system: skill trees, synergies, cooldowns, and investment tracking.
//!
//! This crate provides a D2-style skill system with:
//! - **Skill definitions** ([`SkillDef`]) describing costs, prerequisites, and synergies
//! - **Skill registry** ([`SkillRegistry`]) for indexing all available skills
//! - **Skill book** ([`SkillBook`]) tracking a character's invested skill levels
//! - **Cooldown tracking** ([`Cooldown`], [`SkillCooldownTracker`])
//! - **Synergy calculation** ([`SynergyCalculator`]) for passive inter-skill bonuses

mod book;
mod cooldown;
mod def;
mod registry;
mod skill_id;
mod synergy;

#[cfg(test)]
mod tests;

pub use book::SkillBook;
pub use cooldown::{Cooldown, SkillCooldownTracker};
pub use def::{SkillDef, SynergyDef};
pub use registry::SkillRegistry;
pub use skill_id::SkillId;
pub use synergy::SynergyCalculator;

/// Errors that can occur during skill investment and lookup.
#[derive(Debug, thiserror::Error)]
pub enum SkillError {
    /// The requested skill does not exist in the registry.
    #[error("Skill '{id}' not found in registry")]
    NotFound {
        /// The skill id that was not found.
        id: SkillId,
    },

    /// The skill has already reached its maximum investable level.
    #[error("Skill '{id}' is at maximum level {max}")]
    MaxLevelReached {
        /// The skill id that is at max level.
        id: SkillId,
        /// The maximum level cap.
        max: u32,
    },

    /// The character has no unspent skill points.
    #[error("No skill points available")]
    NoPointsAvailable,

    /// One or more prerequisite skills are not at level >= 1.
    #[error("Prerequisites not met for skill '{id}'")]
    PrerequisitesNotMet {
        /// The skill id whose prerequisites are not met.
        id: SkillId,
    },
}
