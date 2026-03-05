//! Rapport PDF synthétique des ventes JayManga.
//!
//! Génération de données structurées pour un rapport vendeur.
//! Le rendu PDF effectif est délégué à un moteur externe.

use serde::Serialize;

// ---------------------------------------------------------------------------
// Erreurs
// ---------------------------------------------------------------------------

/// Erreur de génération de rapport.
#[derive(Debug, thiserror::Error)]
pub enum PdfReportError {
    #[error("No data available for report")]
    NoData,
    #[error("Generation error: {0}")]
    Generation(String),
}

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/// Données structurées pour le rapport PDF vendeur.
#[derive(Debug, Clone, Serialize)]
pub struct SalesReportData {
    pub shop_name: String,
    pub period: String,
    pub generated_at: String,
    pub summary: SalesReportSummary,
    pub top_works: Vec<TopWorkEntry>,
    pub revenue_by_method: Vec<RevenueByMethod>,
}

/// Résumé des ventes.
#[derive(Debug, Clone, Serialize)]
pub struct SalesReportSummary {
    pub total_revenue: i64, // centimes
    pub total_transactions: i32,
    pub total_refunds: i32,
    pub net_revenue: i64, // centimes
    pub currency: String,
    pub unique_buyers: i32,
}

/// Oeuvre la plus vendue.
#[derive(Debug, Clone, Serialize)]
pub struct TopWorkEntry {
    pub work_id: String,
    pub title: String,
    pub sales_count: i32,
    pub revenue: i64, // centimes
}

/// Revenu par méthode de paiement.
#[derive(Debug, Clone, Serialize)]
pub struct RevenueByMethod {
    pub method: String,
    pub transaction_count: i32,
    pub revenue: i64, // centimes
}

// ---------------------------------------------------------------------------
// Construction
// ---------------------------------------------------------------------------

/// Formate un montant en centimes pour l'affichage.
#[must_use]
pub fn format_price(cents: i64, currency: &str) -> String {
    let euros = cents as f64 / 100.0;
    match currency {
        "EUR" => format!("{euros:.2} €"),
        "USD" => format!("${euros:.2}"),
        "GBP" => format!("£{euros:.2}"),
        _ => format!("{euros:.2} {currency}"),
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_price_eur() {
        assert_eq!(format_price(799, "EUR"), "7.99 €");
        assert_eq!(format_price(0, "EUR"), "0.00 €");
        assert_eq!(format_price(100_000, "EUR"), "1000.00 €");
    }

    #[test]
    fn test_format_price_usd() {
        assert_eq!(format_price(1299, "USD"), "$12.99");
    }

    #[test]
    fn test_format_price_unknown() {
        assert_eq!(format_price(500, "JPY"), "5.00 JPY");
    }

    #[test]
    fn test_report_data_serialize() {
        let data = SalesReportData {
            shop_name: "Ma Boutique".to_string(),
            period: "2026-02".to_string(),
            generated_at: "2026-02-25T12:00:00Z".to_string(),
            summary: SalesReportSummary {
                total_revenue: 15000,
                total_transactions: 20,
                total_refunds: 1,
                net_revenue: 14500,
                currency: "EUR".to_string(),
                unique_buyers: 15,
            },
            top_works: vec![],
            revenue_by_method: vec![],
        };
        let json = serde_json::to_string(&data).unwrap();
        assert!(json.contains("Ma Boutique"));
        assert!(json.contains("15000"));
    }
}
