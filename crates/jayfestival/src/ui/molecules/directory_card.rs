//! DirectoryCard — molecule titre, description optionnelle, CTA (Specification UI § 2.2 M2).

use crate::theme::JayFestivalTheme;
use eframe::egui::{self, RichText};

/// @id: molecule_directory_card
/// @do: define_directory_card_props
/// @layer: ui
/// Carte répertoire : titre, description optionnelle, bouton CTA en bas.

/// @id: directory_card_render
/// @do: render_directory_card_with_cta
/// @layer: ui
/// Affiche une carte avec titre, description optionnelle et bouton CTA ; clic sur le bouton déclenche l'action.
/// Tokens : section.card.*, radius_medium, card_padding, navigation.button.* pour CTA.
pub fn directory_card_render(
    theme: &JayFestivalTheme,
    ui: &mut egui::Ui,
    title: &str,
    description: Option<&str>,
    cta_label: &str,
) -> egui::Response {
    let frame = egui::Frame::default()
        .fill(theme.section_card_background())
        .stroke(egui::Stroke::new(1.0, theme.section_border()))
        .corner_radius(egui::CornerRadius::same(theme.radius_medium() as u8))
        .inner_margin(egui::Margin::same(theme.card_padding() as i8));

    let inner = frame.show(ui, |ui| {
        ui.label(
            RichText::new(title)
                .size(theme.font_size_heading())
                .color(theme.section_title()),
        );
        if let Some(desc) = description {
            ui.add_space(theme.item_spacing());
            ui.label(
                RichText::new(desc)
                    .size(theme.font_size_sm())
                    .color(theme.section_description()),
            );
        }
        ui.add_space(theme.item_spacing());
        let btn = egui::Button::new(
            RichText::new(cta_label)
                .size(theme.font_size_md())
                .color(theme.text_primary()),
        )
        .min_size(egui::vec2(0.0, 40.0)); // PROTO-7 zone cliquable min 40 px
        ui.add(btn)
    });

    // Retourne la réponse du bouton CTA pour que le caller puisse vérifier .clicked().
    inner.inner
}
