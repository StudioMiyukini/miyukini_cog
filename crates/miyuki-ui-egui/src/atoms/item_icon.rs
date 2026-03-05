// @id: MUIE-ItemIcon @do: item-icon-atom @role: atom @layer: 6 @human: miyuk

//! Item icon with quality-colored overlay border.
//!
//! Used inside inventory slots and equipment slots to represent an item
//! with its rarity tier visible at a glance.

use crate::atoms::quality_text::ItemQuality;
use crate::atoms::slot_frame::CELL_PX;
use egui::{Color32, Response, Rounding, Stroke, Ui, Vec2};

/// An item icon atom.
pub struct ItemIcon<'a> {
    /// Short display name (up to ~4 chars shown).
    pub name: &'a str,
    /// Item quality (determines border color).
    pub quality: ItemQuality,
    /// Width in grid cells.
    pub cols: u32,
    /// Height in grid cells.
    pub rows: u32,
    /// Whether the item has been identified.
    pub identified: bool,
}

impl<'a> ItemIcon<'a> {
    /// Create a new item icon.
    pub fn new(name: &'a str, quality: ItemQuality, cols: u32, rows: u32) -> Self {
        Self {
            name,
            quality,
            cols,
            rows,
            identified: true,
        }
    }

    /// Set the identified state.
    pub fn identified(mut self, identified: bool) -> Self {
        self.identified = identified;
        self
    }

    /// Draw the item icon and return the egui [`Response`].
    pub fn show(self, ui: &mut Ui) -> Response {
        let width = self.cols as f32 * CELL_PX;
        let height = self.rows as f32 * CELL_PX;
        let size = Vec2::new(width, height);

        let (response, painter) = ui.allocate_painter(size, egui::Sense::click_and_drag());
        let rect = response.rect;

        // Item background
        let bg = Color32::from_rgba_premultiplied(30, 25, 15, 200);
        painter.rect_filled(rect, Rounding::same(1.0), bg);

        // Quality-colored border
        let border_color = self.quality.color();
        painter.rect_stroke(rect, Rounding::same(1.0), Stroke::new(1.0, border_color));

        // Icon placeholder: abbreviated name or "?" if unidentified
        if self.identified {
            let short_name: String = self.name.chars().take(4).collect();
            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                &short_name,
                egui::FontId::proportional(9.0),
                border_color,
            );
        } else {
            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                "?",
                egui::FontId::proportional(10.0),
                Color32::from_rgb(200, 190, 160),
            );
        }

        // Hover highlight
        if response.hovered() {
            let hover_overlay = Color32::from_rgba_premultiplied(255, 255, 255, 15);
            painter.rect_filled(rect, Rounding::same(1.0), hover_overlay);
        }

        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_icon_builder() {
        let icon = ItemIcon::new("Windforce", ItemQuality::Unique, 2, 4).identified(true);
        assert_eq!(icon.name, "Windforce");
        assert_eq!(icon.quality, ItemQuality::Unique);
        assert_eq!(icon.cols, 2);
        assert_eq!(icon.rows, 4);
        assert!(icon.identified);
    }

    #[test]
    fn test_item_icon_unidentified() {
        let icon = ItemIcon::new("???", ItemQuality::Magic, 1, 1).identified(false);
        assert!(!icon.identified);
    }
}
