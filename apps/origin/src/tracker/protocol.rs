//! Protocole Tracker.
//!
//! Messages binaires pour la communication avec le Tracker.

use bytes::{Buf, BufMut, Bytes, BytesMut};
use serde::{Deserialize, Serialize};

/// Version du protocole Tracker.
pub const TRACKER_PROTOCOL_VERSION: u8 = 1;

/// Taille maximale d'un message Tracker.
pub const MAX_TRACKER_MESSAGE_SIZE: usize = 65536;

/// Types de messages Tracker.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TrackerMessageType {
    // ─────────────────────────────────────────────────────────────────────────
    // Annonce et découverte
    // ─────────────────────────────────────────────────────────────────────────
    /// COG annonce sa présence.
    Announce = 0x01,
    /// Réponse à une annonce.
    AnnounceAck = 0x02,
    /// Retrait d'un COG.
    Withdraw = 0x03,
    /// Heartbeat (maintien).
    Heartbeat = 0x04,
    /// Réponse au heartbeat.
    HeartbeatAck = 0x05,

    // ─────────────────────────────────────────────────────────────────────────
    // Recherche
    // ─────────────────────────────────────────────────────────────────────────
    /// Recherche de COGs.
    SearchCogs = 0x10,
    /// Résultats de recherche de COGs.
    SearchCogsResult = 0x11,
    /// Recherche de lobbys.
    SearchLobbys = 0x12,
    /// Résultats de recherche de lobbys.
    SearchLobbysResult = 0x13,
    /// Recherche de services.
    SearchServices = 0x14,
    /// Résultats de recherche de services.
    SearchServicesResult = 0x15,

    // ─────────────────────────────────────────────────────────────────────────
    // Lobbys
    // ─────────────────────────────────────────────────────────────────────────
    /// Création de lobby.
    CreateLobby = 0x20,
    /// Confirmation de création.
    CreateLobbyOk = 0x21,
    /// Erreur de création.
    CreateLobbyErr = 0x22,
    /// Mise à jour de lobby.
    UpdateLobby = 0x23,
    /// Suppression de lobby.
    DeleteLobby = 0x24,
    /// Accusé de suppression de lobby.
    DeleteLobbyOk = 0x28,
    /// Demande pour rejoindre.
    JoinLobby = 0x25,
    /// Résultat de la demande.
    JoinLobbyResult = 0x26,
    /// Quitter un lobby.
    LeaveLobby = 0x27,

    // ─────────────────────────────────────────────────────────────────────────
    // Informations
    // ─────────────────────────────────────────────────────────────────────────
    /// Demande de statistiques.
    GetStats = 0x30,
    /// Réponse statistiques.
    StatsResult = 0x31,
    /// Demande d'info sur un COG.
    GetCogInfo = 0x32,
    /// Info sur un COG.
    CogInfoResult = 0x33,

    // ─────────────────────────────────────────────────────────────────────────
    // Erreurs
    // ─────────────────────────────────────────────────────────────────────────
    /// Erreur générique.
    Error = 0xFF,
}

impl TrackerMessageType {
    /// Convertit un byte en type de message.
    pub fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            0x01 => Some(Self::Announce),
            0x02 => Some(Self::AnnounceAck),
            0x03 => Some(Self::Withdraw),
            0x04 => Some(Self::Heartbeat),
            0x05 => Some(Self::HeartbeatAck),
            0x10 => Some(Self::SearchCogs),
            0x11 => Some(Self::SearchCogsResult),
            0x12 => Some(Self::SearchLobbys),
            0x13 => Some(Self::SearchLobbysResult),
            0x14 => Some(Self::SearchServices),
            0x15 => Some(Self::SearchServicesResult),
            0x20 => Some(Self::CreateLobby),
            0x21 => Some(Self::CreateLobbyOk),
            0x22 => Some(Self::CreateLobbyErr),
            0x23 => Some(Self::UpdateLobby),
            0x24 => Some(Self::DeleteLobby),
            0x28 => Some(Self::DeleteLobbyOk),
            0x25 => Some(Self::JoinLobby),
            0x26 => Some(Self::JoinLobbyResult),
            0x27 => Some(Self::LeaveLobby),
            0x30 => Some(Self::GetStats),
            0x31 => Some(Self::StatsResult),
            0x32 => Some(Self::GetCogInfo),
            0x33 => Some(Self::CogInfoResult),
            0xFF => Some(Self::Error),
            _ => None,
        }
    }
}

/// En-tête de message Tracker.
#[derive(Debug, Clone)]
pub struct TrackerHeader {
    /// Version du protocole.
    pub version: u8,
    /// Type de message.
    pub message_type: TrackerMessageType,
    /// Taille du payload.
    pub payload_len: u16,
}

