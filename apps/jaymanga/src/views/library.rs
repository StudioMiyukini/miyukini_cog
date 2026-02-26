//! Bibliotheque lecteur JayManga — 5 onglets (Favoris, Achats, Telecharges, En cours, Termines).

use dioxus::prelude::*;
use miyukini_service_ui::use_palette;
use crate::use_db;
use super::components::{EmptyState, Badge};
use super::{JayMangaSection, JayMangaState, LibraryTab};

#[component]
pub fn Library(state: Signal<JayMangaState>) -> Element {
    let c = use_palette();
    let db = use_db();

    // Progression lecteur
    let progression = db.progression_get().ok().flatten();
    let total_xp = progression.as_ref().and_then(|p| p.total_xp).unwrap_or(0);
    let current_streak = progression.as_ref().and_then(|p| p.current_streak).unwrap_or(0);

    // Niveau via domain
    let (level, level_name) = jaymanga::domain::gamification::gamification_level_for_xp(total_xp);
    let level_progress = jaymanga::domain::gamification::gamification_level_progress(total_xp);
    let xp_for_next = jaymanga::domain::gamification::gamification_xp_for_next_level(total_xp);
    let progress_pct = (level_progress * 100.0) as i32;

    // Donnees bibliotheque
    let favorites = db.favorite_list().unwrap_or_default();
    let licenses = db.license_list_all().unwrap_or_default();

    // Classement par onglet
    let fav_count = favorites.len();
    let purchases_count = licenses.iter().filter(|l| l.status.as_deref() == Some("active")).count();
    let in_progress: Vec<_> = favorites.iter()
        .filter(|f| {
            let p = f.reading_progress.unwrap_or(0.0);
            p > 0.0 && p < 1.0
        })
        .collect();
    let completed: Vec<_> = favorites.iter()
        .filter(|f| f.reading_progress.unwrap_or(0.0) >= 1.0)
        .collect();
    let downloaded: Vec<_> = favorites.iter()
        .filter(|f| f.purchase_status.as_deref() == Some("downloaded"))
        .collect();

    let active_tab = state.read().library_tab;

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 20px;",

            // En-tete avec progression
            div {
                style: "display: flex; align-items: center; justify-content: space-between;",

                div {
                    h2 {
                        style: "font-size: 24px; color: {c.text_white};",
                        "\u{1F4DA} Ma Bibliotheque JayManga"
                    }
                }
                div {
                    style: "display: flex; align-items: center; gap: 16px;",

                    // Niveau et XP
                    div {
                        style: "text-align: right;",
                        p {
                            style: "font-size: 13px; color: {c.text_white}; font-weight: 500;",
                            "Niveau {level} \u{2014} {level_name}"
                        }
                        div {
                            style: "display: flex; align-items: center; gap: 8px; margin-top: 4px;",
                            div {
                                style: "width: 120px; height: 8px; background: {c.bg_hover}; border-radius: 4px; overflow: hidden;",
                                div {
                                    style: "width: {progress_pct}%; height: 100%; background: #FFD700; border-radius: 4px; transition: width 0.3s;",
                                }
                            }
                            span {
                                style: "font-size: 11px; color: {c.text_muted};",
                                {
                                    if xp_for_next > 0 {
                                        format!("{} / {} XP", total_xp, total_xp + xp_for_next)
                                    } else {
                                        format!("{} XP \u{2014} MAX", total_xp)
                                    }
                                }
                            }
                        }
                    }

                    // Streak
                    if current_streak > 0 {
                        div {
                            style: "display: flex; align-items: center; gap: 4px; padding: 6px 12px; background: #FF450020; border-radius: 8px;",
                            span { style: "font-size: 18px;", "\u{1F525}" }
                            span {
                                style: "font-size: 14px; color: #FF4500; font-weight: 600;",
                                "{current_streak} jours"
                            }
                        }
                    }
                }
            }

            // Onglets
            div {
                style: "display: flex; gap: 4px; border-bottom: 1px solid {c.border}; padding-bottom: 0;",

                TabButton { label: "Favoris", count: fav_count, is_active: active_tab == LibraryTab::Favorites, onclick: move |_| { state.write().library_tab = LibraryTab::Favorites; } }
                TabButton { label: "Achats", count: purchases_count, is_active: active_tab == LibraryTab::Purchases, onclick: move |_| { state.write().library_tab = LibraryTab::Purchases; } }
                TabButton { label: "Telecharges", count: downloaded.len(), is_active: active_tab == LibraryTab::Downloads, onclick: move |_| { state.write().library_tab = LibraryTab::Downloads; } }
                TabButton { label: "En cours", count: in_progress.len(), is_active: active_tab == LibraryTab::InProgress, onclick: move |_| { state.write().library_tab = LibraryTab::InProgress; } }
                TabButton { label: "Termines", count: completed.len(), is_active: active_tab == LibraryTab::Completed, onclick: move |_| { state.write().library_tab = LibraryTab::Completed; } }
            }

            // Contenu de l'onglet actif
            match active_tab {
                LibraryTab::Favorites => rsx! {
                    FavoritesTab { favorites: favorites.clone(), state: state }
                },
                LibraryTab::Purchases => rsx! {
                    PurchasesTab { licenses: licenses.clone() }
                },
                LibraryTab::Downloads => rsx! {
                    DownloadsTab { favorites: favorites.clone(), state: state }
                },
                LibraryTab::InProgress => rsx! {
                    InProgressTab { favorites: favorites.clone(), state: state }
                },
                LibraryTab::Completed => rsx! {
                    CompletedTab { favorites: favorites.clone(), state: state }
                },
            }
        }
    }
}

