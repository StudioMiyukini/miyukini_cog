//! Portail Agrégé JayManga (vue inter-COG unifiée).
//!
//! Structures de données pour le rendu SSR du Portail Agrégé,
//! la page vendeur, et le moteur de recherche unifié.

use serde::Serialize;

use super::portal::WorkCard;

// ---------------------------------------------------------------------------
// Contextes de template
// ---------------------------------------------------------------------------

/// Contexte pour le template Portail Agrégé (`/manga/aggregate`).
#[derive(Debug, Clone, Serialize)]
pub struct AggregatePageContext {
    pub portal_name: String,
    pub works: Vec<AggregatedWorkCard>,
    pub sellers: Vec<SellerCard>,
    pub genres: Vec<String>,
    pub total_works: i32,
    pub current_page: i32,
    pub total_pages: i32,
    pub search_query: Option<String>,
    pub filters: AggregateFilters,
    pub cache_freshness: String, // "Mis à jour il y a X heures"
}

/// Filtres actifs du Portail Agrégé.
#[derive(Debug, Clone, Default, Serialize)]
pub struct AggregateFilters {
    pub genre: Option<String>,
    pub format: Option<String>,
    pub pricing: Option<String>,
    pub language: Option<String>,
    pub online_only: bool,
    pub seller_cog_id: Option<String>,
}

/// Carte d'oeuvre dans le Portail Agrégé.
#[derive(Debug, Clone, Serialize)]
pub struct AggregatedWorkCard {
    pub work: WorkCard,
    pub seller_cog_id: String,
    pub seller_name: String,
    pub is_online: bool,
    pub portal_url: String, // URL vers le portail du vendeur
}

/// Carte vendeur dans le Portail Agrégé.
#[derive(Debug, Clone, Serialize)]
pub struct SellerCard {
    pub cog_id: String,
    pub shop_name: String,
    pub shop_description: Option<String>,
    pub avatar_url: Option<String>,
    pub work_count: i32,
    pub is_online: bool,
    pub last_synced: String,
}

/// Contexte pour la page vendeur (`/manga/aggregate/seller/{cog_id}`).
#[derive(Debug, Clone, Serialize)]
pub struct SellerPageContext {
    pub seller: SellerCard,
    pub works: Vec<AggregatedWorkCard>,
    pub total_works: i32,
    pub current_page: i32,
    pub total_pages: i32,
}
