//! @id     central_remote_pairing
//! @do     definir_pairing_manager
//! @role   orchestration
//! @layer  core
//! @human  central-remote — module « pairing ».

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

/// Durée de validité par défaut d'un token API (30 jours).
pub const DEFAULT_API_TOKEN_TTL_SECS: u64 = 30 * 24 * 3600;

/// Gestionnaire d'appairage mobile.
pub struct PairingManager {
    /// Durée de validité des tokens API générés (secondes, jamais 0).
    api_token_ttl_secs: u64,
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
    /// Expiration (secondes UNIX). Un token à 0 est considéré comme invalide.
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
        Self::with_token_ttl(DEFAULT_API_TOKEN_TTL_SECS)
    }

    /// Crée un gestionnaire avec une durée de vie de token API donnée
    /// (0 → valeur par défaut de 30 jours).
    pub fn with_token_ttl(ttl_secs: u64) -> Self {
        Self {
            api_token_ttl_secs: if ttl_secs == 0 { DEFAULT_API_TOKEN_TTL_SECS } else { ttl_secs },
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
                // Fail-closed : un token sans expiration (0) n'est jamais valide.
                tokens
                    .get(token)
                    .map_or(false, |t| t.expires_at != 0 && t.expires_at > now_secs())
            })
            .unwrap_or(false)
    }

    /// Génère un nouveau token API (affiché en QR code sur le desktop).
    /// Le token expire après la durée configurée (30 jours par défaut).
    pub fn generate_api_token(&self, device_name: &str) -> String {
        self.generate_api_token_with_ttl(device_name, self.api_token_ttl_secs)
    }

    /// Génère un token API avec une durée de vie explicite (secondes, min. 1).
    pub fn generate_api_token_with_ttl(&self, device_name: &str, ttl_secs: u64) -> String {
        let token = format!("myk_{}", Uuid::new_v4().as_simple());
        let now = now_secs();
        let api_token = ApiToken {
            token: token.clone(),
            device_name: device_name.to_string(),
            created_at: now,
            expires_at: now.saturating_add(ttl_secs.max(1)),
        };
        if let Ok(mut tokens) = self.api_tokens.lock() {
            // Purge des tokens expirés.
            tokens.retain(|_, t| t.expires_at > now);
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

    /// Liste les devices appairés (identifiants complets : usage Rust local uniquement).
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

/// Masque un identifiant de device pour toute exposition hors processus
/// (ex. `abc…yz`). Les identifiants courts sont entièrement masqués.
pub fn mask_device_id(id: &str) -> String {
    let chars: Vec<char> = id.chars().collect();
    if chars.len() <= 8 {
        return "***".into();
    }
    let head: String = chars[..3].iter().collect();
    let tail: String = chars[chars.len() - 2..].iter().collect();
    format!("{head}…{tail}")
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
    fn api_token_has_expiry_by_default() {
        let mgr = PairingManager::new();
        let token = mgr.generate_api_token("Tel");
        let exp = mgr.api_tokens.lock().unwrap().get(&token).unwrap().expires_at;
        assert_ne!(exp, 0);
        let delta = exp - now_secs();
        assert!(delta > 29 * 24 * 3600 && delta <= DEFAULT_API_TOKEN_TTL_SECS);
    }

    #[test]
    fn api_token_expires() {
        let mgr = PairingManager::with_token_ttl(3600);
        let token = mgr.generate_api_token("Tel");
        assert!(mgr.validate_api_token(&token));
        // Simule l'écoulement du temps : expiration dans le passé.
        mgr.api_tokens.lock().unwrap().get_mut(&token).unwrap().expires_at = now_secs() - 1;
        assert!(!mgr.validate_api_token(&token));
        // Un token sans expiration (0) est refusé (fail-closed).
        mgr.api_tokens.lock().unwrap().get_mut(&token).unwrap().expires_at = 0;
        assert!(!mgr.validate_api_token(&token));
    }

    #[test]
    fn device_id_masked() {
        assert_eq!(mask_device_id("abcdef-1234-uvwxyz"), "abc…yz");
        assert_eq!(mask_device_id("short"), "***");
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