// -- Bouton d'onglet --

#[component]
fn TabButton(
    label: &'static str,
    count: usize,
    is_active: bool,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let c = use_palette();
    let border_color = if is_active { c.accent_blue } else { "transparent" };
    let text_color = if is_active { c.text_white } else { c.text_secondary };

    rsx! {
        button {
            style: "padding: 10px 16px; background: transparent; border: none; border-bottom: 2px solid {border_color}; color: {text_color}; cursor: pointer; font-size: 13px; display: flex; align-items: center; gap: 6px; transition: all 0.2s;",
            onclick: move |evt| onclick.call(evt),

            span { "{label}" }
            span {
                style: "padding: 2px 8px; background: {c.bg_hover}; border-radius: 10px; font-size: 11px; color: {c.text_muted};",
                "{count}"
            }
        }
    }
}

// -- Carte d'oeuvre bibliotheque --

#[component]
fn WorkCard(
    title: String,
    progress: f64,
    status: String,
    #[props(default)] chapter_info: String,
    #[props(default)] cog_name: String,
    #[props(default)] cog_online: bool,
    #[props(default)] is_downloaded: bool,
    #[props(default)] on_read: Option<EventHandler<MouseEvent>>,
) -> Element {
    let c = use_palette();
    let progress_pct = (progress * 100.0) as i32;
    let progress_color = if progress >= 1.0 { c.accent_green } else { c.accent_blue };
    let cog_indicator = if cog_online { "\u{1F7E2}" } else { "\u{26AB}" };

    let status_info = match status.as_str() {
        "owned" => ("Achete", c.accent_green),
        "demo" => ("Demo", c.accent_blue),
        "downloaded" => ("Telecharge", "#8b5cf6"),
        _ => ("\u{2014}", c.text_muted),
    };

    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 16px; padding: 16px; background: {c.bg_secondary}; border-radius: 8px; border: 1px solid {c.border};",

            // Cover placeholder
            div {
                style: "width: 56px; height: 80px; background: {c.bg_hover}; border-radius: 4px; display: flex; align-items: center; justify-content: center; font-size: 24px; flex-shrink: 0;",
                "\u{1F4D6}"
            }

            // Infos
            div {
                style: "flex: 1; min-width: 0;",
                div {
                    style: "display: flex; align-items: center; gap: 8px; margin-bottom: 4px;",
                    h4 {
                        style: "font-size: 14px; color: {c.text_white}; font-weight: 500; white-space: nowrap; overflow: hidden; text-overflow: ellipsis;",
                        "{title}"
                    }
                    Badge { text: status_info.0.to_string(), color: status_info.1.to_string() }
                    if is_downloaded {
                        Badge { text: "\u{1F4BE} Local".to_string(), color: "#8b5cf6".to_string() }
                    }
                }

                if !chapter_info.is_empty() {
                    p {
                        style: "font-size: 12px; color: {c.text_secondary}; margin-bottom: 6px;",
                        "{chapter_info}"
                    }
                }

                // Barre de progression
                div {
                    style: "display: flex; align-items: center; gap: 10px;",
                    div {
                        style: "flex: 1; height: 6px; background: {c.bg_hover}; border-radius: 3px; overflow: hidden;",
                        div {
                            style: "width: {progress_pct}%; height: 100%; background: {progress_color}; border-radius: 3px; transition: width 0.3s;",
                        }
                    }
                    span {
                        style: "font-size: 12px; color: {c.text_muted}; min-width: 35px; text-align: right;",
                        "{progress_pct}%"
                    }
                }

                if !cog_name.is_empty() {
                    p {
                        style: "font-size: 11px; color: {c.text_muted}; margin-top: 4px;",
                        "{cog_indicator} {cog_name}"
                    }
                }
            }

            // Action
            button {
                style: "padding: 8px 16px; background: {c.accent_blue}; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 12px; flex-shrink: 0;",
                onclick: move |evt| {
                    if let Some(ref handler) = on_read {
                        handler.call(evt);
                    }
                },
                if progress > 0.0 && progress < 1.0 {
                    "Continuer \u{25B6}"
                } else if progress >= 1.0 {
                    "Relire \u{25B6}"
                } else {
                    "Lire \u{25B6}"
                }
            }
        }
    }
}

