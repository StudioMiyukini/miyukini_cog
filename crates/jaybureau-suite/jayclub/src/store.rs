//! Store en memoire de JayClub (MVP, a remplacer par KindMother).

use crate::profile::Follow;
use crate::{
    Comment, ItemId, JayClubError, Notification, Post, Profile, Reaction, ReactionKind, Story,
    UserId,
};
use std::collections::HashMap;
use std::sync::RwLock;

/// Store global de JayClub.
pub struct JayClubStore {
    profiles: RwLock<HashMap<UserId, Profile>>,
    posts: RwLock<HashMap<ItemId, Post>>,
    comments: RwLock<HashMap<ItemId, Comment>>,
    stories: RwLock<HashMap<ItemId, Story>>,
    follows: RwLock<Vec<Follow>>,
    notifications: RwLock<HashMap<UserId, Vec<Notification>>>,
}

impl JayClubStore {
    pub fn new() -> Self {
        Self {
            profiles: RwLock::new(HashMap::new()),
            posts: RwLock::new(HashMap::new()),
            comments: RwLock::new(HashMap::new()),
            stories: RwLock::new(HashMap::new()),
            follows: RwLock::new(Vec::new()),
            notifications: RwLock::new(HashMap::new()),
        }
    }

    // ─── Profils ──────────────────────────────────────────────────────

    pub fn upsert_profile(&self, profile: Profile) {
        if let Ok(mut p) = self.profiles.write() {
            p.insert(profile.user_id.clone(), profile);
        }
    }

    pub fn get_profile(&self, user_id: &str) -> Option<Profile> {
        self.profiles.read().ok().and_then(|p| p.get(user_id).cloned())
    }

    // ─── Posts ────────────────────────────────────────────────────────

    pub fn create_post(&self, post: Post) -> ItemId {
        let id = post.id.clone();
        let author_id = post.author_id.clone();
        if let Ok(mut posts) = self.posts.write() {
            posts.insert(id.clone(), post.clone());
        }
        // Incrementer post_count du profil
        if let Ok(mut profiles) = self.profiles.write() {
            if let Some(p) = profiles.get_mut(&author_id) {
                p.post_count += 1;
            }
        }
        // Notifier les utilisateurs mentionnes
        for mention in &post.mentions {
            self.add_notification(
                mention.clone(),
                crate::NotificationKind::Mention {
                    from_user_id: author_id.clone(),
                    post_id: id.clone(),
                },
            );
        }
        id
    }

    pub fn get_post(&self, post_id: &str) -> Option<Post> {
        self.posts.read().ok().and_then(|p| p.get(post_id).cloned())
    }

    pub fn delete_post(&self, post_id: &str, user_id: &str) -> Result<(), JayClubError> {
        let mut posts = self
            .posts
            .write()
            .map_err(|_| JayClubError::Invalid("lock".into()))?;
        let post = posts
            .get(post_id)
            .ok_or_else(|| JayClubError::NotFound(post_id.into()))?;
        if post.author_id != user_id {
            return Err(JayClubError::PermissionDenied);
        }
        posts.remove(post_id);
        Ok(())
    }

    /// Ajoute ou remplace une reaction d'un user sur un post.
    pub fn add_reaction(
        &self,
        post_id: &str,
        user_id: UserId,
        kind: ReactionKind,
    ) -> Result<(), JayClubError> {
        let mut posts = self
            .posts
            .write()
            .map_err(|_| JayClubError::Invalid("lock".into()))?;
        let post = posts
            .get_mut(post_id)
            .ok_or_else(|| JayClubError::NotFound(post_id.into()))?;
        post.reactions.retain(|r| r.user_id != user_id);
        post.reactions.push(Reaction {
            user_id: user_id.clone(),
            kind,
            at: chrono::Utc::now(),
        });
        let author_id = post.author_id.clone();
        drop(posts);

        // Notifier l'auteur du post
        if author_id != user_id {
            self.add_notification(
                author_id,
                crate::NotificationKind::Reaction {
                    from_user_id: user_id,
                    post_id: post_id.to_string(),
                    reaction_kind: format!("{kind:?}").to_lowercase(),
                },
            );
        }
        Ok(())
    }

    /// Retire la reaction d'un user.
    pub fn remove_reaction(&self, post_id: &str, user_id: &str) -> Result<(), JayClubError> {
        let mut posts = self
            .posts
            .write()
            .map_err(|_| JayClubError::Invalid("lock".into()))?;
        let post = posts
            .get_mut(post_id)
            .ok_or_else(|| JayClubError::NotFound(post_id.into()))?;
        post.reactions.retain(|r| r.user_id != user_id);
        Ok(())
    }

