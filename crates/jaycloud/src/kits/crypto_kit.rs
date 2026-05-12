//! `crypto_kit` — wrapper ChaCha20-Poly1305 pour chiffrement bloc.
//!
//! Aligné sur le pattern `miyucloud::crypto::at_rest` : AEAD avec nonce
//! aléatoire 12 bytes préfixé au ciphertext.
//!
//! Les clés sont obtenues via :
//! 1. `derive_key_hkdf(master, context)` pour dériver une clé sous-domaine
//!    à partir d'une master key (master stockée par KindMother).
//! 2. `generate_random_key()` pour usages éphémères ou tests.
//!
//! Conforme DT-05 de la Spec (chiffrement au repos systématique).

use chacha20poly1305::{
    aead::{Aead, KeyInit, OsRng},
    AeadCore, ChaCha20Poly1305, Key, Nonce,
};
use hkdf::Hkdf;
use rand::RngCore;
use sha2::Sha256;

/// Taille d'une clé de chiffrement (256 bits).
pub const KEY_LEN: usize = 32;
/// Taille d'un nonce ChaCha20-Poly1305 (96 bits).
pub const NONCE_LEN: usize = 12;
/// Taille de l'overhead AEAD (tag d'authentification 128 bits).
pub const TAG_LEN: usize = 16;

/// Erreurs du Kit crypto.
#[derive(Debug, thiserror::Error)]
pub enum CryptoKitError {
    /// Échec de chiffrement (clé invalide, problème AEAD).
    #[error("chiffrement : {0}")]
    Encrypt(String),
    /// Échec de déchiffrement (clé invalide ou ciphertext corrompu / non authentifié).
    #[error("déchiffrement : {0} (clé invalide ou ciphertext corrompu)")]
    Decrypt(String),
    /// Ciphertext trop court pour contenir nonce + tag.
    #[error("ciphertext trop court ({0} octets) — minimum requis : {1}")]
    TooShort(usize, usize),
    /// Échec de dérivation HKDF.
    #[error("dérivation HKDF : {0}")]
    Hkdf(String),
}

/// Clé symétrique 256 bits utilisée par ChaCha20-Poly1305.
#[derive(Clone)]
pub struct Key32(pub [u8; KEY_LEN]);

impl Key32 {
    /// Génère une clé aléatoire (OS RNG).
    #[must_use]
    pub fn generate() -> Self {
        let mut bytes = [0u8; KEY_LEN];
        rand::rngs::OsRng.fill_bytes(&mut bytes);
        Self(bytes)
    }

    /// Construit depuis des octets bruts.
    #[must_use]
    pub fn from_bytes(bytes: [u8; KEY_LEN]) -> Self {
        Self(bytes)
    }

    /// Renvoie la référence en `chacha20poly1305::Key`.
    fn as_chacha_key(&self) -> &Key {
        Key::from_slice(&self.0)
    }
}

impl std::fmt::Debug for Key32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Ne révèle jamais la clé dans les logs.
        f.write_str("Key32(***)")
    }
}

/// Chiffre `plaintext` avec `key`. Le résultat est `nonce || ciphertext || tag`.
///
/// Le nonce de 12 octets est généré aléatoirement (compatible OS RNG).
/// L'overhead total est `NONCE_LEN + TAG_LEN = 28` octets.
pub fn encrypt(plaintext: &[u8], key: &Key32) -> Result<Vec<u8>, CryptoKitError> {
    let cipher = ChaCha20Poly1305::new(key.as_chacha_key());
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
    let ciphertext = cipher
        .encrypt(&nonce, plaintext)
        .map_err(|e| CryptoKitError::Encrypt(e.to_string()))?;

    let mut out = Vec::with_capacity(NONCE_LEN + ciphertext.len());
    out.extend_from_slice(nonce.as_slice());
    out.extend_from_slice(&ciphertext);
    Ok(out)
}

