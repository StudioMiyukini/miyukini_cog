//! Web connection management ports — MWS tunnel, DDNS, and Central client.

use crate::common::errors::DomainError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Status of a single connection mode.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionStatus {
    pub mode: String,
    pub active: bool,
    pub public_url: Option<String>,
    pub state: String,
    pub last_update: Option<DateTime<Utc>>,
    pub error: Option<String>,
}

impl ConnectionStatus {
    pub fn disabled(mode: &str) -> Self {
        Self {
            mode: mode.to_string(),
            active: false,
            public_url: None,
            state: "disabled".into(),
            last_update: None,
            error: None,
        }
    }
}

/// Combined status of all connection modes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebConnectionStatus {
    pub mws: Option<ConnectionStatus>,
    pub ddns: Option<ConnectionStatus>,
    pub central: Option<ConnectionStatus>,
    pub primary_url: Option<String>,
}

/// Port for the connection manager (orchestrates tunnel + DDNS + Central).
pub trait ConnectionManagerPort: Send + Sync + 'static {
    fn start(&self) -> impl std::future::Future<Output = Result<(), DomainError>> + Send;
    fn stop(&self) -> impl std::future::Future<Output = Result<(), DomainError>> + Send;
    fn status(&self) -> impl std::future::Future<Output = Result<WebConnectionStatus, DomainError>> + Send;
}
