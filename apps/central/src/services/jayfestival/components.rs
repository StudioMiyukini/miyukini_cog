//! Composants partagés JayFestival — StatCard, Badge, TabButton, EmptyState, PlaceholderSection.
//!
//! @id: jf_components @do: export_shared_ui_components
//! @role: ui @layer: service
//! @human: Composants UI reutilisables JayFestival: StatCard, Badge, TabButton, EmptyState.

use dioxus::prelude::*;
use miyuki_ui_dioxus::context::use_palette;
use crate::state::use_app_state;

/// Carte statistique avec bordure colorée.
#[component]
pub fn StatCard(label: String, value: String, icon: String, color: String) -> Element {
    let p = use_palette();
    rsx! {
        div {
            style: "background: {p.bg_secondary}; border-radius: 8px; padding: 16px; border-left: 3px solid {color};",

            div {
                style: "display: flex; justify-content: space-between; align-items: flex-start;",

                div {
                    p {
                        style: "font-size: 12px; color: {p.text_secondary}; margin-bottom: 4px;",
                        "{label}"
                    }
                    p {
                        style: "font-size: 24px; font-weight: 600; color: {p.text_high};",
                        "{value}"
                    }
                }
                span {
                    style: "font-size: 20px; opacity: 0.6;",
                    "{icon}"
                }
            }
        }
    }
}

/// Bouton onglet réutilisable.
#[component]
pub fn TabButton(label: String, is_active: bool, onclick: EventHandler<MouseEvent>) -> Element {
    let p = use_palette();
    let bg = if is_active { p.accent_primary } else { p.bg_overlay };
    let color = if is_active { "white" } else { p.text_secondary };

    rsx! {
        button {
            style: "padding: 8px 16px; background: {bg}; color: {color}; border: none; border-radius: 4px 4px 0 0; cursor: pointer; font-size: 13px; font-weight: 500; transition: all 0.2s;",
            onclick: move |evt| onclick.call(evt),
            "{label}"
        }
    }
}

/// Badge coloré (statut, compteur).
#[component]
pub fn Badge(text: String, color: String) -> Element {
    rsx! {
        span {
            style: "padding: 4px 10px; background: {color}20; color: {color}; border-radius: 4px; font-size: 11px; font-weight: 500;",
            "{text}"
        }
    }
}

/// Bouton d'action.
#[component]
pub fn ActionButton(
    label: String,
    icon: String,
    #[props(default)] accent: bool,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let p = use_palette();
    let bg = if accent { p.accent_primary.to_string() } else { p.bg_overlay.to_string() };
    let color = if accent { "white".to_string() } else { p.text_primary.to_string() };
    let border = if accent { "none".to_string() } else { format!("1px solid {}", p.border_default) };

    rsx! {
        button {
            style: "display: flex; align-items: center; gap: 8px; padding: 10px 16px; background: {bg}; border: {border}; border-radius: 4px; color: {color}; cursor: pointer; font-size: 13px; transition: all 0.2s;",
            onclick: move |evt| onclick.call(evt),

            span { "{icon}" }
            span { "{label}" }
        }
    }
}

/// Section placeholder (fonctionnalité en cours).
#[component]
pub fn PlaceholderSection(title: &'static str, icon: &'static str) -> Element {
    let p = use_palette();
    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100%; color: {p.text_muted};",

            span {
                style: "font-size: 64px; margin-bottom: 16px; opacity: 0.3;",
                "{icon}"
            }
            h2 {
                style: "font-size: 20px; color: {p.text_secondary};",
                "{title}"
            }
            p {
                style: "font-size: 14px; margin-top: 8px;",
                "Cette section sera bientot disponible"
            }
        }
    }
}

/// État vide (aucune donnée).
#[component]
pub fn EmptyState(title: String, message: String, icon: String) -> Element {
    let p = use_palette();
    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; justify-content: center; height: 300px; color: {p.text_muted};",

            span {
                style: "font-size: 64px; margin-bottom: 16px; opacity: 0.3;",
                "{icon}"
            }
            h2 {
                style: "font-size: 20px; color: {p.text_secondary};",
                "{title}"
            }
            p {
                style: "font-size: 14px; margin-top: 8px;",
                "{message}"
            }
        }
    }
}

/// Carte événement cliquable.
#[component]
pub fn EventCard(
    name: String,
    date: String,
    location: String,
    status: String,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let p = use_palette();
    let status_color = match status.as_str() {
        "en_preparation" | "En preparation" => p.warning,
        "en_cours" | "En cours" => p.success,
        "termine" | "Termine" => p.text_muted,
        "publie" | "Publie" => p.accent_primary,
        _ => p.text_secondary,
    };

    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 16px; background: {p.bg_secondary}; border-radius: 8px; padding: 16px; cursor: pointer; transition: background 0.2s;",
            onclick: move |evt| onclick.call(evt),

            // Icône
            div {
                style: "width: 60px; height: 60px; background: linear-gradient(135deg, {p.bg_overlay} 0%, {p.bg_secondary} 100%); border-radius: 8px; display: flex; align-items: center; justify-content: center; font-size: 24px;",
                "🎪"
            }

            // Infos
            div {
                style: "flex: 1;",

                h3 {
                    style: "font-size: 16px; color: {p.text_high}; margin-bottom: 4px;",
                    "{name}"
                }
                div {
                    style: "display: flex; gap: 16px; font-size: 12px; color: {p.text_secondary};",

                    span { "📆 {date}" }
                    span { "📍 {location}" }
                }
            }

            // Badge statut
            Badge {
                text: status.clone(),
                color: status_color.to_string(),
            }

            // Action
            button {
                style: "padding: 8px 16px; background: {p.bg_overlay}; border: 1px solid {p.border_default}; border-radius: 4px; color: {p.text_primary}; cursor: pointer; font-size: 12px;",
                "Gerer →"
            }
        }
    }
}

/// Formate une plage de dates pour affichage.
pub fn format_date_range(start: Option<&String>, end: Option<&String>) -> String {
    match (start, end) {
        (Some(s), Some(e)) => format!("{s} — {e}"),
        (Some(s), None) => s.clone(),
        (None, Some(e)) => e.clone(),
        (None, None) => "Dates non definies".to_string(),
    }
}
