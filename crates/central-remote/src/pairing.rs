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
//! ## Appairage v2 : engagement puis révélation (R-026)
//!
//! Voir `miyukini_cog_bridge::pairing` pour le protocole et ses garanties.
//! Machine à états d'une demande (une entrée par `device_id`) :
//!
//! ```text
//! Committed ──reveal(Nd) valide──▶ Revealed ──approve(code affiché)──▶ Approved (liste blanche)
//!     │                               │
//!     ├──reveal(Nd) invalide──▶ Rejected (entrée supprimée)
//!     ├──reject (desktop)──────────▶ Rejected
//!     └──délai dépassé─────────────▶ Expired (purge à chaque opération)
//! ```
//!
//! - Le COG n'accepte la révélation que pour une entrée existante, donc
//!   **après** avoir émis `Nc` ; une seule révélation par entrée.
//! - Une entrée en attente n'est **jamais** remplacée par une autre clé (R-027) :
//!   l'appareil attend l'expiration ou le refus explicite. Le même appareil
//!   avec la **même** clé peut relancer sa demande (nouveaux aléas, ancien
//!   code invalidé).
//! - L'approbation exige le code vu par l'administrateur, comparé en temps
//!   constant au code de l'entrée (R-027).
//! - Durée de vie des demandes (10 min par défaut), plafond global et
//!   plafond par adresse source ; plein = refus, jamais d'éviction (R-028).
//! - Liste blanche persistée (`crate::paired_store`), fail-closed (R-024).

use crate::paired_store::{load_paired_devices, save_paired_devices, PairedStoreError};
use central_remote_protocol::pairing::PairingStatus;
use miyukini_cog_bridge::{
    identity_fingerprint_hex, pairing_sas_code, parse_identity_key_hex, sas_codes_equal,
    verify_pairing_commitment, PairingNonce,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Présentation lisible d'un code SAS (`1234 5678`), pour l'UI de Central.
pub use miyukini_cog_bridge::format_sas_code;
use std::net::IpAddr;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use uuid::Uuid;

/// Durée de validité par défaut d'un token API (30 jours).
pub const DEFAULT_API_TOKEN_TTL_SECS: u64 = 30 * 24 * 3600;

/// Durée de vie par défaut d'une demande d'appairage en attente (R-028).
pub const DEFAULT_PENDING_TTL: Duration = Duration::from_mins(10);

/// Nombre maximal de demandes d'appairage en attente (C-13).
pub const MAX_PENDING_DEVICES: usize = 64;

/// Nombre maximal de demandes en attente par adresse source (R-028).
pub const DEFAULT_MAX_PENDING_PER_SOURCE: usize = 3;

/// Nombre maximal d'issues (refus / expiration) mémorisées pour le suivi.
const MAX_OUTCOMES: usize = 256;

/// Longueur maximale d'un `device_id` (octets, C-13).
pub const MAX_DEVICE_ID_LEN: usize = 128;

/// Longueur maximale d'un nom de device (caractères, C-13).
pub const MAX_DEVICE_NAME_CHARS: usize = 64;

/// Paramètres de la file des demandes d'appairage (R-028).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PairingConfig {
    /// Durée de vie d'une demande non approuvée.
    pub pending_ttl: Duration,
    /// Plafond global de demandes en attente.
    pub max_pending: usize,
    /// Plafond de demandes en attente par adresse source.
    pub max_pending_per_source: usize,
}

impl Default for PairingConfig {
    fn default() -> Self {
        Self {
            pending_ttl: DEFAULT_PENDING_TTL,
            max_pending: MAX_PENDING_DEVICES,
            max_pending_per_source: DEFAULT_MAX_PENDING_PER_SOURCE,
        }
    }
}

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

/// Vue d'une demande en attente pour Central (bureau). Ne contient aucun aléa.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PendingDevice {
    pub device_id: String,
    pub name: String,
    pub requested_at: u64,
    /// Clé publique d'identité E2E présentée par l'appareil (hex 32 octets).
    pub e2e_identity_key: String,
    /// Empreinte lisible de la clé de l'appareil (à afficher).
    pub key_fingerprint: String,
    /// `Committed` (révélation attendue) ou `Revealed` (code disponible).
    pub status: PairingStatus,
    /// Code SAS à 8 chiffres, disponible seulement après révélation de `Nd`.
    /// L'administrateur le compare à celui du téléphone et le repasse tel quel
    /// à [`PairingManager::approve_device`].
    pub confirmation_code: Option<String>,
    /// Secondes restantes avant expiration.
    pub expires_in_secs: u64,
}

/// Requête de validation whitelist.
#[derive(Debug, Deserialize)]
pub struct WhitelistValidateRequest {
    pub device_id: String,
}

