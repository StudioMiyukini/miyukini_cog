//! Thème pixel art style Chrome pour Miyukini Central.
//!
//! Ce module adapte les techniques CSS de chrome-tabs dans egui pour créer
//! une interface pixel art fidèle au style Chrome.

use eframe::egui::{self, Color32};

/// Couleurs Chrome exactes (style pixel art).
/// Palette mode clair : beige light (fridaahlsen) — #d9b99b barre exe, #fff0db header, #e4d5b7 sidebars.
#[allow(missing_docs)]
pub mod chrome_colors {
    use eframe::egui::Color32;

    /// Barre de l'exécutable (ligne 1 du header : titre + connexion) en mode clair.
    pub const BARRE_EXE_LIGHT: Color32 = Color32::from_rgb(217, 185, 155); // #d9b99b
    /// Fond de la barre d'onglets / header (ligne 2) en mode clair.
    pub const TAB_BAR_BG_LIGHT: Color32 = Color32::from_rgb(255, 240, 219); // #fff0db
    /// Fond des sidebars du Hub en mode clair.
    pub const SIDEBAR_BG_LIGHT: Color32 = Color32::from_rgb(228, 213, 183); // #e4d5b7

    /// Fond du header en mode nuit (gris moyen).
    pub const TAB_BAR_BG_DARK: Color32 = Color32::from_rgb(85, 85, 85);
    /// Barre exe en mode nuit (même zone que clair).
    pub const BARRE_EXE_DARK: Color32 = Color32::from_rgb(65, 65, 70);
    /// Sidebars Hub en mode nuit.
    pub const SIDEBAR_BG_DARK: Color32 = Color32::from_rgb(55, 55, 60);

    /// Fond du body (zone centrale) en mode nuit.
    pub const BODY_BG_DARK: Color32 = Color32::from_rgb(45, 45, 45);

    /// Onglet actif - fond blanc pur.
    pub const TAB_ACTIVE_BG_LIGHT: Color32 = Color32::WHITE;
    pub const TAB_ACTIVE_BG_DARK: Color32 = Color32::from_rgb(50, 50, 55);

    /// Onglet actif - texte très foncé.
    pub const TAB_ACTIVE_TEXT_LIGHT: Color32 = Color32::from_rgb(32, 33, 36);
    pub const TAB_ACTIVE_TEXT_DARK: Color32 = Color32::from_rgb(240, 240, 240);

    /// Onglet inactif - fond gris clair (niveau normal).
    pub const TAB_INACTIVE_BG_LIGHT: Color32 = Color32::from_rgb(236, 236, 236);
    pub const TAB_INACTIVE_BG_DARK: Color32 = Color32::from_rgb(60, 60, 65);

    /// Onglet inactif au survol - fond gris (niveau hover, entre normal et actif).
    pub const TAB_INACTIVE_HOVER_BG_LIGHT: Color32 = Color32::from_rgb(220, 220, 220);
    pub const TAB_INACTIVE_HOVER_BG_DARK: Color32 = Color32::from_rgb(70, 70, 75);

    /// Onglet inactif - texte gris moyen.
    pub const TAB_INACTIVE_TEXT_LIGHT: Color32 = Color32::from_rgb(95, 99, 104);
    pub const TAB_INACTIVE_TEXT_DARK: Color32 = Color32::from_rgb(180, 180, 185);

    /// Séparateur entre onglets inactifs.
    pub const TAB_SEPARATOR_LIGHT: Color32 = Color32::from_rgb(200, 200, 200);
    pub const TAB_SEPARATOR_DARK: Color32 = Color32::from_rgb(80, 80, 85);

    /// Bordure supérieure onglet actif.
    pub const TAB_ACTIVE_BORDER_LIGHT: Color32 = Color32::from_rgb(218, 220, 224);
    pub const TAB_ACTIVE_BORDER_DARK: Color32 = Color32::from_rgb(80, 80, 85);

    /// Bouton fermer - couleur par défaut.
    pub const CLOSE_BUTTON_LIGHT: Color32 = Color32::from_rgb(95, 99, 104);
    pub const CLOSE_BUTTON_DARK: Color32 = Color32::from_rgb(180, 180, 185);

    /// Bouton fermer - couleur au survol.
    pub const CLOSE_BUTTON_HOVER_LIGHT: Color32 = Color32::from_rgb(32, 33, 36);
    pub const CLOSE_BUTTON_HOVER_DARK: Color32 = Color32::from_rgb(240, 240, 240);

    /// Fond bouton fermer au survol (cercle gris).
    pub const CLOSE_BUTTON_BG_HOVER_LIGHT: Color32 = Color32::from_rgb(232, 234, 237);
    pub const CLOSE_BUTTON_BG_HOVER_DARK: Color32 = Color32::from_rgb(70, 70, 75);
}

