#![allow(missing_docs)]
//! # MiyuExport — toolkit.export.miyuexport
//!
//! Kit d'outils d'export (CSV, XLSX, PDF). Données et options fournies dans le flux.
//! Alignement MIP : domaine `export`, layer tool/toolkit.

/// @id: miyuexport_toolkit_lib
/// @role: infrastructure
/// @layer: toolkit
/// @human: Point d'entrée du toolkit MiyuExport ; expose les modules tools.
/// @do: expose_miyuexport_toolkit

pub mod admin_cell;
pub mod context;
pub mod csv;
pub mod errors;
pub mod pdf;
pub mod xlsx;

pub use admin_cell::{
    miyuexport_admin_cell, MiyuExportAdminCell, MiyuExportIdentification, MiyuExportIntegrity,
    MiyuExportTestManifest, TOOLKIT_ID,
};
pub use context::GovernedContext;
pub use csv::{generate as csv_generate, CsvOptions};
pub use errors::MiyuExportError;
pub use pdf::render as pdf_render;
pub use xlsx::{generate as xlsx_generate, XlsxOptions};
