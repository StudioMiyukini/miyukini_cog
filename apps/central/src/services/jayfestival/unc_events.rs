//! UNC-E02 / UNC-E03 — Liste des evenements et Fiche evenement (facade publique).
//!
//! Catalogue des evenements publies, filtres, et detail d'un evenement.

use dioxus::prelude::*;
use crate::data::use_service_connections;
use crate::state::use_app_state;
use super::{UncSection, JayFestivalState};
use super::components::{Badge, ActionButton, format_date_range};
use jayfestival::data::Edition;

fn opt_str(s: &Option<String>) -> String {
    s.clone().unwrap_or_default()
}

/// UNC-E02 — Liste des evenements.
#[component]
pub fn UncEventsList(state: Signal<JayFestivalState>) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let conns = use_service_connections();

    let editions = {
        let db = &conns.read().jayfestival;
        db.editions_published().unwrap_or_default()
    };

    let mut filter_location = use_signal(String::new);
    let filter_loc = filter_location.read().clone();

    let filtered: Vec<_> = editions
        .iter()
        .filter(|e| {
            filter_loc.is_empty()
                || e.location
                    .as_ref()
                    .is_some_and(|l| l.to_lowercase().contains(&filter_loc.to_lowercase()))
        })
        .collect();

    let count = filtered.len();

    rsx! {
        div {
            style: "max-width: 1200px; margin: 0 auto;",

            // Header
            div {
                style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 24px;",

                div {
                    button {
                        style: "background: none; border: none; color: {c.text_secondary}; cursor: pointer; font-size: 13px;",
                        onclick: move |_| {
                            state.write().unc_section = UncSection::Landing;
                        },
                        "← Retour"
                    }
                    h1 {
                        style: "font-size: 24px; color: {c.text_white}; margin-top: 8px;",
                        "Evenements"
                    }
                }
            }

            // Layout filtres + résultats
            div {
                style: "display: grid; grid-template-columns: 280px 1fr; gap: 24px;",

                // Panneau filtres
                aside {
                    style: "background: {c.bg_secondary}; border-radius: 8px; padding: 20px;",

                    h3 {
                        style: "font-size: 14px; color: {c.text_white}; margin-bottom: 16px;",
                        "Filtres"
                    }

                    div {
                        style: "margin-bottom: 20px;",
                        label {
                            style: "display: block; font-size: 12px; color: {c.text_secondary}; margin-bottom: 8px;",
                            "Lieu"
                        }
                        input {
                            r#type: "text",
                            style: "width: 100%; padding: 8px; background: {c.bg_main}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_primary}; font-size: 12px;",
                            placeholder: "Ville, region...",
                            value: "{filter_location}",
                            oninput: move |evt| {
                                filter_location.set(evt.value());
                            },
                        }
                    }

                    button {
                        style: "width: 100%; padding: 10px; background: transparent; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_secondary}; cursor: pointer; font-size: 12px;",
                        onclick: move |_| {
                            filter_location.set(String::new());
                        },
                        "Reinitialiser"
                    }
                }

                // Résultats
                div {
                    p {
                        style: "font-size: 13px; color: {c.text_secondary}; margin-bottom: 16px;",
                        "{count} evenements"
                    }

                    if filtered.is_empty() {
                        div {
                            style: "padding: 40px; text-align: center; background: {c.bg_secondary}; border-radius: 8px; color: {c.text_muted};",
                            "Aucun evenement ne correspond a vos criteres"
                        }
                    } else {
                        div {
                            style: "display: flex; flex-direction: column; gap: 12px;",

                            for edition in filtered {
                                {
                                    let id_str = opt_str(&edition.id);
                                    let id_clone = edition.id.clone();
                                    rsx! {
                                        EventListItem {
                                            key: "{id_str}",
                                            edition: edition.clone(),
                                            onclick: move |_| {
                                                state.write().selected_edition_id = id_clone.clone();
                                                state.write().unc_section = UncSection::EventDetail;
                                            },
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// UNC-E03 — Fiche evenement detaillee.
#[component]
pub fn UncEventDetail(state: Signal<JayFestivalState>) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let conns = use_service_connections();

    let edition_id = state.read().selected_edition_id.clone();

    let edition = {
        let db = &conns.read().jayfestival;
        edition_id
            .as_ref()
            .and_then(|id| db.edition_by_id(id).ok().flatten())
    };

    let Some(edition) = edition else {
        return rsx! {
            div {
                style: "text-align: center; padding: 40px; color: {c.text_muted};",
                "Evenement non trouve"
            }
        };
    };

    // Charger les exposants de l'événement
    let exposants = {
        let db = &conns.read().jayfestival;
        edition_id
            .as_ref()
            .map(|id| db.exposants_by_edition(id).unwrap_or_default())
            .unwrap_or_default()
    };

    // Charger les animations
    let animations = {
        let db = &conns.read().jayfestival;
        edition_id
            .as_ref()
            .map(|id| db.animations_by_edition(id).unwrap_or_default())
            .unwrap_or_default()
    };

    let status_str = opt_str(&edition.status);
    let status_color = match status_str.as_str() {
        "publie" | "en_cours" => c.accent_green,
        "brouillon" => c.accent_orange,
        _ => c.text_muted,
    };

    let name = opt_str(&edition.name);
    let location = opt_str(&edition.location);
    let date_range = format_date_range(edition.start_date.as_ref(), edition.end_date.as_ref());
    let exposants_count = exposants.len();

    rsx! {
        div {
            style: "max-width: 900px; margin: 0 auto;",

            // Navigation retour
            button {
                style: "background: none; border: none; color: {c.text_secondary}; cursor: pointer; font-size: 13px; margin-bottom: 16px;",
                onclick: move |_| {
                    state.write().unc_section = UncSection::Events;
                },
                "← Retour aux evenements"
            }

            // Bannière
            div {
                style: "width: 100%; height: 200px; background: linear-gradient(135deg, {c.bg_secondary} 0%, {c.bg_hover} 100%); border-radius: 12px; margin-bottom: 24px; display: flex; align-items: center; justify-content: center;",
                span {
                    style: "font-size: 64px; opacity: 0.5;",
                    "🎪"
                }
            }

            // Titre et infos
            div {
                style: "margin-bottom: 24px;",

                div {
                    style: "display: flex; align-items: center; gap: 12px; margin-bottom: 12px;",

                    h1 {
                        style: "font-size: 28px; color: {c.text_white};",
                        "{name}"
                    }
                    Badge {
                        text: status_str.clone(),
                        color: status_color.to_string(),
                    }
                }

                div {
                    style: "display: flex; gap: 24px; font-size: 14px; color: {c.text_secondary};",

                    span { "📅 {date_range}" }
                    span { "📍 {location}" }
                }
            }

            // Description
            if let Some(theme) = &edition.theme {
                section {
                    style: "margin-bottom: 32px;",

                    h2 {
                        style: "font-size: 18px; color: {c.text_white}; margin-bottom: 12px;",
                        "Description"
                    }
                    p {
                        style: "font-size: 14px; color: {c.text_secondary}; line-height: 1.6;",
                        "{theme}"
                    }
                }
            }

            // Programme
            section {
                style: "margin-bottom: 32px;",

                h2 {
                    style: "font-size: 18px; color: {c.text_white}; margin-bottom: 12px;",
                    "Programme"
                }

                if animations.is_empty() {
                    p {
                        style: "font-size: 14px; color: {c.text_muted};",
                        "Le programme sera bientot disponible"
                    }
                } else {
                    div {
                        style: "background: {c.bg_secondary}; border-radius: 8px; overflow: hidden;",

                        for anim in animations.iter().take(5) {
                            {
                                let id_str = opt_str(&anim.id);
                                let start_time = opt_str(&anim.start_time);
                                let anim_name = opt_str(&anim.name);
                                let room = anim.room.clone();
                                rsx! {
                                    div {
                                        key: "{id_str}",
                                        style: "display: flex; align-items: center; gap: 16px; padding: 12px 16px; border-bottom: 1px solid {c.border};",

                                        span {
                                            style: "font-size: 12px; color: {c.accent_blue}; min-width: 80px;",
                                            "{start_time}"
                                        }
                                        div {
                                            style: "flex: 1;",
                                            p {
                                                style: "font-size: 14px; color: {c.text_white};",
                                                "{anim_name}"
                                            }
                                            if let Some(r) = room {
                                                p {
                                                    style: "font-size: 12px; color: {c.text_muted};",
                                                    "{r}"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Exposants
            section {
                style: "margin-bottom: 32px;",

                h2 {
                    style: "font-size: 18px; color: {c.text_white}; margin-bottom: 12px;",
                    "Exposants ({exposants_count})"
                }

                if exposants.is_empty() {
                    p {
                        style: "font-size: 14px; color: {c.text_muted};",
                        "Liste des exposants bientot disponible"
                    }
                } else {
                    div {
                        style: "display: grid; grid-template-columns: repeat(4, 1fr); gap: 12px;",

                        for exp in exposants.iter().take(8) {
                            {
                                let exp_id_str = opt_str(&exp.id);
                                let company_name = opt_str(&exp.company_name);
                                let exp_id_clone = exp.id.clone();
                                rsx! {
                                    div {
                                        key: "{exp_id_str}",
                                        style: "background: {c.bg_secondary}; border-radius: 8px; padding: 16px; text-align: center; cursor: pointer;",
                                        onclick: move |_| {
                                            state.write().selected_exposant_id = exp_id_clone.clone();
                                            state.write().unc_section = UncSection::ExposantDetail;
                                        },

                                        div {
                                            style: "width: 48px; height: 48px; margin: 0 auto 8px; background: {c.bg_hover}; border-radius: 50%; display: flex; align-items: center; justify-content: center; font-size: 20px;",
                                            "🧑‍💼"
                                        }
                                        p {
                                            style: "font-size: 12px; color: {c.text_primary};",
                                            "{company_name}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // CTA
            section {
                style: "background: {c.bg_secondary}; border-radius: 8px; padding: 24px; text-align: center;",

                p {
                    style: "font-size: 14px; color: {c.text_secondary}; margin-bottom: 16px;",
                    "Vous etes professionnel ? Participez en tant qu'exposant"
                }

                ActionButton {
                    label: "Candidater".to_string(),
                    icon: "📝".to_string(),
                    accent: true,
                    onclick: move |_| {
                        state.write().unc_section = UncSection::CtaModal;
                        state.write().cta_action = Some("candidater".to_string());
                    },
                }
            }
        }
    }
}

/// Item de liste d'evenement.
#[component]
fn EventListItem(
    edition: Edition,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let status_str = opt_str(&edition.status);
    let status_color = match status_str.as_str() {
        "publie" | "en_cours" => c.accent_green,
        "brouillon" => c.accent_orange,
        _ => c.text_muted,
    };

    let name = opt_str(&edition.name);
    let location = opt_str(&edition.location);
    let date_range = format_date_range(edition.start_date.as_ref(), edition.end_date.as_ref());
    let theme = edition.theme.clone();

    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 16px; background: {c.bg_secondary}; border-radius: 8px; padding: 16px; cursor: pointer; transition: background 0.2s;",
            onclick: move |evt| onclick.call(evt),

            // Image placeholder
            div {
                style: "width: 80px; height: 80px; background: linear-gradient(135deg, {c.bg_hover} 0%, {c.bg_secondary} 100%); border-radius: 8px; display: flex; align-items: center; justify-content: center; font-size: 32px;",
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
                    style: "display: flex; gap: 16px; font-size: 12px; color: {c.text_secondary}; margin-bottom: 8px;",

                    span { "📆 {date_range}" }
                    span { "📍 {location}" }
                }
                if let Some(t) = theme {
                    div {
                        style: "display: flex; gap: 8px;",
                        span {
                            style: "padding: 2px 8px; background: {c.bg_hover}; border-radius: 4px; font-size: 11px; color: {c.text_muted};",
                            "#{t}"
                        }
                    }
                }
            }

            // Badge + action
            div {
                style: "display: flex; flex-direction: column; align-items: flex-end; gap: 8px;",

                Badge {
                    text: status_str,
                    color: status_color.to_string(),
                }
                span {
                    style: "font-size: 12px; color: {c.accent_blue};",
                    "Voir fiche →"
                }
            }
        }
    }
}
