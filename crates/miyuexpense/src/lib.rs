#![allow(missing_docs)]
//! # MiyuExpense — toolkit.expense.claims
//!
//! Kit d'outils notes de frais et indemnités (justificatifs, notes, validation, export).
//! Persistance = KindMother (WriteIntent) ; validation et export = StrongFather.
//! Alignement MIP : domaine `expense`, layer tool/toolkit.

pub mod admin_cell;
pub mod claim;
pub mod context;
pub mod errors;
pub mod mileage;
pub mod receipt;

pub use admin_cell::{
    miyuexpense_admin_cell, MiyuexpenseAdminCell, MiyuexpenseIdentification, MiyuexpenseIntegrity,
    MiyuexpenseTestManifest, TOOLKIT_ID,
};
pub use claim::{create as claim_create, export as claim_export, list as claim_list, update as claim_update, validate as claim_validate};
pub use context::GovernedContext;
pub use errors::MiyuexpenseError;
pub use mileage::{calculate as mileage_calculate, export as mileage_export};
pub use receipt::{capture as receipt_capture, extract as receipt_extract};

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx() -> GovernedContext {
        GovernedContext::new("m".into(), 0)
    }

    #[test]
    fn claim_receipt_mileage() {
        let c = ctx();
        assert_eq!(claim_create(&c, "{}").unwrap(), "claim:stub");
        assert!(claim_update(&c, "c1", "{}").is_ok());
        assert!(claim_list(&c, None).unwrap().is_empty());
        assert!(claim_validate(&c, "c1").is_ok());
        assert!(claim_export(&c, "c1", None).unwrap().is_empty());
        assert_eq!(receipt_capture(&c, "r", "{}").unwrap(), "receipt:stub");
        assert_eq!(receipt_extract(&c, "r").unwrap(), "{}");
        assert_eq!(mileage_calculate(&c, "{}", None).unwrap(), "0");
        assert!(mileage_export(&c, "{}", "csv").unwrap().is_empty());
    }

    #[test]
    fn no_mandate_refused() {
        let c = GovernedContext::new(String::new(), 0);
        assert!(claim_create(&c, "").is_err());
    }
}
