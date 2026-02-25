//! Adaptateur d'export transactions JayManga → JayKonta.
//!
//! Convertit les transactions de paiement JayManga en écritures
//! comptables JayKonta. Phase 2 — stub pour l'instant.

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Erreurs
// ---------------------------------------------------------------------------

/// Erreur de l'adaptateur JayKonta.
#[derive(Debug, thiserror::Error)]
pub enum JayKontaAdapterError {
    #[error("Export error: {0}")]
    Export(String),
    #[error("JayKonta not available")]
    NotAvailable,
}

// ---------------------------------------------------------------------------
// Types de contrat
// ---------------------------------------------------------------------------

/// Écriture comptable à exporter vers JayKonta.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KontaEntry {
    pub source_service: String,
    pub transaction_id: String,
    pub amount_cents: i64,
    pub currency: String,
    pub entry_type: String, // "revenue", "refund"
    pub label: String,
    pub date: String,
}

/// Résultat d'export vers JayKonta.
#[derive(Debug, Clone, Default)]
pub struct KontaExportResult {
    pub entries_exported: i32,
    pub errors: Vec<String>,
}

// ---------------------------------------------------------------------------
// Conversion (stub)
// ---------------------------------------------------------------------------

/// Convertit une transaction JayManga en écriture JayKonta.
#[must_use]
pub fn konta_entry_from_transaction(
    transaction_id: &str,
    amount: i64,
    currency: &str,
    is_refund: bool,
    label: &str,
    date: &str,
) -> KontaEntry {
    KontaEntry {
        source_service: "jaymanga".to_string(),
        transaction_id: transaction_id.to_string(),
        amount_cents: amount,
        currency: currency.to_string(),
        entry_type: if is_refund { "refund" } else { "revenue" }.to_string(),
        label: label.to_string(),
        date: date.to_string(),
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_konta_entry_revenue() {
        let entry = konta_entry_from_transaction(
            "tx-001",
            999,
            "EUR",
            false,
            "Vente One Piece Vol.1",
            "2026-02-25T10:00:00Z",
        );
        assert_eq!(entry.entry_type, "revenue");
        assert_eq!(entry.amount_cents, 999);
    }

    #[test]
    fn test_konta_entry_refund() {
        let entry = konta_entry_from_transaction(
            "tx-002",
            500,
            "EUR",
            true,
            "Remboursement",
            "2026-02-25T12:00:00Z",
        );
        assert_eq!(entry.entry_type, "refund");
    }
}
