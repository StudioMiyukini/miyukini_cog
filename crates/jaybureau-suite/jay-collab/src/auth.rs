//! @id     jaybureau_suite_jay_collab_auth
//! @do     authentifier_participant_collab
//! @role   securite
//! @layer  core
//! @human  jaybureau-suite — module « jay-collab auth » : jeton de connexion signé ed25519 et annuaire de clés (R-008).

//! Authentification des participants d'une salle collaborative.
//!
//! ## Modèle
//!
//! - Chaque utilisateur possède une identité ed25519 (même mécanisme que
//!   `jaymessage::IdentityKey`, clés publiques en hex).
//! - Pour rejoindre une salle, le client envoie en **premier message**
//!   WebSocket un [`JoinRequest`] contenant un [`JoinToken`] signé sur
//!   (domaine, doc_id, user_id, horodatage, nonce aléatoire).
//! - Le serveur vérifie le jeton via un [`IdentityVerifier`] (par défaut
//!   [`KeyDirectory`]) : clé connue pour ce `user_id`, `verify_strict`,
//!   fraîcheur, nonce jamais vu (anti-rejeu). Toute erreur = refus (C-12).
//! - L'identité retenue est **celle du jeton vérifié**, jamais un champ libre
//!   des messages suivants (C-20).

use crate::CollabError;
use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
use jaybureau_core::{DocId, UserId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Mutex, RwLock};

/// Domaine de séparation des signatures de jeton.
pub const JOIN_DOMAIN: &[u8] = b"miyukini/jay-collab/join/v1";

/// Durée de validité d'un jeton (secondes).
pub const MAX_TOKEN_AGE_SECS: i64 = 120;

/// Avance d'horloge tolérée (secondes).
pub const MAX_CLOCK_SKEW_SECS: i64 = 30;

/// Taille maximale d'un identifiant (user_id, doc_id) dans un jeton (C-13).
pub const MAX_ID_LEN: usize = 256;

/// Taille du nonce de jeton (octets).
const TOKEN_NONCE_LEN: usize = 16;

/// Nombre maximal de nonces mémorisés par l'anti-rejeu (C-13).
const MAX_SEEN_NONCES: usize = 100_000;

/// Jeton de connexion à une salle, signé par l'identité de l'utilisateur.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JoinToken {
    pub user_id: UserId,
    pub doc_id: DocId,
    /// Horodatage Unix (secondes) d'émission.
    pub issued_at: i64,
    /// Nonce aléatoire (16 octets, hex).
    pub nonce: String,
    /// Signature ed25519 (64 octets, hex).
    pub signature: String,
}

/// Premier message envoyé par le client sur la WebSocket collab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum JoinRequest {
    #[serde(rename = "join")]
    Join { token: JoinToken },
}

fn token_message(user_id: &str, doc_id: &str, issued_at: i64, nonce: &[u8]) -> Vec<u8> {
    let mut m = Vec::with_capacity(JOIN_DOMAIN.len() + 8 + user_id.len() + doc_id.len() + 8 + nonce.len());
    m.extend_from_slice(JOIN_DOMAIN);
    // Longueurs préfixées : aucune ambiguïté de concaténation.
    m.extend_from_slice(&(user_id.len() as u32).to_be_bytes());
    m.extend_from_slice(user_id.as_bytes());
    m.extend_from_slice(&(doc_id.len() as u32).to_be_bytes());
    m.extend_from_slice(doc_id.as_bytes());
    m.extend_from_slice(&issued_at.to_be_bytes());
    m.extend_from_slice(nonce);
    m
}

/// Horodatage Unix courant (secondes).
pub fn unix_now() -> Result<i64, CollabError> {
    let d = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_| CollabError::AuthInvalid("horloge système invalide".into()))?;
    i64::try_from(d.as_secs()).map_err(|_| CollabError::AuthInvalid("horloge hors bornes".into()))
}

