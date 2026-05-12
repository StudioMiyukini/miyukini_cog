//! `cas_kit` — Content-Addressed Storage.
//!
//! Stocke des blobs sur disque, **adressés par hash SHA-256 du plaintext**.
//! Deux blobs avec le même contenu ont le même `BlobHash` et le même chemin
//! → dédup naturelle entre snapshots.
//!
//! Pipeline :
//! 1. Hash SHA-256 du plaintext → `BlobHash`
//! 2. Compression zstd (optionnelle, niveau configurable)
//! 3. Chiffrement ChaCha20-Poly1305 via `crypto_kit`
//! 4. Écriture dans `root/ab/abcd1234.../`
//!
//! Lecture : symétrique (déchiffre → décompresse → vérifie hash → renvoie).
//!
//! Conforme DT-08 de la Spec MSCM/MIP.

use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

use crate::kits::crypto_kit::{self, CryptoKitError, Key32};

/// Hash SHA-256 d'un blob (avant chiffrement).
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct BlobHash(pub [u8; 32]);

impl BlobHash {
    /// Calcule le hash d'un plaintext.
    #[must_use]
    pub fn from_plaintext(plaintext: &[u8]) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(plaintext);
        let result = hasher.finalize();
        let mut out = [0u8; 32];
        out.copy_from_slice(&result);
        Self(out)
    }

    /// Représentation hex minuscule (64 caractères).
    #[must_use]
    pub fn to_hex(&self) -> String {
        hex::encode(self.0)
    }

    /// Parse depuis une chaîne hex.
    pub fn from_hex(s: &str) -> Result<Self, CasKitError> {
        let bytes = hex::decode(s).map_err(|e| CasKitError::InvalidHash(e.to_string()))?;
        if bytes.len() != 32 {
            return Err(CasKitError::InvalidHash(format!(
                "longueur incorrecte : {} (attendu 32)",
                bytes.len()
            )));
        }
        let mut out = [0u8; 32];
        out.copy_from_slice(&bytes);
        Ok(Self(out))
    }

    /// Préfixe 2 caractères hex utilisé pour le sharding (`ab/abcd.../`).
    #[must_use]
    pub fn shard_prefix(&self) -> String {
        format!("{:02x}", self.0[0])
    }
}

impl std::fmt::Display for BlobHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_hex())
    }
}

