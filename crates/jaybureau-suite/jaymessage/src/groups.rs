//! @id     jaybureau_suite_jaymessage_groups
//! @do     definir_group_key
//! @role   ui
//! @layer  core
//! @human  jaybureau-suite — module « jaymessage groups ».

//! Chiffrement de groupe — Sender Keys.
//!
//! Modele :
//! 1. Le createur d'un groupe genere une cle symetrique (Sender Key).
//! 2. La cle est distribuee a chaque membre via le canal 1:1 chiffre.
//! 3. Tous les membres chiffrent/dechiffrent les messages du groupe avec cette meme cle.
//! 4. Lors d'un membership change (ajout/retrait), la cle est rotee.

use crate::{ConversationId, JayMessageError, Result, UserId};
use chacha20poly1305::aead::{Aead, KeyInit, Payload};
use chacha20poly1305::{XChaCha20Poly1305, XNonce};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use zeroize::{Zeroize, ZeroizeOnDrop};

/// Domaine des donnees associees (AAD) des messages de groupe.
const GROUP_AAD_DOMAIN: &[u8] = b"miyukini/jaymessage/group/v2";

/// Cle symetrique de groupe.
///
/// La cle est privee, effacee a la destruction et jamais affichee par `Debug`.
/// Le chiffrement utilise XChaCha20-Poly1305 avec un nonce aleatoire de
/// 192 bits tire en interne : l'appelant ne fournit plus de compteur (R-009).
#[derive(Clone, Zeroize, ZeroizeOnDrop)]
pub struct GroupKey {
    #[zeroize(skip)]
    pub conversation_id: ConversationId,
    key: [u8; 32],
    #[zeroize(skip)]
    pub generation: u32,
    #[zeroize(skip)]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl std::fmt::Debug for GroupKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GroupKey")
            .field("conversation_id", &self.conversation_id)
            .field("key", &"<redacted>")
            .field("generation", &self.generation)
            .field("created_at", &self.created_at)
            .finish()
    }
}

impl GroupKey {
    /// Genere une nouvelle cle aleatoire.
    pub fn generate(conversation_id: ConversationId, generation: u32) -> Self {
        use rand::RngCore;
        let mut key = [0u8; 32];
        rand::rngs::OsRng.fill_bytes(&mut key);
        Self {
            conversation_id,
            key,
            generation,
            created_at: chrono::Utc::now(),
        }
    }

    /// Donnees associees : lient le chiffre au groupe et a la generation.
    fn aad(&self) -> Vec<u8> {
        let mut aad = Vec::with_capacity(GROUP_AAD_DOMAIN.len() + self.conversation_id.len() + 4);
        aad.extend_from_slice(GROUP_AAD_DOMAIN);
        aad.extend_from_slice(&self.generation.to_be_bytes());
        aad.extend_from_slice(self.conversation_id.as_bytes());
        aad
    }

    /// Chiffre un message avec la cle de groupe.
    ///
    /// Retourne `(nonce_hex, ciphertext_hex)`. Le nonce (24 octets) est tire
    /// de `OsRng` a chaque appel.
    pub fn encrypt(&self, plaintext: &[u8]) -> Result<(String, String)> {
        let cipher = XChaCha20Poly1305::new((&self.key).into());
        let nonce_bytes = crate::session::random_nonce();
        let aad = self.aad();
        let ciphertext = cipher
            .encrypt(
                XNonce::from_slice(&nonce_bytes),
                Payload {
                    msg: plaintext,
                    aad: &aad,
                },
            )
            .map_err(|e| JayMessageError::Crypto(format!("group encrypt: {e}")))?;
        Ok((hex::encode(nonce_bytes), hex::encode(ciphertext)))
    }

    /// Dechiffre un message du groupe.
    pub fn decrypt(&self, nonce_hex: &str, ciphertext_hex: &str) -> Result<Vec<u8>> {
        let nonce_bytes = hex::decode(nonce_hex)
            .map_err(|e| JayMessageError::Crypto(format!("nonce hex: {e}")))?;
        if nonce_bytes.len() != crate::session::NONCE_LEN {
            return Err(JayMessageError::Crypto("nonce 24 bytes".into()));
        }
        let ciphertext = hex::decode(ciphertext_hex)
            .map_err(|e| JayMessageError::Crypto(format!("ct hex: {e}")))?;

        let cipher = XChaCha20Poly1305::new((&self.key).into());
        let aad = self.aad();
        cipher
            .decrypt(
                XNonce::from_slice(&nonce_bytes),
                Payload {
                    msg: &ciphertext,
                    aad: &aad,
                },
            )
            .map_err(|e| JayMessageError::Crypto(format!("group decrypt: {e}")))
    }
}

