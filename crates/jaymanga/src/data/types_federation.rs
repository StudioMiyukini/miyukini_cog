//! Types pour l'API de fédération inter-COG JayManga.
//!
//! Utilisés par l'API `/api/jaymanga/federation/*` pour exposer un résumé
//! du catalogue d'un vendeur aux Portails Agrégés distants.

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Information vendeur fédérée
// ---------------------------------------------------------------------------

/// Informations du vendeur exposées via l'API de fédération.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FederationInfo {
    pub cog_id: Option<String>,
    pub shop_name: Option<String>,
    pub shop_description: Option<String>,
    pub work_count: Option<i32>,
    pub free_work_count: Option<i32>,
    pub formats: Option<String>,              // JSON Vec<String>
    pub genres: Option<String>,               // JSON Vec<String>
    pub languages: Option<String>,            // JSON Vec<String>
    pub allow_aggregation: Option<bool>,
    pub federation_api_version: Option<String>,
    pub last_catalog_update: Option<String>,
}

// ---------------------------------------------------------------------------
// Entrée catalogue fédérée
// ---------------------------------------------------------------------------

/// Résumé d'une oeuvre exposé via l'API de fédération.
/// Contient les métadonnées publiques sans les fichiers.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FederationCatalogEntry {
    pub work_id: Option<String>,
    pub title: Option<String>,
    pub authors: Option<String>,              // JSON
    pub genres: Option<String>,               // JSON
    pub synopsis: Option<String>,             // tronqué à federation_synopsis_length
    pub cover_thumb_url: Option<String>,
    pub reading_format: Option<String>,
    pub pricing_model: Option<String>,
    pub price: Option<i64>,                   // centimes (si federation_include_prices)
    pub currency: Option<String>,
    pub chapter_count: Option<i32>,
    pub total_pages: Option<i32>,
    pub demo_pages_count: Option<i32>,
    pub series_id: Option<String>,
    pub series_title: Option<String>,
    pub tags: Option<String>,                 // JSON
    pub language: Option<String>,
    pub portal_url: Option<String>,
    pub published_at: Option<String>,
    pub updated_at: Option<String>,
}

/// Réponse de l'API de fédération catalogue.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FederationCatalogResponse {
    pub info: FederationInfo,
    pub entries: Vec<FederationCatalogEntry>,
    pub total_count: i64,
    pub has_more: bool,
}

/// Réponse delta de l'API de fédération.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FederationDeltaResponse {
    pub since: String,
    pub added: Vec<FederationCatalogEntry>,
    pub updated: Vec<FederationCatalogEntry>,
    pub removed_work_ids: Vec<String>,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_federation_info_serde() {
        let info = FederationInfo {
            cog_id: Some("cog-abc-123".to_string()),
            shop_name: Some("Ma Librairie Manga".to_string()),
            work_count: Some(42),
            allow_aggregation: Some(true),
            ..Default::default()
        };
        let json = serde_json::to_string(&info).unwrap();
        let parsed: FederationInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.work_count, Some(42));
        assert_eq!(parsed.allow_aggregation, Some(true));
    }

    #[test]
    fn test_federation_catalog_response_empty() {
        let resp = FederationCatalogResponse::default();
        assert_eq!(resp.total_count, 0);
        assert!(resp.entries.is_empty());
    }
}
