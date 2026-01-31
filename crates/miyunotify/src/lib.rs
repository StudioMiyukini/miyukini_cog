//! # MiyuNotify — toolkit.notify.miyunotify
//!
//! Kit d'outils de notification (email, push, inbox). Décision d'envoi = StrongFather.

pub mod admin_cell;
pub mod context;
pub mod email;
pub mod errors;
pub mod inbox;
pub mod push;

pub use admin_cell::{
    miyunotify_admin_cell, MiyunotifyAdminCell, MiyunotifyIdentification, MiyunotifyIntegrity,
    MiyunotifyTestManifest, TOOLKIT_ID,
};
pub use context::GovernedContext;
pub use email::send as email_send;
pub use errors::MiyunotifyError;
pub use inbox::{write as inbox_write, InboxWriteResult};
pub use push::send as push_send;
