//! Types fondamentaux du protocole MWS Relay.
//!
//! Définit les énumérations et structures de base utilisées dans
//! le protocole relay binaire (version 1).

use serde::{Deserialize, Serialize};

/// Version actuelle du protocole MWS Relay.
pub const PROTOCOL_VERSION: u8 = 1;

/// Port par défaut du relay Origin.
pub const DEFAULT_RELAY_PORT: u16 = 7000;

/// Port par défaut du tracker Origin.
pub const DEFAULT_TRACKER_PORT: u16 = 21000;

/// Taille maximale du cog_id.
pub const MAX_COG_ID_LEN: usize = 256;

/// Taille maximale du token.
pub const MAX_TOKEN_LEN: usize = 512;

/// Taille maximale du manifest de services.
pub const MAX_SVC_MANIFEST_LEN: usize = 4096;

/// Taille maximale du payload DATA.
pub const MAX_DATA_PAYLOAD_LEN: usize = 65536; // 64 Ko

/// Taille d'un nonce (protection anti-rejeu).
pub const NONCE_SIZE: usize = 16;

/// Taille d'un session_id.
pub const SESSION_ID_SIZE: usize = 16;

/// Taille d'une signature Ed25519.
pub const ED25519_SIGNATURE_SIZE: usize = 64;

/// Taille d'un HMAC-SHA256.
pub const HMAC_SHA256_SIZE: usize = 32;

/// Taille d'un hash SHA-256.
pub const SHA256_SIZE: usize = 32;

// ═══════════════════════════════════════════════════════════════════════════
// Types de messages du protocole
// ═══════════════════════════════════════════════════════════════════════════

/// Type de message dans le protocole relay.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum MessageType {
    // Messages principaux
    /// Enregistrement du tunnel (COG → Relay)
    Register = 0x01,
    /// Enregistrement réussi (Relay → COG)
    RegisterOk = 0x02,
    /// Refus d'enregistrement (Relay → COG)
    RegisterErr = 0x03,
    /// Demande de connexion vers un cog_id (Client → Relay)
    Connect = 0x04,
    /// Tunnel établi (Relay → Client)
    ConnectOk = 0x05,
    /// Refus de connexion (Relay → Client)
    ConnectErr = 0x06,
    /// Données opaques (bidirectionnel)
    Data = 0x07,
    /// Garde la connexion vivante (bidirectionnel)
    Heartbeat = 0x08,
    /// Accusé de HEARTBEAT (réponse)
    HeartbeatAck = 0x09,
    /// Fermeture propre (bidirectionnel)
    Close = 0x0A,
    /// Erreur protocolaire (bidirectionnel)
    Error = 0x0B,

    // Messages du Registre
    /// Interrogation du Registre de Services (COG → Relay)
    RegistryQuery = 0x0C,
    /// Réponse du Registre (Relay → COG)
    RegistryResponse = 0x0D,
    /// Notification de mise à jour (Relay → COG)
    UpdateAvailable = 0x0E,

    // Messages de vérification
    /// Clé de conformité des Cores - Phase A (COG → Relay)
    CoreKey = 0x10,
    /// Bloc de code MIP d'un Service - Phase B (COG → Relay)
    ServiceBlock = 0x11,
    /// Résultat d'une phase de vérification (Relay → COG)
    VerifyResult = 0x12,
    /// Redirection vers un autre relay (Relay/Origin → COG)
    Redirect = 0x13,

    // Messages de contrôle
    /// Révocation d'un Permis de circulation (Relay/Origin → Tracker) - R-009
    PermitRevoke = 0x30,
}

impl MessageType {
    /// Convertit un octet en `MessageType`.
    #[must_use]
    pub fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            0x01 => Some(Self::Register),
            0x02 => Some(Self::RegisterOk),
            0x03 => Some(Self::RegisterErr),
            0x04 => Some(Self::Connect),
            0x05 => Some(Self::ConnectOk),
            0x06 => Some(Self::ConnectErr),
            0x07 => Some(Self::Data),
            0x08 => Some(Self::Heartbeat),
            0x09 => Some(Self::HeartbeatAck),
            0x0A => Some(Self::Close),
            0x0B => Some(Self::Error),
            0x0C => Some(Self::RegistryQuery),
            0x0D => Some(Self::RegistryResponse),
            0x0E => Some(Self::UpdateAvailable),
            0x10 => Some(Self::CoreKey),
            0x11 => Some(Self::ServiceBlock),
            0x12 => Some(Self::VerifyResult),
            0x13 => Some(Self::Redirect),
            0x30 => Some(Self::PermitRevoke),
            _ => None,
        }
    }

    /// Convertit le `MessageType` en octet.
    #[must_use]
    pub const fn to_byte(self) -> u8 {
        self as u8
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Types de COG
// ═══════════════════════════════════════════════════════════════════════════

/// Type de COG dans le réseau MWS.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum CogType {
    /// Point central de vérité (un seul par réseau)
    Origin = 0x00,
    /// COG de contrôle d'intégrité
    Relay = 0x01,
    /// Mapping et contrôle
    Tracker = 0x02,
    /// COG d'utilisateur commun
    Stable = 0x03,
    /// COG professionnel à forte utilisation
    Special = 0x04,
    /// COG mobile enfant d'un STABLE
    Terminal = 0x05,
    /// COG isolé volontairement
    Lone = 0x06,
}

