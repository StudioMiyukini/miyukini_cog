//! @id     miou_llm_bridge_config
//! @do     definir_bridge_config
//! @role   config
//! @layer  app
//! @human  miou-llm-bridge — module « config ».

//! Configuration du Miyukini AI Studio.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Configuration principale.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeConfig {
    /// Adresse d'écoute. Défaut : "127.0.0.1:11435" (loopback uniquement).
    /// Une adresse non-loopback (ex: "0.0.0.0:11435") exige un `auth_token` :
    /// sans token, le service refuse de démarrer (voir `validate_startup`).
    #[serde(default = "default_bind_addr")]
    pub bind_addr: String,

    /// URL upstream de LM Studio / Ollama (backup).
    #[serde(default = "default_upstream_url")]
    pub upstream_url: String,

    /// Modèle par défaut (optionnel, auto-detect si absent).
    #[serde(default)]
    pub default_model: Option<String>,

    /// Token d'authentification (Bearer token). Obligatoire hors loopback.
    #[serde(default)]
    pub auth_token: Option<String>,

    /// Active les logs des requêtes.
    #[serde(default = "default_true")]
    pub log_requests: bool,

    /// Configuration de l'inférence native GGUF.
    #[serde(default)]
    pub native: NativeInferenceConfig,

    /// Configuration de sécurité renforcée.
    #[serde(default)]
    pub security: SecurityConfig,
}

/// Configuration pour l'inférence GGUF native (llama-cpp-2).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NativeInferenceConfig {
    /// Dossier contenant les fichiers .gguf.
    #[serde(default = "default_models_dir")]
    pub models_dir: String,

    /// Préférer l'inférence native plutôt que l'upstream.
    #[serde(default = "default_true")]
    pub prefer_native: bool,

    /// Nombre de couches à décharger sur le GPU (0 = CPU only, -1 = auto).
    #[serde(default = "default_gpu_layers")]
    pub gpu_layers: i32,

    /// Taille du contexte en tokens (0 = défaut du modèle).
    #[serde(default = "default_context_size")]
    pub context_size: u32,

    /// Nombre de threads CPU pour l'inférence (0 = auto).
    #[serde(default)]
    pub threads: u32,

    /// Charger automatiquement le meilleur modèle au démarrage.
    #[serde(default = "default_true")]
    pub auto_load: bool,
}

/// Configuration de sécurité renforcée.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Origines autorisées (IP/CIDR source). Vide = loopback uniquement
    /// (ce qui n'est PAS une restriction si le bridge est derrière un proxy
    /// local : voir `trusted_proxies`).
    /// Exemples : ["127.0.0.1", "::1", "192.168.1.0/24"]
    #[serde(default)]
    pub allowed_origins: Vec<String>,

    /// Requiert l'authentification Bearer sur tous les endpoints protégés.
    /// Non défini (défaut) : activé automatiquement dès que le bind n'est pas
    /// loopback, désactivé en loopback (usage local sans token). Explicite
    /// `true` sans `auth_token` => refus de démarrer (fail-closed).
    #[serde(default)]
    pub require_auth_all: Option<bool>,

    /// Proxies de confiance (IP ou CIDR IPv4). `X-Forwarded-For` n'est lu QUE
    /// si la connexion TCP provient d'une de ces adresses. Vide par défaut :
    /// l'IP source est toujours l'adresse réelle de la connexion.
    #[serde(default)]
    pub trusted_proxies: Vec<String>,

    /// Origines CORS autorisées (ex: ["http://localhost:3000"]).
    /// Vide (défaut) = pas de CORS cross-origin. Le joker `*` est ignoré.
    #[serde(default)]
    pub cors_allowed_origins: Vec<String>,

    /// Clé secrète HMAC-SHA256 pour la signature inter-services COG.
    /// ATTENTION : la signature n'est PAS appliquée par le middleware actuel
    /// (`verify_hmac` n'est appelée par aucun handler) ; seul un contrôle de
    /// fraîcheur du timestamp est fait si X-COG-Signature est présent.
    /// Ne pas s'appuyer sur ce champ comme protection anti-MITM.
    #[serde(default)]
    pub hmac_secret: Option<String>,

    /// Active le chiffrement AES-256-GCM des réponses sensibles.
    /// Les clients envoient X-COG-Encrypt: true pour recevoir des réponses chiffrées.
    #[serde(default)]
    pub encryption_key: Option<String>,

    /// Bloque les requêtes sans User-Agent COG reconnu (défaut: false).
    #[serde(default)]
    pub strict_user_agent: bool,

    /// Durée max d'une session HMAC en secondes (anti-replay). Défaut: 300s.
    #[serde(default = "default_hmac_max_age")]
    pub hmac_max_age_secs: u64,

    /// Limite les IP qui peuvent accéder aux endpoints d'administration.
    /// Vide = mêmes règles que allowed_origins.
    #[serde(default)]
    pub admin_origins: Vec<String>,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            allowed_origins: Vec::new(),
            require_auth_all: None,
            trusted_proxies: Vec::new(),
            cors_allowed_origins: Vec::new(),
            hmac_secret: None,
            encryption_key: None,
            strict_user_agent: false,
            hmac_max_age_secs: 300,
            admin_origins: Vec::new(),
        }
    }
}

