//! Gestion des cles cryptographiques MiyuCloud.
//!
//! @id: miyucloud_crypto_keys
//! @do: derive_master_and_file_keys_with_argon2_hkdf
//! @role: crypto
//! @layer: infra
//!
//! Hierarchie des cles :
//! - Passphrase utilisateur (jamais stockee)
//! - Master Key (256 bits, en memoire uniquement, derivee via Argon2id)
//! - File Key (256 bits, derivee a la volee via HKDF-SHA256)
//!
//! INVARIANT : La master key n'est JAMAIS ecrite sur disque.

use crate::errors::MiyucloudError;
use argon2::Argon2;
use hkdf::Hkdf;
use rand::RngCore;
use sha2::Sha256;

/// Gestionnaire de cles cryptographiques.
///
/// Stocke la master key en memoire et fournit des methodes pour deriver
/// des cles de fichiers.
///
/// INVARIANT : La master key n'est JAMAIS serialisee ou ecrite sur disque.
/// L'implementation Debug masque volontairement la cle.
pub struct KeyManager {
    master_key: [u8; 32],
}

impl std::fmt::Debug for KeyManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KeyManager")
            .field("master_key", &"[REDACTED]")
            .finish()
    }
}

impl KeyManager {
    /// Cree un `KeyManager` a partir d'une passphrase et d'un sel.
    pub fn from_passphrase(passphrase: &str, salt: &[u8]) -> Result<Self, MiyucloudError> {
        let master_key = Self::derive_master_key(passphrase, salt)?;
        Ok(Self { master_key })
    }

    /// Cree un `KeyManager` a partir d'une master key existante.
    pub fn from_master_key(key: [u8; 32]) -> Self {
        Self { master_key: key }
    }

    /// Retourne une reference a la master key.
    pub fn master_key(&self) -> &[u8; 32] {
        &self.master_key
    }

    /// Genere un sel aleatoire de 16 bytes.
    pub fn generate_salt() -> [u8; 16] {
        let mut salt = [0u8; 16];
        rand::rngs::OsRng.fill_bytes(&mut salt);
        salt
    }

    /// Derive la master key depuis une passphrase et un sel via Argon2id.
    ///
    /// Parametres :
    /// - En mode release : memory = 64 MB, iterations = 3, parallelism = 4
    /// - En mode debug : parametres reduits pour les tests (memory = 4 KB, iterations = 1, parallelism = 1)
    pub fn derive_master_key(passphrase: &str, salt: &[u8]) -> Result<[u8; 32], MiyucloudError> {
        let params = argon2_params();
        let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);
        let mut key = [0u8; 32];
        argon2
            .hash_password_into(passphrase.as_bytes(), salt, &mut key)
            .map_err(|e| MiyucloudError::Crypto(format!("Argon2id derivation failed: {e}")))?;
        Ok(key)
    }

    /// Derive une cle de fichier depuis la master key et un file_id via HKDF-SHA256.
    ///
    /// Le contexte HKDF est `"miyucloud-file-{file_id}"`.
    pub fn derive_file_key(
        master_key: &[u8; 32],
        file_id: &str,
    ) -> Result<[u8; 32], MiyucloudError> {
        let info = format!("miyucloud-file-{file_id}");
        let hkdf = Hkdf::<Sha256>::new(None, master_key);
        let mut file_key = [0u8; 32];
        hkdf.expand(info.as_bytes(), &mut file_key)
            .map_err(|e| MiyucloudError::Crypto(format!("HKDF derivation failed: {e}")))?;
        Ok(file_key)
    }

    /// Derive la cle pour un fichier specifique.
    pub fn file_key(&self, file_id: &str) -> Result<[u8; 32], MiyucloudError> {
        Self::derive_file_key(&self.master_key, file_id)
    }

    /// Verifie une passphrase en tentant de dechiffrer un canary.
    ///
    /// Le canary est un petit bloc de donnees connues chiffre avec la master key.
    /// Si le dechiffrement reussit, la passphrase est correcte.
    pub fn verify_passphrase(
        master_key: &[u8; 32],
        canary_ciphertext: &[u8],
        canary_nonce: &[u8; 12],
    ) -> bool {
        use crate::crypto::at_rest::{decrypt_chunk, EncryptedChunk};
        let encrypted = EncryptedChunk {
            nonce: *canary_nonce,
            ciphertext: canary_ciphertext.to_vec(),
        };
        decrypt_chunk(master_key, &encrypted).is_ok()
    }

    /// Cree un canary chiffre pour verification future de la passphrase.
    pub fn create_canary(
        master_key: &[u8; 32],
    ) -> Result<crate::crypto::at_rest::EncryptedChunk, MiyucloudError> {
        use crate::crypto::at_rest::encrypt_chunk;
        // Canary = donnees connues
        let canary_data = b"MIYUCLOUD_CANARY_V1";
        encrypt_chunk(master_key, canary_data)
    }
}

