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

/// Types de messages du protocole Relay (bytes alignés sur Origin).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RelayMessageType {
    /// Enregistrement du COG.
    Register = 0x01,
    /// Accusé d'enregistrement (Origin: RegisterOk).
    RegisterAck = 0x02,
    /// Rejet d'enregistrement (Origin: RegisterErr).
    RegisterReject = 0x03,
    /// Données (Origin: Data).
    Data = 0x07,
    /// Heartbeat.
    Heartbeat = 0x08,
    /// Accusé Heartbeat.
    HeartbeatAck = 0x09,
    /// Fermeture de session.
    Close = 0x0A,
    /// Clé du Core (Phase A, Origin).
    CoreKey = 0x10,
    /// Bloc de service (Phase B, Origin: ServiceBlock).
    ServiceBlock = 0x11,
    /// Résultat vérification (Origin: VerifyResult).
    VerifyResult = 0x12,
    /// Redirection (Origin).
    Redirect = 0x13,
}

impl RelayMessageType {
    /// Convertit un byte en type de message.
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0x01 => Some(Self::Register),
            0x02 => Some(Self::RegisterAck),
            0x03 => Some(Self::RegisterReject),
            0x07 => Some(Self::Data),
            0x08 => Some(Self::Heartbeat),
            0x09 => Some(Self::HeartbeatAck),
            0x0A => Some(Self::Close),
            0x10 => Some(Self::CoreKey),
            0x11 => Some(Self::ServiceBlock),
            0x12 => Some(Self::VerifyResult),
            0x13 => Some(Self::Redirect),
            _ => None,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// TYPES DE MESSAGES TRACKER (alignés sur Origin : apps/origin/src/tracker/protocol)
// ═══════════════════════════════════════════════════════════════════════════════

/// Types de messages du protocole Tracker (bytes identiques à Origin).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TrackerMessageType {
    /// Annonce de présence.
    Announce = 0x01,
    /// Accusé d'annonce.
    AnnounceAck = 0x02,
    /// Retrait du réseau.
    Withdraw = 0x03,
    /// Accusé de retrait.
    WithdrawAck = 0x06,
    /// Heartbeat.
    Heartbeat = 0x04,
    /// Accusé Heartbeat.
    HeartbeatAck = 0x05,
    /// Recherche de COGs.
    SearchCogs = 0x10,
    /// Résultats de recherche COGs.
    SearchCogsResult = 0x11,
    /// Recherche de lobbys.
    SearchLobbys = 0x12,
    /// Résultats de recherche lobbys.
    SearchLobbysResult = 0x13,
    /// Création de lobby.
    CreateLobby = 0x20,
    /// Accusé création lobby.
    CreateLobbyAck = 0x21,
    /// Erreur création lobby.
    CreateLobbyErr = 0x22,
    /// Mise à jour lobby.
    UpdateLobby = 0x23,
    /// Suppression lobby.
    DeleteLobby = 0x24,
    /// Accusé suppression lobby.
    DeleteLobbyOk = 0x28,
    /// Rejoindre lobby.
    JoinLobby = 0x25,
    /// Résultat join lobby.
    JoinLobbyResult = 0x26,
    /// Demande de stats.
    GetStats = 0x30,
    /// Réponse stats.
    StatsResult = 0x31,
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
            0x06 => Some(Self::WithdrawAck),
            0x04 => Some(Self::Heartbeat),
            0x05 => Some(Self::HeartbeatAck),
            0x10 => Some(Self::SearchCogs),
            0x11 => Some(Self::SearchCogsResult),
            0x12 => Some(Self::SearchLobbys),
            0x13 => Some(Self::SearchLobbysResult),
            0x20 => Some(Self::CreateLobby),
            0x21 => Some(Self::CreateLobbyAck),
            0x22 => Some(Self::CreateLobbyErr),
            0x23 => Some(Self::UpdateLobby),
            0x24 => Some(Self::DeleteLobby),
            0x28 => Some(Self::DeleteLobbyOk),
            0x25 => Some(Self::JoinLobby),
            0x26 => Some(Self::JoinLobbyResult),
            0x30 => Some(Self::GetStats),
            0x31 => Some(Self::StatsResult),
            0xFF => Some(Self::Error),
            _ => None,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// TRAME RELAY
// ═══════════════════════════════════════════════════════════════════════════════

/// Version du protocole Relay (identique à Origin).
pub const RELAY_PROTOCOL_VERSION: u8 = 1;

/// En-tête de trame Relay (format Origin : 5 octets).
#[derive(Debug, Clone)]
pub struct RelayHeader {
    /// Version du protocole.
    pub version: u8,
    /// Type de message.
    pub message_type: RelayMessageType,
    /// Flags (0 = normal).
    pub flags: u8,
    /// Taille du payload.
    pub payload_len: u16,
}

impl RelayHeader {
    /// Taille de l'en-tête en bytes.
    pub const SIZE: usize = 1 + 1 + 1 + 2;

