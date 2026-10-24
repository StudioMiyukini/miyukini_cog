//! @id     origin_config
//! @do     definir_config_error
//! @role   config
//! @layer  app
//! @human  origin — module « config ».

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

    /// Ancienne section des clés de conformité (ignorée depuis le lot 2, voir [`CoresConfig`]).
    #[serde(default)]
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

    /// Configuration du serveur web (isolement de /visit, proxy direct).
    #[serde(default)]
    pub web: WebConfig,

    /// Authentification forum (passeport + visa, sync signée par COG).
    #[serde(default)]
    pub forum: ForumConfig,

    /// Market : clés d'éditeurs officiels autorisés à publier.
    #[serde(default)]
    pub market: MarketConfig,
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
        self.validate_web()?;

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

    /// Valide la section web (règles indépendantes des fichiers).
    fn validate_web(&self) -> Result<(), ConfigError> {
        // O-04 : l'hôte bac à sable de /visit ne doit pas partager le site du domaine principal.
        if let Some(visit_host) = self.web.visit_host.as_deref() {
            let visit_host = visit_host.trim().to_ascii_lowercase();
            let domaine = self
                .identity
                .domain
                .as_deref()
                .unwrap_or("miyukini.com")
                .to_ascii_lowercase();
            if visit_host.is_empty()
                || visit_host == domaine
                || visit_host.ends_with(&format!(".{domaine}"))
                || visit_host.contains(['/', ':', ' '])
            {
                return Err(ConfigError::InvalidValue {
                    field: "web.visit_host".to_string(),
                    message: "must be a distinct site (not the main domain nor one of its subdomains)"
                        .to_string(),
                });
            }
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
    ///
    /// O-01 : **vrai par défaut** (clé absente du TOML = strict). `false` doit être écrit
    /// explicitement et journalise un avertissement ; la preuve d'identité signée reste
    /// exigée dans les deux modes.
    #[serde(default = "default_true")]
    pub strict_verification: bool,
    /// Identité du Relay incluse dans la preuve REGISTER signée par le COG ; doit être égale
    /// au nom TLS utilisé par les COG (défaut : `identity.domain`, sinon `miyukini.com`,
    /// comme le reste d'Origin).
    #[serde(default)]
    pub identity_name: Option<String>,
    /// Fichier JSON des clés COG épinglées (TOFU). Absent : épinglage en mémoire seule.
    #[serde(default)]
    pub identity_pins_path: Option<String>,
}

fn default_true() -> bool {
    true
}

impl RelayConfig {
    /// Identité du Relay attendue dans la preuve REGISTER.
    #[must_use]
    pub fn identite_relay(&self, identity: &IdentityConfig) -> String {
        self.identity_name
            .clone()
            .or_else(|| identity.domain.clone())
            .unwrap_or_else(|| "miyukini.com".to_string())
    }
}

/// Configuration du serveur web.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebConfig {
    /// Hôte bac à sable dédié à `/visit` (O-04), sur un site distinct du domaine principal
    /// (ex. `miyukini-visit.net`). Absent : `/visit` est servi sur l'origine principale avec
    /// `Content-Security-Policy: sandbox` sans scripts.
    #[serde(default)]
    pub visit_host: Option<String>,
    /// Désactive le proxy direct « Phase 1 » (repli sans tunnel). Par défaut le proxy reste
    /// actif mais n'atteint que des adresses publiques vérifiées (O-03).
    #[serde(default)]
    pub disable_direct_proxy: bool,
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

/// Ancienne configuration des « clés de conformité des Cores » (Phase A du Relay).
///
/// Sortie du lot 2 (inventaire #17) : la clé statique par version était livrée dans
/// chaque binaire et n'apportait aucune sécurité. Elle ne participe plus à aucune
/// décision de confiance ; la section reste lisible pour ne pas casser les
/// configurations existantes, mais `keys_dir` est **ignoré**.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CoresConfig {
    /// Ignoré (conservé pour compatibilité de lecture).
    #[serde(default)]
    pub keys_dir: String,
}

/// COG reconnu par Origin pour la connexion forum : sa clé d'émission de passeports.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CogReconnuConfig {
    /// `cog_id` du COG (26 caractères base32).
    pub cog_id: String,
    /// Clé publique Ed25519 d'émission (64 caractères hexadécimaux).
    pub cle_emission: String,
}

