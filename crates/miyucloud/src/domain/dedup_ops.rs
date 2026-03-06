use crate::errors::MiyucloudError;
use crate::storage::dedup::{ContentAddressableStorage, ContentHash};

/// Dedup-aware file upload: hash content, check existence, store only if new
pub fn dedup_upload<S: ContentAddressableStorage>(
    storage: &S,
    data: &[u8],
) -> Result<ContentHash, MiyucloudError> {
    let hash = ContentHash::from_data(data);

    if storage.blob_exists(&hash)? {
        storage.increment_refcount(&hash)?;
    } else {
        storage.store_blob(data)?;
    }

    Ok(hash)
}

/// Dedup-aware file delete: decrement refcount, remove blob if zero
pub fn dedup_delete<S: ContentAddressableStorage>(
    storage: &S,
    hash: &ContentHash,
) -> Result<(), MiyucloudError> {
    storage.decrement_refcount(hash)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockStorage {
        stored: std::cell::RefCell<std::collections::HashMap<String, (Vec<u8>, u32)>>,
    }

    impl MockStorage {
        fn new() -> Self {
            Self {
                stored: std::cell::RefCell::new(std::collections::HashMap::new()),
            }
        }
    }

    impl ContentAddressableStorage for MockStorage {
        fn store_blob(&self, data: &[u8]) -> Result<ContentHash, MiyucloudError> {
            let hash = ContentHash::from_data(data);
            self.stored
                .borrow_mut()
                .insert(hash.to_hex(), (data.to_vec(), 1));
            Ok(hash)
        }

        fn read_blob(&self, hash: &ContentHash) -> Result<Vec<u8>, MiyucloudError> {
            self.stored
                .borrow()
                .get(&hash.to_hex())
                .map(|(data, _)| data.clone())
                .ok_or_else(|| MiyucloudError::NotFound("blob not found".into()))
        }

        fn blob_exists(&self, hash: &ContentHash) -> Result<bool, MiyucloudError> {
            Ok(self.stored.borrow().contains_key(&hash.to_hex()))
        }

        fn increment_refcount(&self, hash: &ContentHash) -> Result<(), MiyucloudError> {
            if let Some(entry) = self.stored.borrow_mut().get_mut(&hash.to_hex()) {
                entry.1 += 1;
            }
            Ok(())
        }

        fn decrement_refcount(&self, hash: &ContentHash) -> Result<(), MiyucloudError> {
            let mut stored = self.stored.borrow_mut();
            if let Some(entry) = stored.get_mut(&hash.to_hex()) {
                entry.1 = entry.1.saturating_sub(1);
                if entry.1 == 0 {
                    stored.remove(&hash.to_hex());
                }
            }
            Ok(())
        }
    }

    #[test]
    fn test_dedup_upload_new_file() {
        let storage = MockStorage::new();
        let data = b"unique file content";
        let hash = dedup_upload(&storage, data).unwrap();
        assert!(storage.blob_exists(&hash).unwrap());
    }

    #[test]
    fn test_dedup_upload_duplicate_increments_refcount() {
        let storage = MockStorage::new();
        let data = b"shared content";
        let hash1 = dedup_upload(&storage, data).unwrap();
        let hash2 = dedup_upload(&storage, data).unwrap();
        assert_eq!(hash1, hash2);
        // Refcount should be 2
        assert_eq!(storage.stored.borrow()[&hash1.to_hex()].1, 2);
    }

    #[test]
    fn test_dedup_delete_removes_when_refcount_zero() {
        let storage = MockStorage::new();
        let data = b"deletable content";
        let hash = dedup_upload(&storage, data).unwrap();
        assert!(storage.blob_exists(&hash).unwrap());
        dedup_delete(&storage, &hash).unwrap();
        assert!(!storage.blob_exists(&hash).unwrap());
    }
}
