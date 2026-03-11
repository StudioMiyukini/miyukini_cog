use crate::state::{MipowerCtx, Notification, View};
use dioxus::prelude::*;

#[component]
pub fn Settings(
    mut view: Signal<View>,
    mut notification: Signal<Option<Notification>>,
) -> Element {
    let mut ctx     = use_context::<MipowerCtx>();
    let mut mip_root_input = use_signal(|| ctx.mip_root.read().clone());
    let mut status  = use_signal(|| String::new());

    rsx! {
        div { class: "view-content",
            div { class: "view-header",
                h1 { "Parametres" }
            }
            div { class: "settings-form",
                label {
                    span { class: "field-label", "Chemin du workspace MIP (.mip/)" }
                    div { class: "input-row",
                        input {
                            r#type: "text",
                            class: "text-input",
                            placeholder: "ex: C:\\Users\\miyuk\\Cursor\\Miyukini-COG",
                            value: "{mip_root_input}",
                            oninput: move |e| mip_root_input.set(e.value()),
                        }
                        button {
                            class: "btn-primary",
                            onclick: move |_| {
                                let root = mip_root_input.read().trim().to_string();
                                if root.is_empty() {
                                    status.set("Chemin vide.".into());
                                    return;
                                }
                                if !std::path::Path::new(&root).exists() {
                                    status.set(format!("Chemin introuvable : {root}"));
                                    return;
                                }
                                *ctx.mip_root.write() = root.clone();
                                // Persister dans APPDATA
                                if let Some(appdata) = std::env::var_os("APPDATA") {
                                    let dir = std::path::PathBuf::from(appdata).join("mipower");
                                    let _ = std::fs::create_dir_all(&dir);
                                    let _ = std::fs::write(dir.join("mip_root.txt"), &root);
                                }
                                status.set("Chemin enregistre.".into());
                                notification.set(Some(Notification { message: "mip_root mis a jour".into(), is_error: false }));
                            },
                            "Enregistrer"
                        }
                    }
                    if !status.read().is_empty() {
                        span { class: "settings-status", "{status}" }
                    }
                }

                div { class: "settings-section",
                    h3 { "Actions" }
                    button {
                        class: "btn-secondary",
                        onclick: move |_| {
                            let root = ctx.mip_root.read().clone();
                            spawn(async move {
                                let mip_root = std::path::Path::new(&root);
                                match crate::init::rebuild_index(mip_root) {
                                    Ok(()) => status.set("Index regenere.".into()),
                                    Err(e) => status.set(format!("Erreur : {e}")),
                                }
                            });
                        },
                        "Regenerer sequences/index.json"
                    }
                }

                div { class: "settings-section settings-info",
                    h3 { "Informations" }
                    p { "MIPOWER v0.2 — Application desktop native (Dioxus 0.7)" }
                    p { "Pas de navigateur, pas de npm, pas de dependances TypeScript." }
                    p { "Markdown rendu par pulldown-cmark cote Rust." }
                }
            }
        }
    }
}
