//! Export CSV des transactions de vente JayManga.
//!
//! Génération de fichiers CSV filtrés par période pour le vendeur.

// ---------------------------------------------------------------------------
// Erreurs
// ---------------------------------------------------------------------------

/// Erreur d'export CSV.
#[derive(Debug, thiserror::Error)]
pub enum CsvExportError {
    #[error("Database error: {0}")]
    Db(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("No transactions found for the given period")]
    NoData,
}

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/// Ligne CSV de transaction.
#[derive(Debug, Clone)]
pub struct CsvTransactionRow {
    pub transaction_id: String,
    pub date: String,
    pub buyer_cog_id: String,
    pub work_title: String,
    pub amount_cents: i64,
    pub currency: String,
    pub method: String,
    pub status: String,
}

// ---------------------------------------------------------------------------
// Génération CSV
// ---------------------------------------------------------------------------

/// En-têtes du fichier CSV transactions.
pub const CSV_HEADERS: &str =
    "transaction_id,date,buyer_cog_id,work_title,amount,currency,method,status";

/// Formate une ligne CSV à partir d'une row.
#[must_use]
pub fn csv_format_row(row: &CsvTransactionRow) -> String {
    // Montant en euros (centimes → décimal)
    let amount = format!("{:.2}", row.amount_cents as f64 / 100.0);
    format!(
        "{},{},{},{},{},{},{},{}",
        csv_escape(&row.transaction_id),
        csv_escape(&row.date),
        csv_escape(&row.buyer_cog_id),
        csv_escape(&row.work_title),
        amount,
        csv_escape(&row.currency),
        csv_escape(&row.method),
        csv_escape(&row.status),
    )
}

/// Génère le contenu CSV complet (headers + lignes).
#[must_use]
pub fn csv_generate(rows: &[CsvTransactionRow]) -> String {
    let mut output = String::from(CSV_HEADERS);
    output.push('\n');
    for row in rows {
        output.push_str(&csv_format_row(row));
        output.push('\n');
    }
    output
}

/// Échappe une valeur CSV (guillemets si contient virgule ou guillemet).
#[must_use]
fn csv_escape(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn make_row() -> CsvTransactionRow {
        CsvTransactionRow {
            transaction_id: "tx-001".to_string(),
            date: "2026-02-25T10:00:00Z".to_string(),
            buyer_cog_id: "cog-buyer".to_string(),
            work_title: "One Piece Vol.1".to_string(),
            amount_cents: 799,
            currency: "EUR".to_string(),
            method: "card".to_string(),
            status: "completed".to_string(),
        }
    }

    #[test]
    fn test_csv_format_row() {
        let row = make_row();
        let csv = csv_format_row(&row);
        assert!(csv.contains("tx-001"));
        assert!(csv.contains("7.99"));
    }

    #[test]
    fn test_csv_escape_comma() {
        assert_eq!(csv_escape("hello,world"), "\"hello,world\"");
    }

    #[test]
    fn test_csv_escape_normal() {
        assert_eq!(csv_escape("hello"), "hello");
    }

    #[test]
    fn test_csv_generate() {
        let rows = vec![make_row(), make_row()];
        let csv = csv_generate(&rows);
        let lines: Vec<&str> = csv.trim().lines().collect();
        assert_eq!(lines.len(), 3); // header + 2 rows
        assert!(lines[0].starts_with("transaction_id"));
    }
}
