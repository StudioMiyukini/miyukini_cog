// @id: MGE-ARPG-Items-Slot @do: item-slot @role: back-end @layer: 3 @human: miyuk
//! Equipment and storage slot definitions.
//!
//! Each slot represents a location where an item can reside: an equipment
//! slot on the character, the inventory grid, or the stash.

/// A named slot where an item can be placed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ItemSlot {
    /// Head armor slot.
    Helm,
    /// Body armor slot.
    Armor,
    /// Hand armor slot.
    Gloves,
    /// Waist slot.
    Belt,
    /// Foot armor slot.
    Boots,
    /// Neck accessory slot.
    Amulet,
    /// First ring slot.
    Ring1,
    /// Second ring slot.
    Ring2,
    /// Primary weapon slot.
    WeaponMain,
    /// Off-hand weapon or shield slot.
    WeaponOff,
    /// In the inventory grid (backpack).
    Inventory,
    /// In the shared stash.
    Stash,
}
