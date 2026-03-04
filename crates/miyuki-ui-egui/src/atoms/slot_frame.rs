// @id: MUIE-SlotFrame @do: slot-frame-atom @role: atom @layer: 6 @human: miyuk

//! Inventory and equipment slot frames.
//!
//! A slot frame is the basic rectangular container for items in grids,
//! equipment panels, and belt areas. The size is measured in grid cells,
//! with each cell being 29x29 logical pixels (D2 standard).

use egui::{Color32, Response, Rounding, Stroke, Ui, Vec2};
use miyuki_ui_tokens::palette::d2::D2_PALETTE;
use crate::convert::rgba_to_color32;

/// Cell size in logical pixels (D2 standard: 29x29 for inventory).
pub const CELL_PX: f32 = 29.0;

/// Slot size variants following D2 item grid sizes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlotSize {
    /// 1x1 (potions, gems, runes, rings, amulets).
    S1x1,
    /// 1x2 (sceptres, wands, some maces).
    S1x2,
    /// 1x3 (short swords, javelins).
    S1x3,
    /// 1x4 (short spears).
    S1x4,
    /// 2x2 (helms, small shields, boots, gloves, belts).
    S2x2,
    /// 2x3 (armors, large shields, bows).
    S2x3,
    /// 2x4 (longbows, polearms).
    S2x4,
}

impl SlotSize {
    /// Width in grid cells.
    pub const fn cols(self) -> u32 {
        match self {
            Self::S1x1 | Self::S1x2 | Self::S1x3 | Self::S1x4 => 1,
            Self::S2x2 | Self::S2x3 | Self::S2x4 => 2,
        }
    }

    /// Height in grid cells.
    pub const fn rows(self) -> u32 {
        match self {
            Self::S1x1 => 1,
            Self::S1x2 | Self::S2x2 => 2,
            Self::S1x3 | Self::S2x3 => 3,
            Self::S1x4 | Self::S2x4 => 4,
        }
    }

    /// Width in logical pixels.
    pub fn width_px(self) -> f32 {
        self.cols() as f32 * CELL_PX
    }

    /// Height in logical pixels.
    pub fn height_px(self) -> f32 {
        self.rows() as f32 * CELL_PX
    }

    /// Size as a Vec2 in logical pixels.
    pub fn to_vec2(self) -> Vec2 {
        Vec2::new(self.width_px(), self.height_px())
    }
}

/// A slot frame atom.
///
/// Draws a rectangular slot with D2-style dark background and gold/gray border.
/// The visual state changes based on whether the slot is occupied, highlighted,
/// or hovered.
pub struct SlotFrame {
    /// Slot dimensions.
    pub size: SlotSize,
    /// Whether an item currently occupies this slot.
    pub occupied: bool,
    /// Whether this slot is highlighted (e.g., valid drop target).
    pub highlighted: bool,
}

impl SlotFrame {
    /// Create a new empty slot frame.
    pub fn new(size: SlotSize) -> Self {
        Self {
            size,
            occupied: false,
            highlighted: false,
        }
    }

    /// Set the occupied state.
    pub fn occupied(mut self, occupied: bool) -> Self {
        self.occupied = occupied;
        self
    }

    /// Set the highlighted state.
    pub fn highlighted(mut self, highlighted: bool) -> Self {
        self.highlighted = highlighted;
        self
    }

    /// Draw the slot frame and return the egui [`Response`].
    pub fn show(self, ui: &mut Ui) -> Response {
        let size = self.size.to_vec2();
        let (response, painter) = ui.allocate_painter(size, egui::Sense::click_and_drag());
        let rect = response.rect;

        // Background color depends on state
        let bg_color = if self.highlighted {
            Color32::from_rgba_premultiplied(80, 65, 30, 180)
        } else if response.hovered() {
            rgba_to_color32(&D2_PALETTE.bg_elevated)
        } else if self.occupied {
            Color32::from_rgba_premultiplied(30, 25, 15, 200)
        } else {
            rgba_to_color32(&D2_PALETTE.bg_surface)
        };

        painter.rect_filled(rect, Rounding::same(1.0), bg_color);

        // Border color depends on state
        let border_color = if self.highlighted {
            rgba_to_color32(&D2_PALETTE.accent_primary_hover)
        } else if response.hovered() {
            rgba_to_color32(&D2_PALETTE.border_strong)
        } else {
            rgba_to_color32(&D2_PALETTE.border_default)
        };

        painter.rect_stroke(rect, Rounding::same(1.0), Stroke::new(1.0, border_color));

        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slot_size_dimensions() {
        assert_eq!(SlotSize::S1x1.cols(), 1);
        assert_eq!(SlotSize::S1x1.rows(), 1);
        assert_eq!(SlotSize::S2x4.cols(), 2);
        assert_eq!(SlotSize::S2x4.rows(), 4);
    }

    #[test]
    fn test_slot_size_pixels() {
        let size = SlotSize::S2x3;
        assert!((size.width_px() - 58.0).abs() < f32::EPSILON);
        assert!((size.height_px() - 87.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_cell_px_is_29() {
        assert!((CELL_PX - 29.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_slot_size_vec2() {
        let v = SlotSize::S1x1.to_vec2();
        assert!((v.x - 29.0).abs() < f32::EPSILON);
        assert!((v.y - 29.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_all_slot_sizes_positive() {
        let sizes = [
            SlotSize::S1x1, SlotSize::S1x2, SlotSize::S1x3, SlotSize::S1x4,
            SlotSize::S2x2, SlotSize::S2x3, SlotSize::S2x4,
        ];
        for size in sizes {
            assert!(size.width_px() > 0.0);
            assert!(size.height_px() > 0.0);
        }
    }

    #[test]
    fn test_slot_frame_builder() {
        let frame = SlotFrame::new(SlotSize::S2x2)
            .occupied(true)
            .highlighted(true);
        assert_eq!(frame.size, SlotSize::S2x2);
        assert!(frame.occupied);
        assert!(frame.highlighted);
    }
}
