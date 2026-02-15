//! # KindMother DB Key
//!
//! Dérivation de clé AES-256 pour le chiffrement au repos des bases SQLite
//! via SQLCipher. Utilise Argon2id (OWASP 2024) et un secret d'installation
//! stocké localement.
//!
//! @id: kindmother_db_key
//! @do: derive_encryption_keys_for_sqlite
//! @layer: core

mod key_derivation;

pub use key_derivation::{DbKeyError, KeyDerivation};
