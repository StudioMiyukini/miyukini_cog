//! FeatureCard — molecule titre, description, icône (Specification UI § 2.2 M1).

use crate::theme::JayFestivalTheme;
use eframe::egui::{self, RichText};

/// @id: molecule_feature_card
/// @do: define_feature_card_props_and_variant
/// @layer: ui
/// Variante visuelle : default ou accent (bordure/couleur accent).
#[derive(Clone, Copy, Default)]
pub enum FeatureCardVariant {
    /// Fond carte standard, bordure section.
    #[default]
    Default,
    /// Légère teinte accent ou bordure accent.
    Accent,
}

/// @id: feature_card_render
/// @do: render_feature_card_with_title_description_icon
/// @layer: ui
/// Affiche une carte avec titre, description et icône (emoji ou glyphe) ; optionnellement cliquable.
/// Tokens : section.card.background, section.border, radius_medium, card_padding, section.title, section.description.
pub fn feature_card_render(
    theme: &JayFestivalTheme,
    ui: &mut egui::Ui,
    title: &str,
    description: &str,
    icon: &str,
    variant: FeatureCardVariant,
) -> egui::Response {
    let fill = theme.section_card_background();
    let stroke_color = match variant {
        FeatureCardVariant::Default => theme.section_border(),
        FeatureCardVariant::Accent => theme.accent_primary(),
    };
    let frame = egui::Frame::default()
        .fill(fill)
        .stroke(egui::Stroke::new(1.0, stroke_color))
        .corner_radius(egui::CornerRadius::same(theme.radius_medium() as u8))
        .inner_margin(egui::Margin::same(theme.card_padding() as i8));

    let response = frame.show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label(
                RichText::new(icon)
                    .size(theme.font_size_lg())
                    .color(theme.section_title()),
            );
            ui.add_space(theme.item_spacing());
            ui.vertical(|ui| {
                ui.label(
                    RichText::new(title)
                        .size(theme.font_size_heading())
                        .color(theme.section_title()),
                );
                ui.label(
                    RichText::new(description)
                        .size(theme.font_size_sm())
                        .color(theme.section_description()),
                );
            });
        });
    });

    response.response
}
