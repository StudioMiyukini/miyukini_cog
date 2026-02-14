//! Client Tracker MWS — Communication avec le Tracker Origin.
//!
//! Gère l'annonce, le heartbeat, et la découverte de COGs/lobbys.

use crate::errors::MiyuwebwayParticipantError;
use crate::protocol::{
    AnnounceAckPayload, AnnouncePayload, CogInfo, LobbyInfo, LobbySearchResult,
    SearchCogsPayload, SearchCogsResultPayload, SearchLobbysPayload, SearchLobbysResultPayload,
    TrackerFrame, TrackerHeartbeatPayload, TrackerMessageType,
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
}

/// Client Tracker MWS (protocole aligné Origin).
pub struct TrackerClient {
    /// Configuration.
    config: TrackerClientConfig,
    /// État actuel.
    state: Arc<RwLock<TrackerState>>,
    /// Buffer de lecture.
    read_buffer: Arc<RwLock<BytesMut>>,
    /// Intervalle de heartbeat actuel.
    heartbeat_interval: Arc<RwLock<u32>>,
}

impl TrackerClient {
    /// Crée un nouveau client Tracker.
    pub fn new(config: TrackerClientConfig) -> Self {
        Self {
            config,
            state: Arc::new(RwLock::new(TrackerState::Disconnected)),
            read_buffer: Arc::new(RwLock::new(BytesMut::with_capacity(8192))),
            heartbeat_interval: Arc::new(RwLock::new(60)),
        }
    }

    /// Crée un client avec la configuration par défaut.
    pub fn with_defaults() -> Self {
        Self::new(TrackerClientConfig::default())
    }

    /// Annonce le COG sur le réseau.
    pub async fn announce(
        &self,
        announcement: &TrackerAnnouncement,
    ) -> Result<(), MiyuwebwayParticipantError> {
        info!(
            "Announcing COG {} on Tracker at {}",
            announcement.cog_id, self.config.tracker_address
        );

        // Connexion TCP
        let mut stream = TcpStream::connect(&self.config.tracker_address)
            .await
            .map_err(|e| {
                error!("TCP connection to Tracker failed: {}", e);
                MiyuwebwayParticipantError::ConnectionFailed(e.to_string())
            })?;

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
        };

        // Envoyer l'annonce (format Origin : pas de request_id)
        let frame = TrackerFrame::new(TrackerMessageType::Announce, payload.to_bytes());

        stream.write_all(&frame.to_bytes()).await.map_err(|e| {
            MiyuwebwayParticipantError::SendError(e.to_string())
        })?;

        debug!("ANNOUNCE sent, waiting for response");

        // Lire la réponse
        let response = self.read_frame(&mut stream).await?;

        match response.header.message_type {
            TrackerMessageType::AnnounceAck => {
                let ack = AnnounceAckPayload::from_bytes(&response.payload)
                    .ok_or(MiyuwebwayParticipantError::InvalidPayload)?;

                if ack.success {
                    info!("Announce successful: {} (ttl: {}s)", ack.message, ack.ttl);

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

        let mut stream = TcpStream::connect(&self.config.tracker_address)
            .await
            .map_err(|e| {
                MiyuwebwayParticipantError::ConnectionFailed(e.to_string())
            })?;

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

        stream.write_all(&frame.to_bytes()).await.map_err(|e| {
            MiyuwebwayParticipantError::SendError(e.to_string())
        })?;

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
        info!("Withdrawing COG {} from Tracker", cog_id);

        let mut stream = TcpStream::connect(&self.config.tracker_address)
            .await
            .map_err(|e| {
                MiyuwebwayParticipantError::ConnectionFailed(e.to_string())
            })?;

        let payload = serde_json::json!({ "cog_id": cog_id });
        let payload_bytes = bytes::Bytes::from(serde_json::to_vec(&payload).unwrap_or_default());

        let frame = TrackerFrame::new(TrackerMessageType::Withdraw, payload_bytes);

        stream.write_all(&frame.to_bytes()).await.map_err(|e| {
            MiyuwebwayParticipantError::SendError(e.to_string())
        })?;

        let mut state = self.state.write().await;
        *state = TrackerState::Disconnected;

        info!("COG withdrawn from network");
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

        let mut stream = TcpStream::connect(&self.config.tracker_address)
            .await
            .map_err(|e| {
                MiyuwebwayParticipantError::ConnectionFailed(e.to_string())
            })?;

        let payload = SearchCogsPayload {
            query: None,
            version: version_filter,
            service: service_filter,
            limit: Some(limit),
            offset: None,
        };

        let frame = TrackerFrame::new(TrackerMessageType::SearchCogs, payload.to_bytes());

        stream.write_all(&frame.to_bytes()).await.map_err(|e| {
            MiyuwebwayParticipantError::SendError(e.to_string())
        })?;

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

        let mut stream = TcpStream::connect(&self.config.tracker_address)
            .await
            .map_err(|e| {
                MiyuwebwayParticipantError::ConnectionFailed(e.to_string())
            })?;

        let payload = SearchLobbysPayload {
            query: name_filter,
            version: None,
            available_only: Some(public_only),
            limit: Some(limit),
            offset: None,
        };

        let frame = TrackerFrame::new(TrackerMessageType::SearchLobbys, payload.to_bytes());

        stream.write_all(&frame.to_bytes()).await.map_err(|e| {
            MiyuwebwayParticipantError::SendError(e.to_string())
        })?;

        let response = self.read_frame(&mut stream).await?;

        match response.header.message_type {
            TrackerMessageType::SearchLobbysResult => {
                let result = SearchLobbysResultPayload::from_bytes(&response.payload)
                    .ok_or(MiyuwebwayParticipantError::InvalidPayload)?;

                debug!("Found {} lobbys (total: {})", result.lobbys.len(), result.total);
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

        let mut stream = TcpStream::connect(&self.config.tracker_address)
            .await
            .map_err(|e| {
                MiyuwebwayParticipantError::ConnectionFailed(e.to_string())
            })?;

        let payload = crate::protocol::CreateLobbyPayload {
            cog_id: cog_id.to_string(),
            lobby,
            password,
        };

        let frame = TrackerFrame::new(TrackerMessageType::CreateLobby, payload.to_bytes());

        stream.write_all(&frame.to_bytes()).await.map_err(|e| {
            MiyuwebwayParticipantError::SendError(e.to_string())
        })?;

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

        let mut stream = TcpStream::connect(&self.config.tracker_address)
            .await
            .map_err(|e| {
                MiyuwebwayParticipantError::ConnectionFailed(e.to_string())
            })?;

        let payload = serde_json::json!({
            "cog_id": cog_id,
            "lobby_id": lobby_id,
        });
        let payload_bytes = bytes::Bytes::from(serde_json::to_vec(&payload).unwrap_or_default());

        let frame = TrackerFrame::new(TrackerMessageType::DeleteLobby, payload_bytes);

        stream.write_all(&frame.to_bytes()).await.map_err(|e| {
            MiyuwebwayParticipantError::SendError(e.to_string())
        })?;

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
            let n = stream.read(&mut temp).await.map_err(|e| {
                MiyuwebwayParticipantError::ReadError(e.to_string())
            })?;

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
}