/// Retourne les parametres Argon2id.
/// En debug: reduits pour les tests.
/// En release: parametres complets (64 MB, 3 iterations, 4 parallelisme).
fn argon2_params() -> argon2::Params {
    #[cfg(debug_assertions)]
    {
        argon2::Params::new(4096, 1, 1, Some(32))
            .expect("valid argon2 params")
    }
    #[cfg(not(debug_assertions))]
    {
        argon2::Params::new(65536, 3, 4, Some(32))
            .expect("valid argon2 params")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_master_key_deterministic() {
        let salt = [1u8; 16];
        let key1 = KeyManager::derive_master_key("my passphrase", &salt).unwrap();
        let key2 = KeyManager::derive_master_key("my passphrase", &salt).unwrap();
        assert_eq!(key1, key2);
    }

    #[test]
    fn test_derive_master_key_different_passphrase() {
        let salt = [2u8; 16];
        let key1 = KeyManager::derive_master_key("passphrase A", &salt).unwrap();
        let key2 = KeyManager::derive_master_key("passphrase B", &salt).unwrap();
        assert_ne!(key1, key2);
    }

    #[test]
    fn test_derive_file_key() {
        let master_key = [3u8; 32];
        let fk1 = KeyManager::derive_file_key(&master_key, "file-001").unwrap();
        let fk2 = KeyManager::derive_file_key(&master_key, "file-001").unwrap();
        assert_eq!(fk1, fk2);

        let fk3 = KeyManager::derive_file_key(&master_key, "file-002").unwrap();
        assert_ne!(fk1, fk3);
    }

    #[test]
    fn test_generate_salt() {
        let salt = KeyManager::generate_salt();
        assert_eq!(salt.len(), 16);
        // Salt should be random, check it's not all zeros
        assert_ne!(salt, [0u8; 16]);
    }

    #[test]
    fn test_canary_creation_and_verification() {
        let salt = KeyManager::generate_salt();
        let key = KeyManager::derive_master_key("test passphrase", &salt).unwrap();
        let canary = KeyManager::create_canary(&key).unwrap();
        assert!(KeyManager::verify_passphrase(&key, &canary.ciphertext, &canary.nonce));

        // Wrong key should fail
        let wrong_key = [0u8; 32];
        assert!(!KeyManager::verify_passphrase(
            &wrong_key,
            &canary.ciphertext,
            &canary.nonce
        ));
    }

    #[test]
    fn test_from_passphrase() {
        let salt = [4u8; 16];
        let km = KeyManager::from_passphrase("my pass", &salt).unwrap();
        let expected = KeyManager::derive_master_key("my pass", &salt).unwrap();
        assert_eq!(km.master_key(), &expected);
    }

    #[test]
    fn test_file_key_via_manager() {
        let km = KeyManager::from_master_key([5u8; 32]);
        let fk = km.file_key("file-abc").unwrap();
        let expected = KeyManager::derive_file_key(&[5u8; 32], "file-abc").unwrap();
        assert_eq!(fk, expected);
    }
}
