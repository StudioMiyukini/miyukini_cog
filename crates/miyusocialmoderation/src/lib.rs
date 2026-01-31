//! # MiyuSocialModeration — toolkit.social.moderation
//!
//! Kit d'outils modération social (report, block, post). Décision = StrongFather ; WriteIntent KindMother.
//! Alignement MIP : domaine `social`, layer tool/toolkit.

/// @id: miyusocialmoderation_toolkit_lib
/// @role: infrastructure
/// @layer: toolkit
/// @human: Point d'entrée du toolkit MiyuSocialModeration ; expose les modules tools.
/// @do: expose_miyusocialmoderation_toolkit

pub mod admin_cell;
pub mod block;
pub mod context;
pub mod errors;
pub mod post;
pub mod report;

pub use admin_cell::{
    miyusocialmoderation_admin_cell, MiyusocialmoderationAdminCell, MiyusocialmoderationIdentification,
    MiyusocialmoderationIntegrity, MiyusocialmoderationTestManifest, TOOLKIT_ID,
};
pub use block::{add as block_add, list as block_list, remove as block_remove};
pub use context::GovernedContext;
pub use errors::MiyusocialmoderationError;
pub use post::delete as post_delete;
pub use report::{create as report_create, list as report_list, ReportItem};
