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
