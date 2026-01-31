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
pub mod transaction;

pub use admin_cell::{
    miyucptaledger_admin_cell, MiyucptaledgerAdminCell, MiyucptaledgerIdentification,
    MiyucptaledgerIntegrity, MiyucptaledgerTestManifest, TOOLKIT_ID,
};
pub use bank::sync as bank_sync;
pub use company::{siret_resolve as company_siret_resolve, structure_register as company_structure_register, structure_resolve as company_structure_resolve};
pub use context::GovernedContext;
pub use errors::MiyucptaledgerError;
pub use reconciliation::{record as reconciliation_record, suggest as reconciliation_suggest};
pub use transaction::{categorize as transaction_categorize, vat_resolve as transaction_vat_resolve};
