//! @id     miyukini_cog_bridge_e2e
//! @do     definir_nonce_size
//! @role   orchestration
//! @layer  core
//! @human  miyukini-cog-bridge — module « e2e ».

//! Chiffrement de bout en bout (E2E) entre le client Android et le COG Host.
//!
//! ## Protocole v2 (R-019)
//!
//! La version 1 (clé X25519 non signée, « SHA-256 » maison, compteur de nonce
//! partagé entre les deux sens) est **refusée explicitement** : toute offre ou
//! tout message de version différente de [`E2E_PROTOCOL_VERSION`] produit
//! [`E2eError::UnsupportedVersion`], sans repli silencieux.
//!
//! 1. **Identités** : l'appareil (Central Mobile) et le COG possèdent chacun
//!    une clé d'identité ed25519. Chaque côté connaît déjà la clé publique du
//!    pair (appairage hors bande : QR code, validation sur le desktop).
//! 2. **Offre signée** : chaque côté génère une clé X25519 éphémère et signe le
//!    transcript (voir [`transcript`]) :
//!    `HANDSHAKE_DOMAIN || version || rôle émetteur || identité émetteur ||
//!    identité destinataire || clé éphémère`.
//! 3. **Vérification avant dérivation** (C-23) : version, clé éphémère non
//!    réfléchie, rôle opposé, identité égale à celle épinglée, signature
//!    `verify_strict`, puis X25519 contributif. Toute anomalie = refus (C-12).
//! 4. **Dérivation** : HKDF-SHA256 (sel = domaine du handshake, info = domaine,
//!    identités et clés éphémères dans l'ordre appareil puis COG) produit
//!    deux clés directionnelles : appareil → COG et COG → appareil.
//! 5. **Chiffrement** : XChaCha20-Poly1305, nonce aléatoire de 192 bits tiré
//!    en interne (`OsRng`) — l'appelant ne fournit jamais de nonce. Les données
//!    associées lient le sens d'émission, l'identifiant de session, un numéro
//!    de séquence par sens, l'identifiant de requête, la méthode et le point
//!    d'accès HTTP (voir [`RequestBinding`]).
//! 6. **Anti-rejeu** : chaque sens a une séquence u64 strictement croissante ;
//!    le récepteur refuse toute valeur ≤ à la dernière acceptée (pas de fenêtre :
//!    échanges requête/réponse séquentiels). La réponse doit porter le même
//!    identifiant de requête que la requête.
//!
//! ## Garanties
//!
//! - **Authentification mutuelle** : un intermédiaire ne peut pas substituer
//!   sa clé éphémère (signature liée aux deux identités).
//! - **Pas de réutilisation de nonce** : une clé par sens + nonces aléatoires
//!   de 192 bits.
//! - **Forward secrecy** : les clés éphémères sont consommées par le handshake.
//! - **Pas de rejeu** : un message rejoué, plus ancien, ou présenté sur un autre
//!   point d'accès est refusé ; une réponse d'une autre requête est refusée.
//! - **Hygiène des secrets** : clés effacées à la destruction (C-25), jamais
//!   affichées par `Debug`.

use chacha20poly1305::aead::{Aead, KeyInit, Payload};
use chacha20poly1305::{XChaCha20Poly1305, XNonce};
use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
use hkdf::Hkdf;
use rand::rngs::OsRng;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::sync::atomic::{AtomicU64, Ordering};
use x25519_dalek::{EphemeralSecret, PublicKey};
use zeroize::Zeroizing;

use crate::{BridgeError, BridgeResult};

/// Version courante du protocole E2E du bridge.
pub const E2E_PROTOCOL_VERSION: u8 = 2;

/// Domaine de séparation des signatures de handshake (et sel HKDF).
pub const HANDSHAKE_DOMAIN: &[u8] = b"miyukini/cog-bridge/e2e-handshake/v2";

/// Domaine HKDF des clés de session directionnelles.
const SESSION_KDF_INFO: &[u8] = b"miyukini/cog-bridge/e2e-session-keys/v2";

/// Domaine de l'identifiant de session (SHA-256).
const SESSION_ID_DOMAIN: &[u8] = b"miyukini/cog-bridge/e2e-session-id/v2";

/// Domaine des données associées (AAD) des messages.
const MESSAGE_AAD_DOMAIN: &[u8] = b"miyukini/cog-bridge/e2e-msg/v2";

/// Domaine du code de confirmation d'appairage.
const PAIRING_CODE_DOMAIN: &[u8] = b"miyukini/cog-bridge/pairing-code/v2";

/// En-tête HTTP portant l'identifiant (public) de session E2E, en hex.
pub const E2E_SESSION_HEADER: &str = "x-miyukini-e2e-session";

/// Taille de l'identifiant de requête (128 bits).
pub const REQUEST_ID_SIZE: usize = 16;

/// Longueur maximale d'une méthode HTTP liée dans l'AAD (C-13).
pub const MAX_METHOD_LEN: usize = 16;

/// Longueur maximale d'un point d'accès lié dans l'AAD (C-13).
pub const MAX_ENDPOINT_LEN: usize = 256;

/// Taille du nonce XChaCha20 (192 bits = 24 octets).
pub const NONCE_SIZE: usize = 24;

/// Taille du tag Poly1305.
const TAG_SIZE: usize = 16;

/// Taille maximale d'un clair chiffré par le bridge (C-13).
pub const MAX_PLAINTEXT_LEN: usize = 4 * 1024 * 1024;

/// Taille maximale d'une offre de handshake sérialisée en JSON (C-13).
pub const MAX_HANDSHAKE_JSON_LEN: usize = 1024;

/// Taille maximale d'un message chiffré sérialisé en JSON (hex + enveloppe).
pub const MAX_MESSAGE_JSON_LEN: usize = 2 * (MAX_PLAINTEXT_LEN + TAG_SIZE) + 2 * NONCE_SIZE + 256;

// ═══════════════════════════════════════════════════════════════════════════
// Erreurs
// ═══════════════════════════════════════════════════════════════════════════

/// Causes de refus du protocole E2E (une variante par cause, C-11).
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum E2eError {
    #[error("version de protocole E2E non supportée: {0} (attendue: {E2E_PROTOCOL_VERSION})")]
    UnsupportedVersion(u8),

    #[error("rôle inattendu dans l'offre de handshake")]
    UnexpectedRole,

    #[error("clé d'identité du pair différente de celle épinglée")]
    IdentityMismatch,

    #[error("clé d'identité ed25519 invalide ou faible")]
    InvalidIdentityKey,

    #[error("handshake avec sa propre identité")]
    SelfHandshake,

    #[error("clé éphémère réfléchie")]
    ReflectedKey,

    #[error("signature de handshake invalide")]
    InvalidSignature,

    #[error("échange X25519 non contributif (clé éphémère faible)")]
    NonContributory,

    #[error("champ mal formé: {0}")]
    Malformed(&'static str),

    #[error("taille hors bornes: {0}")]
    TooLarge(&'static str),

    #[error("dérivation HKDF échouée")]
    Kdf,

    #[error("chiffrement échoué")]
    Encrypt,

    #[error("déchiffrement refusé (authentification AEAD)")]
    Decrypt,

    #[error("message rejoué ou hors séquence")]
    Replay,

    #[error("compteur de séquence épuisé (nouveau handshake requis)")]
    SequenceExhausted,

    #[error("identifiant de requête différent de celui attendu")]
    RequestIdMismatch,

    #[error("identités E2E non configurées (appairage requis)")]
    MissingIdentity,

    #[error("JSON: {0}")]
    Json(String),
}

// ═══════════════════════════════════════════════════════════════════════════
// Rôles et identités
// ═══════════════════════════════════════════════════════════════════════════

/// Rôle d'un pair dans le bridge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum E2eRole {
    /// Appareil mobile (Central Mobile, Android).
    Device,
    /// COG Host (desktop de l'utilisateur).
    Cog,
}

impl E2eRole {
    /// Code binaire du rôle dans le transcript et l'AAD.
    pub const fn code(self) -> u8 {
        match self {
            Self::Device => 0x01,
            Self::Cog => 0x02,
        }
    }

    /// Rôle attendu du pair.
    pub const fn peer(self) -> Self {
        match self {
            Self::Device => Self::Cog,
            Self::Cog => Self::Device,
        }
    }
}

/// Identité ed25519 d'un pair (appareil ou COG). Effacée à la destruction.
pub struct E2eIdentity {
    signing: SigningKey,
}

impl std::fmt::Debug for E2eIdentity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("E2eIdentity")
            .field("public_key", &hex::encode(self.public_key_bytes()))
            .field("secret", &"<redacted>")
            .finish()
    }
}

