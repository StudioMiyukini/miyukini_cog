// @id: MGE-ARPG-Combat @do: arpg-combat-root @role: back-end @layer: 3 @human: miyuk
//! # mge-arpg-combat
//!
//! ARPG combat pipeline for the MGE Sodomight game.
//!
//! Provides hit-chance calculations (Diablo 2 style), damage rolling with
//! critical hits and elemental resistances, combat event generation, and
//! temporary status effects.
//!
//! This crate is intentionally self-contained: it does **not** depend on
//! `mge-arpg-stats` and re-declares the minimal stat blocks it needs
//! locally ([`AttackerStats`], [`DefenderStats`]).

pub mod calculator;
pub mod damage;
pub mod events;
pub mod hit;
pub mod processor;
pub mod status;

#[cfg(test)]
mod tests;

// ----- Re-exports for ergonomic top-level access ----- //

pub use calculator::DamageCalculator;
pub use damage::{AttackerStats, DamageRoll, DamageType, DefenderStats};
pub use events::{CombatEvent, CombatResult};
pub use hit::HitChance;
pub use processor::CombatProcessor;
pub use status::StatusEffect;

use thiserror::Error;

/// Errors that can occur inside the combat pipeline.
#[derive(Debug, Error)]
pub enum CombatError {
    /// An attacker or defender ID was not found in the world.
    #[error("entity {0} not found")]
    EntityNotFound(u32),

    /// An invalid stat value was provided.
    #[error("invalid stat: {0}")]
    InvalidStat(String),
}
