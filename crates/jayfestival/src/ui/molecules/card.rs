//! Card — conteneur shadcn-like avec header, body, footer optionnels (Specification UI § 2.2 M5).

use crate::theme::JayFestivalTheme;
use eframe::egui;

/// @id: molecule_card
/// @do: define_card_container_props
/// @layer: ui
/// Conteneur Card : zones header (optionnel), body (obligatoire), footer (optionnel).

/// @id: card_render
/// @do: render_card_with_header_body_footer
/// @layer: ui
/// Affiche un conteneur Frame avec header optionnel, body (closure), footer optionnel.
/// Tokens : section.card.background, section.border, radius_medium, card_padding.
pub fn card_show<B, F>(
    theme: &JayFestivalTheme,
    ui: &mut egui::Ui,
    header: Option<&str>,
    body: B,
    footer: Option<F>,
) where
    B: FnOnce(&mut egui::Ui),
    F: FnOnce(&mut egui::Ui),
{
    let frame = egui::Frame::default()
        .fill(theme.section_card_background())
        .stroke(egui::Stroke::new(1.0, theme.section_border()))
        .corner_radius(egui::CornerRadius::same(theme.radius_medium() as u8))
        .inner_margin(egui::Margin::same(theme.card_padding() as i8));

    frame.show(ui, |ui| {
        if let Some(h) = header {
            ui.heading(h);
            ui.add_space(theme.item_spacing());
        }
        body(ui);
        if let Some(f) = footer {
            ui.add_space(theme.item_spacing());
            f(ui);
        }
    });
}
