#![allow(missing_docs)]
//! # MiyuContacts — toolkit.communication.contacts
//!
//! Kit d'outils contacts (friend, foe, list). Décision = StrongFather ; WriteIntent KindMother.
//! Alignement MIP : domaine `communication`, layer tool/toolkit.

/// @id: miyucontacts_toolkit_lib
/// @role: infrastructure
/// @layer: toolkit
/// @human: Point d'entrée du toolkit MiyuContacts ; expose les modules tools.
/// @do: expose_miyucontacts_toolkit

pub mod admin_cell;
pub mod contacts;
pub mod context;
pub mod errors;
pub mod foe;
pub mod friend;

pub use admin_cell::{
    miyucontacts_admin_cell, MiyucontactsAdminCell, MiyucontactsIdentification, MiyucontactsIntegrity,
    MiyucontactsTestManifest, TOOLKIT_ID,
};
pub use contacts::list as contacts_list;
pub use context::GovernedContext;
pub use errors::MiyucontactsError;
pub use foe::{add as foe_add, list as foe_list, remove as foe_remove};
pub use friend::{add as friend_add, list as friend_list, remove as friend_remove, ContactItem};