    /// Liste tous les posts d'un user (chronologique inverse).
    pub fn list_user_posts(&self, user_id: &str) -> Vec<Post> {
        let posts = self.posts.read().map(|p| {
            let mut v: Vec<Post> = p
                .values()
                .filter(|p| p.author_id == user_id)
                .cloned()
                .collect();
            v.sort_by_key(|p| std::cmp::Reverse(p.created_at));
            v
        }).unwrap_or_default();
        posts
    }

    /// Feed = posts publics ou des personnes suivies.
    pub fn feed_for(&self, user_id: &str, limit: usize) -> Vec<Post> {
        let following = self.list_following(user_id);
        let posts = self.posts.read().map(|p| {
            let mut v: Vec<Post> = p
                .values()
                .filter(|post| {
                    use crate::PostVisibility;
                    match post.visibility {
                        PostVisibility::Public => true,
                        PostVisibility::Followers => {
                            post.author_id == user_id
                                || following.contains(&post.author_id)
                        }
                        PostVisibility::Friends => post.author_id == user_id,
                        PostVisibility::Tribe { .. } => false,
                        PostVisibility::Draft => post.author_id == user_id,
                    }
                })
                .cloned()
                .collect();
            v.sort_by_key(|p| std::cmp::Reverse(p.created_at));
            v.truncate(limit);
            v
        }).unwrap_or_default();
        posts
    }

    // ─── Commentaires ─────────────────────────────────────────────────

    pub fn create_comment(&self, comment: Comment) -> ItemId {
        let id = comment.id.clone();
        let post_id = comment.post_id.clone();
        let author_id = comment.author_id.clone();
        if let Ok(mut comments) = self.comments.write() {
            comments.insert(id.clone(), comment);
        }
        if let Ok(mut posts) = self.posts.write() {
            if let Some(p) = posts.get_mut(&post_id) {
                p.comment_count += 1;
                let post_author = p.author_id.clone();
                drop(posts);
                if post_author != author_id {
                    self.add_notification(
                        post_author,
                        crate::NotificationKind::Comment {
                            from_user_id: author_id,
                            post_id: post_id.clone(),
                            comment_id: id.clone(),
                        },
                    );
                }
            }
        }
        id
    }

    pub fn list_comments(&self, post_id: &str) -> Vec<Comment> {
        let mut v: Vec<Comment> = self
            .comments
            .read()
            .map(|c| {
                c.values()
                    .filter(|c| c.post_id == post_id)
                    .cloned()
                    .collect()
            })
            .unwrap_or_default();
        v.sort_by_key(|c| c.created_at);
        v
    }

    // ─── Stories ──────────────────────────────────────────────────────

    pub fn create_story(&self, story: Story) -> ItemId {
        let id = story.id.clone();
        if let Ok(mut s) = self.stories.write() {
            s.insert(id.clone(), story);
        }
        id
    }

    /// Stories actives (non expirees) des utilisateurs suivis (+ soi-meme).
    pub fn stories_for(&self, user_id: &str) -> Vec<Story> {
        let following = self.list_following(user_id);
        let mut v: Vec<Story> = self
            .stories
            .read()
            .map(|s| {
                s.values()
                    .filter(|story| {
                        !story.is_expired()
                            && (story.author_id == user_id
                                || following.contains(&story.author_id))
                    })
                    .cloned()
                    .collect()
            })
            .unwrap_or_default();
        v.sort_by_key(|s| std::cmp::Reverse(s.created_at));
        v
    }

    /// Nettoyage : supprime les stories expirees. A appeler periodiquement.
    pub fn purge_expired_stories(&self) -> usize {
        let mut count = 0;
        if let Ok(mut s) = self.stories.write() {
            let before = s.len();
            s.retain(|_, story| !story.is_expired());
            count = before - s.len();
        }
        count
    }

    // ─── Follows ──────────────────────────────────────────────────────

    pub fn follow(&self, follower_id: UserId, following_id: UserId) {
        if follower_id == following_id {
            return;
        }
        if let Ok(mut f) = self.follows.write() {
            // Eviter les doublons
            if f.iter().any(|x| x.follower_id == follower_id && x.following_id == following_id) {
                return;
            }
            f.push(Follow {
                follower_id: follower_id.clone(),
                following_id: following_id.clone(),
                at: chrono::Utc::now(),
                notify: true,
            });
        }

        // Mettre a jour les compteurs
        if let Ok(mut profiles) = self.profiles.write() {
            if let Some(p) = profiles.get_mut(&follower_id) {
                p.following_count += 1;
            }
            if let Some(p) = profiles.get_mut(&following_id) {
                p.follower_count += 1;
            }
        }

        self.add_notification(
            following_id,
            crate::NotificationKind::Follow {
                from_user_id: follower_id,
            },
        );
    }