/// Distribution d'une SenderKey a un membre.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SenderKeyDistribution {
    pub conversation_id: ConversationId,
    pub generation: u32,
    /// La SenderKey chiffree avec la session 1:1 du destinataire.
    pub encrypted_key_hex: String,
    pub nonce_hex: String,
    pub from_user: UserId,
    pub to_user: UserId,
    pub at: chrono::DateTime<chrono::Utc>,
}

/// Store local des cles de groupe.
pub struct GroupKeyStore {
    keys: Mutex<HashMap<ConversationId, GroupKey>>,
}

impl GroupKeyStore {
    pub fn new() -> Self {
        Self {
            keys: Mutex::new(HashMap::new()),
        }
    }

    pub fn set(&self, group_key: GroupKey) {
        if let Ok(mut k) = self.keys.lock() {
            k.insert(group_key.conversation_id.clone(), group_key);
        }
    }

    pub fn get(&self, conversation_id: &str) -> Option<GroupKey> {
        self.keys.lock().ok().and_then(|k| k.get(conversation_id).cloned())
    }

    pub fn rotate(&self, conversation_id: ConversationId) -> GroupKey {
        let new_gen = self
            .get(&conversation_id)
            .map_or(1, |k| k.generation.saturating_add(1));
        let key = GroupKey::generate(conversation_id, new_gen);
        self.set(key.clone());
        key
    }
}

impl Default for GroupKeyStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn group_encrypt_decrypt_roundtrip() {
        let key = GroupKey::generate("group-1".into(), 1);
        let (nonce, ct) = key.encrypt(b"secret group message").unwrap();
        let pt = key.decrypt(&nonce, &ct).unwrap();
        assert_eq!(pt, b"secret group message");
    }

    #[test]
    fn rotate_increments_generation() {
        let store = GroupKeyStore::new();
        let k1 = store.rotate("g1".into());
        let k2 = store.rotate("g1".into());
        let k3 = store.rotate("g1".into());
        assert_eq!(k1.generation, 1);
        assert_eq!(k2.generation, 2);
        assert_eq!(k3.generation, 3);
        assert_ne!(k1.key, k2.key);
        assert_ne!(k2.key, k3.key);
    }

    #[test]
    fn wrong_key_cannot_decrypt() {
        let k1 = GroupKey::generate("g1".into(), 1);
        let k2 = GroupKey::generate("g2".into(), 1);
        let (nonce, ct) = k1.encrypt(b"top secret").unwrap();
        assert!(k2.decrypt(&nonce, &ct).is_err());
    }

    #[test]
    fn group_nonces_never_repeat() {
        let key = GroupKey::generate("g1".into(), 1);
        let mut seen = std::collections::HashSet::new();
        for _ in 0..10_000 {
            let (nonce, _) = key.encrypt(b"m").unwrap();
            assert_eq!(nonce.len(), 48);
            assert!(seen.insert(nonce), "nonce de groupe repete");
        }
    }

    #[test]
    fn old_generation_ciphertext_rejected_after_rotation() {
        // Meme materiau de cle mais generation differente : l'AAD differe.
        let k1 = GroupKey::generate("g1".into(), 1);
        let mut k1_next = k1.clone();
        k1_next.generation = 2;
        let (nonce, ct) = k1.encrypt(b"x").unwrap();
        assert!(k1_next.decrypt(&nonce, &ct).is_err());
    }

    #[test]
    fn short_nonce_rejected() {
        let key = GroupKey::generate("g1".into(), 1);
        let (_, ct) = key.encrypt(b"x").unwrap();
        assert!(key.decrypt(&"00".repeat(12), &ct).is_err());
    }

    #[test]
    fn debug_does_not_leak_key() {
        let key = GroupKey::generate("g1".into(), 1);
        let dbg = format!("{key:?}");
        assert!(!dbg.contains(&hex::encode(key.key)));
        assert!(dbg.contains("<redacted>"));
    }
}
