//! Catalogue JayManga — liste des œuvres avec gestion CRUD.

use dioxus::prelude::*;
use miyuki_ui_dioxus::context::use_palette;
use crate::data::use_service_connections;
use crate::state::use_app_state;
use super::components::{PageHeader, Badge, EmptyState};
use super::{JayMangaSection, JayMangaState};

#[component]
pub fn Catalogue(state: Signal<JayMangaState>) -> Element {
    let p = use_palette();
    let conns = use_service_connections();

    let db = &conns.read().jaymanga;
    let works = db.work_list(&jaymanga::data::WorkFilters::default()).unwrap_or_default();
    let work_count = works.len();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 16px;",

            PageHeader {
                title: "📖 Catalogue".to_string(),
                subtitle: "Gérez vos œuvres manga".to_string(),
                count: Some(work_count),
                action_label: "Nouvelle œuvre".to_string(),
                action_icon: "➕".to_string(),
                on_action: move |_| {
                    let mut s = state.write();
                    s.editing_work_id = None;
                    s.section = JayMangaSection::NouvelleOeuvre;
                },
            }

            if works.is_empty() {
                EmptyState {
                    title: "Aucune œuvre".to_string(),
                    message: "Ajoutez votre première œuvre manga pour commencer.".to_string(),
                    icon: "📖".to_string(),
                    action_label: "Créer une œuvre".to_string(),
                    on_action: move |_| {
                        let mut s = state.write();
                        s.editing_work_id = None;
                        s.section = JayMangaSection::NouvelleOeuvre;
                    },
                }
            } else {
                // Liste des œuvres
                div {
                    style: "display: flex; flex-direction: column; gap: 8px;",

                    for work in works.iter() {
                        {
                            let work_id = work.id.clone().unwrap_or_default();
                            let title = work.title.clone().unwrap_or_else(|| "Sans titre".to_string());
                            let status = work.status.clone().unwrap_or_else(|| "draft".to_string());
                            let pricing = work.pricing_model.clone().unwrap_or_else(|| "free".to_string());
                            let price_cents = work.price.unwrap_or(0);
                            let total_pages = work.total_pages.unwrap_or(0);
                            let format = work.reading_format.clone().unwrap_or_else(|| "manga".to_string());
                            let language = work.language.clone().unwrap_or_else(|| "fr".to_string());
                            let chapter_count = db.chapter_list_by_work(&work_id).map(|v| v.len()).unwrap_or(0);

                            let (status_color, status_label) = match status.as_str() {
                                "published" => (p.success, "Publié"),
                                "draft" => (p.text_muted, "Brouillon"),
                                "archived" => (p.warning, "Archivé"),
                                _ => (p.text_muted, "Inconnu"),
                            };

                            let price_display = if pricing == "free" {
                                "Gratuit".to_string()
                            } else {
                                format!("{},{:02} €", price_cents / 100, price_cents % 100)
                            };

                            let wid_edit = work_id.clone();
                            let wid_chapters = work_id.clone();
                            let wid_delete = work_id.clone();

                            rsx! {
                                div {
                                    style: "display: flex; align-items: center; gap: 16px; padding: 16px; background: {p.bg_secondary}; border-radius: 8px; border: 1px solid {p.border_default};",

                                    // Couverture placeholder
                                    div {
                                        style: "width: 60px; height: 80px; background: {p.bg_overlay}; border-radius: 4px; display: flex; align-items: center; justify-content: center; font-size: 24px; flex-shrink: 0;",
                                        "📖"
                                    }

                                    // Infos
                                    div {
                                        style: "flex: 1; min-width: 0;",

                                        div {
                                            style: "display: flex; align-items: center; gap: 8px; margin-bottom: 4px;",
                                            h4 {
                                                style: "font-size: 15px; color: {p.text_high}; font-weight: 600;",
                                                "{title}"
                                            }
                                            Badge { text: status_label.to_string(), color: status_color.to_string() }
                                        }

                                        div {
                                            style: "display: flex; gap: 16px; font-size: 12px; color: {p.text_muted};",
                                            span { "{chapter_count} chap." }
                                            span { "{total_pages} pages" }
                                            span { "{format}" }
                                            span { "{language}" }
                                            span {
                                                style: "color: {p.success};",
                                                "{price_display}"
                                            }
                                        }
                                    }

                                    // Actions
                                    div {
                                        style: "display: flex; gap: 4px; flex-shrink: 0;",

                                        button {
                                            style: "padding: 6px 12px; background: {p.bg_overlay}; border: 1px solid {p.border_default}; border-radius: 4px; color: {p.text_secondary}; cursor: pointer; font-size: 12px;",
                                            onclick: move |_| {
                                                let mut s = state.write();
                                                s.chapters_work_id = Some(wid_chapters.clone());
                                                s.section = JayMangaSection::Chapters;
                                            },
                                            "📑 Chapitres"
                                        }
                                        button {
                                            style: "padding: 6px 12px; background: {p.bg_overlay}; border: 1px solid {p.border_default}; border-radius: 4px; color: {p.text_secondary}; cursor: pointer; font-size: 12px;",
                                            onclick: move |_| {
                                                let mut s = state.write();
                                                s.editing_work_id = Some(wid_edit.clone());
                                                s.section = JayMangaSection::ModifierOeuvre;
                                            },
                                            "✏️ Modifier"
                                        }
                                        button {
                                            style: "padding: 6px 12px; background: transparent; border: 1px solid {p.border_default}; border-radius: 4px; color: {p.error}; cursor: pointer; font-size: 12px;",
                                            onclick: move |_| {
                                                let db = &conns.read().jaymanga;
                                                let _ = db.work_delete(&wid_delete);
                                            },
                                            "🗑️"
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