impl E2eIdentity {
    /// Génère une nouvelle identité (OsRng).
    pub fn generate() -> Self {
        Self {
            signing: SigningKey::generate(&mut OsRng),
        }
    }

    /// Reconstruit une identité depuis sa graine secrète (stockage sécurisé).
    pub fn from_secret_bytes(secret: &[u8; 32]) -> Self {
        Self {
            signing: SigningKey::from_bytes(secret),
        }
    }

    /// Graine secrète (32 octets) pour le stockage sécurisé ; effacée à la
    /// destruction du conteneur. Ne jamais journaliser.
    pub fn secret_bytes(&self) -> Zeroizing<[u8; 32]> {
        Zeroizing::new(self.signing.to_bytes())
    }

    /// Clé publique d'identité (32 octets) à communiquer au pair lors de l'appairage.
    pub fn public_key_bytes(&self) -> [u8; 32] {
        self.signing.verifying_key().to_bytes()
    }

    fn sign(&self, message: &[u8]) -> [u8; 64] {
        self.signing.sign(message).to_bytes()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Handshake
// ═══════════════════════════════════════════════════════════════════════════

/// Offre de handshake (en clair sur le réseau, mais signée).
///
/// Format JSON v2 :
/// `{"protocol_version":2,"role":"device"|"cog","identity_key":<hex 32>,
///   "ephemeral_key":<hex 32>,"signature":<hex 64>}`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct E2eHandshake {
    /// Version du protocole E2E (doit valoir [`E2E_PROTOCOL_VERSION`]).
    pub protocol_version: u8,
    /// Rôle de l'émetteur.
    pub role: E2eRole,
    /// Clé d'identité ed25519 de l'émetteur (32 octets, hex).
    pub identity_key: String,
    /// Clé publique X25519 éphémère (32 octets, hex).
    pub ephemeral_key: String,
    /// Signature ed25519 du transcript (64 octets, hex).
    pub signature: String,
}

/// Sonde de version : lue avant toute autre interprétation du JSON, pour
/// refuser explicitement les versions inconnues (v1 comprise).
#[derive(Deserialize)]
struct VersionProbe {
    #[serde(default = "legacy_version")]
    protocol_version: u8,
}

/// Les messages v1 ne portaient pas de champ de version.
const fn legacy_version() -> u8 {
    1
}

fn probe_version(bytes: &[u8]) -> Result<(), E2eError> {
    let probe: VersionProbe =
        serde_json::from_slice(bytes).map_err(|e| E2eError::Json(e.to_string()))?;
    if probe.protocol_version != E2E_PROTOCOL_VERSION {
        return Err(E2eError::UnsupportedVersion(probe.protocol_version));
    }
    Ok(())
}

impl E2eHandshake {
    /// Décode une offre reçue du réseau (taille bornée, version vérifiée d'abord).
    pub fn from_json(bytes: &[u8]) -> Result<Self, E2eError> {
        if bytes.len() > MAX_HANDSHAKE_JSON_LEN {
            return Err(E2eError::TooLarge("offre de handshake"));
        }
        probe_version(bytes)?;
        serde_json::from_slice(bytes).map_err(|e| E2eError::Json(e.to_string()))
    }
}

/// Construit le transcript signé par l'émetteur `sender_role` à destination
/// du pair d'identité `recipient_identity`.
///
/// Encodage (taille fixe, sans ambiguïté) :
/// `HANDSHAKE_DOMAIN || version (1) || rôle (1) || identité émetteur (32) ||
///  identité destinataire (32) || clé éphémère (32)`.
fn transcript(
    sender_role: E2eRole,
    sender_identity: &[u8; 32],
    recipient_identity: &[u8; 32],
    ephemeral: &[u8; 32],
) -> Vec<u8> {
    let mut m = Vec::with_capacity(HANDSHAKE_DOMAIN.len() + 2 + 96);
    m.extend_from_slice(HANDSHAKE_DOMAIN);
    m.push(E2E_PROTOCOL_VERSION);
    m.push(sender_role.code());
    m.extend_from_slice(sender_identity);
    m.extend_from_slice(recipient_identity);
    m.extend_from_slice(ephemeral);
    m
}

fn decode_hex_fixed<const N: usize>(s: &str, what: &'static str) -> Result<[u8; N], E2eError> {
    if s.len() != 2 * N {
        return Err(E2eError::Malformed(what));
    }
    let bytes = hex::decode(s).map_err(|_| E2eError::Malformed(what))?;
    bytes.try_into().map_err(|_| E2eError::Malformed(what))
}

/// Matériel de session dérivé : clés directionnelles (64 octets) et identifiant.
struct SessionMaterial {
    /// `[..32]` : appareil → COG ; `[32..]` : COG → appareil.
    okm: Zeroizing<[u8; 64]>,
    session_id: [u8; 32],
}

/// Dérive les clés de session à partir du secret X25519 et des éléments publics
/// du handshake, toujours dans l'ordre appareil puis COG.
fn derive_session_material(
    shared: &[u8; 32],
    device_identity: &[u8; 32],
    device_ephemeral: &[u8; 32],
    cog_identity: &[u8; 32],
    cog_ephemeral: &[u8; 32],
) -> Result<SessionMaterial, E2eError> {
    let mut publics = Vec::with_capacity(1 + 128);
    publics.push(E2E_PROTOCOL_VERSION);
    publics.extend_from_slice(device_identity);
    publics.extend_from_slice(device_ephemeral);
    publics.extend_from_slice(cog_identity);
    publics.extend_from_slice(cog_ephemeral);

    let mut info = Vec::with_capacity(SESSION_KDF_INFO.len() + publics.len());
    info.extend_from_slice(SESSION_KDF_INFO);
    info.extend_from_slice(&publics);

    let hk = Hkdf::<Sha256>::new(Some(HANDSHAKE_DOMAIN), shared);
    let mut okm = Zeroizing::new([0u8; 64]);
    hk.expand(&info, okm.as_mut()).map_err(|_| E2eError::Kdf)?;

    let mut id_input = Vec::with_capacity(SESSION_ID_DOMAIN.len() + publics.len());
    id_input.extend_from_slice(SESSION_ID_DOMAIN);
    id_input.extend_from_slice(&publics);

    Ok(SessionMaterial {
        okm,
        session_id: sha256(&id_input),
    })
}

/// Handshake en cours : la clé éphémère privée attend l'offre du pair.
pub struct PendingHandshake {
    role: E2eRole,
    local_identity: [u8; 32],
    peer_identity: VerifyingKey,
    ephemeral: EphemeralSecret,
    ephemeral_public: [u8; 32],
}

impl std::fmt::Debug for PendingHandshake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PendingHandshake")
            .field("role", &self.role)
            .field("ephemeral", &"<redacted>")
            .finish_non_exhaustive()
    }
}

