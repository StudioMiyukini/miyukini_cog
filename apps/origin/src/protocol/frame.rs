//! Parsing et construction des trames binaires du protocole MWS Relay.
//!
//! Format de trame :
//! ```text
//! +--------+--------+--------+------------------+
//! | Version|  Type  | Flags  | Payload length   |
//! +--------+--------+--------+------------------+
//! | Payload (variable)                           |
//! +----------------------------------------------+
//! ```

use bytes::{Buf, BufMut, Bytes, BytesMut};
use thiserror::Error;

use super::types::{FrameFlags, MessageType, PROTOCOL_VERSION};

/// Erreurs de parsing de trame.
#[derive(Debug, Error)]
pub enum FrameError {
    /// Données insuffisantes pour parser l'en-tête.
    #[error("insufficient data: need {needed} bytes, have {available}")]
    InsufficientData { needed: usize, available: usize },

    /// Version de protocole non supportée.
    #[error("unsupported protocol version: {0}")]
    UnsupportedVersion(u8),

    /// Type de message inconnu.
    #[error("unknown message type: 0x{0:02X}")]
    UnknownMessageType(u8),

    /// Payload trop grand.
    #[error("payload too large: {size} bytes (max: {max})")]
    PayloadTooLarge { size: usize, max: usize },

    /// Trame malformée.
    #[error("malformed frame: {0}")]
    Malformed(String),
}

/// En-tête d'une trame MWS.
#[derive(Debug, Clone)]
pub struct FrameHeader {
    /// Version du protocole.
    pub version: u8,
    /// Type de message.
    pub message_type: MessageType,
    /// Flags.
    pub flags: FrameFlags,
    /// Longueur du payload.
    pub payload_len: u32,
}

impl FrameHeader {
    /// Taille minimale de l'en-tête (sans extension de longueur).
    pub const MIN_SIZE: usize = 5; // version(1) + type(1) + flags(1) + len(2)

    /// Taille maximale de l'en-tête (avec extension de longueur).
    pub const MAX_SIZE: usize = 7; // version(1) + type(1) + flags(1) + len(4)

    /// Crée un nouvel en-tête.
    #[must_use]
    pub fn new(message_type: MessageType, payload_len: u32) -> Self {
        let flags = if payload_len > u16::MAX as u32 {
            FrameFlags::new().with_flag(FrameFlags::EXTENDED_LENGTH)
        } else {
            FrameFlags::new()
        };

        Self {
            version: PROTOCOL_VERSION,
            message_type,
            flags,
            payload_len,
        }
    }

    /// Taille de cet en-tête en octets.
    #[must_use]
    pub fn size(&self) -> usize {
        if self.flags.is_extended_length() {
            Self::MAX_SIZE
        } else {
            Self::MIN_SIZE
        }
    }

    /// Parse un en-tête depuis un buffer.
    pub fn parse(buf: &mut impl Buf) -> Result<Self, FrameError> {
        if buf.remaining() < Self::MIN_SIZE {
            return Err(FrameError::InsufficientData {
                needed: Self::MIN_SIZE,
                available: buf.remaining(),
            });
        }

        let version = buf.get_u8();
        if version != PROTOCOL_VERSION {
            return Err(FrameError::UnsupportedVersion(version));
        }

        let type_byte = buf.get_u8();
        let message_type =
            MessageType::from_byte(type_byte).ok_or(FrameError::UnknownMessageType(type_byte))?;

        let flags = FrameFlags::from_byte(buf.get_u8());

        let payload_len = if flags.is_extended_length() {
            if buf.remaining() < 4 {
                return Err(FrameError::InsufficientData {
                    needed: 4,
                    available: buf.remaining(),
                });
            }
            buf.get_u32()
        } else {
            u32::from(buf.get_u16())
        };

        Ok(Self {
            version,
            message_type,
            flags,
            payload_len,
        })
    }

    /// Sérialise l'en-tête dans un buffer.
    pub fn write(&self, buf: &mut impl BufMut) {
        buf.put_u8(self.version);
        buf.put_u8(self.message_type.to_byte());
        buf.put_u8(self.flags.to_byte());

        if self.flags.is_extended_length() {
            buf.put_u32(self.payload_len);
        } else {
            #[allow(clippy::cast_possible_truncation)]
            buf.put_u16(self.payload_len as u16);
        }
    }
}

/// Une trame complète du protocole MWS.
#[derive(Debug, Clone)]
pub struct Frame {
    /// En-tête de la trame.
    pub header: FrameHeader,
    /// Payload de la trame.
    pub payload: Bytes,
}

impl Frame {
    /// Taille maximale du payload.
    pub const MAX_PAYLOAD_SIZE: usize = 64 * 1024; // 64 Ko

