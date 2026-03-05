#![allow(missing_docs)]
//! # MiyuFeeds — toolkit.content.feeds
//!
//! Kit d'outils flux ATOM (board, forum, topic). Lecture = KindMother ; décision accès = StrongFather.
//! Alignement MIP : domaine `content`, layer tool/toolkit.

// @id: toolkit.content.miyufeeds
/// @role: infrastructure
/// @layer: toolkit
/// @human: Point d'entrée du toolkit MiyuFeeds ; expose les modules tools.
/// @do: expose_miyufeeds_toolkit
pub mod admin_cell;
pub mod context;
pub mod errors;
pub mod feed;

pub use admin_cell::{
    miyufeeds_admin_cell, MiyufeedsAdminCell, MiyufeedsIdentification, MiyufeedsIntegrity,
    MiyufeedsTestManifest, TOOLKIT_ID,
};
pub use context::GovernedContext;
pub use errors::MiyufeedsError;
pub use feed::{atom_board, atom_forum, atom_topic};
