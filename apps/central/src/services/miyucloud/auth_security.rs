//! Composants 2FA, sessions, onboarding et health MiyuCloud.
//!
//! @id: miyucloud_auth_security_components
//! @do: provide_totp_sessions_onboarding_health_ui
//! @role: component
//! @layer: presentation

use dioxus::prelude::*;

use super::client::MiyuCloudClient;
use super::components::format_size;
use super::state::MiyuCloudState;
use crate::state::use_app_state;

/// Wizard de setup 2FA TOTP.
#[component]
pub fn TotpSetupWizard(
    state: Signal<MiyuCloudState>,
    client: Signal<Option<MiyuCloudClient>>,
) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let enabled = state.read().totp_enabled;
    let account_name = state.read().totp_account_name.clone();
    let otpauth = state.read().totp_otpauth_uri.clone();

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
            if let Ok(v) = http.get_totp_status().await {
                state.write().totp_enabled = v;
            }
        });
    });

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 10px;",
            div {
                style: "display: flex; align-items: center; gap: 8px;",
                span { style: "font-size: 12px; color: {c.text_secondary};", "Statut 2FA :" }
                span {
                    style: "font-size: 12px; color: {if enabled { c.accent_green } else { c.accent_red }}; font-weight: 600;",
                    if enabled { "ACTIVE" } else { "INACTIVE" }
                }
            }
            input {
                r#type: "text",
                value: "{account_name}",
                placeholder: "Compte TOTP (ex: user@miyucloud.local)",
                style: "padding: 8px 10px; background: {c.bg_main}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_primary};",
                oninput: move |evt| state.write().totp_account_name = evt.value(),
            }
            div {
                style: "display: flex; gap: 8px; flex-wrap: wrap;",
                button {
                    style: "padding: 8px 12px; background: {c.accent_blue}; color: {c.text_white}; border: none; border-radius: 4px; cursor: pointer;",
                    onclick: move |_| {
                        spawn(async move {
                            let http = {
                                let c = client.read();
                                c.clone()
                            };
                            let Some(http) = http else { return };
                            let account = {
                                let value = state.read().totp_account_name.trim().to_string();
                                if value.is_empty() { "owner@miyucloud.local".to_string() } else { value }
                            };
                            match http.setup_totp(&account).await {
                                Ok(data) => {
                                    state.write().totp_enabled = true;
                                    state.write().totp_otpauth_uri = Some(data.otpauth_uri);
                                    state.write().recovery_codes = data.recovery_codes;
                                    state.write().recovery_modal_open = true;
                                    state.write().error_message = None;
                                }
                                Err(e) => {
                                    state.write().error_message = Some(format!("Echec setup 2FA: {e}"));
                                }
                            }
                        });
                    },
                    "Configurer 2FA"
                }
                if enabled {
                    button {
                        style: "padding: 8px 12px; background: {c.bg_hover}; color: {c.text_primary}; border: 1px solid {c.border}; border-radius: 4px; cursor: pointer;",
                        onclick: move |_| {
                            spawn(async move {
                                let http = {
                                    let c = client.read();
                                    c.clone()
                                };
                                let Some(http) = http else { return };
                                match http.regenerate_recovery_codes().await {
                                    Ok(codes) => {
                                        state.write().recovery_codes = codes;
                                        state.write().recovery_modal_open = true;
                                    }
                                    Err(e) => {
                                        state.write().error_message = Some(format!("Echec regeneration recovery codes: {e}"));
                                    }
                                }
                            });
                        },
                        "Regenerer Codes"
                    }
                    button {
                        style: "padding: 8px 12px; background: {c.accent_red}; color: {c.text_white}; border: none; border-radius: 4px; cursor: pointer;",
                        onclick: move |_| {
                            spawn(async move {
                                let http = {
                                    let c = client.read();
                                    c.clone()
                                };
                                let Some(http) = http else { return };
                                match http.disable_totp().await {
                                    Ok(()) => {
                                        state.write().totp_enabled = false;
                                        state.write().totp_otpauth_uri = None;
                                    }
                                    Err(e) => {
                                        state.write().error_message = Some(format!("Echec desactivation 2FA: {e}"));
                                    }
                                }
                            });
                        },
                        "Desactiver 2FA"
                    }
                }
            }
            if let Some(uri) = otpauth {
                p {
                    style: "font-size: 11px; color: {c.text_muted}; margin: 0; word-break: break-all;",
                    "URI OTPAuth: {uri}"
                }
            }
        }
    }
}

