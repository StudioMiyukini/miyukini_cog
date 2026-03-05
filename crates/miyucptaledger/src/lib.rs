#![allow(missing_docs)]
//! # MiyuComptaLedger — toolkit.compta.ledger
//!
//! Kit d'outils tenue des livres (synchro bancaire, écritures, rapprochement, structure entreprise).
//! Persistance = KindMother (WriteIntent) ; validation rapprochement = StrongFather.
//! Alignement MIP : domaine `compta`, layer tool/toolkit.

pub mod admin_cell;
pub mod bank;
pub mod company;
pub mod context;
pub mod errors;
pub mod reconciliation;
mod store;
pub mod transaction;

pub use admin_cell::{
    miyucptaledger_admin_cell, MiyucptaledgerAdminCell, MiyucptaledgerIdentification,
    MiyucptaledgerIntegrity, MiyucptaledgerTestManifest, TOOLKIT_ID,
};
pub use bank::sync as bank_sync;
pub use company::{
    siret_resolve as company_siret_resolve, structure_register as company_structure_register,
    structure_resolve as company_structure_resolve,
};
pub use context::GovernedContext;
pub use errors::MiyucptaledgerError;
pub use reconciliation::{record as reconciliation_record, suggest as reconciliation_suggest};
pub use transaction::{
    categorize as transaction_categorize, vat_resolve as transaction_vat_resolve,
};

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx() -> GovernedContext {
        GovernedContext::new("m".into(), 0)
    }

    #[test]
    fn bank_company_transaction_reconciliation() {
        let c = ctx();
        assert_eq!(bank_sync(&c, "{}").unwrap(), "sync:ok");
        assert_eq!(company_structure_resolve(&c, None).unwrap(), "micro");
        let id = company_structure_register(&c, "EURL").unwrap();
        assert!(id.starts_with("struct:"));
        assert_eq!(company_structure_resolve(&c, Some(&id)).unwrap(), "EURL");
        assert!(company_siret_resolve(&c, "123").unwrap().contains("123"));
        assert_eq!(transaction_categorize(&c, "t1", "{}").unwrap(), "default");
        assert_eq!(transaction_vat_resolve(&c, "t1").unwrap(), "20");
        assert!(reconciliation_suggest(&c, None).unwrap().is_empty());
        assert_eq!(reconciliation_record(&c, "{}").unwrap(), "rec:ok");
    }

    #[test]
    fn no_mandate_refused() {
        let c = GovernedContext::new(String::new(), 0);
        assert!(bank_sync(&c, "").is_err());
    }
}