fn default_hmac_max_age() -> u64 {
    300
}

fn default_bind_addr() -> String {
    "127.0.0.1:11435".into()
}
fn default_upstream_url() -> String {
    "http://localhost:1234".into()
}
fn default_true() -> bool {
    true
}
fn default_models_dir() -> String {
    "models".into()
}
fn default_gpu_layers() -> i32 {
    -1 // auto
}
fn default_context_size() -> u32 {
    4096
}

impl Default for NativeInferenceConfig {
    fn default() -> Self {
        Self {
            models_dir: default_models_dir(),
            prefer_native: true,
            gpu_layers: -1,
            context_size: 4096,
            threads: 0,
            auto_load: true,
        }
    }
}

impl Default for BridgeConfig {
    fn default() -> Self {
        Self {
            bind_addr: default_bind_addr(),
            upstream_url: default_upstream_url(),
            default_model: None,
            auth_token: None,
            log_requests: true,
            native: NativeInferenceConfig::default(),
            security: SecurityConfig::default(),
        }
    }
}

/// Vrai si l'adresse d'écoute est strictement loopback.
/// Fail-closed : adresse non parsable ou nom d'hôte => non-loopback.
pub fn is_loopback_bind(bind_addr: &str) -> bool {
    bind_addr
        .parse::<std::net::SocketAddr>()
        .map(|a| a.ip().to_canonical().is_loopback())
        .unwrap_or(false)
}

impl BridgeConfig {
    /// Valeur effective de `require_auth_all` : explicite, sinon vrai hors loopback.
    pub fn effective_require_auth_all(&self) -> bool {
        self.security
            .require_auth_all
            .unwrap_or_else(|| !is_loopback_bind(&self.bind_addr))
    }

    fn has_token(&self) -> bool {
        self.auth_token.as_deref().is_some_and(|t| !t.trim().is_empty())
    }

    /// Valide la cohérence bind/auth (fail-closed). À appeler avant d'ouvrir le port.
    pub fn validate_startup(&self) -> Result<(), String> {
        if !is_loopback_bind(&self.bind_addr) && !self.has_token() {
            return Err(format!(
                "Refus de démarrer : bind_addr={} n'est pas loopback et aucun auth_token \
                 n'est configuré. Définissez `auth_token` dans bridge.toml ou utilisez \
                 127.0.0.1.",
                self.bind_addr
            ));
        }
        if self.effective_require_auth_all() && !self.has_token() {
            return Err("Refus de démarrer : require_auth_all est actif mais aucun \
                        auth_token n'est configuré."
                .into());
        }
        Ok(())
    }
}

/// Résout le chemin du fichier de configuration.
/// Priorité : `MIYUKINI_DATA_DIR/bridge.toml` > `./bridge.toml`.
pub fn config_path() -> PathBuf {
    if let Ok(data_dir) = std::env::var("MIYUKINI_DATA_DIR") {
        Path::new(&data_dir).join("bridge.toml")
    } else {
        PathBuf::from("bridge.toml")
    }
}

/// Charge la config depuis le fichier TOML. Crée le fichier par défaut si absent.
pub fn load_or_create() -> BridgeConfig {
    let path = config_path();

    if path.exists() {
        match std::fs::read_to_string(&path) {
            Ok(content) => match toml::from_str::<BridgeConfig>(&content) {
                Ok(cfg) => return cfg,
                Err(e) => {
                    tracing::warn!("Config invalide ({path:?}), utilisation des défauts : {e}");
                }
            },
            Err(e) => {
                tracing::warn!("Impossible de lire {path:?} : {e}");
            }
        }
    }

    let cfg = BridgeConfig::default();

    // Créer le fichier par défaut
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    match toml::to_string_pretty(&cfg) {
        Ok(content) => {
            if let Err(e) = std::fs::write(&path, &content) {
                tracing::warn!("Impossible d'écrire la config par défaut : {e}");
            } else {
                tracing::info!("Config par défaut créée : {path:?}");
            }
        }
        Err(e) => tracing::warn!("Impossible de sérialiser la config : {e}"),
    }

    cfg
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaut_loopback_utilisable_sans_token() {
        let c = BridgeConfig::default();
        assert!(c.bind_addr.starts_with("127.0.0.1:"));
        assert!(!c.effective_require_auth_all());
        assert!(c.validate_startup().is_ok());
    }

    #[test]
    fn refus_demarrage_non_loopback_sans_auth() {
        let mut c = BridgeConfig::default();
        c.bind_addr = "0.0.0.0:11435".into();
        assert!(c.validate_startup().is_err());
        c.auth_token = Some("   ".into());
        assert!(c.validate_startup().is_err());
        c.auth_token = Some("secret".into());
        assert!(c.effective_require_auth_all());
        assert!(c.validate_startup().is_ok());
    }

    #[test]
    fn require_auth_all_explicite_sans_token_refuse() {
        let mut c = BridgeConfig::default();
        c.security.require_auth_all = Some(true);
        assert!(c.validate_startup().is_err());
    }

    #[test]
    fn hote_non_parsable_non_loopback() {
        assert!(!is_loopback_bind("localhost:11435"));
        assert!(is_loopback_bind("[::1]:11435"));
    }
}