/// Erreurs d'appairage (une variante par cause de refus, C-11).
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum PairingError {
    #[error("device_id invalide")]
    InvalidDeviceId,
    #[error("clé d'identité E2E invalide")]
    InvalidIdentityKey,
    #[error("trop de demandes d'appairage en attente")]
    TooManyPending,
    #[error("trop de demandes d'appairage en attente pour cette source")]
    TooManyPendingForSource,
    #[error("une demande en attente existe déjà pour cet appareil avec une autre clé")]
    PendingConflict,
    #[error("clé d'identité déjà utilisée par un autre appareil")]
    KeyInUse,
    #[error("appareil déjà appairé avec cette clé (révoquer avant de ré-appairer)")]
    AlreadyPaired,
    #[error("aucune demande d'appairage en attente")]
    NoPending,
    #[error("aléa déjà révélé pour cette demande")]
    AlreadyRevealed,
    #[error("aléa révélé incompatible avec l'engagement : appairage abandonné")]
    CommitmentMismatch,
    #[error("aléa de l'appareil pas encore révélé")]
    NotRevealed,
    #[error("code de confirmation différent de celui de la demande")]
    CodeMismatch,
    #[error("persistance de la liste blanche impossible: {0}")]
    Persistence(String),
    #[error("état d'appairage indisponible")]
    Unavailable,
}

impl From<PairedStoreError> for PairingError {
    fn from(e: PairedStoreError) -> Self {
        Self::Persistence(e.to_string())
    }
}

/// Info de découverte LAN.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryInfo {
    pub host_name: String,
    pub remote_port: u16,
    pub bridge_port: u16,
    pub version: String,
}

/// État interne d'une demande en attente.
enum PendingState {
    /// Engagement reçu, `Nc` émis.
    Committed,
    /// `Nd` révélé et vérifié ; code SAS calculé.
    Revealed { code: String },
}

/// Demande d'appairage en attente (interne : contient les aléas).
struct PendingEntry {
    name: String,
    requested_at: u64,
    expires_at: Instant,
    source: IpAddr,
    /// Clé de l'appareil (hex canonique).
    key_hex: String,
    device_key: [u8; 32],
    commitment: [u8; 32],
    cog_nonce: PairingNonce,
    state: PendingState,
}

/// Issue terminale récente (pour le suivi côté appareil).
struct Outcome {
    key_hex: String,
    status: PairingStatus,
    until: Instant,
}

/// Demandes en attente et issues récentes (un seul verrou).
#[derive(Default)]
struct PendingBook {
    entries: HashMap<String, PendingEntry>,
    outcomes: HashMap<String, Outcome>,
}

impl PendingBook {
    /// Purge les demandes expirées (→ `Expired`) et les issues périmées.
    fn purge(&mut self, ttl: Duration) {
        let now = Instant::now();
        let expired: Vec<String> = self
            .entries
            .iter()
            .filter(|(_, e)| e.expires_at <= now)
            .map(|(id, _)| id.clone())
            .collect();
        for id in expired {
            if let Some(e) = self.entries.remove(&id) {
                self.record(id, e.key_hex, PairingStatus::Expired, ttl);
            }
        }
        self.outcomes.retain(|_, o| o.until > now);
    }

    fn record(&mut self, device_id: String, key_hex: String, status: PairingStatus, ttl: Duration) {
        if self.outcomes.len() >= MAX_OUTCOMES && !self.outcomes.contains_key(&device_id) {
            return;
        }
        self.outcomes.insert(
            device_id,
            Outcome {
                key_hex,
                status,
                until: Instant::now() + ttl,
            },
        );
    }
}

/// Gestionnaire d'appairage mobile.
pub struct PairingManager {
    /// Durée de validité des tokens API générés (secondes, jamais 0).
    api_token_ttl_secs: u64,
    /// Paramètres de la file d'attente.
    config: PairingConfig,
    /// Fichier de persistance de la liste blanche (`None` = mémoire seule).
    store_path: Option<PathBuf>,
    /// Devices whitelistés (device_id → infos).
    whitelist: Mutex<HashMap<String, PairedDevice>>,
    /// Tokens API actifs (token → infos).
    api_tokens: Mutex<HashMap<String, ApiToken>>,
    /// Demandes en attente. Ordre des verrous : `pending` puis `whitelist`.
    pending: Mutex<PendingBook>,
}

impl PairingManager {
    /// Gestionnaire en mémoire seule (tests, outils).
    pub fn new() -> Self {
        Self::with_token_ttl(DEFAULT_API_TOKEN_TTL_SECS)
    }

    /// Crée un gestionnaire avec une durée de vie de token API donnée
    /// (0 → valeur par défaut de 30 jours).
    pub fn with_token_ttl(ttl_secs: u64) -> Self {
        Self::with_config(ttl_secs, PairingConfig::default())
    }

