//! UNC-E01 — Landing JayFestival (facade publique).
//!
//! Page d'accueil du catalogue d'evenements, accessible sans authentification.
//! Affiche : hero, recherche, categories, prochains evenements, CTA inscription.
//!
//! @id: jf_unc_landing @do: render_unc_landing
//! @role: ui @layer: service
//! @human: Ecran UNC-E01 JayFestival: landing page publique (hero, recherche, CTA inscription).

use dioxus::prelude::*;
use crate::data::use_service_connections;
use crate::state::use_app_state;
use super::{UncSection, JayFestivalState};
use super::components::{EventCard, format_date_range};

fn opt_str(s: &Option<String>) -> String {
    s.clone().unwrap_or_default()
}

/// Page d'accueil UNC — Landing.
#[component]
pub fn UncLanding(state: Signal<JayFestivalState>) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let conns = use_service_connections();

    // Charger les événements publiés
    let editions = {
        let db = &conns.read().jayfestival;
        db.editions_published().unwrap_or_default()
    };

    // Charger les compteurs
    let organisateurs_count = {
        let db = &conns.read().jayfestival;
        db.organisateurs_public_list().map(|v| v.len()).unwrap_or(0)
    };

    let exposants_count = {
        let db = &conns.read().jayfestival;
        db.exposants_public_list().map(|v| v.len()).unwrap_or(0)
    };

    rsx! {
        div {
            style: "max-width: 1200px; margin: 0 auto;",

            // Hero Section
            section {
                style: "text-align: center; padding: 48px 24px; background: linear-gradient(135deg, {c.bg_secondary} 0%, {c.bg_main} 100%); border-radius: 12px; margin-bottom: 32px;",

                h1 {
                    style: "font-size: 32px; color: {c.text_white}; margin-bottom: 12px;",
                    "Decouvrez les evenements et festivals"
                }
                p {
                    style: "font-size: 16px; color: {c.text_secondary}; margin-bottom: 24px;",
                    "Trouvez votre prochain evenement, participez en tant qu'exposant ou organisez le votre"
                }

                // Barre de recherche
                div {
                    style: "max-width: 600px; margin: 0 auto;",

                    div {
                        style: "display: flex; gap: 8px;",

                        input {
                            style: "flex: 1; padding: 14px 16px; background: {c.bg_main}; border: 1px solid {c.border}; border-radius: 8px; color: {c.text_primary}; font-size: 14px;",
                            r#type: "text",
                            placeholder: "Rechercher un evenement, organisateur, exposant...",
                            onkeypress: move |evt: KeyboardEvent| {
                                if evt.key() == Key::Enter {
                                    state.write().unc_section = UncSection::Search;
                                }
                            },
                        }
                        button {
                            style: "padding: 14px 24px; background: {c.accent_blue}; border: none; border-radius: 8px; color: white; cursor: pointer; font-size: 14px;",
                            onclick: move |_| {
                                state.write().unc_section = UncSection::Search;
                            },
                            "Rechercher"
                        }
                    }
                }
            }

            // Catégories rapides
            section {
                style: "display: grid; grid-template-columns: repeat(3, 1fr); gap: 16px; margin-bottom: 40px;",

                CategoryCard {
                    icon: "🎪",
                    title: "Evenements",
                    count: editions.len(),
                    onclick: move |_| {
                        state.write().unc_section = UncSection::Events;
                    },
                }

                CategoryCard {
                    icon: "🏢",
                    title: "Organisateurs",
                    count: organisateurs_count,
                    onclick: move |_| {
                        state.write().unc_section = UncSection::Organisateurs;
                    },
                }

                CategoryCard {
                    icon: "🧑‍💼",
                    title: "Exposants",
                    count: exposants_count,
                    onclick: move |_| {
                        state.write().unc_section = UncSection::Exposants;
                    },
                }
            }

            // Prochains événements
            section {
                style: "margin-bottom: 40px;",

                div {
                    style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 16px;",

                    h2 {
                        style: "font-size: 20px; color: {c.text_white};",
                        "Prochains evenements"
                    }
                    button {
                        style: "padding: 8px 16px; background: transparent; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_secondary}; cursor: pointer; font-size: 13px;",
                        onclick: move |_| {
                            state.write().unc_section = UncSection::Events;
                        },
                        "Voir tout →"
                    }
                }

                if editions.is_empty() {
                    div {
                        style: "padding: 40px; text-align: center; background: {c.bg_secondary}; border-radius: 8px; color: {c.text_muted};",
                        "Aucun evenement disponible pour le moment"
                    }
                } else {
                    div {
                        style: "display: flex; flex-direction: column; gap: 12px;",

                        for edition in editions.iter().take(5) {
                            {
                                let id_str = opt_str(&edition.id);
                                let name = opt_str(&edition.name);
                                let date = format_date_range(edition.start_date.as_ref(), edition.end_date.as_ref());
                                let location = opt_str(&edition.location);
                                let status = opt_str(&edition.status);
                                let id_clone = edition.id.clone();
                                rsx! {
                                    EventCard {
                                        key: "{id_str}",
                                        name: name,
                                        date: date,
                                        location: location,
                                        status: status,
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

            // CTA Rejoindre la plateforme
            section {
                style: "margin-bottom: 40px;",

                h2 {
                    style: "font-size: 20px; color: {c.text_white}; text-align: center; margin-bottom: 24px;",
                    "Rejoignez la plateforme"
                }

                div {
                    style: "display: grid; grid-template-columns: repeat(3, 1fr); gap: 16px;",

                    RoleCard {
                        icon: "🏢",
                        title: "Je suis Organisateur",
                        description: "Creez et gerez vos evenements, festivals et salons",
                        cta: "S'inscrire →",
                        onclick: move |_| {
                            state.write().unc_section = UncSection::Inscription;
                            state.write().inscription_type = Some("organisateur".to_string());
                        },
                    }

                    RoleCard {
                        icon: "🧑‍💼",
                        title: "Je suis Exposant",
                        description: "Participez a des evenements et presentez votre activite",
                        cta: "S'inscrire →",
                        onclick: move |_| {
                            state.write().unc_section = UncSection::Inscription;
                            state.write().inscription_type = Some("exposant".to_string());
                        },
                    }

                    RoleCard {
                        icon: "👤",
                        title: "Je suis Visiteur",
                        description: "Decouvrez les evenements et reservez vos places",
                        cta: "S'inscrire →",
                        onclick: move |_| {
                            state.write().unc_section = UncSection::Inscription;
                            state.write().inscription_type = Some("visiteur".to_string());
                        },
                    }
                }
            }

            // Lien connexion
            div {
                style: "text-align: center; padding: 24px; border-top: 1px solid {c.border};",

                span {
                    style: "color: {c.text_secondary}; font-size: 14px;",
                    "Deja inscrit ? "
                }
                button {
                    style: "background: none; border: none; color: {c.accent_blue}; cursor: pointer; font-size: 14px;",
                    onclick: move |_| {
                        state.write().unc_section = UncSection::Connexion;
                    },
                    "Se connecter"
                }
            }
        }
    }
}

/// Carte catégorie cliquable.
#[component]
fn CategoryCard(icon: &'static str, title: &'static str, count: usize, onclick: EventHandler<MouseEvent>) -> Element {
    let c = use_app_state().read().current_theme.palette();
    rsx! {
        div {
            style: "background: {c.bg_secondary}; border-radius: 12px; padding: 24px; text-align: center; cursor: pointer; transition: transform 0.2s, background 0.2s;",
            onclick: move |evt| onclick.call(evt),

            span {
                style: "font-size: 40px; display: block; margin-bottom: 12px;",
                "{icon}"
            }
            h3 {
                style: "font-size: 16px; color: {c.text_white}; margin-bottom: 4px;",
                "{title}"
            }
            p {
                style: "font-size: 13px; color: {c.text_muted};",
                "{count} disponibles"
            }
        }
    }
}

/// Carte rôle pour inscription.
#[component]
fn RoleCard(
    icon: &'static str,
    title: &'static str,
    description: &'static str,
    cta: &'static str,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let c = use_app_state().read().current_theme.palette();
    rsx! {
        div {
            style: "background: {c.bg_secondary}; border-radius: 12px; padding: 24px; display: flex; flex-direction: column; align-items: center; text-align: center;",

            span {
                style: "font-size: 40px; margin-bottom: 16px;",
                "{icon}"
            }
            h3 {
                style: "font-size: 16px; color: {c.text_white}; margin-bottom: 8px;",
                "{title}"
            }
            p {
                style: "font-size: 13px; color: {c.text_secondary}; margin-bottom: 16px; flex: 1;",
                "{description}"
            }
            button {
                style: "width: 100%; padding: 10px 16px; background: {c.accent_blue}; border: none; border-radius: 6px; color: white; cursor: pointer; font-size: 13px;",
                onclick: move |evt| onclick.call(evt),
                "{cta}"
            }
        }
    }
}
