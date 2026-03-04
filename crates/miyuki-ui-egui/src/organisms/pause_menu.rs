// @id: MUIE-PauseMenu @do: pause-menu-organism @role: organism @layer: 6 @human: miyuk

//! Pause menu organism -- overlay with resume/options/save/quit.

use egui::Context;
use miyuki_ui_tokens::palette::d2::D2_PALETTE;
use crate::atoms::d2_button::{D2Button, D2ButtonVariant};
use crate::convert::rgba_to_color32;

/// Action from the pause menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PauseAction {
    /// No action.
    None,
    /// Resume game.
    Resume,
    /// Open options.
    Options,
    /// Save and exit.
    SaveAndExit,
    /// Quit to main menu.
    QuitToMenu,
}

/// The pause menu organism.
pub struct PauseMenu;

impl PauseMenu {
    /// Draw the pause menu centered on screen.
    pub fn show(ctx: &Context) -> PauseAction {
        let mut action = PauseAction::None;

        // Semi-transparent overlay
        egui::Area::new("pause_overlay".into())
            .fixed_pos(egui::Pos2::ZERO)
            .order(egui::Order::Background)
            .show(ctx, |ui| {
                let screen = ui.ctx().screen_rect();
                ui.painter().rect_filled(
                    screen,
                    0.0,
                    egui::Color32::from_rgba_premultiplied(0, 0, 0, 150),
                );
            });

        egui::Window::new("Pause")
            .resizable(false)
            .collapsible(false)
            .title_bar(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    let gold = rgba_to_color32(&D2_PALETTE.text_high);
                    ui.label(egui::RichText::new("PAUSE").color(gold).size(20.0));
                    ui.add_space(16.0);

                    if D2Button::new("Reprendre")
                        .variant(D2ButtonVariant::Menu)
                        .show(ui)
                        .clicked()
                    {
                        action = PauseAction::Resume;
                    }
                    ui.add_space(8.0);

                    if D2Button::new("Options")
                        .variant(D2ButtonVariant::Menu)
                        .show(ui)
                        .clicked()
                    {
                        action = PauseAction::Options;
                    }
                    ui.add_space(8.0);

                    if D2Button::new("Sauvegarder et quitter")
                        .variant(D2ButtonVariant::Menu)
                        .show(ui)
                        .clicked()
                    {
                        action = PauseAction::SaveAndExit;
                    }
                    ui.add_space(8.0);

                    if D2Button::new("Menu principal")
                        .variant(D2ButtonVariant::Secondary)
                        .show(ui)
                        .clicked()
                    {
                        action = PauseAction::QuitToMenu;
                    }
                });
            });

        action
    }
}
