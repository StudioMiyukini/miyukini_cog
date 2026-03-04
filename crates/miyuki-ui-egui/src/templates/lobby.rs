// @id: MUIE-TmplLobby @do: lobby-template @role: template @layer: 6 @human: miyuk

//! Lobby layout template -- game browser + chat.

use egui::Context;
use miyuki_ui_tokens::palette::d2::D2_PALETTE;
use crate::atoms::d2_button::{D2Button, D2ButtonVariant};
use crate::convert::rgba_to_color32;

/// Game listing in the lobby.
#[derive(Debug, Clone)]
pub struct LobbyGameEntry {
    /// Game name.
    pub name: String,
    /// Player count.
    pub players: u32,
    /// Maximum players.
    pub max_players: u32,
    /// Difficulty.
    pub difficulty: String,
}

/// Action from the lobby.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LobbyAction {
    /// No action.
    None,
    /// Join a game by index.
    Join(usize),
    /// Create a new game.
    Create,
    /// Go back to main menu.
    Back,
}

/// The lobby layout template.
pub struct LobbyLayout;

impl LobbyLayout {
    /// Draw the lobby screen.
    pub fn show(ctx: &Context, games: &[LobbyGameEntry]) -> LobbyAction {
        let mut action = LobbyAction::None;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                let gold = rgba_to_color32(&D2_PALETTE.text_high);
                ui.label(egui::RichText::new("Lobby").color(gold).size(18.0));
                ui.separator();

                // Game list
                for (i, game) in games.iter().enumerate() {
                    let text = format!(
                        "{} ({}/{}) - {}",
                        game.name, game.players, game.max_players, game.difficulty
                    );
                    let color = rgba_to_color32(&D2_PALETTE.text_primary);
                    let resp = ui.add(
                        egui::Label::new(egui::RichText::new(&text).color(color).size(12.0))
                            .sense(egui::Sense::click()),
                    );
                    if resp.double_clicked() {
                        action = LobbyAction::Join(i);
                    }
                }

                if games.is_empty() {
                    let muted = rgba_to_color32(&D2_PALETTE.text_muted);
                    ui.label(egui::RichText::new("Aucune partie disponible").color(muted).size(12.0));
                }

                ui.add_space(16.0);

                ui.horizontal(|ui| {
                    if D2Button::new("Creer une partie").show(ui).clicked() {
                        action = LobbyAction::Create;
                    }
                    if D2Button::new("Retour")
                        .variant(D2ButtonVariant::Secondary)
                        .show(ui)
                        .clicked()
                    {
                        action = LobbyAction::Back;
                    }
                });
            });
        });

        action
    }
}