    /// Parse un en-tête depuis un buffer.
    pub fn parse(buf: &mut impl Buf) -> Option<Self> {
        if buf.remaining() < Self::SIZE {
            return None;
        }
        let version = buf.get_u8();
        let msg_type_byte = buf.get_u8();
        let message_type = RelayMessageType::from_u8(msg_type_byte)?;
        let flags = buf.get_u8();
        let payload_len = buf.get_u16();
        Some(Self {
            version,
            message_type,
            flags,
            payload_len,
        })
    }

    /// Sérialise l'en-tête.
    pub fn to_bytes(&self) -> BytesMut {
        let mut buf = BytesMut::with_capacity(Self::SIZE);
        buf.put_u8(self.version);
        buf.put_u8(self.message_type as u8);
        buf.put_u8(self.flags);
        buf.put_u16(self.payload_len);
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
    /// Crée une nouvelle trame (aligné Origin : pas de session_id dans l'en-tête).
    pub fn new(message_type: RelayMessageType, payload: Bytes) -> Self {
        let payload_len = payload.len().min(u16::MAX as usize) as u16;
        Self {
            header: RelayHeader {
                version: RELAY_PROTOCOL_VERSION,
                message_type,
                flags: 0,
                payload_len,
            },
            payload,
        }
    }

    /// Parse une trame depuis un buffer.
    pub fn parse(buf: &mut BytesMut) -> Option<Self> {
        if buf.len() < RelayHeader::SIZE {
            return None;
        }
        let mut peek = buf.clone();
        let header = RelayHeader::parse(&mut peek)?;
        let total_len = RelayHeader::SIZE + header.payload_len as usize;
        if buf.len() < total_len {
            return None;
        }
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
// TRAME TRACKER (alignée sur Origin : version + type + payload_len u16 = 4 octets)
// ═══════════════════════════════════════════════════════════════════════════════

/// Version du protocole Tracker (identique à Origin).
pub const TRACKER_PROTOCOL_VERSION: u8 = 1;

/// En-tête de trame Tracker (format Origin : 4 octets).
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
    /// Taille de l'en-tête en bytes.
    pub const SIZE: usize = 1 + 1 + 2;

    /// Parse un en-tête depuis un buffer.
    pub fn parse(buf: &mut impl Buf) -> Option<Self> {
        if buf.remaining() < Self::SIZE {
            return None;
        }
        let version = buf.get_u8();
        let msg_type_byte = buf.get_u8();
        let message_type = TrackerMessageType::from_u8(msg_type_byte)?;
        let payload_len = buf.get_u16();
        Some(Self {
            version,
            message_type,
            payload_len,
        })
    }

    /// Sérialise l'en-tête.
    pub fn to_bytes(&self) -> BytesMut {
        let mut buf = BytesMut::with_capacity(Self::SIZE);
        buf.put_u8(self.version);
        buf.put_u8(self.message_type as u8);
        buf.put_u16(self.payload_len);
        buf
    }
}

/// Trame Tracker complète (compatible Origin).
#[derive(Debug, Clone)]
pub struct TrackerFrame {
    /// En-tête.
    pub header: TrackerHeader,
    /// Payload.
    pub payload: Bytes,
}

impl TrackerFrame {
    /// Crée une nouvelle trame (sans request_id, aligné Origin).
    pub fn new(message_type: TrackerMessageType, payload: Bytes) -> Self {
        let payload_len = payload.len().min(u16::MAX as usize) as u16;
        Self {
            header: TrackerHeader {
                version: TRACKER_PROTOCOL_VERSION,
                message_type,
                payload_len,
            },
            payload,
        }
    }

    /// Parse une trame depuis un buffer.
    pub fn parse(buf: &mut BytesMut) -> Option<Self> {
        if buf.len() < TrackerHeader::SIZE {
            return None;
        }
        let mut peek = buf.clone();
        let header = TrackerHeader::parse(&mut peek)?;
        let total_len = TrackerHeader::SIZE + header.payload_len as usize;
        if buf.len() < total_len {
            return None;
        }
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

/// Payload d'enregistrement (structure interne ; envoi au Relay = format binaire Origin).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterPayload {
    /// ID du COG.
    pub cog_id: String,
    /// Version des Cores.
    pub core_version: String,
    /// Type de COG (Origin: 0x03 = Stable).
    pub cog_type: u8,
    /// Type d'OS.
    pub os_type: u8,
    /// Type de passeport (Origin: 0 = Standard).
    pub passport_type: u8,
    /// Services exposés.
    pub services: Vec<String>,
    /// Nonce.
    pub nonce: [u8; NONCE_SIZE],
    /// Timestamp.
    pub timestamp: u64,
}

impl RegisterPayload {
    /// Sérialise au format binaire Origin (length-prefixed fields).
    pub fn to_origin_bytes(&self) -> Bytes {
        let mut buf = BytesMut::new();
        // token (u16 len + bytes) — vide
        buf.put_u16(0);
        // cog_id (u16 len + bytes)
        let cog_id_b = self.cog_id.as_bytes();
        buf.put_u16(cog_id_b.len() as u16);
        buf.extend_from_slice(cog_id_b);
        // cog_type, os_type
        buf.put_u8(self.cog_type);
        buf.put_u8(self.os_type);
        // core_version (u8 len + bytes)
        let cv = self.core_version.as_bytes();
        buf.put_u8(cv.len() as u8);
        buf.extend_from_slice(cv);
        // svc_manifest (u16) — JSON des services
        let manifest = serde_json::to_vec(&self.services).unwrap_or_default();
        buf.put_u16(manifest.len() as u16);
        buf.extend_from_slice(&manifest);
        // environment_health (u16) — vide
        buf.put_u16(0);
        // previous_permis (u16) — vide
        buf.put_u16(0);
        // passport_type
        buf.put_u8(self.passport_type);
        // special_key (u16) — vide
        buf.put_u16(0);
        // parent_cog_id (u16) — vide
        buf.put_u16(0);
        // nonce (16) + timestamp (u64)
        buf.extend_from_slice(&self.nonce);
        buf.put_u64(self.timestamp);
        buf.freeze()
    }

    /// Sérialise en JSON (legacy / tests).
    pub fn to_bytes(&self) -> Bytes {
        Bytes::from(serde_json::to_vec(self).unwrap_or_default())
    }

    /// Parse depuis JSON.
    pub fn from_bytes(data: &[u8]) -> Option<Self> {
        serde_json::from_slice(data).ok()
    }
}

/// Phase de vérification (aligné Origin).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum VerifyPhase {
    PhaseA = 1,
    PhaseB = 2,
    PhaseC = 3,
}

/// Résultat de vérification (aligné Origin).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum VerifyResult {
    Ok = 0,
    Fail = 1,
    ExtendedRequired = 2,
}

/// Payload VERIFY_RESULT (Origin → participant).
#[derive(Debug, Clone)]
pub struct VerifyResultPayload {
    pub session_id: [u8; SESSION_ID_SIZE],
    pub phase: VerifyPhase,
    pub result: VerifyResult,
    pub message: Bytes,
}

impl VerifyResultPayload {
    /// Parse depuis le format binaire Origin.
    pub fn from_origin_bytes(data: &[u8]) -> Option<Self> {
        if data.len() < SESSION_ID_SIZE + 1 + 1 + 2 {
            return None;
        }
        let mut i = 0;
        let mut session_id = [0u8; SESSION_ID_SIZE];
        session_id.copy_from_slice(&data[i..i + SESSION_ID_SIZE]);
        i += SESSION_ID_SIZE;
        let phase_byte = data[i];
        i += 1;
        let result_byte = data[i];
        i += 1;
        let phase = match phase_byte {
            1 => VerifyPhase::PhaseA,
            2 => VerifyPhase::PhaseB,
            3 => VerifyPhase::PhaseC,
            _ => return None,
        };
        let result = match result_byte {
            0 => VerifyResult::Ok,
            1 => VerifyResult::Fail,
            2 => VerifyResult::ExtendedRequired,
            _ => return None,
        };
        if data.len() < i + 2 {
            return None;
        }
        let service_id_len = u16::from_be_bytes([data[i], data[i + 1]]) as usize;
        i += 2;
        if data.len() < i + service_id_len {
            return None;
        }
        i += service_id_len;
        if data.len() < i + 2 {
            return None;
        }
        let message_len = u16::from_be_bytes([data[i], data[i + 1]]) as usize;
        i += 2;
        if data.len() < i + message_len {
            return None;
        }
        let message = Bytes::copy_from_slice(&data[i..i + message_len]);
        Some(Self {
            session_id,
            phase,
            result,
            message,
        })
    }
}

/// Construit le payload binaire CORE_KEY (participant → Origin).
#[must_use]
pub fn core_key_payload_bytes(session_id: &[u8; SESSION_ID_SIZE], key: &[u8]) -> Bytes {
    let mut buf = BytesMut::new();
    buf.extend_from_slice(session_id);
    buf.put_u16(key.len() as u16);
    buf.extend_from_slice(key);
    buf.freeze()
}

/// Payload d'accusé d'enregistrement (RegisterOk Origin).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterAckPayload {
    /// ID de session assigné.
    pub session_id: [u8; SESSION_ID_SIZE],
    /// ID du Permis de circulation (accord relay) — utilisé pour ANNOUNCE au Tracker.
    #[serde(default)]
    pub permis_id: Vec<u8>,
    /// Durée de validité en secondes (dérivé de permis_expires_at).
    pub ttl: u32,
    /// Intervalle de heartbeat en secondes (non présent dans Origin; défaut 30).
    pub heartbeat_interval: u32,
}

impl RegisterAckPayload {
    /// Parse depuis le format binaire Origin (RegisterOkPayload).
    pub fn from_origin_bytes(data: &[u8]) -> Option<Self> {
        if data.len() < SESSION_ID_SIZE + 2 {
            return None;
        }
        let mut i = 0;
        let mut session_id = [0u8; SESSION_ID_SIZE];
        session_id.copy_from_slice(&data[i..i + SESSION_ID_SIZE]);
        i += SESSION_ID_SIZE;
        let permis_len = u16::from_be_bytes([data[i], data[i + 1]]) as usize;
        i += 2;
        if data.len() < i + permis_len {
            return None;
        }
        let permis_id = data[i..i + permis_len].to_vec();
        i += permis_len;
        if data.len() < i + 8 {
            return None;
        }
        let permis_expires_at = u64::from_be_bytes(data[i..i + 8].try_into().ok()?);
        i += 8;
        let scope_len = u16::from_be_bytes([data[i], data[i + 1]]) as usize;
        i += 2;
        if data.len() < i + scope_len {
            return None;
        }
        i += scope_len;
        if data.len() < i + 2 {
            return None;
        }
        let tracker_len = u16::from_be_bytes([data[i], data[i + 1]]) as usize;
        i += 2;
        if data.len() < i + tracker_len + 64 + 1 {
            return None;
        }
        i += tracker_len;
        i += 64; // tracker_signature
        let _status = data[i];
        let ttl = if permis_expires_at > 0 {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .ok()
                .map(|d| d.as_secs())
                .unwrap_or(0);
            (permis_expires_at.saturating_sub(now)) as u32
        } else {
            86400
        };
        Some(Self {
            session_id,
            permis_id,
            ttl,
            heartbeat_interval: 30,
        })
    }

