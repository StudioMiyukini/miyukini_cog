//! @id     jaybureau_suite_jaymessage_session
//! @do     definir_session_store
//! @role   ui
//! @layer  core
//! @human  jaybureau-suite — module « jaymessage session ».

//! Sessions E2E par conversation.
//!
//! ## Handshake authentifie (R-009)
//!
//! Chaque pair genere une cle X25519 ephemere et **la signe** avec sa cle
//! d'identite ed25519. Le message signe lie :
//! - une chaine de domaine ([`HANDSHAKE_DOMAIN`]) ;
//! - l'identifiant et la cle d'identite de l'emetteur ;
//! - la cle d'identite du destinataire attendu ;
//! - la cle publique ephemere.
//!
//! Le destinataire verifie la signature (`verify_strict`) avec la cle
//! d'identite **qu'il connait deja** pour ce pair avant toute derivation ;
//! un echec = refus (C-12). Le secret X25519 est ensuite passe dans
//! HKDF-SHA256 pour obtenir deux cles directionnelles (une par sens), ce qui
//! evite qu'un meme couple (cle, nonce) serve aux deux pairs.
//!
//! ## Chiffrement
//!
//! XChaCha20-Poly1305 avec nonce aleatoire de 192 bits genere en interne
//! (`OsRng`) : l'appelant ne fournit jamais de nonce.

use crate::identity::{IdentityKey, PublicIdentity};
use crate::{ConversationId, JayMessageError, Result, UserId};
use chacha20poly1305::aead::{Aead, KeyInit, Payload};
use chacha20poly1305::{XChaCha20Poly1305, XNonce};
use hkdf::Hkdf;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::collections::HashMap;
use std::sync::Mutex;
use x25519_dalek::{EphemeralSecret, PublicKey};
use zeroize::{Zeroize, ZeroizeOnDrop, Zeroizing};

/// Domaine de separation des signatures de handshake.
pub const HANDSHAKE_DOMAIN: &[u8] = b"miyukini/jaymessage/handshake/v2";

/// Domaine HKDF des cles de session.
const SESSION_KDF_INFO: &[u8] = b"miyukini/jaymessage/session-keys/v2";

/// Domaine des donnees associees (AAD) des messages 1:1.
const MESSAGE_AAD_DOMAIN: &[u8] = b"miyukini/jaymessage/msg/v2";

/// Taille du nonce XChaCha20 (192 bits).
pub const NONCE_LEN: usize = 24;

/// Taille maximale d'un identifiant utilisateur dans un handshake (C-13).
const MAX_USER_ID_LEN: usize = 256;

/// Offre de handshake transmise au pair (en clair, mais signee).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandshakeOffer {
    /// Version du protocole de handshake.
    pub version: u8,
    /// Emetteur de l'offre.
    pub user_id: UserId,
    /// Cle d'identite ed25519 de l'emetteur (32 octets, hex).
    pub identity_key: String,
    /// Cle publique X25519 ephemere (32 octets, hex).
    pub ephemeral_key: String,
    /// Signature ed25519 de l'emetteur sur le transcript (64 octets, hex).
    pub signature: String,
}

/// Version courante du handshake.
pub const HANDSHAKE_VERSION: u8 = 2;

/// Handshake en cours : la cle ephemere privee attend l'offre du pair.
pub struct PendingHandshake {
    local_user: UserId,
    local_identity: [u8; 32],
    peer: PublicIdentity,
    peer_identity: [u8; 32],
    ephemeral: EphemeralSecret,
    ephemeral_public: [u8; 32],
}

impl std::fmt::Debug for PendingHandshake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PendingHandshake")
            .field("local_user", &self.local_user)
            .field("peer", &self.peer.user_id)
            .field("ephemeral", &"<redacted>")
            .finish_non_exhaustive()
    }
}

/// Cles de session directionnelles (effacees a la destruction).
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct SessionKeys {
    send: [u8; 32],
    recv: [u8; 32],
}

impl std::fmt::Debug for SessionKeys {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("SessionKeys(<redacted>)")
    }
}

