#![allow(missing_docs)]
//! # MiyuSocialMessaging — toolkit.social.messaging
//!
//! Kit d'outils messagerie (dm, conversation). Décision envoi = StrongFather ; WriteIntent KindMother.
//! Alignement MIP : domaine `social`, layer tool/toolkit.

// @id: toolkit.social.miyusocialmessaging
/// @role: infrastructure
/// @layer: toolkit
/// @human: Point d'entrée du toolkit MiyuSocialMessaging ; expose les modules tools.
/// @do: expose_miyusocialmessaging_toolkit

pub mod admin_cell;
pub mod context;
pub mod conversation;
pub mod dm;
pub mod errors;

pub use admin_cell::{
    miyusocialmessaging_admin_cell, MiyusocialmessagingAdminCell, MiyusocialmessagingIdentification,
    MiyusocialmessagingIntegrity, MiyusocialmessagingTestManifest, TOOLKIT_ID,
};
pub use context::GovernedContext;
pub use conversation::{get as conversation_get, list as conversation_list, ConversationDetail, ConversationItem};
pub use dm::{get as dm_get, list as dm_list, readmark_set, reaction_add as dm_reaction_add, reaction_remove as dm_reaction_remove, send as dm_send, DmItem};
pub use errors::MiyusocialmessagingError;
