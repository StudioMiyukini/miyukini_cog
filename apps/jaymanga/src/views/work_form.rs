//! Editeur d'oeuvre manga multi-etapes.
//!
//! 6 etapes : Fichiers, Metadonnees, Format/Structure, Prix/Demo, Optimisation, Publication.
//! Navigation libre entre etapes. Indicateur de completion par etape.

use super::components::{ActionButton, Badge, FormField, FormSection, FormTextarea};
use super::{JayMangaSection, JayMangaState};
use crate::use_db;
use dioxus::html::HasFileData;
use dioxus::prelude::*;
use jaymanga::data::{Chapter, Page, Work};
use miyukini_service_ui::use_palette;
use std::sync::Arc;

// ── Etapes de l'editeur ──────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum EditorStep {
    #[default]
    Files,
    Metadata,
    FormatStructure,
    PricingDemo,
    Optimization,
    Publication,
}

impl EditorStep {
    fn index(&self) -> usize {
        match self {
            Self::Files => 0,
            Self::Metadata => 1,
            Self::FormatStructure => 2,
            Self::PricingDemo => 3,
            Self::Optimization => 4,
            Self::Publication => 5,
        }
    }

    fn label(&self) -> &'static str {
        match self {
            Self::Files => "Fichiers",
            Self::Metadata => "Metadonnees",
            Self::FormatStructure => "Format",
            Self::PricingDemo => "Prix",
            Self::Optimization => "Optimisation",
            Self::Publication => "Publication",
        }
    }

    fn icon(&self) -> &'static str {
        match self {
            Self::Files => "\u{1F4C1}",
            Self::Metadata => "\u{1F4DD}",
            Self::FormatStructure => "\u{1F4D0}",
            Self::PricingDemo => "\u{1F4B0}",
            Self::Optimization => "\u{26A1}",
            Self::Publication => "\u{1F680}",
        }
    }

    fn all() -> &'static [EditorStep] {
        &[
            Self::Files,
            Self::Metadata,
            Self::FormatStructure,
            Self::PricingDemo,
            Self::Optimization,
            Self::Publication,
        ]
    }
}

// ── Composant principal ──────────────────────────────────────────────────

