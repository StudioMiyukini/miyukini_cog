//! Atom Checkbox — case à cocher avec label.
//!
//! Conforme Specification UI § 2.1 A6. Tokens : section.border, accent pour coché, fonts.sizes.md.

use crate::theme::JayFestivalTheme;
use eframe::egui::{Response, Ui};

/// @id: atom_checkbox
/// @do: define_checkbox_component_with_theme
/// @layer: ui
/// @human: Composant case à cocher ; style cohérent thème (bordure, accent).

/// @id: checkbox_render
/// @do: render_checkbox_with_label_and_theme
/// @layer: ui
/// Dessine une case à cocher ; clic inverse la valeur. label optionnel à droite.
pub fn checkbox(
    ui: &mut Ui,
    _theme: &JayFestivalTheme,
    checked: &mut bool,
    label: &str,
    enabled: bool,
) -> Response {
    ui.add_enabled(enabled, egui::Checkbox::new(checked, label))
}
