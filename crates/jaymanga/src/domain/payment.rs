//! Logique de paiement et gestion des licences JayManga.
//!
//! Création de licences, vérification, remboursement, expiration,
//! application de promotions.

use crate::data::*;

// ---------------------------------------------------------------------------
// Erreurs
// ---------------------------------------------------------------------------

/// Erreur du module paiement.
#[derive(Debug, thiserror::Error)]
pub enum PaymentError {
    #[error("Database error: {0}")]
    Db(#[from] DbError),
    #[error("License not found: {0}")]
    LicenseNotFound(String),
    #[error("Already purchased: {0}")]
    AlreadyPurchased(String),
    #[error("Invalid amount: {0}")]
    InvalidAmount(String),
    #[error("Gateway error: {0}")]
    Gateway(String),
}

// ---------------------------------------------------------------------------
// Application de promotions
// ---------------------------------------------------------------------------

/// Applique une promotion sur un prix (RM-05 : centimes).
#[must_use]
pub fn payment_apply_promotion(price: i64, promotion: &Promotion) -> i64 {
    let discount_type = promotion
        .discount_type
        .as_deref()
        .unwrap_or("percent");
    let discount_value = promotion.discount_value.unwrap_or(0);

    match DiscountType::from_str(discount_type) {
        DiscountType::Percent => {
            let discount = price * discount_value / 100;
            (price - discount).max(0)
        }
        DiscountType::FixedAmount => (price - discount_value).max(0),
        DiscountType::Free => 0,
    }
}

/// Expire les transactions pending au-delà de `max_age_days`.
#[cfg(feature = "legacy-sqlite")]
pub fn payment_expire_pending(
    _db: &JayMangaDb,
    max_age_days: i32,
) -> Result<usize, PaymentError> {
    let cutoff = chrono::Utc::now()
        - chrono::Duration::days(i64::from(max_age_days));
    let cutoff_str = cutoff.to_rfc3339();

    // Récupérer les transactions pending plus anciennes que le cutoff
    let filters = TransactionFilters {
        status: Some("pending".to_string()),
        to_date: Some(cutoff_str),
        ..Default::default()
    };

    // Pour l'instant, on utilise une approche simplifiée
    let _ = filters;
    // TODO: Implémenter la requête de batch update quand le CRUD transaction_list est disponible
    Ok(0)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn make_promo(discount_type: &str, value: i64) -> Promotion {
        Promotion {
            id: Some("promo-1".to_string()),
            discount_type: Some(discount_type.to_string()),
            discount_value: Some(value),
            ..Default::default()
        }
    }

    #[test]
    fn test_apply_percent_discount() {
        let promo = make_promo("percent", 20);
        assert_eq!(payment_apply_promotion(1000, &promo), 800); // -20% de 10€
    }

    #[test]
    fn test_apply_fixed_discount() {
        let promo = make_promo("fixed_amount", 250);
        assert_eq!(payment_apply_promotion(1000, &promo), 750); // -2.50€ de 10€
    }

    #[test]
    fn test_apply_free_discount() {
        let promo = make_promo("free", 0);
        assert_eq!(payment_apply_promotion(1000, &promo), 0);
    }

    #[test]
    fn test_discount_never_negative() {
        let promo = make_promo("fixed_amount", 2000);
        assert_eq!(payment_apply_promotion(1000, &promo), 0); // pas négatif
    }

    #[test]
    fn test_100_percent_discount() {
        let promo = make_promo("percent", 100);
        assert_eq!(payment_apply_promotion(1000, &promo), 0);
    }
}
