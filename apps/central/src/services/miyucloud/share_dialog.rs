//! Dialog de creation de partage MiyuCloud.
//!
//! @id: miyucloud_share_dialog
//! @do: create_share_links_and_permissions
//! @role: component
//! @layer: presentation
//! @human: Lise (Dev Front-End)
//!
//! Modal pour creer des liens de partage externes ou des partages internes
//! (Tribu/profil). Gere deux onglets : "Lien externe" et "Partage Tribu".

use dioxus::prelude::*;

use super::client::MiyuCloudClient;
use super::state::{
    CreateShareLinkRequest, CreateSharePermissionRequest, GranteeType, MiyuCloudState,
    PermissionLevel,
};
use crate::state::use_app_state;

// ════════════════════════════════════════════════════════════════════════════
// Types locaux
// ════════════════════════════════════════════════════════════════════════════

/// Onglet actif dans le dialog de partage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ShareTab {
    /// Lien externe (URL publique avec expiration).
    ExternalLink,
    /// Partage interne (Tribu / profil avec permissions).
    TribeShare,
}

/// Duree d'expiration predefinie.
#[derive(Debug, Clone, Copy)]
struct ExpirationOption {
    label: &'static str,
    hours: u32,
}

const EXPIRATION_OPTIONS: &[ExpirationOption] = &[
    ExpirationOption {
        label: "1 heure",
        hours: 1,
    },
    ExpirationOption {
        label: "6 heures",
        hours: 6,
    },
    ExpirationOption {
        label: "24 heures",
        hours: 24,
    },
    ExpirationOption {
        label: "7 jours",
        hours: 168,
    },
    ExpirationOption {
        label: "30 jours",
        hours: 720,
    },
];

// ════════════════════════════════════════════════════════════════════════════
// ShareDialog
// ════════════════════════════════════════════════════════════════════════════

