//! Logique d'agrégation inter-COG JayManga (Portail Agrégé).
//!
//! Collecte les catalogues des COGs JayManga via le MWS,
//! les met en cache localement, et fournit une interface de recherche
//! et de recommandation unifiée.

use crate::data::*;

// ---------------------------------------------------------------------------
// Erreurs
// ---------------------------------------------------------------------------

/// Erreur du module agrégation.
#[derive(Debug, thiserror::Error)]
pub enum AggregatorError {
    #[error("Database error: {0}")]
    Db(#[from] DbError),
    #[error("MWS error: {0}")]
    Mws(String),
    #[error("Sync error: {0}")]
    Sync(String),
    #[error("Aggregator not enabled")]
    NotEnabled,
}

// ---------------------------------------------------------------------------
// Configuration agrégateur
// ---------------------------------------------------------------------------

/// Configuration du Portail Agrégé.
#[derive(Debug, Clone)]
pub struct AggregatorConfig {
    pub enabled: bool,
    pub name: String,
    pub sync_interval_minutes: i32,
    pub presence_refresh_minutes: i32,
    pub max_indexed_cogs: i32,
    pub show_offline_works: bool,
    pub highlight_own_catalog: bool,
    pub blocked_cogs: Vec<String>,
}

impl Default for AggregatorConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            name: "Portail Manga".to_string(),
            sync_interval_minutes: 120,
            presence_refresh_minutes: 15,
            max_indexed_cogs: 500,
            show_offline_works: true,
            highlight_own_catalog: true,
            blocked_cogs: Vec::new(),
        }
    }
}

/// Rapport de synchronisation.
#[derive(Debug, Clone, Default)]
pub struct SyncReport {
    pub cogs_discovered: i32,
    pub cogs_synced: i32,
    pub entries_added: i32,
    pub entries_updated: i32,
    pub errors: Vec<String>,
}

// ---------------------------------------------------------------------------
// Recherche et recommandations
// ---------------------------------------------------------------------------

/// Score de similarité entre deux oeuvres agrégées.
#[must_use]
pub fn aggregator_similarity_score(
    work_a: &AggregatedCatalogEntry,
    work_b: &AggregatedCatalogEntry,
) -> f64 {
    let mut score = 0.0;

    // Genres en commun
    let genres_a = parse_json_vec(work_a.genres.as_deref());
    let genres_b = parse_json_vec(work_b.genres.as_deref());
    let common_genres = genres_a
        .iter()
        .filter(|g| genres_b.contains(g))
        .count();
    if !genres_a.is_empty() {
        score += (common_genres as f64 / genres_a.len() as f64) * 40.0;
    }

    // Tags en commun
    let tags_a = parse_json_vec(work_a.tags.as_deref());
    let tags_b = parse_json_vec(work_b.tags.as_deref());
    let common_tags = tags_a.iter().filter(|t| tags_b.contains(t)).count();
    if !tags_a.is_empty() {
        score += (common_tags as f64 / tags_a.len() as f64) * 30.0;
    }

    // Même format de lecture
    if work_a.reading_format == work_b.reading_format {
        score += 15.0;
    }

    // Même langue
    if work_a.language == work_b.language && work_a.language.is_some() {
        score += 15.0;
    }

    score
}

/// Vérifie si un COG est bloqué par l'agrégateur.
#[must_use]
pub fn aggregator_is_cog_blocked(config: &AggregatorConfig, cog_id: &str) -> bool {
    config.blocked_cogs.iter().any(|id| id == cog_id)
}

/// Vérifie si la fraîcheur du cache est acceptable.
#[must_use]
pub fn aggregator_cache_is_fresh(
    cached_at: &str,
    now: &str,
    max_age_minutes: i32,
) -> bool {
    let cached = chrono::DateTime::parse_from_rfc3339(cached_at).ok();
    let current = chrono::DateTime::parse_from_rfc3339(now).ok();

    match (cached, current) {
        (Some(c), Some(n)) => {
            let age = n.signed_duration_since(c);
            age.num_minutes() < i64::from(max_age_minutes)
        }
        _ => false,
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn parse_json_vec(json: Option<&str>) -> Vec<String> {
    json.and_then(|s| serde_json::from_str::<Vec<String>>(s).ok())
        .unwrap_or_default()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn make_entry(genres: &str, tags: &str, format: &str, lang: &str) -> AggregatedCatalogEntry {
        AggregatedCatalogEntry {
            genres: Some(genres.to_string()),
            tags: Some(tags.to_string()),
            reading_format: Some(format.to_string()),
            language: Some(lang.to_string()),
            ..Default::default()
        }
    }

    #[test]
    fn test_similarity_identical() {
        let a = make_entry(
            r#"["action","fantasy"]"#,
            r#"["sword","magic"]"#,
            "manga",
            "ja",
        );
        let b = a.clone();
        let score = aggregator_similarity_score(&a, &b);
        assert!((score - 100.0).abs() < 0.01);
    }

    #[test]
    fn test_similarity_different() {
        let a = make_entry(r#"["action"]"#, r#"["sword"]"#, "manga", "ja");
        let b = make_entry(r#"["romance"]"#, r#"["school"]"#, "webtoon", "ko");
        let score = aggregator_similarity_score(&a, &b);
        assert!(score < 10.0);
    }

    #[test]
    fn test_cog_blocked() {
        let config = AggregatorConfig {
            blocked_cogs: vec!["bad-cog".to_string()],
            ..Default::default()
        };
        assert!(aggregator_is_cog_blocked(&config, "bad-cog"));
        assert!(!aggregator_is_cog_blocked(&config, "good-cog"));
    }

    #[test]
    fn test_cache_freshness() {
        assert!(aggregator_cache_is_fresh(
            "2026-02-25T10:00:00Z",
            "2026-02-25T10:30:00Z",
            60,
        ));
        assert!(!aggregator_cache_is_fresh(
            "2026-02-25T08:00:00Z",
            "2026-02-25T10:30:00Z",
            60,
        ));
    }

    #[test]
    fn test_default_config() {
        let config = AggregatorConfig::default();
        assert!(!config.enabled);
        assert_eq!(config.sync_interval_minutes, 120);
        assert!(config.blocked_cogs.is_empty());
    }
}
