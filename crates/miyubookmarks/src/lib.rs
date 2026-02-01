#![allow(missing_docs)]
//! # MiyuBookmarks — toolkit.content.bookmarks
//!
//! Kit d'outils signets (add, remove, list). Décision = StrongFather ; WriteIntent KindMother.
//! Alignement MIP : domaine `content`, layer tool/toolkit.

/// @id: miyubookmarks_toolkit_lib
/// @role: infrastructure
/// @layer: toolkit
/// @human: Point d'entrée du toolkit MiyuBookmarks ; expose les modules tools.
/// @do: expose_miyubookmarks_toolkit

pub mod admin_cell;
pub mod bookmark;
pub mod context;
pub mod errors;

pub use admin_cell::{
    miyubookmarks_admin_cell, MiyubookmarksAdminCell, MiyubookmarksIdentification, MiyubookmarksIntegrity,
    MiyubookmarksTestManifest, TOOLKIT_ID,
};
pub use bookmark::{add as bookmark_add, list as bookmark_list, remove as bookmark_remove, BookmarkFilters, BookmarkItem};
pub use context::GovernedContext;
pub use errors::MiyubookmarksError;
