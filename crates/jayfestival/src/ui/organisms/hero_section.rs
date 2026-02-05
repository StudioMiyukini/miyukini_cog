//! Organism HeroSection — section héro (titre, sous-titre, CTA optionnel).
//!
//! Conforme [Specification UI] § 2. Style Catakana_Orga : panneau semi-transparent, titre mis en avant.

use crate::theme::JayFestivalTheme;
use crate::ui::{button, label, ButtonVariant, LabelLevel};
use eframe::egui::{Response, Ui};

/// @id: organism_hero_section
/// @do: define_hero_section_with_title_subtitle_cta
/// @layer: ui
/// @human: Section d'accroche avec titre, sous-titre et bouton CTA optionnel (panneau semi-transparent).

/// @id: hero_section_render
/// @do: render_hero_section_with_optional_cta
/// @layer: ui
/// Dessine la section héro dans un panneau semi-transparent : titre grand, sous-titre, CTA optionnel.
/// Retourne la réponse du bouton CTA si présent.
pub fn hero_section_render(
    ui: &mut Ui,
    theme: &JayFestivalTheme,
    title: &str,
    subtitle: &str,
    cta_label: Option<&str>,
) -> Response {
    let frame = eframe::egui::Frame::default()
        .fill(theme.section_background())
        .stroke(eframe::egui::Stroke::new(1.0, theme.section_border()))
        .corner_radius(eframe::egui::CornerRadius::same(theme.radius_large() as u8))
        .inner_margin(eframe::egui::Margin::same((theme.card_padding() * 1.5) as i8));

    let out = frame.show(ui, |ui| {
        ui.add_space(theme.item_spacing() * 2.0);
        ui.label(
            eframe::egui::RichText::new(title)
                .size(theme.font_size_heading() + 8.0)
                .color(theme.section_title()),
        );
        ui.add_space(theme.item_spacing());
        label(ui, theme, subtitle, LabelLevel::Muted);
        let r = if let Some(cta) = cta_label {
            ui.add_space(theme.item_spacing() * 1.5);
            button(ui, theme, cta, ButtonVariant::Primary, Default::default())
        } else {
            ui.allocate_response(eframe::egui::vec2(0.0, 0.0), eframe::egui::Sense::hover())
        };
        ui.add_space(theme.item_spacing() * 2.0);
        r
    });
    out.inner
}
