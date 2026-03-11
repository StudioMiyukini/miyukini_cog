//! Configuration TEAM.

use serde::{Deserialize, Serialize};

/// Configuration complète du hub TEAM.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamConfig {
    /// Adresse d'écoute (ex: "0.0.0.0:7800").
    #[serde(default = "default_bind_addr")]
    pub bind_addr: String,
    /// Secret HMAC partagé avec les nœuds MAIA (anti-MITM).
    pub hmac_secret: Option<String>,
    /// Politique d'isolation des nœuds.
    #[serde(default)]
    pub isolation_policy: IsolationPolicy,
    /// Intervalle de check santé (secondes).
    #[serde(default = "default_health_interval")]
    pub health_interval_secs: u64,
    /// Délai avant de marquer un nœud comme stale (secondes).
    #[serde(default = "default_stale_threshold")]
    pub stale_threshold_secs: u64,
    /// Chemin de la base de données SQLite.
    #[serde(default = "default_db_path")]
    pub db_path: String,
    /// Activer les logs des requêtes.
    #[serde(default)]
    pub log_requests: bool,
}

impl Default for TeamConfig {
    fn default() -> Self {
        Self {
            bind_addr: default_bind_addr(),
            hmac_secret: None,
            isolation_policy: IsolationPolicy::default(),
            health_interval_secs: default_health_interval(),
            stale_threshold_secs: default_stale_threshold(),
            db_path: default_db_path(),
            log_requests: false,
        }
    }
}

fn default_bind_addr() -> String {
    "0.0.0.0:7800".into()
}
fn default_health_interval() -> u64 {
    30
}
fn default_stale_threshold() -> u64 {
    90
}
fn default_db_path() -> String {
    "team.db".into()
}

/// Politique d'isolation des nœuds MWS.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum IsolationPolicy {
    /// Aucun accès réseau sortant depuis les nœuds.
    Airgap,
    /// Accès réseau interne uniquement (pas d'internet).
    #[default]
    InternalOnly,
    /// Accès réseau complet (mode dev).
    Unrestricted,
}

impl std::fmt::Display for IsolationPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Airgap => write!(f, "airgap"),
            Self::InternalOnly => write!(f, "internal_only"),
            Self::Unrestricted => write!(f, "unrestricted"),
        }
    }
}

/// Charge la config depuis le fichier TOML ou retourne les défauts.
pub fn load_or_create() -> TeamConfig {
    let config_path = std::env::var("TEAM_CONFIG").unwrap_or_else(|_| "team.toml".into());

    match std::fs::read_to_string(&config_path) {
        Ok(content) => match toml::from_str(&content) {
            Ok(cfg) => {
                tracing::info!("Config TEAM chargée depuis {config_path}");
                cfg
            }
            Err(e) => {
                tracing::warn!("Erreur parse {config_path} : {e} — utilisation des défauts");
                TeamConfig::default()
            }
        },
        Err(_) => {
            tracing::info!("Pas de config trouvée — utilisation des défauts");
            TeamConfig::default()
        }
    }
}
