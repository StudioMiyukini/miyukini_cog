//! Main menu screen (D2-style title screen with action buttons).

use egui::Context;

use crate::theme::D2Colors;

/// Actions that can be triggered from the main menu.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MainMenuAction {
    /// Start a single-player game.
    SinglePlayer,
    /// Open the multiplayer lobby browser.
    Multiplayer,
    /// Open the options screen.
    Options,
    /// Show credits.
    Credits,
    /// Quit the application.
    Exit,
}

/// Persistent state for the main menu screen.
#[derive(Debug, Clone)]
pub struct MainMenuState {
    /// Ordered list of available buttons.
    pub buttons: Vec<MainMenuAction>,
}

impl MainMenuState {
    /// Create the default main-menu state with all five buttons.
    pub fn new() -> Self {
        Self {
            buttons: vec![
                MainMenuAction::SinglePlayer,
                MainMenuAction::Multiplayer,
                MainMenuAction::Options,
                MainMenuAction::Credits,
                MainMenuAction::Exit,
            ],
        }
    }
}

impl Default for MainMenuState {
    fn default() -> Self {
        Self::new()
    }
}

/// A labeled button definition for the main menu.
type MenuButton = (&'static str, fn() -> MainMenuAction);

/// Draw the main menu and return the action selected (if any).
pub fn draw_main_menu(ctx: &Context) -> Option<MainMenuAction> {
    let mut action = None;

    egui::CentralPanel::default()
        .frame(egui::Frame::none().fill(D2Colors::BG_DARK))
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(80.0);

                ui.label(
                    egui::RichText::new("SODOMIGHT")
                        .color(D2Colors::GOLD_BRIGHT)
                        .size(48.0)
                        .strong(),
                );
                ui.label(
                    egui::RichText::new("Dark is the Night")
                        .color(D2Colors::TEXT_NORMAL)
                        .size(16.0),
                );

                ui.add_space(60.0);

                let btn_size = egui::vec2(200.0, 36.0);
                let buttons: &[MenuButton] = &[
                    ("Nouvelle partie", || MainMenuAction::SinglePlayer),
                    ("Multijoueur", || MainMenuAction::Multiplayer),
                    ("Options", || MainMenuAction::Options),
                    ("Credits", || MainMenuAction::Credits),
                    ("Quitter", || MainMenuAction::Exit),
                ];

                for &(label, make_action) in buttons {
                    let btn = egui::Button::new(
                        egui::RichText::new(label)
                            .color(D2Colors::GOLD)
                            .size(14.0),
                    )
                    .min_size(btn_size)
                    .fill(D2Colors::PANEL_BG)
                    .stroke(egui::Stroke::new(1.0, D2Colors::PANEL_BORDER));

                    if ui.add(btn).clicked() {
                        action = Some(make_action());
                    }
                    ui.add_space(8.0);
                }
            });
        });

    action
}
