//! Service Miyukini UI Editor — personnalisation persistante de l'interface (thèmes, couleurs, formes).
//!
//! Modifie l'**affichage réel** de toute l'app en temps réel : header, sidebar, boutons, champs,
//! onglets, etc. Pas de démo ni mock : les réglages s'appliquent partout et sont persistés.

use crate::catalog::ServiceId;
use crate::pixel_theme::{chrome_colors, PixelChromeTheme};
use crate::services::ServiceUi;
use eframe::egui::{self, Color32, CornerRadius, Visuals};

/// État éditable du thème (couleurs, formes, états). S'applique à toute l'app en temps réel.
#[derive(Clone)]
pub struct EditorThemeState {
    /// Mode sombre (base).
    pub dark_mode: bool,
    // --- Fonds (fenêtres, panneaux, widgets) ---
    /// Fond des fenêtres / panneaux principaux.
    pub window_fill: Color32,
    /// Fond des panneaux latéraux.
    pub panel_fill: Color32,
    /// Fond des widgets non interactifs (labels, séparateurs).
    pub widget_noninteractive_bg: Color32,
    /// Fond des widgets inactifs (bouton au repos, champ, etc.).
    pub widget_inactive_bg: Color32,
    /// Fond des widgets au survol (hover).
    pub widget_hovered_bg: Color32,
    /// Fond des widgets actifs / enfoncés (pressed).
    pub widget_active_bg: Color32,
    // --- Couleurs de texte / avant-plan (fg = foreground) par état ---
    /// Texte / icône des widgets non interactifs.
    pub widget_noninteractive_fg: Color32,
    /// Texte / icône des widgets inactifs.
    pub widget_inactive_fg: Color32,
    /// Texte / icône des widgets au survol.
    pub widget_hovered_fg: Color32,
    /// Texte / icône des widgets actifs.
    pub widget_active_fg: Color32,
    /// Couleur de texte globale (override). None = utiliser fg par état.
    pub override_text_color: Option<Color32>,
    /// Couleur de texte faible (labels secondaires). None = dérivée du thème.
    pub weak_text_color: Option<Color32>,
    // --- Formes ---
    /// Rayon des coins des widgets (px).
    pub corner_radius: f32,
    /// Rayon des coins des fenêtres (px).
    pub window_rounding: f32,
    /// Angle en degrés (0..360) — option visuelle (ex. forme tournée).
    pub angle_deg: f32,
    // --- Header, Hub, onglets (optionnel : None = palette Chrome selon dark_mode) ---
    /// Barre exécutable (ligne 1 header : titre + connexion). None = Chrome.
    pub barre_exe_bg: Option<Color32>,
    /// Fond barre d'onglets (ligne 2 header). None = Chrome.
    pub tab_bar_bg: Option<Color32>,
    /// Fond sidebars Hub. None = Chrome.
    pub sidebar_bg: Option<Color32>,
    /// Fond zone centrale (body). None = Chrome.
    pub body_bg: Option<Color32>,
    /// Fond onglet inactif. None = Chrome.
    pub tab_inactive_bg: Option<Color32>,
    /// Fond onglet inactif au survol. None = Chrome.
    pub tab_inactive_hover_bg: Option<Color32>,
    /// Texte onglet actif. None = Chrome.
    pub tab_active_text: Option<Color32>,
    /// Texte onglet inactif. None = Chrome.
    pub tab_inactive_text: Option<Color32>,
    /// Séparateur entre onglets. None = Chrome.
    pub tab_separator: Option<Color32>,
    /// Bouton fermer onglet (icône). None = Chrome.
    pub close_btn_fg: Option<Color32>,
    /// Bouton fermer au survol (icône). None = Chrome.
    pub close_btn_hover_fg: Option<Color32>,
    /// Fond bouton fermer au survol. None = Chrome.
    pub close_btn_hover_bg: Option<Color32>,
}

impl Default for EditorThemeState {
    fn default() -> Self {
        let v = Visuals::dark();
        Self {
            dark_mode: true,
            window_fill: v.window_fill,
            panel_fill: v.panel_fill,
            widget_noninteractive_bg: v.widgets.noninteractive.bg_fill,
            widget_inactive_bg: v.widgets.inactive.bg_fill,
            widget_hovered_bg: v.widgets.hovered.bg_fill,
            widget_active_bg: v.widgets.active.bg_fill,
            widget_noninteractive_fg: v.widgets.noninteractive.fg_stroke.color,
            widget_inactive_fg: v.widgets.inactive.fg_stroke.color,
            widget_hovered_fg: v.widgets.hovered.fg_stroke.color,
            widget_active_fg: v.widgets.active.fg_stroke.color,
            override_text_color: v.override_text_color,
            weak_text_color: None,
            corner_radius: 4.0,
            window_rounding: 6.0,
            angle_deg: 0.0,
            barre_exe_bg: None,
            tab_bar_bg: None,
            sidebar_bg: None,
            body_bg: None,
            tab_inactive_bg: None,
            tab_inactive_hover_bg: None,
            tab_active_text: None,
            tab_inactive_text: None,
            tab_separator: None,
            close_btn_fg: None,
            close_btn_hover_fg: None,
            close_btn_hover_bg: None,
        }
    }
}

impl EditorThemeState {
    /// Synchronise les couleurs et textes stockés avec le mode (sombre/clair) actuel.
    pub fn sync_colors_from_mode(&mut self) {
        let v = if self.dark_mode {
            Visuals::dark()
        } else {
            Visuals::light()
        };
        self.window_fill = v.window_fill;
        self.panel_fill = v.panel_fill;
        self.widget_noninteractive_bg = v.widgets.noninteractive.bg_fill;
        self.widget_inactive_bg = v.widgets.inactive.bg_fill;
        self.widget_hovered_bg = v.widgets.hovered.bg_fill;
        self.widget_active_bg = v.widgets.active.bg_fill;
        self.widget_noninteractive_fg = v.widgets.noninteractive.fg_stroke.color;
        self.widget_inactive_fg = v.widgets.inactive.fg_stroke.color;
        self.widget_hovered_fg = v.widgets.hovered.fg_stroke.color;
        self.widget_active_fg = v.widgets.active.fg_stroke.color;
        self.override_text_color = v.override_text_color;
    }

