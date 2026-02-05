//! Organism HeaderWithEdition — header avec sélecteur d'édition (Select).
//!
//! Dépend de [51] Header et de l'atom Select. Utilisé dans les espaces ORG/EXP (édition courante).

use crate::theme::JayFestivalTheme;
use crate::ui::{button, label, select, ButtonVariant, LabelLevel};
use eframe::egui::{Response, Ui};

/// @id: organism_header_with_edition
/// @do: define_header_with_edition_selector
/// @layer: ui
/// @human: En-tête avec titre, sélecteur d'édition et boutons de navigation.

/// @id: header_with_edition_render
/// @do: render_header_with_edition_select_and_nav
/// @layer: ui
/// Dessine le header avec titre, liste déroulante d'éditions (options, selected) et boutons nav.
/// Retourne (réponses des boutons nav, réponse du select).
pub fn header_with_edition_render(
    ui: &mut Ui,
    theme: &JayFestivalTheme,
    title: &str,
    edition_options: &[String],
    selected_edition: &mut usize,
    nav_labels: &[&str],
) -> (Vec<Response>, Response) {
    let mut nav_responses = Vec::new();
    let mut select_response = ui.allocate_response(eframe::egui::vec2(0.0, 0.0), eframe::egui::Sense::hover());

    ui.horizontal(|ui| {
        label(ui, theme, title, LabelLevel::Heading);
        ui.add_space(theme.item_spacing());

        if !edition_options.is_empty() {
            select_response = select(
                ui,
                theme,
                selected_edition,
                edition_options,
                Some("Édition"),
                "header_edition_select",
            );
        }

        ui.with_layout(
            eframe::egui::Layout::right_to_left(eframe::egui::Align::Center),
            |ui| {
                for &label_text in nav_labels.iter().rev() {
                    let r = button(ui, theme, label_text, ButtonVariant::Ghost, Default::default());
                    nav_responses.push(r);
                }
            },
        );
    });

    nav_responses.reverse();
    (nav_responses, select_response)
}
