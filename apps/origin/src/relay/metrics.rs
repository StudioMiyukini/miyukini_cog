//! Métriques et statistiques du Relay.
//!
//! Collecte des métriques pour le monitoring et le débogage.

use std::sync::atomic::{AtomicU64, Ordering};

/// Métriques du serveur Relay.
#[derive(Debug)]
pub struct RelayMetrics {
    // Connexions
    /// Total des connexions acceptées.
    pub total_connections: AtomicU64,
    /// Connexions actuellement actives.
    pub active_connections: AtomicU64,
    /// Connexions rejetées (rate limit, etc.).
    pub rejected_connections: AtomicU64,
    /// Erreurs TLS.
    pub tls_errors: AtomicU64,

    // Sessions
    /// Total des sessions créées.
    pub total_sessions: AtomicU64,
    /// Sessions actuellement actives.
    pub active_sessions: AtomicU64,
    /// Sessions expirées nettoyées.
    pub expired_sessions: AtomicU64,

    // Enregistrements
    /// Total des REGISTER reçus.
    pub total_registrations: AtomicU64,
    /// Enregistrements réussis.
    pub successful_registrations: AtomicU64,
    /// Enregistrements échoués.
    pub failed_registrations: AtomicU64,

    // Vérification
    /// Phase A réussies.
    pub phase_a_passed: AtomicU64,
    /// Phase A échouées.
    pub phase_a_failed: AtomicU64,
    /// Phase B réussies.
    pub phase_b_passed: AtomicU64,
    /// Phase B échouées.
    pub phase_b_failed: AtomicU64,
    /// Phase C réussies.
    pub phase_c_passed: AtomicU64,
    /// Phase C échouées.
    pub phase_c_failed: AtomicU64,

    // Permis
    /// Total des Permis délivrés.
    pub total_permits_issued: AtomicU64,
    /// Permis actuellement valides.
    pub active_permits: AtomicU64,
    /// Permis révoqués.
    pub revoked_permits: AtomicU64,

    // Messages
    /// Total des messages reçus.
    pub messages_received: AtomicU64,
    /// Total des messages envoyés.
    pub messages_sent: AtomicU64,
    /// Messages DATA relayés.
    pub data_messages_relayed: AtomicU64,
    /// Bytes reçus.
    pub bytes_received: AtomicU64,
    /// Bytes envoyés.
    pub bytes_sent: AtomicU64,

    // Tunnels
    /// Total des tunnels créés.
    pub total_tunnels: AtomicU64,
    /// Tunnels actuellement actifs.
    pub active_tunnels: AtomicU64,

    // Heartbeats
    /// Total des HEARTBEAT reçus.
    pub heartbeats_received: AtomicU64,
    /// HEARTBEAT_ACK envoyés.
    pub heartbeats_sent: AtomicU64,

    // Errors
    /// Erreurs de parsing de trame.
    pub frame_parse_errors: AtomicU64,
    /// Erreurs de protocole.
    pub protocol_errors: AtomicU64,
    /// Erreurs internes.
    pub internal_errors: AtomicU64,

    // Timestamps
    /// Timestamp du démarrage.
    start_time: chrono::DateTime<chrono::Utc>,
}

