//! Chiffrement bloc spécifique au filesystem (wrap autour de `kits::crypto_kit`).
//!
//! Implémentation en PR-2 (P3.a). API attendue :
//! - `encrypt_block(plaintext, file_id, block_index) -> Vec<u8>`
//!   (clé dérivée per-fichier, nonce dérivé de `block_index` pour seek)
//! - `decrypt_block(ciphertext, file_id, block_index) -> plaintext`
