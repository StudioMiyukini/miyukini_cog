//! Protocole MWS — Messages et types pour la communication avec Origin.
//!
//! Définit les structures de messages pour le Relay et le Tracker.

use bytes::{Buf, BufMut, Bytes, BytesMut};
use serde::{Deserialize, Serialize};

// ═══════════════════════════════════════════════════════════════════════════════
// CONSTANTES
// ═══════════════════════════════════════════════════════════════════════════════

/// Taille de l'ID de session.
pub const SESSION_ID_SIZE: usize = 16;
/// Taille du nonce.
pub const NONCE_SIZE: usize = 16;
/// Taille de l'ID de permis.
pub const PERMIS_ID_SIZE: usize = 16;
/// Magic number pour les trames Relay.
pub const RELAY_MAGIC: u32 = 0x4D575352; // "MWSR"
/// Magic number pour les trames Tracker.
pub const TRACKER_MAGIC: u32 = 0x4D575354; // "MWST"

// ═══════════════════════════════════════════════════════════════════════════════
// TYPES DE MESSAGES RELAY
// ═══════════════════════════════════════════════════════════════════════════════

/// Types de messages du protocole Relay.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RelayMessageType {
    /// Enregistrement du COG.
    Register = 0x01,
    /// Accusé d'enregistrement.
    RegisterAck = 0x02,
    /// Rejet d'enregistrement.
    RegisterReject = 0x03,
    /// Clé du Core (Phase A).
    CoreKey = 0x10,
    /// Challenge du Relay (Phase A).
    CoreChallenge = 0x11,
    /// Réponse au challenge.
    CoreResponse = 0x12,
    /// Bloc de service (Phase B).
    ServiceBlock = 0x20,
    /// Vérification du bloc.
    ServiceVerify = 0x21,
    /// Permis accordé (Phase C).
    PermisGranted = 0x30,
    /// Permis refusé.
    PermisRejected = 0x31,
    /// Données relayées.
    Data = 0x40,
    /// Heartbeat.
    Heartbeat = 0x50,
    /// Accusé Heartbeat.
    HeartbeatAck = 0x51,
    /// Fermeture de session.
    Close = 0x60,
    /// Redirection vers un autre Relay.
    Redirect = 0x70,
}

impl RelayMessageType {
    /// Convertit un byte en type de message.
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0x01 => Some(Self::Register),
            0x02 => Some(Self::RegisterAck),
            0x03 => Some(Self::RegisterReject),
            0x10 => Some(Self::CoreKey),
            0x11 => Some(Self::CoreChallenge),
            0x12 => Some(Self::CoreResponse),
            0x20 => Some(Self::ServiceBlock),
            0x21 => Some(Self::ServiceVerify),
            0x30 => Some(Self::PermisGranted),
            0x31 => Some(Self::PermisRejected),
            0x40 => Some(Self::Data),
            0x50 => Some(Self::Heartbeat),
            0x51 => Some(Self::HeartbeatAck),
            0x60 => Some(Self::Close),
            0x70 => Some(Self::Redirect),
            _ => None,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// TYPES DE MESSAGES TRACKER
// ═══════════════════════════════════════════════════════════════════════════════

/// Types de messages du protocole Tracker.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TrackerMessageType {
    /// Annonce de présence.
    Announce = 0x01,
    /// Accusé d'annonce.
    AnnounceAck = 0x02,
    /// Retrait du réseau.
    Withdraw = 0x03,
    /// Heartbeat.
    Heartbeat = 0x10,
    /// Accusé Heartbeat.
    HeartbeatAck = 0x11,
    /// Recherche de COGs.
    SearchCogs = 0x20,
    /// Résultats de recherche COGs.
    SearchCogsResult = 0x21,
    /// Recherche de lobbys.
    SearchLobbys = 0x22,
    /// Résultats de recherche lobbys.
    SearchLobbysResult = 0x23,
    /// Création de lobby.
    CreateLobby = 0x30,
    /// Accusé création lobby.
    CreateLobbyAck = 0x31,
    /// Mise à jour lobby.
    UpdateLobby = 0x32,
    /// Suppression lobby.
    DeleteLobby = 0x33,
    /// Rejoindre lobby.
    JoinLobby = 0x34,
    /// Demande de stats.
    GetStats = 0x40,
    /// Réponse stats.
    StatsResult = 0x41,
    /// Erreur.
    Error = 0xFF,
}

