#![allow(missing_docs)]
//! # MiyuValidate — toolkit.validate.miyuvalidate
//!
//! Kit d'outils de validation et sanitization (schéma, sanitize). Schéma et politique fournis dans le flux.
//! Alignement MIP : domaine `validate`, layer tool/toolkit.

// C-10 : aucun unwrap / expect / panic! / todo! / unimplemented! / unreachable! hors tests.
#![cfg_attr(
    not(test),
    deny(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::todo,
        clippy::unimplemented,
        clippy::unreachable
    )
)]

// @id toolkit.validate.miyuvalidate
/// @role infrastructure
/// @layer toolkit
/// @do expose_miyuvalidate_toolkit
//@human / @human Point d'entrée du toolkit MiyuValidate ; expose les modules tools.
pub mod admin_cell;
pub mod context;
pub mod errors;
pub mod sanitize;
pub mod schema;

pub use admin_cell::{
    miyuvalidate_admin_cell, MiyuValidateAdminCell, MiyuValidateIdentification,
    MiyuValidateIntegrity, MiyuValidateTestManifest, TOOLKIT_ID,
};
pub use context::GovernedContext;
pub use errors::MiyuValidateError;
pub use sanitize::{
    sanitize as validate_sanitize, SanitizePolicy, SanitizeValueType, MAX_LIST_ITEMS,
    MAX_VALUE_BYTES,
};
pub use schema::{
    check as schema_check, SchemaCheckResult, MAX_DATA_BYTES, MAX_SCHEMA_BYTES, MAX_SCHEMA_ENTRIES,
};