impl JoinToken {
    /// Signe un jeton pour `user_id` sur `doc_id` à l'instant `issued_at`.
    pub fn sign(
        signing_key: &SigningKey,
        user_id: impl Into<UserId>,
        doc_id: impl Into<DocId>,
        issued_at: i64,
    ) -> Result<Self, CollabError> {
        use rand::RngCore;
        let user_id = user_id.into();
        let doc_id = doc_id.into();
        if user_id.is_empty() || user_id.len() > MAX_ID_LEN || doc_id.len() > MAX_ID_LEN {
            return Err(CollabError::AuthInvalid("identifiant vide ou trop long".into()));
        }
        let mut nonce = [0u8; TOKEN_NONCE_LEN];
        rand::rngs::OsRng.fill_bytes(&mut nonce);
        let sig: Signature = signing_key.sign(&token_message(&user_id, &doc_id, issued_at, &nonce));
        Ok(Self {
            user_id,
            doc_id,
            issued_at,
            nonce: hex::encode(nonce),
            signature: hex::encode(sig.to_bytes()),
        })
    }
}

/// Vérifie un jeton de connexion et retourne l'identité authentifiée.
pub trait IdentityVerifier: Send + Sync {
    /// `expected_doc` : document de la salle visée (un jeton n'est valable
    /// que pour ce document).
    fn verify(&self, token: &JoinToken, expected_doc: &str) -> Result<UserId, CollabError>;
}

/// Annuaire de clés publiques ed25519 (user_id → clé) avec anti-rejeu.
///
/// Seules les identités enregistrées par le serveur (source de confiance :
/// annuaire Miyukini Connect, fédération) peuvent se connecter.
pub struct KeyDirectory {
    keys: RwLock<HashMap<UserId, VerifyingKey>>,
    /// Nonces déjà acceptés → horodatage du jeton (purgés après expiration).
    seen: Mutex<HashMap<String, i64>>,
}

impl KeyDirectory {
    pub fn new() -> Self {
        Self {
            keys: RwLock::new(HashMap::new()),
            seen: Mutex::new(HashMap::new()),
        }
    }

    /// Enregistre (ou remplace) la clé publique d'un utilisateur.
    pub fn register(&self, user_id: UserId, key: VerifyingKey) -> Result<(), CollabError> {
        let mut keys = self
            .keys
            .write()
            .map_err(|_| CollabError::AuthInvalid("annuaire verrouillé".into()))?;
        keys.insert(user_id, key);
        Ok(())
    }

    /// Enregistre une clé publique fournie en hex (format `PublicIdentity`).
    pub fn register_hex(&self, user_id: UserId, public_key_hex: &str) -> Result<(), CollabError> {
        let bytes: [u8; 32] = hex::decode(public_key_hex)
            .ok()
            .and_then(|b| b.as_slice().try_into().ok())
            .ok_or_else(|| CollabError::AuthInvalid("clé publique: 32 octets hex attendus".into()))?;
        let key = VerifyingKey::from_bytes(&bytes)
            .map_err(|_| CollabError::AuthInvalid("clé publique invalide".into()))?;
        self.register(user_id, key)
    }

    /// Retire un utilisateur de l'annuaire.
    pub fn revoke(&self, user_id: &str) {
        if let Ok(mut keys) = self.keys.write() {
            keys.remove(user_id);
        }
    }

    /// Vérification avec une horloge explicite (tests).
    pub fn verify_at(
        &self,
        token: &JoinToken,
        expected_doc: &str,
        now: i64,
    ) -> Result<UserId, CollabError> {
        if token.user_id.is_empty()
            || token.user_id.len() > MAX_ID_LEN
            || token.doc_id.len() > MAX_ID_LEN
        {
            return Err(CollabError::AuthInvalid("identifiant vide ou trop long".into()));
        }
        if token.doc_id != expected_doc {
            return Err(CollabError::AuthInvalid("jeton émis pour un autre document".into()));
        }
        if token.issued_at > now.saturating_add(MAX_CLOCK_SKEW_SECS)
            || now.saturating_sub(token.issued_at) > MAX_TOKEN_AGE_SECS
        {
            return Err(CollabError::TokenExpired);
        }
        let key = {
            let keys = self
                .keys
                .read()
                .map_err(|_| CollabError::AuthInvalid("annuaire verrouillé".into()))?;
            *keys.get(&token.user_id).ok_or(CollabError::UnknownIdentity)?
        };
        // Hex canonique minuscule exigé : un même nonce ne doit avoir qu'une
        // seule représentation (sinon rejeu par changement de casse).
        let nonce: [u8; TOKEN_NONCE_LEN] = decode_canonical_hex(&token.nonce)
            .ok_or_else(|| CollabError::AuthInvalid("nonce invalide".into()))?;
        let sig: [u8; 64] = decode_canonical_hex(&token.signature)
            .ok_or_else(|| CollabError::AuthInvalid("signature invalide".into()))?;
        let msg = token_message(&token.user_id, &token.doc_id, token.issued_at, &nonce);
        key.verify_strict(&msg, &Signature::from_bytes(&sig))
            .map_err(|_| CollabError::AuthInvalid("signature invalide".into()))?;

        // Anti-rejeu : nonce enregistré seulement après signature valide.
        let mut seen = self
            .seen
            .lock()
            .map_err(|_| CollabError::AuthInvalid("anti-rejeu verrouillé".into()))?;
        let horizon = now.saturating_sub(MAX_TOKEN_AGE_SECS + MAX_CLOCK_SKEW_SECS);
        seen.retain(|_, issued| *issued >= horizon);
        // Clé d'anti-rejeu sur les octets décodés (forme canonique).
        let replay_key = format!("{}:{}", token.user_id, hex::encode(nonce));
        if seen.contains_key(&replay_key) {
            return Err(CollabError::TokenReplayed);
        }
        if seen.len() >= MAX_SEEN_NONCES {
            // Saturation : on refuse plutôt que d'oublier des nonces (C-12).
            return Err(CollabError::AuthInvalid("anti-rejeu saturé".into()));
        }
        seen.insert(replay_key, token.issued_at);
        Ok(token.user_id.clone())
    }
}

