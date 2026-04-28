//! Jay Message — messagerie chiffree E2E.

use dioxus::prelude::*;
use jaymessage::{Conversation, ConversationKind, IdentityKey, Message, MessageStatus, SessionStore};
use miyukini_cog_bridge::e2e::E2eKeypair;
use std::sync::Arc;

fn main() {
    tracing_subscriber::fmt().init();
    let cfg = dioxus::desktop::Config::new().with_window(
        dioxus::desktop::WindowBuilder::new()
            .with_title("Jay Message")
            .with_inner_size(dioxus::desktop::LogicalSize::new(1100.0, 800.0)),
    );
    dioxus::LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

#[component]
fn App() -> Element {
    let state = use_hook(|| {
        // Identite locale + sessions demo
        let identity = IdentityKey::generate("me".into());
        let sessions = Arc::new(SessionStore::new());

        // Demo : etablir 2 sessions chiffrees avec Alice et Bob
        let alice_kp = E2eKeypair::generate();
        let me_kp1 = E2eKeypair::generate();
        let alice_pub = alice_kp.public_key_bytes();
        let me_pub1 = me_kp1.public_key_bytes();
        let _ = alice_kp.derive_shared_secret(&me_pub1);
        let me_secret = me_kp1.derive_shared_secret(&alice_pub).unwrap();
        sessions.register("conv-alice".into(), "alice".into(), me_secret);

        let bob_kp = E2eKeypair::generate();
        let me_kp2 = E2eKeypair::generate();
        let bob_pub = bob_kp.public_key_bytes();
        let me_pub2 = me_kp2.public_key_bytes();
        let me_secret2 = me_kp2.derive_shared_secret(&bob_pub).unwrap();
        sessions.register("conv-bob".into(), "bob".into(), me_secret2);

        AppState {
            identity: Arc::new(identity),
            sessions,
            conversations: vec![
                Conv {
                    id: "conv-alice".into(),
                    name: "Alice".into(),
                    peer_id: "alice".into(),
                    kind: ConversationKind::Direct,
                    last_preview: "Salut !".into(),
                },
                Conv {
                    id: "conv-bob".into(),
                    name: "Bob".into(),
                    peer_id: "bob".into(),
                    kind: ConversationKind::Direct,
                    last_preview: "On se voit lundi ?".into(),
                },
            ],
        }
    });

    let mut selected = use_signal(|| Some("conv-alice".to_string()));
    let mut messages = use_signal(|| {
        vec![
            DemoMsg {
                from: "alice".into(),
                text: "Salut !".into(),
                ts: chrono::Utc::now() - chrono::Duration::minutes(15),
                me: false,
            },
            DemoMsg {
                from: "me".into(),
                text: "Hey ! Comment tu vas ?".into(),
                ts: chrono::Utc::now() - chrono::Duration::minutes(14),
                me: true,
            },
            DemoMsg {
                from: "alice".into(),
                text: "Très bien ! Et toi ?".into(),
                ts: chrono::Utc::now() - chrono::Duration::minutes(13),
                me: false,
            },
        ]
    });
    let mut composer = use_signal(String::new);

    let current_conv = state
        .conversations
        .iter()
        .find(|c| Some(&c.id) == selected.read().as_ref())
        .cloned();
    let pubkey = state.identity.public().public_key.clone();

    rsx! {
        style { "
            * {{ margin: 0; padding: 0; box-sizing: border-box; }}
            body, html {{ height: 100%; font-family: 'Segoe UI', sans-serif; background: #1a1a2e; color: #e0e0e0; }}
            .msg-app {{ display: flex; height: 100vh; }}
            .msg-sidebar {{ width: 320px; background: #13132a; border-right: 1px solid #2d2d44; display: flex; flex-direction: column; }}
            .msg-sidebar-header {{ padding: 16px; border-bottom: 1px solid #2d2d44; }}
            .msg-logo {{ font-size: 16px; font-weight: 700; color: #22c55e; margin-bottom: 4px; }}
            .msg-identity {{ font-size: 10px; color: #666; font-family: monospace; word-break: break-all; }}
            .conv-item {{ padding: 14px 16px; border-bottom: 1px solid #2d2d44; cursor: pointer; }}
            .conv-item:hover {{ background: #1f1f3a; }}
            .conv-item.active {{ background: #22c55e22; border-left: 3px solid #22c55e; }}
            .conv-name {{ font-weight: 600; margin-bottom: 4px; }}
            .conv-preview {{ font-size: 12px; color: #888; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }}
            .conv-encrypted {{ display: inline-block; padding: 2px 6px; background: #22c55e; color: white; border-radius: 4px; font-size: 9px; font-weight: 700; margin-left: 6px; }}
            .msg-main {{ flex: 1; display: flex; flex-direction: column; }}
            .msg-header {{ padding: 14px 20px; background: #13132a; border-bottom: 1px solid #2d2d44; display: flex; align-items: center; gap: 12px; }}
            .msg-header-name {{ font-weight: 600; font-size: 16px; }}
            .msg-header-status {{ font-size: 11px; color: #22c55e; }}
            .msg-stage {{ flex: 1; overflow-y: auto; padding: 24px; display: flex; flex-direction: column; gap: 8px; }}
            .bubble {{ max-width: 70%; padding: 10px 14px; border-radius: 16px; font-size: 14px; line-height: 1.4; }}
            .bubble.me {{ background: #22c55e; color: white; align-self: flex-end; border-bottom-right-radius: 4px; }}
            .bubble.them {{ background: #2d2d44; color: #e0e0e0; align-self: flex-start; border-bottom-left-radius: 4px; }}
            .bubble-time {{ font-size: 10px; opacity: 0.7; margin-top: 4px; }}
            .msg-composer {{ padding: 16px; background: #13132a; border-top: 1px solid #2d2d44; display: flex; gap: 8px; }}
            .msg-composer input {{ flex: 1; padding: 10px 14px; background: #1a1a2e; border: 1px solid #2d2d44; color: #e0e0e0; border-radius: 20px; outline: none; font-family: inherit; }}
            .msg-send {{ padding: 10px 20px; background: #22c55e; color: white; border: none; border-radius: 20px; cursor: pointer; font-weight: 600; }}
            .empty-state {{ display: flex; align-items: center; justify-content: center; flex: 1; color: #666; }}
        " }
        div {
            class: "msg-app",
            // Sidebar
            div {
                class: "msg-sidebar",
                div {
                    class: "msg-sidebar-header",
                    div { class: "msg-logo", "🔒 Jay Message" }
                    div { class: "msg-identity", "Ma cle: {pubkey.chars().take(20).collect::<String>()}..." }
                }
                for conv in state.conversations.iter() {
                    {
                        let cid = conv.id.clone();
                        let cid_active = selected.read().as_ref() == Some(&conv.id);
                        rsx! {
                            div {
                                key: "{conv.id}",
                                class: if cid_active { "conv-item active" } else { "conv-item" },
                                onclick: move |_| selected.set(Some(cid.clone())),
                                div {
                                    class: "conv-name",
                                    "{conv.name}"
                                    span { class: "conv-encrypted", "🔒 E2E" }
                                }
                                div { class: "conv-preview", "{conv.last_preview}" }
                            }
                        }
                    }
                }
            }

            // Stage principal
            div {
                class: "msg-main",
                if let Some(conv) = current_conv {
                    div {
                        class: "msg-header",
                        div { class: "avatar", style: "width: 40px; height: 40px; border-radius: 50%; background: #22c55e; display: flex; align-items: center; justify-content: center; font-weight: 600; color: white;",
                            "{conv.name.chars().next().unwrap_or('?').to_uppercase().next().unwrap()}"
                        }
                        div {
                            div { class: "msg-header-name", "{conv.name}" }
                            div { class: "msg-header-status", "🔒 Chiffré de bout en bout (X25519 + ChaCha20-Poly1305)" }
                        }
                    }
                    div {
                        class: "msg-stage",
                        for (i, msg) in messages.read().iter().enumerate() {
                            {
                                let time = msg.ts.format("%H:%M").to_string();
                                let cls = if msg.me { "bubble me" } else { "bubble them" };
                                rsx! {
                                    div {
                                        key: "{i}",
                                        class: "{cls}",
                                        "{msg.text}"
                                        div { class: "bubble-time", "{time}" }
                                    }
                                }
                            }
                        }
                    }
                    div {
                        class: "msg-composer",
                        input {
                            placeholder: "Message chiffré...",
                            value: "{composer}",
                            oninput: move |e| composer.set(e.value()),
                            onkeydown: move |evt| {
                                if evt.key() == dioxus::prelude::Key::Enter {
                                    let text = composer.read().clone();
                                    if !text.trim().is_empty() {
                                        messages.write().push(DemoMsg {
                                            from: "me".into(),
                                            text: text.clone(),
                                            ts: chrono::Utc::now(),
                                            me: true,
                                        });
                                        composer.set(String::new());
                                    }
                                }
                            },
                        }
                        button {
                            class: "msg-send",
                            onclick: move |_| {
                                let text = composer.read().clone();
                                if !text.trim().is_empty() {
                                    messages.write().push(DemoMsg {
                                        from: "me".into(),
                                        text,
                                        ts: chrono::Utc::now(),
                                        me: true,
                                    });
                                    composer.set(String::new());
                                }
                            },
                            "Envoyer"
                        }
                    }
                } else {
                    div { class: "empty-state", "Sélectionnez une conversation" }
                }
            }
        }
    }
}

#[derive(Clone)]
struct AppState {
    identity: Arc<IdentityKey>,
    sessions: Arc<SessionStore>,
    conversations: Vec<Conv>,
}

#[derive(Clone, PartialEq)]
struct Conv {
    id: String,
    name: String,
    peer_id: String,
    kind: ConversationKind,
    last_preview: String,
}

#[derive(Clone)]
struct DemoMsg {
    from: String,
    text: String,
    ts: chrono::DateTime<chrono::Utc>,
    me: bool,
}
