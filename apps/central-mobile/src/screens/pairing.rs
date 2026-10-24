//! @id     central_mobile_screens_pairing
//! @do     definir_pairing_step
//! @role   ui
//! @layer  app
//! @human  central-mobile — module « screens pairing ».

//! Écran d'appairage — connexion initiale au COG Host.
//!
//! Flow :
//!   Idle → [user clicks "WiFi"] → Searching (UDP broadcast 5s)
//!     → Found(host) → [user confirms] → Pairing (POST /api/bridge/pair)
//!       → poll /api/bridge/validate jusqu'a approbation
//!         → Approved → persiste SavedConnection → transition vers Connexion
//!
//! E2E (R-023) : la demande d'appairage transporte la clé publique d'identité
//! E2E de l'appareil (générée une fois, stockée en `0600`) ; la réponse porte
//! la clé publique du COG, épinglée dans la `SavedConnection` à l'approbation.
//! Un code de confirmation à 6 chiffres, calculé localement à partir des deux
//! clés, est affiché pour comparaison avec le desktop avant approbation.

use dioxus::prelude::*;
use miyukini_cog_bridge::{
    discovery, pairing_confirmation_code, parse_identity_key_hex, E2eIdentity,
};
use serde::Deserialize;
use std::net::SocketAddr;
use std::time::Duration;

/// Taille maximale de la réponse de `/api/bridge/pair` (C-13).
const MAX_PAIR_REPLY_LEN: usize = 4096;

/// Réponse de `/api/bridge/pair` (champs utiles).
#[derive(Deserialize)]
struct PairReply {
    status: String,
    cog_identity_key: String,
}

/// Clé du COG à épingler et code de confirmation calculé localement.
#[derive(Debug, Clone, PartialEq, Eq)]
struct PinnedCog {
    cog_identity_key_hex: String,
    confirmation_code: String,
}

