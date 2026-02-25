//! Métriques du Tracker.
//!
//! Statistiques et compteurs pour le monitoring.
//!
//! Note : Code préparé pour fonctionnalités futures (métriques avancées).
#![allow(dead_code)]

use std::sync::atomic::{AtomicU64, Ordering};

/// Métriques du Tracker.
#[derive(Debug)]
pub struct TrackerMetrics {
    // Connexions
    /// Total des connexions.
    pub total_connections: AtomicU64,
    /// Connexions actives.
    pub active_connections: AtomicU64,

    // Annonces
    /// Total des annonces reçues.
    pub total_announces: AtomicU64,
    /// Annonces réussies.
    pub successful_announces: AtomicU64,
    /// Annonces rejetées.
    pub rejected_announces: AtomicU64,

    // Recherches
    /// Recherches de COGs.
    pub cog_searches: AtomicU64,
    /// Recherches de lobbys.
    pub lobby_searches: AtomicU64,
    /// Recherches de services.
    pub service_searches: AtomicU64,

    // Lobbys
    /// Lobbys créés.
    pub lobbys_created: AtomicU64,
    /// Lobbys supprimés.
    pub lobbys_deleted: AtomicU64,
    /// Demandes de join.
    pub join_requests: AtomicU64,
    /// Joins réussis.
    pub successful_joins: AtomicU64,
    /// Joins échoués.
    pub failed_joins: AtomicU64,

    // Heartbeats
    /// Heartbeats reçus.
    pub heartbeats_received: AtomicU64,

    // COGs
    /// COGs enregistrés (total historique).
    pub total_cogs_registered: AtomicU64,
    /// COGs retirés.
    pub cogs_withdrawn: AtomicU64,
    /// COGs expirés.
    pub cogs_expired: AtomicU64,

    // Erreurs
    /// Erreurs de protocole.
    pub protocol_errors: AtomicU64,
    /// Erreurs internes.
    pub internal_errors: AtomicU64,

    // Bytes
    /// Bytes reçus.
    pub bytes_received: AtomicU64,
    /// Bytes envoyés.
    pub bytes_sent: AtomicU64,

    // Timestamps
    /// Timestamp du démarrage.
    start_time: chrono::DateTime<chrono::Utc>,
}