/// Déchiffre `input` (format `nonce || ciphertext || tag`).
pub fn decrypt(input: &[u8], key: &Key32) -> Result<Vec<u8>, CryptoKitError> {
    if input.len() < NONCE_LEN + TAG_LEN {
        return Err(CryptoKitError::TooShort(input.len(), NONCE_LEN + TAG_LEN));
    }
    let (nonce_bytes, ciphertext) = input.split_at(NONCE_LEN);
    let nonce = Nonce::from_slice(nonce_bytes);
    let cipher = ChaCha20Poly1305::new(key.as_chacha_key());
    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| CryptoKitError::Decrypt(e.to_string()))
}

/// Dérive une clé enfant à partir d'une master key via HKDF-SHA256.
///
/// `context` est typiquement un domain string comme `"jaycloud_files_v1"`
/// ou `"jaycloud_storage_v1"`. Deux contextes différents → clés indépendantes.
pub fn derive_key_hkdf(master: &Key32, context: &[u8]) -> Result<Key32, CryptoKitError> {
    let hk = Hkdf::<Sha256>::new(None, &master.0);
    let mut child = [0u8; KEY_LEN];
    hk.expand(context, &mut child)
        .map_err(|e| CryptoKitError::Hkdf(e.to_string()))?;
    Ok(Key32(child))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_basic() {
        let key = Key32::generate();
        let plaintext = b"Hello JayCloud!";
        let ciphertext = encrypt(plaintext, &key).unwrap();
        assert_ne!(&ciphertext[..], plaintext);
        let decrypted = decrypt(&ciphertext, &key).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn ciphertext_is_unique_each_call() {
        // Nonce aléatoire → 2 chiffrements du même plaintext donnent 2 ciphertexts différents.
        let key = Key32::generate();
        let plaintext = b"same input";
        let c1 = encrypt(plaintext, &key).unwrap();
        let c2 = encrypt(plaintext, &key).unwrap();
        assert_ne!(c1, c2);
    }

    #[test]
    fn wrong_key_fails() {
        let k1 = Key32::generate();
        let k2 = Key32::generate();
        let ciphertext = encrypt(b"secret", &k1).unwrap();
        let result = decrypt(&ciphertext, &k2);
        assert!(matches!(result, Err(CryptoKitError::Decrypt(_))));
    }

    #[test]
    fn truncated_ciphertext_fails() {
        let key = Key32::generate();
        let ciphertext = encrypt(b"hi", &key).unwrap();
        let truncated = &ciphertext[..5]; // moins que NONCE+TAG
        let result = decrypt(truncated, &key);
        assert!(matches!(result, Err(CryptoKitError::TooShort(_, _))));
    }

    #[test]
    fn tampered_ciphertext_fails() {
        let key = Key32::generate();
        let mut ciphertext = encrypt(b"data", &key).unwrap();
        let last = ciphertext.len() - 1;
        ciphertext[last] ^= 0x01; // flip un bit du tag
        let result = decrypt(&ciphertext, &key);
        assert!(matches!(result, Err(CryptoKitError::Decrypt(_))));
    }

    #[test]
    fn hkdf_different_contexts_give_different_keys() {
        let master = Key32::generate();
        let k1 = derive_key_hkdf(&master, b"jaycloud_files_v1").unwrap();
        let k2 = derive_key_hkdf(&master, b"jaycloud_storage_v1").unwrap();
        assert_ne!(k1.0, k2.0);
    }

    #[test]
    fn hkdf_is_deterministic() {
        let master = Key32::from_bytes([42u8; KEY_LEN]);
        let k1 = derive_key_hkdf(&master, b"jaycloud_files_v1").unwrap();
        let k2 = derive_key_hkdf(&master, b"jaycloud_files_v1").unwrap();
        assert_eq!(k1.0, k2.0);
    }

    #[test]
    fn hkdf_different_masters_give_different_keys() {
        let m1 = Key32::generate();
        let m2 = Key32::generate();
        let k1 = derive_key_hkdf(&m1, b"context").unwrap();
        let k2 = derive_key_hkdf(&m2, b"context").unwrap();
        assert_ne!(k1.0, k2.0);
    }

    #[test]
    fn debug_does_not_leak_key() {
        let key = Key32::from_bytes([0xff; KEY_LEN]);
        let debug_str = format!("{key:?}");
        assert_eq!(debug_str, "Key32(***)");
        assert!(!debug_str.contains("ff"));
    }
}
