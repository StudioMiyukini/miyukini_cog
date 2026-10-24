//! @id     team_config
//! @do     definir_team_config
//! @role   config
//! @layer  app
//! @human  team — module « config ».

//! Configuration TEAM.
//!
//! Sécurité (fail-closed, C-12 / C-21 / C-22) :
//! - écoute par défaut sur `127.0.0.1:7800` ;
//! - une adresse non-loopback exige un secret partagé (`hmac_secret` ou
//!   variable d'environnement `TEAM_HMAC_SECRET`), sinon refus de démarrer ;
//! - si un secret est configuré, **toutes** les routes (sauf `GET /team/health`)
//!   exigent `Authorization: Bearer <secret>`, y compris en loopback ;
//! - sans secret (loopback uniquement), les routes restent ouvertes aux
//!   processus locaux — comportement de développement, signalé au démarrage ;
//! - CORS : liste blanche explicite `cors_allowed_origins`, vide par défaut ;
//! - URL de nœuds : loopback ou `node_allowed_hosts` (voir `mws::policy`).

use serde::{Deserialize, Serialize};

/// Variable d'environnement prioritaire sur `hmac_secret` du fichier.
pub const SECRET_ENV_VAR: &str = "TEAM_HMAC_SECRET";

/// Longueur minimale d'un secret configuré (octets).
pub const MIN_SECRET_LEN: usize = 16;

/// Configuration complète du hub TEAM.
#[derive(Clone, Serialize, Deserialize)]
pub struct TeamConfig {
    /// Adresse d'écoute (défaut : "127.0.0.1:7800", loopback uniquement).
    /// Une adresse non-loopback exige `hmac_secret`.
    #[serde(default = "default_bind_addr")]
    pub bind_addr: String,
    /// Secret partagé avec les nœuds MAIA, présenté en `Authorization: Bearer`.
    pub hmac_secret: Option<String>,
    /// Origines CORS autorisées (liste explicite ; vide = aucune ; `*` ignoré).
    #[serde(default)]
    pub cors_allowed_origins: Vec<String>,
    /// Hôtes (IP ou nom) autorisés pour les URL de nœuds, en plus du loopback.
    #[serde(default)]
    pub node_allowed_hosts: Vec<String>,
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

// `Debug` manuel : ne jamais afficher le secret.
impl std::fmt::Debug for TeamConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TeamConfig")
            .field("bind_addr", &self.bind_addr)
            .field("hmac_secret", &self.hmac_secret.as_ref().map(|_| "<masqué>"))
            .field("cors_allowed_origins", &self.cors_allowed_origins)
            .field("node_allowed_hosts", &self.node_allowed_hosts)
            .field("isolation_policy", &self.isolation_policy)
            .field("health_interval_secs", &self.health_interval_secs)
            .field("stale_threshold_secs", &self.stale_threshold_secs)
            .field("db_path", &self.db_path)
            .field("log_requests", &self.log_requests)
            .finish()
    }
}

impl Default for TeamConfig {
    fn default() -> Self {
        Self {
            bind_addr: default_bind_addr(),
            hmac_secret: None,
            cors_allowed_origins: Vec::new(),
            node_allowed_hosts: Vec::new(),
            isolation_policy: IsolationPolicy::default(),
            health_interval_secs: default_health_interval(),
            stale_threshold_secs: default_stale_threshold(),
            db_path: default_db_path(),
            log_requests: false,
        }
    }
}

impl TeamConfig {
    /// Secret effectif : `None` si absent ou vide (après trim).
    pub fn secret(&self) -> Option<&str> {
        self.hmac_secret
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
    }
}

fn default_bind_addr() -> String {
    "127.0.0.1:7800".into()
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

/// Refus de démarrage (une variante par cause, C-11).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StartupError {
    /// Adresse d'écoute non-loopback sans secret configuré.
    NonLoopbackWithoutSecret(String),
    /// Secret configuré mais trop court.
    SecretTooShort,
}

impl std::fmt::Display for StartupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonLoopbackWithoutSecret(addr) => write!(
                f,
                "Refus de démarrer : bind_addr={addr} n'est pas loopback et aucun \
                 hmac_secret n'est configuré. Définissez `hmac_secret` (ou \
                 {SECRET_ENV_VAR}) ou utilisez 127.0.0.1."
            ),
            Self::SecretTooShort => write!(
                f,
                "Refus de démarrer : hmac_secret trop court (minimum {MIN_SECRET_LEN} caractères)."
            ),
        }
    }
}

impl std::error::Error for StartupError {}

/// Vrai si l'adresse d'écoute est strictement loopback.
/// Fail-closed : toute adresse non parsable ou « nom d'hôte » est considérée non-loopback.
pub fn is_loopback_bind(bind_addr: &str) -> bool {
    bind_addr
        .parse::<std::net::SocketAddr>()
        .is_ok_and(|a| a.ip().to_canonical().is_loopback())
}