/// Dimensions et constantes pour le style Chrome pixel art.
#[allow(missing_docs)]
pub mod chrome_dimensions {
    /// Hauteur des onglets (actif = collé au body, même hauteur que barre).
    pub const TAB_HEIGHT_ACTIVE: f32 = 32.0;
    pub const TAB_HEIGHT_INACTIVE: f32 = 32.0;

    /// Largeur des onglets.
    pub const TAB_WIDTH: f32 = 160.0;

    /// Rayon des coins arrondis en haut uniquement (5 px spec).
    pub const TAB_CORNER_RADIUS: u8 = 5;

    /// Espacement entre onglets (12 px spec).
    pub const TAB_GAP: f32 = 12.0;

    /// Offset vertical des onglets inactifs (alignés en bas avec l’actif).
    pub const TAB_INACTIVE_OFFSET_Y: f32 = 0.0;

    /// Espace (px) sous l'onglet inactif : l'onglet flotte au-dessus de la barre, laissant voir le fond.
    pub const INACTIVE_TAB_BOTTOM_GAP: f32 = 2.0;

    /// Padding horizontal dans les onglets.
    pub const TAB_PADDING_X: f32 = 12.0;
    pub const TAB_PADDING_Y: f32 = 6.0;

    /// Taille des icônes dans les onglets.
    pub const TAB_ICON_SIZE: f32 = 16.0;

    /// Taille du texte dans les onglets.
    pub const TAB_TEXT_SIZE: f32 = 12.0;

    /// Espacement entre icône et texte.
    pub const TAB_ICON_TEXT_SPACING: f32 = 8.0;

    /// Taille du bouton fermer.
    pub const CLOSE_BUTTON_SIZE: f32 = 18.0;
    pub const CLOSE_BUTTON_PADDING: f32 = 8.0;
    pub const CLOSE_CROSS_SIZE: f32 = 12.0;
    pub const CLOSE_CROSS_STROKE: f32 = 1.5;
}

/// Thème pixel art Chrome pour egui.
pub struct PixelChromeTheme {
    /// Mode sombre ou clair.
    pub dark_mode: bool,
}

impl PixelChromeTheme {
    /// Crée un nouveau thème Chrome pixel art.
    pub fn new(dark_mode: bool) -> Self {
        Self { dark_mode }
    }

    /// Applique le thème Chrome pixel art au contexte egui.
    pub fn apply(&self, ctx: &egui::Context) {
        use chrome_colors::*;
        use chrome_dimensions::*;

        let mut style = (*ctx.style()).clone();
        let visuals = if self.dark_mode {
            egui::Visuals::dark()
        } else {
            egui::Visuals::light()
        };

        style.visuals = visuals;

        // Coins arrondis pour les widgets (style Chrome)
        let radius = egui::CornerRadius::same(TAB_CORNER_RADIUS);
        style.visuals.widgets.noninteractive.corner_radius = radius;
        style.visuals.widgets.inactive.corner_radius = radius;
        style.visuals.widgets.hovered.corner_radius = radius;
        style.visuals.widgets.active.corner_radius = radius;

        // Couleurs des widgets selon le thème (constantes Chrome pour cohérence).
        if self.dark_mode {
            style.visuals.widgets.inactive.bg_fill = TAB_INACTIVE_BG_DARK;
            style.visuals.widgets.hovered.bg_fill = TAB_INACTIVE_HOVER_BG_DARK;
            style.visuals.widgets.active.bg_fill = TAB_ACTIVE_BG_DARK;
            style.visuals.override_text_color = Some(TAB_ACTIVE_TEXT_DARK);
        } else {
            style.visuals.widgets.inactive.bg_fill = TAB_INACTIVE_BG_LIGHT;
            style.visuals.widgets.hovered.bg_fill = TAB_INACTIVE_HOVER_BG_LIGHT;
            style.visuals.widgets.active.bg_fill = TAB_ACTIVE_BG_LIGHT;
            style.visuals.override_text_color = Some(TAB_ACTIVE_TEXT_LIGHT);
        }

        // Pas de contours noirs : strokes à zéro (header, panels, fenêtres, widgets type Recherche/HUB).
        let no_stroke = egui::Stroke::new(0.0, Color32::TRANSPARENT);
        style.visuals.window_stroke = no_stroke;
        style.visuals.widgets.noninteractive.bg_stroke = no_stroke;
        style.visuals.widgets.inactive.bg_stroke = no_stroke;
        style.visuals.widgets.hovered.bg_stroke = no_stroke;
        style.visuals.widgets.active.bg_stroke = no_stroke;

        // Espacement style Chrome
        style.spacing.button_padding = egui::vec2(12.0, 6.0);
        style.spacing.item_spacing = egui::vec2(8.0, 6.0);
        style.spacing.window_margin = egui::Margin::same(8i8);

        ctx.set_style(style);
    }

