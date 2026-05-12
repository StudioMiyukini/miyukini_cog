//! `token_kit` — jetons applicatifs WebDAV signés.
//!
//! Implémentation en PR-2 (P3.a). Format :
//! `mws-jc-<base32(random_16_bytes)>-<hmac>`
//!
//! API attendue :
//! - `generate(user_id, scopes) -> (raw_token, hash)`
//! - `verify(raw_token, expected_hash) -> bool` (comparaison constant-time)
//! - `hash(raw_token) -> hash` (SHA-256, jamais stocker le raw)
