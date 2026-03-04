// @id: MUIE-BeltSlot @do: belt-slot-atom @role: atom @layer: 6 @human: miyuk

//! Belt potion slot -- one cell in the 4-column belt.
//!
//! Belt slots hold consumables (potions, scrolls). They show a key binding
//! label and optional quantity for stacks.

use egui::{Color32, Pos2, Response, Rounding, Stroke, Ui, Vec2};
use miyuki_ui_tokens::palette::d2::D2_PALETTE;
use crate::convert::rgba_to_color32;

/// Belt slot size in logical pixels.
pub const BELT_SLOT_SIZE: f32 = 36.0;

/// A single belt slot.
pub struct BeltSlot<'a> {
    /// Key binding label (e.g., "1", "2", "3", "4").
    pub key_label: &'a str,
    /// Whether an item is in this slot.
    pub occupied: bool,
    /// Stack quantity (0 if not applicable or empty).
    pub quantity: u32,
    /// Item name for tooltip (empty if no item).
    pub item_name: &'a str,
}

impl<'a> BeltSlot<'a> {
    /// Create a new empty belt slot.
    pub fn new(key_label: &'a str) -> Self {
        Self {
            key_label,
            occupied: false,
            quantity: 0,
            item_name: "",
        }
    }

    /// Set the slot as occupied with an item.
    pub fn item(mut self, name: &'a str, quantity: u32) -> Self {
        self.occupied = true;
        self.item_name = name;
        self.quantity = quantity;
        self
    }

    /// Draw the belt slot and return the egui [`Response`].
    pub fn show(self, ui: &mut Ui) -> Response {
        let size = Vec2::splat(BELT_SLOT_SIZE);
        let (response, painter) = ui.allocate_painter(size, egui::Sense::click());
        let rect = response.rect;

        // Background
        let bg = if response.hovered() {
            rgba_to_color32(&D2_PALETTE.bg_elevated)
        } else {
            rgba_to_color32(&D2_PALETTE.bg_surface)
        };
        painter.rect_filled(rect, Rounding::same(2.0), bg);

        // Border
        let border_color = rgba_to_color32(&D2_PALETTE.border_default);
        painter.rect_stroke(rect, Rounding::same(2.0), Stroke::new(1.0, border_color));

        // Item placeholder (colored rect for potions)
        if self.occupied {
            let item_rect = rect.shrink(5.0);
            painter.rect_filled(item_rect, Rounding::same(2.0), Color32::from_rgb(160, 30, 30));

            // Quantity
            if self.quantity > 0 {
                let qty_text = self.quantity.to_string();
                painter.text(
                    rect.center_bottom() + egui::vec2(0.0, -2.0),
                    egui::Align2::CENTER_BOTTOM,
                    &qty_text,
                    egui::FontId::proportional(9.0),
                    Color32::WHITE,
                );
            }
        }

        // Key binding label (top-left)
        let key_color = rgba_to_color32(&D2_PALETTE.accent_primary);
        painter.text(
            Pos2::new(rect.min.x + 2.0, rect.min.y + 2.0),
            egui::Align2::LEFT_TOP,
            self.key_label,
            egui::FontId::proportional(8.0),
            key_color,
        );

        // Tooltip
        if self.occupied && !self.item_name.is_empty() {
            response.clone().on_hover_text(self.item_name);
        }

        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_belt_slot_empty() {
        let slot = BeltSlot::new("1");
        assert_eq!(slot.key_label, "1");
        assert!(!slot.occupied);
        assert_eq!(slot.quantity, 0);
    }

    #[test]
    fn test_belt_slot_with_item() {
        let slot = BeltSlot::new("2").item("Potion de vie", 5);
        assert!(slot.occupied);
        assert_eq!(slot.quantity, 5);
        assert_eq!(slot.item_name, "Potion de vie");
    }

    #[test]
    fn test_belt_slot_size_positive() {
        assert!(BELT_SLOT_SIZE > 0.0);
    }
}
