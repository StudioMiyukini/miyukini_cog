//! # MiyuInvoice — toolkit.invoice.standalone
//!
//! Kit d'outils facturation métier (devis, factures, électronique, relances, lien paiement, clients).
//! Persistance = KindMother (WriteIntent) ; décisions (relance, devis→facture) = StrongFather.
//! Alignement MIP : domaine `invoice`, layer tool/toolkit.

pub mod admin_cell;
pub mod context;
pub mod customer;
pub mod electronic;
pub mod errors;
pub mod invoice;
pub mod payment_link;
pub mod quote;
pub mod reminder;

pub use admin_cell::{
    miyuinvoice_admin_cell, MiyuinvoiceAdminCell, MiyuinvoiceIdentification, MiyuinvoiceIntegrity,
    MiyuinvoiceTestManifest, TOOLKIT_ID,
};
pub use context::GovernedContext;
pub use customer::{list as customer_list, resolve as customer_resolve};
pub use electronic::submit as electronic_submit;
pub use errors::MiyuinvoiceError;
pub use invoice::{create as invoice_create, send as invoice_send};
pub use payment_link::generate as payment_link_generate;
pub use quote::{create as quote_create, to_invoice as quote_to_invoice, update as quote_update};
pub use reminder::send as reminder_send;