/// Formulaire de verification TOTP ou recovery code.
#[component]
pub fn TotpVerifyForm(
    state: Signal<MiyuCloudState>,
    client: Signal<Option<MiyuCloudClient>>,
) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let code = state.read().totp_code_input.clone();
    let recovery = state.read().recovery_code_input.clone();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 10px;",
            div {
                style: "display: flex; gap: 8px; align-items: center;",
                input {
                    r#type: "text",
                    maxlength: 6,
                    value: "{code}",
                    placeholder: "Code TOTP (6 chiffres)",
                    style: "flex: 1; padding: 8px 10px; background: {c.bg_main}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_primary};",
                    oninput: move |evt| state.write().totp_code_input = evt.value(),
                }
                button {
                    style: "padding: 8px 12px; background: {c.accent_blue}; color: {c.text_white}; border: none; border-radius: 4px; cursor: pointer;",
                    onclick: move |_| {
                        spawn(async move {
                            let input = state.read().totp_code_input.trim().to_string();
                            if input.is_empty() {
                                return;
                            }
                            let http = {
                                let c = client.read();
                                c.clone()
                            };
                            let Some(http) = http else { return };
                            match http.verify_totp_code(&input, None).await {
                                Ok(true) => state.write().error_message = Some("Code TOTP valide.".to_string()),
                                Ok(false) => state.write().error_message = Some("Code TOTP invalide.".to_string()),
                                Err(e) => state.write().error_message = Some(format!("Erreur verification TOTP: {e}")),
                            }
                        });
                    },
                    "Verifier"
                }
            }
            div {
                style: "display: flex; gap: 8px; align-items: center;",
                input {
                    r#type: "text",
                    value: "{recovery}",
                    placeholder: "Recovery code",
                    style: "flex: 1; padding: 8px 10px; background: {c.bg_main}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_primary};",
                    oninput: move |evt| state.write().recovery_code_input = evt.value(),
                }
                button {
                    style: "padding: 8px 12px; background: {c.bg_hover}; color: {c.text_primary}; border: 1px solid {c.border}; border-radius: 4px; cursor: pointer;",
                    onclick: move |_| {
                        spawn(async move {
                            let input = state.read().recovery_code_input.trim().to_string();
                            if input.is_empty() {
                                return;
                            }
                            let http = {
                                let c = client.read();
                                c.clone()
                            };
                            let Some(http) = http else { return };
                            match http.verify_recovery_code(&input, None).await {
                                Ok(true) => state.write().error_message = Some("Recovery code valide.".to_string()),
                                Ok(false) => state.write().error_message = Some("Recovery code invalide.".to_string()),
                                Err(e) => state.write().error_message = Some(format!("Erreur verification recovery code: {e}")),
                            }
                        });
                    },
                    "Verifier Recovery"
                }
            }
        }
    }
}

