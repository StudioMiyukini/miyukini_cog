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

mod auth_security;
mod calendar_view;
mod client;
mod components;
mod contacts_view;
mod explorer;
mod file_detail;
mod settings;
mod setup_wizard;
mod share_dialog;
mod sidebar;
pub mod state;
mod sync_status;
mod trash_view;
mod upload;

use std::path::PathBuf;

use dioxus::prelude::*;

use crate::services::miyucloud_settings::load_miyucloud_config;
use crate::state::use_app_state;
use calendar_view::CalendarView;
use client::MiyuCloudClient;
use contacts_view::ContactsView;
use explorer::FileExplorer;
use file_detail::FileDetailPanel;
use settings::CloudSettings;
use setup_wizard::FirstLaunchSetupWizard;
use share_dialog::ShareDialog;
use sidebar::CloudSidebar;
use state::{BreadcrumbItem, CloudSection, MiyuCloudState, UploadProgress};
use sync_status::{SyncPanel, SyncStatusBadge};
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

        let config = load_miyucloud_config();
        // Batch all state writes into a single write to avoid triggering N re-renders
        {
            let mut s = state.write();
            s.storage_path = config.storage_path.clone();
            s.web_surface_enabled = config.web_enabled;
            s.web_surface_port = config.web_port;
            s.configured_quota_bytes = config.quota_bytes;
            s.first_launch_completed = config.first_launch_completed;
            s.setup_wizard_open = !config.first_launch_completed;
            s.public_share_links_enabled = config.share_policy.public_links_enabled;
            s.internal_sharing_enabled = config.share_policy.internal_sharing_enabled;
            s.default_share_permission =
                share_permission_to_state(config.share_policy.default_permission);
            s.ssh_enabled = config.ssh.enabled;
            s.ssh_host = config.ssh.host.clone();
            s.ssh_port = config.ssh.port;
            s.ssh_username = config.ssh.username.clone();
            s.ssh_root_path = config.ssh.root_path.clone();
            s.ssh_private_key_path = config.ssh.private_key_path.clone();
            s.ssh_keepalive = config.ssh.keepalive;
        }

        let http_client = MiyuCloudClient::new(config.api_port, &config.cog_token);
        client.set(Some(http_client));
    });

    // Derive reactive memos for folder_id and section to avoid re-running
    // the loading effect on every unrelated state change.
    let folder_id_memo = use_memo(move || state.read().current_folder_id.clone());
    let section_memo = use_memo(move || state.read().section);
    let reload_memo = use_memo(move || state.read().reload_counter);

    // Charger le contenu du dossier courant quand il change (ou sur reload forcé)
    use_effect(move || {
        // Track folder_id, section, and reload_counter
        let folder_to_load = folder_id_memo.read().clone();
        let current_section = *section_memo.read();
        let _reload = *reload_memo.read();

        spawn(async move {
            let http = {
                let c = client.peek();
                c.clone()
            };
            let Some(http) = http else { return };

            {
                let mut s = state.write();
                s.loading = true;
                s.error_message = None;
            }

            let result = if current_section == CloudSection::Trash {
                http.list_trash().await
            } else {
                http.list_files(folder_to_load.as_deref()).await
            };

            match result {
                Ok(contents) => {
                    let mut s = state.write();
                    let new_folders = contents.subfolders.clone();
                    for f in &new_folders {
                        if !s.folder_tree.iter().any(|t| t.id == f.id) {
                            s.folder_tree.push(f.clone());
                        }
                    }
                    s.current_contents = contents;
                    s.loading = false;
                }
                Err(e) => {
                    let mut s = state.write();
                    s.loading = false;
                    let is_connection_error = matches!(e, client::MiyuCloudClientError::Http(_));
                    if is_connection_error {
                        s.error_message = Some(
                            "Service MiyuCloud hors ligne. Verifiez que le service est demarre."
                                .to_string(),
                        );
                    } else {
                        s.error_message = Some(format!("{e}"));
                    }
                }
            }
        });
    });

    // Calculer le breadcrumb comme etat derive (use_memo) au lieu de use_effect
    // pour eviter le cycle ecriture-state -> re-render -> re-ecriture
    let breadcrumb_memo = use_memo(move || {
        let current_folder = state.read().current_folder_id.clone();
        let folder_tree = state.read().folder_tree.clone();

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
        crumbs
    });

    // Listener pour la creation de dossier
    let new_folder_dialog_memo = use_memo(move || state.read().new_folder_dialog);
    let new_folder_name_memo = use_memo(move || state.read().new_folder_name.clone());

    let mut prev_dialog_open = use_signal(|| false);
    use_effect(move || {
        let is_open = *new_folder_dialog_memo.read();
        let new_folder_name_val = new_folder_name_memo.read().clone();

        let was_open = *prev_dialog_open.peek();
        if was_open != is_open {
            prev_dialog_open.set(is_open);
        }

        if was_open && !is_open && !new_folder_name_val.is_empty() {
            let name = new_folder_name_val.clone();
            spawn(async move {
                let parent = state.peek().current_folder_id.clone();
                let http = {
                    let c = client.peek();
                    c.clone()
                };
                let Some(http) = http else { return };

                match http.create_folder(&name, parent.as_deref()).await {
                    Ok(new_folder) => {
                        let mut s = state.write();
                        if !s.folder_tree.iter().any(|t| t.id == new_folder.id) {
                            s.folder_tree.push(new_folder.clone());
                        }
                        s.current_contents.subfolders.push(new_folder);
                    }
                    Err(e) => {
                        state.write().error_message = Some(format!("Echec creation dossier : {e}"));
                    }
                }
            });
        }
    });

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
                            {
                                let current_section = state.read().section;
                                match current_section {
                                    CloudSection::Files => "\u{2601}",
                                    CloudSection::Calendar => "\u{1F4C5}",
                                    CloudSection::Contacts => "\u{1F464}",
                                    CloudSection::Shares => "\u{1F517}",
                                    CloudSection::Trash => "\u{1F5D1}",
                                    CloudSection::Settings => "\u{2699}",
                                }
                            }
                        }
                        h3 { style: "font-size: 15px; color: {c.text_white}; margin: 0;",
                            {
                                let current_section = state.read().section;
                                match current_section {
                                    CloudSection::Files => "MiyuCloud",
                                    CloudSection::Calendar => "Calendrier",
                                    CloudSection::Contacts => "Contacts",
                                    CloudSection::Shares => "Partages",
                                    CloudSection::Trash => "Corbeille",
                                    CloudSection::Settings => "Parametres",
                                }
                            }
                        }
                    }

                    div {
                        style: "display: flex; align-items: center; gap: 10px;",

                        // Interrupteur serveur MiyuCloud
                        ServerToggle { state: state }

                        // Badge sync (toujours visible)
                        SyncStatusBadge { state: state }
                    }
                }

            // Zone principale : routage par section
            {
                let current_section = state.read().section;
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
                                    breadcrumb_memo: breadcrumb_memo,
                                    on_upload: move |()| {
                                        trigger_upload(state, client);
                                    },
                                }

                                // Panel detail (droite, si fichier selectionne)
                                {
                                    let selected_file = state.read().selected_file.clone();
                                    if let Some(file) = selected_file {
                                        rsx! {
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
                                    } else {
                                        rsx! {}
                                    }
                                }
                            }
                        }
                    },
                    CloudSection::Calendar => rsx! {
                        CalendarView { state: state, client: client }
                    },
                    CloudSection::Contacts => rsx! {
                        ContactsView { state: state, client: client }
                    },
                    CloudSection::Shares => rsx! {
                        div {
                            style: "flex: 1; display: flex; flex-direction: column; min-width: 0; overflow: hidden;",
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
            }

            } // fin div zone principale avec header

            // Dialog de partage (overlay, au-dessus de tout)
            if state.read().share_dialog_open {
                ShareDialog { state: state, client: client }
            }

            // Panel sync P2P (overlay droite)
            if state.read().sync_panel_open {
                SyncPanel { state: state, client: client }
            }

            FirstLaunchSetupWizard { state: state, client: client }
        }
    }
}

