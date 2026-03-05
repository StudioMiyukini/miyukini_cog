//! Gestion des chapitres et pages d'une oeuvre manga.

use super::components::{ActionButton, Badge, EmptyState, PageHeader};
use super::{JayMangaSection, JayMangaState};
use crate::use_db;
use dioxus::prelude::*;
use jaymanga::data::{Chapter, Page};
use miyukini_service_ui::use_palette;

#[component]
pub fn Chapters(state: Signal<JayMangaState>) -> Element {
    let c = use_palette();
    let db = use_db();

    let work_id = state.read().chapters_work_id.clone();

    // Si aucune oeuvre selectionnee, afficher la liste des oeuvres
    if work_id.is_none() {
        let works = db
            .work_list(&jaymanga::data::WorkFilters::default())
            .unwrap_or_default();

        return rsx! {
            div {
                style: "display: flex; flex-direction: column; gap: 16px;",

                PageHeader {
                    title: "\u{1F4D1} Chapitres".to_string(),
                    subtitle: "Selectionnez une oeuvre pour gerer ses chapitres".to_string(),
                }

                if works.is_empty() {
                    EmptyState {
                        title: "Aucune oeuvre".to_string(),
                        message: "Creez d'abord une oeuvre dans le catalogue.".to_string(),
                        icon: "\u{1F4D6}".to_string(),
                        action_label: "Aller au catalogue".to_string(),
                        on_action: move |_| { state.write().section = JayMangaSection::Catalogue; },
                    }
                } else {
                    div {
                        style: "display: grid; grid-template-columns: repeat(3, 1fr); gap: 12px;",

                        for work in works.iter() {
                            {
                                let wid = work.id.clone().unwrap_or_default();
                                let title = work.title.clone().unwrap_or_else(|| "Sans titre".to_string());
                                let chapter_count = db.chapter_list_by_work(&wid).map(|v| v.len()).unwrap_or(0);

                                rsx! {
                                    button {
                                        style: "padding: 20px; background: {c.bg_secondary}; border: 1px solid {c.border}; border-radius: 8px; cursor: pointer; text-align: left; color: {c.text_primary};",
                                        onclick: move |_| {
                                            let mut s = state.write();
                                            s.chapters_work_id = Some(wid.clone());
                                        },

                                        h4 {
                                            style: "font-size: 15px; color: {c.text_white}; margin-bottom: 4px;",
                                            "\u{1F4D6} {title}"
                                        }
                                        p {
                                            style: "font-size: 12px; color: {c.text_muted};",
                                            "{chapter_count} chapitre(s)"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        };
    }

    let work_id = work_id.unwrap();
    let work = db.work_by_id(&work_id).ok().flatten();
    let work_title = work
        .as_ref()
        .and_then(|w| w.title.clone())
        .unwrap_or_else(|| "Oeuvre".to_string());
    let chapters = db.chapter_list_by_work(&work_id).unwrap_or_default();
    let chapter_count = chapters.len();

    let mut new_chapter_title = use_signal(String::new);
    let mut expanded_chapter = use_signal(|| Option::<String>::None);

    let wid_for_create = work_id.clone();
    let db_create = db.clone();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 16px;",

            // En-tete avec navigation retour
            div {
                style: "display: flex; align-items: center; gap: 12px; margin-bottom: 8px;",
                button {
                    style: "padding: 6px 12px; background: {c.bg_hover}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_secondary}; cursor: pointer; font-size: 12px;",
                    onclick: move |_| { state.write().chapters_work_id = None; },
                    "\u{2190} Toutes les oeuvres"
                }
            }

            PageHeader {
                title: format!("\u{1F4D1} {work_title}"),
                subtitle: "Gestion des chapitres".to_string(),
                count: Some(chapter_count),
            }

            // Formulaire ajout rapide
            div {
                style: "display: flex; gap: 12px; align-items: flex-end;",

                div {
                    style: "flex: 1;",
                    label {
                        style: "font-size: 12px; color: {c.text_secondary}; display: block; margin-bottom: 4px;",
                        "Titre du chapitre"
                    }
                    input {
                        style: "width: 100%; padding: 10px 12px; background: {c.bg_main}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_primary}; font-size: 13px; box-sizing: border-box;",
                        placeholder: "Titre du nouveau chapitre (optionnel)",
                        value: "{new_chapter_title}",
                        oninput: move |evt: FormEvent| { new_chapter_title.set(evt.value()); },
                    }
                }
                ActionButton {
                    label: "Ajouter un chapitre".to_string(),
                    icon: "\u{2795}".to_string(),
                    accent: true,
                    onclick: move |_| {
                        let next_number = chapter_count as i32 + 1;
                        let chapter = Chapter {
                            id: Some(uuid::Uuid::new_v4().to_string()),
                            work_id: Some(wid_for_create.clone()),
                            chapter_number: Some(next_number),
                            title: if new_chapter_title.read().is_empty() { None } else { Some(new_chapter_title.read().clone()) },
                            sort_order: Some(next_number),
                            created_at: Some(chrono::Utc::now().to_rfc3339()),
                            ..Default::default()
                        };
                        let _ = db_create.chapter_create(&chapter);
                        new_chapter_title.set(String::new());
                    },
                }
            }

            // Liste des chapitres
            if chapters.is_empty() {
                EmptyState {
                    title: "Aucun chapitre".to_string(),
                    message: "Ajoutez des chapitres a cette oeuvre.".to_string(),
                    icon: "\u{1F4D1}".to_string(),
                }
            } else {
                div {
                    style: "display: flex; flex-direction: column; gap: 8px;",

                    for chapter in chapters.iter() {
                        {
                            let chap_id = chapter.id.clone().unwrap_or_default();
                            let chap_num = chapter.chapter_number.unwrap_or(0);
                            let chap_title = chapter.title.clone().unwrap_or_default();
                            let page_count = chapter.page_count.unwrap_or(0);
                            let cid_delete = chap_id.clone();
                            let cid_expand = chap_id.clone();
                            let cid_pages = chap_id.clone();
                            let db_del = db.clone();

                            let is_expanded = expanded_chapter.read().as_deref() == Some(chap_id.as_str());

                            // Pages du chapitre
                            let pages = if is_expanded {
                                db.page_list_by_chapter(&cid_pages).unwrap_or_default()
                            } else {
                                Vec::new()
                            };

                            rsx! {
                                div {
                                    style: "background: {c.bg_secondary}; border-radius: 4px; border: 1px solid {c.border}; overflow: hidden;",

                                    // En-tete chapitre
                                    div {
                                        style: "display: flex; align-items: center; gap: 16px; padding: 12px 16px; cursor: pointer;",
                                        onclick: move |_| {
                                            let current = expanded_chapter.read().clone();
                                            if current.as_deref() == Some(cid_expand.as_str()) {
                                                expanded_chapter.set(None);
                                            } else {
                                                expanded_chapter.set(Some(cid_expand.clone()));
                                            }
                                        },

                                        // Numero
                                        div {
                                            style: "width: 40px; height: 40px; background: {c.bg_hover}; border-radius: 8px; display: flex; align-items: center; justify-content: center; font-size: 16px; font-weight: 600; color: {c.accent_blue}; flex-shrink: 0;",
                                            "{chap_num}"
                                        }

                                        // Infos
                                        div {
                                            style: "flex: 1;",
                                            div {
                                                style: "display: flex; align-items: center; gap: 8px;",
                                                p {
                                                    style: "font-size: 14px; color: {c.text_white}; font-weight: 500;",
                                                    if chap_title.is_empty() {
                                                        "Chapitre {chap_num}"
                                                    } else {
                                                        "Chapitre {chap_num} \u{2014} {chap_title}"
                                                    }
                                                }
                                                Badge { text: format!("{page_count} page(s)"), color: c.accent_blue.to_string() }
                                            }
                                        }

                                        // Chevron expand
                                        span {
                                            style: "font-size: 12px; color: {c.text_muted}; transition: transform 0.2s;",
                                            if is_expanded { "\u{25BC}" } else { "\u{25B6}" }
                                        }

                                        // Delete
                                        button {
                                            style: "padding: 6px 10px; background: transparent; border: 1px solid {c.border}; border-radius: 4px; color: {c.accent_red}; cursor: pointer; font-size: 11px;",
                                            onclick: move |evt| {
                                                evt.stop_propagation();
                                                let _ = db_del.chapter_delete(&cid_delete);
                                            },
                                            "\u{1F5D1}\u{FE0F}"
                                        }
                                    }

                                    // Pages (expanded)
                                    if is_expanded {
                                        {
                                            let cid_add_page = chap_id.clone();
                                            let page_total = pages.len();
                                            let db_add = db.clone();

                                            rsx! {
                                                div {
                                                    style: "padding: 0 16px 16px 16px; border-top: 1px solid {c.border};",

                                                    // Bouton ajouter page
                                                    div {
                                                        style: "display: flex; align-items: center; justify-content: space-between; padding: 12px 0 8px;",
                                                        span {
                                                            style: "font-size: 12px; color: {c.text_secondary}; font-weight: 500;",
                                                            "Pages ({page_total})"
                                                        }
                                                        button {
                                                            style: "padding: 4px 10px; background: {c.accent_blue}; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 11px;",
                                                            onclick: move |_| {
                                                                let next_order = page_total as i32 + 1;
                                                                let page = Page {
                                                                    id: Some(uuid::Uuid::new_v4().to_string()),
                                                                    chapter_id: Some(cid_add_page.clone()),
                                                                    page_number: Some(next_order),
                                                                    sort_order: Some(next_order),
                                                                    ..Default::default()
                                                                };
                                                                let _ = db_add.page_create(&page);
                                                            },
                                                            "\u{2795} Ajouter une page"
                                                        }
                                                    }

                                                    if pages.is_empty() {
                                                        p {
                                                            style: "font-size: 12px; color: {c.text_muted}; text-align: center; padding: 16px;",
                                                            "Aucune page \u{2014} ajoutez des pages a ce chapitre."
                                                        }
                                                    } else {
                                                        div {
                                                            style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(80px, 1fr)); gap: 8px;",

                                                            for page in pages.iter() {
                                                                {
                                                                    let pid = page.id.clone().unwrap_or_default();
                                                                    let page_num = page.page_number.unwrap_or(0);
                                                                    let has_original = page.original_image_path.is_some();
                                                                    let has_variants = page.optimized_variants.is_some();
                                                                    let pid_delete = pid.clone();
                                                                    let db_pdel = db.clone();

                                                                    rsx! {
                                                                        div {
                                                                            style: "position: relative; padding: 8px; background: {c.bg_main}; border-radius: 4px; text-align: center;",

                                                                            div {
                                                                                style: "width: 100%; aspect-ratio: 3/4; background: {c.bg_hover}; border-radius: 2px; display: flex; align-items: center; justify-content: center; font-size: 16px; margin-bottom: 4px;",
                                                                                if has_original { "\u{1F5BC}\u{FE0F}" } else { "\u{1F4C4}" }
                                                                            }
                                                                            p {
                                                                                style: "font-size: 11px; color: {c.text_secondary};",
                                                                                "P.{page_num}"
                                                                            }
                                                                            if has_variants {
                                                                                span {
                                                                                    style: "font-size: 9px; color: {c.accent_green};",
                                                                                    "\u{2713} opt"
                                                                                }
                                                                            }

                                                                            // Bouton supprimer
                                                                            button {
                                                                                style: "position: absolute; top: 2px; right: 2px; width: 18px; height: 18px; background: {c.accent_red}80; color: white; border: none; border-radius: 50%; cursor: pointer; font-size: 10px; display: flex; align-items: center; justify-content: center;",
                                                                                onclick: move |_| {
                                                                                    let _ = db_pdel.page_delete(&pid_delete);
                                                                                },
                                                                                "\u{00D7}"
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
            }
        }
    }
}
