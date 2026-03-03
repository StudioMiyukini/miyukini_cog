//! Sidebar MiyuCloud -- arborescence dossiers + sections.
//!
//! @id: miyucloud_sidebar
//! @do: navigate_folder_tree
//! @role: component
//! @layer: presentation
//!
//! Panneau gauche fixe (220px) avec :
//! - Arborescence des dossiers (expand/collapse)
//! - Section "Partages" (placeholder Phase A)
//! - Section "Corbeille"

use dioxus::prelude::*;

use crate::state::use_app_state;
use super::state::{CloudSection, FolderEntry, MiyuCloudState};

/// Sidebar de navigation MiyuCloud.
#[component]
pub fn CloudSidebar(state: Signal<MiyuCloudState>) -> Element {
    let c = use_app_state().read().current_theme.palette();

    let current_section = state.read().section;
    let current_folder_id = state.read().current_folder_id.clone();
    let folder_tree = state.read().folder_tree.clone();
    let expanded = state.read().expanded_folders.clone();

    // Filtrer les dossiers racine (parent_id == None)
    let root_folders: Vec<FolderEntry> = folder_tree
        .iter()
        .filter(|f| f.parent_id.is_none())
        .cloned()
        .collect();

    let files_active = current_section == CloudSection::Files;
    let shares_active = current_section == CloudSection::Shares;
    let trash_active = current_section == CloudSection::Trash;
    let settings_active = current_section == CloudSection::Settings;

    let files_bg = if files_active { c.bg_hover } else { "transparent" };
    let files_color = if files_active { c.text_white } else { c.text_secondary };
    let files_border = if files_active {
        format!("2px solid {}", c.accent_blue)
    } else {
        "2px solid transparent".to_string()
    };

    let shares_bg = if shares_active { c.bg_hover } else { "transparent" };
    let shares_color = if shares_active { c.text_white } else { c.text_secondary };
    let shares_border = if shares_active {
        format!("2px solid {}", c.accent_blue)
    } else {
        "2px solid transparent".to_string()
    };

    let trash_bg = if trash_active { c.bg_hover } else { "transparent" };
    let trash_color = if trash_active { c.text_white } else { c.text_secondary };
    let trash_border = if trash_active {
        format!("2px solid {}", c.accent_blue)
    } else {
        "2px solid transparent".to_string()
    };

    let settings_bg = if settings_active { c.bg_hover } else { "transparent" };
    let settings_color = if settings_active { c.text_white } else { c.text_secondary };
    let settings_border = if settings_active {
        format!("2px solid {}", c.accent_blue)
    } else {
        "2px solid transparent".to_string()
    };

    rsx! {
        aside {
            style: "width: 220px; min-width: 220px; background: {c.bg_secondary}; border-right: 1px solid {c.border}; display: flex; flex-direction: column; overflow: hidden;",

            // En-tete
            div {
                style: "padding: 16px; border-bottom: 1px solid {c.border};",
                h3 {
                    style: "font-size: 15px; color: {c.text_white}; margin: 0 0 4px 0;",
                    "\u{2601} MiyuCloud"
                }
                p {
                    style: "font-size: 11px; color: {c.text_muted}; margin: 0;",
                    "Cloud prive"
                }
            }

            // Section Fichiers (navigation arborescence)
            div {
                style: "flex: 1; overflow-y: auto; display: flex; flex-direction: column;",

                // Bouton "Mes fichiers" (racine)
                button {
                    style: "display: flex; align-items: center; gap: 10px; padding: 10px 16px; background: {files_bg}; color: {files_color}; border: none; border-left: {files_border}; cursor: pointer; font-size: 13px; text-align: left; width: 100%; transition: all 0.15s;",
                    onclick: move |_| {
                        state.write().section = CloudSection::Files;
                        state.write().current_folder_id = None;
                        state.write().selected_file = None;
                    },
                    span { "\u{1F4C2}" }
                    span { "Mes fichiers" }
                }

                // Arborescence dossiers
                div {
                    style: "padding-left: 12px;",
                    for folder in root_folders.iter() {
                        {
                            let folder_id = folder.id.clone();
                            let folder_name = folder.name.clone();
                            let is_selected = current_folder_id.as_deref() == Some(folder_id.as_str());
                            let is_expanded = expanded.contains(&folder_id);
                            let child_list: Vec<FolderEntry> = folder_tree
                                .iter()
                                .filter(|f| f.parent_id.as_deref() == Some(folder_id.as_str()))
                                .cloned()
                                .collect();
                            let has_children = !child_list.is_empty();

                            rsx! {
                                FolderTreeItem {
                                    state: state,
                                    folder_id: folder_id,
                                    name: folder_name,
                                    is_selected: is_selected,
                                    is_expanded: is_expanded,
                                    has_children: has_children,
                                    child_folders: child_list,
                                    depth: 0,
                                    all_folders: folder_tree.clone(),
                                    expanded_ids: expanded.clone(),
                                    current_selected: current_folder_id.clone(),
                                }
                            }
                        }
                    }
                }

                // Separateur
                div { style: "height: 1px; background: {c.border}; margin: 8px 16px;" }

                // Section Partages
                button {
                    style: "display: flex; align-items: center; gap: 10px; padding: 10px 16px; background: {shares_bg}; color: {shares_color}; border: none; border-left: {shares_border}; cursor: pointer; font-size: 13px; text-align: left; width: 100%; transition: all 0.15s;",
                    onclick: move |_| {
                        state.write().section = CloudSection::Shares;
                        state.write().selected_file = None;
                    },
                    span { "\u{1F517}" }
                    span { "Partages" }
                }

                // Separateur
                div { style: "height: 1px; background: {c.border}; margin: 8px 16px;" }

                // Section Corbeille
                button {
                    style: "display: flex; align-items: center; gap: 10px; padding: 10px 16px; background: {trash_bg}; color: {trash_color}; border: none; border-left: {trash_border}; cursor: pointer; font-size: 13px; text-align: left; width: 100%; transition: all 0.15s;",
                    onclick: move |_| {
                        state.write().section = CloudSection::Trash;
                        state.write().selected_file = None;
                    },
                    span { "\u{1F5D1}" }
                    span { "Corbeille" }
                }

                // Separateur
                div { style: "height: 1px; background: {c.border}; margin: 8px 16px;" }

                // Section Parametres
                button {
                    style: "display: flex; align-items: center; gap: 10px; padding: 10px 16px; background: {settings_bg}; color: {settings_color}; border: none; border-left: {settings_border}; cursor: pointer; font-size: 13px; text-align: left; width: 100%; transition: all 0.15s;",
                    onclick: move |_| {
                        state.write().section = CloudSection::Settings;
                        state.write().selected_file = None;
                    },
                    span { "\u{2699}" }
                    span { "Parametres" }
                }
            }
        }
    }
}

