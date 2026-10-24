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
    /// Une adresse non-loopback (ex: "0.0.0.0:11435") exige au moins un appelant
    /// épinglé : sinon, le service refuse de démarrer (voir `validate_bind`).
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

    /// Clés publiques Ed25519 (base64 URL ou hexadécimal) des appelants autorisés
    /// sur `/v1/*` (requêtes signées, usage `maia-llm/inference`). Vide (défaut) :
    /// toute requête d'inférence est refusée (503), y compris en loopback.
    #[serde(default)]
    pub appelants_inference: Vec<String>,

    /// Clés publiques des appelants autorisés sur `/maia/llm/models*`
    /// (usage `maia-llm/modeles`). Vide (défaut) : refus.
    #[serde(default)]
    pub appelants_modeles: Vec<String>,

    /// Obsolète (lot 2) : ancienne clé partagée. Ignorée — jamais une
    /// autorisation ; avertissement au démarrage si présente.
    #[serde(default, skip_serializing)]
    pub auth_key: Option<String>,

    /// Origines CORS autorisées (ex: ["http://localhost:3000"]).
    /// Vide (défaut) = aucun en-tête CORS émis, donc pas d'accès cross-origin
    /// depuis un navigateur. Jamais de joker `*`.
    #[serde(default)]
    pub cors_allowed_origins: Vec<String>,

    /// Active les logs des requêtes.
    #[serde(default = "default_true")]
    pub log_requests: bool,

    /// Ancres de confiance des modèles (EX-26) : clés publiques Ed25519 en
    /// hexadécimal (64 caractères) autorisées à signer les manifestes
    /// `X.gguf.sceau`. Vide (défaut) = aucun modèle chargeable.
    #[serde(default)]
    pub model_trust_anchors: Vec<String>,
}

impl Default for MaiaLlmConfig {
    fn default() -> Self {
        Self {
            bind_addr: default_bind_addr(),
            models_dir: default_models_dir(),
            gpu_layers: default_gpu_layers(),
            context_size: default_context_size(),
            auto_load: true,
            appelants_inference: Vec::new(),
            appelants_modeles: Vec::new(),
            auth_key: None,
            cors_allowed_origins: Vec::new(),
            log_requests: true,
            model_trust_anchors: Vec::new(),
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

/// Clés publiques épinglées `(inférence, modèles)`.
pub type ClesAppelants = (Vec<[u8; 32]>, Vec<[u8; 32]>);

/// Clés publiques épinglées `(inférence, modèles)`. Une entrée mal formée ⇒
/// erreur (refus de démarrer).
pub fn cles_appelants(cfg: &MaiaLlmConfig) -> Result<ClesAppelants, String> {
    use miyauth::http::service::analyser_cles_publiques;
    let inference = analyser_cles_publiques(&cfg.appelants_inference)
        .map_err(|e| format!("appelants_inference : {e}"))?;
    let modeles = analyser_cles_publiques(&cfg.appelants_modeles)
        .map_err(|e| format!("appelants_modeles : {e}"))?;
    Ok((inference, modeles))
}

/// Valide la combinaison bind/appelants : clés bien formées, et pas de bind
/// non-loopback sans appelant épinglé.
pub fn validate_bind(cfg: &MaiaLlmConfig) -> Result<(), String> {
    let (inference, modeles) = cles_appelants(cfg).map_err(|e| format!("Refus de démarrer : {e}"))?;
    if !is_loopback_bind(&cfg.bind_addr) && inference.is_empty() && modeles.is_empty() {
        return Err(format!(
            "Refus de démarrer : bind_addr={} n'est pas loopback et aucun appelant n'est \
             épinglé. Renseignez `appelants_inference` / `appelants_modeles` ou utilisez 127.0.0.1.",
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
            axum::http::HeaderName::from_static(miyauth::http::ENTETE_SIGNATAIRE),
            axum::http::HeaderName::from_static(miyauth::http::ENTETE_HORODATAGE),
            axum::http::HeaderName::from_static(miyauth::http::ENTETE_NONCE),
            axum::http::HeaderName::from_static(miyauth::http::ENTETE_SIGNATURE),
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
    fn refus_non_loopback_sans_appelant() {
        let mut c = MaiaLlmConfig {
            bind_addr: "0.0.0.0:11435".into(),
            ..MaiaLlmConfig::default()
        };
        assert!(validate_bind(&c).is_err());
        // L'ancienne clé partagée n'autorise plus rien.
        c.auth_key = Some("secret".into());
        assert!(validate_bind(&c).is_err());
        let cle = miyusceau::CleSignature::generer().cle_publique();
        c.appelants_inference = vec![miyauth::http::service::encoder_cle_publique(&cle)];
        assert!(validate_bind(&c).is_ok());
        assert_eq!(cles_appelants(&c).unwrap().0, vec![cle]);
    }

    #[test]
    fn cle_epinglee_mal_formee_refus_de_demarrer() {
        let c = MaiaLlmConfig {
            appelants_modeles: vec!["pas-une-cle".into()],
            ..MaiaLlmConfig::default()
        };
        assert!(validate_bind(&c).is_err());
    }

    #[test]
    fn auth_key_jamais_reecrite() {
        let c = MaiaLlmConfig {
            auth_key: Some("secret".into()),
            ..MaiaLlmConfig::default()
        };
        assert!(!toml::to_string(&c).unwrap().contains("secret"));
    }

    #[test]
    fn hote_non_parsable_est_non_loopback() {
        assert!(!is_loopback_bind("localhost:11435"));
        assert!(is_loopback_bind("[::1]:11435"));
    }
}
