#![allow(missing_docs)]
//! # MiyuLocale — toolkit.locale.miyulocale
//!
//! Kit d'outils de locale et i18n (date, number, translate). Locale et catalogue fournis dans le flux.
//! Alignement MIP : domaine `locale`, layer tool/toolkit.

// @id: toolkit.locale.miyulocale
/// @role: infrastructure
/// @layer: toolkit
/// @human: Point d'entrée du toolkit MiyuLocale ; expose les modules tools.
/// @do: expose_miyulocale_toolkit
pub mod admin_cell;
pub mod context;
pub mod date;
pub mod errors;
pub mod number;
pub mod translate;

pub use admin_cell::{
    miyulocale_admin_cell, MiyuLocaleAdminCell, MiyuLocaleIdentification, MiyuLocaleIntegrity,
    MiyuLocaleTestManifest, TOOLKIT_ID,
};
pub use context::GovernedContext;
pub use date::{format as date_format, DateFormatOptions};
pub use errors::MiyuLocaleError;
pub use number::{format as locale_number_format, LocaleNumberOptions};
pub use translate::translate as locale_translate;
