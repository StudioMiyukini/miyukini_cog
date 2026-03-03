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
pub mod uniques;
pub mod identify;
pub mod sockets;
pub mod runewords;

#[cfg(test)]
mod tests;

// Re-exports for convenience.
pub use errors::ItemError;
pub use identify::{IdentifiableItem, IdentifyError, IdentifyMethod};
pub use quality::{
    Affix, AffixDef, AffixInstance, AffixKind, ItemQuality, MagicItemResult, RareItemResult,
    AFFIX_POOL_PREFIXES, AFFIX_POOL_SUFFIXES, RARE_PREFIXES, RARE_SUFFIXES,
    generate_magic_item, generate_rare_item,
};
pub use instance::ItemInstance;
pub use slot::ItemSlot;
pub use inventory::{Inventory, INV_COLS, INV_ROWS};
pub use equipment::Equipment;
pub use uniques::{
    UniqueItemDef, UniqueItemInstance, ACT1_UNIQUES,
    all_act1_uniques, generate_unique_item, get_unique_def,
};
pub use sockets::{
    GemQuality, GemType, SocketError, SocketFiller, SocketedItem,
    max_sockets_for_base,
};
pub use runewords::{
    RunewordDef, all_runewords, check_runeword,
    rune_name_to_id, rune_id_to_name,
};
