// @id: MUIE-CharSheet @do: character-sheet-organism @role: organism @layer: 6 @human: miyuk

//! Character sheet organism -- attributes, derived stats, resistances.

use crate::atoms::d2_label::D2Label;
use crate::convert::rgba_to_color32;
use crate::molecules::stat_row::{StatRow, StatVariant};
use egui::Context;
use miyuki_ui_tokens::palette::d2::D2_PALETTE;

/// Character data for the stat panel.
pub struct CharacterSheetData {
    /// Character name.
    pub name: String,
    /// Class name.
    pub class: String,
    /// Current level.
    pub level: i32,
    /// Base stats.
    pub strength: i32,
    pub dexterity: i32,
    pub vitality: i32,
    pub energy: i32,
    /// Unspent stat points.
    pub unspent_points: i32,
    /// Derived stats.
    pub life: i32,
    pub mana: i32,
    pub defense: i32,
    pub attack_rating: i32,
    pub damage_min: i32,
    pub damage_max: i32,
    /// Resistances (%).
    pub fire_res: i32,
    pub cold_res: i32,
    pub lightning_res: i32,
    pub poison_res: i32,
}

/// The character sheet organism.
pub struct CharacterSheet;

impl CharacterSheet {
    /// Draw the character sheet as a window.
    pub fn show(ctx: &Context, is_open: &mut bool, data: &CharacterSheetData) {
        if !*is_open {
            return;
        }

        egui::Window::new("Personnage")
            .resizable(false)
            .collapsible(false)
            .open(is_open)
            .default_pos([20.0, 50.0])
            .min_width(220.0)
            .show(ctx, |ui| {
                // Header
                ui.vertical_centered(|ui| {
                    let gold_bright = rgba_to_color32(&D2_PALETTE.text_high);
                    ui.label(
                        egui::RichText::new(&data.name)
                            .color(gold_bright)
                            .size(14.0),
                    );
                    let class_level = format!("{} - Nv {}", data.class, data.level);
                    let text_color = rgba_to_color32(&D2_PALETTE.text_primary);
                    ui.label(
                        egui::RichText::new(&class_level)
                            .color(text_color)
                            .size(11.0),
                    );
                });
                ui.separator();

                // Unspent points
                if data.unspent_points > 0 {
                    let pts = format!("Points a distribuer : {}", data.unspent_points);
                    let highlight = rgba_to_color32(&D2_PALETTE.text_high);
                    ui.label(egui::RichText::new(&pts).color(highlight).size(11.0));
                    ui.separator();
                }

                // Base stats
                D2Label::new("-- Stats de base --").show(ui);
                let can_add = data.unspent_points > 0;
                StatRow::base("Force", data.strength)
                    .can_add(can_add)
                    .show(ui);
                StatRow::base("Dexterite", data.dexterity)
                    .can_add(can_add)
                    .show(ui);
                StatRow::base("Vitalite", data.vitality)
                    .can_add(can_add)
                    .show(ui);
                StatRow::base("Energie", data.energy)
                    .can_add(can_add)
                    .show(ui);
                ui.separator();

                // Derived stats
                D2Label::new("-- Stats derivees --").show(ui);
                StatRow::derived("Vie", data.life).show(ui);
                StatRow::derived("Mana", data.mana).show(ui);
                StatRow::derived("Defense", data.defense).show(ui);
                StatRow::derived("Attaque", data.attack_rating).show(ui);
                let dmg = format!("Degats {}-{}", data.damage_min, data.damage_max);
                StatRow::derived(&dmg, 0).show(ui);
                ui.separator();

                // Resistances
                D2Label::new("-- Resistances --").show(ui);
                Self::draw_resistance(ui, "Feu", data.fire_res);
                Self::draw_resistance(ui, "Froid", data.cold_res);
                Self::draw_resistance(ui, "Foudre", data.lightning_res);
                Self::draw_resistance(ui, "Poison", data.poison_res);
            });
    }

    fn draw_resistance(ui: &mut egui::Ui, label: &str, value: i32) {
        let variant = if value >= 75 {
            StatVariant::Bonus
        } else if value < 0 {
            StatVariant::Penalty
        } else {
            StatVariant::Derived
        };
        let text = format!("{value}%");
        ui.horizontal(|ui| {
            let color = rgba_to_color32(&D2_PALETTE.text_primary);
            ui.label(egui::RichText::new(label).color(color).size(11.0));

            let val_color = match variant {
                StatVariant::Bonus => rgba_to_color32(&D2_PALETTE.text_high),
                StatVariant::Penalty => rgba_to_color32(&D2_PALETTE.error),
                _ => rgba_to_color32(&D2_PALETTE.accent_primary),
            };
            ui.label(egui::RichText::new(&text).color(val_color).size(11.0));
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character_sheet_data() {
        let data = CharacterSheetData {
            name: "TestChar".to_string(),
            class: "Sorceress".to_string(),
            level: 85,
            strength: 100,
            dexterity: 100,
            vitality: 200,
            energy: 150,
            unspent_points: 5,
            life: 1000,
            mana: 800,
            defense: 500,
            attack_rating: 2000,
            damage_min: 100,
            damage_max: 500,
            fire_res: 75,
            cold_res: 50,
            lightning_res: 75,
            poison_res: -10,
        };
        assert_eq!(data.level, 85);
        assert_eq!(data.fire_res, 75);
    }
}
