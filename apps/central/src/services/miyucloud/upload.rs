//! Composant upload MiyuCloud -- televersement de fichiers.
//!
//! @id: miyucloud_upload
//! @do: upload_files_to_cloud
//! @role: component
//! @layer: presentation
//!
//! Bouton de selection de fichiers natif + barre de progression.
//! Utilise `rfd` (native file dialog) via spawn_blocking.

use std::path::PathBuf;

use dioxus::prelude::*;

use super::client::MiyuCloudClient;
use super::state::{MiyuCloudState, UploadProgress};
use crate::state::use_app_state;

/// Zone d'upload avec selecteur de fichiers et barre de progression.
#[component]
pub fn FileUploadZone(
    state: Signal<MiyuCloudState>,
    client: Signal<Option<MiyuCloudClient>>,
) -> Element {
    let c = use_app_state().read().current_theme.palette();

    let upload_progress = state.read().upload_progress.clone();
    let has_progress = upload_progress.is_some();

    // Calculer les valeurs de progression avant RSX
    let progress_label;
    let progress_pct;
    let progress_file;
    if let Some(ref progress) = upload_progress {
        let total = progress.bytes_total;
        let sent = progress.bytes_sent;
        progress_file = progress.file_name.clone();
        if total > 0 {
            progress_pct = (sent * 100 / total) as u32;
        } else {
            progress_pct = 0;
        }
        let sent_mb = sent as f64 / (1024.0 * 1024.0);
        let total_mb = total as f64 / (1024.0 * 1024.0);
        progress_label = format!("{sent_mb:.1} / {total_mb:.1} MB ({progress_pct}%)");
    } else {
        progress_file = String::new();
        progress_pct = 0;
        progress_label = String::new();
    }

    let bar_width = format!("{progress_pct}%");

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 12px;",

            // Barre de progression (si upload en cours)
            if has_progress {
                div {
                    style: "padding: 12px 16px; background: {c.bg_secondary}; border-radius: 8px; border: 1px solid {c.border};",

                    div {
                        style: "display: flex; align-items: center; justify-content: space-between; margin-bottom: 8px;",
                        span {
                            style: "font-size: 13px; color: {c.text_primary}; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; max-width: 300px;",
                            "\u{2B06} {progress_file}"
                        }
                        span {
                            style: "font-size: 12px; color: {c.text_secondary};",
                            "{progress_label}"
                        }
                    }

                    // Barre de progression
                    div {
                        style: "height: 6px; background: {c.bg_hover}; border-radius: 3px; overflow: hidden;",
                        div {
                            style: "height: 100%; width: {bar_width}; background: {c.accent_blue}; border-radius: 3px; transition: width 0.3s;",
                        }
                    }
                }
            }
        }
    }
}

/// Lance le selecteur de fichiers natif et televerse les fichiers selectionnes.
#[allow(dead_code)]
pub fn trigger_file_upload(
    mut state: Signal<MiyuCloudState>,
    client: Signal<Option<MiyuCloudClient>>,
) {
    spawn(async move {
        // Ouvrir le selecteur de fichiers natif via rfd (dans un thread bloquant)
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

        // Definir la progression
        state.write().upload_progress = Some(UploadProgress {
            file_name: file_name.clone(),
            bytes_sent: 0,
            bytes_total: file_size,
        });

        // Recuperer le client
        let http_client = {
            let c = client.read();
            c.clone()
        };

        let Some(http_client) = http_client else {
            state.write().error_message = Some("Client MiyuCloud non initialise".to_string());
            state.write().upload_progress = None;
            return;
        };

        // Lire le folder parent courant
        let parent_id = state.read().current_folder_id.clone();

        // Simuler la progression (le client reqwest n'offre pas de callback)
        state.write().upload_progress = Some(UploadProgress {
            file_name: file_name.clone(),
            bytes_sent: file_size / 2,
            bytes_total: file_size,
        });

        // Executer l'upload
        match http_client.upload_file(parent_id.as_deref(), &path).await {
            Ok(_entry) => {
                state.write().upload_progress = None;
                // Le rafraichissement sera declenche par le composant parent
            }
            Err(e) => {
                state.write().upload_progress = None;
                state.write().error_message = Some(format!("Echec upload : {e}"));
            }
        }
    });
}
