//! CTACard — molecule titre, description optionnelle, bouton d'action (Specification UI § 2.2 M4).

use crate::theme::JayFestivalTheme;
use eframe::egui::{self, RichText};

/// @id: molecule_cta_card
/// @do: define_cta_card_props
/// @layer: ui
/// Carte CTA : titre, description optionnelle, bouton d'action (Primary).

/// @id: cta_card_render
/// @do: render_cta_card_with_button
/// @layer: ui
/// Affiche une carte avec titre, description optionnelle et bouton Primary ; clic = on_click.
/// Tokens : FeatureCard + bouton Primary (accent).
pub fn cta_card_render(
    theme: &JayFestivalTheme,
    ui: &mut egui::Ui,
    title: &str,
    description: Option<&str>,
    button_label: &str,
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
            RichText::new(button_label)
                .size(theme.font_size_md())
                .color(theme.text_primary()),
        )
        .fill(theme.accent_primary())
        .min_size(egui::vec2(0.0, 40.0));
        ui.add(btn)
    });

    // Retourne la réponse du bouton pour que le caller puisse vérifier .clicked().
    inner.inner
}