    /// Gestionnaire en mémoire avec paramètres de file explicites.
    pub fn with_config(ttl_secs: u64, config: PairingConfig) -> Self {
        Self {
            api_token_ttl_secs: if ttl_secs == 0 { DEFAULT_API_TOKEN_TTL_SECS } else { ttl_secs },
            config: sanitize_config(config),
            store_path: None,
            whitelist: Mutex::new(HashMap::new()),
            api_tokens: Mutex::new(HashMap::new()),
            pending: Mutex::new(PendingBook::default()),
        }
    }

    /// Gestionnaire à liste blanche persistée dans `store_path`, chargée ici.
    /// Fichier corrompu ⇒ erreur (le serveur refuse de démarrer, R-024).
    pub fn open(
        ttl_secs: u64,
        config: PairingConfig,
        store_path: PathBuf,
    ) -> Result<Self, PairedStoreError> {
        let devices = load_paired_devices(&store_path)?;
        let mut mgr = Self::with_config(ttl_secs, config);
        mgr.whitelist = Mutex::new(
            devices
                .into_iter()
                .map(|d| (d.device_id.clone(), d))
                .collect(),
        );
        mgr.store_path = Some(store_path);
        Ok(mgr)
    }

    /// Paramètres de la file d'attente effectifs.
    pub fn config(&self) -> PairingConfig {
        self.config
    }

    /// Vérifie si un device est whitelisté.
    pub fn is_whitelisted(&self, device_id: &str) -> bool {
        self.whitelist
            .lock()
            .is_ok_and(|w| w.contains_key(device_id))
    }

    /// Met à jour le last_seen d'un device whitelisté (mémoire seule).
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

    // ── Appairage v2 (R-026 / R-027 / R-028) ────────────────────────────

    /// Étape 1 : enregistre l'engagement `commitment` de l'appareil (clé
    /// `e2e_identity_key_hex`, hex canonique déjà validé) venu de `source`, et
    /// retourne l'aléa `Nc` du COG à lui renvoyer.
    ///
    /// Refus : `device_id` invalide, appareil déjà appairé avec cette clé,
    /// clé utilisée par un autre appareil, demande en attente avec une autre
    /// clé (jamais écrasée), plafond global ou par source atteint.
    pub fn request_pairing(
        &self,
        device_id: &str,
        device_name: &str,
        e2e_identity_key_hex: &str,
        commitment: [u8; 32],
        source: IpAddr,
    ) -> Result<PairingNonce, PairingError> {
        if device_id.is_empty()
            || device_id.len() > MAX_DEVICE_ID_LEN
            || device_id.chars().any(char::is_control)
        {
            return Err(PairingError::InvalidDeviceId);
        }
        // Clé valide (point ed25519 non faible) et forme hex canonique : les
        // comparaisons de clés se font ensuite sur cette forme.
        let device_key = parse_identity_key_hex(e2e_identity_key_hex)
            .map_err(|_| PairingError::InvalidIdentityKey)?;
        if hex::encode(device_key) != e2e_identity_key_hex {
            return Err(PairingError::InvalidIdentityKey);
        }
        let name: String = device_name
            .chars()
            .filter(|c| !c.is_control())
            .take(MAX_DEVICE_NAME_CHARS)
            .collect();
        let source = source.to_canonical();

        let mut book = self.pending.lock().map_err(|_| PairingError::Unavailable)?;
        book.purge(self.config.pending_ttl);

        {
            let whitelist = self.whitelist.lock().map_err(|_| PairingError::Unavailable)?;
            for d in whitelist.values() {
                if d.e2e_identity_key == e2e_identity_key_hex {
                    return Err(if d.device_id == device_id {
                        PairingError::AlreadyPaired
                    } else {
                        PairingError::KeyInUse
                    });
                }
            }
        }

        let replacing = match book.entries.get(device_id) {
            // R-027 : jamais d'écrasement par une autre clé.
            Some(existing) if existing.key_hex != e2e_identity_key_hex => {
                return Err(PairingError::PendingConflict);
            }
            Some(_) => true,
            None => false,
        };
        if book
            .entries
            .iter()
            .any(|(id, e)| id != device_id && e.key_hex == e2e_identity_key_hex)
        {
            return Err(PairingError::KeyInUse);
        }
        // R-028 : plafonds global et par source ; plein = refus, sans éviction.
        let others = book.entries.iter().filter(|(id, _)| id.as_str() != device_id);
        let (total, from_source) = others.fold((0usize, 0usize), |(t, s), (_, e)| {
            (t + 1, s + usize::from(e.source == source))
        });
        if !replacing && total >= self.config.max_pending {
            return Err(PairingError::TooManyPending);
        }
        if from_source >= self.config.max_pending_per_source {
            return Err(PairingError::TooManyPendingForSource);
        }

        let cog_nonce = PairingNonce::generate();
        book.outcomes.remove(device_id);
        book.entries.insert(
            device_id.to_string(),
            PendingEntry {
                name,
                requested_at: now_secs(),
                expires_at: Instant::now() + self.config.pending_ttl,
                source,
                key_hex: e2e_identity_key_hex.to_string(),
                device_key,
                commitment,
                cog_nonce: cog_nonce.clone(),
                state: PendingState::Committed,
            },
        );
        Ok(cog_nonce)
    }

