//! Types de domaine pour le paiement et les licences JayManga.
//!
//! Définit : PurchaseLicense, PaymentTransaction, Promotion, CartItem,
//! et les enums PurchaseType, LicenseStatus, TransactionStatus, DiscountType.

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Licence d'achat (PurchaseLicense)
// ---------------------------------------------------------------------------

/// Licence d'achat liant un acheteur à une oeuvre/chapitre/série.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PurchaseLicense {
    pub id: Option<String>,
    pub buyer_cog_id: Option<String>,
    pub buyer_identity: Option<String>, // LSI, VID, WID
    pub work_id: Option<String>,
    pub purchase_type: Option<String>, // PurchaseType as_str
    pub target_id: Option<String>,
    pub amount_paid: Option<i64>, // centimes
    pub currency: Option<String>,
    pub payment_method: Option<String>,
    pub download_allowed: Option<bool>,
    pub status: Option<String>, // LicenseStatus as_str
    #[serde(default)]
    pub purchased_at: Option<String>,
    pub refunded_at: Option<String>,
}

/// Type d'achat.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PurchaseType {
    Work,
    Chapter,
    Series,
}

impl PurchaseType {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Work => "work",
            Self::Chapter => "chapter",
            Self::Series => "series",
        }
    }

    #[must_use]
    pub fn from_str(s: &str) -> Self {
        match s {
            "chapter" => Self::Chapter,
            "series" => Self::Series,
            _ => Self::Work,
        }
    }
}

/// Statut d'une licence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LicenseStatus {
    Active,
    Refunded,
    Revoked,
}

impl LicenseStatus {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Refunded => "refunded",
            Self::Revoked => "revoked",
        }
    }

    #[must_use]
    pub fn from_str(s: &str) -> Self {
        match s {
            "refunded" => Self::Refunded,
            "revoked" => Self::Revoked,
            _ => Self::Active,
        }
    }
}

// ---------------------------------------------------------------------------
// Transaction de paiement (PaymentTransaction)
// ---------------------------------------------------------------------------

/// Transaction de paiement associée à une licence.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PaymentTransaction {
    pub id: Option<String>,
    pub license_id: Option<String>,
    pub buyer_cog_id: Option<String>,
    pub amount: Option<i64>, // centimes
    pub currency: Option<String>,
    pub method: Option<String>, // "card", "transfer", "other"
    pub status: Option<String>, // TransactionStatus as_str
    pub external_ref: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    pub completed_at: Option<String>,
}

/// Statut d'une transaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransactionStatus {
    Pending,
    Completed,
    Failed,
    Refunded,
    Expired,
}

impl TransactionStatus {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::Refunded => "refunded",
            Self::Expired => "expired",
        }
    }

    #[must_use]
    pub fn from_str(s: &str) -> Self {
        match s {
            "completed" => Self::Completed,
            "failed" => Self::Failed,
            "refunded" => Self::Refunded,
            "expired" => Self::Expired,
            _ => Self::Pending,
        }
    }
}

// ---------------------------------------------------------------------------
// Promotion
// ---------------------------------------------------------------------------

/// Promotion (remise temporaire sur une ou plusieurs oeuvres).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Promotion {
    pub id: Option<String>,
    pub name: Option<String>,
    pub discount_type: Option<String>, // DiscountType as_str
    pub discount_value: Option<i64>,   // pourcentage ou centimes
    pub target_scope: Option<String>,  // "work", "chapter", "series", "catalog"
    pub target_ids: Option<String>,    // JSON Vec<String>
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub active: Option<bool>,
}

/// Type de remise.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiscountType {
    Percent,
    FixedAmount,
    Free,
}

impl DiscountType {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Percent => "percent",
            Self::FixedAmount => "fixed_amount",
            Self::Free => "free",
        }
    }

    #[must_use]
    pub fn from_str(s: &str) -> Self {
        match s {
            "percent" => Self::Percent,
            "fixed_amount" => Self::FixedAmount,
            "free" => Self::Free,
            _ => Self::Percent,
        }
    }
}

// ---------------------------------------------------------------------------
// Article panier (CartItem)
// ---------------------------------------------------------------------------

/// Article dans le panier du lecteur.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CartItem {
    pub id: Option<String>,
    pub buyer_cog_id: Option<String>,
    pub work_id: Option<String>,
    pub purchase_type: Option<String>,
    pub target_id: Option<String>,
    pub unit_price: Option<i64>,
    pub currency: Option<String>,
    pub promotion_id: Option<String>,
    pub discounted_price: Option<i64>,
    #[serde(default)]
    pub added_at: Option<String>,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_purchase_type_roundtrip() {
        for pt in [
            PurchaseType::Work,
            PurchaseType::Chapter,
            PurchaseType::Series,
        ] {
            assert_eq!(PurchaseType::from_str(pt.as_str()), pt);
        }
    }

    #[test]
    fn test_license_status_roundtrip() {
        for s in [
            LicenseStatus::Active,
            LicenseStatus::Refunded,
            LicenseStatus::Revoked,
        ] {
            assert_eq!(LicenseStatus::from_str(s.as_str()), s);
        }
    }

    #[test]
    fn test_transaction_status_roundtrip() {
        for s in [
            TransactionStatus::Pending,
            TransactionStatus::Completed,
            TransactionStatus::Failed,
            TransactionStatus::Refunded,
            TransactionStatus::Expired,
        ] {
            assert_eq!(TransactionStatus::from_str(s.as_str()), s);
        }
    }

    #[test]
    fn test_discount_type_roundtrip() {
        for dt in [
            DiscountType::Percent,
            DiscountType::FixedAmount,
            DiscountType::Free,
        ] {
            assert_eq!(DiscountType::from_str(dt.as_str()), dt);
        }
    }

    #[test]
    fn test_payment_amounts_in_centimes() {
        let license = PurchaseLicense {
            amount_paid: Some(999),
            ..Default::default()
        };
        assert_eq!(license.amount_paid, Some(999)); // 9,99 EUR
    }
}