    /// Applique l'état au contexte egui (visuals + style). Affichage réel de toute l'app.
    pub fn apply(&self, ctx: &egui::Context) {
        use egui::Stroke;
        ctx.set_visuals(if self.dark_mode {
            egui::Visuals::dark()
        } else {
            egui::Visuals::light()
        });
        let mut visuals = if self.dark_mode {
            Visuals::dark()
        } else {
            Visuals::light()
        };
        visuals.window_fill = self.window_fill;
        visuals.panel_fill = self.panel_fill;
        visuals.widgets.noninteractive.bg_fill = self.widget_noninteractive_bg;
        visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, self.widget_noninteractive_fg);
        visuals.widgets.inactive.bg_fill = self.widget_inactive_bg;
        visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, self.widget_inactive_fg);
        visuals.widgets.hovered.bg_fill = self.widget_hovered_bg;
        visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, self.widget_hovered_fg);
        visuals.widgets.active.bg_fill = self.widget_active_bg;
        visuals.widgets.active.fg_stroke = Stroke::new(1.0, self.widget_active_fg);
        visuals.override_text_color = self.override_text_color;
        visuals.weak_text_color = self.weak_text_color;
        let r = self.corner_radius.clamp(0.0, 255.0) as u8;
        visuals.widgets.noninteractive.corner_radius = CornerRadius::same(r);
        visuals.widgets.inactive.corner_radius = CornerRadius::same(r);
        visuals.widgets.hovered.corner_radius = CornerRadius::same(r);
        visuals.widgets.active.corner_radius = CornerRadius::same(r);
        visuals.window_corner_radius = CornerRadius::same(self.window_rounding.clamp(0.0, 255.0) as u8);
        let no_stroke = Stroke::new(0.0, Color32::TRANSPARENT);
        visuals.window_stroke = no_stroke;
        visuals.widgets.noninteractive.bg_stroke = no_stroke;
        visuals.widgets.inactive.bg_stroke = no_stroke;
        visuals.widgets.hovered.bg_stroke = no_stroke;
        visuals.widgets.active.bg_stroke = no_stroke;
        ctx.set_visuals(visuals);

        let mut style = (*ctx.style()).clone();
        style.visuals.widgets.noninteractive.bg_fill = self.widget_noninteractive_bg;
        style.visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, self.widget_noninteractive_fg);
        style.visuals.widgets.inactive.bg_fill = self.widget_inactive_bg;
        style.visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, self.widget_inactive_fg);
        style.visuals.widgets.hovered.bg_fill = self.widget_hovered_bg;
        style.visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, self.widget_hovered_fg);
        style.visuals.widgets.active.bg_fill = self.widget_active_bg;
        style.visuals.widgets.active.fg_stroke = Stroke::new(1.0, self.widget_active_fg);
        style.visuals.override_text_color = self.override_text_color;
        style.visuals.weak_text_color = self.weak_text_color;
        style.visuals.widgets.noninteractive.corner_radius = CornerRadius::same(r);
        style.visuals.widgets.inactive.corner_radius = CornerRadius::same(r);
        style.visuals.widgets.hovered.corner_radius = CornerRadius::same(r);
        style.visuals.widgets.active.corner_radius = CornerRadius::same(r);
        style.visuals.window_fill = self.window_fill;
        // Pas de contours noirs sur fenêtres/widgets (boutons, champs, etc.) — comme Chrome.
        let no_stroke = Stroke::new(0.0, Color32::TRANSPARENT);
        style.visuals.window_stroke = no_stroke;
        style.visuals.widgets.noninteractive.bg_stroke = no_stroke;
        style.visuals.widgets.inactive.bg_stroke = no_stroke;
        style.visuals.widgets.hovered.bg_stroke = no_stroke;
        style.visuals.widgets.active.bg_stroke = no_stroke;
        ctx.set_style(style);
    }

    /// Réinitialise aux valeurs par défaut (dark).
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Sérialise pour persistance (JSON : mode + couleurs r,g,b + rayons + angle + header/hub).
    pub fn to_storage_string(&self) -> String {
        let r = |c: Color32| (c.r(), c.g(), c.b());
        let opt_rgb = |o: Option<Color32>| o.map(r);
        serde_json::json!({
            "v": 3,
            "dark_mode": self.dark_mode,
            "window_fill": r(self.window_fill),
            "panel_fill": r(self.panel_fill),
            "widget_noninteractive_bg": r(self.widget_noninteractive_bg),
            "widget_inactive_bg": r(self.widget_inactive_bg),
            "widget_hovered_bg": r(self.widget_hovered_bg),
            "widget_active_bg": r(self.widget_active_bg),
            "widget_noninteractive_fg": r(self.widget_noninteractive_fg),
            "widget_inactive_fg": r(self.widget_inactive_fg),
            "widget_hovered_fg": r(self.widget_hovered_fg),
            "widget_active_fg": r(self.widget_active_fg),
            "override_text_color": opt_rgb(self.override_text_color),
            "weak_text_color": opt_rgb(self.weak_text_color),
            "corner_radius": self.corner_radius,
            "window_rounding": self.window_rounding,
            "angle_deg": self.angle_deg,
            "barre_exe_bg": opt_rgb(self.barre_exe_bg),
            "tab_bar_bg": opt_rgb(self.tab_bar_bg),
            "sidebar_bg": opt_rgb(self.sidebar_bg),
            "body_bg": opt_rgb(self.body_bg),
            "tab_inactive_bg": opt_rgb(self.tab_inactive_bg),
            "tab_inactive_hover_bg": opt_rgb(self.tab_inactive_hover_bg),
            "tab_active_text": opt_rgb(self.tab_active_text),
            "tab_inactive_text": opt_rgb(self.tab_inactive_text),
            "tab_separator": opt_rgb(self.tab_separator),
            "close_btn_fg": opt_rgb(self.close_btn_fg),
            "close_btn_hover_fg": opt_rgb(self.close_btn_hover_fg),
            "close_btn_hover_bg": opt_rgb(self.close_btn_hover_bg),
        })
        .to_string()
    }

    /// Désérialise depuis la persistance ; retourne None si invalide. v1 = sans noninteractive/fg, v2 = idem, v3 = + header/hub.
    pub fn from_storage_string(s: &str) -> Option<Self> {
        let j: serde_json::Value = serde_json::from_str(s).ok()?;
        let rgb = |v: &serde_json::Value| {
            let arr = v.as_array()?;
            let r = arr.get(0)?.as_u64()? as u8;
            let g = arr.get(1)?.as_u64()? as u8;
            let b = arr.get(2)?.as_u64()? as u8;
            Some(Color32::from_rgb(r, g, b))
        };
        let opt_rgb = |v: Option<&serde_json::Value>| v.and_then(|x| rgb(x));
        let def = Self::default();
        let dark = j.get("dark_mode").and_then(|x| x.as_bool()).unwrap_or(true);
        let base = if dark { Visuals::dark() } else { Visuals::light() };
        Some(Self {
            dark_mode: dark,
            window_fill: opt_rgb(j.get("window_fill")).unwrap_or(base.window_fill),
            panel_fill: opt_rgb(j.get("panel_fill")).unwrap_or(base.panel_fill),
            widget_noninteractive_bg: opt_rgb(j.get("widget_noninteractive_bg")).unwrap_or(base.widgets.noninteractive.bg_fill),
            widget_inactive_bg: opt_rgb(j.get("widget_inactive_bg")).unwrap_or(base.widgets.inactive.bg_fill),
            widget_hovered_bg: opt_rgb(j.get("widget_hovered_bg")).unwrap_or(base.widgets.hovered.bg_fill),
            widget_active_bg: opt_rgb(j.get("widget_active_bg")).unwrap_or(base.widgets.active.bg_fill),
            widget_noninteractive_fg: opt_rgb(j.get("widget_noninteractive_fg")).unwrap_or(base.widgets.noninteractive.fg_stroke.color),
            widget_inactive_fg: opt_rgb(j.get("widget_inactive_fg")).unwrap_or(base.widgets.inactive.fg_stroke.color),
            widget_hovered_fg: opt_rgb(j.get("widget_hovered_fg")).unwrap_or(base.widgets.hovered.fg_stroke.color),
            widget_active_fg: opt_rgb(j.get("widget_active_fg")).unwrap_or(base.widgets.active.fg_stroke.color),
            override_text_color: j.get("override_text_color").and_then(rgb),
            weak_text_color: j.get("weak_text_color").and_then(rgb),
            corner_radius: j.get("corner_radius").and_then(|x| x.as_f64()).map(|x| x as f32).unwrap_or(def.corner_radius),
            window_rounding: j.get("window_rounding").and_then(|x| x.as_f64()).map(|x| x as f32).unwrap_or(def.window_rounding),
            angle_deg: j.get("angle_deg").and_then(|x| x.as_f64()).map(|x| x as f32).unwrap_or(def.angle_deg),
            barre_exe_bg: j.get("barre_exe_bg").and_then(rgb),
            tab_bar_bg: j.get("tab_bar_bg").and_then(rgb),
            sidebar_bg: j.get("sidebar_bg").and_then(rgb),
            body_bg: j.get("body_bg").and_then(rgb),
            tab_inactive_bg: j.get("tab_inactive_bg").and_then(rgb),
            tab_inactive_hover_bg: j.get("tab_inactive_hover_bg").and_then(rgb),
            tab_active_text: j.get("tab_active_text").and_then(rgb),
            tab_inactive_text: j.get("tab_inactive_text").and_then(rgb),
            tab_separator: j.get("tab_separator").and_then(rgb),
            close_btn_fg: j.get("close_btn_fg").and_then(rgb),
            close_btn_hover_fg: j.get("close_btn_hover_fg").and_then(rgb),
            close_btn_hover_bg: j.get("close_btn_hover_bg").and_then(rgb),
        })
    }
}