/// Dialog de creation/gestion de partage.
/// S'affiche comme un overlay modal quand `state.share_dialog_open` est true.
#[component]
pub fn ShareDialog(
    state: Signal<MiyuCloudState>,
    client: Signal<Option<MiyuCloudClient>>,
) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let public_links_enabled = state.read().public_share_links_enabled;
    let internal_sharing_enabled = state.read().internal_sharing_enabled;
    let web_surface_enabled = state.read().web_surface_enabled;
    let web_surface_port = state.read().web_surface_port;
    let external_links_available = public_links_enabled && web_surface_enabled;
    let initial_tab = if external_links_available {
        ShareTab::ExternalLink
    } else if internal_sharing_enabled {
        ShareTab::TribeShare
    } else {
        ShareTab::ExternalLink
    };

    // Signaux locaux pour le formulaire
    let mut active_tab = use_signal(|| initial_tab);
    let mut expiration_index = use_signal(|| 2_usize); // 24h par defaut
    let mut password_enabled = use_signal(|| false);
    let mut password_value = use_signal(String::new);
    let mut max_downloads_enabled = use_signal(|| false);
    let mut max_downloads_value = use_signal(|| "10".to_string());
    let mut creating = use_signal(|| false);
    let mut error_msg: Signal<Option<String>> = use_signal(|| None);

    // Partage Tribu
    let mut grantee_id = use_signal(String::new);
    let mut grantee_type = use_signal(|| GranteeType::Profile);
    let mut permission_level = use_signal(|| state.read().default_share_permission);

    // Valeurs lues pour eviter les nested braces dans RSX
    let tab = *active_tab.read();
    let exp_idx = *expiration_index.read();
    let is_password = *password_enabled.read();
    let is_max_dl = *max_downloads_enabled.read();
    let is_creating = *creating.read();
    let current_error = error_msg.read().clone();
    let generated_url = state.read().generated_share_url.clone();
    let has_generated_url = generated_url.is_some();
    let external_link_notice =
        share_availability_message(public_links_enabled, web_surface_enabled);
    let tribe_share_notice = if internal_sharing_enabled {
        None
    } else {
        Some("Le partage interne est desactive dans le setup MiyuCloud.")
    };

    let link_tab_active = tab == ShareTab::ExternalLink;
    let tribe_tab_active = tab == ShareTab::TribeShare;

    let link_tab_bg = if link_tab_active {
        c.accent_blue
    } else if !external_links_available {
        c.bg_main
    } else {
        c.bg_hover
    };
    let link_tab_color = if link_tab_active {
        c.text_white
    } else if !external_links_available {
        c.text_muted
    } else {
        c.text_secondary
    };
    let tribe_tab_bg = if tribe_tab_active {
        c.accent_blue
    } else if !internal_sharing_enabled {
        c.bg_main
    } else {
        c.bg_hover
    };
    let tribe_tab_color = if tribe_tab_active {
        c.text_white
    } else if !internal_sharing_enabled {
        c.text_muted
    } else {
        c.text_secondary
    };
    let link_tab_cursor = if external_links_available {
        "pointer"
    } else {
        "not-allowed"
    };
    let tribe_tab_cursor = if internal_sharing_enabled {
        "pointer"
    } else {
        "not-allowed"
    };

    let action_disabled = is_creating
        || (link_tab_active && !external_links_available)
        || (tribe_tab_active && !internal_sharing_enabled);
    let btn_opacity = if action_disabled { "0.6" } else { "1" };
    let btn_cursor = if action_disabled {
        "not-allowed"
    } else {
        "pointer"
    };

    // File info for the dialog title
    let file_name = state
        .read()
        .selected_file
        .as_ref()
        .map_or_else(|| "Fichier".to_string(), |f| f.name.clone());

    rsx! {
        div {
            style: "position: fixed; inset: 0; z-index: 1000; display: flex; align-items: center; justify-content: center; background: rgba(0,0,0,0.65);",
            role: "dialog",
            aria_modal: "true",
            aria_label: "Partager un fichier",
            onclick: move |_| {
                state.write().share_dialog_open = false;
                state.write().generated_share_url = None;
            },

            div {
                style: "background: {c.bg_card}; border-radius: 10px; padding: 0; min-width: 420px; max-width: 500px; width: 90%; border: 1px solid {c.border}; box-shadow: 0 12px 48px rgba(0,0,0,0.5); overflow: hidden;",
                onclick: move |evt| evt.stop_propagation(),

                // Header
                div {
                    style: "padding: 18px 20px 14px; border-bottom: 1px solid {c.border}; display: flex; align-items: center; justify-content: space-between;",
                    h3 {
                        style: "font-size: 16px; color: {c.text_white}; margin: 0;",
                        "Partager : {file_name}"
                    }
                    button {
                        style: "background: none; border: none; color: {c.text_muted}; cursor: pointer; font-size: 18px; padding: 4px;",
                        aria_label: "Fermer le dialog",
                        onclick: move |_| {
                            state.write().share_dialog_open = false;
                            state.write().generated_share_url = None;
                        },
                        "\u{2715}"
                    }
                }

                // Tab bar
                div {
                    style: "display: flex; padding: 8px 20px 0; gap: 4px;",
                    button {
                        style: "flex: 1; padding: 8px 12px; background: {link_tab_bg}; color: {link_tab_color}; border: none; border-radius: 6px 6px 0 0; cursor: {link_tab_cursor}; font-size: 13px; font-weight: 500;",
                        disabled: !external_links_available,
                        onclick: move |_| {
                            if external_links_available {
                                active_tab.set(ShareTab::ExternalLink);
                            }
                        },
                        "Lien externe"
                    }
                    button {
                        style: "flex: 1; padding: 8px 12px; background: {tribe_tab_bg}; color: {tribe_tab_color}; border: none; border-radius: 6px 6px 0 0; cursor: {tribe_tab_cursor}; font-size: 13px; font-weight: 500;",
                        disabled: !internal_sharing_enabled,
                        onclick: move |_| {
                            if internal_sharing_enabled {
                                active_tab.set(ShareTab::TribeShare);
                            }
                        },
                        "Partage Tribu"
                    }
                }

                // Content
                div {
                    style: "padding: 16px 20px 20px;",

                    // Error
                    if let Some(ref err) = current_error {
                        div {
                            style: "padding: 8px 12px; background: {c.accent_red}20; border-left: 3px solid {c.accent_red}; border-radius: 4px; color: {c.accent_red}; font-size: 13px; margin-bottom: 12px;",
                            "{err}"
                        }
                    }

                    if link_tab_active {
                        if let Some(notice) = external_link_notice {
                            div {
                                style: "padding: 8px 12px; background: {c.bg_main}; border-left: 3px solid {c.accent_blue}; border-radius: 4px; color: {c.text_secondary}; font-size: 13px; margin-bottom: 12px;",
                                "{notice}"
                            }
                        }
                    }

                    if tribe_tab_active {
                        if let Some(notice) = tribe_share_notice {
                            div {
                                style: "padding: 8px 12px; background: {c.bg_main}; border-left: 3px solid {c.accent_blue}; border-radius: 4px; color: {c.text_secondary}; font-size: 13px; margin-bottom: 12px;",
                                "{notice}"
                            }
                        }
                    }

                    // External link tab
                    if link_tab_active {
                        // Generated URL display
                        if has_generated_url {
                            div {
                                style: "display: flex; flex-direction: column; gap: 10px; margin-bottom: 16px;",
                                label {
                                    style: "font-size: 12px; color: {c.text_muted}; text-transform: uppercase; letter-spacing: 0.3px;",
                                    "Lien de partage"
                                }
                                div {
                                    style: "display: flex; gap: 8px;",
                                    input {
                                        style: "flex: 1; padding: 10px 12px; background: {c.bg_secondary}; border: 1px solid {c.border}; border-radius: 4px; color: {c.accent_green}; font-size: 13px; font-family: monospace; box-sizing: border-box;",
                                        r#type: "text",
                                        readonly: true,
                                        value: "{generated_url.clone().unwrap_or_default()}",
                                        aria_label: "Lien de partage genere",
                                    }
                                    button {
                                        style: "padding: 8px 14px; background: {c.accent_blue}; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 13px; white-space: nowrap;",
                                        onclick: {
                                            let url_to_copy = generated_url.clone().unwrap_or_default();
                                            move |_| {
                                                copy_to_clipboard(&url_to_copy);
                                            }
                                        },
                                        "Copier"
                                    }
                                }
                                p {
                                    style: "font-size: 11px; color: {c.accent_green}; margin: 0;",
                                    "Lien cree avec succes. Partagez-le avec le destinataire."
                                }
                            }
                        }

                        // Expiration
                        div {
                            style: "margin-bottom: 14px;",
                            label {
                                style: "font-size: 12px; color: {c.text_muted}; text-transform: uppercase; letter-spacing: 0.3px; display: block; margin-bottom: 6px;",
                                "Duree de validite"
                            }
                            div {
                                style: "display: flex; flex-wrap: wrap; gap: 6px;",
                                for (i, opt) in EXPIRATION_OPTIONS.iter().enumerate() {
                                    {
                                        let is_selected = i == exp_idx;
                                        let chip_bg = if is_selected { c.accent_blue } else { c.bg_secondary };
                                        let chip_color = if is_selected { c.text_white } else { c.text_secondary };
                                        let chip_border = if is_selected {
                                            format!("1px solid {}", c.accent_blue)
                                        } else {
                                            format!("1px solid {}", c.border)
                                        };
                                        let label_text = opt.label;

                                        rsx! {
                                            button {
                                                style: "padding: 6px 12px; background: {chip_bg}; color: {chip_color}; border: {chip_border}; border-radius: 16px; cursor: pointer; font-size: 12px;",
                                                onclick: move |_| expiration_index.set(i),
                                                "{label_text}"
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // Password toggle
                        div {
                            style: "margin-bottom: 14px;",
                            div {
                                style: "display: flex; align-items: center; gap: 8px; margin-bottom: 6px;",
                                input {
                                    r#type: "checkbox",
                                    checked: is_password,
                                    oninput: move |evt| {
                                        let val = evt.value() == "true";
                                        password_enabled.set(val);
                                    },
                                    id: "password-toggle",
                                    aria_label: "Activer le mot de passe",
                                }
                                label {
                                    r#for: "password-toggle",
                                    style: "font-size: 13px; color: {c.text_primary}; cursor: pointer;",
                                    "Proteger par un mot de passe"
                                }
                            }
                            if is_password {
                                input {
                                    style: "width: 100%; padding: 8px 12px; background: {c.bg_secondary}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_primary}; font-size: 13px; box-sizing: border-box;",
                                    r#type: "password",
                                    placeholder: "Mot de passe...",
                                    oninput: move |evt| password_value.set(evt.value()),
                                    aria_label: "Mot de passe du lien",
                                }
                            }
                        }

                        // Max downloads toggle
                        div {
                            style: "margin-bottom: 16px;",
                            div {
                                style: "display: flex; align-items: center; gap: 8px; margin-bottom: 6px;",
                                input {
                                    r#type: "checkbox",
                                    checked: is_max_dl,
                                    oninput: move |evt| {
                                        let val = evt.value() == "true";
                                        max_downloads_enabled.set(val);
                                    },
                                    id: "maxdl-toggle",
                                    aria_label: "Limiter le nombre de telechargements",
                                }
                                label {
                                    r#for: "maxdl-toggle",
                                    style: "font-size: 13px; color: {c.text_primary}; cursor: pointer;",
                                    "Limiter le nombre de telechargements"
                                }
                            }
                            if is_max_dl {
                                input {
                                    style: "width: 120px; padding: 8px 12px; background: {c.bg_secondary}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_primary}; font-size: 13px; box-sizing: border-box;",
                                    r#type: "number",
                                    min: "1",
                                    max: "1000",
                                    value: "{max_downloads_value.read()}",
                                    oninput: move |evt| max_downloads_value.set(evt.value()),
                                    aria_label: "Nombre maximum de telechargements",
                                }
                            }
                        }

                        // Create button
                        button {
                            style: "width: 100%; padding: 10px 16px; background: {c.accent_blue}; color: white; border: none; border-radius: 6px; cursor: {btn_cursor}; font-size: 14px; font-weight: 600; opacity: {btn_opacity};",
                            disabled: action_disabled,
                            onclick: move |_| {
                                if !external_links_available {
                                    error_msg.set(Some(
                                        share_availability_message(public_links_enabled, web_surface_enabled)
                                            .unwrap_or("Les liens externes sont indisponibles.".to_string())
                                    ));
                                    return;
                                }

                                let pwd_enabled = *password_enabled.read();
                                let pwd_val = password_value.read().clone();
                                let max_dl_on = *max_downloads_enabled.read();
                                let max_dl_val = max_downloads_value.read().clone();
                                let exp = EXPIRATION_OPTIONS[*expiration_index.read()].hours;

                                let file_id = state.read().share_target_file_id.clone()
                                    .or_else(|| state.read().selected_file.as_ref().map(|f| f.id.clone()));
                                let folder_id = state.read().share_target_folder_id.clone();

                                let password = if pwd_enabled && !pwd_val.is_empty() {
                                    Some(pwd_val)
                                } else {
                                    None
                                };

                                let max_downloads = if max_dl_on {
                                    max_dl_val.parse::<u32>().ok()
                                } else {
                                    None
                                };

                                let request = CreateShareLinkRequest {
                                    file_id,
                                    folder_id,
                                    password,
                                    expires_in_hours: exp,
                                    max_downloads,
                                };

                                creating.set(true);
                                error_msg.set(None);

                                spawn(async move {
                                    let http = {
                                        let c = client.read();
                                        c.clone()
                                    };
                                    let Some(http) = http else {
                                        error_msg.set(Some("Client non initialise".to_string()));
                                        creating.set(false);
                                        return;
                                    };

                                    match http.create_share_link(&request).await {
                                        Ok(link) => {
                                            // Construire l'URL de partage
                                            let share_url = format!(
                                                "https://127.0.0.1:{}/share/{}",
                                                web_surface_port,
                                                link.token
                                            );
                                            state.write().generated_share_url = Some(share_url);
                                            // Ajouter a la liste locale
                                            let mut links = state.read().share_links.clone();
                                            links.push(link);
                                            state.write().share_links = links;
                                            creating.set(false);
                                        }
                                        Err(e) => {
                                            error_msg.set(Some(format!("Echec : {e}")));
                                            creating.set(false);
                                        }
                                    }
                                });
                            },
                            if is_creating {
                                "Creation en cours..."
                            } else if !external_links_available {
                                "Liens externes indisponibles"
                            } else {
                                "Creer le lien"
                            }
                        }
                    }

                    // Tribe share tab
                    if tribe_tab_active {
                        div {
                            style: "display: flex; flex-direction: column; gap: 14px;",

                            // Grantee ID
                            div {
                                label {
                                    style: "font-size: 12px; color: {c.text_muted}; text-transform: uppercase; letter-spacing: 0.3px; display: block; margin-bottom: 6px;",
                                    "ID du destinataire"
                                }
                                input {
                                    style: "width: 100%; padding: 8px 12px; background: {c.bg_secondary}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_primary}; font-size: 13px; box-sizing: border-box;",
                                    r#type: "text",
                                    placeholder: "UUID du profil ou de la tribu...",
                                    oninput: move |evt| grantee_id.set(evt.value()),
                                    aria_label: "Identifiant du destinataire",
                                }
                            }

                            // Grantee type
                            div {
                                label {
                                    style: "font-size: 12px; color: {c.text_muted}; text-transform: uppercase; letter-spacing: 0.3px; display: block; margin-bottom: 6px;",
                                    "Type de destinataire"
                                }
                                div {
                                    style: "display: flex; gap: 8px;",
                                    {
                                        let gt = *grantee_type.read();
                                        let profile_bg = if gt == GranteeType::Profile { c.accent_blue } else { c.bg_secondary };
                                        let profile_color = if gt == GranteeType::Profile { c.text_white } else { c.text_secondary };
                                        let tribe_bg = if gt == GranteeType::Tribe { c.accent_blue } else { c.bg_secondary };
                                        let tribe_color = if gt == GranteeType::Tribe { c.text_white } else { c.text_secondary };

                                        rsx! {
                                            button {
                                                style: "flex: 1; padding: 8px; background: {profile_bg}; color: {profile_color}; border: 1px solid {c.border}; border-radius: 4px; cursor: pointer; font-size: 13px;",
                                                onclick: move |_| grantee_type.set(GranteeType::Profile),
                                                "Profil"
                                            }
                                            button {
                                                style: "flex: 1; padding: 8px; background: {tribe_bg}; color: {tribe_color}; border: 1px solid {c.border}; border-radius: 4px; cursor: pointer; font-size: 13px;",
                                                onclick: move |_| grantee_type.set(GranteeType::Tribe),
                                                "Tribu"
                                            }
                                        }
                                    }
                                }
                            }

                            // Permission level
                            div {
                                label {
                                    style: "font-size: 12px; color: {c.text_muted}; text-transform: uppercase; letter-spacing: 0.3px; display: block; margin-bottom: 6px;",
                                    "Niveau de permission"
                                }
                                div {
                                    style: "display: flex; gap: 8px;",
                                    {
                                        let pl = *permission_level.read();
                                        let read_bg = if pl == PermissionLevel::Read { c.accent_blue } else { c.bg_secondary };
                                        let read_color = if pl == PermissionLevel::Read { c.text_white } else { c.text_secondary };
                                        let rw_bg = if pl == PermissionLevel::ReadWrite { c.accent_blue } else { c.bg_secondary };
                                        let rw_color = if pl == PermissionLevel::ReadWrite { c.text_white } else { c.text_secondary };

                                        rsx! {
                                            button {
                                                style: "flex: 1; padding: 8px; background: {read_bg}; color: {read_color}; border: 1px solid {c.border}; border-radius: 4px; cursor: pointer; font-size: 13px;",
                                                onclick: move |_| permission_level.set(PermissionLevel::Read),
                                                "Lecture seule"
                                            }
                                            button {
                                                style: "flex: 1; padding: 8px; background: {rw_bg}; color: {rw_color}; border: 1px solid {c.border}; border-radius: 4px; cursor: pointer; font-size: 13px;",
                                                onclick: move |_| permission_level.set(PermissionLevel::ReadWrite),
                                                "Lecture + Ecriture"
                                            }
                                        }
                                    }
                                }
                            }

                            // Share button
                            button {
                                style: "width: 100%; padding: 10px 16px; background: {c.accent_blue}; color: white; border: none; border-radius: 6px; cursor: {btn_cursor}; font-size: 14px; font-weight: 600; opacity: {btn_opacity};",
                                disabled: action_disabled,
                                onclick: move |_| {
                                    if !internal_sharing_enabled {
                                        error_msg.set(Some(
                                            "Le partage interne est desactive dans le setup MiyuCloud.".to_string()
                                        ));
                                        return;
                                    }

                                    let gid = grantee_id.read().clone();
                                    if gid.is_empty() {
                                        error_msg.set(Some("Veuillez saisir l'ID du destinataire.".to_string()));
                                        return;
                                    }

                                    let file_id = state.read().share_target_file_id.clone()
                                        .or_else(|| state.read().selected_file.as_ref().map(|f| f.id.clone()));
                                    let folder_id = state.read().share_target_folder_id.clone();

                                    let request = CreateSharePermissionRequest {
                                        file_id,
                                        folder_id,
                                        grantee_id: gid,
                                        grantee_type: *grantee_type.read(),
                                        permission: *permission_level.read(),
                                    };

                                    creating.set(true);
                                    error_msg.set(None);

                                    spawn(async move {
                                        let http = {
                                            let c = client.read();
                                            c.clone()
                                        };
                                        let Some(http) = http else {
                                            error_msg.set(Some("Client non initialise".to_string()));
                                            creating.set(false);
                                            return;
                                        };

                                        match http.share_with(&request).await {
                                            Ok(_perm) => {
                                                creating.set(false);
                                                state.write().share_dialog_open = false;
                                            }
                                            Err(e) => {
                                                error_msg.set(Some(format!("Echec : {e}")));
                                                creating.set(false);
                                            }
                                        }
                                    });
                                },
                                if is_creating {
                                    "Partage en cours..."
                                } else if !internal_sharing_enabled {
                                    "Partage interne indisponible"
                                } else {
                                    "Partager"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Copie une chaine dans le presse-papier via l'API Dioxus/Clipboard.
/// Fallback : ne fait rien si le clipboard n'est pas disponible.
fn copy_to_clipboard(text: &str) {
    #[cfg(target_os = "windows")]
    {
        // On Windows desktop, use the clipboard crate via spawn_blocking
        let text = text.to_string();
        spawn(async move {
            let _ = tokio::task::spawn_blocking(move || {
                use std::process::Command;
                let _ = Command::new("cmd")
                    .args(["/C", &format!("echo {text}| clip")])
                    .output();
            })
            .await;
        });
    }
    #[cfg(not(target_os = "windows"))]
    {
        // Best-effort: log the URL
        let _ = text;
        tracing::info!("Share URL generated (clipboard not available on this platform)");
    }
}

fn share_availability_message(
    public_links_enabled: bool,
    web_surface_enabled: bool,
) -> Option<String> {
    if !public_links_enabled {
        Some("Les liens publics sont desactives dans le setup MiyuCloud.".to_string())
    } else if !web_surface_enabled {
        Some(
            "La surface web MiyuCloud est desactivee. Active-la dans les reglages avant de partager un lien."
                .to_string(),
        )
    } else {
        None
    }
}
