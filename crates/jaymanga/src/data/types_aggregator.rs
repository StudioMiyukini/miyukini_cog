//! Types de domaine pour l'agrégation inter-COG JayManga.
//!
//! Définit : AggregatedCatalogEntry, IndexedSeller, AggregatorConfig,
//! OnlineStatus.

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Entrée catalogue agrégée
// ---------------------------------------------------------------------------

/// Entrée en cache du catalogue d'un COG vendeur distant.
/// Stockée sur le COG agrégateur.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AggregatedCatalogEntry {
    pub id: Option<String>,
    pub seller_cog_id: Option<String>,
    pub work_id: Option<String>,
    pub title: Option<String>,
    pub authors: Option<String>,              // JSON
    pub genres: Option<String>,               // JSON
    pub synopsis: Option<String>,
    pub cover_thumb_path: Option<String>,
    pub reading_format: Option<String>,
    pub pricing_model: Option<String>,
    pub price: Option<i64>,
    pub currency: Option<String>,
    pub chapter_count: Option<i32>,
    pub total_pages: Option<i32>,
    pub demo_pages_count: Option<i32>,
    pub series_title: Option<String>,
    pub tags: Option<String>,                 // JSON
    pub language: Option<String>,
    pub portal_url: Option<String>,
    pub published_at: Option<String>,
    pub updated_at: Option<String>,
    #[serde(default)]
    pub cached_at: Option<String>,
}

// ---------------------------------------------------------------------------
// Vendeur indexé
// ---------------------------------------------------------------------------

/// Vendeur connu du Portail Agrégé.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IndexedSeller {
    pub cog_id: Option<String>,
    pub shop_name: Option<String>,
    pub shop_description: Option<String>,
    pub avatar_path: Option<String>,
    pub work_count: Option<i32>,
    pub online_status: Option<String>,        // OnlineStatus as_str
    #[serde(default)]
    pub last_synced_at: Option<String>,
    pub last_seen_online_at: Option<String>,
    pub blocked: Option<bool>,
}

/// Statut de présence en ligne d'un COG.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OnlineStatus {
    Online,
    Offline,
    Unknown,
}

impl OnlineStatus {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Online => "online",
            Self::Offline => "offline",
            Self::Unknown => "unknown",
        }
    }

    #[must_use]
    pub fn from_str(s: &str) -> Self {
        match s {
            "online" => Self::Online,
            "offline" => Self::Offline,
            _ => Self::Unknown,
        }
    }
}

// ---------------------------------------------------------------------------
// Configuration agrégateur
// ---------------------------------------------------------------------------

/// Configuration du Portail Agrégé.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AggregatorConfig {
    pub enabled: Option<bool>,
    pub name: Option<String>,
    pub sync_interval_minutes: Option<i32>,
    pub presence_refresh_minutes: Option<i32>,
    pub max_indexed_cogs: Option<i32>,
    pub show_offline_works: Option<bool>,
    pub highlight_own_catalog: Option<bool>,
    pub blocked_cogs: Option<String>,         // JSON Vec<String>
}

impl AggregatorConfig {
    /// Retourne une configuration agrégateur par défaut.
    #[must_use]
    pub fn defaults() -> Self {
        Self {
            enabled: Some(false),
            name: Some("Portail Agrégé JayManga".to_string()),
            sync_interval_minutes: Some(30),
            presence_refresh_minutes: Some(5),
            max_indexed_cogs: Some(500),
            show_offline_works: Some(true),
            highlight_own_catalog: Some(true),
            blocked_cogs: Some("[]".to_string()),
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_online_status_roundtrip() {
        for s in [OnlineStatus::Online, OnlineStatus::Offline, OnlineStatus::Unknown] {
            assert_eq!(OnlineStatus::from_str(s.as_str()), s);
        }
    }

    #[test]
    fn test_aggregator_config_defaults() {
        let cfg = AggregatorConfig::defaults();
        assert_eq!(cfg.enabled, Some(false));
        assert_eq!(cfg.sync_interval_minutes, Some(30));
        assert_eq!(cfg.max_indexed_cogs, Some(500));
    }
}