impl TrackerHeader {
    /// Taille de l'en-tête.
    pub const SIZE: usize = 4;

    /// Parse un en-tête.
    pub fn parse(buf: &mut impl Buf) -> Result<Self, TrackerError> {
        if buf.remaining() < Self::SIZE {
            return Err(TrackerError::InsufficientData);
        }

        let version = buf.get_u8();
        if version != TRACKER_PROTOCOL_VERSION {
            return Err(TrackerError::UnsupportedVersion(version));
        }

        let type_byte = buf.get_u8();
        let message_type = TrackerMessageType::from_byte(type_byte)
            .ok_or(TrackerError::UnknownMessageType(type_byte))?;

        let payload_len = buf.get_u16();

        Ok(Self {
            version,
            message_type,
            payload_len,
        })
    }

    /// Sérialise l'en-tête.
    pub fn to_bytes(&self) -> Bytes {
        let mut buf = BytesMut::with_capacity(Self::SIZE);
        buf.put_u8(self.version);
        buf.put_u8(self.message_type as u8);
        buf.put_u16(self.payload_len);
        buf.freeze()
    }
}

/// Message Tracker complet.
#[derive(Debug, Clone)]
pub struct TrackerMessage {
    /// En-tête.
    pub header: TrackerHeader,
    /// Payload.
    pub payload: Bytes,
}

impl TrackerMessage {
    /// Crée un nouveau message.
    pub fn new(message_type: TrackerMessageType, payload: Bytes) -> Self {
        Self {
            header: TrackerHeader {
                version: TRACKER_PROTOCOL_VERSION,
                message_type,
                payload_len: payload.len() as u16,
            },
            payload,
        }
    }

    /// Parse un message depuis un buffer.
    pub fn parse(buf: &mut BytesMut) -> Result<Option<Self>, TrackerError> {
        if buf.len() < TrackerHeader::SIZE {
            return Ok(None);
        }

        // Peek the header
        let mut peek = &buf[..];
        let header = TrackerHeader::parse(&mut peek)?;

        let total_len = TrackerHeader::SIZE + header.payload_len as usize;
        if buf.len() < total_len {
            return Ok(None);
        }

        // Consume header
        buf.advance(TrackerHeader::SIZE);

        // Extract payload
        let payload = buf.split_to(header.payload_len as usize).freeze();

        Ok(Some(Self { header, payload }))
    }

    /// Sérialise le message.
    pub fn to_bytes(&self) -> Bytes {
        let mut buf = BytesMut::with_capacity(TrackerHeader::SIZE + self.payload.len());
        buf.extend_from_slice(&self.header.to_bytes());
        buf.extend_from_slice(&self.payload);
        buf.freeze()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Payloads
// ═══════════════════════════════════════════════════════════════════════════════

/// Payload ANNOUNCE.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnouncePayload {
    /// Identifiant du COG.
    pub cog_id: String,
    /// Version des Cores.
    pub core_version: String,
    /// Adresse de connexion (IP:Port).
    pub address: String,
    /// Services exposés.
    pub services: Vec<String>,
    /// Permis ID (pour vérification).
    pub permis_id: String,
}

impl AnnouncePayload {
    /// Sérialise en JSON.
    pub fn to_bytes(&self) -> Bytes {
        Bytes::from(serde_json::to_vec(self).unwrap_or_default())
    }

    /// Parse depuis JSON.
    pub fn parse(data: &[u8]) -> Result<Self, TrackerError> {
        serde_json::from_slice(data).map_err(|e| TrackerError::InvalidPayload(e.to_string()))
    }
}

/// Payload ANNOUNCE_ACK.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnounceAckPayload {
    /// Succès.
    pub success: bool,
    /// Message.
    pub message: String,
    /// Intervalle de heartbeat recommandé (secondes).
    pub heartbeat_interval: u32,
    /// TTL de l'annonce (secondes).
    pub ttl: u32,
}

impl AnnounceAckPayload {
    /// Sérialise en JSON.
    pub fn to_bytes(&self) -> Bytes {
        Bytes::from(serde_json::to_vec(self).unwrap_or_default())
    }
}

/// Payload SEARCH_COGS.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchCogsPayload {
    /// Requête (optionnel, vide = tous).
    pub query: Option<String>,
    /// Filtrer par version.
    pub version: Option<String>,
    /// Filtrer par service.
    pub service: Option<String>,
    /// Nombre maximum de résultats.
    pub limit: Option<u32>,
    /// Offset pour pagination.
    pub offset: Option<u32>,
}

