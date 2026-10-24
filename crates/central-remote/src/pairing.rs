//! @id     central_remote_pairing
//! @do     definir_pairing_manager
//! @role   orchestration
//! @layer  core
//! @human  central-remote — module « pairing ».

//! Appairage mobile — gestion des devices autorisés et tokens API.
//!
//! Endpoints pour permettre à Central Mobile (Android) de se connecter
//! au COG Host via 3 modes : Whitelist LAN, Clé API, ou SSH.

//!
//! ## Clés E2E (R-023)
//!
//! L'appairage échange les clés publiques d'identité ed25519 : l'appareil
//! envoie la sienne dans `/api/bridge/pair`, le COG la conserve avec l'entrée
//! de whitelist après approbation sur le desktop, et renvoie sa propre clé que
//! l'appareil épingle. Un code de confirmation à 6 chiffres dérivé des deux
//! clés est affiché sur l'appareil et exposé au desktop pour comparaison.
//! Le handshake E2E (`/api/bridge/e2e-handshake`) n'est accepté que pour une
//! clé présente dans la whitelist.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

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
}

/// Nombre maximal de demandes d'appairage en attente (C-13).
pub const MAX_PENDING_DEVICES: usize = 64;

/// Longueur maximale d'un `device_id` (octets, C-13).
pub const MAX_DEVICE_ID_LEN: usize = 128;

/// Longueur maximale d'un nom de device (caractères, C-13).
pub const MAX_DEVICE_NAME_CHARS: usize = 64;

/// Device appairé et autorisé.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairedDevice {
    pub device_id: String,
    pub name: String,
    pub paired_at: u64,
    pub last_seen: u64,
    pub mode: String,
    /// Clé publique d'identité E2E ed25519 de l'appareil (hex 32 octets),
    /// épinglée à l'approbation. Seule clé acceptée au handshake E2E.
    pub e2e_identity_key: String,
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
    /// Clé publique d'identité E2E présentée par l'appareil (hex 32 octets).
    pub e2e_identity_key: String,
    /// Code de confirmation (6 chiffres) à comparer avec celui de l'appareil
    /// avant d'approuver.
    pub confirmation_code: String,
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
    /// Clé publique d'identité E2E ed25519 de l'appareil (hex 32 octets).
    /// Obligatoire : un appareil sans clé ne peut pas être appairé.
    pub e2e_identity_key: String,
}

/// Réponse d'appairage.
#[derive(Debug, Serialize)]
pub struct PairResponse {
    pub status: String,
    pub token: Option<String>,
    pub message: String,
}

