//! Mode Whitelist — connexion directe HTTP en LAN.
//!
//! Le COG Host expose un endpoint de validation sur le réseau local.
//! Le device Android, une fois autorisé (whitelisté), se connecte directement
//! sans tunnel — latence minimale, idéal pour le WiFi maison.

use crate::{BridgeError, BridgeResult};

/// Établit une connexion directe au COG Host en LAN via whitelist.
pub async fn connect(host: &str, port: u16, device_id: &str) -> BridgeResult<()> {
    tracing::info!("Whitelist: connexion directe à {host}:{port} (device={device_id})");

    let url = format!("http://{host}:{port}/api/bridge/validate");

    let client = reqwest::Client::new();
    let resp = client
        .post(&url)
        .json(&serde_json::json!({ "device_id": device_id }))
        .timeout(std::time::Duration::from_secs(5))
        .send()
        .await
        .map_err(|e| BridgeError::NetworkError(format!("LAN: {e}")))?;

    if resp.status().is_success() {
        tracing::info!("Whitelist: device autorisé");
        Ok(())
    } else if resp.status().as_u16() == 403 {
        Err(BridgeError::DeviceNotWhitelisted)
    } else {
        Err(BridgeError::ConnectionRefused)
    }
}
