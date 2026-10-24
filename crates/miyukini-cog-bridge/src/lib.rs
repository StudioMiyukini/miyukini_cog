//! @id     miyukini_cog_bridge_lib
//! @do     declarer_modules_bridge
//! @role   orchestration
//! @layer  core
//! @human  miyukini-cog-bridge — module « lib ».

//! Miyukini COG Bridge — connexion Android ↔ COG Host.
//!
//! Fournit une abstraction unifiée pour connecter Central Mobile au COG Host
//! (le PC desktop de l'utilisateur) :
//!
//! - **SSH Tunnel** : sécurité maximale, traverse NAT/firewalls, port forwarding automatique
//! - **LAN** (appareil appairé) : connexion directe HTTP en WiFi local, latence minimale
//!
//! Authentification sans secret partagé (sortie du lot 2) : passeport d'appareil,
//! visa et preuve de possession ([`identite`]). Le mode « clé API » (jeton au
//! porteur scanné en QR) est retiré.
//!
//! ## Principe de souveraineté
//!
//! Aucune donnée n'est stockée sur le téléphone. Toutes les requêtes passent par
//! le bridge vers le COG Host. Les données restent sur la machine de l'utilisateur.

pub mod bridge;
pub mod discovery;
pub mod e2e;
pub mod identite;
pub mod pairing;
pub mod reconnect;
pub mod ssh;
pub mod whitelist;

pub use bridge::{BridgeStatus, CogBridge, ConnectionMode};
pub use identite::{ClientAppareil, EntetesAuth};
pub use e2e::{
    parse_identity_key_hex, E2eError, E2eHandshake, E2eIdentity, E2eRole, E2eSession,
    EncryptedMessage, PendingHandshake, RequestBinding, E2E_PROTOCOL_VERSION, E2E_SESSION_HEADER,
};
pub use pairing::{
    format_sas_code, identity_fingerprint, identity_fingerprint_hex, normalize_sas_code,
    pairing_commitment, pairing_sas_code, parse_commitment_hex, parse_pairing_signature_hex,
    sas_codes_equal, sign_pairing_commit, sign_pairing_reveal, verify_pairing_commit_signature,
    verify_pairing_commitment, verify_pairing_reveal_signature, DevicePairing, PairingNonce,
    RevealedDevicePairing, PAIRING_NONCE_SIZE, PAIRING_PROTOCOL_VERSION, PAIRING_SIGNATURE_SIZE,
    SAS_DIGITS,
};

/// Résultat standard du bridge.
pub type BridgeResult<T> = Result<T, BridgeError>;

/// Erreurs du bridge.
#[derive(Debug, thiserror::Error)]
pub enum BridgeError {
    #[error("Connexion refusée par le COG Host")]
    ConnectionRefused,

    #[error("COG Host introuvable sur le réseau")]
    HostNotFound,

    #[error("Authentification échouée: {0}")]
    AuthFailed(String),

    #[error("Tunnel SSH: {0}")]
    SshError(String),

    #[error("Réseau: {0}")]
    NetworkError(String),

    #[error("Timeout de connexion")]
    Timeout,

    #[error("Bridge déconnecté")]
    Disconnected,

    #[error("Appareil non autorisé (non appairé ou révoqué)")]
    DeviceNotWhitelisted,

    #[error("E2E: {0}")]
    E2e(#[from] e2e::E2eError),
}