impl CogType {
    /// Convertit un octet en `CogType`.
    #[must_use]
    pub fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            0x00 => Some(Self::Origin),
            0x01 => Some(Self::Relay),
            0x02 => Some(Self::Tracker),
            0x03 => Some(Self::Stable),
            0x04 => Some(Self::Special),
            0x05 => Some(Self::Terminal),
            0x06 => Some(Self::Lone),
            _ => None,
        }
    }

    /// Convertit le `CogType` en octet.
    #[must_use]
    pub const fn to_byte(self) -> u8 {
        self as u8
    }

    /// Vérifie si ce type de COG peut s'enregistrer.
    #[must_use]
    pub const fn can_register(self) -> bool {
        !matches!(self, Self::Lone)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Types d'OS
// ═══════════════════════════════════════════════════════════════════════════

/// Type d'OS du COG.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum OsType {
    /// Microsoft Windows
    Windows = 0x00,
    /// Distributions Linux
    Linux = 0x01,
    /// Apple macOS
    MacOs = 0x02,
    /// Google Android
    Android = 0x03,
    /// Apple iOS
    Ios = 0x04,
}

impl OsType {
    /// Convertit un octet en `OsType`.
    #[must_use]
    pub fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            0x00 => Some(Self::Windows),
            0x01 => Some(Self::Linux),
            0x02 => Some(Self::MacOs),
            0x03 => Some(Self::Android),
            0x04 => Some(Self::Ios),
            _ => None,
        }
    }

    /// Convertit l'`OsType` en octet.
    #[must_use]
    pub const fn to_byte(self) -> u8 {
        self as u8
    }

    /// Détecte l'OS courant.
    #[must_use]
    pub fn current() -> Self {
        #[cfg(target_os = "windows")]
        return Self::Windows;
        #[cfg(target_os = "linux")]
        return Self::Linux;
        #[cfg(target_os = "macos")]
        return Self::MacOs;
        #[cfg(target_os = "android")]
        return Self::Android;
        #[cfg(target_os = "ios")]
        return Self::Ios;
        #[cfg(not(any(
            target_os = "windows",
            target_os = "linux",
            target_os = "macos",
            target_os = "android",
            target_os = "ios"
        )))]
        return Self::Linux; // Fallback
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Types de Passeport
// ═══════════════════════════════════════════════════════════════════════════

/// Type de Passeport COG.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum PassportType {
    /// Passeport standard
    Standard = 0x00,
    /// Passeport spécial (professionnel, clé spéciale requise)
    Special = 0x01,
}

impl PassportType {
    /// Convertit un octet en `PassportType`.
    #[must_use]
    pub fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            0x00 => Some(Self::Standard),
            0x01 => Some(Self::Special),
            _ => None,
        }
    }

    /// Convertit le `PassportType` en octet.
    #[must_use]
    pub const fn to_byte(self) -> u8 {
        self as u8
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Codes d'erreur
// ═══════════════════════════════════════════════════════════════════════════

/// Codes d'erreur pour REGISTER_ERR.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum RegisterErrorCode {
    /// Token invalide
    InvalidToken = 1,
    /// cog_id déjà enregistré
    CogIdConflict = 2,
    /// Version de protocole non supportée
    UnsupportedProtocolVersion = 3,
    /// Échec d'authentification
    AuthFailed = 4,
    /// Rate limiting
    RateLimited = 5,
    /// Erreur interne
    InternalError = 6,
    /// Version des Cores incompatible
    IncompatibleCoreVersion = 7,
    /// Service non répertorié
    UnregisteredService = 8,
    /// Clé de conformité Cores incorrecte
    CoreKeyMismatch = 9,
    /// Bloc de code Service incorrect
    ServiceBlockMismatch = 10,
    /// Santé de l'environnement non conforme
    EnvironmentHealthFailed = 11,
    /// COG mis en quarantaine
    Quarantine = 12,
    /// COG blacklisté
    Blacklisted = 13,
    /// Redirection vers un autre relay
    Redirect = 14,
    /// Clé spéciale invalide
    SpecialKeyInvalid = 15,
}

