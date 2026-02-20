//! Configuration d'Origin.
//!
//! Parse le fichier de configuration TOML (origin.toml).

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use thiserror::Error;

/// Erreurs de configuration.
#[derive(Debug, Error)]
pub enum ConfigError {
    /// Erreur de lecture du fichier.
    #[error("failed to read config file: {0}")]
    ReadError(#[from] std::io::Error),

    /// Erreur de parsing TOML.
    #[error("failed to parse config: {0}")]
    ParseError(#[from] toml::de::Error),

    /// Fichier non trouvé.
    #[error("config file not found: {0}")]
    NotFound(PathBuf),

    /// Valeur invalide.
    #[error("invalid value for {field}: {message}")]
    InvalidValue { field: String, message: String },
}

/// Configuration complète d'Origin.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OriginConfig {
    /// Identité d'Origin.
    pub identity: IdentityConfig,

    /// Configuration du relay.
    pub relay: RelayConfig,

    /// Configuration du tracker.
    pub tracker: TrackerConfig,

    /// Configuration TLS.
    pub tls: TlsConfig,

    /// Configuration d'authentification.
    pub auth: AuthConfig,

    /// Configuration du Registre de Services.
    pub registry: RegistryConfig,

    /// Configuration des clés de conformité.
    pub cores: CoresConfig,

    /// Configuration des politiques.
    pub policies: PoliciesConfig,

    /// Configuration du rate limiting.
    #[serde(default)]
    pub rate_limits: RateLimitsConfig,

    /// Configuration du Proof of Work.
    #[serde(default)]
    pub pow: PowConfig,

    /// Configuration de MiyukiniAdmin.
    #[serde(default)]
    pub admin: AdminConfig,

    /// Configuration de la journalisation.
    #[serde(default)]
    pub logging: LoggingConfig,

    /// Configuration des limites.
    #[serde(default)]
    pub limits: LimitsConfig,
}

impl OriginConfig {
    /// Charge la configuration depuis un fichier.
    pub fn load(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let path = path.as_ref();
        if !path.exists() {
            return Err(ConfigError::NotFound(path.to_path_buf()));
        }

        let content = std::fs::read_to_string(path)?;
        let config: Self = toml::from_str(&content)?;
        config.validate()?;
        Ok(config)
    }

    /// Valide la configuration.
    fn validate(&self) -> Result<(), ConfigError> {
        // Vérifier les ports
        if self.relay.port == self.tracker.port {
            return Err(ConfigError::InvalidValue {
                field: "relay.port/tracker.port".to_string(),
                message: "relay and tracker must use different ports".to_string(),
            });
        }

        // Vérifier les chemins TLS
        if !Path::new(&self.tls.cert_path).exists() {
            return Err(ConfigError::InvalidValue {
                field: "tls.cert_path".to_string(),
                message: format!("certificate file not found: {}", self.tls.cert_path),
            });
        }
        if !Path::new(&self.tls.key_path).exists() {
            return Err(ConfigError::InvalidValue {
                field: "tls.key_path".to_string(),
                message: format!("key file not found: {}", self.tls.key_path),
            });
        }

        Ok(())
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Sections de configuration
// ═══════════════════════════════════════════════════════════════════════════

/// Configuration d'identité.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityConfig {
    /// Rôle (origin, relay, tracker).
    pub role: String,
    /// Adresse IP publique.
    pub ip: String,
    /// Domaine (optionnel).
    pub domain: Option<String>,
}

/// Configuration du relay.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelayConfig {
    /// Adresse d'écoute.
    #[serde(default = "default_host")]
    pub host: String,
    /// Port d'écoute.
    #[serde(default = "default_relay_port")]
    pub port: u16,
    /// Vérification stricte en 3 phases (Phase A CORE_KEY, Phase C environment_health).
    /// Si true : après REGISTER on envoie VERIFY_RESULT(phase A, EXTENDED_REQUIRED),
    /// on attend CORE_KEY puis on fait Phase C et REGISTER_OK.
    #[serde(default)]
    pub strict_verification: bool,
}

fn default_host() -> String {
    "0.0.0.0".to_string()
}

fn default_relay_port() -> u16 {
    7000
}

/// Configuration du tracker.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackerConfig {
    /// Adresse d'écoute.
    #[serde(default = "default_host")]
    pub host: String,
    /// Port d'écoute.
    #[serde(default = "default_tracker_port")]
    pub port: u16,
    /// Port du catalogue web interne.
    #[serde(default = "default_web_port")]
    pub web_port: u16,
    /// Configuration des pools.
    #[serde(default)]
    pub pools: PoolsConfig,
    /// Configuration des lobbys.
    #[serde(default)]
    pub lobbys: LobbysConfig,
    /// Chemin optionnel vers la base JayXpose (vitrines publiques). Si présent, les routes /vitrine/* sont servies.
    #[serde(default)]
    pub jayxpose_db_path: Option<String>,
    /// Chemin optionnel vers la base SQLite des profils forum (auth unifiée Central). Si présent, les routes /api/auth/forum/* sont activées.
    #[serde(default)]
    pub forum_profiles_db_path: Option<String>,
}

