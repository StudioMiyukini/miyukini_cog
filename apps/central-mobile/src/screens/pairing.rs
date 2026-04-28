//! Écran d'appairage — connexion initiale au COG Host.
//!
//! Flow :
//!   Idle → [user clicks "WiFi"] → Searching (UDP broadcast 5s)
//!     → Found(host) → [user confirms] → Pairing (POST /api/bridge/pair)
//!       → poll /api/bridge/validate jusqu'a approbation
//!         → Approved → persiste SavedConnection → transition vers Connexion

use dioxus::prelude::*;
use miyukini_cog_bridge::discovery;
use std::net::SocketAddr;
use std::time::Duration;

use crate::platform;
use crate::state::{AppScreen, AppState, SavedConnection};

/// Étape du flow d'appairage.
#[derive(Debug, Clone, PartialEq)]
enum PairingStep {
    /// Choix du mode de connexion.
    Idle,
    /// Recherche du COG Host en LAN.
    Searching,
    /// COG Host trouve, en attente de confirmation utilisateur.
    Found {
        addr: SocketAddr,
        host_name: String,
        bridge_port: u16,
    },
    /// Demande d'appairage envoyee, en attente d'approbation sur le COG Host.
    Pairing { host: String, port: u16 },
    /// Erreur.
    Error(String),
}

/// Écran d'appairage avec le COG Host.
#[component]
pub fn PairingScreen() -> Element {
    let mut step = use_signal(|| PairingStep::Idle);
    let mut state = use_context::<Signal<AppState>>();

    let current_step = step.read().clone();

    rsx! {
        div {
            style: "flex: 1; display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 24px; gap: 20px;",

            // Logo + titre
            h1 {
                style: "font-size: 28px; font-weight: bold; color: #7c3aed; margin: 0;",
                "Miyukini Central"
            }

            match current_step {
                PairingStep::Idle => rsx! { IdleView { step_signal: step } },
                PairingStep::Searching => rsx! { SearchingView { step_signal: step } },
                PairingStep::Found { addr, host_name, bridge_port } => rsx! {
                    FoundView {
                        step_signal: step,
                        state_signal: state,
                        addr,
                        host_name,
                        bridge_port,
                    }
                },
                PairingStep::Pairing { host, port } => rsx! {
                    PairingView {
                        step_signal: step,
                        state_signal: state,
                        host,
                        port,
                    }
                },
                PairingStep::Error(msg) => rsx! {
                    ErrorView { step_signal: step, message: msg }
                },
            }
        }
    }
}

/// Vue initiale : 3 boutons.
#[component]
fn IdleView(step_signal: Signal<PairingStep>) -> Element {
    let mut step = step_signal;

    rsx! {
        p {
            style: "color: #a0a0b0; text-align: center; font-size: 16px; margin: 0 0 16px 0;",
            "Connectez-vous à votre COG Host"
        }

        button {
            style: "width: 100%; max-width: 320px; padding: 16px; background: #2d2d44; border: 1px solid #3d3d55; border-radius: 12px; color: #e0e0e0; font-size: 16px; cursor: pointer;",
            onclick: move |_| {
                step.set(PairingStep::Searching);
            },
            div { style: "font-weight: bold;", "Rechercher sur le WiFi" }
            div { style: "font-size: 13px; color: #888;", "Découverte automatique en LAN" }
        }

        button {
            style: "width: 100%; max-width: 320px; padding: 16px; background: #2d2d44; border: 1px solid #3d3d55; border-radius: 12px; color: #e0e0e0; font-size: 16px; cursor: pointer; opacity: 0.6;",
            onclick: move |_| {
                step.set(PairingStep::Error("QR code pas encore implémenté".to_string()));
            },
            div { style: "font-weight: bold;", "Scanner un QR Code" }
            div { style: "font-size: 13px; color: #888;", "Clé API (bientôt)" }
        }

        button {
            style: "width: 100%; max-width: 320px; padding: 16px; background: #2d2d44; border: 1px solid #3d3d55; border-radius: 12px; color: #e0e0e0; font-size: 16px; cursor: pointer; opacity: 0.6;",
            onclick: move |_| {
                step.set(PairingStep::Error("SSH pas encore implémenté".to_string()));
            },
            div { style: "font-weight: bold;", "Connexion SSH" }
            div { style: "font-size: 13px; color: #888;", "Hors LAN (bientôt)" }
        }
    }
}

