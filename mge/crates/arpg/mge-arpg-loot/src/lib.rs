//! # mge-arpg-loot
//!
//! D2-style loot generation for the MGE ARPG pack (Sodomight).
//!
//! This crate provides:
//! - **Treasure Classes** — weighted drop tables with NoDrop support
//! - **Recursive TC resolution** — nested TCs up to 8 levels deep
//! - **Magic Find quality rolling** — item quality influenced by MF%
//! - **Gold drops** — direct quantity-based gold generation
//!
//! All identifiers are plain `String` IDs; this crate intentionally avoids
//! depending on `mge-arpg-items` or `mge-arpg-stats` to stay decoupled.
//!
//! @id MGE-ARPG-Loot
//! @do arpg-loot
//! @role back-end
//! @layer 3
//! @human miyuk

pub mod drop;
pub mod entry;
pub mod quality;
pub mod registry;

#[cfg(test)]
mod tests;

// ---- Re-exports for ergonomic use ----------------------------------------

pub use drop::{DropRoll, LootGenerator};
pub use entry::{DropEntry, TreasureClass};
pub use quality::{effective_mf, roll_quality, DropQuality, QualityRoller};
pub use registry::TreasureClassRegistry;

/// Errors that can occur during loot generation.
#[derive(Debug, thiserror::Error)]
pub enum LootError {
    /// The requested treasure class was not found in the registry.
    #[error("Treasure class '{id}' not found")]
    TcNotFound {
        /// The identifier that could not be resolved.
        id: String,
    },

    /// Nested TC resolution exceeded the maximum allowed depth.
    #[error("Recursion depth exceeded for TC '{id}'")]
    RecursionLimit {
        /// The identifier at which the limit was hit.
        id: String,
    },

    /// The treasure class exists but contains no drop entries.
    #[error("Treasure class '{id}' has no entries")]
    EmptyTc {
        /// The empty TC's identifier.
        id: String,
    },
}
