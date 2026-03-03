// @id: MGE-ARPG-Stats @do: arpg-stats @role: back-end @layer: 3 @human: miyuk
//! ARPG stat system: base attributes, derived formulas, leveling, and stat blocks.
//!
//! Designed for D2-style action RPGs (Sodomight). Provides:
//! - [`StatValue`] -- single stat with additive/multiplicative modifiers
//! - [`BaseStats`] -- four primary attributes (STR, DEX, VIT, ENE)
//! - [`DerivedStats`] -- computed secondary stats (life, mana, resistances, etc.)
//! - [`CharacterLevel`] / [`ExpTable`] -- experience and leveling
//! - [`StatBlock`] -- full entity stat profile
//! - [`Resistances`] / [`Absorb`] -- elemental resistance system with difficulty penalties
//! - [`damage_after_resist`] / [`difficulty_resist_penalty`] -- damage calculation helpers

pub mod base;
pub mod block;
pub mod breakpoints;
pub mod class;
pub mod derived;
pub mod level;
pub mod resistances;
pub mod stat_value;

#[cfg(test)]
mod tests;

pub use base::BaseStats;
pub use block::StatBlock;
pub use breakpoints::BreakpointTable;
pub use class::CharacterClass;
pub use derived::DerivedStats;
pub use level::{CharacterLevel, ExpTable};
pub use resistances::{
    damage_after_resist, difficulty_resist_penalty, Absorb, Resistances,
};
pub use stat_value::StatValue;

/// Errors that can occur in stat operations.
#[derive(Debug, thiserror::Error)]
pub enum StatsError {
    /// A stat point allocation was requested but none are available.
    #[error("no stat points available")]
    NoStatPoints,

    /// A skill point allocation was requested but none are available.
    #[error("no skill points available")]
    NoSkillPoints,

    /// The requested level is out of the valid range (1..=99).
    #[error("level {0} is out of range (1..=99)")]
    LevelOutOfRange(u32),

    /// A negative or otherwise invalid amount was provided.
    #[error("invalid amount: {0}")]
    InvalidAmount(i32),
}
