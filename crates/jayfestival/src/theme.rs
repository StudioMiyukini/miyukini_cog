//! Thème JayFestival — tokens UI (couleurs, rayons, espacements, polices).
//!
//! Conforme [Specification UI Conforme Catakana] et [Reference UI Transcription Catakana] § 2.
//! Style Catakana_Orga : accent orange vif, fonds bruns/anthracite, panneaux semi-transparents.
//! Aucun style en dur (PROTO-1) : tous les composants utilisent ce thème.

use eframe::egui::{self, Color32, CornerRadius, Visuals};

/// @id: jayfestival_theme_struct
/// @do: hold_theme_tokens_and_apply_to_egui
/// @layer: ui
/// Struct principale du thème JayFestival (tokens couleurs, borders, spacing, fonts).
#[derive(Clone)]
pub struct JayFestivalTheme {
    /// Mode sombre (true) ou clair (false).
    pub dark_mode: bool,
}

// --- Palette Catakana_Orga (orange vif, bruns, anthracite) ---
const CATAKANA_ORANGE: (u8, u8, u8) = (249, 115, 22); // #F97316 orange vif
const CATAKANA_ORANGE_DARK: (u8, u8, u8) = (234, 88, 12); // #EA580C
const CATAKANA_BROWN_SIDEBAR: (u8, u8, u8) = (45, 38, 32); // brun foncé sidebar
const CATAKANA_BROWN_HEADER: (u8, u8, u8) = (36, 30, 26); // brun très foncé header
const CATAKANA_SECTION_DARK: (u8, u8, u8) = (55, 48, 42); // brun anthracite section
const CATAKANA_CARD_DARK: (u8, u8, u8) = (45, 38, 32); // brun carte
const CATAKANA_WHITE_ON_ORANGE: (u8, u8, u8) = (255, 255, 255);

/// @id: jayfestival_theme_tokens
/// @do: define_colors_radius_spacing_fonts
/// @layer: ui
/// Tokens : couleurs (section, card, navigation, accent), borders.radius, spacing, fonts.sizes.

impl JayFestivalTheme {
    /// Crée un thème JayFestival (mode clair ou sombre).
    #[must_use] 
    pub fn new(dark_mode: bool) -> Self {
        Self { dark_mode }
    }

    // --- Couleurs (Reference UI § 2.2, Catakana_Orga : orange, bruns) ---

    /// Fond général / background primary (brun très foncé en mode Catakana_Orga).
    #[must_use] 
    pub fn background_primary(&self) -> Color32 {
        if self.dark_mode {
            Color32::from_rgb(CATAKANA_BROWN_HEADER.0, CATAKANA_BROWN_HEADER.1, CATAKANA_BROWN_HEADER.2)
        } else {
            Color32::from_rgb(250, 250, 252)
        }
    }

    /// Fond section (opacité 0,4 — PROTO-5, brun anthracite semi-transparent).
    #[must_use] 
    pub fn section_background(&self) -> Color32 {
        if self.dark_mode {
            Color32::from_rgba_unmultiplied(
                CATAKANA_SECTION_DARK.0,
                CATAKANA_SECTION_DARK.1,
                CATAKANA_SECTION_DARK.2,
                102,
            )
        } else {
            Color32::from_rgba_unmultiplied(255, 255, 255, 102)
        }
    }

    /// Fond carte (section.card.background, brun foncé).
    #[must_use] 
    pub fn section_card_background(&self) -> Color32 {
        if self.dark_mode {
            Color32::from_rgba_unmultiplied(
                CATAKANA_CARD_DARK.0,
                CATAKANA_CARD_DARK.1,
                CATAKANA_CARD_DARK.2,
                200,
            )
        } else {
            Color32::from_rgba_unmultiplied(255, 255, 255, 230)
        }
    }

    /// Bordure section / carte (légèrement plus claire en brun).
    #[must_use] 
    pub fn section_border(&self) -> Color32 {
        if self.dark_mode {
            Color32::from_rgb(80, 70, 62)
        } else {
            Color32::from_rgb(228, 228, 231)
        }
    }

    /// Couleur titre de section.
    #[must_use] 
    pub fn section_title(&self) -> Color32 {
        if self.dark_mode {
            Color32::from_rgb(250, 250, 248)
        } else {
            Color32::from_rgb(24, 24, 27)
        }
    }

    /// Couleur description / texte secondaire.
    #[must_use] 
    pub fn section_description(&self) -> Color32 {
        if self.dark_mode {
            Color32::from_rgb(200, 190, 180)
        } else {
            Color32::from_rgb(113, 113, 122)
        }
    }

