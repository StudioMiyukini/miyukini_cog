//! @id     maia_llm_config
//! @do     definir_maia_llm_config
//! @role   config
//! @layer  app
//! @human  maia-llm — module « config ».

//! Configuration de MAIA-LLM.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Configuration principale de maia-llm.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaiaLlmConfig {
    /// Adresse d'écoute. Défaut : "127.0.0.1:11435" (loopback uniquement).
    /// Une adresse non-loopback (ex: "0.0.0.0:11435") exige `auth_key` :
    /// sans clé, le service refuse de démarrer (voir `validate_bind`).
    #[serde(default = "default_bind_addr")]
    pub bind_addr: String,

    /// Dossier contenant les fichiers .gguf.
    #[serde(default = "default_models_dir")]
    pub models_dir: String,

    /// Nombre de couches GPU (-1 = auto, 0 = CPU only, N = N couches).
    #[serde(default = "default_gpu_layers")]
    pub gpu_layers: i32,

    /// Taille du contexte en tokens (0 = défaut modèle, ~4096).
    #[serde(default = "default_context_size")]
    pub context_size: u32,

    /// Charger automatiquement le meilleur modèle au démarrage.
    #[serde(default = "default_true")]
    pub auto_load: bool,

    /// Clé d'authentification Bearer (X-MAIA-Key header). None = pas d'auth
    /// (autorisé uniquement si le bind est loopback).
    #[serde(default)]
    pub auth_key: Option<String>,

    /// Origines CORS autorisées (ex: ["http://localhost:3000"]).
    /// Vide (défaut) = aucun en-tête CORS émis, donc pas d'accès cross-origin
    /// depuis un navigateur. Jamais de joker `*`.
    #[serde(default)]
    pub cors_allowed_origins: Vec<String>,

    /// Active les logs des requêtes.
    #[serde(default = "default_true")]
    pub log_requests: bool,
}

impl Default for MaiaLlmConfig {
    fn default() -> Self {
        Self {
            bind_addr: default_bind_addr(),
            models_dir: default_models_dir(),
            gpu_layers: default_gpu_layers(),
            context_size: default_context_size(),
            auto_load: true,
            auth_key: None,
            cors_allowed_origins: Vec::new(),
            log_requests: true,
        }
    }
}

fn default_bind_addr() -> String {
    "127.0.0.1:11435".into()
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
fn default_true() -> bool {
    true
}

/// Vrai si l'adresse d'écoute est strictement loopback.
/// Fail-closed : toute adresse non parsable ou « nom d'hôte » est considérée non-loopback.
pub fn is_loopback_bind(bind_addr: &str) -> bool {
    bind_addr
        .parse::<std::net::SocketAddr>()
        .map(|a| a.ip().to_canonical().is_loopback())
        .unwrap_or(false)
}

/// Valide la combinaison bind/auth : refuse un bind non-loopback sans `auth_key`.
pub fn validate_bind(cfg: &MaiaLlmConfig) -> Result<(), String> {
    let has_key = cfg.auth_key.as_deref().is_some_and(|k| !k.trim().is_empty());
    if !is_loopback_bind(&cfg.bind_addr) && !has_key {
        return Err(format!(
            "Refus de démarrer : bind_addr={} n'est pas loopback et aucune auth_key n'est \
             configurée. Définissez `auth_key` (ou --auth-key) ou utilisez 127.0.0.1.",
            cfg.bind_addr
        ));
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
        .filter_map(|o| match HeaderValue::from_str(o.trim()) {
            Ok(v) => Some(v),
            Err(_) => {
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
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
            axum::http::HeaderName::from_static("x-maia-key"),
        ])
}

/// Résout le chemin du fichier de configuration.
/// Priorité : `MAIA_DATA_DIR/maia-llm.toml` > `./maia-llm.toml`.
pub fn config_path() -> PathBuf {
    if let Ok(data_dir) = std::env::var("MAIA_DATA_DIR") {
        Path::new(&data_dir).join("maia-llm.toml")
    } else {
        PathBuf::from("maia-llm.toml")
    }
}

/// Charge la config depuis le fichier TOML, ou crée le fichier par défaut si absent.
pub fn load_or_create() -> MaiaLlmConfig {
    let path = config_path();

    if path.exists() {
        match std::fs::read_to_string(&path) {
            Ok(content) => match toml::from_str::<MaiaLlmConfig>(&content) {
                Ok(cfg) => return cfg,
                Err(e) => tracing::warn!("Config invalide ({path:?}), défauts utilisés : {e}"),
            },
            Err(e) => tracing::warn!("Impossible de lire {path:?} : {e}"),
        }
    }

    let cfg = MaiaLlmConfig::default();

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
    fn defaut_est_loopback() {
        assert!(is_loopback_bind(&MaiaLlmConfig::default().bind_addr));
        assert!(validate_bind(&MaiaLlmConfig::default()).is_ok());
    }

    #[test]
    fn refus_non_loopback_sans_auth() {
        let mut c = MaiaLlmConfig::default();
        c.bind_addr = "0.0.0.0:11435".into();
        assert!(validate_bind(&c).is_err());
        c.auth_key = Some("  ".into());
        assert!(validate_bind(&c).is_err());
        c.auth_key = Some("secret".into());
        assert!(validate_bind(&c).is_ok());
    }

    #[test]
    fn hote_non_parsable_est_non_loopback() {
        assert!(!is_loopback_bind("localhost:11435"));
        assert!(is_loopback_bind("[::1]:11435"));
    }
}