/// Clé de persistance du thème UI Editor (storage Central).
pub const UI_EDITOR_THEME_STORAGE_KEY: &str = "miyukini_ui_editor_theme";
/// Clé de persistance des thèmes personnalisés (liste JSON).
pub const UI_EDITOR_CUSTOM_THEMES_STORAGE_KEY: &str = "miyukini_ui_editor_custom_themes";

/// Thème nommé (préconstruit ou créé par l'utilisateur).
#[derive(Clone)]
pub struct NamedTheme {
    pub name: String,
    pub state: EditorThemeState,
}

/// Thèmes préconstruits (nom affiché + état). Une dizaine de palettes cohérentes.
fn builtin_themes() -> [(&'static str, EditorThemeState); 12] {
    use crate::pixel_theme::chrome_colors;
    let sombre = EditorThemeState::default();
    let clair_beige = {
        let mut t = EditorThemeState::default();
        t.dark_mode = false;
        t.sync_colors_from_mode();
        t.barre_exe_bg = Some(chrome_colors::BARRE_EXE_LIGHT);
        t.tab_bar_bg = Some(chrome_colors::TAB_BAR_BG_LIGHT);
        t.sidebar_bg = Some(chrome_colors::SIDEBAR_BG_LIGHT);
        t.body_bg = Some(egui::Color32::WHITE);
        t.tab_inactive_bg = Some(chrome_colors::TAB_INACTIVE_BG_LIGHT);
        t.tab_inactive_hover_bg = Some(chrome_colors::TAB_INACTIVE_HOVER_BG_LIGHT);
        t.tab_active_text = Some(chrome_colors::TAB_ACTIVE_TEXT_LIGHT);
        t.tab_inactive_text = Some(chrome_colors::TAB_INACTIVE_TEXT_LIGHT);
        t.tab_separator = Some(chrome_colors::TAB_SEPARATOR_LIGHT);
        t.close_btn_fg = Some(chrome_colors::CLOSE_BUTTON_LIGHT);
        t.close_btn_hover_fg = Some(chrome_colors::CLOSE_BUTTON_HOVER_LIGHT);
        t.close_btn_hover_bg = Some(chrome_colors::CLOSE_BUTTON_BG_HOVER_LIGHT);
        t
    };
    let contraste = {
        let mut t = EditorThemeState::default();
        t.dark_mode = true;
        t.window_fill = Color32::from_rgb(24, 24, 24);
        t.panel_fill = Color32::from_rgb(28, 28, 28);
        t.widget_inactive_bg = Color32::from_rgb(45, 45, 45);
        t.widget_hovered_bg = Color32::from_rgb(70, 70, 70);
        t.widget_active_bg = Color32::from_rgb(100, 100, 100);
        t.widget_noninteractive_fg = Color32::from_rgb(200, 200, 200);
        t.widget_inactive_fg = Color32::WHITE;
        t.widget_hovered_fg = Color32::WHITE;
        t.widget_active_fg = Color32::WHITE;
        t.override_text_color = Some(Color32::WHITE);
        t.corner_radius = 2.0;
        t.window_rounding = 2.0;
        t
    };
    let bleu_nuit = {
        let mut t = EditorThemeState::default();
        t.dark_mode = true;
        t.window_fill = Color32::from_rgb(25, 32, 48);
        t.panel_fill = Color32::from_rgb(30, 40, 58);
        t.widget_noninteractive_bg = Color32::from_rgb(35, 48, 68);
        t.widget_inactive_bg = Color32::from_rgb(40, 55, 80);
        t.widget_hovered_bg = Color32::from_rgb(55, 75, 110);
        t.widget_active_bg = Color32::from_rgb(70, 95, 140);
        t.widget_noninteractive_fg = Color32::from_rgb(180, 200, 230);
        t.widget_inactive_fg = Color32::from_rgb(200, 220, 255);
        t.widget_hovered_fg = Color32::WHITE;
        t.widget_active_fg = Color32::WHITE;
        t.override_text_color = Some(Color32::from_rgb(220, 230, 255));
        t.barre_exe_bg = Some(Color32::from_rgb(30, 42, 62));
        t.tab_bar_bg = Some(Color32::from_rgb(35, 48, 70));
        t.sidebar_bg = Some(Color32::from_rgb(32, 44, 64));
        t.body_bg = Some(Color32::from_rgb(28, 38, 55));
        t.corner_radius = 6.0;
        t.window_rounding = 8.0;
        t
    };
    // Vert forêt — tons verts sombres
    let vert_foret = {
        let mut t = EditorThemeState::default();
        t.dark_mode = true;
        t.window_fill = Color32::from_rgb(22, 38, 28);
        t.panel_fill = Color32::from_rgb(28, 46, 34);
        t.widget_noninteractive_bg = Color32::from_rgb(35, 55, 40);
        t.widget_inactive_bg = Color32::from_rgb(42, 65, 48);
        t.widget_hovered_bg = Color32::from_rgb(55, 85, 62);
        t.widget_active_bg = Color32::from_rgb(70, 105, 78);
        t.widget_noninteractive_fg = Color32::from_rgb(180, 210, 185);
        t.widget_inactive_fg = Color32::from_rgb(200, 225, 205);
        t.widget_hovered_fg = Color32::from_rgb(220, 245, 222);
        t.widget_active_fg = Color32::WHITE;
        t.override_text_color = Some(Color32::from_rgb(220, 240, 225));
        t.barre_exe_bg = Some(Color32::from_rgb(26, 42, 30));
        t.tab_bar_bg = Some(Color32::from_rgb(30, 48, 36));
        t.sidebar_bg = Some(Color32::from_rgb(28, 44, 32));
        t.body_bg = Some(Color32::from_rgb(24, 40, 28));
        t.corner_radius = 6.0;
        t.window_rounding = 8.0;
        t
    };
    // Coucher de soleil — ambre / orange chaud
    let coucher_soleil = {
        let mut t = EditorThemeState::default();
        t.dark_mode = true;
        t.window_fill = Color32::from_rgb(48, 32, 22);
        t.panel_fill = Color32::from_rgb(58, 40, 28);
        t.widget_noninteractive_bg = Color32::from_rgb(68, 48, 35);
        t.widget_inactive_bg = Color32::from_rgb(85, 60, 42);
        t.widget_hovered_bg = Color32::from_rgb(110, 78, 52);
        t.widget_active_bg = Color32::from_rgb(140, 98, 65);
        t.widget_noninteractive_fg = Color32::from_rgb(230, 200, 170);
        t.widget_inactive_fg = Color32::from_rgb(255, 230, 200);
        t.widget_hovered_fg = Color32::from_rgb(255, 245, 220);
        t.widget_active_fg = Color32::from_rgb(255, 250, 240);
        t.override_text_color = Some(Color32::from_rgb(255, 238, 218));
        t.barre_exe_bg = Some(Color32::from_rgb(55, 38, 26));
        t.tab_bar_bg = Some(Color32::from_rgb(62, 44, 30));
        t.sidebar_bg = Some(Color32::from_rgb(52, 36, 26));
        t.body_bg = Some(Color32::from_rgb(45, 32, 22));
        t.corner_radius = 8.0;
        t.window_rounding = 10.0;
        t
    };
    // Lavande — violets doux
    let lavande = {
        let mut t = EditorThemeState::default();
        t.dark_mode = true;
        t.window_fill = Color32::from_rgb(38, 32, 52);
        t.panel_fill = Color32::from_rgb(46, 38, 62);
        t.widget_noninteractive_bg = Color32::from_rgb(55, 46, 75);
        t.widget_inactive_bg = Color32::from_rgb(68, 56, 92);
        t.widget_hovered_bg = Color32::from_rgb(88, 72, 120);
        t.widget_active_bg = Color32::from_rgb(108, 90, 148);
        t.widget_noninteractive_fg = Color32::from_rgb(200, 190, 230);
        t.widget_inactive_fg = Color32::from_rgb(220, 210, 250);
        t.widget_hovered_fg = Color32::from_rgb(238, 228, 255);
        t.widget_active_fg = Color32::WHITE;
        t.override_text_color = Some(Color32::from_rgb(235, 225, 255));
        t.barre_exe_bg = Some(Color32::from_rgb(42, 36, 58));
        t.tab_bar_bg = Some(Color32::from_rgb(48, 40, 65));
        t.sidebar_bg = Some(Color32::from_rgb(44, 36, 60));
        t.body_bg = Some(Color32::from_rgb(40, 34, 55));
        t.corner_radius = 8.0;
        t.window_rounding = 10.0;
        t
    };
    // Océan — bleu-vert / teal
    let ocean = {
        let mut t = EditorThemeState::default();
        t.dark_mode = true;
        t.window_fill = Color32::from_rgb(22, 42, 48);
        t.panel_fill = Color32::from_rgb(28, 52, 58);
        t.widget_noninteractive_bg = Color32::from_rgb(35, 62, 68);
        t.widget_inactive_bg = Color32::from_rgb(42, 75, 82);
        t.widget_hovered_bg = Color32::from_rgb(55, 95, 105);
        t.widget_active_bg = Color32::from_rgb(70, 118, 130);
        t.widget_noninteractive_fg = Color32::from_rgb(180, 215, 225);
        t.widget_inactive_fg = Color32::from_rgb(200, 230, 238);
        t.widget_hovered_fg = Color32::from_rgb(220, 245, 250);
        t.widget_active_fg = Color32::WHITE;
        t.override_text_color = Some(Color32::from_rgb(210, 235, 242));
        t.barre_exe_bg = Some(Color32::from_rgb(26, 48, 54));
        t.tab_bar_bg = Some(Color32::from_rgb(30, 55, 62));
        t.sidebar_bg = Some(Color32::from_rgb(28, 50, 56));
        t.body_bg = Some(Color32::from_rgb(24, 46, 52));
        t.corner_radius = 6.0;
        t.window_rounding = 8.0;
        t
    };
    // Clair minimal — gris clair épuré
    let clair_minimal = {
        let mut t = EditorThemeState::default();
        t.dark_mode = false;
        t.sync_colors_from_mode();
        t.window_fill = Color32::from_rgb(250, 250, 250);
        t.panel_fill = Color32::from_rgb(245, 245, 245);
        t.widget_noninteractive_bg = Color32::from_rgb(248, 248, 248);
        t.widget_inactive_bg = Color32::from_rgb(240, 240, 240);
        t.widget_hovered_bg = Color32::from_rgb(228, 228, 228);
        t.widget_active_bg = Color32::from_rgb(210, 210, 210);
        t.widget_noninteractive_fg = Color32::from_rgb(100, 100, 100);
        t.widget_inactive_fg = Color32::from_rgb(50, 50, 50);
        t.widget_hovered_fg = Color32::from_rgb(30, 30, 30);
        t.widget_active_fg = Color32::from_rgb(20, 20, 20);
        t.override_text_color = Some(Color32::from_rgb(40, 40, 40));
        t.barre_exe_bg = Some(Color32::from_rgb(235, 235, 235));
        t.tab_bar_bg = Some(Color32::from_rgb(242, 242, 242));
        t.sidebar_bg = Some(Color32::from_rgb(238, 238, 238));
        t.body_bg = Some(Color32::WHITE);
        t.tab_inactive_bg = Some(Color32::from_rgb(232, 232, 232));
        t.tab_inactive_hover_bg = Some(Color32::from_rgb(220, 220, 220));
        t.tab_active_text = Some(Color32::from_rgb(25, 25, 25));
        t.tab_inactive_text = Some(Color32::from_rgb(90, 90, 90));
        t.tab_separator = Some(Color32::from_rgb(210, 210, 210));
        t.close_btn_fg = Some(Color32::from_rgb(90, 90, 90));
        t.close_btn_hover_fg = Some(Color32::from_rgb(25, 25, 25));
        t.close_btn_hover_bg = Some(Color32::from_rgb(218, 218, 218));
        t.corner_radius = 4.0;
        t.window_rounding = 6.0;
        t
    };
    // Sépias — vintage / brun
    let sepias = {
        let mut t = EditorThemeState::default();
        t.dark_mode = false;
        t.sync_colors_from_mode();
        t.window_fill = Color32::from_rgb(245, 238, 228);
        t.panel_fill = Color32::from_rgb(240, 232, 220);
        t.widget_noninteractive_bg = Color32::from_rgb(242, 235, 225);
        t.widget_inactive_bg = Color32::from_rgb(228, 218, 205);
        t.widget_hovered_bg = Color32::from_rgb(210, 198, 182);
        t.widget_active_bg = Color32::from_rgb(192, 178, 160);
        t.widget_noninteractive_fg = Color32::from_rgb(100, 85, 70);
        t.widget_inactive_fg = Color32::from_rgb(60, 50, 42);
        t.widget_hovered_fg = Color32::from_rgb(45, 38, 32);
        t.widget_active_fg = Color32::from_rgb(38, 32, 28);
        t.override_text_color = Some(Color32::from_rgb(55, 48, 40));
        t.barre_exe_bg = Some(Color32::from_rgb(218, 205, 188));
        t.tab_bar_bg = Some(Color32::from_rgb(232, 222, 208));
        t.sidebar_bg = Some(Color32::from_rgb(225, 215, 198));
        t.body_bg = Some(Color32::from_rgb(252, 248, 242));
        t.tab_inactive_bg = Some(Color32::from_rgb(220, 210, 195));
        t.tab_inactive_hover_bg = Some(Color32::from_rgb(205, 192, 175));
        t.tab_active_text = Some(Color32::from_rgb(45, 38, 32));
        t.tab_inactive_text = Some(Color32::from_rgb(95, 82, 68));
        t.tab_separator = Some(Color32::from_rgb(195, 182, 168));
        t.close_btn_fg = Some(Color32::from_rgb(95, 82, 68));
        t.close_btn_hover_fg = Some(Color32::from_rgb(45, 38, 32));
        t.close_btn_hover_bg = Some(Color32::from_rgb(210, 198, 182));
        t.corner_radius = 6.0;
        t.window_rounding = 8.0;
        t
    };
    // Arctique — bleu glacé clair
    let arctique = {
        let mut t = EditorThemeState::default();
        t.dark_mode = false;
        t.sync_colors_from_mode();
        t.window_fill = Color32::from_rgb(238, 245, 252);
        t.panel_fill = Color32::from_rgb(230, 240, 250);
        t.widget_noninteractive_bg = Color32::from_rgb(235, 242, 250);
        t.widget_inactive_bg = Color32::from_rgb(218, 232, 248);
        t.widget_hovered_bg = Color32::from_rgb(200, 220, 242);
        t.widget_active_bg = Color32::from_rgb(178, 208, 238);
        t.widget_noninteractive_fg = Color32::from_rgb(70, 95, 130);
        t.widget_inactive_fg = Color32::from_rgb(45, 75, 115);
        t.widget_hovered_fg = Color32::from_rgb(30, 60, 100);
        t.widget_active_fg = Color32::from_rgb(20, 50, 90);
        t.override_text_color = Some(Color32::from_rgb(35, 65, 105));
        t.barre_exe_bg = Some(Color32::from_rgb(210, 228, 245));
        t.tab_bar_bg = Some(Color32::from_rgb(222, 235, 248));
        t.sidebar_bg = Some(Color32::from_rgb(215, 230, 246));
        t.body_bg = Some(Color32::from_rgb(248, 252, 255));
        t.tab_inactive_bg = Some(Color32::from_rgb(208, 224, 240));
        t.tab_inactive_hover_bg = Some(Color32::from_rgb(195, 215, 235));
        t.tab_active_text = Some(Color32::from_rgb(25, 55, 95));
        t.tab_inactive_text = Some(Color32::from_rgb(85, 110, 150));
        t.tab_separator = Some(Color32::from_rgb(190, 210, 230));
        t.close_btn_fg = Some(Color32::from_rgb(85, 110, 150));
        t.close_btn_hover_fg = Some(Color32::from_rgb(25, 55, 95));
        t.close_btn_hover_bg = Some(Color32::from_rgb(200, 220, 242));
        t.corner_radius = 6.0;
        t.window_rounding = 8.0;
        t
    };
    // Rose nuit — rose / mauve sombre
    let rose_nuit = {
        let mut t = EditorThemeState::default();
        t.dark_mode = true;
        t.window_fill = Color32::from_rgb(48, 35, 45);
        t.panel_fill = Color32::from_rgb(58, 42, 55);
        t.widget_noninteractive_bg = Color32::from_rgb(68, 50, 64);
        t.widget_inactive_bg = Color32::from_rgb(82, 60, 78);
        t.widget_hovered_bg = Color32::from_rgb(105, 78, 98);
        t.widget_active_bg = Color32::from_rgb(128, 95, 120);
        t.widget_noninteractive_fg = Color32::from_rgb(230, 200, 220);
        t.widget_inactive_fg = Color32::from_rgb(245, 220, 238);
        t.widget_hovered_fg = Color32::from_rgb(255, 238, 250);
        t.widget_active_fg = Color32::WHITE;
        t.override_text_color = Some(Color32::from_rgb(250, 232, 245));
        t.barre_exe_bg = Some(Color32::from_rgb(52, 38, 50));
        t.tab_bar_bg = Some(Color32::from_rgb(58, 44, 55));
        t.sidebar_bg = Some(Color32::from_rgb(55, 40, 52));
        t.body_bg = Some(Color32::from_rgb(45, 32, 42));
        t.corner_radius = 8.0;
        t.window_rounding = 10.0;
        t
    };
    [
        ("Sombre (défaut)", sombre),
        ("Clair beige (Central)", clair_beige),
        ("Contraste élevé", contraste),
        ("Bleu nuit", bleu_nuit),
        ("Vert forêt", vert_foret),
        ("Coucher de soleil", coucher_soleil),
        ("Lavande", lavande),
        ("Océan", ocean),
        ("Clair minimal", clair_minimal),
        ("Sépias", sepias),
        ("Arctique", arctique),
        ("Rose nuit", rose_nuit),
    ]
}

/// Catégorie de modification affichée dans le body (options de configuration).
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum UiEditorCategory {
    Themes,
    #[default]
    Mode,
    CouleursFonds,
    CouleursBoutons,
    CouleursTexte,
    Formes,
    HeaderHubOnglets,
    Apercu,
    LiensEgui,
}

impl UiEditorCategory {
    fn label(self) -> &'static str {
        match self {
            UiEditorCategory::Themes => "Thèmes",
            UiEditorCategory::Mode => "Mode",
            UiEditorCategory::CouleursFonds => "Couleurs — Fonds",
            UiEditorCategory::CouleursBoutons => "Couleurs — Boutons",
            UiEditorCategory::CouleursTexte => "Couleurs — Texte",
            UiEditorCategory::Formes => "Formes",
            UiEditorCategory::HeaderHubOnglets => "Header, Hub, onglets",
            UiEditorCategory::Apercu => "Aperçu",
            UiEditorCategory::LiensEgui => "Démo egui",
        }
    }
}