    /// Texte principal (body).
    #[must_use] 
    pub fn text_primary(&self) -> Color32 {
        if self.dark_mode {
            Color32::from_rgb(250, 250, 248)
        } else {
            Color32::from_rgb(24, 24, 27)
        }
    }

    /// Accent primaire (Catakana_Orga orange vif #F97316).
    #[must_use] 
    pub fn accent_primary(&self) -> Color32 {
        Color32::from_rgb(CATAKANA_ORANGE.0, CATAKANA_ORANGE.1, CATAKANA_ORANGE.2)
    }

    /// Accent secondaire (orange plus foncé pour hover/bordures).
    #[must_use] 
    pub fn accent_secondary(&self) -> Color32 {
        Color32::from_rgb(CATAKANA_ORANGE_DARK.0, CATAKANA_ORANGE_DARK.1, CATAKANA_ORANGE_DARK.2)
    }

    /// Accent organisateur (référence).
    #[must_use] 
    pub fn accent_organisateur(&self) -> Color32 {
        Color32::from_rgb(59, 130, 246)
    }

    /// Accent exposant (Green #10B981).
    #[must_use] 
    pub fn accent_exposant(&self) -> Color32 {
        Color32::from_rgb(16, 185, 129)
    }

    /// Accent visiteur / bénévole (Amber #F59E0B).
    #[must_use] 
    pub fn accent_visiteur(&self) -> Color32 {
        Color32::from_rgb(245, 158, 11)
    }

    /// Couleur succès (badge, validation — vert).
    #[must_use] 
    pub fn badge_success(&self) -> Color32 {
        Color32::from_rgb(16, 185, 129)
    }

    /// Couleur avertissement (badge — jaune/ambre).
    #[must_use] 
    pub fn badge_warning(&self) -> Color32 {
        Color32::from_rgb(245, 158, 11)
    }

    /// Couleur erreur (badge, refusé — rouge).
    #[must_use] 
    pub fn badge_error(&self) -> Color32 {
        Color32::from_rgb(239, 68, 68)
    }

    /// Fond navigation (sidebar, brun foncé Catakana_Orga).
    #[must_use] 
    pub fn navigation_container_background(&self) -> Color32 {
        if self.dark_mode {
            Color32::from_rgb(
                CATAKANA_BROWN_SIDEBAR.0,
                CATAKANA_BROWN_SIDEBAR.1,
                CATAKANA_BROWN_SIDEBAR.2,
            )
        } else {
            Color32::from_rgb(250, 250, 252)
        }
    }

    /// Bouton nav : normal, hover, active (couleurs pour widgets).
    /// Couleur bouton navigation à l'état normal.
    #[must_use] 
    pub fn navigation_button_normal(&self) -> Color32 {
        if self.dark_mode {
            Color32::from_rgb(55, 48, 42)
        } else {
            Color32::from_rgb(244, 244, 245)
        }
    }

    /// Couleur bouton navigation au survol.
    #[must_use] 
    pub fn navigation_button_hover(&self) -> Color32 {
        if self.dark_mode {
            Color32::from_rgb(70, 60, 52)
        } else {
            Color32::from_rgb(228, 228, 231)
        }
    }

    /// Couleur bouton navigation à l'état actif (sélectionné, orange Catakana_Orga).
    #[must_use] 
    pub fn navigation_button_active(&self) -> Color32 {
        if self.dark_mode {
            Color32::from_rgb(CATAKANA_ORANGE.0, CATAKANA_ORANGE.1, CATAKANA_ORANGE.2)
        } else {
            Color32::from_rgb(CATAKANA_ORANGE.0, CATAKANA_ORANGE.1, CATAKANA_ORANGE.2)
        }
    }

    /// Couleur logo carré (fond orange, texte blanc).
    #[must_use] 
    pub fn logo_fill(&self) -> Color32 {
        Color32::from_rgb(CATAKANA_ORANGE.0, CATAKANA_ORANGE.1, CATAKANA_ORANGE.2)
    }

    /// Couleur texte sur le logo (blanc sur orange).
    #[must_use] 
    pub fn logo_text(&self) -> Color32 {
        Color32::from_rgb(
            CATAKANA_WHITE_ON_ORANGE.0,
            CATAKANA_WHITE_ON_ORANGE.1,
            CATAKANA_WHITE_ON_ORANGE.2,
        )
    }

    // --- Borders radius (borders.radius small/medium/large) ---

    /// Rayon petit (4 px).
    #[must_use] 
    pub fn radius_small(&self) -> f32 {
        4.0
    }

    /// Rayon moyen (8 px).
    #[must_use] 
    pub fn radius_medium(&self) -> f32 {
        8.0
    }

    /// Rayon grand (12 px).
    #[must_use] 
    pub fn radius_large(&self) -> f32 {
        12.0
    }

