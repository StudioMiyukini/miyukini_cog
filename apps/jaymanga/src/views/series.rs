//! Gestion des series JayManga — regroupement d'oeuvres en series.

use dioxus::prelude::*;
use jaymanga::data::Series;
use miyukini_service_ui::use_palette;
use crate::use_db;
use super::components::{PageHeader, EmptyState, ActionButton, FormField, FormTextarea, FormSection, Badge};
use super::JayMangaState;

#[component]
pub fn SeriesView(state: Signal<JayMangaState>) -> Element {
    let c = use_palette();
    let db = use_db();

    let series_list = db.series_list().unwrap_or_default();
    let works = db.work_list(&jaymanga::data::WorkFilters::default()).unwrap_or_default();

    let mut show_form = use_signal(|| false);
    let mut editing_series_id = use_signal(|| Option::<String>::None);

    // Champs formulaire
    let mut series_title = use_signal(String::new);
    let mut series_synopsis = use_signal(String::new);

    let db_save = db.clone();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 20px;",

            PageHeader {
                title: "\u{1F4DA} Series".to_string(),
                subtitle: "Regroupement d'oeuvres en series".to_string(),
                count: Some(series_list.len()),
                action_label: "Nouvelle serie".to_string(),
                action_icon: "\u{2795}".to_string(),
                on_action: move |_| {
                    series_title.set(String::new());
                    series_synopsis.set(String::new());
                    editing_series_id.set(None);
                    show_form.set(true);
                },
            }

            // Formulaire de creation/edition
            if *show_form.read() {
                FormSection { title: if editing_series_id.read().is_some() { "Modifier la serie".to_string() } else { "Nouvelle serie".to_string() },
                    div {
                        style: "display: grid; grid-template-columns: 1fr; gap: 16px;",

                        FormField {
                            label: "Titre de la serie".to_string(),
                            value: series_title.read().clone(),
                            placeholder: "Ex : Dragon Quest Saga".to_string(),
                            oninput: move |evt: FormEvent| { series_title.set(evt.value()); },
                        }
                    }

                    FormTextarea {
                        label: "Description".to_string(),
                        value: series_synopsis.read().clone(),
                        placeholder: "Description de la serie...".to_string(),
                        rows: 3,
                        optional: true,
                        oninput: move |evt: FormEvent| { series_synopsis.set(evt.value()); },
                    }

                    div {
                        style: "display: flex; gap: 12px; justify-content: flex-end;",

                        ActionButton {
                            label: "Annuler".to_string(),
                            icon: "\u{2715}".to_string(),
                            onclick: move |_| { show_form.set(false); },
                        }
                        ActionButton {
                            label: if editing_series_id.read().is_some() { "Enregistrer".to_string() } else { "Creer".to_string() },
                            icon: "\u{1F4BE}".to_string(),
                            accent: true,
                            onclick: move |_| {
                                let now = chrono::Utc::now().to_rfc3339();

                                let series = Series {
                                    id: Some(editing_series_id.read().clone().unwrap_or_else(|| uuid::Uuid::new_v4().to_string())),
                                    title: Some(series_title.read().clone()),
                                    synopsis: if series_synopsis.read().is_empty() { None } else { Some(series_synopsis.read().clone()) },
                                    created_at: Some(now.clone()),
                                    updated_at: Some(now),
                                    ..Default::default()
                                };

                                if editing_series_id.read().is_some() {
                                    let _ = db_save.series_update(&series);
                                } else {
                                    let _ = db_save.series_create(&series);
                                }
                                show_form.set(false);
                            },
                        }
                    }
                }
            }

            // Liste des series
            if series_list.is_empty() && !*show_form.read() {
                EmptyState {
                    title: "Aucune serie".to_string(),
                    message: "Creez des series pour regrouper vos oeuvres.".to_string(),
                    icon: "\u{1F4DA}".to_string(),
                    action_label: "Creer une serie".to_string(),
                    on_action: move |_| { show_form.set(true); },
                }
            } else {
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",

                    for series in series_list.iter() {
                        {
                            let sid = series.id.clone().unwrap_or_default();
                            let title = series.title.clone().unwrap_or_else(|| "Sans titre".to_string());
                            let description = series.synopsis.clone().unwrap_or_default();
                            let work_count_in_series = works.iter()
                                .filter(|w| w.series_id.as_deref() == Some(sid.as_str()))
                                .count();
                            let series_works: Vec<_> = works.iter()
                                .filter(|w| w.series_id.as_deref() == Some(sid.as_str()))
                                .collect();
                            let status = series.status.clone().unwrap_or_else(|| "active".to_string());

                            let sid_edit = sid.clone();
                            let title_edit = title.clone();
                            let desc_edit = description.clone();

                            rsx! {
                                div {
                                    style: "background: {c.bg_secondary}; border-radius: 8px; padding: 20px; border: 1px solid {c.border};",

                                    div {
                                        style: "display: flex; align-items: center; justify-content: space-between; margin-bottom: 12px;",

                                        div {
                                            style: "display: flex; align-items: center; gap: 12px;",
                                            span { style: "font-size: 24px;", "\u{1F4DA}" }
                                            div {
                                                div {
                                                    style: "display: flex; align-items: center; gap: 8px;",
                                                    h3 {
                                                        style: "font-size: 16px; color: {c.text_white}; font-weight: 600;",
                                                        "{title}"
                                                    }
                                                    Badge { text: format!("{work_count_in_series} oeuvre(s)"), color: c.accent_blue.to_string() }
                                                    Badge { text: status.clone(), color: c.accent_green.to_string() }
                                                }
                                                if !description.is_empty() {
                                                    p {
                                                        style: "font-size: 12px; color: {c.text_secondary}; margin-top: 2px;",
                                                        "{description}"
                                                    }
                                                }
                                            }
                                        }

                                        button {
                                            style: "padding: 6px 12px; background: transparent; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_secondary}; cursor: pointer; font-size: 12px;",
                                            onclick: move |_| {
                                                series_title.set(title_edit.clone());
                                                series_synopsis.set(desc_edit.clone());
                                                editing_series_id.set(Some(sid_edit.clone()));
                                                show_form.set(true);
                                            },
                                            "\u{270F}\u{FE0F} Modifier"
                                        }
                                    }

                                    // Oeuvres de la serie
                                    if !series_works.is_empty() {
                                        div {
                                            style: "display: flex; gap: 8px; overflow-x: auto; padding-top: 8px; border-top: 1px solid {c.border};",

                                            for work in series_works.iter() {
                                                {
                                                    let wtitle = work.title.clone().unwrap_or_else(|| "Sans titre".to_string());
                                                    let wstatus = work.status.clone().unwrap_or_else(|| "draft".to_string());
                                                    let status_color = match wstatus.as_str() {
                                                        "published" => c.accent_green,
                                                        "draft" => c.text_muted,
                                                        "archived" => c.accent_orange,
                                                        _ => c.text_muted,
                                                    };

                                                    rsx! {
                                                        div {
                                                            style: "min-width: 120px; padding: 12px; background: {c.bg_main}; border-radius: 4px; text-align: center;",
                                                            div {
                                                                style: "width: 60px; height: 80px; background: {c.bg_hover}; border-radius: 4px; margin: 0 auto 8px; display: flex; align-items: center; justify-content: center; font-size: 20px;",
                                                                "\u{1F4D6}"
                                                            }
                                                            p {
                                                                style: "font-size: 11px; color: {c.text_white}; white-space: nowrap; overflow: hidden; text-overflow: ellipsis;",
                                                                "{wtitle}"
                                                            }
                                                            span {
                                                                style: "font-size: 10px; color: {status_color};",
                                                                "{wstatus}"
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
            }
        }
    }
}
