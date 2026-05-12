//! Configuration runtime du service JayCloud.
//!
//! Annexe C de la Spec liste les variables d'environnement attendues.
//! Le parsing depuis env vars / fichier TOML viendra en PR-3.

use serde::{Deserialize, Serialize};

/// Configuration runtime de JayCloud.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JayCloudConfig {
    /// Racine du storage local (chiffré).
    pub storage_path: String,
    /// Port HTTPS du portail.
    pub http_port: u16,
    /// Limite Depth infinity sur PROPFIND WebDAV.
    pub dav_depth_limit: usize,
    /// TTL des sessions web (secondes).
    pub session_ttl_seconds: u64,
    /// TTL des LOCK WebDAV (secondes).
    pub lock_ttl_seconds: u64,
    /// Expiration par défaut des liens publics (jours).
    pub share_link_default_expiry_days: u32,
    /// Fenêtre de transition MiyuCloud (jours).
    pub miyucloud_transition_days: u32,
    /// Rate limit par app-password (requêtes / minute).
    pub rate_limit_per_token_per_min: u32,
    /// Politique de rétention par défaut (JSON sérialisé en P3).
    pub default_retention: String,
    /// Cron du job d'intégrité périodique.
    pub integrity_check_cron: String,
    /// Niveau zstd pour la compression CAS (0 = pas de compression).
    pub cas_compression_level: i32,
}

impl Default for JayCloudConfig {
    fn default() -> Self {
        Self {
            storage_path: "~/.miyukini/jaycloud/".to_string(),
            http_port: 8443,
            dav_depth_limit: 1000,
            session_ttl_seconds: 86_400,
            lock_ttl_seconds: 3_600,
            share_link_default_expiry_days: 30,
            miyucloud_transition_days: 90,
            rate_limit_per_token_per_min: 600,
            default_retention: r#"{"daily":7,"weekly":4,"monthly":12}"#.to_string(),
            integrity_check_cron: "0 4 * * 0".to_string(),
            cas_compression_level: 3,
        }
    }
}
