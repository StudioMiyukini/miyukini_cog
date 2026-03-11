//! Configuration de MAIA-LLM.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Configuration principale de maia-llm.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaiaLlmConfig {
    /// Adresse d'écoute (ex: "0.0.0.0:11435").
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

    /// Clé d'authentification Bearer (X-MAIA-Key header). None = pas d'auth.
    #[serde(default)]
    pub auth_key: Option<String>,

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
            log_requests: true,
        }
    }
}

fn default_bind_addr() -> String {
    "0.0.0.0:11435".into()
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
