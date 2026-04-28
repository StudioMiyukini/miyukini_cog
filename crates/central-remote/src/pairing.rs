//! Appairage mobile — gestion des devices autorisés et tokens API.
//!
//! Endpoints pour permettre à Central Mobile (Android) de se connecter
//! au COG Host via 3 modes : Whitelist LAN, Clé API, ou SSH.

use chacha20poly1305::aead::{Aead, KeyInit};
use chacha20poly1305::{ChaCha20Poly1305, Nonce};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use x25519_dalek::{EphemeralSecret, PublicKey};

/// Gestionnaire d'appairage mobile.
pub struct PairingManager {
    /// Devices whitelistés (device_id → infos).
    whitelist: Mutex<HashMap<String, PairedDevice>>,
    /// Tokens API actifs (token → infos).
    api_tokens: Mutex<HashMap<String, ApiToken>>,
    /// Devices en attente de validation (device_id → infos).
    pending: Mutex<HashMap<String, PendingDevice>>,
    /// Sessions E2E actives (session_id → cipher + compteur).
    e2e_sessions: Mutex<HashMap<String, E2eServerSession>>,
}

/// Session E2E côté serveur (COG Host).
pub struct E2eServerSession {
    pub cipher: ChaCha20Poly1305,
    pub send_counter: u64,
    pub created_at: u64,
}

/// Requête de handshake E2E.
#[derive(Debug, Serialize, Deserialize)]
pub struct E2eHandshakeRequest {
    pub public_key: String,
    pub protocol_version: u8,
}

/// Réponse de handshake E2E.
#[derive(Debug, Serialize, Deserialize)]
pub struct E2eHandshakeResponse {
    pub public_key: String,
    pub session_id: String,
    pub protocol_version: u8,
}

/// Device appairé et autorisé.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairedDevice {
    pub device_id: String,
    pub name: String,
    pub paired_at: u64,
    pub last_seen: u64,
    pub mode: String,
}

/// Token API pour un device mobile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiToken {
    pub token: String,
    pub device_name: String,
    pub created_at: u64,
    /// 0 = pas d'expiration.
    pub expires_at: u64,
}

/// Device en attente de validation par l'utilisateur desktop.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingDevice {
    pub device_id: String,
    pub name: String,
    pub requested_at: u64,
}

/// Requête de validation whitelist.
#[derive(Debug, Deserialize)]
pub struct WhitelistValidateRequest {
    pub device_id: String,
}

/// Requête d'appairage initial.
#[derive(Debug, Deserialize)]
pub struct PairRequest {
    pub device_id: String,
    pub device_name: String,
    pub mode: String,
}

/// Réponse d'appairage.
#[derive(Debug, Serialize)]
pub struct PairResponse {
    pub status: String,
    pub token: Option<String>,
    pub message: String,
}

/// Info de découverte LAN.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryInfo {
    pub host_name: String,
    pub remote_port: u16,
    pub bridge_port: u16,
    pub version: String,
}

impl PairingManager {
    pub fn new() -> Self {
        Self {
            whitelist: Mutex::new(HashMap::new()),
            api_tokens: Mutex::new(HashMap::new()),
            pending: Mutex::new(HashMap::new()),
            e2e_sessions: Mutex::new(HashMap::new()),
        }
    }

    /// Vérifie si un device est whitelisté.
    pub fn is_whitelisted(&self, device_id: &str) -> bool {
        self.whitelist
            .lock()
            .map(|w| w.contains_key(device_id))
            .unwrap_or(false)
    }

    /// Met à jour le last_seen d'un device whitelisté.
    pub fn touch_device(&self, device_id: &str) {
        if let Ok(mut whitelist) = self.whitelist.lock() {
            if let Some(device) = whitelist.get_mut(device_id) {
                device.last_seen = now_secs();
            }
        }
    }

    /// Valide un token API.
    pub fn validate_api_token(&self, token: &str) -> bool {
        self.api_tokens
            .lock()
            .map(|tokens| {
                tokens.get(token).map_or(false, |t| {
                    t.expires_at == 0 || t.expires_at > now_secs()
                })
            })
            .unwrap_or(false)
    }

    /// Génère un nouveau token API (affiché en QR code sur le desktop).
    pub fn generate_api_token(&self, device_name: &str) -> String {
        let token = format!("myk_{}", Uuid::new_v4().as_simple());
        let api_token = ApiToken {
            token: token.clone(),
            device_name: device_name.to_string(),
            created_at: now_secs(),
            expires_at: 0, // pas d'expiration
        };
        if let Ok(mut tokens) = self.api_tokens.lock() {
            tokens.insert(token.clone(), api_token);
        }
        token
    }

