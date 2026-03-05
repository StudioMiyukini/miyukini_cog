//! Logique de téléchargement hors-ligne JayManga.
//!
//! Vérification des droits de téléchargement, intégrité SHA-256,
//! gestion du stockage local des oeuvres achetées.

use std::path::{Path, PathBuf};

// ---------------------------------------------------------------------------
// Erreurs
// ---------------------------------------------------------------------------

/// Erreur du module téléchargement.
#[derive(Debug, thiserror::Error)]
pub enum DownloadError {
    #[error("Download not allowed for this work")]
    NotAllowed,
    #[error("License required to download")]
    LicenseRequired,
    #[error("Seller COG is offline")]
    SellerOffline,
    #[error("Integrity check failed for {file}: expected {expected}, got {actual}")]
    IntegrityFailed {
        file: String,
        expected: String,
        actual: String,
    },
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Storage error: {0}")]
    Storage(String),
}

// ---------------------------------------------------------------------------
// Vérification des droits
// ---------------------------------------------------------------------------

/// Vérifie si un téléchargement est autorisé.
#[must_use]
pub fn download_check_eligibility(
    allow_download: bool,
    has_license: bool,
    seller_online: bool,
) -> Result<(), DownloadError> {
    if !allow_download {
        return Err(DownloadError::NotAllowed);
    }
    if !has_license {
        return Err(DownloadError::LicenseRequired);
    }
    if !seller_online {
        return Err(DownloadError::SellerOffline);
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Intégrité
// ---------------------------------------------------------------------------

/// Calcule le hash SHA-256 d'un fichier.
pub fn download_compute_sha256(path: &Path) -> Result<String, DownloadError> {
    use sha2::{Digest, Sha256};
    let data = std::fs::read(path)?;
    let hash = Sha256::digest(&data);
    Ok(format!("{hash:x}"))
}

/// Vérifie l'intégrité d'un fichier téléchargé.
pub fn download_verify_integrity(path: &Path, expected_hash: &str) -> Result<(), DownloadError> {
    let actual = download_compute_sha256(path)?;
    if actual != expected_hash {
        return Err(DownloadError::IntegrityFailed {
            file: path.to_string_lossy().to_string(),
            expected: expected_hash.to_string(),
            actual,
        });
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Chemins de stockage local
// ---------------------------------------------------------------------------

/// Construit le chemin de stockage local pour une oeuvre téléchargée.
#[must_use]
pub fn download_local_path(base_dir: &Path, seller_cog_id: &str, work_id: &str) -> PathBuf {
    base_dir.join("downloads").join(seller_cog_id).join(work_id)
}

/// Construit le chemin vers un chapitre dans le stockage local.
#[must_use]
pub fn download_chapter_path(
    base_dir: &Path,
    seller_cog_id: &str,
    work_id: &str,
    chapter_id: &str,
) -> PathBuf {
    download_local_path(base_dir, seller_cog_id, work_id)
        .join("chapters")
        .join(chapter_id)
}

/// Vérifie si une oeuvre est disponible localement.
#[must_use]
pub fn download_is_available_locally(base_dir: &Path, seller_cog_id: &str, work_id: &str) -> bool {
    let path = download_local_path(base_dir, seller_cog_id, work_id);
    path.join("metadata.json").exists()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_eligibility_ok() {
        assert!(download_check_eligibility(true, true, true).is_ok());
    }

    #[test]
    fn test_check_eligibility_not_allowed() {
        let err = download_check_eligibility(false, true, true);
        assert!(matches!(err, Err(DownloadError::NotAllowed)));
    }

    #[test]
    fn test_check_eligibility_no_license() {
        let err = download_check_eligibility(true, false, true);
        assert!(matches!(err, Err(DownloadError::LicenseRequired)));
    }

    #[test]
    fn test_check_eligibility_offline() {
        let err = download_check_eligibility(true, true, false);
        assert!(matches!(err, Err(DownloadError::SellerOffline)));
    }

    #[test]
    fn test_local_path() {
        let base = Path::new("/jaymanga");
        let path = download_local_path(base, "cog-abc", "work-123");
        assert_eq!(path, PathBuf::from("/jaymanga/downloads/cog-abc/work-123"));
    }

    #[test]
    fn test_chapter_path() {
        let base = Path::new("/jaymanga");
        let path = download_chapter_path(base, "cog-abc", "work-123", "ch-1");
        assert_eq!(
            path,
            PathBuf::from("/jaymanga/downloads/cog-abc/work-123/chapters/ch-1")
        );
    }

    #[test]
    fn test_not_available_locally() {
        assert!(!download_is_available_locally(
            Path::new("/nonexistent"),
            "cog",
            "work"
        ));
    }
}