    /// Étape 3 : l'appareil révèle `Nd`. Vérifie l'engagement en temps
    /// constant ; incohérence ⇒ demande supprimée (`Rejected`). Une seule
    /// révélation par demande. Retourne le code SAS calculé avec `cog_key`.
    pub fn reveal_pairing(
        &self,
        device_id: &str,
        device_nonce: &PairingNonce,
        cog_key: &[u8; 32],
    ) -> Result<String, PairingError> {
        let mut book = self.pending.lock().map_err(|_| PairingError::Unavailable)?;
        book.purge(self.config.pending_ttl);
        let entry = book.entries.get_mut(device_id).ok_or(PairingError::NoPending)?;
        if matches!(entry.state, PendingState::Revealed { .. }) {
            return Err(PairingError::AlreadyRevealed);
        }
        if !verify_pairing_commitment(&entry.commitment, &entry.device_key, device_nonce) {
            if let Some(e) = book.entries.remove(device_id) {
                book.record(
                    device_id.to_string(),
                    e.key_hex,
                    PairingStatus::Rejected,
                    self.config.pending_ttl,
                );
            }
            return Err(PairingError::CommitmentMismatch);
        }
        let code = pairing_sas_code(&entry.device_key, cog_key, device_nonce, &entry.cog_nonce);
        entry.state = PendingState::Revealed { code: code.clone() };
        Ok(code)
    }

    /// Approuve une demande (appelé depuis Central bureau) avec le code que
    /// l'administrateur a vu et comparé au téléphone. Refus si la demande
    /// n'est pas révélée ou si le code diffère (temps constant, R-027).
    ///
    /// La liste blanche est persistée avant de confirmer ; en cas d'échec
    /// d'écriture, rien n'est approuvé (fail-closed).
    pub fn approve_device(&self, device_id: &str, displayed_code: &str) -> Result<(), PairingError> {
        let mut book = self.pending.lock().map_err(|_| PairingError::Unavailable)?;
        book.purge(self.config.pending_ttl);
        let entry = book.entries.get(device_id).ok_or(PairingError::NoPending)?;
        let PendingState::Revealed { code } = &entry.state else {
            return Err(PairingError::NotRevealed);
        };
        if !sas_codes_equal(code, displayed_code) {
            return Err(PairingError::CodeMismatch);
        }
        let mut whitelist = self.whitelist.lock().map_err(|_| PairingError::Unavailable)?;
        let now = now_secs();
        let paired = PairedDevice {
            device_id: device_id.to_string(),
            name: entry.name.clone(),
            paired_at: now,
            last_seen: now,
            mode: "whitelist".into(),
            e2e_identity_key: entry.key_hex.clone(),
        };
        let previous = whitelist.insert(device_id.to_string(), paired);
        if let Err(e) = self.persist(&whitelist) {
            // Retour arrière : l'ancienne entrée (éventuelle) est restaurée.
            match previous {
                Some(p) => whitelist.insert(device_id.to_string(), p),
                None => whitelist.remove(device_id),
            };
            return Err(e);
        }
        book.entries.remove(device_id);
        Ok(())
    }

    /// Refuse une demande en attente (issue `Rejected` visible par l'appareil).
    pub fn reject_device(&self, device_id: &str) -> bool {
        let Ok(mut book) = self.pending.lock() else {
            return false;
        };
        book.purge(self.config.pending_ttl);
        match book.entries.remove(device_id) {
            Some(e) => {
                book.record(
                    device_id.to_string(),
                    e.key_hex,
                    PairingStatus::Rejected,
                    self.config.pending_ttl,
                );
                true
            }
            None => false,
        }
    }

