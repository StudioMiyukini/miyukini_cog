//! Client Tracker MWS — Communication avec le Tracker Origin.
//!
//! Gère l'annonce, le heartbeat, et la découverte de COGs/lobbys.

use crate::errors::MiyuwebwayParticipantError;
use crate::protocol::{
    AnnounceAckPayload, AnnouncePayload, CogInfo, LobbyInfo, LobbySearchResult, SearchCogsPayload,
    SearchCogsResultPayload, SearchLobbysPayload, SearchLobbysResultPayload, TrackerFrame,
    TrackerHeartbeatPayload, TrackerMessageType,
};
use bytes::BytesMut;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

/// Configuration du client Tracker.
#[derive(Debug, Clone)]
pub struct TrackerClientConfig {
    /// Adresse du Tracker (host:port).
    pub tracker_address: String,
    /// Timeout de connexion en secondes.
    pub connect_timeout: u64,
    /// Intervalle de heartbeat en secondes.
    pub heartbeat_interval: u64,
}

impl Default for TrackerClientConfig {
    fn default() -> Self {
        Self {
            tracker_address: "miyukini.com:21000".to_string(),
            connect_timeout: 30,
            heartbeat_interval: 60,
        }
    }
}

fn normalize_tracker_addresses(trackers: Vec<String>) -> Vec<String> {
    let mut normalized = Vec::new();
    for tracker in trackers {
        let tracker = tracker.trim().to_string();
        if tracker.is_empty() || normalized.iter().any(|existing| existing == &tracker) {
            continue;
        }
        normalized.push(tracker);
    }
    normalized
}

/// État de connexion au Tracker.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrackerState {
    /// Déconnecté.
    Disconnected,
    /// Connecté.
    Connected,
    /// Annoncé sur le réseau.
    Announced,
    /// Erreur.
    Error,
}

/// Informations d'annonce.
#[derive(Debug, Clone)]
pub struct TrackerAnnouncement {
    /// ID du COG.
    pub cog_id: String,
    /// Version des Cores.
    pub core_version: String,
    /// ID du permis Relay.
    pub permis_id: String,
    /// Adresse de connexion.
    pub address: String,
    /// Services exposés.
    pub services: Vec<String>,
    /// Lobbys hébergés.
    pub lobbys: Vec<LobbyInfo>,
    /// Slug de sous-domaine personnalisé (optionnel).
    /// Si fourni, le COG sera accessible via `<slug>.miyukini.com`.
    pub slug: Option<String>,
}

/// Client Tracker MWS (protocole aligné Origin).
pub struct TrackerClient {
    /// Configuration.
    config: Arc<RwLock<TrackerClientConfig>>,
    /// État actuel.
    state: Arc<RwLock<TrackerState>>,
    /// Buffer de lecture.
    read_buffer: Arc<RwLock<BytesMut>>,
    /// Intervalle de heartbeat actuel.
    heartbeat_interval: Arc<RwLock<u32>>,
    /// Liste des trackers officiels fournie par Origin.
    official_trackers: Arc<RwLock<Vec<String>>>,
}

