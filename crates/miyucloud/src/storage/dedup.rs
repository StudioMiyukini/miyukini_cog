use crate::errors::MiyucloudError;
use sha2::{Digest, Sha256};

/// Content hash for content-addressable dedup
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContentHash(pub [u8; 32]);

impl ContentHash {
    pub fn from_data(data: &[u8]) -> Self {
        let hash = Sha256::digest(data);
        let mut arr = [0u8; 32];
        arr.copy_from_slice(&hash);
        Self(arr)
    }

    pub fn to_hex(&self) -> String {
        hex_encode(&self.0)
    }

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
    pub hash: ContentHash,
    pub size: u64,
    pub ref_count: u32,
    pub compressed: bool,
    pub created_at: String,
}

/// Trait for content-addressable storage
pub trait ContentAddressableStorage {
    fn store_blob(&self, data: &[u8]) -> Result<ContentHash, MiyucloudError>;
    fn read_blob(&self, hash: &ContentHash) -> Result<Vec<u8>, MiyucloudError>;
    fn blob_exists(&self, hash: &ContentHash) -> Result<bool, MiyucloudError>;
    fn increment_refcount(&self, hash: &ContentHash) -> Result<(), MiyucloudError>;
    fn decrement_refcount(&self, hash: &ContentHash) -> Result<(), MiyucloudError>;
}

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn hex_decode(hex: &str) -> Result<Vec<u8>, String> {
    if hex.len() % 2 != 0 {
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