/// Service Miyukini UI Editor — personnalisation UI persistante (thèmes, couleurs, formes).
pub struct EguiEditorService {
    /// Catégorie sélectionnée dans la sidebar (options affichées dans le body).
    pub selected_category: UiEditorCategory,
    /// État du thème éditable.
    pub theme: EditorThemeState,
    /// True si le thème a été modifié (pour sauvegarde en storage).
    pub theme_dirty: bool,
    /// Thèmes créés par l'utilisateur (persistés).
    pub custom_themes: Vec<NamedTheme>,
    /// True si custom_themes a été modifié (pour sauvegarde en storage).
    pub custom_themes_dirty: bool,
    /// Nom saisi pour enregistrer un nouveau thème.
    pub new_theme_name: String,
    /// Texte démo pour le champ de saisie.
    pub demo_text: String,
    /// Valeur démo pour le slider.
    pub demo_slider: f32,
    /// Index sélectionné pour le combo démo.
    pub demo_combo_selected: usize,
    /// Options du combo démo.
    pub demo_combo_options: Vec<String>,
    /// États des cases à cocher démo.
    pub demo_check_1: bool,
    /// Deuxième case démo (décochée par défaut).
    pub demo_check_2: bool,
    /// État du radio démo (0, 1 ou 2).
    pub demo_radio: usize,
}

