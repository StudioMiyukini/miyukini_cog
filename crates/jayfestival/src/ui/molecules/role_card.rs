//! RoleCard — molecule pastille colorée, titre, description optionnelle (Specification UI § 2.2 M3).

use crate::theme::JayFestivalTheme;
use eframe::egui::{self, Color32, RichText, Shape};

/// @id: molecule_role_card
/// @do: define_role_card_props
/// @layer: ui
/// Carte rôle : pastille colorée (organisateur / exposant / visiteur / bénévole), titre, description optionnelle.

/// @id: role_card_render
/// @do: render_role_card_with_accent_pill
/// @layer: ui
/// Affiche une petite carte avec pastille colorée (cercle) + titre (+ description) ; optionnellement cliquable.
/// Tokens : section.card.background, radius_medium, card_padding ; accent_color = organisateur / exposant / visiteur.
pub fn role_card_render(
    theme: &JayFestivalTheme,
    ui: &mut egui::Ui,
    title: &str,
    description: Option<&str>,
    accent_color: Color32,
) -> egui::Response {
    let frame = egui::Frame::default()
        .fill(theme.section_card_background())
        .stroke(egui::Stroke::new(1.0, theme.section_border()))
        .corner_radius(egui::CornerRadius::same(theme.radius_medium() as u8))
        .inner_margin(egui::Margin::same(theme.card_padding() as i8));

    let response = frame.show(ui, |ui| {
        ui.horizontal(|ui| {
            let size = theme.font_size_md();
            let (rect, _) = ui.allocate_exact_size(
                egui::vec2(size, size),
                egui::Sense::hover(),
            );
            let center = rect.center();
            let r = size * 0.35;
            ui.painter().add(Shape::circle_filled(
                center,
                r,
                accent_color,
            ));
            ui.add_space(theme.item_spacing());
            ui.vertical(|ui| {
                ui.label(
                    RichText::new(title)
                        .size(theme.font_size_heading())
                        .color(theme.section_title()),
                );
                if let Some(desc) = description {
                    ui.label(
                        RichText::new(desc)
                            .size(theme.font_size_sm())
                            .color(theme.section_description()),
                    );
                }
            });
        });
    });

    response.response
}
