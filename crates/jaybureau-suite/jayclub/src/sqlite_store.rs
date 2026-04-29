//! Backend SQLite pour JayClub — persistence production.
//!
//! Schema :
//! - profiles (user_id PK, json)
//! - posts (id PK, author_id, content, json_full, created_at)
//! - comments (id PK, post_id, author_id, json_full, created_at)
//! - reactions (post_id, user_id, kind, at)  PK (post_id, user_id)
//! - stories (id PK, author_id, json_full, expires_at)
//! - follows (follower_id, following_id)  PK (follower_id, following_id)
//! - notifications (id PK, user_id, json_full, read, created_at)

use crate::profile::Follow;
use crate::{
    Comment, ItemId, JayClubError, Notification, NotificationKind, Post, PostVisibility, Profile,
    Reaction, ReactionKind, Story, UserId,
};
use rusqlite::{params, Connection, OptionalExtension};
use std::path::Path;
use std::sync::Mutex;

/// Store SQLite de JayClub.
pub struct SqliteStore {
    conn: Mutex<Connection>,
}

impl SqliteStore {
    /// Ouvre ou cree la DB.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, JayClubError> {
        let conn = Connection::open(path).map_err(|e| JayClubError::Invalid(e.to_string()))?;
        let store = Self {
            conn: Mutex::new(conn),
        };
        store.migrate()?;
        Ok(store)
    }

    /// Cree une DB en memoire (pour tests).
    pub fn in_memory() -> Result<Self, JayClubError> {
        let conn = Connection::open_in_memory().map_err(|e| JayClubError::Invalid(e.to_string()))?;
        let store = Self {
            conn: Mutex::new(conn),
        };
        store.migrate()?;
        Ok(store)
    }

    fn migrate(&self) -> Result<(), JayClubError> {
        let conn = self.conn.lock().map_err(|_| JayClubError::Invalid("lock".into()))?;
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS profiles (
                user_id TEXT PRIMARY KEY,
                json TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS posts (
                id TEXT PRIMARY KEY,
                author_id TEXT NOT NULL,
                content TEXT NOT NULL,
                visibility TEXT NOT NULL,
                created_at TEXT NOT NULL,
                json_full TEXT NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_posts_author ON posts(author_id);
            CREATE INDEX IF NOT EXISTS idx_posts_created ON posts(created_at DESC);

            CREATE TABLE IF NOT EXISTS comments (
                id TEXT PRIMARY KEY,
                post_id TEXT NOT NULL,
                author_id TEXT NOT NULL,
                created_at TEXT NOT NULL,
                json_full TEXT NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_comments_post ON comments(post_id);

            CREATE TABLE IF NOT EXISTS reactions (
                post_id TEXT NOT NULL,
                user_id TEXT NOT NULL,
                kind TEXT NOT NULL,
                at TEXT NOT NULL,
                PRIMARY KEY (post_id, user_id)
            );
            CREATE INDEX IF NOT EXISTS idx_reactions_post ON reactions(post_id);

            CREATE TABLE IF NOT EXISTS stories (
                id TEXT PRIMARY KEY,
                author_id TEXT NOT NULL,
                expires_at TEXT NOT NULL,
                json_full TEXT NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_stories_expires ON stories(expires_at);
            CREATE INDEX IF NOT EXISTS idx_stories_author ON stories(author_id);

            CREATE TABLE IF NOT EXISTS follows (
                follower_id TEXT NOT NULL,
                following_id TEXT NOT NULL,
                at TEXT NOT NULL,
                notify INTEGER NOT NULL DEFAULT 1,
                PRIMARY KEY (follower_id, following_id)
            );
            CREATE INDEX IF NOT EXISTS idx_follows_follower ON follows(follower_id);
            CREATE INDEX IF NOT EXISTS idx_follows_following ON follows(following_id);

            CREATE TABLE IF NOT EXISTS notifications (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                read INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL,
                json_full TEXT NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_notifs_user ON notifications(user_id);
            "#,
        )
        .map_err(|e| JayClubError::Invalid(format!("migrate: {e}")))?;
        Ok(())
    }

    // ─── Profils ──────────────────────────────────────────────────────

    pub fn upsert_profile(&self, profile: &Profile) -> Result<(), JayClubError> {
        let json = serde_json::to_string(profile)
            .map_err(|e| JayClubError::Invalid(format!("ser profile: {e}")))?;
        let conn = self.conn.lock().map_err(|_| JayClubError::Invalid("lock".into()))?;
        conn.execute(
            "INSERT OR REPLACE INTO profiles (user_id, json) VALUES (?1, ?2)",
            params![profile.user_id, json],
        )
        .map_err(|e| JayClubError::Invalid(format!("upsert profile: {e}")))?;
        Ok(())
    }

    pub fn get_profile(&self, user_id: &str) -> Result<Option<Profile>, JayClubError> {
        let conn = self.conn.lock().map_err(|_| JayClubError::Invalid("lock".into()))?;
        let json_opt: Option<String> = conn
            .query_row(
                "SELECT json FROM profiles WHERE user_id = ?1",
                params![user_id],
                |row| row.get(0),
            )
            .optional()
            .map_err(|e| JayClubError::Invalid(format!("get profile: {e}")))?;
        match json_opt {
            Some(j) => Ok(Some(
                serde_json::from_str(&j)
                    .map_err(|e| JayClubError::Invalid(format!("deser profile: {e}")))?,
            )),
            None => Ok(None),
        }
    }

    // ─── Posts ────────────────────────────────────────────────────────

    pub fn create_post(&self, post: &Post) -> Result<(), JayClubError> {
        let json = serde_json::to_string(post)
            .map_err(|e| JayClubError::Invalid(format!("ser post: {e}")))?;
        let visibility_str = match &post.visibility {
            PostVisibility::Public => "public",
            PostVisibility::Followers => "followers",
            PostVisibility::Friends => "friends",
            PostVisibility::Tribe { .. } => "tribe",
            PostVisibility::Draft => "draft",
        };
        let conn = self.conn.lock().map_err(|_| JayClubError::Invalid("lock".into()))?;
        conn.execute(
            "INSERT INTO posts (id, author_id, content, visibility, created_at, json_full) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                post.id,
                post.author_id,
                post.content,
                visibility_str,
                post.created_at.to_rfc3339(),
                json
            ],
        )
        .map_err(|e| JayClubError::Invalid(format!("insert post: {e}")))?;
        Ok(())
    }

    pub fn get_post(&self, post_id: &str) -> Result<Option<Post>, JayClubError> {
        let conn = self.conn.lock().map_err(|_| JayClubError::Invalid("lock".into()))?;
        let json_opt: Option<String> = conn
            .query_row(
                "SELECT json_full FROM posts WHERE id = ?1",
                params![post_id],
                |row| row.get(0),
            )
            .optional()
            .map_err(|e| JayClubError::Invalid(format!("get post: {e}")))?;
        match json_opt {
            Some(j) => Ok(Some(self.hydrate_post(&j)?)),
            None => Ok(None),
        }
    }

    pub fn delete_post(&self, post_id: &str, user_id: &str) -> Result<(), JayClubError> {
        let conn = self.conn.lock().map_err(|_| JayClubError::Invalid("lock".into()))?;
        let author: Option<String> = conn
            .query_row(
                "SELECT author_id FROM posts WHERE id = ?1",
                params![post_id],
                |row| row.get(0),
            )
            .optional()
            .map_err(|e| JayClubError::Invalid(format!("query: {e}")))?;
        match author {
            Some(a) if a == user_id => {
                conn.execute("DELETE FROM posts WHERE id = ?1", params![post_id])
                    .map_err(|e| JayClubError::Invalid(format!("delete: {e}")))?;
                conn.execute("DELETE FROM reactions WHERE post_id = ?1", params![post_id]).ok();
                conn.execute("DELETE FROM comments WHERE post_id = ?1", params![post_id]).ok();
                Ok(())
            }
            Some(_) => Err(JayClubError::PermissionDenied),
            None => Err(JayClubError::NotFound(post_id.into())),
        }
    }

    pub fn list_user_posts(&self, user_id: &str) -> Result<Vec<Post>, JayClubError> {
        let conn = self.conn.lock().map_err(|_| JayClubError::Invalid("lock".into()))?;
        let mut stmt = conn
            .prepare(
                "SELECT json_full FROM posts WHERE author_id = ?1 ORDER BY created_at DESC LIMIT 100",
            )
            .map_err(|e| JayClubError::Invalid(format!("prepare: {e}")))?;
        let rows = stmt
            .query_map(params![user_id], |row| row.get::<_, String>(0))
            .map_err(|e| JayClubError::Invalid(format!("query: {e}")))?;
        let mut posts = Vec::new();
        for r in rows {
            let json = r.map_err(|e| JayClubError::Invalid(format!("row: {e}")))?;
            posts.push(self.hydrate_post(&json)?);
        }
        Ok(posts)
    }

    /// Feed = posts publics + posts des followings (followers visibility).
    pub fn feed_for(&self, user_id: &str, limit: usize) -> Result<Vec<Post>, JayClubError> {
        let conn = self.conn.lock().map_err(|_| JayClubError::Invalid("lock".into()))?;
        // Construire la liste des authorIds visibles (self + followings + tous public)
        let mut stmt = conn
            .prepare(
                "SELECT json_full FROM posts
                 WHERE visibility = 'public'
                    OR author_id = ?1
                    OR (visibility = 'followers'
                        AND author_id IN (SELECT following_id FROM follows WHERE follower_id = ?1))
                 ORDER BY created_at DESC
                 LIMIT ?2",
            )
            .map_err(|e| JayClubError::Invalid(format!("prepare: {e}")))?;
        let rows = stmt
            .query_map(params![user_id, limit as i64], |row| {
                row.get::<_, String>(0)
            })
            .map_err(|e| JayClubError::Invalid(format!("query: {e}")))?;
        let mut posts = Vec::new();
        for r in rows {
            let json = r.map_err(|e| JayClubError::Invalid(format!("row: {e}")))?;
            posts.push(self.hydrate_post(&json)?);
        }
        Ok(posts)
    }

    /// Ajoute/remplace une reaction d'un user sur un post.
    pub fn add_reaction(
        &self,
        post_id: &str,
        user_id: &str,
        kind: ReactionKind,
    ) -> Result<(), JayClubError> {
        let kind_str = format!("{kind:?}").to_lowercase();
        let now = chrono::Utc::now().to_rfc3339();
        let conn = self.conn.lock().map_err(|_| JayClubError::Invalid("lock".into()))?;
        conn.execute(
            "INSERT OR REPLACE INTO reactions (post_id, user_id, kind, at) VALUES (?1, ?2, ?3, ?4)",
            params![post_id, user_id, kind_str, now],
        )
        .map_err(|e| JayClubError::Invalid(format!("insert reaction: {e}")))?;
        Ok(())
    }

    pub fn remove_reaction(&self, post_id: &str, user_id: &str) -> Result<(), JayClubError> {
        let conn = self.conn.lock().map_err(|_| JayClubError::Invalid("lock".into()))?;
        conn.execute(
            "DELETE FROM reactions WHERE post_id = ?1 AND user_id = ?2",
            params![post_id, user_id],
        )
        .map_err(|e| JayClubError::Invalid(format!("delete reaction: {e}")))?;
        Ok(())
    }

    pub fn list_reactions(&self, post_id: &str) -> Result<Vec<Reaction>, JayClubError> {
        let conn = self.conn.lock().map_err(|_| JayClubError::Invalid("lock".into()))?;
        let mut stmt = conn
            .prepare("SELECT user_id, kind, at FROM reactions WHERE post_id = ?1")
            .map_err(|e| JayClubError::Invalid(format!("prepare: {e}")))?;
        let rows = stmt
            .query_map(params![post_id], |row| {
                let user_id: String = row.get(0)?;
                let kind: String = row.get(1)?;
                let at: String = row.get(2)?;
                Ok((user_id, kind, at))
            })
            .map_err(|e| JayClubError::Invalid(format!("query: {e}")))?;
        let mut reactions = Vec::new();
        for r in rows {
            let (user_id, kind, at) = r.map_err(|e| JayClubError::Invalid(format!("row: {e}")))?;
            let kind = parse_reaction_kind(&kind);
            let at = chrono::DateTime::parse_from_rfc3339(&at)
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now());
            reactions.push(Reaction { user_id, kind, at });
        }
        Ok(reactions)
    }

    // ─── Commentaires ─────────────────────────────────────────────────

    pub fn create_comment(&self, comment: &Comment) -> Result<(), JayClubError> {
        let json = serde_json::to_string(comment)
            .map_err(|e| JayClubError::Invalid(format!("ser comment: {e}")))?;
        let conn = self.conn.lock().map_err(|_| JayClubError::Invalid("lock".into()))?;
        conn.execute(
            "INSERT INTO comments (id, post_id, author_id, created_at, json_full) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                comment.id,
                comment.post_id,
                comment.author_id,
                comment.created_at.to_rfc3339(),
                json
            ],
        )
        .map_err(|e| JayClubError::Invalid(format!("insert comment: {e}")))?;
        Ok(())
    }

    pub fn list_comments(&self, post_id: &str) -> Result<Vec<Comment>, JayClubError> {
        let conn = self.conn.lock().map_err(|_| JayClubError::Invalid("lock".into()))?;
        let mut stmt = conn
            .prepare("SELECT json_full FROM comments WHERE post_id = ?1 ORDER BY created_at ASC")
            .map_err(|e| JayClubError::Invalid(format!("prepare: {e}")))?;
        let rows = stmt
            .query_map(params![post_id], |row| row.get::<_, String>(0))
            .map_err(|e| JayClubError::Invalid(format!("query: {e}")))?;
        let mut comments = Vec::new();
        for r in rows {
            let json = r.map_err(|e| JayClubError::Invalid(format!("row: {e}")))?;
            let c: Comment = serde_json::from_str(&json)
                .map_err(|e| JayClubError::Invalid(format!("deser comment: {e}")))?;
            comments.push(c);
        }
        Ok(comments)
    }

    // ─── Stories ──────────────────────────────────────────────────────

    pub fn create_story(&self, story: &Story) -> Result<(), JayClubError> {
        let json = serde_json::to_string(story)
            .map_err(|e| JayClubError::Invalid(format!("ser story: {e}")))?;
        let conn = self.conn.lock().map_err(|_| JayClubError::Invalid("lock".into()))?;
        conn.execute(
            "INSERT INTO stories (id, author_id, expires_at, json_full) VALUES (?1, ?2, ?3, ?4)",
            params![
                story.id,
                story.author_id,
                story.expires_at.to_rfc3339(),
                json
            ],
        )
        .map_err(|e| JayClubError::Invalid(format!("insert story: {e}")))?;
        Ok(())
    }

    pub fn stories_for(&self, user_id: &str) -> Result<Vec<Story>, JayClubError> {
        let now = chrono::Utc::now().to_rfc3339();
        let conn = self.conn.lock().map_err(|_| JayClubError::Invalid("lock".into()))?;
        let mut stmt = conn
            .prepare(
                "SELECT json_full FROM stories
                 WHERE expires_at > ?1
                   AND (author_id = ?2
                        OR author_id IN (SELECT following_id FROM follows WHERE follower_id = ?2))
                 ORDER BY json_extract(json_full, '$.created_at') DESC",
            )
            .map_err(|e| JayClubError::Invalid(format!("prepare: {e}")))?;
        let rows = stmt
            .query_map(params![now, user_id], |row| row.get::<_, String>(0))
            .map_err(|e| JayClubError::Invalid(format!("query: {e}")))?;
        let mut stories = Vec::new();
        for r in rows {
            let json = r.map_err(|e| JayClubError::Invalid(format!("row: {e}")))?;
            let s: Story = serde_json::from_str(&json)
                .map_err(|e| JayClubError::Invalid(format!("deser story: {e}")))?;
            stories.push(s);
        }
        Ok(stories)
    }

    pub fn purge_expired_stories(&self) -> Result<usize, JayClubError> {
        let now = chrono::Utc::now().to_rfc3339();
        let conn = self.conn.lock().map_err(|_| JayClubError::Invalid("lock".into()))?;
        let removed = conn
            .execute("DELETE FROM stories WHERE expires_at <= ?1", params![now])
            .map_err(|e| JayClubError::Invalid(format!("purge: {e}")))?;
        Ok(removed)
    }

    // ─── Follows ──────────────────────────────────────────────────────

    pub fn follow(&self, follower_id: &str, following_id: &str) -> Result<(), JayClubError> {
        if follower_id == following_id {
            return Ok(());
        }
        let now = chrono::Utc::now().to_rfc3339();
        let conn = self.conn.lock().map_err(|_| JayClubError::Invalid("lock".into()))?;
        conn.execute(
            "INSERT OR IGNORE INTO follows (follower_id, following_id, at, notify) VALUES (?1, ?2, ?3, 1)",
            params![follower_id, following_id, now],
        )
        .map_err(|e| JayClubError::Invalid(format!("follow: {e}")))?;
        Ok(())
    }

    pub fn unfollow(&self, follower_id: &str, following_id: &str) -> Result<(), JayClubError> {
        let conn = self.conn.lock().map_err(|_| JayClubError::Invalid("lock".into()))?;
        conn.execute(
            "DELETE FROM follows WHERE follower_id = ?1 AND following_id = ?2",
            params![follower_id, following_id],
        )
        .map_err(|e| JayClubError::Invalid(format!("unfollow: {e}")))?;
        Ok(())
    }

    pub fn list_followers(&self, user_id: &str) -> Result<Vec<UserId>, JayClubError> {
        let conn = self.conn.lock().map_err(|_| JayClubError::Invalid("lock".into()))?;
        let mut stmt = conn
            .prepare("SELECT follower_id FROM follows WHERE following_id = ?1")
            .map_err(|e| JayClubError::Invalid(format!("prepare: {e}")))?;
        let rows = stmt
            .query_map(params![user_id], |row| row.get::<_, String>(0))
            .map_err(|e| JayClubError::Invalid(format!("query: {e}")))?;
        Ok(rows.filter_map(|r| r.ok()).collect())
    }

    pub fn list_following(&self, user_id: &str) -> Result<Vec<UserId>, JayClubError> {
        let conn = self.conn.lock().map_err(|_| JayClubError::Invalid("lock".into()))?;
        let mut stmt = conn
            .prepare("SELECT following_id FROM follows WHERE follower_id = ?1")
            .map_err(|e| JayClubError::Invalid(format!("prepare: {e}")))?;
        let rows = stmt
            .query_map(params![user_id], |row| row.get::<_, String>(0))
            .map_err(|e| JayClubError::Invalid(format!("query: {e}")))?;
        Ok(rows.filter_map(|r| r.ok()).collect())
    }

    // ─── Notifications ────────────────────────────────────────────────

    pub fn add_notification(&self, user_id: &str, kind: NotificationKind) -> Result<(), JayClubError> {
        let notif = Notification::new(user_id.to_string(), kind);
        let json = serde_json::to_string(&notif)
            .map_err(|e| JayClubError::Invalid(format!("ser notif: {e}")))?;
        let conn = self.conn.lock().map_err(|_| JayClubError::Invalid("lock".into()))?;
        conn.execute(
            "INSERT INTO notifications (id, user_id, read, created_at, json_full) VALUES (?1, ?2, 0, ?3, ?4)",
            params![notif.id, user_id, notif.created_at.to_rfc3339(), json],
        )
        .map_err(|e| JayClubError::Invalid(format!("insert notif: {e}")))?;
        Ok(())
    }

    pub fn list_notifications(&self, user_id: &str) -> Result<Vec<Notification>, JayClubError> {
        let conn = self.conn.lock().map_err(|_| JayClubError::Invalid("lock".into()))?;
        let mut stmt = conn
            .prepare(
                "SELECT json_full FROM notifications WHERE user_id = ?1 ORDER BY created_at DESC LIMIT 100",
            )
            .map_err(|e| JayClubError::Invalid(format!("prepare: {e}")))?;
        let rows = stmt
            .query_map(params![user_id], |row| row.get::<_, String>(0))
            .map_err(|e| JayClubError::Invalid(format!("query: {e}")))?;
        let mut notifs = Vec::new();
        for r in rows {
            let json = r.map_err(|e| JayClubError::Invalid(format!("row: {e}")))?;
            let n: Notification = serde_json::from_str(&json)
                .map_err(|e| JayClubError::Invalid(format!("deser notif: {e}")))?;
            notifs.push(n);
        }
        Ok(notifs)
    }

    pub fn unread_count(&self, user_id: &str) -> Result<usize, JayClubError> {
        let conn = self.conn.lock().map_err(|_| JayClubError::Invalid("lock".into()))?;
        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM notifications WHERE user_id = ?1 AND read = 0",
                params![user_id],
                |row| row.get(0),
            )
            .map_err(|e| JayClubError::Invalid(format!("count: {e}")))?;
        Ok(count as usize)
    }

    pub fn mark_notification_read(&self, notif_id: &str) -> Result<(), JayClubError> {
        let conn = self.conn.lock().map_err(|_| JayClubError::Invalid("lock".into()))?;
        conn.execute(
            "UPDATE notifications SET read = 1 WHERE id = ?1",
            params![notif_id],
        )
        .map_err(|e| JayClubError::Invalid(format!("update: {e}")))?;
        Ok(())
    }

    // ─── Helpers ──────────────────────────────────────────────────────

    fn hydrate_post(&self, json: &str) -> Result<Post, JayClubError> {
        let mut post: Post = serde_json::from_str(json)
            .map_err(|e| JayClubError::Invalid(format!("deser post: {e}")))?;
        // Recharger reactions/comment_count depuis les tables
        post.reactions = self.list_reactions(&post.id)?;
        Ok(post)
    }
}

