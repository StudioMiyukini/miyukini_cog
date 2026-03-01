//! Skill tree panel (stub).
//!
//! Full implementation will render a class-based skill tree with interconnected
//! nodes, prerequisite lines, and point allocation. For now this is a minimal
//! placeholder window.

use egui::Context;

use crate::theme::D2Colors;

/// Draw the skill tree panel (stub).
///
/// Opens as an egui window when `is_open` is `true`. Currently displays a
/// placeholder message. Will be expanded to a full D2-style skill tree with
/// three tab columns per class.
pub fn draw_skill_tree(ctx: &Context, is_open: &mut bool) {
    if !*is_open {
        return;
    }

    egui::Window::new("Arbre de competences")
        .resizable(false)
        .collapsible(false)
        .open(is_open)
        .default_pos([100.0, 50.0])
        .min_width(300.0)
        .show(ctx, |ui| {
            ui.label(
                egui::RichText::new("Arbre de competences")
                    .color(D2Colors::GOLD_BRIGHT)
                    .size(14.0),
            );
            ui.separator();
            ui.label(
                egui::RichText::new("(A implementer -- arbre par classe avec 3 colonnes)")
                    .color(D2Colors::TEXT_NORMAL)
                    .size(11.0),
            );
        });
}