fn load_custom_themes(storage: Option<&dyn eframe::Storage>) -> Option<Vec<NamedTheme>> {
    let s = storage?.get_string(UI_EDITOR_CUSTOM_THEMES_STORAGE_KEY)?;
    let arr: Vec<(String, String)> = serde_json::from_str(&s).ok()?;
    let mut out = Vec::with_capacity(arr.len());
    for (name, theme_json) in arr {
        if let Some(state) = EditorThemeState::from_storage_string(&theme_json) {
            out.push(NamedTheme { name, state });
        }
    }
    Some(out)
}

impl EguiEditorService {
    /// Charge depuis le storage (thème persisté + thèmes personnalisés) ; sinon défaut.
    pub fn from_storage(storage: Option<&dyn eframe::Storage>) -> Self {
        let theme = storage
            .and_then(|s| s.get_string(UI_EDITOR_THEME_STORAGE_KEY))
            .as_ref()
            .and_then(|s| EditorThemeState::from_storage_string(s))
            .unwrap_or_default();
        let custom_themes = load_custom_themes(storage).unwrap_or_default();
        Self {
            selected_category: UiEditorCategory::default(),
            theme,
            theme_dirty: false,
            custom_themes,
            custom_themes_dirty: false,
            new_theme_name: String::new(),
            demo_text: String::from("Modifiez le thème à gauche…"),
            demo_slider: 0.5,
            demo_combo_selected: 0,
            demo_combo_options: vec![
                "Option A".to_string(),
                "Option B".to_string(),
                "Option C".to_string(),
            ],
            demo_check_1: true,
            demo_check_2: false,
            demo_radio: 0,
        }
    }

