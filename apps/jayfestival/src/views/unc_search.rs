//! UNC-E10 — Recherche globale (facade publique).
//!
//! Recherche sur evenements, organisateurs et exposants.

use dioxus::prelude::*;
use miyukini_service_ui::use_palette;
use super::{UncSection, JayFestivalState};
use super::components::{Badge, format_date_range};

fn opt_str(s: &Option<String>) -> String {
    s.clone().unwrap_or_default()
}

/// UNC-E10 — Page de recherche.
#[component]
pub fn UncSearch(state: Signal<JayFestivalState>) -> Element {
    let c = use_palette();

    let initial_query = state.read().search_query.clone().unwrap_or_default();
    let mut query = use_signal(|| initial_query);
    let mut active_tab = use_signal(|| "all".to_string());

    let query_str = query.read().clone();
    let (editions, organisateurs, exposants) = {
        let db = crate::use_db();
        if query_str.is_empty() {
            (vec![], vec![], vec![])
        } else {
            db.search_all(&query_str).unwrap_or_default()
        }
    };

    let total = editions.len() + organisateurs.len() + exposants.len();
    let tab = active_tab.read().clone();

    rsx! {
        div {
            style: "max-width: 1200px; margin: 0 auto;",

            // Header
            div {
                style: "margin-bottom: 24px;",

                button {
                    style: "background: none; border: none; color: {c.text_secondary}; cursor: pointer; font-size: 13px;",
                    onclick: move |_| {
                        state.write().unc_section = UncSection::Landing;
                    },
                    "← Retour"
                }
                h1 {
                    style: "font-size: 24px; color: {c.text_white}; margin-top: 8px;",
                    "Recherche"
                }
            }

            // Barre de recherche
            div {
                style: "margin-bottom: 24px;",

                div {
                    style: "display: flex; gap: 8px;",

                    input {
                        style: "flex: 1; padding: 14px 16px; background: {c.bg_secondary}; border: 1px solid {c.border}; border-radius: 8px; color: {c.text_primary}; font-size: 14px;",
                        r#type: "text",
                        placeholder: "Rechercher un evenement, organisateur, exposant...",
                        value: "{query}",
                        oninput: move |evt| {
                            query.set(evt.value());
                        },
                        onkeypress: move |evt: KeyboardEvent| {
                            if evt.key() == Key::Enter {
                                state.write().search_query = Some(query.read().clone());
                            }
                        },
                    }
                    button {
                        style: "padding: 14px 24px; background: {c.accent_blue}; border: none; border-radius: 8px; color: white; cursor: pointer; font-size: 14px;",
                        onclick: move |_| {
                            state.write().search_query = Some(query.read().clone());
                        },
                        "Rechercher"
                    }
                }
            }

            // Onglets de filtrage
            div {
                style: "display: flex; gap: 8px; margin-bottom: 24px; border-bottom: 1px solid {c.border}; padding-bottom: 12px;",

                SearchTabButton {
                    label: format!("Tous ({total})"),
                    is_active: tab == "all",
                    onclick: move |_| active_tab.set("all".to_string()),
                }
                SearchTabButton {
                    label: format!("Evenements ({0})", editions.len()),
                    is_active: tab == "events",
                    onclick: move |_| active_tab.set("events".to_string()),
                }
                SearchTabButton {
                    label: format!("Organisateurs ({0})", organisateurs.len()),
                    is_active: tab == "orgs",
                    onclick: move |_| active_tab.set("orgs".to_string()),
                }
                SearchTabButton {
                    label: format!("Exposants ({0})", exposants.len()),
                    is_active: tab == "exps",
                    onclick: move |_| active_tab.set("exps".to_string()),
                }
            }

            // Résultats
            if query_str.is_empty() {
                div {
                    style: "text-align: center; padding: 60px; color: {c.text_muted};",
                    span {
                        style: "font-size: 48px; display: block; margin-bottom: 16px; opacity: 0.3;",
                        "🔍"
                    }
                    p { "Entrez un terme de recherche" }
                }
            } else if total == 0 {
                div {
                    style: "text-align: center; padding: 60px; color: {c.text_muted};",
                    span {
                        style: "font-size: 48px; display: block; margin-bottom: 16px; opacity: 0.3;",
                        "😕"
                    }
                    p { "Aucun resultat pour \"{query_str}\"" }
                }
            } else {
                div {
                    style: "display: flex; flex-direction: column; gap: 24px;",

                    // Événements
                    if (tab == "all" || tab == "events") && !editions.is_empty() {
                        section {
                            if tab == "all" {
                                h2 {
                                    style: "font-size: 16px; color: {c.text_white}; margin-bottom: 12px;",
                                    "Evenements"
                                }
                            }

                            div {
                                style: "display: flex; flex-direction: column; gap: 8px;",

                                for edition in &editions {
                                    {
                                        let id_str = opt_str(&edition.id);
                                        let name = opt_str(&edition.name);
                                        let location = opt_str(&edition.location);
                                        let subtitle = format!("{} — {}", format_date_range(edition.start_date.as_ref(), edition.end_date.as_ref()), location);
                                        let badge = edition.status.clone();
                                        let id_clone = edition.id.clone();
                                        rsx! {
                                            SearchResultItem {
                                                key: "{id_str}",
                                                icon: "🎪",
                                                title: name,
                                                subtitle: subtitle,
                                                badge: badge,
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

                    // Organisateurs
                    if (tab == "all" || tab == "orgs") && !organisateurs.is_empty() {
                        section {
                            if tab == "all" {
                                h2 {
                                    style: "font-size: 16px; color: {c.text_white}; margin-bottom: 12px;",
                                    "Organisateurs"
                                }
                            }

                            div {
                                style: "display: flex; flex-direction: column; gap: 8px;",

                                for org in &organisateurs {
                                    {
                                        let id_str = opt_str(&org.id);
                                        let name = opt_str(&org.name);
                                        let region = opt_str(&org.region);
                                        let id_clone = org.id.clone();
                                        rsx! {
                                            SearchResultItem {
                                                key: "{id_str}",
                                                icon: "🏢",
                                                title: name,
                                                subtitle: region,
                                                badge: None,
                                                onclick: move |_| {
                                                    state.write().selected_organisateur_id = id_clone.clone();
                                                    state.write().unc_section = UncSection::OrganisateurDetail;
                                                },
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Exposants
                    if (tab == "all" || tab == "exps") && !exposants.is_empty() {
                        section {
                            if tab == "all" {
                                h2 {
                                    style: "font-size: 16px; color: {c.text_white}; margin-bottom: 12px;",
                                    "Exposants"
                                }
                            }

                            div {
                                style: "display: flex; flex-direction: column; gap: 8px;",

                                for exp in &exposants {
                                    {
                                        let id_str = opt_str(&exp.id);
                                        let company = opt_str(&exp.company_name);
                                        let secteur = opt_str(&exp.secteur);
                                        let cat = exp.category.clone();
                                        let id_clone = exp.id.clone();
                                        rsx! {
                                            SearchResultItem {
                                                key: "{id_str}",
                                                icon: "🧑‍💼",
                                                title: company,
                                                subtitle: secteur,
                                                badge: cat,
                                                onclick: move |_| {
                                                    state.write().selected_exposant_id = id_clone.clone();
                                                    state.write().unc_section = UncSection::ExposantDetail;
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
}

/// Onglet de filtrage.
#[component]
fn SearchTabButton(label: String, is_active: bool, onclick: EventHandler<MouseEvent>) -> Element {
    let c = use_palette();
    let color = if is_active { c.text_white } else { c.text_muted };
    let border = if is_active { c.accent_blue } else { "transparent" };

    rsx! {
        button {
            style: "padding: 8px 16px; background: transparent; border: none; border-bottom: 2px solid {border}; color: {color}; cursor: pointer; font-size: 13px;",
            onclick: move |evt| onclick.call(evt),
            "{label}"
        }
    }
}

/// Item de résultat de recherche.
#[component]
fn SearchResultItem(
    icon: &'static str,
    title: String,
    subtitle: String,
    badge: Option<String>,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let c = use_palette();

    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 16px; background: {c.bg_secondary}; border-radius: 8px; padding: 16px; cursor: pointer; transition: background 0.2s;",
            onclick: move |evt| onclick.call(evt),

            span {
                style: "font-size: 24px;",
                "{icon}"
            }

            div {
                style: "flex: 1;",
                p {
                    style: "font-size: 14px; color: {c.text_white};",
                    "{title}"
                }
                if !subtitle.is_empty() {
                    p {
                        style: "font-size: 12px; color: {c.text_muted};",
                        "{subtitle}"
                    }
                }
            }

            if let Some(b) = badge {
                Badge {
                    text: b,
                    color: c.accent_blue.to_string(),
                }
            }

            span {
                style: "font-size: 12px; color: {c.accent_blue};",
                "→"
            }
        }
    }
}
