//! Chiffrement E2E pour la synchronisation P2P.
//!
//! @id: miyucloud_crypto_e2e
//! @do: implement_x25519_dh_ephemeral_e2e_encryption_with_pfs
//! @role: crypto
//! @layer: infra
//!
//! Echange de cles X25519 Diffie-Hellman ephemere avec Perfect Forward Secrecy.
//! Chaque session de sync genere une nouvelle paire de cles ephemeres.
//! La cle de session est derivee via HKDF-SHA256 et utilisee pour chiffrer
//! tous les messages avec ChaCha20-Poly1305.

use crate::errors::MiyucloudError;
use chacha20poly1305::aead::{Aead, KeyInit};
use chacha20poly1305::{ChaCha20Poly1305, Nonce};
use hkdf::Hkdf;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use x25519_dalek::{PublicKey, StaticSecret};

/// Contexte HKDF pour la derivation de la cle de session sync.
const SESSION_HKDF_CONTEXT: &[u8] = b"miyucloud-sync-session";

/// Message chiffre E2E.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EncryptedMessage {
    /// Nonce de 12 bytes.
    pub nonce: [u8; 12],
    /// Ciphertext (inclut le tag Poly1305 de 16 bytes).
    pub ciphertext: Vec<u8>,
}

/// Paire de cles ephemeres pour une session de sync.
///
/// La cle privee est gardee en memoire uniquement et jamais serialisee.
/// La cle publique est envoyee au pair distant.
pub struct EphemeralKeypair {
    /// Cle privee X25519 (jamais serialisee).
    secret: StaticSecret,
    /// Cle publique X25519 correspondante.
    public: PublicKey,
}

impl EphemeralKeypair {
    /// Retourne une reference a la cle publique.
    pub fn public_key(&self) -> &PublicKey {
        &self.public
    }

    /// Retourne les bytes de la cle publique.
    pub fn public_key_bytes(&self) -> [u8; 32] {
        self.public.to_bytes()
    }
}

impl std::fmt::Debug for EphemeralKeypair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EphemeralKeypair")
            .field("secret", &"[REDACTED]")
            .field("public", &self.public)
            .finish()
    }
}

/// Genere une paire de cles ephemeres X25519.
///
/// Chaque appel produit une nouvelle paire pour garantir le Perfect Forward Secrecy.
pub fn generate_ephemeral_keypair() -> EphemeralKeypair {
    let rng = rand::rngs::OsRng;
    let secret = StaticSecret::random_from_rng(rng);
    let public = PublicKey::from(&secret);
    EphemeralKeypair { secret, public }
}

/// Derive la cle de session partagee via X25519 DH + HKDF-SHA256.
///
/// Les deux pairs appellent cette fonction avec leur cle privee locale
/// et la cle publique du pair distant. Le secret partage resultant est
/// identique des deux cotes.
///
/// Contexte HKDF : `"miyucloud-sync-session"`.
pub fn derive_session_key(
    local_keypair: &EphemeralKeypair,
    remote_public: &PublicKey,
) -> Result<[u8; 32], MiyucloudError> {
    // X25519 Diffie-Hellman
    let shared_secret = local_keypair.secret.diffie_hellman(remote_public);

    // HKDF-SHA256 pour deriver une cle de session robuste
    let hkdf = Hkdf::<Sha256>::new(None, shared_secret.as_bytes());
    let mut session_key = [0u8; 32];
    hkdf.expand(SESSION_HKDF_CONTEXT, &mut session_key)
        .map_err(|e| MiyucloudError::Crypto(format!("HKDF session key derivation failed: {e}")))?;

    Ok(session_key)
}

/// Chiffre un message avec la cle de session (ChaCha20-Poly1305).
///
/// Genere un nonce aleatoire de 12 bytes via `OsRng`.
pub fn encrypt_message(
    session_key: &[u8; 32],
    plaintext: &[u8],
) -> Result<EncryptedMessage, MiyucloudError> {
    let cipher = ChaCha20Poly1305::new_from_slice(session_key)
        .map_err(|e| MiyucloudError::Crypto(format!("Invalid session key: {e}")))?;

    let mut nonce_bytes = [0u8; 12];
    rand::rngs::OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| MiyucloudError::Crypto(format!("E2E encryption failed: {e}")))?;

    Ok(EncryptedMessage {
        nonce: nonce_bytes,
        ciphertext,
    })
}

/// Dechiffre un message avec la cle de session (ChaCha20-Poly1305).
///
/// Verifie le tag Poly1305 et retourne une erreur si les donnees sont
/// corrompues ou si la cle est incorrecte.
pub fn decrypt_message(
    session_key: &[u8; 32],
    encrypted: &EncryptedMessage,
) -> Result<Vec<u8>, MiyucloudError> {
    let cipher = ChaCha20Poly1305::new_from_slice(session_key)
        .map_err(|e| MiyucloudError::Crypto(format!("Invalid session key: {e}")))?;

    let nonce = Nonce::from_slice(&encrypted.nonce);

    cipher
        .decrypt(nonce, encrypted.ciphertext.as_ref())
        .map_err(|e| {
            MiyucloudError::Crypto(format!(
                "E2E decryption failed (corrupted data or wrong key): {e}"
            ))
        })
}

/// Calcule le fingerprint d'une cle publique X25519.
///
/// Le fingerprint est le SHA-256 de la cle publique, tronque a 16 caracteres hex.
/// Il est affichable pour verification manuelle par l'utilisateur.
pub fn compute_fingerprint(public_key: &PublicKey) -> String {
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(public_key.as_bytes());
    let hash = hasher.finalize();
    // Truncate to 16 hex chars (8 bytes)
    hash.iter()
        .take(8)
        .fold(String::with_capacity(16), |mut acc, b| {
            use std::fmt::Write;
            let _ = write!(acc, "{b:02x}");
            acc
        })
}