impl PendingHandshake {
    /// Démarre un handshake en tant que `role`, avec un pair dont la clé
    /// d'identité `peer_identity` est déjà connue et de confiance (appairage).
    ///
    /// Retourne l'état en attente et l'offre signée à envoyer au pair.
    pub fn initiate(
        role: E2eRole,
        local: &E2eIdentity,
        peer_identity: &[u8; 32],
    ) -> Result<(Self, E2eHandshake), E2eError> {
        let peer_vk =
            VerifyingKey::from_bytes(peer_identity).map_err(|_| E2eError::InvalidIdentityKey)?;
        if peer_vk.is_weak() {
            return Err(E2eError::InvalidIdentityKey);
        }
        let local_identity = local.public_key_bytes();
        if &local_identity == peer_identity {
            return Err(E2eError::SelfHandshake);
        }

        let ephemeral = EphemeralSecret::random_from_rng(OsRng);
        let ephemeral_public = *PublicKey::from(&ephemeral).as_bytes();

        let signature = local.sign(&transcript(
            role,
            &local_identity,
            peer_identity,
            &ephemeral_public,
        ));

        let offer = E2eHandshake {
            protocol_version: E2E_PROTOCOL_VERSION,
            role,
            identity_key: hex::encode(local_identity),
            ephemeral_key: hex::encode(ephemeral_public),
            signature: hex::encode(signature),
        };
        Ok((
            Self {
                role,
                local_identity,
                peer_identity: peer_vk,
                ephemeral,
                ephemeral_public,
            },
            offer,
        ))
    }

    /// Termine le handshake avec l'offre reçue du pair (fail-closed).
    ///
    /// Refuse si : version inconnue, clé éphémère réfléchie, rôle non opposé,
    /// identité différente de celle épinglée, signature invalide (vérifiée
    /// avec `verify_strict` **avant** toute dérivation), DH non contributif.
    pub fn complete(self, offer: &E2eHandshake) -> Result<E2eSession, E2eError> {
        if offer.protocol_version != E2E_PROTOCOL_VERSION {
            return Err(E2eError::UnsupportedVersion(offer.protocol_version));
        }
        let peer_ephemeral: [u8; 32] = decode_hex_fixed(&offer.ephemeral_key, "ephemeral_key")?;
        if peer_ephemeral == self.ephemeral_public {
            return Err(E2eError::ReflectedKey);
        }
        if offer.role != self.role.peer() {
            return Err(E2eError::UnexpectedRole);
        }
        let claimed_identity: [u8; 32] = decode_hex_fixed(&offer.identity_key, "identity_key")?;
        let peer_identity = self.peer_identity.to_bytes();
        if claimed_identity != peer_identity {
            return Err(E2eError::IdentityMismatch);
        }
        let signature: [u8; 64] = decode_hex_fixed(&offer.signature, "signature")?;
        let signature = Signature::from_bytes(&signature);

        // Signature vérifiée AVANT toute dérivation (C-23).
        let msg = transcript(
            offer.role,
            &peer_identity,
            &self.local_identity,
            &peer_ephemeral,
        );
        self.peer_identity
            .verify_strict(&msg, &signature)
            .map_err(|_| E2eError::InvalidSignature)?;

        let shared = self
            .ephemeral
            .diffie_hellman(&PublicKey::from(peer_ephemeral));
        if !shared.was_contributory() {
            return Err(E2eError::NonContributory);
        }

        let material = match self.role {
            E2eRole::Device => derive_session_material(
                shared.as_bytes(),
                &self.local_identity,
                &self.ephemeral_public,
                &peer_identity,
                &peer_ephemeral,
            )?,
            E2eRole::Cog => derive_session_material(
                shared.as_bytes(),
                &peer_identity,
                &peer_ephemeral,
                &self.local_identity,
                &self.ephemeral_public,
            )?,
        };
        Ok(E2eSession::from_material(self.role, &material))
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Liaison requête / réponse (anti-rejeu)
// ═══════════════════════════════════════════════════════════════════════════

/// Liaison d'un message à une requête : identifiant aléatoire, méthode et
/// point d'accès HTTP. Liée dans l'AAD : un chiffré produit pour un point
/// d'accès A n'est pas accepté sur un point d'accès B, et la réponse doit
/// porter le même identifiant de requête.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestBinding {
    request_id: [u8; REQUEST_ID_SIZE],
    method: String,
    endpoint: String,
}

impl RequestBinding {
    /// Nouvelle liaison pour une requête émise (identifiant tiré via `OsRng`).
    pub fn new(method: &str, endpoint: &str) -> Result<Self, E2eError> {
        let mut request_id = [0u8; REQUEST_ID_SIZE];
        OsRng.fill_bytes(&mut request_id);
        Self::with_request_id(request_id, method, endpoint)
    }

    /// Liaison d'une requête reçue : l'identifiant vient du message, la
    /// méthode et le point d'accès viennent de la requête HTTP réellement reçue.
    pub fn for_received(
        msg: &EncryptedMessage,
        method: &str,
        endpoint: &str,
    ) -> Result<Self, E2eError> {
        Self::with_request_id(msg.request_id, method, endpoint)
    }

    fn with_request_id(
        request_id: [u8; REQUEST_ID_SIZE],
        method: &str,
        endpoint: &str,
    ) -> Result<Self, E2eError> {
        if method.is_empty() || method.len() > MAX_METHOD_LEN {
            return Err(E2eError::TooLarge("méthode"));
        }
        if endpoint.is_empty() || endpoint.len() > MAX_ENDPOINT_LEN {
            return Err(E2eError::TooLarge("point d'accès"));
        }
        Ok(Self {
            request_id,
            method: method.to_string(),
            endpoint: endpoint.to_string(),
        })
    }

    /// Identifiant de la requête.
    pub fn request_id(&self) -> [u8; REQUEST_ID_SIZE] {
        self.request_id
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Session
// ═══════════════════════════════════════════════════════════════════════════

/// Session E2E établie — chiffre et déchiffre les messages.
///
/// Deux clés distinctes (une par sens) ; les ciphers effacent leur clé à la
/// destruction. Chaque sens porte un numéro de séquence strictement croissant
/// (lié dans l'AAD) : le récepteur refuse toute valeur inférieure ou égale à
/// la dernière acceptée (rejeu, désordre).
pub struct E2eSession {
    role: E2eRole,
    send: XChaCha20Poly1305,
    recv: XChaCha20Poly1305,
    session_id: [u8; 32],
    /// Dernier numéro de séquence émis (0 = aucun).
    send_seq: AtomicU64,
    /// Dernier numéro de séquence accepté en réception (0 = aucun).
    recv_seq: AtomicU64,
}

impl std::fmt::Debug for E2eSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("E2eSession")
            .field("role", &self.role)
            .field("session_id", &hex::encode(self.session_id))
            .field("keys", &"<redacted>")
            .finish_non_exhaustive()
    }
}

impl E2eSession {
    fn from_material(role: E2eRole, material: &SessionMaterial) -> Self {
        let (device_to_cog, cog_to_device) = material.okm.split_at(32);
        let (send, recv) = match role {
            E2eRole::Device => (device_to_cog, cog_to_device),
            E2eRole::Cog => (cog_to_device, device_to_cog),
        };
        Self {
            role,
            send: XChaCha20Poly1305::new(send.into()),
            recv: XChaCha20Poly1305::new(recv.into()),
            session_id: material.session_id,
            send_seq: AtomicU64::new(0),
            recv_seq: AtomicU64::new(0),
        }
    }