// -- Onglet Favoris --

#[component]
fn FavoritesTab(favorites: Vec<jaymanga::data::ReaderFavorite>, state: Signal<JayMangaState>) -> Element {
    if favorites.is_empty() {
        return rsx! {
            EmptyState {
                title: "Aucun favori".to_string(),
                message: "Visitez le Portail d'un COG pour decouvrir des manga et les ajouter a vos favoris.".to_string(),
                icon: "\u{2764}\u{FE0F}".to_string(),
            }
        };
    }

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 8px;",

            for fav in favorites.iter() {
                {
                    let title = fav.cached_title.clone().unwrap_or_else(|| "Titre inconnu".to_string());
                    let progress = fav.reading_progress.unwrap_or(0.0);
                    let status = fav.purchase_status.clone().unwrap_or_else(|| "demo".to_string());
                    let cog_id = fav.seller_cog_id.clone().unwrap_or_default();
                    let cog_short = if cog_id.len() > 16 { format!("{}...", &cog_id[..16]) } else { cog_id };
                    let downloaded = fav.purchase_status.as_deref() == Some("downloaded");
                    let chapter = fav.last_read_chapter.map(|n| format!("Ch. {n}")).unwrap_or_default();
                    let work_id = fav.work_id.clone().unwrap_or_default();

                    rsx! {
                        WorkCard {
                            title: title,
                            progress: progress,
                            status: status,
                            chapter_info: chapter,
                            cog_name: cog_short,
                            is_downloaded: downloaded,
                            on_read: move |_| {
                                let mut s = state.write();
                                s.reader_work_id = Some(work_id.clone());
                                s.reader_chapter_id = None;
                                s.section = JayMangaSection::Reader;
                            },
                        }
                    }
                }
            }
        }
    }
}

// -- Onglet Achats --

