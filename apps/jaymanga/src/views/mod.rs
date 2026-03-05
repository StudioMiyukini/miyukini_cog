//! Module JayManga — lecture et vente de manga en ligne.
//!
//! Architecture : sidebar + contenu dynamique par section.
//! Donnees reelles via JayMangaDb (crate::use_db()).

mod aggregation;
mod boutique;
mod catalogue;
mod chapters;
pub mod components;
mod dashboard;
mod library;
mod onboarding;
mod profile;
mod promotions;
mod reader;
mod reader_stats;
mod sales;
mod series;
mod sidebar;
mod work_form;

use crate::use_db;
use dioxus::prelude::*;

use aggregation::Aggregation;
use boutique::Boutique;
use catalogue::Catalogue;
use chapters::Chapters;
use dashboard::Dashboard;
use library::Library;
use onboarding::Onboarding;
use profile::Profile;
use promotions::Promotions;
use reader::Reader;
use reader_stats::ReaderStats;
use sales::Sales;
use series::SeriesView;
use sidebar::JayMangaSidebar;
use work_form::WorkForm;

// ── State ──────────────────────────────────────────────────────────────

/// Section active dans JayManga.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum JayMangaSection {
    #[default]
    Dashboard,
    Catalogue,
    NouvelleOeuvre,
    ModifierOeuvre,
    Chapters,
    Sales,
    ReaderStats,
    Boutique,
    Aggregation,
    Library,
    Profile,
    Series,
    Promotions,
    Reader,
}

/// Onglet actif de la bibliotheque lecteur.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LibraryTab {
    #[default]
    Favorites,
    Purchases,
    Downloads,
    InProgress,
    Completed,
}

/// Etat local complet de la vue JayManga.
#[derive(Debug, Clone, Default)]
pub struct JayMangaState {
    /// Section active.
    pub section: JayMangaSection,
    /// ID de l'oeuvre en cours d'edition (None = creation).
    pub editing_work_id: Option<String>,
    /// ID de l'oeuvre dont on gere les chapitres.
    pub chapters_work_id: Option<String>,
    /// Onglet actif de la bibliotheque.
    pub library_tab: LibraryTab,
    /// ID de l'oeuvre en cours de lecture (liseuse).
    pub reader_work_id: Option<String>,
    /// ID du chapitre initial pour la liseuse.
    pub reader_chapter_id: Option<String>,
}

// ── Composant racine ───────────────────────────────────────────────────

#[component]
pub fn JayMangaView() -> Element {
    let db = use_db();
    let mut state = use_signal(JayMangaState::default);

    // Verifier si l'onboarding est termine (ReaderProgression.onboarding_completed)
    let db_onboard = db.clone();
    let mut onboarding_done = use_signal(move || {
        db_onboard
            .progression_get()
            .ok()
            .flatten()
            .and_then(|p| p.onboarding_completed)
            .unwrap_or(false)
    });

    // Si l'onboarding n'est pas termine, afficher l'onboarding Miou
    if !*onboarding_done.read() {
        let db_complete = db.clone();
        return rsx! {
            div {
                style: "display: flex; flex-grow: 1; flex-shrink: 1; flex-basis: 0; min-height: 0; overflow: hidden;",

                Onboarding {
                    on_complete: move |activated_seller: bool| {
                        let db = db_complete.clone();

                        // Creer ou mettre a jour la progression avec onboarding_completed = true
                        let mut prog = db.progression_get().ok().flatten()
                            .unwrap_or_default();
                        if prog.id.is_none() {
                            prog.id = Some(uuid::Uuid::new_v4().to_string());
                        }
                        prog.onboarding_completed = Some(true);
                        prog.current_level = prog.current_level.or(Some(1));
                        prog.total_xp = prog.total_xp.or(Some(0));
                        prog.current_streak = prog.current_streak.or(Some(0));
                        let _ = db.progression_upsert(&prog);

                        // Si le vendeur a ete active, creer la config boutique
                        if activated_seller {
                            if db.seller_config_get().ok().flatten().is_none() {
                                let config = jaymanga::data::SellerConfig {
                                    id: Some(uuid::Uuid::new_v4().to_string()),
                                    shop_name: Some("Ma Boutique Manga".to_string()),
                                    ..Default::default()
                                };
                                let _ = db.seller_config_upsert(&config);
                            }
                        }

                        onboarding_done.set(true);
                    },
                }
            }
        };
    }

    // Verifier si le vendeur a une configuration boutique
    let has_seller_config = { db.seller_config_get().ok().flatten().is_some() };

    let db_setup = db.clone();

    rsx! {
        div {
            style: "display: flex; flex-grow: 1; flex-shrink: 1; flex-basis: 0; min-height: 0; overflow: hidden;",

            // Sidebar
            JayMangaSidebar { state: state }

            // Contenu principal
            div {
                style: "flex: 1; min-height: 0; display: flex; flex-direction: column; overflow: hidden;",

                div {
                    style: "flex: 1; min-height: 0; padding: 24px; overflow-y: auto; overflow-x: hidden;",

                    if !has_seller_config {
                        components::EmptyState {
                            title: "Bienvenue sur JayManga".to_string(),
                            message: "Configurez votre boutique manga pour commencer a publier et vendre.".to_string(),
                            icon: "\u{1F4DA}".to_string(),
                            action_label: "Configurer ma boutique".to_string(),
                            on_action: move |_| {
                                let db = db_setup.clone();
                                let config = jaymanga::data::SellerConfig {
                                    id: Some(uuid::Uuid::new_v4().to_string()),
                                    shop_name: Some("Ma Boutique Manga".to_string()),
                                    ..Default::default()
                                };
                                if db.seller_config_upsert(&config).is_ok() {
                                    state.write().section = JayMangaSection::Boutique;
                                }
                            },
                        }
                    } else {
                        match state.read().section {
                            JayMangaSection::Dashboard => rsx! {
                                Dashboard { state: state }
                            },
                            JayMangaSection::Catalogue => rsx! {
                                Catalogue { state: state }
                            },
                            JayMangaSection::NouvelleOeuvre | JayMangaSection::ModifierOeuvre => rsx! {
                                WorkForm { state: state }
                            },
                            JayMangaSection::Chapters => rsx! {
                                Chapters { state: state }
                            },
                            JayMangaSection::Sales => rsx! {
                                Sales { state: state }
                            },
                            JayMangaSection::ReaderStats => rsx! {
                                ReaderStats { state: state }
                            },
                            JayMangaSection::Boutique => rsx! {
                                Boutique { state: state }
                            },
                            JayMangaSection::Aggregation => rsx! {
                                Aggregation { state: state }
                            },
                            JayMangaSection::Library => rsx! {
                                Library { state: state }
                            },
                            JayMangaSection::Profile => rsx! {
                                Profile { state: state }
                            },
                            JayMangaSection::Series => rsx! {
                                SeriesView { state: state }
                            },
                            JayMangaSection::Promotions => rsx! {
                                Promotions { state: state }
                            },
                            JayMangaSection::Reader => rsx! {
                                Reader { state: state }
                            },
                        }
                    }
                }
            }
        }
    }
}