impl SearchCogsPayload {
    /// Parse depuis JSON.
    pub fn parse(data: &[u8]) -> Result<Self, TrackerError> {
        serde_json::from_slice(data).map_err(|e| TrackerError::InvalidPayload(e.to_string()))
    }
}

/// Résultat de recherche COG.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CogSearchResult {
    /// Identifiant du COG.
    pub cog_id: String,
    /// Version des Cores.
    pub core_version: String,
    /// Adresse.
    pub address: String,
    /// Services.
    pub services: Vec<String>,
    /// Nombre de lobbys.
    pub lobby_count: u32,
}

/// Payload SEARCH_COGS_RESULT.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchCogsResultPayload {
    /// Résultats.
    pub results: Vec<CogSearchResult>,
    /// Nombre total (avant pagination).
    pub total: u32,
}

impl SearchCogsResultPayload {
    /// Sérialise en JSON.
    pub fn to_bytes(&self) -> Bytes {
        Bytes::from(serde_json::to_vec(self).unwrap_or_default())
    }
}

/// Payload SEARCH_LOBBYS.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchLobbysPayload {
    /// Requête (nom du lobby).
    pub query: Option<String>,
    /// Filtrer par version.
    pub version: Option<String>,
    /// Afficher seulement les lobbys avec places disponibles.
    pub available_only: Option<bool>,
    /// Nombre maximum de résultats.
    pub limit: Option<u32>,
    /// Offset.
    pub offset: Option<u32>,
}

impl SearchLobbysPayload {
    /// Parse depuis JSON.
    pub fn parse(data: &[u8]) -> Result<Self, TrackerError> {
        serde_json::from_slice(data).map_err(|e| TrackerError::InvalidPayload(e.to_string()))
    }
}

/// Résultat de recherche Lobby.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbySearchResult {
    /// ID du lobby.
    pub lobby_id: String,
    /// Nom.
    pub name: String,
    /// COG hôte.
    pub host_cog_id: String,
    /// Version du pool.
    pub pool_version: String,
    /// Mot de passe requis.
    pub password_required: bool,
    /// Joueurs actuels.
    pub current_players: u32,
    /// Max joueurs.
    pub max_players: u32,
    /// Métadonnées.
    pub metadata: serde_json::Value,
}

/// Payload SEARCH_LOBBYS_RESULT.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchLobbysResultPayload {
    /// Résultats.
    pub results: Vec<LobbySearchResult>,
    /// Nombre total.
    pub total: u32,
}

impl SearchLobbysResultPayload {
    /// Sérialise en JSON.
    pub fn to_bytes(&self) -> Bytes {
        Bytes::from(serde_json::to_vec(self).unwrap_or_default())
    }
}

/// Payload CREATE_LOBBY.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLobbyPayload {
    /// COG hôte.
    pub cog_id: String,
    /// Nom du lobby.
    pub name: String,
    /// Public.
    pub is_public: bool,
    /// Mot de passe (optionnel).
    pub password: Option<String>,
    /// Max joueurs.
    pub max_players: u32,
    /// Métadonnées.
    pub metadata: serde_json::Value,
}

impl CreateLobbyPayload {
    /// Parse depuis JSON.
    pub fn parse(data: &[u8]) -> Result<Self, TrackerError> {
        serde_json::from_slice(data).map_err(|e| TrackerError::InvalidPayload(e.to_string()))
    }
}

/// Payload CREATE_LOBBY_OK.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLobbyOkPayload {
    /// ID du lobby créé.
    pub lobby_id: String,
}

impl CreateLobbyOkPayload {
    /// Sérialise en JSON.
    pub fn to_bytes(&self) -> Bytes {
        Bytes::from(serde_json::to_vec(self).unwrap_or_default())
    }
}

/// Payload UPDATE_LOBBY.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateLobbyPayload {
    /// COG hôte.
    pub cog_id: String,
    /// ID du lobby.
    pub lobby_id: String,
    /// Nouveau nom (optionnel).
    pub name: Option<String>,
    /// Nouveau nombre de joueurs.
    pub current_players: Option<u32>,
    /// Nouvelles métadonnées.
    pub metadata: Option<serde_json::Value>,
}

impl UpdateLobbyPayload {
    /// Parse depuis JSON.
    pub fn parse(data: &[u8]) -> Result<Self, TrackerError> {
        serde_json::from_slice(data).map_err(|e| TrackerError::InvalidPayload(e.to_string()))
    }
}

/// Payload JOIN_LOBBY.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinLobbyPayload {
    /// COG qui veut rejoindre.
    pub cog_id: String,
    /// ID du lobby.
    pub lobby_id: String,
    /// Mot de passe (si requis).
    pub password: Option<String>,
}

