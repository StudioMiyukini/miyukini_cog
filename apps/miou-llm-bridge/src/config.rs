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
    /// Une adresse non-loopback (ex: "0.0.0.0:11435") exige au moins un appelant
    /// épinglé (`[security] cles_appelants` / `cles_hub_team`) : sinon, le
    /// service refuse de démarrer (voir `validate_startup`).
    #[serde(default = "default_bind_addr")]
    pub bind_addr: String,

    /// URL upstream de LM Studio / Ollama (backup).
    #[serde(default = "default_upstream_url")]
    pub upstream_url: String,

    /// Modèle par défaut (optionnel, auto-detect si absent).
    #[serde(default)]
    pub default_model: Option<String>,

    /// Graine de la clé de service de MAIA (signe ses appels à maia-llm et au hub
    /// TEAM ; fichier 0600 créé une seule fois). Absent :
    /// `$MIYUKINI_DATA_DIR/keys/maia.seed`, sinon `keys/maia.seed`.
    #[serde(default)]
    pub cle_service: Option<String>,

    /// Obsolète (lot 2) : ancien jeton Bearer partagé. Ignoré — jamais une
    /// autorisation ; avertissement au démarrage s'il est présent.
    #[serde(default, skip_serializing)]
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

    /// Répertoire des politiques StrongFather (fichier `maia.toml`). Absent :
    /// `COG_POLITIQUES_DIR`, sinon `config/strongfather`. Politiques absentes
    /// ou invalides ⇒ tout outil et toute écriture d'agent refusés (K-11).
    #[serde(default)]
    pub politiques_dir: Option<String>,
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

    /// Clés publiques Ed25519 (base64 URL ou hex) des appelants autorisés sur
    /// toute l'API protégée (requêtes signées, usage `maia/api` : Central, …).
    /// Vide (défaut) : toute requête protégée est refusée, y compris en loopback.
    #[serde(default)]
    pub cles_appelants: Vec<String>,

    /// Clés publiques des hubs TEAM autorisés, **uniquement** sur
    /// `POST /v1/agents/{id}/chat` (usage `maia/team`).
    #[serde(default)]
    pub cles_hub_team: Vec<String>,

    /// Obsolète (lot 2) : l'authentification est désormais toujours exigée.
    /// Ignoré.
    #[serde(default, skip_serializing)]
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

    /// Obsolète (lot 2) : ancien secret HMAC partagé (jamais appliqué).
    /// Ignoré ; remplacé par les requêtes signées Ed25519.
    #[serde(default, skip_serializing)]
    pub hmac_secret: Option<String>,

    /// Active le chiffrement AES-256-GCM des réponses sensibles.
    /// Les clients envoient X-COG-Encrypt: true pour recevoir des réponses chiffrées.
    #[serde(default)]
    pub encryption_key: Option<String>,

    /// Bloque les requêtes sans User-Agent COG reconnu (défaut: false).
    #[serde(default)]
    pub strict_user_agent: bool,

    /// Limite les IP qui peuvent accéder aux endpoints d'administration.
    /// Vide = mêmes règles que allowed_origins.
    #[serde(default)]
    pub admin_origins: Vec<String>,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            allowed_origins: Vec::new(),
            cles_appelants: Vec::new(),
            cles_hub_team: Vec::new(),
            require_auth_all: None,
            trusted_proxies: Vec::new(),
            cors_allowed_origins: Vec::new(),
            hmac_secret: None,
            encryption_key: None,
            strict_user_agent: false,
            admin_origins: Vec::new(),
        }
    }
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
            cle_service: None,
            auth_token: None,
            log_requests: true,
            native: NativeInferenceConfig::default(),
            security: SecurityConfig::default(),
            politiques_dir: None,
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

/// Clés publiques épinglées `(appelants, hubs TEAM)`.
pub type ClesEpinglees = (Vec<[u8; 32]>, Vec<[u8; 32]>);

impl BridgeConfig {
    /// Clés épinglées `(appelants, hubs TEAM)`. Une entrée mal formée ⇒ erreur.
    pub fn cles_epinglees(&self) -> Result<ClesEpinglees, String> {
        use miyauth::http::service::analyser_cles_publiques;
        let appelants = analyser_cles_publiques(&self.security.cles_appelants)
            .map_err(|e| format!("security.cles_appelants : {e}"))?;
        let hubs = analyser_cles_publiques(&self.security.cles_hub_team)
            .map_err(|e| format!("security.cles_hub_team : {e}"))?;
        Ok((appelants, hubs))
    }