impl TrackerMessageType {
    /// Convertit un byte en type de message.
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0x01 => Some(Self::Announce),
            0x02 => Some(Self::AnnounceAck),
            0x03 => Some(Self::Withdraw),
            0x10 => Some(Self::Heartbeat),
            0x11 => Some(Self::HeartbeatAck),
            0x20 => Some(Self::SearchCogs),
            0x21 => Some(Self::SearchCogsResult),
            0x22 => Some(Self::SearchLobbys),
            0x23 => Some(Self::SearchLobbysResult),
            0x30 => Some(Self::CreateLobby),
            0x31 => Some(Self::CreateLobbyAck),
            0x32 => Some(Self::UpdateLobby),
            0x33 => Some(Self::DeleteLobby),
            0x34 => Some(Self::JoinLobby),
            0x40 => Some(Self::GetStats),
            0x41 => Some(Self::StatsResult),
            0xFF => Some(Self::Error),
            _ => None,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// TRAME RELAY
// ═══════════════════════════════════════════════════════════════════════════════

/// En-tête de trame Relay.
#[derive(Debug, Clone)]
pub struct RelayHeader {
    /// Magic number.
    pub magic: u32,
    /// Version du protocole.
    pub version: u8,
    /// Type de message.
    pub message_type: RelayMessageType,
    /// Flags.
    pub flags: u16,
    /// ID de session.
    pub session_id: [u8; SESSION_ID_SIZE],
    /// Taille du payload.
    pub payload_len: u32,
}

impl RelayHeader {
    /// Taille de l'en-tête en bytes.
    pub const SIZE: usize = 4 + 1 + 1 + 2 + SESSION_ID_SIZE + 4;

    /// Parse un en-tête depuis un buffer.
    pub fn parse(buf: &mut impl Buf) -> Option<Self> {
        if buf.remaining() < Self::SIZE {
            return None;
        }

        let magic = buf.get_u32();
        if magic != RELAY_MAGIC {
            return None;
        }

        let version = buf.get_u8();
        let msg_type_byte = buf.get_u8();
        let message_type = RelayMessageType::from_u8(msg_type_byte)?;
        let flags = buf.get_u16();

        let mut session_id = [0u8; SESSION_ID_SIZE];
        buf.copy_to_slice(&mut session_id);

        let payload_len = buf.get_u32();

        Some(Self {
            magic,
            version,
            message_type,
            flags,
            session_id,
            payload_len,
        })
    }

    /// Sérialise l'en-tête.
    pub fn to_bytes(&self) -> BytesMut {
        let mut buf = BytesMut::with_capacity(Self::SIZE);
        buf.put_u32(self.magic);
        buf.put_u8(self.version);
        buf.put_u8(self.message_type as u8);
        buf.put_u16(self.flags);
        buf.put_slice(&self.session_id);
        buf.put_u32(self.payload_len);
        buf
    }
}

/// Trame Relay complète.
#[derive(Debug, Clone)]
pub struct RelayFrame {
    /// En-tête.
    pub header: RelayHeader,
    /// Payload.
    pub payload: Bytes,
}

impl RelayFrame {
    /// Crée une nouvelle trame.
    pub fn new(message_type: RelayMessageType, session_id: [u8; SESSION_ID_SIZE], payload: Bytes) -> Self {
        Self {
            header: RelayHeader {
                magic: RELAY_MAGIC,
                version: 1,
                message_type,
                flags: 0,
                session_id,
                payload_len: payload.len() as u32,
            },
            payload,
        }
    }

    /// Parse une trame depuis un buffer.
    pub fn parse(buf: &mut BytesMut) -> Option<Self> {
        if buf.len() < RelayHeader::SIZE {
            return None;
        }

        // Peek header
        let mut peek = buf.clone();
        let header = RelayHeader::parse(&mut peek)?;

        let total_len = RelayHeader::SIZE + header.payload_len as usize;
        if buf.len() < total_len {
            return None;
        }

        // Consume header
        buf.advance(RelayHeader::SIZE);
        let payload = buf.split_to(header.payload_len as usize).freeze();

        Some(Self { header, payload })
    }

