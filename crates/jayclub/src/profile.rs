//! Profil JayClub — bio, avatar, follows.

use crate::UserId;
use serde::{Deserialize, Serialize};

/// Profil utilisateur enrichi.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub user_id: UserId,
    pub username: String,
    pub display_name: String,
    pub bio: String,
    /// ID MiyuCloud de l'avatar.
    pub avatar_media_id: Option<String>,
    /// ID MiyuCloud de l'image de header.
    pub header_media_id: Option<String>,
    /// URL de site web personnel.
    pub website: Option<String>,
    /// Lieu (ville, pays).
    pub location: Option<String>,
    /// Profil verifie (badge bleu).
    pub badges: Vec<ProfileBadge>,
    /// Profil prive (les non-followers ne voient pas les posts).
    pub private: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Stats (denormalisees, recalculees periodiquement).
    pub follower_count: u32,
    pub following_count: u32,
    pub post_count: u32,
}

/// Badge affiche sur un profil.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProfileBadge {
    /// Profil verifie (identite confirmee).
    Verified,
    /// Createur de contenu.
    Creator,
    /// Membre fondateur Miyukini.
    Founder,
    /// Membre Premium.
    Premium,
}

impl ProfileBadge {
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Verified => "✓",
            Self::Creator => "🎨",
            Self::Founder => "⭐",
            Self::Premium => "💎",
        }
    }

    pub fn color(&self) -> &'static str {
        match self {
            Self::Verified => "#1d9bf0",
            Self::Creator => "#a855f7",
            Self::Founder => "#eab308",
            Self::Premium => "#22c55e",
        }
    }
}

impl Profile {
    pub fn new(user_id: UserId, username: String) -> Self {
        Self {
            user_id,
            username: username.clone(),
            display_name: username,
            bio: String::new(),
            avatar_media_id: None,
            header_media_id: None,
            website: None,
            location: None,
            badges: Vec::new(),
            private: false,
            created_at: chrono::Utc::now(),
            follower_count: 0,
            following_count: 0,
            post_count: 0,
        }
    }
}

/// Lien follow entre deux utilisateurs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Follow {
    pub follower_id: UserId,
    pub following_id: UserId,
    pub at: chrono::DateTime<chrono::Utc>,
    /// Notification activee pour les posts de cet utilisateur.
    pub notify: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_profile_defaults() {
        let p = Profile::new("alice".into(), "alice_doe".into());
        assert_eq!(p.username, "alice_doe");
        assert_eq!(p.display_name, "alice_doe");
        assert_eq!(p.follower_count, 0);
        assert!(!p.private);
        assert!(p.badges.is_empty());
    }
}
