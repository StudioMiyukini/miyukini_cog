//! Atom Badge — pastille de statut (Default, Success, Warning, Error).
//!
//! Conforme Specification UI § 2.1 A5. Tokens : couleurs par variant, borders.radius.small, spacing.badge_padding, fonts.sizes.sm.

use crate::theme::JayFestivalTheme;
use eframe::egui::{Color32, Response, RichText, Ui};

/// Variante visuelle du badge (Default, Success, Warning, Error).
#[derive(Clone, Copy, Debug, Default)]
pub enum BadgeVariant {
    /// Style par défaut (neutre).
    #[default]
    Default,
    /// Validé / succès (vert).
    Success,
    /// En attente / avertissement (jaune).
    Warning,
    /// Refusé / erreur (rouge).
    Error,
}

/// @id: atom_badge
/// @do: define_badge_component_with_theme_variants
/// @layer: ui
/// @human: Composant pastille de statut ; couleurs et padding via thème.

fn badge_fill_and_text(theme: &JayFestivalTheme, variant: BadgeVariant) -> (Color32, Color32) {
    match variant {
        BadgeVariant::Default => (theme.navigation_button_normal(), theme.text_primary()),
        BadgeVariant::Success => (theme.badge_success(), Color32::WHITE),
        BadgeVariant::Warning => (theme.badge_warning(), Color32::from_rgb(24, 24, 27)),
        BadgeVariant::Error => (theme.badge_error(), Color32::WHITE),
    }
}

/// @id: badge_render
/// @do: render_badge_in_ui_with_theme
/// @layer: ui
/// Dessine une pastille de statut ; pas d'interaction.
pub fn badge(ui: &mut Ui, theme: &JayFestivalTheme, text: &str, variant: BadgeVariant) -> Response {
    let (fill, text_color) = badge_fill_and_text(theme, variant);
    let padding = theme.badge_padding();
    let radius = theme.radius_small();
    let font_size = theme.font_size_sm();
    let margin = egui::Margin::same(padding as i8);

    let rich = RichText::new(text).color(text_color).size(font_size);
    let frame = egui::Frame::new()
        .fill(fill)
        .corner_radius(egui::CornerRadius::same(radius as u8))
        .inner_margin(margin);

    frame.show(ui, |ui| ui.label(rich)).response
}