/// Valide la configuration avant démarrage.
pub fn validate_startup(cfg: &TeamConfig) -> Result<(), StartupError> {
    match cfg.secret() {
        Some(s) if s.len() < MIN_SECRET_LEN => Err(StartupError::SecretTooShort),
        None if !is_loopback_bind(&cfg.bind_addr) => {
            Err(StartupError::NonLoopbackWithoutSecret(cfg.bind_addr.clone()))
        }
        // Secret valide, ou loopback sans secret (dev local)
        _ => Ok(()),
    }
}

/// Construit la couche CORS à partir de la liste explicite d'origines.
/// Liste vide → aucune origine autorisée (pas d'en-têtes CORS). Les entrées
/// invalides ou `*` sont ignorées (avertissement).
pub fn build_cors(origins: &[String]) -> tower_http::cors::CorsLayer {
    use axum::http::{header, HeaderValue, Method};
    let values: Vec<HeaderValue> = origins
        .iter()
        .filter(|o| o.trim() != "*")
        .filter_map(|o| {
            if let Ok(v) = HeaderValue::from_str(o.trim()) {
                Some(v)
            } else {
                tracing::warn!("Origine CORS invalide ignorée : {o}");
                None
            }
        })
        .collect();
    if values.len() != origins.len() {
        tracing::warn!("Certaines origines CORS ont été ignorées (joker '*' interdit ou invalide)");
    }
    let layer = tower_http::cors::CorsLayer::new();
    if values.is_empty() {
        return layer;
    }
    layer
        .allow_origin(values)
        .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::OPTIONS])
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
}

/// Charge la config depuis le fichier TOML ou retourne les défauts.
/// `TEAM_HMAC_SECRET` (si non vide) remplace `hmac_secret` du fichier.
pub fn load_or_create() -> TeamConfig {
    let config_path = std::env::var("TEAM_CONFIG").unwrap_or_else(|_| "team.toml".into());

    let mut cfg = if let Ok(content) = std::fs::read_to_string(&config_path) {
        match toml::from_str(&content) {
            Ok(cfg) => {
                tracing::info!("Config TEAM chargée depuis {config_path}");
                cfg
            }
            Err(e) => {
                tracing::warn!("Erreur parse {config_path} : {e} — utilisation des défauts");
                TeamConfig::default()
            }
        }
    } else {
        tracing::info!("Pas de config trouvée — utilisation des défauts");
        TeamConfig::default()
    };

    if let Ok(secret) = std::env::var(SECRET_ENV_VAR) {
        if !secret.trim().is_empty() {
            cfg.hmac_secret = Some(secret);
        }
    }
    cfg
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaut_est_loopback_et_valide() {
        let cfg = TeamConfig::default();
        assert!(is_loopback_bind(&cfg.bind_addr));
        assert_eq!(cfg.cors_allowed_origins, Vec::<String>::new());
        assert_eq!(validate_startup(&cfg), Ok(()));
    }

    #[test]
    fn refus_non_loopback_sans_secret() {
        let cfg = TeamConfig {
            bind_addr: "0.0.0.0:7800".into(),
            ..TeamConfig::default()
        };
        assert!(matches!(
            validate_startup(&cfg),
            Err(StartupError::NonLoopbackWithoutSecret(_))
        ));
        // Secret vide = absent
        let cfg = TeamConfig {
            bind_addr: "0.0.0.0:7800".into(),
            hmac_secret: Some("   ".into()),
            ..TeamConfig::default()
        };
        assert!(matches!(
            validate_startup(&cfg),
            Err(StartupError::NonLoopbackWithoutSecret(_))
        ));
    }

    #[test]
    fn non_loopback_avec_secret_accepte() {
        let cfg = TeamConfig {
            bind_addr: "0.0.0.0:7800".into(),
            hmac_secret: Some("0123456789abcdef".into()),
            ..TeamConfig::default()
        };
        assert_eq!(validate_startup(&cfg), Ok(()));
    }

    #[test]
    fn refus_secret_trop_court() {
        let cfg = TeamConfig {
            hmac_secret: Some("court".into()),
            ..TeamConfig::default()
        };
        assert_eq!(validate_startup(&cfg), Err(StartupError::SecretTooShort));
    }

    #[test]
    fn hote_non_parsable_est_non_loopback() {
        assert!(!is_loopback_bind("localhost:7800"));
        assert!(!is_loopback_bind("0.0.0.0:7800"));
        assert!(is_loopback_bind("[::1]:7800"));
        assert!(is_loopback_bind("127.0.0.1:7800"));
    }

    #[test]
    fn debug_masque_le_secret() {
        let cfg = TeamConfig {
            hmac_secret: Some("super-secret-value-123".into()),
            ..TeamConfig::default()
        };
        assert!(!format!("{cfg:?}").contains("super-secret-value-123"));
    }
}
