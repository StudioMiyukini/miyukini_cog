//! Système de gamification JayManga.
//!
//! XP par page/chapitre/oeuvre, niveaux (Curieux → Légendaire),
//! streaks, badges, événements de progression.

// ---------------------------------------------------------------------------
// Constantes XP
// ---------------------------------------------------------------------------

/// XP gagné par page lue.
pub const XP_PER_PAGE: i64 = 1;
/// XP bonus à la fin d'un chapitre.
pub const XP_CHAPTER_BONUS: i64 = 10;
/// XP bonus à la fin d'une oeuvre.
pub const XP_WORK_BONUS: i64 = 50;
/// XP bonus pour la première lecture du jour.
pub const XP_DAILY_BONUS: i64 = 5;
/// XP bonus pour un nouveau genre exploré.
pub const XP_NEW_GENRE: i64 = 15;
/// XP bonus pour un nouveau COG visité.
pub const XP_NEW_COG: i64 = 10;
/// XP bonus pour un nouveau format exploré.
pub const XP_NEW_FORMAT: i64 = 20;
/// XP bonus à 7 jours de streak.
pub const XP_STREAK_7: i64 = 25;
/// XP bonus à 30 jours de streak.
pub const XP_STREAK_30: i64 = 100;

// ---------------------------------------------------------------------------
// Niveaux
// ---------------------------------------------------------------------------

/// Seuils de niveaux (niveau, nom, XP minimum).
pub const LEVEL_THRESHOLDS: [(i32, &str, i64); 8] = [
    (1, "Curieux", 0),
    (2, "Lecteur", 100),
    (3, "Passionné", 500),
    (4, "Dévoreur", 1_500),
    (5, "Otaku", 5_000),
    (6, "Connaisseur", 15_000),
    (7, "Sage", 40_000),
    (8, "Légendaire", 100_000),
];

/// Nombre de pages minimum pour qu'un jour de lecture soit comptabilisé
/// dans le streak.
pub const STREAK_MIN_PAGES: i32 = 5;

// ---------------------------------------------------------------------------
// Erreurs
// ---------------------------------------------------------------------------

/// Erreur du module gamification.
#[derive(Debug, thiserror::Error)]
pub enum GamificationError {
    #[error("Database error: {0}")]
    Db(String),
    #[error("Progression not initialized")]
    NotInitialized,
}

// ---------------------------------------------------------------------------
// Événements
// ---------------------------------------------------------------------------

/// Événement XP (retourné après attribution).
#[derive(Debug, Clone)]
pub struct XpEvent {
    pub xp_gained: i64,
    pub total_xp: i64,
    pub new_level: Option<(i32, String)>,
    pub source: String,
}

/// Événement streak.
#[derive(Debug, Clone)]
pub struct StreakEvent {
    pub current_streak: i32,
    pub longest_streak: i32,
    pub bonus_xp: i64,
    pub shield_used: bool,
}

// ---------------------------------------------------------------------------
// Fonctions de calcul
// ---------------------------------------------------------------------------

/// Retourne le niveau et son nom pour un XP donné.
#[must_use]
pub fn gamification_level_for_xp(xp: i64) -> (i32, &'static str) {
    let mut result = (1, "Curieux");
    for &(level, name, threshold) in &LEVEL_THRESHOLDS {
        if xp >= threshold {
            result = (level, name);
        }
    }
    result
}

/// Retourne l'XP nécessaire pour le prochain niveau.
/// Retourne 0 si le joueur est au niveau maximum.
#[must_use]
pub fn gamification_xp_for_next_level(current_xp: i64) -> i64 {
    for &(_, _, threshold) in &LEVEL_THRESHOLDS {
        if threshold > current_xp {
            return threshold - current_xp;
        }
    }
    0 // Niveau max atteint
}

/// Retourne le pourcentage de progression vers le prochain niveau (0.0 à 1.0).
#[must_use]
pub fn gamification_level_progress(current_xp: i64) -> f64 {
    let (current_level, _) = gamification_level_for_xp(current_xp);

    let current_threshold = LEVEL_THRESHOLDS
        .iter()
        .find(|&&(l, _, _)| l == current_level)
        .map(|&(_, _, t)| t)
        .unwrap_or(0);

    let next_threshold = LEVEL_THRESHOLDS
        .iter()
        .find(|&&(l, _, _)| l == current_level + 1)
        .map(|&(_, _, t)| t);

    match next_threshold {
        Some(next) => {
            let range = next - current_threshold;
            if range == 0 {
                return 1.0;
            }
            let progress = current_xp - current_threshold;
            (progress as f64 / range as f64).clamp(0.0, 1.0)
        }
        None => 1.0, // Niveau max
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_for_xp_boundaries() {
        assert_eq!(gamification_level_for_xp(0), (1, "Curieux"));
        assert_eq!(gamification_level_for_xp(99), (1, "Curieux"));
        assert_eq!(gamification_level_for_xp(100), (2, "Lecteur"));
        assert_eq!(gamification_level_for_xp(500), (3, "Passionné"));
        assert_eq!(gamification_level_for_xp(5000), (5, "Otaku"));
        assert_eq!(gamification_level_for_xp(100_000), (8, "Légendaire"));
        assert_eq!(gamification_level_for_xp(999_999), (8, "Légendaire"));
    }

    #[test]
    fn test_xp_for_next_level() {
        assert_eq!(gamification_xp_for_next_level(0), 100); // → Lecteur
        assert_eq!(gamification_xp_for_next_level(50), 50); // → Lecteur
        assert_eq!(gamification_xp_for_next_level(100), 400); // → Passionné
        assert_eq!(gamification_xp_for_next_level(100_000), 0); // Max
    }

    #[test]
    fn test_level_progress() {
        assert!((gamification_level_progress(0) - 0.0).abs() < 0.01);
        assert!((gamification_level_progress(50) - 0.5).abs() < 0.01);
        assert!((gamification_level_progress(100_000) - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_constants_consistency() {
        // Vérifier que les niveaux sont ordonnés
        for i in 1..LEVEL_THRESHOLDS.len() {
            assert!(LEVEL_THRESHOLDS[i].2 > LEVEL_THRESHOLDS[i - 1].2);
            assert_eq!(LEVEL_THRESHOLDS[i].0, LEVEL_THRESHOLDS[i - 1].0 + 1);
        }
    }
}