impl RelayMetrics {
    /// Crée de nouvelles métriques.
    #[must_use]
    pub fn new() -> Self {
        Self {
            total_connections: AtomicU64::new(0),
            active_connections: AtomicU64::new(0),
            rejected_connections: AtomicU64::new(0),
            tls_errors: AtomicU64::new(0),

            total_sessions: AtomicU64::new(0),
            active_sessions: AtomicU64::new(0),
            expired_sessions: AtomicU64::new(0),

            total_registrations: AtomicU64::new(0),
            successful_registrations: AtomicU64::new(0),
            failed_registrations: AtomicU64::new(0),

            phase_a_passed: AtomicU64::new(0),
            phase_a_failed: AtomicU64::new(0),
            phase_b_passed: AtomicU64::new(0),
            phase_b_failed: AtomicU64::new(0),
            phase_c_passed: AtomicU64::new(0),
            phase_c_failed: AtomicU64::new(0),

            total_permits_issued: AtomicU64::new(0),
            active_permits: AtomicU64::new(0),
            revoked_permits: AtomicU64::new(0),

            messages_received: AtomicU64::new(0),
            messages_sent: AtomicU64::new(0),
            data_messages_relayed: AtomicU64::new(0),
            bytes_received: AtomicU64::new(0),
            bytes_sent: AtomicU64::new(0),

            total_tunnels: AtomicU64::new(0),
            active_tunnels: AtomicU64::new(0),

            heartbeats_received: AtomicU64::new(0),
            heartbeats_sent: AtomicU64::new(0),

            frame_parse_errors: AtomicU64::new(0),
            protocol_errors: AtomicU64::new(0),
            internal_errors: AtomicU64::new(0),

            start_time: chrono::Utc::now(),
        }
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Méthodes d'incrémentation
    // ─────────────────────────────────────────────────────────────────────────

    /// Enregistre une nouvelle connexion.
    pub fn record_connection(&self) {
        self.total_connections.fetch_add(1, Ordering::Relaxed);
        self.active_connections.fetch_add(1, Ordering::Relaxed);
    }

    /// Enregistre la fermeture d'une connexion.
    pub fn record_connection_closed(&self) {
        self.active_connections.fetch_sub(1, Ordering::Relaxed);
    }

    /// Enregistre une connexion rejetée.
    pub fn record_rejected_connection(&self) {
        self.rejected_connections.fetch_add(1, Ordering::Relaxed);
    }

    /// Enregistre une erreur TLS.
    pub fn record_tls_error(&self) {
        self.tls_errors.fetch_add(1, Ordering::Relaxed);
    }

    /// Enregistre une nouvelle session.
    pub fn record_session_created(&self) {
        self.total_sessions.fetch_add(1, Ordering::Relaxed);
        self.active_sessions.fetch_add(1, Ordering::Relaxed);
    }

    /// Enregistre la fermeture d'une session.
    pub fn record_session_closed(&self) {
        self.active_sessions.fetch_sub(1, Ordering::Relaxed);
    }

    /// Enregistre une session expirée.
    pub fn record_session_expired(&self) {
        self.expired_sessions.fetch_add(1, Ordering::Relaxed);
        self.active_sessions.fetch_sub(1, Ordering::Relaxed);
    }

    /// Enregistre une tentative d'enregistrement.
    pub fn record_registration(&self, success: bool) {
        self.total_registrations.fetch_add(1, Ordering::Relaxed);
        if success {
            self.successful_registrations
                .fetch_add(1, Ordering::Relaxed);
        } else {
            self.failed_registrations.fetch_add(1, Ordering::Relaxed);
        }
    }

    /// Enregistre un résultat Phase A.
    pub fn record_phase_a(&self, passed: bool) {
        if passed {
            self.phase_a_passed.fetch_add(1, Ordering::Relaxed);
        } else {
            self.phase_a_failed.fetch_add(1, Ordering::Relaxed);
        }
    }

    /// Enregistre un résultat Phase B.
    pub fn record_phase_b(&self, passed: bool) {
        if passed {
            self.phase_b_passed.fetch_add(1, Ordering::Relaxed);
        } else {
            self.phase_b_failed.fetch_add(1, Ordering::Relaxed);
        }
    }

    /// Enregistre un résultat Phase C.
    pub fn record_phase_c(&self, passed: bool) {
        if passed {
            self.phase_c_passed.fetch_add(1, Ordering::Relaxed);
        } else {
            self.phase_c_failed.fetch_add(1, Ordering::Relaxed);
        }
    }

    /// Enregistre un Permis délivré.
    pub fn record_permit_issued(&self) {
        self.total_permits_issued.fetch_add(1, Ordering::Relaxed);
        self.active_permits.fetch_add(1, Ordering::Relaxed);
    }

    /// Enregistre un Permis expiré/révoqué.
    pub fn record_permit_expired(&self) {
        self.active_permits.fetch_sub(1, Ordering::Relaxed);
    }

    /// Enregistre un Permis révoqué.
    pub fn record_permit_revoked(&self) {
        self.revoked_permits.fetch_add(1, Ordering::Relaxed);
        self.active_permits.fetch_sub(1, Ordering::Relaxed);
    }

    /// Enregistre un message reçu.
    pub fn record_message_received(&self, bytes: usize) {
        self.messages_received.fetch_add(1, Ordering::Relaxed);
        self.bytes_received
            .fetch_add(bytes as u64, Ordering::Relaxed);
    }

    /// Enregistre un message envoyé.
    pub fn record_message_sent(&self, bytes: usize) {
        self.messages_sent.fetch_add(1, Ordering::Relaxed);
        self.bytes_sent.fetch_add(bytes as u64, Ordering::Relaxed);
    }

    /// Enregistre un message DATA relayé.
    pub fn record_data_relayed(&self) {
        self.data_messages_relayed.fetch_add(1, Ordering::Relaxed);
    }

    /// Enregistre un tunnel créé.
    pub fn record_tunnel_created(&self) {
        self.total_tunnels.fetch_add(1, Ordering::Relaxed);
        self.active_tunnels.fetch_add(1, Ordering::Relaxed);
    }

    /// Enregistre un tunnel fermé.
    pub fn record_tunnel_closed(&self) {
        self.active_tunnels.fetch_sub(1, Ordering::Relaxed);
    }

    /// Enregistre un heartbeat reçu.
    pub fn record_heartbeat(&self) {
        self.heartbeats_received.fetch_add(1, Ordering::Relaxed);
        self.heartbeats_sent.fetch_add(1, Ordering::Relaxed);
    }

    /// Enregistre une erreur de parsing.
    pub fn record_parse_error(&self) {
        self.frame_parse_errors.fetch_add(1, Ordering::Relaxed);
    }

    /// Enregistre une erreur de protocole.
    pub fn record_protocol_error(&self) {
        self.protocol_errors.fetch_add(1, Ordering::Relaxed);
    }

    /// Enregistre une erreur interne.
    pub fn record_internal_error(&self) {
        self.internal_errors.fetch_add(1, Ordering::Relaxed);
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Méthodes de lecture
    // ─────────────────────────────────────────────────────────────────────────

    /// Retourne l'uptime en secondes.
    #[must_use]
    pub fn uptime_seconds(&self) -> i64 {
        (chrono::Utc::now() - self.start_time).num_seconds()
    }

    /// Convertit les métriques en JSON.
    #[must_use]
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "uptime_seconds": self.uptime_seconds(),
            "start_time": self.start_time.to_rfc3339(),
            "connections": {
                "total": self.total_connections.load(Ordering::Relaxed),
                "active": self.active_connections.load(Ordering::Relaxed),
                "rejected": self.rejected_connections.load(Ordering::Relaxed),
                "tls_errors": self.tls_errors.load(Ordering::Relaxed),
            },
            "sessions": {
                "total": self.total_sessions.load(Ordering::Relaxed),
                "active": self.active_sessions.load(Ordering::Relaxed),
                "expired": self.expired_sessions.load(Ordering::Relaxed),
            },
            "registrations": {
                "total": self.total_registrations.load(Ordering::Relaxed),
                "successful": self.successful_registrations.load(Ordering::Relaxed),
                "failed": self.failed_registrations.load(Ordering::Relaxed),
            },
            "verification": {
                "phase_a_passed": self.phase_a_passed.load(Ordering::Relaxed),
                "phase_a_failed": self.phase_a_failed.load(Ordering::Relaxed),
                "phase_b_passed": self.phase_b_passed.load(Ordering::Relaxed),
                "phase_b_failed": self.phase_b_failed.load(Ordering::Relaxed),
                "phase_c_passed": self.phase_c_passed.load(Ordering::Relaxed),
                "phase_c_failed": self.phase_c_failed.load(Ordering::Relaxed),
            },
            "permits": {
                "total_issued": self.total_permits_issued.load(Ordering::Relaxed),
                "active": self.active_permits.load(Ordering::Relaxed),
                "revoked": self.revoked_permits.load(Ordering::Relaxed),
            },
            "messages": {
                "received": self.messages_received.load(Ordering::Relaxed),
                "sent": self.messages_sent.load(Ordering::Relaxed),
                "data_relayed": self.data_messages_relayed.load(Ordering::Relaxed),
            },
            "bytes": {
                "received": self.bytes_received.load(Ordering::Relaxed),
                "sent": self.bytes_sent.load(Ordering::Relaxed),
            },
            "tunnels": {
                "total": self.total_tunnels.load(Ordering::Relaxed),
                "active": self.active_tunnels.load(Ordering::Relaxed),
            },
            "heartbeats": {
                "received": self.heartbeats_received.load(Ordering::Relaxed),
                "sent": self.heartbeats_sent.load(Ordering::Relaxed),
            },
            "errors": {
                "frame_parse": self.frame_parse_errors.load(Ordering::Relaxed),
                "protocol": self.protocol_errors.load(Ordering::Relaxed),
                "internal": self.internal_errors.load(Ordering::Relaxed),
            },
        })
    }

    /// Retourne un résumé court.
    #[must_use]
    pub fn summary(&self) -> MetricsSummary {
        MetricsSummary {
            uptime_seconds: self.uptime_seconds(),
            active_connections: self.active_connections.load(Ordering::Relaxed),
            active_sessions: self.active_sessions.load(Ordering::Relaxed),
            active_tunnels: self.active_tunnels.load(Ordering::Relaxed),
            messages_per_second: self.calculate_mps(),
            bytes_received: self.bytes_received.load(Ordering::Relaxed),
            bytes_sent: self.bytes_sent.load(Ordering::Relaxed),
        }
    }

    /// Calcule les messages par seconde.
    fn calculate_mps(&self) -> f64 {
        let uptime = self.uptime_seconds();
        if uptime > 0 {
            let total = self.messages_received.load(Ordering::Relaxed)
                + self.messages_sent.load(Ordering::Relaxed);
            total as f64 / uptime as f64
        } else {
            0.0
        }
    }
}

