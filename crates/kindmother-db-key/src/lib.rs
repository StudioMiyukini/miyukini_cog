//! # KindMother DB Key
//!
//! Dérivation de clé AES-256 pour le chiffrement au repos des bases SQLite
//! via SQLCipher. Utilise Argon2id (OWASP 2024) et un secret d'installation
//! stocké localement.
//!
//! [`DatabaseKeyStore`] (K-07) : une clé par base dérivée d'un secret aléatoire
//! **propre à la base** (donc au service), détenu par `kindmother-service` seul.
//!
//! @id     kindmother_db_key
//! @do     derive_encryption_keys_for_sqlite

// C-10 : aucun unwrap / expect / panic! / todo! / unimplemented! / unreachable! hors tests.
#![cfg_attr(
    not(test),
    deny(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::todo,
        clippy::unimplemented,
        clippy::unreachable
    )
)]

//@human ! @layer  core

mod key_derivation;
mod key_store;

pub use key_derivation::{DbKeyError, KeyDerivation};
pub use key_store::{DatabaseKeyStore, KeyScheme, KEYS_DIR};
