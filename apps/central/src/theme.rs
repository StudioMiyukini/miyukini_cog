//! Thèmes visuels Miyukini Central. Le thème actuel "Gaming" reprend la DA type Steam.

/// Thème visuel sélectionnable (Profil > Changer de thème).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Theme {
    /// Style sombre type Steam (actuel).
    #[default]
    Gaming,
}

impl Theme {
    /// Libellé affiché dans l'UI (ex. sélecteur de thème).
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
#[allow(dead_code)] // champs réservés pour futurs thèmes / styles
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

/// Styles dérivés du thème (tous les composants doivent les utiliser avec le thème courant).
pub mod styles {
    use super::{spacing, Theme, ThemePalette};

    fn c(theme: Theme) -> ThemePalette {
        theme.palette()
    }

    pub fn main_container(theme: Theme) -> String {
        let c = c(theme);
        format!(
            "display: flex; flex-direction: column; min-height: 100vh; background: {}; color: {}; font-family: 'Segoe UI', Arial, sans-serif;",
            c.bg_main, c.text_primary
        )
    }

    pub fn header(theme: Theme) -> String {
        let c = c(theme);
        format!(
            "display: flex; align-items: center; justify-content: space-between; height: {}; background: {}; padding: 0 {};",
            spacing::HEADER_HEIGHT, c.bg_header, spacing::PADDING
        )
    }

    pub fn nav_tab(theme: Theme, is_active: bool) -> String {
        let c = c(theme);
        let bg = if is_active { c.bg_main } else { "transparent" };
        let color = if is_active { c.text_white } else { c.text_primary };
        let border_bottom = if is_active {
            format!("2px solid {}", c.accent_blue)
        } else {
            "2px solid transparent".to_string()
        };
        format!(
            "padding: 8px 16px; background: {}; color: {}; border: none; border-bottom: {}; cursor: pointer; font-size: 13px; font-weight: 500; text-transform: uppercase; letter-spacing: 0.5px; transition: all 0.2s;",
            bg, color, border_bottom
        )
    }

    #[allow(dead_code)]
    pub fn nav_secondary(theme: Theme) -> String {
        let c = c(theme);
        format!(
            "display: flex; align-items: center; height: {}; background: {}; padding: 0 {}; gap: 8px; border-bottom: 1px solid {};",
            spacing::NAV_HEIGHT, c.bg_secondary, spacing::PADDING, c.border
        )
    }

    #[allow(dead_code)]
    pub fn nav_item(theme: Theme, is_active: bool) -> String {
        let c = c(theme);
        let color = if is_active { c.text_white } else { c.text_secondary };
        format!(
            "padding: 6px 12px; color: {}; font-size: 12px; cursor: pointer; border-radius: {}; transition: all 0.2s;",
            color, spacing::RADIUS
        )
    }

    pub fn content_area(_theme: Theme) -> String {
        "flex: 1; display: flex; flex-direction: column; overflow: hidden;".to_string()
    }

    pub fn tab_bar(theme: Theme) -> String {
        let c = c(theme);
        format!(
            "display: flex; align-items: center; background: {}; padding: 4px {} 0; gap: 2px; border-bottom: 1px solid {};",
            c.bg_secondary, spacing::PADDING_SM, c.border
        )
    }

    pub fn service_tab(theme: Theme, is_active: bool) -> String {
        let c = c(theme);
        let bg = if is_active { c.bg_card } else { "transparent" };
        let color = if is_active { c.text_white } else { c.text_secondary };
        format!(
            "display: flex; align-items: center; gap: 8px; padding: 8px 16px; background: {}; color: {}; border: none; border-radius: 4px 4px 0 0; cursor: pointer; font-size: 13px; max-width: 200px; transition: all 0.2s;",
            bg, color
        )
    }

    pub fn tab_close_btn(theme: Theme) -> String {
        let c = c(theme);
        format!(
            "width: 16px; height: 16px; display: flex; align-items: center; justify-content: center; border-radius: 50%; background: transparent; color: {}; border: none; cursor: pointer; font-size: 12px; opacity: 0.6; transition: all 0.2s;",
            c.text_secondary
        )
    }

