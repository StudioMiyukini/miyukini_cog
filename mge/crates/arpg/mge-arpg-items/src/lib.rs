// @id: MGE-ARPG-Items @do: item-system @role: back-end @layer: 3 @human: miyuk
//! ARPG item system: quality tiers, affixes, instances, inventory, and equipment.
//!
//! This crate provides the core item model for a Diablo 2-style ARPG:
//!
//! - **Quality tiers** (`Normal`, `Magic`, `Rare`, `Unique`, `Set`, `Crafted`)
//! - **Affixes** (prefix/suffix modifiers with random rolls)
//! - **Item instances** with durability, identification, and stack tracking
//! - **Grid inventory** (10x4 D2-style)
//! - **Equipment slots** (helm, armor, gloves, belt, boots, amulet, rings, weapons)

pub mod slot;
pub mod errors;
pub mod quality;
pub mod instance;
pub mod inventory;
pub mod equipment;

#[cfg(test)]
mod tests;

// Re-exports for convenience.
pub use errors::ItemError;
pub use quality::{Affix, AffixKind, ItemQuality};
pub use instance::ItemInstance;
pub use slot::ItemSlot;
pub use inventory::{Inventory, INV_COLS, INV_ROWS};
pub use equipment::Equipment;
