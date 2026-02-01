#![allow(missing_docs)]
//! # MiyuValidate — toolkit.validate.miyuvalidate
//!
//! Kit d'outils de validation et sanitization (schéma, sanitize). Schéma et politique fournis dans le flux.
//! Alignement MIP : domaine `validate`, layer tool/toolkit.

/// @id: miyuvalidate_toolkit_lib
/// @role: infrastructure
/// @layer: toolkit
/// @human: Point d'entrée du toolkit MiyuValidate ; expose les modules tools.
/// @do: expose_miyuvalidate_toolkit

pub mod admin_cell;
pub mod context;
pub mod errors;
pub mod sanitize;
pub mod schema;

pub use admin_cell::{
    miyuvalidate_admin_cell, MiyuValidateAdminCell, MiyuValidateIdentification, MiyuValidateIntegrity,
    MiyuValidateTestManifest, TOOLKIT_ID,
};
pub use context::GovernedContext;
pub use errors::MiyuValidateError;
pub use sanitize::{sanitize as validate_sanitize, SanitizePolicy, SanitizeValueType};
pub use schema::{check as schema_check, SchemaCheckResult};
