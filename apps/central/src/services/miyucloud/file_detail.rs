//! Panel de detail fichier MiyuCloud.
//!
//! @id: miyucloud_file_detail
//! @do: display_file_metadata_and_actions
//! @role: component
//! @layer: presentation
//!
//! Affiche les metadonnees d'un fichier selectionne et propose
//! des actions : telecharger, supprimer, renommer, partager (Phase B).

use dioxus::prelude::*;

use super::components::{format_size, ActionButton, FileIcon};
use super::state::{FileEntry, MiyuCloudState};
use crate::state::use_app_state;

/// Panel de detail d'un fichier selectionne.
/// S'affiche a droite de l'explorateur quand un fichier est selectionne.
#[component]
pub fn FileDetailPanel(
    state: Signal<MiyuCloudState>,
    file: FileEntry,
    on_download: EventHandler<String>,
    on_delete: EventHandler<String>,
    on_rename: EventHandler<String>,
) -> Element {
    let c = use_app_state().read().current_theme.palette();

    let file_name = file.name.clone();
    let file_id = file.id.clone();
    let file_mime = file.mime_type.clone();
    let file_size = file.size_bytes;
    let file_checksum = file.checksum_sha256.clone();
    let file_created = format_date_detail(&file.created_at);
    let file_updated = format_date_detail(&file.updated_at);
    let chunk_count = file.chunk_count;
    let is_encrypted = file.encryption_iv.is_some();

    let size_label = format_size(file_size);
    let encryption_label = if is_encrypted {
        "Chiffre (ChaCha20-Poly1305)"
    } else {
        "Non chiffre"
    };
    let chunk_label = format!("{chunk_count} chunk(s)");

    // Checksum tronque pour l'affichage
    let checksum_short = if file_checksum.len() > 16 {
        format!("{}...", &file_checksum[..16])
    } else {
        file_checksum.clone()
    };

    let rename_dialog = state.read().rename_dialog;
    let rename_value = state.read().rename_value.clone();

    let fid_download = file_id.clone();
    let fid_delete = file_id.clone();
    let fid_rename = file_id.clone();

    rsx! {
        div {
            style: "width: 280px; min-width: 280px; background: {c.bg_secondary}; border-left: 1px solid {c.border}; display: flex; flex-direction: column; overflow-y: auto;",

            // En-tete
            div {
                style: "padding: 16px; border-bottom: 1px solid {c.border}; display: flex; align-items: center; justify-content: space-between;",
                h3 {
                    style: "font-size: 14px; color: {c.text_white}; margin: 0;",
                    "Details"
                }
                button {
                    style: "background: none; border: none; color: {c.text_muted}; cursor: pointer; font-size: 16px; padding: 4px;",
                    onclick: move |_| state.write().selected_file = None,
                    "\u{2715}"
                }
            }

            // Icone et nom
            div {
                style: "padding: 20px 16px; display: flex; flex-direction: column; align-items: center; gap: 12px; border-bottom: 1px solid {c.border};",
                FileIcon { mime_type: file_mime.clone(), is_folder: false, size: 56 }
                p {
                    style: "font-size: 14px; color: {c.text_primary}; text-align: center; word-break: break-all; margin: 0;",
                    "{file_name}"
                }
                span {
                    style: "font-size: 12px; color: {c.text_secondary};",
                    "{size_label}"
                }
            }

            // Metadonnees
            div {
                style: "padding: 16px; display: flex; flex-direction: column; gap: 10px; border-bottom: 1px solid {c.border};",

                DetailRow { label: "Type", value: file_mime }
                DetailRow { label: "Taille", value: size_label }
                DetailRow { label: "Cree le", value: file_created }
                DetailRow { label: "Modifie le", value: file_updated }
                DetailRow { label: "Chiffrement", value: encryption_label.to_string() }
                DetailRow { label: "Chunks", value: chunk_label }
                DetailRow { label: "SHA-256", value: checksum_short }
            }

            // Actions
            div {
                style: "padding: 16px; display: flex; flex-direction: column; gap: 8px;",

                ActionButton {
                    label: "Telecharger".to_string(),
                    icon: "\u{2B07}".to_string(),
                    primary: true,
                    onclick: move |_| on_download.call(fid_download.clone()),
                }
                ActionButton {
                    label: "Renommer".to_string(),
                    icon: "\u{270F}".to_string(),
                    onclick: move |_| {
                        let name = state.read().selected_file.as_ref().map(|f| f.name.clone()).unwrap_or_default();
                        state.write().rename_dialog = true;
                        state.write().rename_value = name;
                    },
                }
                ActionButton {
                    label: "Partager".to_string(),
                    icon: "\u{1F517}".to_string(),
                    onclick: move |_| {
                        let fid = state.read().selected_file.as_ref().map(|f| f.id.clone());
                        state.write().share_target_file_id = fid;
                        state.write().share_target_folder_id = None;
                        state.write().generated_share_url = None;
                        state.write().share_dialog_open = true;
                    },
                }
                ActionButton {
                    label: "Supprimer".to_string(),
                    icon: "\u{1F5D1}".to_string(),
                    danger: true,
                    onclick: move |_| on_delete.call(fid_delete.clone()),
                }
            }

            // Dialog de renommage
            if rename_dialog {
                RenameDialog {
                    state: state,
                    current_name: rename_value,
                    file_id: fid_rename,
                    on_rename: on_rename,
                }
            }
        }
    }
}

