//! Thème pixel art style Chrome pour Miyukini Central.
//!
//! Ce module adapte les techniques CSS de chrome-tabs dans egui pour créer
//! une interface pixel art fidèle au style Chrome.

use eframe::egui;

/// Couleurs Chrome exactes (style pixel art).
#[allow(missing_docs)]
pub mod chrome_colors {
    use eframe::egui::Color32;

    /// Fond de la barre d'onglets (gris très clair Chrome).
    pub const TAB_BAR_BG_LIGHT: Color32 = Color32::from_rgb(242, 242, 244);
    pub const TAB_BAR_BG_DARK: Color32 = Color32::from_rgb(45, 45, 50);

    /// Onglet actif - fond blanc pur.
    pub const TAB_ACTIVE_BG_LIGHT: Color32 = Color32::WHITE;
    pub const TAB_ACTIVE_BG_DARK: Color32 = Color32::from_rgb(50, 50, 55);

    /// Onglet actif - texte très foncé.
    pub const TAB_ACTIVE_TEXT_LIGHT: Color32 = Color32::from_rgb(32, 33, 36);
    pub const TAB_ACTIVE_TEXT_DARK: Color32 = Color32::from_rgb(240, 240, 240);

    /// Onglet inactif - fond gris clair.
    pub const TAB_INACTIVE_BG_LIGHT: Color32 = Color32::from_rgb(236, 236, 236);
    pub const TAB_INACTIVE_BG_DARK: Color32 = Color32::from_rgb(60, 60, 65);

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
    /// Hauteur des onglets.
    pub const TAB_HEIGHT_ACTIVE: f32 = 32.0;
    pub const TAB_HEIGHT_INACTIVE: f32 = 28.0;

    /// Largeur des onglets.
    pub const TAB_WIDTH: f32 = 200.0;

    /// Rayon des coins arrondis en haut.
    pub const TAB_CORNER_RADIUS: u8 = 8;

    /// Profondeur de la courbe concave en bas (onglet actif).
    pub const TAB_CURVE_DEPTH: f32 = 2.5;

    /// Offset vertical des onglets inactifs.
    pub const TAB_INACTIVE_OFFSET_Y: f32 = 2.0;

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

        // Couleurs des widgets selon le thème
        use egui::Color32;
        if self.dark_mode {
            style.visuals.widgets.inactive.bg_fill = TAB_INACTIVE_BG_DARK;
            style.visuals.widgets.hovered.bg_fill = Color32::from_rgb(70, 70, 75);
            style.visuals.widgets.active.bg_fill = TAB_ACTIVE_BG_DARK;
            style.visuals.override_text_color = Some(TAB_ACTIVE_TEXT_DARK);
        } else {
            style.visuals.widgets.inactive.bg_fill = TAB_INACTIVE_BG_LIGHT;
            style.visuals.widgets.hovered.bg_fill = Color32::from_rgb(220, 220, 220);
            style.visuals.widgets.active.bg_fill = TAB_ACTIVE_BG_LIGHT;
            style.visuals.override_text_color = Some(TAB_ACTIVE_TEXT_LIGHT);
        }

        // Espacement style Chrome
        style.spacing.button_padding = egui::vec2(12.0, 6.0);
        style.spacing.item_spacing = egui::vec2(8.0, 6.0);
        style.spacing.window_margin = egui::Margin::same(8i8);

        ctx.set_style(style);
    }

    /// Retourne la couleur de fond de la barre d'onglets.
    pub fn tab_bar_bg(&self) -> egui::Color32 {
        if self.dark_mode {
            chrome_colors::TAB_BAR_BG_DARK
        } else {
            chrome_colors::TAB_BAR_BG_LIGHT
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
pub mod chrome_shapes {
    use super::chrome_dimensions;
    use eframe::egui;

    /// Crée la forme d'un onglet Chrome actif avec courbe concave en bas.
    pub fn create_active_tab_shape(
        rect: egui::Rect,
        bg_color: egui::Color32,
    ) -> Vec<egui::Shape> {
        use chrome_dimensions::*;

        let mut shapes = Vec::new();

        // Partie supérieure avec coins arrondis
        let base_rect = egui::Rect::from_min_max(
            rect.min,
            egui::pos2(rect.max.x, rect.max.y - 1.0),
        );

        let top_shape = egui::Shape::rect_filled(
            base_rect,
            egui::CornerRadius {
                nw: TAB_CORNER_RADIUS,
                ne: TAB_CORNER_RADIUS,
                sw: 0,
                se: 0,
            },
            bg_color,
        );
        shapes.push(top_shape);

        // Courbe concave en bas (style Chrome)
        let curve_depth = TAB_CURVE_DEPTH;
        let mut bottom_points = Vec::new();
        bottom_points.push(egui::pos2(rect.min.x, base_rect.max.y));

        // Créer la courbe concave avec plusieurs points
        let num_points = 16;
        for i in 0..=num_points {
            let t = i as f32 / num_points as f32;
            let x = rect.min.x + (rect.max.x - rect.min.x) * t;
            // Courbe quadratique concave (remonte au centre)
            let y = rect.max.y - curve_depth * (1.0 - 4.0 * t * (1.0 - t));
            bottom_points.push(egui::pos2(x, y));
        }

        bottom_points.push(egui::pos2(rect.max.x, base_rect.max.y));

        let bottom_shape = egui::Shape::convex_polygon(bottom_points, bg_color, egui::Stroke::NONE);
        shapes.push(bottom_shape);

        shapes
    }

    /// Crée la forme d'un onglet Chrome inactif (coins arrondis en haut, bord plat en bas).
    pub fn create_inactive_tab_shape(
        rect: egui::Rect,
        bg_color: egui::Color32,
    ) -> egui::Shape {
        use chrome_dimensions::*;

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
