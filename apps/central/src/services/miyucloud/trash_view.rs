//! Vue corbeille MiyuCloud -- liste des elements supprimes.
//!
//! @id: miyucloud_trash_view
//! @do: display_and_manage_trashed_items
//! @role: component
//! @layer: presentation
//! @human: Lise (Dev Front-End)
//!
//! Affiche les fichiers et dossiers en corbeille avec :
//! - Date de mise en corbeille
//! - Boutons Restaurer / Supprimer definitivement / Vider la corbeille
//! - Confirmation avant suppression definitive

use dioxus::prelude::*;

use crate::state::use_app_state;
use super::client::MiyuCloudClient;
use super::components::{ActionButton, FileIcon, format_size};
use super::state::MiyuCloudState;

/// Vue corbeille complete avec liste et actions.
#[component]
pub fn TrashView(
    state: Signal<MiyuCloudState>,
    client: Signal<Option<MiyuCloudClient>>,
) -> Element {
    let c = use_app_state().read().current_theme.palette();

    let files = state.read().current_contents.files.clone();
    let folders = state.read().current_contents.subfolders.clone();
    let confirm_purge = state.read().confirm_purge_id.clone();
    let error_msg = state.read().error_message.clone();
    let loading = state.read().loading;

    let is_empty = files.is_empty() && folders.is_empty();
    let total_items = files.len() + folders.len();
    let total_size: u64 = files.iter().map(|f| f.size_bytes).sum();
    let total_label = format_size(total_size);
    let count_label = format!("{total_items} element(s) en corbeille - {total_label}");

    rsx! {
        div {
            style: "flex: 1; display: flex; flex-direction: column; overflow: hidden; min-width: 0;",

            // Header
            div {
                style: "padding: 14px 16px; border-bottom: 1px solid {c.border}; display: flex; align-items: center; gap: 12px;",

                span { style: "font-size: 22px;", "\u{1F5D1}" }
                h3 { style: "font-size: 16px; color: {c.text_white}; margin: 0; flex: 1;", "Corbeille" }

                if !is_empty {
                    ActionButton {
                        label: "Vider la corbeille".to_string(),
                        icon: "\u{26A0}".to_string(),
                        danger: true,
                        small: true,
                        onclick: move |_| {
                            // Demander confirmation pour vider la corbeille
                            state.write().confirm_purge_id = Some("__all__".to_string());
                        },
                    }
                }
            }

            // Error message
            if let Some(ref err) = error_msg {
                div {
                    style: "padding: 8px 16px; background: {c.accent_red}20; border-left: 3px solid {c.accent_red}; margin: 8px 16px 0; border-radius: 4px; color: {c.accent_red}; font-size: 13px;",
                    "{err}"
                    button {
                        style: "margin-left: 12px; background: none; border: none; color: {c.accent_red}; cursor: pointer; font-size: 16px;",
                        onclick: move |_| state.write().error_message = None,
                        "\u{2715}"
                    }
                }
            }

            // Loading
            if loading {
                div {
                    style: "padding: 24px; text-align: center; color: {c.text_secondary}; font-size: 14px;",
                    "Chargement..."
                }
            }

            // Content
            div {
                style: "flex: 1; overflow-y: auto; padding: 8px 0;",

                if !loading && is_empty {
                    // Empty state
                    div {
                        style: "display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 60px 24px; color: {c.text_muted};",
                        span { style: "font-size: 56px; margin-bottom: 16px; opacity: 0.5;", "\u{1F5D1}" }
                        p { style: "font-size: 16px; margin: 0 0 4px;", "La corbeille est vide" }
                        p { style: "font-size: 13px;", "Les fichiers supprimes apparaitront ici." }
                    }
                }

                if !loading && !is_empty {
                    // Table header
                    div {
                        style: "display: grid; grid-template-columns: 1fr 100px 140px 140px 160px; gap: 8px; padding: 8px 16px; border-bottom: 1px solid {c.border}; font-size: 11px; color: {c.text_muted}; text-transform: uppercase; letter-spacing: 0.5px;",
                        span { "Nom" }
                        span { "Taille" }
                        span { "Type" }
                        span { "Supprime le" }
                        span { style: "text-align: right;", "Actions" }
                    }

                    // Folders
                    for folder in folders.iter() {
                        {
                            let folder_id = folder.id.clone();
                            let folder_name = folder.name.clone();
                            let trashed_at = folder.trashed_at.clone().unwrap_or_default();
                            let display_date = format_trash_date(&trashed_at);
                            let fid_restore = folder_id.clone();
                            let fid_purge = folder_id.clone();

                            rsx! {
                                div {
                                    style: "display: grid; grid-template-columns: 1fr 100px 140px 140px 160px; gap: 8px; padding: 8px 16px; border-bottom: 1px solid {c.border}10; align-items: center; transition: background 0.1s;",

                                    // Name
                                    div {
                                        style: "display: flex; align-items: center; gap: 8px; overflow: hidden;",
                                        FileIcon { mime_type: String::new(), is_folder: true, size: 18 }
                                        span {
                                            style: "font-size: 13px; color: {c.text_primary}; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; opacity: 0.7;",
                                            "{folder_name}"
                                        }
                                    }

                                    // Size
                                    span { style: "color: {c.text_muted}; font-size: 12px;", "--" }

                                    // Type
                                    span { style: "color: {c.text_muted}; font-size: 12px;", "Dossier" }

                                    // Trashed at
                                    span { style: "color: {c.text_secondary}; font-size: 12px;", "{display_date}" }

                                    // Actions
                                    div {
                                        style: "display: flex; gap: 6px; justify-content: flex-end;",
                                        button {
                                            style: "padding: 4px 10px; background: {c.accent_blue}; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 11px;",
                                            aria_label: "Restaurer le dossier",
                                            onclick: move |_| {
                                                trigger_restore(fid_restore.clone(), client, state);
                                            },
                                            "Restaurer"
                                        }
                                        button {
                                            style: "padding: 4px 10px; background: {c.accent_red}; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 11px;",
                                            aria_label: "Supprimer definitivement le dossier",
                                            onclick: move |_| {
                                                state.write().confirm_purge_id = Some(fid_purge.clone());
                                            },
                                            "Supprimer"
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Files
                    for file in files.iter() {
                        {
                            let file_id = file.id.clone();
                            let file_name = file.name.clone();
                            let file_mime = file.mime_type.clone();
                            let file_size = file.size_bytes;
                            let trashed_at = file.trashed_at.clone().unwrap_or_default();
                            let display_date = format_trash_date(&trashed_at);
                            let size_label = format_size(file_size);
                            let fid_restore = file_id.clone();
                            let fid_purge = file_id.clone();
                            let mime_label = file_mime.clone();

                            rsx! {
                                div {
                                    style: "display: grid; grid-template-columns: 1fr 100px 140px 140px 160px; gap: 8px; padding: 8px 16px; border-bottom: 1px solid {c.border}10; align-items: center; transition: background 0.1s;",

                                    // Name
                                    div {
                                        style: "display: flex; align-items: center; gap: 8px; overflow: hidden;",
                                        FileIcon { mime_type: file_mime, is_folder: false, size: 18 }
                                        span {
                                            style: "font-size: 13px; color: {c.text_primary}; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; opacity: 0.7;",
                                            "{file_name}"
                                        }
                                    }

                                    // Size
                                    span { style: "color: {c.text_secondary}; font-size: 12px;", "{size_label}" }

                                    // Type
                                    span {
                                        style: "color: {c.text_secondary}; font-size: 12px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;",
                                        "{mime_label}"
                                    }

                                    // Trashed at
                                    span { style: "color: {c.text_secondary}; font-size: 12px;", "{display_date}" }

                                    // Actions
                                    div {
                                        style: "display: flex; gap: 6px; justify-content: flex-end;",
                                        button {
                                            style: "padding: 4px 10px; background: {c.accent_blue}; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 11px;",
                                            aria_label: "Restaurer le fichier",
                                            onclick: move |_| {
                                                trigger_restore(fid_restore.clone(), client, state);
                                            },
                                            "Restaurer"
                                        }
                                        button {
                                            style: "padding: 4px 10px; background: {c.accent_red}; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 11px;",
                                            aria_label: "Supprimer definitivement le fichier",
                                            onclick: move |_| {
                                                state.write().confirm_purge_id = Some(fid_purge.clone());
                                            },
                                            "Supprimer"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Status bar
            if !is_empty {
                div {
                    style: "padding: 6px 16px; border-top: 1px solid {c.border}; font-size: 11px; color: {c.text_muted};",
                    "{count_label}"
                }
            }

            // Confirmation dialog
            if let Some(ref purge_id) = confirm_purge {
                PurgeConfirmDialog {
                    state: state,
                    client: client,
                    target_id: purge_id.clone(),
                }
            }
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// Confirmation dialog
// ════════════════════════════════════════════════════════════════════════════

/// Dialog de confirmation avant suppression definitive.
#[component]
fn PurgeConfirmDialog(
    state: Signal<MiyuCloudState>,
    client: Signal<Option<MiyuCloudClient>>,
    target_id: String,
) -> Element {
    let c = use_app_state().read().current_theme.palette();

    let is_all = target_id == "__all__";
    let message = if is_all {
        "Voulez-vous vider la corbeille ? Tous les elements seront supprimes definitivement. Cette action est irreversible.".to_string()
    } else {
        "Voulez-vous supprimer definitivement cet element ? Cette action est irreversible.".to_string()
    };
    let title = if is_all {
        "Vider la corbeille"
    } else {
        "Supprimer definitivement"
    };

    rsx! {
        div {
            style: "position: fixed; inset: 0; z-index: 1100; display: flex; align-items: center; justify-content: center; background: rgba(0,0,0,0.65);",
            role: "alertdialog",
            aria_modal: "true",
            aria_label: "{title}",
            onclick: move |_| state.write().confirm_purge_id = None,

            div {
                style: "background: {c.bg_card}; border-radius: 8px; padding: 24px; min-width: 340px; max-width: 420px; border: 1px solid {c.border}; box-shadow: 0 8px 32px rgba(0,0,0,0.5);",
                onclick: move |evt| evt.stop_propagation(),

                // Warning icon
                div {
                    style: "text-align: center; margin-bottom: 12px;",
                    span { style: "font-size: 36px;", "\u{26A0}" }
                }

                h3 {
                    style: "margin: 0 0 12px; color: {c.accent_red}; font-size: 16px; text-align: center;",
                    "{title}"
                }

                p {
                    style: "margin: 0 0 20px; color: {c.text_secondary}; font-size: 13px; text-align: center; line-height: 1.5;",
                    "{message}"
                }

                div {
                    style: "display: flex; justify-content: center; gap: 10px;",

                    button {
                        style: "padding: 8px 20px; background: {c.bg_hover}; color: {c.text_primary}; border: 1px solid {c.border}; border-radius: 4px; cursor: pointer; font-size: 13px;",
                        onclick: move |_| state.write().confirm_purge_id = None,
                        "Annuler"
                    }
                    button {
                        style: "padding: 8px 20px; background: {c.accent_red}; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 13px; font-weight: 600;",
                        onclick: {
                            let tid = target_id.clone();
                            move |_| {
                                state.write().confirm_purge_id = None;
                                if tid == "__all__" {
                                    trigger_empty_trash(client, state);
                                } else {
                                    trigger_purge(tid.clone(), client, state);
                                }
                            }
                        },
                        "Supprimer definitivement"
                    }
                }
            }
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// Actions async
// ════════════════════════════════════════════════════════════════════════════

/// Restaure un element depuis la corbeille.
fn trigger_restore(
    id: String,
    client: Signal<Option<MiyuCloudClient>>,
    mut state: Signal<MiyuCloudState>,
) {
    spawn(async move {
        let http = {
            let c = client.read();
            c.clone()
        };
        let Some(http) = http else { return };

        match http.restore_from_trash(&id).await {
            Ok(()) => {
                // Retirer l'element du contenu de la corbeille
                let mut contents = state.read().current_contents.clone();
                contents.files.retain(|f| f.id != id);
                contents.subfolders.retain(|f| f.id != id);
                state.write().current_contents = contents;
            }
            Err(e) => {
                state.write().error_message = Some(format!("Echec restauration : {e}"));
            }
        }
    });
}

/// Supprime definitivement un element de la corbeille.
fn trigger_purge(
    id: String,
    client: Signal<Option<MiyuCloudClient>>,
    mut state: Signal<MiyuCloudState>,
) {
    spawn(async move {
        let http = {
            let c = client.read();
            c.clone()
        };
        let Some(http) = http else { return };

        match http.purge_from_trash(&id).await {
            Ok(()) => {
                let mut contents = state.read().current_contents.clone();
                contents.files.retain(|f| f.id != id);
                contents.subfolders.retain(|f| f.id != id);
                state.write().current_contents = contents;
            }
            Err(e) => {
                state.write().error_message = Some(format!("Echec suppression : {e}"));
            }
        }
    });
}

/// Vide la corbeille entierement.
fn trigger_empty_trash(
    client: Signal<Option<MiyuCloudClient>>,
    mut state: Signal<MiyuCloudState>,
) {
    spawn(async move {
        let http = {
            let c = client.read();
            c.clone()
        };
        let Some(http) = http else { return };

        match http.empty_trash().await {
            Ok(()) => {
                let mut contents = state.read().current_contents.clone();
                contents.files.clear();
                contents.subfolders.clear();
                state.write().current_contents = contents;
            }
            Err(e) => {
                state.write().error_message = Some(format!("Echec vidage corbeille : {e}"));
            }
        }
    });
}

/// Formate une date ISO 8601 pour l'affichage dans la corbeille.
fn format_trash_date(iso: &str) -> String {
    if iso.len() >= 16 {
        let date_part = &iso[..10];
        let time_part = &iso[11..16];
        let parts: Vec<&str> = date_part.split('-').collect();
        if parts.len() == 3 {
            return format!("{}/{}/{} {time_part}", parts[2], parts[1], parts[0]);
        }
    }
    if iso.is_empty() {
        return "Inconnue".to_string();
    }
    iso.to_string()
}
