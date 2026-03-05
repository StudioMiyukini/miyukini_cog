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
mod store;

pub use admin_cell::{
    miyusocialmessaging_admin_cell, MiyusocialmessagingAdminCell,
    MiyusocialmessagingIdentification, MiyusocialmessagingIntegrity,
    MiyusocialmessagingTestManifest, TOOLKIT_ID,
};
pub use context::GovernedContext;
pub use conversation::{
    get as conversation_get, list as conversation_list, ConversationDetail, ConversationItem,
};
pub use dm::{
    get as dm_get, list as dm_list, reaction_add as dm_reaction_add,
    reaction_remove as dm_reaction_remove, readmark_set, send as dm_send, DmItem,
};
pub use errors::MiyusocialmessagingError;

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx() -> GovernedContext {
        GovernedContext::new("user-1".into(), 0)
    }

    #[test]
    fn conversation_and_dm() {
        let c = ctx();
        let msg_id = dm_send(&c, "conv-1", "hello").unwrap();
        assert!(msg_id.starts_with("msg:"));
        let list = conversation_list(&c).unwrap();
        assert_eq!(list.len(), 1);
        let detail = conversation_get(&c, "conv-1").unwrap();
        assert_eq!(detail.message_ids.len(), 1);
        assert_eq!(dm_list(&c, "conv-1").unwrap().len(), 1);
        assert_eq!(dm_get(&c, &msg_id).unwrap().content, "hello");
        dm_reaction_add(&c, &msg_id, "like").unwrap();
        readmark_set(&c, "conv-1", &msg_id).unwrap();
    }

    #[test]
    fn no_mandate_refused() {
        let c = GovernedContext::new(String::new(), 0);
        assert!(dm_send(&c, "c", "x").is_err());
    }
}
