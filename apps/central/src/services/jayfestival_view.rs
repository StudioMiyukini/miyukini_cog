//! Vue du service JayFestival — donnees reelles via JayFestivalDb.

use dioxus::prelude::*;
use crate::data::use_service_connections;
use crate::state::use_app_state;

#[component]
pub fn JayFestivalView() -> Element {
    let state = use_app_state();
    let c = state.read().current_theme.palette();
    let conns = use_service_connections();

    // Charger les editions depuis la DB
    let editions = {
        let db = &conns.read().jayfestival;
        db.editions_list().unwrap_or_default()
    };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px;",

            // Header
            div {
                style: "display: flex; justify-content: space-between; align-items: center;",

                h2 {
                    style: "font-size: 24px; color: {c.text_white};",
                    "📅 JayFestival"
                }

                button {
                    style: "padding: 10px 20px; background: {c.accent_blue}; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 13px;",
                    "➕ Nouvelle edition"
                }
            }

            // Onglets internes
            div {
                style: "display: flex; gap: 4px; border-bottom: 1px solid {c.border}; padding-bottom: 8px;",

                TabButton { label: "Mes evenements", is_active: true }
                TabButton { label: "Candidatures", is_active: false }
                TabButton { label: "Participants", is_active: false }
                TabButton { label: "Statistiques", is_active: false }
            }

            // Liste des editions (donnees reelles ou etat vide)
            if editions.is_empty() {
                div {
                    style: "display: flex; flex-direction: column; align-items: center; justify-content: center; height: 300px; color: {c.text_muted};",

                    span {
                        style: "font-size: 64px; margin-bottom: 16px; opacity: 0.3;",
                        "📅"
                    }
                    h3 {
                        style: "font-size: 20px; color: {c.text_secondary};",
                        "Aucune edition"
                    }
                    p {
                        style: "font-size: 14px; margin-top: 8px;",
                        "Creez votre premiere edition pour commencer"
                    }
                }
            } else {
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",

                    for edition in editions.iter() {
                        EventCard {
                            name: edition.name.clone().unwrap_or_else(|| "Sans nom".to_string()),
                            date: format_date_range(edition.start_date.as_ref(), edition.end_date.as_ref()),
                            location: edition.location.clone().unwrap_or_else(|| "Lieu non defini".to_string()),
                            status: edition.status.clone().unwrap_or_else(|| "brouillon".to_string()),
                        }
                    }
                }
            }
        }
    }
}

/// Formate une plage de dates pour affichage.
fn format_date_range(start: Option<&String>, end: Option<&String>) -> String {
    match (start, end) {
        (Some(s), Some(e)) => format!("{s} — {e}"),
        (Some(s), None) => s.clone(),
        (None, Some(e)) => e.clone(),
        (None, None) => "Dates non definies".to_string(),
    }
}

#[component]
fn TabButton(label: &'static str, is_active: bool) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let bg = if is_active { c.bg_hover } else { "transparent" };
    let color = if is_active { c.text_white } else { c.text_secondary };

    rsx! {
        button {
            style: "padding: 8px 16px; background: {bg}; color: {color}; border: none; border-radius: 4px 4px 0 0; cursor: pointer; font-size: 13px;",
            "{label}"
        }
    }
}

#[component]
fn EventCard(
    name: String,
    date: String,
    location: String,
    status: String,
) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let status_color = match status.as_str() {
        "en_preparation" | "En preparation" => c.accent_orange,
        "en_cours" | "En cours" => c.accent_green,
        "termine" | "Termine" => c.text_muted,
        _ => c.text_secondary,
    };

    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 16px; background: {c.bg_secondary}; border-radius: 8px; padding: 16px; cursor: pointer; transition: background 0.2s;",

            // Icone
            div {
                style: "width: 60px; height: 60px; background: linear-gradient(135deg, {c.bg_hover} 0%, {c.bg_secondary} 100%); border-radius: 8px; display: flex; align-items: center; justify-content: center; font-size: 24px;",
                "🎪"
            }

            // Infos
            div {
                style: "flex: 1;",

                h3 {
                    style: "font-size: 16px; color: {c.text_white}; margin-bottom: 4px;",
                    "{name}"
                }
                div {
                    style: "display: flex; gap: 16px; font-size: 12px; color: {c.text_secondary};",

                    span { "📆 {date}" }
                    span { "📍 {location}" }
                }
            }

            // Status badge
            span {
                style: "padding: 6px 12px; background: {status_color}20; color: {status_color}; border-radius: 4px; font-size: 12px; font-weight: 500;",
                "{status}"
            }

            // Action
            button {
                style: "padding: 8px 16px; background: {c.bg_hover}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_primary}; cursor: pointer; font-size: 12px;",
                "Gerer →"
            }
        }
    }
}
