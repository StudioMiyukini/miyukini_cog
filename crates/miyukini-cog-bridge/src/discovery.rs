//! Découverte automatique du COG Host sur le réseau local.
//!
//! Utilise un broadcast UDP simple pour détecter le Central Desktop
//! sur le même réseau WiFi. Le COG Host répond avec son IP et port.

use crate::{BridgeError, BridgeResult};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::time::Duration;

/// Port de découverte (broadcast UDP).
pub const DISCOVERY_PORT: u16 = 19847;

/// Message de découverte envoyé par le mobile.
const DISCOVERY_MAGIC: &[u8] = b"MIYUKINI_COG_DISCOVER_V1";

/// Réponse du COG Host à la découverte.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryResponse {
    /// Nom du COG Host (ex: "PC-Bureau-Miyukini").
    pub host_name: String,
    /// Port du CentralRemote WebSocket.
    pub remote_port: u16,
    /// Port HTTP pour l'API bridge.
    pub bridge_port: u16,
    /// Version de Central Desktop.
    pub version: String,
}

/// Cherche un COG Host sur le réseau local via broadcast UDP.
///
/// Retourne l'adresse et les infos du premier host trouvé,
/// ou `HostNotFound` après le timeout.
pub async fn discover_lan(timeout: Duration) -> BridgeResult<(SocketAddr, DiscoveryResponse)> {
    tracing::info!("Découverte LAN: broadcast UDP sur port {DISCOVERY_PORT}");

    let socket = tokio::net::UdpSocket::bind("0.0.0.0:0")
        .await
        .map_err(|e| BridgeError::NetworkError(format!("UDP bind: {e}")))?;

    socket
        .set_broadcast(true)
        .map_err(|e| BridgeError::NetworkError(format!("UDP broadcast: {e}")))?;

    // Envoyer le magic sur le broadcast
    let broadcast_addr: SocketAddr = format!("255.255.255.255:{DISCOVERY_PORT}").parse().unwrap();
    socket
        .send_to(DISCOVERY_MAGIC, broadcast_addr)
        .await
        .map_err(|e| BridgeError::NetworkError(format!("UDP send: {e}")))?;

    // Attendre une réponse
    let mut buf = [0u8; 1024];
    match tokio::time::timeout(timeout, socket.recv_from(&mut buf)).await {
        Ok(Ok((len, addr))) => {
            let response: DiscoveryResponse = serde_json::from_slice(&buf[..len])
                .map_err(|e| BridgeError::NetworkError(format!("Parse discovery: {e}")))?;
            tracing::info!("COG Host trouvé: {} à {addr}", response.host_name);
            Ok((addr, response))
        }
        Ok(Err(e)) => Err(BridgeError::NetworkError(format!("UDP recv: {e}"))),
        Err(_) => {
            tracing::info!("Découverte LAN: timeout, aucun COG Host trouvé");
            Err(BridgeError::HostNotFound)
        }
    }
}
