//! MiyuCloud -- Cloud prive dans Central.
//!
//! @id: miyucloud_central_view
//! @do: provide_miyucloud_ui_in_central
//! @role: view
//! @layer: presentation
//! @human: Lise (Dev Front-End)
//!
//! Point d'entree du service MiyuCloud dans Central.
//! Layout 2 panneaux : sidebar gauche (220px) + zone principale (flex).
//! Initialise le state et le client HTTP au montage.

mod client;
mod components;
mod explorer;
mod file_detail;
mod settings;
mod share_dialog;
mod sidebar;
pub mod state;
mod sync_status;
mod trash_view;
mod upload;

use std::path::PathBuf;

use dioxus::prelude::*;

use crate::state::use_app_state;
use client::MiyuCloudClient;
use explorer::FileExplorer;
use file_detail::FileDetailPanel;
use settings::CloudSettings;
use share_dialog::ShareDialog;
use sidebar::CloudSidebar;
use state::{BreadcrumbItem, CloudSection, FolderContents, MiyuCloudState, UploadProgress};
use sync_status::{SyncStatusBadge, SyncPanel};
use trash_view::TrashView;
use upload::FileUploadZone;

/// Vue racine MiyuCloud dans Central.
///
/// Initialise le state et le client, charge le dossier racine au montage,
/// et organise le layout sidebar + explorer + detail.
#[component]
pub fn MiyuCloudView() -> Element {
    let app_state = use_app_state();
    let c = app_state.read().current_theme.palette();

    // State MiyuCloud (signal local)
    let mut state = use_signal(MiyuCloudState::default);

    // Client HTTP (None = pas encore initialise)
    let mut client: Signal<Option<MiyuCloudClient>> = use_signal(|| None);

    // Initialisation du client au premier rendu
    use_effect(move || {
        let existing = client.read();
        if existing.is_some() {
            return;
        }
        drop(existing);

        // TODO: lire le port et le token depuis la configuration du service
        let http_client = MiyuCloudClient::new(11440, "dev-token");
        client.set(Some(http_client));
    });

    // Charger le contenu du dossier courant quand il change
    let folder_id = state.read().current_folder_id.clone();
    let section = state.read().section;
    use_effect(move || {
        let folder_to_load = folder_id.clone();
        let current_section = section;

        spawn(async move {
            let http = {
                let c = client.read();
                c.clone()
            };
            let Some(http) = http else { return };

            state.write().loading = true;
            state.write().error_message = None;

            let result = if current_section == CloudSection::Trash {
                http.list_trash().await
            } else {
                http.list_files(folder_to_load.as_deref()).await
            };

            match result {
                Ok(contents) => {
                    state.write().current_contents = contents;
                    state.write().loading = false;

                    // Mettre a jour le folder_tree pour la sidebar
                    let new_folders = state.read().current_contents.subfolders.clone();
                    let mut tree = state.read().folder_tree.clone();
                    for f in &new_folders {
                        if !tree.iter().any(|t| t.id == f.id) {
                            tree.push(f.clone());
                        }
                    }
                    state.write().folder_tree = tree;
                }
                Err(e) => {
                    state.write().loading = false;
                    // Si le serveur n'est pas encore lance, on charge des donnees de demo
                    let is_connection_error = matches!(e, client::MiyuCloudClientError::Http(_));
                    if is_connection_error {
                        state.write().current_contents = mock_root_contents();
                        state.write().folder_tree = mock_folder_tree();
                    } else {
                        state.write().error_message = Some(format!("{e}"));
                    }
                }
            }
        });
    });

    // Observer le changement de folder pour mettre a jour le breadcrumb
    let current_folder = state.read().current_folder_id.clone();
    let folder_tree = state.read().folder_tree.clone();
    use_effect(move || {
        let mut crumbs = vec![BreadcrumbItem {
            id: None,
            name: "Mon Cloud".to_string(),
        }];

        // Construire le chemin vers le dossier courant
        if let Some(ref fid) = current_folder {
            let mut path_ids: Vec<String> = Vec::new();
            let mut current = Some(fid.clone());
            while let Some(ref cid) = current {
                path_ids.push(cid.clone());
                current = folder_tree
                    .iter()
                    .find(|f| f.id == *cid)
                    .and_then(|f| f.parent_id.clone());
            }
            path_ids.reverse();
            for pid in &path_ids {
                if let Some(folder) = folder_tree.iter().find(|f| &f.id == pid) {
                    crumbs.push(BreadcrumbItem {
                        id: Some(folder.id.clone()),
                        name: folder.name.clone(),
                    });
                }
            }
        }
        state.write().breadcrumb = crumbs;
    });

    // Listener pour la creation de dossier
    let new_folder_pending = state.read().new_folder_dialog;
    let new_folder_name_val = state.read().new_folder_name.clone();

    // Verifier si le dialog vient d'etre ferme avec un nom valide
    // (on observe la transition close via un signal supplementaire)
    let mut prev_dialog_open = use_signal(|| false);
    use_effect(move || {
        let was_open = *prev_dialog_open.read();
        let is_open = new_folder_pending;
        prev_dialog_open.set(is_open);

        if was_open && !is_open && !new_folder_name_val.is_empty() {
            let name = new_folder_name_val.clone();
            spawn(async move {
                let parent = state.read().current_folder_id.clone();
                let http = {
                    let c = client.read();
                    c.clone()
                };
                let Some(http) = http else { return };

                match http.create_folder(&name, parent.as_deref()).await {
                    Ok(new_folder) => {
                        let mut contents = state.read().current_contents.clone();
                        contents.subfolders.push(new_folder.clone());
                        state.write().current_contents = contents;
                        // Ajouter a l'arbre
                        let mut tree = state.read().folder_tree.clone();
                        if !tree.iter().any(|t| t.id == new_folder.id) {
                            tree.push(new_folder);
                        }
                        state.write().folder_tree = tree;
                    }
                    Err(e) => {
                        state.write().error_message = Some(format!("Echec creation dossier : {e}"));
                    }
                }
            });
        }
    });

    let selected_file = state.read().selected_file.clone();
    let current_section = section;
    let share_dialog_open = state.read().share_dialog_open;
    let sync_panel_open = state.read().sync_panel_open;

    rsx! {
        div {
            style: "display: flex; height: 100%; overflow: hidden; background: {c.bg_main}; color: {c.text_primary};",

            // Sidebar gauche (220px fixe)
            CloudSidebar { state: state }

            // Zone principale avec header sync + routage par section
            div {
                style: "flex: 1; display: flex; flex-direction: column; min-width: 0; overflow: hidden;",

                // Header MiyuCloud avec badge sync
                div {
                    style: "display: flex; align-items: center; justify-content: space-between; padding: 8px 16px; border-bottom: 1px solid {c.border}; flex-shrink: 0;",

                    // Titre de la section
                    div {
                        style: "display: flex; align-items: center; gap: 8px;",
                        span { style: "font-size: 18px;",
                            match current_section {
                                CloudSection::Files => "\u{2601}",
                                CloudSection::Shares => "\u{1F517}",
                                CloudSection::Trash => "\u{1F5D1}",
                                CloudSection::Settings => "\u{2699}",
                            }
                        }
                        h3 { style: "font-size: 15px; color: {c.text_white}; margin: 0;",
                            match current_section {
                                CloudSection::Files => "MiyuCloud",
                                CloudSection::Shares => "Partages",
                                CloudSection::Trash => "Corbeille",
                                CloudSection::Settings => "Parametres",
                            }
                        }
                    }

                    // Badge sync (toujours visible)
                    SyncStatusBadge { state: state }
                }

            // Zone principale : routage par section
            match current_section {
                CloudSection::Files => rsx! {
                    div {
                        style: "flex: 1; display: flex; flex-direction: column; min-width: 0; overflow: hidden;",

                        // Zone d'upload (progression)
                        FileUploadZone { state: state, client: client }

                        // Explorateur + detail
                        div {
                            style: "flex: 1; display: flex; overflow: hidden;",

                            // Explorateur (zone centrale)
                            FileExplorer {
                                state: state,
                                on_upload: move |()| {
                                    trigger_upload(state, client);
                                },
                            }

                            // Panel detail (droite, si fichier selectionne)
                            if let Some(ref file) = selected_file {
                                FileDetailPanel {
                                    state: state,
                                    file: file.clone(),
                                    on_download: move |file_id: String| {
                                        trigger_download(file_id, client, state);
                                    },
                                    on_delete: move |file_id: String| {
                                        trigger_delete(file_id, client, state);
                                    },
                                    on_rename: move |file_id: String| {
                                        trigger_rename(file_id, client, state);
                                    },
                                }
                            }
                        }
                    }
                },
                CloudSection::Shares => rsx! {
                    div {
                        style: "flex: 1; display: flex; flex-direction: column; min-width: 0; overflow: hidden;",
                        // Placeholder pour la vue partages (liste des liens actifs)
                        SharesListView { state: state, client: client }
                    }
                },
                CloudSection::Trash => rsx! {
                    TrashView { state: state, client: client }
                },
                CloudSection::Settings => rsx! {
                    CloudSettings { state: state, client: client }
                },
            }

            } // fin div zone principale avec header

            // Dialog de partage (overlay, au-dessus de tout)
            if share_dialog_open {
                ShareDialog { state: state, client: client }
            }

            // Panel sync P2P (overlay droite)
            if sync_panel_open {
                SyncPanel { state: state, client: client }
            }
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// SharesListView — liste des liens de partage actifs
// ════════════════════════════════════════════════════════════════════════════

/// Vue de la section Partages : liste les liens actifs.
#[component]
fn SharesListView(
    state: Signal<MiyuCloudState>,
    client: Signal<Option<MiyuCloudClient>>,
) -> Element {
    let c = use_app_state().read().current_theme.palette();

    let links = state.read().share_links.clone();
    let is_empty = links.is_empty();
    let error_msg = state.read().error_message.clone();

    // Charger les liens au montage
    let mut loaded = use_signal(|| false);
    use_effect(move || {
        if *loaded.read() {
            return;
        }
        loaded.set(true);

        spawn(async move {
            let http = {
                let c = client.read();
                c.clone()
            };
            let Some(http) = http else { return };

            match http.list_share_links().await {
                Ok(links) => {
                    state.write().share_links = links;
                }
                Err(e) => {
                    state.write().error_message = Some(format!("Echec chargement partages : {e}"));
                }
            }
        });
    });

    rsx! {
        div {
            style: "flex: 1; display: flex; flex-direction: column; overflow: hidden;",

            // Header
            div {
                style: "padding: 14px 16px; border-bottom: 1px solid {c.border}; display: flex; align-items: center; gap: 10px;",
                span { style: "font-size: 20px;", "\u{1F517}" }
                h3 { style: "font-size: 16px; color: {c.text_white}; margin: 0;", "Partages" }
            }

            // Error
            if let Some(ref err) = error_msg {
                div {
                    style: "padding: 8px 16px; background: {c.accent_red}20; border-left: 3px solid {c.accent_red}; margin: 8px 16px 0; border-radius: 4px; color: {c.accent_red}; font-size: 13px;",
                    "{err}"
                }
            }

            // Content
            div {
                style: "flex: 1; overflow-y: auto; padding: 16px;",

                if is_empty {
                    div {
                        style: "display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 48px; color: {c.text_muted};",
                        span { style: "font-size: 48px; margin-bottom: 16px; opacity: 0.5;", "\u{1F517}" }
                        p { style: "font-size: 16px; margin: 0 0 8px;", "Aucun lien de partage actif" }
                        p { style: "font-size: 13px;", "Selectionnez un fichier et cliquez sur Partager pour creer un lien." }
                    }
                }

                if !is_empty {
                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",
                        for link in links.iter() {
                            {
                                let link_id = link.id.clone();
                                let token_short = if link.token.len() > 12 {
                                    format!("{}...", &link.token[..12])
                                } else {
                                    link.token.clone()
                                };
                                let expires = format_share_date(&link.expires_at);
                                let dl_count = link.download_count;
                                let max_dl = link.max_downloads;
                                let is_revoked = link.is_revoked;
                                let has_pwd = link.has_password;

                                let status_label = if is_revoked {
                                    "Revoque"
                                } else {
                                    "Actif"
                                };
                                let status_color = if is_revoked { c.accent_red } else { c.accent_green };
                                let dl_label = if let Some(max) = max_dl {
                                    format!("{dl_count}/{max} telechargements")
                                } else {
                                    format!("{dl_count} telechargements")
                                };
                                let pwd_label = if has_pwd { "Protege" } else { "Sans mot de passe" };

                                rsx! {
                                    div {
                                        style: "padding: 12px 16px; background: {c.bg_secondary}; border: 1px solid {c.border}; border-radius: 8px; display: flex; flex-direction: column; gap: 6px;",

                                        // Top row
                                        div {
                                            style: "display: flex; align-items: center; justify-content: space-between;",
                                            div {
                                                style: "display: flex; align-items: center; gap: 8px;",
                                                span {
                                                    style: "font-size: 13px; color: {c.text_primary}; font-family: monospace;",
                                                    "{token_short}"
                                                }
                                                span {
                                                    style: "font-size: 11px; color: {status_color}; background: {status_color}15; padding: 1px 8px; border-radius: 8px;",
                                                    "{status_label}"
                                                }
                                            }
                                            if !is_revoked {
                                                button {
                                                    style: "padding: 4px 10px; background: {c.accent_red}20; color: {c.accent_red}; border: 1px solid {c.accent_red}40; border-radius: 4px; cursor: pointer; font-size: 11px;",
                                                    aria_label: "Revoquer le lien",
                                                    onclick: move |_| {
                                                        let lid = link_id.clone();
                                                        spawn(async move {
                                                            let http = {
                                                                let c = client.read();
                                                                c.clone()
                                                            };
                                                            let Some(http) = http else { return };
                                                            match http.revoke_share_link(&lid).await {
                                                                Ok(()) => {
                                                                    let mut links = state.read().share_links.clone();
                                                                    if let Some(l) = links.iter_mut().find(|l| l.id == lid) {
                                                                        l.is_revoked = true;
                                                                    }
                                                                    state.write().share_links = links;
                                                                }
                                                                Err(e) => {
                                                                    state.write().error_message = Some(format!("Echec revocation : {e}"));
                                                                }
                                                            }
                                                        });
                                                    },
                                                    "Revoquer"
                                                }
                                            }
                                        }

                                        // Detail row
                                        div {
                                            style: "display: flex; gap: 16px; font-size: 12px; color: {c.text_secondary};",
                                            span { "Expire : {expires}" }
                                            span { "{dl_label}" }
                                            span { "{pwd_label}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Formate une date ISO 8601 pour les partages.
fn format_share_date(iso: &str) -> String {
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

// ════════════════════════════════════════════════════════════════════════════
// Actions async
// ════════════════════════════════════════════════════════════════════════════

/// Lance le selecteur de fichiers et l'upload.
fn trigger_upload(
    mut state: Signal<MiyuCloudState>,
    client: Signal<Option<MiyuCloudClient>>,
) {
    spawn(async move {
        // Ouvrir le selecteur de fichiers natif
        let file_path: Option<PathBuf> = tokio::task::spawn_blocking(|| {
            rfd::FileDialog::new()
                .set_title("Selectionner un fichier a uploader")
                .pick_file()
        })
        .await
        .ok()
        .flatten();

        let Some(path) = file_path else { return };

        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("fichier")
            .to_string();

        let file_size = tokio::fs::metadata(&path)
            .await
            .map(|m| m.len())
            .unwrap_or(0);

        state.write().upload_progress = Some(UploadProgress {
            file_name: file_name.clone(),
            bytes_sent: 0,
            bytes_total: file_size,
        });

        let http = {
            let c = client.read();
            c.clone()
        };
        let Some(http) = http else {
            state.write().error_message = Some("Client non initialise".to_string());
            state.write().upload_progress = None;
            return;
        };

        let parent_id = state.read().current_folder_id.clone();

        // Simuler la progression a mi-chemin
        state.write().upload_progress = Some(UploadProgress {
            file_name: file_name.clone(),
            bytes_sent: file_size / 2,
            bytes_total: file_size,
        });

        match http.upload_file(parent_id.as_deref(), &path).await {
            Ok(entry) => {
                state.write().upload_progress = None;
                let mut contents = state.read().current_contents.clone();
                contents.files.push(entry);
                state.write().current_contents = contents;
            }
            Err(e) => {
                state.write().upload_progress = None;
                state.write().error_message = Some(format!("Echec upload : {e}"));
            }
        }
    });
}

/// Telecharge un fichier et le sauvegarde via un dialog natif.
fn trigger_download(
    file_id: String,
    client: Signal<Option<MiyuCloudClient>>,
    mut state: Signal<MiyuCloudState>,
) {
    spawn(async move {
        let http = {
            let c = client.read();
            c.clone()
        };
        let Some(http) = http else { return };

        let file_name = state
            .read()
            .selected_file
            .as_ref()
            .map_or_else(|| "download".to_string(), |f| f.name.clone());

        match http.download_file(&file_id).await {
            Ok(data) => {
                // Ouvrir un dialog de sauvegarde
                let save_path: Option<PathBuf> = tokio::task::spawn_blocking(move || {
                    rfd::FileDialog::new()
                        .set_title("Sauvegarder le fichier")
                        .set_file_name(&file_name)
                        .save_file()
                })
                .await
                .ok()
                .flatten();

                if let Some(path) = save_path {
                    if let Err(e) = tokio::fs::write(&path, &data).await {
                        state.write().error_message = Some(format!("Echec sauvegarde : {e}"));
                    }
                }
            }
            Err(e) => {
                state.write().error_message = Some(format!("Echec telechargement : {e}"));
            }
        }
    });
}

/// Supprime (met en corbeille) un fichier.
fn trigger_delete(
    file_id: String,
    client: Signal<Option<MiyuCloudClient>>,
    mut state: Signal<MiyuCloudState>,
) {
    spawn(async move {
        let http = {
            let c = client.read();
            c.clone()
        };
        let Some(http) = http else { return };

        match http.delete_file(&file_id).await {
            Ok(()) => {
                // Retirer le fichier du contenu courant
                let mut contents = state.read().current_contents.clone();
                contents.files.retain(|f| f.id != file_id);
                state.write().current_contents = contents;
                state.write().selected_file = None;
            }
            Err(e) => {
                state.write().error_message = Some(format!("Echec suppression : {e}"));
            }
        }
    });
}

/// Renomme un fichier.
fn trigger_rename(
    file_id: String,
    client: Signal<Option<MiyuCloudClient>>,
    mut state: Signal<MiyuCloudState>,
) {
    spawn(async move {
        let new_name = state.read().rename_value.clone();
        if new_name.is_empty() {
            return;
        }

        let http = {
            let c = client.read();
            c.clone()
        };
        let Some(http) = http else { return };

        match http.rename_file(&file_id, &new_name).await {
            Ok(updated) => {
                // Mettre a jour dans le contenu courant
                let mut contents = state.read().current_contents.clone();
                if let Some(f) = contents.files.iter_mut().find(|f| f.id == file_id) {
                    *f = updated.clone();
                }
                state.write().current_contents = contents;
                state.write().selected_file = Some(updated);
            }
            Err(e) => {
                state.write().error_message = Some(format!("Echec renommage : {e}"));
            }
        }
    });
}

// ════════════════════════════════════════════════════════════════════════════
// Mock data (utilise quand le serveur n'est pas encore lance)
// ════════════════════════════════════════════════════════════════════════════

/// Contenu racine de demonstration.
#[allow(clippy::too_many_lines)]
fn mock_root_contents() -> FolderContents {
    use state::{FileEntry, FolderEntry};

    FolderContents {
        folder: None,
        subfolders: vec![
            FolderEntry {
                id: "folder-photos".to_string(),
                parent_id: None,
                owner_id: "user-1".to_string(),
                name: "Photos".to_string(),
                is_trashed: false,
                trashed_at: None,
                created_at: "2026-02-15T10:00:00Z".to_string(),
                updated_at: "2026-02-28T14:30:00Z".to_string(),
            },
            FolderEntry {
                id: "folder-docs".to_string(),
                parent_id: None,
                owner_id: "user-1".to_string(),
                name: "Documents".to_string(),
                is_trashed: false,
                trashed_at: None,
                created_at: "2026-01-10T08:00:00Z".to_string(),
                updated_at: "2026-02-25T09:15:00Z".to_string(),
            },
            FolderEntry {
                id: "folder-projets".to_string(),
                parent_id: None,
                owner_id: "user-1".to_string(),
                name: "Projets".to_string(),
                is_trashed: false,
                trashed_at: None,
                created_at: "2026-01-20T14:00:00Z".to_string(),
                updated_at: "2026-03-01T11:00:00Z".to_string(),
            },
            FolderEntry {
                id: "folder-musique".to_string(),
                parent_id: None,
                owner_id: "user-1".to_string(),
                name: "Musique".to_string(),
                is_trashed: false,
                trashed_at: None,
                created_at: "2026-02-01T16:00:00Z".to_string(),
                updated_at: "2026-02-20T18:45:00Z".to_string(),
            },
        ],
        files: vec![
            FileEntry {
                id: "file-readme".to_string(),
                parent_id: None,
                owner_id: "user-1".to_string(),
                name: "README.md".to_string(),
                mime_type: "text/markdown".to_string(),
                size_bytes: 2_048,
                checksum_sha256: "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2".to_string(),
                encryption_iv: Some("base64iv==".to_string()),
                chunk_count: 1,
                is_trashed: false,
                trashed_at: None,
                created_at: "2026-02-20T10:30:00Z".to_string(),
                updated_at: "2026-02-28T16:00:00Z".to_string(),
            },
            FileEntry {
                id: "file-photo1".to_string(),
                parent_id: None,
                owner_id: "user-1".to_string(),
                name: "vacances_2026.jpg".to_string(),
                mime_type: "image/jpeg".to_string(),
                size_bytes: 3_458_000,
                checksum_sha256: "b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2c3".to_string(),
                encryption_iv: Some("base64iv2==".to_string()),
                chunk_count: 14,
                is_trashed: false,
                trashed_at: None,
                created_at: "2026-02-25T12:00:00Z".to_string(),
                updated_at: "2026-02-25T12:00:00Z".to_string(),
            },
            FileEntry {
                id: "file-rapport".to_string(),
                parent_id: None,
                owner_id: "user-1".to_string(),
                name: "rapport_annuel.pdf".to_string(),
                mime_type: "application/pdf".to_string(),
                size_bytes: 1_250_000,
                checksum_sha256: "c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2c3d4".to_string(),
                encryption_iv: Some("base64iv3==".to_string()),
                chunk_count: 5,
                is_trashed: false,
                trashed_at: None,
                created_at: "2026-03-01T09:00:00Z".to_string(),
                updated_at: "2026-03-01T09:00:00Z".to_string(),
            },
            FileEntry {
                id: "file-backup".to_string(),
                parent_id: None,
                owner_id: "user-1".to_string(),
                name: "backup_2026-02.zip".to_string(),
                mime_type: "application/zip".to_string(),
                size_bytes: 52_430_000,
                checksum_sha256: "d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2c3d4e5".to_string(),
                encryption_iv: Some("base64iv4==".to_string()),
                chunk_count: 200,
                is_trashed: false,
                trashed_at: None,
                created_at: "2026-02-28T22:00:00Z".to_string(),
                updated_at: "2026-02-28T22:00:00Z".to_string(),
            },
            FileEntry {
                id: "file-track".to_string(),
                parent_id: None,
                owner_id: "user-1".to_string(),
                name: "ambient_track.mp3".to_string(),
                mime_type: "audio/mpeg".to_string(),
                size_bytes: 8_720_000,
                checksum_sha256: "e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2c3d4e5f6".to_string(),
                encryption_iv: Some("base64iv5==".to_string()),
                chunk_count: 34,
                is_trashed: false,
                trashed_at: None,
                created_at: "2026-02-10T15:30:00Z".to_string(),
                updated_at: "2026-02-10T15:30:00Z".to_string(),
            },
        ],
    }
}

/// Arborescence de dossiers de demonstration pour la sidebar.
fn mock_folder_tree() -> Vec<state::FolderEntry> {
    vec![
        state::FolderEntry {
            id: "folder-photos".to_string(),
            parent_id: None,
            owner_id: "user-1".to_string(),
            name: "Photos".to_string(),
            is_trashed: false,
            trashed_at: None,
            created_at: "2026-02-15T10:00:00Z".to_string(),
            updated_at: "2026-02-28T14:30:00Z".to_string(),
        },
        state::FolderEntry {
            id: "folder-photos-2026".to_string(),
            parent_id: Some("folder-photos".to_string()),
            owner_id: "user-1".to_string(),
            name: "2026".to_string(),
            is_trashed: false,
            trashed_at: None,
            created_at: "2026-01-01T00:00:00Z".to_string(),
            updated_at: "2026-02-28T14:30:00Z".to_string(),
        },
        state::FolderEntry {
            id: "folder-photos-2025".to_string(),
            parent_id: Some("folder-photos".to_string()),
            owner_id: "user-1".to_string(),
            name: "2025".to_string(),
            is_trashed: false,
            trashed_at: None,
            created_at: "2025-01-01T00:00:00Z".to_string(),
            updated_at: "2025-12-31T23:59:00Z".to_string(),
        },
        state::FolderEntry {
            id: "folder-docs".to_string(),
            parent_id: None,
            owner_id: "user-1".to_string(),
            name: "Documents".to_string(),
            is_trashed: false,
            trashed_at: None,
            created_at: "2026-01-10T08:00:00Z".to_string(),
            updated_at: "2026-02-25T09:15:00Z".to_string(),
        },
        state::FolderEntry {
            id: "folder-projets".to_string(),
            parent_id: None,
            owner_id: "user-1".to_string(),
            name: "Projets".to_string(),
            is_trashed: false,
            trashed_at: None,
            created_at: "2026-01-20T14:00:00Z".to_string(),
            updated_at: "2026-03-01T11:00:00Z".to_string(),
        },
        state::FolderEntry {
            id: "folder-musique".to_string(),
            parent_id: None,
            owner_id: "user-1".to_string(),
            name: "Musique".to_string(),
            is_trashed: false,
            trashed_at: None,
            created_at: "2026-02-01T16:00:00Z".to_string(),
            updated_at: "2026-02-20T18:45:00Z".to_string(),
        },
    ]
}
