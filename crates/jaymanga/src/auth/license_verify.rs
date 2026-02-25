//! Vérification des licences d'achat JayManga.
//!
//! Contrôle l'accès aux pages payantes en vérifiant qu'une licence
//! active existe pour l'acheteur et le contenu demandé.

#[cfg(feature = "legacy-sqlite")]
use crate::data::JayMangaDb;

/// Vérifie si un acheteur possède une licence active pour un contenu.
#[cfg(feature = "legacy-sqlite")]
pub fn license_verify_access(
    db: &JayMangaDb,
    buyer_cog_id: &str,
    content_id: &str,
) -> Result<bool, crate::data::DbError> {
    let license = db.license_by_buyer_and_target(buyer_cog_id, content_id)?;
    Ok(license.is_some())
}

/// Vérifie si un acheteur peut télécharger un contenu.
#[cfg(feature = "legacy-sqlite")]
pub fn license_verify_download(
    db: &JayMangaDb,
    buyer_cog_id: &str,
    content_id: &str,
) -> Result<bool, crate::data::DbError> {
    let license = db.license_by_buyer_and_target(buyer_cog_id, content_id)?;
    match license {
        Some(l) => Ok(l.download_allowed.unwrap_or(false)),
        None => Ok(false),
    }
}
