//! # MiyuPM — toolkit.communication.pm
//!
//! Kit d'outils de messages privés (envoi, liste, dossiers, brouillons, conversations, export).
//! Décision d'envoi = StrongFather ; toute écriture = WriteIntent KindMother.
//! Alignement MIP : domaine `communication` / pm, layer tool/toolkit.

pub mod admin_cell;
pub mod context;
pub mod conversation;
pub mod draft;
pub mod errors;
pub mod export;
pub mod folder;
pub mod message;

pub use admin_cell::{
    miyupm_admin_cell, MiyupmAdminCell, MiyupmIdentification, MiyupmIntegrity,
    MiyupmTestManifest, TOOLKIT_ID,
};
pub use context::GovernedContext;
pub use conversation::{get as conversation_get, list as conversation_list};
pub use draft::{create as draft_create, list as draft_list, update as draft_update};
pub use errors::MiyupmError;
pub use export::export as pm_export;
pub use folder::{create as folder_create, list as folder_list, update as folder_update};
pub use message::{get as message_get, list as message_list, send as message_send};
