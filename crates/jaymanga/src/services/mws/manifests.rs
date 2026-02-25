//! Publication et mise à jour des manifestes JayManga sur le Tracker MWS.
//!
//! Le COG vendeur publie un manifeste de service décrivant sa boutique manga
//! (nom, nombre d'oeuvres, formats, genres, langues, opt-in agrégation).
//! Ce manifeste est mis à jour automatiquement lors de changements significatifs.

// ---------------------------------------------------------------------------
// Erreurs
// ---------------------------------------------------------------------------

/// Erreur du module manifestes MWS.
#[derive(Debug, thiserror::Error)]
pub enum MwsManifestError {
    #[error("Failed to build manifest: {0}")]
    Build(String),
    #[error("Failed to publish manifest: {0}")]
    Publish(String),
    #[error("MWS connection error: {0}")]
    Connection(String),
}

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/// Payload du manifeste JayManga publié sur le Tracker.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct JayMangaManifestPayload {
    pub service_id: String,
    pub shop_name: String,
    pub shop_description: Option<String>,
    pub work_count: i32,
    pub free_work_count: i32,
    pub formats: Vec<String>,
    pub genres: Vec<String>,
    pub languages: Vec<String>,
    pub allow_aggregation: bool,
    pub federation_api_base: Option<String>,
    pub last_catalog_update: String,
}

impl Default for JayMangaManifestPayload {
    fn default() -> Self {
        Self {
            service_id: "jaymanga".to_string(),
            shop_name: String::new(),
            shop_description: None,
            work_count: 0,
            free_work_count: 0,
            formats: Vec::new(),
            genres: Vec::new(),
            languages: Vec::new(),
            allow_aggregation: true,
            federation_api_base: None,
            last_catalog_update: String::new(),
        }
    }
}

// ---------------------------------------------------------------------------
// Construction du manifeste
// ---------------------------------------------------------------------------

/// Construit le payload du manifeste JayManga à partir de la DB.
///
/// Cette fonction sera connectée à `JayMangaDb` lorsque les méthodes
/// d'agrégation seront disponibles.
#[cfg(feature = "legacy-sqlite")]
pub fn manifest_build_payload(
    db: &crate::data::JayMangaDb,
) -> Result<JayMangaManifestPayload, MwsManifestError> {
    use crate::data::WorkFilters;

    let config = db
        .seller_config_get()
        .map_err(|e| MwsManifestError::Build(e.to_string()))?;

    let shop_name = config
        .as_ref()
        .and_then(|c| c.shop_name.clone())
        .unwrap_or_default();
    let shop_description = config.as_ref().and_then(|c| c.shop_description.clone());
    let allow_aggregation = config
        .as_ref()
        .and_then(|c| c.allow_aggregation)
        .unwrap_or(true);

    // Compter les oeuvres publiées
    let published_works = db
        .work_list(&WorkFilters {
            status: Some("published".to_string()),
            ..Default::default()
        })
        .map_err(|e| MwsManifestError::Build(e.to_string()))?;

    let work_count = published_works.len() as i32;
    let free_work_count = published_works
        .iter()
        .filter(|w| w.pricing_model.as_deref() == Some("free"))
        .count() as i32;

    // Extraire les formats, genres et langues uniques
    let mut formats = Vec::new();
    let mut genres_set = Vec::new();
    let mut languages = Vec::new();

    for work in &published_works {
        if let Some(fmt) = &work.reading_format {
            if !formats.contains(fmt) {
                formats.push(fmt.clone());
            }
        }
        if let Some(lang) = &work.language {
            if !lang.is_empty() && !languages.contains(lang) {
                languages.push(lang.clone());
            }
        }
        if let Some(genres_json) = &work.genres {
            if let Ok(gs) = serde_json::from_str::<Vec<String>>(genres_json) {
                for g in gs {
                    if !genres_set.contains(&g) {
                        genres_set.push(g);
                    }
                }
            }
        }
    }

    Ok(JayMangaManifestPayload {
        service_id: "jaymanga".to_string(),
        shop_name,
        shop_description,
        work_count,
        free_work_count,
        formats,
        genres: genres_set,
        languages,
        allow_aggregation,
        federation_api_base: Some("/api/jaymanga/federation".to_string()),
        last_catalog_update: chrono::Utc::now().to_rfc3339(),
    })
}

/// Détermine si le manifeste doit être mis à jour suite à un événement.
#[must_use]
pub fn manifest_should_update(event: &str) -> bool {
    matches!(
        event,
        "work_published"
            | "work_archived"
            | "work_deleted"
            | "seller_config_changed"
            | "aggregation_toggled"
            | "periodic_refresh"
    )
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_payload() {
        let payload = JayMangaManifestPayload::default();
        assert_eq!(payload.service_id, "jaymanga");
        assert_eq!(payload.work_count, 0);
        assert!(payload.allow_aggregation);
    }

    #[test]
    fn test_should_update() {
        assert!(manifest_should_update("work_published"));
        assert!(manifest_should_update("seller_config_changed"));
        assert!(!manifest_should_update("page_read"));
        assert!(!manifest_should_update("unknown_event"));
    }

    #[test]
    fn test_payload_serialization() {
        let payload = JayMangaManifestPayload {
            shop_name: "Manga Shop".to_string(),
            work_count: 42,
            ..Default::default()
        };
        let json = serde_json::to_string(&payload).unwrap();
        assert!(json.contains("Manga Shop"));
        assert!(json.contains("42"));
    }
}