    /// État de la demande de `device_id` pour la clé `e2e_identity_key_hex`
    /// (suivi côté appareil). `Approved` uniquement si cette clé précise est
    /// épinglée dans la liste blanche.
    pub fn pairing_status(&self, device_id: &str, e2e_identity_key_hex: &str) -> PairingStatus {
        let Ok(mut book) = self.pending.lock() else {
            return PairingStatus::Unknown;
        };
        book.purge(self.config.pending_ttl);
        if let Some(e) = book.entries.get(device_id) {
            if e.key_hex == e2e_identity_key_hex {
                return match e.state {
                    PendingState::Committed => PairingStatus::Committed,
                    PendingState::Revealed { .. } => PairingStatus::Revealed,
                };
            }
        }
        if self.is_paired_with_key(device_id, e2e_identity_key_hex) {
            return PairingStatus::Approved;
        }
        book.outcomes
            .get(device_id)
            .filter(|o| o.key_hex == e2e_identity_key_hex)
            .map_or(PairingStatus::Unknown, |o| o.status)
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

    /// Liste les demandes en attente (purge des expirées au passage).
    pub fn pending_devices(&self) -> Vec<PendingDevice> {
        let Ok(mut book) = self.pending.lock() else {
            return Vec::new();
        };
        book.purge(self.config.pending_ttl);
        let now = Instant::now();
        let mut out: Vec<PendingDevice> = book
            .entries
            .iter()
            .map(|(id, e)| {
                let (status, confirmation_code) = match &e.state {
                    PendingState::Committed => (PairingStatus::Committed, None),
                    PendingState::Revealed { code } => (PairingStatus::Revealed, Some(code.clone())),
                };
                PendingDevice {
                    device_id: id.clone(),
                    name: e.name.clone(),
                    requested_at: e.requested_at,
                    e2e_identity_key: e.key_hex.clone(),
                    key_fingerprint: identity_fingerprint_hex(&e.key_hex).unwrap_or_default(),
                    status,
                    confirmation_code,
                    expires_in_secs: e.expires_at.saturating_duration_since(now).as_secs(),
                }
            })
            .collect();
        out.sort_by_key(|d| d.requested_at);
        out
    }

    /// Liste les devices appairés (identifiants complets : usage Rust local uniquement).
    pub fn paired_devices(&self) -> Vec<PairedDevice> {
        self.whitelist
            .lock()
            .map(|w| w.values().cloned().collect())
            .unwrap_or_default()
    }

    /// Révoque un device appairé. Le retrait en mémoire est immédiat (accès
    /// coupé) ; une erreur de persistance est remontée pour que l'UI la
    /// signale (sinon l'appareil réapparaîtrait au redémarrage).
    pub fn revoke_device(&self, device_id: &str) -> Result<bool, PairingError> {
        let mut whitelist = self.whitelist.lock().map_err(|_| PairingError::Unavailable)?;
        if whitelist.remove(device_id).is_none() {
            return Ok(false);
        }
        self.persist(&whitelist)?;
        Ok(true)
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

    /// Écrit la liste blanche (appelé sous le verrou `whitelist`).
    fn persist(&self, whitelist: &HashMap<String, PairedDevice>) -> Result<(), PairingError> {
        match &self.store_path {
            Some(path) => {
                let devices: Vec<PairedDevice> = whitelist.values().cloned().collect();
                save_paired_devices(path, &devices).map_err(PairingError::from)
            }
            None => Ok(()),
        }
    }
}

impl Default for PairingManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Bornes minimales des paramètres (jamais de file sans plafond ni durée).
fn sanitize_config(c: PairingConfig) -> PairingConfig {
    PairingConfig {
        pending_ttl: if c.pending_ttl.is_zero() { DEFAULT_PENDING_TTL } else { c.pending_ttl },
        max_pending: c.max_pending.clamp(1, MAX_PENDING_DEVICES),
        max_pending_per_source: c.max_pending_per_source.max(1),
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
    use crate::paired_store::DEFAULT_PAIRED_DEVICES_FILE;
    use miyukini_cog_bridge::{parse_commitment_hex, DevicePairing, E2eIdentity};

    const IP_A: &str = "192.168.1.10";
    const IP_B: &str = "192.168.1.11";

    fn ip(s: &str) -> IpAddr {
        s.parse().unwrap()
    }

    fn cog_key() -> [u8; 32] {
        E2eIdentity::from_secret_bytes(&[0x42; 32]).public_key_bytes()
    }

    /// Appareil de test : identité + appairage démarré.
    struct TestDevice {
        key: [u8; 32],
        key_hex: String,
        pairing: DevicePairing,
    }

    fn new_device() -> TestDevice {
        let key = E2eIdentity::generate().public_key_bytes();
        TestDevice {
            key,
            key_hex: hex::encode(key),
            pairing: DevicePairing::start(key),
        }
    }

    fn commit(mgr: &PairingManager, id: &str, d: &TestDevice, src: &str) -> Result<PairingNonce, PairingError> {
        let c = parse_commitment_hex(&d.pairing.commitment_hex()).unwrap();
        mgr.request_pairing(id, "Pixel", &d.key_hex, c, ip(src))
    }

    /// Appairage honnête complet jusqu'à `Revealed` ; retourne (code appareil, code COG).
    fn commit_and_reveal(mgr: &PairingManager, id: &str, src: &str) -> (TestDevice, String, String) {
        let d = new_device();
        let nc = commit(mgr, id, &d, src).unwrap();
        let key_hex = d.key_hex.clone();
        let key = d.key;
        let revealed = d.pairing.receive_cog_nonce(cog_key(), nc).unwrap();
        let nd = PairingNonce::from_hex(&revealed.device_nonce_hex()).unwrap();
        let cog_code = mgr.reveal_pairing(id, &nd, &cog_key()).unwrap();
        let device_code = revealed.code().to_string();
        let d = TestDevice {
            key,
            key_hex,
            pairing: DevicePairing::start(key),
        };
        (d, device_code, cog_code)
    }

    fn tmp_store() -> PathBuf {
        let d = std::env::temp_dir().join(format!("pairing-mgr-{}", Uuid::new_v4()));
        std::fs::create_dir_all(&d).unwrap();
        d.join(DEFAULT_PAIRED_DEVICES_FILE)
    }

    #[test]
    fn honest_pairing_same_code_and_approval() {
        let mgr = PairingManager::new();
        let (d, device_code, cog_code) = commit_and_reveal(&mgr, "dev-1", IP_A);
        assert_eq!(device_code, cog_code);
        assert_eq!(cog_code.len(), 8);
        assert_eq!(mgr.pairing_status("dev-1", &d.key_hex), PairingStatus::Revealed);
        let pending = mgr.pending_devices();
        assert_eq!(pending[0].confirmation_code.as_deref(), Some(cog_code.as_str()));
        assert_eq!(pending[0].key_fingerprint.split(' ').count(), 8);
        // Approbation avec le code affiché (format lisible accepté).
        let shown = miyukini_cog_bridge::format_sas_code(&cog_code);
        mgr.approve_device("dev-1", &shown).unwrap();
        assert!(mgr.is_paired_with_key("dev-1", &d.key_hex));
        assert_eq!(mgr.pairing_status("dev-1", &d.key_hex), PairingStatus::Approved);
        assert_eq!(mgr.pending_devices().len(), 0);
        // Ré-appairage avec la même clé : refusé (révoquer d'abord).
        assert_eq!(commit(&mgr, "dev-1", &d, IP_A).unwrap_err(), PairingError::AlreadyPaired);
    }

    #[test]
    fn reveal_with_wrong_nonce_aborts() {
        let mgr = PairingManager::new();
        let d = new_device();
        commit(&mgr, "dev-1", &d, IP_A).unwrap();
        assert_eq!(
            mgr.reveal_pairing("dev-1", &PairingNonce::generate(), &cog_key()).unwrap_err(),
            PairingError::CommitmentMismatch
        );
        assert_eq!(mgr.pending_devices().len(), 0);
        assert_eq!(mgr.pairing_status("dev-1", &d.key_hex), PairingStatus::Rejected);
        // La bonne révélation arrive trop tard : plus d'entrée.
        assert_eq!(
            mgr.reveal_pairing("dev-1", &PairingNonce::generate(), &cog_key()).unwrap_err(),
            PairingError::NoPending
        );
    }

    #[test]
    fn reveal_before_cog_nonce_refused() {
        // Aucune entrée (donc aucun Nc émis) : révélation refusée.
        let mgr = PairingManager::new();
        assert_eq!(
            mgr.reveal_pairing("dev-1", &PairingNonce::generate(), &cog_key()).unwrap_err(),
            PairingError::NoPending
        );
        assert_eq!(mgr.pairing_status("dev-1", &"aa".repeat(32)), PairingStatus::Unknown);
    }

    #[test]
    fn second_reveal_refused() {
        let mgr = PairingManager::new();
        let d = new_device();
        let nc = commit(&mgr, "dev-1", &d, IP_A).unwrap();
        let revealed = d.pairing.receive_cog_nonce(cog_key(), nc).unwrap();
        let nd = PairingNonce::from_hex(&revealed.device_nonce_hex()).unwrap();
        let code = mgr.reveal_pairing("dev-1", &nd, &cog_key()).unwrap();
        assert_eq!(
            mgr.reveal_pairing("dev-1", &nd, &cog_key()).unwrap_err(),
            PairingError::AlreadyRevealed
        );
        // L'entrée révélée reste approuvable avec son code.
        mgr.approve_device("dev-1", &code).unwrap();
    }

    #[test]
    fn approve_requires_reveal_and_matching_code() {
        let mgr = PairingManager::new();
        let d = new_device();
        commit(&mgr, "dev-1", &d, IP_A).unwrap();
        assert_eq!(mgr.approve_device("dev-1", "00000000").unwrap_err(), PairingError::NotRevealed);
        let (_d2, _, code) = commit_and_reveal(&mgr, "dev-2", IP_B);
        let wrong = if code == "00000000" { "11111111" } else { "00000000" };
        assert_eq!(mgr.approve_device("dev-2", wrong).unwrap_err(), PairingError::CodeMismatch);
        assert_eq!(mgr.approve_device("dev-2", "abc").unwrap_err(), PairingError::CodeMismatch);
        assert!(!mgr.is_whitelisted("dev-2"));
        assert_eq!(mgr.approve_device("absent", &code).unwrap_err(), PairingError::NoPending);
        mgr.approve_device("dev-2", &code).unwrap();
    }

    #[test]
    fn pending_never_overwritten_by_other_key() {
        let mgr = PairingManager::new();
        let (legit, _, code) = commit_and_reveal(&mgr, "dev-1", IP_A);
        let attacker = new_device();
        assert_eq!(
            commit(&mgr, "dev-1", &attacker, IP_B).unwrap_err(),
            PairingError::PendingConflict
        );
        // La demande légitime et son code sont intacts.
        let p = mgr.pending_devices();
        assert_eq!(p[0].e2e_identity_key, legit.key_hex);
        assert_eq!(p[0].confirmation_code.as_deref(), Some(code.as_str()));
        // Même clé sous un autre device_id : refusé aussi.
        assert_eq!(commit(&mgr, "dev-9", &legit, IP_B).unwrap_err(), PairingError::KeyInUse);
        // Après refus explicite, une nouvelle demande est possible.
        assert!(mgr.reject_device("dev-1"));
        assert_eq!(mgr.pairing_status("dev-1", &legit.key_hex), PairingStatus::Rejected);
        commit(&mgr, "dev-1", &attacker, IP_B).unwrap();
    }

    #[test]
    fn same_key_restart_invalidates_old_code() {
        let mgr = PairingManager::new();
        let (d, _, old_code) = commit_and_reveal(&mgr, "dev-1", IP_A);
        // L'appareil relance (nouveaux aléas) : l'ancien code n'est plus approuvable.
        commit(&mgr, "dev-1", &d, IP_A).unwrap();
        assert_eq!(mgr.pending_devices().len(), 1);
        assert_eq!(mgr.approve_device("dev-1", &old_code).unwrap_err(), PairingError::NotRevealed);
    }

    #[test]
    fn pending_expires_after_ttl() {
        let mgr = PairingManager::with_config(
            0,
            PairingConfig {
                pending_ttl: Duration::from_secs(600),
                ..PairingConfig::default()
            },
        );
        let (d, _, code) = commit_and_reveal(&mgr, "dev-1", IP_A);
        assert!(mgr.pending_devices()[0].expires_in_secs > 590);
        // Simule l'écoulement du délai.
        {
            let mut book = mgr.pending.lock().unwrap();
            let e = book.entries.get_mut("dev-1").unwrap();
            e.expires_at = Instant::now().checked_sub(Duration::from_secs(1)).unwrap();
        }
        assert_eq!(mgr.approve_device("dev-1", &code).unwrap_err(), PairingError::NoPending);
        assert_eq!(mgr.pending_devices().len(), 0);
        assert_eq!(mgr.pairing_status("dev-1", &d.key_hex), PairingStatus::Expired);
        assert!(!mgr.is_whitelisted("dev-1"));
    }

    #[test]
    fn per_source_and_global_caps_refuse_without_eviction() {
        let mgr = PairingManager::with_config(
            0,
            PairingConfig {
                max_pending: 5,
                max_pending_per_source: 3,
                ..PairingConfig::default()
            },
        );
        for i in 0..3 {
            commit(&mgr, &format!("dev-a{i}"), &new_device(), IP_A).unwrap();
        }
        assert_eq!(
            commit(&mgr, "dev-a3", &new_device(), IP_A).unwrap_err(),
            PairingError::TooManyPendingForSource
        );
        // IPv4 mappée en IPv6 = même source.
        assert_eq!(
            commit(&mgr, "dev-a4", &new_device(), "::ffff:192.168.1.10").unwrap_err(),
            PairingError::TooManyPendingForSource
        );
        for i in 0..2 {
            commit(&mgr, &format!("dev-b{i}"), &new_device(), IP_B).unwrap();
        }
        assert_eq!(
            commit(&mgr, "dev-c0", &new_device(), "10.0.0.1").unwrap_err(),
            PairingError::TooManyPending
        );
        // Aucune demande existante n'a été évincée.
        assert_eq!(mgr.pending_devices().len(), 5);
    }

    #[test]
    fn pairing_inputs_bounded() {
        let mgr = PairingManager::new();
        let d = new_device();
        assert_eq!(commit(&mgr, "", &d, IP_A).unwrap_err(), PairingError::InvalidDeviceId);
        assert_eq!(
            commit(&mgr, &"d".repeat(MAX_DEVICE_ID_LEN + 1), &d, IP_A).unwrap_err(),
            PairingError::InvalidDeviceId
        );
        let c = parse_commitment_hex(&d.pairing.commitment_hex()).unwrap();
        mgr.request_pairing("dev-0", &"n".repeat(500), &d.key_hex, c, ip(IP_A)).unwrap();
        assert_eq!(mgr.pending_devices()[0].name.chars().count(), MAX_DEVICE_NAME_CHARS);
        assert!(mgr.device_for_identity_key("").is_none());
        // Configuration nulle : bornes minimales appliquées.
        let c = sanitize_config(PairingConfig {
            pending_ttl: Duration::ZERO,
            max_pending: 0,
            max_pending_per_source: 0,
        });
        assert_eq!(c.pending_ttl, DEFAULT_PENDING_TTL);
        assert_eq!((c.max_pending, c.max_pending_per_source), (1, 1));
    }

    #[test]
    fn key_change_requires_new_approval() {
        let mgr = PairingManager::new();
        let (a, _, code) = commit_and_reveal(&mgr, "dev-1", IP_A);
        mgr.approve_device("dev-1", &code).unwrap();
        // Même device_id, autre clé : en attente, l'ancienne clé reste épinglée.
        let (b, _, code_b) = commit_and_reveal(&mgr, "dev-1", IP_A);
        assert!(mgr.device_for_identity_key(&b.key_hex).is_none());
        assert!(mgr.device_for_identity_key(&a.key_hex).is_some());
        mgr.approve_device("dev-1", &code_b).unwrap();
        assert!(mgr.device_for_identity_key(&b.key_hex).is_some());
        assert!(mgr.device_for_identity_key(&a.key_hex).is_none());
    }

    #[test]
    fn whitelist_persists_across_restart() {
        let path = tmp_store();
        let key_hex;
        {
            let mgr = PairingManager::open(0, PairingConfig::default(), path.clone()).unwrap();
            let (d, _, code) = commit_and_reveal(&mgr, "dev-1", IP_A);
            key_hex = d.key_hex;
            mgr.approve_device("dev-1", &code).unwrap();
            let (_, _, code2) = commit_and_reveal(&mgr, "dev-2", IP_B);
            mgr.approve_device("dev-2", &code2).unwrap();
            assert_eq!(mgr.revoke_device("dev-2"), Ok(true));
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = std::fs::metadata(&path).unwrap().permissions().mode();
            assert_eq!(mode & 0o777, 0o600);
        }
        let mgr = PairingManager::open(0, PairingConfig::default(), path).unwrap();
        assert!(mgr.is_paired_with_key("dev-1", &key_hex));
        assert!(!mgr.is_whitelisted("dev-2"), "révocation persistée");
        let p = mgr.device_for_identity_key(&key_hex).unwrap();
        assert!(p.paired_at > 0);
    }

    #[test]
    fn corrupted_store_refused_at_open() {
        let path = tmp_store();
        std::fs::write(&path, b"{corrompu").unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600)).unwrap();
        }
        assert!(matches!(
            PairingManager::open(0, PairingConfig::default(), path),
            Err(PairedStoreError::Malformed(_))
        ));
    }