/// Ligne de detail (label: valeur).
#[component]
fn DetailRow(label: &'static str, value: String) -> Element {
    let c = use_app_state().read().current_theme.palette();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 2px;",
            span {
                style: "font-size: 11px; color: {c.text_muted}; text-transform: uppercase; letter-spacing: 0.3px;",
                "{label}"
            }
            span {
                style: "font-size: 13px; color: {c.text_primary}; word-break: break-all;",
                "{value}"
            }
        }
    }
}

/// Dialog de renommage de fichier.
#[component]
fn RenameDialog(
    state: Signal<MiyuCloudState>,
    current_name: String,
    file_id: String,
    on_rename: EventHandler<String>,
) -> Element {
    let c = use_app_state().read().current_theme.palette();

    rsx! {
        div {
            style: "position: fixed; inset: 0; z-index: 1000; display: flex; align-items: center; justify-content: center; background: rgba(0,0,0,0.6);",
            onclick: move |_| state.write().rename_dialog = false,

            div {
                style: "background: {c.bg_card}; border-radius: 8px; padding: 24px; min-width: 320px; border: 1px solid {c.border}; box-shadow: 0 8px 32px rgba(0,0,0,0.4);",
                onclick: move |evt| evt.stop_propagation(),

                h3 { style: "margin: 0 0 16px; color: {c.text_white}; font-size: 16px;", "Renommer" }

                input {
                    style: "width: 100%; padding: 10px 12px; background: {c.bg_secondary}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_primary}; font-size: 14px; box-sizing: border-box; outline: none;",
                    r#type: "text",
                    value: "{current_name}",
                    oninput: move |evt| {
                        state.write().rename_value = evt.value();
                    },
                }

                div {
                    style: "display: flex; justify-content: flex-end; gap: 8px; margin-top: 16px;",

                    button {
                        style: "padding: 8px 16px; background: {c.bg_hover}; color: {c.text_primary}; border: 1px solid {c.border}; border-radius: 4px; cursor: pointer; font-size: 13px;",
                        onclick: move |_| state.write().rename_dialog = false,
                        "Annuler"
                    }
                    button {
                        style: "padding: 8px 16px; background: {c.accent_blue}; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 13px;",
                        onclick: {
                            let file_id = file_id.clone();
                            move |_| {
                                state.write().rename_dialog = false;
                                on_rename.call(file_id.clone());
                            }
                        },
                        "Renommer"
                    }
                }
            }
        }
    }
}

/// Formate une date ISO 8601 en format lisible detaille.
fn format_date_detail(iso: &str) -> String {
    if iso.len() >= 19 {
        let date_part = &iso[..10];
        let time_part = &iso[11..19];
        let parts: Vec<&str> = date_part.split('-').collect();
        if parts.len() == 3 {
            return format!("{}/{}/{} {time_part}", parts[2], parts[1], parts[0]);
        }
    } else if iso.len() >= 16 {
        let date_part = &iso[..10];
        let time_part = &iso[11..16];
        let parts: Vec<&str> = date_part.split('-').collect();
        if parts.len() == 3 {
            return format!("{}/{}/{} {time_part}", parts[2], parts[1], parts[0]);
        }
    }
    iso.to_string()
}