/// Construit le transcript signe par `sender` a destination de `recipient`.
fn transcript(
    sender_user: &str,
    sender_identity: &[u8; 32],
    recipient_identity: &[u8; 32],
    ephemeral: &[u8; 32],
) -> Vec<u8> {
    let mut m = Vec::with_capacity(HANDSHAKE_DOMAIN.len() + 4 + sender_user.len() + 96 + 1);
    m.extend_from_slice(HANDSHAKE_DOMAIN);
    m.push(HANDSHAKE_VERSION);
    // Longueur prefixee : pas d'ambiguite de concatenation.
    m.extend_from_slice(&(sender_user.len() as u32).to_be_bytes());
    m.extend_from_slice(sender_user.as_bytes());
    m.extend_from_slice(sender_identity);
    m.extend_from_slice(recipient_identity);
    m.extend_from_slice(ephemeral);
    m
}

fn decode_32(hex_str: &str, what: &str) -> Result<[u8; 32]> {
    let bytes = hex::decode(hex_str)
        .map_err(|e| JayMessageError::Crypto(format!("{what} hex: {e}")))?;
    bytes
        .as_slice()
        .try_into()
        .map_err(|_| JayMessageError::Crypto(format!("{what}: 32 octets attendus")))
}

impl PendingHandshake {
    /// Demarre un handshake avec `peer`, dont la cle d'identite est deja connue
    /// et de confiance (annuaire, verification hors bande).
    ///
    /// Retourne l'etat en attente et l'offre signee a envoyer au pair.
    pub fn initiate(local: &IdentityKey, peer: &PublicIdentity) -> Result<(Self, HandshakeOffer)> {
        if local.user_id.len() > MAX_USER_ID_LEN || peer.user_id.len() > MAX_USER_ID_LEN {
            return Err(JayMessageError::Invalid("user_id trop long".into()));
        }
        // Valide la cle du pair des maintenant (point ed25519 valide).
        let peer_identity = peer.verifying_key()?.to_bytes();
        let local_identity = local.verifying_key_bytes();
        if peer_identity == local_identity {
            return Err(JayMessageError::Crypto("handshake avec soi-meme".into()));
        }

        let ephemeral = EphemeralSecret::random_from_rng(rand::rngs::OsRng);
        let ephemeral_public = *PublicKey::from(&ephemeral).as_bytes();

        let msg = transcript(&local.user_id, &local_identity, &peer_identity, &ephemeral_public);
        let signature = local.sign(&msg);

        let offer = HandshakeOffer {
            version: HANDSHAKE_VERSION,
            user_id: local.user_id.clone(),
            identity_key: hex::encode(local_identity),
            ephemeral_key: hex::encode(ephemeral_public),
            signature: hex::encode(signature),
        };
        Ok((
            Self {
                local_user: local.user_id.clone(),
                local_identity,
                peer: peer.clone(),
                peer_identity,
                ephemeral,
                ephemeral_public,
            },
            offer,
        ))
    }