#[component]
pub fn WorkForm(state: Signal<JayMangaState>) -> Element {
    let c = use_palette();
    let db = use_db();
    let editing_id = state.read().editing_work_id.clone();
    let is_edit = editing_id.is_some();

    // Charger l'oeuvre existante si edition
    let existing = if let Some(ref wid) = editing_id {
        db.work_by_id(wid).ok().flatten()
    } else {
        None
    };

    // Etape active
    let mut current_step = use_signal(|| {
        if is_edit {
            EditorStep::Metadata
        } else {
            EditorStep::Files
        }
    });

    // Champs du formulaire
    let mut title = use_signal(|| {
        existing
            .as_ref()
            .and_then(|w| w.title.clone())
            .unwrap_or_default()
    });
    let mut synopsis = use_signal(|| {
        existing
            .as_ref()
            .and_then(|w| w.synopsis.clone())
            .unwrap_or_default()
    });
    let mut authors = use_signal(|| {
        existing
            .as_ref()
            .and_then(|w| w.authors.clone())
            .unwrap_or_default()
    });
    let mut genres = use_signal(|| {
        existing
            .as_ref()
            .and_then(|w| w.genres.clone())
            .unwrap_or_default()
    });
    let mut language = use_signal(|| {
        existing
            .as_ref()
            .and_then(|w| w.language.clone())
            .unwrap_or_else(|| "fr".to_string())
    });
    let mut pricing_model = use_signal(|| {
        existing
            .as_ref()
            .and_then(|w| w.pricing_model.clone())
            .unwrap_or_else(|| "free".to_string())
    });
    let mut price = use_signal(|| {
        existing
            .as_ref()
            .and_then(|w| w.price)
            .unwrap_or(0)
            .to_string()
    });
    let mut reading_format = use_signal(|| {
        existing
            .as_ref()
            .and_then(|w| w.reading_format.clone())
            .unwrap_or_else(|| "manga".to_string())
    });
    let mut demo_pages = use_signal(|| {
        existing
            .as_ref()
            .and_then(|w| w.demo_pages_count)
            .unwrap_or(5)
            .to_string()
    });
    let mut tags = use_signal(|| {
        existing
            .as_ref()
            .and_then(|w| w.tags.clone())
            .unwrap_or_default()
    });
    let mut allow_download = use_signal(|| {
        existing
            .as_ref()
            .and_then(|w| w.allow_download)
            .unwrap_or(true)
    });
    let mut cover_path = use_signal(|| {
        existing
            .as_ref()
            .and_then(|w| w.cover_image_path.clone())
            .unwrap_or_default()
    });
    let mut volume_number = use_signal(|| {
        existing
            .as_ref()
            .and_then(|w| w.volume_number)
            .unwrap_or(1)
            .to_string()
    });
    let mut series_id = use_signal(|| {
        existing
            .as_ref()
            .and_then(|w| w.series_id.clone())
            .unwrap_or_default()
    });

    let work_status = existing
        .as_ref()
        .and_then(|w| w.status.clone())
        .unwrap_or_else(|| "draft".to_string());

    // Chapitres/pages existants
    let work_id_ref = editing_id.clone().unwrap_or_default();
    let chapters = if is_edit {
        db.chapter_list_by_work(&work_id_ref).unwrap_or_default()
    } else {
        Vec::new()
    };
    let total_pages: usize = chapters
        .iter()
        .map(|ch| ch.page_count.unwrap_or(0) as usize)
        .sum();

    // Indicateurs de completion par etape
    let step_complete = |step: EditorStep| -> bool {
        match step {
            EditorStep::Files => is_edit || total_pages > 0,
            EditorStep::Metadata => !title.read().is_empty(),
            EditorStep::FormatStructure => !reading_format.read().is_empty(),
            EditorStep::PricingDemo => !pricing_model.read().is_empty(),
            EditorStep::Optimization => true,
            EditorStep::Publication => {
                !title.read().is_empty() && !reading_format.read().is_empty()
            }
        }
    };

    let page_title = if is_edit {
        "\u{270F}\u{FE0F} Modifier l'oeuvre"
    } else {
        "\u{2795} Nouvelle oeuvre"
    };
    let step = *current_step.read();

    let db_draft = db.clone();
    let db_publish = db.clone();
    let editing_id_draft = editing_id.clone();
    let editing_id_publish = editing_id.clone();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 20px;",

            // ── En-tete ──────────────────────────────────────────────
            div {
                style: "display: flex; align-items: center; justify-content: space-between;",

                div {
                    style: "display: flex; align-items: center; gap: 12px;",
                    button {
                        style: "padding: 8px 12px; background: {c.bg_hover}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_secondary}; cursor: pointer; font-size: 13px;",
                        onclick: move |_| { state.write().section = JayMangaSection::Catalogue; },
                        "\u{2190} Retour"
                    }
                    h2 {
                        style: "font-size: 22px; color: {c.text_white};",
                        "{page_title}"
                    }
                    if is_edit {
                        Badge { text: work_status.clone(), color: c.accent_green.to_string() }
                    }
                }

                {
                    let completed = EditorStep::all().iter().filter(|s| step_complete(**s)).count();
                    rsx! {
                        span {
                            style: "font-size: 12px; color: {c.text_muted};",
                            "{completed}/6 etapes completees"
                        }
                    }
                }
            }

            // ── Indicateur d'etapes ──────────────────────────────────
            div {
                style: "display: flex; gap: 4px;",

                for s in EditorStep::all().iter() {
                    {
                        let is_current = *s == step;
                        let is_complete = step_complete(*s);
                        let bg = if is_current {
                            format!("{}30", c.accent_blue)
                        } else if is_complete {
                            format!("{}20", c.accent_green)
                        } else {
                            c.bg_secondary.to_string()
                        };
                        let border = if is_current {
                            format!("2px solid {}", c.accent_blue)
                        } else if is_complete {
                            format!("1px solid {}60", c.accent_green)
                        } else {
                            format!("1px solid {}", c.border)
                        };
                        let text_c = if is_current { c.text_white } else if is_complete { c.accent_green } else { c.text_muted };
                        let step_val = *s;

                        rsx! {
                            button {
                                style: "flex: 1; padding: 10px 8px; background: {bg}; border: {border}; border-radius: 6px; cursor: pointer; text-align: center;",
                                onclick: move |_| { current_step.set(step_val); },

                                p {
                                    style: "font-size: 16px; margin-bottom: 2px;",
                                    "{s.icon()}"
                                }
                                p {
                                    style: "font-size: 11px; color: {text_c}; font-weight: 500;",
                                    "{s.label()}"
                                }
                                if is_complete {
                                    span {
                                        style: "font-size: 10px; color: {c.accent_green};",
                                        "\u{2713}"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // ── Contenu de l'etape active ────────────────────────────
            match step {
                EditorStep::Files => rsx! {
                    StepFiles {
                        is_edit: is_edit,
                        chapters_count: chapters.len(),
                        total_pages: total_pages,
                        work_id: work_id_ref.clone(),
                    }
                },
                EditorStep::Metadata => rsx! {
                    FormSection { title: "\u{1F4DD} Metadonnees de l'oeuvre".to_string(),
                        div {
                            style: "display: grid; grid-template-columns: 1fr 1fr; gap: 16px;",

                            FormField {
                                label: "Titre".to_string(),
                                value: title.read().clone(),
                                placeholder: "Titre de l'oeuvre".to_string(),
                                oninput: move |evt: FormEvent| { title.set(evt.value()); },
                            }
                            FormField {
                                label: "Auteur(s)".to_string(),
                                value: authors.read().clone(),
                                placeholder: "JSON [\"Auteur A\", \"Auteur B\"]".to_string(),
                                optional: true,
                                oninput: move |evt: FormEvent| { authors.set(evt.value()); },
                            }
                        }

                        FormTextarea {
                            label: "Synopsis".to_string(),
                            value: synopsis.read().clone(),
                            placeholder: "Resume de l'oeuvre...".to_string(),
                            rows: 4,
                            optional: true,
                            oninput: move |evt: FormEvent| { synopsis.set(evt.value()); },
                        }

                        div {
                            style: "display: grid; grid-template-columns: 1fr 1fr 1fr 1fr; gap: 16px;",

                            FormField {
                                label: "Genres".to_string(),
                                value: genres.read().clone(),
                                placeholder: "[\"action\", \"fantasy\"]".to_string(),
                                optional: true,
                                oninput: move |evt: FormEvent| { genres.set(evt.value()); },
                            }
                            FormField {
                                label: "Tags".to_string(),
                                value: tags.read().clone(),
                                placeholder: "[\"shonen\"]".to_string(),
                                optional: true,
                                oninput: move |evt: FormEvent| { tags.set(evt.value()); },
                            }
                            FormField {
                                label: "Langue".to_string(),
                                value: language.read().clone(),
                                placeholder: "fr".to_string(),
                                oninput: move |evt: FormEvent| { language.set(evt.value()); },
                            }
                            FormField {
                                label: "Volume".to_string(),
                                value: volume_number.read().clone(),
                                placeholder: "1".to_string(),
                                field_type: "number".to_string(),
                                optional: true,
                                oninput: move |evt: FormEvent| { volume_number.set(evt.value()); },
                            }
                        }

                        FormField {
                            label: "Couverture (chemin image)".to_string(),
                            value: cover_path.read().clone(),
                            placeholder: "/chemin/vers/couverture.jpg".to_string(),
                            optional: true,
                            oninput: move |evt: FormEvent| { cover_path.set(evt.value()); },
                        }

                        FormField {
                            label: "Serie (ID)".to_string(),
                            value: series_id.read().clone(),
                            placeholder: "Laisser vide si hors serie".to_string(),
                            optional: true,
                            oninput: move |evt: FormEvent| { series_id.set(evt.value()); },
                        }
                    }
                },
                EditorStep::FormatStructure => rsx! {
                    FormSection { title: "\u{1F4D0} Format et structure".to_string(),
                        div {
                            style: "display: grid; grid-template-columns: 1fr 1fr; gap: 16px;",

                            FormField {
                                label: "Format de lecture".to_string(),
                                value: reading_format.read().clone(),
                                placeholder: "manga / webtoon / landscape / comics / free".to_string(),
                                oninput: move |evt: FormEvent| { reading_format.set(evt.value()); },
                            }

                            div {
                                style: "padding: 10px; background: {c.bg_main}; border-radius: 4px;",
                                p { style: "font-size: 12px; color: {c.text_secondary}; font-weight: 500; margin-bottom: 8px;", "Formats disponibles" }
                                div {
                                    style: "display: flex; flex-wrap: wrap; gap: 6px;",
                                    FormatBadge { name: "manga", description: "Droite \u{2192} Gauche (RTL)" }
                                    FormatBadge { name: "webtoon", description: "Defilement vertical" }
                                    FormatBadge { name: "landscape", description: "Plein ecran 16:9" }
                                    FormatBadge { name: "comics", description: "Gauche \u{2192} Droite (LTR)" }
                                    FormatBadge { name: "free", description: "Navigation libre" }
                                }
                            }
                        }

                        div {
                            style: "padding: 16px; background: {c.bg_main}; border-radius: 4px; margin-top: 12px;",
                            h4 { style: "font-size: 13px; color: {c.text_white}; margin-bottom: 8px;", "\u{1F4CA} Structure actuelle" }
                            div {
                                style: "display: flex; gap: 20px; font-size: 13px;",
                                span { style: "color: {c.text_secondary};", "Chapitres : " }
                                span { style: "color: {c.text_white}; font-weight: 500;", {chapters.len().to_string()} }
                                span { style: "color: {c.text_secondary}; margin-left: 16px;", "Pages totales : " }
                                span { style: "color: {c.text_white}; font-weight: 500;", "{total_pages}" }
                            }
                            p {
                                style: "font-size: 11px; color: {c.text_muted}; margin-top: 8px;",
                                "Gerez les chapitres et pages depuis l'ecran Chapitres du catalogue."
                            }
                        }
                    }
                },
                EditorStep::PricingDemo => rsx! {
                    FormSection { title: "\u{1F4B0} Prix et demonstration".to_string(),
                        div {
                            style: "display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 16px;",

                            FormField {
                                label: "Modele de prix".to_string(),
                                value: pricing_model.read().clone(),
                                placeholder: "free / paid / freemium".to_string(),
                                oninput: move |evt: FormEvent| { pricing_model.set(evt.value()); },
                            }
                            FormField {
                                label: "Prix (centimes)".to_string(),
                                value: price.read().clone(),
                                placeholder: "0".to_string(),
                                field_type: "number".to_string(),
                                oninput: move |evt: FormEvent| { price.set(evt.value()); },
                            }
                            FormField {
                                label: "Pages demo".to_string(),
                                value: demo_pages.read().clone(),
                                placeholder: "5".to_string(),
                                field_type: "number".to_string(),
                                oninput: move |evt: FormEvent| { demo_pages.set(evt.value()); },
                            }
                        }

                        div {
                            style: "display: flex; align-items: center; gap: 12px; margin-top: 12px;",
                            label {
                                style: "font-size: 12px; color: {c.text_secondary}; font-weight: 500;",
                                "Autoriser le telechargement"
                            }
                            button {
                                style: "padding: 8px 16px; background: {c.bg_main}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_primary}; cursor: pointer; font-size: 13px;",
                                onclick: move |_| { let val = *allow_download.read(); allow_download.set(!val); },
                                if *allow_download.read() { "\u{2705} Oui" } else { "\u{274C} Non" }
                            }
                        }

                        {
                            let price_val = price.read().parse::<i64>().unwrap_or(0);
                            let model = pricing_model.read().clone();
                            let display_price = if model == "free" {
                                "Gratuit".to_string()
                            } else if price_val > 0 {
                                format!("{},{:02} \u{20AC}", price_val / 100, (price_val % 100).abs())
                            } else {
                                "0,00 \u{20AC}".to_string()
                            };
                            rsx! {
                                div {
                                    style: "padding: 12px; background: {c.bg_main}; border-radius: 4px; margin-top: 12px; text-align: center;",
                                    p { style: "font-size: 11px; color: {c.text_muted};", "Apercu prix" }
                                    p { style: "font-size: 24px; color: {c.accent_green}; font-weight: 700;", "{display_price}" }
                                    p { style: "font-size: 11px; color: {c.text_muted};", "Modele : {model}" }
                                }
                            }
                        }
                    }
                },
                EditorStep::Optimization => rsx! {
                    StepOptimization { work_id: work_id_ref.clone(), is_edit: is_edit }
                },
                EditorStep::Publication => rsx! {
                    StepPublication {
                        title: title.read().clone(),
                        reading_format: reading_format.read().clone(),
                        pricing_model: pricing_model.read().clone(),
                        price_cents: price.read().parse::<i64>().unwrap_or(0),
                        chapters_count: chapters.len(),
                        total_pages: total_pages,
                        status: work_status.clone(),
                    }
                },
            }

            // ── Boutons de navigation + sauvegarde ───────────────────
            div {
                style: "display: flex; justify-content: space-between; align-items: center;",

                div {
                    style: "display: flex; gap: 8px;",

                    if step.index() > 0 {
                        ActionButton {
                            label: "\u{2190} Precedent".to_string(),
                            icon: String::new(),
                            onclick: move |_| {
                                let idx = step.index();
                                if idx > 0 {
                                    current_step.set(EditorStep::all()[idx - 1]);
                                }
                            },
                        }
                    }
                    if step.index() < 5 {
                        ActionButton {
                            label: "Suivant \u{2192}".to_string(),
                            icon: String::new(),
                            onclick: move |_| {
                                let idx = step.index();
                                if idx < 5 {
                                    current_step.set(EditorStep::all()[idx + 1]);
                                }
                            },
                        }
                    }
                }

                div {
                    style: "display: flex; gap: 12px;",

                    ActionButton {
                        label: "Annuler".to_string(),
                        icon: "\u{2715}".to_string(),
                        onclick: move |_| { state.write().section = JayMangaSection::Catalogue; },
                    }
                    ActionButton {
                        label: "Enregistrer brouillon".to_string(),
                        icon: "\u{1F4BE}".to_string(),
                        onclick: {
                            let editing_id = editing_id_draft.clone();
                            move |_| {
                                save_work(&db_draft, &editing_id, "draft",
                                    &title, &synopsis, &authors, &genres, &language,
                                    &pricing_model, &price, &reading_format, &demo_pages,
                                    &tags, &allow_download, &cover_path, &volume_number, &series_id);
                                state.write().section = JayMangaSection::Catalogue;
                            }
                        },
                    }
                    if step == EditorStep::Publication {
                        ActionButton {
                            label: "Publier".to_string(),
                            icon: "\u{1F680}".to_string(),
                            accent: true,
                            onclick: {
                                let editing_id = editing_id_publish.clone();
                                move |_| {
                                    save_work(&db_publish, &editing_id, "published",
                                        &title, &synopsis, &authors, &genres, &language,
                                        &pricing_model, &price, &reading_format, &demo_pages,
                                        &tags, &allow_download, &cover_path, &volume_number, &series_id);
                                    state.write().section = JayMangaSection::Catalogue;
                                }
                            },
                        }
                    }
                }
            }
        }
    }
}

// ── Fonction sauvegarde ──────────────────────────────────────────────────

fn save_work(
    db: &Arc<jaymanga::data::JayMangaDb>,
    editing_id: &Option<String>,
    status: &str,
    title: &Signal<String>,
    synopsis: &Signal<String>,
    authors: &Signal<String>,
    genres: &Signal<String>,
    language: &Signal<String>,
    pricing_model: &Signal<String>,
    price: &Signal<String>,
    reading_format: &Signal<String>,
    demo_pages: &Signal<String>,
    tags: &Signal<String>,
    allow_download: &Signal<bool>,
    cover_path: &Signal<String>,
    volume_number: &Signal<String>,
    series_id: &Signal<String>,
) {
    let now = chrono::Utc::now().to_rfc3339();
    let is_edit = editing_id.is_some();
    let work = Work {
        id: Some(
            editing_id
                .clone()
                .unwrap_or_else(|| uuid::Uuid::new_v4().to_string()),
        ),
        title: Some(title.read().clone()),
        synopsis: if synopsis.read().is_empty() {
            None
        } else {
            Some(synopsis.read().clone())
        },
        authors: if authors.read().is_empty() {
            None
        } else {
            Some(authors.read().clone())
        },
        genres: if genres.read().is_empty() {
            None
        } else {
            Some(genres.read().clone())
        },
        language: Some(language.read().clone()),
        pricing_model: Some(pricing_model.read().clone()),
        price: price.read().parse::<i64>().ok(),
        reading_format: Some(reading_format.read().clone()),
        demo_pages_count: demo_pages.read().parse::<i32>().ok(),
        tags: if tags.read().is_empty() {
            None
        } else {
            Some(tags.read().clone())
        },
        allow_download: Some(*allow_download.read()),
        cover_image_path: if cover_path.read().is_empty() {
            None
        } else {
            Some(cover_path.read().clone())
        },
        volume_number: volume_number.read().parse::<i32>().ok(),
        series_id: if series_id.read().is_empty() {
            None
        } else {
            Some(series_id.read().clone())
        },
        status: Some(status.to_string()),
        created_at: if is_edit { None } else { Some(now.clone()) },
        updated_at: Some(now),
        ..Default::default()
    };

    if is_edit {
        let _ = db.work_update(&work);
    } else {
        let _ = db.work_create(&work);
    }
}

// ── Etape 1 : Fichiers ──────────────────────────────────────────────────

const IMAGE_EXTENSIONS: &[&str] = &["png", "jpg", "jpeg", "webp", "gif", "bmp"];

fn is_image_file(path: &str) -> bool {
    let lower = path.to_lowercase();
    IMAGE_EXTENSIONS.iter().any(|ext| lower.ends_with(ext))
}

#[component]
fn StepFiles(is_edit: bool, chapters_count: usize, total_pages: usize, work_id: String) -> Element {
    let c = use_palette();
    let db = use_db();

    let mut selected_files: Signal<Vec<String>> = use_signal(Vec::new);
    let mut import_status: Signal<Option<String>> = use_signal(|| None);
    let mut imported_count: Signal<usize> = use_signal(|| 0);
    let mut is_drag_over: Signal<bool> = use_signal(|| false);

    let border_color = if *is_drag_over.read() {
        c.accent_blue
    } else {
        c.border
    };
    let drop_bg = if *is_drag_over.read() {
        format!("{}15", c.accent_blue)
    } else {
        c.bg_main.to_string()
    };

    let file_count = selected_files.read().len();
    let db_import = db.clone();

    rsx! {
        FormSection { title: "\u{1F4C1} Import des fichiers".to_string(),

            div {
                style: "padding: 40px; border: 2px dashed {border_color}; border-radius: 8px; text-align: center; background: {drop_bg}; transition: border-color 0.2s, background 0.2s;",

                ondragover: move |evt: DragEvent| {
                    evt.prevent_default();
                    is_drag_over.set(true);
                },
                ondragleave: move |_| {
                    is_drag_over.set(false);
                },
                ondrop: move |evt: DragEvent| {
                    evt.prevent_default();
                    is_drag_over.set(false);

                    let files = evt.files();
                    let paths: Vec<String> = files.into_iter()
                        .map(|f| f.name())
                        .filter(|f| is_image_file(f))
                        .collect();
                    if !paths.is_empty() {
                        let mut current = selected_files.write();
                        current.extend(paths);
                        current.sort();
                        current.dedup();
                    }
                },

                p { style: "font-size: 36px; margin-bottom: 12px;", "\u{1F4C1}" }
                p {
                    style: "font-size: 14px; color: {c.text_white}; font-weight: 500;",
                    if *is_drag_over.read() {
                        "Relachez pour ajouter les fichiers"
                    } else {
                        "Glissez-deposez vos fichiers ici"
                    }
                }
                p {
                    style: "font-size: 12px; color: {c.text_muted}; margin-top: 4px;",
                    "Images (PNG, JPG, WebP) ou archives (ZIP, CBZ)"
                }
                p {
                    style: "font-size: 11px; color: {c.text_muted}; margin-top: 12px;",
                    "\u{2014} ou \u{2014}"
                }

                label {
                    style: "display: inline-block; margin-top: 8px; padding: 8px 20px; background: {c.accent_blue}; color: white; border-radius: 4px; cursor: pointer; font-size: 13px;",
                    "Selectionner des fichiers"
                    input {
                        r#type: "file",
                        accept: "image/png,image/jpeg,image/webp,image/gif,.png,.jpg,.jpeg,.webp,.gif,.bmp",
                        multiple: true,
                        style: "display: none;",
                        onchange: move |evt: FormEvent| {
                            let files = evt.files();
                            let paths: Vec<String> = files.into_iter()
                                .map(|f| f.name())
                                .filter(|f| is_image_file(f))
                                .collect();
                            if !paths.is_empty() {
                                let mut current = selected_files.write();
                                current.extend(paths);
                                current.sort();
                                current.dedup();
                            }
                        },
                    }
                }
            }

            if file_count > 0 {
                div {
                    style: "margin-top: 16px; padding: 16px; background: {c.bg_main}; border-radius: 4px; border: 1px solid {c.border};",

                    div {
                        style: "display: flex; align-items: center; justify-content: space-between; margin-bottom: 12px;",
                        h4 {
                            style: "font-size: 13px; color: {c.text_white};",
                            "\u{1F4CE} {file_count} fichier(s) selectionne(s)"
                        }
                        div {
                            style: "display: flex; gap: 8px;",
                            button {
                                style: "padding: 6px 12px; background: {c.bg_hover}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_secondary}; cursor: pointer; font-size: 11px;",
                                onclick: move |_| { selected_files.write().clear(); },
                                "\u{2715} Tout retirer"
                            }
                            button {
                                style: "padding: 6px 16px; background: {c.accent_green}; border: none; border-radius: 4px; color: white; cursor: pointer; font-size: 12px; font-weight: 500;",
                                onclick: {
                                    let work_id = work_id.clone();
                                    move |_| {
                                        let files = selected_files.read().clone();
                                        if files.is_empty() { return; }

                                        let now = chrono::Utc::now().to_rfc3339();

                                        let existing_chapters = db_import.chapter_list_by_work(&work_id).unwrap_or_default();
                                        let next_ch_num = existing_chapters.iter()
                                            .filter_map(|ch| ch.chapter_number)
                                            .max()
                                            .unwrap_or(0) + 1;

                                        let chapter_id = uuid::Uuid::new_v4().to_string();
                                        let chapter = Chapter {
                                            id: Some(chapter_id.clone()),
                                            work_id: Some(work_id.clone()),
                                            chapter_number: Some(next_ch_num),
                                            title: Some(format!("Chapitre {next_ch_num}")),
                                            page_count: Some(files.len() as i32),
                                            sort_order: Some(next_ch_num),
                                            created_at: Some(now.clone()),
                                        };

                                        if let Err(e) = db_import.chapter_create(&chapter) {
                                            import_status.set(Some(format!("Erreur chapitre : {e}")));
                                            return;
                                        }

                                        let mut ok_count = 0usize;
                                        for (i, file_path) in files.iter().enumerate() {
                                            let page_id = uuid::Uuid::new_v4().to_string();
                                            let file_size = std::fs::metadata(file_path)
                                                .map(|m| m.len() as i64)
                                                .unwrap_or(0);

                                            let page = Page {
                                                id: Some(page_id),
                                                chapter_id: Some(chapter_id.clone()),
                                                page_number: Some((i + 1) as i32),
                                                original_image_path: Some(file_path.clone()),
                                                optimized_variants: None,
                                                width: None,
                                                height: None,
                                                file_size: Some(file_size),
                                                optimization_status: Some("pending".to_string()),
                                                sort_order: Some((i + 1) as i32),
                                            };

                                            if db_import.page_create(&page).is_ok() {
                                                ok_count += 1;
                                            }
                                        }

                                        let prev = *imported_count.read();
                                        imported_count.set(prev + ok_count);
                                        import_status.set(Some(format!(
                                            "\u{2705} Chapitre {next_ch_num} cree \u{2014} {ok_count}/{} page(s) importee(s)",
                                            files.len()
                                        )));
                                        selected_files.write().clear();
                                    }
                                },
                                "\u{1F4E5} Importer comme nouveau chapitre"
                            }
                        }
                    }

                    div {
                        style: "max-height: 200px; overflow-y: auto; display: flex; flex-direction: column; gap: 2px;",
                        for (idx, path) in selected_files.read().iter().enumerate() {
                            {
                                let filename = path.rsplit(['/', '\\']).next().unwrap_or(path).to_string();
                                let idx_display = idx + 1;
                                rsx! {
                                    div {
                                        style: "display: flex; align-items: center; gap: 8px; padding: 4px 8px; background: {c.bg_secondary}; border-radius: 2px; font-size: 11px;",
                                        span { style: "color: {c.text_muted}; min-width: 20px;", "{idx_display}" }
                                        span { style: "font-size: 12px;", "\u{1F5BC}\u{FE0F}" }
                                        span { style: "color: {c.text_secondary}; flex: 1; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;", "{filename}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if let Some(ref status) = *import_status.read() {
                div {
                    style: "padding: 10px 16px; background: {c.accent_green}15; border: 1px solid {c.accent_green}40; border-radius: 4px; margin-top: 12px;",
                    p { style: "font-size: 12px; color: {c.accent_green};", "{status}" }
                }
            }

            if is_edit || *imported_count.read() > 0 {
                div {
                    style: "padding: 16px; background: {c.bg_main}; border-radius: 4px; margin-top: 16px;",
                    div {
                        style: "display: flex; gap: 20px; align-items: center;",
                        span { style: "font-size: 24px;", "\u{1F4CA}" }
                        div {
                            p { style: "font-size: 13px; color: {c.text_white};", "Fichiers existants" }
                            p {
                                style: "font-size: 12px; color: {c.text_muted};",
                                "{chapters_count} chapitre(s), {total_pages} page(s) importees"
                            }
                            if *imported_count.read() > 0 {
                                {
                                    let imported_display = *imported_count.read();
                                    rsx! {
                                        p {
                                            style: "font-size: 11px; color: {c.accent_green}; margin-top: 2px;",
                                            "+ {imported_display} page(s) ajoutee(s) cette session"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            p {
                style: "font-size: 11px; color: {c.text_muted}; margin-top: 8px;",
                "Selectionnez des images dans l'ordre des pages. Un chapitre sera automatiquement cree pour chaque lot importe."
            }
        }
    }
}

// ── Etape 5 : Optimisation ──────────────────────────────────────────────

#[component]
fn StepOptimization(work_id: String, is_edit: bool) -> Element {
    let c = use_palette();
    let db = use_db();

    let opt_config = db
        .optimization_config_get()
        .unwrap_or_else(|_| jaymanga::data::OptimizationConfig::defaults());

    let quality_hd = opt_config.quality_hd.unwrap_or(85);
    let quality_sd = opt_config.quality_sd.unwrap_or(80);
    let quality_mobile = opt_config.quality_mobile.unwrap_or(75);
    let format = opt_config
        .output_format
        .clone()
        .unwrap_or_else(|| "webp".to_string());

    let mut optimized = 0usize;
    let mut pending = 0usize;
    if is_edit {
        let chapters = db.chapter_list_by_work(&work_id).unwrap_or_default();
        for ch in chapters.iter() {
            if let Some(cid) = ch.id.as_ref() {
                let pages = db.page_list_by_chapter(cid).unwrap_or_default();
                for page in pages.iter() {
                    if page.optimized_variants.is_some() {
                        optimized += 1;
                    } else {
                        pending += 1;
                    }
                }
            }
        }
    }

    rsx! {
        FormSection { title: "\u{26A1} Optimisation des images".to_string(),
            div {
                style: "display: grid; grid-template-columns: repeat(4, 1fr); gap: 16px;",

                OptConfigCard { label: "HD", value: format!("{}%", quality_hd), icon: "\u{1F5A5}\u{FE0F}" }
                OptConfigCard { label: "SD", value: format!("{}%", quality_sd), icon: "\u{1F4F1}" }
                OptConfigCard { label: "Mobile", value: format!("{}%", quality_mobile), icon: "\u{1F4F2}" }
                OptConfigCard { label: "Format", value: format, icon: "\u{1F5BC}\u{FE0F}" }
            }

            if is_edit {
                div {
                    style: "padding: 20px; background: {c.bg_main}; border-radius: 8px; margin-top: 16px;",

                    h4 { style: "font-size: 14px; color: {c.text_white}; margin-bottom: 12px;", "Etat de l'optimisation" }

                    div {
                        style: "display: flex; gap: 20px; align-items: center;",

                        div {
                            style: "flex: 1;",
                            {
                                let total = optimized + pending;
                                let pct = if total > 0 { optimized * 100 / total } else { 0 };
                                rsx! {
                                    div {
                                        style: "width: 100%; height: 8px; background: {c.bg_hover}; border-radius: 4px; overflow: hidden;",
                                        div {
                                            style: "width: {pct}%; height: 100%; background: {c.accent_green}; border-radius: 4px;",
                                        }
                                    }
                                    div {
                                        style: "display: flex; justify-content: space-between; margin-top: 4px; font-size: 11px; color: {c.text_muted};",
                                        span { "{optimized} optimisees" }
                                        span { "{pending} en attente" }
                                    }
                                }
                            }
                        }

                        if pending > 0 {
                            button {
                                style: "padding: 8px 16px; background: {c.accent_blue}; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 12px;",
                                "\u{26A1} Re-optimiser"
                            }
                        }
                    }
                }
            }

            p {
                style: "font-size: 11px; color: {c.text_muted}; margin-top: 8px;",
                "Les parametres de compression sont configures dans l'ecran Boutique."
            }
        }
    }
}

// ── Etape 6 : Publication ───────────────────────────────────────────────

#[component]
fn StepPublication(
    title: String,
    reading_format: String,
    pricing_model: String,
    price_cents: i64,
    chapters_count: usize,
    total_pages: usize,
    status: String,
) -> Element {
    let c = use_palette();

    let price_display = if pricing_model == "free" {
        "Gratuit".to_string()
    } else if price_cents > 0 {
        format!(
            "{},{:02} \u{20AC}",
            price_cents / 100,
            (price_cents % 100).abs()
        )
    } else {
        "0,00 \u{20AC}".to_string()
    };

    let ready = !title.is_empty() && chapters_count > 0 && total_pages > 0;

    rsx! {
        FormSection { title: "\u{1F680} Publication".to_string(),
            div {
                style: "padding: 24px; background: {c.bg_main}; border-radius: 8px;",

                h3 {
                    style: "font-size: 20px; color: {c.text_white}; margin-bottom: 16px; text-align: center;",
                    "\u{1F4D6} {title}"
                }

                div {
                    style: "display: grid; grid-template-columns: repeat(4, 1fr); gap: 16px; text-align: center;",

                    div {
                        p { style: "font-size: 11px; color: {c.text_muted};", "Format" }
                        p { style: "font-size: 14px; color: {c.text_white}; font-weight: 500;", "{reading_format}" }
                    }
                    div {
                        p { style: "font-size: 11px; color: {c.text_muted};", "Chapitres" }
                        p { style: "font-size: 14px; color: {c.text_white}; font-weight: 500;", "{chapters_count}" }
                    }
                    div {
                        p { style: "font-size: 11px; color: {c.text_muted};", "Pages" }
                        p { style: "font-size: 14px; color: {c.text_white}; font-weight: 500;", "{total_pages}" }
                    }
                    div {
                        p { style: "font-size: 11px; color: {c.text_muted};", "Prix" }
                        p { style: "font-size: 14px; color: {c.accent_green}; font-weight: 500;", "{price_display}" }
                    }
                }
            }

            div {
                style: "display: flex; align-items: center; justify-content: center; gap: 8px; margin-top: 16px;",
                span { style: "font-size: 13px; color: {c.text_secondary};", "Statut actuel :" }
                Badge { text: status, color: c.accent_green.to_string() }
            }

            if ready {
                div {
                    style: "padding: 12px; background: {c.accent_green}15; border: 1px solid {c.accent_green}40; border-radius: 4px; margin-top: 12px; text-align: center;",
                    p { style: "font-size: 14px; color: {c.accent_green}; font-weight: 600;", "\u{2705} Pret pour publication" }
                    p { style: "font-size: 12px; color: {c.text_secondary}; margin-top: 4px;", "Cliquez sur Publier pour rendre cette oeuvre disponible." }
                }
            } else {
                div {
                    style: "padding: 12px; background: {c.accent_orange}15; border: 1px solid {c.accent_orange}40; border-radius: 4px; margin-top: 12px; text-align: center;",
                    p { style: "font-size: 14px; color: {c.accent_orange}; font-weight: 600;", "\u{26A0}\u{FE0F} Etapes manquantes" }
                    if title.is_empty() { p { style: "font-size: 12px; color: {c.text_muted};", "\u{2014} Titre requis" } }
                    if chapters_count == 0 { p { style: "font-size: 12px; color: {c.text_muted};", "\u{2014} Au moins un chapitre requis" } }
                    if total_pages == 0 { p { style: "font-size: 12px; color: {c.text_muted};", "\u{2014} Au moins une page requise" } }
                }
            }
        }
    }
}

// ── Composants utilitaires ──────────────────────────────────────────────

#[component]
fn FormatBadge(name: &'static str, description: &'static str) -> Element {
    let c = use_palette();

    rsx! {
        div {
            style: "padding: 4px 10px; background: {c.bg_hover}; border-radius: 4px; font-size: 11px;",
            title: "{description}",
            span { style: "color: {c.text_white}; font-weight: 500;", "{name}" }
            span { style: "color: {c.text_muted}; margin-left: 4px;", "\u{2014} {description}" }
        }
    }
}

#[component]
fn OptConfigCard(label: &'static str, value: String, icon: &'static str) -> Element {
    let c = use_palette();

    rsx! {
        div {
            style: "padding: 16px; background: {c.bg_main}; border-radius: 4px; text-align: center;",
            span { style: "font-size: 24px;", "{icon}" }
            p { style: "font-size: 11px; color: {c.text_muted}; margin-top: 4px;", "{label}" }
            p { style: "font-size: 16px; color: {c.text_white}; font-weight: 600;", "{value}" }
        }
    }
}