impl RegisterErrorCode {
    /// Convertit un octet en `RegisterErrorCode`.
    #[must_use]
    pub fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            1 => Some(Self::InvalidToken),
            2 => Some(Self::CogIdConflict),
            3 => Some(Self::UnsupportedProtocolVersion),
            4 => Some(Self::AuthFailed),
            5 => Some(Self::RateLimited),
            6 => Some(Self::InternalError),
            7 => Some(Self::IncompatibleCoreVersion),
            8 => Some(Self::UnregisteredService),
            9 => Some(Self::CoreKeyMismatch),
            10 => Some(Self::ServiceBlockMismatch),
            11 => Some(Self::EnvironmentHealthFailed),
            12 => Some(Self::Quarantine),
            13 => Some(Self::Blacklisted),
            14 => Some(Self::Redirect),
            15 => Some(Self::SpecialKeyInvalid),
            _ => None,
        }
    }

    /// Convertit le code en octet.
    #[must_use]
    pub const fn to_byte(self) -> u8 {
        self as u8
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Phases de vérification
// ═══════════════════════════════════════════════════════════════════════════

/// Phase de vérification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum VerifyPhase {
    /// Phase A : Clé Cores
    PhaseA = 1,
    /// Phase B : Blocs Services
    PhaseB = 2,
    /// Phase C : Santé environnement
    PhaseC = 3,
}

impl VerifyPhase {
    /// Convertit un octet en `VerifyPhase`.
    #[must_use]
    pub fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            1 => Some(Self::PhaseA),
            2 => Some(Self::PhaseB),
            3 => Some(Self::PhaseC),
            _ => None,
        }
    }

    /// Convertit la phase en octet.
    #[must_use]
    pub const fn to_byte(self) -> u8 {
        self as u8
    }
}

/// Résultat d'une phase de vérification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum VerifyResult {
    /// Vérification réussie
    Ok = 0,
    /// Vérification échouée
    Fail = 1,
    /// Vérification étendue requise
    ExtendedRequired = 2,
}

impl VerifyResult {
    /// Convertit un octet en `VerifyResult`.
    #[must_use]
    pub fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            0 => Some(Self::Ok),
            1 => Some(Self::Fail),
            2 => Some(Self::ExtendedRequired),
            _ => None,
        }
    }

    /// Convertit le résultat en octet.
    #[must_use]
    pub const fn to_byte(self) -> u8 {
        self as u8
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Raisons de fermeture
// ═══════════════════════════════════════════════════════════════════════════

/// Raison de fermeture de connexion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum CloseReason {
    /// Fermeture normale
    Normal = 0,
    /// Timeout
    Timeout = 1,
    /// Erreur de protocole
    ProtocolError = 2,
    /// Révocation du Permis
    PermitRevoked = 3,
    /// Maintenance
    Maintenance = 4,
    /// Politique
    Policy = 5,
}

impl CloseReason {
    /// Convertit un octet en `CloseReason`.
    #[must_use]
    pub fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            0 => Some(Self::Normal),
            1 => Some(Self::Timeout),
            2 => Some(Self::ProtocolError),
            3 => Some(Self::PermitRevoked),
            4 => Some(Self::Maintenance),
            5 => Some(Self::Policy),
            _ => None,
        }
    }

    /// Convertit la raison en octet.
    #[must_use]
    pub const fn to_byte(self) -> u8 {
        self as u8
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Raisons de redirection
// ═══════════════════════════════════════════════════════════════════════════

/// Raison de redirection vers un autre relay.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum RedirectReason {
    /// Origin saturé
    Saturated = 0,
    /// Maintenance
    Maintenance = 1,
    /// Politique
    Policy = 2,
}

impl RedirectReason {
    /// Convertit un octet en `RedirectReason`.
    #[must_use]
    pub fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            0 => Some(Self::Saturated),
            1 => Some(Self::Maintenance),
            2 => Some(Self::Policy),
            _ => None,
        }
    }

    /// Convertit la raison en octet.
    #[must_use]
    pub const fn to_byte(self) -> u8 {
        self as u8
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Sévérité des mises à jour
// ═══════════════════════════════════════════════════════════════════════════

/// Sévérité d'une mise à jour disponible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum UpdateSeverity {
    /// Optionnelle
    Optional = 0,
    /// Recommandée
    Recommended = 1,
    /// Critique
    Critical = 2,
}

