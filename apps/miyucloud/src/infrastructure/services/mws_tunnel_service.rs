//! MWS Tunnel Service — wraps `miyuwebway_participant::MwsService` for
//! registering Miyukini Cloud as a COG on the Webway network.
//!
//! Inbound HTTP traffic arrives via DATA frames and is forwarded to the
//! local Axum server through `home_http_bind`.

use crate::application::ports::web_connection_ports::ConnectionStatus;
use crate::common::config::MwsTunnelConfig;
use crate::common::errors::DomainError;
use chrono::Utc;
use miyuwebway_participant::{
    CogIdentity, MwsService, MwsServiceConfig, MwsServiceState,
    RelayClientConfig, TrackerClientConfig,
};
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct MwsTunnelService {
    config: MwsTunnelConfig,
    local_bind: String,
    mws: Arc<RwLock<Option<MwsService>>>,
    state: Arc<RwLock<ConnectionStatus>>,
}

impl MwsTunnelService {
    pub fn new(config: MwsTunnelConfig, local_bind: String) -> Self {
        Self {
            config,
            local_bind,
            mws: Arc::new(RwLock::new(None)),
            state: Arc::new(RwLock::new(ConnectionStatus::disabled("mws_tunnel"))),
        }
    }

    /// Connect to the MWS network (Relay + Tracker).
    pub async fn connect(&self) -> Result<(), DomainError> {
        if !self.config.enabled {
            return Ok(());
        }

        tracing::info!(
            "Connecting to MWS network (relay={}, tracker={}, cog_id={})",
            self.config.relay_address,
            self.config.tracker_address,
            self.config.cog_id
        );

        // Update state
        {
            let mut s = self.state.write().await;
            s.state = "connecting".into();
            s.active = false;
        }

        // Build MwsServiceConfig from our config
        let relay_host = self
            .config
            .relay_address
            .split(':')
            .next()
            .unwrap_or("localhost")
            .to_string();

        let mws_config = MwsServiceConfig {
            relay: RelayClientConfig {
                relay_address: self.config.relay_address.clone(),
                tls_domain: relay_host,
                home_http_bind: Some(self.local_bind.clone()),
                ..Default::default()
            },
            tracker: TrackerClientConfig {
                tracker_address: self.config.tracker_address.clone(),
                ..Default::default()
            },
            auto_heartbeat: true,
            auto_reconnect: self.config.auto_reconnect,
            reconnect_delay: self.config.reconnect_delay_secs,
            max_reconnect_attempts: self.config.max_reconnect_attempts,
            subdomain_slug: self.config.subdomain_slug.clone(),
            home_http_bind: Some(self.local_bind.clone()),
        };

        let service = MwsService::new(mws_config);

        let identity = CogIdentity {
            cog_id: self.config.cog_id.clone(),
            core_version: env!("CARGO_PKG_VERSION").to_string(),
            public_address: self.local_bind.clone(),
            services: vec!["miyucloud".to_string()],
        };

        // Start the MWS service (connect relay + announce tracker + heartbeat)
        service.start(identity).await.map_err(|e| {
            DomainError::internal_error("MWS", format!("Connection failed: {}", e))
        })?;

        // Build public URL
        let public_url = self
            .config
            .subdomain_slug
            .as_ref()
            .map(|slug| format!("https://{}.miyukini.com", slug))
            .unwrap_or_else(|| {
                format!("https://miyukini.com/cog/{}", self.config.cog_id)
            });

        // Store service and update state
        *self.mws.write().await = Some(service);
        *self.state.write().await = ConnectionStatus {
            mode: "mws_tunnel".into(),
            active: true,
            public_url: Some(public_url),
            state: "online".into(),
            last_update: Some(Utc::now()),
            error: None,
        };

        tracing::info!("MWS tunnel connected — COG {} is online", self.config.cog_id);
        Ok(())
    }

    /// Disconnect from the MWS network.
    pub async fn disconnect(&self) -> Result<(), DomainError> {
        let mut guard = self.mws.write().await;
        if let Some(ref service) = *guard {
            service.stop().await.map_err(|e| {
                DomainError::internal_error("MWS", format!("Disconnect failed: {}", e))
            })?;
        }
        *guard = None;

        *self.state.write().await = ConnectionStatus {
            mode: "mws_tunnel".into(),
            active: false,
            public_url: None,
            state: "stopped".into(),
            last_update: Some(Utc::now()),
            error: None,
        };

        tracing::info!("MWS tunnel disconnected");
        Ok(())
    }

    /// Get current tunnel status.
    pub async fn status(&self) -> ConnectionStatus {
        // Sync state from MwsService if available
        let guard = self.mws.read().await;
        if let Some(ref service) = *guard {
            let mws_state = service.get_state().await;
            let mut status = self.state.read().await.clone();
            status.state = match mws_state {
                MwsServiceState::Stopped => "stopped".into(),
                MwsServiceState::ConnectingRelay => "connecting_relay".into(),
                MwsServiceState::ConnectingTracker => "connecting_tracker".into(),
                MwsServiceState::Online => "online".into(),
                MwsServiceState::Offline => "offline".into(),
                MwsServiceState::Reconnecting => "reconnecting".into(),
            };
            status.active = mws_state == MwsServiceState::Online;
            status
        } else {
            self.state.read().await.clone()
        }
    }

    /// Check if the tunnel is currently online.
    pub async fn is_online(&self) -> bool {
        let guard = self.mws.read().await;
        match *guard {
            Some(ref service) => service.is_online().await,
            None => false,
        }
    }
}
