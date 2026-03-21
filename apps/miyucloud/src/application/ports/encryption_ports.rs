//! Blob encryption port — hexagonal boundary for encryption at rest.
//!
//! ISO 27001 A.8.24, RGPD Art.32, HDS mandatory.

use crate::common::errors::DomainError;
use bytes::Bytes;
use std::path::Path;
use std::pin::Pin;
use futures::Stream;

/// Port for encrypting/decrypting file blobs at rest.
///
/// Design: encryption happens AFTER dedup hashing (hash plaintext for dedup,
/// then encrypt before writing to disk). This preserves deduplication.
pub trait BlobEncryptionPort: Send + Sync + 'static {
    /// Encrypt a plaintext file in-place (read → encrypt → write back).
    fn encrypt_file(
        &self,
        path: &Path,
    ) -> impl std::future::Future<Output = Result<(), DomainError>> + Send;

    /// Return a decrypting stream over an encrypted file.
    fn decrypt_stream(
        &self,
        path: &Path,
    ) -> impl std::future::Future<
        Output = Result<Pin<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>>, DomainError>,
    > + Send;

    /// Decrypt an encrypted file fully into memory.
    fn decrypt_bytes(
        &self,
        path: &Path,
    ) -> impl std::future::Future<Output = Result<Bytes, DomainError>> + Send;

    /// Whether encryption is enabled.
    fn is_enabled(&self) -> bool;
}
