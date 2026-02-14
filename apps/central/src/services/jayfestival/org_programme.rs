//! ORG-E17 — Programme et animations d'une édition.

use dioxus::prelude::*;
use crate::data::use_service_connections;
use crate::state::use_app_state;
use super::components::Badge;

#[component]
pub fn OrgProgramme(edition_id: String) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let conns = use_service_connections();
    let mut show_form = use_signal(|| false);
    let mut new_name = use_signal(String::new);
    let mut new_type = use_signal(|| "conference".to_string());
    let mut new_room = use_signal(String::new);
    let mut new_start = use_signal(String::new);
    let mut new_end = use_signal(String::new);
    let mut new_desc = use_signal(String::new);

    // Charger les animations
    let animations = {
        let db = &conns.read().jayfestival;
        db.animations_by_edition(&edition_id).unwrap_or_default()
    };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 20px;",

            // En-tête
            div {
                style: "display: flex; justify-content: space-between; align-items: center;",

                h3 {
                    style: "font-size: 18px; color: {c.text_white};",
                    "Programme ({animations.len()} animations)"
                }

                button {
                    style: "padding: 8px 16px; background: {c.accent_blue}; border: none; border-radius: 4px; color: white; cursor: pointer; font-size: 13px;",
                    onclick: move |_| show_form.set(!show_form()),
                    "➕ Ajouter une animation"
                }
            }

            // Formulaire d'ajout
            if *show_form.read() {
                div {
                    style: "background: {c.bg_secondary}; border-radius: 8px; padding: 20px; border: 1px solid {c.border};",

                    h4 {
                        style: "font-size: 14px; color: {c.text_white}; margin-bottom: 12px;",
                        "Nouvelle animation"
                    }

                    div {
                        style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px; margin-bottom: 16px;",

                        div {
                            label {
                                style: "display: block; font-size: 12px; color: {c.text_secondary}; margin-bottom: 4px;",
                                "Nom *"
                            }
                            input {
                                style: "width: 100%; padding: 8px 12px; background: {c.bg_main}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_white}; font-size: 13px; box-sizing: border-box;",
                                r#type: "text",
                                placeholder: "Nom de l'animation",
                                value: "{new_name}",
                                oninput: move |evt| new_name.set(evt.value()),
                            }
                        }
                        div {
                            label {
                                style: "display: block; font-size: 12px; color: {c.text_secondary}; margin-bottom: 4px;",
                                "Type"
                            }
                            select {
                                style: "width: 100%; padding: 8px 12px; background: {c.bg_main}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_white}; font-size: 13px; box-sizing: border-box;",
                                value: "{new_type}",
                                onchange: move |evt| new_type.set(evt.value()),

                                option { value: "conference", "Conference" }
                                option { value: "atelier", "Atelier" }
                                option { value: "spectacle", "Spectacle" }
                                option { value: "demo", "Demo" }
                                option { value: "autre", "Autre" }
                            }
                        }
                        div {
                            label {
                                style: "display: block; font-size: 12px; color: {c.text_secondary}; margin-bottom: 4px;",
                                "Salle"
                            }
                            input {
                                style: "width: 100%; padding: 8px 12px; background: {c.bg_main}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_white}; font-size: 13px; box-sizing: border-box;",
                                r#type: "text",
                                placeholder: "Ex: Salle A",
                                value: "{new_room}",
                                oninput: move |evt| new_room.set(evt.value()),
                            }
                        }
                        div {
                            label {
                                style: "display: block; font-size: 12px; color: {c.text_secondary}; margin-bottom: 4px;",
                                "Description"
                            }
                            input {
                                style: "width: 100%; padding: 8px 12px; background: {c.bg_main}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_white}; font-size: 13px; box-sizing: border-box;",
                                r#type: "text",
                                placeholder: "Description courte",
                                value: "{new_desc}",
                                oninput: move |evt| new_desc.set(evt.value()),
                            }
                        }
                        div {
                            label {
                                style: "display: block; font-size: 12px; color: {c.text_secondary}; margin-bottom: 4px;",
                                "Debut"
                            }
                            input {
                                style: "width: 100%; padding: 8px 12px; background: {c.bg_main}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_white}; font-size: 13px; box-sizing: border-box;",
                                r#type: "datetime-local",
                                value: "{new_start}",
                                oninput: move |evt| new_start.set(evt.value()),
                            }
                        }
                        div {
                            label {
                                style: "display: block; font-size: 12px; color: {c.text_secondary}; margin-bottom: 4px;",
                                "Fin"
                            }
                            input {
                                style: "width: 100%; padding: 8px 12px; background: {c.bg_main}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_white}; font-size: 13px; box-sizing: border-box;",
                                r#type: "datetime-local",
                                value: "{new_end}",
                                oninput: move |evt| new_end.set(evt.value()),
                            }
                        }
                    }

                    div {
                        style: "display: flex; gap: 8px; justify-content: flex-end;",

                        button {
                            style: "padding: 8px 16px; background: transparent; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_secondary}; cursor: pointer; font-size: 13px;",
                            onclick: move |_| show_form.set(false),
                            "Annuler"
                        }
                        button {
                            style: "padding: 8px 16px; background: {c.accent_blue}; border: none; border-radius: 4px; color: white; cursor: pointer; font-size: 13px;",
                            onclick: {
                                let eid = edition_id.clone();
                                move |_| {
                                    let name = new_name.read().clone();
                                    if !name.is_empty() {
                                        let atype = new_type.read().clone();
                                        let start = { let v = new_start.read().clone(); if v.is_empty() { None } else { Some(v) } };
                                        let end = { let v = new_end.read().clone(); if v.is_empty() { None } else { Some(v) } };
                                        let room = { let v = new_room.read().clone(); if v.is_empty() { None } else { Some(v) } };
                                        let desc = { let v = new_desc.read().clone(); if v.is_empty() { None } else { Some(v) } };
                                        let db = &conns.read().jayfestival;
                                        let _ = db.animation_create(
                                            &eid,
                                            &name,
                                            Some(atype.as_str()),
                                            start.as_deref(),
                                            end.as_deref(),
                                            room.as_deref(),
                                            desc.as_deref(),
                                        );
                                        show_form.set(false);
                                        // Reset form
                                        new_name.set(String::new());
                                        new_room.set(String::new());
                                        new_start.set(String::new());
                                        new_end.set(String::new());
                                        new_desc.set(String::new());
                                    }
                                }
                            },
                            "Ajouter"
                        }
                    }
                }
            }

            // Liste des animations
            if animations.is_empty() {
                div {
                    style: "display: flex; flex-direction: column; align-items: center; justify-content: center; height: 200px; color: {c.text_muted}; background: {c.bg_secondary}; border-radius: 8px;",

                    span { style: "font-size: 48px; margin-bottom: 12px; opacity: 0.3;", "🎭" }
                    p { style: "font-size: 14px;", "Aucune animation programmee" }
                }
            } else {
                div {
                    style: "display: flex; flex-direction: column; gap: 8px;",

                    for anim in animations.iter() {
                        {
                            let anim_id = anim.id.clone().unwrap_or_default();
                            let anim_name = anim.name.clone().unwrap_or_else(|| "Sans nom".to_string());
                            let anim_type = anim.animation_type.clone().unwrap_or_default();
                            let anim_room = anim.room.clone().unwrap_or_else(|| "-".to_string());
                            let anim_start = anim.start_time.clone().unwrap_or_default();
                            let anim_end = anim.end_time.clone().unwrap_or_default();
                            let anim_status = anim.status.clone().unwrap_or_else(|| "brouillon".to_string());
                            let status_color = match anim_status.as_str() {
                                "confirme" => c.accent_green,
                                "annule" => c.accent_orange,
                                _ => c.text_secondary,
                            };
                            let delete_id = anim_id.clone();
                            let conns_del = conns;
                            rsx! {
                                div {
                                    style: "display: flex; align-items: center; gap: 12px; background: {c.bg_secondary}; border-radius: 8px; padding: 12px 16px;",

                                    div {
                                        style: "flex: 1;",
                                        div {
                                            style: "display: flex; align-items: center; gap: 8px;",
                                            span {
                                                style: "color: {c.text_white}; font-size: 14px; font-weight: 500;",
                                                "{anim_name}"
                                            }
                                            Badge {
                                                text: anim_type.clone(),
                                                color: c.accent_blue.to_string(),
                                            }
                                        }
                                        div {
                                            style: "font-size: 12px; color: {c.text_secondary}; margin-top: 4px;",
                                            span { "📍 {anim_room}" }
                                            if !anim_start.is_empty() {
                                                span { " | 🕐 {anim_start}" }
                                            }
                                            if !anim_end.is_empty() {
                                                span { " → {anim_end}" }
                                            }
                                        }
                                    }

                                    Badge {
                                        text: anim_status.clone(),
                                        color: status_color.to_string(),
                                    }

                                    button {
                                        style: "padding: 6px 10px; background: transparent; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_muted}; cursor: pointer; font-size: 11px;",
                                        onclick: move |_| {
                                            let db = &conns_del.read().jayfestival;
                                            let _ = db.animation_delete(&delete_id);
                                        },
                                        "🗑"
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
