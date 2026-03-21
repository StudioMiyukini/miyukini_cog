//! Connection Manager — orchestrates MWS tunnel, DDNS, and Central client.

use crate::application::ports::web_connection_ports::WebConnectionStatus;
use crate::common::errors::DomainError;
use crate::infrastructure::services::central_client_service::CentralClientService;
use crate::infrastructure::services::ddns_service::DdnsService;
use crate::infrastructure::services::mws_tunnel_service::MwsTunnelService;
use std::sync::Arc;

pub struct ConnectionManagerService {
    pub mws: Option<Arc<MwsTunnelService>>,
    pub ddns: Option<Arc<DdnsService>>,
    pub central: Option<Arc<CentralClientService>>,
}

impl ConnectionManagerService {
    pub fn new(
        mws: Option<Arc<MwsTunnelService>>,
        ddns: Option<Arc<DdnsService>>,
        central: Option<Arc<CentralClientService>>,
    ) -> Self {
        Self { mws, ddns, central }
    }

    /// Start all enabled connection modes as background tasks.
    pub async fn start_all(&self) -> Result<(), DomainError> {
        // Start MWS tunnel
        if let Some(ref mws) = self.mws {
            if let Err(e) = mws.connect().await {
                tracing::error!("MWS tunnel failed to connect: {}", e);
                // Non-fatal — continue with other modes
            }
        }

        // Start DDNS updates
        if let Some(ref ddns) = self.ddns {
            if let Err(e) = ddns.start().await {
                tracing::error!("DDNS service failed to start: {}", e);
            }
        }

        // Register with Central and start health reporting
        if let Some(ref central) = self.central {
            let status = self.status().await?;
            central.register_service(&status).await?;
            central.start_reporting().await?;
        }

        Ok(())
    }

    /// Stop all connection modes.
    pub async fn stop_all(&self) -> Result<(), DomainError> {
        if let Some(ref mws) = self.mws {
            mws.disconnect().await.ok();
        }
        if let Some(ref ddns) = self.ddns {
            ddns.stop().await;
        }
        if let Some(ref central) = self.central {
            central.stop().await;
        }
        Ok(())
    }

    /// Get combined status of all connection modes.
    pub async fn status(&self) -> Result<WebConnectionStatus, DomainError> {
        let mws_status = match &self.mws {
            Some(mws) => Some(mws.status().await),
            None => None,
        };

        let ddns_status = match &self.ddns {
            Some(ddns) => Some(ddns.status().await),
            None => None,
        };

        let central_status = match &self.central {
            Some(central) => Some(central.status().await),
            None => None,
        };

        // Determine primary URL: prefer MWS tunnel URL, fall back to DDNS
        let primary_url = mws_status
            .as_ref()
            .filter(|s| s.active)
            .and_then(|s| s.public_url.clone())
            .or_else(|| {
                ddns_status
                    .as_ref()
                    .filter(|s| s.active)
                    .and_then(|s| s.public_url.clone())
            });

        Ok(WebConnectionStatus {
            mws: mws_status,
            ddns: ddns_status,
            central: central_status,
            primary_url,
        })
    }
}