    pub fn unfollow(&self, follower_id: &str, following_id: &str) {
        let removed = if let Ok(mut f) = self.follows.write() {
            let before = f.len();
            f.retain(|x| !(x.follower_id == follower_id && x.following_id == following_id));
            before - f.len()
        } else {
            0
        };
        if removed > 0 {
            if let Ok(mut profiles) = self.profiles.write() {
                if let Some(p) = profiles.get_mut(follower_id) {
                    p.following_count = p.following_count.saturating_sub(1);
                }
                if let Some(p) = profiles.get_mut(following_id) {
                    p.follower_count = p.follower_count.saturating_sub(1);
                }
            }
        }
    }

    pub fn list_followers(&self, user_id: &str) -> Vec<UserId> {
        self.follows
            .read()
            .map(|f| {
                f.iter()
                    .filter(|x| x.following_id == user_id)
                    .map(|x| x.follower_id.clone())
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn list_following(&self, user_id: &str) -> Vec<UserId> {
        self.follows
            .read()
            .map(|f| {
                f.iter()
                    .filter(|x| x.follower_id == user_id)
                    .map(|x| x.following_id.clone())
                    .collect()
            })
            .unwrap_or_default()
    }

    // ─── Notifications ────────────────────────────────────────────────

    pub fn add_notification(&self, user_id: UserId, kind: crate::NotificationKind) {
        let notif = Notification::new(user_id.clone(), kind);
        if let Ok(mut n) = self.notifications.write() {
            n.entry(user_id).or_default().push(notif);
        }
    }

    pub fn list_notifications(&self, user_id: &str) -> Vec<Notification> {
        let mut v: Vec<Notification> = self
            .notifications
            .read()
            .map(|n| n.get(user_id).cloned().unwrap_or_default())
            .unwrap_or_default();
        v.sort_by_key(|n| std::cmp::Reverse(n.created_at));
        v
    }

    pub fn unread_count(&self, user_id: &str) -> usize {
        self.notifications
            .read()
            .map(|n| {
                n.get(user_id)
                    .map(|v| v.iter().filter(|n| !n.read).count())
                    .unwrap_or(0)
            })
            .unwrap_or(0)
    }
}

impl Default for JayClubStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Post, PostVisibility, Profile, ReactionKind};

    #[test]
    fn create_and_list_post() {
        let store = JayClubStore::new();
        store.upsert_profile(Profile::new("alice".into(), "alice".into()));
        let post = Post::new("alice".into(), "Hello world!".into());
        store.create_post(post);
        let posts = store.list_user_posts("alice");
        assert_eq!(posts.len(), 1);
    }

    #[test]
    fn react_to_post_notifies_author() {
        let store = JayClubStore::new();
        store.upsert_profile(Profile::new("alice".into(), "alice".into()));
        store.upsert_profile(Profile::new("bob".into(), "bob".into()));
        let post = Post::new("alice".into(), "Test".into());
        let pid = post.id.clone();
        store.create_post(post);

        store
            .add_reaction(&pid, "bob".into(), ReactionKind::Love)
            .unwrap();

        // Alice doit avoir une notification
        assert_eq!(store.unread_count("alice"), 1);
    }

    #[test]
    fn follow_increments_counts() {
        let store = JayClubStore::new();
        store.upsert_profile(Profile::new("alice".into(), "alice".into()));
        store.upsert_profile(Profile::new("bob".into(), "bob".into()));

        store.follow("alice".into(), "bob".into());
        assert_eq!(store.list_following("alice"), vec!["bob"]);
        assert_eq!(store.list_followers("bob"), vec!["alice"]);

        let alice = store.get_profile("alice").unwrap();
        let bob = store.get_profile("bob").unwrap();
        assert_eq!(alice.following_count, 1);
        assert_eq!(bob.follower_count, 1);
    }

    #[test]
    fn feed_includes_only_followed_for_followers_visibility() {
        let store = JayClubStore::new();
        store.upsert_profile(Profile::new("alice".into(), "alice".into()));
        store.upsert_profile(Profile::new("bob".into(), "bob".into()));
        store.upsert_profile(Profile::new("carol".into(), "carol".into()));

        // Alice suit Bob mais pas Carol
        store.follow("alice".into(), "bob".into());

        let mut bob_post = Post::new("bob".into(), "Bob's post".into());
        bob_post.visibility = PostVisibility::Followers;
        store.create_post(bob_post);

        let mut carol_post = Post::new("carol".into(), "Carol's post".into());
        carol_post.visibility = PostVisibility::Followers;
        store.create_post(carol_post);

        let feed = store.feed_for("alice", 10);
        assert_eq!(feed.len(), 1);
        assert_eq!(feed[0].author_id, "bob");
    }

    #[test]
    fn purge_expired_stories() {
        let store = JayClubStore::new();
        let mut story = crate::Story::new(
            "alice".into(),
            crate::StoryMedia::Text {
                text: "Old".into(),
                background: "#000".into(),
                font: "sans".into(),
            },
        );
        story.expires_at = chrono::Utc::now() - chrono::Duration::seconds(1);
        store.create_story(story);
        let removed = store.purge_expired_stories();
        assert_eq!(removed, 1);
    }
}
