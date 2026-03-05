//! Opérateur Collector — collecte passive des métriques.

use crate::data::{MetricRecord, MiyukiniWatchDb, Prefs, TimeSlot};
use crate::errors::MiyukiniWatchError;
use chrono::{Local, Timelike};
use std::sync::Arc;
use uuid::Uuid;

/// Limite métriques par session (configurable via TAMR).
const MAX_METRICS_PER_SESSION: i64 = 10_000;

/// Collecteur passif de métriques. Ne provoque jamais d'événement.
pub struct MiyukiniWatchCollector {
    db: Arc<MiyukiniWatchDb>,
}

impl MiyukiniWatchCollector {
    /// Crée un nouveau collecteur.
    #[must_use]
    pub fn new(db: Arc<MiyukiniWatchDb>) -> Self {
        Self { db }
    }

    /// Enregistre une métrique si la collecte est activée.
    /// Non bloquant : l'écriture est synchrone mais légère.
    pub fn record(
        &self,
        profile_id: &str,
        record: &MetricRecord,
    ) -> Result<(), MiyukiniWatchError> {
        let prefs = self.db.get_prefs(profile_id)?;
        if !prefs.collect_enabled {
            return Err(MiyukiniWatchError::CollectionDisabled);
        }

        if !Self::category_allowed(&record.metric_id, &prefs) {
            return Err(MiyukiniWatchError::CategoryDisabled(
                Self::metric_category(&record.metric_id).to_string(),
            ));
        }

        let count = self
            .db
            .count_metrics_for_session(profile_id, &record.session_id)?;
        if count >= MAX_METRICS_PER_SESSION {
            return Err(MiyukiniWatchError::VolumeLimitReached);
        }

        self.db.insert_metric(record)
    }

    fn category_allowed(metric_id: &str, prefs: &Prefs) -> bool {
        match Self::metric_category(metric_id) {
            "sessions" => prefs.collect_sessions,
            "services" => prefs.collect_services,
            "friends" => prefs.collect_friends,
            "activity" => prefs.collect_activity,
            "lifecycle" => true,
            _ => true,
        }
    }

    fn metric_category(metric_id: &str) -> &'static str {
        if metric_id.starts_with('S') {
            "sessions"
        } else if metric_id.starts_with("SV") {
            "services"
        } else if metric_id.starts_with('A') {
            "friends"
        } else if metric_id.starts_with('I') {
            "activity"
        } else if metric_id.starts_with('L') {
            "lifecycle"
        } else {
            "other"
        }
    }
}

/// Helpers pour émettre des métriques depuis Central.
impl MiyukiniWatchCollector {
    /// Session démarrée (S-01).
    pub fn record_session_start(
        &self,
        profile_id: &str,
        session_id: &str,
    ) -> Result<(), MiyukiniWatchError> {
        let slot = TimeSlot::from_hour(Local::now().hour());
        let record = MetricRecord {
            record_id: Uuid::new_v4().to_string(),
            profile_id: profile_id.to_string(),
            metric_id: "S-01".to_string(),
            session_id: session_id.to_string(),
            timestamp: Local::now(),
            service_id: None,
            friend_id: None,
            value_int: None,
            value_str: Some(format!("{:?}", slot)),
        };
        self.record(profile_id, &record)
    }

    /// Session terminée (S-02, S-03).
    pub fn record_session_end(
        &self,
        profile_id: &str,
        session_id: &str,
        duration_seconds: i64,
    ) -> Result<(), MiyukiniWatchError> {
        let now = Local::now();
        let record_end = MetricRecord {
            record_id: Uuid::new_v4().to_string(),
            profile_id: profile_id.to_string(),
            metric_id: "S-02".to_string(),
            session_id: session_id.to_string(),
            timestamp: now,
            service_id: None,
            friend_id: None,
            value_int: None,
            value_str: None,
        };
        self.record(profile_id, &record_end)?;

        let record_dur = MetricRecord {
            record_id: Uuid::new_v4().to_string(),
            profile_id: profile_id.to_string(),
            metric_id: "S-03".to_string(),
            session_id: session_id.to_string(),
            timestamp: now,
            service_id: None,
            friend_id: None,
            value_int: Some(duration_seconds),
            value_str: None,
        };
        self.record(profile_id, &record_dur)
    }

    /// Service ouvert (SV-01).
    pub fn record_service_opened(
        &self,
        profile_id: &str,
        session_id: &str,
        service_id: &str,
    ) -> Result<(), MiyukiniWatchError> {
        let record = MetricRecord {
            record_id: Uuid::new_v4().to_string(),
            profile_id: profile_id.to_string(),
            metric_id: "SV-01".to_string(),
            session_id: session_id.to_string(),
            timestamp: Local::now(),
            service_id: Some(service_id.to_string()),
            friend_id: None,
            value_int: None,
            value_str: None,
        };
        self.record(profile_id, &record)
    }

    /// Service fermé (SV-02, SV-03).
    pub fn record_service_closed(
        &self,
        profile_id: &str,
        session_id: &str,
        service_id: &str,
        duration_seconds: i64,
    ) -> Result<(), MiyukiniWatchError> {
        let now = Local::now();
        let record_end = MetricRecord {
            record_id: Uuid::new_v4().to_string(),
            profile_id: profile_id.to_string(),
            metric_id: "SV-02".to_string(),
            session_id: session_id.to_string(),
            timestamp: now,
            service_id: Some(service_id.to_string()),
            friend_id: None,
            value_int: None,
            value_str: None,
        };
        self.record(profile_id, &record_end)?;

        let record_dur = MetricRecord {
            record_id: Uuid::new_v4().to_string(),
            profile_id: profile_id.to_string(),
            metric_id: "SV-03".to_string(),
            session_id: session_id.to_string(),
            timestamp: now,
            service_id: Some(service_id.to_string()),
            friend_id: None,
            value_int: Some(duration_seconds),
            value_str: None,
        };
        self.record(profile_id, &record_dur)
    }

    /// Clic utilisateur (I-01) — agrégat uniquement, pas de cible.
    pub fn record_click(
        &self,
        profile_id: &str,
        session_id: &str,
    ) -> Result<(), MiyukiniWatchError> {
        let record = MetricRecord {
            record_id: Uuid::new_v4().to_string(),
            profile_id: profile_id.to_string(),
            metric_id: "I-01".to_string(),
            session_id: session_id.to_string(),
            timestamp: Local::now(),
            service_id: None,
            friend_id: None,
            value_int: Some(1),
            value_str: None,
        };
        self.record(profile_id, &record)
    }
}