    // --- Spacing (spacing.*) ---

    /// Padding bouton.
    #[must_use] 
    pub fn button_padding(&self) -> f32 {
        12.0
    }

    /// Padding input.
    #[must_use] 
    pub fn input_padding(&self) -> f32 {
        10.0
    }

    /// Padding carte.
    #[must_use] 
    pub fn card_padding(&self) -> f32 {
        16.0
    }

    /// Padding badge.
    #[must_use] 
    pub fn badge_padding(&self) -> f32 {
        6.0
    }

    /// Espacement entre éléments (item_spacing).
    #[must_use] 
    pub fn item_spacing(&self) -> f32 {
        8.0
    }

    // --- Fonts sizes (fonts.sizes.*) — 14 px sous 800 px, 16 px au-dessus (PROTO-6) ---

    /// Taille sm (14 pt).
    #[must_use] 
    pub fn font_size_sm(&self) -> f32 {
        14.0
    }

    /// Taille md (16 pt).
    #[must_use] 
    pub fn font_size_md(&self) -> f32 {
        16.0
    }

    /// Taille lg (20 pt).
    #[must_use] 
    pub fn font_size_lg(&self) -> f32 {
        20.0
    }

    /// Taille heading (titre section).
    #[must_use] 
    pub fn font_size_heading(&self) -> f32 {
        18.0
    }

    /// Applique le thème au contexte egui (visuals + style de base).
    pub fn apply(&self, ctx: &egui::Context) {
        let visuals = if self.dark_mode {
            Visuals::dark()
        } else {
            Visuals::light()
        };
        ctx.set_visuals(visuals);

        let mut style = (*ctx.style()).clone();
        style.spacing.item_spacing.x = self.item_spacing();
        style.spacing.item_spacing.y = self.item_spacing();
        let radius = CornerRadius::same(self.radius_medium() as u8);
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
    use super::JayFestivalTheme;
    use eframe::egui::Color32;

    #[test]
    fn theme_new_dark() {
        let t = JayFestivalTheme::new(true);
        assert!(t.dark_mode);
    }

    #[test]
    fn theme_new_light() {
        let t = JayFestivalTheme::new(false);
        assert!(!t.dark_mode);
    }

    #[test]
    fn theme_background_primary_dark_light_differ() {
        let dark = JayFestivalTheme::new(true);
        let light = JayFestivalTheme::new(false);
        assert_ne!(dark.background_primary(), light.background_primary());
    }

    #[test]
    fn theme_radius_constants() {
        let t = JayFestivalTheme::new(false);
        assert_eq!(t.radius_small(), 4.0);
        assert_eq!(t.radius_medium(), 8.0);
        assert_eq!(t.radius_large(), 12.0);
    }

    #[test]
    fn theme_spacing_constants() {
        let t = JayFestivalTheme::new(false);
        assert_eq!(t.button_padding(), 12.0);
        assert_eq!(t.input_padding(), 10.0);
        assert_eq!(t.card_padding(), 16.0);
        assert_eq!(t.badge_padding(), 6.0);
        assert_eq!(t.item_spacing(), 8.0);
    }

    #[test]
    fn theme_font_sizes() {
        let t = JayFestivalTheme::new(false);
        assert_eq!(t.font_size_sm(), 14.0);
        assert_eq!(t.font_size_md(), 16.0);
        assert_eq!(t.font_size_lg(), 20.0);
        assert_eq!(t.font_size_heading(), 18.0);
    }

    #[test]
    fn theme_accent_primary_fixed() {
        let t = JayFestivalTheme::new(false);
        let c = t.accent_primary();
        assert_eq!(c, Color32::from_rgb(249, 115, 22));
    }

    #[test]
    fn theme_accent_organisateur_exposant_visiteur_differ() {
        let t = JayFestivalTheme::new(false);
        assert_eq!(t.accent_organisateur(), Color32::from_rgb(59, 130, 246));
        assert_eq!(t.accent_exposant(), Color32::from_rgb(16, 185, 129));
        assert_eq!(t.accent_visiteur(), Color32::from_rgb(245, 158, 11));
    }

    #[test]
    fn theme_badge_colors() {
        let t = JayFestivalTheme::new(false);
        assert_eq!(t.badge_success(), Color32::from_rgb(16, 185, 129));
        assert_eq!(t.badge_warning(), Color32::from_rgb(245, 158, 11));
        assert_eq!(t.badge_error(), Color32::from_rgb(239, 68, 68));
    }

    #[test]
    fn theme_section_border_dark_light_differ() {
        let dark = JayFestivalTheme::new(true);
        let light = JayFestivalTheme::new(false);
        assert_ne!(dark.section_border(), light.section_border());
    }
}
