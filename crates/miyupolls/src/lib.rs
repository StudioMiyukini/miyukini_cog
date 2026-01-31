//! # MiyuPolls — toolkit.content.polls
//!
//! Kit d'outils sondages (create, vote, list, result). Décision = StrongFather ; WriteIntent KindMother.
//! Alignement MIP : domaine `content`, layer tool/toolkit.

/// @id: miyupolls_toolkit_lib
/// @role: infrastructure
/// @layer: toolkit
/// @human: Point d'entrée du toolkit MiyuPolls ; expose les modules tools.
/// @do: expose_miyupolls_toolkit

pub mod admin_cell;
pub mod context;
pub mod errors;
pub mod poll;

pub use admin_cell::{
    miyupolls_admin_cell, MiyupollsAdminCell, MiyupollsIdentification, MiyupollsIntegrity,
    MiyupollsTestManifest, TOOLKIT_ID,
};
pub use context::GovernedContext;
pub use errors::MiyupollsError;
pub use poll::{create as poll_create, list as poll_list, result as poll_result, vote as poll_vote, PollItem, PollResult};
