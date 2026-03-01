//! Character stats panel (D2-style): base stats, derived stats, resistances.

use egui::Context;

use crate::theme::D2Colors;

// ---------------------------------------------------------------------------
// Data
// ---------------------------------------------------------------------------

/// Character data displayed in the stats panel.
pub struct CharacterData {
    /// Character name.
    pub name: String,
    /// Class name.
    pub class: String,
    /// Current level.
    pub level: i32,
    /// Strength.
    pub strength: i32,
    /// Dexterity.
    pub dexterity: i32,
    /// Vitality.
    pub vitality: i32,
    /// Energy.
    pub energy: i32,
    /// Stat points available to distribute.
    pub unspent_points: i32,
    /// Base life (before modifiers).
    pub life_base: i32,
    /// Base mana (before modifiers).
    pub mana_base: i32,
    /// Total defense.
    pub defense: i32,
    /// Minimum weapon damage.
    pub damage_min: i32,
    /// Maximum weapon damage.
    pub damage_max: i32,
    /// Attack rating.
    pub attack_rating: i32,
    /// Fire resistance (%).
    pub fire_res: i32,
    /// Cold resistance (%).
    pub cold_res: i32,
    /// Lightning resistance (%).
    pub lightning_res: i32,
    /// Poison resistance (%).
    pub poison_res: i32,
}

// ---------------------------------------------------------------------------
// Public draw entry-point
// ---------------------------------------------------------------------------

/// Draw the character stats panel.
pub fn draw_character_panel(ctx: &Context, is_open: &mut bool, data: &CharacterData) {
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
                ui.label(
                    egui::RichText::new(&data.name)
                        .color(D2Colors::GOLD_BRIGHT)
                        .size(14.0),
                );
                let class_level = format!("{} - Nv {}", data.class, data.level);
                ui.label(
                    egui::RichText::new(&class_level)
                        .color(D2Colors::TEXT_NORMAL)
                        .size(11.0),
                );
            });
            ui.separator();

            // Unspent points
            if data.unspent_points > 0 {
                let pts_text = format!("Points a distribuer : {}", data.unspent_points);
                ui.label(
                    egui::RichText::new(&pts_text)
                        .color(D2Colors::SKILL_ACTIVE)
                        .size(11.0),
                );
                ui.separator();
            }

            // Base stats
            ui.label(
                egui::RichText::new("-- Stats de base --")
                    .color(D2Colors::GOLD)
                    .size(11.0),
            );
            let can_add = data.unspent_points > 0;
            egui::Grid::new("base_stats").num_columns(3).show(ui, |ui| {
                stat_row(ui, "Force", data.strength, can_add);
                stat_row(ui, "Dexterite", data.dexterity, can_add);
                stat_row(ui, "Vitalite", data.vitality, can_add);
                stat_row(ui, "Energie", data.energy, can_add);
            });
            ui.separator();

            // Derived stats
            ui.label(
                egui::RichText::new("-- Stats derivees --")
                    .color(D2Colors::GOLD)
                    .size(11.0),
            );
            egui::Grid::new("derived_stats")
                .num_columns(2)
                .show(ui, |ui| {
                    derived_stat(ui, "Vie", data.life_base);
                    derived_stat(ui, "Mana", data.mana_base);
                    derived_stat(ui, "Defense", data.defense);
                    derived_stat(ui, "Attaque", data.attack_rating);
                    let dmg_label = format!("Degats {}-{}", data.damage_min, data.damage_max);
                    derived_stat(ui, &dmg_label, 0);
                });
            ui.separator();

            // Resistances
            ui.label(
                egui::RichText::new("-- Resistances --")
                    .color(D2Colors::GOLD)
                    .size(11.0),
            );
            egui::Grid::new("res_stats").num_columns(2).show(ui, |ui| {
                res_row(ui, "Feu", data.fire_res);
                res_row(ui, "Froid", data.cold_res);
                res_row(ui, "Foudre", data.lightning_res);
                res_row(ui, "Poison", data.poison_res);
            });
        });
}

// ---------------------------------------------------------------------------
// Private helpers
// ---------------------------------------------------------------------------

/// A base stat row with optional "+" button for point distribution.
fn stat_row(ui: &mut egui::Ui, label: &str, value: i32, can_add: bool) {
    ui.label(
        egui::RichText::new(label)
            .color(D2Colors::TEXT_NORMAL)
            .size(11.0),
    );
    let val_text = value.to_string();
    ui.label(
        egui::RichText::new(&val_text)
            .color(D2Colors::GOLD)
            .size(11.0),
    );
    if can_add {
        if ui.small_button("+").clicked() {
            // Signal to game: add 1 point to this stat
        }
    } else {
        ui.label("");
    }
    ui.end_row();
}

/// A derived (read-only) stat row.
fn derived_stat(ui: &mut egui::Ui, label: &str, value: i32) {
    ui.label(
        egui::RichText::new(label)
            .color(D2Colors::TEXT_NORMAL)
            .size(11.0),
    );
    if value > 0 {
        let val_text = value.to_string();
        ui.label(
            egui::RichText::new(&val_text)
                .color(D2Colors::GOLD)
                .size(11.0),
        );
    }
    ui.end_row();
}

/// A resistance row with color-coded value.
fn res_row(ui: &mut egui::Ui, label: &str, value: i32) {
    ui.label(
        egui::RichText::new(label)
            .color(D2Colors::TEXT_NORMAL)
            .size(11.0),
    );
    let color = if value >= 75 {
        D2Colors::GOLD_BRIGHT
    } else if value >= 0 {
        D2Colors::GOLD
    } else {
        D2Colors::RED_LIFE
    };
    let res_text = format!("{value}%");
    ui.label(egui::RichText::new(&res_text).color(color).size(11.0));
    ui.end_row();
}
