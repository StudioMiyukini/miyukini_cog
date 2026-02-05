//! Atom IconWrapper — affichage d'une icône (emoji ou glyphe) avec taille et couleur du thème.
//!
//! Conforme Specification UI § 2.1 A1. Tokens : fonts.sizes.sm/md/lg, colors.section.title ou text.primary.

use crate::theme::JayFestivalTheme;
use eframe::egui::{Color32, Response, RichText, Ui};

/// Taille d'icône (sm=14, md=16, lg=20 pt).
#[derive(Clone, Copy, Debug, Default)]
pub enum IconSize {
    /// 14 pt.
    #[default]
    Sm,
    /// 16 pt.
    Md,
    /// 20 pt.
    Lg,
}

impl IconSize {
    /// Taille en points depuis le thème.
    pub fn size_pt(self, theme: &JayFestivalTheme) -> f32 {
        match self {
            IconSize::Sm => theme.font_size_sm(),
            IconSize::Md => theme.font_size_md(),
            IconSize::Lg => theme.font_size_lg(),
        }
    }
}

/// @id: atom_icon_wrapper
/// @do: wrap_icon_display_with_theme_size_and_color
/// @layer: ui
/// @human: Encapsule l'affichage d'une icône (caractère unicode) avec taille et couleur du thème.
///
/// Affiche une icône (emoji ou glyphe unicode) à la taille et couleur demandées ; pas d'interaction.
pub fn icon_wrapper(
    ui: &mut Ui,
    theme: &JayFestivalTheme,
    icon_char: char,
    size: IconSize,
    color: Option<Color32>,
) -> Response {
    let size_pt = size.size_pt(theme);
    let c = color.unwrap_or(theme.section_title());
    let rich = RichText::new(icon_char.to_string()).size(size_pt).color(c);
    ui.label(rich)
}

/// @id: icon_wrapper_render
/// @do: render_icon_in_ui_with_theme
/// @layer: ui
/// Alias de rendu : même signature que icon_wrapper pour cohérence avec le plan MSCM.
#[inline(always)]
pub fn icon_wrapper_render(
    ui: &mut Ui,
    theme: &JayFestivalTheme,
    icon_char: char,
    size: IconSize,
    color: Option<Color32>,
) -> Response {
    icon_wrapper(ui, theme, icon_char, size, color)
}
