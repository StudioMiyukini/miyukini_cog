//! Configuration du serveur API REST Alicia.
//!
//! Chargee depuis la section `[api]` de `alicia.toml`.

// @id service.alicia.api.config
// @role api_configuration
// @layer 7
// @do define_api_config
// @human Configuration API REST Alicia : port, JWT TTL, rate limit, CORS.

use serde::{Deserialize, Serialize};

use crate::auth::JwtScope;
use crate::errors::ApiError;

/// Client API autorise a obtenir un token (registre configure).
///
/// Le secret n'est jamais stocke en clair : `secret_hash` est un hash Argon2id au format PHC
/// (`$argon2id$v=19$...`). Les scopes viennent EXCLUSIVEMENT de cette config.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiClientConfig {
    /// Identifiant du client.
    pub client_id: String,
    /// Hash Argon2id (PHC) du secret du client.
    pub secret_hash: String,
    /// Scopes accordes a ce client (ex: `["read","write"]`, `["admin"]`).
    pub scopes: Vec<JwtScope>,
    /// Personas qu'Alicia peut incarner pour ce client (R-002). Liste explicite,
    /// sans joker. Vide (defaut) : aucune persona, donc aucun dispatch ni
    /// resolution de confirmation possible (fail-closed).
    #[serde(default)]
    pub personas: Vec<String>,
    /// Le client peut agir pour **toutes** les personas existantes (decision
    /// proprietaire). Drapeau explicite, accepte uniquement si le client a
    /// aussi le scope `admin` (la validation refuse sinon) ; aucun joker dans
    /// `personas`. Porte dans le JWT signe (claim `all_personas`). Defaut :
    /// `false`.
    #[serde(default)]
    pub all_personas: bool,
}

/// Configuration du serveur API REST Alicia.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    /// Port d'ecoute. Defaut : 7890.
    #[serde(default = "ApiConfig::default_port")]
    pub port: u16,

    /// Hote d'ecoute. Defaut : "127.0.0.1" (loopback). "0.0.0.0" doit etre choisi
    /// explicitement. Une adresse invalide fait echouer le demarrage (pas de repli).
    #[serde(default = "ApiConfig::default_host")]
    pub host: String,

    /// Duree de vie des tokens JWT en secondes. Defaut : 3600 (1h).
    #[serde(default = "ApiConfig::default_token_ttl")]
    pub token_ttl_secs: u64,

    /// Nombre maximal de requetes par minute par adresse IP (limiteur en memoire, fenetre
    /// fixe de 60 s, derriere un proxy : IP du proxy). `0` = limitation desactivee.
    #[serde(default = "ApiConfig::default_rate_limit")]
    pub rate_limit_per_min: u32,

    /// Activer les headers CORS.
    #[serde(default)]
    pub cors_enabled: bool,

    /// Origines CORS autorisees (liste explicite, `*` refuse). Ignoree si `cors_enabled = false`.
    /// `cors_enabled = true` avec liste vide : demarrage refuse.
    #[serde(default)]
    pub cors_origins: Vec<String>,

    /// Registre des clients autorises pour `/auth/token` (feature `jeton-secret-client`
    /// seulement ; sans elle, un registre non vide est refuse a la validation).
    /// Vide (defaut) : la route refuse
    /// toute demande (fail-closed).
    #[serde(default)]
    pub clients: Vec<ApiClientConfig>,
}

impl ApiConfig {
    fn default_port() -> u16 {
        7890
    }
    fn default_host() -> String {
        "127.0.0.1".to_string()
    }
    fn default_token_ttl() -> u64 {
        3600
    }
    fn default_rate_limit() -> u32 {
        100
    }
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            port: Self::default_port(),
            host: Self::default_host(),
            token_ttl_secs: Self::default_token_ttl(),
            rate_limit_per_min: Self::default_rate_limit(),
            cors_enabled: false,
            cors_origins: vec![],
            clients: vec![],
        }
    }
}

