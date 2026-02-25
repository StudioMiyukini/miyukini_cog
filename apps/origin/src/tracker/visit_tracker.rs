//! @id: origin.tracker.visit_tracker
//! @do: track_catalog_to_home_visits_for_presence
//! @role: data
//! @layer: domain
//! @human: Suivi des visites catalogue → Home COG pour présence.
//!
//! Permet au Tracker de considérer un COG comme « présent » si un utilisateur
//! est passé par le catalogue vers le Home du COG récemment (~1 min), en plus du heartbeat.

use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Durée en secondes pendant laquelle une visite catalogue → Home garde le COG considéré présent.
pub const CATALOG_VISIT_PRESENT_SECONDS: i64 = 60;

/// Enregistre la dernière visite (clic « Visiter » depuis le catalogue) par COG.
#[derive(Debug, Default)]
pub struct CatalogVisitTracker {
    /// Dernière visite par cog_id.
    last_visit: Arc<RwLock<HashMap<String, DateTime<Utc>>>>,
}

impl CatalogVisitTracker {
    /// Crée un nouveau suivi.
    #[must_use]
    pub fn new() -> Self {
        Self {
            last_visit: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Enregistre une visite vers le Home du COG (utilisateur passé par le catalogue).
    pub async fn record_visit(&self, cog_id: &str) {
        let mut map = self.last_visit.write().await;
        map.insert(cog_id.to_string(), Utc::now());
    }

    /// Retourne la date de la dernière visite catalogue → Home pour ce COG.
    pub async fn get_last_visit(&self, cog_id: &str) -> Option<DateTime<Utc>> {
        let map = self.last_visit.read().await;
        map.get(cog_id).copied()
    }

    /// Indique si une visite récente (dans les CATALOG_VISIT_PRESENT_SECONDS) existe pour ce COG.
    pub async fn has_recent_visit(&self, cog_id: &str) -> bool {
        let last = self.get_last_visit(cog_id).await;
        match last {
            Some(t) => (Utc::now() - t).num_seconds() <= CATALOG_VISIT_PRESENT_SECONDS,
            None => false,
        }
    }
}
