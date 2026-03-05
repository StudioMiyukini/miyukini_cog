// @id: MUIE-BeltCol @do: belt-column-molecule @role: molecule @layer: 6 @human: miyuk

//! Belt column molecule -- a vertical stack of 4 belt slots.
//!
//! In D2, the belt has 4 columns (keys 1-4), each with 1-4 rows depending
//! on belt type. This molecule draws one column.

use crate::atoms::belt_slot::BeltSlot;
use egui::{Response, Ui};

/// Data for one potion/item in a belt slot.
#[derive(Debug, Clone, Default)]
pub struct BeltSlotData {
    /// Item name (empty string if empty slot).
    pub item_name: String,
    /// Stack quantity (0 if empty).
    pub quantity: u32,
}

/// A belt column molecule (4 slots vertically stacked).
pub struct BeltColumn<'a> {
    /// Column index (0-3), determines key labels (1-4).
    pub column: u32,
    /// Slot data for each row (up to 4 rows).
    pub slots: &'a [BeltSlotData],
}

impl<'a> BeltColumn<'a> {
    /// Create a new belt column.
    pub fn new(column: u32, slots: &'a [BeltSlotData]) -> Self {
        Self { column, slots }
    }

    /// Draw the belt column.
    ///
    /// Returns the response of the column layout (for layout purposes).
    pub fn show(self, ui: &mut Ui) -> Response {
        let key_base = self.column + 1;

        let inner = ui.vertical(|ui| {
            for (row_idx, slot_data) in self.slots.iter().enumerate() {
                let key_label = format!("{key_base}");

                let slot = if slot_data.quantity > 0 {
                    BeltSlot::new(&key_label).item(&slot_data.item_name, slot_data.quantity)
                } else {
                    BeltSlot::new(&key_label)
                };

                let _resp = slot.show(ui);
                // Only the bottom slot shows the key label in D2
                // but for simplicity, each slot shows it.
                let _ = row_idx;
            }
        });

        inner.response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_belt_slot_data_default() {
        let data = BeltSlotData::default();
        assert!(data.item_name.is_empty());
        assert_eq!(data.quantity, 0);
    }

    #[test]
    fn test_belt_column_creation() {
        let slots = vec![
            BeltSlotData {
                item_name: "Potion".to_string(),
                quantity: 5,
            },
            BeltSlotData::default(),
        ];
        let col = BeltColumn::new(0, &slots);
        assert_eq!(col.column, 0);
        assert_eq!(col.slots.len(), 2);
    }
}
