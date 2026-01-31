//! # MiyuPosAnalytics — toolkit.pos.analytics
//!
//! Kit d'outils analytics ventes PoS (sales, cash, tax, shift, export). Données = KindMother.
//! Alignement MIP : domaine `pos`, layer tool/toolkit.

/// @id: miyuposanalytics_toolkit_lib
/// @role: infrastructure
/// @layer: toolkit
/// @human: Point d'entrée du toolkit MiyuPosAnalytics ; expose les modules tools.
/// @do: expose_miyuposanalytics_toolkit

pub mod admin_cell;
pub mod cash;
pub mod context;
pub mod errors;
pub mod export;
pub mod sales;
pub mod shift;
pub mod tax;

pub use admin_cell::{
    miyuposanalytics_admin_cell, MiyuposanalyticsAdminCell, MiyuposanalyticsIdentification,
    MiyuposanalyticsIntegrity, MiyuposanalyticsTestManifest, TOOLKIT_ID,
};
pub use cash::{discrepancy as cash_discrepancy, CashDiscrepancyResult};
pub use context::GovernedContext;
pub use errors::MiyuposanalyticsError;
pub use export::spreadsheet as export_spreadsheet;
pub use sales::{by_employee as sales_by_employee, by_item as sales_by_item, trend as sales_trend};
pub use shift::close as shift_close;
pub use tax::report as tax_report;