    /// Retourne la couleur de la barre exécutable (ligne 1 du header : titre + connexion).
    pub fn barre_exe_bg(&self) -> egui::Color32 {
        if self.dark_mode {
            chrome_colors::BARRE_EXE_DARK
        } else {
            chrome_colors::BARRE_EXE_LIGHT
        }
    }

    /// Retourne la couleur de fond de la barre d'onglets (ligne 2 du header).
    pub fn tab_bar_bg(&self) -> egui::Color32 {
        if self.dark_mode {
            chrome_colors::TAB_BAR_BG_DARK
        } else {
            chrome_colors::TAB_BAR_BG_LIGHT
        }
    }

    /// Retourne la couleur de fond des sidebars du Hub.
    pub fn sidebar_bg(&self) -> egui::Color32 {
        if self.dark_mode {
            chrome_colors::SIDEBAR_BG_DARK
        } else {
            chrome_colors::SIDEBAR_BG_LIGHT
        }
    }

    /// Retourne la couleur de fond du body (zone centrale).
    pub fn body_bg(&self) -> egui::Color32 {
        if self.dark_mode {
            chrome_colors::BODY_BG_DARK
        } else {
            egui::Color32::WHITE
        }
    }

    /// Retourne la couleur de fond d'un onglet actif.
    pub fn tab_active_bg(&self) -> egui::Color32 {
        if self.dark_mode {
            chrome_colors::TAB_ACTIVE_BG_DARK
        } else {
            chrome_colors::TAB_ACTIVE_BG_LIGHT
        }
    }

    /// Retourne la couleur de texte d'un onglet actif.
    pub fn tab_active_text(&self) -> egui::Color32 {
        if self.dark_mode {
            chrome_colors::TAB_ACTIVE_TEXT_DARK
        } else {
            chrome_colors::TAB_ACTIVE_TEXT_LIGHT
        }
    }

    /// Retourne la couleur de fond d'un onglet inactif.
    pub fn tab_inactive_bg(&self) -> egui::Color32 {
        if self.dark_mode {
            chrome_colors::TAB_INACTIVE_BG_DARK
        } else {
            chrome_colors::TAB_INACTIVE_BG_LIGHT
        }
    }

    /// Retourne la couleur de texte d'un onglet inactif.
    pub fn tab_inactive_text(&self) -> egui::Color32 {
        if self.dark_mode {
            chrome_colors::TAB_INACTIVE_TEXT_DARK
        } else {
            chrome_colors::TAB_INACTIVE_TEXT_LIGHT
        }
    }

    /// Retourne la couleur de fond d'un onglet inactif au survol (2e niveau de gris).
    pub fn tab_inactive_hover_bg(&self) -> egui::Color32 {
        if self.dark_mode {
            chrome_colors::TAB_INACTIVE_HOVER_BG_DARK
        } else {
            chrome_colors::TAB_INACTIVE_HOVER_BG_LIGHT
        }
    }

    /// Retourne la couleur du séparateur entre onglets.
    pub fn tab_separator(&self) -> egui::Color32 {
        if self.dark_mode {
            chrome_colors::TAB_SEPARATOR_DARK
        } else {
            chrome_colors::TAB_SEPARATOR_LIGHT
        }
    }

    /// Retourne la couleur de la bordure supérieure de l'onglet actif.
    pub fn tab_active_border(&self) -> egui::Color32 {
        if self.dark_mode {
            chrome_colors::TAB_ACTIVE_BORDER_DARK
        } else {
            chrome_colors::TAB_ACTIVE_BORDER_LIGHT
        }
    }
}

/// Fonctions utilitaires pour créer des formes Chrome dans egui.
/// Spec : onglets carrés en bas, arrondis 5 px en haut.
pub mod chrome_shapes {
    use super::chrome_dimensions;
    use eframe::egui;

    /// Crée la forme d'un onglet actif : carré en bas, arrondi 5 px en haut (même fond que body).
    pub fn create_active_tab_shape(
        rect: egui::Rect,
        bg_color: egui::Color32,
    ) -> Vec<egui::Shape> {
        use chrome_dimensions::TAB_CORNER_RADIUS;

        let shape = egui::Shape::rect_filled(
            rect,
            egui::CornerRadius {
                nw: TAB_CORNER_RADIUS,
                ne: TAB_CORNER_RADIUS,
                sw: 0,
                se: 0,
            },
            bg_color,
        );
        vec![shape]
    }

    /// Crée la forme d'un onglet inactif : carré en bas, arrondi 5 px en haut.
    pub fn create_inactive_tab_shape(
        rect: egui::Rect,
        bg_color: egui::Color32,
    ) -> egui::Shape {
        use chrome_dimensions::TAB_CORNER_RADIUS;

        egui::Shape::rect_filled(
            rect,
            egui::CornerRadius {
                nw: TAB_CORNER_RADIUS,
                ne: TAB_CORNER_RADIUS,
                sw: 0,
                se: 0,
            },
            bg_color,
        )
    }
}
