//! Explorateur de fichiers MiyuCloud -- vue principale.
//!
//! @id: miyucloud_explorer
//! @do: display_files_and_folders
//! @role: component
//! @layer: presentation
//!
//! Vue principale du cloud : grille/liste de fichiers avec tri, navigation
//! dans les dossiers, breadcrumb, boutons Upload et Nouveau dossier.

use dioxus::prelude::*;

use super::components::{format_size, ActionButton, Breadcrumb, FileIcon, SizeLabel};
use super::state::{CloudSection, FileEntry, FolderEntry, MiyuCloudState, SortField, ViewMode};
use crate::state::use_app_state;

/// Explorateur de fichiers (zone principale droite).
#[component]
pub fn FileExplorer(state: Signal<MiyuCloudState>, on_upload: EventHandler<()>) -> Element {
    let c = use_app_state().read().current_theme.palette();

    let section = state.read().section;
    let view_mode = state.read().view_mode;
    let sort_by = state.read().sort_by;
    let sort_asc = state.read().sort_asc;
    let breadcrumb = state.read().breadcrumb.clone();
    let loading = state.read().loading;
    let error_msg = state.read().error_message.clone();
    let new_folder_open = state.read().new_folder_dialog;
    let new_folder_name = state.read().new_folder_name.clone();

    // Lire et trier les contenus
    let subfolders = state.read().current_contents.subfolders.clone();
    let files = state.read().current_contents.files.clone();
    let sorted_files = sort_files(&files, sort_by, sort_asc);
    let sorted_folders = sort_folders(&subfolders, sort_by, sort_asc);

    let is_trash = section == CloudSection::Trash;
    let section_title = if is_trash { "Corbeille" } else { "" };

    let grid_active_bg = if view_mode == ViewMode::Grid {
        c.accent_blue
    } else {
        c.bg_hover
    };
    let list_active_bg = if view_mode == ViewMode::List {
        c.accent_blue
    } else {
        c.bg_hover
    };
    let grid_color = if view_mode == ViewMode::Grid {
        c.text_white
    } else {
        c.text_secondary
    };
    let list_color = if view_mode == ViewMode::List {
        c.text_white
    } else {
        c.text_secondary
    };

    let total_items = sorted_files.len() + sorted_folders.len();
    let total_size: u64 = sorted_files.iter().map(|f| f.size_bytes).sum();
    let total_label = format_size(total_size);
    let count_label = format!("{total_items} element(s) - {total_label}");

    rsx! {
        div {
            style: "flex: 1; display: flex; flex-direction: column; overflow: hidden; min-width: 0;",

            // Header : breadcrumb + actions
            div {
                style: "padding: 12px 16px; border-bottom: 1px solid {c.border}; display: flex; flex-direction: column; gap: 8px;",

                // Ligne 1 : Breadcrumb ou titre Corbeille
                if is_trash {
                    div {
                        style: "display: flex; align-items: center; gap: 8px;",
                        span { style: "font-size: 20px;", "\u{1F5D1}" }
                        h3 { style: "font-size: 16px; color: {c.text_white}; margin: 0;", "{section_title}" }
                    }
                } else {
                    Breadcrumb {
                        items: breadcrumb,
                        on_navigate: move |folder_id: Option<String>| {
                            state.write().current_folder_id = folder_id;
                            state.write().selected_file = None;
                        },
                    }
                }

                // Ligne 2 : actions
                div {
                    style: "display: flex; align-items: center; gap: 8px;",

                    if !is_trash {
                        ActionButton {
                            label: "Upload".to_string(),
                            icon: "\u{2B06}".to_string(),
                            primary: true,
                            onclick: move |_| on_upload.call(()),
                        }
                        ActionButton {
                            label: "Nouveau dossier".to_string(),
                            icon: "\u{1F4C1}".to_string(),
                            onclick: move |_| {
                                state.write().new_folder_dialog = true;
                                state.write().new_folder_name = String::new();
                            },
                        }
                    }

                    // Spacer
                    div { style: "flex: 1;" }

                    // Toggle grille/liste
                    div {
                        style: "display: flex; gap: 2px; background: {c.bg_secondary}; border-radius: 4px; padding: 2px;",

                        button {
                            style: "padding: 4px 8px; background: {grid_active_bg}; color: {grid_color}; border: none; border-radius: 3px; cursor: pointer; font-size: 14px;",
                            onclick: move |_| state.write().view_mode = ViewMode::Grid,
                            title: "Grille",
                            "\u{25A6}"
                        }
                        button {
                            style: "padding: 4px 8px; background: {list_active_bg}; color: {list_color}; border: none; border-radius: 3px; cursor: pointer; font-size: 14px;",
                            onclick: move |_| state.write().view_mode = ViewMode::List,
                            title: "Liste",
                            "\u{2630}"
                        }
                    }

                    // Tri
                    SortDropdown { state: state }
                }
            }

            // Erreur
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

            // Modal nouveau dossier
            if new_folder_open {
                NewFolderDialog { state: state, folder_name: new_folder_name }
            }

            // Chargement
            if loading {
                div {
                    style: "padding: 24px; text-align: center; color: {c.text_secondary}; font-size: 14px;",
                    "Chargement..."
                }
            }

            // Contenu principal : grille ou liste
            div {
                style: "flex: 1; overflow-y: auto; padding: 16px;",

                if !loading && sorted_folders.is_empty() && sorted_files.is_empty() {
                    // Etat vide
                    div {
                        style: "display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 48px; color: {c.text_muted};",
                        span { style: "font-size: 48px; margin-bottom: 16px;", "\u{2601}" }
                        p { style: "font-size: 16px; margin: 0 0 8px;",
                            if is_trash { "La corbeille est vide" } else { "Ce dossier est vide" }
                        }
                        if !is_trash {
                            p { style: "font-size: 13px;", "Uploade des fichiers ou cree un dossier pour commencer." }
                        }
                    }
                } else {
                    match view_mode {
                        ViewMode::Grid => rsx! {
                            GridView {
                                state: state,
                                folders: sorted_folders,
                                files: sorted_files,
                                is_trash: is_trash,
                            }
                        },
                        ViewMode::List => rsx! {
                            ListView {
                                state: state,
                                folders: sorted_folders,
                                files: sorted_files,
                                is_trash: is_trash,
                            }
                        },
                    }
                }
            }

            // Barre de statut
            div {
                style: "padding: 6px 16px; border-top: 1px solid {c.border}; font-size: 11px; color: {c.text_muted};",
                "{count_label}"
            }
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// GridView
// ════════════════════════════════════════════════════════════════════════════

#[component]
fn GridView(
    state: Signal<MiyuCloudState>,
    folders: Vec<FolderEntry>,
    files: Vec<FileEntry>,
    is_trash: bool,
) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let selected_id = state.read().selected_file.as_ref().map(|f| f.id.clone());

    rsx! {
        div {
            style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(140px, 1fr)); gap: 12px;",

            // Dossiers
            for folder in folders.iter() {
                {
                    let fid = folder.id.clone();
                    let fname = folder.name.clone();

                    rsx! {
                        div {
                            style: "display: flex; flex-direction: column; align-items: center; gap: 6px; padding: 16px 8px; background: {c.bg_secondary}; border-radius: 8px; cursor: pointer; border: 1px solid transparent; transition: all 0.15s;",
                            ondoubleclick: move |_| {
                                if !is_trash {
                                    state.write().current_folder_id = Some(fid.clone());
                                    state.write().selected_file = None;
                                }
                            },
                            FileIcon { mime_type: String::new(), is_folder: true, size: 40 }
                            span {
                                style: "font-size: 12px; color: {c.text_primary}; text-align: center; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; max-width: 120px;",
                                "{fname}"
                            }
                        }
                    }
                }
            }

            // Fichiers
            for file in files.iter() {
                {
                    let file_clone = file.clone();
                    let fid = file.id.clone();
                    let is_selected = selected_id.as_deref() == Some(fid.as_str());
                    let border_color = if is_selected { c.accent_blue } else { "transparent" };
                    let file_name = file.name.clone();
                    let file_mime = file.mime_type.clone();
                    let file_size = file.size_bytes;

                    rsx! {
                        div {
                            style: "display: flex; flex-direction: column; align-items: center; gap: 6px; padding: 16px 8px; background: {c.bg_secondary}; border-radius: 8px; cursor: pointer; border: 1px solid {border_color}; transition: all 0.15s;",
                            onclick: move |_| {
                                state.write().selected_file = Some(file_clone.clone());
                            },
                            FileIcon { mime_type: file_mime, is_folder: false, size: 40 }
                            span {
                                style: "font-size: 12px; color: {c.text_primary}; text-align: center; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; max-width: 120px;",
                                "{file_name}"
                            }
                            SizeLabel { bytes: file_size, compact: true }
                        }
                    }
                }
            }
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// ListView
// ════════════════════════════════════════════════════════════════════════════

#[component]
fn ListView(
    state: Signal<MiyuCloudState>,
    folders: Vec<FolderEntry>,
    files: Vec<FileEntry>,
    is_trash: bool,
) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let selected_id = state.read().selected_file.as_ref().map(|f| f.id.clone());

    rsx! {
        div {
            style: "display: flex; flex-direction: column;",

            // En-tete du tableau
            div {
                style: "display: grid; grid-template-columns: 1fr 100px 120px 140px; gap: 8px; padding: 8px 12px; border-bottom: 1px solid {c.border}; font-size: 11px; color: {c.text_muted}; text-transform: uppercase; letter-spacing: 0.5px;",
                span { "Nom" }
                span { "Taille" }
                span { "Type" }
                span { "Modifie" }
            }

            // Dossiers
            for folder in folders.iter() {
                {
                    let fid = folder.id.clone();
                    let fname = folder.name.clone();
                    let fdate = folder.updated_at.clone();
                    let display_date = format_date(&fdate);

                    rsx! {
                        div {
                            style: "display: grid; grid-template-columns: 1fr 100px 120px 140px; gap: 8px; padding: 8px 12px; border-bottom: 1px solid {c.border}10; cursor: pointer; transition: background 0.1s; align-items: center;",
                            ondoubleclick: move |_| {
                                if !is_trash {
                                    state.write().current_folder_id = Some(fid.clone());
                                    state.write().selected_file = None;
                                }
                            },
                            div {
                                style: "display: flex; align-items: center; gap: 8px; overflow: hidden;",
                                FileIcon { mime_type: String::new(), is_folder: true, size: 18 }
                                span {
                                    style: "font-size: 13px; color: {c.text_primary}; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;",
                                    "{fname}"
                                }
                            }
                            span { style: "color: {c.text_muted}; font-size: 12px;", "--" }
                            span { style: "color: {c.text_muted}; font-size: 12px;", "Dossier" }
                            span { style: "color: {c.text_secondary}; font-size: 12px;", "{display_date}" }
                        }
                    }
                }
            }

            // Fichiers
            for file in files.iter() {
                {
                    let file_clone = file.clone();
                    let fid = file.id.clone();
                    let is_selected = selected_id.as_deref() == Some(fid.as_str());
                    let row_bg = if is_selected {
                        format!("{}30", c.accent_blue)
                    } else {
                        "transparent".to_string()
                    };
                    let file_name = file.name.clone();
                    let file_mime = file.mime_type.clone();
                    let file_mime_label = file.mime_type.clone();
                    let file_size = file.size_bytes;
                    let file_date = file.updated_at.clone();
                    let display_date = format_date(&file_date);

                    rsx! {
                        div {
                            style: "display: grid; grid-template-columns: 1fr 100px 120px 140px; gap: 8px; padding: 8px 12px; border-bottom: 1px solid {c.border}10; cursor: pointer; transition: background 0.1s; align-items: center; background: {row_bg};",
                            onclick: move |_| {
                                state.write().selected_file = Some(file_clone.clone());
                            },
                            div {
                                style: "display: flex; align-items: center; gap: 8px; overflow: hidden;",
                                FileIcon { mime_type: file_mime, is_folder: false, size: 18 }
                                span {
                                    style: "font-size: 13px; color: {c.text_primary}; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;",
                                    "{file_name}"
                                }
                            }
                            SizeLabel { bytes: file_size, compact: true }
                            span {
                                style: "color: {c.text_secondary}; font-size: 12px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;",
                                "{file_mime_label}"
                            }
                            span { style: "color: {c.text_secondary}; font-size: 12px;", "{display_date}" }
                        }
                    }
                }
            }
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// SortDropdown
// ════════════════════════════════════════════════════════════════════════════

#[component]
fn SortDropdown(state: Signal<MiyuCloudState>) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let sort_by = state.read().sort_by;
    let sort_asc = state.read().sort_asc;

    let sort_label = match sort_by {
        SortField::Name => "Nom",
        SortField::Size => "Taille",
        SortField::Date => "Date",
        SortField::Type => "Type",
    };
    let arrow = if sort_asc { "\u{2191}" } else { "\u{2193}" };

    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 4px;",

            span { style: "font-size: 11px; color: {c.text_muted};", "Tri:" }

            button {
                style: "padding: 4px 8px; background: {c.bg_secondary}; color: {c.text_secondary}; border: 1px solid {c.border}; border-radius: 4px; cursor: pointer; font-size: 12px;",
                onclick: move |_| {
                    let current = state.read().sort_by;
                    let next = match current {
                        SortField::Name => SortField::Size,
                        SortField::Size => SortField::Date,
                        SortField::Date => SortField::Type,
                        SortField::Type => SortField::Name,
                    };
                    state.write().sort_by = next;
                },
                "{sort_label}"
            }

            button {
                style: "padding: 4px 6px; background: {c.bg_secondary}; color: {c.text_secondary}; border: 1px solid {c.border}; border-radius: 4px; cursor: pointer; font-size: 12px;",
                onclick: move |_| {
                    let prev = state.read().sort_asc;
                    state.write().sort_asc = !prev;
                },
                "{arrow}"
            }
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// NewFolderDialog
// ════════════════════════════════════════════════════════════════════════════

#[component]
fn NewFolderDialog(state: Signal<MiyuCloudState>, folder_name: String) -> Element {
    let c = use_app_state().read().current_theme.palette();

    rsx! {
        div {
            style: "position: fixed; inset: 0; z-index: 1000; display: flex; align-items: center; justify-content: center; background: rgba(0,0,0,0.6);",
            onclick: move |_| state.write().new_folder_dialog = false,

            div {
                style: "background: {c.bg_card}; border-radius: 8px; padding: 24px; min-width: 320px; border: 1px solid {c.border}; box-shadow: 0 8px 32px rgba(0,0,0,0.4);",
                onclick: move |evt| evt.stop_propagation(),

                h3 { style: "margin: 0 0 16px; color: {c.text_white}; font-size: 16px;", "Nouveau dossier" }

                input {
                    style: "width: 100%; padding: 10px 12px; background: {c.bg_secondary}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_primary}; font-size: 14px; box-sizing: border-box; outline: none;",
                    r#type: "text",
                    placeholder: "Nom du dossier...",
                    value: "{folder_name}",
                    oninput: move |evt| {
                        state.write().new_folder_name = evt.value();
                    },
                }

                div {
                    style: "display: flex; justify-content: flex-end; gap: 8px; margin-top: 16px;",

                    button {
                        style: "padding: 8px 16px; background: {c.bg_hover}; color: {c.text_primary}; border: 1px solid {c.border}; border-radius: 4px; cursor: pointer; font-size: 13px;",
                        onclick: move |_| state.write().new_folder_dialog = false,
                        "Annuler"
                    }
                    button {
                        style: "padding: 8px 16px; background: {c.accent_blue}; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 13px;",
                        onclick: move |_| {
                            // Le dossier sera cree via le client HTTP dans le composant parent (mod.rs)
                            // Ici on ferme juste le dialog, la creation sera declenchee par le parent
                            state.write().new_folder_dialog = false;
                        },
                        "Creer"
                    }
                }
            }
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// Fonctions utilitaires
// ════════════════════════════════════════════════════════════════════════════

/// Trie les fichiers selon le champ et le sens specifies.
fn sort_files(files: &[FileEntry], sort_by: SortField, asc: bool) -> Vec<FileEntry> {
    let mut sorted = files.to_vec();
    sorted.sort_by(|a, b| {
        let ord = match sort_by {
            SortField::Name => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
            SortField::Size => a.size_bytes.cmp(&b.size_bytes),
            SortField::Date => a.updated_at.cmp(&b.updated_at),
            SortField::Type => a.mime_type.cmp(&b.mime_type),
        };
        if asc {
            ord
        } else {
            ord.reverse()
        }
    });
    sorted
}

/// Trie les dossiers (toujours par nom pour les dossiers, sauf date).
fn sort_folders(folders: &[FolderEntry], sort_by: SortField, asc: bool) -> Vec<FolderEntry> {
    let mut sorted = folders.to_vec();
    sorted.sort_by(|a, b| {
        let ord = match sort_by {
            SortField::Date => a.updated_at.cmp(&b.updated_at),
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        };
        if asc {
            ord
        } else {
            ord.reverse()
        }
    });
    sorted
}

/// Formate une date ISO 8601 en affichage court.
fn format_date(iso: &str) -> String {
    // Format entree : "2026-03-01T14:30:00Z" ou "2026-03-01 14:30:00"
    // Format sortie : "01/03/2026 14:30"
    if iso.len() >= 16 {
        let date_part = &iso[..10];
        let time_part = &iso[11..16];
        let parts: Vec<&str> = date_part.split('-').collect();
        if parts.len() == 3 {
            return format!("{}/{}/{} {time_part}", parts[2], parts[1], parts[0]);
        }
    }
    iso.to_string()
}