impl ApiConfig {
    /// Valide la configuration au demarrage (fail-closed) : hote, CORS, registre de clients.
    pub fn validate(&self) -> Result<(), ApiError> {
        self.host.parse::<std::net::IpAddr>().map_err(|e| {
            ApiError::InternalError(format!("config api.host invalide '{}': {e}", self.host))
        })?;
        if self.cors_enabled {
            if self.cors_origins.is_empty() {
                return Err(ApiError::InternalError(
                    "config api: cors_enabled=true exige une liste cors_origins explicite".into(),
                ));
            }
            for o in &self.cors_origins {
                if o.trim() == "*" || o.parse::<axum::http::HeaderValue>().is_err() {
                    return Err(ApiError::InternalError(format!(
                        "config api: origine CORS invalide ou joker interdit : '{o}'"
                    )));
                }
            }
        }
        // Secret partage client_id/secret : desactive par defaut (lot 2, #23).
        #[cfg(not(feature = "jeton-secret-client"))]
        if !self.clients.is_empty() {
            return Err(ApiError::InternalError(
                "config api: api.clients exige la feature `jeton-secret-client` \
                 (authentification par secret partage desactivee, lot 2)"
                    .into(),
            ));
        }
        for c in &self.clients {
            if c.client_id.is_empty() {
                return Err(ApiError::InternalError("config api: client_id vide".into()));
            }
            #[cfg(feature = "jeton-secret-client")]
            argon2::password_hash::PasswordHash::new(&c.secret_hash).map_err(|e| {
                ApiError::InternalError(format!(
                    "config api: secret_hash invalide pour '{}': {e}",
                    c.client_id
                ))
            })?;
            if c.all_personas && !c.scopes.contains(&JwtScope::Admin) {
                return Err(ApiError::InternalError(format!(
                    "config api: all_personas=true exige le scope admin pour '{}'",
                    c.client_id
                )));
            }
            for p in &c.personas {
                if p.trim().is_empty() || p.contains('*') {
                    return Err(ApiError::InternalError(format!(
                        "config api: persona invalide ou joker interdit pour '{}': '{p}'",
                        c.client_id
                    )));
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_api_config() {
        let cfg = ApiConfig::default();
        assert_eq!(cfg.port, 7890);
        assert_eq!(cfg.host, "127.0.0.1");
        assert!(cfg.clients.is_empty());
        assert_eq!(cfg.token_ttl_secs, 3600);
        assert_eq!(cfg.rate_limit_per_min, 100);
        assert!(!cfg.cors_enabled);
        assert_eq!(cfg.cors_origins, [] as [std::string::String; 0]);
    }

    #[test]
    fn test_api_config_serde_roundtrip() {
        let cfg = ApiConfig::default();
        let json = serde_json::to_string(&cfg).expect("serialize");
        let back: ApiConfig = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back.port, cfg.port);
        assert_eq!(back.token_ttl_secs, cfg.token_ttl_secs);
    }

    #[test]
    fn test_validate_default_ok() {
        assert!(ApiConfig::default().validate().is_ok());
    }

    #[test]
    fn test_validate_invalid_host_fails() {
        let cfg = ApiConfig {
            host: "pas-une-ip".into(),
            ..Default::default()
        };
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn test_validate_cors_rules() {
        let mut cfg = ApiConfig {
            cors_enabled: true,
            ..Default::default()
        };
        assert!(cfg.validate().is_err(), "liste vide refusee");
        cfg.cors_origins = vec!["*".into()];
        assert!(cfg.validate().is_err(), "joker refuse");
        cfg.cors_origins = vec!["https://alicia.example".into()];
        assert!(cfg.validate().is_ok());
    }

    #[test]
    fn test_validate_bad_secret_hash_fails() {
        let cfg = ApiConfig {
            clients: vec![ApiClientConfig {
                client_id: "c".into(),
                secret_hash: "plaintext".into(),
                scopes: vec![JwtScope::Read],
                personas: vec![],
                all_personas: false,
            }],
            ..Default::default()
        };
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn test_validate_persona_wildcard_or_empty_fails() {
        // Hash PHC syntaxiquement valide (la verification n'est pas en jeu ici).
        let hash = "$argon2id$v=19$m=19456,t=2,p=1$ZHVtbXlzYWx0ZHVtbXk$Kq3b3m3mQ4h7m1o0cYk3x5v0o4d8Qm0pJm3wq0ZkZ9o";
        for bad in ["*", "", "  "] {
            let cfg = ApiConfig {
                clients: vec![ApiClientConfig {
                    client_id: "c".into(),
                    secret_hash: hash.into(),
                    scopes: vec![JwtScope::Write],
                    personas: vec![bad.into()],
                    all_personas: false,
                }],
                ..Default::default()
            };
            assert!(cfg.validate().is_err(), "persona '{bad}' doit etre refusee");
        }
    }

    fn client(scopes: Vec<JwtScope>, all_personas: bool) -> ApiClientConfig {
        ApiClientConfig {
            client_id: "c".into(),
            secret_hash: "$argon2id$v=19$m=19456,t=2,p=1$ZHVtbXlzYWx0ZHVtbXk$Kq3b3m3mQ4h7m1o0cYk3x5v0o4d8Qm0pJm3wq0ZkZ9o".into(),
            scopes,
            personas: vec![],
            all_personas,
        }
    }

    #[cfg(not(feature = "jeton-secret-client"))]
    #[test]
    fn test_clients_refused_without_feature() {
        let cfg = ApiConfig {
            clients: vec![client(vec![JwtScope::Admin], false)],
            ..Default::default()
        };
        assert!(cfg.validate().is_err(), "secret partage desactive par defaut");
    }

    #[cfg(feature = "jeton-secret-client")]
    #[test]
    fn test_validate_all_personas_requires_admin_scope() {
        for scopes in [vec![], vec![JwtScope::Read, JwtScope::Write, JwtScope::History]] {
            let cfg = ApiConfig {
                clients: vec![client(scopes, true)],
                ..Default::default()
            };
            assert!(cfg.validate().is_err(), "all_personas sans admin doit etre refuse");
        }
        let cfg = ApiConfig {
            clients: vec![client(vec![JwtScope::Admin], true)],
            ..Default::default()
        };
        assert!(cfg.validate().is_ok());
    }

    #[test]
    fn test_all_personas_defaults_false_in_toml_shape() {
        let c: ApiClientConfig = serde_json::from_value(serde_json::json!({
            "client_id": "c", "secret_hash": "x", "scopes": ["admin"]
        }))
        .expect("deserialize");
        assert!(!c.all_personas);
    }
}
