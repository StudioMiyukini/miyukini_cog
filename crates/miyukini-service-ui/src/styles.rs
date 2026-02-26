//! Styles dérivés du thème — fonctions de style réutilisables.
//!
//! Chaque fonction prend un `Theme` et retourne un `String` CSS inline.

use crate::theme::{spacing, Theme, ThemePalette};

fn c(theme: Theme) -> ThemePalette {
    theme.palette()
}

// ── Layout ────────────────────────────────────────────────────────────

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

pub fn content_area(_theme: Theme) -> String {
    "flex: 1; display: flex; flex-direction: column; overflow: hidden;".to_string()
}

pub fn content_panel(theme: Theme) -> String {
    let c = c(theme);
    format!(
        "flex: 1; display: flex; flex-direction: column; min-height: 0; background: {}; padding: {}; overflow: hidden;",
        c.bg_card, spacing::PADDING_LG
    )
}

// ── Navigation ────────────────────────────────────────────────────────

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
        "padding: 8px 16px; background: {bg}; color: {color}; border: none; border-bottom: {border_bottom}; cursor: pointer; font-size: 13px; font-weight: 500; text-transform: uppercase; letter-spacing: 0.5px; transition: all 0.2s;"
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
        "display: flex; align-items: center; gap: 8px; padding: 8px 16px; background: {bg}; color: {color}; border: none; border-radius: 4px 4px 0 0; cursor: pointer; font-size: 13px; max-width: 200px; transition: all 0.2s;"
    )
}

pub fn tab_close_btn(theme: Theme) -> String {
    let c = c(theme);
    format!(
        "width: 16px; height: 16px; display: flex; align-items: center; justify-content: center; border-radius: 50%; background: transparent; color: {}; border: none; cursor: pointer; font-size: 12px; opacity: 0.6; transition: all 0.2s;",
        c.text_secondary
    )
}

// ── Cards & Grid ──────────────────────────────────────────────────────

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

pub fn services_grid() -> String {
    "display: grid; grid-template-columns: repeat(3, 1fr); gap: 16px;".to_string()
}

pub fn section_title(theme: Theme) -> String {
    let c = c(theme);
    format!(
        "font-size: 18px; font-weight: 500; color: {}; margin-bottom: 16px;",
        c.text_white
    )
}

/// Badge de type avec couleur personnalisable.
pub fn type_badge_color(color: &str) -> String {
    format!(
        "display: inline-flex; align-items: center; gap: 4px; padding: 2px 6px; background: {}20; color: {}; border-radius: {}; font-size: 10px; font-weight: 500;",
        color, color, spacing::RADIUS
    )
}

// ── Search ────────────────────────────────────────────────────────────

pub fn search_input(theme: Theme) -> String {
    let c = c(theme);
    format!(
        "padding: 6px 12px; background: {}; border: 1px solid {}; border-radius: {}; color: {}; font-size: 13px; width: 250px; outline: none;",
        c.bg_card, c.border, spacing::RADIUS, c.text_primary
    )
}

// ── User Profile ──────────────────────────────────────────────────────

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

// ── Modal / Overlay ───────────────────────────────────────────────────

pub fn overlay_backdrop(_theme: Theme) -> String {
    "position: fixed; inset: 0; z-index: 1000; display: flex; align-items: center; justify-content: center; background: rgba(0,0,0,0.6);".to_string()
}

pub fn modal_card(theme: Theme) -> String {
    let c = c(theme);
    format!(
        "background: {}; border-radius: 8px; padding: 24px; min-width: 360px; max-width: 90vw; border: 1px solid {}; box-shadow: 0 8px 32px rgba(0,0,0,0.4);",
        c.bg_card, c.border
    )
}

pub fn modal_title(theme: Theme) -> String {
    let c = c(theme);
    format!("margin: 0 0 20px 0; font-size: 18px; color: {};", c.text_primary)
}

pub fn modal_body_text(theme: Theme) -> String {
    let c = c(theme);
    format!("display: flex; flex-direction: column; gap: 10px; margin-bottom: 20px; font-size: 14px; color: {};", c.text_secondary)
}

pub fn modal_label(theme: Theme) -> String {
    let c = c(theme);
    format!("color: {}; margin-right: 8px;", c.text_link)
}

pub fn modal_muted_small(theme: Theme) -> String {
    let c = c(theme);
    format!("font-size: 11px; color: {};", c.text_muted)
}

