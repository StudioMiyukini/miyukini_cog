//! @id     miyukini_cog_bridge_lib
//! @do     declarer_module_apikey
//! @role   orchestration
//! @layer  core
//! @human  miyukini-cog-bridge — module « lib ».

//! Miyukini COG Bridge — connexion Android ↔ COG Host.
//!
//! Fournit une abstraction unifiée pour connecter Central Mobile au COG Host
//! (le PC desktop de l'utilisateur) via 3 modes de connexion :
//!
//! - **SSH Tunnel** : sécurité maximale, traverse NAT/firewalls, port forwarding automatique
//! - **Whitelist LAN** : connexion directe HTTP en WiFi local, latence minimale
//! - **Clé API** : token Bearer signé (scanné via QR code), simple et universel
//!
//! ## Principe de souveraineté
//!
//! Aucune donnée n'est stockée sur le téléphone. Toutes les requêtes passent par
//! le bridge vers le COG Host. Les données restent sur la machine de l'utilisateur.

pub mod apikey;
pub mod bridge;
pub mod discovery;
pub mod e2e;
pub mod reconnect;
pub mod ssh;
pub mod whitelist;

pub use bridge::{BridgeStatus, CogBridge, ConnectionMode};
pub use e2e::{
    pairing_confirmation_code, parse_identity_key_hex, E2eError, E2eHandshake, E2eIdentity,
    E2eRole, E2eSession, EncryptedMessage, PendingHandshake, RequestBinding,
    E2E_PROTOCOL_VERSION, E2E_SESSION_HEADER,
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

    #[error("Clé API invalide ou expirée")]
    InvalidApiKey,

    #[error("Device non autorisé (whitelist)")]
    DeviceNotWhitelisted,

    #[error("E2E: {0}")]
    E2e(#[from] e2e::E2eError),
}
