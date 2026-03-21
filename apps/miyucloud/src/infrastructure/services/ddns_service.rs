//! Dynamic DNS client — supports No-IP, DuckDNS, and generic update protocols.
//!
//! Spawns a background Tokio task that periodically detects the public IP
//! and updates the DNS record when it changes.

use crate::application::ports::web_connection_ports::ConnectionStatus;
use crate::common::config::{DdnsConfig, DdnsProvider};
use crate::common::errors::DomainError;
use chrono::Utc;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{Duration, interval};

pub struct DdnsService {
    config: DdnsConfig,
    http_client: reqwest::Client,
    current_ip: Arc<RwLock<Option<String>>>,
    state: Arc<RwLock<ConnectionStatus>>,
    shutdown: Arc<RwLock<bool>>,
}

impl DdnsService {
    pub fn new(config: DdnsConfig) -> Self {
        Self {
            http_client: reqwest::Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .expect("Failed to build HTTP client"),
            config,
            current_ip: Arc::new(RwLock::new(None)),
            state: Arc::new(RwLock::new(ConnectionStatus::disabled("ddns"))),
            shutdown: Arc::new(RwLock::new(false)),
        }
    }

    /// Start the periodic DDNS update background task.
    pub async fn start(&self) -> Result<(), DomainError> {
        if !self.config.enabled {
            return Ok(());
        }

        tracing::info!(
            "Starting DDNS client (provider={:?}, hostname={})",
            self.config.provider,
            self.config.hostname
        );

        // Initial update
        if let Err(e) = self.update_once().await {
            tracing::warn!("Initial DDNS update failed: {}", e);
        }

        // Spawn periodic task
        let config = self.config.clone();
        let http_client = self.http_client.clone();
        let current_ip = self.current_ip.clone();
        let state = self.state.clone();
        let shutdown = self.shutdown.clone();

        tokio::spawn(async move {
            let mut timer = interval(Duration::from_secs(config.update_interval_secs));

            loop {
                timer.tick().await;

                if *shutdown.read().await {
                    tracing::debug!("DDNS task shutting down");
                    break;
                }

                match Self::detect_ip_static(&http_client, &config.detect_ip_url).await {
                    Ok(new_ip) => {
                        let old_ip = current_ip.read().await.clone();
                        if old_ip.as_deref() != Some(&new_ip) {
                            tracing::info!("Public IP changed: {:?} -> {}", old_ip, new_ip);
                            match Self::update_dns_static(
                                &http_client,
                                &config,
                                &new_ip,
                            )
                            .await
                            {
                                Ok(()) => {
                                    *current_ip.write().await = Some(new_ip.clone());
                                    let url = format!("http://{}", config.hostname);
                                    *state.write().await = ConnectionStatus {
                                        mode: "ddns".into(),
                                        active: true,
                                        public_url: Some(url),
                                        state: format!("online ({})", new_ip),
                                        last_update: Some(Utc::now()),
                                        error: None,
                                    };
                                    tracing::info!("DDNS updated: {} -> {}", config.hostname, new_ip);
                                }
                                Err(e) => {
                                    tracing::error!("DDNS update failed: {}", e);
                                    let mut s = state.write().await;
                                    s.error = Some(e.to_string());
                                    s.state = "error".into();
                                }
                            }
                        }
                    }
                    Err(e) => {
                        tracing::warn!("Failed to detect public IP: {}", e);
                    }
                }
            }
        });

        Ok(())
    }

    /// Stop the DDNS background task.
    pub async fn stop(&self) {
        *self.shutdown.write().await = true;
    }

    /// Force an immediate DDNS update.
    pub async fn force_update(&self) -> Result<String, DomainError> {
        self.update_once().await
    }

    /// Get current DDNS status.
    pub async fn status(&self) -> ConnectionStatus {
        self.state.read().await.clone()
    }

    async fn update_once(&self) -> Result<String, DomainError> {
        let ip = Self::detect_ip_static(&self.http_client, &self.config.detect_ip_url).await?;
        Self::update_dns_static(&self.http_client, &self.config, &ip).await?;
        *self.current_ip.write().await = Some(ip.clone());
        let url = format!("http://{}", self.config.hostname);
        *self.state.write().await = ConnectionStatus {
            mode: "ddns".into(),
            active: true,
            public_url: Some(url),
            state: format!("online ({})", ip),
            last_update: Some(Utc::now()),
            error: None,
        };
        Ok(ip)
    }

    async fn detect_ip_static(
        client: &reqwest::Client,
        url: &str,
    ) -> Result<String, DomainError> {
        let resp = client
            .get(url)
            .send()
            .await
            .map_err(|e| DomainError::internal_error("DDNS", format!("IP detect failed: {}", e)))?;
        let ip = resp
            .text()
            .await
            .map_err(|e| DomainError::internal_error("DDNS", format!("IP parse failed: {}", e)))?;
        Ok(ip.trim().to_string())
    }

    async fn update_dns_static(
        client: &reqwest::Client,
        config: &DdnsConfig,
        ip: &str,
    ) -> Result<(), DomainError> {
        let resp = match config.provider {
            DdnsProvider::NoIp => {
                let username = config.username.as_deref().unwrap_or("");
                client
                    .get(format!(
                        "https://dynupdate.no-ip.com/nic/update?hostname={}&myip={}",
                        config.hostname, ip
                    ))
                    .basic_auth(username, Some(&config.token))
                    .send()
                    .await
            }
            DdnsProvider::DuckDns => {
                client
                    .get(format!(
                        "https://www.duckdns.org/update?domains={}&token={}&ip={}",
                        config.hostname, config.token, ip
                    ))
                    .send()
                    .await
            }
            DdnsProvider::Generic => {
                let url = config
                    .update_url
                    .as_deref()
                    .unwrap_or("")
                    .replace("{ip}", ip)
                    .replace("{hostname}", &config.hostname);
                client.get(&url).send().await
            }
        };

        let resp = resp.map_err(|e| {
            DomainError::internal_error("DDNS", format!("Update request failed: {}", e))
        })?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(DomainError::internal_error(
                "DDNS",
                format!("Update failed ({}): {}", status, body),
            ));
        }

        Ok(())
    }
}
