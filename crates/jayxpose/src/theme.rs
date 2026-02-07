//! Thème JayXpose — tokens UI (couleurs, rayons, espacements, polices).
//!
//! Style professionnel JayXpose : accent bleu vif, fonds slate foncé, panneaux épurés.
//! Distinct du orange Catakana de JayFestival.
//!
//! @id: jayxpose_theme_struct
//! @do: hold_theme_tokens_and_apply_to_egui
//! @layer: ui

use eframe::egui::{self, Color32, CornerRadius, Visuals};

// ---------------------------------------------------------------------------
// Palette JayXpose (bleu professionnel)
// ---------------------------------------------------------------------------

/// Bleu vif principal (#3B82F6).
const XPOSE_BLUE: (u8, u8, u8) = (59, 130, 246);
/// Bleu foncé hover / accent secondaire (#2563EB).
const XPOSE_BLUE_DARK: (u8, u8, u8) = (37, 99, 235);
/// Slate foncé sidebar (#1E293B).
const XPOSE_SIDEBAR: (u8, u8, u8) = (30, 41, 59);
/// Très foncé header (#0F172A).
const XPOSE_HEADER: (u8, u8, u8) = (15, 23, 42);
/// Slate section (#334155).
const XPOSE_SECTION: (u8, u8, u8) = (51, 65, 85);
/// Slate card (#1E293B).
const XPOSE_CARD: (u8, u8, u8) = (30, 41, 59);

/// @id: jayxpose_theme_tokens
/// @do: define_colors_radius_spacing_fonts
/// @layer: ui
/// Struct principale du thème JayXpose (tokens couleurs, borders, spacing, fonts).
#[derive(Clone)]
pub struct JayXposeTheme {
    /// Mode sombre (true) ou clair (false).
    pub dark_mode: bool,
}

impl JayXposeTheme {
    /// Crée un thème JayXpose (mode clair ou sombre).
    pub fn new(dark_mode: bool) -> Self {
        Self { dark_mode }
    }

    // === Couleurs ===

    /// Fond général / background primary (slate très foncé en mode dark).
    pub fn background_primary(&self) -> Color32 {
        if self.dark_mode {
            Color32::from_rgb(XPOSE_HEADER.0, XPOSE_HEADER.1, XPOSE_HEADER.2)
        } else {
            Color32::from_rgb(248, 250, 252) // slate-50
        }
    }

    /// Fond section (slate section semi-transparent en dark).
    pub fn section_background(&self) -> Color32 {
        if self.dark_mode {
            Color32::from_rgba_unmultiplied(
                XPOSE_SECTION.0,
                XPOSE_SECTION.1,
                XPOSE_SECTION.2,
                102,
            )
        } else {
            Color32::from_rgba_unmultiplied(255, 255, 255, 102)
        }
    }

    /// Fond carte (slate card).
    pub fn card_background(&self) -> Color32 {
        if self.dark_mode {
            Color32::from_rgba_unmultiplied(XPOSE_CARD.0, XPOSE_CARD.1, XPOSE_CARD.2, 200)
        } else {
            Color32::from_rgba_unmultiplied(255, 255, 255, 230)
        }
    }

    /// Fond sidebar (slate foncé).
    pub fn sidebar_background(&self) -> Color32 {
        if self.dark_mode {
            Color32::from_rgb(XPOSE_SIDEBAR.0, XPOSE_SIDEBAR.1, XPOSE_SIDEBAR.2)
        } else {
            Color32::from_rgb(248, 250, 252)
        }
    }

    /// Fond header (très foncé).
    pub fn header_background(&self) -> Color32 {
        if self.dark_mode {
            Color32::from_rgb(XPOSE_HEADER.0, XPOSE_HEADER.1, XPOSE_HEADER.2)
        } else {
            Color32::from_rgb(255, 255, 255)
        }
    }

