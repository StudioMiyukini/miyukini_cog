//! Thème et tokens visuels JayKoa.
//!
//! @id: jaykoa_theme
//! @do: define_jaykoa_visual_tokens_and_colors
//! @role: theme
//! @layer: app

use egui::Color32;

/// Thème JayKoa : tokens couleurs, espacements, tailles.
pub struct JayKoaTheme {
    /// Mode sombre actif.
    pub dark_mode: bool,
    /// Fond principal.
    pub bg_primary: Color32,
    /// Fond secondaire (sidebar, header).
    pub bg_secondary: Color32,
    /// Fond de la grille calendrier.
    pub bg_grid: Color32,
    /// Lignes de la grille.
    pub grid_line: Color32,
    /// Texte principal.
    pub text_primary: Color32,
    /// Texte secondaire (labels, heures).
    pub text_secondary: Color32,
    /// Texte désactivé.
    pub text_muted: Color32,
    /// Accent principal (boutons, sélections).
    pub accent: Color32,
    /// Accent hover.
    pub accent_hover: Color32,
    /// Marqueur "Maintenant" (ligne rouge).
    pub now_marker: Color32,
    /// Aujourd'hui (cercle date).
    pub today_highlight: Color32,
    /// Couleur événements internes.
    pub event_internal: Color32,
    /// Couleur événements JayFestival.
    pub event_jayfestival: Color32,
    /// Couleur événements JayRDV.
    pub event_jayrdv: Color32,
    /// Couleur conflit.
    pub conflict_indicator: Color32,
    /// Padding intérieur des cellules.
    pub cell_padding: f32,
    /// Hauteur d'une heure dans la grille.
    pub hour_height: f32,
    /// Largeur de la colonne des heures.
    pub hour_column_width: f32,
    /// Largeur de la sidebar.
    pub sidebar_width: f32,
    /// Hauteur du header.
    pub header_height: f32,
}

impl JayKoaTheme {
    /// Crée le thème JayKoa (sombre par défaut).
    pub fn new(dark_mode: bool) -> Self {
        if dark_mode { Self::dark() } else { Self::light() }
    }

    fn dark() -> Self {
        Self {
            dark_mode: true,
            bg_primary: Color32::from_rgb(32, 33, 36), bg_secondary: Color32::from_rgb(41, 42, 45),
            bg_grid: Color32::from_rgb(32, 33, 36), grid_line: Color32::from_rgb(60, 64, 67),
            text_primary: Color32::from_rgb(232, 234, 237), text_secondary: Color32::from_rgb(154, 160, 166),
            text_muted: Color32::from_rgb(95, 99, 104),
            accent: Color32::from_rgb(138, 180, 248), accent_hover: Color32::from_rgb(174, 203, 250),
            now_marker: Color32::from_rgb(234, 67, 53), today_highlight: Color32::from_rgb(138, 180, 248),
            event_internal: Color32::from_rgb(66, 133, 244), event_jayfestival: Color32::from_rgb(230, 124, 115),
            event_jayrdv: Color32::from_rgb(51, 182, 121), conflict_indicator: Color32::from_rgb(251, 188, 4),
            cell_padding: 2.0, hour_height: 48.0, hour_column_width: 56.0,
            sidebar_width: 240.0, header_height: 48.0,
        }
    }

    fn light() -> Self {
        Self {
            dark_mode: false,
            bg_primary: Color32::WHITE, bg_secondary: Color32::from_rgb(241, 243, 244),
            bg_grid: Color32::WHITE, grid_line: Color32::from_rgb(218, 220, 224),
            text_primary: Color32::from_rgb(32, 33, 36), text_secondary: Color32::from_rgb(95, 99, 104),
            text_muted: Color32::from_rgb(154, 160, 166),
            accent: Color32::from_rgb(26, 115, 232), accent_hover: Color32::from_rgb(24, 90, 188),
            now_marker: Color32::from_rgb(234, 67, 53), today_highlight: Color32::from_rgb(26, 115, 232),
            event_internal: Color32::from_rgb(66, 133, 244), event_jayfestival: Color32::from_rgb(230, 124, 115),
            event_jayrdv: Color32::from_rgb(51, 182, 121), conflict_indicator: Color32::from_rgb(251, 188, 4),
            cell_padding: 2.0, hour_height: 48.0, hour_column_width: 56.0,
            sidebar_width: 240.0, header_height: 48.0,
        }
    }

    /// Couleur d'un bloc événement selon sa source.
    pub fn event_color_for_source(&self, source: &str) -> Color32 {
        match source {
            "jayfestival" | "reflect_jayfestival" => self.event_jayfestival,
            "jayrdv" | "reflect_jayrdv" => self.event_jayrdv,
            _ => self.event_internal,
        }
    }
}
