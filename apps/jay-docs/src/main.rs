//! Jay Docs — éditeur de documents collaboratif.
//!
//! MVP : éditeur texte simple avec CRDT Yrs, barre d'outils basique,
//! indicateurs de présence multi-utilisateurs.

use dioxus::prelude::*;
use jay_collab::{apply_update, new_doc};
use std::sync::Arc;
use std::sync::Mutex;
use yrs::{Doc, GetString, Observable, Text, Transact};

mod toolbar;
mod presence_bar;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("jay_docs=debug,info")
        .init();

    let cfg = dioxus::desktop::Config::new().with_window(
        dioxus::desktop::WindowBuilder::new()
            .with_title("Jay Docs — Document sans titre")
            .with_inner_size(dioxus::desktop::LogicalSize::new(1200.0, 800.0)),
    );

    dioxus::LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

/// État partagé de l'éditeur.
#[derive(Clone)]
pub struct EditorCtx {
    pub doc: Arc<Doc>,
    /// Cache du texte actuel (mis à jour après chaque changement).
    pub text_cache: Signal<String>,
    pub title: Signal<String>,
    pub word_count: Signal<u32>,
    /// Nombre de participants en ligne (placeholder).
    pub participants: Signal<u32>,
}

#[component]
fn App() -> Element {
    let doc = use_hook(|| Arc::new(new_doc()));
    let text_cache = use_signal(String::new);
    let title = use_signal(|| "Document sans titre".to_string());
    let word_count = use_signal(|| 0u32);
    let participants = use_signal(|| 1u32);

    let ctx = EditorCtx {
        doc: doc.clone(),
        text_cache,
        title,
        word_count,
        participants,
    };

    use_context_provider(|| ctx.clone());

    // Initialiser le cache depuis le CRDT
    let doc_for_init = doc.clone();
    let mut text_cache_sig = text_cache;
    let mut word_count_sig = word_count;
    use_hook(move || {
        let text = doc_for_init.get_or_insert_text("content");
        let txn = doc_for_init.transact();
        let content = text.get_string(&txn);
        drop(txn);
        let words = content.split_whitespace().count() as u32;
        text_cache_sig.set(content);
        word_count_sig.set(words);
    });

    rsx! {
        style { {include_str!("../assets/style.css")} }
        div {
            class: "doc-app",
            TitleBar { title }
            toolbar::Toolbar {}
            presence_bar::PresenceBar {}
            Editor {}
            StatusBar { word_count, participants }
        }
    }
}

#[component]
fn TitleBar(title: Signal<String>) -> Element {
    let mut title_sig = title;
    rsx! {
        div {
            class: "doc-titlebar",
            span { class: "doc-logo", "📄 Jay Docs" }
            input {
                class: "doc-title-input",
                r#type: "text",
                value: "{title}",
                oninput: move |evt| title_sig.set(evt.value()),
            }
            div {
                class: "doc-actions",
                button { class: "doc-btn-ghost", "Partager" }
                button { class: "doc-btn-primary", "Partager" }
            }
        }
    }
}

#[component]
fn Editor() -> Element {
    let ctx = use_context::<EditorCtx>();
    let doc = ctx.doc.clone();
    let mut text_cache_sig = ctx.text_cache;
    let mut word_count_sig = ctx.word_count;

    rsx! {
        div {
            class: "doc-editor-wrap",
            div {
                class: "doc-page",
                textarea {
                    class: "doc-textarea",
                    placeholder: "Commencez à écrire...",
                    value: "{ctx.text_cache}",
                    oninput: move |evt| {
                        let new_text = evt.value();
                        // Appliquer au CRDT
                        let text = doc.get_or_insert_text("content");
                        let mut txn = doc.transact_mut();
                        let current = text.get_string(&txn);
                        // Remplacement intégral (MVP simple — en production on ferait du diff)
                        text.remove_range(&mut txn, 0, current.chars().count() as u32);
                        text.insert(&mut txn, 0, &new_text);
                        drop(txn);

                        text_cache_sig.set(new_text.clone());
                        word_count_sig.set(new_text.split_whitespace().count() as u32);
                    },
                }
            }
        }
    }
}

#[component]
fn StatusBar(word_count: Signal<u32>, participants: Signal<u32>) -> Element {
    rsx! {
        footer {
            class: "doc-statusbar",
            span { "{word_count} mots" }
            span { class: "sep", "•" }
            span { "{participants} participant(s)" }
            span { class: "sep", "•" }
            span { "Enregistré automatiquement" }
        }
    }
}
