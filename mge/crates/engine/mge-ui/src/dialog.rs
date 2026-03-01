//! NPC dialog window (D2-style bottom-anchored text box).

use egui::Context;

use crate::theme::D2Colors;

/// Draw an NPC dialog window anchored near the bottom of the screen.
///
/// The window is titled with the NPC's name and displays the dialog `text`.
/// The user can close it via the "Fermer" button.
pub fn draw_npc_dialog(ctx: &Context, npc_name: &str, text: &str, is_open: &mut bool) {
    if !*is_open {
        return;
    }

    // We avoid `.open(is_open)` because the closure also needs mutable access
    // to `is_open` for the "Fermer" button. Instead we manage closing manually.
    let mut should_close = false;

    egui::Window::new(npc_name)
        .resizable(false)
        .collapsible(false)
        .anchor(egui::Align2::CENTER_BOTTOM, [0.0, -90.0])
        .min_width(400.0)
        .max_width(600.0)
        .show(ctx, |ui| {
            ui.label(
                egui::RichText::new(text)
                    .color(D2Colors::TEXT_NORMAL)
                    .size(12.0),
            );
            ui.add_space(8.0);
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui
                    .button(egui::RichText::new("Fermer").color(D2Colors::GOLD))
                    .clicked()
                {
                    should_close = true;
                }
            });
        });

    if should_close {
        *is_open = false;
    }
}
