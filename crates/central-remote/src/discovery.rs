//! Listener UDP pour la découverte LAN par les clients mobiles.
//!
//! Écoute le broadcast UDP sur le port `DISCOVERY_PORT` et répond
//! avec les infos du COG Host (nom, ports, version).
//!
//! Protocole :
//! - Client envoie `MIYUKINI_COG_DISCOVER_V1` en UDP broadcast port 19847
//! - Serveur répond avec `DiscoveryInfo` (JSON) à l'adresse du client

use crate::pairing::{DiscoveryInfo, PairingManager};
use std::sync::Arc;
use tokio::net::UdpSocket;
use tracing::{info, warn};

/// Port de découverte (identique côté mobile dans miyukini-cog-bridge::discovery).
pub const DISCOVERY_PORT: u16 = 19847;

/// Magic bytes de la requête de découverte.
const DISCOVERY_MAGIC: &[u8] = b"MIYUKINI_COG_DISCOVER_V1";

/// Lance le listener UDP de découverte en background.
/// Retourne un handle pour l'arrêter.
pub async fn start_discovery_listener(
    pairing: Arc<PairingManager>,
) -> Result<DiscoveryHandle, String> {
    let addr = format!("0.0.0.0:{DISCOVERY_PORT}");
    let socket = UdpSocket::bind(&addr)
        .await
        .map_err(|e| format!("UDP discovery bind {addr}: {e}"))?;

    info!("UDP discovery listener actif sur {addr}");

    let (shutdown_tx, mut shutdown_rx) = tokio::sync::oneshot::channel::<()>();

    let task = tokio::spawn(async move {
        let mut buf = [0u8; 256];
        loop {
            tokio::select! {
                _ = &mut shutdown_rx => {
                    info!("UDP discovery listener arrêté");
                    break;
                }
                result = socket.recv_from(&mut buf) => {
                    match result {
                        Ok((len, from)) => {
                            let received = &buf[..len];
                            if received == DISCOVERY_MAGIC {
                                info!("Découverte reçue de {from}");
                                let info = pairing.discovery_info();
                                match serde_json::to_vec(&info) {
                                    Ok(response) => {
                                        if let Err(e) = socket.send_to(&response, from).await {
                                            warn!("Erreur envoi réponse discovery: {e}");
                                        }
                                    }
                                    Err(e) => warn!("Sérialisation discovery info: {e}"),
                                }
                            }
                        }
                        Err(e) => {
                            warn!("UDP discovery recv error: {e}");
                        }
                    }
                }
            }
        }
    });

    Ok(DiscoveryHandle {
        shutdown_tx: Some(shutdown_tx),
        _task: task,
    })
}

/// Handle du listener UDP.
pub struct DiscoveryHandle {
    shutdown_tx: Option<tokio::sync::oneshot::Sender<()>>,
    _task: tokio::task::JoinHandle<()>,
}

impl DiscoveryHandle {
    pub fn shutdown(&mut self) {
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }
    }
}

impl Drop for DiscoveryHandle {
    fn drop(&mut self) {
        self.shutdown();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discovery_port_matches_bridge() {
        // Le port doit être identique côté mobile (miyukini-cog-bridge::discovery::DISCOVERY_PORT)
        assert_eq!(DISCOVERY_PORT, 19847);
    }

    #[test]
    fn magic_bytes_match_bridge() {
        // Le magic doit être identique côté mobile
        assert_eq!(DISCOVERY_MAGIC, b"MIYUKINI_COG_DISCOVER_V1");
    }

    #[test]
    fn discovery_info_serializable() {
        let info = DiscoveryInfo {
            host_name: "Test-COG".into(),
            remote_port: 8091,
            bridge_port: 8091,
            version: "0.1.0".into(),
        };
        let json = serde_json::to_string(&info).unwrap();
        assert!(json.contains("Test-COG"));
    }
}
