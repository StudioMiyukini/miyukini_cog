#![allow(missing_docs)]
//! # MiyuFeeds — toolkit.content.feeds
//!
//! Kit d'outils flux ATOM (board, forum, topic). Lecture = KindMother ; décision accès = StrongFather.
//! Alignement MIP : domaine `content`, layer tool/toolkit.

// C-10 : aucun unwrap/expect/panic hors tests dans le code exposé au réseau.
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

// @id toolkit.content.miyufeeds
/// @role infrastructure
/// @layer toolkit
/// @do expose_miyufeeds_toolkit
//@human / @human Point d'entrée du toolkit MiyuFeeds ; expose les modules tools.
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
