#![allow(missing_docs)]
//! # MiyuTreasury — toolkit.treasury.forecast
//!
//! Kit d'outils trésorerie et prévisionnel (tableau de bord, prévisionnel, alertes).
//! Données = KindMother (lecture) ; règles alertes = StrongFather.
//! Alignement MIP : domaine `treasury`, layer tool/toolkit.

pub mod admin_cell;
pub mod alert;
pub mod context;
pub mod dashboard;
pub mod errors;
pub mod forecast;

pub use admin_cell::{
    miyutreasury_admin_cell, MiyutreasuryAdminCell, MiyutreasuryIdentification,
    MiyutreasuryIntegrity, MiyutreasuryTestManifest, TOOLKIT_ID,
};
pub use alert::check as alert_check;
pub use context::GovernedContext;
pub use dashboard::aggregate as dashboard_aggregate;
pub use errors::MiyutreasuryError;
pub use forecast::compute as forecast_compute;
