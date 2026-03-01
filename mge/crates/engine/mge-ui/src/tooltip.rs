//! Item tooltip system (D2-style floating panel with quality coloring).

use egui::{Context, Pos2};

use crate::theme::{item_quality_color, D2Colors};

// ---------------------------------------------------------------------------
// Data
// ---------------------------------------------------------------------------

/// Data needed to render an item tooltip.
pub struct ItemTooltipData {
    /// Item display name.
    pub name: String,
    /// Quality (`"normal"`, `"magic"`, `"rare"`, `"unique"`, `"set"`, `"rune_word"`).
    pub quality: String,
    /// Base item type (e.g. "Epee large").
    pub base_type: String,
    /// Internal item level.
    pub item_level: u32,
    /// Level required to equip.
    pub required_level: u32,
    /// Strength required to equip.
    pub required_strength: Option<u32>,
    /// Dexterity required to equip.
    pub required_dexterity: Option<u32>,
    /// Magical properties (e.g. "+20% Vitesse d'attaque").
    pub properties: Vec<String>,
    /// Affixes with label and numeric value.
    pub affixes: Vec<(String, f32)>,
    /// Durability (current, maximum).
    pub durability: Option<(u32, u32)>,
}

// ---------------------------------------------------------------------------
// Public draw entry-point
// ---------------------------------------------------------------------------

/// Draw an item tooltip anchored near the given screen position.
pub fn draw_tooltip(ctx: &Context, pos: Pos2, data: &ItemTooltipData) {
    egui::Area::new("item_tooltip".into())
        .fixed_pos(pos + egui::vec2(16.0, 0.0))
        .order(egui::Order::Tooltip)
        .show(ctx, |ui| {
            egui::Frame::popup(ui.style()).show(ui, |ui| {
                ui.set_max_width(220.0);

                // Item name (quality-colored)
                ui.label(
                    egui::RichText::new(&data.name)
                        .color(item_quality_color(&data.quality))
                        .size(13.0)
                        .strong(),
                );

                // Base type
                ui.label(
                    egui::RichText::new(&data.base_type)
                        .color(D2Colors::TEXT_NORMAL)
                        .size(11.0),
                );

                ui.separator();

                // Magical properties
                for prop in &data.properties {
                    ui.label(
                        egui::RichText::new(prop)
                            .color(D2Colors::TEXT_MAGIC)
                            .size(11.0),
                    );
                }

                // Affixes
                for (label, _val) in &data.affixes {
                    ui.label(
                        egui::RichText::new(label)
                            .color(D2Colors::TEXT_MAGIC)
                            .size(10.0),
                    );
                }

                ui.separator();

                // Durability
                if let Some((cur, max)) = data.durability {
                    let dur_text = format!("Durabilite : {cur}/{max}");
                    ui.label(
                        egui::RichText::new(&dur_text)
                            .color(D2Colors::TEXT_NORMAL)
                            .size(10.0),
                    );
                }

                // Required level
                if data.required_level > 0 {
                    let req_text = format!("Niveau requis : {}", data.required_level);
                    ui.label(
                        egui::RichText::new(&req_text)
                            .color(D2Colors::TEXT_NORMAL)
                            .size(10.0),
                    );
                }

                // Required strength
                if let Some(req_str) = data.required_strength {
                    let str_text = format!("Force requise : {req_str}");
                    ui.label(
                        egui::RichText::new(&str_text)
                            .color(D2Colors::TEXT_NORMAL)
                            .size(10.0),
                    );
                }
            });
        });
}
