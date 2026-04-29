//! Stories JayClub — contenu ephemere 24h.

use crate::{ItemId, UserId};
use serde::{Deserialize, Serialize};

/// Duree de vie d'une story (24h).
pub const STORY_TTL_SECS: i64 = 24 * 3600;

/// Story.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Story {
    pub id: ItemId,
    pub author_id: UserId,
    pub media: StoryMedia,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    /// Texte overlay (legende).
    pub caption: Option<String>,
    /// Mentions @user.
    pub mentions: Vec<UserId>,
    /// Vues (qui a vu la story).
    pub views: Vec<StoryView>,
    /// Reactions emoji rapides.
    pub reactions: Vec<StoryReaction>,
}

/// Type de media d'une story.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum StoryMedia {
    /// Image (photo prise/uploadee).
    #[serde(rename = "image")]
    Image { media_id: String },
    /// Video (max 30s).
    #[serde(rename = "video")]
    Video { media_id: String, duration_ms: u32 },
    /// Texte sur fond colore.
    #[serde(rename = "text")]
    Text {
        text: String,
        background: String,
        font: String,
    },
}

/// Vue d'une story par un utilisateur.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryView {
    pub viewer_id: UserId,
    pub at: chrono::DateTime<chrono::Utc>,
}

/// Reaction emoji rapide sur une story.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryReaction {
    pub user_id: UserId,
    pub emoji: String,
    pub at: chrono::DateTime<chrono::Utc>,
}

impl Story {
    pub fn new(author_id: UserId, media: StoryMedia) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: crate::new_id("story"),
            author_id,
            media,
            created_at: now,
            expires_at: now + chrono::Duration::seconds(STORY_TTL_SECS),
            caption: None,
            mentions: Vec::new(),
            views: Vec::new(),
            reactions: Vec::new(),
        }
    }

    /// True si la story est expiree.
    pub fn is_expired(&self) -> bool {
        chrono::Utc::now() >= self.expires_at
    }

    /// Marque comme vu par un utilisateur (idempotent).
    pub fn add_view(&mut self, viewer_id: UserId) {
        if !self.views.iter().any(|v| v.viewer_id == viewer_id) {
            self.views.push(StoryView {
                viewer_id,
                at: chrono::Utc::now(),
            });
        }
    }

    /// Nombre de vues uniques.
    pub fn view_count(&self) -> u32 {
        self.views.len() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn story_has_24h_ttl() {
        let s = Story::new(
            "alice".into(),
            StoryMedia::Text {
                text: "Hello!".into(),
                background: "#7c3aed".into(),
                font: "sans-serif".into(),
            },
        );
        let diff = s.expires_at - s.created_at;
        assert_eq!(diff.num_hours(), 24);
        assert!(!s.is_expired());
    }

    #[test]
    fn add_view_is_idempotent() {
        let mut s = Story::new(
            "alice".into(),
            StoryMedia::Image { media_id: "m1".into() },
        );
        s.add_view("bob".into());
        s.add_view("bob".into());
        s.add_view("carol".into());
        assert_eq!(s.view_count(), 2);
    }
}
