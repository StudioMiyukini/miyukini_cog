// @id: MUIE-Trade @do: trade-window-organism @role: organism @layer: 6 @human: miyuk

//! Trade window organism -- two grids (yours/theirs) + gold + accept/cancel.

use egui::Context;
use miyuki_ui_tokens::palette::d2::D2_PALETTE;
use crate::atoms::d2_button::{D2Button, D2ButtonVariant};
use crate::convert::rgba_to_color32;

/// Trade window data.
pub struct TradeWindowData {
    /// Other player's name.
    pub other_name: String,
    /// Your offered gold.
    pub your_gold: i64,
    /// Their offered gold.
    pub their_gold: i64,
    /// Whether the other player has accepted.
    pub other_accepted: bool,
}

/// The trade window organism.
pub struct TradeWindow;

impl TradeWindow {
    /// Draw the trade window.
    pub fn show(ctx: &Context, is_open: &mut bool, data: &TradeWindowData) -> TradeAction {
        let mut action = TradeAction::None;

        if !*is_open {
            return action;
        }

        egui::Window::new("Echange")
            .resizable(false)
            .collapsible(false)
            .open(is_open)
            .default_pos([200.0, 100.0])
            .min_width(400.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    // Your side
                    ui.vertical(|ui| {
                        let gold = rgba_to_color32(&D2_PALETTE.accent_primary);
                        ui.label(egui::RichText::new("Votre offre").color(gold).size(12.0));
                        ui.separator();
                        let your_gold_text = format!("Or : {}", data.your_gold);
                        ui.label(egui::RichText::new(&your_gold_text).color(gold).size(11.0));
                        // Item grid placeholder
                        ui.label(
                            egui::RichText::new("(Grille d'items)")
                                .color(rgba_to_color32(&D2_PALETTE.text_muted))
                                .size(10.0),
                        );
                    });

                    ui.separator();

                    // Their side
                    ui.vertical(|ui| {
                        let gold = rgba_to_color32(&D2_PALETTE.accent_primary);
                        let label = format!("Offre de {}", data.other_name);
                        ui.label(egui::RichText::new(&label).color(gold).size(12.0));
                        ui.separator();
                        let their_gold_text = format!("Or : {}", data.their_gold);
                        ui.label(egui::RichText::new(&their_gold_text).color(gold).size(11.0));

                        if data.other_accepted {
                            let accepted_color = rgba_to_color32(&D2_PALETTE.success);
                            ui.label(egui::RichText::new("Accepte !").color(accepted_color).size(11.0));
                        }
                    });
                });

                ui.separator();

                // Action buttons
                ui.horizontal(|ui| {
                    if D2Button::new("Accepter").show(ui).clicked() {
                        action = TradeAction::Accept;
                    }
                    if D2Button::new("Annuler")
                        .variant(D2ButtonVariant::Secondary)
                        .show(ui)
                        .clicked()
                    {
                        action = TradeAction::Cancel;
                    }
                });
            });

        action
    }
}

/// Action returned from the trade window.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TradeAction {
    /// No action taken.
    None,
    /// Player clicked accept.
    Accept,
    /// Player clicked cancel.
    Cancel,
}
