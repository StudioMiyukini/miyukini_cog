//! Liseuse native JayManga — affichage des pages, navigation,
//! progression, XP temps reel, raccourcis clavier.
//!
//! Formats supportes : manga (RTL), webtoon (scroll vertical),
//! landscape (16:9), comics (LTR), free.

use dioxus::prelude::*;
use jaymanga::domain::reader::{
    reader_chapter_progress, reader_is_demo_page,
    reader_next_page_index, reader_prev_page_index,
};
use jaymanga::domain::gamification::XP_PER_PAGE;
use miyukini_service_ui::use_palette;
use crate::use_db;
use super::components::Badge;
use super::{JayMangaSection, JayMangaState};

// -- Etat interne de la liseuse --

/// Mode de lecture actif.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ReadingMode {
    /// Manga — navigation droite->gauche (RTL), page par page.
    Manga,
    /// Webtoon — defilement vertical continu.
    Webtoon,
    /// Landscape — images plein ecran 16:9.
    Landscape,
    /// Comics — navigation gauche->droite (LTR).
    Comics,
    /// Free — navigation libre.
    Free,
}

impl ReadingMode {
    fn from_str(s: &str) -> Self {
        match s {
            "webtoon" | "manhwa" => Self::Webtoon,
            "landscape" => Self::Landscape,
            "comics" => Self::Comics,
            "free" => Self::Free,
            _ => Self::Manga,
        }
    }

    fn label(&self) -> &'static str {
        match self {
            Self::Manga => "Manga (RTL)",
            Self::Webtoon => "Webtoon (\u{2195})",
            Self::Landscape => "Paysage (16:9)",
            Self::Comics => "Comics (LTR)",
            Self::Free => "Libre",
        }
    }

    fn is_rtl(&self) -> bool {
        matches!(self, Self::Manga)
    }

    fn is_vertical_scroll(&self) -> bool {
        matches!(self, Self::Webtoon)
    }
}

// -- Composant principal --

