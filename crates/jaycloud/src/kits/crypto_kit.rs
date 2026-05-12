//! `crypto_kit` — wrap chacha20poly1305 pour chiffrement bloc.
//!
//! Implémentation en PR-2 (P3.a). API attendue :
//! - `encrypt(plaintext, key) -> Vec<u8>` (préfixe avec nonce 12 bytes)
//! - `decrypt(ciphertext, key) -> plaintext`
//! - Les clés sont dérivées via `kindmother::derive_key("jaycloud_files_v1")`