    /// Parse depuis JSON (legacy).
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

/// Payload d'accusé d'annonce (aligné Origin : success, message, heartbeat_interval, ttl).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnounceAckPayload {
    /// Succès ou non.
    pub success: bool,
    /// Message.
    pub message: String,
    /// Intervalle de heartbeat (secondes).
    pub heartbeat_interval: u32,
    /// TTL de l'annonce en secondes (Origin).
    #[serde(default)]
    pub ttl: u32,
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
/// Champs alignés sur le format attendu côté Origin : `cog_id` + `timestamp`.
/// `health` et `load` sont conservés pour enrichissement futur.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackerHeartbeatPayload {
    /// ID du COG.
    pub cog_id: String,
    /// Timestamp Unix (secondes) — requis par le Tracker Origin.
    pub timestamp: u64,
    /// État de santé (0-100).
    #[serde(default)]
    pub health: u8,
    /// Charge (0-100).
    #[serde(default)]
    pub load: u8,
}

impl TrackerHeartbeatPayload {
    /// Sérialise en JSON.
    pub fn to_bytes(&self) -> Bytes {
        Bytes::from(serde_json::to_vec(self).unwrap_or_default())
    }
}

/// Payload de recherche de COGs (champs alignés Origin : query, version, service, limit, offset).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchCogsPayload {
    /// Requête (optionnel).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Filtre version (Origin).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Filtre service (Origin).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    /// Nombre max de résultats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Offset pagination (Origin).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
}