    /// Rôle local de la session.
    pub fn role(&self) -> E2eRole {
        self.role
    }

    /// Identifiant public de la session (SHA-256 des éléments publics du handshake).
    pub fn session_id(&self) -> [u8; 32] {
        self.session_id
    }

    /// AAD : domaine || version || rôle de l'émetteur || identifiant de session
    /// || séquence (u64 BE) || identifiant de requête (16) || len(méthode) (u8)
    /// || méthode || len(point d'accès) (u16 BE) || point d'accès.
    fn aad(&self, sender: E2eRole, seq: u64, binding: &RequestBinding) -> Vec<u8> {
        let mut aad = Vec::with_capacity(
            MESSAGE_AAD_DOMAIN.len()
                + 2
                + 32
                + 8
                + REQUEST_ID_SIZE
                + 3
                + binding.method.len()
                + binding.endpoint.len(),
        );
        aad.extend_from_slice(MESSAGE_AAD_DOMAIN);
        aad.push(E2E_PROTOCOL_VERSION);
        aad.push(sender.code());
        aad.extend_from_slice(&self.session_id);
        aad.extend_from_slice(&seq.to_be_bytes());
        aad.extend_from_slice(&binding.request_id);
        // Longueurs bornées à la construction de la liaison (≤ 16 et ≤ 256).
        aad.push(u8::try_from(binding.method.len()).unwrap_or(u8::MAX));
        aad.extend_from_slice(binding.method.as_bytes());
        aad.extend_from_slice(
            &u16::try_from(binding.endpoint.len())
                .unwrap_or(u16::MAX)
                .to_be_bytes(),
        );
        aad.extend_from_slice(binding.endpoint.as_bytes());
        aad
    }

    /// Réserve le prochain numéro de séquence d'émission (strictement croissant).
    fn next_send_seq(&self) -> Result<u64, E2eError> {
        advance_counter(&self.send_seq, |last| last.checked_add(1))
            .ok_or(E2eError::SequenceExhausted)
    }

    /// Chiffre un message vers le pair, lié à `binding`. Le nonce (192 bits)
    /// est tiré en interne, le numéro de séquence est attribué en interne.
    pub fn encrypt(
        &self,
        binding: &RequestBinding,
        plaintext: &[u8],
    ) -> BridgeResult<EncryptedMessage> {
        if plaintext.len() > MAX_PLAINTEXT_LEN {
            return Err(E2eError::TooLarge("clair").into());
        }
        let seq = self.next_send_seq()?;
        let mut nonce = [0u8; NONCE_SIZE];
        OsRng.fill_bytes(&mut nonce);
        Ok(self.seal_with_nonce(nonce, seq, binding, plaintext)?)
    }

    fn seal_with_nonce(
        &self,
        nonce: [u8; NONCE_SIZE],
        seq: u64,
        binding: &RequestBinding,
        plaintext: &[u8],
    ) -> Result<EncryptedMessage, E2eError> {
        if plaintext.len() > MAX_PLAINTEXT_LEN {
            return Err(E2eError::TooLarge("clair"));
        }
        let aad = self.aad(self.role, seq, binding);
        let ciphertext = self
            .send
            .encrypt(
                XNonce::from_slice(&nonce),
                Payload {
                    msg: plaintext,
                    aad: &aad,
                },
            )
            .map_err(|_| E2eError::Encrypt)?;
        Ok(EncryptedMessage {
            protocol_version: E2E_PROTOCOL_VERSION,
            seq,
            request_id: binding.request_id,
            nonce,
            ciphertext,
        })
    }

    /// Déchiffre un message reçu du pair, lié à `binding` (version, identifiant
    /// de requête, taille, AEAD puis séquence strictement croissante vérifiés).
    ///
    /// La séquence n'est enregistrée qu'après authentification AEAD : un message
    /// forgé ne peut pas faire avancer le compteur.
    pub fn decrypt(
        &self,
        binding: &RequestBinding,
        msg: &EncryptedMessage,
    ) -> BridgeResult<Vec<u8>> {
        if msg.protocol_version != E2E_PROTOCOL_VERSION {
            return Err(E2eError::UnsupportedVersion(msg.protocol_version).into());
        }
        if msg.request_id != binding.request_id {
            return Err(E2eError::RequestIdMismatch.into());
        }
        if msg.ciphertext.len() > MAX_PLAINTEXT_LEN + TAG_SIZE {
            return Err(E2eError::TooLarge("chiffré").into());
        }
        if msg.seq == 0 || msg.seq <= self.recv_seq.load(Ordering::SeqCst) {
            return Err(E2eError::Replay.into());
        }
        let aad = self.aad(self.role.peer(), msg.seq, binding);
        let plaintext = self
            .recv
            .decrypt(
                XNonce::from_slice(&msg.nonce),
                Payload {
                    msg: &msg.ciphertext,
                    aad: &aad,
                },
            )
            .map_err(|_| BridgeError::E2e(E2eError::Decrypt))?;
        // Engagement atomique : refusé si un message de séquence ≥ a été
        // accepté entre-temps (rejeu concurrent).
        advance_counter(&self.recv_seq, |last| (msg.seq > last).then_some(msg.seq))
            .ok_or(BridgeError::E2e(E2eError::Replay))?;
        Ok(plaintext)
    }

    /// Chiffre un message JSON sérialisable.
    pub fn encrypt_json<T: Serialize>(
        &self,
        binding: &RequestBinding,
        value: &T,
    ) -> BridgeResult<EncryptedMessage> {
        let json =
            Zeroizing::new(serde_json::to_vec(value).map_err(|e| E2eError::Json(e.to_string()))?);
        self.encrypt(binding, &json)
    }

    /// Déchiffre et désérialise un message JSON.
    pub fn decrypt_json<T: for<'de> Deserialize<'de>>(
        &self,
        binding: &RequestBinding,
        msg: &EncryptedMessage,
    ) -> BridgeResult<T> {
        let plaintext = Zeroizing::new(self.decrypt(binding, msg)?);
        Ok(serde_json::from_slice(&plaintext).map_err(|e| E2eError::Json(e.to_string()))?)
    }
}

/// Message chiffré transporté sur le réseau.
///
/// Format JSON v2 : `{"protocol_version":2,"seq":<u64>,"request_id":<hex 16>,
/// "nonce":<hex 24>,"ciphertext":<hex>}`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EncryptedMessage {
    /// Version du protocole E2E.
    pub protocol_version: u8,
    /// Numéro de séquence de l'émetteur (strictement croissant, ≥ 1).
    pub seq: u64,
    /// Identifiant de la requête à laquelle le message est lié.
    #[serde(with = "hex_request_id")]
    pub request_id: [u8; REQUEST_ID_SIZE],
    /// Nonce XChaCha20 (24 octets), tiré par l'émetteur.
    #[serde(with = "hex_nonce")]
    pub nonce: [u8; NONCE_SIZE],
    /// Ciphertext + tag Poly1305.
    #[serde(with = "hex_vec")]
    pub ciphertext: Vec<u8>,
}

impl EncryptedMessage {
    /// Décode un message reçu du réseau (taille bornée, version vérifiée d'abord).
    pub fn from_json(bytes: &[u8]) -> Result<Self, E2eError> {
        Self::from_json_bounded(bytes, MAX_MESSAGE_JSON_LEN)
    }