impl JoinLobbyPayload {
    /// Parse depuis JSON.
    pub fn parse(data: &[u8]) -> Result<Self, TrackerError> {
        serde_json::from_slice(data).map_err(|e| TrackerError::InvalidPayload(e.to_string()))
    }
}

/// Payload JOIN_LOBBY_RESULT.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinLobbyResultPayload {
    /// Succès.
    pub success: bool,
    /// Message.
    pub message: String,
    /// Adresse du COG hôte (si succès).
    pub host_address: Option<String>,
}

impl JoinLobbyResultPayload {
    /// Sérialise en JSON.
    pub fn to_bytes(&self) -> Bytes {
        Bytes::from(serde_json::to_vec(self).unwrap_or_default())
    }
}

/// Payload HEARTBEAT.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeartbeatPayload {
    /// ID du COG.
    pub cog_id: String,
    /// Timestamp.
    pub timestamp: u64,
}

impl HeartbeatPayload {
    /// Parse depuis JSON.
    pub fn parse(data: &[u8]) -> Result<Self, TrackerError> {
        serde_json::from_slice(data).map_err(|e| TrackerError::InvalidPayload(e.to_string()))
    }

    /// Sérialise en JSON.
    pub fn to_bytes(&self) -> Bytes {
        Bytes::from(serde_json::to_vec(self).unwrap_or_default())
    }
}

/// Payload GET_STATS.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsResultPayload {
    /// Nombre total de COGs.
    pub total_cogs: u32,
    /// Nombre de lobbys.
    pub total_lobbys: u32,
    /// Versions disponibles.
    pub versions: Vec<String>,
    /// Uptime en secondes.
    pub uptime_seconds: u64,
}

impl StatsResultPayload {
    /// Sérialise en JSON.
    pub fn to_bytes(&self) -> Bytes {
        Bytes::from(serde_json::to_vec(self).unwrap_or_default())
    }
}

/// Payload ERROR.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorPayload {
    /// Code d'erreur.
    pub code: u16,
    /// Message.
    pub message: String,
}

impl ErrorPayload {
    /// Crée un nouveau payload d'erreur.
    pub fn new(code: u16, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }

    /// Sérialise en JSON.
    pub fn to_bytes(&self) -> Bytes {
        Bytes::from(serde_json::to_vec(self).unwrap_or_default())
    }
}

/// Codes d'erreur Tracker.
pub mod error_codes {
    /// COG non trouvé.
    pub const COG_NOT_FOUND: u16 = 1001;
    /// Lobby non trouvé.
    pub const LOBBY_NOT_FOUND: u16 = 1002;
    /// Lobby plein.
    pub const LOBBY_FULL: u16 = 1003;
    /// Mot de passe incorrect.
    pub const WRONG_PASSWORD: u16 = 1004;
    /// Permis invalide.
    pub const INVALID_PERMIS: u16 = 1005;
    /// Rate limit dépassé.
    pub const RATE_LIMITED: u16 = 1006;
    /// Erreur interne.
    pub const INTERNAL_ERROR: u16 = 5000;
}

/// Erreur du protocole Tracker.
#[derive(Debug, Clone)]
pub enum TrackerError {
    /// Données insuffisantes.
    InsufficientData,
    /// Version non supportée.
    UnsupportedVersion(u8),
    /// Type de message inconnu.
    UnknownMessageType(u8),
    /// Payload invalide.
    InvalidPayload(String),
}

impl std::fmt::Display for TrackerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InsufficientData => write!(f, "Insufficient data"),
            Self::UnsupportedVersion(v) => write!(f, "Unsupported version: {}", v),
            Self::UnknownMessageType(t) => write!(f, "Unknown message type: 0x{:02x}", t),
            Self::InvalidPayload(e) => write!(f, "Invalid payload: {}", e),
        }
    }
}

impl std::error::Error for TrackerError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_roundtrip() {
        let payload = AnnouncePayload {
            cog_id: "test-cog".to_string(),
            core_version: "1.0.0".to_string(),
            address: "127.0.0.1:12345".to_string(),
            services: vec!["jayfestival".to_string()],
            permis_id: "permis-123".to_string(),
        };

        let msg = TrackerMessage::new(TrackerMessageType::Announce, payload.to_bytes());
        let bytes = msg.to_bytes();

        let mut buf = BytesMut::from(&bytes[..]);
        let parsed = TrackerMessage::parse(&mut buf).unwrap().unwrap();

        assert_eq!(parsed.header.message_type, TrackerMessageType::Announce);
    }
}
