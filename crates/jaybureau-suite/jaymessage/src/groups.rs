//! Chiffrement de groupe — Sender Keys.
//!
//! Modele :
//! 1. Le createur d'un groupe genere une cle symetrique (Sender Key).
//! 2. La cle est distribuee a chaque membre via le canal 1:1 chiffre.
//! 3. Tous les membres chiffrent/dechiffrent les messages du groupe avec cette meme cle.
//! 4. Lors d'un membership change (ajout/retrait), la cle est rotee.

use crate::{ConversationId, JayMessageError, Result, UserId};
use chacha20poly1305::aead::{Aead, KeyInit};
use chacha20poly1305::{ChaCha20Poly1305, Nonce};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

/// Cle symetrique de groupe.
#[derive(Debug, Clone)]
pub struct GroupKey {
    pub conversation_id: ConversationId,
    pub key: [u8; 32],
    pub generation: u32,
    pub created_at: chrono::DateTime<chrono::Utc>,
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

    /// Chiffre un message avec la cle de groupe.
    pub fn encrypt(&self, plaintext: &[u8], nonce_counter: u64) -> Result<(String, String)> {
        let cipher = ChaCha20Poly1305::new(&self.key.into());
        let mut nonce_bytes = [0u8; 12];
        nonce_bytes[4..].copy_from_slice(&nonce_counter.to_le_bytes());
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, plaintext)
            .map_err(|e| JayMessageError::Crypto(format!("group encrypt: {e}")))?;
        Ok((hex::encode(nonce_bytes), hex::encode(ciphertext)))
    }

    /// Dechiffre un message du groupe.
    pub fn decrypt(&self, nonce_hex: &str, ciphertext_hex: &str) -> Result<Vec<u8>> {
        let nonce_bytes = hex::decode(nonce_hex)
            .map_err(|e| JayMessageError::Crypto(format!("nonce hex: {e}")))?;
        if nonce_bytes.len() != 12 {
            return Err(JayMessageError::Crypto("nonce 12 bytes".into()));
        }
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = hex::decode(ciphertext_hex)
            .map_err(|e| JayMessageError::Crypto(format!("ct hex: {e}")))?;

        let cipher = ChaCha20Poly1305::new(&self.key.into());
        cipher
            .decrypt(nonce, ciphertext.as_ref())
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
            .map(|k| k.generation + 1)
            .unwrap_or(1);
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
        let (nonce, ct) = key.encrypt(b"secret group message", 0).unwrap();
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
        let (nonce, ct) = k1.encrypt(b"top secret", 0).unwrap();
        assert!(k2.decrypt(&nonce, &ct).is_err());
    }
}