/// Vue de recherche LAN en cours.
#[component]
fn SearchingView(step_signal: Signal<PairingStep>) -> Element {
    let mut step = step_signal;

    // Lancer la découverte en tâche asynchrone
    use_effect(move || {
        spawn(async move {
            tracing::info!("Démarrage découverte LAN...");
            match discovery::discover_lan(Duration::from_secs(5)).await {
                Ok((addr, response)) => {
                    tracing::info!("COG Host trouvé: {} à {addr}", response.host_name);
                    step.set(PairingStep::Found {
                        addr,
                        host_name: response.host_name,
                        bridge_port: response.bridge_port,
                    });
                }
                Err(e) => {
                    tracing::warn!("Découverte échouée: {e}");
                    step.set(PairingStep::Error(format!("Aucun COG Host trouvé sur le WiFi.\nVérifiez que Central Desktop est lancé et que le Remote est activé.")));
                }
            }
        });
    });

    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; gap: 16px;",

            // Spinner simple
            div {
                style: "width: 48px; height: 48px; border: 4px solid #2d2d44; border-top: 4px solid #7c3aed; border-radius: 50%; animation: spin 1s linear infinite;",
            }
            style { "@keyframes spin {{ from {{ transform: rotate(0deg); }} to {{ transform: rotate(360deg); }} }}" }

            p {
                style: "color: #e0e0e0; font-size: 16px;",
                "Recherche du COG Host..."
            }
            p {
                style: "color: #888; font-size: 13px;",
                "Broadcast UDP sur le WiFi"
            }
        }
    }
}

/// Vue de confirmation quand un host est trouve.
#[component]
fn FoundView(
    step_signal: Signal<PairingStep>,
    state_signal: Signal<AppState>,
    addr: SocketAddr,
    host_name: String,
    bridge_port: u16,
) -> Element {
    let mut step = step_signal;
    let host_ip = addr.ip().to_string();
    let host_name_display = host_name.clone();

    rsx! {
        div {
            style: "width: 100%; max-width: 320px; padding: 20px; background: #1a1a2e; border: 1px solid #22c55e; border-radius: 12px; text-align: center;",

            div {
                style: "font-size: 48px; margin-bottom: 12px;",
                "✓"
            }

            h2 {
                style: "color: #22c55e; font-size: 18px; margin: 0 0 4px 0;",
                "COG Host trouvé"
            }

            p {
                style: "color: #e0e0e0; font-size: 16px; font-weight: bold; margin: 12px 0 4px 0;",
                "{host_name_display}"
            }

            p {
                style: "color: #888; font-size: 13px; font-family: monospace; margin: 0;",
                "{host_ip}:{bridge_port}"
            }
        }

        button {
            style: "width: 100%; max-width: 320px; padding: 14px; background: #7c3aed; border: none; border-radius: 10px; color: white; font-size: 16px; font-weight: bold; cursor: pointer;",
            onclick: move |_| {
                step.set(PairingStep::Pairing {
                    host: host_ip.clone(),
                    port: bridge_port,
                });
            },
            "Appairer ce device"
        }

        button {
            style: "width: 100%; max-width: 320px; padding: 12px; background: transparent; border: 1px solid #3d3d55; border-radius: 10px; color: #888; font-size: 14px; cursor: pointer;",
            onclick: move |_| {
                step.set(PairingStep::Idle);
            },
            "Annuler"
        }
    }
}