impl TrackerMetrics {
    /// Crée de nouvelles métriques.
    #[must_use]
    pub fn new() -> Self {
        Self {
            total_connections: AtomicU64::new(0),
            active_connections: AtomicU64::new(0),
            total_announces: AtomicU64::new(0),
            successful_announces: AtomicU64::new(0),
            rejected_announces: AtomicU64::new(0),
            cog_searches: AtomicU64::new(0),
            lobby_searches: AtomicU64::new(0),
            service_searches: AtomicU64::new(0),
            lobbys_created: AtomicU64::new(0),
            lobbys_deleted: AtomicU64::new(0),
            join_requests: AtomicU64::new(0),
            successful_joins: AtomicU64::new(0),
            failed_joins: AtomicU64::new(0),
            heartbeats_received: AtomicU64::new(0),
            total_cogs_registered: AtomicU64::new(0),
            cogs_withdrawn: AtomicU64::new(0),
            cogs_expired: AtomicU64::new(0),
            protocol_errors: AtomicU64::new(0),
            internal_errors: AtomicU64::new(0),
            bytes_received: AtomicU64::new(0),
            bytes_sent: AtomicU64::new(0),
            start_time: chrono::Utc::now(),
        }
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Méthodes d'incrémentation
    // ─────────────────────────────────────────────────────────────────────────

    /// Enregistre une connexion.
    pub fn record_connection(&self) {
        self.total_connections.fetch_add(1, Ordering::Relaxed);
        self.active_connections.fetch_add(1, Ordering::Relaxed);
    }

    /// Enregistre la fermeture d'une connexion.
    pub fn record_connection_closed(&self) {
        self.active_connections.fetch_sub(1, Ordering::Relaxed);
    }

    /// Enregistre une annonce.
    pub fn record_announce(&self, success: bool) {
        self.total_announces.fetch_add(1, Ordering::Relaxed);
        if success {
            self.successful_announces.fetch_add(1, Ordering::Relaxed);
            self.total_cogs_registered.fetch_add(1, Ordering::Relaxed);
        } else {
            self.rejected_announces.fetch_add(1, Ordering::Relaxed);
        }
    }

    /// Enregistre une recherche de COGs.
    pub fn record_cog_search(&self) {
        self.cog_searches.fetch_add(1, Ordering::Relaxed);
    }

    /// Enregistre une recherche de lobbys.
    pub fn record_lobby_search(&self) {
        self.lobby_searches.fetch_add(1, Ordering::Relaxed);
    }

    /// Enregistre une recherche de services.
    pub fn record_service_search(&self) {
        self.service_searches.fetch_add(1, Ordering::Relaxed);
    }

    /// Enregistre la création d'un lobby.
    pub fn record_lobby_created(&self) {
        self.lobbys_created.fetch_add(1, Ordering::Relaxed);
    }

    /// Enregistre la suppression d'un lobby.
    pub fn record_lobby_deleted(&self) {
        self.lobbys_deleted.fetch_add(1, Ordering::Relaxed);
    }

    /// Enregistre une demande de join.
    pub fn record_join_request(&self, success: bool) {
        self.join_requests.fetch_add(1, Ordering::Relaxed);
        if success {
            self.successful_joins.fetch_add(1, Ordering::Relaxed);
        } else {
            self.failed_joins.fetch_add(1, Ordering::Relaxed);
        }
    }

    /// Enregistre un heartbeat.
    pub fn record_heartbeat(&self) {
        self.heartbeats_received.fetch_add(1, Ordering::Relaxed);
    }

    /// Enregistre le retrait d'un COG.
    pub fn record_cog_withdrawn(&self) {
        self.cogs_withdrawn.fetch_add(1, Ordering::Relaxed);
    }

    /// Enregistre l'expiration d'un COG.
    pub fn record_cog_expired(&self) {
        self.cogs_expired.fetch_add(1, Ordering::Relaxed);
    }

    /// Enregistre une erreur de protocole.
    pub fn record_protocol_error(&self) {
        self.protocol_errors.fetch_add(1, Ordering::Relaxed);
    }

    /// Enregistre une erreur interne.
    pub fn record_internal_error(&self) {
        self.internal_errors.fetch_add(1, Ordering::Relaxed);
    }

    /// Enregistre des bytes reçus.
    pub fn record_bytes_received(&self, bytes: usize) {
        self.bytes_received.fetch_add(bytes as u64, Ordering::Relaxed);
    }

    /// Enregistre des bytes envoyés.
    pub fn record_bytes_sent(&self, bytes: usize) {
        self.bytes_sent.fetch_add(bytes as u64, Ordering::Relaxed);
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Méthodes de lecture
    // ─────────────────────────────────────────────────────────────────────────

    /// Retourne l'uptime en secondes.
    #[must_use]
    pub fn uptime_seconds(&self) -> u64 {
        (chrono::Utc::now() - self.start_time).num_seconds() as u64
    }

    /// Convertit en JSON.
    #[must_use]
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "uptime_seconds": self.uptime_seconds(),
            "start_time": self.start_time.to_rfc3339(),
            "connections": {
                "total": self.total_connections.load(Ordering::Relaxed),
                "active": self.active_connections.load(Ordering::Relaxed),
            },
            "announces": {
                "total": self.total_announces.load(Ordering::Relaxed),
                "successful": self.successful_announces.load(Ordering::Relaxed),
                "rejected": self.rejected_announces.load(Ordering::Relaxed),
            },
            "searches": {
                "cogs": self.cog_searches.load(Ordering::Relaxed),
                "lobbys": self.lobby_searches.load(Ordering::Relaxed),
                "services": self.service_searches.load(Ordering::Relaxed),
            },
            "lobbys": {
                "created": self.lobbys_created.load(Ordering::Relaxed),
                "deleted": self.lobbys_deleted.load(Ordering::Relaxed),
            },
            "joins": {
                "total": self.join_requests.load(Ordering::Relaxed),
                "successful": self.successful_joins.load(Ordering::Relaxed),
                "failed": self.failed_joins.load(Ordering::Relaxed),
            },
            "heartbeats": self.heartbeats_received.load(Ordering::Relaxed),
            "cogs": {
                "total_registered": self.total_cogs_registered.load(Ordering::Relaxed),
                "withdrawn": self.cogs_withdrawn.load(Ordering::Relaxed),
                "expired": self.cogs_expired.load(Ordering::Relaxed),
            },
            "errors": {
                "protocol": self.protocol_errors.load(Ordering::Relaxed),
                "internal": self.internal_errors.load(Ordering::Relaxed),
            },
            "bytes": {
                "received": self.bytes_received.load(Ordering::Relaxed),
                "sent": self.bytes_sent.load(Ordering::Relaxed),
            },
        })
    }
}

impl Default for TrackerMetrics {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics() {
        let metrics = TrackerMetrics::new();

        metrics.record_connection();
        metrics.record_announce(true);
        metrics.record_cog_search();
        metrics.record_heartbeat();

        assert_eq!(metrics.total_connections.load(Ordering::Relaxed), 1);
        assert_eq!(metrics.successful_announces.load(Ordering::Relaxed), 1);
        assert_eq!(metrics.cog_searches.load(Ordering::Relaxed), 1);
        assert_eq!(metrics.heartbeats_received.load(Ordering::Relaxed), 1);
    }
}
