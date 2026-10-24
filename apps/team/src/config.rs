//! @id     team_config
//! @do     definir_team_config
//! @role   config
//! @layer  app
//! @human  team — module « config ».

//! Configuration TEAM.
//!
//! Sécurité (fail-closed, C-12 / C-21 / C-22, sortie du lot 2) :
//! - écoute par défaut sur `127.0.0.1:7800` ;
//! - plus de secret partagé : **toutes** les routes (sauf `GET /team/health`)
//!   exigent une requête signée par un appelant épinglé (`noeuds_autorises`
//!   pour les nœuds MAIA, `clients_autorises` pour `/team/nodes` et `/api/*`),
//!   y compris en loopback ; famille sans clé épinglée ⇒ 503 ;
//! - une adresse non-loopback exige au moins un appelant épinglé, sinon refus
//!   de démarrer ; une clé épinglée mal formée ⇒ refus de démarrer ;
//! - le hub signe ses propres appels aux nœuds MAIA avec sa clé de service
//!   (graine `cle_service`, fichier 0600 créé une seule fois) ;
//! - `hmac_secret` / `TEAM_HMAC_SECRET` : obsolètes, ignorés (avertissement) ;
//! - CORS : liste blanche explicite `cors_allowed_origins`, vide par défaut ;
//! - URL de nœuds : loopback ou `node_allowed_hosts` (voir `mws::policy`).

use serde::{Deserialize, Serialize};

/// Ancienne variable d'environnement du secret partagé (obsolète, ignorée).
pub const SECRET_ENV_VAR: &str = "TEAM_HMAC_SECRET";

/// Configuration complète du hub TEAM.
#[derive(Clone, Serialize, Deserialize)]
pub struct TeamConfig {
    /// Adresse d'écoute (défaut : "127.0.0.1:7800", loopback uniquement).
    /// Une adresse non-loopback exige au moins un appelant épinglé.
    #[serde(default = "default_bind_addr")]
    pub bind_addr: String,
    /// Clés publiques Ed25519 (base64 URL ou hex) des nœuds MAIA autorisés à
    /// s'enregistrer (usage `team/noeud`). Vide : enregistrement refusé (503).
    #[serde(default)]
    pub noeuds_autorises: Vec<String>,
    /// Clés publiques des clients autorisés sur `/team/nodes` et `/api/*`
    /// (usage `team/api`). Vide : refus (503).
    #[serde(default)]
    pub clients_autorises: Vec<String>,
    /// Graine de la clé de service du hub (signe ses appels aux nœuds MAIA ;
    /// fichier 0600 créé au premier démarrage).
    #[serde(default = "default_cle_service")]
    pub cle_service: String,
    /// Obsolète (lot 2) : ancien secret partagé. Ignoré — jamais une
    /// autorisation ; avertissement au démarrage s'il est présent.
    #[serde(default, skip_serializing)]
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
            .field("noeuds_autorises", &self.noeuds_autorises)
            .field("clients_autorises", &self.clients_autorises)
            .field("cle_service", &self.cle_service)
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
            noeuds_autorises: Vec::new(),
            clients_autorises: Vec::new(),
            cle_service: default_cle_service(),
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

/// Clés publiques épinglées `(nœuds, clients)`.
pub type ClesAutorisees = (Vec<[u8; 32]>, Vec<[u8; 32]>);

impl TeamConfig {
    /// Clés épinglées `(nœuds, clients)`. Une entrée mal formée ⇒ erreur.
    pub fn cles_autorisees(&self) -> Result<ClesAutorisees, StartupError> {
        use miyauth::http::service::analyser_cles_publiques;
        let noeuds = analyser_cles_publiques(&self.noeuds_autorises)
            .map_err(|e| StartupError::MalformedKey(format!("noeuds_autorises : {e}")))?;
        let clients = analyser_cles_publiques(&self.clients_autorises)
            .map_err(|e| StartupError::MalformedKey(format!("clients_autorises : {e}")))?;
        Ok((noeuds, clients))
    }

