//! Configuration du Miyukini AI Studio.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Configuration principale.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeConfig {
    /// Adresse d'écoute (ex: "0.0.0.0:11435").
    #[serde(default = "default_bind_addr")]
    pub bind_addr: String,

    /// URL upstream de LM Studio / Ollama (backup).
    #[serde(default = "default_upstream_url")]
    pub upstream_url: String,

    /// Modèle par défaut (optionnel, auto-detect si absent).
    #[serde(default)]
    pub default_model: Option<String>,

    /// Token d'authentification optionnel (Bearer token).
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
    /// Origines autorisées (IP/host). Vide = localhost uniquement.
    /// Exemples : ["127.0.0.1", "::1", "192.168.1.0/24"]
    #[serde(default)]
    pub allowed_origins: Vec<String>,

    /// Requiert l'authentification Bearer sur tous les endpoints (pas seulement le proxy).
    #[serde(default)]
    pub require_auth_all: bool,

    /// Clé secrète HMAC-SHA256 pour la signature inter-services COG.
    /// Si définie, les requêtes inter-services doivent inclure X-COG-Signature.
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
            require_auth_all: false,
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
    "0.0.0.0:11435".into()
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