    pub fn content_panel(theme: Theme) -> String {
        let c = c(theme);
        format!(
            "flex: 1; background: {}; padding: {}; overflow-y: auto;",
            c.bg_card, spacing::PADDING_LG
        )
    }

    pub fn service_card(theme: Theme) -> String {
        let c = c(theme);
        format!(
            "display: flex; flex-direction: column; background: {}; border-radius: {}; overflow: hidden; cursor: pointer; transition: transform 0.2s, box-shadow 0.2s; border: 1px solid transparent;",
            c.bg_secondary, spacing::RADIUS_LG
        )
    }

    pub fn service_icon_large(theme: Theme) -> String {
        let c = c(theme);
        format!(
            "width: 100%; height: 140px; display: flex; align-items: center; justify-content: center; background: linear-gradient(135deg, {} 0%, {} 100%); font-size: 48px;",
            c.bg_hover, c.bg_secondary
        )
    }

    pub fn service_card_content(_theme: Theme) -> String {
        format!("padding: {}; display: flex; flex-direction: column; gap: 4px;", spacing::PADDING_SM)
    }

    pub fn service_title(theme: Theme) -> String {
        let c = c(theme);
        format!(
            "color: {}; font-size: 14px; font-weight: 500; white-space: nowrap; overflow: hidden; text-overflow: ellipsis;",
            c.text_white
        )
    }

    pub fn price_badge(theme: Theme, is_free: bool) -> String {
        let c = c(theme);
        let bg = if is_free { c.accent_green } else { c.bg_hover };
        format!(
            "display: inline-block; padding: 4px 8px; background: {}; border-radius: {}; font-size: 12px; font-weight: 500;",
            bg, spacing::RADIUS
        )
    }

    pub fn search_input(theme: Theme) -> String {
        let c = c(theme);
        format!(
            "padding: 6px 12px; background: {}; border: 1px solid {}; border-radius: {}; color: {}; font-size: 13px; width: 250px; outline: none;",
            c.bg_card, c.border, spacing::RADIUS, c.text_primary
        )
    }

    pub fn user_profile(_theme: Theme) -> String {
        format!(
            "display: flex; align-items: center; gap: 8px; padding: 4px 8px; cursor: pointer; border-radius: {};",
            spacing::RADIUS
        )
    }

    pub fn avatar(theme: Theme) -> String {
        let c = c(theme);
        format!(
            "width: 32px; height: 32px; border-radius: {}; background: {}; display: flex; align-items: center; justify-content: center; font-size: 14px; border: 2px solid {};",
            spacing::RADIUS, c.bg_hover, c.accent_blue
        )
    }

    pub fn services_grid() -> String {
        "display: grid; grid-template-columns: repeat(auto-fill, minmax(200px, 1fr)); gap: 16px;".to_string()
    }

    pub fn section_title(theme: Theme) -> String {
        let c = c(theme);
        format!(
            "font-size: 18px; font-weight: 500; color: {}; margin-bottom: 16px;",
            c.text_white
        )
    }

    pub fn type_badge(theme: Theme, service_type: crate::state::ServiceType) -> String {
        let _c = c(theme);
        let color = service_type.color();
        format!(
            "display: inline-flex; align-items: center; gap: 4px; padding: 2px 6px; background: {}20; color: {}; border-radius: {}; font-size: 10px; font-weight: 500;",
            color, color, spacing::RADIUS
        )
    }

    /// Overlay modal (fond assombri).
    pub fn overlay_backdrop(theme: Theme) -> String {
        let _ = theme;
        "position: fixed; inset: 0; z-index: 1000; display: flex; align-items: center; justify-content: center; background: rgba(0,0,0,0.6);".to_string()
    }

    /// Carte / panneau modal (ex. fenêtre profil).
    pub fn modal_card(theme: Theme) -> String {
        let c = c(theme);
        format!(
            "background: {}; border-radius: 8px; padding: 24px; min-width: 360px; max-width: 90vw; border: 1px solid {}; box-shadow: 0 8px 32px rgba(0,0,0,0.4);",
            c.bg_card, c.border
        )
    }

