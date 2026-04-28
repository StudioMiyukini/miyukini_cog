//! Sessions E2E par conversation.
//!
//! Reutilise `miyukini_cog_bridge::e2e::E2eSession` (ChaCha20-Poly1305).

use crate::{ConversationId, JayMessageError, Result, UserId};
use miyukini_cog_bridge::e2e::{E2eKeypair, E2eSession};
use std::collections::HashMap;
use std::sync::Mutex;

/// Store des sessions actives.
pub struct SessionStore {
    sessions: Mutex<HashMap<String, SessionEntry>>,
}

struct SessionEntry {
    session: E2eSession,
    /// Cle partagee (pour pouvoir recreer une session).
    shared_key: [u8; 32],
}

impl SessionStore {
    pub fn new() -> Self {
        Self {
            sessions: Mutex::new(HashMap::new()),
        }
    }

    /// Enregistre une session 1:1 apres un handshake reussi.
    pub fn register(
        &self,
        conversation_id: ConversationId,
        peer_id: UserId,
        shared_key: [u8; 32],
    ) {
        let key = session_key(&conversation_id, &peer_id);
        let session = E2eSession::from_shared_key(&shared_key);
        if let Ok(mut s) = self.sessions.lock() {
            s.insert(
                key,
                SessionEntry {
                    session,
                    shared_key,
                },
            );
        }
    }

    /// Verifie si une session existe.
    pub fn has_session(&self, conversation_id: &str, peer_id: &str) -> bool {
        let key = session_key(conversation_id, peer_id);
        self.sessions.lock().map(|s| s.contains_key(&key)).unwrap_or(false)
    }

    /// Chiffre un payload pour un destinataire specifique.
    pub fn encrypt_for(
        &self,
        conversation_id: &str,
        peer_id: &str,
        plaintext: &[u8],
    ) -> Result<(String, String)> {
        let key = session_key(conversation_id, peer_id);
        let mut sessions = self
            .sessions
            .lock()
            .map_err(|_| JayMessageError::Crypto("lock".into()))?;
        let entry = sessions
            .get_mut(&key)
            .ok_or_else(|| JayMessageError::NoSession(key.clone()))?;
        let encrypted = entry
            .session
            .encrypt(plaintext)
            .map_err(|e| JayMessageError::Crypto(format!("{e:?}")))?;
        Ok((hex::encode(encrypted.nonce), hex::encode(encrypted.ciphertext)))
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
        if nonce_bytes.len() != 12 {
            return Err(JayMessageError::Crypto("nonce must be 12 bytes".into()));
        }
        let mut nonce = [0u8; 12];
        nonce.copy_from_slice(&nonce_bytes);

        let ciphertext = hex::decode(ciphertext_hex)
            .map_err(|e| JayMessageError::Crypto(format!("ciphertext hex: {e}")))?;

        let msg = miyukini_cog_bridge::e2e::EncryptedMessage { nonce, ciphertext };
        entry
            .session
            .decrypt(&msg)
            .map_err(|e| JayMessageError::Crypto(format!("{e:?}")))
    }

    /// Nombre de sessions actives.
    pub fn count(&self) -> usize {
        self.sessions.lock().map(|s| s.len()).unwrap_or(0)
    }
}

impl Default for SessionStore {
    fn default() -> Self {
        Self::new()
    }
}

/// Genere une cle de session deterministe.
fn session_key(conversation_id: &str, peer_id: &str) -> String {
    format!("{conversation_id}::{peer_id}")
}

/// Effectue un handshake 1:1 et retourne la cle partagee.
pub fn perform_handshake(local: E2eKeypair, peer_public_key: &[u8; 32]) -> Result<[u8; 32]> {
    local
        .derive_shared_secret(peer_public_key)
        .map_err(|e| JayMessageError::Crypto(format!("{e:?}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handshake_and_encrypt_decrypt() {
        // Alice et Bob font un handshake
        let alice_kp = E2eKeypair::generate();
        let bob_kp = E2eKeypair::generate();

        let alice_pub = alice_kp.public_key_bytes();
        let bob_pub = bob_kp.public_key_bytes();

        let alice_secret = alice_kp.derive_shared_secret(&bob_pub).unwrap();
        let bob_secret = bob_kp.derive_shared_secret(&alice_pub).unwrap();
        assert_eq!(alice_secret, bob_secret);

        // Chacun enregistre sa session
        let alice_store = SessionStore::new();
        let bob_store = SessionStore::new();

        alice_store.register("conv-1".into(), "bob".into(), alice_secret);
        bob_store.register("conv-1".into(), "alice".into(), bob_secret);

        // Alice chiffre un message
        let (nonce, ciphertext) = alice_store
            .encrypt_for("conv-1", "bob", b"hello bob")
            .unwrap();

        // Bob dechiffre
        let plaintext = bob_store
            .decrypt_from("conv-1", "alice", &nonce, &ciphertext)
            .unwrap();
        assert_eq!(plaintext, b"hello bob");
    }
}
