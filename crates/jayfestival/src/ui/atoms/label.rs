//! Atom Label — texte avec niveaux Heading, Body, Small, Muted.
//!
//! Conforme Specification UI § 2.1 A4. Tokens : section.title (heading), text.primary (body), section.description (muted), fonts.sizes.*.

use crate::theme::JayFestivalTheme;
use eframe::egui::{Response, RichText, Ui};

/// Niveau visuel du label (Heading, Body, Small, Muted).
#[derive(Clone, Copy, Debug, Default)]
pub enum LabelLevel {
    /// Titre de section (couleur titre, taille heading).
    Heading,
    /// Corps de texte (couleur primaire, taille md).
    #[default]
    Body,
    /// Petit texte (taille sm).
    Small,
    /// Texte atténué (couleur description, taille sm).
    Muted,
}

/// @id: atom_label
/// @do: define_label_component_with_theme_levels
/// @layer: ui
/// @human: Composant label avec niveaux Heading/Body/Small/Muted ; couleurs et tailles via thème.

/// @id: label_render
/// @do: render_label_text_with_theme_level
/// @layer: ui
/// Dessine un texte avec le niveau demandé (couleur et taille depuis le thème).
pub fn label(ui: &mut Ui, theme: &JayFestivalTheme, text: &str, level: LabelLevel) -> Response {
    let (color, size) = match level {
        LabelLevel::Heading => (theme.section_title(), theme.font_size_heading()),
        LabelLevel::Body => (theme.text_primary(), theme.font_size_md()),
        LabelLevel::Small => (theme.text_primary(), theme.font_size_sm()),
        LabelLevel::Muted => (theme.section_description(), theme.font_size_sm()),
    };
    let rich = RichText::new(text).color(color).size(size);
    ui.label(rich)
}
