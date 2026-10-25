//! @id     miyucloud_storage_dedup
//! @do     definir_content_hash
//! @role   orchestration
//! @layer  core
//! @human  miyucloud — module « storage dedup ».

use crate::errors::MiyucloudError;
use sha2::{Digest, Sha256};

/// Content hash for content-addressable dedup
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContentHash(pub [u8; 32]);

impl ContentHash {
    /// Calcule l'empreinte SHA-256 de `data`.
    pub fn from_data(data: &[u8]) -> Self {
        let hash = Sha256::digest(data);
        let mut arr = [0u8; 32];
        arr.copy_from_slice(&hash);
        Self(arr)
    }

    /// Représentation hexadécimale minuscule (64 caractères).
    pub fn to_hex(&self) -> String {
        hex_encode(&self.0)
    }

    /// Relit une empreinte depuis sa forme hexadécimale.
    ///
    /// # Errors
    ///
    /// [`MiyucloudError::Internal`] si la chaîne n'est pas de l'hexadécimal valide
    /// ou ne décode pas exactement 32 octets.
    pub fn from_hex(hex: &str) -> Result<Self, MiyucloudError> {
        let bytes = hex_decode(hex).map_err(|e| {
            MiyucloudError::Internal(format!("Invalid content hash hex: {e}"))
        })?;
        if bytes.len() != 32 {
            return Err(MiyucloudError::Internal(
                "Content hash must be 32 bytes".into(),
            ));
        }
        let mut arr = [0u8; 32];
        arr.copy_from_slice(&bytes);
        Ok(Self(arr))
    }
}

impl std::fmt::Display for ContentHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

/// A stored content blob (deduplicated)
#[derive(Debug, Clone)]
pub struct ContentBlob {
    /// Empreinte du contenu (clé d'adressage).
    pub hash: ContentHash,
    /// Taille du contenu en octets.
    pub size: u64,
    /// Nombre de références vers ce blob.
    pub ref_count: u32,
    /// Vrai si le blob est stocké compressé.
    pub compressed: bool,
    /// Date de création (RFC 3339).
    pub created_at: String,
}

/// Trait for content-addressable storage
pub trait ContentAddressableStorage {
    /// Stocke `data` et renvoie son empreinte.
    ///
    /// # Errors
    ///
    /// Erreur d'entrée/sortie ou de base selon l'implémentation.
    fn store_blob(&self, data: &[u8]) -> Result<ContentHash, MiyucloudError>;
    /// Lit le blob d'empreinte `hash`.
    ///
    /// # Errors
    ///
    /// Blob absent ou erreur d'entrée/sortie selon l'implémentation.
    fn read_blob(&self, hash: &ContentHash) -> Result<Vec<u8>, MiyucloudError>;
    /// Indique si le blob `hash` existe.
    ///
    /// # Errors
    ///
    /// Erreur d'accès au stockage selon l'implémentation.
    fn blob_exists(&self, hash: &ContentHash) -> Result<bool, MiyucloudError>;
    /// Incrémente le compteur de références du blob.
    ///
    /// # Errors
    ///
    /// Blob absent ou erreur de stockage selon l'implémentation.
    fn increment_refcount(&self, hash: &ContentHash) -> Result<(), MiyucloudError>;
    /// Décrémente le compteur de références du blob.
    ///
    /// # Errors
    ///
    /// Blob absent ou erreur de stockage selon l'implémentation.
    fn decrement_refcount(&self, hash: &ContentHash) -> Result<(), MiyucloudError>;
}

fn hex_encode(bytes: &[u8]) -> String {
    use std::fmt::Write as _;
    bytes.iter().fold(String::with_capacity(bytes.len() * 2), |mut s, b| {
        // Écrire dans une String ne peut pas échouer.
        let _ = write!(s, "{b:02x}");
        s
    })
}

fn hex_decode(hex: &str) -> Result<Vec<u8>, String> {
    if !hex.len().is_multiple_of(2) {
        return Err("Odd length hex string".into());
    }
    (0..hex.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&hex[i..i + 2], 16)
                .map_err(|e| format!("Invalid hex at {i}: {e}"))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content_hash_deterministic() {
        let data = b"hello world";
        let h1 = ContentHash::from_data(data);
        let h2 = ContentHash::from_data(data);
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_content_hash_different_data() {
        let h1 = ContentHash::from_data(b"hello");
        let h2 = ContentHash::from_data(b"world");
        assert_ne!(h1, h2);
    }

    #[test]
    fn test_content_hash_hex_roundtrip() {
        let h = ContentHash::from_data(b"test data");
        let hex = h.to_hex();
        let h2 = ContentHash::from_hex(&hex).unwrap();
        assert_eq!(h, h2);
    }
}
