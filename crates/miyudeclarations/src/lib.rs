#![allow(missing_docs)]
//! # MiyuDeclarations — toolkit.compta.declarations
//!
//! Kit d'outils déclarations fiscales et sociales (URSSAF, TVA, deadline, list, estimate).
//! Alignement MIP : domaine `compta`, layer tool/toolkit.

/// @id: miyudeclarations_toolkit_lib
/// @role: infrastructure
/// @layer: toolkit
/// @human: Point d'entrée du toolkit MiyuDeclarations ; expose les modules tools.
/// @do: expose_miyudeclarations_toolkit

pub mod admin_cell;
pub mod context;
pub mod deadline;
pub mod declaration;
pub mod errors;
pub mod estimate;
pub mod tva;
pub mod urssaf;

pub use admin_cell::{
    miyudeclarations_admin_cell, MiyudeclarationsAdminCell, MiyudeclarationsIdentification,
    MiyudeclarationsIntegrity, MiyudeclarationsTestManifest, TOOLKIT_ID,
};
pub use context::GovernedContext;
pub use deadline::{list as deadline_list, DeadlineItem};
pub use declaration::{list as declaration_list, DeclarationFilters, DeclarationItem};
pub use errors::MiyudeclarationsError;
pub use estimate::{cotisations as estimate_cotisations, CotisationsEstimate};
pub use tva::{prepare as tva_prepare, submit as tva_submit, TvaPrepareResult};
pub use urssaf::{prepare as urssaf_prepare, submit as urssaf_submit, UrssafPrepareResult};
