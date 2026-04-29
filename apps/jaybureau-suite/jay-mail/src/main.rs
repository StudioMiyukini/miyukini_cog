//! Jay Mail — client email.

use dioxus::prelude::*;
use jaymail::{Account, Email, ImapConfig, MailboxKind};

fn main() {
    tracing_subscriber::fmt().init();
    let cfg = dioxus::desktop::Config::new().with_window(
        dioxus::desktop::WindowBuilder::new()
            .with_title("Jay Mail")
            .with_inner_size(dioxus::desktop::LogicalSize::new(1280.0, 800.0)),
    );
    dioxus::LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

#[component]
fn App() -> Element {
    let mut current_folder = use_signal(|| MailboxKind::Inbox);
    let mut selected = use_signal(|| None::<u32>);
    let mut composing = use_signal(|| false);

    // Charger les emails (stub — retourne demo)
    let imap_cfg = ImapConfig::miyukini_default("user@miyukini-cog.com".into(), "demo".into());
    let emails = use_hook(move || jaymail::fetch_inbox(&imap_cfg, 50).unwrap_or_default());

    let selected_email = selected
        .read()
        .and_then(|uid| emails.iter().find(|e| e.uid == uid).cloned());

    rsx! {
        style { "
            * {{ margin: 0; padding: 0; box-sizing: border-box; }}
            body, html {{ height: 100%; font-family: 'Segoe UI', sans-serif; background: #f5f5fa; }}
            .mail-app {{ display: flex; height: 100vh; }}
            .mail-sidebar {{ width: 220px; background: #1a1a2e; color: #e0e0e0; padding: 16px 0; }}
            .mail-logo {{ padding: 0 16px 20px 16px; font-size: 16px; font-weight: 700; color: #c2185b; border-bottom: 1px solid #2d2d44; }}
            .mail-compose {{ margin: 16px; padding: 12px; background: #c2185b; color: white; border: none; border-radius: 8px; cursor: pointer; font-weight: 600; width: calc(100% - 32px); }}
            .folder {{ padding: 10px 16px; cursor: pointer; display: flex; gap: 10px; align-items: center; font-size: 14px; }}
            .folder:hover {{ background: #2d2d44; }}
            .folder.active {{ background: #2d2d44; color: #c2185b; font-weight: 600; }}
            .mail-list {{ width: 380px; background: white; overflow-y: auto; border-right: 1px solid #e0e0e8; }}
            .mail-list-header {{ padding: 14px 20px; font-weight: 600; border-bottom: 1px solid #e0e0e8; }}
            .mail-item {{ padding: 14px 20px; border-bottom: 1px solid #f0f0f5; cursor: pointer; }}
            .mail-item:hover {{ background: #f5f5fa; }}
            .mail-item.selected {{ background: #fce4ec; border-left: 3px solid #c2185b; }}
            .mail-item.unread {{ font-weight: 600; }}
            .mail-from {{ font-size: 14px; color: #202124; margin-bottom: 4px; }}
            .mail-subject {{ font-size: 13px; color: #555; margin-bottom: 4px; }}
            .mail-preview {{ font-size: 12px; color: #888; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }}
            .mail-time {{ font-size: 11px; color: #888; float: right; }}
            .mail-reader {{ flex: 1; background: white; overflow-y: auto; padding: 24px 32px; }}
            .reader-empty {{ display: flex; align-items: center; justify-content: center; height: 100%; color: #888; }}
            .reader-subject {{ font-size: 22px; font-weight: 600; margin-bottom: 16px; }}
            .reader-meta {{ padding: 12px 0; border-bottom: 1px solid #e0e0e8; margin-bottom: 16px; }}
            .reader-from {{ font-weight: 600; }}
            .reader-body {{ font-size: 14px; line-height: 1.6; white-space: pre-wrap; }}
            .composer-overlay {{ position: fixed; bottom: 20px; right: 20px; width: 600px; height: 500px; background: white; border-radius: 12px; box-shadow: 0 4px 24px rgba(0,0,0,0.15); display: flex; flex-direction: column; }}
            .composer-header {{ padding: 12px 16px; background: #c2185b; color: white; border-radius: 12px 12px 0 0; display: flex; justify-content: space-between; align-items: center; }}
            .composer-body {{ flex: 1; padding: 16px; display: flex; flex-direction: column; gap: 8px; }}
            .composer-body input, .composer-body textarea {{ padding: 8px 12px; border: 1px solid #e0e0e8; border-radius: 6px; outline: none; font-family: inherit; font-size: 14px; }}
            .composer-body textarea {{ flex: 1; resize: none; }}
            .composer-footer {{ padding: 12px 16px; border-top: 1px solid #e0e0e8; display: flex; gap: 8px; }}
            .btn {{ padding: 8px 16px; background: #c2185b; color: white; border: none; border-radius: 6px; cursor: pointer; }}
            .btn-secondary {{ background: transparent; color: #555; border: 1px solid #e0e0e8; }}
        " }
        div {
            class: "mail-app",
            // Sidebar dossiers
            div {
                class: "mail-sidebar",
                div { class: "mail-logo", "✉️ Jay Mail" }
                button {
                    class: "mail-compose",
                    onclick: move |_| composing.set(true),
                    "+ Nouveau message"
                }
                for kind in MailboxKind::standard() {
                    {
                        let k = *kind;
                        rsx! {
                            div {
                                key: "{kind:?}",
                                class: if *current_folder.read() == k { "folder active" } else { "folder" },
                                onclick: move |_| current_folder.set(k),
                                span { "{kind.icon()}" }
                                span { "{kind.label()}" }
                            }
                        }
                    }
                }
            }

            // Liste des messages
            div {
                class: "mail-list",
                div { class: "mail-list-header", "{current_folder.read().label()}" }
                for email in emails.iter() {
                    EmailItem {
                        key: "{email.uid}",
                        email: email.clone(),
                        selected: *selected.read() == Some(email.uid),
                        on_click: move |uid| selected.set(Some(uid)),
                    }
                }
            }

            // Lecteur
            div {
                class: "mail-reader",
                if let Some(email) = selected_email {
                    {
                        let from_name = email.from.name.clone().unwrap_or(email.from.email.clone());
                        let to_list = email.to.iter().map(|t| t.email.clone()).collect::<Vec<_>>().join(", ");
                        let date_str = email.date.format("%d %B %Y, %H:%M").to_string();
                        rsx! {
                            div { class: "reader-subject", "{email.subject}" }
                            div {
                                class: "reader-meta",
                                div { class: "reader-from", "{from_name}" }
                                div { style: "color: #888; font-size: 13px;", "à {to_list}" }
                                div { style: "color: #888; font-size: 12px; margin-top: 4px;", "{date_str}" }
                            }
                            div { class: "reader-body", "{email.body_text}" }
                        }
                    }
                } else {
                    div { class: "reader-empty", "Sélectionnez un message" }
                }
            }

            // Composer overlay
            if *composing.read() {
                Composer { on_close: move |_| composing.set(false) }
            }
        }
    }
}

#[component]
fn EmailItem(email: Email, selected: bool, on_click: EventHandler<u32>) -> Element {
    let from_name = email
        .from
        .name
        .clone()
        .unwrap_or_else(|| email.from.email.clone());
    let preview = email
        .body_text
        .lines()
        .next()
        .unwrap_or("")
        .chars()
        .take(80)
        .collect::<String>();

    let class = if selected {
        "mail-item selected"
    } else if !email.flags.seen {
        "mail-item unread"
    } else {
        "mail-item"
    };

    let uid = email.uid;
    let time = email.date.format("%H:%M").to_string();

    rsx! {
        div {
            class: "{class}",
            onclick: move |_| on_click.call(uid),
            div {
                span { class: "mail-time", "{time}" }
                div { class: "mail-from", "{from_name}" }
            }
            div { class: "mail-subject", "{email.subject}" }
            div { class: "mail-preview", "{preview}" }
        }
    }
}

#[component]
fn Composer(on_close: EventHandler<()>) -> Element {
    let mut to = use_signal(String::new);
    let mut subject = use_signal(String::new);
    let mut body = use_signal(String::new);

    rsx! {
        div {
            class: "composer-overlay",
            div {
                class: "composer-header",
                span { "Nouveau message" }
                button {
                    style: "background: transparent; border: none; color: white; cursor: pointer; font-size: 18px;",
                    onclick: move |_| on_close.call(()),
                    "✕"
                }
            }
            div {
                class: "composer-body",
                input {
                    placeholder: "Destinataire (email)",
                    value: "{to}",
                    oninput: move |e| to.set(e.value()),
                }
                input {
                    placeholder: "Sujet",
                    value: "{subject}",
                    oninput: move |e| subject.set(e.value()),
                }
                textarea {
                    placeholder: "Votre message...",
                    value: "{body}",
                    oninput: move |e| body.set(e.value()),
                }
            }
            div {
                class: "composer-footer",
                button {
                    class: "btn",
                    onclick: move |_| {
                        tracing::info!("[Demo] Envoi vers {} : {}", to.read(), subject.read());
                        on_close.call(());
                    },
                    "Envoyer"
                }
                button {
                    class: "btn btn-secondary",
                    onclick: move |_| on_close.call(()),
                    "Annuler"
                }
            }
        }
    }
}
