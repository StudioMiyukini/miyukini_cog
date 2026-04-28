//! Écran de connexion — login utilisateur COG.
//!
//! Flow :
//!   1. Etablir le bridge avec le COG Host (via SavedConnection)
//!   2. Negocier le handshake E2E (X25519 + ChaCha20-Poly1305)
//!   3. Authentifier l'utilisateur via /auth sur CentralRemote
//!   4. Transition vers Main

use dioxus::prelude::*;
use miyukini_cog_bridge::{CogBridge, ConnectionMode};

use crate::state::{AppScreen, AppState};

/// État de la connexion.
#[derive(Debug, Clone, PartialEq)]
enum ConnexionStatus {
    /// Formulaire visible, pas de connexion en cours.
    Idle,
    /// Etablissement du bridge en cours.
    ConnectingBridge,
    /// Handshake E2E en cours.
    NegotiatingE2e,
    /// Auth en cours.
    Authenticating,
    /// Erreur.
    Error(String),
}

/// Écran de connexion (email + mot de passe).
#[component]
pub fn ConnexionScreen() -> Element {
    let mut state = use_context::<Signal<AppState>>();
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut status = use_signal(|| ConnexionStatus::Idle);

    let current_status = status.read().clone();
    let is_loading = !matches!(current_status, ConnexionStatus::Idle | ConnexionStatus::Error(_));

    let saved = state.read().saved_connection.clone();
    let host_label = saved
        .as_ref()
        .map(|s| format!("{}:{}", s.host, s.port))
        .unwrap_or_else(|| "non configuré".to_string());

    rsx! {
        div {
            style: "flex: 1; display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 24px; gap: 16px;",

            h1 {
                style: "font-size: 24px; font-weight: bold; color: #7c3aed; margin: 0;",
                "Connexion COG"
            }

            // Indicateur du host
            div {
                style: "padding: 8px 16px; background: #1a1a2e; border: 1px solid #3d3d55; border-radius: 8px; font-size: 12px; color: #888; font-family: monospace;",
                "→ {host_label}"
            }

            // Email
            input {
                style: "width: 100%; max-width: 320px; padding: 14px; background: #2d2d44; border: 1px solid #3d3d55; border-radius: 8px; color: #e0e0e0; font-size: 16px;",
                r#type: "email",
                placeholder: "Email",
                value: "{email}",
                disabled: is_loading,
                oninput: move |evt| email.set(evt.value()),
            }

            // Mot de passe
            input {
                style: "width: 100%; max-width: 320px; padding: 14px; background: #2d2d44; border: 1px solid #3d3d55; border-radius: 8px; color: #e0e0e0; font-size: 16px;",
                r#type: "password",
                placeholder: "Mot de passe",
                value: "{password}",
                disabled: is_loading,
                oninput: move |evt| password.set(evt.value()),
            }

            // Statut / erreur
            match current_status.clone() {
                ConnexionStatus::ConnectingBridge => rsx! {
                    StatusLabel { text: "Connexion au COG Host...".to_string(), color: "#7c3aed".to_string() }
                },
                ConnexionStatus::NegotiatingE2e => rsx! {
                    StatusLabel { text: "Chiffrement E2E...".to_string(), color: "#7c3aed".to_string() }
                },
                ConnexionStatus::Authenticating => rsx! {
                    StatusLabel { text: "Authentification...".to_string(), color: "#7c3aed".to_string() }
                },
                ConnexionStatus::Error(msg) => rsx! {
                    p {
                        style: "color: #ef4444; font-size: 14px; text-align: center; max-width: 320px;",
                        "{msg}"
                    }
                },
                _ => rsx! {},
            }

            // Bouton connexion
            button {
                style: if is_loading {
                    "width: 100%; max-width: 320px; padding: 14px; background: #3d3d55; border: none; border-radius: 8px; color: #666; font-size: 16px; font-weight: bold; cursor: not-allowed;"
                } else {
                    "width: 100%; max-width: 320px; padding: 14px; background: #7c3aed; border: none; border-radius: 8px; color: white; font-size: 16px; font-weight: bold; cursor: pointer;"
                },
                disabled: is_loading,
                onclick: move |_| {
                    let email_val = email.read().clone();
                    let password_val = password.read().clone();
                    let saved_clone = state.read().saved_connection.clone();

                    if email_val.is_empty() || password_val.is_empty() {
                        status.set(ConnexionStatus::Error("Veuillez remplir tous les champs".to_string()));
                        return;
                    }

                    let Some(saved) = saved_clone else {
                        status.set(ConnexionStatus::Error("Aucune connexion sauvegardée".to_string()));
                        return;
                    };

                    let mut state_sig = state;
                    let mut status_sig = status;

                    spawn(async move {
                        status_sig.set(ConnexionStatus::ConnectingBridge);

                        // 1. Construire le ConnectionMode
                        let mode = match saved.mode.as_str() {
                            "whitelist" => ConnectionMode::Whitelist {
                                host: saved.host.clone(),
                                port: saved.port,
                                device_id: saved.credential.clone(),
                            },
                            "apikey" => ConnectionMode::ApiKey {
                                host: saved.host.clone(),
                                port: saved.port,
                                token: saved.credential.clone(),
                            },
                            other => {
                                status_sig.set(ConnexionStatus::Error(format!(
                                    "Mode {other} pas encore supporté"
                                )));
                                return;
                            }
                        };

                        let bridge = CogBridge::new(mode);

                        // 2. Etablir le bridge
                        if let Err(e) = bridge.connect().await {
                            status_sig.set(ConnexionStatus::Error(format!("Bridge: {e}")));
                            return;
                        }
                        state_sig.write().bridge_status = bridge.status().await;

                        // 3. Handshake E2E
                        status_sig.set(ConnexionStatus::NegotiatingE2e);
                        if let Err(e) = bridge.negotiate_e2e().await {
                            // Si E2E echoue, on continue quand meme en clair (mode degrade)
                            tracing::warn!("E2E handshake échoué (fallback clair): {e}");
                        }

                        // 4. Auth via /auth sur CentralRemote
                        status_sig.set(ConnexionStatus::Authenticating);
                        let auth_url = format!("{}/auth", bridge.base_url());
                        let client = reqwest::Client::new();
                        let auth_resp = client
                            .post(&auth_url)
                            .json(&serde_json::json!({
                                "email": email_val,
                                "password": password_val,
                            }))
                            .timeout(std::time::Duration::from_secs(10))
                            .send()
                            .await;

                        match auth_resp {
                            Ok(resp) => {
                                if let Ok(body) = resp.json::<serde_json::Value>().await {
                                    let success = body.get("success").and_then(|v| v.as_bool()).unwrap_or(false);
                                    if success {
                                        let display = body.get("user_display_name")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or(&email_val)
                                            .to_string();
                                        state_sig.write().user_display_name = Some(display);
                                        state_sig.write().screen = AppScreen::Main;
                                    } else {
                                        let err_msg = body.get("error")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or("Identifiants incorrects")
                                            .to_string();
                                        status_sig.set(ConnexionStatus::Error(err_msg));
                                    }
                                } else {
                                    status_sig.set(ConnexionStatus::Error("Réponse invalide du COG Host".to_string()));
                                }
                            }
                            Err(e) => {
                                status_sig.set(ConnexionStatus::Error(format!("Auth: {e}")));
                            }
                        }

                        // Stocker le bridge pour les prochaines requetes (bridge est drop ici,
                        // a stocker dans state pour un usage plus long)
                        let _ = bridge; // TODO: persister dans state
                    });
                },
                if is_loading { "Connexion..." } else { "Se connecter" }
            }

            // Bouton "changer de COG"
            button {
                style: "margin-top: 8px; padding: 8px 16px; background: transparent; border: none; color: #888; font-size: 13px; cursor: pointer; text-decoration: underline;",
                disabled: is_loading,
                onclick: move |_| {
                    let _ = crate::platform::clear_connection();
                    state.write().saved_connection = None;
                    state.write().screen = AppScreen::Pairing;
                },
                "Changer de COG Host"
            }
        }
    }
}

#[component]
fn StatusLabel(text: String, color: String) -> Element {
    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 10px;",
            div {
                style: "width: 16px; height: 16px; border: 2px solid #2d2d44; border-top: 2px solid {color}; border-radius: 50%; animation: spin 1s linear infinite;",
            }
            p {
                style: "color: {color}; font-size: 14px; margin: 0;",
                "{text}"
            }
        }
    }
}
