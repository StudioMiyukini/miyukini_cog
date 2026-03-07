//! Gestion des séries JayManga — regroupement d'œuvres en séries.

use dioxus::prelude::*;
use miyuki_ui_dioxus::context::use_palette;
use jaymanga::data::Series;
use crate::data::use_service_connections;
use crate::state::use_app_state;
use super::components::{PageHeader, EmptyState, ActionButton, FormField, FormTextarea, FormSection, Badge};
use super::JayMangaState;

#[component]
pub fn SeriesView(state: Signal<JayMangaState>) -> Element {
    let p = use_palette();
    let conns = use_service_connections();

    let db = &conns.read().jaymanga;
    let series_list = db.series_list().unwrap_or_default();
    let works = db.work_list(&jaymanga::data::WorkFilters::default()).unwrap_or_default();

    let mut show_form = use_signal(|| false);
    let mut editing_series_id = use_signal(|| Option::<String>::None);

    // Champs formulaire
    let mut series_title = use_signal(String::new);
    let mut series_synopsis = use_signal(String::new);

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 20px;",

            PageHeader {
                title: "📚 Séries".to_string(),
                subtitle: "Regroupement d'œuvres en séries".to_string(),
                count: Some(series_list.len()),
                action_label: "Nouvelle série".to_string(),
                action_icon: "➕".to_string(),
                on_action: move |_| {
                    series_title.set(String::new());
                    series_synopsis.set(String::new());
                    editing_series_id.set(None);
                    show_form.set(true);
                },
            }

            // Formulaire de création/édition
            if *show_form.read() {
                FormSection { title: if editing_series_id.read().is_some() { "Modifier la série".to_string() } else { "Nouvelle série".to_string() },
                    div {
                        style: "display: grid; grid-template-columns: 1fr; gap: 16px;",

                        FormField {
                            label: "Titre de la série".to_string(),
                            value: series_title.read().clone(),
                            placeholder: "Ex : Dragon Quest Saga".to_string(),
                            oninput: move |evt: FormEvent| { series_title.set(evt.value()); },
                        }
                    }

                    FormTextarea {
                        label: "Description".to_string(),
                        value: series_synopsis.read().clone(),
                        placeholder: "Description de la série...".to_string(),
                        rows: 3,
                        optional: true,
                        oninput: move |evt: FormEvent| { series_synopsis.set(evt.value()); },
                    }

                    div {
                        style: "display: flex; gap: 12px; justify-content: flex-end;",

                        ActionButton {
                            label: "Annuler".to_string(),
                            icon: "✕".to_string(),
                            onclick: move |_| { show_form.set(false); },
                        }
                        ActionButton {
                            label: if editing_series_id.read().is_some() { "Enregistrer".to_string() } else { "Créer".to_string() },
                            icon: "💾".to_string(),
                            accent: true,
                            onclick: move |_| {
                                let now = chrono::Utc::now().to_rfc3339();
                                let db = &conns.read().jaymanga;

                                let series = Series {
                                    id: Some(editing_series_id.read().clone().unwrap_or_else(|| uuid::Uuid::new_v4().to_string())),
                                    title: Some(series_title.read().clone()),
                                    synopsis: if series_synopsis.read().is_empty() { None } else { Some(series_synopsis.read().clone()) },
                                    created_at: Some(now.clone()),
                                    updated_at: Some(now),
                                    ..Default::default()
                                };

                                if editing_series_id.read().is_some() {
                                    let _ = db.series_update(&series);
                                } else {
                                    let _ = db.series_create(&series);
                                }
                                show_form.set(false);
                            },
                        }
                    }
                }
            }

            // Liste des séries
            if series_list.is_empty() && !*show_form.read() {
                EmptyState {
                    title: "Aucune série".to_string(),
                    message: "Créez des séries pour regrouper vos œuvres.".to_string(),
                    icon: "📚".to_string(),
                    action_label: "Créer une série".to_string(),
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
                                    style: "background: {p.bg_secondary}; border-radius: 8px; padding: 20px; border: 1px solid {p.border_default};",

                                    div {
                                        style: "display: flex; align-items: center; justify-content: space-between; margin-bottom: 12px;",

                                        div {
                                            style: "display: flex; align-items: center; gap: 12px;",
                                            span { style: "font-size: 24px;", "📚" }
                                            div {
                                                div {
                                                    style: "display: flex; align-items: center; gap: 8px;",
                                                    h3 {
                                                        style: "font-size: 16px; color: {p.text_high}; font-weight: 600;",
                                                        "{title}"
                                                    }
                                                    Badge { text: format!("{work_count_in_series} œuvre(s)"), color: p.accent_primary.to_string() }
                                                    Badge { text: status.clone(), color: p.success.to_string() }
                                                }
                                                if !description.is_empty() {
                                                    p {
                                                        style: "font-size: 12px; color: {p.text_secondary}; margin-top: 2px;",
                                                        "{description}"
                                                    }
                                                }
                                            }
                                        }

                                        button {
                                            style: "padding: 6px 12px; background: transparent; border: 1px solid {p.border_default}; border-radius: 4px; color: {p.text_secondary}; cursor: pointer; font-size: 12px;",
                                            onclick: move |_| {
                                                series_title.set(title_edit.clone());
                                                series_synopsis.set(desc_edit.clone());
                                                editing_series_id.set(Some(sid_edit.clone()));
                                                show_form.set(true);
                                            },
                                            "✏️ Modifier"
                                        }
                                    }

                                    // Œuvres de la série
                                    if !series_works.is_empty() {
                                        div {
                                            style: "display: flex; gap: 8px; overflow-x: auto; padding-top: 8px; border-top: 1px solid {p.border_default};",

                                            for work in series_works.iter() {
                                                {
                                                    let wtitle = work.title.clone().unwrap_or_else(|| "Sans titre".to_string());
                                                    let wstatus = work.status.clone().unwrap_or_else(|| "draft".to_string());
                                                    let status_color = match wstatus.as_str() {
                                                        "published" => p.success,
                                                        "draft" => p.text_muted,
                                                        "archived" => p.warning,
                                                        _ => p.text_muted,
                                                    };

                                                    rsx! {
                                                        div {
                                                            style: "min-width: 120px; padding: 12px; background: {p.bg_base}; border-radius: 4px; text-align: center;",
                                                            div {
                                                                style: "width: 60px; height: 80px; background: {p.bg_overlay}; border-radius: 4px; margin: 0 auto 8px; display: flex; align-items: center; justify-content: center; font-size: 20px;",
                                                                "📖"
                                                            }
                                                            p {
                                                                style: "font-size: 11px; color: {p.text_high}; white-space: nowrap; overflow: hidden; text-overflow: ellipsis;",
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
