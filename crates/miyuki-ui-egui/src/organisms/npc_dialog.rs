// @id: MUIE-NpcDialog @do: npc-dialog-organism @role: organism @layer: 6 @human: miyuk

//! NPC dialog organism -- portrait + text + interactive options.

use crate::convert::rgba_to_color32;
use crate::molecules::npc_option::NpcOption;
use egui::Context;
use miyuki_ui_tokens::palette::d2::D2_PALETTE;

/// NPC dialog data.
pub struct NpcDialogData {
    /// NPC name.
    pub npc_name: String,
    /// Dialog text.
    pub text: String,
    /// Available options.
    pub options: Vec<NpcDialogOption>,
}

/// A single dialog option.
#[derive(Debug, Clone)]
pub struct NpcDialogOption {
    /// Option text.
    pub text: String,
    /// Whether this option is enabled.
    pub enabled: bool,
}

/// The NPC dialog organism.
pub struct NpcDialog;

impl NpcDialog {
    /// Draw the NPC dialog.
    ///
    /// Returns the index of the clicked option, or `None`.
    pub fn show(ctx: &Context, data: &NpcDialogData) -> Option<usize> {
        let mut clicked_option = None;

        egui::Window::new("Dialogue")
            .resizable(false)
            .collapsible(false)
            .title_bar(false)
            .anchor(egui::Align2::CENTER_BOTTOM, [0.0, -90.0])
            .show(ctx, |ui| {
                ui.set_min_width(350.0);

                // NPC name
                let gold = rgba_to_color32(&D2_PALETTE.text_high);
                ui.label(egui::RichText::new(&data.npc_name).color(gold).size(14.0));
                ui.separator();

                // Dialog text
                let text_color = rgba_to_color32(&D2_PALETTE.text_primary);
                ui.label(egui::RichText::new(&data.text).color(text_color).size(12.0));

                ui.separator();

                // Options
                for (i, option) in data.options.iter().enumerate() {
                    if NpcOption::new(&option.text)
                        .enabled(option.enabled)
                        .show(ui)
                        .clicked()
                    {
                        clicked_option = Some(i);
                    }
                }
            });

        clicked_option
    }
}
