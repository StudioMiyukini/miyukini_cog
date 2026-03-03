//! Verification d'integrite SHA-256.
//!
//! @id: miyucloud_storage_integrity
//! @do: compute_and_verify_sha256_checksums
//! @role: integrity
//! @layer: infra
//!
//! Fournit le calcul et la verification de checksums SHA-256 pour les fichiers.
//! Les hashes sont retournes en hexadecimal lowercase.

use crate::errors::MiyucloudError;
use sha2::{Digest, Sha256};

/// Calcule le SHA-256 d'un buffer de donnees.
/// Retourne le hash en hexadecimal lowercase.
pub fn compute_sha256(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    hex_encode(&result)
}

/// Verifie que le SHA-256 des donnees correspond au hash attendu.
/// Retourne `Ok(())` si la verification passe, `MiyucloudError::Integrity` sinon.
pub fn verify_sha256(data: &[u8], expected: &str) -> Result<(), MiyucloudError> {
    let actual = compute_sha256(data);
    if actual == expected {
        Ok(())
    } else {
        Err(MiyucloudError::Integrity(format!(
            "SHA-256 mismatch: expected {expected}, got {actual}"
        )))
    }
}

/// Encode un tableau d'octets en hexadecimal lowercase.
fn hex_encode(bytes: &[u8]) -> String {
    use std::fmt::Write;
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        let _ = write!(s, "{b:02x}");
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256_known_value() {
        // SHA-256 of "hello" is well-known
        let hash = compute_sha256(b"hello");
        assert_eq!(
            hash,
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
    }

    #[test]
    fn test_verify_integrity_pass() {
        let data = b"test data for integrity";
        let hash = compute_sha256(data);
        let result = verify_sha256(data, &hash);
        assert!(result.is_ok());
    }

    #[test]
    fn test_verify_integrity_fail() {
        let data = b"original data";
        let hash = compute_sha256(data);
        let modified = b"modified data";
        let result = verify_sha256(modified, &hash);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("SHA-256 mismatch"));
    }

    #[test]
    fn test_empty_data_hash() {
        let hash = compute_sha256(b"");
        assert_eq!(
            hash,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }
}
