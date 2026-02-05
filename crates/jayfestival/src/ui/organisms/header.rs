//! Organism Header — barre supérieure avec logo (carré orange) et boutons de navigation.
//!
//! Conforme [Specification UI Conforme Catakana] § 2, style Catakana_Orga.
//! Logo = carré orange + initiale + titre ; Connexion (Outline), S'inscrire (Primary).

use crate::theme::JayFestivalTheme;
use crate::ui::{button, label, ButtonVariant, LabelLevel};
use eframe::egui::{Response, Ui};

/// @id: organism_header
/// @do: define_header_component_with_logo_and_nav
/// @layer: ui
/// @human: Barre d'en-tête avec logo (carré orange Catakana_Orga), titre et boutons nav/auth.

/// @id: header_render
/// @do: render_header_bar_with_logo_and_nav_buttons
/// @layer: ui
/// Dessine la barre d'en-tête : logo (carré orange + J), titre, puis nav (Ghost) et auth (Outline + Primary).
/// Les deux derniers éléments de nav_labels sont stylés Connexion (Outline) et S'inscrire (Primary).
/// Retourne les réponses des boutons dans l'ordre nav_labels.
pub fn header_render(
    ui: &mut Ui,
    theme: &JayFestivalTheme,
    title: &str,
    nav_labels: &[&str],
) -> Vec<Response> {
    let mut responses = Vec::with_capacity(nav_labels.len());
    let n = nav_labels.len();
    let auth_start = n.saturating_sub(2); // indices auth : avant-dernier = Outline, dernier = Primary

    ui.horizontal(|ui| {
        // Logo Catakana_Orga : carré orange + initiale "J" en blanc
        let size = 32.0_f32;
        let (rect, _) = ui.allocate_exact_size(
            eframe::egui::vec2(size, size),
            eframe::egui::Sense::hover(),
        );
        let rounding = eframe::egui::CornerRadius::same(4);
        ui.painter().rect_filled(rect, rounding, theme.logo_fill());
        ui.painter().text(
            rect.center(),
            eframe::egui::Align2::CENTER_CENTER,
            "J",
            eframe::egui::FontId::new(18.0, eframe::egui::FontFamily::Proportional),
            theme.logo_text(),
        );
        ui.allocate_rect(rect, eframe::egui::Sense::hover()); // consommer l'espace
        ui.add_space(theme.item_spacing());
        label(ui, theme, title, LabelLevel::Heading);

        ui.with_layout(
            eframe::egui::Layout::right_to_left(eframe::egui::Align::Center),
            |ui| {
                for (i, &label_text) in nav_labels.iter().enumerate().rev() {
                    let variant = if i == n - 1 {
                        ButtonVariant::Primary
                    } else if i == auth_start && auth_start < n {
                        ButtonVariant::Outline
                    } else {
                        ButtonVariant::Ghost
                    };
                    let r = button(ui, theme, label_text, variant, Default::default());
                    responses.push(r);
                }
            },
        );
    });
    responses.reverse();
    responses
}
