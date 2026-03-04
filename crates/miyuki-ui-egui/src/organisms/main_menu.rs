// @id: MUIE-MainMenu @do: main-menu-organism @role: organism @layer: 6 @human: miyuk

//! Main menu organism -- title + menu buttons.

use egui::Context;
use miyuki_ui_tokens::palette::d2::D2_PALETTE;
use crate::atoms::d2_button::{D2Button, D2ButtonVariant};
use crate::convert::rgba_to_color32;

/// Action returned from the main menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MainMenuAction {
    /// No action.
    None,
    /// Single player selected.
    SinglePlayer,
    /// Multiplayer selected.
    Multiplayer,
    /// Options selected.
    Options,
    /// Quit selected.
    Quit,
}

/// The main menu organism.
pub struct MainMenu;

impl MainMenu {
    /// Draw the main menu centered on screen.
    pub fn show(ctx: &Context, screen_w: f32, screen_h: f32) -> MainMenuAction {
        let mut action = MainMenuAction::None;

        egui::Area::new("main_menu".into())
            .fixed_pos(egui::Pos2::new(screen_w / 2.0 - 120.0, screen_h / 3.0))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    // Title
                    let gold = rgba_to_color32(&D2_PALETTE.text_high);
                    ui.label(
                        egui::RichText::new("SODOMIGHT")
                            .color(gold)
                            .size(32.0),
                    );
                    ui.add_space(20.0);

                    // Menu buttons
                    if D2Button::new("Solo")
                        .variant(D2ButtonVariant::Menu)
                        .show(ui)
                        .clicked()
                    {
                        action = MainMenuAction::SinglePlayer;
                    }
                    ui.add_space(8.0);

                    if D2Button::new("Multijoueur")
                        .variant(D2ButtonVariant::Menu)
                        .show(ui)
                        .clicked()
                    {
                        action = MainMenuAction::Multiplayer;
                    }
                    ui.add_space(8.0);

                    if D2Button::new("Options")
                        .variant(D2ButtonVariant::Menu)
                        .show(ui)
                        .clicked()
                    {
                        action = MainMenuAction::Options;
                    }
                    ui.add_space(8.0);

                    if D2Button::new("Quitter")
                        .variant(D2ButtonVariant::Menu)
                        .show(ui)
                        .clicked()
                    {
                        action = MainMenuAction::Quit;
                    }
                });
            });

        action
    }
}