    /// Termine le handshake avec l'offre recue du pair.
    ///
    /// Refuse (fail-closed) si : version inconnue, identite differente de celle
    /// attendue, signature absente/invalide, cle ephemere reflechie ou faible.
    pub fn complete(self, offer: &HandshakeOffer) -> Result<SessionKeys> {
        if offer.version != HANDSHAKE_VERSION {
            return Err(JayMessageError::Crypto(format!(
                "version de handshake non supportee: {}",
                offer.version
            )));
        }
        if offer.user_id != self.peer.user_id {
            return Err(JayMessageError::Crypto("emetteur inattendu".into()));
        }
        let claimed_identity = decode_32(&offer.identity_key, "identity_key")?;
        if claimed_identity != self.peer_identity {
            return Err(JayMessageError::Crypto(
                "cle d'identite du pair differente de celle attendue".into(),
            ));
        }
        let peer_ephemeral = decode_32(&offer.ephemeral_key, "ephemeral_key")?;
        if peer_ephemeral == self.ephemeral_public {
            return Err(JayMessageError::Crypto("cle ephemere reflechie".into()));
        }
        let signature = hex::decode(&offer.signature)
            .map_err(|e| JayMessageError::Crypto(format!("signature hex: {e}")))?;

        // Signature verifiee AVANT toute derivation (C-23).
        let msg = transcript(
            &offer.user_id,
            &self.peer_identity,
            &self.local_identity,
            &peer_ephemeral,
        );
        self.peer.verify(&msg, &signature)?;

        let shared = self.ephemeral.diffie_hellman(&PublicKey::from(peer_ephemeral));
        if !shared.was_contributory() {
            return Err(JayMessageError::Crypto("cle ephemere faible".into()));
        }

        // Ordre canonique des deux pairs (par cle d'identite) pour que les deux
        // cotes derivent les memes cles et choisissent le meme sens.
        let local_first = self.local_identity < self.peer_identity;
        let (first_id, first_eph, second_id, second_eph) = if local_first {
            (self.local_identity, self.ephemeral_public, self.peer_identity, peer_ephemeral)
        } else {
            (self.peer_identity, peer_ephemeral, self.local_identity, self.ephemeral_public)
        };
        let mut info = Vec::with_capacity(SESSION_KDF_INFO.len() + 128);
        info.extend_from_slice(SESSION_KDF_INFO);
        info.extend_from_slice(&first_id);
        info.extend_from_slice(&first_eph);
        info.extend_from_slice(&second_id);
        info.extend_from_slice(&second_eph);

        let hk = Hkdf::<Sha256>::new(Some(HANDSHAKE_DOMAIN), shared.as_bytes());
        let mut okm = Zeroizing::new([0u8; 64]);
        hk.expand(&info, okm.as_mut())
            .map_err(|e| JayMessageError::Crypto(format!("hkdf: {e}")))?;

        let mut keys = SessionKeys {
            send: [0u8; 32],
            recv: [0u8; 32],
        };
        // okm[..32] : premier -> second ; okm[32..] : second -> premier.
        if local_first {
            keys.send.copy_from_slice(&okm[..32]);
            keys.recv.copy_from_slice(&okm[32..]);
        } else {
            keys.send.copy_from_slice(&okm[32..]);
            keys.recv.copy_from_slice(&okm[..32]);
        }
        Ok(keys)
    }
}

/// Store des sessions actives.
pub struct SessionStore {
    sessions: Mutex<HashMap<String, SessionEntry>>,
}

struct SessionEntry {
    send: XChaCha20Poly1305,
    recv: XChaCha20Poly1305,
}

impl SessionStore {
    pub fn new() -> Self {
        Self {
            sessions: Mutex::new(HashMap::new()),
        }
    }

    /// Enregistre une session 1:1 apres un handshake authentifie reussi.
    ///
    /// Seules des [`SessionKeys`] issues de [`PendingHandshake::complete`] sont
    /// acceptees : il n'est plus possible d'enregistrer une cle brute.
    pub fn register(
        &self,
        conversation_id: ConversationId,
        peer_id: UserId,
        keys: SessionKeys,
    ) -> Result<()> {
        let key = session_key(&conversation_id, &peer_id);
        let entry = SessionEntry {
            send: XChaCha20Poly1305::new((&keys.send).into()),
            recv: XChaCha20Poly1305::new((&keys.recv).into()),
        };
        drop(keys);
        let mut s = self
            .sessions
            .lock()
            .map_err(|_| JayMessageError::Crypto("lock".into()))?;
        s.insert(key, entry);
        Ok(())
    }

    /// Verifie si une session existe.
    pub fn has_session(&self, conversation_id: &str, peer_id: &str) -> bool {
        let key = session_key(conversation_id, peer_id);
        self.sessions.lock().is_ok_and(|s| s.contains_key(&key))
    }

