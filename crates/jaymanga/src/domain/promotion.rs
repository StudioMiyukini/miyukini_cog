//! Logique de gestion des promotions JayManga.
//!
//! Création, validation, et vérification d'applicabilité des promotions
//! sur les oeuvres, chapitres, séries ou le catalogue entier.

use crate::data::*;

// ---------------------------------------------------------------------------
// Erreurs
// ---------------------------------------------------------------------------

/// Erreur du module promotions.
#[derive(Debug, thiserror::Error)]
pub enum PromotionError {
    #[error("Database error: {0}")]
    Db(#[from] DbError),
    #[error("Promotion not found: {0}")]
    NotFound(String),
    #[error("Promotion expired: {0}")]
    Expired(String),
    #[error("Invalid promotion: {0}")]
    Invalid(String),
}

// ---------------------------------------------------------------------------
// Validation
// ---------------------------------------------------------------------------

/// Valide qu'une promotion est cohérente avant création.
pub fn promotion_validate(promo: &Promotion) -> Result<(), PromotionError> {
    let discount_type = promo.discount_type.as_deref().unwrap_or("percent");
    let discount_value = promo.discount_value.unwrap_or(0);

    match discount_type {
        "percent" => {
            if discount_value < 0 || discount_value > 100 {
                return Err(PromotionError::Invalid(format!(
                    "Le pourcentage doit être entre 0 et 100, reçu : {discount_value}"
                )));
            }
        }
        "fixed_amount" => {
            if discount_value < 0 {
                return Err(PromotionError::Invalid(
                    "Le montant fixe ne peut pas être négatif.".to_string(),
                ));
            }
        }
        "free" => { /* Toujours valide */ }
        other => {
            return Err(PromotionError::Invalid(format!(
                "Type de remise inconnu : {other}"
            )));
        }
    }

    // Vérifier les dates si présentes
    if let (Some(start), Some(end)) = (&promo.start_date, &promo.end_date) {
        if !start.is_empty() && !end.is_empty() && start > end {
            return Err(PromotionError::Invalid(
                "La date de début ne peut pas être après la date de fin.".to_string(),
            ));
        }
    }

    Ok(())
}

/// Vérifie si une promotion est actuellement active (dates + flag active).
#[must_use]
pub fn promotion_is_active(promo: &Promotion, now: &str) -> bool {
    let active = promo.active.unwrap_or(false);
    if !active {
        return false;
    }

    let start_ok = promo
        .start_date
        .as_deref()
        .map_or(true, |s| s.is_empty() || now >= s);
    let end_ok = promo
        .end_date
        .as_deref()
        .map_or(true, |e| e.is_empty() || now <= e);

    start_ok && end_ok
}

/// Vérifie si une promotion s'applique à une cible donnée (work_id, chapter_id, series_id).
#[must_use]
pub fn promotion_applies_to(promo: &Promotion, target_id: &str) -> bool {
    let scope = promo.target_scope.as_deref().unwrap_or("catalog");
    if scope == "catalog" {
        return true;
    }

    // target_ids est un JSON sérialisé Vec<String>
    promo
        .target_ids
        .as_deref()
        .and_then(|json| serde_json::from_str::<Vec<String>>(json).ok())
        .map_or(false, |ids| ids.iter().any(|id| id == target_id))
}

/// Trouve la meilleure promotion applicable pour un target donné,
/// parmi une liste de promotions actives.
#[must_use]
pub fn promotion_best_for_target<'a>(
    promotions: &'a [Promotion],
    target_id: &str,
    price: i64,
    now: &str,
) -> Option<&'a Promotion> {
    promotions
        .iter()
        .filter(|p| promotion_is_active(p, now) && promotion_applies_to(p, target_id))
        .max_by_key(|p| {
            // Calculer l'économie effective pour comparer
            let discount_type = p.discount_type.as_deref().unwrap_or("percent");
            let discount_value = p.discount_value.unwrap_or(0);
            match discount_type {
                "percent" => price * discount_value / 100,
                "fixed_amount" => discount_value.min(price),
                "free" => price,
                _ => 0,
            }
        })
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn make_promo(
        discount_type: &str,
        value: i64,
        scope: &str,
        active: bool,
    ) -> Promotion {
        Promotion {
            id: Some("promo-1".to_string()),
            discount_type: Some(discount_type.to_string()),
            discount_value: Some(value),
            target_scope: Some(scope.to_string()),
            target_ids: Some(r#"["work-1","work-2"]"#.to_string()),
            active: Some(active),
            start_date: Some("2026-01-01T00:00:00Z".to_string()),
            end_date: Some("2026-12-31T23:59:59Z".to_string()),
            ..Default::default()
        }
    }

    #[test]
    fn test_validate_percent_ok() {
        let promo = make_promo("percent", 30, "catalog", true);
        assert!(promotion_validate(&promo).is_ok());
    }

    #[test]
    fn test_validate_percent_out_of_range() {
        let promo = make_promo("percent", 150, "catalog", true);
        assert!(promotion_validate(&promo).is_err());
    }

    #[test]
    fn test_validate_negative_fixed() {
        let promo = make_promo("fixed_amount", -100, "catalog", true);
        assert!(promotion_validate(&promo).is_err());
    }

    #[test]
    fn test_is_active() {
        let promo = make_promo("percent", 20, "catalog", true);
        assert!(promotion_is_active(&promo, "2026-06-15T12:00:00Z"));
        assert!(!promotion_is_active(&promo, "2027-01-01T00:00:00Z"));
    }

    #[test]
    fn test_is_active_disabled() {
        let promo = make_promo("percent", 20, "catalog", false);
        assert!(!promotion_is_active(&promo, "2026-06-15T12:00:00Z"));
    }

    #[test]
    fn test_applies_to_catalog() {
        let promo = make_promo("percent", 20, "catalog", true);
        assert!(promotion_applies_to(&promo, "any-work-id"));
    }

    #[test]
    fn test_applies_to_specific_work() {
        let promo = make_promo("percent", 20, "work", true);
        assert!(promotion_applies_to(&promo, "work-1"));
        assert!(!promotion_applies_to(&promo, "work-99"));
    }

    #[test]
    fn test_best_promotion() {
        let promos = vec![
            make_promo("percent", 10, "catalog", true),
            make_promo("percent", 30, "catalog", true),
            make_promo("percent", 20, "catalog", true),
        ];
        let best = promotion_best_for_target(&promos, "work-1", 1000, "2026-06-15T12:00:00Z");
        assert!(best.is_some());
        assert_eq!(best.unwrap().discount_value, Some(30));
    }
}
