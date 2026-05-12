//! `cas_kit` — Content-Addressed Storage.
//!
//! Implémentation en PR-2 (P3.a). API attendue :
//! - `store(plaintext) -> BlobHash` (hash SHA-256, chiffre, compresse zstd, écrit `~/.miyukini/jaycloud/cas/AB/abcd.../`)
//! - `read(blob_hash) -> plaintext`
//! - `exists(blob_hash) -> bool` (pour dédup)
//! - `delete_orphans(referenced_hashes) -> count`
