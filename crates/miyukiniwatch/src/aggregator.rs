//! Opérateur Aggregator — agrégation périodique et purge.

use crate::aggregates::compute_aggregates;
use crate::data::MiyukiniWatchDb;
use crate::errors::MiyukiniWatchError;
use chrono::{Local, Duration};
use std::sync::Arc;

/// Agrège les métriques et exécute les purges selon la rétention.
pub struct MiyukiniWatchAggregator {
    db: Arc<MiyukiniWatchDb>,
}

impl MiyukiniWatchAggregator {
    /// Crée un nouvel agrégateur.
    #[must_use]
    pub fn new(db: Arc<MiyukiniWatchDb>) -> Self {
        Self { db }
    }

    /// Calcule et retourne les agrégats pour Miou.
    pub fn get_aggregates(
        &self,
        profile_id: &str,
    ) -> Result<Vec<crate::aggregates::Aggregate>, MiyukiniWatchError> {
        compute_aggregates(&self.db, profile_id)
    }

    /// Purge les métriques brutes expirées (première session du jour).
    pub fn purge_expired_raw(&self, profile_id: &str) -> Result<i64, MiyukiniWatchError> {
        let prefs = self.db.get_prefs(profile_id)?;
        let cutoff = Local::now() - Duration::days(prefs.retention_raw_days as i64);
        let to_ts = cutoff.to_rfc3339();
        let deleted = self.db.purge_raw_before(profile_id, &to_ts)?;

        if deleted > 0 {
            let _ = self.db.insert_audit(
                profile_id,
                "purge_automatic",
                Some(&format!("{} métriques brutes expirées", deleted)),
                Some(deleted),
            );
        }
        Ok(deleted)
    }
}