impl SearchCogsPayload {
    /// Sérialise en JSON (format Origin).
    pub fn to_bytes(&self) -> Bytes {
        Bytes::from(serde_json::to_vec(self).unwrap_or_default())
    }
}

/// Résultat de recherche de COGs (Origin envoie "results" + "total").
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchCogsResultPayload {
    /// COGs trouvés (Origin utilise le champ "results").
    #[serde(alias = "results")]
    pub cogs: Vec<CogInfo>,
    /// Nombre total.
    pub total: u32,
}

impl SearchCogsResultPayload {
    /// Parse depuis JSON.
    pub fn from_bytes(data: &[u8]) -> Option<Self> {
        serde_json::from_slice(data).ok()
    }
}

/// Informations sur un COG (compatible Origin : CogSearchResult avec lobby_count).
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
    /// Dernière activité (optionnel, Origin envoie lobby_count à la place).
    #[serde(default)]
    pub last_seen: String,
    /// Nombre de lobbys (champ Origin, ignoré si absent).
    #[serde(default, rename = "lobby_count")]
    pub _lobby_count: Option<u32>,
}

/// Payload de recherche de lobbys (aligné Origin : query, version, available_only, limit, offset).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchLobbysPayload {
    /// Requête nom (Origin).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Filtre version (Origin).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Seulement avec places dispo (Origin: available_only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_only: Option<bool>,
    /// Nombre max de résultats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Offset (Origin).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
}

impl SearchLobbysPayload {
    /// Sérialise en JSON.
    pub fn to_bytes(&self) -> Bytes {
        Bytes::from(serde_json::to_vec(self).unwrap_or_default())
    }
}

/// Résultat de recherche de lobbys (Origin envoie "results" + "total").
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchLobbysResultPayload {
    /// Lobbys trouvés (Origin utilise le champ "results").
    #[serde(alias = "results")]
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