    /// Crée une nouvelle trame.
    #[must_use]
    pub fn new(message_type: MessageType, payload: Bytes) -> Self {
        let header = FrameHeader::new(message_type, payload.len() as u32);
        Self { header, payload }
    }

    /// Crée une trame sans payload.
    #[must_use]
    pub fn empty(message_type: MessageType) -> Self {
        Self::new(message_type, Bytes::new())
    }

    /// Taille totale de la trame en octets.
    #[must_use]
    pub fn size(&self) -> usize {
        self.header.size() + self.payload.len()
    }

    /// Parse une trame complète depuis un buffer.
    ///
    /// Retourne `Ok(None)` si les données sont insuffisantes (attendre plus de données).
    /// Retourne `Ok(Some(frame))` si une trame complète a été parsée.
    /// Retourne `Err(...)` si la trame est invalide.
    pub fn parse(buf: &mut BytesMut) -> Result<Option<Self>, FrameError> {
        // Vérifier qu'on a assez de données pour l'en-tête minimal
        if buf.len() < FrameHeader::MIN_SIZE {
            return Ok(None);
        }

        // Peek le flag pour déterminer la taille de l'en-tête
        let flags = FrameFlags::from_byte(buf[2]);
        let header_size = if flags.is_extended_length() {
            FrameHeader::MAX_SIZE
        } else {
            FrameHeader::MIN_SIZE
        };

        if buf.len() < header_size {
            return Ok(None);
        }

        // Parser l'en-tête (sans consommer le buffer encore)
        let mut header_buf = &buf[..header_size];
        let header = FrameHeader::parse(&mut header_buf)?;

        // Vérifier la taille du payload
        let payload_len = header.payload_len as usize;
        if payload_len > Self::MAX_PAYLOAD_SIZE {
            return Err(FrameError::PayloadTooLarge {
                size: payload_len,
                max: Self::MAX_PAYLOAD_SIZE,
            });
        }

        // Vérifier qu'on a le payload complet
        let total_size = header_size + payload_len;
        if buf.len() < total_size {
            return Ok(None);
        }

        // Consommer le buffer
        let _ = buf.split_to(header_size);
        let payload = buf.split_to(payload_len).freeze();

        Ok(Some(Self { header, payload }))
    }

    /// Sérialise la trame dans un buffer.
    pub fn write(&self, buf: &mut BytesMut) {
        buf.reserve(self.size());
        self.header.write(buf);
        buf.extend_from_slice(&self.payload);
    }