    /// Sauvegarde le thème et les thèmes personnalisés en storage si modifiés ; à appeler par le Hub après show().
    pub fn persist_theme_if_dirty(&mut self, storage: Option<&mut dyn eframe::Storage>) {
        if let Some(storage) = storage {
            if self.theme_dirty {
                let _ = storage.set_string(UI_EDITOR_THEME_STORAGE_KEY, self.theme.to_storage_string());
                self.theme_dirty = false;
            }
            if self.custom_themes_dirty {
                let arr: Vec<(String, String)> = self
                    .custom_themes
                    .iter()
                    .map(|nt| (nt.name.clone(), nt.state.to_storage_string()))
                    .collect();
                if let Ok(json) = serde_json::to_string(&arr) {
                    let _ = storage.set_string(UI_EDITOR_CUSTOM_THEMES_STORAGE_KEY, json);
                }
                self.custom_themes_dirty = false;
            }
        }
    }
}

impl Default for EguiEditorService {
    fn default() -> Self {
        Self {
            selected_category: UiEditorCategory::default(),
            theme: EditorThemeState::default(),
            theme_dirty: false,
            custom_themes: Vec::new(),
            custom_themes_dirty: false,
            new_theme_name: String::new(),
            demo_text: String::from("Modifiez le thème à gauche…"),
            demo_slider: 0.5,
            demo_combo_selected: 0,
            demo_combo_options: vec![
                "Option A".to_string(),
                "Option B".to_string(),
                "Option C".to_string(),
            ],
            demo_check_1: true,
            demo_check_2: false,
            demo_radio: 0,
        }
    }
}

impl EguiEditorService {
    /// Sidebar : uniquement les catégories de modification (sélection) + réinitialiser + liens.
    fn ui_sidebar_categories(&mut self, ui: &mut egui::Ui) {
        ui.heading("Catégories");
        ui.label("Sélectionnez une catégorie ; les options s'affichent dans la zone à droite.");
        ui.add_space(8.0);

        for cat in [
            UiEditorCategory::Themes,
            UiEditorCategory::Mode,
            UiEditorCategory::CouleursFonds,
            UiEditorCategory::CouleursBoutons,
            UiEditorCategory::CouleursTexte,
            UiEditorCategory::Formes,
            UiEditorCategory::HeaderHubOnglets,
            UiEditorCategory::Apercu,
            UiEditorCategory::LiensEgui,
        ] {
            let selected = self.selected_category == cat;
            if ui.selectable_label(selected, cat.label()).clicked() {
                self.selected_category = cat;
            }
        }

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(8.0);
        if ui.button("Réinitialiser le thème").clicked() {
            self.theme.reset();
            self.theme_dirty = true;
        }

        ui.add_space(12.0);
        ui.separator();
        ui.add_space(8.0);
        ui.label("Démo egui complète :");
        ui.add_space(4.0);
        ui.code("cd deps/egui");
        ui.code("cargo run --release -p egui_demo_app");
        ui.add_space(4.0);
        ui.hyperlink_to("egui.rs", "https://www.egui.rs/#demo");
    }

    /// Body : options de configuration de la catégorie sélectionnée.
    fn ui_body_options(&mut self, ui: &mut egui::Ui) {
        match self.selected_category {
            UiEditorCategory::Themes => self.ui_body_themes(ui),
            UiEditorCategory::Mode => self.ui_body_mode(ui),
            UiEditorCategory::CouleursFonds => self.ui_body_couleurs_fonds(ui),
            UiEditorCategory::CouleursBoutons => self.ui_body_couleurs_boutons(ui),
            UiEditorCategory::CouleursTexte => self.ui_body_couleurs_texte(ui),
            UiEditorCategory::Formes => self.ui_body_formes(ui),
            UiEditorCategory::HeaderHubOnglets => self.ui_body_header_hub_onglets(ui),
            UiEditorCategory::Apercu => self.ui_body_apercu(ui),
            UiEditorCategory::LiensEgui => self.ui_body_liens_egui(ui),
        }
    }

