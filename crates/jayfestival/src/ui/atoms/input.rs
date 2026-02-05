//! Atom Input — champ texte une ligne avec placeholder.
//!
//! Conforme Specification UI § 2.1 A3. Tokens : section.border, borders.radius.small, spacing.input_padding, fonts.sizes.md.

use crate::theme::JayFestivalTheme;
use eframe::egui::{Response, Stroke, Ui};

/// @id: atom_input
/// @do: define_input_component_with_theme_tokens
/// @layer: ui
/// @human: Composant champ texte une ligne ; style via thème (bordure, rayon, padding, police).

/// @id: input_render
/// @do: render_singleline_text_edit_with_theme
/// @layer: ui
/// Dessine un champ texte une ligne ; placeholder si value vide. password masque le contenu.
pub fn input(
    ui: &mut Ui,
    theme: &JayFestivalTheme,
    value: &mut String,
    placeholder: Option<&str>,
    password: bool,
    enabled: bool,
) -> Response {
    let padding = theme.input_padding();
    let radius = theme.radius_small();
    let stroke = Stroke::new(1.0, theme.section_border());
    let margin = egui::Margin::same(padding as i8);

    let mut output = egui::TextEdit::singleline(value)
        .password(password)
        .font(egui::TextStyle::Body)
        .desired_width(f32::INFINITY);

    if let Some(ph) = placeholder {
        output = output.hint_text(ph);
    }

    let frame = egui::Frame::new()
        .fill(theme.background_primary())
        .stroke(stroke)
        .corner_radius(egui::CornerRadius::same(radius as u8))
        .inner_margin(margin);

    frame.show(ui, |ui| ui.add_enabled(enabled, output)).response
}