    /// Sérialise la trame en `Bytes`.
    #[must_use]
    pub fn to_bytes(&self) -> Bytes {
        let mut buf = BytesMut::with_capacity(self.size());
        self.write(&mut buf);
        buf.freeze()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Helpers pour construire des trames spécifiques
// ═══════════════════════════════════════════════════════════════════════════

/// Builder pour construire des messages REGISTER.
#[derive(Debug, Default)]
pub struct RegisterBuilder {
    token: Vec<u8>,
    cog_id: Vec<u8>,
    cog_type: u8,
    os_type: u8,
    core_version: Vec<u8>,
    svc_manifest: Vec<u8>,
    environment_health: Vec<u8>,
    previous_permis: Vec<u8>,
    passport_type: u8,
    special_key: Vec<u8>,
    parent_cog_id: Vec<u8>,
    nonce: [u8; 16],
    timestamp: u64,
}

impl RegisterBuilder {
    /// Crée un nouveau builder.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Définit le token.
    #[must_use]
    pub fn token(mut self, token: impl Into<Vec<u8>>) -> Self {
        self.token = token.into();
        self
    }

    /// Définit le cog_id.
    #[must_use]
    pub fn cog_id(mut self, cog_id: impl Into<Vec<u8>>) -> Self {
        self.cog_id = cog_id.into();
        self
    }

    /// Définit le type de COG.
    #[must_use]
    pub const fn cog_type(mut self, cog_type: u8) -> Self {
        self.cog_type = cog_type;
        self
    }

    /// Définit le type d'OS.
    #[must_use]
    pub const fn os_type(mut self, os_type: u8) -> Self {
        self.os_type = os_type;
        self
    }

    /// Définit la version des Cores.
    #[must_use]
    pub fn core_version(mut self, version: impl Into<Vec<u8>>) -> Self {
        self.core_version = version.into();
        self
    }

    /// Définit le manifest des services.
    #[must_use]
    pub fn svc_manifest(mut self, manifest: impl Into<Vec<u8>>) -> Self {
        self.svc_manifest = manifest.into();
        self
    }

    /// Définit le rapport de santé.
    #[must_use]
    pub fn environment_health(mut self, health: impl Into<Vec<u8>>) -> Self {
        self.environment_health = health.into();
        self
    }

    /// Définit l'historique des permis.
    #[must_use]
    pub fn previous_permis(mut self, permis: impl Into<Vec<u8>>) -> Self {
        self.previous_permis = permis.into();
        self
    }

    /// Définit le type de passeport.
    #[must_use]
    pub const fn passport_type(mut self, passport_type: u8) -> Self {
        self.passport_type = passport_type;
        self
    }

    /// Définit la clé spéciale (pour passeport SPECIAL).
    #[must_use]
    pub fn special_key(mut self, key: impl Into<Vec<u8>>) -> Self {
        self.special_key = key.into();
        self
    }

    /// Définit le cog_id parent (pour TERMINAL).
    #[must_use]
    pub fn parent_cog_id(mut self, parent: impl Into<Vec<u8>>) -> Self {
        self.parent_cog_id = parent.into();
        self
    }

    /// Définit le nonce.
    #[must_use]
    pub const fn nonce(mut self, nonce: [u8; 16]) -> Self {
        self.nonce = nonce;
        self
    }

    /// Définit le timestamp.
    #[must_use]
    pub const fn timestamp(mut self, timestamp: u64) -> Self {
        self.timestamp = timestamp;
        self
    }

    /// Construit la trame REGISTER.
    #[must_use]
    pub fn build(self) -> Frame {
        let mut payload = BytesMut::new();

        // token
        #[allow(clippy::cast_possible_truncation)]
        payload.put_u16(self.token.len() as u16);
        payload.extend_from_slice(&self.token);

        // cog_id
        #[allow(clippy::cast_possible_truncation)]
        payload.put_u16(self.cog_id.len() as u16);
        payload.extend_from_slice(&self.cog_id);

        // cog_type, os_type
        payload.put_u8(self.cog_type);
        payload.put_u8(self.os_type);

        // core_version
        #[allow(clippy::cast_possible_truncation)]
        payload.put_u8(self.core_version.len() as u8);
        payload.extend_from_slice(&self.core_version);

        // svc_manifest
        #[allow(clippy::cast_possible_truncation)]
        payload.put_u16(self.svc_manifest.len() as u16);
        payload.extend_from_slice(&self.svc_manifest);

        // environment_health
        #[allow(clippy::cast_possible_truncation)]
        payload.put_u16(self.environment_health.len() as u16);
        payload.extend_from_slice(&self.environment_health);

        // previous_permis
        #[allow(clippy::cast_possible_truncation)]
        payload.put_u16(self.previous_permis.len() as u16);
        payload.extend_from_slice(&self.previous_permis);

        // passport_type
        payload.put_u8(self.passport_type);

        // special_key
        #[allow(clippy::cast_possible_truncation)]
        payload.put_u16(self.special_key.len() as u16);
        payload.extend_from_slice(&self.special_key);

        // parent_cog_id
        #[allow(clippy::cast_possible_truncation)]
        payload.put_u16(self.parent_cog_id.len() as u16);
        payload.extend_from_slice(&self.parent_cog_id);

        // nonce
        payload.extend_from_slice(&self.nonce);

        // timestamp
        payload.put_u64(self.timestamp);

        Frame::new(MessageType::Register, payload.freeze())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frame_header_roundtrip() {
        let header = FrameHeader::new(MessageType::Register, 100);
        let mut buf = BytesMut::new();
        header.write(&mut buf);

        let parsed = FrameHeader::parse(&mut buf.as_ref()).unwrap();
        assert_eq!(parsed.version, PROTOCOL_VERSION);
        assert_eq!(parsed.message_type, MessageType::Register);
        assert_eq!(parsed.payload_len, 100);
    }

    #[test]
    fn test_frame_roundtrip() {
        let payload = Bytes::from_static(b"test payload data");
        let frame = Frame::new(MessageType::Data, payload.clone());

        let mut buf = BytesMut::new();
        frame.write(&mut buf);

        let parsed = Frame::parse(&mut buf).unwrap().unwrap();
        assert_eq!(parsed.header.message_type, MessageType::Data);
        assert_eq!(parsed.payload, payload);
    }

    #[test]
    fn test_extended_length() {
        // Payload > 65535 bytes devrait utiliser la longueur étendue
        let header = FrameHeader::new(MessageType::Data, 70000);
        assert!(header.flags.is_extended_length());
        assert_eq!(header.size(), FrameHeader::MAX_SIZE);
    }

    #[test]
    fn test_register_builder() {
        let frame = RegisterBuilder::new()
            .token(b"test-token".to_vec())
            .cog_id(b"cog-001".to_vec())
            .cog_type(0x03) // STABLE
            .os_type(0x01) // LINUX
            .core_version(b"1.0.0".to_vec())
            .nonce([0u8; 16])
            .timestamp(1234567890)
            .build();

        assert_eq!(frame.header.message_type, MessageType::Register);
        assert!(!frame.payload.is_empty());
    }
}