    #[test]
    fn approval_fails_closed_when_store_unwritable() {
        let dir = std::env::temp_dir().join(format!("pairing-ro-{}", Uuid::new_v4()));
        std::fs::create_dir_all(&dir).unwrap();
        // Le chemin de stockage est un dossier : écriture (rename) impossible.
        let path = dir.join("store-as-dir");
        std::fs::create_dir_all(path.join("x")).unwrap();
        let mgr = PairingManager::with_config(0, PairingConfig::default());
        let mgr = PairingManager {
            store_path: Some(path),
            ..mgr
        };
        let (d, _, code) = commit_and_reveal(&mgr, "dev-1", IP_A);
        assert!(matches!(mgr.approve_device("dev-1", &code), Err(PairingError::Persistence(_))));
        assert!(!mgr.is_paired_with_key("dev-1", &d.key_hex));
        // La demande reste en attente (approbation retentable).
        assert_eq!(mgr.pairing_status("dev-1", &d.key_hex), PairingStatus::Revealed);
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
        commit(&mgr, "dev-2", &new_device(), IP_A).unwrap();
        assert_eq!(mgr.pending_devices().len(), 1);

        assert!(mgr.reject_device("dev-2"));
        assert_eq!(mgr.pending_devices().len(), 0);
        assert!(!mgr.is_whitelisted("dev-2"));
        assert!(!mgr.reject_device("dev-2"));
    }
}
