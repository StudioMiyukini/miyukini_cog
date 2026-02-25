//! Logique de gestion du catalogue vendeur JayManga.
//!
//! Import d'oeuvres (fichiers, dossiers, archives), gestion des séries,
//! publication, archivage, validation des pages de démonstration (RM-07).

use std::path::Path;

use crate::data::*;

// ---------------------------------------------------------------------------
// Erreurs catalogue
// ---------------------------------------------------------------------------

/// Erreur du module catalogue.
#[derive(Debug, thiserror::Error)]
pub enum CatalogError {
    #[error("Database error: {0}")]
    Db(#[from] DbError),
    #[error("Invalid demo pages: {0}")]
    InvalidDemoPages(String),
    #[error("Import error: {0}")]
    Import(String),
    #[error("Work not found: {0}")]
    NotFound(String),
    #[error("Invalid status transition: {from} -> {to}")]
    InvalidTransition { from: String, to: String },
}

/// Résultat d'un import.
#[derive(Debug, Clone, Default)]
pub struct ImportResult {
    pub work_id: String,
    pub chapters_created: i32,
    pub pages_created: i32,
    pub errors: Vec<String>,
}

// ---------------------------------------------------------------------------
// Validation
// ---------------------------------------------------------------------------

/// Valide le nombre de pages de démonstration (RM-07).
/// Minimum 1 page, maximum 50% du total.
pub fn catalog_validate_demo_pages(total_pages: i32, demo_count: i32) -> Result<(), CatalogError> {
    if demo_count < 1 {
        return Err(CatalogError::InvalidDemoPages(
            "Le nombre de pages de démonstration doit être au minimum 1.".to_string(),
        ));
    }
    let max = total_pages / 2;
    if demo_count > max {
        return Err(CatalogError::InvalidDemoPages(format!(
            "Le nombre de pages de démonstration ({demo_count}) ne peut pas dépasser 50% du total ({total_pages} → max {max})."
        )));
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Import
// ---------------------------------------------------------------------------

/// Importe une oeuvre depuis un dossier de fichiers images.
/// Chaque sous-dossier est traité comme un chapitre.
#[cfg(feature = "legacy-sqlite")]
pub fn catalog_import_from_directory(
    db: &JayMangaDb,
    path: &Path,
    work: &Work,
) -> Result<ImportResult, CatalogError> {
    let now = chrono::Utc::now().to_rfc3339();
    let work_id = work
        .id
        .clone()
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

    let mut work = work.clone();
    work.id = Some(work_id.clone());
    work.created_at = Some(now.clone());
    work.updated_at = Some(now.clone());
    work.status = Some(WorkStatus::Draft.as_str().to_string());

    db.work_create(&work)?;

    let mut result = ImportResult {
        work_id: work_id.clone(),
        ..Default::default()
    };

    // Lister les sous-dossiers comme chapitres, ou traiter le dossier racine
    let entries = std::fs::read_dir(path)
        .map_err(|e| CatalogError::Import(format!("Cannot read directory: {e}")))?;

    let mut subdirs: Vec<std::path::PathBuf> = Vec::new();
    let mut root_images: Vec<std::path::PathBuf> = Vec::new();

    for entry in entries.flatten() {
        let p = entry.path();
        if p.is_dir() {
            subdirs.push(p);
        } else if is_image_file(&p) {
            root_images.push(p);
        }
    }

    subdirs.sort();

    if subdirs.is_empty() && !root_images.is_empty() {
        // Dossier plat → un seul chapitre
        let chapter_id = uuid::Uuid::new_v4().to_string();
        let chapter = Chapter {
            id: Some(chapter_id.clone()),
            work_id: Some(work_id.clone()),
            chapter_number: Some(1),
            title: Some("Chapitre 1".to_string()),
            page_count: Some(root_images.len() as i32),
            sort_order: Some(1),
            created_at: Some(now.clone()),
        };
        db.chapter_create(&chapter)?;
        result.chapters_created += 1;

        root_images.sort();
        for (i, img) in root_images.iter().enumerate() {
            let page = create_page_from_path(&chapter_id, i as i32 + 1, img);
            db.page_create(&page)?;
            result.pages_created += 1;
        }
    } else {
        // Sous-dossiers → un chapitre par sous-dossier
        for (idx, dir) in subdirs.iter().enumerate() {
            let chapter_id = uuid::Uuid::new_v4().to_string();
            let ch_name = dir
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Chapitre")
                .to_string();

            let mut images: Vec<std::path::PathBuf> = std::fs::read_dir(dir)
                .map_err(|e| CatalogError::Import(format!("Cannot read subdir: {e}")))?
                .flatten()
                .map(|e| e.path())
                .filter(|p| is_image_file(p))
                .collect();
            images.sort();

            let chapter = Chapter {
                id: Some(chapter_id.clone()),
                work_id: Some(work_id.clone()),
                chapter_number: Some(idx as i32 + 1),
                title: Some(ch_name),
                page_count: Some(images.len() as i32),
                sort_order: Some(idx as i32 + 1),
                created_at: Some(now.clone()),
            };
            db.chapter_create(&chapter)?;
            result.chapters_created += 1;

            for (i, img) in images.iter().enumerate() {
                let page = create_page_from_path(&chapter_id, i as i32 + 1, img);
                db.page_create(&page)?;
                result.pages_created += 1;
            }
        }
    }

    // Mettre à jour total_pages
    let mut updated_work = work;
    updated_work.total_pages = Some(result.pages_created);
    updated_work.updated_at = Some(chrono::Utc::now().to_rfc3339());
    db.work_update(&updated_work)?;

    Ok(result)
}

/// Publie une oeuvre (draft → published).
#[cfg(feature = "legacy-sqlite")]
pub fn catalog_publish(db: &JayMangaDb, work_id: &str) -> Result<(), CatalogError> {
    let work = db
        .work_by_id(work_id)?
        .ok_or_else(|| CatalogError::NotFound(work_id.to_string()))?;

    let current_status = work.status.as_deref().unwrap_or("draft");
    if current_status != "draft" && current_status != "unlisted" {
        return Err(CatalogError::InvalidTransition {
            from: current_status.to_string(),
            to: "published".to_string(),
        });
    }

    let total = work.total_pages.unwrap_or(0);
    let demo = work.demo_pages_count.unwrap_or(0);
    if total > 0 {
        catalog_validate_demo_pages(total, demo)?;
    }

    let mut updated = work;
    updated.status = Some("published".to_string());
    updated.updated_at = Some(chrono::Utc::now().to_rfc3339());
    db.work_update(&updated)?;
    Ok(())
}

/// Archive une oeuvre (any → archived).
#[cfg(feature = "legacy-sqlite")]
pub fn catalog_archive(db: &JayMangaDb, work_id: &str) -> Result<(), CatalogError> {
    let work = db
        .work_by_id(work_id)?
        .ok_or_else(|| CatalogError::NotFound(work_id.to_string()))?;

    let mut updated = work;
    updated.status = Some("archived".to_string());
    updated.updated_at = Some(chrono::Utc::now().to_rfc3339());
    db.work_update(&updated)?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn is_image_file(path: &Path) -> bool {
    match path.extension().and_then(|e| e.to_str()) {
        Some(ext) => matches!(
            ext.to_lowercase().as_str(),
            "jpg" | "jpeg" | "png" | "webp" | "avif"
        ),
        None => false,
    }
}

fn create_page_from_path(chapter_id: &str, page_number: i32, path: &Path) -> Page {
    let file_size = std::fs::metadata(path).map(|m| m.len() as i64).unwrap_or(0);
    Page {
        id: Some(uuid::Uuid::new_v4().to_string()),
        chapter_id: Some(chapter_id.to_string()),
        page_number: Some(page_number),
        original_image_path: Some(path.to_string_lossy().to_string()),
        optimized_variants: None,
        width: None,
        height: None,
        file_size: Some(file_size),
        optimization_status: Some(OptimizationStatus::Pending.as_str().to_string()),
        sort_order: Some(page_number),
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_demo_pages_ok() {
        assert!(catalog_validate_demo_pages(100, 50).is_ok());
        assert!(catalog_validate_demo_pages(100, 1).is_ok());
        assert!(catalog_validate_demo_pages(10, 5).is_ok());
    }

    #[test]
    fn test_validate_demo_pages_too_many() {
        assert!(catalog_validate_demo_pages(100, 51).is_err());
        assert!(catalog_validate_demo_pages(10, 6).is_err());
    }

    #[test]
    fn test_validate_demo_pages_zero() {
        assert!(catalog_validate_demo_pages(100, 0).is_err());
    }

    #[test]
    fn test_is_image_file() {
        assert!(is_image_file(Path::new("page.jpg")));
        assert!(is_image_file(Path::new("page.PNG")));
        assert!(is_image_file(Path::new("page.webp")));
        assert!(!is_image_file(Path::new("readme.txt")));
        assert!(!is_image_file(Path::new("data.json")));
    }
}
