//! Logique de lecture JayManga.
//!
//! Gestion de la progression de lecture, navigation par pages,
//! détection de fin de chapitre/oeuvre, attribution d'XP.

use crate::data::*;
use crate::domain::gamification::*;

// ---------------------------------------------------------------------------
// Erreurs
// ---------------------------------------------------------------------------

/// Erreur du module lecture.
#[derive(Debug, thiserror::Error)]
pub enum ReaderError {
    #[error("Database error: {0}")]
    Db(String),
    #[error("Page not found: {0}")]
    PageNotFound(String),
    #[error("Chapter not found: {0}")]
    ChapterNotFound(String),
    #[error("Work not found: {0}")]
    WorkNotFound(String),
    #[error("Access denied: license required")]
    AccessDenied,
}

// ---------------------------------------------------------------------------
// Événements de lecture
// ---------------------------------------------------------------------------

/// Résultat d'une action de lecture de page.
#[derive(Debug, Clone)]
pub struct ReadPageResult {
    pub page: Page,
    pub xp_events: Vec<XpEvent>,
    pub chapter_completed: bool,
    pub work_completed: bool,
    pub next_page: Option<String>,
    pub prev_page: Option<String>,
}

/// Contexte de navigation pour la liseuse.
#[derive(Debug, Clone)]
pub struct ReaderNavigation {
    pub current_page_index: i32,
    pub total_pages: i32,
    pub chapter_title: Option<String>,
    pub work_title: Option<String>,
    pub is_demo: bool,
    pub progress_percent: f64,
}

// ---------------------------------------------------------------------------
// Fonctions de lecture
// ---------------------------------------------------------------------------

/// Calcule le pourcentage de progression dans un chapitre.
#[must_use]
pub fn reader_chapter_progress(current_page: i32, total_pages: i32) -> f64 {
    if total_pages == 0 {
        return 0.0;
    }
    (f64::from(current_page) / f64::from(total_pages)).clamp(0.0, 1.0)
}

/// Détermine si une page est accessible en mode démo.
#[must_use]
pub fn reader_is_demo_page(page_number: i32, demo_pages: i32) -> bool {
    page_number <= demo_pages
}

/// Calcule les XP gagnés après lecture d'une page.
#[must_use]
pub fn reader_xp_for_page_read(
    is_chapter_end: bool,
    is_work_end: bool,
    is_first_today: bool,
) -> Vec<XpEvent> {
    let mut events = Vec::new();
    let mut total = XP_PER_PAGE;

    events.push(XpEvent {
        xp_gained: XP_PER_PAGE,
        total_xp: 0, // Sera mis à jour par l'appelant
        new_level: None,
        source: "page_read".to_string(),
    });

    if is_chapter_end {
        total += XP_CHAPTER_BONUS;
        events.push(XpEvent {
            xp_gained: XP_CHAPTER_BONUS,
            total_xp: 0,
            new_level: None,
            source: "chapter_complete".to_string(),
        });
    }

    if is_work_end {
        total += XP_WORK_BONUS;
        events.push(XpEvent {
            xp_gained: XP_WORK_BONUS,
            total_xp: 0,
            new_level: None,
            source: "work_complete".to_string(),
        });
    }

    if is_first_today {
        total += XP_DAILY_BONUS;
        events.push(XpEvent {
            xp_gained: XP_DAILY_BONUS,
            total_xp: 0,
            new_level: None,
            source: "daily_bonus".to_string(),
        });
    }

    let _ = total; // Utilisé pour vérification
    events
}

/// Vérifie si un chapitre est entièrement lu.
#[must_use]
pub fn reader_is_chapter_complete(pages_read: i32, total_pages: i32) -> bool {
    total_pages > 0 && pages_read >= total_pages
}

/// Calcule la page suivante dans l'ordre de lecture.
#[must_use]
pub fn reader_next_page_index(current: i32, total: i32) -> Option<i32> {
    if current < total {
        Some(current + 1)
    } else {
        None
    }
}

/// Calcule la page précédente dans l'ordre de lecture.
#[must_use]
pub fn reader_prev_page_index(current: i32) -> Option<i32> {
    if current > 1 {
        Some(current - 1)
    } else {
        None
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chapter_progress() {
        assert!((reader_chapter_progress(5, 10) - 0.5).abs() < 0.01);
        assert!((reader_chapter_progress(10, 10) - 1.0).abs() < 0.01);
        assert!((reader_chapter_progress(0, 10) - 0.0).abs() < 0.01);
        assert!((reader_chapter_progress(0, 0) - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_is_demo_page() {
        assert!(reader_is_demo_page(1, 5));
        assert!(reader_is_demo_page(5, 5));
        assert!(!reader_is_demo_page(6, 5));
    }

    #[test]
    fn test_xp_page_read_basic() {
        let events = reader_xp_for_page_read(false, false, false);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].xp_gained, XP_PER_PAGE);
    }

    #[test]
    fn test_xp_chapter_end() {
        let events = reader_xp_for_page_read(true, false, false);
        assert_eq!(events.len(), 2);
        assert_eq!(events[1].xp_gained, XP_CHAPTER_BONUS);
    }

    #[test]
    fn test_xp_work_end_with_daily() {
        let events = reader_xp_for_page_read(true, true, true);
        assert_eq!(events.len(), 4);
        let total: i64 = events.iter().map(|e| e.xp_gained).sum();
        assert_eq!(
            total,
            XP_PER_PAGE + XP_CHAPTER_BONUS + XP_WORK_BONUS + XP_DAILY_BONUS
        );
    }

    #[test]
    fn test_chapter_complete() {
        assert!(reader_is_chapter_complete(10, 10));
        assert!(reader_is_chapter_complete(15, 10));
        assert!(!reader_is_chapter_complete(9, 10));
        assert!(!reader_is_chapter_complete(0, 0));
    }

    #[test]
    fn test_navigation() {
        assert_eq!(reader_next_page_index(5, 10), Some(6));
        assert_eq!(reader_next_page_index(10, 10), None);
        assert_eq!(reader_prev_page_index(5), Some(4));
        assert_eq!(reader_prev_page_index(1), None);
    }
}