/// Décode une chaîne hex de longueur exacte `N` octets, uniquement si elle est
/// en forme canonique (minuscules) ; sinon `None`.
fn decode_canonical_hex<const N: usize>(s: &str) -> Option<[u8; N]> {
    if s.len() != N * 2 {
        return None;
    }
    let bytes: [u8; N] = hex::decode(s).ok()?.as_slice().try_into().ok()?;
    (hex::encode(bytes) == s).then_some(bytes)
}

impl Default for KeyDirectory {
    fn default() -> Self {
        Self::new()
    }
}

impl IdentityVerifier for KeyDirectory {
    fn verify(&self, token: &JoinToken, expected_doc: &str) -> Result<UserId, CollabError> {
        self.verify_at(token, expected_doc, unix_now()?)
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    pub(crate) fn new_key() -> SigningKey {
        use rand::RngCore;
        let mut seed = [0u8; 32];
        rand::rngs::OsRng.fill_bytes(&mut seed);
        SigningKey::from_bytes(&seed)
    }

    #[test]
    fn valid_token_accepted_once() {
        let dir = KeyDirectory::new();
        let k = new_key();
        dir.register("alice".into(), k.verifying_key()).unwrap();
        let now = unix_now().unwrap();
        let t = JoinToken::sign(&k, "alice", "doc-1", now).unwrap();
        assert_eq!(dir.verify_at(&t, "doc-1", now).unwrap(), "alice");
        // Rejeu refusé
        assert!(matches!(dir.verify_at(&t, "doc-1", now), Err(CollabError::TokenReplayed)));
    }

    #[test]
    fn replay_with_uppercased_nonce_refused() {
        let dir = KeyDirectory::new();
        let k = new_key();
        dir.register("alice".into(), k.verifying_key()).unwrap();
        let now = unix_now().unwrap();
        let t = JoinToken::sign(&k, "alice", "doc-1", now).unwrap();
        // Variante non canonique refusée même avant le premier usage.
        let mut upper = t.clone();
        upper.nonce = upper.nonce.to_uppercase();
        if upper.nonce != t.nonce {
            assert!(matches!(
                dir.verify_at(&upper, "doc-1", now),
                Err(CollabError::AuthInvalid(_))
            ));
        }
        assert!(dir.verify_at(&t, "doc-1", now).is_ok());
        // Rejeu par changement de casse (nonce et signature) refusé.
        let mut replay = t.clone();
        replay.nonce = replay.nonce.to_uppercase();
        replay.signature = replay.signature.to_uppercase();
        assert!(dir.verify_at(&replay, "doc-1", now).is_err());
        // Forcer un nonce avec lettres pour que le test soit significatif.
        let mut t2 = JoinToken::sign(&k, "alice", "doc-1", now).unwrap();
        while t2.nonce == t2.nonce.to_uppercase() {
            t2 = JoinToken::sign(&k, "alice", "doc-1", now).unwrap();
        }
        assert!(dir.verify_at(&t2, "doc-1", now).is_ok());
        let mut r2 = t2.clone();
        r2.nonce = r2.nonce.to_uppercase();
        assert!(dir.verify_at(&r2, "doc-1", now).is_err());
        assert!(matches!(dir.verify_at(&t2, "doc-1", now), Err(CollabError::TokenReplayed)));
    }

    #[test]
    fn replay_cache_expires_entries() {
        let dir = KeyDirectory::new();
        let k = new_key();
        dir.register("alice".into(), k.verifying_key()).unwrap();
        let now = unix_now().unwrap();
        let t = JoinToken::sign(&k, "alice", "doc-1", now).unwrap();
        assert!(dir.verify_at(&t, "doc-1", now).is_ok());
        assert_eq!(dir.seen.lock().unwrap().len(), 1);
        // Après la fenêtre de validité, l'entrée est purgée au prochain appel.
        let later = now + MAX_TOKEN_AGE_SECS + MAX_CLOCK_SKEW_SECS + 1;
        let t2 = JoinToken::sign(&k, "alice", "doc-1", later).unwrap();
        assert!(dir.verify_at(&t2, "doc-1", later).is_ok());
        assert_eq!(dir.seen.lock().unwrap().len(), 1);
        // Et le vieux jeton reste refusé (expiré).
        assert!(matches!(dir.verify_at(&t, "doc-1", later), Err(CollabError::TokenExpired)));
    }

    #[test]
    fn unknown_identity_refused() {
        let dir = KeyDirectory::new();
        let k = new_key();
        let now = unix_now().unwrap();
        let t = JoinToken::sign(&k, "mallory", "doc-1", now).unwrap();
        assert!(matches!(dir.verify_at(&t, "doc-1", now), Err(CollabError::UnknownIdentity)));
    }

    #[test]
    fn forged_signature_refused() {
        let dir = KeyDirectory::new();
        let alice = new_key();
        let mallory = new_key();
        dir.register("alice".into(), alice.verifying_key()).unwrap();
        let now = unix_now().unwrap();
        // Mallory signe un jeton au nom d'alice avec sa propre clé.
        let t = JoinToken::sign(&mallory, "alice", "doc-1", now).unwrap();
        assert!(dir.verify_at(&t, "doc-1", now).is_err());
    }

    #[test]
    fn tampered_token_refused() {
        let dir = KeyDirectory::new();
        let k = new_key();
        dir.register("alice".into(), k.verifying_key()).unwrap();
        dir.register("bob".into(), new_key().verifying_key()).unwrap();
        let now = unix_now().unwrap();
        let mut t = JoinToken::sign(&k, "alice", "doc-1", now).unwrap();
        t.user_id = "bob".into();
        assert!(dir.verify_at(&t, "doc-1", now).is_err());
    }

    #[test]
    fn token_for_other_doc_refused() {
        let dir = KeyDirectory::new();
        let k = new_key();
        dir.register("alice".into(), k.verifying_key()).unwrap();
        let now = unix_now().unwrap();
        let t = JoinToken::sign(&k, "alice", "doc-1", now).unwrap();
        assert!(dir.verify_at(&t, "doc-2", now).is_err());
    }

    #[test]
    fn expired_token_refused() {
        let dir = KeyDirectory::new();
        let k = new_key();
        dir.register("alice".into(), k.verifying_key()).unwrap();
        let now = unix_now().unwrap();
        let t = JoinToken::sign(&k, "alice", "doc-1", now - MAX_TOKEN_AGE_SECS - 1).unwrap();
        assert!(matches!(dir.verify_at(&t, "doc-1", now), Err(CollabError::TokenExpired)));
        let t = JoinToken::sign(&k, "alice", "doc-1", now + MAX_CLOCK_SKEW_SECS + 60).unwrap();
        assert!(matches!(dir.verify_at(&t, "doc-1", now), Err(CollabError::TokenExpired)));
    }

    #[test]
    fn empty_signature_refused() {
        let dir = KeyDirectory::new();
        let k = new_key();
        dir.register("alice".into(), k.verifying_key()).unwrap();
        let now = unix_now().unwrap();
        let mut t = JoinToken::sign(&k, "alice", "doc-1", now).unwrap();
        t.signature.clear();
        assert!(dir.verify_at(&t, "doc-1", now).is_err());
    }
}
