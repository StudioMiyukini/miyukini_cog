//! ORG-E05/E06 — Liste des éditions + création d'une nouvelle édition.
//!
//! @id: jf_org_editions @do: render_org_editions
//! @role: ui @layer: service
//! @human: Ecrans ORG-E05/E06 JayFestival: liste des editions et formulaire de creation.

use dioxus::prelude::*;
use miyuki_ui_dioxus::context::use_palette;
use jayfestival::data::Edition;
use crate::data::use_service_connections;
use crate::state::use_app_state;
use super::components::{ActionButton, EventCard, EmptyState, format_date_range};
use super::{JayFestivalState, OrgEditionTab, OrgSection};

#[component]
pub fn OrgEditions(
    editions: Vec<Edition>,
    state: Signal<JayFestivalState>,
) -> Element {
    let p = use_palette();
    let conns = use_service_connections();
    let mut show_form = use_signal(|| false);
    let mut new_name = use_signal(String::new);
    let mut new_location = use_signal(String::new);
    let mut new_start = use_signal(String::new);
    let mut new_end = use_signal(String::new);

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px;",

            // En-tête
            div {
                style: "display: flex; justify-content: space-between; align-items: center;",

                h2 {
                    style: "font-size: 24px; color: {p.text_high};",
                    "Mes editions"
                }

                ActionButton {
                    label: "Nouvelle edition".to_string(),
                    icon: "➕".to_string(),
                    accent: true,
                    onclick: move |_| {
                        show_form.set(!show_form());
                    },
                }
            }

            // Formulaire de création (inline)
            if *show_form.read() {
                div {
                    style: "background: {p.bg_secondary}; border-radius: 8px; padding: 20px; border: 1px solid {p.border_default};",

                    h3 {
                        style: "font-size: 16px; color: {p.text_high}; margin-bottom: 16px;",
                        "Creer une nouvelle edition"
                    }

                    div {
                        style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px; margin-bottom: 16px;",

                        div {
                            label {
                                style: "display: block; font-size: 12px; color: {p.text_secondary}; margin-bottom: 4px;",
                                "Nom de l'edition *"
                            }
                            input {
                                style: "width: 100%; padding: 8px 12px; background: {p.bg_base}; border: 1px solid {p.border_default}; border-radius: 4px; color: {p.text_high}; font-size: 13px; box-sizing: border-box;",
                                r#type: "text",
                                placeholder: "Ex: Festival 2026",
                                value: "{new_name}",
                                oninput: move |evt| new_name.set(evt.value()),
                            }
                        }
                        div {
                            label {
                                style: "display: block; font-size: 12px; color: {p.text_secondary}; margin-bottom: 4px;",
                                "Lieu"
                            }
                            input {
                                style: "width: 100%; padding: 8px 12px; background: {p.bg_base}; border: 1px solid {p.border_default}; border-radius: 4px; color: {p.text_high}; font-size: 13px; box-sizing: border-box;",
                                r#type: "text",
                                placeholder: "Ex: Paris Expo",
                                value: "{new_location}",
                                oninput: move |evt| new_location.set(evt.value()),
                            }
                        }
                        div {
                            label {
                                style: "display: block; font-size: 12px; color: {p.text_secondary}; margin-bottom: 4px;",
                                "Date de debut"
                            }
                            input {
                                style: "width: 100%; padding: 8px 12px; background: {p.bg_base}; border: 1px solid {p.border_default}; border-radius: 4px; color: {p.text_high}; font-size: 13px; box-sizing: border-box;",
                                r#type: "date",
                                value: "{new_start}",
                                oninput: move |evt| new_start.set(evt.value()),
                            }
                        }
                        div {
                            label {
                                style: "display: block; font-size: 12px; color: {p.text_secondary}; margin-bottom: 4px;",
                                "Date de fin"
                            }
                            input {
                                style: "width: 100%; padding: 8px 12px; background: {p.bg_base}; border: 1px solid {p.border_default}; border-radius: 4px; color: {p.text_high}; font-size: 13px; box-sizing: border-box;",
                                r#type: "date",
                                value: "{new_end}",
                                oninput: move |evt| new_end.set(evt.value()),
                            }
                        }
                    }

                    div {
                        style: "display: flex; gap: 8px; justify-content: flex-end;",

                        button {
                            style: "padding: 8px 16px; background: transparent; border: 1px solid {p.border_default}; border-radius: 4px; color: {p.text_secondary}; cursor: pointer; font-size: 13px;",
                            onclick: move |_| show_form.set(false),
                            "Annuler"
                        }
                        button {
                            style: "padding: 8px 16px; background: {p.accent_primary}; border: none; border-radius: 4px; color: white; cursor: pointer; font-size: 13px;",
                            onclick: move |_| {
                                let name = new_name.read().clone();
                                if !name.is_empty() {
                                    let slug = name.to_lowercase().replace(' ', "-");
                                    let start = {
                                        let v = new_start.read().clone();
                                        if v.is_empty() { None } else { Some(v) }
                                    };
                                    let end = {
                                        let v = new_end.read().clone();
                                        if v.is_empty() { None } else { Some(v) }
                                    };
                                    let loc = {
                                        let v = new_location.read().clone();
                                        if v.is_empty() { None } else { Some(v) }
                                    };
                                    let db = &conns.read().jayfestival;
                                    if let Ok(new_id) = db.edition_create(
                                        &name,
                                        &slug,
                                        start.as_deref(),
                                        end.as_deref(),
                                        loc.as_deref(),
                                        None,
                                        Some("brouillon"),
                                    ) {
                                        let mut s = state.write();
                                        s.selected_edition_id = Some(new_id);
                                        s.org_section = OrgSection::EditionHub;
                                        s.edition_tab = OrgEditionTab::Overview;
                                    }
                                    show_form.set(false);
                                }
                            },
                            "Creer l'edition"
                        }
                    }
                }
            }

            // Liste des éditions
            if editions.is_empty() {
                EmptyState {
                    title: "Aucune edition".to_string(),
                    message: "Creez votre premiere edition pour commencer".to_string(),
                    icon: "📅".to_string(),
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
                            onclick: {
                                let eid = edition.id.clone();
                                move |_| {
                                    let mut s = state.write();
                                    s.selected_edition_id.clone_from(&eid);
                                    s.org_section = OrgSection::EditionHub;
                                    s.edition_tab = OrgEditionTab::Overview;
                                }
                            },
                        }
                    }
                }
            }
        }
    }
}
