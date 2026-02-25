//! Endpoints REST du catalogue JayManga.
//!
//! Routes publiques pour consulter les oeuvres publiées, les séries,
//! et effectuer des recherches textuelles.

use serde::{Deserialize, Serialize};

use crate::data::WorkFilters;

// ---------------------------------------------------------------------------
// Types de requête / réponse
// ---------------------------------------------------------------------------

/// Paramètres de requête pour le listing du catalogue.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct CatalogQuery {
    pub genre: Option<String>,
    pub format: Option<String>,
    pub pricing: Option<String>,
    pub search: Option<String>,
    pub series_id: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

impl CatalogQuery {
    /// Convertit en filtres DB.
    #[must_use]
    pub fn to_work_filters(&self) -> WorkFilters {
        let per_page = self.per_page.unwrap_or(20).min(100);
        let page = self.page.unwrap_or(1).max(1);
        WorkFilters {
            status: Some("published".to_string()),
            genre: self.genre.clone(),
            reading_format: self.format.clone(),
            pricing_model: self.pricing.clone(),
            search: self.search.clone(),
            series_id: self.series_id.clone(),
            limit: Some(per_page),
            offset: Some((page - 1) * per_page),
        }
    }
}

/// Réponse paginée du catalogue.
#[derive(Debug, Clone, Serialize)]
pub struct CatalogResponse<T: Serialize> {
    pub items: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}

impl<T: Serialize> CatalogResponse<T> {
    #[must_use]
    pub fn new(items: Vec<T>, total: i64, page: i64, per_page: i64) -> Self {
        let total_pages = if per_page > 0 {
            (total + per_page - 1) / per_page
        } else {
            0
        };
        Self {
            items,
            total,
            page,
            per_page,
            total_pages,
        }
    }
}

/// Résumé d'oeuvre pour le listing catalogue.
#[derive(Debug, Clone, Serialize)]
pub struct WorkSummary {
    pub id: String,
    pub title: String,
    pub authors: Vec<String>,
    pub genres: Vec<String>,
    pub cover_url: Option<String>,
    pub reading_format: String,
    pub pricing_model: String,
    pub price: i64,
    pub currency: String,
    pub demo_pages_count: i32,
    pub total_pages: i32,
    pub series_title: Option<String>,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catalog_query_defaults() {
        let query = CatalogQuery::default();
        let filters = query.to_work_filters();
        assert_eq!(filters.status.as_deref(), Some("published"));
        assert_eq!(filters.limit, Some(20));
        assert_eq!(filters.offset, Some(0));
    }

    #[test]
    fn test_catalog_query_pagination() {
        let query = CatalogQuery {
            page: Some(3),
            per_page: Some(10),
            ..Default::default()
        };
        let filters = query.to_work_filters();
        assert_eq!(filters.limit, Some(10));
        assert_eq!(filters.offset, Some(20));
    }

    #[test]
    fn test_catalog_response_pages() {
        let resp = CatalogResponse::<String>::new(vec![], 95, 1, 20);
        assert_eq!(resp.total_pages, 5);
    }
}
