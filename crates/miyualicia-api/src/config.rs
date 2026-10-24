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

    /// Registre des clients autorises pour `/auth/token`. Vide (defaut) : la route refuse
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
        for c in &self.clients {
            if c.client_id.is_empty() {
                return Err(ApiError::InternalError("config api: client_id vide".into()));
            }
            argon2::password_hash::PasswordHash::new(&c.secret_hash).map_err(|e| {
                ApiError::InternalError(format!(
                    "config api: secret_hash invalide pour '{}': {e}",
                    c.client_id
                ))
            })?;
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
        assert!(cfg.cors_origins.is_empty());
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
            }],
            ..Default::default()
        };
        assert!(cfg.validate().is_err());
    }
}