/// Vue en attente d'approbation sur le COG Host.
#[component]
fn PairingView(
    step_signal: Signal<PairingStep>,
    state_signal: Signal<AppState>,
    host: String,
    port: u16,
) -> Element {
    let mut step = step_signal;
    let mut state = state_signal;

    // Envoyer la demande d'appairage, puis poll jusqu'a approbation
    use_effect(move || {
        let host_clone = host.clone();
        spawn(async move {
            let device_id = platform::get_device_id();
            let device_name = format!("Android-{}", &device_id[..8]);

            let pair_url = format!("http://{host_clone}:{port}/api/bridge/pair");
            let validate_url = format!("http://{host_clone}:{port}/api/bridge/validate");

            let client = reqwest::Client::new();

            // 1. Envoyer la demande d'appairage
            let pair_resp = client
                .post(&pair_url)
                .json(&serde_json::json!({
                    "device_id": device_id,
                    "device_name": device_name,
                    "mode": "whitelist",
                }))
                .timeout(Duration::from_secs(10))
                .send()
                .await;

            match pair_resp {
                Ok(resp) => {
                    tracing::info!("Pair request envoyée: {:?}", resp.status());
                }
                Err(e) => {
                    step.set(PairingStep::Error(format!("Erreur de connexion: {e}")));
                    return;
                }
            }

            // 2. Poller /api/bridge/validate jusqu'a approbation (max 60s)
            let device_id_for_loop = device_id.clone();
            for attempt in 0..30 {
                tokio::time::sleep(Duration::from_secs(2)).await;

                let validate_resp = client
                    .post(&validate_url)
                    .json(&serde_json::json!({ "device_id": device_id_for_loop }))
                    .timeout(Duration::from_secs(5))
                    .send()
                    .await;

                if let Ok(resp) = validate_resp {
                    if resp.status().is_success() {
                        tracing::info!("Device approuvé apres {} tentatives", attempt + 1);
                        // Sauvegarder et transitionner
                        let saved = SavedConnection {
                            mode: "whitelist".to_string(),
                            host: host_clone.clone(),
                            port,
                            credential: device_id.clone(),
                        };
                        if let Err(e) = platform::save_connection(&saved) {
                            tracing::warn!("Persistance connection échouée: {e}");
                        }
                        state.write().saved_connection = Some(saved);
                        state.write().screen = AppScreen::Connexion;
                        return;
                    }
                }
            }

            step.set(PairingStep::Error(
                "Délai dépassé. Approuvez le device sur Central Desktop.".to_string(),
            ));
        });
    });

    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; gap: 16px; max-width: 320px;",

            div {
                style: "width: 48px; height: 48px; border: 4px solid #2d2d44; border-top: 4px solid #7c3aed; border-radius: 50%; animation: spin 1s linear infinite;",
            }

            h2 {
                style: "color: #e0e0e0; font-size: 18px; margin: 0; text-align: center;",
                "Approbation en attente"
            }

            p {
                style: "color: #888; font-size: 14px; text-align: center; line-height: 1.5;",
                "Ouvrez Central Desktop sur votre PC\net approuvez ce device dans le panneau des devices mobiles."
            }

            button {
                style: "margin-top: 20px; padding: 10px 20px; background: transparent; border: 1px solid #3d3d55; border-radius: 8px; color: #888; font-size: 13px; cursor: pointer;",
                onclick: move |_| {
                    step.set(PairingStep::Idle);
                },
                "Annuler"
            }
        }
    }
}

/// Vue d'erreur.
#[component]
fn ErrorView(step_signal: Signal<PairingStep>, message: String) -> Element {
    let mut step = step_signal;

    rsx! {
        div {
            style: "width: 100%; max-width: 320px; padding: 20px; background: #1a0f0f; border: 1px solid #ef4444; border-radius: 12px; text-align: center;",

            div {
                style: "font-size: 48px; margin-bottom: 12px;",
                "✗"
            }

            p {
                style: "color: #ef4444; font-size: 14px; white-space: pre-line; margin: 0;",
                "{message}"
            }
        }

        button {
            style: "width: 100%; max-width: 320px; padding: 12px; background: #7c3aed; border: none; border-radius: 10px; color: white; font-size: 15px; font-weight: bold; cursor: pointer;",
            onclick: move |_| {
                step.set(PairingStep::Idle);
            },
            "Retour"
        }
    }
}
