//! Package service — format du paquet distribué via le Market.
//!
//! # Format du package
//!
//! Un package service est une archive `.msp` (Miyukini Service Package)
//! qui contient :
//!
//! ```text
//! mon-service-0.1.0.msp
//! ├── service.manifest.json    # Manifeste (ServiceManifest)
//! ├── service.wasm             # Module WASM du service (Phase 2)
//! ├── schema.sql               # Schéma de la base de données
//! └── assets/                  # Assets statiques (icônes, etc.)
//!     ├── icon.png
//!     └── ...
//! ```
//!
//! # Phases d'implémentation
//!
//! - **Phase 1** (actuelle) : Les services officiels sont compilés via Cargo features.
//!   Le Market sert uniquement de catalogue et de manifeste de version.
//!   L'installation requiert une recompilation de Central.
//!
//! - **Phase 2** (future) : Les services tiers sont distribués en WASM/dylib.
//!   Le Market distribue des packages `.msp` que Central charge dynamiquement.

use crate::manifest::{ManifestError, ServiceManifest};
use sha2::{Digest, Sha256};

/// Extension des packages Market.
pub const PACKAGE_EXTENSION: &str = "msp";

/// Nom du fichier manifeste dans un package.
pub const MANIFEST_FILENAME: &str = "service.manifest.json";

/// Métadonnées d'un package stocké sur Origin.
#[derive(Debug, Clone)]
pub struct PackageInfo {
    /// Manifeste du service.
    pub manifest: ServiceManifest,
    /// Hash SHA-256 du fichier package.
    pub checksum: String,
    /// Taille en octets.
    pub size: u64,
    /// Chemin relatif sur le serveur (ex: "packages/jaymanga/0.1.0/jaymanga-0.1.0.msp").
    pub storage_path: String,
}

/// Calcule le hash SHA-256 d'un bloc de données.
pub fn sha256_hex(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    hex_encode(&result)
}

/// Construit le nom de fichier standard d'un package.
///
/// Format : `{service_id}-{version}.msp`
pub fn package_filename(service_id: &str, version: &str) -> String {
    format!("{service_id}-{version}.{PACKAGE_EXTENSION}")
}

/// Construit le chemin de stockage relatif sur Origin.
///
/// Format : `packages/{service_id}/{version}/{service_id}-{version}.msp`
pub fn storage_path(service_id: &str, version: &str) -> String {
    let filename = package_filename(service_id, version);
    format!("packages/{service_id}/{version}/{filename}")
}

/// Valide un package (vérifie le manifeste intégré et le checksum).
pub fn validate_package(
    data: &[u8],
    expected_manifest: &ServiceManifest,
) -> Result<PackageInfo, PackageError> {
    // Phase 1 : on vérifie uniquement le manifeste et le checksum
    expected_manifest
        .validate()
        .map_err(PackageError::Manifest)?;

    let checksum = sha256_hex(data);
    let size = data.len() as u64;
    let path = storage_path(&expected_manifest.id, &expected_manifest.version);

    Ok(PackageInfo {
        manifest: expected_manifest.clone(),
        checksum,
        size,
        storage_path: path,
    })
}

/// Erreurs liées aux packages.
#[derive(Debug, thiserror::Error)]
pub enum PackageError {
    #[error("manifeste invalide : {0}")]
    Manifest(#[from] ManifestError),
    #[error("checksum incorrect : attendu {expected}, obtenu {actual}")]
    ChecksumMismatch { expected: String, actual: String },
    #[error("package trop volumineux : {size} octets (max {max})")]
    TooLarge { size: u64, max: u64 },
    #[error("erreur I/O : {0}")]
    Io(String),
}

/// Encode un slice d'octets en hexadécimal.
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256() {
        let hash = sha256_hex(b"hello world");
        assert_eq!(
            hash,
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
        );
    }

    #[test]
    fn test_package_filename() {
        assert_eq!(package_filename("jaymanga", "0.1.0"), "jaymanga-0.1.0.msp");
    }

    #[test]
    fn test_storage_path() {
        assert_eq!(
            storage_path("jaymanga", "0.1.0"),
            "packages/jaymanga/0.1.0/jaymanga-0.1.0.msp"
        );
    }
}
