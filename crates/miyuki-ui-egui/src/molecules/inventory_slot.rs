// @id: MUIE-InvSlot @do: inventory-slot-molecule @role: molecule @layer: 6 @human: miyuk

//! Inventory slot molecule -- a slot frame with an optional item icon.
//!
//! Combines [`SlotFrame`] with [`ItemIcon`] and provides hover/click
//! interaction for drag-and-drop and tooltip display.

use egui::{Response, Ui};
use crate::atoms::quality_text::ItemQuality;
use crate::atoms::slot_frame::{SlotFrame, SlotSize};

/// Data for an item in an inventory slot.
#[derive(Debug, Clone)]
pub struct SlotItemData {
    /// Item display name.
    pub name: String,
    /// Item quality.
    pub quality: ItemQuality,
    /// Whether the item has been identified.
    pub identified: bool,
}

/// An inventory slot molecule.
///
/// Draws a slot frame and, if an item is present, overlays the item icon.
/// Supports highlight state for valid drop targets during drag-and-drop.
pub struct InventorySlot<'a> {
    /// Slot grid size.
    pub size: SlotSize,
    /// Item in this slot (if any).
    pub item: Option<&'a SlotItemData>,
    /// Whether this slot is a valid drop target.
    pub drop_target: bool,
}

impl<'a> InventorySlot<'a> {
    /// Create a new empty inventory slot.
    pub fn new(size: SlotSize) -> Self {
        Self {
            size,
            item: None,
            drop_target: false,
        }
    }

    /// Set the item in this slot.
    pub fn item(mut self, item: &'a SlotItemData) -> Self {
        self.item = Some(item);
        self
    }

    /// Set whether this slot is a valid drop target.
    pub fn drop_target(mut self, target: bool) -> Self {
        self.drop_target = target;
        self
    }

    /// Draw the inventory slot and return the egui [`Response`].
    pub fn show(self, ui: &mut Ui) -> Response {
        let has_item = self.item.is_some();

        // Draw the slot frame
        let frame_resp = SlotFrame::new(self.size)
            .occupied(has_item)
            .highlighted(self.drop_target)
            .show(ui);

        // If there is an item, draw the icon on top
        // Note: In egui, we cannot easily overlay widgets. Instead, the slot
        // frame already handles drawing via the painter. The item icon content
        // is best drawn within the same painter call. For the molecule pattern,
        // we draw the item info as a tooltip.
        if let Some(item_data) = self.item {
            let tooltip_text = format!(
                "{}\n({})",
                item_data.name,
                quality_label(item_data.quality),
            );
            frame_resp.clone().on_hover_text(tooltip_text);
        }

        frame_resp
    }
}

/// Human-readable label for an item quality.
fn quality_label(q: ItemQuality) -> &'static str {
    match q {
        ItemQuality::Normal => "Normal",
        ItemQuality::Socketed => "Socketed",
        ItemQuality::Magic => "Magique",
        ItemQuality::Rare => "Rare",
        ItemQuality::Unique => "Unique",
        ItemQuality::Set => "Set",
        ItemQuality::Crafted => "Craft",
        ItemQuality::RuneWord => "Mot de Rune",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inventory_slot_empty() {
        let slot = InventorySlot::new(SlotSize::S1x1);
        assert!(slot.item.is_none());
        assert!(!slot.drop_target);
    }

    #[test]
    fn test_inventory_slot_with_item() {
        let data = SlotItemData {
            name: "Amulette de vie".to_string(),
            quality: ItemQuality::Magic,
            identified: true,
        };
        let slot = InventorySlot::new(SlotSize::S1x1).item(&data);
        assert!(slot.item.is_some());
    }

    #[test]
    fn test_quality_labels_all_valid() {
        let qualities = [
            ItemQuality::Normal, ItemQuality::Socketed, ItemQuality::Magic,
            ItemQuality::Rare, ItemQuality::Unique, ItemQuality::Set,
            ItemQuality::Crafted, ItemQuality::RuneWord,
        ];
        for q in qualities {
            assert!(!quality_label(q).is_empty());
        }
    }
}
