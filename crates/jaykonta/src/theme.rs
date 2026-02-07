//! Theme visuel JayKonta.

use egui::Color32;

/// Statut de maturite d'un module.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FeatureStatus {
    /// Disponible et exploitable.
    Live,
    /// En construction.
    Build,
    /// Hors phase en cours.
    Locked,
}

/// Theme JayKonta (palette inspiree du prototype dashboard).
pub struct JayKontaTheme {
    /// Fond principal.
    pub bg: Color32,
    /// Fond des panneaux.
    pub panel_bg: Color32,
    /// Fond des cartes.
    pub card_bg: Color32,
    /// Couleur des bordures.
    pub border: Color32,
    /// Texte principal.
    pub text: Color32,
    /// Texte secondaire.
    pub muted_text: Color32,
    /// Accent cyan.
    pub accent: Color32,
    /// Accent bleu.
    pub accent_blue: Color32,
    /// Accent positif.
    pub positive: Color32,
    /// Accent avertissement.
    pub warning: Color32,
    /// Accent danger.
    pub danger: Color32,
}

impl Default for JayKontaTheme {
    fn default() -> Self {
        Self {
            bg: Color32::from_rgb(10, 17, 23),
            panel_bg: Color32::from_rgb(19, 36, 50),
            card_bg: Color32::from_rgb(14, 28, 39),
            border: Color32::from_rgb(44, 71, 93),
            text: Color32::from_rgb(234, 244, 252),
            muted_text: Color32::from_rgb(156, 181, 200),
            accent: Color32::from_rgb(93, 230, 215),
            accent_blue: Color32::from_rgb(120, 184, 255),
            positive: Color32::from_rgb(144, 245, 173),
            warning: Color32::from_rgb(255, 200, 120),
            danger: Color32::from_rgb(255, 139, 139),
        }
    }
}

impl JayKontaTheme {
    /// Applique les couleurs globales a `egui`.
    pub fn apply(&self, ctx: &egui::Context) {
        let mut visuals = egui::Visuals::dark();
        visuals.panel_fill = self.bg;
        visuals.window_fill = self.panel_bg;
        visuals.widgets.noninteractive.bg_fill = self.card_bg;
        visuals.widgets.noninteractive.bg_stroke.color = self.border;
        visuals.widgets.inactive.fg_stroke.color = self.text;
        visuals.widgets.inactive.bg_fill = self.card_bg;
        visuals.widgets.hovered.bg_fill = Color32::from_rgb(25, 50, 68);
        visuals.widgets.active.bg_fill = Color32::from_rgb(28, 58, 76);
        visuals.selection.bg_fill = self.accent_blue.linear_multiply(0.25);
        visuals.override_text_color = Some(self.text);
        ctx.set_visuals(visuals);
    }

    /// Retourne la couleur associee a un statut de fonctionnalite.
    pub fn status_color(&self, status: FeatureStatus) -> Color32 {
        match status {
            FeatureStatus::Live => self.positive,
            FeatureStatus::Build => self.warning,
            FeatureStatus::Locked => self.danger,
        }
    }
}