    /// Chiffre un payload pour un destinataire specifique.
    ///
    /// Retourne `(nonce_hex, ciphertext_hex)` ; le nonce (192 bits) est tire
    /// en interne via `OsRng`.
    pub fn encrypt_for(
        &self,
        conversation_id: &str,
        peer_id: &str,
        plaintext: &[u8],
    ) -> Result<(String, String)> {
        let key = session_key(conversation_id, peer_id);
        let sessions = self
            .sessions
            .lock()
            .map_err(|_| JayMessageError::Crypto("lock".into()))?;
        let entry = sessions
            .get(&key)
            .ok_or_else(|| JayMessageError::NoSession(key.clone()))?;
        let nonce = random_nonce();
        let aad = message_aad(conversation_id);
        let ciphertext = entry
            .send
            .encrypt(
                XNonce::from_slice(&nonce),
                Payload {
                    msg: plaintext,
                    aad: &aad,
                },
            )
            .map_err(|e| JayMessageError::Crypto(format!("encrypt: {e}")))?;
        Ok((hex::encode(nonce), hex::encode(ciphertext)))
    }

    /// Dechiffre un payload d'un sender.
    pub fn decrypt_from(
        &self,
        conversation_id: &str,
        peer_id: &str,
        nonce_hex: &str,
        ciphertext_hex: &str,
    ) -> Result<Vec<u8>> {
        let key = session_key(conversation_id, peer_id);
        let sessions = self
            .sessions
            .lock()
            .map_err(|_| JayMessageError::Crypto("lock".into()))?;
        let entry = sessions
            .get(&key)
            .ok_or_else(|| JayMessageError::NoSession(key.clone()))?;

        let nonce_bytes = hex::decode(nonce_hex)
            .map_err(|e| JayMessageError::Crypto(format!("nonce hex: {e}")))?;
        if nonce_bytes.len() != NONCE_LEN {
            return Err(JayMessageError::Crypto("nonce must be 24 bytes".into()));
        }
        let ciphertext = hex::decode(ciphertext_hex)
            .map_err(|e| JayMessageError::Crypto(format!("ciphertext hex: {e}")))?;
        let aad = message_aad(conversation_id);
        entry
            .recv
            .decrypt(
                XNonce::from_slice(&nonce_bytes),
                Payload {
                    msg: &ciphertext,
                    aad: &aad,
                },
            )
            .map_err(|e| JayMessageError::Crypto(format!("decrypt: {e}")))
    }

    /// Nombre de sessions actives.
    pub fn count(&self) -> usize {
        self.sessions.lock().map_or(0, |s| s.len())
    }
}

impl Default for SessionStore {
    fn default() -> Self {
        Self::new()
    }
}

/// Tire un nonce XChaCha20 de 192 bits (OsRng).
pub(crate) fn random_nonce() -> [u8; NONCE_LEN] {
    use rand::RngCore;
    let mut nonce = [0u8; NONCE_LEN];
    rand::rngs::OsRng.fill_bytes(&mut nonce);
    nonce
}

fn message_aad(conversation_id: &str) -> Vec<u8> {
    let mut aad = Vec::with_capacity(MESSAGE_AAD_DOMAIN.len() + conversation_id.len());
    aad.extend_from_slice(MESSAGE_AAD_DOMAIN);
    aad.extend_from_slice(conversation_id.as_bytes());
    aad
}