    /// Décode un message avec une borne plus stricte que [`MAX_MESSAGE_JSON_LEN`]
    /// (ex. point d'accès d'authentification).
    pub fn from_json_bounded(bytes: &[u8], max: usize) -> Result<Self, E2eError> {
        if bytes.len() > max.min(MAX_MESSAGE_JSON_LEN) {
            return Err(E2eError::TooLarge("message chiffré"));
        }
        probe_version(bytes)?;
        serde_json::from_slice(bytes).map_err(|e| E2eError::Json(e.to_string()))
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Appairage : clés d'identité et code de confirmation
// ═══════════════════════════════════════════════════════════════════════════

/// Décode et valide une clé publique d'identité ed25519 (hex, 32 octets) :
/// point valide et non faible. Utilisé à l'appairage (C-12, C-13).
pub fn parse_identity_key_hex(s: &str) -> Result<[u8; 32], E2eError> {
    let bytes: [u8; 32] = decode_hex_fixed(s, "identity_key")?;
    let vk = VerifyingKey::from_bytes(&bytes).map_err(|_| E2eError::InvalidIdentityKey)?;
    if vk.is_weak() {
        return Err(E2eError::InvalidIdentityKey);
    }
    Ok(bytes)
}

/// Code de confirmation d'appairage à 6 chiffres, à comparer visuellement sur
/// l'appareil et sur le COG (substitution de clé par un intermédiaire = codes
/// différents). Calculé sur SHA-256(domaine || identité appareil || identité COG).
pub fn pairing_confirmation_code(device_identity: &[u8; 32], cog_identity: &[u8; 32]) -> String {
    let mut input = Vec::with_capacity(PAIRING_CODE_DOMAIN.len() + 64);
    input.extend_from_slice(PAIRING_CODE_DOMAIN);
    input.extend_from_slice(device_identity);
    input.extend_from_slice(cog_identity);
    let digest = sha256(&input);
    let n = u32::from_be_bytes([digest[0], digest[1], digest[2], digest[3]]) % 1_000_000;
    format!("{n:06}")
}

// ═══════════════════════════════════════════════════════════════════════════
// Helpers
// ═══════════════════════════════════════════════════════════════════════════

/// SHA-256 (crate `sha2`, C-24).
pub(crate) fn sha256(data: &[u8]) -> [u8; 32] {
    Sha256::digest(data).into()
}

/// Avance atomiquement un compteur : `next(last)` donne la nouvelle valeur, ou
/// `None` pour refuser. Retourne la valeur enregistrée.
fn advance_counter(counter: &AtomicU64, next: impl Fn(u64) -> Option<u64>) -> Option<u64> {
    let mut last = counter.load(Ordering::SeqCst);
    loop {
        let candidate = next(last)?;
        match counter.compare_exchange(last, candidate, Ordering::SeqCst, Ordering::SeqCst) {
            Ok(_) => return Some(candidate),
            Err(actual) => last = actual,
        }
    }
}

/// Serde helper pour [u8; 24] en hex.
mod hex_nonce {
    use super::NONCE_SIZE;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(bytes: &[u8; NONCE_SIZE], s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&hex::encode(bytes))
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<[u8; NONCE_SIZE], D::Error> {
        let s = String::deserialize(d)?;
        if s.len() != 2 * NONCE_SIZE {
            return Err(serde::de::Error::custom("nonce: 24 octets attendus"));
        }
        let bytes = hex::decode(s).map_err(serde::de::Error::custom)?;
        bytes
            .try_into()
            .map_err(|_| serde::de::Error::custom("nonce: 24 octets attendus"))
    }
}

/// Serde helper pour [u8; 16] (identifiant de requête) en hex.
mod hex_request_id {
    use super::REQUEST_ID_SIZE;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(bytes: &[u8; REQUEST_ID_SIZE], s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&hex::encode(bytes))
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        d: D,
    ) -> Result<[u8; REQUEST_ID_SIZE], D::Error> {
        let s = String::deserialize(d)?;
        if s.len() != 2 * REQUEST_ID_SIZE {
            return Err(serde::de::Error::custom("request_id: 16 octets attendus"));
        }
        let bytes = hex::decode(s).map_err(serde::de::Error::custom)?;
        bytes
            .try_into()
            .map_err(|_| serde::de::Error::custom("request_id: 16 octets attendus"))
    }
}

/// Serde helper pour Vec<u8> en hex.
mod hex_vec {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(bytes: &[u8], s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&hex::encode(bytes))
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<u8>, D::Error> {
        let s = String::deserialize(d)?;
        hex::decode(s).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use x25519_dalek::StaticSecret;

    struct Pair {
        device: E2eIdentity,
        cog: E2eIdentity,
    }

    fn pair() -> Pair {
        Pair {
            device: E2eIdentity::generate(),
            cog: E2eIdentity::generate(),
        }
    }

    fn bnd() -> RequestBinding {
        RequestBinding::new("POST", "/test").unwrap()
    }

    fn established() -> (E2eSession, E2eSession) {
        let p = pair();
        let (d_pending, d_offer) =
            PendingHandshake::initiate(E2eRole::Device, &p.device, &p.cog.public_key_bytes())
                .unwrap();
        let (c_pending, c_offer) =
            PendingHandshake::initiate(E2eRole::Cog, &p.cog, &p.device.public_key_bytes()).unwrap();
        let device = d_pending.complete(&c_offer).unwrap();
        let cog = c_pending.complete(&d_offer).unwrap();
        (device, cog)
    }

    fn err_of(r: BridgeResult<Vec<u8>>) -> E2eError {
        match r {
            Err(BridgeError::E2e(e)) => e,
            other => panic!("erreur E2E attendue, obtenu {other:?}"),
        }
    }

    // ── SHA-256 ──────────────────────────────────────────────────────────

    #[test]
    fn sha256_known_answer() {
        // FIPS 180-2, annexe B.1 et chaîne vide.
        assert_eq!(
            hex::encode(sha256(b"abc")),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
        assert_eq!(
            hex::encode(sha256(b"")),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    // ── Handshake ────────────────────────────────────────────────────────

    #[test]
    fn handshake_and_both_directions() {
        let (device, cog) = established();
        let bd = bnd();
        assert_eq!(device.session_id(), cog.session_id());

        let m = device.encrypt(&bd, b"Salut depuis Android!").unwrap();
        assert_eq!(cog.decrypt(&bd, &m).unwrap(), b"Salut depuis Android!");

        let m = cog.encrypt(&bd, b"Bonjour depuis le COG").unwrap();
        assert_eq!(device.decrypt(&bd, &m).unwrap(), b"Bonjour depuis le COG");
    }

    #[test]
    fn json_encrypt_decrypt() {
        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct TestMsg {
            action: String,
            value: u32,
        }
        let (device, cog) = established();
        let bd = bnd();
        let msg = TestMsg {
            action: "navigate".into(),
            value: 42,
        };
        let enc = device.encrypt_json(&bd, &msg).unwrap();
        let dec: TestMsg = cog.decrypt_json(&bd, &enc).unwrap();
        assert_eq!(dec, msg);
    }

    #[test]
    fn tampered_ephemeral_key_fails() {
        let p = pair();
        let (d_pending, _) =
            PendingHandshake::initiate(E2eRole::Device, &p.device, &p.cog.public_key_bytes())
                .unwrap();
        let (_c, mut c_offer) =
            PendingHandshake::initiate(E2eRole::Cog, &p.cog, &p.device.public_key_bytes()).unwrap();
        // MITM : substitue sa propre clé éphémère.
        let mallory = EphemeralSecret::random_from_rng(OsRng);
        c_offer.ephemeral_key = hex::encode(PublicKey::from(&mallory).as_bytes());
        assert_eq!(
            d_pending.complete(&c_offer).unwrap_err(),
            E2eError::InvalidSignature
        );
    }

    #[test]
    fn missing_signature_fails() {
        let p = pair();
        let (d_pending, _) =
            PendingHandshake::initiate(E2eRole::Device, &p.device, &p.cog.public_key_bytes())
                .unwrap();
        let (_c, mut c_offer) =
            PendingHandshake::initiate(E2eRole::Cog, &p.cog, &p.device.public_key_bytes()).unwrap();
        c_offer.signature = String::new();
        assert_eq!(
            d_pending.complete(&c_offer).unwrap_err(),
            E2eError::Malformed("signature")
        );
    }

    #[test]
    fn wrong_signer_fails() {
        let p = pair();
        let mallory = E2eIdentity::generate();
        // Mallory se présente avec sa propre identité : refus d'identité.
        let (d_pending, _) =
            PendingHandshake::initiate(E2eRole::Device, &p.device, &p.cog.public_key_bytes())
                .unwrap();
        let (_m, mut m_offer) =
            PendingHandshake::initiate(E2eRole::Cog, &mallory, &p.device.public_key_bytes())
                .unwrap();
        assert_eq!(
            d_pending.complete(&m_offer).unwrap_err(),
            E2eError::IdentityMismatch
        );

        // Même en recopiant la clé d'identité du COG, la signature ne passe pas.
        let (d_pending, _) =
            PendingHandshake::initiate(E2eRole::Device, &p.device, &p.cog.public_key_bytes())
                .unwrap();
        m_offer.identity_key = hex::encode(p.cog.public_key_bytes());
        assert_eq!(
            d_pending.complete(&m_offer).unwrap_err(),
            E2eError::InvalidSignature
        );
    }

    #[test]
    fn offer_for_other_recipient_fails() {
        // Une offre du COG destinée à un autre appareil ne peut pas être rejouée.
        let p = pair();
        let other_device = E2eIdentity::generate();
        let (d_pending, _) =
            PendingHandshake::initiate(E2eRole::Device, &p.device, &p.cog.public_key_bytes())
                .unwrap();
        let (_c, c_to_other) =
            PendingHandshake::initiate(E2eRole::Cog, &p.cog, &other_device.public_key_bytes())
                .unwrap();
        assert_eq!(
            d_pending.complete(&c_to_other).unwrap_err(),
            E2eError::InvalidSignature
        );
    }

    #[test]
    fn reflected_offer_fails() {
        let p = pair();
        let (d_pending, d_offer) =
            PendingHandshake::initiate(E2eRole::Device, &p.device, &p.cog.public_key_bytes())
                .unwrap();
        assert_eq!(
            d_pending.complete(&d_offer).unwrap_err(),
            E2eError::ReflectedKey
        );
    }

    #[test]
    fn same_role_offer_fails() {
        // Une offre d'un autre « appareil » (même rôle) est refusée.
        let p = pair();
        let (d_pending, _) =
            PendingHandshake::initiate(E2eRole::Device, &p.device, &p.cog.public_key_bytes())
                .unwrap();
        let (_c, mut c_offer) =
            PendingHandshake::initiate(E2eRole::Cog, &p.cog, &p.device.public_key_bytes()).unwrap();
        c_offer.role = E2eRole::Device;
        assert_eq!(
            d_pending.complete(&c_offer).unwrap_err(),
            E2eError::UnexpectedRole
        );
    }

    #[test]
    fn wrong_version_fails() {
        let p = pair();
        let (d_pending, _) =
            PendingHandshake::initiate(E2eRole::Device, &p.device, &p.cog.public_key_bytes())
                .unwrap();
        let (_c, mut c_offer) =
            PendingHandshake::initiate(E2eRole::Cog, &p.cog, &p.device.public_key_bytes()).unwrap();
        c_offer.protocol_version = 1;
        assert_eq!(
            d_pending.complete(&c_offer).unwrap_err(),
            E2eError::UnsupportedVersion(1)
        );
    }

    #[test]
    fn legacy_v1_wire_is_refused_explicitly() {
        // Offre v1 telle qu'émise par l'ancien bridge.
        let v1 = br#"{"public_key":"0000000000000000000000000000000000000000000000000000000000000000","protocol_version":1}"#;
        assert_eq!(
            E2eHandshake::from_json(v1).unwrap_err(),
            E2eError::UnsupportedVersion(1)
        );
        // Message chiffré v1 : sans champ de version, nonce de 12 octets.
        let v1_msg = br#"{"nonce":"000000000000000000000000","ciphertext":"00"}"#;
        assert_eq!(
            EncryptedMessage::from_json(v1_msg).unwrap_err(),
            E2eError::UnsupportedVersion(1)
        );
        // Version future également refusée.
        let v3 = br#"{"protocol_version":3}"#;
        assert_eq!(
            E2eHandshake::from_json(v3).unwrap_err(),
            E2eError::UnsupportedVersion(3)
        );
    }

    #[test]
    fn oversized_inputs_refused() {
        let big = vec![b' '; MAX_HANDSHAKE_JSON_LEN + 1];
        assert!(matches!(
            E2eHandshake::from_json(&big),
            Err(E2eError::TooLarge(_))
        ));
        let (device, _) = established();
        let bd = bnd();
        let huge = vec![0u8; MAX_PLAINTEXT_LEN + 1];
        assert!(matches!(
            device.encrypt(&bd, &huge),
            Err(BridgeError::E2e(E2eError::TooLarge(_)))
        ));
    }

    #[test]
    fn self_handshake_and_invalid_identity_refused() {
        let me = E2eIdentity::generate();
        assert_eq!(
            PendingHandshake::initiate(E2eRole::Device, &me, &me.public_key_bytes()).unwrap_err(),
            E2eError::SelfHandshake
        );
        // Point d'ordre faible (identité ed25519 faible).
        let mut weak = [0u8; 32];
        weak[0] = 1;
        assert_eq!(
            PendingHandshake::initiate(E2eRole::Device, &me, &weak).unwrap_err(),
            E2eError::InvalidIdentityKey
        );
    }

    #[test]
    fn non_contributory_dh_fails() {
        // Le COG (légitime ou compromis) signe une clé éphémère d'ordre faible.
        let p = pair();
        let (d_pending, _) =
            PendingHandshake::initiate(E2eRole::Device, &p.device, &p.cog.public_key_bytes())
                .unwrap();
        let low_order = [0u8; 32];
        let sig = p.cog.sign(&transcript(
            E2eRole::Cog,
            &p.cog.public_key_bytes(),
            &p.device.public_key_bytes(),
            &low_order,
        ));
        let offer = E2eHandshake {
            protocol_version: E2E_PROTOCOL_VERSION,
            role: E2eRole::Cog,
            identity_key: hex::encode(p.cog.public_key_bytes()),
            ephemeral_key: hex::encode(low_order),
            signature: hex::encode(sig),
        };
        assert_eq!(
            d_pending.complete(&offer).unwrap_err(),
            E2eError::NonContributory
        );
    }

    #[test]
    fn handshake_offer_roundtrip_json() {
        let p = pair();
        let (_d, offer) =
            PendingHandshake::initiate(E2eRole::Device, &p.device, &p.cog.public_key_bytes())
                .unwrap();
        let json = serde_json::to_vec(&offer).unwrap();
        assert_eq!(E2eHandshake::from_json(&json).unwrap(), offer);
    }

    // ── Session ──────────────────────────────────────────────────────────

    #[test]
    fn nonces_never_repeat_in_each_direction() {
        let (device, cog) = established();
        let bd = bnd();
        let mut seen = HashSet::new();
        for _ in 0..10_000 {
            assert!(
                seen.insert(device.encrypt(&bd, b"m").unwrap().nonce),
                "nonce répété (appareil)"
            );
        }
        for _ in 0..10_000 {
            assert!(
                seen.insert(cog.encrypt(&bd, b"m").unwrap().nonce),
                "nonce répété (COG)"
            );
        }
    }

    #[test]
    fn message_cannot_be_decrypted_as_other_direction() {
        let (device, cog) = established();
        let bd = bnd();
        // Réflexion : un message de l'appareil renvoyé à l'appareil.
        let m = device.encrypt(&bd, b"x").unwrap();
        assert_eq!(err_of(device.decrypt(&bd, &m)), E2eError::Decrypt);
        // Idem côté COG.
        let m = cog.encrypt(&bd, b"y").unwrap();
        assert_eq!(err_of(cog.decrypt(&bd, &m)), E2eError::Decrypt);
    }

    #[test]
    fn tampered_message_fails() {
        let (device, cog) = established();
        let bd = bnd();
        let mut m = device.encrypt(&bd, b"secret").unwrap();
        if let Some(b) = m.ciphertext.first_mut() {
            *b ^= 0xff;
        }
        assert_eq!(err_of(cog.decrypt(&bd, &m)), E2eError::Decrypt);
        let mut m = device.encrypt(&bd, b"secret").unwrap();
        m.nonce[0] ^= 1;
        assert_eq!(err_of(cog.decrypt(&bd, &m)), E2eError::Decrypt);
    }

    #[test]
    fn other_session_cannot_decrypt() {
        let (device, _cog) = established();
        let bd = bnd();
        let (_d2, cog2) = established();
        let m = device.encrypt(&bd, b"confidentiel").unwrap();
        assert_eq!(err_of(cog2.decrypt(&bd, &m)), E2eError::Decrypt);
    }

    #[test]
    fn message_wrong_version_refused() {
        let (device, cog) = established();
        let bd = bnd();
        let mut m = device.encrypt(&bd, b"x").unwrap();
        m.protocol_version = 1;
        assert_eq!(err_of(cog.decrypt(&bd, &m)), E2eError::UnsupportedVersion(1));
    }

    #[test]
    fn encrypted_message_wire_roundtrip() {
        let (device, cog) = established();
        let bd = bnd();
        let m = device.encrypt(&bd, b"test over the wire").unwrap();
        let json = serde_json::to_vec(&m).unwrap();
        let restored = EncryptedMessage::from_json(&json).unwrap();
        assert_eq!(cog.decrypt(&bd, &restored).unwrap(), b"test over the wire");
    }

    #[test]
    fn debug_redacts_secrets() {
        let id = E2eIdentity::from_secret_bytes(&[0x11; 32]);
        let dbg = format!("{id:?}");
        assert!(dbg.contains("<redacted>"));
        assert!(!dbg.contains(&hex::encode([0x11u8; 32])));
        let (device, _) = established();
        assert!(format!("{device:?}").contains("<redacted>"));
    }

    // ── Anti-rejeu et liaison de requête ─────────────────────────────────

    #[test]
    fn replayed_message_rejected() {
        let (device, cog) = established();
        let bd = bnd();
        let m = device.encrypt(&bd, b"ordre").unwrap();
        assert_eq!(cog.decrypt(&bd, &m).unwrap(), b"ordre");
        assert_eq!(err_of(cog.decrypt(&bd, &m)), E2eError::Replay);
    }

    #[test]
    fn lower_sequence_rejected_out_of_order() {
        let (device, cog) = established();
        let bd = bnd();
        let m1 = device.encrypt(&bd, b"un").unwrap();
        let m2 = device.encrypt(&bd, b"deux").unwrap();
        assert_eq!((m1.seq, m2.seq), (1, 2));
        assert_eq!(cog.decrypt(&bd, &m2).unwrap(), b"deux");
        assert_eq!(err_of(cog.decrypt(&bd, &m1)), E2eError::Replay);
    }

    #[test]
    fn forged_sequence_cannot_advance_counter() {
        let (device, cog) = established();
        let bd = bnd();
        let m1 = device.encrypt(&bd, b"un").unwrap();
        // Un intermédiaire relève la séquence : l'AEAD échoue, le compteur reste.
        let mut forged = m1.clone();
        forged.seq = 1_000;
        assert_eq!(err_of(cog.decrypt(&bd, &forged)), E2eError::Decrypt);
        assert_eq!(cog.decrypt(&bd, &m1).unwrap(), b"un");
        // Séquence nulle refusée.
        let mut zero = device.encrypt(&bd, b"x").unwrap();
        zero.seq = 0;
        assert_eq!(err_of(cog.decrypt(&bd, &zero)), E2eError::Replay);
    }

    #[test]
    fn response_with_other_request_id_rejected() {
        let (device, cog) = established();
        let req_a = RequestBinding::new("POST", "/auth").unwrap();
        let req_b = RequestBinding::new("POST", "/auth").unwrap();
        assert_ne!(req_a.request_id(), req_b.request_id());
        // Le COG répond à la requête B ; l'appareil attend la réponse de A.
        let resp_b = cog.encrypt(&req_b, b"reponse B").unwrap();
        assert_eq!(
            err_of(device.decrypt(&req_a, &resp_b)),
            E2eError::RequestIdMismatch
        );
        // Même en réécrivant l'identifiant, l'AEAD refuse.
        let mut forged = cog.encrypt(&req_b, b"reponse B").unwrap();
        forged.request_id = req_a.request_id();
        assert_eq!(err_of(device.decrypt(&req_a, &forged)), E2eError::Decrypt);
    }

    #[test]
    fn ciphertext_for_endpoint_a_refused_on_endpoint_b() {
        let (device, cog) = established();
        let on_a = RequestBinding::new("POST", "/auth").unwrap();
        let m = device.encrypt(&on_a, b"{}").unwrap();
        let on_b = RequestBinding::for_received(&m, "POST", "/api/autre").unwrap();
        assert_eq!(err_of(cog.decrypt(&on_b, &m)), E2eError::Decrypt);
        // Autre méthode, même point d'accès : refus.
        let get_a = RequestBinding::for_received(&m, "GET", "/auth").unwrap();
        assert_eq!(err_of(cog.decrypt(&get_a, &m)), E2eError::Decrypt);
        // Bonne liaison : accepté.
        let ok = RequestBinding::for_received(&m, "POST", "/auth").unwrap();
        assert_eq!(cog.decrypt(&ok, &m).unwrap(), b"{}");
    }

    #[test]
    fn binding_bounds_enforced() {
        assert!(RequestBinding::new("", "/x").is_err());
        assert!(RequestBinding::new("POST", "").is_err());
        assert!(RequestBinding::new(&"M".repeat(MAX_METHOD_LEN + 1), "/x").is_err());
        assert!(RequestBinding::new("POST", &"/".repeat(MAX_ENDPOINT_LEN + 1)).is_err());
    }

    #[test]
    fn send_sequence_exhaustion_refused() {
        let (device, _) = established();
        device.send_seq.store(u64::MAX, Ordering::SeqCst);
        assert!(matches!(
            device.encrypt(&bnd(), b"x"),
            Err(BridgeError::E2e(E2eError::SequenceExhausted))
        ));
    }

    // ── Appairage ────────────────────────────────────────────────────────

    #[test]
    fn identity_key_parsing_and_secret_roundtrip() {
        let id = E2eIdentity::generate();
        let pk = id.public_key_bytes();
        assert_eq!(parse_identity_key_hex(&hex::encode(pk)).unwrap(), pk);
        let restored = E2eIdentity::from_secret_bytes(&id.secret_bytes());
        assert_eq!(restored.public_key_bytes(), pk);
        let mut weak = [0u8; 32];
        weak[0] = 1;
        assert_eq!(
            parse_identity_key_hex(&hex::encode(weak)).unwrap_err(),
            E2eError::InvalidIdentityKey
        );
        assert!(parse_identity_key_hex("zz").is_err());
        assert!(parse_identity_key_hex(&"00".repeat(33)).is_err());
    }

    #[test]
    fn confirmation_code_depends_on_both_keys() {
        let d = E2eIdentity::generate().public_key_bytes();
        let c = E2eIdentity::generate().public_key_bytes();
        let m = E2eIdentity::generate().public_key_bytes();
        let code = pairing_confirmation_code(&d, &c);
        assert_eq!(code.len(), 6);
        assert!(code.chars().all(|ch| ch.is_ascii_digit()));
        assert_eq!(code, pairing_confirmation_code(&d, &c));
        assert_ne!(code, pairing_confirmation_code(&d, &m));
        assert_eq!(
            pairing_confirmation_code(&[0x01; 32], &[0x02; 32]),
            pairing_confirmation_code(&[0x01; 32], &[0x02; 32])
        );
    }

    // ── Vecteur de référence figé (C-61) ─────────────────────────────────

    #[test]
    fn frozen_reference_vector() {
        let device = E2eIdentity::from_secret_bytes(&[0x01; 32]);
        let cog = E2eIdentity::from_secret_bytes(&[0x02; 32]);
        let d_eph = StaticSecret::from([0x03; 32]);
        let c_eph = StaticSecret::from([0x04; 32]);
        let d_eph_pub = *PublicKey::from(&d_eph).as_bytes();
        let c_eph_pub = *PublicKey::from(&c_eph).as_bytes();
        let d_id = device.public_key_bytes();
        let c_id = cog.public_key_bytes();

        let t = transcript(E2eRole::Device, &d_id, &c_id, &d_eph_pub);
        let mut expected_t = HANDSHAKE_DOMAIN.to_vec();
        expected_t.extend_from_slice(&[0x02, 0x01]);
        expected_t.extend_from_slice(&d_id);
        expected_t.extend_from_slice(&c_id);
        expected_t.extend_from_slice(&d_eph_pub);
        assert_eq!(t, expected_t);
        assert_eq!(t.len(), HANDSHAKE_DOMAIN.len() + 98);

        let frozen = [
            ("device_identity", hex::encode(d_id)),
            ("cog_identity", hex::encode(c_id)),
            ("device_ephemeral", hex::encode(d_eph_pub)),
            ("cog_ephemeral", hex::encode(c_eph_pub)),
            ("transcript_sha256", hex::encode(sha256(&t))),
            ("device_signature", hex::encode(device.sign(&t))),
        ];

        let shared = d_eph.diffie_hellman(&PublicKey::from(c_eph_pub));
        let shared2 = c_eph.diffie_hellman(&PublicKey::from(d_eph_pub));
        assert_eq!(shared.as_bytes(), shared2.as_bytes());
        let mat = derive_session_material(shared.as_bytes(), &d_id, &d_eph_pub, &c_id, &c_eph_pub)
            .unwrap();
        let session = E2eSession::from_material(E2eRole::Device, &mat);
        let bd = RequestBinding::with_request_id([0x06; REQUEST_ID_SIZE], "POST", "/auth").unwrap();
        let aad = session.aad(E2eRole::Device, 1, &bd);
        let mut expected_aad = MESSAGE_AAD_DOMAIN.to_vec();
        expected_aad.extend_from_slice(&[0x02, 0x01]);
        expected_aad.extend_from_slice(&mat.session_id);
        expected_aad.extend_from_slice(&1u64.to_be_bytes());
        expected_aad.extend_from_slice(&[0x06; REQUEST_ID_SIZE]);
        expected_aad.push(4);
        expected_aad.extend_from_slice(b"POST");
        expected_aad.extend_from_slice(&[0x00, 0x05]);
        expected_aad.extend_from_slice(b"/auth");
        assert_eq!(aad, expected_aad);
        let sealed = session
            .seal_with_nonce([0x05; NONCE_SIZE], 1, &bd, b"miyukini")
            .unwrap();
        assert_eq!(sealed.seq, 1);
        assert_eq!(sealed.request_id, [0x06; REQUEST_ID_SIZE]);

        let derived = [
            ("okm", hex::encode(mat.okm.as_ref())),
            ("session_id", hex::encode(mat.session_id)),
            ("aad_sha256", hex::encode(sha256(&aad))),
            ("ciphertext", hex::encode(&sealed.ciphertext)),
        ];

        let actual: Vec<(&str, String)> = frozen.into_iter().chain(derived).collect();
        for (name, value) in &actual {
            let want = REFERENCE
                .iter()
                .find(|(n, _)| n == name)
                .map_or("<absent>", |(_, v)| *v);
            assert_eq!(value, want, "vecteur figé « {name} »");
        }

        // Le côté COG déchiffre le message de référence.
        let cog_session = E2eSession::from_material(E2eRole::Cog, &mat);
        assert_eq!(cog_session.decrypt(&bd, &sealed).unwrap(), b"miyukini");
    }

    /// Valeurs figées du protocole E2E v2 (toute modification = rupture du format).
    ///
    /// Entrées : graines ed25519 appareil `01`×32 et COG `02`×32, secrets X25519
    /// éphémères appareil `03`×32 et COG `04`×32, nonce `05`×24, séquence 1,
    /// identifiant de requête `06`×16, liaison `POST /auth`, clair `miyukini`
    /// chiffré dans le sens appareil → COG.
    const REFERENCE: &[(&str, &str)] = &[
        (
            "device_identity",
            "8a88e3dd7409f195fd52db2d3cba5d72ca6709bf1d94121bf3748801b40f6f5c",
        ),
        (
            "cog_identity",
            "8139770ea87d175f56a35466c34c7ecccb8d8a91b4ee37a25df60f5b8fc9b394",
        ),
        (
            "device_ephemeral",
            "5dfedd3b6bd47f6fa28ee15d969d5bb0ea53774d488bdaf9df1c6e0124b3ef22",
        ),
        (
            "cog_ephemeral",
            "ac01b2209e86354fb853237b5de0f4fab13c7fcbf433a61c019369617fecf10b",
        ),
        (
            "transcript_sha256",
            "3da5f1c51407dff4ad1af533ac2bac8124bd61716e20c943136fe46e72b7f146",
        ),
        (
            "device_signature",
            "e706032f8556b5a22a38c9124af58e840c8b2aa680494df2e920bb0c9a42b24ceb8837b15ebbe13aa2b597c0f2836a207b5734a75cbd83e1c63d3e59bd56dd09",
        ),
        (
            "okm",
            "43e69ded6ab000eeb6a451dd620314665ba651e92f825e745d6d51236950e07748c63d42eb32faaef9fdd96082a0b69245b0a61c33ba8c9451729fc34f23e548",
        ),
        (
            "session_id",
            "7749e2bf3a201b0020d4dc8546dddbb506a7918a4596306a94d568a31b1a976c",
        ),
        (
            "aad_sha256",
            "cb74d0f7d0ae766e0aafc184fa0c48e1ef9f3e04051e6172eee6ddd84af4d908",
        ),
        ("ciphertext", "1ff15b16b2fd7845d4e829c8bbb4651b51bc8ff0ab72ea4a"),
    ];
}
