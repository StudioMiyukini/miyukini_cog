//! Thème type « store » : moderne, dynamique, engageant.
//!
//! Inspiré des stores (Steam, Microsoft Store) : coins arrondis, couleurs d'accent,
//! cartes visuelles, espacement généreux.
//!
//! **Legacy :** Ce module n'est pas utilisé par Miyukini Central. L'app utilise
//! [pixel_theme](crate::pixel_theme) et [EditorThemeState](crate::services::EditorThemeState).
//! Pour harmoniser avec les thèmes, préférer `ctx.style().visuals` ou les couleurs
//! résolues via le Hub (global_ui_theme / resolve_chrome).

use eframe::egui;

/// Couleur d'accent (bleu-cyan type store).
const ACCENT_RGB: [u8; 3] = [0, 120, 215]; // Bleu Microsoft Store–like
const ACCENT_HOVER_RGB: [u8; 3] = [30, 144, 255];
const CARD_BG_ALPHA_DARK: u8 = 45;
const CARD_BG_ALPHA_LIGHT: u8 = 230;
/// Rayon des cartes (store), en pixels (u8 pour egui).
const CARD_CORNER_RADIUS: u8 = 12;
const CORNER_RADIUS_SMALL: u8 = 8;

fn accent_color() -> egui::Color32 {
    egui::Color32::from_rgb(ACCENT_RGB[0], ACCENT_RGB[1], ACCENT_RGB[2])
}

fn accent_hover_color() -> egui::Color32 {
    egui::Color32::from_rgb(ACCENT_HOVER_RGB[0], ACCENT_HOVER_RGB[1], ACCENT_HOVER_RGB[2])
}

/// Applique le thème « store » (sombre ou clair).
pub fn apply_store_theme(ctx: &egui::Context, dark: bool) {
    let mut style = (*ctx.style()).clone();
    let visuals = if dark {
        egui::Visuals::dark()
    } else {
        egui::Visuals::light()
    };

    let accent = accent_color();
    let _accent_hover = accent_hover_color();

    // Coins arrondis sur les widgets (egui 0.33 : CornerRadius)
    let radius = egui::CornerRadius::same(CORNER_RADIUS_SMALL);
    style.visuals = visuals;
    style.visuals.widgets.noninteractive.corner_radius = radius;
    style.visuals.widgets.inactive.corner_radius = radius;
    style.visuals.widgets.hovered.corner_radius = radius;
    style.visuals.widgets.active.corner_radius = radius;

    // Boutons plus ronds et visibles
    style.visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(60, 60, 60);
    style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(80, 80, 80);
    style.visuals.widgets.active.bg_fill = accent;

    if !dark {
        style.visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(220, 220, 220);
        style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(200, 200, 200);
        style.visuals.widgets.active.bg_fill = accent;
    }

    // Espacement plus généreux
    style.spacing.button_padding = egui::vec2(16.0, 8.0);
    style.spacing.item_spacing = egui::vec2(12.0, 8.0);
    style.spacing.window_margin = egui::Margin::same(16i8);

    ctx.set_style(style);
}

/// Rayon des cartes (store), pour Frame::corner_radius (egui 0.33).
#[must_use] 
pub fn card_corner_radius() -> u8 {
    CARD_CORNER_RADIUS
}

/// Fond de carte (store) selon thème.
#[must_use] 
pub fn card_bg_color(dark: bool) -> egui::Color32 {
    if dark {
        egui::Color32::from_rgba_unmultiplied(50, 50, 55, CARD_BG_ALPHA_DARK)
    } else {
        egui::Color32::from_rgba_unmultiplied(255, 255, 255, CARD_BG_ALPHA_LIGHT)
    }
}

/// Couleur d'accent pour bouton principal.
#[must_use] 
pub fn primary_button_color() -> egui::Color32 {
    accent_color()
}

/// Couleur au survol du bouton principal.
#[must_use] 
pub fn primary_button_hover_color() -> egui::Color32 {
    accent_hover_color()
}
