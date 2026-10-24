//! @id     central_remote_pairing
//! @do     definir_pairing_manager
//! @role   orchestration
//! @layer  core
//! @human  central-remote — module « pairing ».

//! Appairage mobile — gestion des appareils autorisés et de leur révocation.
//!
//! L'appairage v2 épingle la clé d'identité Ed25519 de l'appareil : c'est la
//! racine de confiance des passeports d'appareil émis ensuite par le COG
//! ([`crate::identite`]). Le mode « clé API » (jeton au porteur de 30 jours) est
//! retiré (sortie du lot 2, inventaire #2).
//!
//! ## Révocation (EX-14, fail-closed)
//!
//! [`PairingManager::revoke_device`] retire l'appareil de la liste blanche
//! persistée **et** ajoute sa clé à la [`ListeRevocation`] du COG (activée par
//! [`PairingManager::activer_revocations`]), consultée à chaque vérification de
//! passeport et de visa. Sans liste activée, tout passeport est refusé. Une clé
//! révoquée ne peut pas être ré-appairée pendant la vie du processus
//! ([`PairingError::KeyRevoked`]) ; au redémarrage, la révocation durable est
//! l'absence de la liste blanche (toute clé non appairée est traitée comme
//! révoquée par [`crate::identite`]).
//!
//! ## Appairage v2 : engagement puis révélation (R-026)
//!
//! Voir `miyukini_cog_bridge::pairing` pour le protocole et ses garanties.
//! Machine à états d'une demande (une entrée par `device_id`) :
//!
//! ```text
//! Committed ──reveal(Nd) signé, valide──▶ Revealed ──approve(code affiché)──▶ Approved (liste blanche)
//!     │                                       │
//!     ├──reveal(Nd) signé, Nd faux──▶ Rejected (entrée supprimée)
//!     ├──reject (desktop)──────────────────▶ Rejected
//!     └──délai dépassé─────────────────────▶ Expired (purge à chaque opération)
//! ```
//!
//! - **Preuve de possession** : engagement et révélation sont signés par la
//!   clé d'identité de l'appareil `Kd` et vérifiés (`verify_strict`) avant
//!   toute autre opération. Signature absente ou invalide = refus **sans
//!   toucher** à la demande en attente : connaître `device_id` ne suffit
//!   plus pour l'abandonner. Seule une révélation valablement signée avec un
//!   mauvais `Nd` (donc émise par le détenteur de `Kd`) supprime l'entrée.
//! - **Liaison à la source** : la révélation (et la relance) doit venir de
//!   l'adresse IP de l'engagement ([`PairingConfig::require_same_source`],
//!   actif par défaut) ; sinon refus sans toucher à l'entrée. Derrière un NAT
//!   ou un proxy, tous les appareils partagent une même adresse : la liaison
//!   y est sans effet mais reste sûre (les signatures suffisent) ; un
//!   téléphone qui change de réseau entre engagement et révélation doit
//!   recommencer. Désactiver l'option seulement si l'adresse source n'est
//!   pas stable entre deux requêtes.
//! - Le COG n'accepte la révélation que pour une entrée existante, donc
//!   **après** avoir émis `Nc` ; une seule révélation par entrée.
//! - Une entrée en attente n'est **jamais** remplacée par une autre clé (R-027) :
//!   l'appareil attend l'expiration ou le refus explicite. Le même appareil
//!   avec la **même** clé peut relancer sa demande (nouveaux aléas, ancien
//!   code invalidé) seulement si la relance est signée par `Kd`, vient de la
//!   même source, porte un nouvel engagement, et que la demande est encore
//!   `Committed` (jamais après la révélation : `PendingConflict`).
//! - L'approbation exige le code vu par l'administrateur, comparé en temps
//!   constant au code de l'entrée (R-027).
//! - Durée de vie des demandes (10 min par défaut), plafond global et
//!   plafond par adresse source ; plein = refus, jamais d'éviction (R-028).
//! - Liste blanche persistée (`crate::paired_store`), fail-closed (R-024).

