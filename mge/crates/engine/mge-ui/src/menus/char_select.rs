//! Character selection screen (stub).
//!
//! Will display a list of existing characters with class, level, and a "Create"
//! button. Currently a minimal placeholder.

use egui::Context;

use crate::theme::D2Colors;

/// Draw the character selection screen (stub).
///
/// Returns `true` if the player selected a character and wants to proceed.
pub fn draw_char_select(ctx: &Context) -> bool {
    let mut selected = false;

    egui::CentralPanel::default()
        .frame(egui::Frame::none().fill(D2Colors::BG_DARK))
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(60.0);
                ui.label(
                    egui::RichText::new("Selection de personnage")
                        .color(D2Colors::GOLD_BRIGHT)
                        .size(24.0)
                        .strong(),
                );
                ui.add_space(30.0);
                ui.label(
                    egui::RichText::new("(A implementer -- liste des personnages)")
                        .color(D2Colors::TEXT_NORMAL)
                        .size(12.0),
                );
                ui.add_space(20.0);

                let btn = egui::Button::new(
                    egui::RichText::new("Entrer en jeu")
                        .color(D2Colors::GOLD)
                        .size(14.0),
                )
                .min_size(egui::vec2(180.0, 32.0))
                .fill(D2Colors::PANEL_BG)
                .stroke(egui::Stroke::new(1.0, D2Colors::PANEL_BORDER));

                if ui.add(btn).clicked() {
                    selected = true;
                }
            });
        });

    selected
}
