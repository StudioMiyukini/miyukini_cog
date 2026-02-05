//! Atom Select — liste déroulante (ComboBox) avec options.
//!
//! Conforme Specification UI § 2.1 A7. Tokens : même que Input + section.title pour libellé.

use crate::theme::JayFestivalTheme;
use eframe::egui::{Response, Ui};

/// @id: atom_select
/// @do: define_select_component_with_theme
/// @layer: ui
/// @human: Composant liste déroulante ; style via thème (bordure, padding, libellé).

/// @id: select_render
/// @do: render_combo_box_with_options_and_theme
/// @layer: ui
/// Dessine une liste déroulante ; selected est l'index dans options ; label optionnel au-dessus.
pub fn select(
    ui: &mut Ui,
    theme: &JayFestivalTheme,
    selected: &mut usize,
    options: &[String],
    label: Option<&str>,
    id: impl std::hash::Hash,
) -> Response {
    let mut response = if let Some(lbl) = label {
        let rich = eframe::egui::RichText::new(lbl)
            .color(theme.section_title())
            .size(theme.font_size_md());
        ui.label(rich)
    } else {
        ui.allocate_response(egui::vec2(0.0, 0.0), egui::Sense::hover())
    };
    if options.is_empty() {
        return response;
    }
    let current = (*selected).min(options.len().saturating_sub(1));
    let selected_text = options.get(current).cloned().unwrap_or_default();

    let combo_response = egui::ComboBox::from_id_salt(id)
        .selected_text(selected_text)
        .show_ui(ui, |ui| {
            for (i, opt) in options.iter().enumerate() {
                ui.selectable_value(selected, i, opt.as_str());
            }
        });

    let changed = combo_response.response.changed();
    response = response.union(combo_response.response);
    if changed {
        *selected = (*selected).min(options.len().saturating_sub(1));
    }
    response
}
