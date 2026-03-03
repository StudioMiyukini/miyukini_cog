// @id: MGE-ARPG-Items-Inventory @do: inventory-grid @role: back-end @layer: 3 @human: miyuk
//! Grid-based inventory system (10 columns x 4 rows, D2-style).
//!
//! Each cell holds at most one `ItemInstance`. Items occupy a single cell
//! (multi-cell footprints are not yet supported).

use crate::errors::ItemError;
use crate::instance::ItemInstance;

/// Number of columns in the inventory grid.
pub const INV_COLS: usize = 10;
/// Number of rows in the inventory grid.
pub const INV_ROWS: usize = 4;

/// Total number of cells in the inventory grid.
const INV_SIZE: usize = INV_COLS * INV_ROWS;

/// A grid-based inventory holding up to `INV_COLS * INV_ROWS` items.
///
/// Slots are addressed by `(col, row)` where `col < INV_COLS` and `row < INV_ROWS`.
#[derive(Debug, Clone)]
pub struct Inventory {
    slots: Vec<Option<ItemInstance>>,
}

impl Default for Inventory {
    fn default() -> Self {
        Self::new()
    }
}

impl Inventory {
    /// Create an empty inventory with all slots vacant.
    #[must_use]
    pub fn new() -> Self {
        let mut slots = Vec::with_capacity(INV_SIZE);
        slots.resize_with(INV_SIZE, || None);
        Self { slots }
    }

    /// Convert `(col, row)` to a flat index, or `None` if out of bounds.
    fn index(col: usize, row: usize) -> Option<usize> {
        if col < INV_COLS && row < INV_ROWS {
            Some(row * INV_COLS + col)
        } else {
            None
        }
    }

    /// Get a reference to the item at `(col, row)`, if any.
    ///
    /// Returns `None` if the slot is empty or coordinates are out of bounds.
    #[must_use]
    pub fn get(&self, col: usize, row: usize) -> Option<&ItemInstance> {
        let idx = Self::index(col, row)?;
        self.slots[idx].as_ref()
    }

    /// Attempt to place an item at the given `(col, row)`.
    ///
    /// # Errors
    /// - `ItemError::OutOfBounds` if coordinates exceed the grid.
    /// - `ItemError::SlotOccupied` if the cell already contains an item.
    pub fn try_place(
        &mut self,
        item: ItemInstance,
        col: usize,
        row: usize,
    ) -> Result<(), ItemError> {
        let idx = Self::index(col, row).ok_or(ItemError::OutOfBounds { col, row })?;
        if self.slots[idx].is_some() {
            return Err(ItemError::SlotOccupied { col, row });
        }
        self.slots[idx] = Some(item);
        Ok(())
    }

    /// Remove and return the item at `(col, row)`, or `None` if empty / out of bounds.
    pub fn remove(&mut self, col: usize, row: usize) -> Option<ItemInstance> {
        let idx = Self::index(col, row)?;
        self.slots[idx].take()
    }

    /// Find the first free slot, scanning left-to-right then top-to-bottom.
    ///
    /// Returns `Some((col, row))` or `None` if the inventory is full.
    #[must_use]
    pub fn find_free_slot(&self) -> Option<(usize, usize)> {
        for (i, slot) in self.slots.iter().enumerate() {
            if slot.is_none() {
                let col = i % INV_COLS;
                let row = i / INV_COLS;
                return Some((col, row));
            }
        }
        None
    }

    /// Number of items currently stored in the inventory.
    #[must_use]
    pub fn item_count(&self) -> usize {
        self.slots.iter().filter(|s| s.is_some()).count()
    }

    /// Returns `true` if every slot is occupied.
    #[must_use]
    pub fn is_full(&self) -> bool {
        self.slots.iter().all(Option::is_some)
    }

    /// Find the first item matching `base_id`, scanning left-to-right then top-to-bottom.
    ///
    /// Returns `Some((col, row))` or `None` if not found.
    #[must_use]
    pub fn find_item(&self, base_id: &str) -> Option<(usize, usize)> {
        for (i, slot) in self.slots.iter().enumerate() {
            if let Some(item) = slot {
                if item.base_id == base_id {
                    let col = i % INV_COLS;
                    let row = i / INV_COLS;
                    return Some((col, row));
                }
            }
        }
        None
    }

    /// Read-only access to the full slot array for GUI rendering.
    #[must_use]
    pub fn slots(&self) -> &[Option<ItemInstance>] {
        &self.slots
    }
}
