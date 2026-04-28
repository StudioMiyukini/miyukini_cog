//! Feed JayClub — timeline algorithmique.

use crate::UserId;
use serde::{Deserialize, Serialize};

/// Filtre de feed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FeedFilter {
    /// Posts des personnes que je suis.
    Following { user_id: UserId },
    /// Tous les posts publics (decouverte).
    Discover,
    /// Posts d'un utilisateur specifique (page profil).
    User { user_id: UserId },
    /// Posts contenant un hashtag.
    Hashtag { tag: String },
    /// Posts mentionnant un utilisateur.
    Mention { user_id: UserId },
    /// Brouillons d'un utilisateur.
    Drafts { user_id: UserId },
}

/// Tri du feed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FeedSort {
    /// Plus recents en premier (chronologique inverse).
    Recent,
    /// Plus populaires (algorithme : reactions + comments + temps).
    Trending,
}

impl Default for FeedSort {
    fn default() -> Self {
        Self::Recent
    }
}

/// Pagination cursor (timestamp + id pour eviter les doublons).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedCursor {
    pub before: chrono::DateTime<chrono::Utc>,
    pub limit: u32,
}

impl Default for FeedCursor {
    fn default() -> Self {
        Self {
            before: chrono::Utc::now(),
            limit: 20,
        }
    }
}