fn default_tracker_port() -> u16 {
    21000
}

fn default_web_port() -> u16 {
    8080
}

/// Configuration des pools.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PoolsConfig {
    /// Activer l'isolation par version.
    #[serde(default)]
    pub enable_version_isolation: bool,
}

/// Configuration des lobbys.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbysConfig {
    /// Nombre maximum de lobbys par COG.
    #[serde(default = "default_max_lobbys")]
    pub max_lobbys_per_cog: u32,
    /// Tentatives maximum de mot de passe.
    #[serde(default = "default_password_attempts")]
    pub password_max_attempts: u32,
}

impl Default for LobbysConfig {
    fn default() -> Self {
        Self {
            max_lobbys_per_cog: default_max_lobbys(),
            password_max_attempts: default_password_attempts(),
        }
    }
}

fn default_max_lobbys() -> u32 {
    10
}

fn default_password_attempts() -> u32 {
    3
}

/// Configuration TLS.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsConfig {
    /// Chemin du certificat.
    pub cert_path: String,
    /// Chemin de la clé privée.
    pub key_path: String,
    /// Version TLS minimale.
    #[serde(default = "default_tls_version")]
    pub min_version: String,
}

fn default_tls_version() -> String {
    "1.3".to_string()
}

/// Configuration d'authentification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    /// Chemin du fichier de tokens.
    pub token_file: String,
    /// Jours avant rotation des tokens.
    #[serde(default = "default_rotation_days")]
    pub token_rotation_days: u32,
}

fn default_rotation_days() -> u32 {
    7
}

/// Configuration du Registre de Services.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryConfig {
    /// Répertoire des données.
    pub data_dir: String,
}

/// Configuration des clés de conformité.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoresConfig {
    /// Répertoire des clés.
    pub keys_dir: String,
}

/// Configuration des politiques.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoliciesConfig {
    /// Répertoire des données.
    pub data_dir: String,
    /// Escalade de quarantaine (secondes).
    #[serde(default = "default_quarantine_escalation")]
    pub quarantine_escalation: Vec<u64>,
    /// Fenêtre de timestamp (secondes).
    #[serde(default = "default_timestamp_window")]
    pub timestamp_window_seconds: u64,
}

fn default_quarantine_escalation() -> Vec<u64> {
    vec![3600, 7200] // 1h, 2h
}

fn default_timestamp_window() -> u64 {
    10
}

/// Configuration du rate limiting.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitsConfig {
    /// Enregistrements par minute par IP.
    #[serde(default = "default_register_per_minute")]
    pub register_per_minute_per_ip: u32,
    /// Connexions par token.
    #[serde(default = "default_connections_per_token")]
    pub connections_per_token: u32,
    /// Requêtes par heure par COG.
    #[serde(default = "default_requests_per_hour")]
    pub requests_per_hour_per_cog: u32,
    /// Connexions TCP par IP.
    #[serde(default = "default_tcp_per_ip")]
    pub tcp_connections_per_ip: u32,
}

impl Default for RateLimitsConfig {
    fn default() -> Self {
        Self {
            register_per_minute_per_ip: default_register_per_minute(),
            connections_per_token: default_connections_per_token(),
            requests_per_hour_per_cog: default_requests_per_hour(),
            tcp_connections_per_ip: default_tcp_per_ip(),
        }
    }
}

fn default_register_per_minute() -> u32 {
    10
}

fn default_connections_per_token() -> u32 {
    100
}

fn default_requests_per_hour() -> u32 {
    1000
}

fn default_tcp_per_ip() -> u32 {
    5000
}

/// Configuration du Proof of Work.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowConfig {
    /// Activer le PoW.
    #[serde(default)]
    pub enabled: bool,
    /// Difficulté normale.
    #[serde(default = "default_pow_normal")]
    pub difficulty_normal: u8,
    /// Difficulté sous attaque.
    #[serde(default = "default_pow_attack")]
    pub difficulty_attack: u8,
    /// TTL du challenge (secondes).
    #[serde(default = "default_challenge_ttl")]
    pub challenge_ttl_seconds: u32,
}

impl Default for PowConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            difficulty_normal: default_pow_normal(),
            difficulty_attack: default_pow_attack(),
            challenge_ttl_seconds: default_challenge_ttl(),
        }
    }
}

fn default_pow_normal() -> u8 {
    16
}

fn default_pow_attack() -> u8 {
    22
}

fn default_challenge_ttl() -> u32 {
    30
}

