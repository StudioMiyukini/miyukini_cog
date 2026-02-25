//! Endpoints REST de la fédération JayManga (opt-in).
//!
//! API exposée aux COGs agrégateurs pour collecter le catalogue
//! public du vendeur. Protégée par le flag `allow_aggregation`.

use serde::Serialize;

// ---------------------------------------------------------------------------
// Types de réponse
// ---------------------------------------------------------------------------

/// Informations vendeur pour la fédération.
#[derive(Debug, Clone, Serialize)]
pub struct FederationInfoResponse {
    pub shop_name: String,
    pub shop_description: Option<String>,
    pub work_count: i32,
    pub free_work_count: i32,
    pub formats: Vec<String>,
    pub genres: Vec<String>,
    pub languages: Vec<String>,
    pub last_catalog_update: String,
}

/// Entrée de catalogue fédéré.
#[derive(Debug, Clone, Serialize)]
pub struct FederationCatalogEntry {
    pub work_id: String,
    pub title: String,
    pub authors_json: String,
    pub genres_json: String,
    pub synopsis: Option<String>,
    pub cover_url: Option<String>,
    pub reading_format: String,
    pub pricing_model: String,
    pub price: i64,
    pub currency: String,
    pub chapter_count: i32,
    pub total_pages: i32,
    pub demo_pages_count: i32,
    pub series_title: Option<String>,
    pub tags_json: Option<String>,
    pub language: Option<String>,
    pub published_at: String,
    pub updated_at: String,
}

/// Réponse du catalogue fédéré (avec support delta).
#[derive(Debug, Clone, Serialize)]
pub struct FederationCatalogResponse {
    pub entries: Vec<FederationCatalogEntry>,
    pub total: i32,
    pub since: Option<String>,
    pub generated_at: String,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Vérifie si la fédération est autorisée pour ce COG.
#[must_use]
pub fn federation_is_allowed(allow_aggregation: bool) -> bool {
    allow_aggregation
}

/// Filtre les entrées du catalogue modifiées depuis un timestamp.
#[must_use]
pub fn federation_filter_since(
    entries: Vec<FederationCatalogEntry>,
    since: &str,
) -> Vec<FederationCatalogEntry> {
    entries
        .into_iter()
        .filter(|e| e.updated_at.as_str() > since)
        .collect()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_federation_allowed() {
        assert!(federation_is_allowed(true));
        assert!(!federation_is_allowed(false));
    }

    #[test]
    fn test_filter_since() {
        let entries = vec![
            FederationCatalogEntry {
                work_id: "old".to_string(),
                updated_at: "2026-01-01T00:00:00Z".to_string(),
                title: String::new(),
                authors_json: "[]".to_string(),
                genres_json: "[]".to_string(),
                synopsis: None,
                cover_url: None,
                reading_format: "manga".to_string(),
                pricing_model: "free".to_string(),
                price: 0,
                currency: "EUR".to_string(),
                chapter_count: 0,
                total_pages: 0,
                demo_pages_count: 0,
                series_title: None,
                tags_json: None,
                language: None,
                published_at: String::new(),
            },
            FederationCatalogEntry {
                work_id: "new".to_string(),
                updated_at: "2026-02-25T00:00:00Z".to_string(),
                title: String::new(),
                authors_json: "[]".to_string(),
                genres_json: "[]".to_string(),
                synopsis: None,
                cover_url: None,
                reading_format: "manga".to_string(),
                pricing_model: "free".to_string(),
                price: 0,
                currency: "EUR".to_string(),
                chapter_count: 0,
                total_pages: 0,
                demo_pages_count: 0,
                series_title: None,
                tags_json: None,
                language: None,
                published_at: String::new(),
            },
        ];
        let filtered = federation_filter_since(entries, "2026-02-01T00:00:00Z");
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].work_id, "new");
    }
}
