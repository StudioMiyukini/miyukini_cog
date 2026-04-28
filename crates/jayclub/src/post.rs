//! Post JayClub — unite de contenu publiee dans le feed.

use crate::{ItemId, UserId};
use serde::{Deserialize, Serialize};

/// Visibilite d'un post.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PostVisibility {
    /// Visible par tous (meme non connectes).
    Public,
    /// Visible uniquement par les followers.
    Followers,
    /// Visible par les amis (relation bidirectionnelle).
    Friends,
    /// Visible par une tribu specifique.
    Tribe { tribe_id: String },
    /// Brouillon (non publie).
    Draft,
}

/// Post — texte + medias + reactions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Post {
    pub id: ItemId,
    pub author_id: UserId,
    pub content: String,
    /// IDs MiyuCloud des medias attaches (photos, videos).
    pub media_ids: Vec<String>,
    pub visibility: PostVisibility,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub edited_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Mentions @user dans le contenu.
    pub mentions: Vec<UserId>,
    /// Hashtags # (sans le #).
    pub hashtags: Vec<String>,
    /// Lieu (geotag).
    pub location: Option<String>,
    /// Reactions par type (Like, Love, Haha, etc.).
    pub reactions: Vec<Reaction>,
    pub comment_count: u32,
    pub share_count: u32,
}

impl Post {
    pub fn new(author_id: UserId, content: String) -> Self {
        Self {
            id: crate::new_id("post"),
            author_id,
            content: content.clone(),
            media_ids: Vec::new(),
            visibility: PostVisibility::Public,
            created_at: chrono::Utc::now(),
            edited_at: None,
            mentions: extract_mentions(&content),
            hashtags: extract_hashtags(&content),
            location: None,
            reactions: Vec::new(),
            comment_count: 0,
            share_count: 0,
        }
    }

    /// Met a jour le contenu et recalcule mentions/hashtags.
    pub fn update_content(&mut self, content: String) {
        self.mentions = extract_mentions(&content);
        self.hashtags = extract_hashtags(&content);
        self.content = content;
        self.edited_at = Some(chrono::Utc::now());
    }

    /// Compte les reactions par type.
    pub fn reaction_count(&self, kind: ReactionKind) -> u32 {
        self.reactions.iter().filter(|r| r.kind == kind).count() as u32
    }

    /// Total de reactions (toutes types confondus).
    pub fn total_reactions(&self) -> u32 {
        self.reactions.len() as u32
    }
}

/// Type de reaction (style Facebook).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReactionKind {
    Like,
    Love,
    Haha,
    Wow,
    Sad,
    Angry,
    Care,
}

impl ReactionKind {
    pub fn emoji(&self) -> &'static str {
        match self {
            Self::Like => "👍",
            Self::Love => "❤️",
            Self::Haha => "😂",
            Self::Wow => "😮",
            Self::Sad => "😢",
            Self::Angry => "😡",
            Self::Care => "🤗",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Like => "J'aime",
            Self::Love => "J'adore",
            Self::Haha => "Haha",
            Self::Wow => "Wouah",
            Self::Sad => "Triste",
            Self::Angry => "Grrr",
            Self::Care => "Solidaire",
        }
    }

    pub fn all() -> &'static [ReactionKind] {
        &[
            Self::Like,
            Self::Love,
            Self::Haha,
            Self::Wow,
            Self::Sad,
            Self::Angry,
            Self::Care,
        ]
    }
}

/// Une reaction = (utilisateur, type, timestamp).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reaction {
    pub user_id: UserId,
    pub kind: ReactionKind,
    pub at: chrono::DateTime<chrono::Utc>,
}

/// Extrait les mentions @user du texte.
pub fn extract_mentions(text: &str) -> Vec<String> {
    let mut mentions = Vec::new();
    let mut chars = text.char_indices().peekable();
    while let Some((i, c)) = chars.next() {
        if c == '@' {
            // Doit etre precede d'un espace ou en debut de chaine
            if i > 0 {
                let prev = text[..i].chars().last().unwrap_or(' ');
                if !prev.is_whitespace() {
                    continue;
                }
            }
            // Capturer le username (alphanum + _ + -)
            let start = i + 1;
            let mut end = start;
            while let Some(&(j, c2)) = chars.peek() {
                if c2.is_alphanumeric() || c2 == '_' || c2 == '-' {
                    end = j + c2.len_utf8();
                    chars.next();
                } else {
                    break;
                }
            }
            if end > start {
                mentions.push(text[start..end].to_string());
            }
        }
    }
    mentions
}

/// Extrait les hashtags #tag (sans le #).
pub fn extract_hashtags(text: &str) -> Vec<String> {
    let mut tags = Vec::new();
    let mut chars = text.char_indices().peekable();
    while let Some((i, c)) = chars.next() {
        if c == '#' {
            if i > 0 {
                let prev = text[..i].chars().last().unwrap_or(' ');
                if !prev.is_whitespace() {
                    continue;
                }
            }
            let start = i + 1;
            let mut end = start;
            while let Some(&(j, c2)) = chars.peek() {
                if c2.is_alphanumeric() || c2 == '_' {
                    end = j + c2.len_utf8();
                    chars.next();
                } else {
                    break;
                }
            }
            if end > start {
                tags.push(text[start..end].to_lowercase());
            }
        }
    }
    tags
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_mentions_simple() {
        let m = extract_mentions("Hello @alice and @bob, how are you?");
        assert_eq!(m, vec!["alice", "bob"]);
    }

    #[test]
    fn extract_mentions_ignores_inline_at() {
        let m = extract_mentions("user@example.com is not a mention");
        assert!(m.is_empty());
    }

    #[test]
    fn extract_hashtags_simple() {
        let t = extract_hashtags("Vive #Miyukini et #JayClub !");
        assert_eq!(t, vec!["miyukini", "jayclub"]);
    }

    #[test]
    fn create_post_extracts_mentions_and_hashtags() {
        let post = Post::new(
            "alice".into(),
            "Hello @bob, check #miyukini #cog".into(),
        );
        assert_eq!(post.mentions, vec!["bob"]);
        assert_eq!(post.hashtags, vec!["miyukini", "cog"]);
    }

    #[test]
    fn reaction_count() {
        let mut post = Post::new("alice".into(), "Test".into());
        post.reactions.push(Reaction {
            user_id: "bob".into(),
            kind: ReactionKind::Like,
            at: chrono::Utc::now(),
        });
        post.reactions.push(Reaction {
            user_id: "carol".into(),
            kind: ReactionKind::Love,
            at: chrono::Utc::now(),
        });
        assert_eq!(post.reaction_count(ReactionKind::Like), 1);
        assert_eq!(post.reaction_count(ReactionKind::Love), 1);
        assert_eq!(post.total_reactions(), 2);
    }
}
