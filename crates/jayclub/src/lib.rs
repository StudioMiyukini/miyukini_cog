//! JayClub — réseau social Miyukini.
//!
//! Refonte modernisée de Jay1Tribu, qui devient le pendant Miyukini de Meta :
//! posts, stories, profils, follows, réactions, commentaires, mentions, hashtags.
//!
//! Réutilise jay1tribu pour les amis/tribus existantes.

pub mod feed;
pub mod notification;
pub mod post;
pub mod profile;
pub mod sqlite_store;
pub mod story;
pub mod store;

pub use sqlite_store::SqliteStore;

pub use feed::FeedFilter;
pub use notification::{Notification, NotificationKind};
pub use post::{Post, PostVisibility, Reaction, ReactionKind};
pub use profile::{Profile, ProfileBadge};
pub use story::{Story, StoryMedia, StoryView};
pub use store::JayClubStore;

// Re-exports pour compat Jay1Tribu
pub use jay1tribu::{Friend, Salon, Tribe};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Identifiant utilisateur (profile id Miyukini Connect).
pub type UserId = String;

/// Identifiant unique d'un objet JayClub (post, story, comment, etc.).
pub type ItemId = String;

/// Génère un nouvel id avec préfixe sémantique (ex: "post:abc-123").
pub fn new_id(prefix: &str) -> ItemId {
    format!("{prefix}:{}", Uuid::new_v4())
}

/// Erreurs JayClub.
#[derive(Debug, thiserror::Error)]
pub enum JayClubError {
    #[error("Élément introuvable: {0}")]
    NotFound(String),
    #[error("Permission refusée")]
    PermissionDenied,
    #[error("Limite atteinte: {0}")]
    LimitReached(String),
    #[error("Données invalides: {0}")]
    Invalid(String),
}

/// Commentaire sur un post.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub id: ItemId,
    pub post_id: ItemId,
    pub author_id: UserId,
    pub content: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Commentaire parent (pour les threads).
    pub reply_to: Option<ItemId>,
    /// Mentions @user dans le commentaire.
    pub mentions: Vec<UserId>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_id_has_prefix() {
        let id = new_id("post");
        assert!(id.starts_with("post:"));
    }
}