    /// Ajoute un device en attente de validation.
    pub fn request_pairing(&self, device_id: &str, device_name: &str) -> PairResponse {
        // Si déjà whitelisté, accepter directement.
        if self.is_whitelisted(device_id) {
            self.touch_device(device_id);
            return PairResponse {
                status: "accepted".into(),
                token: None,
                message: "Device déjà autorisé".into(),
            };
        }

        // Ajouter en attente.
        if let Ok(mut pending) = self.pending.lock() {
            pending.insert(
                device_id.to_string(),
                PendingDevice {
                    device_id: device_id.to_string(),
                    name: device_name.to_string(),
                    requested_at: now_secs(),
                },
            );
        }

        PairResponse {
            status: "pending".into(),
            token: None,
            message: "En attente de validation sur le COG Host".into(),
        }
    }

    /// Approuve un device en attente (appelé par l'utilisateur desktop).
    pub fn approve_device(&self, device_id: &str) -> bool {
        let pending_device = self
            .pending
            .lock()
            .ok()
            .and_then(|mut p| p.remove(device_id));

        if let Some(device) = pending_device {
            let paired = PairedDevice {
                device_id: device.device_id.clone(),
                name: device.name,
                paired_at: now_secs(),
                last_seen: now_secs(),
                mode: "whitelist".into(),
            };
            if let Ok(mut whitelist) = self.whitelist.lock() {
                whitelist.insert(device.device_id, paired);
            }
            true
        } else {
            false
        }
    }

    /// Refuse un device en attente.
    pub fn reject_device(&self, device_id: &str) -> bool {
        self.pending
            .lock()
            .ok()
            .map(|mut p| p.remove(device_id).is_some())
            .unwrap_or(false)
    }

    /// Liste les devices en attente de validation.
    pub fn pending_devices(&self) -> Vec<PendingDevice> {
        self.pending
            .lock()
            .map(|p| p.values().cloned().collect())
            .unwrap_or_default()
    }

    /// Liste les devices appairés.
    pub fn paired_devices(&self) -> Vec<PairedDevice> {
        self.whitelist
            .lock()
            .map(|w| w.values().cloned().collect())
            .unwrap_or_default()
    }

    /// Révoque un device appairé.
    pub fn revoke_device(&self, device_id: &str) -> bool {
        self.whitelist
            .lock()
            .ok()
            .map(|mut w| w.remove(device_id).is_some())
            .unwrap_or(false)
    }

    /// Révoque un token API.
    pub fn revoke_api_token(&self, token: &str) -> bool {
        self.api_tokens
            .lock()
            .ok()
            .map(|mut t| t.remove(token).is_some())
            .unwrap_or(false)
    }

    /// Info de découverte pour le broadcast LAN.
    pub fn discovery_info(&self) -> DiscoveryInfo {
        DiscoveryInfo {
            host_name: hostname(),
            remote_port: 8091,
            bridge_port: 8091,
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

impl Default for PairingManager {
    fn default() -> Self {
        Self::new()
    }
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn hostname() -> String {
    std::env::var("COMPUTERNAME")
        .or_else(|_| std::env::var("HOSTNAME"))
        .unwrap_or_else(|_| "COG-Host".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn whitelist_flow() {
        let mgr = PairingManager::new();

        // Pas encore whitelisté.
        assert!(!mgr.is_whitelisted("dev-1"));

        // Demande d'appairage.
        let resp = mgr.request_pairing("dev-1", "Pixel 7");
        assert_eq!(resp.status, "pending");

        // Approuver.
        assert!(mgr.approve_device("dev-1"));
        assert!(mgr.is_whitelisted("dev-1"));

        // Re-connexion directe.
        let resp2 = mgr.request_pairing("dev-1", "Pixel 7");
        assert_eq!(resp2.status, "accepted");
    }

    #[test]
    fn api_token_flow() {
        let mgr = PairingManager::new();

        let token = mgr.generate_api_token("Mon Telephone");
        assert!(token.starts_with("myk_"));
        assert!(mgr.validate_api_token(&token));
        assert!(!mgr.validate_api_token("bogus"));

        mgr.revoke_api_token(&token);
        assert!(!mgr.validate_api_token(&token));
    }

    #[test]
    fn reject_device() {
        let mgr = PairingManager::new();
        mgr.request_pairing("dev-2", "Samsung S24");
        assert_eq!(mgr.pending_devices().len(), 1);

        mgr.reject_device("dev-2");
        assert!(mgr.pending_devices().is_empty());
        assert!(!mgr.is_whitelisted("dev-2"));
    }
}
