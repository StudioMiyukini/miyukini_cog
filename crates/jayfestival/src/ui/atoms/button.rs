//! Atom Button — bouton avec variantes Primary, Secondary, Outline, Ghost.
//!
//! Conforme Specification UI § 2.1 A2. Tokens : navigation.button, borders.radius.medium, spacing.button_padding, fonts.sizes.md.

use crate::theme::JayFestivalTheme;
use eframe::egui::{Color32, Response, RichText, Ui};

/// Variante visuelle du bouton (shadcn).
#[derive(Clone, Copy, Debug, Default)]
pub enum ButtonVariant {
    /// Fond accent (primaire).
    Primary,
    /// Fond secondaire, bordure.
    #[default]
    Secondary,
    /// Bordure, fond transparent.
    Outline,
    /// Transparent, hover léger.
    Ghost,
    /// Item de nav sidebar sélectionné (fond orange Catakana_Orga).
    NavSelected,
}

/// Taille du bouton (sm, md, lg).
#[derive(Clone, Copy, Debug, Default)]
pub enum ButtonSize {
    /// Petit (32 px hauteur min).
    Sm,
    /// Moyen (40 px, PROTO-7).
    #[default]
    Md,
    /// Grand (48 px).
    Lg,
}

impl ButtonSize {
    /// Hauteur minimale du bouton en pixels (accessibilité PROTO-7).
    pub fn min_height(self) -> f32 {
        match self {
            ButtonSize::Sm => 32.0,
            ButtonSize::Md => 40.0,
            ButtonSize::Lg => 48.0,
        }
    }
}

/// @id: atom_button
/// @do: define_button_component_with_theme_variants
/// @layer: ui
/// @human: Composant bouton avec variantes Primary/Secondary/Outline/Ghost et taille, style via thème.
fn button_style(theme: &JayFestivalTheme, variant: ButtonVariant) -> (Color32, Color32, Color32) {
    let (bg, stroke, text) = match variant {
        ButtonVariant::Primary => (
            theme.accent_primary(),
            theme.accent_primary(),
            Color32::WHITE,
        ),
        ButtonVariant::Secondary => (
            theme.navigation_button_normal(),
            theme.section_border(),
            theme.text_primary(),
        ),
        ButtonVariant::Outline => (
            Color32::TRANSPARENT,
            theme.accent_primary(),
            theme.text_primary(),
        ),
        ButtonVariant::Ghost => (
            Color32::TRANSPARENT,
            Color32::TRANSPARENT,
            theme.text_primary(),
        ),
        ButtonVariant::NavSelected => (
            theme.navigation_button_active(),
            theme.navigation_button_active(),
            Color32::WHITE,
        ),
    };
    (bg, stroke, text)
}

/// @id: button_render
/// @do: render_button_in_ui_with_theme_and_return_response
/// @layer: ui
/// Dessine un bouton avec label, variante et taille ; retourne la Response au clic.
/// Zone cliquable minimale 40 px (PROTO-7) : min_height au moins 40 pour Md.
pub fn button(
    ui: &mut Ui,
    theme: &JayFestivalTheme,
    label: &str,
    variant: ButtonVariant,
    size: ButtonSize,
) -> Response {
    let (bg, stroke, text_color) = button_style(theme, variant);
    let min_height = size.min_height().max(40.0);
    let radius = theme.radius_medium();
    let font_size = theme.font_size_md();

    let rich = RichText::new(label).size(font_size).color(text_color);
    let btn = egui::Button::new(rich)
        .fill(bg)
        .stroke(egui::Stroke::new(1.0, stroke))
        .corner_radius(egui::CornerRadius::same(radius as u8))
        .min_size(egui::vec2(0.0, min_height));
    ui.add(btn)
}