/// Erreurs de demande d'appairage (une variante par cause, C-11).
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum PairingError {
    #[error("device_id invalide")]
    InvalidDeviceId,
    #[error("trop de demandes d'appairage en attente")]
    TooManyPending,
    #[error("état d'appairage indisponible")]
    Unavailable,
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
        }
    }

    /// Vérifie si un device est whitelisté.
    pub fn is_whitelisted(&self, device_id: &str) -> bool {
        self.whitelist
            .lock()
            .is_ok_and(|w| w.contains_key(device_id))
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
            .is_ok_and(|tokens| {
                // Fail-closed : un token sans expiration (0) n'est jamais valide.
                tokens
                    .get(token)
                    .is_some_and(|t| t.expires_at != 0 && t.expires_at > now_secs())
            })
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

    /// Ajoute un device en attente de validation, avec sa clé publique
    /// d'identité E2E (déjà validée, hex canonique) et le code de confirmation.
    ///
    /// Un device déjà whitelisté avec la **même** clé est accepté directement ;
    /// avec une clé différente, la demande repasse en attente d'approbation
    /// (la clé épinglée n'est jamais remplacée sans validation desktop).
    pub fn request_pairing(
        &self,
        device_id: &str,
        device_name: &str,
        e2e_identity_key_hex: &str,
        confirmation_code: &str,
    ) -> Result<PairResponse, PairingError> {
        if device_id.is_empty()
            || device_id.len() > MAX_DEVICE_ID_LEN
            || device_id.chars().any(char::is_control)
        {
            return Err(PairingError::InvalidDeviceId);
        }
        let name: String = device_name
            .chars()
            .filter(|c| !c.is_control())
            .take(MAX_DEVICE_NAME_CHARS)
            .collect();

        let same_key = self
            .whitelist
            .lock()
            .map_err(|_| PairingError::Unavailable)?
            .get(device_id)
            .is_some_and(|d| d.e2e_identity_key == e2e_identity_key_hex);
        if same_key {
            self.touch_device(device_id);
            return Ok(PairResponse {
                status: "accepted".into(),
                token: None,
                message: "Device déjà autorisé".into(),
            });
        }

        let mut pending = self.pending.lock().map_err(|_| PairingError::Unavailable)?;
        if !pending.contains_key(device_id) && pending.len() >= MAX_PENDING_DEVICES {
            return Err(PairingError::TooManyPending);
        }
        pending.insert(
            device_id.to_string(),
            PendingDevice {
                device_id: device_id.to_string(),
                name,
                requested_at: now_secs(),
                e2e_identity_key: e2e_identity_key_hex.to_string(),
                confirmation_code: confirmation_code.to_string(),
            },
        );

        Ok(PairResponse {
            status: "pending".into(),
            token: None,
            message: "En attente de validation sur le COG Host".into(),
        })
    }

    /// Vrai si `device_id` est toujours appairé avec la clé E2E `e2e_identity_key_hex`
    /// (révocation ou changement de clé = sessions E2E existantes invalides).
    pub fn is_paired_with_key(&self, device_id: &str, e2e_identity_key_hex: &str) -> bool {
        !e2e_identity_key_hex.is_empty()
            && self
                .whitelist
                .lock()
                .is_ok_and(|w| {
                    w.get(device_id)
                        .is_some_and(|d| d.e2e_identity_key == e2e_identity_key_hex)
                })
    }

    /// Retrouve le device appairé dont la clé d'identité E2E épinglée vaut
    /// `e2e_identity_key_hex` (hex canonique). Aucun résultat = refus.
    pub fn device_for_identity_key(&self, e2e_identity_key_hex: &str) -> Option<PairedDevice> {
        if e2e_identity_key_hex.is_empty() {
            return None;
        }
        self.whitelist.lock().ok().and_then(|w| {
            w.values()
                .find(|d| d.e2e_identity_key == e2e_identity_key_hex)
                .cloned()
        })
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
                e2e_identity_key: device.e2e_identity_key,
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
            .is_ok_and(|mut p| p.remove(device_id).is_some())
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
            .is_ok_and(|mut w| w.remove(device_id).is_some())
    }

    /// Révoque un token API.
    pub fn revoke_api_token(&self, token: &str) -> bool {
        self.api_tokens
            .lock()
            .is_ok_and(|mut t| t.remove(token).is_some())
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

    const KEY_A: &str = "aa00000000000000000000000000000000000000000000000000000000000000";
    const KEY_B: &str = "bb00000000000000000000000000000000000000000000000000000000000000";

    fn request(mgr: &PairingManager, id: &str, name: &str, key: &str) -> PairResponse {
        mgr.request_pairing(id, name, key, "123456").unwrap()
    }

    #[test]
    fn whitelist_flow() {
        let mgr = PairingManager::new();

        // Pas encore whitelisté.
        assert!(!mgr.is_whitelisted("dev-1"));

        // Demande d'appairage.
        let resp = request(&mgr, "dev-1", "Pixel 7", KEY_A);
        assert_eq!(resp.status, "pending");
        assert!(mgr.device_for_identity_key(KEY_A).is_none());

        // Approuver : la clé E2E est épinglée avec l'entrée de whitelist.
        assert!(mgr.approve_device("dev-1"));
        assert!(mgr.is_whitelisted("dev-1"));
        let paired = mgr.device_for_identity_key(KEY_A).unwrap();
        assert_eq!(paired.device_id, "dev-1");
        assert_eq!(paired.e2e_identity_key, KEY_A);

        // Re-connexion directe avec la même clé.
        let resp2 = request(&mgr, "dev-1", "Pixel 7", KEY_A);
        assert_eq!(resp2.status, "accepted");
    }

    #[test]
    fn key_change_requires_new_approval() {
        let mgr = PairingManager::new();
        request(&mgr, "dev-1", "Pixel", KEY_A);
        assert!(mgr.approve_device("dev-1"));
        // Même device_id, autre clé : en attente, l'ancienne clé reste épinglée.
        let resp = request(&mgr, "dev-1", "Pixel", KEY_B);
        assert_eq!(resp.status, "pending");
        assert!(mgr.device_for_identity_key(KEY_B).is_none());
        assert!(mgr.device_for_identity_key(KEY_A).is_some());
        assert!(mgr.approve_device("dev-1"));
        assert!(mgr.device_for_identity_key(KEY_B).is_some());
        assert!(mgr.device_for_identity_key(KEY_A).is_none());
    }

    #[test]
    fn pairing_inputs_bounded() {
        let mgr = PairingManager::new();
        assert_eq!(
            mgr.request_pairing("", "x", KEY_A, "1").unwrap_err(),
            PairingError::InvalidDeviceId
        );
        assert_eq!(
            mgr.request_pairing(&"d".repeat(MAX_DEVICE_ID_LEN + 1), "x", KEY_A, "1")
                .unwrap_err(),
            PairingError::InvalidDeviceId
        );
        for i in 0..MAX_PENDING_DEVICES {
            request(&mgr, &format!("dev-{i}"), "x", KEY_A);
        }
        assert_eq!(
            mgr.request_pairing("dev-extra", "x", KEY_A, "1").unwrap_err(),
            PairingError::TooManyPending
        );
        let long = "n".repeat(500);
        request(&mgr, "dev-0", &long, KEY_A);
        let p = mgr.pending_devices().into_iter().find(|d| d.device_id == "dev-0").unwrap();
        assert_eq!(p.name.chars().count(), MAX_DEVICE_NAME_CHARS);
        assert!(mgr.device_for_identity_key("").is_none());
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
        request(&mgr, "dev-2", "Samsung S24", KEY_A);
        assert_eq!(mgr.pending_devices().len(), 1);

        mgr.reject_device("dev-2");
        assert!(mgr.pending_devices().is_empty());
        assert!(!mgr.is_whitelisted("dev-2"));
    }
}
