//! Multiplayer lobby browser screen (stub).
//!
//! Will display a scrollable list of open games with player count, difficulty,
//! and a "Join" / "Create" action bar. Currently a minimal placeholder.

use egui::Context;

use crate::theme::D2Colors;

/// Draw the lobby browser screen (stub).
///
/// Returns `true` if the player chose to join or create a game.
pub fn draw_lobby_browser(ctx: &Context) -> bool {
    let mut joined = false;

    egui::CentralPanel::default()
        .frame(egui::Frame::none().fill(D2Colors::BG_DARK))
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(60.0);
                ui.label(
                    egui::RichText::new("Parties multijoueur")
                        .color(D2Colors::GOLD_BRIGHT)
                        .size(24.0)
                        .strong(),
                );
                ui.add_space(30.0);
                ui.label(
                    egui::RichText::new("(A implementer -- liste des parties)")
                        .color(D2Colors::TEXT_NORMAL)
                        .size(12.0),
                );
                ui.add_space(20.0);

                let btn = egui::Button::new(
                    egui::RichText::new("Creer une partie")
                        .color(D2Colors::GOLD)
                        .size(14.0),
                )
                .min_size(egui::vec2(180.0, 32.0))
                .fill(D2Colors::PANEL_BG)
                .stroke(egui::Stroke::new(1.0, D2Colors::PANEL_BORDER));

                if ui.add(btn).clicked() {
                    joined = true;
                }
            });
        });

    joined
}
