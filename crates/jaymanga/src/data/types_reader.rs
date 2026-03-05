//! Types de domaine pour le lecteur JayManga.
//!
//! Définit : ReaderFavorite, ReaderProgression, ReaderBadge,
//! et les enums PurchaseStatus, BadgeCategory.

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Favori lecteur (ReaderFavorite)
// ---------------------------------------------------------------------------

/// Favori cross-COG : référence une oeuvre sur un COG vendeur distant.
/// Stocké sur le COG du lecteur (LOI-3).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReaderFavorite {
    pub id: Option<String>,
    pub seller_cog_id: Option<String>,
    pub work_id: Option<String>,
    pub cached_title: Option<String>,
    pub cached_cover_url: Option<String>,
    pub cached_authors: Option<String>, // JSON sérialisé
    pub cached_format: Option<String>,
    pub purchase_status: Option<String>, // PurchaseStatus as_str
    pub last_read_chapter: Option<i32>,
    pub last_read_page: Option<i32>,
    pub reading_progress: Option<f64>, // 0.0 à 1.0
    #[serde(default)]
    pub added_at: Option<String>,
    #[serde(default)]
    pub last_synced_at: Option<String>,
}

/// Statut d'achat d'un favori.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PurchaseStatus {
    Demo,
    Purchased,
    Downloaded,
}

impl PurchaseStatus {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Demo => "demo",
            Self::Purchased => "purchased",
            Self::Downloaded => "downloaded",
        }
    }

    #[must_use]
    pub fn from_str(s: &str) -> Self {
        match s {
            "purchased" => Self::Purchased,
            "downloaded" => Self::Downloaded,
            _ => Self::Demo,
        }
    }
}

// ---------------------------------------------------------------------------
// Progression lecteur (ReaderProgression)
// ---------------------------------------------------------------------------

/// Progression globale du lecteur (XP, niveaux, streaks).
/// Instance unique par COG lecteur.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReaderProgression {
    pub id: Option<String>,
    pub total_xp: Option<i64>,
    pub current_level: Option<i32>, // 1-8
    pub current_streak: Option<i32>,
    pub longest_streak: Option<i32>,
    pub streak_shield_available: Option<bool>,
    pub last_read_date: Option<String>,
    pub total_pages_read: Option<i64>,
    pub total_works_completed: Option<i32>,
    pub total_chapters_completed: Option<i32>,
    pub genres_explored: Option<String>,  // JSON Vec<String>
    pub formats_explored: Option<String>, // JSON Vec<String>
    pub cogs_visited: Option<String>,     // JSON Vec<String>
    pub languages_read: Option<String>,   // JSON Vec<String>
    pub onboarding_completed: Option<bool>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

// ---------------------------------------------------------------------------
// Badge lecteur (ReaderBadge)
// ---------------------------------------------------------------------------

/// Badge gagné par le lecteur.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReaderBadge {
    pub id: Option<String>,
    pub badge_id: Option<String>, // ex: "first_page", "centurion"
    pub badge_name: Option<String>,
    pub badge_category: Option<String>, // BadgeCategory as_str
    #[serde(default)]
    pub earned_at: Option<String>,
}

/// Catégorie de badge.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BadgeCategory {
    Reading,
    Regularity,
    Exploration,
}

impl BadgeCategory {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Reading => "reading",
            Self::Regularity => "regularity",
            Self::Exploration => "exploration",
        }
    }

    #[must_use]
    pub fn from_str(s: &str) -> Self {
        match s {
            "regularity" => Self::Regularity,
            "exploration" => Self::Exploration,
            _ => Self::Reading,
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_purchase_status_roundtrip() {
        for s in [
            PurchaseStatus::Demo,
            PurchaseStatus::Purchased,
            PurchaseStatus::Downloaded,
        ] {
            assert_eq!(PurchaseStatus::from_str(s.as_str()), s);
        }
    }

    #[test]
    fn test_badge_category_roundtrip() {
        for c in [
            BadgeCategory::Reading,
            BadgeCategory::Regularity,
            BadgeCategory::Exploration,
        ] {
            assert_eq!(BadgeCategory::from_str(c.as_str()), c);
        }
    }

    #[test]
    fn test_reader_progression_default() {
        let prog = ReaderProgression::default();
        assert!(prog.total_xp.is_none());
        assert!(prog.current_level.is_none());
    }
}
