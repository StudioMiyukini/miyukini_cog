//! Module de chiffrement MiyuCloud.
//!
//! @id: miyucloud_crypto_module
//! @do: expose_crypto_key_management_and_encryption
//! @role: crypto
//! @layer: infra
//!
//! Fournit la gestion des cles (derivation Argon2id, HKDF) et le chiffrement
//! at-rest (ChaCha20-Poly1305).

pub mod at_rest;
pub mod e2e;
pub mod keys;
pub mod streaming;

pub use at_rest::{decrypt_chunk, encrypt_chunk, EncryptedChunk};
pub use keys::KeyManager;
pub use streaming::StreamingDecryptor;
