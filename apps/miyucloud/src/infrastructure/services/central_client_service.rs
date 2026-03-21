//! Central (Miyukini COG) client — registers miyucloud as a managed service
//! and periodically reports health status.

use crate::application::ports::web_connection_ports::{ConnectionStatus, WebConnectionStatus};
use crate::common::config::CentralClientConfig;
use crate::common::errors::DomainError;
use chrono::Utc;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{Duration, interval};

pub struct CentralClientService {
    config: CentralClientConfig,
    http_client: reqwest::Client,
    state: Arc<RwLock<ConnectionStatus>>,
    shutdown: Arc<RwLock<bool>>,
}

impl CentralClientService {
    pub fn new(config: CentralClientConfig) -> Self {
        Self {
            http_client: reqwest::Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .expect("Failed to build HTTP client"),
            config,
            state: Arc::new(RwLock::new(ConnectionStatus::disabled("central"))),
            shutdown: Arc::new(RwLock::new(false)),
        }
    }

    /// Register this service with Central.
    pub async fn register_service(
        &self,
        web_status: &WebConnectionStatus,
    ) -> Result<(), DomainError> {
        if !self.config.enabled {
            return Ok(());
        }

        let url = format!(
            "{}/api/services/register",
            self.config.central_url.trim_end_matches('/')
        );

        let body = serde_json::json!({
            "service_id": self.config.service_id,
            "service_type": "cloud_storage",
            "version": env!("CARGO_PKG_VERSION"),
            "status": "online",
            "urls": {
                "mws": web_status.mws.as_ref().and_then(|s| s.public_url.clone()),
                "ddns": web_status.ddns.as_ref().and_then(|s| s.public_url.clone()),
                "local": format!("http://127.0.0.1:{}", std::env::var("OXICLOUD_SERVER_PORT").unwrap_or_else(|_| "8086".into())),
            },
            "capabilities": ["webdav", "caldav", "carddav", "wopi", "rest_api"],
        });

        tracing::info!("Registering with Central at {}", url);

        match self
            .http_client
            .post(&url)
            .json(&body)
            .send()
            .await
        {
            Ok(resp) if resp.status().is_success() => {
                *self.state.write().await = ConnectionStatus {
                    mode: "central".into(),
                    active: true,
                    public_url: Some(self.config.central_url.clone()),
                    state: "registered".into(),
                    last_update: Some(Utc::now()),
                    error: None,
                };
                tracing::info!("Registered with Central as '{}'", self.config.service_id);
                Ok(())
            }
            Ok(resp) => {
                let status = resp.status();
                let body = resp.text().await.unwrap_or_default();
                tracing::warn!("Central registration failed ({}): {}", status, body);
                // Non-fatal — Central may not be running yet
                let mut s = self.state.write().await;
                s.state = format!("registration_failed ({})", status);
                s.error = Some(body);
                Ok(())
            }
            Err(e) => {
                tracing::warn!("Cannot reach Central at {}: {}", self.config.central_url, e);
                let mut s = self.state.write().await;
                s.state = "unreachable".into();
                s.error = Some(e.to_string());
                Ok(()) // Non-fatal
            }
        }
    }

    /// Start periodic health reporting to Central.
    pub async fn start_reporting(&self) -> Result<(), DomainError> {
        if !self.config.enabled {
            return Ok(());
        }

        let config = self.config.clone();
        let client = self.http_client.clone();
        let state = self.state.clone();
        let shutdown = self.shutdown.clone();

        tokio::spawn(async move {
            let mut timer = interval(Duration::from_secs(config.report_interval_secs));

            loop {
                timer.tick().await;

                if *shutdown.read().await {
                    break;
                }

                let url = format!(
                    "{}/api/services/{}/health",
                    config.central_url.trim_end_matches('/'),
                    config.service_id
                );

                let body = serde_json::json!({
                    "status": "online",
                    "timestamp": Utc::now().to_rfc3339(),
                    "version": env!("CARGO_PKG_VERSION"),
                });

                match client.post(&url).json(&body).send().await {
                    Ok(resp) if resp.status().is_success() => {
                        let mut s = state.write().await;
                        s.active = true;
                        s.state = "reporting".into();
                        s.last_update = Some(Utc::now());
                        s.error = None;
                    }
                    Ok(resp) => {
                        tracing::debug!("Central health report: {}", resp.status());
                    }
                    Err(e) => {
                        tracing::debug!("Central health report failed: {}", e);
                        let mut s = state.write().await;
                        s.state = "unreachable".into();
                        s.error = Some(e.to_string());
                    }
                }
            }
        });

        Ok(())
    }

    /// Stop the reporting task.
    pub async fn stop(&self) {
        *self.shutdown.write().await = true;
    }

    /// Get current status.
    pub async fn status(&self) -> ConnectionStatus {
        self.state.read().await.clone()
    }
}
