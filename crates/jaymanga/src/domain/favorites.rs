//! Logique de gestion des favoris cross-COG JayManga.
//!
//! Ajout/suppression de favoris, mise à jour du cache de métadonnées,
//! synchronisation lors de la reconnexion aux COGs vendeurs.

use crate::data::*;

// ---------------------------------------------------------------------------
// Erreurs
// ---------------------------------------------------------------------------

/// Erreur du module favoris.
#[derive(Debug, thiserror::Error)]
pub enum FavoritesError {
    #[error("Database error: {0}")]
    Db(#[from] DbError),
    #[error("Favorite not found: {0}")]
    NotFound(String),
    #[error("Already in favorites: {0}")]
    AlreadyExists(String),
    #[error("Sync error: {0}")]
    Sync(String),
}

// ---------------------------------------------------------------------------
// Gestion des favoris
// ---------------------------------------------------------------------------

/// Crée un nouveau favori cross-COG.
#[cfg(feature = "legacy-sqlite")]
pub fn favorite_add(
    db: &JayMangaDb,
    seller_cog_id: &str,
    work_id: &str,
    title: &str,
    cover_url: Option<&str>,
    authors_json: &str,
    reading_format: Option<&str>,
) -> Result<ReaderFavorite, FavoritesError> {
    // Vérifier si déjà en favori
    let existing = db.favorite_list()?;
    let key = format!("{seller_cog_id}:{work_id}");
    if existing.iter().any(|f| {
        f.seller_cog_id.as_deref() == Some(seller_cog_id) && f.work_id.as_deref() == Some(work_id)
    }) {
        return Err(FavoritesError::AlreadyExists(key));
    }

    let now = chrono::Utc::now().to_rfc3339();
    let fav = ReaderFavorite {
        id: Some(uuid::Uuid::new_v4().to_string()),
        seller_cog_id: Some(seller_cog_id.to_string()),
        work_id: Some(work_id.to_string()),
        cached_title: Some(title.to_string()),
        cached_cover_url: cover_url.map(ToString::to_string),
        cached_authors: Some(authors_json.to_string()),
        cached_format: reading_format.map(ToString::to_string),
        purchase_status: Some("demo".to_string()),
        last_read_chapter: None,
        last_read_page: None,
        reading_progress: Some(0.0),
        added_at: Some(now.clone()),
        last_synced_at: Some(now),
    };

    db.favorite_create(&fav)?;
    Ok(fav)
}

/// Supprime un favori par son ID.
#[cfg(feature = "legacy-sqlite")]
pub fn favorite_remove(db: &JayMangaDb, favorite_id: &str) -> Result<(), FavoritesError> {
    db.favorite_delete(favorite_id)?;
    Ok(())
}

/// Met à jour la progression de lecture d'un favori.
#[cfg(feature = "legacy-sqlite")]
pub fn favorite_update_reading_progress(
    db: &JayMangaDb,
    favorite_id: &str,
    chapter: i32,
    page: i32,
    progress: f64,
) -> Result<(), FavoritesError> {
    db.favorite_update_progress(favorite_id, chapter, page, progress)?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Calculs utilitaires
// ---------------------------------------------------------------------------

/// Calcule la progression globale de lecture d'un favori.
#[must_use]
pub fn favorite_compute_progress(
    chapters_read: i32,
    total_chapters: i32,
    current_chapter_progress: f64,
) -> f64 {
    if total_chapters == 0 {
        return 0.0;
    }
    let completed = f64::from(chapters_read);
    let total = f64::from(total_chapters);
    let partial = current_chapter_progress / total;
    ((completed + partial) / total).clamp(0.0, 1.0)
}

/// Détermine le statut d'achat à afficher pour un favori.
#[must_use]
pub fn favorite_display_status(purchase_status: &str, has_local_files: bool) -> &'static str {
    match (purchase_status, has_local_files) {
        ("purchased", true) => "downloaded",
        ("purchased", false) => "purchased",
        ("downloaded", _) => "downloaded",
        _ => "demo",
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_progress_empty() {
        assert!((favorite_compute_progress(0, 0, 0.0) - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_compute_progress_half() {
        // 5 chapitres lus sur 10, à moitié du 6e
        let progress = favorite_compute_progress(5, 10, 0.5);
        assert!(progress > 0.5);
        assert!(progress < 0.6);
    }

    #[test]
    fn test_compute_progress_complete() {
        let progress = favorite_compute_progress(10, 10, 0.0);
        assert!((progress - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_display_status() {
        assert_eq!(favorite_display_status("purchased", true), "downloaded");
        assert_eq!(favorite_display_status("purchased", false), "purchased");
        assert_eq!(favorite_display_status("demo", false), "demo");
        assert_eq!(favorite_display_status("downloaded", false), "downloaded");
    }
}
