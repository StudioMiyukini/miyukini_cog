//! ORG-E08 — Parametrage d'une edition (dates, lieu, theme, reglement).

use dioxus::prelude::*;
use crate::data::use_service_connections;
use crate::state::use_app_state;
use super::components::ActionButton;

fn opt_str(s: &Option<String>) -> String {
    s.clone().unwrap_or_default()
}

/// Parametrage edition.
#[component]
pub fn OrgParametres(edition_id: String) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let conns = use_service_connections();

    // Charger l'édition
    let edition = {
        let db = &conns.read().jayfestival;
        db.edition_by_id(&edition_id).ok().flatten()
    };

    let Some(edition) = edition else {
        return rsx! {
            div {
                style: "text-align: center; padding: 40px; color: {c.text_muted};",
                "Edition non trouvee"
            }
        };
    };

    // Signaux d'édition
    let mut name = use_signal(|| opt_str(&edition.name));
    let mut location = use_signal(|| opt_str(&edition.location));
    let mut theme = use_signal(|| opt_str(&edition.theme));
    let mut start_date = use_signal(|| opt_str(&edition.start_date));
    let mut end_date = use_signal(|| opt_str(&edition.end_date));
    let mut status = use_signal(|| opt_str(&edition.status));
    let mut saved = use_signal(|| false);

    let handle_save = {
        let edition_id = edition_id.clone();
        move |_| {
            let db = &conns.read().jayfestival;
            let updated = jayfestival::data::Edition {
                id: Some(edition_id.clone()),
                name: Some(name.read().clone()),
                slug: edition.slug.clone(),
                start_date: Some(start_date.read().clone()),
                end_date: Some(end_date.read().clone()),
                location: Some(location.read().clone()),
                theme: Some(theme.read().clone()),
                status: Some(status.read().clone()),
                created_at: edition.created_at.clone(),
                updated_at: None,
            };
            if db.edition_update(&updated).is_ok() {
                saved.set(true);
            }
        }
    };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px;",

            // Titre
            h3 {
                style: "font-size: 18px; color: {c.text_white};",
                "Parametres de l'edition"
            }

            // Formulaire
            div {
                style: "display: grid; grid-template-columns: repeat(2, 1fr); gap: 20px;",

                // Nom
                div {
                    label {
                        style: "display: block; font-size: 12px; color: {c.text_secondary}; margin-bottom: 6px;",
                        "Nom de l'evenement"
                    }
                    input {
                        r#type: "text",
                        style: "width: 100%; padding: 10px 12px; background: {c.bg_secondary}; border: 1px solid {c.border}; border-radius: 6px; color: {c.text_primary}; font-size: 14px;",
                        value: "{name}",
                        oninput: move |evt| name.set(evt.value()),
                    }
                }

                // Lieu
                div {
                    label {
                        style: "display: block; font-size: 12px; color: {c.text_secondary}; margin-bottom: 6px;",
                        "Lieu"
                    }
                    input {
                        r#type: "text",
                        style: "width: 100%; padding: 10px 12px; background: {c.bg_secondary}; border: 1px solid {c.border}; border-radius: 6px; color: {c.text_primary}; font-size: 14px;",
                        value: "{location}",
                        oninput: move |evt| location.set(evt.value()),
                    }
                }

                // Date debut
                div {
                    label {
                        style: "display: block; font-size: 12px; color: {c.text_secondary}; margin-bottom: 6px;",
                        "Date de debut"
                    }
                    input {
                        r#type: "date",
                        style: "width: 100%; padding: 10px 12px; background: {c.bg_secondary}; border: 1px solid {c.border}; border-radius: 6px; color: {c.text_primary}; font-size: 14px;",
                        value: "{start_date}",
                        oninput: move |evt| start_date.set(evt.value()),
                    }
                }

                // Date fin
                div {
                    label {
                        style: "display: block; font-size: 12px; color: {c.text_secondary}; margin-bottom: 6px;",
                        "Date de fin"
                    }
                    input {
                        r#type: "date",
                        style: "width: 100%; padding: 10px 12px; background: {c.bg_secondary}; border: 1px solid {c.border}; border-radius: 6px; color: {c.text_primary}; font-size: 14px;",
                        value: "{end_date}",
                        oninput: move |evt| end_date.set(evt.value()),
                    }
                }

                // Statut
                div {
                    label {
                        style: "display: block; font-size: 12px; color: {c.text_secondary}; margin-bottom: 6px;",
                        "Statut"
                    }
                    select {
                        style: "width: 100%; padding: 10px 12px; background: {c.bg_secondary}; border: 1px solid {c.border}; border-radius: 6px; color: {c.text_primary}; font-size: 14px;",
                        value: "{status}",
                        onchange: move |evt| status.set(evt.value()),

                        option { value: "brouillon", "Brouillon" }
                        option { value: "en_preparation", "En preparation" }
                        option { value: "publie", "Publie" }
                        option { value: "en_cours", "En cours" }
                        option { value: "termine", "Termine" }
                    }
                }
            }

            // Theme / Description
            div {
                label {
                    style: "display: block; font-size: 12px; color: {c.text_secondary}; margin-bottom: 6px;",
                    "Theme / Description"
                }
                textarea {
                    style: "width: 100%; padding: 10px 12px; background: {c.bg_secondary}; border: 1px solid {c.border}; border-radius: 6px; color: {c.text_primary}; font-size: 14px; min-height: 100px; resize: vertical;",
                    value: "{theme}",
                    oninput: move |evt| theme.set(evt.value()),
                }
            }

            // Boutons
            div {
                style: "display: flex; gap: 12px; align-items: center;",

                ActionButton {
                    label: "Enregistrer".to_string(),
                    icon: "💾".to_string(),
                    accent: true,
                    onclick: handle_save,
                }

                if *saved.read() {
                    span {
                        style: "color: {c.accent_green}; font-size: 13px;",
                        "✓ Modifications enregistrees"
                    }
                }
            }

            // Section Reglement
            section {
                style: "margin-top: 24px; padding-top: 24px; border-top: 1px solid {c.border};",

                h3 {
                    style: "font-size: 16px; color: {c.text_white}; margin-bottom: 16px;",
                    "Reglement de l'evenement"
                }

                div {
                    style: "background: {c.bg_secondary}; border-radius: 8px; padding: 20px;",

                    p {
                        style: "font-size: 14px; color: {c.text_secondary}; margin-bottom: 16px;",
                        "Definissez les regles de participation pour les exposants."
                    }

                    div {
                        style: "display: flex; flex-direction: column; gap: 12px;",

                        label {
                            style: "display: flex; align-items: center; gap: 8px; font-size: 13px; color: {c.text_primary}; cursor: pointer;",
                            input {
                                r#type: "checkbox",
                            }
                            "Candidature obligatoire pour les exposants"
                        }
                        label {
                            style: "display: flex; align-items: center; gap: 8px; font-size: 13px; color: {c.text_primary}; cursor: pointer;",
                            input {
                                r#type: "checkbox",
                            }
                            "Paiement requis avant validation"
                        }
                        label {
                            style: "display: flex; align-items: center; gap: 8px; font-size: 13px; color: {c.text_primary}; cursor: pointer;",
                            input {
                                r#type: "checkbox",
                            }
                            "Afficher le programme au public"
                        }
                    }
                }
            }
        }
    }
}
