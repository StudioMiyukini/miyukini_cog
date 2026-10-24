//! Serveur API REST Alicia.
//!
//! Lance le serveur axum sur le port configure.

// @id service.alicia.api.server
// @role axum_server
// @layer 7
// @do start_alicia_api_server
// @human Serveur axum Alicia API ; demarrage, bind, shutdown propre.

use std::net::SocketAddr;
use std::sync::Arc;

use miyualicia::AliciaService;

use crate::config::ApiConfig;
use crate::errors::ApiError;
use crate::router::build_router;

/// Serveur API REST Alicia.
///
/// Destine a etre cree une seule fois et lance via `tokio::spawn`.
pub struct AliciaApiServer {
    config: ApiConfig,
    alicia: Arc<AliciaService>,
    secret: Vec<u8>,
}

impl AliciaApiServer {
    /// Cree un nouveau serveur API.
    pub fn new(alicia: Arc<AliciaService>, config: ApiConfig, secret: Vec<u8>) -> Self {
        Self {
            config,
            alicia,
            secret,
        }
    }

    /// Demarre le serveur et bloque jusqu'a arret.
    ///
    /// Retourne `ApiError::InternalError` si le port est deja pris.
    pub async fn start(self) -> Result<(), ApiError> {
        self.config.validate()?;
        let addr = self.listen_addr()?;
        let router = build_router(self.alicia, self.config, self.secret);

        tracing::info!("API Alicia demarree sur http://{addr}");

        let listener = tokio::net::TcpListener::bind(addr)
            .await
            .map_err(|e| ApiError::InternalError(format!("impossible de bind {addr}: {e}")))?;

        // ConnectInfo requis par le limiteur de debit par IP.
        axum::serve(
            listener,
            router.into_make_service_with_connect_info::<SocketAddr>(),
        )
            .with_graceful_shutdown(shutdown_signal())
            .await
            .map_err(|e| ApiError::InternalError(format!("erreur serveur : {e}")))?;

        tracing::info!("API Alicia arretee proprement");
        Ok(())
    }

    /// Retourne l'adresse d'ecoute effective.
    ///
    /// Fail-closed : un hote invalide est une erreur (plus de repli sur 0.0.0.0).
    pub fn listen_addr(&self) -> Result<SocketAddr, ApiError> {
        let host: std::net::IpAddr = self.config.host.parse().map_err(|e| {
            ApiError::InternalError(format!("host invalide '{}': {e}", self.config.host))
        })?;
        Ok(SocketAddr::new(host, self.config.port))
    }
}

/// Signal d'arret propre du serveur (Ctrl+C).
async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
    tracing::info!("Signal d'arret recu, fermeture de l'API Alicia...");
}

#[cfg(test)]
mod tests {
    use super::*;
    use miyualicia::AliciaConfig;

    #[test]
    fn test_server_listen_addr() {
        let config = AliciaConfig {
            mqtt: None,
            ..Default::default()
        };
        let alicia = Arc::new(AliciaService::new(config));
        let api_config = ApiConfig {
            port: 9999,
            host: "127.0.0.1".to_string(),
            ..Default::default()
        };
        let server = AliciaApiServer::new(alicia, api_config, b"secret".to_vec());
        let addr = server.listen_addr().unwrap();
        assert_eq!(addr.port(), 9999);
        assert_eq!(
            addr.ip(),
            std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1))
        );
    }

    #[test]
    fn test_server_default_addr() {
        let config = AliciaConfig {
            mqtt: None,
            ..Default::default()
        };
        let alicia = Arc::new(AliciaService::new(config));
        let server = AliciaApiServer::new(alicia, ApiConfig::default(), b"secret".to_vec());
        let addr = server.listen_addr().unwrap();
        assert_eq!(addr.port(), 7890);
        assert!(addr.ip().is_loopback(), "bind par defaut = loopback");
    }

    #[test]
    fn test_server_invalid_host_is_error() {
        let config = AliciaConfig {
            mqtt: None,
            ..Default::default()
        };
        let alicia = Arc::new(AliciaService::new(config));
        let api_config = ApiConfig {
            host: "nimporte quoi".to_string(),
            ..Default::default()
        };
        let server = AliciaApiServer::new(alicia, api_config, b"secret".to_vec());
        assert!(server.listen_addr().is_err());
    }
}
