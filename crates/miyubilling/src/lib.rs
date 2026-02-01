#![allow(missing_docs)]
//! # MiyuBilling — toolkit.billing.saas
//!
//! Kit d'outils facturation SaaS (souscriptions, factures, paiements, tenant).
//! Toute écriture = WriteIntent KindMother ; décision = StrongFather.
//! Alignement MIP : domaine `billing`, layer tool/toolkit.

pub mod admin_cell;
pub mod context;
pub mod errors;
pub mod invoice;
pub mod payment;
pub mod subscription;
pub mod tenant;

pub use admin_cell::{
    miyubilling_admin_cell, MiyubillingAdminCell, MiyubillingIdentification, MiyubillingIntegrity,
    MiyubillingTestManifest, TOOLKIT_ID,
};
pub use context::GovernedContext;
pub use errors::MiyubillingError;
pub use invoice::{generate as invoice_generate, list as invoice_list};
pub use payment::record as payment_record;
pub use subscription::{cancel as subscription_cancel, create as subscription_create, status as subscription_status, update as subscription_update};
pub use tenant::resolve as tenant_resolve;
