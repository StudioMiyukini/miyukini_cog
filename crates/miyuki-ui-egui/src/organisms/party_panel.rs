// @id: MUIE-PartyPanel @do: party-panel-organism @role: organism @layer: 6 @human: miyuk

//! Party panel organism -- list of party members with actions.

use crate::convert::rgba_to_color32;
use crate::molecules::party_member::{PartyMember, PartyMemberData};
use egui::Context;
use miyuki_ui_tokens::palette::d2::D2_PALETTE;

/// The party panel organism.
pub struct PartyPanel;

impl PartyPanel {
    /// Draw the party panel as a window.
    pub fn show(ctx: &Context, is_open: &mut bool, members: &[PartyMemberData]) {
        if !*is_open {
            return;
        }

        egui::Window::new("Groupe")
            .resizable(false)
            .collapsible(false)
            .open(is_open)
            .default_pos([200.0, 50.0])
            .min_width(250.0)
            .show(ctx, |ui| {
                let gold = rgba_to_color32(&D2_PALETTE.accent_primary);
                let count = members.len();
                let header = format!("Groupe ({count}/8)");
                ui.label(egui::RichText::new(&header).color(gold).size(12.0));
                ui.separator();

                for member in members {
                    PartyMember::new(member).show(ui);
                    ui.separator();
                }

                if members.is_empty() {
                    let muted = rgba_to_color32(&D2_PALETTE.text_muted);
                    ui.label(
                        egui::RichText::new("Aucun joueur dans le groupe")
                            .color(muted)
                            .size(11.0),
                    );
                }
            });
    }
}
