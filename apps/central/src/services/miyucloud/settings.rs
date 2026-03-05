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

                // ═════════════════════════════════════════════════════════
                // Section : Stockage
                // ═════════════════════════════════════════════════════════
                SettingsSection {
                    title: "Stockage",
                    icon: "\u{1F4BE}",

                    SettingsRow { label: "Chemin de stockage", value: storage_path_display }
                    SettingsRow { label: "Quota", value: quota_label }

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

                    p {
                        style: "font-size: 12px; color: {c.text_muted}; margin: 8px 0 0; line-height: 1.5;",
                        "La surface web permet de partager des fichiers avec des destinataires exterieurs au COG via un navigateur. Elle est accessible sur le port configure en HTTPS."
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
