//! JayClub — reseau social Miyukini.

use dioxus::prelude::*;
use jayclub::{JayClubStore, Post, PostVisibility, Profile, ReactionKind, Story, StoryMedia};
use std::sync::Arc;

fn main() {
    tracing_subscriber::fmt().init();
    let cfg = dioxus::desktop::Config::new().with_window(
        dioxus::desktop::WindowBuilder::new()
            .with_title("JayClub")
            .with_inner_size(dioxus::desktop::LogicalSize::new(1200.0, 900.0)),
    );
    dioxus::LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

#[derive(Debug, Clone, PartialEq)]
enum Tab {
    Feed,
    Profile,
    Notifications,
}

/// Wrapper PartialEq pour Arc<JayClubStore> (utilise dans les props Dioxus).
#[derive(Clone)]
struct StoreCtx(Arc<JayClubStore>);

impl PartialEq for StoreCtx {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl std::ops::Deref for StoreCtx {
    type Target = JayClubStore;
    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

#[component]
fn App() -> Element {
    let store = use_hook(|| {
        let store = Arc::new(JayClubStore::new());

        // Demo seed
        store.upsert_profile({
            let mut p = Profile::new("alice".into(), "alice".into());
            p.display_name = "Alice Miyukini".into();
            p.bio = "Creatrice de Miyukini · Tokyo".into();
            p.badges.push(jayclub::ProfileBadge::Founder);
            p
        });
        store.upsert_profile({
            let mut p = Profile::new("bob".into(), "bob".into());
            p.display_name = "Bob".into();
            p.bio = "Dev passionne".into();
            p
        });
        store.upsert_profile({
            let mut p = Profile::new("me".into(), "moi".into());
            p.display_name = "Moi".into();
            p
        });

        // Posts demo
        let mut p1 = Post::new(
            "alice".into(),
            "Bienvenue sur JayClub ! Le réseau social qui respecte votre souveraineté #miyukini #cog".into(),
        );
        p1.visibility = PostVisibility::Public;
        store.create_post(p1);

        let mut p2 = Post::new(
            "bob".into(),
            "Premier post depuis Jay Bureau ! 🚀 #JayClub".into(),
        );
        p2.visibility = PostVisibility::Public;
        store.create_post(p2);

        // Story demo
        store.create_story(Story::new(
            "alice".into(),
            StoryMedia::Text {
                text: "Hello le monde".into(),
                background: "#7c3aed".into(),
                font: "sans-serif".into(),
            },
        ));

        // Suit Alice
        store.follow("me".into(), "alice".into());
        store.follow("me".into(), "bob".into());

        StoreCtx(store)
    });

    let mut tab = use_signal(|| Tab::Feed);
    let current = tab.read().clone();

    rsx! {
        style { "
            * {{ margin: 0; padding: 0; box-sizing: border-box; }}
            body, html {{ height: 100%; font-family: 'Segoe UI', sans-serif; background: #fafafa; color: #262626; }}
            .club-app {{ display: flex; flex-direction: column; height: 100vh; }}
            .club-header {{ display: flex; align-items: center; justify-content: space-between; padding: 12px 24px; background: white; border-bottom: 1px solid #dbdbdb; }}
            .club-logo {{ font-size: 20px; font-weight: 700; color: #c2185b; }}
            .club-tabs {{ display: flex; gap: 24px; }}
            .club-tab {{ padding: 8px 12px; cursor: pointer; color: #555; border-bottom: 2px solid transparent; }}
            .club-tab.active {{ color: #c2185b; border-bottom-color: #c2185b; font-weight: 600; }}
            .club-main {{ flex: 1; overflow-y: auto; padding: 24px; max-width: 720px; margin: 0 auto; width: 100%; }}
            .stories-bar {{ display: flex; gap: 12px; padding: 16px; background: white; border-radius: 12px; margin-bottom: 16px; overflow-x: auto; }}
            .story-thumb {{ width: 64px; min-width: 64px; height: 64px; border-radius: 50%; background: linear-gradient(45deg, #f09433, #e6683c, #dc2743, #cc2366, #bc1888); padding: 3px; cursor: pointer; }}
            .story-thumb-inner {{ width: 100%; height: 100%; border-radius: 50%; background: white; display: flex; align-items: center; justify-content: center; }}
            .post {{ background: white; border-radius: 12px; padding: 16px 20px; margin-bottom: 16px; }}
            .post-header {{ display: flex; align-items: center; gap: 12px; margin-bottom: 12px; }}
            .avatar {{ width: 40px; height: 40px; border-radius: 50%; background: #e0e0e0; display: flex; align-items: center; justify-content: center; font-weight: 600; }}
            .post-author {{ font-weight: 600; }}
            .post-time {{ color: #888; font-size: 12px; }}
            .badge {{ display: inline-block; padding: 1px 6px; border-radius: 8px; font-size: 10px; font-weight: 700; color: white; margin-left: 6px; }}
            .post-content {{ font-size: 15px; line-height: 1.5; margin-bottom: 12px; white-space: pre-wrap; }}
            .post-actions {{ display: flex; gap: 4px; padding-top: 8px; border-top: 1px solid #efefef; }}
            .reaction-btn {{ padding: 6px 12px; background: transparent; border: none; cursor: pointer; font-size: 14px; border-radius: 6px; }}
            .reaction-btn:hover {{ background: #f5f5f5; }}
            .post-stats {{ font-size: 12px; color: #888; padding-top: 4px; }}
            .composer {{ background: white; border-radius: 12px; padding: 16px; margin-bottom: 16px; }}
            .composer textarea {{ width: 100%; border: none; outline: none; resize: none; font-family: inherit; font-size: 15px; min-height: 60px; }}
            .btn-publish {{ padding: 8px 20px; background: #c2185b; color: white; border: none; border-radius: 6px; cursor: pointer; font-weight: 600; float: right; }}
            .hashtag {{ color: #c2185b; font-weight: 500; }}
        " }
        div {
            class: "club-app",
            div {
                class: "club-header",
                div { class: "club-logo", "🌐 JayClub" }
                div {
                    class: "club-tabs",
                    div {
                        class: if current == Tab::Feed { "club-tab active" } else { "club-tab" },
                        onclick: move |_| tab.set(Tab::Feed),
                        "Fil"
                    }
                    div {
                        class: if current == Tab::Profile { "club-tab active" } else { "club-tab" },
                        onclick: move |_| tab.set(Tab::Profile),
                        "Profil"
                    }
                    div {
                        class: if current == Tab::Notifications { "club-tab active" } else { "club-tab" },
                        onclick: move |_| tab.set(Tab::Notifications),
                        "Notifs ({store.unread_count(\"me\")})"
                    }
                }
            }
            div {
                class: "club-main",
                match current {
                    Tab::Feed => rsx! { FeedView { store: store.clone() } },
                    Tab::Profile => rsx! { ProfileView { store: store.clone() } },
                    Tab::Notifications => rsx! { NotificationsView { store: store.clone() } },
                }
            }
        }
    }
}

#[component]
fn FeedView(store: StoreCtx) -> Element {
    let mut composer = use_signal(String::new);
    let store_compose = store.clone();
    let store_render = store.clone();

    let stories = store_render.stories_for("me");
    let posts = store_render.feed_for("me", 20);

    rsx! {
        // Stories bar
        if !stories.is_empty() {
            div {
                class: "stories-bar",
                for story in stories.iter() {
                    div {
                        key: "{story.id}",
                        class: "story-thumb",
                        title: "{story.author_id}",
                        div {
                            class: "story-thumb-inner",
                            "{story.author_id.chars().next().unwrap_or('?').to_uppercase().next().unwrap()}"
                        }
                    }
                }
            }
        }

        // Composer
        div {
            class: "composer",
            textarea {
                placeholder: "Quoi de neuf ? (utilisez @ pour mentionner et # pour les hashtags)",
                value: "{composer}",
                oninput: move |evt| composer.set(evt.value()),
            }
            div {
                style: "display: flex; align-items: center;",
                span { style: "color: #888; font-size: 12px;", "Visible par : Public" }
                button {
                    class: "btn-publish",
                    onclick: move |_| {
                        let text = composer.read().clone();
                        if !text.trim().is_empty() {
                            store_compose.create_post(Post::new("me".into(), text));
                            composer.set(String::new());
                        }
                    },
                    "Publier"
                }
            }
        }

        // Feed
        for post in posts.iter() {
            PostCard { key: "{post.id}", post: post.clone(), store: store_render.clone() }
        }

        if posts.is_empty() {
            div {
                style: "text-align: center; padding: 40px; color: #888;",
                "Aucun post. Suivez des amis ou publiez le premier !"
            }
        }
    }
}

#[component]
fn PostCard(post: Post, store: StoreCtx) -> Element {
    let store_react = store.clone();
    let post_id = post.id.clone();

    let author = store
        .get_profile(&post.author_id)
        .unwrap_or_else(|| Profile::new(post.author_id.clone(), post.author_id.clone()));

    let initial = author
        .display_name
        .chars()
        .next()
        .unwrap_or('?')
        .to_uppercase()
        .next()
        .unwrap();

    let elapsed = chrono::Utc::now() - post.created_at;
    let time_str = if elapsed.num_minutes() < 60 {
        format!("il y a {} min", elapsed.num_minutes().max(1))
    } else if elapsed.num_hours() < 24 {
        format!("il y a {} h", elapsed.num_hours())
    } else {
        format!("il y a {} j", elapsed.num_days())
    };

    let total = post.total_reactions();
    let comments = post.comment_count;

    rsx! {
        div {
            class: "post",
            div {
                class: "post-header",
                div { class: "avatar", "{initial}" }
                div {
                    div {
                        span { class: "post-author", "{author.display_name}" }
                        for badge in author.badges.iter() {
                            span {
                                class: "badge",
                                style: "background: {badge.color()};",
                                "{badge.icon()}"
                            }
                        }
                    }
                    div { class: "post-time", "@{author.username} · {time_str}" }
                }
            }

            div {
                class: "post-content",
                {render_post_content(&post.content)}
            }

            if total > 0 || comments > 0 {
                div {
                    class: "post-stats",
                    "{total} reaction(s) · {comments} commentaire(s)"
                }
            }

            div {
                class: "post-actions",
                for kind in ReactionKind::all() {
                    {
                        let pid = post_id.clone();
                        let store_r = store_react.clone();
                        let k = *kind;
                        rsx! {
                            button {
                                key: "{kind:?}",
                                class: "reaction-btn",
                                title: "{kind.label()}",
                                onclick: move |_| {
                                    store_r.add_reaction(&pid, "me".into(), k).ok();
                                },
                                "{kind.emoji()}"
                            }
                        }
                    }
                }
            }
        }
    }
}

fn render_post_content(text: &str) -> Element {
    // Met les hashtags en couleur
    let parts: Vec<String> = text.split_whitespace().map(String::from).collect();
    rsx! {
        for (i, part) in parts.iter().enumerate() {
            {
                let styled = if part.starts_with('#') || part.starts_with('@') {
                    rsx! { span { class: "hashtag", "{part}" } }
                } else {
                    rsx! { span { "{part}" } }
                };
                rsx! {
                    {styled}
                    if i < parts.len() - 1 { " " }
                }
            }
        }
    }
}

#[component]
fn ProfileView(store: StoreCtx) -> Element {
    let profile = store.get_profile("me").unwrap_or_else(|| Profile::new("me".into(), "moi".into()));
    let posts = store.list_user_posts("me");
    let following = store.list_following("me").len();
    let followers = store.list_followers("me").len();

    rsx! {
        div {
            style: "background: white; border-radius: 12px; padding: 32px; margin-bottom: 16px; text-align: center;",
            div {
                style: "width: 100px; height: 100px; border-radius: 50%; background: linear-gradient(135deg, #c2185b, #7c3aed); margin: 0 auto 16px auto; display: flex; align-items: center; justify-content: center; font-size: 40px; color: white; font-weight: 700;",
                "{profile.display_name.chars().next().unwrap_or('?').to_uppercase().next().unwrap()}"
            }
            h1 { style: "font-size: 22px; margin-bottom: 4px;", "{profile.display_name}" }
            div { style: "color: #888; margin-bottom: 12px;", "@{profile.username}" }
            if !profile.bio.is_empty() {
                p { style: "margin-bottom: 16px;", "{profile.bio}" }
            }
            div {
                style: "display: flex; justify-content: center; gap: 24px; padding-top: 16px; border-top: 1px solid #efefef;",
                div { strong { "{posts.len()}" } " posts" }
                div { strong { "{followers}" } " followers" }
                div { strong { "{following}" } " suivis" }
            }
        }

        h3 { style: "margin-bottom: 12px; color: #555;", "Mes posts" }
        for post in posts.iter() {
            PostCard { key: "{post.id}", post: post.clone(), store: store.clone() }
        }
    }
}

#[component]
fn NotificationsView(store: StoreCtx) -> Element {
    let notifs = store.list_notifications("me");

    rsx! {
        div {
            style: "background: white; border-radius: 12px; padding: 16px;",
            h3 { style: "margin-bottom: 16px;", "Notifications" }
            if notifs.is_empty() {
                div { style: "text-align: center; padding: 24px; color: #888;", "Aucune notification" }
            } else {
                for notif in notifs.iter() {
                    div {
                        key: "{notif.id}",
                        style: "padding: 12px 0; border-bottom: 1px solid #f0f0f0;",
                        {render_notif(&notif.kind)}
                    }
                }
            }
        }
    }
}

fn render_notif(kind: &jayclub::NotificationKind) -> String {
    use jayclub::NotificationKind::*;
    match kind {
        Reaction { from_user_id, .. } => format!("👍 {from_user_id} a réagi à votre post"),
        Comment { from_user_id, .. } => format!("💬 {from_user_id} a commenté votre post"),
        Follow { from_user_id } => format!("➕ {from_user_id} vous suit maintenant"),
        Mention { from_user_id, .. } => format!("@ {from_user_id} vous a mentionné"),
        Share { from_user_id, .. } => format!("🔁 {from_user_id} a partagé votre post"),
        StoryView { from_user_id, .. } => format!("👁 {from_user_id} a vu votre story"),
    }
}
