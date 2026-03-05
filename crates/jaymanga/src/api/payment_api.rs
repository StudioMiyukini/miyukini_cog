//! Endpoints REST du paiement JayManga.
//!
//! Panier, checkout, webhooks passerelle, vérification de licences.

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Types de requête / réponse
// ---------------------------------------------------------------------------

/// Requête d'ajout au panier.
#[derive(Debug, Clone, Deserialize)]
pub struct AddToCartRequest {
    pub item_type: String, // "work", "chapter", "series"
    pub item_id: String,
    pub quantity: Option<i32>,
}

/// Élément du panier.
#[derive(Debug, Clone, Serialize)]
pub struct CartItem {
    pub item_type: String,
    pub item_id: String,
    pub title: String,
    pub price: i64,
    pub currency: String,
    pub promotion_discount: Option<i64>,
}

/// Réponse du panier complet.
#[derive(Debug, Clone, Serialize)]
pub struct CartResponse {
    pub items: Vec<CartItem>,
    pub subtotal: i64,
    pub discount: i64,
    pub total: i64,
    pub currency: String,
}

/// Requête de checkout.
#[derive(Debug, Clone, Deserialize)]
pub struct CheckoutRequest {
    pub payment_method: String, // "card", "transfer", "other"
    pub buyer_cog_id: String,
    pub buyer_identity: String,
}

/// Réponse de checkout.
#[derive(Debug, Clone, Serialize)]
pub struct CheckoutResponse {
    pub transaction_id: String,
    pub status: String,
    pub redirect_url: Option<String>, // Pour les passerelles externes
    pub licenses_created: i32,
}

/// Réponse de vérification de licence.
#[derive(Debug, Clone, Serialize)]
pub struct LicenseVerifyResponse {
    pub has_license: bool,
    pub license_status: Option<String>,
    pub download_allowed: bool,
}

// ---------------------------------------------------------------------------
// Calcul panier
// ---------------------------------------------------------------------------

/// Calcule le total d'un panier.
#[must_use]
pub fn cart_compute_total(items: &[CartItem]) -> (i64, i64, i64) {
    let subtotal: i64 = items.iter().map(|i| i.price).sum();
    let discount: i64 = items.iter().filter_map(|i| i.promotion_discount).sum();
    let total = (subtotal - discount).max(0);
    (subtotal, discount, total)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cart_compute_total() {
        let items = vec![
            CartItem {
                item_type: "work".to_string(),
                item_id: "w-1".to_string(),
                title: "Manga 1".to_string(),
                price: 799,
                currency: "EUR".to_string(),
                promotion_discount: Some(100),
            },
            CartItem {
                item_type: "work".to_string(),
                item_id: "w-2".to_string(),
                title: "Manga 2".to_string(),
                price: 599,
                currency: "EUR".to_string(),
                promotion_discount: None,
            },
        ];
        let (subtotal, discount, total) = cart_compute_total(&items);
        assert_eq!(subtotal, 1398);
        assert_eq!(discount, 100);
        assert_eq!(total, 1298);
    }

    #[test]
    fn test_cart_empty() {
        let (subtotal, discount, total) = cart_compute_total(&[]);
        assert_eq!(subtotal, 0);
        assert_eq!(discount, 0);
        assert_eq!(total, 0);
    }
}
