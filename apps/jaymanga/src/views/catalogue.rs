//! Catalogue JayManga — liste des oeuvres avec gestion CRUD.

use dioxus::prelude::*;
use miyukini_service_ui::use_palette;
use crate::use_db;
use super::components::{PageHeader, Badge, EmptyState};
use super::{JayMangaSection, JayMangaState};

#[component]
pub fn Catalogue(state: Signal<JayMangaState>) -> Element {
    let c = use_palette();
    let db = use_db();

    let works = db.work_list(&jaymanga::data::WorkFilters::default()).unwrap_or_default();
    let work_count = works.len();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 16px;",

            PageHeader {
                title: "\u{1F4D6} Catalogue".to_string(),
                subtitle: "Gerez vos oeuvres manga".to_string(),
                count: Some(work_count),
                action_label: "Nouvelle oeuvre".to_string(),
                action_icon: "\u{2795}".to_string(),
                on_action: move |_| {
                    let mut s = state.write();
                    s.editing_work_id = None;
                    s.section = JayMangaSection::NouvelleOeuvre;
                },
            }

            if works.is_empty() {
                EmptyState {
                    title: "Aucune oeuvre".to_string(),
                    message: "Ajoutez votre premiere oeuvre manga pour commencer.".to_string(),
                    icon: "\u{1F4D6}".to_string(),
                    action_label: "Creer une oeuvre".to_string(),
                    on_action: move |_| {
                        let mut s = state.write();
                        s.editing_work_id = None;
                        s.section = JayMangaSection::NouvelleOeuvre;
                    },
                }
            } else {
                // Liste des oeuvres
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
                                "published" => (c.accent_green, "Publie"),
                                "draft" => (c.text_muted, "Brouillon"),
                                "archived" => (c.accent_orange, "Archive"),
                                _ => (c.text_muted, "Inconnu"),
                            };

                            let price_display = if pricing == "free" {
                                "Gratuit".to_string()
                            } else {
                                format!("{},{:02} \u{20AC}", price_cents / 100, price_cents % 100)
                            };

                            let wid_edit = work_id.clone();
                            let wid_chapters = work_id.clone();
                            let wid_delete = work_id.clone();
                            let db_delete = db.clone();

                            rsx! {
                                div {
                                    style: "display: flex; align-items: center; gap: 16px; padding: 16px; background: {c.bg_secondary}; border-radius: 8px; border: 1px solid {c.border};",

                                    // Couverture placeholder
                                    div {
                                        style: "width: 60px; height: 80px; background: {c.bg_hover}; border-radius: 4px; display: flex; align-items: center; justify-content: center; font-size: 24px; flex-shrink: 0;",
                                        "\u{1F4D6}"
                                    }

                                    // Infos
                                    div {
                                        style: "flex: 1; min-width: 0;",

                                        div {
                                            style: "display: flex; align-items: center; gap: 8px; margin-bottom: 4px;",
                                            h4 {
                                                style: "font-size: 15px; color: {c.text_white}; font-weight: 600;",
                                                "{title}"
                                            }
                                            Badge { text: status_label.to_string(), color: status_color.to_string() }
                                        }

                                        div {
                                            style: "display: flex; gap: 16px; font-size: 12px; color: {c.text_muted};",
                                            span { "{chapter_count} chap." }
                                            span { "{total_pages} pages" }
                                            span { "{format}" }
                                            span { "{language}" }
                                            span {
                                                style: "color: {c.accent_green};",
                                                "{price_display}"
                                            }
                                        }
                                    }

                                    // Actions
                                    div {
                                        style: "display: flex; gap: 4px; flex-shrink: 0;",

                                        button {
                                            style: "padding: 6px 12px; background: {c.bg_hover}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_secondary}; cursor: pointer; font-size: 12px;",
                                            onclick: move |_| {
                                                let mut s = state.write();
                                                s.chapters_work_id = Some(wid_chapters.clone());
                                                s.section = JayMangaSection::Chapters;
                                            },
                                            "\u{1F4D1} Chapitres"
                                        }
                                        button {
                                            style: "padding: 6px 12px; background: {c.bg_hover}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_secondary}; cursor: pointer; font-size: 12px;",
                                            onclick: move |_| {
                                                let mut s = state.write();
                                                s.editing_work_id = Some(wid_edit.clone());
                                                s.section = JayMangaSection::ModifierOeuvre;
                                            },
                                            "\u{270F}\u{FE0F} Modifier"
                                        }
                                        button {
                                            style: "padding: 6px 12px; background: transparent; border: 1px solid {c.border}; border-radius: 4px; color: {c.accent_red}; cursor: pointer; font-size: 12px;",
                                            onclick: move |_| {
                                                let _ = db_delete.work_delete(&wid_delete);
                                            },
                                            "\u{1F5D1}\u{FE0F}"
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
