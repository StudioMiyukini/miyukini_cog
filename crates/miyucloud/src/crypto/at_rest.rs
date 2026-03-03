//! Chiffrement at-rest ChaCha20-Poly1305.
//!
//! @id: miyucloud_crypto_at_rest
//! @do: encrypt_decrypt_chunks_with_chacha20poly1305
//! @role: crypto
//! @layer: infra
//!
//! Algorithme : ChaCha20-Poly1305 (AEAD).
//! Nonce : 12 bytes genere aleatoirement via `rand::OsRng`.
//! Le tag d'authentification (16 bytes) est inclus dans le ciphertext par la librairie.
//!
//! INVARIANT : jamais de lecture silencieuse de donnees corrompues. Si le tag
//! est invalide, une erreur `MiyucloudError::Crypto` est retournee.

use crate::errors::MiyucloudError;
use chacha20poly1305::aead::{Aead, KeyInit};
use chacha20poly1305::{ChaCha20Poly1305, Nonce};
use rand::RngCore;
use serde::{Deserialize, Serialize};

/// Chunk chiffre : nonce + ciphertext (avec tag inclus).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EncryptedChunk {
    /// Nonce de 12 bytes.
    pub nonce: [u8; 12],
    /// Ciphertext (inclut le tag Poly1305 de 16 bytes).
    pub ciphertext: Vec<u8>,
}

/// Chiffre un chunk de donnees avec une cle de 32 bytes.
///
/// Genere un nonce aleatoire de 12 bytes via `OsRng`.
/// Retourne un `EncryptedChunk` contenant le nonce et le ciphertext.
pub fn encrypt_chunk(key: &[u8; 32], plaintext: &[u8]) -> Result<EncryptedChunk, MiyucloudError> {
    let cipher = ChaCha20Poly1305::new_from_slice(key)
        .map_err(|e| MiyucloudError::Crypto(format!("Invalid key: {e}")))?;

    let mut nonce_bytes = [0u8; 12];
    rand::rngs::OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| MiyucloudError::Crypto(format!("Encryption failed: {e}")))?;

    Ok(EncryptedChunk {
        nonce: nonce_bytes,
        ciphertext,
    })
}

/// Dechiffre un chunk de donnees avec une cle de 32 bytes.
///
/// Verifie le tag Poly1305 et retourne une erreur si les donnees sont corrompues
/// ou si la cle est incorrecte.
pub fn decrypt_chunk(
    key: &[u8; 32],
    encrypted: &EncryptedChunk,
) -> Result<Vec<u8>, MiyucloudError> {
    let cipher = ChaCha20Poly1305::new_from_slice(key)
        .map_err(|e| MiyucloudError::Crypto(format!("Invalid key: {e}")))?;

    let nonce = Nonce::from_slice(&encrypted.nonce);

    cipher
        .decrypt(nonce, encrypted.ciphertext.as_ref())
        .map_err(|e| MiyucloudError::Crypto(format!("Decryption failed (corrupted data or wrong key): {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_key() -> [u8; 32] {
        [42u8; 32]
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let key = test_key();
        let plaintext = b"Hello, MiyuCloud! This is secret data.";
        let encrypted = encrypt_chunk(&key, plaintext).unwrap();
        let decrypted = decrypt_chunk(&key, &encrypted).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_encrypt_produces_different_ciphertext() {
        let key = test_key();
        let plaintext = b"same data";
        let enc1 = encrypt_chunk(&key, plaintext).unwrap();
        let enc2 = encrypt_chunk(&key, plaintext).unwrap();
        // Nonces should differ (random), therefore ciphertexts differ
        assert_ne!(enc1.nonce, enc2.nonce);
        assert_ne!(enc1.ciphertext, enc2.ciphertext);
        // But both should decrypt to the same plaintext
        assert_eq!(decrypt_chunk(&key, &enc1).unwrap(), plaintext);
        assert_eq!(decrypt_chunk(&key, &enc2).unwrap(), plaintext);
    }

    #[test]
    fn test_decrypt_wrong_key_fails() {
        let key = test_key();
        let plaintext = b"secret";
        let encrypted = encrypt_chunk(&key, plaintext).unwrap();
        let wrong_key = [0u8; 32];
        let result = decrypt_chunk(&wrong_key, &encrypted);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Decryption failed"));
    }

    #[test]
    fn test_decrypt_corrupted_data_fails() {
        let key = test_key();
        let plaintext = b"secret data";
        let mut encrypted = encrypt_chunk(&key, plaintext).unwrap();
        // Corrupt the ciphertext
        if let Some(byte) = encrypted.ciphertext.first_mut() {
            *byte ^= 0xFF;
        }
        let result = decrypt_chunk(&key, &encrypted);
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_plaintext() {
        let key = test_key();
        let encrypted = encrypt_chunk(&key, b"").unwrap();
        let decrypted = decrypt_chunk(&key, &encrypted).unwrap();
        assert!(decrypted.is_empty());
    }

    #[test]
    fn test_large_plaintext() {
        let key = test_key();
        let plaintext = vec![0xABu8; 256 * 1024]; // 256 KB
        let encrypted = encrypt_chunk(&key, &plaintext).unwrap();
        let decrypted = decrypt_chunk(&key, &encrypted).unwrap();
        assert_eq!(decrypted, plaintext);
    }
}
