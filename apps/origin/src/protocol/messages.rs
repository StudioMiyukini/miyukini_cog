//! Messages spécifiques du protocole MWS Relay.
//!
//! Structures pour parser et construire les payloads des différents types de messages.

use bytes::{Buf, BufMut, Bytes, BytesMut};
use serde::{Deserialize, Serialize};

use super::frame::FrameError;
use super::types::*;

// ═══════════════════════════════════════════════════════════════════════════
// Message REGISTER
// ═══════════════════════════════════════════════════════════════════════════

/// Payload du message REGISTER.
#[derive(Debug, Clone)]
pub struct RegisterPayload {
    /// Token d'authentification.
    pub token: Bytes,
    /// Identifiant du COG.
    pub cog_id: Bytes,
    /// Type de COG.
    pub cog_type: CogType,
    /// Type d'OS.
    pub os_type: OsType,
    /// Version des Cores.
    pub core_version: Bytes,
    /// Manifest des services (JSON).
    pub svc_manifest: Bytes,
    /// Rapport de santé de l'environnement.
    pub environment_health: Bytes,
    /// Historique des permis précédents (JSON).
    pub previous_permis: Bytes,
    /// Type de passeport.
    pub passport_type: PassportType,
    /// Clé spéciale (pour passeport SPECIAL).
    pub special_key: Bytes,
    /// cog_id du parent (pour TERMINAL).
    pub parent_cog_id: Bytes,
    /// Nonce anti-rejeu.
    pub nonce: [u8; NONCE_SIZE],
    /// Horodatage (secondes depuis epoch).
    pub timestamp: u64,
}

impl RegisterPayload {
    /// Parse un payload REGISTER.
    pub fn parse(mut buf: impl Buf) -> Result<Self, FrameError> {
        // Helper pour lire une chaîne avec préfixe de longueur (2 octets)
        fn read_bytes_u16(buf: &mut impl Buf, field: &str) -> Result<Bytes, FrameError> {
            if buf.remaining() < 2 {
                return Err(FrameError::Malformed(format!("{field}: missing length")));
            }
            let len = buf.get_u16() as usize;
            if buf.remaining() < len {
                return Err(FrameError::Malformed(format!(
                    "{field}: expected {len} bytes, have {}",
                    buf.remaining()
                )));
            }
            Ok(buf.copy_to_bytes(len))
        }

        // Helper pour lire une chaîne avec préfixe de longueur (1 octet)
        fn read_bytes_u8(buf: &mut impl Buf, field: &str) -> Result<Bytes, FrameError> {
            if buf.remaining() < 1 {
                return Err(FrameError::Malformed(format!("{field}: missing length")));
            }
            let len = buf.get_u8() as usize;
            if buf.remaining() < len {
                return Err(FrameError::Malformed(format!(
                    "{field}: expected {len} bytes, have {}",
                    buf.remaining()
                )));
            }
            Ok(buf.copy_to_bytes(len))
        }

        let token = read_bytes_u16(&mut buf, "token")?;
        let cog_id = read_bytes_u16(&mut buf, "cog_id")?;

        if buf.remaining() < 2 {
            return Err(FrameError::Malformed("missing cog_type/os_type".into()));
        }
        let cog_type = CogType::from_byte(buf.get_u8())
            .ok_or_else(|| FrameError::Malformed("invalid cog_type".into()))?;
        let os_type = OsType::from_byte(buf.get_u8())
            .ok_or_else(|| FrameError::Malformed("invalid os_type".into()))?;

        let core_version = read_bytes_u8(&mut buf, "core_version")?;
        let svc_manifest = read_bytes_u16(&mut buf, "svc_manifest")?;
        let environment_health = read_bytes_u16(&mut buf, "environment_health")?;
        let previous_permis = read_bytes_u16(&mut buf, "previous_permis")?;

        if buf.remaining() < 1 {
            return Err(FrameError::Malformed("missing passport_type".into()));
        }
        let passport_type = PassportType::from_byte(buf.get_u8())
            .ok_or_else(|| FrameError::Malformed("invalid passport_type".into()))?;

        let special_key = read_bytes_u16(&mut buf, "special_key")?;
        let parent_cog_id = read_bytes_u16(&mut buf, "parent_cog_id")?;

        if buf.remaining() < NONCE_SIZE + 8 {
            return Err(FrameError::Malformed("missing nonce/timestamp".into()));
        }
        let mut nonce = [0u8; NONCE_SIZE];
        buf.copy_to_slice(&mut nonce);
        let timestamp = buf.get_u64();

        Ok(Self {
            token,
            cog_id,
            cog_type,
            os_type,
            core_version,
            svc_manifest,
            environment_health,
            previous_permis,
            passport_type,
            special_key,
            parent_cog_id,
            nonce,
            timestamp,
        })
    }

