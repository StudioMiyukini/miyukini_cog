//! @id     central_mobile_screens_connexion
//! @do     definir_connexion_status
//! @role   ui
//! @layer  app
//! @human  central-mobile — module « screens connexion ».

//! Écran de connexion — session de l'appareil appairé.
//!
//! Flow :
//!   1. Construire le bridge (SavedConnection + identité de l'appareil)
//!   2. Negocier le handshake E2E v2 (identités épinglées à l'appairage)
//!   3. Passeport d'appareil → visa → validation (preuve de possession)
//!   4. Transition vers Main
//!
//! Plus de courriel ni de mot de passe (sortie du lot 2) : le titulaire est le
//! profil Central qui a approuvé l'appareil. Fail-closed (R-023, C-12) : si le
//! handshake E2E échoue, la connexion est abandonnée (voir [`crate::login`]).

use dioxus::prelude::*;

use crate::login;
use crate::state::{AppScreen, AppState};

/// État de la connexion.
#[derive(Debug, Clone, PartialEq)]
enum ConnexionStatus {
    /// Formulaire visible, pas de connexion en cours.
    Idle,
    /// Préparation du bridge en cours.
    ConnectingBridge,
    /// Handshake E2E puis passeport + visa en cours.
    NegotiatingE2e,
    /// Erreur.
    Error(String),
}

/// Écran de connexion (appareil appairé, sans mot de passe).
#[component]
pub fn ConnexionScreen() -> Element {
    let mut state = use_context::<Signal<AppState>>();
    let mut status = use_signal(|| ConnexionStatus::Idle);

    let current_status = status.read().clone();
    let is_loading = !matches!(current_status, ConnexionStatus::Idle | ConnexionStatus::Error(_));

    let saved = state.read().saved_connection.clone();
    let host_label = saved
        .as_ref()
        .map_or_else(|| "non configuré".to_string(), |s| format!("{}:{}", s.host, s.port));

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

            // Statut / erreur
            match current_status.clone() {
                ConnexionStatus::ConnectingBridge => rsx! {
                    StatusLabel { text: "Connexion au COG Host...".to_string(), color: "#7c3aed".to_string() }
                },
                ConnexionStatus::NegotiatingE2e => rsx! {
                    StatusLabel { text: "Chiffrement E2E et passeport de l'appareil...".to_string(), color: "#7c3aed".to_string() }
                },
                ConnexionStatus::Error(msg) => rsx! {
                    p {
                        style: "color: #ef4444; font-size: 14px; text-align: center; max-width: 320px;",
                        "{msg}"
                    }
                },
                ConnexionStatus::Idle => rsx! {},
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
                    let saved_clone = state.read().saved_connection.clone();

                    let Some(saved) = saved_clone else {
                        status.set(ConnexionStatus::Error("Aucune connexion sauvegardée".to_string()));
                        return;
                    };

                    let mut state_sig = state;
                    let mut status_sig = status;

                    spawn(async move {
                        status_sig.set(ConnexionStatus::ConnectingBridge);

                        // 1. Identités E2E (appairage) et bridge.
                        let device_identity = match crate::platform::load_e2e_identity() {
                            Ok(id) => id,
                            Err(e) => {
                                status_sig.set(ConnexionStatus::Error(e));
                                return;
                            }
                        };
                        let bridge = match login::build_bridge(&saved, device_identity) {
                            Ok(b) => b,
                            Err(e) => {
                                status_sig.set(ConnexionStatus::Error(e));
                                return;
                            }
                        };

                        // 2-3. Handshake E2E puis passeport + visa (aucun repli).
                        status_sig.set(ConnexionStatus::NegotiatingE2e);
                        match login::login(&bridge).await {
                            Ok(display) => {
                                state_sig.write().bridge_status = bridge.status().await;
                                state_sig.write().user_display_name = Some(display);
                                state_sig.write().screen = AppScreen::Main;
                            }
                            Err(e) => {
                                if let login::LoginError::E2eFailed(cause) = &e {
                                    tracing::warn!("Connexion abandonnée : handshake E2E échoué ({cause})");
                                }
                                status_sig.set(ConnexionStatus::Error(e.user_message()));
                            }
                        }

                        // Le bridge n'est pas encore conservé dans l'état
                        // (appels API suivants non câblés) : il est libéré ici,
                        // avec sa session E2E.
                        drop(bridge);
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

            // Régénération de l'identité (clé perdue, compromise ou révoquée).
            button {
                style: "padding: 8px 16px; background: transparent; border: none; color: #888; font-size: 13px; cursor: pointer; text-decoration: underline;",
                disabled: is_loading,
                onclick: move |_| {
                    state.write().screen = AppScreen::Identite;
                },
                "Régénérer l'identité de l'appareil"
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