/// Erreurs du Kit CAS.
#[derive(Debug, thiserror::Error)]
pub enum CasKitError {
    /// Erreur d'I/O sur le filesystem CAS.
    #[error("io CAS : {0}")]
    Io(#[from] std::io::Error),
    /// Erreur du `crypto_kit` (chiffrement / déchiffrement).
    #[error("crypto : {0}")]
    Crypto(#[from] CryptoKitError),
    /// Erreur de compression / décompression zstd.
    #[error("compression : {0}")]
    Compression(String),
    /// Hash invalide (format ou longueur).
    #[error("hash invalide : {0}")]
    InvalidHash(String),
    /// Le hash recalculé après lecture ne correspond pas → corruption détectée.
    #[error("intégrité : hash attendu {expected}, obtenu {actual}")]
    IntegrityMismatch {
        /// Hash attendu (préfixe du blob).
        expected: String,
        /// Hash recalculé du plaintext.
        actual: String,
    },
}

/// Configuration runtime du CAS.
#[derive(Debug, Clone)]
pub struct CasConfig {
    /// Niveau zstd : 0 = pas de compression, sinon 1-22.
    pub compression_level: i32,
}

impl Default for CasConfig {
    fn default() -> Self {
        Self {
            compression_level: 3,
        }
    }
}

/// Stocke `plaintext` dans le CAS sous `root_dir`. Si le blob existe déjà
/// (même hash), l'écriture est un no-op (dédup).
///
/// Renvoie le `BlobHash` du contenu.
pub fn store(
    plaintext: &[u8],
    key: &Key32,
    root_dir: &Path,
    config: &CasConfig,
) -> Result<BlobHash, CasKitError> {
    let hash = BlobHash::from_plaintext(plaintext);
    let path = blob_path(root_dir, &hash);

    if path.exists() {
        // Dédup : blob déjà présent, rien à faire.
        return Ok(hash);
    }

    let compressed = if config.compression_level == 0 {
        plaintext.to_vec()
    } else {
        zstd::stream::encode_all(plaintext, config.compression_level)
            .map_err(|e| CasKitError::Compression(e.to_string()))?
    };

    let ciphertext = crypto_kit::encrypt(&compressed, key)?;

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    // Écriture atomique : tmp file + rename.
    let tmp_path = path.with_extension("tmp");
    std::fs::write(&tmp_path, &ciphertext)?;
    std::fs::rename(&tmp_path, &path)?;

    Ok(hash)
}

/// Lit un blob depuis le CAS et **vérifie son intégrité** (SHA-256 du
/// plaintext déchiffré doit matcher le hash demandé).
pub fn read(hash: &BlobHash, key: &Key32, root_dir: &Path) -> Result<Vec<u8>, CasKitError> {
    let path = blob_path(root_dir, hash);
    let ciphertext = std::fs::read(&path)?;
    let compressed = crypto_kit::decrypt(&ciphertext, key)?;

    // Tente de décompresser ; si ce n'est pas du zstd, fallback raw.
    let plaintext = match zstd::stream::decode_all(compressed.as_slice()) {
        Ok(decoded) => decoded,
        Err(_) => compressed, // niveau 0 : pas de compression à l'écriture.
    };

    // Vérification d'intégrité systématique.
    let actual = BlobHash::from_plaintext(&plaintext);
    if &actual != hash {
        return Err(CasKitError::IntegrityMismatch {
            expected: hash.to_hex(),
            actual: actual.to_hex(),
        });
    }
    Ok(plaintext)
}

/// Indique si un blob existe dans le CAS (sans lecture).
pub fn exists(hash: &BlobHash, root_dir: &Path) -> bool {
    blob_path(root_dir, hash).exists()
}

/// Garbage-collect les blobs orphelins (non référencés par `referenced`).
///
/// Renvoie le nombre de blobs supprimés.
pub fn delete_orphans(
    referenced: &std::collections::HashSet<BlobHash>,
    root_dir: &Path,
) -> Result<usize, CasKitError> {
    let mut deleted = 0;
    if !root_dir.exists() {
        return Ok(0);
    }
    for shard in std::fs::read_dir(root_dir)? {
        let shard = shard?;
        if !shard.file_type()?.is_dir() {
            continue;
        }
        for blob in std::fs::read_dir(shard.path())? {
            let blob = blob?;
            if !blob.file_type()?.is_file() {
                continue;
            }
            let filename = blob.file_name().to_string_lossy().to_string();
            // Skip les .tmp d'écritures interrompues si jamais ils restent.
            if filename.ends_with(".tmp") {
                continue;
            }
            if let Ok(hash) = BlobHash::from_hex(&filename) {
                if !referenced.contains(&hash) {
                    std::fs::remove_file(blob.path())?;
                    deleted += 1;
                }
            }
        }
    }
    Ok(deleted)
}

/// Calcule le chemin disque d'un blob : `root/<2 hex>/<full hex>`.
fn blob_path(root_dir: &Path, hash: &BlobHash) -> PathBuf {
    root_dir.join(hash.shard_prefix()).join(hash.to_hex())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn temp_root() -> tempfile::TempDir {
        tempfile::tempdir().unwrap()
    }

    #[test]
    fn hash_is_deterministic() {
        let h1 = BlobHash::from_plaintext(b"hello");
        let h2 = BlobHash::from_plaintext(b"hello");
        assert_eq!(h1, h2);
    }

    #[test]
    fn hash_changes_with_content() {
        let h1 = BlobHash::from_plaintext(b"hello");
        let h2 = BlobHash::from_plaintext(b"world");
        assert_ne!(h1, h2);
    }

    #[test]
    fn hex_roundtrip() {
        let h = BlobHash::from_plaintext(b"jaycloud");
        let parsed = BlobHash::from_hex(&h.to_hex()).unwrap();
        assert_eq!(h, parsed);
    }

    #[test]
    fn invalid_hex_fails() {
        let r = BlobHash::from_hex("zzzz");
        assert!(matches!(r, Err(CasKitError::InvalidHash(_))));
    }

    #[test]
    fn shard_prefix_uses_first_byte() {
        let mut bytes = [0u8; 32];
        bytes[0] = 0xab;
        bytes[1] = 0xcd;
        let h = BlobHash(bytes);
        assert_eq!(h.shard_prefix(), "ab");
    }

    #[test]
    fn store_and_read_roundtrip() {
        let dir = temp_root();
        let key = Key32::generate();
        let cfg = CasConfig::default();
        let payload = b"Hello CAS!";

        let hash = store(payload, &key, dir.path(), &cfg).unwrap();
        assert!(exists(&hash, dir.path()));

        let read_back = read(&hash, &key, dir.path()).unwrap();
        assert_eq!(read_back, payload);
    }

    #[test]
    fn store_dedups_same_content() {
        let dir = temp_root();
        let key = Key32::generate();
        let cfg = CasConfig::default();

        let h1 = store(b"dup", &key, dir.path(), &cfg).unwrap();
        let h2 = store(b"dup", &key, dir.path(), &cfg).unwrap();
        assert_eq!(h1, h2);

        // Un seul fichier sur disque.
        let shard_dir = dir.path().join(h1.shard_prefix());
        let count = std::fs::read_dir(&shard_dir).unwrap().count();
        assert_eq!(count, 1);
    }

    #[test]
    fn read_with_wrong_key_fails() {
        let dir = temp_root();
        let k1 = Key32::generate();
        let k2 = Key32::generate();
        let cfg = CasConfig::default();

        let hash = store(b"secret", &k1, dir.path(), &cfg).unwrap();
        let r = read(&hash, &k2, dir.path());
        assert!(matches!(r, Err(CasKitError::Crypto(_))));
    }

    #[test]
    fn corruption_detected() {
        let dir = temp_root();
        let key = Key32::generate();
        let cfg = CasConfig::default();

        let hash = store(b"important data", &key, dir.path(), &cfg).unwrap();
        // Corrompt le fichier sur disque.
        let path = dir.path().join(hash.shard_prefix()).join(hash.to_hex());
        let mut content = std::fs::read(&path).unwrap();
        let last = content.len() - 1;
        content[last] ^= 0xff;
        std::fs::write(&path, &content).unwrap();

        // Le déchiffrement AEAD doit échouer en premier (tag invalide).
        let r = read(&hash, &key, dir.path());
        assert!(r.is_err());
    }

    #[test]
    fn no_compression_mode() {
        let dir = temp_root();
        let key = Key32::generate();
        let cfg = CasConfig {
            compression_level: 0,
        };
        let payload = b"raw mode";
        let hash = store(payload, &key, dir.path(), &cfg).unwrap();
        let read_back = read(&hash, &key, dir.path()).unwrap();
        assert_eq!(read_back, payload);
    }

    #[test]
    fn compression_actually_reduces_size_for_compressible_data() {
        let dir = temp_root();
        let key = Key32::generate();
        let cfg_off = CasConfig {
            compression_level: 0,
        };
        let cfg_on = CasConfig {
            compression_level: 19,
        };
        let payload = vec![b'a'; 10_000]; // hautement compressible

        let h_off = store(&payload, &key, dir.path(), &cfg_off).unwrap();
        let dir2 = temp_root();
        let h_on = store(&payload, &key, dir2.path(), &cfg_on).unwrap();

        // Même hash (calculé sur plaintext) malgré compression différente.
        assert_eq!(h_off, h_on);

        let size_off = std::fs::metadata(dir.path().join(h_off.shard_prefix()).join(h_off.to_hex()))
            .unwrap()
            .len();
        let size_on = std::fs::metadata(dir2.path().join(h_on.shard_prefix()).join(h_on.to_hex()))
            .unwrap()
            .len();
        assert!(
            size_on < size_off,
            "compression devrait réduire la taille : {size_on} vs {size_off}"
        );
    }

    #[test]
    fn delete_orphans_keeps_referenced() {
        let dir = temp_root();
        let key = Key32::generate();
        let cfg = CasConfig::default();

        let h_kept = store(b"keep me", &key, dir.path(), &cfg).unwrap();
        let h_drop = store(b"drop me", &key, dir.path(), &cfg).unwrap();

        let mut referenced = HashSet::new();
        referenced.insert(h_kept.clone());

        let deleted = delete_orphans(&referenced, dir.path()).unwrap();
        assert_eq!(deleted, 1);
        assert!(exists(&h_kept, dir.path()));
        assert!(!exists(&h_drop, dir.path()));
    }

    #[test]
    fn delete_orphans_no_op_on_empty_root() {
        let dir = temp_root();
        let referenced = HashSet::new();
        let deleted = delete_orphans(&referenced, dir.path()).unwrap();
        assert_eq!(deleted, 0);
    }
}