/// Configuration de MiyukiniAdmin.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminConfig {
    /// Adresse d'écoute.
    #[serde(default = "default_admin_host")]
    pub host: String,
    /// Port d'écoute.
    #[serde(default = "default_admin_port")]
    pub port: u16,
    /// Chemin du fichier de configuration admin.
    #[serde(default)]
    pub config_file: Option<String>,
}

impl Default for AdminConfig {
    fn default() -> Self {
        Self {
            host: default_admin_host(),
            port: default_admin_port(),
            config_file: None,
        }
    }
}

fn default_admin_host() -> String {
    "127.0.0.1".to_string()
}

fn default_admin_port() -> u16 {
    8081
}

/// Configuration de la journalisation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Niveau de log.
    #[serde(default = "default_log_level")]
    pub level: String,
    /// Chemin du log relay.
    #[serde(default)]
    pub relay_log: Option<String>,
    /// Chemin du log tracker.
    #[serde(default)]
    pub tracker_log: Option<String>,
    /// Chemin du log audit.
    #[serde(default)]
    pub audit_log: Option<String>,
    /// Chemin du log admin.
    #[serde(default)]
    pub admin_log: Option<String>,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: default_log_level(),
            relay_log: None,
            tracker_log: None,
            audit_log: None,
            admin_log: None,
        }
    }
}

fn default_log_level() -> String {
    "info".to_string()
}

/// Configuration des limites.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LimitsConfig {
    /// Nombre maximum de connexions.
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,
    /// Intervalle de heartbeat (secondes).
    #[serde(default = "default_heartbeat_interval")]
    pub heartbeat_interval_seconds: u32,
    /// Timeout du tunnel (secondes).
    #[serde(default = "default_tunnel_timeout")]
    pub tunnel_timeout_seconds: u32,
}

impl Default for LimitsConfig {
    fn default() -> Self {
        Self {
            max_connections: default_max_connections(),
            heartbeat_interval_seconds: default_heartbeat_interval(),
            tunnel_timeout_seconds: default_tunnel_timeout(),
        }
    }
}

fn default_max_connections() -> u32 {
    10000
}

fn default_heartbeat_interval() -> u32 {
    30
}

fn default_tunnel_timeout() -> u32 {
    300
}

// ═══════════════════════════════════════════════════════════════════════════
// Configuration par défaut pour le développement
// ═══════════════════════════════════════════════════════════════════════════

impl Default for OriginConfig {
    fn default() -> Self {
        Self {
            identity: IdentityConfig {
                role: "origin".to_string(),
                ip: "127.0.0.1".to_string(),
                domain: None,
            },
            relay: RelayConfig {
                host: default_host(),
                port: default_relay_port(),
                strict_verification: true,
            },
            tracker: TrackerConfig {
                host: default_host(),
                port: default_tracker_port(),
                web_port: default_web_port(),
                pools: PoolsConfig::default(),
                lobbys: LobbysConfig::default(),
                jayxpose_db_path: None,
                forum_profiles_db_path: None,
            },
            tls: TlsConfig {
                cert_path: "tls/origin.crt".to_string(),
                key_path: "tls/origin.key".to_string(),
                min_version: default_tls_version(),
            },
            auth: AuthConfig {
                token_file: "tokens.json".to_string(),
                token_rotation_days: default_rotation_days(),
            },
            registry: RegistryConfig {
                data_dir: "data/registry".to_string(),
            },
            cores: CoresConfig {
                keys_dir: "data/keys".to_string(),
            },
            policies: PoliciesConfig {
                data_dir: "data/policies".to_string(),
                quarantine_escalation: default_quarantine_escalation(),
                timestamp_window_seconds: default_timestamp_window(),
            },
            rate_limits: RateLimitsConfig::default(),
            pow: PowConfig::default(),
            admin: AdminConfig::default(),
            logging: LoggingConfig::default(),
            limits: LimitsConfig::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = OriginConfig::default();
        assert_eq!(config.identity.role, "origin");
        assert_eq!(config.relay.port, 7000);
        assert_eq!(config.tracker.port, 21000);
    }

    #[test]
    fn test_parse_minimal_toml() {
        let toml = r#"
[identity]
role = "origin"
ip = "84.235.227.152"

[relay]
port = 7000

[tracker]
port = 21000

[tls]
cert_path = "/etc/miyukini/tls/origin.crt"
key_path = "/etc/miyukini/tls/origin.key"

[auth]
token_file = "/etc/miyukini/tokens.json"

[registry]
data_dir = "/var/lib/miyukini/registry"

[cores]
keys_dir = "/var/lib/miyukini/keys"

[policies]
data_dir = "/var/lib/miyukini/policies"
"#;
        let config: OriginConfig = toml::from_str(toml).expect("test config should parse");
        assert_eq!(config.identity.ip, "84.235.227.152");
        assert_eq!(config.relay.port, 7000);
    }
}
