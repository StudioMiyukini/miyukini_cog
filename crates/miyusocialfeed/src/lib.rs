#![allow(missing_docs)]
//! # MiyuSocialFeed — toolkit.social.feed
//!
//! Kit d'outils flux social (post, feed, reaction, share, comment). Décision = StrongFather ; WriteIntent KindMother.
//! Alignement MIP : domaine `social`, layer tool/toolkit.

// @id: toolkit.social.miyusocialfeed
/// @role: infrastructure
/// @layer: toolkit
/// @human: Point d'entrée du toolkit MiyuSocialFeed ; expose les modules tools.
/// @do: expose_miyusocialfeed_toolkit

pub mod admin_cell;
pub mod comment;
pub mod context;
pub mod errors;
pub mod feed;
pub mod post;
pub mod reaction;
pub mod share;

pub use admin_cell::{
    miyusocialfeed_admin_cell, MiyusocialfeedAdminCell, MiyusocialfeedIdentification,
    MiyusocialfeedIntegrity, MiyusocialfeedTestManifest, TOOLKIT_ID,
};
pub use comment::{create as comment_create, delete as comment_delete, list as comment_list, CommentItem};
pub use context::GovernedContext;
pub use errors::MiyusocialfeedError;
pub use feed::{list as feed_list, FeedFilters};
pub use post::{create as post_create, delete as post_delete, get as post_get, update as post_update, PostItem};
pub use reaction::{add as reaction_add, list as reaction_list, remove as reaction_remove, ReactionItem};
pub use share::{create as share_create, list as share_list, ShareItem};