#[component]
pub fn Reader(state: Signal<JayMangaState>) -> Element {
    let c = use_palette();
    let db = use_db();

    let work_id = state.read().reader_work_id.clone();
    let chapter_id_init = state.read().reader_chapter_id.clone();

    // Si pas d'oeuvre selectionnee, retourner a la bibliotheque
    if work_id.is_none() {
        return rsx! {
            div {
                style: "display: flex; align-items: center; justify-content: center; height: 100%; color: {c.text_muted};",
                "Aucune oeuvre selectionnee."
            }
        };
    }

    let work_id = work_id.unwrap();

    // Charger les donnees de l'oeuvre
    let work = db.work_by_id(&work_id).ok().flatten();

    if work.is_none() {
        return rsx! {
            div {
                style: "display: flex; align-items: center; justify-content: center; height: 100%; color: {c.text_muted};",
                "Oeuvre introuvable."
            }
        };
    }

    let work = work.unwrap();
    let work_title = work.title.clone().unwrap_or_else(|| "Sans titre".to_string());
    let reading_format = work.reading_format.clone().unwrap_or_else(|| "manga".to_string());
    let demo_pages = work.demo_pages_count.unwrap_or(5);

    // Chapitres et pages
    let chapters = db.chapter_list_by_work(&work_id).unwrap_or_default();

    if chapters.is_empty() {
        return rsx! {
            div {
                style: "display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100%; gap: 12px;",
                p { style: "color: {c.text_muted}; font-size: 16px;", "Aucun chapitre disponible." }
                button {
                    style: "padding: 8px 16px; background: {c.bg_hover}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_secondary}; cursor: pointer;",
                    onclick: move |_| { state.write().section = JayMangaSection::Library; },
                    "\u{2190} Retour a la bibliotheque"
                }
            }
        };
    }

    // Etat local de la liseuse
    let mut current_chapter_idx = use_signal(|| {
        if let Some(ref cid) = chapter_id_init {
            chapters.iter().position(|ch| ch.id.as_deref() == Some(cid.as_str())).unwrap_or(0)
        } else {
            0
        }
    });
    let mut current_page_idx = use_signal(|| 0i32);
    let mut reading_mode = use_signal(|| ReadingMode::from_str(&reading_format));
    let mut is_fullscreen = use_signal(|| false);
    let mut show_controls = use_signal(|| true);
    let mut xp_toast = use_signal(|| Option::<(i64, i64)>::None); // (xp_gained, timestamp counter)
    let mut double_page = use_signal(|| false);
    let mut bookmarked = use_signal(|| false);
    let mut zoom_level = use_signal(|| 1.0f64);
    let mut is_favorited = use_signal(|| false);

    // Sauvegarde automatique de la progression dans ReaderFavorite
    {
        let db_save = db.clone();
        let work_id_save = work_id.clone();
        let _ = use_memo(move || {
            let chap_idx_val = *current_chapter_idx.read();
            let page_idx_val = *current_page_idx.read();

            // Calculer la progression globale approximative
            let chapters_save = db_save.chapter_list_by_work(&work_id_save).unwrap_or_default();
            let total_ch = chapters_save.len().max(1) as f64;
            let pages_in_chap = chapters_save.get(chap_idx_val)
                .and_then(|ch| ch.page_count)
                .unwrap_or(1) as f64;
            let chap_progress = if pages_in_chap > 0.0 { (page_idx_val as f64 + 1.0) / pages_in_chap } else { 0.0 };
            let global_progress = ((chap_idx_val as f64 + chap_progress) / total_ch).min(1.0);

            let chapter_num = chapters_save.get(chap_idx_val)
                .and_then(|ch| ch.chapter_number)
                .unwrap_or(0);

            // Mettre a jour le favori existant via favorite_update_progress(id, chapter, page, progress)
            if let Ok(favs) = db_save.favorite_list() {
                if let Some(fav) = favs.iter().find(|f| f.work_id.as_deref() == Some(&work_id_save)) {
                    if let Some(ref fav_id) = fav.id {
                        let _ = db_save.favorite_update_progress(
                            fav_id,
                            chapter_num,
                            page_idx_val + 1,
                            global_progress,
                        );
                    }
                }
            }
        });
    }

    // Auto-disparition du toast XP apres ~1 seconde
    {
        let _ = use_memo(move || {
            let toast = *xp_toast.read();
            if toast.is_some() {
                spawn(async move {
                    #[cfg(not(target_arch = "wasm32"))]
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                    xp_toast.set(None);
                });
            }
        });
    }

    // Chapitre courant
    let chap_idx = *current_chapter_idx.read();
    let chapter = chapters.get(chap_idx).cloned();
    let chapter_title = chapter.as_ref().and_then(|ch| ch.title.clone()).unwrap_or_default();
    let chapter_num = chapter.as_ref().and_then(|ch| ch.chapter_number).unwrap_or(0);
    let chapter_id = chapter.as_ref().and_then(|ch| ch.id.clone()).unwrap_or_default();

    // Pages du chapitre courant
    let pages = db.page_list_by_chapter(&chapter_id).unwrap_or_default();
    let total_pages = pages.len() as i32;
    let page_idx = *current_page_idx.read();
    let current_page = pages.get(page_idx as usize).cloned();

    // Progression
    let progress = reader_chapter_progress(page_idx + 1, total_pages);
    let progress_pct = (progress * 100.0) as i32;

    // Image a afficher
    let has_image = current_page.as_ref().and_then(|p| p.original_image_path.as_ref()).is_some();
    let page_number = current_page.as_ref().and_then(|p| p.page_number).unwrap_or(page_idx + 1);
    let is_demo = reader_is_demo_page(page_number, demo_pages);

    // Mode de lecture
    let mode = *reading_mode.read();
    let is_fs = *is_fullscreen.read();
    let show_ctrl = *show_controls.read();

    // Navigation
    let can_next = page_idx < total_pages - 1;
    let can_prev = page_idx > 0;
    let can_next_chapter = chap_idx < chapters.len() - 1;
    let can_prev_chapter = chap_idx > 0;

    // Pre-calculer la valeur non-Copy necessaire pour go_prev
    let prev_chapter_pages = if can_prev_chapter {
        chapters[chap_idx - 1].page_count.unwrap_or(0)
    } else {
        0
    };

    // Fonctions de navigation (use_callback -> Callback est Copy)
    let go_next = use_callback(move |_| {
        if let Some(next) = reader_next_page_index(page_idx + 1, total_pages) {
            current_page_idx.set(next - 1);
            // Toast XP
            xp_toast.set(Some((XP_PER_PAGE, chrono::Utc::now().timestamp())));
        } else if can_next_chapter {
            current_chapter_idx.set(chap_idx + 1);
            current_page_idx.set(0);
        }
    });

    let go_prev = use_callback(move |_| {
        if let Some(prev) = reader_prev_page_index(page_idx + 1) {
            current_page_idx.set(prev - 1);
        } else if can_prev_chapter {
            current_chapter_idx.set(chap_idx - 1);
            current_page_idx.set((prev_chapter_pages - 1).max(0));
        }
    });

    // Couleur de fond liseuse
    let reader_bg = if is_fs { "#1A1A2E" } else { c.bg_main };
    let reader_height = if is_fs { "100vh" } else { "100%" };
    let zoom = *zoom_level.read();
    let zoom_pct = (zoom * 100.0) as i32;
    let is_bookmarked = *bookmarked.read();
    let fav_top = if is_bookmarked { 40 } else { 8 };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; background: {reader_bg}; height: {reader_height}; position: relative; overflow: hidden; user-select: none;",

            // Zoom via Ctrl+molette
            onwheel: move |evt: WheelEvent| {
                if evt.modifiers().ctrl() {
                    evt.prevent_default();
                    let current = *zoom_level.read();
                    let pixels = evt.delta().strip_units();
                    let delta = if pixels.y < 0.0 { 0.1 } else { -0.1 };
                    let new_zoom = (current + delta).clamp(0.5, 3.0);
                    zoom_level.set(new_zoom);
                }
            },

            // Gestion clavier
            tabindex: "0",
            onkeydown: move |evt: KeyboardEvent| {
                match evt.key() {
                    Key::ArrowRight => {
                        if mode.is_rtl() { go_prev(()) } else { go_next(()) }
                    }
                    Key::ArrowLeft => {
                        if mode.is_rtl() { go_next(()) } else { go_prev(()) }
                    }
                    Key::ArrowDown => { go_next(()); }
                    Key::Character(ref s) if s == " " => { go_next(()); }
                    Key::ArrowUp => { go_prev(()); }
                    Key::Escape => { state.write().section = JayMangaSection::Library; }
                    Key::Character(ref s) if s == "f" || s == "F" => {
                        { let val = *is_favorited.read(); is_favorited.set(!val); }
                    }
                    Key::Character(ref s) if s == "m" || s == "M" => {
                        { let val = *double_page.read(); double_page.set(!val); }
                    }
                    Key::Character(ref s) if s == "b" || s == "B" => {
                        { let val = *bookmarked.read(); bookmarked.set(!val); }
                    }
                    Key::F11 => {
                        { let val = *is_fullscreen.read(); is_fullscreen.set(!val); }
                    }
                    _ => {}
                }
            },

            // -- Barre superieure (si controles visibles) --
            if show_ctrl {
                div {
                    style: "display: flex; align-items: center; justify-content: space-between; padding: 8px 16px; background: #00000080; z-index: 10; flex-shrink: 0;",

                    // Gauche : retour + titre
                    div {
                        style: "display: flex; align-items: center; gap: 12px;",

                        button {
                            style: "padding: 6px 10px; background: transparent; border: 1px solid #ffffff30; border-radius: 4px; color: #ffffffcc; cursor: pointer; font-size: 12px;",
                            onclick: move |_| { state.write().section = JayMangaSection::Library; },
                            "\u{2190} Quitter"
                        }

                        div {
                            p {
                                style: "font-size: 14px; color: #ffffffee; font-weight: 500;",
                                "{work_title}"
                            }
                            p {
                                style: "font-size: 11px; color: #ffffff80;",
                                {
                                    if chapter_title.is_empty() {
                                        format!("Chapitre {}", chapter_num)
                                    } else {
                                        format!("Ch.{} \u{2014} {}", chapter_num, chapter_title)
                                    }
                                }
                            }
                        }
                    }

                    // Centre : mode de lecture
                    div {
                        style: "display: flex; gap: 4px;",

                        ReaderModeButton { label: "M", tooltip: "Manga RTL", active: mode == ReadingMode::Manga, onclick: move |_| { reading_mode.set(ReadingMode::Manga); } }
                        ReaderModeButton { label: "W", tooltip: "Webtoon", active: mode == ReadingMode::Webtoon, onclick: move |_| { reading_mode.set(ReadingMode::Webtoon); } }
                        ReaderModeButton { label: "L", tooltip: "Paysage", active: mode == ReadingMode::Landscape, onclick: move |_| { reading_mode.set(ReadingMode::Landscape); } }
                        ReaderModeButton { label: "C", tooltip: "Comics LTR", active: mode == ReadingMode::Comics, onclick: move |_| { reading_mode.set(ReadingMode::Comics); } }
                    }

                    // Droite : plein ecran + mode double
                    div {
                        style: "display: flex; gap: 8px; align-items: center;",

                        Badge {
                            text: mode.label().to_string(),
                            color: "#FFD700".to_string(),
                        }

                        button {
                            style: "padding: 4px 8px; background: transparent; border: 1px solid #ffffff30; border-radius: 4px; color: #ffffffcc; cursor: pointer; font-size: 12px;",
                            onclick: move |_| { { let val = *double_page.read(); double_page.set(!val); } },
                            if *double_page.read() { "\u{1F4D6} Double" } else { "\u{1F4C4} Simple" }
                        }

                        button {
                            style: "padding: 4px 8px; background: transparent; border: 1px solid #ffffff30; border-radius: 4px; color: #ffffffcc; cursor: pointer; font-size: 12px;",
                            onclick: move |_| { { let val = *is_fullscreen.read(); is_fullscreen.set(!val); } },
                            if is_fs { "\u{2291} Quitter FS" } else { "\u{229E} Plein ecran" }
                        }
                    }
                }
            }

            // -- Zone de lecture principale --
            div {
                style: "flex: 1; display: flex; align-items: center; justify-content: center; position: relative; overflow: hidden; cursor: pointer; min-height: 0; transform: scale({zoom}); transform-origin: center center;",

                // Clic sur la zone pour naviguer
                onclick: move |_| {
                    // Toggle les controles
                    { let val = *show_controls.read(); show_controls.set(!val); }
                },

                // Double-clic pour zoom adapte / reinitialisation
                ondoubleclick: move |_| {
                    let current = *zoom_level.read();
                    if (current - 1.0).abs() < 0.05 {
                        zoom_level.set(1.5); // Zoom modere
                    } else {
                        zoom_level.set(1.0); // Retour a l'original
                    }
                },

                if mode.is_vertical_scroll() {
                    // Mode Webtoon : scroll vertical
                    div {
                        style: "width: 100%; height: 100%; overflow-y: auto; display: flex; flex-direction: column; align-items: center; gap: 2px;",

                        for (idx, page) in pages.iter().enumerate() {
                            {
                                let pnum = page.page_number.unwrap_or(idx as i32 + 1);
                                let has_img = page.original_image_path.is_some();

                                rsx! {
                                    div {
                                        style: "width: 100%; max-width: 800px; min-height: 400px; background: #2A2A3E; display: flex; align-items: center; justify-content: center; position: relative;",

                                        if has_img {
                                            div {
                                                style: "width: 100%; aspect-ratio: 3/4; background: linear-gradient(135deg, #2A2A3E, #3A3A5E); display: flex; align-items: center; justify-content: center;",
                                                span { style: "font-size: 48px; opacity: 0.3;", "\u{1F5BC}\u{FE0F}" }
                                            }
                                        } else {
                                            div {
                                                style: "width: 100%; aspect-ratio: 3/4; background: #2A2A3E; display: flex; align-items: center; justify-content: center;",
                                                span { style: "font-size: 24px; color: #ffffff40;", "Page {pnum}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    // Mode page par page (Manga, Comics, Landscape, Free)
                    div {
                        style: "display: flex; align-items: center; justify-content: center; width: 100%; height: 100%; position: relative;",

                        // Zone clic gauche (precedent ou suivant selon RTL)
                        div {
                            style: "position: absolute; left: 0; top: 0; width: 33%; height: 100%; z-index: 5; cursor: w-resize;",
                            onclick: move |evt| {
                                evt.stop_propagation();
                                if mode.is_rtl() { go_next(()) } else { go_prev(()) }
                            },
                        }

                        // Zone clic droite
                        div {
                            style: "position: absolute; right: 0; top: 0; width: 33%; height: 100%; z-index: 5; cursor: e-resize;",
                            onclick: move |evt| {
                                evt.stop_propagation();
                                if mode.is_rtl() { go_prev(()) } else { go_next(()) }
                            },
                        }

                        // Affichage de la page
                        if has_image {
                            div {
                                style: "max-width: 90%; max-height: 90%; display: flex; align-items: center; justify-content: center;",
                                div {
                                    style: "width: 600px; max-width: 100%; aspect-ratio: 3/4; background: linear-gradient(135deg, #2A2A3E, #3A3A5E); border-radius: 4px; display: flex; align-items: center; justify-content: center; flex-direction: column; gap: 8px;",
                                    span { style: "font-size: 64px; opacity: 0.4;", "\u{1F5BC}\u{FE0F}" }
                                    span { style: "font-size: 14px; color: #ffffff60;", "Page {page_number}" }
                                }
                            }
                        } else {
                            div {
                                style: "width: 600px; max-width: 90%; aspect-ratio: 3/4; background: #2A2A3E; border-radius: 4px; display: flex; align-items: center; justify-content: center; flex-direction: column; gap: 8px;",
                                span { style: "font-size: 48px; color: #ffffff30;", "\u{1F4C4}" }
                                span { style: "font-size: 16px; color: #ffffff50;", "Page {page_number}" }
                            }
                        }

                        // Indicateur demo
                        if !is_demo {
                            div {
                                style: "position: absolute; top: 8px; right: 8px; padding: 4px 8px; background: #FF450090; border-radius: 4px; font-size: 10px; color: white;",
                                "\u{1F512} Contenu payant"
                            }
                        }
                    }
                }

                // Indicateur marque-page (coin superieur gauche)
                if *bookmarked.read() {
                    div {
                        style: "position: absolute; top: 8px; left: 8px; padding: 4px 8px; background: #FFD70090; border-radius: 4px; font-size: 12px; color: #1A1A2E; font-weight: 600; z-index: 20;",
                        "\u{1F516} Marque-page"
                    }
                }

                // Indicateur favori (coin superieur gauche, sous le marque-page)
                if *is_favorited.read() {
                    div {
                        style: "position: absolute; top: {fav_top}px; left: 8px; padding: 4px 8px; background: #ef444490; border-radius: 4px; font-size: 12px; color: white; font-weight: 600; z-index: 20;",
                        "\u{2764}\u{FE0F} Favori"
                    }
                }

                // Indicateur zoom (si != 100%)
                if zoom_pct != 100 {
                    div {
                        style: "position: absolute; top: 8px; right: 8px; padding: 4px 8px; background: #00000080; border-radius: 4px; font-size: 11px; color: #ffffffcc; z-index: 20;",
                        "\u{1F50D} {zoom_pct}%"
                    }
                }

                // Toast XP (coin inferieur droit — disparition auto geree par use_effect)
                if let Some((xp, _ts)) = *xp_toast.read() {
                    div {
                        style: "position: absolute; bottom: 60px; right: 16px; padding: 6px 12px; background: #FFD70090; border-radius: 6px; font-size: 13px; color: #1A1A2E; font-weight: 600; z-index: 20; animation: fadeIn 0.3s;",
                        "+{xp} XP \u{2728}"
                    }
                }
            }

            // -- Barre de progression inferieure --
            if show_ctrl {
                div {
                    style: "display: flex; flex-direction: column; gap: 4px; padding: 8px 16px; background: #00000080; z-index: 10; flex-shrink: 0;",

                    // Barre de progression
                    div {
                        style: "width: 100%; height: 4px; background: #ffffff20; border-radius: 2px; overflow: hidden;",
                        div {
                            style: "width: {progress_pct}%; height: 100%; background: #FF6B35; border-radius: 2px; transition: width 0.3s;",
                        }
                    }

                    // Infos de navigation
                    div {
                        style: "display: flex; align-items: center; justify-content: space-between;",

                        // Gauche : chapitre precedent
                        div {
                            style: "display: flex; gap: 8px; align-items: center;",

                            if can_prev_chapter {
                                button {
                                    style: "padding: 4px 8px; background: transparent; border: 1px solid #ffffff30; border-radius: 4px; color: #ffffffcc; cursor: pointer; font-size: 11px;",
                                    onclick: move |_| {
                                        current_chapter_idx.set(chap_idx - 1);
                                        current_page_idx.set(0);
                                    },
                                    "\u{23EE} Ch.precedent"
                                }
                            }

                            if can_prev {
                                button {
                                    style: "padding: 4px 8px; background: transparent; border: 1px solid #ffffff30; border-radius: 4px; color: #ffffffcc; cursor: pointer; font-size: 11px;",
                                    onclick: move |evt| { evt.stop_propagation(); go_prev(()); },
                                    "\u{25C0} Precedent"
                                }
                            }
                        }

                        // Centre : position
                        div {
                            style: "display: flex; align-items: center; gap: 8px;",

                            span {
                                style: "font-size: 12px; color: #ffffffcc;",
                                "Ch. {chapter_num}"
                            }
                            span {
                                style: "font-size: 14px; color: #ffffffee; font-weight: 600;",
                                {format!("Page {} / {}", page_idx + 1, total_pages)}
                            }
                            span {
                                style: "font-size: 11px; color: #ffffff80;",
                                "({progress_pct}%)"
                            }
                        }

                        // Droite : page suivante + chapitre suivant
                        div {
                            style: "display: flex; gap: 8px; align-items: center;",

                            if can_next {
                                button {
                                    style: "padding: 4px 8px; background: transparent; border: 1px solid #ffffff30; border-radius: 4px; color: #ffffffcc; cursor: pointer; font-size: 11px;",
                                    onclick: move |evt| { evt.stop_propagation(); go_next(()); },
                                    "Suivant \u{25B6}"
                                }
                            }

                            if can_next_chapter {
                                button {
                                    style: "padding: 4px 8px; background: transparent; border: 1px solid #ffffff30; border-radius: 4px; color: #ffffffcc; cursor: pointer; font-size: 11px;",
                                    onclick: move |_| {
                                        current_chapter_idx.set(chap_idx + 1);
                                        current_page_idx.set(0);
                                    },
                                    "Ch.suivant \u{23ED}"
                                }
                            }
                        }
                    }

                    // Raccourcis
                    div {
                        style: "display: flex; gap: 12px; justify-content: center; font-size: 10px; color: #ffffff50;",
                        span { "\u{2190}\u{2192} Navigation" }
                        span { "F Favori" }
                        span { "F11 Plein ecran" }
                        span { "M Mode double" }
                        span { "B Marque-page" }
                        span { "Ctrl+molette Zoom" }
                        span { "Echap Quitter" }
                    }
                }
            }
        }
    }
}

// -- Bouton mode de lecture --

#[component]
fn ReaderModeButton(
    label: &'static str,
    tooltip: &'static str,
    active: bool,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let bg = if active { "#FF6B3580" } else { "transparent" };
    let border = if active { "#FF6B35" } else { "#ffffff30" };
    let color = if active { "#FF6B35" } else { "#ffffffcc" };

    rsx! {
        button {
            style: "width: 32px; height: 32px; background: {bg}; border: 1px solid {border}; border-radius: 4px; color: {color}; cursor: pointer; font-size: 13px; font-weight: 600;",
            title: "{tooltip}",
            onclick: move |evt| { evt.stop_propagation(); onclick.call(evt); },
            "{label}"
        }
    }
}