    /// Retourne le cog_id comme String (si UTF-8 valide).
    #[must_use]
    pub fn cog_id_str(&self) -> Option<String> {
        std::str::from_utf8(&self.cog_id).ok().map(String::from)
    }

    /// Retourne la version des Cores comme String (si UTF-8 valide).
    #[must_use]
    pub fn core_version_str(&self) -> Option<String> {
        std::str::from_utf8(&self.core_version).ok().map(String::from)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Message REGISTER_OK
// ═══════════════════════════════════════════════════════════════════════════

/// Payload du message REGISTER_OK.
#[derive(Debug, Clone)]
pub struct RegisterOkPayload {
    /// Identifiant de session.
    pub session_id: [u8; SESSION_ID_SIZE],
    /// Identifiant du Permis de circulation.
    pub permis_id: Bytes,
    /// Date d'expiration du Permis (timestamp).
    pub permis_expires_at: u64,
    /// Portée du Permis (JSON).
    pub permis_scope: Bytes,
    /// Liste des adresses des trackers officiels.
    pub tracker_addresses: Bytes,
    /// Signature Ed25519 de la liste des trackers.
    pub tracker_signature: [u8; ED25519_SIGNATURE_SIZE],
    /// Statut (0 = OK, 1 = UPDATE_RECOMMENDED).
    pub status: u8,
    /// Version minimale recommandée des Cores (optionnel).
    pub min_core_version: Option<Bytes>,
}

impl RegisterOkPayload {
    /// Construit un payload REGISTER_OK.
    #[must_use]
    pub fn build(
        session_id: [u8; SESSION_ID_SIZE],
        permis_id: impl Into<Bytes>,
        permis_expires_at: u64,
        permis_scope: impl Into<Bytes>,
        tracker_addresses: impl Into<Bytes>,
        tracker_signature: [u8; ED25519_SIGNATURE_SIZE],
    ) -> Self {
        Self {
            session_id,
            permis_id: permis_id.into(),
            permis_expires_at,
            permis_scope: permis_scope.into(),
            tracker_addresses: tracker_addresses.into(),
            tracker_signature,
            status: 0,
            min_core_version: None,
        }
    }

    /// Sérialise le payload.
    #[must_use]
    pub fn to_bytes(&self) -> Bytes {
        let mut buf = BytesMut::new();

        buf.extend_from_slice(&self.session_id);

        #[allow(clippy::cast_possible_truncation)]
        buf.put_u16(self.permis_id.len() as u16);
        buf.extend_from_slice(&self.permis_id);

        buf.put_u64(self.permis_expires_at);

        #[allow(clippy::cast_possible_truncation)]
        buf.put_u16(self.permis_scope.len() as u16);
        buf.extend_from_slice(&self.permis_scope);

        #[allow(clippy::cast_possible_truncation)]
        buf.put_u16(self.tracker_addresses.len() as u16);
        buf.extend_from_slice(&self.tracker_addresses);

        buf.extend_from_slice(&self.tracker_signature);

        buf.put_u8(self.status);

        if let Some(ref version) = self.min_core_version {
            #[allow(clippy::cast_possible_truncation)]
            buf.put_u8(version.len() as u8);
            buf.extend_from_slice(version);
        } else {
            buf.put_u8(0);
        }

        buf.freeze()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Message REGISTER_ERR
// ═══════════════════════════════════════════════════════════════════════════

/// Payload du message REGISTER_ERR.
#[derive(Debug, Clone)]
pub struct RegisterErrPayload {
    /// Code d'erreur.
    pub code: RegisterErrorCode,
    /// Message explicatif.
    pub message: Bytes,
}

impl RegisterErrPayload {
    /// Crée un nouveau payload d'erreur.
    #[must_use]
    pub fn new(code: RegisterErrorCode, message: impl Into<Bytes>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }

    /// Sérialise le payload.
    #[must_use]
    pub fn to_bytes(&self) -> Bytes {
        let mut buf = BytesMut::new();
        buf.put_u8(self.code.to_byte());
        #[allow(clippy::cast_possible_truncation)]
        buf.put_u16(self.message.len() as u16);
        buf.extend_from_slice(&self.message);
        buf.freeze()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Message DATA
// ═══════════════════════════════════════════════════════════════════════════

/// Payload du message DATA.
#[derive(Debug, Clone)]
pub struct DataPayload {
    /// Identifiant de session.
    pub session_id: [u8; SESSION_ID_SIZE],
    /// Numéro de séquence.
    pub sequence: u32,
    /// Données opaques.
    pub payload: Bytes,
    /// HMAC-SHA256 du message.
    pub mac: [u8; HMAC_SHA256_SIZE],
}

impl DataPayload {
    /// Parse un payload DATA.
    pub fn parse(mut buf: impl Buf) -> Result<Self, FrameError> {
        if buf.remaining() < SESSION_ID_SIZE + 4 + 4 + HMAC_SHA256_SIZE {
            return Err(FrameError::Malformed("DATA payload too short".into()));
        }

        let mut session_id = [0u8; SESSION_ID_SIZE];
        buf.copy_to_slice(&mut session_id);

        let sequence = buf.get_u32();
        let payload_len = buf.get_u32() as usize;

        if buf.remaining() < payload_len + HMAC_SHA256_SIZE {
            return Err(FrameError::Malformed("DATA payload incomplete".into()));
        }

        let payload = buf.copy_to_bytes(payload_len);

        let mut mac = [0u8; HMAC_SHA256_SIZE];
        buf.copy_to_slice(&mut mac);

        Ok(Self {
            session_id,
            sequence,
            payload,
            mac,
        })
    }

    /// Sérialise le payload.
    #[must_use]
    pub fn to_bytes(&self) -> Bytes {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(&self.session_id);
        buf.put_u32(self.sequence);
        #[allow(clippy::cast_possible_truncation)]
        buf.put_u32(self.payload.len() as u32);
        buf.extend_from_slice(&self.payload);
        buf.extend_from_slice(&self.mac);
        buf.freeze()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Message HEARTBEAT
// ═══════════════════════════════════════════════════════════════════════════

/// Payload du message HEARTBEAT.
#[derive(Debug, Clone)]
pub struct HeartbeatPayload {
    /// Identifiant de session.
    pub session_id: [u8; SESSION_ID_SIZE],
    /// Horodatage.
    pub timestamp: u64,
}

impl HeartbeatPayload {
    /// Parse un payload HEARTBEAT.
    pub fn parse(mut buf: impl Buf) -> Result<Self, FrameError> {
        if buf.remaining() < SESSION_ID_SIZE + 8 {
            return Err(FrameError::Malformed("HEARTBEAT payload too short".into()));
        }

        let mut session_id = [0u8; SESSION_ID_SIZE];
        buf.copy_to_slice(&mut session_id);
        let timestamp = buf.get_u64();

        Ok(Self {
            session_id,
            timestamp,
        })
    }

    /// Sérialise le payload.
    #[must_use]
    pub fn to_bytes(&self) -> Bytes {
        let mut buf = BytesMut::with_capacity(SESSION_ID_SIZE + 8);
        buf.extend_from_slice(&self.session_id);
        buf.put_u64(self.timestamp);
        buf.freeze()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Message CLOSE
// ═══════════════════════════════════════════════════════════════════════════

/// Payload du message CLOSE.
#[derive(Debug, Clone)]
pub struct ClosePayload {
    /// Identifiant de session.
    pub session_id: [u8; SESSION_ID_SIZE],
    /// Raison de fermeture.
    pub reason: CloseReason,
    /// Message explicatif.
    pub message: Bytes,
}

impl ClosePayload {
    /// Crée un nouveau payload CLOSE.
    #[must_use]
    pub fn new(session_id: [u8; SESSION_ID_SIZE], reason: CloseReason, message: impl Into<Bytes>) -> Self {
        Self {
            session_id,
            reason,
            message: message.into(),
        }
    }

    /// Sérialise le payload.
    #[must_use]
    pub fn to_bytes(&self) -> Bytes {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(&self.session_id);
        buf.put_u8(self.reason.to_byte());
        #[allow(clippy::cast_possible_truncation)]
        buf.put_u16(self.message.len() as u16);
        buf.extend_from_slice(&self.message);
        buf.freeze()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Message CORE_KEY
// ═══════════════════════════════════════════════════════════════════════════

/// Payload du message CORE_KEY.
#[derive(Debug, Clone)]
pub struct CoreKeyPayload {
    /// Identifiant de session.
    pub session_id: [u8; SESSION_ID_SIZE],
    /// Clé de conformité des Cores.
    pub key: Bytes,
}

impl CoreKeyPayload {
    /// Parse un payload CORE_KEY.
    pub fn parse(mut buf: impl Buf) -> Result<Self, FrameError> {
        if buf.remaining() < SESSION_ID_SIZE + 2 {
            return Err(FrameError::Malformed("CORE_KEY payload too short".into()));
        }

        let mut session_id = [0u8; SESSION_ID_SIZE];
        buf.copy_to_slice(&mut session_id);

        let key_len = buf.get_u16() as usize;
        if buf.remaining() < key_len {
            return Err(FrameError::Malformed("CORE_KEY key incomplete".into()));
        }

        let key = buf.copy_to_bytes(key_len);

        Ok(Self { session_id, key })
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Message SERVICE_BLOCK
// ═══════════════════════════════════════════════════════════════════════════

/// Payload du message SERVICE_BLOCK.
#[derive(Debug, Clone)]
pub struct ServiceBlockPayload {
    /// Identifiant de session.
    pub session_id: [u8; SESSION_ID_SIZE],
    /// Identifiant du service.
    pub service_id: Bytes,
    /// Index du bloc MIP.
    pub block_index: u32,
    /// Bloc de code chiffré.
    pub encrypted_block: Bytes,
}

impl ServiceBlockPayload {
    /// Parse un payload SERVICE_BLOCK.
    pub fn parse(mut buf: impl Buf) -> Result<Self, FrameError> {
        if buf.remaining() < SESSION_ID_SIZE + 2 {
            return Err(FrameError::Malformed("SERVICE_BLOCK too short".into()));
        }

        let mut session_id = [0u8; SESSION_ID_SIZE];
        buf.copy_to_slice(&mut session_id);

        let service_id_len = buf.get_u16() as usize;
        if buf.remaining() < service_id_len + 4 + 4 {
            return Err(FrameError::Malformed("SERVICE_BLOCK incomplete".into()));
        }

        let service_id = buf.copy_to_bytes(service_id_len);
        let block_index = buf.get_u32();
        let block_len = buf.get_u32() as usize;

        if buf.remaining() < block_len {
            return Err(FrameError::Malformed("SERVICE_BLOCK block incomplete".into()));
        }

        let encrypted_block = buf.copy_to_bytes(block_len);

        Ok(Self {
            session_id,
            service_id,
            block_index,
            encrypted_block,
        })
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Message VERIFY_RESULT
// ═══════════════════════════════════════════════════════════════════════════

/// Payload du message VERIFY_RESULT.
#[derive(Debug, Clone)]
pub struct VerifyResultPayload {
    /// Identifiant de session.
    pub session_id: [u8; SESSION_ID_SIZE],
    /// Phase de vérification.
    pub phase: VerifyPhase,
    /// Résultat.
    pub result: VerifyResult,
    /// Identifiant du service (Phase B uniquement).
    pub service_id: Option<Bytes>,
    /// Message explicatif.
    pub message: Bytes,
}

impl VerifyResultPayload {
    /// Crée un nouveau payload VERIFY_RESULT.
    #[must_use]
    pub fn new(
        session_id: [u8; SESSION_ID_SIZE],
        phase: VerifyPhase,
        result: VerifyResult,
        message: impl Into<Bytes>,
    ) -> Self {
        Self {
            session_id,
            phase,
            result,
            service_id: None,
            message: message.into(),
        }
    }

    /// Ajoute un service_id (pour Phase B).
    #[must_use]
    pub fn with_service_id(mut self, service_id: impl Into<Bytes>) -> Self {
        self.service_id = Some(service_id.into());
        self
    }

    /// Sérialise le payload.
    #[must_use]
    pub fn to_bytes(&self) -> Bytes {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(&self.session_id);
        buf.put_u8(self.phase.to_byte());
        buf.put_u8(self.result.to_byte());

        if let Some(ref service_id) = self.service_id {
            #[allow(clippy::cast_possible_truncation)]
            buf.put_u16(service_id.len() as u16);
            buf.extend_from_slice(service_id);
        } else {
            buf.put_u16(0);
        }

        #[allow(clippy::cast_possible_truncation)]
        buf.put_u16(self.message.len() as u16);
        buf.extend_from_slice(&self.message);

        buf.freeze()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Message REDIRECT
// ═══════════════════════════════════════════════════════════════════════════

/// Payload du message REDIRECT.
#[derive(Debug, Clone)]
pub struct RedirectPayload {
    /// Adresse du relay.
    pub relay_host: Bytes,
    /// Port du relay.
    pub relay_port: u16,
    /// Raison de la redirection.
    pub reason: RedirectReason,
}

impl RedirectPayload {
    /// Crée un nouveau payload REDIRECT.
    #[must_use]
    pub fn new(relay_host: impl Into<Bytes>, relay_port: u16, reason: RedirectReason) -> Self {
        Self {
            relay_host: relay_host.into(),
            relay_port,
            reason,
        }
    }

    /// Sérialise le payload.
    #[must_use]
    pub fn to_bytes(&self) -> Bytes {
        let mut buf = BytesMut::new();
        #[allow(clippy::cast_possible_truncation)]
        buf.put_u16(self.relay_host.len() as u16);
        buf.extend_from_slice(&self.relay_host);
        buf.put_u16(self.relay_port);
        buf.put_u8(self.reason.to_byte());
        buf.freeze()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Structures JSON pour svc_manifest et permis_scope
// ═══════════════════════════════════════════════════════════════════════════

/// Manifest des services (JSON).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceManifest {
    /// Liste des services déclarés.
    pub services: Vec<ServiceEntry>,
}

/// Entrée dans le manifest des services.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEntry {
    /// Identifiant du service.
    pub id: String,
    /// Version du service.
    pub version: String,
    /// Hash MIP du service.
    pub mip_hash: String,
}

/// Portée du Permis de circulation (JSON).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermisScope {
    /// Liste des services autorisés.
    #[serde(default)]
    pub allowed_services: Vec<String>,
    /// Restrictions géographiques.
    #[serde(default)]
    pub geo_restrictions: Vec<String>,
    /// Niveau de confiance accordé.
    #[serde(default)]
    pub trust_level: u8,
}

impl Default for PermisScope {
    fn default() -> Self {
        Self {
            allowed_services: vec!["*".to_string()],
            geo_restrictions: Vec::new(),
            trust_level: 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_err_roundtrip() {
        let payload = RegisterErrPayload::new(
            RegisterErrorCode::InvalidToken,
            Bytes::from_static(b"Token expired"),
        );
        let bytes = payload.to_bytes();
        assert!(!bytes.is_empty());
    }

    #[test]
    fn test_heartbeat_roundtrip() {
        let payload = HeartbeatPayload {
            session_id: [1u8; SESSION_ID_SIZE],
            timestamp: 1234567890,
        };
        let bytes = payload.to_bytes();
        let parsed = HeartbeatPayload::parse(bytes.as_ref()).unwrap();
        assert_eq!(parsed.session_id, payload.session_id);
        assert_eq!(parsed.timestamp, payload.timestamp);
    }

    #[test]
    fn test_service_manifest_json() {
        let manifest = ServiceManifest {
            services: vec![
                ServiceEntry {
                    id: "jayfestival".to_string(),
                    version: "1.0.0".to_string(),
                    mip_hash: "abc123".to_string(),
                },
            ],
        };
        let json = serde_json::to_string(&manifest).unwrap();
        let parsed: ServiceManifest = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.services.len(), 1);
    }
}
