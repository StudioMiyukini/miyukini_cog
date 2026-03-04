// @id: MUIE-ItemTooltip @do: item-tooltip-molecule @role: molecule @layer: 6 @human: miyuk

//! D2-style item tooltip -- the most complex molecule.
//!
//! Displays a complete item description with quality-colored name, base type,
//! magical properties (blue), requirements (red if unmet), durability, and
//! sell value. The tooltip has a dark background with a thin border.

use egui::{Color32, Pos2, Ui};
use miyuki_ui_tokens::palette::d2::{D2_PALETTE, D2QualityColors};
use crate::atoms::quality_text::ItemQuality;
use crate::convert::rgba_to_color32;

/// Complete data for rendering an item tooltip.
#[derive(Debug, Clone)]
pub struct ItemTooltipData {
    /// Item display name.
    pub name: String,
    /// Item quality.
    pub quality: ItemQuality,
    /// Base item type (e.g., "Hydra Bow").
    pub base_type: String,
    /// Item level.
    pub item_level: u32,
    /// Required character level.
    pub required_level: u32,
    /// Required strength (None if no requirement).
    pub required_strength: Option<u32>,
    /// Required dexterity (None if no requirement).
    pub required_dexterity: Option<u32>,
    /// Whether the character meets all requirements.
    pub requirements_met: bool,
    /// Magical properties (blue text lines).
    pub properties: Vec<String>,
    /// Durability (current, max). None for indestructible or non-applicable.
    pub durability: Option<(u32, u32)>,
    /// Sell value in gold.
    pub sell_value: Option<u32>,
}

/// An item tooltip molecule.
pub struct ItemTooltip<'a> {
    /// Tooltip data.
    pub data: &'a ItemTooltipData,
    /// Maximum width.
    pub max_width: f32,
}

impl<'a> ItemTooltip<'a> {
    /// Create a new item tooltip.
    pub fn new(data: &'a ItemTooltipData) -> Self {
        Self {
            data,
            max_width: 220.0,
        }
    }

    /// Set max width.
    pub fn max_width(mut self, width: f32) -> Self {
        self.max_width = width;
        self
    }

    /// Draw the tooltip as a popup/area at the given position.
    pub fn show_at(self, ctx: &egui::Context, pos: Pos2) {
        egui::Area::new("item_tooltip_area".into())
            .fixed_pos(pos + egui::vec2(16.0, 0.0))
            .order(egui::Order::Tooltip)
            .show(ctx, |ui| {
                egui::Frame::popup(ui.style())
                    .fill(Color32::from_rgba_premultiplied(10, 8, 5, 245))
                    .stroke(egui::Stroke::new(1.0, rgba_to_color32(&D2_PALETTE.border_default)))
                    .show(ui, |ui| {
                        ui.set_max_width(self.max_width);
                        self.draw_contents(ui);
                    });
            });
    }

    /// Draw the tooltip contents inside a `Ui`.
    ///
    /// Useful when embedding the tooltip inside an existing panel.
    pub fn show_inline(self, ui: &mut Ui) {
        ui.set_max_width(self.max_width);
        self.draw_contents(ui);
    }

    fn draw_contents(self, ui: &mut Ui) {
        let data = self.data;

        // Item name (quality-colored)
        let name_color = data.quality.color();
        ui.label(
            egui::RichText::new(&data.name)
                .color(name_color)
                .size(13.0)
                .strong(),
        );

        // Base type
        ui.label(
            egui::RichText::new(&data.base_type)
                .color(rgba_to_color32(&D2_PALETTE.text_primary))
                .size(11.0),
        );

        ui.separator();

        // Magical properties (blue)
        let magic_color = rgba_to_color32(&D2QualityColors::MAGIC);
        for prop in &data.properties {
            ui.label(
                egui::RichText::new(prop)
                    .color(magic_color)
                    .size(11.0),
            );
        }

        if !data.properties.is_empty() {
            ui.separator();
        }

        // Requirements
        let req_color_met = rgba_to_color32(&D2_PALETTE.text_primary);
        let req_color_unmet = rgba_to_color32(&D2_PALETTE.error);
        let req_color = if data.requirements_met { req_color_met } else { req_color_unmet };

        if data.required_level > 0 {
            let text = format!("Niveau requis : {}", data.required_level);
            ui.label(
                egui::RichText::new(&text)
                    .color(req_color)
                    .size(10.0),
            );
        }

        if let Some(str_req) = data.required_strength {
            let text = format!("Force requise : {str_req}");
            ui.label(
                egui::RichText::new(&text)
                    .color(req_color)
                    .size(10.0),
            );
        }

        if let Some(dex_req) = data.required_dexterity {
            let text = format!("Dexterite requise : {dex_req}");
            ui.label(
                egui::RichText::new(&text)
                    .color(req_color)
                    .size(10.0),
            );
        }

        // Durability
        if let Some((cur, max)) = data.durability {
            let text = format!("Durabilite : {cur}/{max}");
            ui.label(
                egui::RichText::new(&text)
                    .color(rgba_to_color32(&D2_PALETTE.text_primary))
                    .size(10.0),
            );
        }

        // Sell value
        if let Some(value) = data.sell_value {
            let text = format!("Prix de vente : {value}");
            ui.label(
                egui::RichText::new(&text)
                    .color(rgba_to_color32(&D2_PALETTE.text_secondary))
                    .size(10.0),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_tooltip_data() -> ItemTooltipData {
        ItemTooltipData {
            name: "Windforce".to_string(),
            quality: ItemQuality::Unique,
            base_type: "Hydra Bow".to_string(),
            item_level: 73,
            required_level: 73,
            required_strength: Some(134),
            required_dexterity: Some(167),
            requirements_met: true,
            properties: vec![
                "+250% Enhanced Damage".to_string(),
                "20% Increased Attack Speed".to_string(),
                "Knockback".to_string(),
            ],
            durability: Some((45, 50)),
            sell_value: Some(35000),
        }
    }

    #[test]
    fn test_tooltip_data_construction() {
        let data = make_tooltip_data();
        assert_eq!(data.name, "Windforce");
        assert_eq!(data.quality, ItemQuality::Unique);
        assert_eq!(data.properties.len(), 3);
    }

    #[test]
    fn test_tooltip_builder() {
        let data = make_tooltip_data();
        let tooltip = ItemTooltip::new(&data).max_width(300.0);
        assert!((tooltip.max_width - 300.0).abs() < f32::EPSILON);
    }
}