impl TrackerClient {
    /// Crée un nouveau client Tracker.
    pub fn new(config: TrackerClientConfig) -> Self {
        Self {
            config: Arc::new(RwLock::new(config)),
            state: Arc::new(RwLock::new(TrackerState::Disconnected)),
            read_buffer: Arc::new(RwLock::new(BytesMut::with_capacity(8192))),
            heartbeat_interval: Arc::new(RwLock::new(60)),
            official_trackers: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Crée un client avec la configuration par défaut.
    pub fn with_defaults() -> Self {
        Self::new(TrackerClientConfig::default())
    }

    /// Met à jour la liste des trackers officiels fournie par Origin.
    pub async fn update_official_trackers(
        &self,
        trackers: Vec<String>,
    ) -> Result<(), MiyuwebwayParticipantError> {
        let trackers = normalize_tracker_addresses(trackers);
        if trackers.is_empty() {
            return Err(MiyuwebwayParticipantError::ProtocolError(
                "Relay did not provide any official tracker".to_string(),
            ));
        }

        {
            let mut official = self.official_trackers.write().await;
            *official = trackers.clone();
        }

        let mut config = self.config.write().await;
        if !trackers.iter().any(|tracker| tracker == &config.tracker_address) {
            info!(
                "Configured tracker {} is not official, switching to {}",
                config.tracker_address, trackers[0]
            );
            config.tracker_address = trackers[0].clone();
        }

        Ok(())
    }

    async fn tracker_endpoint(&self) -> Result<(String, u64), MiyuwebwayParticipantError> {
        let official_trackers = self.official_trackers.read().await.clone();
        let mut config = self.config.write().await;

        if !official_trackers.is_empty()
            && !official_trackers
                .iter()
                .any(|tracker| tracker == &config.tracker_address)
        {
            if let Some(fallback) = official_trackers.first() {
                warn!(
                    "Rejecting non-official tracker {}, switching to {}",
                    config.tracker_address, fallback
                );
                config.tracker_address = fallback.clone();
            }
        }

        let tracker_address = config.tracker_address.trim().to_string();
        if tracker_address.is_empty() {
            return Err(MiyuwebwayParticipantError::ProtocolError(
                "No tracker configured".to_string(),
            ));
        }

        if !official_trackers.is_empty()
            && !official_trackers
                .iter()
                .any(|tracker| tracker == &tracker_address)
        {
            return Err(MiyuwebwayParticipantError::ProtocolError(format!(
                "Tracker {tracker_address} is not in the official list"
            )));
        }

        Ok((tracker_address, config.connect_timeout))
    }

    async fn connect_stream(&self) -> Result<(TcpStream, String), MiyuwebwayParticipantError> {
        let (tracker_address, connect_timeout) = self.tracker_endpoint().await?;
        let stream = tokio::time::timeout(
            std::time::Duration::from_secs(connect_timeout),
            TcpStream::connect(&tracker_address),
        )
        .await
        .map_err(|_| MiyuwebwayParticipantError::Timeout)?
        .map_err(|e| MiyuwebwayParticipantError::ConnectionFailed(e.to_string()))?;

        Ok((stream, tracker_address))
    }

    /// Annonce le COG sur le réseau.
    pub async fn announce(
        &self,
        announcement: &TrackerAnnouncement,
    ) -> Result<(), MiyuwebwayParticipantError> {
        let (mut stream, tracker_address) = self.connect_stream().await?;
        info!(
            "Announcing COG {} on Tracker at {}",
            announcement.cog_id, tracker_address
        );

        {
            let mut state = self.state.write().await;
            *state = TrackerState::Connected;
        }

        // Construire le payload d'annonce
        let payload = AnnouncePayload {
            cog_id: announcement.cog_id.clone(),
            core_version: announcement.core_version.clone(),
            permis_id: announcement.permis_id.clone(),
            address: announcement.address.clone(),
            services: announcement.services.clone(),
            lobbys: announcement.lobbys.clone(),
            slug: announcement.slug.clone(),
        };

        // Envoyer l'annonce (format Origin : pas de request_id)
        let frame = TrackerFrame::new(TrackerMessageType::Announce, payload.to_bytes());

        stream
            .write_all(&frame.to_bytes())
            .await
            .map_err(|e| MiyuwebwayParticipantError::SendError(e.to_string()))?;

        debug!("ANNOUNCE sent, waiting for response");

        // Lire la réponse
        let response = self.read_frame(&mut stream).await?;

        match response.header.message_type {
            TrackerMessageType::AnnounceAck => {
                let ack = AnnounceAckPayload::from_bytes(&response.payload)
                    .ok_or(MiyuwebwayParticipantError::InvalidPayload)?;

                if ack.success {
                    if let Some(ref subdomain) = ack.assigned_subdomain {
                        info!(
                            "Announce successful: {} (ttl: {}s, subdomain: {})",
                            ack.message, ack.ttl, subdomain
                        );
                    } else {
                        info!("Announce successful: {} (ttl: {}s)", ack.message, ack.ttl);
                    }

                    let mut interval = self.heartbeat_interval.write().await;
                    *interval = ack.heartbeat_interval;

                    let mut state = self.state.write().await;
                    *state = TrackerState::Announced;

                    Ok(())
                } else {
                    warn!("Announce failed: {}", ack.message);
                    Err(MiyuwebwayParticipantError::AnnounceFailed(ack.message))
                }
            }
            TrackerMessageType::Error => {
                let message = String::from_utf8_lossy(&response.payload).to_string();
                error!("Tracker error: {}", message);
                Err(MiyuwebwayParticipantError::TrackerError(message))
            }
            _ => {
                error!("Unexpected response: {:?}", response.header.message_type);
                Err(MiyuwebwayParticipantError::UnexpectedMessage)
            }
        }
    }

    /// Envoie un heartbeat au Tracker.
    pub async fn heartbeat(
        &self,
        cog_id: &str,
        health: u8,
        load: u8,
    ) -> Result<(), MiyuwebwayParticipantError> {
        debug!("Sending heartbeat to Tracker for COG {}", cog_id);

        let (mut stream, _) = self.connect_stream().await?;

        let payload = TrackerHeartbeatPayload {
            cog_id: cog_id.to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
            health,
            load,
        };

        let frame = TrackerFrame::new(TrackerMessageType::Heartbeat, payload.to_bytes());

        stream
            .write_all(&frame.to_bytes())
            .await
            .map_err(|e| MiyuwebwayParticipantError::SendError(e.to_string()))?;

        // Lire l'accusé
        let response = self.read_frame(&mut stream).await?;

        match response.header.message_type {
            TrackerMessageType::HeartbeatAck => {
                debug!("Heartbeat acknowledged");
                Ok(())
            }
            _ => {
                warn!("Unexpected heartbeat response");
                Err(MiyuwebwayParticipantError::UnexpectedMessage)
            }
        }
    }

    /// Retire le COG du réseau.
    pub async fn withdraw(&self, cog_id: &str) -> Result<(), MiyuwebwayParticipantError> {
        let (mut stream, tracker_address) = self.connect_stream().await?;
        info!(
            "[Withdraw] Envoi WITHDRAW pour COG {} au Tracker ({})",
            cog_id, tracker_address
        );

        let payload = serde_json::json!({ "cog_id": cog_id });
        let payload_bytes = bytes::Bytes::from(serde_json::to_vec(&payload).unwrap_or_default());

        let frame = TrackerFrame::new(TrackerMessageType::Withdraw, payload_bytes);

        stream
            .write_all(&frame.to_bytes())
            .await
            .map_err(|e| MiyuwebwayParticipantError::SendError(e.to_string()))?;

        // Attendre l'accusé de retrait (WithdrawAck) du Tracker
        match tokio::time::timeout(
            std::time::Duration::from_secs(5),
            self.read_frame(&mut stream),
        )
        .await
        {
            Ok(Ok(response)) => {
                if response.header.message_type == TrackerMessageType::WithdrawAck {
                    info!("[Withdraw] Accusé de retrait reçu pour COG {}", cog_id);
                } else {
                    warn!(
                        "[Withdraw] Réponse inattendue du Tracker: {:?}",
                        response.header.message_type
                    );
                }
            }
            Ok(Err(e)) => {
                warn!(
                    "[Withdraw] Erreur lecture accusé: {} — le COG a quand même été retiré",
                    e
                );
            }
            Err(_) => {
                warn!("[Withdraw] Timeout en attente de l'accusé — le COG a quand même été retiré");
            }
        }

        let mut state = self.state.write().await;
        *state = TrackerState::Disconnected;

        info!("[Withdraw] COG {} déconnecté du réseau", cog_id);
        Ok(())
    }

    /// Recherche des COGs sur le réseau.
    pub async fn search_cogs(
        &self,
        version_filter: Option<String>,
        service_filter: Option<String>,
        limit: u32,
    ) -> Result<Vec<CogInfo>, MiyuwebwayParticipantError> {
        debug!("Searching COGs on Tracker");

        let (mut stream, _) = self.connect_stream().await?;

        let payload = SearchCogsPayload {
            query: None,
            version: version_filter,
            service: service_filter,
            limit: Some(limit),
            offset: None,
        };

        let frame = TrackerFrame::new(TrackerMessageType::SearchCogs, payload.to_bytes());

        stream
            .write_all(&frame.to_bytes())
            .await
            .map_err(|e| MiyuwebwayParticipantError::SendError(e.to_string()))?;

        let response = self.read_frame(&mut stream).await?;

        match response.header.message_type {
            TrackerMessageType::SearchCogsResult => {
                let result = SearchCogsResultPayload::from_bytes(&response.payload)
                    .ok_or(MiyuwebwayParticipantError::InvalidPayload)?;

                debug!("Found {} COGs (total: {})", result.cogs.len(), result.total);
                Ok(result.cogs)
            }
            TrackerMessageType::Error => {
                let message = String::from_utf8_lossy(&response.payload).to_string();
                Err(MiyuwebwayParticipantError::TrackerError(message))
            }
            _ => Err(MiyuwebwayParticipantError::UnexpectedMessage),
        }
    }

    /// Recherche des lobbys sur le réseau.
    pub async fn search_lobbys(
        &self,
        name_filter: Option<String>,
        public_only: bool,
        limit: u32,
    ) -> Result<Vec<LobbySearchResult>, MiyuwebwayParticipantError> {
        debug!("Searching lobbys on Tracker");

        let (mut stream, _) = self.connect_stream().await?;

        let payload = SearchLobbysPayload {
            query: name_filter,
            version: None,
            available_only: Some(public_only),
            limit: Some(limit),
            offset: None,
        };

        let frame = TrackerFrame::new(TrackerMessageType::SearchLobbys, payload.to_bytes());

        stream
            .write_all(&frame.to_bytes())
            .await
            .map_err(|e| MiyuwebwayParticipantError::SendError(e.to_string()))?;

        let response = self.read_frame(&mut stream).await?;

        match response.header.message_type {
            TrackerMessageType::SearchLobbysResult => {
                let result = SearchLobbysResultPayload::from_bytes(&response.payload)
                    .ok_or(MiyuwebwayParticipantError::InvalidPayload)?;

                debug!(
                    "Found {} lobbys (total: {})",
                    result.lobbys.len(),
                    result.total
                );
                Ok(result.lobbys)
            }
            TrackerMessageType::Error => {
                let message = String::from_utf8_lossy(&response.payload).to_string();
                Err(MiyuwebwayParticipantError::TrackerError(message))
            }
            _ => Err(MiyuwebwayParticipantError::UnexpectedMessage),
        }
    }

    /// Crée un lobby.
    pub async fn create_lobby(
        &self,
        cog_id: &str,
        lobby: LobbyInfo,
        password: Option<String>,
    ) -> Result<String, MiyuwebwayParticipantError> {
        info!("Creating lobby '{}' on Tracker", lobby.name);

        let (mut stream, _) = self.connect_stream().await?;

        let payload = crate::protocol::CreateLobbyPayload {
            cog_id: cog_id.to_string(),
            lobby,
            password,
        };

        let frame = TrackerFrame::new(TrackerMessageType::CreateLobby, payload.to_bytes());

        stream
            .write_all(&frame.to_bytes())
            .await
            .map_err(|e| MiyuwebwayParticipantError::SendError(e.to_string()))?;

        let response = self.read_frame(&mut stream).await?;

        match response.header.message_type {
            TrackerMessageType::CreateLobbyAck => {
                let result: serde_json::Value = serde_json::from_slice(&response.payload)
                    .map_err(|_| MiyuwebwayParticipantError::InvalidPayload)?;

                let lobby_id = result["lobby_id"]
                    .as_str()
                    .ok_or(MiyuwebwayParticipantError::InvalidPayload)?
                    .to_string();

                info!("Lobby created with ID: {}", lobby_id);
                Ok(lobby_id)
            }
            TrackerMessageType::Error => {
                let message = String::from_utf8_lossy(&response.payload).to_string();
                Err(MiyuwebwayParticipantError::TrackerError(message))
            }
            _ => Err(MiyuwebwayParticipantError::UnexpectedMessage),
        }
    }

    /// Supprime un lobby sur le Tracker.
    pub async fn delete_lobby(
        &self,
        cog_id: &str,
        lobby_id: &str,
    ) -> Result<(), MiyuwebwayParticipantError> {
        info!("Deleting lobby {} on Tracker", lobby_id);

        let (mut stream, _) = self.connect_stream().await?;

        let payload = serde_json::json!({
            "cog_id": cog_id,
            "lobby_id": lobby_id,
        });
        let payload_bytes = bytes::Bytes::from(serde_json::to_vec(&payload).unwrap_or_default());

        let frame = TrackerFrame::new(TrackerMessageType::DeleteLobby, payload_bytes);

        stream
            .write_all(&frame.to_bytes())
            .await
            .map_err(|e| MiyuwebwayParticipantError::SendError(e.to_string()))?;

        let response = self.read_frame(&mut stream).await?;

        match response.header.message_type {
            TrackerMessageType::DeleteLobbyOk => {
                info!("Lobby {} deleted on Tracker", lobby_id);
                Ok(())
            }
            TrackerMessageType::Error => {
                let message = String::from_utf8_lossy(&response.payload).to_string();
                Err(MiyuwebwayParticipantError::TrackerError(message))
            }
            _ => Err(MiyuwebwayParticipantError::UnexpectedMessage),
        }
    }

    /// Lit une trame depuis le flux TCP.
    async fn read_frame(
        &self,
        stream: &mut TcpStream,
    ) -> Result<TrackerFrame, MiyuwebwayParticipantError> {
        let mut buffer = self.read_buffer.write().await;
        buffer.clear();

        let mut temp = [0u8; 4096];
        loop {
            let n = stream
                .read(&mut temp)
                .await
                .map_err(|e| MiyuwebwayParticipantError::ReadError(e.to_string()))?;

            if n == 0 {
                return Err(MiyuwebwayParticipantError::ConnectionClosed);
            }

            buffer.extend_from_slice(&temp[..n]);

            if let Some(frame) = TrackerFrame::parse(&mut buffer) {
                return Ok(frame);
            }
        }
    }

    /// Retourne l'état actuel.
    pub async fn get_state(&self) -> TrackerState {
        self.state.read().await.clone()
    }

    /// Vérifie si le COG est annoncé.
    pub async fn is_announced(&self) -> bool {
        *self.state.read().await == TrackerState::Announced
    }

    /// Retourne l'intervalle de heartbeat.
    pub async fn get_heartbeat_interval(&self) -> u32 {
        *self.heartbeat_interval.read().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = TrackerClientConfig::default();
        assert_eq!(config.tracker_address, "miyukini.com:21000");
    }

    #[tokio::test]
    async fn update_official_trackers_switches_to_origin_tracker() {
        let client = TrackerClient::new(TrackerClientConfig {
            tracker_address: "rogue.example.net:21000".to_string(),
            ..TrackerClientConfig::default()
        });

        client
            .update_official_trackers(vec![
                "origin.example.net:21000".to_string(),
                "backup.origin.example.net:21000".to_string(),
            ])
            .await
            .expect("official tracker list should be accepted");

        let config = client.config.read().await;
        assert_eq!(config.tracker_address, "origin.example.net:21000");
        drop(config);

        let official = client.official_trackers.read().await.clone();
        assert_eq!(
            official,
            vec![
                "origin.example.net:21000".to_string(),
                "backup.origin.example.net:21000".to_string()
            ]
        );
    }
}