    /// Accent primaire (bleu vif #3B82F6).
    pub fn accent(&self) -> Color32 {
        Color32::from_rgb(XPOSE_BLUE.0, XPOSE_BLUE.1, XPOSE_BLUE.2)
    }

    /// Accent foncé (bleu hover #2563EB).
    pub fn accent_dark(&self) -> Color32 {
        Color32::from_rgb(XPOSE_BLUE_DARK.0, XPOSE_BLUE_DARK.1, XPOSE_BLUE_DARK.2)
    }

    /// Texte principal (body).
    pub fn text_primary(&self) -> Color32 {
        if self.dark_mode {
            Color32::from_rgb(248, 250, 252) // slate-50
        } else {
            Color32::from_rgb(15, 23, 42) // slate-900
        }
    }

    /// Texte secondaire (descriptions, labels).
    pub fn text_secondary(&self) -> Color32 {
        if self.dark_mode {
            Color32::from_rgb(148, 163, 184) // slate-400
        } else {
            Color32::from_rgb(100, 116, 139) // slate-500
        }
    }

    /// Texte sur accent (blanc sur bleu).
    pub fn text_on_accent(&self) -> Color32 {
        Color32::from_rgb(255, 255, 255)
    }

    // === Borders / Radius ===

    /// Rayon de bordure par défaut (8 px).
    pub fn border_radius(&self) -> f32 {
        8.0
    }

    // === Spacing ===

    /// Espacement entre éléments (item_spacing).
    pub fn item_spacing(&self) -> f32 {
        8.0
    }

    /// Espacement entre sections.
    pub fn section_spacing(&self) -> f32 {
        16.0
    }

    // === Fonts ===

    /// Taille heading (titre section, 22 pt).
    pub fn font_size_heading(&self) -> f32 {
        22.0
    }

    /// Taille body (14 pt).
    pub fn font_size_body(&self) -> f32 {
        14.0
    }

    /// Taille small (12 pt).
    pub fn font_size_small(&self) -> f32 {
        12.0
    }

    // === Application ===

    /// Applique le thème au contexte egui (visuals + style de base).
    ///
    /// @id: jayxpose_theme_apply
    /// @do: apply_jayxpose_visuals_to_egui_context
    /// @layer: ui
    pub fn apply(&self, ctx: &egui::Context) {
        let mut visuals = if self.dark_mode {
            Visuals::dark()
        } else {
            Visuals::light()
        };

        // Override backgrounds
        visuals.panel_fill = self.background_primary();
        visuals.window_fill = self.background_primary();
        visuals.extreme_bg_color = if self.dark_mode {
            Color32::from_rgb(XPOSE_SIDEBAR.0, XPOSE_SIDEBAR.1, XPOSE_SIDEBAR.2)
        } else {
            Color32::from_rgb(241, 245, 249) // slate-100
        };

        // Override selection (accent bleu)
        visuals.selection.bg_fill = self.accent();
        visuals.selection.stroke.color = self.text_on_accent();

        // Override widget colors
        visuals.widgets.noninteractive.bg_fill = self.card_background();
        visuals.widgets.inactive.bg_fill = if self.dark_mode {
            Color32::from_rgb(XPOSE_SECTION.0, XPOSE_SECTION.1, XPOSE_SECTION.2)
        } else {
            Color32::from_rgb(241, 245, 249)
        };
        visuals.widgets.hovered.bg_fill = self.accent();
        visuals.widgets.active.bg_fill = self.accent_dark();

        // Override border strokes
        let border_color = if self.dark_mode {
            Color32::from_rgb(51, 65, 85) // slate-700
        } else {
            Color32::from_rgb(226, 232, 240) // slate-200
        };
        visuals.widgets.noninteractive.bg_stroke.color = border_color;
        visuals.widgets.inactive.bg_stroke.color = border_color;

        ctx.set_visuals(visuals);

        let mut style = (*ctx.style()).clone();
        style.spacing.item_spacing.x = self.item_spacing();
        style.spacing.item_spacing.y = self.item_spacing();

        let radius = CornerRadius::same(self.border_radius() as u8);
        style.visuals.widgets.noninteractive.corner_radius = radius;
        style.visuals.widgets.inactive.corner_radius = radius;
        style.visuals.widgets.hovered.corner_radius = radius;
        style.visuals.widgets.active.corner_radius = radius;
        style.visuals.window_fill = self.background_primary();

        ctx.set_style(style);
    }
}

