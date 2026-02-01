#![allow(missing_docs)]
//! # MiyuCMS — toolkit.content.cms
//!
//! Kit d'outils CMS (contenus, révisions, commentaires, médias).
//! Toute écriture = WriteIntent KindMother ; décision (publication, modération) = StrongFather.
//! Alignement MIP : domaine `content`, layer tool/toolkit.

pub mod admin_cell;
pub mod comment;
pub mod content;
pub mod context;
pub mod errors;
pub mod media;
pub mod revision;

pub use admin_cell::{
    miyucms_admin_cell, MiyucmsAdminCell, MiyucmsIdentification, MiyucmsIntegrity,
    MiyucmsTestManifest, TOOLKIT_ID,
};
pub use comment::{create as comment_create, list as comment_list, moderate as comment_moderate};
pub use content::{create as content_create, publish as content_publish, schedule as content_schedule, update as content_update};
pub use context::GovernedContext;
pub use errors::MiyucmsError;
pub use media::{serve as media_serve, transform as media_transform, upload as media_upload};
pub use revision::{compare as revision_compare, list as revision_list, restore as revision_restore};