    /// Titre de section dans un modal.
    pub fn modal_title(theme: Theme) -> String {
        let c = c(theme);
        format!("margin: 0 0 20px 0; font-size: 18px; color: {};", c.text_primary)
    }

    /// Corps de texte secondaire dans un modal.
    pub fn modal_body_text(theme: Theme) -> String {
        let c = c(theme);
        format!("display: flex; flex-direction: column; gap: 10px; margin-bottom: 20px; font-size: 14px; color: {};", c.text_secondary)
    }

    /// Label accent dans un modal.
    pub fn modal_label(theme: Theme) -> String {
        let c = c(theme);
        format!("color: {}; margin-right: 8px;", c.text_link)
    }

    /// Texte muted petit (ex. ID).
    pub fn modal_muted_small(theme: Theme) -> String {
        let c = c(theme);
        format!("font-size: 11px; color: {};", c.text_muted)
    }

    /// Bouton secondaire (ex. Réinitialiser COG).
    pub fn btn_secondary(theme: Theme) -> String {
        let c = c(theme);
        format!(
            "padding: 8px 16px; background: {}; color: {}; border: 1px solid {}; border-radius: 4px; cursor: pointer; font-size: 13px;",
            c.bg_hover, c.text_primary, c.border
        )
    }

    /// Bouton primaire (ex. Fermer, Entrer).
    pub fn btn_primary(theme: Theme) -> String {
        let c = c(theme);
        format!(
            "padding: 8px 16px; background: {}; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 13px;",
            c.accent_blue
        )
    }

    /// Conteneur écran plein (Rite d'Entrée, Connexion).
    pub fn fullscreen_container(theme: Theme) -> String {
        let c = c(theme);
        format!(
            "display: flex; flex-direction: column; align-items: center; justify-content: center; min-height: 100vh; background: {}; color: {}; padding: 24px;",
            c.bg_main, c.text_primary
        )
    }

    /// Carte centrée (formulaire Rite / Connexion).
    pub fn form_card(theme: Theme) -> String {
        let c = c(theme);
        format!(
            "background: {}; border-radius: 8px; padding: 32px; border: 1px solid {};",
            c.bg_card, c.border
        )
    }

    /// Titre principal formulaire.
    pub fn form_title(theme: Theme) -> String {
        let c = c(theme);
        format!("font-size: 18px; margin-bottom: 24px; color: {};", c.text_primary)
    }

    /// Sous-texte / hint formulaire.
    #[allow(dead_code)]
    pub fn form_hint(theme: Theme) -> String {
        let c = c(theme);
        format!("font-size: 12px; color: {}; margin-top: 12px; font-style: italic;", c.text_secondary)
    }

    /// Champ input formulaire.
    pub fn form_input(theme: Theme) -> String {
        let c = c(theme);
        format!(
            "width: 100%; padding: 12px; background: {}; border: 1px solid {}; border-radius: 4px; color: {}; font-size: 14px; box-sizing: border-box;",
            c.bg_secondary, c.border, c.text_primary
        )
    }

    /// Bouton principal formulaire (Signer, Entrer).
    pub fn form_btn_primary(theme: Theme) -> String {
        let c = c(theme);
        format!(
            "margin-top: 24px; padding: 10px 24px; background: {}; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 14px;",
            c.accent_blue
        )
    }

    /// Message d'erreur.
    pub fn form_error(theme: Theme) -> String {
        let c = c(theme);
        format!("font-size: 12px; color: {}; margin-bottom: 8px;", c.accent_red)
    }

    /// Lien (ex. Créer un compte).
    #[allow(dead_code)]
    pub fn link(theme: Theme) -> String {
        let c = c(theme);
        format!("color: {}; cursor: pointer;", c.text_link)
    }
}

/// Rétrocompatibilité : accès aux couleurs du thème par défaut (préférer theme.palette() en composant).
pub mod colors {
    use super::Theme;
    pub use super::ThemePalette;
    /// Couleurs du thème Gaming (par défaut). Pour un thème dynamique, utiliser `state.read().current_theme.palette()`.
    #[allow(dead_code)]
    pub fn gaming() -> ThemePalette {
        Theme::Gaming.palette()
    }
}