#[cfg(test)]
mod tests {
    use super::JayXposeTheme;
    use eframe::egui::Color32;

    #[test]
    fn theme_new_dark() {
        let t = JayXposeTheme::new(true);
        assert!(t.dark_mode);
    }

    #[test]
    fn theme_new_light() {
        let t = JayXposeTheme::new(false);
        assert!(!t.dark_mode);
    }

    #[test]
    fn theme_background_primary_dark_light_differ() {
        let dark = JayXposeTheme::new(true);
        let light = JayXposeTheme::new(false);
        assert_ne!(dark.background_primary(), light.background_primary());
    }

    #[test]
    fn theme_accent_is_blue() {
        let t = JayXposeTheme::new(true);
        assert_eq!(t.accent(), Color32::from_rgb(59, 130, 246));
    }

    #[test]
    fn theme_accent_dark_is_blue_dark() {
        let t = JayXposeTheme::new(true);
        assert_eq!(t.accent_dark(), Color32::from_rgb(37, 99, 235));
    }

    #[test]
    fn theme_text_on_accent_is_white() {
        let t = JayXposeTheme::new(true);
        assert_eq!(t.text_on_accent(), Color32::from_rgb(255, 255, 255));
    }

    #[test]
    fn theme_text_primary_dark_light_differ() {
        let dark = JayXposeTheme::new(true);
        let light = JayXposeTheme::new(false);
        assert_ne!(dark.text_primary(), light.text_primary());
    }

    #[test]
    fn theme_text_secondary_dark_light_differ() {
        let dark = JayXposeTheme::new(true);
        let light = JayXposeTheme::new(false);
        assert_ne!(dark.text_secondary(), light.text_secondary());
    }

    #[test]
    fn theme_border_radius() {
        let t = JayXposeTheme::new(false);
        assert!((t.border_radius() - 8.0).abs() < f32::EPSILON);
    }

    #[test]
    fn theme_spacing_constants() {
        let t = JayXposeTheme::new(false);
        assert!((t.item_spacing() - 8.0).abs() < f32::EPSILON);
        assert!((t.section_spacing() - 16.0).abs() < f32::EPSILON);
    }

    #[test]
    fn theme_font_sizes() {
        let t = JayXposeTheme::new(false);
        assert!((t.font_size_heading() - 22.0).abs() < f32::EPSILON);
        assert!((t.font_size_body() - 14.0).abs() < f32::EPSILON);
        assert!((t.font_size_small() - 12.0).abs() < f32::EPSILON);
    }

    #[test]
    fn theme_sidebar_dark_light_differ() {
        let dark = JayXposeTheme::new(true);
        let light = JayXposeTheme::new(false);
        assert_ne!(dark.sidebar_background(), light.sidebar_background());
    }

    #[test]
    fn theme_header_dark_light_differ() {
        let dark = JayXposeTheme::new(true);
        let light = JayXposeTheme::new(false);
        assert_ne!(dark.header_background(), light.header_background());
    }

    #[test]
    fn theme_card_dark_light_differ() {
        let dark = JayXposeTheme::new(true);
        let light = JayXposeTheme::new(false);
        assert_ne!(dark.card_background(), light.card_background());
    }

    #[test]
    fn theme_section_dark_light_differ() {
        let dark = JayXposeTheme::new(true);
        let light = JayXposeTheme::new(false);
        assert_ne!(dark.section_background(), light.section_background());
    }
}