/// Genere une cle de session deterministe.
fn session_key(conversation_id: &str, peer_id: &str) -> String {
    format!("{conversation_id}::{peer_id}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pair() -> (IdentityKey, IdentityKey) {
        (
            IdentityKey::generate("alice".into()),
            IdentityKey::generate("bob".into()),
        )
    }

    fn established() -> (SessionStore, SessionStore) {
        let (alice, bob) = pair();
        let (a_pending, a_offer) = PendingHandshake::initiate(&alice, &bob.public()).unwrap();
        let (b_pending, b_offer) = PendingHandshake::initiate(&bob, &alice.public()).unwrap();
        let a_keys = a_pending.complete(&b_offer).unwrap();
        let b_keys = b_pending.complete(&a_offer).unwrap();
        let alice_store = SessionStore::new();
        let bob_store = SessionStore::new();
        alice_store.register("conv-1".into(), "bob".into(), a_keys).unwrap();
        bob_store.register("conv-1".into(), "alice".into(), b_keys).unwrap();
        (alice_store, bob_store)
    }

    #[test]
    fn handshake_and_encrypt_decrypt() {
        let (alice_store, bob_store) = established();

        let (nonce, ciphertext) = alice_store
            .encrypt_for("conv-1", "bob", b"hello bob")
            .unwrap();
        let plaintext = bob_store
            .decrypt_from("conv-1", "alice", &nonce, &ciphertext)
            .unwrap();
        assert_eq!(plaintext, b"hello bob");

        // Sens inverse
        let (nonce, ciphertext) = bob_store
            .encrypt_for("conv-1", "alice", b"hello alice")
            .unwrap();
        let plaintext = alice_store
            .decrypt_from("conv-1", "bob", &nonce, &ciphertext)
            .unwrap();
        assert_eq!(plaintext, b"hello alice");
    }

    #[test]
    fn directional_keys_differ() {
        // Un message d'Alice ne doit pas se dechiffrer avec la cle d'envoi
        // d'Alice cote Alice (reflexion) : les deux sens ont des cles distinctes.
        let (alice_store, _bob_store) = established();
        let (nonce, ct) = alice_store.encrypt_for("conv-1", "bob", b"x").unwrap();
        assert!(alice_store.decrypt_from("conv-1", "bob", &nonce, &ct).is_err());
    }

    #[test]
    fn tampered_ephemeral_key_fails() {
        let (alice, bob) = pair();
        let (a_pending, _a_offer) = PendingHandshake::initiate(&alice, &bob.public()).unwrap();
        let (_b_pending, mut b_offer) = PendingHandshake::initiate(&bob, &alice.public()).unwrap();
        // MITM : remplace la cle ephemere de Bob par la sienne.
        let mallory = EphemeralSecret::random_from_rng(rand::rngs::OsRng);
        b_offer.ephemeral_key = hex::encode(PublicKey::from(&mallory).as_bytes());
        assert!(a_pending.complete(&b_offer).is_err());
    }

    #[test]
    fn unsigned_ephemeral_key_fails() {
        let (alice, bob) = pair();
        let (a_pending, _) = PendingHandshake::initiate(&alice, &bob.public()).unwrap();
        let (_b_pending, mut b_offer) = PendingHandshake::initiate(&bob, &alice.public()).unwrap();
        b_offer.signature = String::new();
        assert!(a_pending.complete(&b_offer).is_err());
    }

    #[test]
    fn offer_signed_by_other_identity_fails() {
        let (alice, bob) = pair();
        let mallory = IdentityKey::generate("bob".into());
        let (a_pending, _) = PendingHandshake::initiate(&alice, &bob.public()).unwrap();
        // Mallory se fait passer pour "bob" avec sa propre cle.
        let (_m_pending, mut m_offer) =
            PendingHandshake::initiate(&mallory, &alice.public()).unwrap();
        assert!(a_pending.complete(&m_offer).is_err());
        // Meme en recopiant la cle d'identite de Bob, la signature ne passe pas.
        let (a_pending, _) = PendingHandshake::initiate(&alice, &bob.public()).unwrap();
        m_offer.identity_key = bob.public().public_key;
        assert!(a_pending.complete(&m_offer).is_err());
    }

    #[test]
    fn offer_for_other_recipient_fails() {
        // Une offre de Bob destinee a Carol ne peut pas etre rejouee vers Alice.
        let (alice, bob) = pair();
        let carol = IdentityKey::generate("carol".into());
        let (a_pending, _) = PendingHandshake::initiate(&alice, &bob.public()).unwrap();
        let (_b_pending, b_to_carol) = PendingHandshake::initiate(&bob, &carol.public()).unwrap();
        assert!(a_pending.complete(&b_to_carol).is_err());
    }

    #[test]
    fn reflected_offer_fails() {
        let (alice, bob) = pair();
        let (a_pending, a_offer) = PendingHandshake::initiate(&alice, &bob.public()).unwrap();
        assert!(a_pending.complete(&a_offer).is_err());
    }

    #[test]
    fn nonces_never_repeat() {
        let (alice_store, _) = established();
        let mut seen = std::collections::HashSet::new();
        for _ in 0..10_000 {
            let (nonce, _) = alice_store.encrypt_for("conv-1", "bob", b"m").unwrap();
            assert!(seen.insert(nonce), "nonce repete");
        }
    }
}