    /// Vrai si l'ancien secret partagé est encore configuré (ignoré).
    pub fn ancien_secret_present(&self) -> bool {
        self.hmac_secret.as_deref().is_some_and(|s| !s.trim().is_empty())
    }
}

fn default_bind_addr() -> String {
    "127.0.0.1:7800".into()
}
fn default_cle_service() -> String {
    "keys/team.seed".into()
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
    /// Adresse d'écoute non-loopback sans appelant épinglé.
    NonLoopbackWithoutCaller(String),
    /// Clé publique épinglée mal formée.
    MalformedKey(String),
}

impl std::fmt::Display for StartupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonLoopbackWithoutCaller(addr) => write!(
                f,
                "Refus de démarrer : bind_addr={addr} n'est pas loopback et aucun \
                 appelant n'est épinglé. Renseignez `noeuds_autorises` / \
                 `clients_autorises` ou utilisez 127.0.0.1."
            ),
            Self::MalformedKey(e) => write!(f, "Refus de démarrer : clé épinglée invalide ({e})."),
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
    let (noeuds, clients) = cfg.cles_autorisees()?;
    if !is_loopback_bind(&cfg.bind_addr) && noeuds.is_empty() && clients.is_empty() {
        return Err(StartupError::NonLoopbackWithoutCaller(cfg.bind_addr.clone()));
    }
    Ok(())
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
        .allow_headers([
            header::CONTENT_TYPE,
            axum::http::HeaderName::from_static(miyauth::http::ENTETE_SIGNATAIRE),
            axum::http::HeaderName::from_static(miyauth::http::ENTETE_HORODATAGE),
            axum::http::HeaderName::from_static(miyauth::http::ENTETE_NONCE),
            axum::http::HeaderName::from_static(miyauth::http::ENTETE_SIGNATURE),
        ])
}

/// Charge la config depuis le fichier TOML ou retourne les défauts.
/// `TEAM_HMAC_SECRET` (obsolète) est seulement signalé, jamais utilisé.
pub fn load_or_create() -> TeamConfig {
    let config_path = std::env::var("TEAM_CONFIG").unwrap_or_else(|_| "team.toml".into());

    let cfg = if let Ok(content) = std::fs::read_to_string(&config_path) {
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

    if std::env::var(SECRET_ENV_VAR).is_ok_and(|s| !s.trim().is_empty()) {
        tracing::warn!("{SECRET_ENV_VAR} est obsolète et ignorée : épinglez les clés des nœuds (noeuds_autorises)");
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

    fn cle_b64() -> String {
        miyauth::http::service::encoder_cle_publique(&miyusceau::CleSignature::generer().cle_publique())
    }

    #[test]
    fn refus_non_loopback_sans_appelant() {
        let cfg = TeamConfig {
            bind_addr: "0.0.0.0:7800".into(),
            ..TeamConfig::default()
        };
        assert!(matches!(
            validate_startup(&cfg),
            Err(StartupError::NonLoopbackWithoutCaller(_))
        ));
        // L'ancien secret partagé n'autorise plus rien.
        let cfg = TeamConfig {
            bind_addr: "0.0.0.0:7800".into(),
            hmac_secret: Some("0123456789abcdef".into()),
            ..TeamConfig::default()
        };
        assert!(matches!(
            validate_startup(&cfg),
            Err(StartupError::NonLoopbackWithoutCaller(_))
        ));
    }

    #[test]
    fn non_loopback_avec_appelant_accepte() {
        let cfg = TeamConfig {
            bind_addr: "0.0.0.0:7800".into(),
            noeuds_autorises: vec![cle_b64()],
            ..TeamConfig::default()
        };
        assert_eq!(validate_startup(&cfg), Ok(()));
    }

    #[test]
    fn refus_cle_mal_formee() {
        let cfg = TeamConfig {
            clients_autorises: vec!["pas-une-cle".into()],
            ..TeamConfig::default()
        };
        assert!(matches!(validate_startup(&cfg), Err(StartupError::MalformedKey(_))));
    }

    #[test]
    fn hote_non_parsable_est_non_loopback() {
        assert!(!is_loopback_bind("localhost:7800"));
        assert!(!is_loopback_bind("0.0.0.0:7800"));
        assert!(is_loopback_bind("[::1]:7800"));
        assert!(is_loopback_bind("127.0.0.1:7800"));
    }

    #[test]
    fn ancien_secret_masque_et_jamais_reecrit() {
        let cfg = TeamConfig {
            hmac_secret: Some("super-secret-value-123".into()),
            ..TeamConfig::default()
        };
        assert!(cfg.ancien_secret_present());
        assert!(!format!("{cfg:?}").contains("super-secret-value-123"));
        assert!(!toml::to_string(&cfg).unwrap().contains("super-secret-value-123"));
    }
}
