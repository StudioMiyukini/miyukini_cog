//! Thèmes visuels Miyukini. Le thème actuel "Gaming" reprend la DA type Steam.

/// Thème visuel sélectionnable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Theme {
    /// Style sombre type Steam.
    #[default]
    Gaming,
}

impl Theme {
    /// Libellé affiché dans l'UI.
    pub fn label(self) -> &'static str {
        match self {
            Theme::Gaming => "Gaming (Steam)",
        }
    }

    /// Toutes les valeurs pour itération (sélecteur).
    pub fn all() -> &'static [Theme] {
        &[Theme::Gaming]
    }

    /// Palette de couleurs du thème.
    pub fn palette(self) -> ThemePalette {
        match self {
            Theme::Gaming => ThemePalette::gaming(),
        }
    }
}

/// Palette de couleurs d'un thème (tous les éléments UI doivent l'utiliser).
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ThemePalette {
    pub bg_main: &'static str,
    pub bg_header: &'static str,
    pub bg_card: &'static str,
    pub bg_hover: &'static str,
    pub bg_active: &'static str,
    pub bg_secondary: &'static str,
    pub text_primary: &'static str,
    pub text_secondary: &'static str,
    pub text_muted: &'static str,
    pub text_link: &'static str,
    pub text_white: &'static str,
    pub accent_blue: &'static str,
    pub accent_blue_hover: &'static str,
    pub accent_green: &'static str,
    pub accent_orange: &'static str,
    pub accent_red: &'static str,
    pub border: &'static str,
    pub border_hover: &'static str,
}

impl ThemePalette {
    fn gaming() -> Self {
        Self {
            bg_main: "#171a21",
            bg_header: "#1b2838",
            bg_card: "#1e2329",
            bg_hover: "#2a3f5f",
            bg_active: "#1a9fff",
            bg_secondary: "#232f3e",
            text_primary: "#c6d4df",
            text_secondary: "#8f98a0",
            text_muted: "#5c6873",
            text_link: "#66c0f4",
            text_white: "#ffffff",
            accent_blue: "#1a9fff",
            accent_blue_hover: "#66c0f4",
            accent_green: "#5ba32b",
            accent_orange: "#ff6b00",
            accent_red: "#c83737",
            border: "#2a3f5f",
            border_hover: "#66c0f4",
        }
    }
}

/// Espacements (communs à tous les thèmes).
pub mod spacing {
    pub const HEADER_HEIGHT: &str = "40px";
    #[allow(dead_code)]
    pub const NAV_HEIGHT: &str = "36px";
    pub const PADDING: &str = "16px";
    pub const PADDING_SM: &str = "8px";
    pub const PADDING_LG: &str = "24px";
    pub const RADIUS: &str = "4px";
    pub const RADIUS_LG: &str = "8px";
}