/// Décode la réponse d'appairage : clé du COG valide (point ed25519 non
/// faible) et statut connu, sinon refus. Le code de confirmation est calculé
/// localement (jamais repris de la réponse, qui pourrait être falsifiée).
fn parse_pair_reply(body: &[u8], device_key: &[u8; 32]) -> Result<PinnedCog, String> {
    if body.len() > MAX_PAIR_REPLY_LEN {
        return Err("Réponse d'appairage trop grande".into());
    }
    let reply: PairReply = serde_json::from_slice(body).map_err(|_| {
        "Réponse d'appairage invalide (COG Host sans E2E v2 ?)".to_string()
    })?;
    if reply.status != "pending" && reply.status != "accepted" {
        return Err(format!("Appairage refusé ({})", reply.status));
    }
    let cog_key = parse_identity_key_hex(&reply.cog_identity_key)
        .map_err(|_| "Clé du COG Host invalide".to_string())?;
    if &cog_key == device_key {
        return Err("Clé du COG Host invalide".into());
    }
    Ok(PinnedCog {
        cog_identity_key_hex: hex::encode(cog_key),
        confirmation_code: pairing_confirmation_code(device_key, &cog_key),
    })
}

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
    let step = use_signal(|| PairingStep::Idle);
    let state = use_context::<Signal<AppState>>();

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
                    step.set(PairingStep::Error("Aucun COG Host trouvé sur le WiFi.\nVérifiez que Central Desktop est lancé et que le Remote est activé.".to_string()));
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

    // Code de confirmation affiché pendant l'attente d'approbation.
    let mut confirmation = use_signal(|| None::<String>);

    // Envoyer la demande d'appairage, puis poll jusqu'a approbation
    use_effect(move || {
        let host_clone = host.clone();
        spawn(async move {
            // Identité E2E de l'appareil (générée une fois, stockée 0600).
            let identity: E2eIdentity = match platform::load_or_create_e2e_identity() {
                Ok(id) => id,
                Err(e) => {
                    step.set(PairingStep::Error(format!("Identité E2E indisponible : {e}")));
                    return;
                }
            };
            let device_key = identity.public_key_bytes();
            drop(identity);

            let device_id = platform::get_device_id();
            let short_id: String = device_id.chars().take(8).collect();
            let device_name = format!("Android-{short_id}");

            let pair_url = format!("http://{host_clone}:{port}/api/bridge/pair");
            let validate_url = format!("http://{host_clone}:{port}/api/bridge/validate");

            let client = reqwest::Client::new();

            // 1. Envoyer la demande d'appairage (avec la clé publique E2E)
            let pair_resp = client
                .post(&pair_url)
                .json(&serde_json::json!({
                    "device_id": device_id,
                    "device_name": device_name,
                    "mode": "whitelist",
                    "e2e_identity_key": hex::encode(device_key),
                }))
                .timeout(Duration::from_secs(10))
                .send()
                .await;

            let resp = match pair_resp {
                Ok(resp) if resp.status().is_success() => resp,
                Ok(resp) => {
                    step.set(PairingStep::Error(format!(
                        "Appairage refusé par le COG Host (HTTP {})",
                        resp.status()
                    )));
                    return;
                }
                Err(e) => {
                    step.set(PairingStep::Error(format!("Erreur de connexion: {e}")));
                    return;
                }
            };
            if resp
                .content_length()
                .is_some_and(|n| n > MAX_PAIR_REPLY_LEN as u64)
            {
                step.set(PairingStep::Error("Réponse d'appairage trop grande".into()));
                return;
            }
            let body = match resp.bytes().await {
                Ok(b) => b,
                Err(e) => {
                    step.set(PairingStep::Error(format!("Erreur de connexion: {e}")));
                    return;
                }
            };
            let pinned = match parse_pair_reply(&body, &device_key) {
                Ok(p) => p,
                Err(e) => {
                    step.set(PairingStep::Error(e));
                    return;
                }
            };
            confirmation.set(Some(pinned.confirmation_code.clone()));

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
                        // Sauvegarder (clé du COG épinglée) et transitionner
                        let saved = SavedConnection {
                            mode: "whitelist".to_string(),
                            host: host_clone.clone(),
                            port,
                            credential: device_id.clone(),
                            cog_identity_key: Some(pinned.cog_identity_key_hex.clone()),
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

            if let Some(code) = confirmation.read().clone() {
                p {
                    style: "color: #e0e0e0; font-size: 14px; text-align: center; margin: 0;",
                    "Code de confirmation (doit être identique sur Central Desktop) :"
                }
                p {
                    style: "color: #22c55e; font-size: 28px; font-weight: bold; font-family: monospace; letter-spacing: 4px; margin: 0;",
                    "{code}"
                }
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

#[cfg(test)]
mod tests {
    use super::*;

    fn keys() -> ([u8; 32], [u8; 32]) {
        (
            E2eIdentity::generate().public_key_bytes(),
            E2eIdentity::generate().public_key_bytes(),
        )
    }

    #[test]
    fn pair_reply_pins_cog_key_and_computes_code_locally() {
        let (device, cog) = keys();
        let body = serde_json::json!({
            "status": "pending",
            "cog_identity_key": hex::encode(cog),
            // Code fourni par le réseau : ignoré.
            "confirmation_code": "000000",
        })
        .to_string();
        let pinned = parse_pair_reply(body.as_bytes(), &device).unwrap();
        assert_eq!(pinned.cog_identity_key_hex, hex::encode(cog));
        assert_eq!(pinned.confirmation_code, pairing_confirmation_code(&device, &cog));
    }

    #[test]
    fn pair_reply_without_valid_cog_key_refused() {
        let (device, _) = keys();
        // COG ancien (sans E2E v2) : pas de clé.
        let old = br#"{"status":"pending","token":null,"message":"x"}"#;
        assert!(parse_pair_reply(old, &device).is_err());
        let mut weak = [0u8; 32];
        weak[0] = 1;
        for key in [hex::encode(weak), "zz".to_string(), hex::encode(device)] {
            let body = serde_json::json!({"status": "pending", "cog_identity_key": key}).to_string();
            assert!(parse_pair_reply(body.as_bytes(), &device).is_err());
        }
        let (_, cog) = keys();
        let body = serde_json::json!({"status": "refused", "cog_identity_key": hex::encode(cog)})
            .to_string();
        assert!(parse_pair_reply(body.as_bytes(), &device).is_err());
        assert!(parse_pair_reply(&vec![b' '; MAX_PAIR_REPLY_LEN + 1], &device).is_err());
    }
}
