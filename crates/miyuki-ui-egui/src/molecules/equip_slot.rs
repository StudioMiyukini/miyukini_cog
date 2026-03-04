// @id: MUIE-EquipSlot @do: equip-slot-molecule @role: molecule @layer: 6 @human: miyuk

//! Equipment slot molecule -- paperdoll slot with label and item display.
//!
//! Used in the inventory panel's equipment area. Each slot has a fixed
//! position name (e.g., "Tete", "Corps") and can hold one item.

use egui::{Color32, Response, Rounding, Stroke, Ui, Vec2};
use miyuki_ui_tokens::palette::d2::D2_PALETTE;
use crate::atoms::quality_text::ItemQuality;
use crate::convert::rgba_to_color32;

/// Equipment slot position on the paperdoll.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EquipPosition {
    /// Head slot (helm).
    Head,
    /// Body slot (armor).
    Body,
    /// Belt slot.
    Belt,
    /// Boots slot.
    Boots,
    /// Gloves slot.
    Gloves,
    /// Amulet slot.
    Amulet,
    /// Left ring slot.
    RingLeft,
    /// Right ring slot.
    RingRight,
    /// Main hand weapon slot.
    MainHand,
    /// Off hand (shield/weapon) slot.
    OffHand,
}

impl EquipPosition {
    /// French label for the slot.
    pub fn label(self) -> &'static str {
        match self {
            Self::Head => "Tete",
            Self::Body => "Corps",
            Self::Belt => "Ceinture",
            Self::Boots => "Bottes",
            Self::Gloves => "Gants",
            Self::Amulet => "Amulette",
            Self::RingLeft => "Anneau G",
            Self::RingRight => "Anneau D",
            Self::MainHand => "Main G",
            Self::OffHand => "Main D",
        }
    }

    /// Slot visual size in pixels.
    pub fn slot_size(self) -> Vec2 {
        match self {
            Self::Amulet | Self::RingLeft | Self::RingRight => Vec2::new(29.0, 29.0),
            Self::Belt => Vec2::new(58.0, 29.0),
            Self::Head | Self::Boots | Self::Gloves => Vec2::new(58.0, 58.0),
            Self::Body => Vec2::new(58.0, 87.0),
            Self::MainHand | Self::OffHand => Vec2::new(58.0, 116.0),
        }
    }
}

/// Data for an equipped item.
#[derive(Debug, Clone)]
pub struct EquipItemData {
    /// Item display name.
    pub name: String,
    /// Item quality.
    pub quality: ItemQuality,
}

/// An equipment slot molecule.
pub struct EquipSlot<'a> {
    /// Slot position.
    pub position: EquipPosition,
    /// Equipped item (if any).
    pub item: Option<&'a EquipItemData>,
}

impl<'a> EquipSlot<'a> {
    /// Create a new equipment slot.
    pub fn new(position: EquipPosition) -> Self {
        Self {
            position,
            item: None,
        }
    }

    /// Set the equipped item.
    pub fn item(mut self, item: &'a EquipItemData) -> Self {
        self.item = Some(item);
        self
    }

    /// Draw the equipment slot and return the egui [`Response`].
    pub fn show(self, ui: &mut Ui) -> Response {
        let size = self.position.slot_size();
        let (response, painter) = ui.allocate_painter(size, egui::Sense::hover());
        let rect = response.rect;

        // Background
        let bg = if response.hovered() {
            rgba_to_color32(&D2_PALETTE.bg_elevated)
        } else {
            rgba_to_color32(&D2_PALETTE.bg_surface)
        };
        painter.rect_filled(rect, Rounding::same(3.0), bg);

        // Border
        let border = rgba_to_color32(&D2_PALETTE.border_default);
        painter.rect_stroke(rect, Rounding::same(3.0), Stroke::new(1.0, border));

        if let Some(item_data) = self.item {
            // Draw item placeholder
            painter.rect_filled(rect.shrink(4.0), Rounding::same(2.0), Color32::from_rgb(50, 40, 20));
            let short_name: String = item_data.name.chars().take(4).collect();
            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                &short_name,
                egui::FontId::proportional(8.0),
                item_data.quality.color(),
            );

            // Tooltip
            let tooltip = format!("{} ({})", item_data.name, self.position.label());
            response.clone().on_hover_text(tooltip);
        } else {
            // Empty slot label
            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                self.position.label(),
                egui::FontId::proportional(8.0),
                Color32::from_rgb(80, 70, 50),
            );
        }

        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equip_position_labels() {
        let positions = [
            EquipPosition::Head, EquipPosition::Body, EquipPosition::Belt,
            EquipPosition::Boots, EquipPosition::Gloves, EquipPosition::Amulet,
            EquipPosition::RingLeft, EquipPosition::RingRight,
            EquipPosition::MainHand, EquipPosition::OffHand,
        ];
        for pos in positions {
            assert!(!pos.label().is_empty());
        }
    }

    #[test]
    fn test_equip_slot_sizes_positive() {
        let positions = [
            EquipPosition::Head, EquipPosition::Body, EquipPosition::Amulet,
            EquipPosition::MainHand,
        ];
        for pos in positions {
            let s = pos.slot_size();
            assert!(s.x > 0.0);
            assert!(s.y > 0.0);
        }
    }

    #[test]
    fn test_equip_slot_builder() {
        let data = EquipItemData {
            name: "Shako".to_string(),
            quality: ItemQuality::Unique,
        };
        let slot = EquipSlot::new(EquipPosition::Head).item(&data);
        assert_eq!(slot.position, EquipPosition::Head);
        assert!(slot.item.is_some());
    }
}
