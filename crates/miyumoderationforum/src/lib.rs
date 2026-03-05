#![allow(missing_docs)]
//! # MiyuModerationForum — toolkit.moderation.forum
//!
//! Kit d'outils modération forum (queue, report, topic, post, warning, ban, usernote). Décision = StrongFather ; WriteIntent KindMother.
//! Alignement MIP : domaine `moderation`, layer tool/toolkit.

// @id: toolkit.moderation.miyumoderationforum
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
pub mod store;
pub mod topic;
pub mod usernote;
pub mod warning;

pub use admin_cell::{
    miyumoderationforum_admin_cell, MiyumoderationforumAdminCell,
    MiyumoderationforumIdentification, MiyumoderationforumIntegrity,
    MiyumoderationforumTestManifest, TOOLKIT_ID,
};
pub use ban::{create as ban_create, list as ban_list, BanItem};
pub use context::GovernedContext;
pub use errors::MiyumoderationforumError;
pub use post::{delete as post_delete, edit as post_edit, lock as post_lock};
pub use queue::{get as queue_get, list as queue_list, QueueItem, QueueItemDetail};
pub use report::{create as report_create, list as report_list, ReportItem};
pub use topic::{
    copy as topic_copy, delete as topic_delete, lock as topic_lock, merge as topic_merge,
    r#move as topic_move, split as topic_split,
};
pub use usernote::{create as usernote_create, list as usernote_list, UsernoteItem};
pub use warning::{create as warning_create, list as warning_list, WarningItem};

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx() -> GovernedContext {
        GovernedContext::new("test-mandate".into(), 0)
    }

    #[test]
    fn queue_list_and_get() {
        let c = ctx();
        crate::store::queue_items()
            .lock()
            .unwrap()
            .insert("q1".into(), ("post".into(), "payload".into()));
        let list = queue_list(&c).unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].id, "q1");
        let detail = queue_get(&c, "q1").unwrap();
        assert_eq!(detail.payload, "payload");
        assert!(queue_get(&c, "missing").is_err());
    }

    #[test]
    fn post_edit_lock_delete() {
        let c = ctx();
        crate::store::posts()
            .lock()
            .unwrap()
            .insert("p1".into(), ("content".into(), false));
        post_edit(&c, "p1", "new").unwrap();
        post_lock(&c, "p1").unwrap();
        assert!(crate::store::posts().lock().unwrap().get("p1").unwrap().1);
        post_delete(&c, "p1").unwrap();
        assert!(post_edit(&c, "p1", "x").is_err());
    }

    #[test]
    fn topic_lock_move_merge_split_delete_copy() {
        let c = ctx();
        crate::store::topics()
            .lock()
            .unwrap()
            .insert("t1".into(), ("board1".into(), false));
        topic_lock(&c, "t1").unwrap();
        topic_move(&c, "t1", "board2").unwrap();
        assert_eq!(
            crate::store::topics().lock().unwrap().get("t1").unwrap().0,
            "board2"
        );
        crate::store::topics()
            .lock()
            .unwrap()
            .insert("t2".into(), ("b".into(), false));
        topic_merge(&c, &["t2".to_string()], "t1").unwrap();
        let new_id = topic_split(&c, "t1", &[]).unwrap();
        assert!(new_id.starts_with("topic:"));
        topic_delete(&c, "t1").unwrap();
        let copy_id = topic_copy(&c, &new_id, "board3").unwrap();
        assert!(copy_id.starts_with("topic:"));
    }

    #[test]
    fn report_create_list() {
        let c = ctx();
        let id = report_create(&c, "post", "pid", "spam").unwrap();
        assert!(id.starts_with("rep:"));
        let list = report_list(&c).unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].reason, "spam");
    }

    #[test]
    fn warning_create_list() {
        let c = ctx();
        let id = warning_create(&c, "u1", "reason").unwrap();
        assert!(id.starts_with("warn:"));
        assert_eq!(warning_list(&c).unwrap().len(), 1);
    }

    #[test]
    fn usernote_create_list() {
        let c = ctx();
        let id = usernote_create(&c, "u1", "note content").unwrap();
        assert!(id.starts_with("note:"));
        let list = usernote_list(&c, "u1").unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].content, "note content");
    }

    #[test]
    fn ban_create_list() {
        let c = ctx();
        let id = ban_create(&c, "u1", "reason", Some("2025-12-31")).unwrap();
        assert!(id.starts_with("ban:"));
        let list = ban_list(&c).unwrap();
        assert_eq!(list.len(), 1);
        assert!(list[0].until.is_some());
    }

    #[test]
    fn no_mandate_refused() {
        let c = GovernedContext::new(String::new(), 0);
        assert!(queue_list(&c).is_err());
        assert!(report_create(&c, "a", "b", "c").is_err());
    }
}