/// Element d'arborescence de dossier (recursif).
#[component]
fn FolderTreeItem(
    state: Signal<MiyuCloudState>,
    folder_id: String,
    name: String,
    is_selected: bool,
    is_expanded: bool,
    has_children: bool,
    child_folders: Vec<FolderEntry>,
    depth: u32,
    all_folders: Vec<FolderEntry>,
    expanded_ids: Vec<String>,
    current_selected: Option<String>,
) -> Element {
    let c = use_app_state().read().current_theme.palette();

    let bg = if is_selected { c.bg_hover } else { "transparent" };
    let color = if is_selected { c.text_white } else { c.text_secondary };
    let indent = depth * 16 + 8;
    let chevron = if has_children {
        if is_expanded { "\u{25BE}" } else { "\u{25B8}" }
    } else {
        " "
    };

    let fid_click = folder_id.clone();
    let fid_toggle = folder_id.clone();

    rsx! {
        div {
            // Ligne du dossier
            div {
                style: "display: flex; align-items: center; gap: 4px; padding: 5px 8px 5px {indent}px; background: {bg}; color: {color}; cursor: pointer; font-size: 12px; border-radius: 2px; transition: background 0.1s;",
                onclick: move |_| {
                    state.write().section = CloudSection::Files;
                    state.write().current_folder_id = Some(fid_click.clone());
                    state.write().selected_file = None;
                },

                // Chevron expand/collapse
                if has_children {
                    span {
                        style: "cursor: pointer; font-size: 10px; width: 14px; text-align: center; color: {c.text_muted};",
                        onclick: move |evt| {
                            evt.stop_propagation();
                            let mut st = state.write();
                            if st.expanded_folders.contains(&fid_toggle) {
                                st.expanded_folders.retain(|id| id != &fid_toggle);
                            } else {
                                st.expanded_folders.push(fid_toggle.clone());
                            }
                        },
                        "{chevron}"
                    }
                } else {
                    span { style: "width: 14px;" }
                }

                span { style: "font-size: 14px;", "\u{1F4C1}" }
                span { style: "overflow: hidden; text-overflow: ellipsis; white-space: nowrap;", "{name}" }
            }

            // Enfants (si deplie)
            if is_expanded {
                for child in child_folders.iter() {
                    {
                        let child_id = child.id.clone();
                        let child_name = child.name.clone();
                        let child_selected = current_selected.as_deref() == Some(child_id.as_str());
                        let child_expanded = expanded_ids.contains(&child_id);
                        let grandchildren: Vec<FolderEntry> = all_folders
                            .iter()
                            .filter(|f| f.parent_id.as_deref() == Some(child_id.as_str()))
                            .cloned()
                            .collect();
                        let child_has_children = !grandchildren.is_empty();
                        let child_depth = depth + 1;

                        rsx! {
                            FolderTreeItem {
                                state: state,
                                folder_id: child_id,
                                name: child_name,
                                is_selected: child_selected,
                                is_expanded: child_expanded,
                                has_children: child_has_children,
                                child_folders: grandchildren,
                                depth: child_depth,
                                all_folders: all_folders.clone(),
                                expanded_ids: expanded_ids.clone(),
                                current_selected: current_selected.clone(),
                            }
                        }
                    }
                }
            }
        }
    }
}
