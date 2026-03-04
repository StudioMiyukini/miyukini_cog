// @id: MUIE-TmplDialog @do: dialog-template @role: template @layer: 6 @human: miyuk

//! Dialog layout template -- NPC interaction with darkened game backdrop.

use egui::Context;
use miyuki_ui_tokens::palette::d2::D2_PALETTE;
use crate::organisms::npc_dialog::{NpcDialog, NpcDialogData};
use crate::convert::rgba_to_color32;

/// The dialog layout template.
pub struct DialogLayout;

impl DialogLayout {
    /// Draw the dialog overlay on top of the game.
    ///
    /// A semi-transparent dark backdrop dims the game area while
    /// the NPC dialog window is centered near the bottom.
    ///
    /// Returns the index of the clicked dialog option, or `None`.
    pub fn show(ctx: &Context, data: &NpcDialogData) -> Option<usize> {
        // Semi-transparent backdrop to dim the game
        egui::Area::new("dialog_backdrop".into())
            .fixed_pos(egui::Pos2::ZERO)
            .order(egui::Order::Background)
            .show(ctx, |ui| {
                let screen = ui.ctx().screen_rect();
                let bg = rgba_to_color32(&D2_PALETTE.bg_base);
                let dimmed = egui::Color32::from_rgba_unmultiplied(bg.r(), bg.g(), bg.b(), 128);
                ui.painter().rect_filled(screen, 0.0, dimmed);
            });

        // NPC dialog
        NpcDialog::show(ctx, data)
    }
}