    /// Sérialise la trame.
    pub fn to_bytes(&self) -> BytesMut {
        let mut buf = self.header.to_bytes();
        buf.extend_from_slice(&self.payload);
        buf
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// TRAME TRACKER
// ═══════════════════════════════════════════════════════════════════════════════

/// En-tête de trame Tracker.
#[derive(Debug, Clone)]
pub struct TrackerHeader {
    /// Magic number.
    pub magic: u32,
    /// Version du protocole.
    pub version: u8,
    /// Type de message.
    pub message_type: TrackerMessageType,
    /// Flags.
    pub flags: u16,
    /// ID de requête.
    pub request_id: u32,
    /// Taille du payload.
    pub payload_len: u32,
}

impl TrackerHeader {
    /// Taille de l'en-tête en bytes.
    pub const SIZE: usize = 4 + 1 + 1 + 2 + 4 + 4;

    /// Parse un en-tête depuis un buffer.
    pub fn parse(buf: &mut impl Buf) -> Option<Self> {
        if buf.remaining() < Self::SIZE {
            return None;
        }

        let magic = buf.get_u32();
        if magic != TRACKER_MAGIC {
            return None;
        }

        let version = buf.get_u8();
        let msg_type_byte = buf.get_u8();
        let message_type = TrackerMessageType::from_u8(msg_type_byte)?;
        let flags = buf.get_u16();
        let request_id = buf.get_u32();
        let payload_len = buf.get_u32();

        Some(Self {
            magic,
            version,
            message_type,
            flags,
            request_id,
            payload_len,
        })
    }

    /// Sérialise l'en-tête.
    pub fn to_bytes(&self) -> BytesMut {
        let mut buf = BytesMut::with_capacity(Self::SIZE);
        buf.put_u32(self.magic);
        buf.put_u8(self.version);
        buf.put_u8(self.message_type as u8);
        buf.put_u16(self.flags);
        buf.put_u32(self.request_id);
        buf.put_u32(self.payload_len);
        buf
    }
}

/// Trame Tracker complète.
#[derive(Debug, Clone)]
pub struct TrackerFrame {
    /// En-tête.
    pub header: TrackerHeader,
    /// Payload.
    pub payload: Bytes,
}

impl TrackerFrame {
    /// Crée une nouvelle trame.
    pub fn new(message_type: TrackerMessageType, request_id: u32, payload: Bytes) -> Self {
        Self {
            header: TrackerHeader {
                magic: TRACKER_MAGIC,
                version: 1,
                message_type,
                flags: 0,
                request_id,
                payload_len: payload.len() as u32,
            },
            payload,
        }
    }

    /// Parse une trame depuis un buffer.
    pub fn parse(buf: &mut BytesMut) -> Option<Self> {
        if buf.len() < TrackerHeader::SIZE {
            return None;
        }

        // Peek header
        let mut peek = buf.clone();
        let header = TrackerHeader::parse(&mut peek)?;

        let total_len = TrackerHeader::SIZE + header.payload_len as usize;
        if buf.len() < total_len {
            return None;
        }

        // Consume header
        buf.advance(TrackerHeader::SIZE);
        let payload = buf.split_to(header.payload_len as usize).freeze();

        Some(Self { header, payload })
    }

    /// Sérialise la trame.
    pub fn to_bytes(&self) -> BytesMut {
        let mut buf = self.header.to_bytes();
        buf.extend_from_slice(&self.payload);
        buf
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// PAYLOADS RELAY
// ═══════════════════════════════════════════════════════════════════════════════

/// Payload d'enregistrement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterPayload {
    /// ID du COG.
    pub cog_id: String,
    /// Version des Cores.
    pub core_version: String,
    /// Type de COG (0=standard, 1=service, 2=relay).
    pub cog_type: u8,
    /// Type d'OS.
    pub os_type: u8,
    /// Type de passeport (0=permanent, 1=temporaire).
    pub passport_type: u8,
    /// Services exposés.
    pub services: Vec<String>,
    /// Nonce.
    pub nonce: [u8; NONCE_SIZE],
    /// Timestamp.
    pub timestamp: u64,
}

impl RegisterPayload {
    /// Sérialise en JSON.
    pub fn to_bytes(&self) -> Bytes {
        Bytes::from(serde_json::to_vec(self).unwrap_or_default())
    }

    /// Parse depuis JSON.
    pub fn from_bytes(data: &[u8]) -> Option<Self> {
        serde_json::from_slice(data).ok()
    }
}

/// Payload d'accusé d'enregistrement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterAckPayload {
    /// ID de session assigné.
    pub session_id: [u8; SESSION_ID_SIZE],
    /// Durée de validité en secondes.
    pub ttl: u32,
    /// Intervalle de heartbeat en secondes.
    pub heartbeat_interval: u32,
}

impl RegisterAckPayload {
    /// Parse depuis JSON.
    pub fn from_bytes(data: &[u8]) -> Option<Self> {
        serde_json::from_slice(data).ok()
    }
}

/// Payload de permis accordé.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermisGrantedPayload {
    /// ID du permis.
    pub permis_id: [u8; PERMIS_ID_SIZE],
    /// Durée de validité en secondes.
    pub ttl: u32,
    /// Scopes autorisés.
    pub scopes: Vec<String>,
    /// Signature du permis.
    pub signature: Vec<u8>,
}

impl PermisGrantedPayload {
    /// Parse depuis JSON.
    pub fn from_bytes(data: &[u8]) -> Option<Self> {
        serde_json::from_slice(data).ok()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// PAYLOADS TRACKER
// ═══════════════════════════════════════════════════════════════════════════════

/// Payload d'annonce.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnouncePayload {
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

impl AnnouncePayload {
    /// Sérialise en JSON.
    pub fn to_bytes(&self) -> Bytes {
        Bytes::from(serde_json::to_vec(self).unwrap_or_default())
    }
}

/// Payload d'accusé d'annonce.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnounceAckPayload {
    /// Succès ou non.
    pub success: bool,
    /// Message.
    pub message: String,
    /// Intervalle de heartbeat.
    pub heartbeat_interval: u32,
}

impl AnnounceAckPayload {
    /// Parse depuis JSON.
    pub fn from_bytes(data: &[u8]) -> Option<Self> {
        serde_json::from_slice(data).ok()
    }
}

/// Informations sur un lobby.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyInfo {
    /// ID du lobby.
    pub lobby_id: String,
    /// Nom du lobby.
    pub name: String,
    /// Lobby public.
    pub is_public: bool,
    /// Mot de passe requis.
    pub password_required: bool,
    /// Nombre max de joueurs.
    pub max_players: u32,
    /// Nombre actuel de joueurs.
    pub current_players: u32,
    /// Métadonnées JSON.
    pub metadata: serde_json::Value,
}

/// Payload de heartbeat Tracker.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackerHeartbeatPayload {
    /// ID du COG.
    pub cog_id: String,
    /// État de santé (0-100).
    pub health: u8,
    /// Charge (0-100).
    pub load: u8,
}

impl TrackerHeartbeatPayload {
    /// Sérialise en JSON.
    pub fn to_bytes(&self) -> Bytes {
        Bytes::from(serde_json::to_vec(self).unwrap_or_default())
    }
}

/// Payload de recherche de COGs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchCogsPayload {
    /// Filtre de version (optionnel).
    pub version_filter: Option<String>,
    /// Filtre de service (optionnel).
    pub service_filter: Option<String>,
    /// Nombre max de résultats.
    pub limit: u32,
}