#[component]
fn PurchasesTab(licenses: Vec<jaymanga::data::PurchaseLicense>) -> Element {
    let c = use_palette();

    let active_licenses: Vec<_> = licenses.iter()
        .filter(|l| l.status.as_deref() == Some("active"))
        .collect();

    if active_licenses.is_empty() {
        return rsx! {
            EmptyState {
                title: "Aucun achat".to_string(),
                message: "Les oeuvres achetees apparaitront ici avec leur licence.".to_string(),
                icon: "\u{1F3AB}".to_string(),
            }
        };
    }

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 8px;",

            for license in active_licenses.iter() {
                {
                    let work_id = license.work_id.clone().unwrap_or_default();
                    let work_short = if work_id.len() > 12 { format!("{}...", &work_id[..12]) } else { work_id };
                    let amount = license.amount_paid.unwrap_or(0);
                    let amount_str = if amount == 0 { "Gratuit".to_string() } else { format!("{},{:02} \u{20AC}", amount / 100, amount % 100) };
                    let date = license.purchased_at.clone()
                        .map(|d| if d.len() >= 10 { d[..10].to_string() } else { d })
                        .unwrap_or_default();
                    let purchase_type = license.purchase_type.clone().unwrap_or_else(|| "work".to_string());

                    rsx! {
                        div {
                            style: "display: flex; align-items: center; gap: 16px; padding: 16px; background: {c.bg_secondary}; border-radius: 8px; border: 1px solid {c.border};",

                            div {
                                style: "width: 48px; height: 48px; background: {c.bg_hover}; border-radius: 8px; display: flex; align-items: center; justify-content: center; font-size: 20px; flex-shrink: 0;",
                                "\u{1F3AB}"
                            }
                            div {
                                style: "flex: 1;",
                                div {
                                    style: "display: flex; align-items: center; gap: 8px;",
                                    p {
                                        style: "font-size: 14px; color: {c.text_white}; font-weight: 500;",
                                        "Oeuvre {work_short}"
                                    }
                                    Badge { text: purchase_type, color: c.accent_blue.to_string() }
                                }
                                p {
                                    style: "font-size: 12px; color: {c.text_muted}; margin-top: 2px;",
                                    "Achete le {date}"
                                }
                            }
                            span {
                                style: "font-size: 14px; color: {c.accent_green}; font-weight: 600;",
                                "{amount_str}"
                            }
                        }
                    }
                }
            }
        }
    }
}

// -- Onglet Telecharges --

#[component]
fn DownloadsTab(favorites: Vec<jaymanga::data::ReaderFavorite>, state: Signal<JayMangaState>) -> Element {
    let downloaded: Vec<_> = favorites.iter()
        .filter(|f| f.purchase_status.as_deref() == Some("downloaded"))
        .collect();

    if downloaded.is_empty() {
        return rsx! {
            EmptyState {
                title: "Aucun telechargement".to_string(),
                message: "Les oeuvres telechargees pour lecture hors-ligne apparaitront ici.".to_string(),
                icon: "\u{1F4BE}".to_string(),
            }
        };
    }

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 8px;",

            for fav in downloaded.iter() {
                {
                    let title = fav.cached_title.clone().unwrap_or_else(|| "Titre inconnu".to_string());
                    let progress = fav.reading_progress.unwrap_or(0.0);
                    let chapter = fav.last_read_chapter.map(|n| format!("Ch. {n}")).unwrap_or_default();
                    let work_id = fav.work_id.clone().unwrap_or_default();

                    rsx! {
                        WorkCard {
                            title: title,
                            progress: progress,
                            status: "downloaded".to_string(),
                            chapter_info: chapter,
                            is_downloaded: true,
                            on_read: move |_| {
                                let mut s = state.write();
                                s.reader_work_id = Some(work_id.clone());
                                s.reader_chapter_id = None;
                                s.section = JayMangaSection::Reader;
                            },
                        }
                    }
                }
            }
        }
    }
}

// -- Onglet En cours --