/// Liste des sessions actives avec revocation.
#[component]
pub fn SessionList(
    state: Signal<MiyuCloudState>,
    client: Signal<Option<MiyuCloudClient>>,
) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let sessions = state.read().auth_sessions.clone();

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
            if let Ok(items) = http.list_auth_sessions().await {
                state.write().auth_sessions = items;
            }
        });
    });

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 8px;",
            div {
                style: "display: flex; gap: 8px;",
                button {
                    style: "padding: 6px 10px; background: {c.accent_blue}; color: {c.text_white}; border: none; border-radius: 4px; cursor: pointer;",
                    onclick: move |_| {
                        spawn(async move {
                            let http = {
                                let c = client.read();
                                c.clone()
                            };
                            let Some(http) = http else { return };
                            if http.create_auth_session().await.is_ok() {
                                if let Ok(items) = http.list_auth_sessions().await {
                                    state.write().auth_sessions = items;
                                }
                            }
                        });
                    },
                    "Nouvelle Session"
                }
                button {
                    style: "padding: 6px 10px; background: {c.accent_red}; color: {c.text_white}; border: none; border-radius: 4px; cursor: pointer;",
                    onclick: move |_| {
                        spawn(async move {
                            let http = {
                                let c = client.read();
                                c.clone()
                            };
                            let Some(http) = http else { return };
                            match http.revoke_all_auth_sessions().await {
                                Ok(()) => state.write().auth_sessions.clear(),
                                Err(e) => state.write().error_message = Some(format!("Echec revoke all sessions: {e}")),
                            }
                        });
                    },
                    "Revoquer Toutes"
                }
            }

            if sessions.is_empty() {
                p { style: "font-size: 12px; color: {c.text_muted}; margin: 0;", "Aucune session active." }
            }

            for sess in sessions.iter() {
                {
                    let session_id = sess.id.clone();
                    let sid_short = if session_id.len() > 10 { format!("{}...", &session_id[..10]) } else { session_id.clone() };
                    let verified = if sess.totp_verified { "2FA OK" } else { "2FA PENDING" };
                    let verified_color = if sess.totp_verified { c.accent_green } else { "#f59e0b" };
                    let created = format_short_date(&sess.created_at);
                    rsx! {
                        div {
                            style: "display: flex; align-items: center; justify-content: space-between; gap: 8px; padding: 8px 10px; border: 1px solid {c.border}; border-radius: 6px; background: {c.bg_main};",
                            div {
                                style: "display: flex; flex-direction: column; gap: 2px;",
                                span { style: "font-size: 12px; color: {c.text_primary}; font-family: monospace;", "{sid_short}" }
                                span { style: "font-size: 11px; color: {c.text_muted};", "Creee: {created}" }
                            }
                            div {
                                style: "display: flex; align-items: center; gap: 8px;",
                                span { style: "font-size: 11px; color: {verified_color};", "{verified}" }
                                button {
                                    style: "padding: 4px 8px; background: {c.bg_hover}; color: {c.text_primary}; border: 1px solid {c.border}; border-radius: 4px; cursor: pointer; font-size: 11px;",
                                    onclick: move |_| {
                                        let sid = session_id.clone();
                                        spawn(async move {
                                            let http = {
                                                let c = client.read();
                                                c.clone()
                                            };
                                            let Some(http) = http else { return };
                                            match http.revoke_auth_session(&sid).await {
                                                Ok(()) => {
                                                    let mut items = state.read().auth_sessions.clone();
                                                    items.retain(|s| s.id != sid);
                                                    state.write().auth_sessions = items;
                                                }
                                                Err(e) => state.write().error_message = Some(format!("Echec revoke session: {e}")),
                                            }
                                        });
                                    },
                                    "Revoquer"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Wizard onboarding (4 etapes).
#[component]
pub fn OnboardingWizard(
    state: Signal<MiyuCloudState>,
    client: Signal<Option<MiyuCloudClient>>,
) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let status = state.read().onboarding_status.clone();

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
            if let Ok(s) = http.get_onboarding_status().await {
                state.write().onboarding_status = Some(s);
            }
        });
    });

    let (step1, step2, step3, step4) = if let Some(s) = status.clone() {
        (
            s.passphrase_set,
            s.totp_set,
            s.storage_verified,
            s.completed,
        )
    } else {
        (false, false, false, false)
    };
    let current_step = status.as_ref().map_or(1, |s| s.current_step);

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 8px;",
            p { style: "font-size: 12px; color: {c.text_secondary}; margin: 0;", "Etape courante: {current_step}/4" }
            StepRow { label: "1. Passphrase verifiee".to_string(), done: step1 }
            StepRow { label: "2. 2FA TOTP configuree".to_string(), done: step2 }
            StepRow { label: "3. Stockage verifie".to_string(), done: step3 }
            StepRow { label: "4. Onboarding termine".to_string(), done: step4 }
            div {
                style: "display: flex; gap: 8px; margin-top: 4px;",
                button {
                    style: "padding: 6px 10px; background: {c.bg_hover}; color: {c.text_primary}; border: 1px solid {c.border}; border-radius: 4px; cursor: pointer;",
                    onclick: move |_| {
                        spawn(async move {
                            let http = {
                                let c = client.read();
                                c.clone()
                            };
                            let Some(http) = http else { return };
                            if let Ok(s) = http.get_onboarding_status().await {
                                state.write().onboarding_status = Some(s);
                            }
                        });
                    },
                    "Rafraichir"
                }
                button {
                    style: "padding: 6px 10px; background: {c.accent_blue}; color: {c.text_white}; border: none; border-radius: 4px; cursor: pointer;",
                    onclick: move |_| {
                        spawn(async move {
                            let http = {
                                let c = client.read();
                                c.clone()
                            };
                            let Some(http) = http else { return };
                            if http.complete_onboarding().await.is_ok() {
                                if let Ok(s) = http.get_onboarding_status().await {
                                    state.write().onboarding_status = Some(s);
                                }
                            }
                        });
                    },
                    "Marquer Termine"
                }
                button {
                    style: "padding: 6px 10px; background: {c.accent_red}; color: {c.text_white}; border: none; border-radius: 4px; cursor: pointer;",
                    onclick: move |_| {
                        spawn(async move {
                            let http = {
                                let c = client.read();
                                c.clone()
                            };
                            let Some(http) = http else { return };
                            if http.reset_onboarding().await.is_ok() {
                                if let Ok(s) = http.get_onboarding_status().await {
                                    state.write().onboarding_status = Some(s);
                                }
                            }
                        });
                    },
                    "Reinitialiser"
                }
            }
        }
    }
}