impl SearchCogsPayload {
    /// Sérialise en JSON.
    pub fn to_bytes(&self) -> Bytes {
        Bytes::from(serde_json::to_vec(self).unwrap_or_default())
    }
}

/// Résultat de recherche de COGs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchCogsResultPayload {
    /// COGs trouvés.
    pub cogs: Vec<CogInfo>,
    /// Nombre total (peut dépasser limit).
    pub total: u32,
}

impl SearchCogsResultPayload {
    /// Parse depuis JSON.
    pub fn from_bytes(data: &[u8]) -> Option<Self> {
        serde_json::from_slice(data).ok()
    }
}

/// Informations sur un COG.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CogInfo {
    /// ID du COG.
    pub cog_id: String,
    /// Version des Cores.
    pub core_version: String,
    /// Adresse.
    pub address: String,
    /// Services.
    pub services: Vec<String>,
    /// Dernière activité.
    pub last_seen: String,
}

/// Payload de recherche de lobbys.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchLobbysPayload {
    /// Filtre de nom (optionnel).
    pub name_filter: Option<String>,
    /// Seulement publics.
    pub public_only: bool,
    /// Nombre max de résultats.
    pub limit: u32,
}

impl SearchLobbysPayload {
    /// Sérialise en JSON.
    pub fn to_bytes(&self) -> Bytes {
        Bytes::from(serde_json::to_vec(self).unwrap_or_default())
    }
}

/// Résultat de recherche de lobbys.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchLobbysResultPayload {
    /// Lobbys trouvés.
    pub lobbys: Vec<LobbySearchResult>,
    /// Nombre total.
    pub total: u32,
}

impl SearchLobbysResultPayload {
    /// Parse depuis JSON.
    pub fn from_bytes(data: &[u8]) -> Option<Self> {
        serde_json::from_slice(data).ok()
    }
}

/// Résultat de recherche de lobby.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbySearchResult {
    /// Version du pool.
    pub pool_version: String,
    /// ID du COG hôte.
    pub host_cog_id: String,
    /// Infos du lobby.
    #[serde(flatten)]
    pub lobby: LobbyInfo,
}

/// Payload de création de lobby.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLobbyPayload {
    /// ID du COG.
    pub cog_id: String,
    /// Infos du lobby.
    #[serde(flatten)]
    pub lobby: LobbyInfo,
    /// Mot de passe (si privé).
    pub password: Option<String>,
}

impl CreateLobbyPayload {
    /// Sérialise en JSON.
    pub fn to_bytes(&self) -> Bytes {
        Bytes::from(serde_json::to_vec(self).unwrap_or_default())
    }
}