impl UpdateSeverity {
    /// Convertit un octet en `UpdateSeverity`.
    #[must_use]
    pub fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            0 => Some(Self::Optional),
            1 => Some(Self::Recommended),
            2 => Some(Self::Critical),
            _ => None,
        }
    }

    /// Convertit la sévérité en octet.
    #[must_use]
    pub const fn to_byte(self) -> u8 {
        self as u8
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Types de requête Registre
// ═══════════════════════════════════════════════════════════════════════════

/// Type de requête au Registre de Services.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum RegistryQueryType {
    /// Vérifier un service
    VerifyService = 1,
    /// Lister les mises à jour
    ListUpdates = 2,
}

impl RegistryQueryType {
    /// Convertit un octet en `RegistryQueryType`.
    #[must_use]
    pub fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            1 => Some(Self::VerifyService),
            2 => Some(Self::ListUpdates),
            _ => None,
        }
    }

    /// Convertit le type en octet.
    #[must_use]
    pub const fn to_byte(self) -> u8 {
        self as u8
    }
}

/// Statut d'un service dans le Registre.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum RegistryStatus {
    /// Service trouvé
    Found = 0,
    /// Service non trouvé
    NotFound = 1,
    /// Service suspendu
    Suspended = 2,
}

impl RegistryStatus {
    /// Convertit un octet en `RegistryStatus`.
    #[must_use]
    pub fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            0 => Some(Self::Found),
            1 => Some(Self::NotFound),
            2 => Some(Self::Suspended),
            _ => None,
        }
    }

    /// Convertit le statut en octet.
    #[must_use]
    pub const fn to_byte(self) -> u8 {
        self as u8
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Flags de trame
// ═══════════════════════════════════════════════════════════════════════════

/// Flags de trame du protocole.
#[derive(Debug, Clone, Copy, Default)]
pub struct FrameFlags(u8);

impl FrameFlags {
    /// Flag : direction (0 = vers relay, 1 = depuis relay)
    pub const DIRECTION: u8 = 0b0000_0001;
    /// Flag : fin de flux
    pub const END_OF_STREAM: u8 = 0b0000_0010;
    /// Flag : payload étendu (4 octets au lieu de 2)
    pub const EXTENDED_LENGTH: u8 = 0b0000_0100;

    /// Crée des flags vides.
    #[must_use]
    pub const fn new() -> Self {
        Self(0)
    }

    /// Crée des flags depuis un octet.
    #[must_use]
    pub const fn from_byte(byte: u8) -> Self {
        Self(byte)
    }

    /// Convertit en octet.
    #[must_use]
    pub const fn to_byte(self) -> u8 {
        self.0
    }

    /// Définit un flag.
    #[must_use]
    pub const fn with_flag(self, flag: u8) -> Self {
        Self(self.0 | flag)
    }

    /// Vérifie si un flag est présent.
    #[must_use]
    pub const fn has_flag(self, flag: u8) -> bool {
        (self.0 & flag) != 0
    }

    /// Vérifie si le payload a une longueur étendue (4 octets).
    #[must_use]
    pub const fn is_extended_length(self) -> bool {
        self.has_flag(Self::EXTENDED_LENGTH)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_type_roundtrip() {
        for byte in 0x01..=0x13 {
            if let Some(mt) = MessageType::from_byte(byte) {
                assert_eq!(mt.to_byte(), byte);
            }
        }
    }

    #[test]
    fn test_cog_type_roundtrip() {
        for byte in 0x00..=0x06 {
            if let Some(ct) = CogType::from_byte(byte) {
                assert_eq!(ct.to_byte(), byte);
            }
        }
    }

    #[test]
    fn test_cog_type_can_register() {
        assert!(CogType::Stable.can_register());
        assert!(CogType::Origin.can_register());
        assert!(!CogType::Lone.can_register());
    }

    #[test]
    fn test_frame_flags() {
        let flags = FrameFlags::new()
            .with_flag(FrameFlags::DIRECTION)
            .with_flag(FrameFlags::EXTENDED_LENGTH);

        assert!(flags.has_flag(FrameFlags::DIRECTION));
        assert!(flags.has_flag(FrameFlags::EXTENDED_LENGTH));
        assert!(!flags.has_flag(FrameFlags::END_OF_STREAM));
        assert!(flags.is_extended_length());
    }
}
