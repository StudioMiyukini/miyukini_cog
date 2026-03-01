//! In-game pause menu (D2-style centered overlay).

use egui::Context;

use crate::theme::D2Colors;

/// Actions that can be triggered from the pause menu.
pub enum PauseAction {
    /// Resume gameplay.
    Resume,
    /// Open the options screen.
    Options,
    /// Save the game and return to the main menu.
    SaveAndQuit,
}

/// A labeled button definition for the pause menu.
type PauseButton = (&'static str, fn() -> PauseAction);

/// Draw the pause menu and return the action selected (if any).
pub fn draw_pause_menu(ctx: &Context) -> Option<PauseAction> {
    let mut action = None;

    egui::Window::new("Pause")
        .resizable(false)
        .collapsible(false)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            ui.set_min_width(180.0);
            ui.vertical_centered(|ui| {
                let buttons: &[PauseButton] = &[
                    ("Reprendre", || PauseAction::Resume),
                    ("Options", || PauseAction::Options),
                    ("Sauvegarder & Quitter", || PauseAction::SaveAndQuit),
                ];

                for &(label, make_action) in buttons {
                    let btn = egui::Button::new(
                        egui::RichText::new(label)
                            .color(D2Colors::GOLD)
                            .size(13.0),
                    )
                    .min_size(egui::vec2(160.0, 30.0))
                    .fill(D2Colors::PANEL_BG)
                    .stroke(egui::Stroke::new(1.0, D2Colors::PANEL_BORDER));

                    if ui.add(btn).clicked() {
                        action = Some(make_action());
                    }
                    ui.add_space(4.0);
                }
            });
        });

    action
}