// ── Buttons ───────────────────────────────────────────────────────────

pub fn btn_secondary(theme: Theme) -> String {
    let c = c(theme);
    format!(
        "padding: 8px 16px; background: {}; color: {}; border: 1px solid {}; border-radius: 4px; cursor: pointer; font-size: 13px;",
        c.bg_hover, c.text_primary, c.border
    )
}

pub fn btn_primary(theme: Theme) -> String {
    let c = c(theme);
    format!(
        "padding: 8px 16px; background: {}; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 13px;",
        c.accent_blue
    )
}

// ── Fullscreen / Forms ────────────────────────────────────────────────

pub fn fullscreen_container(theme: Theme) -> String {
    let c = c(theme);
    format!(
        "display: flex; flex-direction: column; align-items: center; justify-content: center; min-height: 100vh; background: {}; color: {}; padding: 24px;",
        c.bg_main, c.text_primary
    )
}

pub fn form_card(theme: Theme) -> String {
    let c = c(theme);
    format!(
        "background: {}; border-radius: 8px; padding: 32px; border: 1px solid {};",
        c.bg_card, c.border
    )
}

pub fn form_title(theme: Theme) -> String {
    let c = c(theme);
    format!("font-size: 18px; margin-bottom: 24px; color: {};", c.text_primary)
}

#[allow(dead_code)]
pub fn form_hint(theme: Theme) -> String {
    let c = c(theme);
    format!("font-size: 12px; color: {}; margin-top: 12px; font-style: italic;", c.text_secondary)
}

pub fn form_input(theme: Theme) -> String {
    let c = c(theme);
    format!(
        "width: 100%; padding: 12px; background: {}; border: 1px solid {}; border-radius: 4px; color: {}; font-size: 14px; box-sizing: border-box;",
        c.bg_secondary, c.border, c.text_primary
    )
}

pub fn form_btn_primary(theme: Theme) -> String {
    let c = c(theme);
    format!(
        "margin-top: 24px; padding: 10px 24px; background: {}; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 14px;",
        c.accent_blue
    )
}

pub fn form_error(theme: Theme) -> String {
    let c = c(theme);
    format!("font-size: 12px; color: {}; margin-bottom: 8px;", c.accent_red)
}

#[allow(dead_code)]
pub fn link(theme: Theme) -> String {
    let c = c(theme);
    format!("color: {}; cursor: pointer;", c.text_link)
}

// ── Sidebar (commun à plusieurs services) ─────────────────────────────

/// Conteneur sidebar (panneau gauche dans un layout sidebar + content).
pub fn sidebar(theme: Theme) -> String {
    let c = c(theme);
    format!(
        "width: 220px; min-width: 220px; background: {}; border-right: 1px solid {}; padding: 16px 0; display: flex; flex-direction: column; gap: 2px; overflow-y: auto;",
        c.bg_secondary, c.border
    )
}

/// Item de navigation dans la sidebar.
pub fn sidebar_item(theme: Theme, is_active: bool) -> String {
    let c = c(theme);
    let bg = if is_active { c.bg_hover } else { "transparent" };
    let color = if is_active { c.text_white } else { c.text_secondary };
    let border_left = if is_active {
        format!("3px solid {}", c.accent_blue)
    } else {
        "3px solid transparent".to_string()
    };
    format!(
        "padding: 10px 16px; background: {bg}; color: {color}; border: none; border-left: {border_left}; cursor: pointer; font-size: 13px; text-align: left; width: 100%; transition: all 0.15s; display: flex; align-items: center; gap: 8px;"
    )
}

/// Titre de section dans la sidebar.
pub fn sidebar_section_title(theme: Theme) -> String {
    let c = c(theme);
    format!(
        "padding: 8px 16px 4px; font-size: 10px; text-transform: uppercase; letter-spacing: 1px; color: {}; font-weight: 600;",
        c.text_muted
    )
}

// ── Service layout (sidebar + content) ────────────────────────────────

/// Layout horizontal sidebar + contenu scrollable.
pub fn service_layout() -> String {
    "display: flex; flex: 1; min-height: 0; overflow: hidden;".to_string()
}

/// Zone de contenu scrollable (à droite de la sidebar).
pub fn service_content() -> String {
    "flex: 1; padding: 24px; overflow-y: auto;".to_string()
}