/// Calcule le fingerprint depuis des bytes de cle publique.
pub fn compute_fingerprint_from_bytes(public_key_bytes: &[u8; 32]) -> String {
    let pk = PublicKey::from(*public_key_bytes);
    compute_fingerprint(&pk)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_exchange_produces_shared_secret() {
        // Alice generates keypair
        let alice = generate_ephemeral_keypair();
        // Bob generates keypair
        let bob = generate_ephemeral_keypair();

        // Both derive the session key
        let alice_key = derive_session_key(&alice, bob.public_key()).unwrap();
        let bob_key = derive_session_key(&bob, alice.public_key()).unwrap();

        // Both should have the same session key
        assert_eq!(alice_key, bob_key);
    }

    #[test]
    fn test_encrypt_decrypt_session() {
        let alice = generate_ephemeral_keypair();
        let bob = generate_ephemeral_keypair();

        let session_key = derive_session_key(&alice, bob.public_key()).unwrap();

        let plaintext = b"Hello from Alice to Bob!";
        let encrypted = encrypt_message(&session_key, plaintext).unwrap();
        let decrypted = decrypt_message(&session_key, &encrypted).unwrap();

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_different_sessions_different_keys() {
        let alice1 = generate_ephemeral_keypair();
        let bob1 = generate_ephemeral_keypair();
        let key1 = derive_session_key(&alice1, bob1.public_key()).unwrap();

        let alice2 = generate_ephemeral_keypair();
        let bob2 = generate_ephemeral_keypair();
        let key2 = derive_session_key(&alice2, bob2.public_key()).unwrap();

        // Different ephemeral keypairs -> different session keys (PFS)
        assert_ne!(key1, key2);
    }

    #[test]
    fn test_fingerprint_from_public_key() {
        let kp = generate_ephemeral_keypair();
        let fp = compute_fingerprint(kp.public_key());

        // Fingerprint should be 16 hex chars
        assert_eq!(fp.len(), 16);
        assert!(fp.chars().all(|c| c.is_ascii_hexdigit()));

        // Same key should produce same fingerprint
        let fp2 = compute_fingerprint(kp.public_key());
        assert_eq!(fp, fp2);
    }

    #[test]
    fn test_fingerprint_different_keys() {
        let kp1 = generate_ephemeral_keypair();
        let kp2 = generate_ephemeral_keypair();

        let fp1 = compute_fingerprint(kp1.public_key());
        let fp2 = compute_fingerprint(kp2.public_key());

        // Different keys should produce different fingerprints (overwhelmingly likely)
        assert_ne!(fp1, fp2);
    }

    #[test]
    fn test_decrypt_wrong_key_fails() {
        let alice = generate_ephemeral_keypair();
        let bob = generate_ephemeral_keypair();
        let session_key = derive_session_key(&alice, bob.public_key()).unwrap();

        let encrypted = encrypt_message(&session_key, b"secret data").unwrap();

        // Try to decrypt with a different key
        let wrong_key = [0u8; 32];
        let result = decrypt_message(&wrong_key, &encrypted);
        assert!(result.is_err());
    }

    #[test]
    fn test_encrypt_produces_different_ciphertext() {
        let alice = generate_ephemeral_keypair();
        let bob = generate_ephemeral_keypair();
        let session_key = derive_session_key(&alice, bob.public_key()).unwrap();

        let enc1 = encrypt_message(&session_key, b"same data").unwrap();
        let enc2 = encrypt_message(&session_key, b"same data").unwrap();

        // Different random nonces -> different ciphertexts
        assert_ne!(enc1.nonce, enc2.nonce);
        assert_ne!(enc1.ciphertext, enc2.ciphertext);

        // But both decrypt to the same plaintext
        assert_eq!(
            decrypt_message(&session_key, &enc1).unwrap(),
            decrypt_message(&session_key, &enc2).unwrap(),
        );
    }

    #[test]
    fn test_encrypted_message_serialize_roundtrip() {
        let key = [42u8; 32];
        let encrypted = encrypt_message(&key, b"test").unwrap();
        let json = serde_json::to_string(&encrypted).unwrap();
        let restored: EncryptedMessage = serde_json::from_str(&json).unwrap();
        assert_eq!(encrypted, restored);
    }

    #[test]
    fn test_fingerprint_from_bytes() {
        let kp = generate_ephemeral_keypair();
        let fp1 = compute_fingerprint(kp.public_key());
        let fp2 = compute_fingerprint_from_bytes(&kp.public_key_bytes());
        assert_eq!(fp1, fp2);
    }

    #[test]
    fn test_empty_message_encrypt_decrypt() {
        let alice = generate_ephemeral_keypair();
        let bob = generate_ephemeral_keypair();
        let key = derive_session_key(&alice, bob.public_key()).unwrap();

        let encrypted = encrypt_message(&key, b"").unwrap();
        let decrypted = decrypt_message(&key, &encrypted).unwrap();
        assert!(decrypted.is_empty());
    }

    #[test]
    fn test_large_message_encrypt_decrypt() {
        let alice = generate_ephemeral_keypair();
        let bob = generate_ephemeral_keypair();
        let key = derive_session_key(&alice, bob.public_key()).unwrap();

        let plaintext = vec![0xABu8; 256 * 1024]; // 256 KB
        let encrypted = encrypt_message(&key, &plaintext).unwrap();
        let decrypted = decrypt_message(&key, &encrypted).unwrap();
        assert_eq!(decrypted, plaintext);
    }
}