    /// Champs obsolètes encore présents (ignorés, signalés au démarrage).
    pub fn champs_obsoletes(&self) -> Vec<&'static str> {
        let mut v = Vec::new();
        if self.auth_token.is_some() {
            v.push("auth_token");
        }
        if self.security.require_auth_all.is_some() {
            v.push("security.require_auth_all");
        }
        if self.security.hmac_secret.is_some() {
            v.push("security.hmac_secret");
        }
        v
    }

    /// Chemin de la graine de la clé de service.
    pub fn chemin_cle_service(&self) -> PathBuf {
        if let Some(p) = self.cle_service.as_deref().filter(|p| !p.trim().is_empty()) {
            return PathBuf::from(p);
        }
        match std::env::var("MIYUKINI_DATA_DIR") {
            Ok(d) => Path::new(&d).join("keys").join("maia.seed"),
            Err(_) => PathBuf::from("keys/maia.seed"),
        }
    }

    /// Ajoute à `[security] cles_appelants` les clés publiques de `valeur`
    /// (séparées par des virgules ou des espaces ; doublons ignorés). Contenu de
    /// [`ENV_CLES_APPELANTS_SUPPLEMENTAIRES`], posé par Central quand il lance
    /// MAIA. En mémoire seulement (jamais réécrit dans `bridge.toml`) ; une entrée
    /// mal formée fera échouer [`Self::validate_startup`] (fail-closed).
    /// Retourne le nombre de clés ajoutées.
    pub fn fusionner_cles_appelants(&mut self, valeur: Option<&str>) -> usize {
        let Some(valeur) = valeur else {
            return 0;
        };
        let mut ajoutees = 0;
        for cle in valeur.split(|c: char| c == ',' || c.is_whitespace()).filter(|c| !c.is_empty()) {
            if !self.security.cles_appelants.iter().any(|c| c.trim() == cle) {
                self.security.cles_appelants.push(cle.to_string());
                ajoutees += 1;
            }
        }
        ajoutees
    }

    /// Valide la configuration (fail-closed). À appeler avant d'ouvrir le port :
    /// clés épinglées bien formées, et pas de bind non-loopback sans appelant.
    pub fn validate_startup(&self) -> Result<(), String> {
        let (appelants, hubs) = self.cles_epinglees().map_err(|e| format!("Refus de démarrer : {e}"))?;
        if !is_loopback_bind(&self.bind_addr) && appelants.is_empty() && hubs.is_empty() {
            return Err(format!(
                "Refus de démarrer : bind_addr={} n'est pas loopback et aucun appelant \
                 n'est épinglé. Renseignez `[security] cles_appelants` dans bridge.toml \
                 ou utilisez 127.0.0.1.",
                self.bind_addr
            ));
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

/// Variable d'environnement : clés publiques d'appelants **supplémentaires**
/// (base64 URL ou hex, séparées par des virgules), ajoutées en mémoire à
/// `[security] cles_appelants`. Posée par Central quand il lance MAIA, pour
/// épingler sa propre clé de service sans éditer `bridge.toml`.
pub const ENV_CLES_APPELANTS_SUPPLEMENTAIRES: &str = "MAIA_CLES_APPELANTS_SUPPLEMENTAIRES";

/// Charge la config (fichier TOML, créé par défaut si absent) puis ajoute les
/// appelants de [`ENV_CLES_APPELANTS_SUPPLEMENTAIRES`].
pub fn load_or_create() -> BridgeConfig {
    let mut cfg = load_or_create_fichier();
    let env = std::env::var(ENV_CLES_APPELANTS_SUPPLEMENTAIRES).ok();
    let n = cfg.fusionner_cles_appelants(env.as_deref());
    if n > 0 {
        tracing::info!("{n} appelant(s) épinglé(s) via {ENV_CLES_APPELANTS_SUPPLEMENTAIRES} (lancement par Central)");
    }
    cfg
}

/// Charge la config depuis le fichier TOML. Crée le fichier par défaut si absent.
fn load_or_create_fichier() -> BridgeConfig {
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

    fn cle_b64() -> String {
        miyauth::http::service::encoder_cle_publique(&miyusceau::CleSignature::generer().cle_publique())
    }

    #[test]
    fn defaut_loopback_demarre_mais_n_epingle_rien() {
        let c = BridgeConfig::default();
        assert!(c.bind_addr.starts_with("127.0.0.1:"));
        assert!(c.validate_startup().is_ok());
        assert_eq!(c.cles_epinglees().unwrap(), (vec![], vec![]));
    }

    #[test]
    fn refus_demarrage_non_loopback_sans_appelant() {
        let mut c = BridgeConfig {
            bind_addr: "0.0.0.0:11435".into(),
            ..BridgeConfig::default()
        };
        assert!(c.validate_startup().is_err());
        // L'ancien jeton partagé n'autorise plus rien.
        c.auth_token = Some("secret".into());
        assert!(c.validate_startup().is_err());
        c.security.cles_appelants = vec![cle_b64()];
        assert!(c.validate_startup().is_ok());
    }

    #[test]
    fn cle_epinglee_mal_formee_refusee() {
        let mut c = BridgeConfig::default();
        c.security.cles_hub_team = vec!["xyz".into()];
        assert!(c.validate_startup().is_err());
    }

    #[test]
    fn champs_obsoletes_signales_et_jamais_reecrits() {
        let c: BridgeConfig = toml::from_str(
            "auth_token = \"s3cret\"\n[security]\nrequire_auth_all = true\nhmac_secret = \"h\"\n",
        )
        .unwrap();
        assert_eq!(c.champs_obsoletes(), vec!["auth_token", "security.require_auth_all", "security.hmac_secret"]);
        let t = toml::to_string(&c).unwrap();
        assert!(!t.contains("s3cret") && !t.contains("hmac_secret") && !t.contains("require_auth_all"));
    }

    #[test]
    fn appelants_supplementaires_fusionnes_sans_doublon() {
        let a = cle_b64();
        let b = cle_b64();
        let mut c = BridgeConfig::default();
        c.security.cles_appelants = vec![a.clone()];
        assert_eq!(c.fusionner_cles_appelants(None), 0);
        assert_eq!(c.fusionner_cles_appelants(Some(&format!("{a}, {b},,"))), 1);
        assert_eq!(c.security.cles_appelants, vec![a.clone(), b.clone()]);
        let (appelants, _) = c.cles_epinglees().unwrap();
        assert_eq!(appelants.len(), 2);
        // Jamais réécrit dans un fichier : seule la mémoire change ; entrée mal formée ⇒ refus.
        c.fusionner_cles_appelants(Some("pas-une-cle"));
        assert!(c.validate_startup().is_err());
    }

    #[test]
    fn hote_non_parsable_non_loopback() {
        assert!(!is_loopback_bind("localhost:11435"));
        assert!(is_loopback_bind("[::1]:11435"));
    }
}