#[component]
fn InProgressTab(favorites: Vec<jaymanga::data::ReaderFavorite>, state: Signal<JayMangaState>) -> Element {
    let mut in_progress: Vec<_> = favorites.iter()
        .filter(|f| {
            let p = f.reading_progress.unwrap_or(0.0);
            p > 0.0 && p < 1.0
        })
        .collect();

    // Trier par derniere lecture (plus recent en premier)
    in_progress.sort_by(|a, b| {
        let da = a.last_synced_at.as_deref().unwrap_or("");
        let db_val = b.last_synced_at.as_deref().unwrap_or("");
        db_val.cmp(da)
    });

    if in_progress.is_empty() {
        return rsx! {
            EmptyState {
                title: "Aucune lecture en cours".to_string(),
                message: "Les oeuvres que vous avez commence a lire apparaitront ici.".to_string(),
                icon: "\u{1F4D6}".to_string(),
            }
        };
    }

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 8px;",

            for fav in in_progress.iter() {
                {
                    let title = fav.cached_title.clone().unwrap_or_else(|| "Titre inconnu".to_string());
                    let progress = fav.reading_progress.unwrap_or(0.0);
                    let status = fav.purchase_status.clone().unwrap_or_else(|| "demo".to_string());
                    let cog_id = fav.seller_cog_id.clone().unwrap_or_default();
                    let cog_short = if cog_id.len() > 16 { format!("{}...", &cog_id[..16]) } else { cog_id };
                    let downloaded = fav.purchase_status.as_deref() == Some("downloaded");
                    let chapter = fav.last_read_chapter.map(|n| format!("Ch. {n}")).unwrap_or_default();
                    let work_id = fav.work_id.clone().unwrap_or_default();

                    rsx! {
                        WorkCard {
                            title: title,
                            progress: progress,
                            status: status,
                            chapter_info: chapter,
                            cog_name: cog_short,
                            is_downloaded: downloaded,
                            on_read: move |_| {
                                let mut s = state.write();
                                s.reader_work_id = Some(work_id.clone());
                                s.reader_chapter_id = None;
                                s.section = JayMangaSection::Reader;
                            },
                        }
                    }
                }
            }
        }
    }
}

// -- Onglet Termines --

#[component]
fn CompletedTab(favorites: Vec<jaymanga::data::ReaderFavorite>, state: Signal<JayMangaState>) -> Element {
    let c = use_palette();

    let completed: Vec<_> = favorites.iter()
        .filter(|f| f.reading_progress.unwrap_or(0.0) >= 1.0)
        .collect();

    if completed.is_empty() {
        return rsx! {
            EmptyState {
                title: "Aucune oeuvre terminee".to_string(),
                message: "Les oeuvres lues a 100% apparaitront ici.".to_string(),
                icon: "\u{1F3C6}".to_string(),
            }
        };
    }

    rsx! {
        div {
            style: "display: grid; grid-template-columns: repeat(4, 1fr); gap: 12px;",

            for fav in completed.iter() {
                {
                    let title = fav.cached_title.clone().unwrap_or_else(|| "Titre inconnu".to_string());
                    let work_id = fav.work_id.clone().unwrap_or_default();

                    rsx! {
                        button {
                            style: "padding: 16px; background: {c.bg_secondary}; border-radius: 8px; border: 1px solid {c.border}; text-align: center; cursor: pointer; color: {c.text_primary};",
                            onclick: move |_| {
                                let mut s = state.write();
                                s.reader_work_id = Some(work_id.clone());
                                s.reader_chapter_id = None;
                                s.section = JayMangaSection::Reader;
                            },

                            div {
                                style: "width: 80px; height: 110px; background: {c.bg_hover}; border-radius: 4px; display: flex; align-items: center; justify-content: center; font-size: 32px; margin: 0 auto 12px;",
                                "\u{1F4D6}"
                            }
                            h4 {
                                style: "font-size: 13px; color: {c.text_white}; margin-bottom: 4px; white-space: nowrap; overflow: hidden; text-overflow: ellipsis;",
                                "{title}"
                            }
                            div {
                                style: "display: flex; align-items: center; justify-content: center; gap: 6px;",
                                span {
                                    style: "font-size: 12px; color: {c.accent_green}; font-weight: 500;",
                                    "\u{2705} Termine"
                                }
                                span {
                                    style: "font-size: 11px; color: {c.accent_blue};",
                                    "Relire \u{25B6}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
