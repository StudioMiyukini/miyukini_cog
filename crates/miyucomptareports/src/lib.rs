#![allow(missing_docs)]
//! # MiyuComptaReports — toolkit.compta.reports
//!
//! Kit d'outils rapports comptables (livre recettes, balance, liasse, cashflow, export ledger).
//! Alignement MIP : domaine `compta`, layer tool/toolkit.

// @id: toolkit.accounting.miyucomptareports
/// @role: infrastructure
/// @layer: toolkit
/// @human: Point d'entrée du toolkit MiyuComptaReports ; expose les modules tools.
/// @do: expose_miyucomptareports_toolkit
pub mod admin_cell;
pub mod context;
pub mod errors;
pub mod export;
pub mod report;

pub use admin_cell::{
    miyucomptareports_admin_cell, MiyucomptareportsAdminCell, MiyucomptareportsIdentification,
    MiyucomptareportsIntegrity, MiyucomptareportsTestManifest, TOOLKIT_ID,
};
pub use context::GovernedContext;
pub use errors::MiyucomptareportsError;
pub use export::ledger as compta_export_ledger;
pub use report::{balance_generate, cashflow_generate, liasse_generate, livre_recettes_generate};

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx() -> GovernedContext {
        GovernedContext::new("m".into(), 0)
    }

    #[test]
    fn reports_and_export() {
        let c = ctx();
        assert!(livre_recettes_generate(&c, "2024-01-01", "2024-12-31")
            .unwrap()
            .is_empty());
        assert!(balance_generate(&c, "2024-12-31").unwrap().is_empty());
        assert!(liasse_generate(&c, "2024").unwrap().is_empty());
        assert!(cashflow_generate(&c, "2024-01-01", "2024-12-31")
            .unwrap()
            .is_empty());
        assert!(compta_export_ledger(&c, "csv", "2024-01-01", "2024-12-31")
            .unwrap()
            .is_empty());
    }

    #[test]
    fn no_mandate_refused() {
        let c = GovernedContext::new(String::new(), 0);
        assert!(livre_recettes_generate(&c, "", "").is_err());
    }
}
