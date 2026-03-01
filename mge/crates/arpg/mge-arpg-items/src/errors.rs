// @id: MGE-ARPG-Items-Errors @do: item-errors @role: back-end @layer: 3 @human: miyuk
//! Error types for the item system.

use crate::slot::ItemSlot;

/// Errors that can occur when manipulating items, inventory, or equipment.
#[derive(Debug, thiserror::Error)]
pub enum ItemError {
    /// The requested grid coordinates exceed the inventory dimensions.
    #[error("Slot ({col}, {row}) is out of bounds")]
    OutOfBounds {
        /// Column index that was requested.
        col: usize,
        /// Row index that was requested.
        row: usize,
    },

    /// The target inventory cell already contains an item.
    #[error("Slot ({col}, {row}) is already occupied")]
    SlotOccupied {
        /// Column index of the occupied cell.
        col: usize,
        /// Row index of the occupied cell.
        row: usize,
    },

    /// No free slot remains in the inventory grid.
    #[error("Inventory is full")]
    InventoryFull,

    /// The item cannot be placed in the specified equipment slot.
    #[error("Item {id} cannot go in slot {slot:?}")]
    WrongSlot {
        /// The item's base definition identifier.
        id: String,
        /// The slot that was attempted.
        slot: ItemSlot,
    },
}