    fn ui_body_themes(&mut self, ui: &mut egui::Ui) {
        ui.heading("Thèmes");
        ui.label("Thèmes préconstruits ou enregistrés par vous. Appliquez un thème pour l'utiliser immédiatement.");
        ui.add_space(12.0);

        ui.label(egui::RichText::new("Thèmes préconstruits").strong());
        ui.add_space(6.0);
        for (label, state) in builtin_themes() {
            if ui.button(label).clicked() {
                self.theme = state;
                self.theme_dirty = true;
            }
        }

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(8.0);
        ui.label(egui::RichText::new("Mes thèmes").strong());
        ui.add_space(6.0);
        if self.custom_themes.is_empty() {
            ui.label("Aucun thème enregistré. Enregistrez le thème actuel ci‑dessous.");
        } else {
            let mut to_remove = None;
            for (i, nt) in self.custom_themes.iter().enumerate() {
                ui.horizontal(|ui| {
                    if ui.button(egui::RichText::new("Appliquer").small()).clicked() {
                        self.theme = nt.state.clone();
                        self.theme_dirty = true;
                    }
                    if ui.button(egui::RichText::new("Supprimer").small()).clicked() {
                        to_remove = Some(i);
                    }
                    ui.label(&nt.name);
                });
            }
            if let Some(i) = to_remove {
                self.custom_themes.remove(i);
                self.custom_themes_dirty = true;
            }
        }

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(8.0);
        ui.label(egui::RichText::new("Enregistrer le thème actuel").strong());
        ui.add_space(6.0);
        ui.horizontal(|ui| {
            ui.label("Nom :");
            ui.text_edit_singleline(&mut self.new_theme_name);
            let can_save = !self.new_theme_name.trim().is_empty();
            if ui.add_enabled(can_save, egui::Button::new("Enregistrer")).clicked() {
                let name = self.new_theme_name.trim().to_string();
                self.new_theme_name.clear();
                self.custom_themes.push(NamedTheme {
                    name,
                    state: self.theme.clone(),
                });
                self.custom_themes_dirty = true;
            }
        });
    }

    fn ui_body_mode(&mut self, ui: &mut egui::Ui) {
        ui.heading("Mode");
        ui.label("Les changements s'appliquent à toute l'app en temps réel (affichage réel, pas de démo).");
        ui.add_space(12.0);
        if ui.checkbox(&mut self.theme.dark_mode, "Mode sombre (base)").changed() {
            self.theme.sync_colors_from_mode();
            self.theme_dirty = true;
        }
    }

    fn ui_body_couleurs_fonds(&mut self, ui: &mut egui::Ui) {
        ui.heading("Couleurs — Fonds");
        ui.add_space(8.0);
        let mut any = false;
        for (label, color) in [
            ("Fenêtre / panneaux principaux", &mut self.theme.window_fill),
            ("Panneaux latéraux", &mut self.theme.panel_fill),
            ("Widget non interactif (labels, sép.)", &mut self.theme.widget_noninteractive_bg),
            ("Widget inactif (bouton au repos)", &mut self.theme.widget_inactive_bg),
            ("Widget survol (hover)", &mut self.theme.widget_hovered_bg),
            ("Widget actif / enfoncé (pressed)", &mut self.theme.widget_active_bg),
        ] {
            ui.horizontal(|ui| {
                ui.label(label);
                if egui::color_picker::color_edit_button_srgba(ui, color, egui::color_picker::Alpha::Opaque).changed() {
                    any = true;
                }
            });
        }
        if any { self.theme_dirty = true; }
    }

    fn ui_body_couleurs_boutons(&mut self, ui: &mut egui::Ui) {
        ui.heading("Couleurs — Boutons");
        ui.label("Fond et texte des boutons par état (inactif, survol, enfoncé). S'applique à tous les boutons de l'app.");
        ui.add_space(8.0);
        let mut any = false;
        for (label, color) in [
            ("Bouton inactif (fond)", &mut self.theme.widget_inactive_bg),
            ("Bouton au survol (fond)", &mut self.theme.widget_hovered_bg),
            ("Bouton enfoncé (fond)", &mut self.theme.widget_active_bg),
        ] {
            ui.horizontal(|ui| {
                ui.label(label);
                if egui::color_picker::color_edit_button_srgba(ui, color, egui::color_picker::Alpha::Opaque).changed() {
                    any = true;
                }
            });
        }
        ui.add_space(6.0);
        for (label, color) in [
            ("Bouton inactif (texte)", &mut self.theme.widget_inactive_fg),
            ("Bouton au survol (texte)", &mut self.theme.widget_hovered_fg),
            ("Bouton enfoncé (texte)", &mut self.theme.widget_active_fg),
        ] {
            ui.horizontal(|ui| {
                ui.label(label);
                if egui::color_picker::color_edit_button_srgba(ui, color, egui::color_picker::Alpha::Opaque).changed() {
                    any = true;
                }
            });
        }
        if any { self.theme_dirty = true; }
    }