/// Authentification forum (inventaire #11 à #13, sortie du lot 2).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ForumConfig {
    /// Graine (32 o, fichier `0600`, créée si absente) de la clé avec laquelle Origin,
    /// COG hôte, scelle les visas forum. Absente : connexion par passeport désactivée.
    #[serde(default)]
    pub cle_emission_path: Option<String>,
    /// COG dont Origin accepte les passeports (ancres directes, niveau « reconnu »).
    #[serde(default)]
    pub cogs_reconnus: Vec<CogReconnuConfig>,
    /// Clés publiques d'identité (hex) des COG autorisés à appeler `/api/auth/forum/sync`
    /// (`cog_id` = empreinte de la clé). Vide : route désactivée (503).
    #[serde(default)]
    pub sync_cles: Vec<String>,
    /// Connexion historique email + mot de passe (`/api/auth/forum/validate`) pour les
    /// personnes sans COG. **Désactivée par défaut** ; à activer explicitement.
    #[serde(default)]
    pub password_login: bool,
    /// `cog_id` propriétaire des profils hérités de l'ancien canal HMAC (`cog_id` vide).
    /// Absent : ces profils restent en lecture seule (aucune adoption automatique).
    #[serde(default)]
    pub cog_heritage: Option<String>,
}

/// Market (inventaire #14, sortie du lot 2).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MarketConfig {
    /// Clés publiques (hex) des éditeurs **officiels** épinglés : seules à pouvoir publier
    /// ou retirer un paquet (décision 4 : paquets officiels uniquement). Vide : désactivé.
    #[serde(default)]
    pub editeurs_officiels: Vec<String>,
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
    /// Origine publique exacte du panneau admin (ex. `https://admin.miyukini.com`), exigée
    /// dans `Origin`/`Referer` de toute requête modifiante (anti-CSRF). Défaut :
    /// `https://<identity.domain>`, sinon `https://<Host>` de la requête.
    #[serde(default)]
    pub public_origin: Option<String>,
}

impl Default for AdminConfig {
    fn default() -> Self {
        Self {
            host: default_admin_host(),
            port: default_admin_port(),
            config_file: None,
            public_origin: None,
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
                identity_name: None,
                identity_pins_path: None,
            },
            tracker: TrackerConfig {
                host: default_host(),
                port: default_tracker_port(),
                web_port: default_web_port(),
                pools: PoolsConfig::default(),
                lobbys: LobbysConfig::default(),
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
            cores: CoresConfig::default(),
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
            web: WebConfig::default(),
            forum: ForumConfig::default(),
            market: MarketConfig::default(),
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
        // O-01 : clé absente du TOML = mode strict.
        assert!(config.relay.strict_verification);
        assert_eq!(config.relay.identite_relay(&config.identity), "miyukini.com");
    }

    #[test]
    fn non_strict_uniquement_si_ecrit_explicitement() {
        let toml = r#"
[identity]
role = "origin"
ip = "1.2.3.4"
domain = "miyukini.com"
[relay]
strict_verification = false
[tracker]
[tls]
cert_path = "a"
key_path = "b"
[auth]
token_file = "t"
[registry]
data_dir = "d"
[cores]
keys_dir = "k"
[policies]
data_dir = "p"
"#;
        let config: OriginConfig = toml::from_str(toml).expect("parse");
        assert!(!config.relay.strict_verification);
        assert_eq!(config.relay.identite_relay(&config.identity), "miyukini.com");
    }

    #[test]
    fn sections_forum_market_et_cores_facultatif() {
        let toml = r#"
[identity]
role = "origin"
ip = "1.2.3.4"
[relay]
[tracker]
[tls]
cert_path = "a"
key_path = "b"
[auth]
token_file = "t"
[registry]
data_dir = "d"
[policies]
data_dir = "p"
[forum]
cle_emission_path = "/var/lib/miyukini/forum.cle"
sync_cles = ["00"]
[[forum.cogs_reconnus]]
cog_id = "aaaaaaaaaaaaaaaaaaaaaaaaaa"
cle_emission = "11"
[market]
editeurs_officiels = ["22"]
"#;
        let config: OriginConfig = toml::from_str(toml).expect("parse");
        // `[cores]` absent : accepté (ignoré depuis le lot 2).
        assert_eq!(config.cores.keys_dir, "");
        // Mot de passe : désactivé tant que non écrit explicitement.
        assert!(!config.forum.password_login);
        assert_eq!(config.forum.cogs_reconnus.len(), 1);
        assert_eq!(config.market.editeurs_officiels, vec!["22".to_string()]);
        let defaut = OriginConfig::default();
        assert!(defaut.forum.cle_emission_path.is_none() && defaut.market.editeurs_officiels.is_empty());
    }

    #[test]
    fn visit_host_sous_domaine_refuse() {
        let mut config = OriginConfig::default();
        config.identity.domain = Some("miyukini.com".to_string());
        config.web.visit_host = Some("visit.miyukini.com".to_string());
        assert!(config.validate_web().is_err());
        config.web.visit_host = Some("miyukini.com".to_string());
        assert!(config.validate_web().is_err());
        config.web.visit_host = Some("miyukini-visit.net".to_string());
        assert!(config.validate_web().is_ok());
    }
}