use crate::paired_store::{load_paired_devices, save_paired_devices, PairedStoreError};
use miyauth::{CibleRevocation, CogId, ErreurRevocation, ListeRevocation, SourceRevocation};
use miyusceau::{empreinte_cle, Kid};
use central_remote_protocol::pairing::PairingStatus;
use miyukini_cog_bridge::{
    identity_fingerprint_hex, pairing_sas_code, parse_identity_key_hex, sas_codes_equal,
    verify_pairing_commit_signature, verify_pairing_commitment, verify_pairing_reveal_signature,
    PairingNonce, PAIRING_SIGNATURE_SIZE,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Présentation lisible d'un code SAS (`1234 5678`), pour l'UI de Central.
pub use miyukini_cog_bridge::format_sas_code;
use std::net::IpAddr;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

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
    /// La révélation et la relance doivent venir de l'adresse source de
    /// l'engagement (défaut : vrai ; voir la note NAT du module).
    pub require_same_source: bool,
}

impl Default for PairingConfig {
    fn default() -> Self {
        Self {
            pending_ttl: DEFAULT_PENDING_TTL,
            max_pending: MAX_PENDING_DEVICES,
            max_pending_per_source: DEFAULT_MAX_PENDING_PER_SOURCE,
            require_same_source: true,
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
    /// épinglée à l'approbation. Seule clé acceptée au handshake E2E et seule
    /// clé pour laquelle le COG émet un passeport d'appareil.
    pub e2e_identity_key: String,
    /// Profil Central (titulaire) lié à l'appareil à l'approbation ; `None` =
    /// titulaire par défaut du COG.
    #[serde(default)]
    pub profile_id: Option<String>,
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
    #[error("signature d'appairage invalide (preuve de possession de la clé)")]
    InvalidSignature,
    #[error("requête d'une autre source que l'engagement")]
    SourceMismatch,
    #[error("persistance de la liste blanche impossible: {0}")]
    Persistence(String),
    #[error("état d'appairage indisponible")]
    Unavailable,
    #[error("clé d'appareil révoquée : générer une nouvelle identité avant de ré-appairer")]
    KeyRevoked,
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
    /// Paramètres de la file d'attente.
    config: PairingConfig,
    /// Fichier de persistance de la liste blanche (`None` = mémoire seule).
    store_path: Option<PathBuf>,
    /// Devices whitelistés (device_id → infos).
    whitelist: Mutex<HashMap<String, PairedDevice>>,
    /// Demandes en attente. Ordre des verrous : `pending` puis `whitelist`
    /// (puis `revocations`).
    pending: Mutex<PendingBook>,
    /// Liste de révocation du COG (`None` = non activée : tout est refusé).
    revocations: Mutex<Option<ListeRevocation>>,
}

impl PairingManager {
    /// Gestionnaire en mémoire seule (tests, outils).
    pub fn new() -> Self {
        Self::with_config(PairingConfig::default())
    }

    /// Gestionnaire en mémoire avec paramètres de file explicites.
    pub fn with_config(config: PairingConfig) -> Self {
        Self {
            config: sanitize_config(config),
            store_path: None,
            whitelist: Mutex::new(HashMap::new()),
            pending: Mutex::new(PendingBook::default()),
            revocations: Mutex::new(None),
        }
    }

    /// Gestionnaire à liste blanche persistée dans `store_path`, chargée ici.
    /// Fichier corrompu ⇒ erreur (le serveur refuse de démarrer, R-024).
    pub fn open(config: PairingConfig, store_path: PathBuf) -> Result<Self, PairedStoreError> {
        let devices = load_paired_devices(&store_path)?;
        let mut mgr = Self::with_config(config);
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

    // ── Appairage v2 (R-026 / R-027 / R-028) ────────────────────────────

    /// Étape 1 : enregistre l'engagement `commitment` de l'appareil (clé
    /// `e2e_identity_key_hex`, hex canonique déjà validé) venu de `source`, et
    /// retourne l'aléa `Nc` du COG à lui renvoyer.
    ///
    /// `commit_signature` est la signature par `Kd` de l'engagement ; elle est
    /// vérifiée avant toute autre opération (aucun état touché si invalide).
    ///
    /// Refus : `device_id` invalide, signature invalide, appareil déjà appairé
    /// avec cette clé, clé utilisée par un autre appareil, demande en attente
    /// avec une autre clé (jamais écrasée), relance après révélation ou avec
    /// le même engagement, relance d'une autre source, plafond global ou par
    /// source atteint.
    pub fn request_pairing(
        &self,
        device_id: &str,
        device_name: &str,
        e2e_identity_key_hex: &str,
        commitment: [u8; 32],
        commit_signature: &[u8; PAIRING_SIGNATURE_SIZE],
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
        // Preuve de possession de Kd, avant toute lecture de l'état (C-23).
        verify_pairing_commit_signature(&device_key, device_id, &commitment, commit_signature)
            .map_err(|_| PairingError::InvalidSignature)?;
        if self.is_key_revoked(e2e_identity_key_hex) {
            return Err(PairingError::KeyRevoked);
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
            // Relance de la même clé : seulement avant la révélation (le code
            // affiché ne change jamais sous les yeux de l'administrateur), et
            // jamais avec l'engagement déjà enregistré (rejeu d'une requête
            // signée capturée).
            Some(existing)
                if matches!(existing.state, PendingState::Revealed { .. })
                    || existing.commitment == commitment =>
            {
                return Err(PairingError::PendingConflict);
            }
            Some(existing) if self.config.require_same_source && existing.source != source => {
                return Err(PairingError::SourceMismatch);
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

    /// Étape 3 : l'appareil révèle `Nd`, venu de `source`, avec la signature
    /// `reveal_signature` par `Kd` de `(device_id, Kd, Nd, Nc)`.
    ///
    /// Ordre des contrôles : signature (`verify_strict` avec la clé de
    /// l'entrée), puis source, puis état ; un refus à ces étapes laisse la
    /// demande **intacte**. Ensuite l'engagement est vérifié en temps
    /// constant : un `Nd` incohérent, valablement signé, supprime la demande
    /// (`Rejected`). Une seule révélation par demande. Retourne le code SAS
    /// calculé avec `cog_key`.
    pub fn reveal_pairing(
        &self,
        device_id: &str,
        device_nonce: &PairingNonce,
        reveal_signature: &[u8; PAIRING_SIGNATURE_SIZE],
        source: IpAddr,
        cog_key: &[u8; 32],
    ) -> Result<String, PairingError> {
        let source = source.to_canonical();
        let mut book = self.pending.lock().map_err(|_| PairingError::Unavailable)?;
        book.purge(self.config.pending_ttl);
        let entry = book.entries.get_mut(device_id).ok_or(PairingError::NoPending)?;
        verify_pairing_reveal_signature(
            &entry.device_key,
            device_id,
            device_nonce,
            &entry.cog_nonce,
            reveal_signature,
        )
        .map_err(|_| PairingError::InvalidSignature)?;
        if self.config.require_same_source && entry.source != source {
            return Err(PairingError::SourceMismatch);
        }
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
        self.approve_device_for(device_id, displayed_code, None)
    }

    /// Comme [`Self::approve_device`], en liant l'appareil au profil Central
    /// `profile_id` (titulaire de ses passeports ; `None` = titulaire par défaut).
    pub fn approve_device_for(
        &self,
        device_id: &str,
        displayed_code: &str,
        profile_id: Option<&str>,
    ) -> Result<(), PairingError> {
        let mut book = self.pending.lock().map_err(|_| PairingError::Unavailable)?;
        book.purge(self.config.pending_ttl);
        let entry = book.entries.get(device_id).ok_or(PairingError::NoPending)?;
        let PendingState::Revealed { code } = &entry.state else {
            return Err(PairingError::NotRevealed);
        };
        if !sas_codes_equal(code, displayed_code) {
            return Err(PairingError::CodeMismatch);
        }
        if self.is_key_revoked(&entry.key_hex) {
            return Err(PairingError::KeyRevoked);
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
            profile_id: profile_id.map(str::to_string),
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
    ///
    /// La clé de l'appareil est ajoutée à la liste de révocation du COG avant
    /// toute autre étape : ses passeports et visas sont refusés immédiatement,
    /// même si l'écriture de la liste blanche échoue (EX-14).
    pub fn revoke_device(&self, device_id: &str) -> Result<bool, PairingError> {
        let mut whitelist = self.whitelist.lock().map_err(|_| PairingError::Unavailable)?;
        let Some(device) = whitelist.remove(device_id) else {
            return Ok(false);
        };
        if let Ok(key) = parse_identity_key_hex(&device.e2e_identity_key) {
            let mut revocations = self.revocations.lock().map_err(|_| PairingError::Unavailable)?;
            if let Some(liste) = revocations.as_mut() {
                liste.revoquer(CibleRevocation::appareil(&key));
            }
        }
        self.persist(&whitelist)?;
        Ok(true)
    }

    // ── Révocation (EX-14) ──────────────────────────────────────────────

    /// Active la liste de révocation du COG `cog` (appelé au démarrage du
    /// serveur, une fois l'identité d'émission chargée). Sans effet si déjà active.
    pub fn activer_revocations(&self, cog: CogId) -> Result<(), PairingError> {
        let mut revocations = self.revocations.lock().map_err(|_| PairingError::Unavailable)?;
        if revocations.is_none() {
            *revocations = Some(ListeRevocation::new(
                cog,
                now_secs(),
                miyauth::revocation::FRAICHEUR_MAX_DEFAUT_SECS,
            ));
        }
        Ok(())
    }

    /// Vrai si la clé d'appareil (hex) figure dans la liste de révocation.
    /// Clé invalide, verrou empoisonné ou horloge incohérente ⇒ vrai
    /// (fail-closed) ; liste non activée ⇒ faux (aucune révocation connue).
    pub fn is_key_revoked(&self, e2e_identity_key_hex: &str) -> bool {
        let Ok(key) = parse_identity_key_hex(e2e_identity_key_hex) else {
            return true;
        };
        let Ok(mut revocations) = self.revocations.lock() else {
            return true;
        };
        let Some(liste) = revocations.as_mut() else {
            return false;
        };
        let maintenant = now_secs();
        liste.rafraichir(maintenant);
        let cog = liste.cog().clone();
        liste
            .est_revoque(&cog, &CibleRevocation::appareil(&key), maintenant)
            .unwrap_or(true)
    }

    /// Appareil appairé dont la clé a pour empreinte `kid`.
    pub fn device_for_key_id(&self, kid: &Kid) -> Option<PairedDevice> {
        self.whitelist.lock().ok().and_then(|w| {
            w.values()
                .find(|d| {
                    parse_identity_key_hex(&d.e2e_identity_key)
                        .is_ok_and(|k| &empreinte_cle(&k) == kid)
                })
                .cloned()
        })
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
        require_same_source: c.require_same_source,
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

/// La liste de révocation est l'état d'autorité **local** du COG (pas une copie
/// distribuée) : elle est à jour à l'instant où elle est lue sous verrou. Liste
/// non activée, verrou empoisonné ou horloge en recul ⇒ erreur (refus).
impl SourceRevocation for PairingManager {
    fn est_revoque(
        &self,
        emetteur: &CogId,
        cible: &CibleRevocation,
        maintenant_secs: u64,
    ) -> Result<bool, ErreurRevocation> {
        let mut revocations = self
            .revocations
            .lock()
            .map_err(|_| ErreurRevocation::Indisponible("verrou de révocation".into()))?;
        let liste = revocations.as_mut().ok_or(ErreurRevocation::ListeAbsente)?;
        liste.rafraichir(maintenant_secs);
        liste.est_revoque(emetteur, cible, maintenant_secs)
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
    use crate::paired_store::DEFAULT_PAIRED_DEVICES_FILE;
    use uuid::Uuid;
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
        identity: E2eIdentity,
        key_hex: String,
        pairing: DevicePairing,
    }

    impl TestDevice {
        /// Nouvel appairage (nouvel aléa Nd) pour la même identité.
        fn restart(&mut self) {
            self.pairing = DevicePairing::start(self.identity.public_key_bytes());
        }
    }

    fn new_device() -> TestDevice {
        let identity = E2eIdentity::generate();
        let key = identity.public_key_bytes();
        TestDevice {
            identity,
            key_hex: hex::encode(key),
            pairing: DevicePairing::start(key),
        }
    }

    fn sig(hex_sig: &str) -> [u8; PAIRING_SIGNATURE_SIZE] {
        miyukini_cog_bridge::parse_pairing_signature_hex(hex_sig).unwrap()
    }

    fn commit(mgr: &PairingManager, id: &str, d: &TestDevice, src: &str) -> Result<PairingNonce, PairingError> {
        let c = parse_commitment_hex(&d.pairing.commitment_hex()).unwrap();
        let s = sig(&d.pairing.commit_signature_hex(&d.identity, id).unwrap());
        mgr.request_pairing(id, "Pixel", &d.key_hex, c, &s, ip(src))
    }

    /// Révélation signée de `d` (consomme l'appairage en cours).
    fn reveal(
        mgr: &PairingManager,
        id: &str,
        d: &mut TestDevice,
        nc: PairingNonce,
        src: &str,
    ) -> (Result<String, PairingError>, String) {
        let pairing = std::mem::replace(&mut d.pairing, DevicePairing::start(d.identity.public_key_bytes()));
        let revealed = pairing.receive_cog_nonce(cog_key(), nc).unwrap();
        let nd = PairingNonce::from_hex(&revealed.device_nonce_hex()).unwrap();
        let s = sig(&revealed.reveal_signature_hex(&d.identity, id).unwrap());
        (mgr.reveal_pairing(id, &nd, &s, ip(src), &cog_key()), revealed.code().to_string())
    }

    /// Appairage honnête complet jusqu'à `Revealed` ; retourne (code appareil, code COG).
    fn commit_and_reveal(mgr: &PairingManager, id: &str, src: &str) -> (TestDevice, String, String) {
        let mut d = new_device();
        let nc = commit(mgr, id, &d, src).unwrap();
        let (cog_code, device_code) = reveal(mgr, id, &mut d, nc, src);
        (d, device_code, cog_code.unwrap())
    }

    /// Révélation valablement signée par `d` d'un aléa `nd` arbitraire.
    fn signed_reveal_of(
        mgr: &PairingManager,
        id: &str,
        d: &TestDevice,
        nd: &PairingNonce,
        nc: &PairingNonce,
        src: &str,
    ) -> Result<String, PairingError> {
        let s = miyukini_cog_bridge::sign_pairing_reveal(&d.identity, id, nd, nc).unwrap();
        mgr.reveal_pairing(id, nd, &s, ip(src), &cog_key())
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
    fn validly_signed_wrong_nonce_aborts() {
        let mgr = PairingManager::new();
        let mut d = new_device();
        let nc = commit(&mgr, "dev-1", &d, IP_A).unwrap();
        // Mauvais Nd, mais signé par Kd : seul le détenteur de la clé peut
        // abandonner sa propre demande.
        assert_eq!(
            signed_reveal_of(&mgr, "dev-1", &d, &PairingNonce::generate(), &nc, IP_A).unwrap_err(),
            PairingError::CommitmentMismatch
        );
        assert_eq!(mgr.pending_devices().len(), 0);
        assert_eq!(mgr.pairing_status("dev-1", &d.key_hex), PairingStatus::Rejected);
        // La bonne révélation arrive trop tard : plus d'entrée.
        assert_eq!(reveal(&mgr, "dev-1", &mut d, nc, IP_A).0.unwrap_err(), PairingError::NoPending);
    }

    #[test]
    fn reveal_with_invalid_signature_leaves_entry_intact() {
        let mgr = PairingManager::new();
        let mut d = new_device();
        let nc = commit(&mgr, "dev-1", &d, IP_A).unwrap();
        let attacker = new_device();
        let wrong_nd = PairingNonce::generate();
        // 1. Signature d'une autre clé (l'attaquant connaît device_id).
        let s = miyukini_cog_bridge::sign_pairing_reveal(&attacker.identity, "dev-1", &wrong_nd, &nc).unwrap();
        assert_eq!(
            mgr.reveal_pairing("dev-1", &wrong_nd, &s, ip(IP_A), &cog_key()).unwrap_err(),
            PairingError::InvalidSignature
        );
        // 2. Signature nulle / altérée.
        assert_eq!(
            mgr.reveal_pairing("dev-1", &wrong_nd, &[0u8; 64], ip(IP_A), &cog_key()).unwrap_err(),
            PairingError::InvalidSignature
        );
        // 3. Signature valide de Kd sur un autre Nc (rejeu d'une ancienne révélation).
        let old_nc = PairingNonce::generate();
        assert_eq!(
            signed_reveal_of(&mgr, "dev-1", &d, &wrong_nd, &old_nc, IP_A).unwrap_err(),
            PairingError::InvalidSignature
        );
        // La demande est intacte : la révélation honnête aboutit.
        assert_eq!(mgr.pairing_status("dev-1", &d.key_hex), PairingStatus::Committed);
        let (code, device_code) = reveal(&mgr, "dev-1", &mut d, nc, IP_A);
        assert_eq!(code.unwrap(), device_code);
    }

    #[test]
    fn reveal_from_other_source_leaves_entry_intact() {
        let mgr = PairingManager::new();
        let mut d = new_device();
        let nc = commit(&mgr, "dev-1", &d, IP_A).unwrap();
        // Même révélation signée (capturée) rejouée depuis une autre IP,
        // y compris avec un mauvais Nd valablement signé : refus sans effet.
        assert_eq!(
            signed_reveal_of(&mgr, "dev-1", &d, &PairingNonce::generate(), &nc, IP_B).unwrap_err(),
            PairingError::SourceMismatch
        );
        assert_eq!(mgr.pairing_status("dev-1", &d.key_hex), PairingStatus::Committed);
        assert!(reveal(&mgr, "dev-1", &mut d, nc, IP_A).0.is_ok());
    }

    #[test]
    fn source_binding_can_be_disabled() {
        let mgr = PairingManager::with_config(PairingConfig {
                require_same_source: false,
                ..PairingConfig::default()
            },
        );
        let mut d = new_device();
        let nc = commit(&mgr, "dev-1", &d, IP_A).unwrap();
        assert!(reveal(&mgr, "dev-1", &mut d, nc, IP_B).0.is_ok());
    }

    #[test]
    fn unsigned_or_wrongly_signed_commit_refused() {
        let mgr = PairingManager::new();
        let d = new_device();
        let c = parse_commitment_hex(&d.pairing.commitment_hex()).unwrap();
        // Signature nulle.
        assert_eq!(
            mgr.request_pairing("dev-1", "Pixel", &d.key_hex, c, &[0u8; 64], ip(IP_A)).unwrap_err(),
            PairingError::InvalidSignature
        );
        // Signature par une autre clé que celle présentée.
        let other = new_device();
        let s = miyukini_cog_bridge::sign_pairing_commit(&other.identity, "dev-1", &c).unwrap();
        assert_eq!(
            mgr.request_pairing("dev-1", "Pixel", &d.key_hex, c, &s, ip(IP_A)).unwrap_err(),
            PairingError::InvalidSignature
        );
        // Signature valide pour un autre device_id.
        let s = sig(&d.pairing.commit_signature_hex(&d.identity, "dev-2").unwrap());
        assert_eq!(
            mgr.request_pairing("dev-1", "Pixel", &d.key_hex, c, &s, ip(IP_A)).unwrap_err(),
            PairingError::InvalidSignature
        );
        assert_eq!(mgr.pending_devices().len(), 0);
    }

    #[test]
    fn unsigned_commit_never_touches_pending_entry() {
        let mgr = PairingManager::new();
        let (legit, _, code) = commit_and_reveal(&mgr, "dev-1", IP_A);
        let c = [0x77; 32];
        assert_eq!(
            mgr.request_pairing("dev-1", "Pixel", &legit.key_hex, c, &[0u8; 64], ip(IP_A)).unwrap_err(),
            PairingError::InvalidSignature
        );
        assert_eq!(mgr.pending_devices()[0].confirmation_code.as_deref(), Some(code.as_str()));
    }

    #[test]
    fn reveal_before_cog_nonce_refused() {
        // Aucune entrée (donc aucun Nc émis) : révélation refusée.
        let mgr = PairingManager::new();
        let d = new_device();
        let nd = PairingNonce::generate();
        assert_eq!(
            signed_reveal_of(&mgr, "dev-1", &d, &nd, &PairingNonce::generate(), IP_A).unwrap_err(),
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
        let s = sig(&revealed.reveal_signature_hex(&d.identity, "dev-1").unwrap());
        let code = mgr.reveal_pairing("dev-1", &nd, &s, ip(IP_A), &cog_key()).unwrap();
        assert_eq!(
            mgr.reveal_pairing("dev-1", &nd, &s, ip(IP_A), &cog_key()).unwrap_err(),
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
    fn same_key_restart_before_reveal_allowed() {
        let mgr = PairingManager::new();
        let mut d = new_device();
        let old_nc = commit(&mgr, "dev-1", &d, IP_A).unwrap();
        // Rejeu de la même requête signée (même engagement) : refusé.
        assert_eq!(commit(&mgr, "dev-1", &d, IP_A).unwrap_err(), PairingError::PendingConflict);
        // Relance honnête (nouvel Nd), signée, même source, avant révélation.
        d.restart();
        let nc = commit(&mgr, "dev-1", &d, IP_A).unwrap();
        assert_eq!(mgr.pending_devices().len(), 1);
        // L'ancien Nc n'est plus valable ; le nouveau oui.
        let stale = signed_reveal_of(&mgr, "dev-1", &d, &PairingNonce::generate(), &old_nc, IP_A);
        assert_eq!(stale.unwrap_err(), PairingError::InvalidSignature);
        assert!(reveal(&mgr, "dev-1", &mut d, nc, IP_A).0.is_ok());
    }

    #[test]
    fn same_key_restart_after_reveal_refused() {
        let mgr = PairingManager::new();
        let (mut d, _, code) = commit_and_reveal(&mgr, "dev-1", IP_A);
        d.restart();
        assert_eq!(commit(&mgr, "dev-1", &d, IP_A).unwrap_err(), PairingError::PendingConflict);
        // Le code affiché sur Central reste celui de la demande révélée.
        assert_eq!(mgr.pending_devices()[0].confirmation_code.as_deref(), Some(code.as_str()));
        mgr.approve_device("dev-1", &code).unwrap();
    }

    #[test]
    fn same_key_restart_from_other_source_refused() {
        let mgr = PairingManager::new();
        let mut d = new_device();
        let nc = commit(&mgr, "dev-1", &d, IP_A).unwrap();
        // Relance (nouvel Nd, signée par Kd) depuis une autre adresse.
        let retry = TestDevice {
            identity: E2eIdentity::from_secret_bytes(&d.identity.secret_bytes()),
            key_hex: d.key_hex.clone(),
            pairing: DevicePairing::start(d.identity.public_key_bytes()),
        };
        assert_eq!(commit(&mgr, "dev-1", &retry, IP_B).unwrap_err(), PairingError::SourceMismatch);
        // L'entrée d'origine (et son Nc) est intacte : la révélation initiale aboutit.
        assert!(reveal(&mgr, "dev-1", &mut d, nc, IP_A).0.is_ok());
    }

    #[test]
    fn pending_expires_after_ttl() {
        let mgr = PairingManager::with_config(PairingConfig {
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
        let mgr = PairingManager::with_config(PairingConfig {
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
        let s = sig(&d.pairing.commit_signature_hex(&d.identity, "dev-0").unwrap());
        mgr.request_pairing("dev-0", &"n".repeat(500), &d.key_hex, c, &s, ip(IP_A)).unwrap();
        assert_eq!(mgr.pending_devices()[0].name.chars().count(), MAX_DEVICE_NAME_CHARS);
        assert!(mgr.device_for_identity_key("").is_none());
        // Configuration nulle : bornes minimales appliquées.
        let c = sanitize_config(PairingConfig {
            pending_ttl: Duration::ZERO,
            max_pending: 0,
            max_pending_per_source: 0,
            require_same_source: true,
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
            let mgr = PairingManager::open(PairingConfig::default(), path.clone()).unwrap();
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
        let mgr = PairingManager::open(PairingConfig::default(), path).unwrap();
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
            PairingManager::open(PairingConfig::default(), path),
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
        let mgr = PairingManager::with_config(PairingConfig::default());
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

    fn cog() -> CogId {
        CogId::new("abcdefghijklmnopqrstuvwxyz").unwrap()
    }

    #[test]
    fn revocation_adds_key_to_list_and_blocks_repairing() {
        let mgr = PairingManager::new();
        mgr.activer_revocations(cog()).unwrap();
        let (d, _, code) = commit_and_reveal(&mgr, "dev-1", IP_A);
        mgr.approve_device_for("dev-1", &code, Some("prof-1")).unwrap();
        let key = parse_identity_key_hex(&d.key_hex).unwrap();
        let cible = CibleRevocation::appareil(&key);
        assert_eq!(mgr.est_revoque(&cog(), &cible, now_secs()), Ok(false));
        assert_eq!(mgr.paired_devices()[0].profile_id.as_deref(), Some("prof-1"));
        assert!(mgr.device_for_key_id(&empreinte_cle(&key)).is_some());

        assert!(mgr.revoke_device("dev-1").unwrap());
        assert_eq!(mgr.est_revoque(&cog(), &cible, now_secs()), Ok(true));
        assert!(mgr.is_key_revoked(&d.key_hex));
        assert!(mgr.device_for_key_id(&empreinte_cle(&key)).is_none());
        // Même clé : ré-appairage refusé.
        assert_eq!(commit(&mgr, "dev-1", &d, IP_A).unwrap_err(), PairingError::KeyRevoked);
    }

    #[test]
    fn revocation_source_fails_closed() {
        let mgr = PairingManager::new();
        let cible = CibleRevocation::appareil(&cog_key());
        // Liste non activée : refus.
        assert_eq!(mgr.est_revoque(&cog(), &cible, now_secs()), Err(ErreurRevocation::ListeAbsente));
        mgr.activer_revocations(cog()).unwrap();
        // Autre COG : refus.
        let autre = CogId::new("zyxwvutsrqponmlkjihgfedcba").unwrap();
        assert_eq!(mgr.est_revoque(&autre, &cible, now_secs()), Err(ErreurRevocation::ListeAbsente));
        // Horloge en recul : refus.
        assert!(matches!(mgr.est_revoque(&cog(), &cible, 1), Err(ErreurRevocation::HorlogeIncoherente)));
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