impl Default for RelayMetrics {
    fn default() -> Self {
        Self::new()
    }
}

/// Résumé des métriques.
#[derive(Debug, Clone)]
pub struct MetricsSummary {
    /// Uptime en secondes.
    pub uptime_seconds: i64,
    /// Connexions actives.
    pub active_connections: u64,
    /// Sessions actives.
    pub active_sessions: u64,
    /// Tunnels actifs.
    pub active_tunnels: u64,
    /// Messages par seconde.
    pub messages_per_second: f64,
    /// Bytes reçus.
    pub bytes_received: u64,
    /// Bytes envoyés.
    pub bytes_sent: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_counters() {
        let metrics = RelayMetrics::new();

        metrics.record_connection();
        metrics.record_connection();
        metrics.record_session_created();
        metrics.record_registration(true);
        metrics.record_registration(false);

        assert_eq!(metrics.total_connections.load(Ordering::Relaxed), 2);
        assert_eq!(metrics.active_connections.load(Ordering::Relaxed), 2);
        assert_eq!(metrics.total_sessions.load(Ordering::Relaxed), 1);
        assert_eq!(metrics.total_registrations.load(Ordering::Relaxed), 2);
        assert_eq!(metrics.successful_registrations.load(Ordering::Relaxed), 1);
        assert_eq!(metrics.failed_registrations.load(Ordering::Relaxed), 1);

        metrics.record_connection_closed();
        assert_eq!(metrics.active_connections.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn test_metrics_to_json() {
        let metrics = RelayMetrics::new();
        metrics.record_connection();
        metrics.record_message_received(1024);
        metrics.record_message_sent(512);

        let json = metrics.to_json();

        assert!(json["connections"]["total"].as_u64().unwrap() >= 1);
        assert_eq!(json["bytes"]["received"].as_u64().unwrap(), 1024);
        assert_eq!(json["bytes"]["sent"].as_u64().unwrap(), 512);
    }
}
