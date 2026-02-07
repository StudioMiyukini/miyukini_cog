//! Module UI JayXpose — fonctions helper pour composants communs.
//!
//! @id: jayxpose_ui_helpers
//! @do: provide_ui_helper_functions
//! @layer: ui
//!
//! Composants UI réutilisables par les écrans JayXpose (Atomic Design atoms/molecules).

use crate::theme::JayXposeTheme;
use eframe::egui::{self, Color32, Response, RichText, Ui};

// ---------------------------------------------------------------------------
// Texte
// ---------------------------------------------------------------------------

/// Affiche un label avec le style heading (titre de section).
pub fn heading(ui: &mut Ui, theme: &JayXposeTheme, text: &str) {
    ui.label(
        RichText::new(text)
            .size(theme.font_size_heading())
            .color(theme.text_primary())
            .strong(),
    );
}

/// Affiche un label avec le style body (texte principal).
pub fn body(ui: &mut Ui, theme: &JayXposeTheme, text: &str) {
    ui.label(
        RichText::new(text)
            .size(theme.font_size_body())
            .color(theme.text_primary()),
    );
}

/// Affiche un label secondaire (petite taille, couleur secondaire).
pub fn caption(ui: &mut Ui, theme: &JayXposeTheme, text: &str) {
    ui.label(
        RichText::new(text)
            .size(theme.font_size_small())
            .color(theme.text_secondary()),
    );
}

// ---------------------------------------------------------------------------
// Badge
// ---------------------------------------------------------------------------

/// Affiche un badge (chip) coloré — texte blanc sur fond de couleur.
pub fn badge(ui: &mut Ui, text: &str, color: Color32) {
    let label = RichText::new(text).size(11.0).color(Color32::WHITE);
    egui::Frame::new()
        .fill(color)
        .corner_radius(4.0)
        .inner_margin(egui::Margin::symmetric(6, 2))
        .show(ui, |ui| {
            ui.label(label);
        });
}

// ---------------------------------------------------------------------------
// Boutons
// ---------------------------------------------------------------------------

/// Affiche un bouton accent (couleur du thème, texte contrasté).
pub fn accent_button(ui: &mut Ui, theme: &JayXposeTheme, text: &str) -> Response {
    let button = egui::Button::new(
        RichText::new(text)
            .color(theme.text_on_accent())
            .size(theme.font_size_body()),
    )
    .fill(theme.accent());
    ui.add(button)
}

/// Affiche un bouton secondaire (outline, texte accent, fond transparent).
pub fn secondary_button(ui: &mut Ui, theme: &JayXposeTheme, text: &str) -> Response {
    let button = egui::Button::new(
        RichText::new(text)
            .color(theme.accent())
            .size(theme.font_size_body()),
    )
    .fill(Color32::TRANSPARENT)
    .stroke(egui::Stroke::new(1.0, theme.accent()));
    ui.add(button)
}

// ---------------------------------------------------------------------------
// Conteneurs
// ---------------------------------------------------------------------------

/// Affiche une carte avec fond et marge arrondie.
pub fn card(ui: &mut Ui, theme: &JayXposeTheme, content: impl FnOnce(&mut Ui)) {
    egui::Frame::new()
        .fill(theme.card_background())
        .corner_radius(theme.border_radius())
        .inner_margin(egui::Margin::same(12))
        .show(ui, |ui| {
            content(ui);
        });
}

/// Affiche une section avec titre et contenu.
pub fn section(
    ui: &mut Ui,
    theme: &JayXposeTheme,
    title: &str,
    content: impl FnOnce(&mut Ui),
) {
    ui.add_space(theme.section_spacing());
    heading(ui, theme, title);
    ui.add_space(theme.item_spacing());
    content(ui);
}

// ---------------------------------------------------------------------------
// Formulaires
// ---------------------------------------------------------------------------

/// Affiche un champ de formulaire labellisé (label + text_edit singleline).
pub fn form_field(ui: &mut Ui, theme: &JayXposeTheme, label: &str, value: &mut String) {
    ui.label(
        RichText::new(label)
            .size(theme.font_size_body())
            .color(theme.text_secondary()),
    );
    ui.text_edit_singleline(value);
}

/// Affiche un champ textarea labellisé (label + text_edit multiline).
pub fn form_textarea(ui: &mut Ui, theme: &JayXposeTheme, label: &str, value: &mut String) {
    ui.label(
        RichText::new(label)
            .size(theme.font_size_body())
            .color(theme.text_secondary()),
    );
    ui.text_edit_multiline(value);
}

// ---------------------------------------------------------------------------
// Couleurs de statut
// ---------------------------------------------------------------------------

/// Couleur pour le badge de statut document.
pub fn doc_status_color(status: &str) -> Color32 {
    match status {
        "valide" => Color32::from_rgb(34, 197, 94),     // vert
        "en_attente" => Color32::from_rgb(249, 115, 22), // orange
        "expire" => Color32::from_rgb(239, 68, 68),     // rouge
        "rejete" => Color32::from_rgb(239, 68, 68),     // rouge
        _ => Color32::GRAY,
    }
}

/// Couleur pour le badge de disponibilité produit.
pub fn availability_color(availability: &str) -> Color32 {
    match availability {
        "disponible" => Color32::from_rgb(34, 197, 94),
        "rupture" => Color32::from_rgb(239, 68, 68),
        "sur_commande" => Color32::from_rgb(249, 115, 22),
        _ => Color32::GRAY,
    }
}
