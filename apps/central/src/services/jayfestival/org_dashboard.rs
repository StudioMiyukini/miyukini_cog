//! ORG-E04 — Dashboard organisateur (stats réelles, éditions récentes, actions rapides).

use dioxus::prelude::*;
use jayfestival::data::Edition;
use crate::state::use_app_state;
use super::components::{ActionButton, EventCard, StatCard, format_date_range};
use super::{JayFestivalState, OrgSection};

#[component]
pub fn OrgDashboard(
    editions_count: usize,
    exposants_count: usize,
    pending_count: usize,
    editions: Vec<Edition>,
    state: Signal<JayFestivalState>,
) -> Element {
    let c = use_app_state().read().current_theme.palette();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px;",

            // En-tête
            div {
                style: "display: flex; justify-content: space-between; align-items: center;",

                h2 {
                    style: "font-size: 24px; color: {c.text_white};",
                    "Tableau de bord organisateur"
                }

                ActionButton {
                    label: "Nouvelle edition".to_string(),
                    icon: "➕".to_string(),
                    accent: true,
                    onclick: move |_| {
                        state.write().org_section = OrgSection::Editions;
                    },
                }
            }

            // Stats
            div {
                style: "display: grid; grid-template-columns: repeat(3, 1fr); gap: 16px;",

                StatCard {
                    label: "Editions".to_string(),
                    value: editions_count.to_string(),
                    icon: "📅".to_string(),
                    color: c.accent_blue.to_string(),
                }
                StatCard {
                    label: "Exposants".to_string(),
                    value: exposants_count.to_string(),
                    icon: "🏪".to_string(),
                    color: c.accent_green.to_string(),
                }
                StatCard {
                    label: "Candidatures en attente".to_string(),
                    value: pending_count.to_string(),
                    icon: "📝".to_string(),
                    color: c.accent_orange.to_string(),
                }
            }

            // Éditions récentes
            h3 {
                style: "font-size: 16px; color: {c.text_white}; margin-top: 8px;",
                "Editions recentes"
            }

            if editions.is_empty() {
                div {
                    style: "display: flex; flex-direction: column; align-items: center; justify-content: center; height: 200px; color: {c.text_muted}; background: {c.bg_secondary}; border-radius: 8px;",

                    span {
                        style: "font-size: 48px; margin-bottom: 12px; opacity: 0.3;",
                        "📅"
                    }
                    p {
                        style: "font-size: 14px;",
                        "Aucune edition — creez votre premiere !"
                    }
                }
            } else {
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",

                    for edition in editions.iter().take(5) {
                        EventCard {
                            name: edition.name.clone().unwrap_or_else(|| "Sans nom".to_string()),
                            date: format_date_range(edition.start_date.as_ref(), edition.end_date.as_ref()),
                            location: edition.location.clone().unwrap_or_else(|| "Lieu non defini".to_string()),
                            status: edition.status.clone().unwrap_or_else(|| "brouillon".to_string()),
                            onclick: {
                                let eid = edition.id.clone();
                                move |_| {
                                    let mut s = state.write();
                                    s.selected_edition_id.clone_from(&eid);
                                    s.org_section = OrgSection::EditionHub;
                                }
                            },
                        }
                    }
                }
            }
        }
    }
}
