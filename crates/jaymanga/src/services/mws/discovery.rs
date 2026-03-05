//! Découverte de COGs JayManga via les trackers MWS.
//!
//! Interroge les trackers pour trouver les COGs qui publient
//! un manifeste de service "jaymanga", utilisé par le Portail Agrégé
//! pour la collecte de catalogues.

// ---------------------------------------------------------------------------
// Erreurs
// ---------------------------------------------------------------------------

/// Erreur du module découverte MWS.
#[derive(Debug, thiserror::Error)]
pub enum MwsDiscoveryError {
    #[error("MWS connection error: {0}")]
    Connection(String),
    #[error("No trackers available")]
    NoTrackers,
    #[error("Query error: {0}")]
    Query(String),
}

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/// COG découvert avec un service JayManga actif.
#[derive(Debug, Clone)]
pub struct DiscoveredCog {
    pub cog_id: String,
    pub shop_name: String,
    pub work_count: i32,
    pub allow_aggregation: bool,
    pub federation_api_base: Option<String>,
    pub last_seen: String,
}

/// Résultat d'une requête de découverte.
#[derive(Debug, Clone, Default)]
pub struct DiscoveryResult {
    pub cogs: Vec<DiscoveredCog>,
    pub tracker_queried: String,
    pub query_time_ms: u64,
}

// ---------------------------------------------------------------------------
// Filtrage
// ---------------------------------------------------------------------------

/// Filtre les COGs découverts pour ne garder que ceux qui autorisent
/// l'agrégation et ne sont pas bloqués.
#[must_use]
pub fn discovery_filter_aggregable(
    cogs: &[DiscoveredCog],
    blocked_cog_ids: &[String],
) -> Vec<DiscoveredCog> {
    cogs.iter()
        .filter(|c| c.allow_aggregation && !blocked_cog_ids.contains(&c.cog_id))
        .cloned()
        .collect()
}

/// Trie les COGs découverts par nombre d'oeuvres (décroissant).
#[must_use]
pub fn discovery_sort_by_catalog_size(mut cogs: Vec<DiscoveredCog>) -> Vec<DiscoveredCog> {
    cogs.sort_by(|a, b| b.work_count.cmp(&a.work_count));
    cogs
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn make_cog(id: &str, works: i32, aggregable: bool) -> DiscoveredCog {
        DiscoveredCog {
            cog_id: id.to_string(),
            shop_name: format!("Shop {id}"),
            work_count: works,
            allow_aggregation: aggregable,
            federation_api_base: Some("/api/jaymanga/federation".to_string()),
            last_seen: "2026-02-25T10:00:00Z".to_string(),
        }
    }

    #[test]
    fn test_filter_aggregable() {
        let cogs = vec![
            make_cog("cog-1", 10, true),
            make_cog("cog-2", 5, false),
            make_cog("cog-3", 20, true),
        ];
        let filtered = discovery_filter_aggregable(&cogs, &[]);
        assert_eq!(filtered.len(), 2);
        assert!(filtered.iter().all(|c| c.allow_aggregation));
    }

    #[test]
    fn test_filter_blocked() {
        let cogs = vec![make_cog("cog-1", 10, true), make_cog("cog-2", 5, true)];
        let blocked = vec!["cog-1".to_string()];
        let filtered = discovery_filter_aggregable(&cogs, &blocked);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].cog_id, "cog-2");
    }

    #[test]
    fn test_sort_by_catalog_size() {
        let cogs = vec![
            make_cog("small", 5, true),
            make_cog("big", 50, true),
            make_cog("medium", 20, true),
        ];
        let sorted = discovery_sort_by_catalog_size(cogs);
        assert_eq!(sorted[0].cog_id, "big");
        assert_eq!(sorted[1].cog_id, "medium");
        assert_eq!(sorted[2].cog_id, "small");
    }
}
