//! Notifications JayClub.

use crate::{ItemId, UserId};
use serde::{Deserialize, Serialize};

/// Type de notification.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum NotificationKind {
    /// Quelqu'un a aime votre post.
    #[serde(rename = "reaction")]
    Reaction {
        from_user_id: UserId,
        post_id: ItemId,
        reaction_kind: String,
    },
    /// Quelqu'un a commente votre post.
    #[serde(rename = "comment")]
    Comment {
        from_user_id: UserId,
        post_id: ItemId,
        comment_id: ItemId,
    },
    /// Quelqu'un vous suit.
    #[serde(rename = "follow")]
    Follow { from_user_id: UserId },
    /// Vous avez ete mentionne dans un post.
    #[serde(rename = "mention")]
    Mention {
        from_user_id: UserId,
        post_id: ItemId,
    },
    /// Quelqu'un a partage votre post.
    #[serde(rename = "share")]
    Share {
        from_user_id: UserId,
        post_id: ItemId,
    },
    /// Quelqu'un a vu votre story.
    #[serde(rename = "story_view")]
    StoryView {
        from_user_id: UserId,
        story_id: ItemId,
    },
}

/// Notification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: ItemId,
    pub user_id: UserId,
    pub kind: NotificationKind,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub read: bool,
}

impl Notification {
    pub fn new(user_id: UserId, kind: NotificationKind) -> Self {
        Self {
            id: crate::new_id("notif"),
            user_id,
            kind,
            created_at: chrono::Utc::now(),
            read: false,
        }
    }
}