/// Dashboard de sante (health).
#[component]
pub fn HealthDashboard(
    state: Signal<MiyuCloudState>,
    client: Signal<Option<MiyuCloudClient>>,
) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let health = state.read().health_status.clone();

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
            if let Ok(h) = http.get_health_status().await {
                state.write().health_status = Some(h);
            }
        });
    });

    let (status_label, status_color) = if let Some(h) = &health {
        match h.status.as_str() {
            "healthy" => ("HEALTHY", c.accent_green),
            "degraded" => ("DEGRADED", "#f59e0b"),
            _ => ("UNHEALTHY", c.accent_red),
        }
    } else {
        ("UNKNOWN", c.text_muted)
    };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 10px;",
            div {
                style: "display: flex; align-items: center; gap: 8px;",
                span { style: "font-size: 12px; color: {c.text_secondary};", "Sante globale:" }
                span { style: "font-size: 12px; color: {status_color}; font-weight: 700;", "{status_label}" }
            }
            if let Some(h) = health {
                div {
                    style: "display: grid; grid-template-columns: 1fr 1fr; gap: 8px;",
                    HealthCell { label: "DB".to_string(), value: if h.db_accessible { "OK".to_string() } else { "KO".to_string() } }
                    HealthCell { label: "Storage".to_string(), value: if h.storage_accessible { "OK".to_string() } else { "KO".to_string() } }
                    HealthCell { label: "Fichiers".to_string(), value: h.metrics.total_files.to_string() }
                    HealthCell { label: "Sessions".to_string(), value: h.metrics.active_sessions.to_string() }
                    HealthCell { label: "Partages".to_string(), value: h.metrics.active_share_links.to_string() }
                    HealthCell { label: "Stockage".to_string(), value: format_size(h.total_size_bytes) }
                }
            }
            button {
                style: "width: fit-content; padding: 6px 10px; background: {c.bg_hover}; color: {c.text_primary}; border: 1px solid {c.border}; border-radius: 4px; cursor: pointer;",
                onclick: move |_| {
                    spawn(async move {
                        let http = {
                            let c = client.read();
                            c.clone()
                        };
                        let Some(http) = http else { return };
                        if let Ok(h) = http.get_health_status().await {
                            state.write().health_status = Some(h);
                        }
                    });
                },
                "Rafraichir Sante"
            }
        }
    }
}

/// Modal d'affichage des recovery codes.
#[component]
pub fn RecoveryCodesModal(state: Signal<MiyuCloudState>) -> Element {
    if !state.read().recovery_modal_open {
        return rsx! {};
    }

    let c = use_app_state().read().current_theme.palette();
    let codes = state.read().recovery_codes.clone();

    rsx! {
        div {
            style: "position: fixed; inset: 0; background: rgba(0,0,0,0.65); z-index: 1200; display: flex; align-items: center; justify-content: center;",
            onclick: move |_| state.write().recovery_modal_open = false,
            div {
                style: "width: min(560px, 92vw); background: {c.bg_card}; border: 1px solid {c.border}; border-radius: 10px; padding: 18px;",
                onclick: move |evt| evt.stop_propagation(),
                h4 { style: "margin: 0 0 10px; color: {c.text_white};", "Recovery Codes" }
                p { style: "margin: 0 0 10px; color: {c.text_secondary}; font-size: 12px;", "Conservez ces codes hors ligne. Chaque code est a usage unique." }
                div {
                    style: "display: grid; grid-template-columns: repeat(2, 1fr); gap: 8px; margin-bottom: 12px;",
                    for code in codes.iter() {
                        div {
                            style: "padding: 8px 10px; border: 1px dashed {c.border}; border-radius: 6px; font-family: monospace; color: {c.text_primary}; text-align: center;",
                            "{code}"
                        }
                    }
                }
                button {
                    style: "padding: 8px 12px; background: {c.accent_blue}; color: {c.text_white}; border: none; border-radius: 4px; cursor: pointer;",
                    onclick: move |_| state.write().recovery_modal_open = false,
                    "Fermer"
                }
            }
        }
    }
}

#[component]
fn StepRow(label: String, done: bool) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let color = if done { c.accent_green } else { c.text_muted };
    let icon = if done { "OK" } else { "..." };
    rsx! {
        div { style: "display: flex; align-items: center; gap: 8px;",
            span { style: "font-size: 11px; color: {color}; font-weight: 700; width: 28px;", "{icon}" }
            span { style: "font-size: 12px; color: {c.text_primary};", "{label}" }
        }
    }
}

#[component]
fn HealthCell(label: String, value: String) -> Element {
    let c = use_app_state().read().current_theme.palette();
    rsx! {
        div {
            style: "padding: 8px 10px; background: {c.bg_main}; border-radius: 6px; border: 1px solid {c.border};",
            div { style: "font-size: 10px; color: {c.text_muted}; text-transform: uppercase;", "{label}" }
            div { style: "font-size: 12px; color: {c.text_primary}; font-weight: 600;", "{value}" }
        }
    }
}

fn format_short_date(iso: &str) -> String {
    if iso.len() >= 16 {
        let date = &iso[..10];
        let time = &iso[11..16];
        return format!("{date} {time}");
    }
    iso.to_string()
}