    fn ui_body_couleurs_texte(&mut self, ui: &mut egui::Ui) {
        ui.heading("Couleurs — Texte / avant-plan (fg)");
        ui.add_space(8.0);
        let mut any = false;
        for (label, color) in [
            ("Texte non interactif", &mut self.theme.widget_noninteractive_fg),
            ("Texte inactif", &mut self.theme.widget_inactive_fg),
            ("Texte survol", &mut self.theme.widget_hovered_fg),
            ("Texte actif", &mut self.theme.widget_active_fg),
        ] {
            ui.horizontal(|ui| {
                ui.label(label);
                if egui::color_picker::color_edit_button_srgba(ui, color, egui::color_picker::Alpha::Opaque).changed() {
                    any = true;
                }
            });
        }
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.label("Texte global (override)");
            let mut has = self.theme.override_text_color.is_some();
            if ui.checkbox(&mut has, "Appliquer").changed() {
                if has {
                    self.theme.override_text_color = Some(self.theme.widget_inactive_fg);
                } else {
                    self.theme.override_text_color = None;
                }
                self.theme_dirty = true;
            }
            if let Some(ref mut c) = self.theme.override_text_color {
                if egui::color_picker::color_edit_button_srgba(ui, c, egui::color_picker::Alpha::Opaque).changed() {
                    any = true;
                }
            }
        });
        ui.horizontal(|ui| {
            ui.label("Texte faible (secondaire)");
            let mut has = self.theme.weak_text_color.is_some();
            if ui.checkbox(&mut has, "Appliquer").changed() {
                if has {
                    self.theme.weak_text_color = Some(self.theme.widget_noninteractive_fg);
                } else {
                    self.theme.weak_text_color = None;
                }
                self.theme_dirty = true;
            }
            if let Some(ref mut c) = self.theme.weak_text_color {
                if egui::color_picker::color_edit_button_srgba(ui, c, egui::color_picker::Alpha::Opaque).changed() {
                    any = true;
                }
            }
        });
        if any { self.theme_dirty = true; }
    }

    fn ui_body_formes(&mut self, ui: &mut egui::Ui) {
        ui.heading("Formes (rayons)");
        ui.add_space(8.0);
        if ui.horizontal(|ui| {
            ui.label("Coins widgets (px) :");
            ui.add(
                egui::DragValue::new(&mut self.theme.corner_radius)
                    .range(0.0..=24.0)
                    .speed(0.5),
            ).changed()
        }).inner {
            self.theme_dirty = true;
        }
        if ui.horizontal(|ui| {
            ui.label("Coins fenêtres (px) :");
            ui.add(
                egui::DragValue::new(&mut self.theme.window_rounding)
                    .range(0.0..=24.0)
                    .speed(0.5),
            ).changed()
        }).inner {
            self.theme_dirty = true;
        }
    }

    fn ui_body_header_hub_onglets(&mut self, ui: &mut egui::Ui) {
        ui.heading("Couleurs — Header, Hub, onglets");
        ui.label("Override des zones Chrome. Décocher « Appliquer » = palette par défaut (mode clair/sombre).");
        ui.add_space(8.0);
        let pt = PixelChromeTheme::new(self.theme.dark_mode);
        let mut any = false;
        let mut opt_color = |label: &str, val: &mut Option<Color32>, default: Color32| {
            ui.horizontal(|ui| {
                ui.label(label);
                let mut has = val.is_some();
                if ui.checkbox(&mut has, "Appliquer").changed() {
                    *val = if has { Some(*val.as_ref().unwrap_or(&default)) } else { None };
                    any = true;
                }
                if let Some(ref mut c) = val {
                    if egui::color_picker::color_edit_button_srgba(ui, c, egui::color_picker::Alpha::Opaque).changed() {
                        any = true;
                    }
                }
            });
        };
        opt_color("Barre exe (ligne 1 header)", &mut self.theme.barre_exe_bg, pt.barre_exe_bg());
        opt_color("Fond barre onglets (ligne 2)", &mut self.theme.tab_bar_bg, pt.tab_bar_bg());
        opt_color("Fond sidebars Hub", &mut self.theme.sidebar_bg, pt.sidebar_bg());
        opt_color("Fond zone centrale (body)", &mut self.theme.body_bg, pt.body_bg());
        opt_color("Onglet inactif (fond)", &mut self.theme.tab_inactive_bg, pt.tab_inactive_bg());
        opt_color("Onglet inactif au survol", &mut self.theme.tab_inactive_hover_bg, pt.tab_inactive_hover_bg());
        opt_color("Texte onglet actif", &mut self.theme.tab_active_text, pt.tab_active_text());
        opt_color("Texte onglet inactif", &mut self.theme.tab_inactive_text, pt.tab_inactive_text());
        opt_color("Séparateur onglets", &mut self.theme.tab_separator, pt.tab_separator());
        opt_color("Bouton fermer (icône)", &mut self.theme.close_btn_fg, if self.theme.dark_mode { chrome_colors::CLOSE_BUTTON_DARK } else { chrome_colors::CLOSE_BUTTON_LIGHT });
        opt_color("Bouton fermer au survol (icône)", &mut self.theme.close_btn_hover_fg, if self.theme.dark_mode { chrome_colors::CLOSE_BUTTON_HOVER_DARK } else { chrome_colors::CLOSE_BUTTON_HOVER_LIGHT });
        opt_color("Fond bouton fermer au survol", &mut self.theme.close_btn_hover_bg, if self.theme.dark_mode { chrome_colors::CLOSE_BUTTON_BG_HOVER_DARK } else { chrome_colors::CLOSE_BUTTON_BG_HOVER_LIGHT });
        if any { self.theme_dirty = true; }
    }

    fn ui_body_apercu(&mut self, ui: &mut egui::Ui) {
        ui.heading("Aperçu");
        ui.label("Les réglages s'appliquent à toute l'app en temps réel : header, sidebar, onglets, boutons, champs, etc.");
        ui.add_space(12.0);
        ui.label("Naviguez (HUB, onglets, services) pour voir les états inactif / survol / actif (pressed) en direct.");
        ui.add_space(16.0);
        ui.separator();
        ui.label("Éléments modifiables (catalogue) :");
        ui.add_space(8.0);
        ui.collapsing("Fonds (bg)", |ui| {
            ui.label("• Fenêtre, panneaux • Non interactif • Inactif • Survol (hover) • Actif (pressed)");
        });
        ui.collapsing("Texte / avant-plan (fg)", |ui| {
            ui.label("• Texte par état (non interactif, inactif, survol, actif) • Override global • Texte faible");
        });
        ui.collapsing("Formes", |ui| {
            ui.label("• Rayon coins widgets (px) • Rayon coins fenêtres (px) • Angle (option visuelle)");
        });
        ui.add_space(12.0);
        ui.separator();
        ui.label("Angle (option visuelle) :");
        if ui.add(egui::Slider::new(&mut self.theme.angle_deg, 0.0..=360.0).step_by(5.0).suffix("°")).changed() {
            self.theme_dirty = true;
        }
        let angle_rad = self.theme.angle_deg.to_radians();
        let size = 40.0_f32;
        let (rect, _) = ui.allocate_exact_size(egui::vec2(size * 2.0, size * 2.0), egui::Sense::hover());
        let center = rect.center();
        let color = self.theme.widget_active_bg;
        let tip = egui::pos2(center.x + angle_rad.cos() * size, center.y - angle_rad.sin() * size);
        ui.painter().circle_filled(center, 3.0, color);
        ui.painter().line_segment([center, tip], egui::Stroke::new(2.0, color));
        ui.painter().circle_filled(tip, 4.0, color);
    }

    fn ui_body_liens_egui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Tous les outils egui");
        ui.label("Pour avoir la sidebar à droite avec toutes les démos (About, Bézier, Code Editor, Widget Gallery, Modals, etc.) :");
        ui.add_space(8.0);
        ui.code("cd deps/egui");
        ui.code("cargo run --release -p egui_demo_app");
        ui.add_space(8.0);
        ui.hyperlink_to("Démo en ligne (egui.rs)", "https://www.egui.rs/#demo");
    }

}

impl ServiceUi for EguiEditorService {
    fn id(&self) -> ServiceId {
        ServiceId::EguiEditor
    }
    fn title(&self) -> &'static str {
        "Miyukini UI Editor"
    }
    fn persist_if_needed(&mut self, storage: Option<&mut dyn eframe::Storage>) {
        self.persist_theme_if_dirty(storage);
    }
    fn ui_editor_theme_mut(&mut self) -> Option<&mut EditorThemeState> {
        Some(&mut self.theme)
    }
    fn show(&mut self, ui: &mut egui::Ui) {
        let ctx = ui.ctx();
        // Le thème est appliqué globalement par l'app ; ici on n'applique que pour l'éditeur si besoin (évite doublon).
        self.theme.apply(ctx);

        const SIDEBAR_WIDTH: f32 = 220.0;
        egui::SidePanel::left("egui_editor_sidebar")
            .resizable(true)
            .default_width(SIDEBAR_WIDTH)
            .min_width(180.0)
            .max_width(320.0)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.ui_sidebar_categories(ui);
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                self.ui_body_options(ui);
            });
        });
    }
}
