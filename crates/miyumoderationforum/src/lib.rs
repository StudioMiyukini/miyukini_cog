//! # MiyuModerationForum — toolkit.moderation.forum
//!
//! Kit d'outils modération forum (queue, report, topic, post, warning, ban, usernote). Décision = StrongFather ; WriteIntent KindMother.
//! Alignement MIP : domaine `moderation`, layer tool/toolkit.

/// @id: miyumoderationforum_toolkit_lib
/// @role: infrastructure
/// @layer: toolkit
/// @human: Point d'entrée du toolkit MiyuModerationForum ; expose les modules tools.
/// @do: expose_miyumoderationforum_toolkit

pub mod admin_cell;
pub mod ban;
pub mod context;
pub mod errors;
pub mod post;
pub mod queue;
pub mod report;
pub mod topic;
pub mod usernote;
pub mod warning;

pub use admin_cell::{
    miyumoderationforum_admin_cell, MiyumoderationforumAdminCell, MiyumoderationforumIdentification,
    MiyumoderationforumIntegrity, MiyumoderationforumTestManifest, TOOLKIT_ID,
};
pub use ban::{create as ban_create, list as ban_list, BanItem};
pub use context::GovernedContext;
pub use errors::MiyumoderationforumError;
pub use post::{delete as post_delete, edit as post_edit, lock as post_lock};
pub use queue::{get as queue_get, list as queue_list, QueueItem, QueueItemDetail};
pub use report::{create as report_create, list as report_list, ReportItem};
pub use topic::{copy as topic_copy, delete as topic_delete, lock as topic_lock, merge as topic_merge, r#move as topic_move, split as topic_split};
pub use usernote::{create as usernote_create, list as usernote_list, UsernoteItem};
pub use warning::{create as warning_create, list as warning_list, WarningItem};
