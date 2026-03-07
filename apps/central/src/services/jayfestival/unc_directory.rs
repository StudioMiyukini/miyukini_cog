//! UNC-E06/E07/E08/E09 — Annuaires publics (organisateurs et exposants).
//!
//! Listes et fiches detaillees des organisateurs et exposants visibles au public.
//!
//! @id: jf_unc_directory @do: render_unc_directory
//! @role: ui @layer: service
//! @human: Ecrans UNC-E06/E09 JayFestival: annuaires publics organisateurs et exposants.

use dioxus::prelude::*;
use miyuki_ui_dioxus::context::use_palette;
use crate::data::use_service_connections;
use crate::state::use_app_state;
use super::{UncSection, JayFestivalState};
use super::components::Badge;
use jayfestival::data::{Organisateur, Exposant};

fn opt_str(s: &Option<String>) -> String {
    s.clone().unwrap_or_default()
}

/// UNC-E06 — Liste des organisateurs.
#[component]
pub fn UncOrganisateursList(state: Signal<JayFestivalState>) -> Element {
    let p = use_palette();
    let conns = use_service_connections();

    let organisateurs = {
        let db = &conns.read().jayfestival;
        db.organisateurs_public_list().unwrap_or_default()
    };

    let mut search = use_signal(String::new);
    let search_str = search.read().clone();

    let filtered: Vec<_> = organisateurs
        .iter()
        .filter(|o| {
            search_str.is_empty()
                || o.name
                    .as_ref()
                    .is_some_and(|n| n.to_lowercase().contains(&search_str.to_lowercase()))
                || o.region
                    .as_ref()
                    .is_some_and(|r| r.to_lowercase().contains(&search_str.to_lowercase()))
        })
        .collect();

    rsx! {
        div {
            style: "max-width: 1200px; margin: 0 auto;",

            // Header
            div {
                style: "margin-bottom: 24px;",

                button {
                    style: "background: none; border: none; color: {p.text_secondary}; cursor: pointer; font-size: 13px;",
                    onclick: move |_| {
                        state.write().unc_section = UncSection::Landing;
                    },
                    "← Retour"
                }
                h1 {
                    style: "font-size: 24px; color: {p.text_high}; margin-top: 8px;",
                    "Organisateurs"
                }
            }

            // Recherche
            div {
                style: "margin-bottom: 24px;",

                input {
                    r#type: "text",
                    style: "width: 100%; max-width: 400px; padding: 12px 16px; background: {p.bg_secondary}; border: 1px solid {p.border_default}; border-radius: 8px; color: {p.text_primary}; font-size: 14px;",
                    placeholder: "Rechercher un organisateur...",
                    value: "{search}",
                    oninput: move |evt| {
                        search.set(evt.value());
                    },
                }
            }

            // Grille
            if filtered.is_empty() {
                div {
                    style: "padding: 40px; text-align: center; background: {p.bg_secondary}; border-radius: 8px; color: {p.text_muted};",
                    "Aucun organisateur trouve"
                }
            } else {
                div {
                    style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(280px, 1fr)); gap: 16px;",

                    for org in filtered {
                        {
                            let id_str = opt_str(&org.id);
                            let id_clone = org.id.clone();
                            rsx! {
                                OrganisateurCard {
                                    key: "{id_str}",
                                    organisateur: org.clone(),
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
    }
}

/// UNC-E07 — Fiche organisateur.
#[component]
pub fn UncOrganisateurDetail(state: Signal<JayFestivalState>) -> Element {
    let p = use_palette();
    let conns = use_service_connections();

    let org_id = state.read().selected_organisateur_id.clone();

    let organisateur = {
        let db = &conns.read().jayfestival;
        org_id
            .as_ref()
            .and_then(|id| db.organisateur_by_id(id).ok().flatten())
    };

    let Some(org) = organisateur else {
        return rsx! {
            div {
                style: "text-align: center; padding: 40px; color: {p.text_muted};",
                "Organisateur non trouve"
            }
        };
    };

    let editions = {
        let db = &conns.read().jayfestival;
        db.editions_published().unwrap_or_default()
    };

    let name = opt_str(&org.name);

    rsx! {
        div {
            style: "max-width: 900px; margin: 0 auto;",

            // Navigation retour
            button {
                style: "background: none; border: none; color: {p.text_secondary}; cursor: pointer; font-size: 13px; margin-bottom: 16px;",
                onclick: move |_| {
                    state.write().unc_section = UncSection::Organisateurs;
                },
                "← Retour aux organisateurs"
            }

            // Header
            div {
                style: "display: flex; gap: 24px; margin-bottom: 32px;",

                div {
                    style: "width: 120px; height: 120px; background: linear-gradient(135deg, {p.bg_secondary} 0%, {p.bg_overlay} 100%); border-radius: 12px; display: flex; align-items: center; justify-content: center; font-size: 48px;",
                    "🏢"
                }

                div {
                    h1 {
                        style: "font-size: 28px; color: {p.text_high}; margin-bottom: 8px;",
                        "{name}"
                    }
                    if let Some(region) = &org.region {
                        p {
                            style: "font-size: 14px; color: {p.text_secondary}; margin-bottom: 8px;",
                            "📍 {region}"
                        }
                    }
                    if let Some(website) = &org.website {
                        a {
                            href: "{website}",
                            style: "font-size: 14px; color: {p.accent_primary}; text-decoration: none;",
                            "🔗 {website}"
                        }
                    }
                }
            }

            // Description
            if let Some(desc) = &org.description {
                section {
                    style: "margin-bottom: 32px;",

                    h2 {
                        style: "font-size: 18px; color: {p.text_high}; margin-bottom: 12px;",
                        "A propos"
                    }
                    p {
                        style: "font-size: 14px; color: {p.text_secondary}; line-height: 1.6;",
                        "{desc}"
                    }
                }
            }

            // Événements
            section {
                h2 {
                    style: "font-size: 18px; color: {p.text_high}; margin-bottom: 16px;",
                    "Evenements organises"
                }

                if editions.is_empty() {
                    p {
                        style: "font-size: 14px; color: {p.text_muted};",
                        "Aucun evenement publie"
                    }
                } else {
                    div {
                        style: "display: flex; flex-direction: column; gap: 12px;",

                        for edition in editions.iter().take(5) {
                            {
                                let ed_id_str = opt_str(&edition.id);
                                let ed_name = opt_str(&edition.name);
                                let ed_start = opt_str(&edition.start_date);
                                let ed_location = opt_str(&edition.location);
                                let ed_id_clone = edition.id.clone();
                                rsx! {
                                    div {
                                        key: "{ed_id_str}",
                                        style: "display: flex; align-items: center; gap: 16px; background: {p.bg_secondary}; border-radius: 8px; padding: 16px; cursor: pointer;",
                                        onclick: move |_| {
                                            state.write().selected_edition_id = ed_id_clone.clone();
                                            state.write().unc_section = UncSection::EventDetail;
                                        },

                                        div {
                                            style: "width: 48px; height: 48px; background: {p.bg_overlay}; border-radius: 8px; display: flex; align-items: center; justify-content: center; font-size: 24px;",
                                            "🎪"
                                        }
                                        div {
                                            style: "flex: 1;",
                                            p {
                                                style: "font-size: 14px; color: {p.text_high};",
                                                "{ed_name}"
                                            }
                                            p {
                                                style: "font-size: 12px; color: {p.text_muted};",
                                                "{ed_start} — {ed_location}"
                                            }
                                        }
                                        span {
                                            style: "font-size: 12px; color: {p.accent_primary};",
                                            "Voir →"
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

/// UNC-E08 — Liste des exposants.
#[component]
pub fn UncExposantsList(state: Signal<JayFestivalState>) -> Element {
    let p = use_palette();
    let conns = use_service_connections();

    let exposants = {
        let db = &conns.read().jayfestival;
        db.exposants_public_list().unwrap_or_default()
    };

    let mut search = use_signal(String::new);
    let mut filter_category = use_signal(String::new);
    let search_str = search.read().clone();
    let cat_str = filter_category.read().clone();

    let filtered: Vec<_> = exposants
        .iter()
        .filter(|e| {
            let search_match = search_str.is_empty()
                || e.company_name
                    .as_ref()
                    .is_some_and(|n| n.to_lowercase().contains(&search_str.to_lowercase()))
                || e.secteur
                    .as_ref()
                    .is_some_and(|s| s.to_lowercase().contains(&search_str.to_lowercase()));
            let cat_match = cat_str.is_empty()
                || e.category
                    .as_ref()
                    .is_some_and(|c| c == &cat_str);
            search_match && cat_match
        })
        .collect();

    rsx! {
        div {
            style: "max-width: 1200px; margin: 0 auto;",

            // Header
            div {
                style: "margin-bottom: 24px;",

                button {
                    style: "background: none; border: none; color: {p.text_secondary}; cursor: pointer; font-size: 13px;",
                    onclick: move |_| {
                        state.write().unc_section = UncSection::Landing;
                    },
                    "← Retour"
                }
                h1 {
                    style: "font-size: 24px; color: {p.text_high}; margin-top: 8px;",
                    "Exposants"
                }
            }

            // Filtres
            div {
                style: "display: flex; gap: 16px; margin-bottom: 24px;",

                input {
                    r#type: "text",
                    style: "flex: 1; max-width: 400px; padding: 12px 16px; background: {p.bg_secondary}; border: 1px solid {p.border_default}; border-radius: 8px; color: {p.text_primary}; font-size: 14px;",
                    placeholder: "Rechercher un exposant...",
                    value: "{search}",
                    oninput: move |evt| {
                        search.set(evt.value());
                    },
                }

                select {
                    style: "padding: 12px 16px; background: {p.bg_secondary}; border: 1px solid {p.border_default}; border-radius: 8px; color: {p.text_primary}; font-size: 14px;",
                    value: "{filter_category}",
                    onchange: move |evt| {
                        filter_category.set(evt.value());
                    },

                    option { value: "", "Toutes categories" }
                    option { value: "artisanat", "Artisanat" }
                    option { value: "edition", "Edition" }
                    option { value: "gastronomie", "Gastronomie" }
                    option { value: "jeux", "Jeux" }
                    option { value: "autre", "Autre" }
                }
            }

            // Grille
            if filtered.is_empty() {
                div {
                    style: "padding: 40px; text-align: center; background: {p.bg_secondary}; border-radius: 8px; color: {p.text_muted};",
                    "Aucun exposant trouve"
                }
            } else {
                div {
                    style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(280px, 1fr)); gap: 16px;",

                    for exp in filtered {
                        {
                            let id_str = opt_str(&exp.id);
                            let id_clone = exp.id.clone();
                            rsx! {
                                ExposantCard {
                                    key: "{id_str}",
                                    exposant: exp.clone(),
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

/// UNC-E09 — Fiche exposant.
#[component]
pub fn UncExposantDetail(state: Signal<JayFestivalState>) -> Element {
    let p = use_palette();
    let conns = use_service_connections();

    let exp_id = state.read().selected_exposant_id.clone();

    let exposant = {
        let db = &conns.read().jayfestival;
        exp_id
            .as_ref()
            .and_then(|id| db.exposant_by_id(id).ok().flatten())
    };

    let Some(exp) = exposant else {
        return rsx! {
            div {
                style: "text-align: center; padding: 40px; color: {p.text_muted};",
                "Exposant non trouve"
            }
        };
    };

    let company_name = opt_str(&exp.company_name);

    rsx! {
        div {
            style: "max-width: 900px; margin: 0 auto;",

            // Navigation retour
            button {
                style: "background: none; border: none; color: {p.text_secondary}; cursor: pointer; font-size: 13px; margin-bottom: 16px;",
                onclick: move |_| {
                    state.write().unc_section = UncSection::Exposants;
                },
                "← Retour aux exposants"
            }

            // Header
            div {
                style: "display: flex; gap: 24px; margin-bottom: 32px;",

                div {
                    style: "width: 120px; height: 120px; background: linear-gradient(135deg, {p.bg_secondary} 0%, {p.bg_overlay} 100%); border-radius: 12px; display: flex; align-items: center; justify-content: center; font-size: 48px;",
                    "🧑‍💼"
                }

                div {
                    h1 {
                        style: "font-size: 28px; color: {p.text_high}; margin-bottom: 8px;",
                        "{company_name}"
                    }
                    if let Some(secteur) = &exp.secteur {
                        p {
                            style: "font-size: 14px; color: {p.text_secondary}; margin-bottom: 8px;",
                            "🏷️ {secteur}"
                        }
                    }
                    if let Some(category) = &exp.category {
                        Badge {
                            text: category.clone(),
                            color: p.accent_primary.to_string(),
                        }
                    }
                }
            }

            // Description
            if let Some(desc) = &exp.description {
                section {
                    style: "margin-bottom: 32px;",

                    h2 {
                        style: "font-size: 18px; color: {p.text_high}; margin-bottom: 12px;",
                        "Description"
                    }
                    p {
                        style: "font-size: 14px; color: {p.text_secondary}; line-height: 1.6;",
                        "{desc}"
                    }
                }
            }

            // Contact
            section {
                style: "background: {p.bg_secondary}; border-radius: 8px; padding: 20px;",

                h2 {
                    style: "font-size: 18px; color: {p.text_high}; margin-bottom: 16px;",
                    "Contact"
                }

                div {
                    style: "display: grid; grid-template-columns: repeat(2, 1fr); gap: 12px;",

                    if let Some(email) = &exp.contact_email {
                        div {
                            span {
                                style: "font-size: 12px; color: {p.text_muted}; display: block; margin-bottom: 4px;",
                                "Email"
                            }
                            span {
                                style: "font-size: 14px; color: {p.text_primary};",
                                "{email}"
                            }
                        }
                    }
                    if let Some(phone) = &exp.contact_phone {
                        div {
                            span {
                                style: "font-size: 12px; color: {p.text_muted}; display: block; margin-bottom: 4px;",
                                "Telephone"
                            }
                            span {
                                style: "font-size: 14px; color: {p.text_primary};",
                                "{phone}"
                            }
                        }
                    }
                    if let Some(site) = &exp.site_web {
                        div {
                            span {
                                style: "font-size: 12px; color: {p.text_muted}; display: block; margin-bottom: 4px;",
                                "Site web"
                            }
                            a {
                                href: "{site}",
                                style: "font-size: 14px; color: {p.accent_primary}; text-decoration: none;",
                                "{site}"
                            }
                        }
                    }
                    if let Some(adresse) = &exp.adresse {
                        div {
                            span {
                                style: "font-size: 12px; color: {p.text_muted}; display: block; margin-bottom: 4px;",
                                "Adresse"
                            }
                            span {
                                style: "font-size: 14px; color: {p.text_primary};",
                                "{adresse}"
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Carte organisateur.
#[component]
fn OrganisateurCard(
    organisateur: Organisateur,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let p = use_palette();
    let name = opt_str(&organisateur.name);

    rsx! {
        div {
            style: "background: {p.bg_secondary}; border-radius: 12px; padding: 20px; cursor: pointer; transition: transform 0.2s;",
            onclick: move |evt| onclick.call(evt),

            div {
                style: "display: flex; align-items: center; gap: 16px; margin-bottom: 12px;",

                div {
                    style: "width: 56px; height: 56px; background: {p.bg_overlay}; border-radius: 50%; display: flex; align-items: center; justify-content: center; font-size: 24px;",
                    "🏢"
                }
                div {
                    h3 {
                        style: "font-size: 16px; color: {p.text_high};",
                        "{name}"
                    }
                    if let Some(region) = &organisateur.region {
                        p {
                            style: "font-size: 12px; color: {p.text_muted};",
                            "📍 {region}"
                        }
                    }
                }
            }

            span {
                style: "font-size: 12px; color: {p.accent_primary};",
                "Voir le profil →"
            }
        }
    }
}

/// Carte exposant.
#[component]
fn ExposantCard(
    exposant: Exposant,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let p = use_palette();
    let company_name = opt_str(&exposant.company_name);

    rsx! {
        div {
            style: "background: {p.bg_secondary}; border-radius: 12px; padding: 20px; cursor: pointer; transition: transform 0.2s;",
            onclick: move |evt| onclick.call(evt),

            div {
                style: "display: flex; align-items: center; gap: 16px; margin-bottom: 12px;",

                div {
                    style: "width: 56px; height: 56px; background: {p.bg_overlay}; border-radius: 50%; display: flex; align-items: center; justify-content: center; font-size: 24px;",
                    "🧑‍💼"
                }
                div {
                    h3 {
                        style: "font-size: 16px; color: {p.text_high};",
                        "{company_name}"
                    }
                    if let Some(secteur) = &exposant.secteur {
                        p {
                            style: "font-size: 12px; color: {p.text_muted};",
                            "{secteur}"
                        }
                    }
                }
            }

            if let Some(cat) = &exposant.category {
                Badge {
                    text: cat.clone(),
                    color: p.accent_primary.to_string(),
                }
            }
        }
    }
}
