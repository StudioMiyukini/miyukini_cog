//! Portail vendeur JayManga (catalogue public, liseuse web).
//!
//! Définit les structures de données pour le rendu SSR des templates
//! du portail vendeur (catalogue, fiche oeuvre, liseuse, panier).

use serde::Serialize;

// ---------------------------------------------------------------------------
// Contextes de template
// ---------------------------------------------------------------------------

/// Contexte pour le template catalogue (`/manga`).
#[derive(Debug, Clone, Serialize)]
pub struct CatalogPageContext {
    pub shop_name: String,
    pub shop_description: Option<String>,
    pub works: Vec<WorkCard>,
    pub series: Vec<SeriesCard>,
    pub genres: Vec<String>,
    pub total_works: i32,
    pub current_page: i32,
    pub total_pages: i32,
    pub current_genre: Option<String>,
    pub current_format: Option<String>,
    pub search_query: Option<String>,
}

/// Carte d'oeuvre pour le catalogue.
#[derive(Debug, Clone, Serialize)]
pub struct WorkCard {
    pub id: String,
    pub title: String,
    pub cover_url: Option<String>,
    pub authors: Vec<String>,
    pub reading_format: String,
    pub pricing_model: String,
    pub price_display: String,
    pub demo_available: bool,
    pub chapter_count: i32,
}

/// Carte de série pour le catalogue.
#[derive(Debug, Clone, Serialize)]
pub struct SeriesCard {
    pub id: String,
    pub title: String,
    pub cover_url: Option<String>,
    pub volume_count: i32,
    pub status: String,
}

/// Contexte pour le template fiche oeuvre (`/manga/{work_id}`).
#[derive(Debug, Clone, Serialize)]
pub struct WorkPageContext {
    pub work: WorkDetailView,
    pub chapters: Vec<ChapterListItem>,
    pub related_works: Vec<WorkCard>,
    pub has_license: bool,
    pub show_buy_button: bool,
}

/// Vue détaillée d'une oeuvre.
#[derive(Debug, Clone, Serialize)]
pub struct WorkDetailView {
    pub id: String,
    pub title: String,
    pub synopsis: Option<String>,
    pub cover_url: Option<String>,
    pub authors: Vec<String>,
    pub genres: Vec<String>,
    pub tags: Vec<String>,
    pub reading_format: String,
    pub pricing_model: String,
    pub price_display: String,
    pub demo_pages_count: i32,
    pub total_pages: i32,
    pub language: Option<String>,
    pub series_title: Option<String>,
    pub volume_number: Option<i32>,
}

/// Élément de la table des matières.
#[derive(Debug, Clone, Serialize)]
pub struct ChapterListItem {
    pub id: String,
    pub number: i32,
    pub title: String,
    pub page_count: i32,
    pub is_free: bool,
}

/// Contexte pour le template liseuse (`/manga/read/{chapter_id}`).
#[derive(Debug, Clone, Serialize)]
pub struct ReaderPageContext {
    pub work_id: String,
    pub work_title: String,
    pub chapter_id: String,
    pub chapter_title: String,
    pub chapter_number: i32,
    pub reading_format: String,
    pub pages: Vec<ReaderPageItem>,
    pub is_demo: bool,
    pub demo_limit: i32,
    pub has_license: bool,
    pub prev_chapter_id: Option<String>,
    pub next_chapter_id: Option<String>,
}

/// Page dans la liseuse web.
#[derive(Debug, Clone, Serialize)]
pub struct ReaderPageItem {
    pub page_number: i32,
    pub image_url: String,
    pub width: i32,
    pub height: i32,
    pub is_accessible: bool, // false si au-delà de la limite demo
}
