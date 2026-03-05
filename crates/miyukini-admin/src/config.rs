//! Module de configuration backend de MiyukiniAdmin

use miyukini_kernel::Config;

/// @id: miyukiniadmin_config_backend_config
/// @role: infrastructure
/// @layer: operator
/// @human: Configuration backend de MiyukiniAdmin.
/// @do: represent_backend_config
#[derive(Debug, Clone)]
pub struct BackendConfig {
    /// @id: miyukiniadmin_config_backend_config_port
    /// @role: data
    /// @layer: operator
    /// @human: Port d'écoute du serveur backend.
    /// @do: store_backend_port
    /// @depends: miyukiniadmin_config_backend_config
    pub port: u16,
    /// @id: miyukiniadmin_config_backend_config_host
    /// @role: data
    /// @layer: operator
    /// @human: Adresse d'écoute du serveur backend.
    /// @do: store_backend_host
    /// @depends: miyukiniadmin_config_backend_config
    pub host: String,
    /// @id: miyukiniadmin_config_backend_config_https
    /// @role: data
    /// @layer: operator
    /// @human: Activer le serveur HTTPS (TLS).
    /// @do: store_https_enabled
    /// @depends: miyukiniadmin_config_backend_config
    pub use_https: bool,
    /// @id: miyukiniadmin_config_backend_config_tls_cert
    /// @role: data
    /// @layer: operator
    /// @human: Chemin vers le certificat TLS (PEM).
    /// @do: store_tls_cert_path
    /// @depends: miyukiniadmin_config_backend_config
    pub tls_cert_path: Option<String>,
    /// @id: miyukiniadmin_config_backend_config_tls_key
    /// @role: data
    /// @layer: operator
    /// @human: Chemin vers la clé privée TLS (PEM).
    /// @do: store_tls_key_path
    /// @depends: miyukiniadmin_config_backend_config
    pub tls_key_path: Option<String>,
}

impl BackendConfig {
    /// @id: miyukiniadmin_config_backend_config_from_env
    /// @role: infrastructure
    /// @layer: operator
    /// @human: Charge la configuration depuis les variables d'environnement.
    /// @do: load_config_from_env
    /// @depends: miyukiniadmin_config_backend_config, kernel_config_get
    /// Charge la configuration depuis les variables d'environnement.
    pub fn from_env(config: &dyn Config) -> Self {
        let port = config
            .get("MIYUKINIADMIN_PORT")
            .and_then(|s| s.parse().ok())
            .unwrap_or(8181);
        let host = config
            .get("MIYUKINIADMIN_HOST")
            .map_or_else(|| "127.0.0.1".to_string(), std::string::ToString::to_string);
        let use_https = config
            .get("MIYUKINIADMIN_HTTPS")
            .is_some_and(|s| s.eq_ignore_ascii_case("1") || s.eq_ignore_ascii_case("true"));
        let tls_cert_path = config
            .get("MIYUKINIADMIN_TLS_CERT")
            .map(std::string::ToString::to_string);
        let tls_key_path = config
            .get("MIYUKINIADMIN_TLS_KEY")
            .map(std::string::ToString::to_string);

        Self {
            port,
            host,
            use_https,
            tls_cert_path,
            tls_key_path,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use miyukini_kernel::EnvConfig;

    /// @id: miyukiniadmin_config_test_from_env
    /// @role: test
    /// @layer: operator
    /// @human: Test de chargement de configuration depuis l'environnement.
    /// @do: verify_config_from_env
    /// @depends: miyukiniadmin_config_backend_config_from_env
    #[test]
    fn test_from_env() {
        let env_config = EnvConfig::from_env();
        let config = BackendConfig::from_env(&env_config);
        assert_eq!(config.port, 8181);
    }
}
