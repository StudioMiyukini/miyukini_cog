//! Parametres MiyuCloud dans Central.
//!
//! @id: miyucloud_settings
//! @do: display_and_manage_miyucloud_settings
//! @role: component
//! @layer: presentation
//! @human: Lise (Dev Front-End)
//!
//! Affiche les parametres de configuration MiyuCloud :
//! - Chemin de stockage + espace utilise/disponible
//! - Surface web : activer/desactiver, port, certificat TLS
//! - Passphrase : bouton "Changer la passphrase" (placeholder Phase C)
//! - Style coherent avec les settings des autres services

use dioxus::prelude::*;

use super::auth_security::{
    HealthDashboard, OnboardingWizard, RecoveryCodesModal, SessionList, TotpSetupWizard,
    TotpVerifyForm,
};
use super::client::MiyuCloudClient;
use super::components::format_size;
use super::state::MiyuCloudState;
use super::sync_status::SyncSettingsSection;
use crate::services::miyucloud_settings::{load_miyucloud_config, save_miyucloud_config};
use crate::state::use_app_state;

/// Vue des parametres MiyuCloud.
#[component]
pub fn CloudSettings(
    state: Signal<MiyuCloudState>,
    client: Signal<Option<MiyuCloudClient>>,
) -> Element {
    let c = use_app_state().read().current_theme.palette();

    // Lire les valeurs avant RSX
    let storage_path = state.read().storage_path.clone();
    let storage_path_display = if storage_path.is_empty() {
        "Non configure".to_string()
    } else {
        storage_path
    };

    let stats = state.read().storage_stats.clone();
    let quota = state.read().user_quota.clone();
    let web_enabled = state.read().web_surface_enabled;
    let web_port = state.read().web_surface_port;
    let wizard_completed = state.read().first_launch_completed;
    let public_links_enabled = state.read().public_share_links_enabled;
    let internal_sharing_enabled = state.read().internal_sharing_enabled;
    let default_share_permission = state.read().default_share_permission;
    let ssh_enabled = state.read().ssh_enabled;
    let ssh_host = state.read().ssh_host.clone();
    let ssh_port = state.read().ssh_port;
    let ssh_username = state.read().ssh_username.clone();
    let ssh_root_path = state.read().ssh_root_path.clone();
    let ssh_private_key_path = state.read().ssh_private_key_path.clone();
    let ssh_keepalive = state.read().ssh_keepalive;

    let total_files = stats.total_files;
    let total_folders = stats.total_folders;
    let total_size_label = format_size(stats.total_size_bytes);
    let trashed_files = stats.trashed_files;
    let trashed_size_label = format_size(stats.trashed_size_bytes);
    let active_shares = stats.active_shares;
    let sync_peers = stats.sync_peers;

    let quota_label = if let Some(ref q) = quota {
        if q.max_bytes == 0 {
            format!("{} utilises (illimite)", format_size(q.used_bytes))
        } else {
            let used = format_size(q.used_bytes);
            let max = format_size(q.max_bytes);
            let pct = if q.max_bytes > 0 {
                (q.used_bytes * 100 / q.max_bytes) as u32
            } else {
                0
            };
            format!("{used} / {max} ({pct}%)")
        }
    } else {
        "Non disponible".to_string()
    };

    let quota_bar_pct = if let Some(ref q) = quota {
        if q.max_bytes > 0 {
            let pct = (q.used_bytes * 100 / q.max_bytes) as u32;
            if pct > 100 {
                100
            } else {
                pct
            }
        } else {
            0
        }
    } else {
        0
    };
    let quota_bar_width = format!("{quota_bar_pct}%");
    let quota_bar_color = if quota_bar_pct > 90 {
        c.accent_red
    } else if quota_bar_pct > 70 {
        "#f59e0b"
    } else {
        c.accent_blue
    };
    let show_quota_bar = quota.as_ref().map_or(false, |q| q.max_bytes > 0);

    let web_status_label = if web_enabled { "Active" } else { "Desactive" };
    let web_status_color = if web_enabled {
        c.accent_green
    } else {
        c.text_muted
    };
    let web_port_label = format!("{web_port}");
    let setup_status_label = if wizard_completed {
        "Termine".to_string()
    } else {
        "Incomplet".to_string()
    };
    let ssh_profile_label = if ssh_enabled {
        let mut summary = format!(
            "{}@{}:{}",
            empty_to_placeholder(&ssh_username),
            empty_to_placeholder(&ssh_host),
            ssh_port
        );
        if !ssh_root_path.trim().is_empty() {
            summary.push_str(" -> ");
            summary.push_str(&ssh_root_path);
        }
        summary
    } else {
        "Desactive".to_string()
    };
    let ssh_key_label = if ssh_enabled && !ssh_private_key_path.trim().is_empty() {
        ssh_private_key_path.clone()
    } else if ssh_enabled {
        "Agent SSH / autre methode".to_string()
    } else {
        "Non configure".to_string()
    };
    let ssh_keepalive_label = if ssh_enabled {
        enabled_label(ssh_keepalive)
    } else {
        "Non applicable".to_string()
    };
    let pending_web_auth = state.read().web_auth_challenges.clone();

    let mut storage_path_input = use_signal(|| load_miyucloud_config().storage_path);
    let mut quota_gb_input =
        use_signal(|| quota_bytes_to_gb_string(load_miyucloud_config().quota_bytes));
    let mut web_port_input = use_signal(|| load_miyucloud_config().web_port.to_string());
    let mut web_enabled_input = use_signal(|| load_miyucloud_config().web_enabled);
    let web_toggle_bg = if *web_enabled_input.read() {
        c.accent_green
    } else {
        c.bg_hover
    };
    let web_toggle_label = if *web_enabled_input.read() {
        "Surface web activee"
    } else {
        "Surface web desactivee"
    };

    // Charger les stats au montage
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

            // Charger quota et stats en parallele
            if let Ok(q) = http.get_quota().await {
                state.write().user_quota = Some(q);
            }
            if let Ok(s) = http.get_storage_stats().await {
                state.write().storage_stats = s;
            }
            if let Ok(challenges) = http.list_web_auth_challenges().await {
                state.write().web_auth_challenges = challenges;
            }
        });
    });

    rsx! {
        div {
            style: "flex: 1; display: flex; flex-direction: column; overflow-y: auto; min-width: 0;",

            // Header
            div {
                style: "padding: 16px 20px; border-bottom: 1px solid {c.border}; display: flex; align-items: center; gap: 10px;",
                span { style: "font-size: 20px;", "\u{2699}" }
                h3 { style: "font-size: 16px; color: {c.text_white}; margin: 0;", "Parametres MiyuCloud" }
            }

            // Content
            div {
                style: "padding: 20px; display: flex; flex-direction: column; gap: 24px; max-width: 600px;",

                SettingsSection {
                    title: "Assistant initial",
                    icon: "\u{1F9ED}",

                    SettingsRow { label: "Etat du setup", value: setup_status_label }
                    SettingsRow {
                        label: "Liens publics",
                        value: enabled_label(public_links_enabled),
                    }
                    SettingsRow {
                        label: "Partage interne",
                        value: enabled_label(internal_sharing_enabled),
                    }
                    SettingsRow {
                        label: "Permission par defaut",
                        value: default_share_permission.label().to_string(),
                    }
                    SettingsRow { label: "Profil SSH", value: ssh_profile_label }
                    SettingsRow { label: "Cle SSH", value: ssh_key_label }
                    SettingsRow { label: "Keepalive SSH", value: ssh_keepalive_label }

                    div { style: "height: 1px; background: {c.border}; margin: 8px 0;" }

                    p {
                        style: "font-size: 12px; color: {c.text_muted}; margin: 0 0 12px; line-height: 1.5;",
                        "Relance l'assistant si tu veux redefinir la racine, la limite de stockage, les regles de partage ou le profil SSH persistant."
                    }
                    button {
                        style: "padding: 10px 14px; background: {c.accent_blue}; color: white; border: none; border-radius: 6px; cursor: pointer; font-size: 13px; font-weight: 600; align-self: flex-start;",
                        onclick: move |_| {
                            state.write().setup_wizard_open = true;
                        },
                        if wizard_completed { "Relancer l'assistant" } else { "Continuer le setup" }
                    }
                }

                // ═════════════════════════════════════════════════════════
                // Section : Stockage
                // ═════════════════════════════════════════════════════════
                SettingsSection {
                    title: "Stockage",
                    icon: "\u{1F4BE}",

                    SettingsRow { label: "Chemin de stockage", value: storage_path_display }
                    SettingsRow { label: "Quota", value: quota_label }

                    div { style: "height: 1px; background: {c.border}; margin: 8px 0;" }

                    div {
                        style: "display: flex; flex-direction: column; gap: 10px;",
                        label {
                            style: "display: flex; flex-direction: column; gap: 6px;",
                            span { style: "font-size: 12px; color: {c.text_secondary};", "Dossier MiyuCloud" }
                            div {
                                style: "display: flex; gap: 8px; align-items: center;",
                                input {
                                    value: "{storage_path_input}",
                                    oninput: move |evt| storage_path_input.set(evt.value()),
                                    style: "flex: 1; padding: 10px 12px; background: {c.bg_main}; color: {c.text_primary}; border: 1px solid {c.border}; border-radius: 6px; font-size: 13px;",
                                    placeholder: "C:\\Data\\MiyuCloud",
                                }
                                button {
                                    style: "padding: 10px 14px; background: {c.bg_hover}; color: {c.text_primary}; border: 1px solid {c.border}; border-radius: 6px; cursor: pointer; font-size: 13px; white-space: nowrap;",
                                    onclick: move |_| {
                                        spawn(async move {
                                            let folder = rfd::AsyncFileDialog::new()
                                                .set_title("Choisir le dossier de stockage MiyuCloud")
                                                .pick_folder()
                                                .await;
                                            if let Some(handle) = folder {
                                                let path = handle.path().to_string_lossy().to_string();
                                                storage_path_input.set(path);
                                            }
                                        });
                                    },
                                    "\u{1F4C2} Parcourir..."
                                }
                            }
                        }
                        label {
                            style: "display: flex; flex-direction: column; gap: 6px;",
                            span { style: "font-size: 12px; color: {c.text_secondary};", "Quota alloue (Go)" }
                            input {
                                value: "{quota_gb_input}",
                                oninput: move |evt| quota_gb_input.set(evt.value()),
                                style: "padding: 10px 12px; background: {c.bg_main}; color: {c.text_primary}; border: 1px solid {c.border}; border-radius: 6px; font-size: 13px;",
                                placeholder: "10",
                            }
                        }
                        p {
                            style: "font-size: 12px; color: {c.text_muted}; margin: 0; line-height: 1.5;",
                            "Le quota par defaut est de 10 Go. Un changement de dossier prend effet au prochain redemarrage du service."
                        }
                        button {
                            style: "padding: 10px 14px; background: {c.accent_blue}; color: white; border: none; border-radius: 6px; cursor: pointer; font-size: 13px; font-weight: 600; align-self: flex-start;",
                            onclick: move |_| {
                                let parsed_quota_gb = quota_gb_input.read().trim().parse::<u64>().unwrap_or(10);
                                let quota_bytes = parsed_quota_gb.saturating_mul(1024 * 1024 * 1024);
                                let mut config = load_miyucloud_config();
                                config.storage_path = storage_path_input.read().trim().to_string();
                                config.quota_bytes = quota_bytes;

                                match save_miyucloud_config(&config) {
                                    Ok(()) => {
                                        state.write().storage_path = config.storage_path.clone();
                                        state.write().configured_quota_bytes = quota_bytes;
                                        state.write().error_message = Some(
                                            "Configuration MiyuCloud enregistree. Le dossier sera repris au prochain redemarrage.".to_string()
                                        );

                                        spawn(async move {
                                            let http = {
                                                let c = client.read();
                                                c.clone()
                                            };
                                            let Some(http) = http else { return; };
                                            if let Ok(updated_quota) = http.set_quota(quota_bytes).await {
                                                state.write().user_quota = Some(updated_quota);
                                            }
                                        });
                                    }
                                    Err(err) => {
                                        state.write().error_message = Some(err);
                                    }
                                }
                            },
                            "Enregistrer stockage"
                        }
                    }

                    // Barre de quota
                    if show_quota_bar {
                        div {
                            style: "margin-top: 4px;",
                            div {
                                style: "height: 8px; background: {c.bg_hover}; border-radius: 4px; overflow: hidden;",
                                div {
                                    style: "height: 100%; width: {quota_bar_width}; background: {quota_bar_color}; border-radius: 4px; transition: width 0.3s;",
                                }
                            }
                        }
                    }

                    // Separateur
                    div { style: "height: 1px; background: {c.border}; margin: 8px 0;" }

                    // Stats
                    div {
                        style: "display: grid; grid-template-columns: 1fr 1fr; gap: 8px;",
                        StatCard { label: "Fichiers", value: format!("{total_files}"), color: c.accent_blue }
                        StatCard { label: "Dossiers", value: format!("{total_folders}"), color: c.accent_blue }
                        StatCard { label: "Taille totale", value: total_size_label, color: c.accent_blue }
                        StatCard { label: "En corbeille", value: format!("{trashed_files} ({trashed_size_label})"), color: c.text_muted }
                        StatCard { label: "Partages actifs", value: format!("{active_shares}"), color: c.accent_green }
                        StatCard { label: "Pairs sync", value: format!("{sync_peers}"), color: c.accent_green }
                    }
                }

                // ═════════════════════════════════════════════════════════
                // Section : Surface web
                // ═════════════════════════════════════════════════════════
                SettingsSection {
                    title: "Surface web",
                    icon: "\u{1F310}",

                    div {
                        style: "display: flex; align-items: center; gap: 10px; margin-bottom: 8px;",
                        span {
                            style: "display: inline-block; width: 10px; height: 10px; border-radius: 50%; background: {web_status_color};",
                        }
                        span {
                            style: "font-size: 14px; color: {c.text_primary}; font-weight: 500;",
                            "{web_status_label}"
                        }
                    }

                    SettingsRow { label: "Port HTTPS", value: web_port_label }
                    SettingsRow {
                        label: "Certificat TLS",
                        value: "Auto-signe (genere au premier demarrage)".to_string(),
                    }

                    div { style: "height: 1px; background: {c.border}; margin: 8px 0;" }

                    div {
                        style: "display: flex; flex-direction: column; gap: 10px;",
                        div {
                            style: "display: flex; align-items: center; gap: 10px;",
                            button {
                                style: "padding: 8px 12px; background: {web_toggle_bg}; color: {c.text_white}; border: 1px solid {c.border}; border-radius: 6px; cursor: pointer; font-size: 13px; font-weight: 600;",
                                onclick: move |_| {
                                    let next = !*web_enabled_input.read();
                                    web_enabled_input.set(next);
                                },
                                "{web_toggle_label}"
                            }
                            input {
                                value: "{web_port_input}",
                                oninput: move |evt| web_port_input.set(evt.value()),
                                style: "width: 120px; padding: 10px 12px; background: {c.bg_main}; color: {c.text_primary}; border: 1px solid {c.border}; border-radius: 6px; font-size: 13px;",
                                placeholder: "11442",
                            }
                        }
                        button {
                            style: "padding: 10px 14px; background: {c.accent_blue}; color: white; border: none; border-radius: 6px; cursor: pointer; font-size: 13px; font-weight: 600; align-self: flex-start;",
                            onclick: move |_| {
                                let mut config = load_miyucloud_config();
                                config.web_enabled = *web_enabled_input.read();
                                config.web_port = web_port_input.read().trim().parse::<u16>().unwrap_or(11442);

                                match save_miyucloud_config(&config) {
                                    Ok(()) => {
                                        state.write().web_surface_enabled = config.web_enabled;
                                        state.write().web_surface_port = config.web_port;
                                        state.write().error_message = Some(
                                            "Configuration web MiyuCloud enregistree. Relance le service pour reappliquer le port.".to_string()
                                        );
                                    }
                                    Err(err) => {
                                        state.write().error_message = Some(err);
                                    }
                                }
                            },
                            "Enregistrer surface web"
                        }
                    }

                    p {
                        style: "font-size: 12px; color: {c.text_muted}; margin: 8px 0 0; line-height: 1.5;",
                        "La surface web MiyuCloud sert le portail web authentifie du COG. Le certificat est auto-genere au premier demarrage."
                    }
                }

                // ═════════════════════════════════════════════════════════
                // Section : Securite
                // ═════════════════════════════════════════════════════════
                SettingsSection {
                    title: "Securite",
                    icon: "\u{1F512}",

                    SettingsRow {
                        label: "Chiffrement",
                        value: "ChaCha20-Poly1305 (at-rest)".to_string(),
                    }
                    SettingsRow {
                        label: "Derivation de cle",
                        value: "Argon2id (64 MB, 3 iterations)".to_string(),
                    }

                    div { style: "height: 1px; background: {c.border}; margin: 8px 0;" }

                    div {
                        style: "display: flex; align-items: center; gap: 12px;",

                        button {
                            style: "padding: 8px 16px; background: {c.bg_hover}; color: {c.text_primary}; border: 1px solid {c.border}; border-radius: 4px; cursor: pointer; font-size: 13px;",
                            aria_label: "Changer la passphrase",
                            onclick: move |_| {
                                // Placeholder Phase C
                                state.write().error_message = Some(
                                    "Changement de passphrase disponible en Phase C.".to_string()
                                );
                            },
                            "\u{1F511} Changer la passphrase"
                        }

                        span {
                            style: "font-size: 11px; color: {c.text_muted}; font-style: italic;",
                            "Phase C"
                        }
                    }

                    p {
                        style: "font-size: 12px; color: {c.text_muted}; margin: 8px 0 0; line-height: 1.5;",
                        "La passphrase protege vos fichiers. Elle n'est jamais stockee sur le disque. Tous les fichiers sont chiffres individuellement avec une cle derivee de votre passphrase."
                    }
                }

                // Onboarding
                SettingsSection {
                    title: "Onboarding",
                    icon: "\u{1F9ED}",
                    OnboardingWizard { state: state, client: client }
                }

                // 2FA + sessions
                SettingsSection {
                    title: "Authentification",
                    icon: "\u{1F510}",
                    TotpSetupWizard { state: state, client: client }
                    div { style: "height: 1px; background: {c.border}; margin: 8px 0;" }
                    TotpVerifyForm { state: state, client: client }
                    div { style: "height: 1px; background: {c.border}; margin: 8px 0;" }
                    SessionList { state: state, client: client }
                    div { style: "height: 1px; background: {c.border}; margin: 8px 0;" }
                    div {
                        style: "display: flex; align-items: center; justify-content: space-between; gap: 12px; margin-bottom: 10px;",
                        div {
                            span { style: "font-size: 13px; color: {c.text_primary}; font-weight: 600;", "Demandes QR portail web" }
                            p { style: "font-size: 12px; color: {c.text_muted}; margin: 4px 0 0;", "Approuve ou rejette les connexions web Miyukini Connect initiees via QR code." }
                        }
                        button {
                            style: "padding: 8px 12px; background: {c.bg_hover}; color: {c.text_primary}; border: 1px solid {c.border}; border-radius: 6px; cursor: pointer; font-size: 12px;",
                            onclick: move |_| {
                                spawn(async move {
                                    let http = {
                                        let c = client.read();
                                        c.clone()
                                    };
                                    let Some(http) = http else { return; };
                                    match http.list_web_auth_challenges().await {
                                        Ok(challenges) => state.write().web_auth_challenges = challenges,
                                        Err(err) => state.write().error_message = Some(format!("Chargement demandes QR: {err}")),
                                    }
                                });
                            },
                            "Rafraichir"
                        }
                    }
                    if pending_web_auth.is_empty() {
                        p {
                            style: "font-size: 12px; color: {c.text_muted}; margin: 0;",
                            "Aucune demande QR en attente."
                        }
                    } else {
                        div {
                            style: "display: flex; flex-direction: column; gap: 10px;",
                            for challenge in pending_web_auth {
                                div {
                                    key: "{challenge.id}",
                                    style: "display: flex; align-items: center; justify-content: space-between; gap: 12px; padding: 12px; background: {c.bg_main}; border: 1px solid {c.border}; border-radius: 8px;",
                                    div {
                                        p { style: "font-size: 13px; color: {c.text_primary}; margin: 0; font-weight: 600;", "{challenge.browser_label}" }
                                        p { style: "font-size: 11px; color: {c.text_muted}; margin: 4px 0 0;", "Creee le {challenge.created_at}" }
                                        p { style: "font-size: 11px; color: {c.text_muted}; margin: 2px 0 0;", "Expire le {challenge.expires_at}" }
                                    }
                                    div {
                                        style: "display: flex; gap: 8px;",
                                        button {
                                            style: "padding: 8px 12px; background: {c.accent_green}; color: white; border: none; border-radius: 6px; cursor: pointer; font-size: 12px; font-weight: 600;",
                                            onclick: {
                                                let challenge_id = challenge.id.clone();
                                                move |_| {
                                                let challenge_id = challenge_id.clone();
                                                spawn(async move {
                                                    let http = {
                                                        let c = client.read();
                                                        c.clone()
                                                    };
                                                    let Some(http) = http else { return; };
                                                    match http.approve_web_auth_challenge(&challenge_id).await {
                                                        Ok(()) => {
                                                            if let Ok(challenges) = http.list_web_auth_challenges().await {
                                                                state.write().web_auth_challenges = challenges;
                                                            }
                                                        }
                                                        Err(err) => state.write().error_message = Some(format!("Approbation QR: {err}")),
                                                    }
                                                });
                                            }},
                                            "Approuver"
                                        }
                                        button {
                                            style: "padding: 8px 12px; background: #7f1d1d; color: white; border: none; border-radius: 6px; cursor: pointer; font-size: 12px; font-weight: 600;",
                                            onclick: {
                                                let challenge_id = challenge.id.clone();
                                                move |_| {
                                                let challenge_id = challenge_id.clone();
                                                spawn(async move {
                                                    let http = {
                                                        let c = client.read();
                                                        c.clone()
                                                    };
                                                    let Some(http) = http else { return; };
                                                    match http.reject_web_auth_challenge(&challenge_id).await {
                                                        Ok(()) => {
                                                            if let Ok(challenges) = http.list_web_auth_challenges().await {
                                                                state.write().web_auth_challenges = challenges;
                                                            }
                                                        }
                                                        Err(err) => state.write().error_message = Some(format!("Rejet QR: {err}")),
                                                    }
                                                });
                                            }},
                                            "Rejeter"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Health dashboard
                SettingsSection {
                    title: "Sante",
                    icon: "\u{1FA7A}",
                    HealthDashboard { state: state, client: client }
                }

                // ═════════════════════════════════════════════════════════
                // Section : Synchronisation P2P
                // ═════════════════════════════════════════════════════════
                SettingsSection {
                    title: "Synchronisation P2P",
                    icon: "\u{1F504}",

                    SyncSettingsSection { state: state, client: client }
                }
            }

            RecoveryCodesModal { state: state }
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// Sub-components
// ════════════════════════════════════════════════════════════════════════════

/// Section de parametres avec titre et icone.
#[component]
fn SettingsSection(title: &'static str, icon: &'static str, children: Element) -> Element {
    let c = use_app_state().read().current_theme.palette();

    rsx! {
        section {
            style: "background: {c.bg_secondary}; border: 1px solid {c.border}; border-radius: 8px; padding: 16px 20px;",

            div {
                style: "display: flex; align-items: center; gap: 8px; margin-bottom: 14px;",
                span { style: "font-size: 18px;", "{icon}" }
                h4 { style: "font-size: 14px; color: {c.text_white}; margin: 0; font-weight: 600;", "{title}" }
            }

            {children}
        }
    }
}

/// Ligne label: valeur dans les settings.
#[component]
fn SettingsRow(label: &'static str, value: String) -> Element {
    let c = use_app_state().read().current_theme.palette();

    rsx! {
        div {
            style: "display: flex; justify-content: space-between; align-items: baseline; padding: 4px 0;",
            span {
                style: "font-size: 13px; color: {c.text_secondary};",
                "{label}"
            }
            span {
                style: "font-size: 13px; color: {c.text_primary}; font-weight: 500; text-align: right; word-break: break-word; max-width: 60%;",
                "{value}"
            }
        }
    }
}

/// Carte de statistique compacte.
#[component]
fn StatCard(label: String, value: String, color: &'static str) -> Element {
    let c = use_app_state().read().current_theme.palette();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 2px; padding: 8px 12px; background: {c.bg_main}; border-radius: 6px;",
            span { style: "font-size: 11px; color: {c.text_muted}; text-transform: uppercase; letter-spacing: 0.3px;", "{label}" }
            span { style: "font-size: 15px; color: {color}; font-weight: 600;", "{value}" }
        }
    }
}

fn quota_bytes_to_gb_string(bytes: u64) -> String {
    let gib = bytes / (1024 * 1024 * 1024);
    gib.max(1).to_string()
}

fn enabled_label(enabled: bool) -> String {
    if enabled {
        "Active".to_string()
    } else {
        "Desactive".to_string()
    }
}

fn empty_to_placeholder(value: &str) -> &str {
    if value.trim().is_empty() {
        "non-configure"
    } else {
        value
    }
}