fn share_permission_to_state(
    value: crate::services::miyucloud_settings::SharePermissionPreset,
) -> state::PermissionLevel {
    match value {
        crate::services::miyucloud_settings::SharePermissionPreset::Read => {
            state::PermissionLevel::Read
        }
        crate::services::miyucloud_settings::SharePermissionPreset::ReadWrite => {
            state::PermissionLevel::ReadWrite
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
// ServerToggle — interrupteur marche/arret du service MiyuCloud
// ════════════════════════════════════════════════════════════════════════════

/// Bouton toggle pour demarrer/arreter le service MiyuCloud.
#[component]
fn ServerToggle(state: Signal<MiyuCloudState>) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let service_manager = crate::state::use_service_manager();
    let app_state = use_app_state();

    let is_running = service_manager.is_running("miyucloud");
    let dot_color = if is_running { c.accent_green } else { c.text_muted };
    let label = if is_running { "Serveur actif" } else { "Serveur eteint" };
    let btn_bg = if is_running { c.accent_green } else { c.bg_hover };

    rsx! {
        button {
            style: "display: inline-flex; align-items: center; gap: 6px; padding: 4px 12px; background: {btn_bg}18; border: 1px solid {btn_bg}40; border-radius: 16px; cursor: pointer; font-size: 12px; color: {c.text_secondary}; transition: all 0.2s;",
            aria_label: if is_running { "Arreter le serveur MiyuCloud" } else { "Demarrer le serveur MiyuCloud" },
            onclick: {
                let service_manager = service_manager.clone();
                move |_| {
                    let sm = service_manager.clone();
                    if sm.is_running("miyucloud") {
                        if let Err(e) = sm.stop_service("miyucloud") {
                            state.write().error_message = Some(format!("Arret echoue : {e}"));
                        }
                    } else {
                        let profile_id = app_state
                            .read()
                            .current_user
                            .as_ref()
                            .map(|u| u.id.clone())
                            .unwrap_or_default();
                        match sm.launch_service("miyucloud", &profile_id) {
                            Ok(()) => {
                                // Attendre que le serveur soit pret (poll /health jusqu'a OK)
                                spawn(async move {
                                    let config = load_miyucloud_config();
                                    let url = format!("http://127.0.0.1:{}/health", config.api_port);
                                    let http = reqwest::Client::builder()
                                        .timeout(std::time::Duration::from_secs(2))
                                        .build()
                                        .unwrap_or_default();
                                    for _ in 0..10 {
                                        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                                        if let Ok(resp) = http.get(&url).send().await {
                                            if resp.status().is_success() {
                                                break;
                                            }
                                        }
                                    }
                                    state.write().error_message = None;
                                    state.write().reload_counter += 1;
                                });
                            }
                            Err(e) => {
                                state.write().error_message =
                                    Some(format!("Demarrage echoue : {e}"));
                            }
                        }
                    }
                }
            },
            span {
                style: "display: inline-block; width: 8px; height: 8px; border-radius: 50%; background: {dot_color};",
            }
            span { "{label}" }
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// Actions async
// ════════════════════════════════════════════════════════════════════════════

/// Lance le selecteur de fichiers et l'upload.
fn trigger_upload(mut state: Signal<MiyuCloudState>, client: Signal<Option<MiyuCloudClient>>) {
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