fn parse_reaction_kind(s: &str) -> ReactionKind {
    match s {
        "love" => ReactionKind::Love,
        "haha" => ReactionKind::Haha,
        "wow" => ReactionKind::Wow,
        "sad" => ReactionKind::Sad,
        "angry" => ReactionKind::Angry,
        "care" => ReactionKind::Care,
        _ => ReactionKind::Like,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fresh_store() -> SqliteStore {
        SqliteStore::in_memory().unwrap()
    }

    #[test]
    fn upsert_get_profile() {
        let store = fresh_store();
        store.upsert_profile(&Profile::new("alice".into(), "alice".into())).unwrap();
        let p = store.get_profile("alice").unwrap().unwrap();
        assert_eq!(p.user_id, "alice");
    }

    #[test]
    fn create_and_list_posts() {
        let store = fresh_store();
        store.upsert_profile(&Profile::new("alice".into(), "alice".into())).unwrap();
        let post = Post::new("alice".into(), "Hello world!".into());
        store.create_post(&post).unwrap();
        let posts = store.list_user_posts("alice").unwrap();
        assert_eq!(posts.len(), 1);
        assert_eq!(posts[0].content, "Hello world!");
    }

    #[test]
    fn reactions_persist() {
        let store = fresh_store();
        let post = Post::new("alice".into(), "Test".into());
        store.create_post(&post).unwrap();
        store.add_reaction(&post.id, "bob", ReactionKind::Love).unwrap();
        store.add_reaction(&post.id, "carol", ReactionKind::Like).unwrap();

        let got = store.get_post(&post.id).unwrap().unwrap();
        assert_eq!(got.reactions.len(), 2);
        assert_eq!(got.reaction_count(ReactionKind::Love), 1);
    }

    #[test]
    fn follows_and_feed() {
        let store = fresh_store();
        store.upsert_profile(&Profile::new("alice".into(), "alice".into())).unwrap();
        store.upsert_profile(&Profile::new("bob".into(), "bob".into())).unwrap();
        store.follow("alice", "bob").unwrap();

        let mut bob_post = Post::new("bob".into(), "Bob's post".into());
        bob_post.visibility = PostVisibility::Followers;
        store.create_post(&bob_post).unwrap();

        // Alice doit voir le post de Bob (qu'elle suit)
        let feed = store.feed_for("alice", 10).unwrap();
        assert_eq!(feed.len(), 1);

        // Carol qui ne suit personne ne le voit pas
        let feed = store.feed_for("carol", 10).unwrap();
        assert_eq!(feed.len(), 0);
    }

    #[test]
    fn delete_post_only_owner() {
        let store = fresh_store();
        let post = Post::new("alice".into(), "Test".into());
        store.create_post(&post).unwrap();

        assert!(store.delete_post(&post.id, "bob").is_err());
        assert!(store.get_post(&post.id).unwrap().is_some());

        store.delete_post(&post.id, "alice").unwrap();
        assert!(store.get_post(&post.id).unwrap().is_none());
    }

    #[test]
    fn purge_expired_stories() {
        let store = fresh_store();
        let mut story = Story::new(
            "alice".into(),
            crate::StoryMedia::Text {
                text: "Old".into(),
                background: "#000".into(),
                font: "sans".into(),
            },
        );
        story.expires_at = chrono::Utc::now() - chrono::Duration::seconds(1);
        store.create_story(&story).unwrap();

        let removed = store.purge_expired_stories().unwrap();
        assert_eq!(removed, 1);
    }

    #[test]
    fn notifications_unread_count() {
        let store = fresh_store();
        store.add_notification(
            "alice",
            crate::NotificationKind::Follow {
                from_user_id: "bob".into(),
            },
        ).unwrap();
        store.add_notification(
            "alice",
            crate::NotificationKind::Follow {
                from_user_id: "carol".into(),
            },
        ).unwrap();
        assert_eq!(store.unread_count("alice").unwrap(), 2);

        let notifs = store.list_notifications("alice").unwrap();
        store.mark_notification_read(&notifs[0].id).unwrap();
        assert_eq!(store.unread_count("alice").unwrap(), 1);
    }
}
