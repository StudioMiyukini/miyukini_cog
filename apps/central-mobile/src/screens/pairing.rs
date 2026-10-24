//! @id     central_mobile_screens_pairing
//! @do     definir_pairing_step
//! @role   ui
//! @layer  app
//! @human  central-mobile — module « screens pairing ».

//! Écran d'appairage — connexion initiale au COG Host.
//!
//! Flow :
//!   Idle → [user clicks "WiFi"] → Searching (UDP broadcast 5s)
//!     → Found(host) → [user confirms] → Pairing :
//!       1. POST /api/bridge/pair (clé E2E + engagement sur Nd) → clé + aléa Nc du COG
//!       2. POST /api/bridge/pair/reveal (Nd, seulement après réception de Nc)
//!       3. affichage du code SAS à 8 chiffres et de l'empreinte du COG
//!       4. poll /api/bridge/pair/status jusqu'à `approved` (ou refus / expiration)
//!         → persiste SavedConnection (clé du COG épinglée) → Connexion
//!
//! Appairage v2 (R-026) : engagement puis révélation (voir
//! `miyukini_cog_bridge::pairing`). Le code est calculé localement à partir
//! des deux clés et des deux aléas ; il n'est jamais lu dans une réponse du
//! réseau. L'utilisateur vérifie qu'il est identique sur Central Desktop avant
//! que l'administrateur n'approuve.

use central_remote_protocol::pairing::{
    PairCommitRequest, PairCommitResponse, PairRevealRequest, PairStatusRequest,
    PairStatusResponse, PairingStatus, PAIRING_PROTOCOL_VERSION, PAIR_ENDPOINT,
    PAIR_REVEAL_ENDPOINT, PAIR_STATUS_ENDPOINT,
};
use dioxus::prelude::*;
use miyukini_cog_bridge::{
    discovery, format_sas_code, identity_fingerprint, parse_identity_key_hex, DevicePairing,
    E2eIdentity, PairingNonce,
};
use serde::Deserialize;
use std::net::SocketAddr;
use std::time::Duration;

/// Taille maximale d'une réponse d'appairage (C-13).
const MAX_PAIR_REPLY_LEN: usize = 4096;

/// Intervalle d'interrogation du suivi d'appairage.
const STATUS_POLL_INTERVAL: Duration = Duration::from_secs(3);

/// Nombre maximal d'interrogations (≈ 10 min 30 : au-delà, la demande a expiré
/// côté COG).
const STATUS_POLL_MAX: u32 = 210;

/// Clé et aléa du COG reçus à l'étape 1.
#[derive(Debug)]
struct CogChallenge {
    cog_key: [u8; 32],
    cog_nonce: PairingNonce,
}

/// Décode la réponse d'engagement : version et statut attendus, clé du COG
/// valide (point ed25519 non faible, différente de celle de l'appareil) et
/// aléa de 32 octets ; sinon refus.
fn parse_commit_reply(body: &[u8], device_key: &[u8; 32]) -> Result<CogChallenge, String> {
    if body.len() > MAX_PAIR_REPLY_LEN {
        return Err("Réponse d'appairage trop grande".into());
    }
    let reply: PairCommitResponse = serde_json::from_slice(body).map_err(|_| {
        "Réponse d'appairage invalide (COG Host d'une version incompatible ?)".to_string()
    })?;
    if reply.pairing_version != PAIRING_PROTOCOL_VERSION {
        return Err("Version d'appairage du COG Host incompatible : mettez-le à jour".into());
    }
    if reply.status != PairingStatus::Committed {
        return Err("Réponse d'appairage inattendue".into());
    }
    let cog_key = parse_identity_key_hex(&reply.cog_identity_key)
        .map_err(|_| "Clé du COG Host invalide".to_string())?;
    if &cog_key == device_key {
        return Err("Clé du COG Host invalide".into());
    }
    let cog_nonce =
        PairingNonce::from_hex(&reply.cog_nonce).map_err(|_| "Aléa du COG Host invalide".to_string())?;
    Ok(CogChallenge { cog_key, cog_nonce })
}

/// Statut d'erreur renvoyé par le COG (`{"status": "..."}`).
#[derive(Deserialize)]
struct ErrorReply {
    status: String,
}

/// Message lisible pour un refus du COG (code HTTP + statut éventuel).
fn refusal_message(http: u16, body: &[u8]) -> String {
    let status = serde_json::from_slice::<ErrorReply>(body)
        .map(|r| r.status)
        .unwrap_or_default();
    match status.as_str() {
        "unsupported_pairing_version" => {
            "Version d'appairage incompatible : mettez à jour Central Mobile et Central Desktop".into()
        }
        "already_paired" => {
            "Cet appareil est déjà appairé. Révoquez-le dans Central Desktop pour ré-appairer.".into()
        }
        "pending_conflict" | "key_in_use" => {
            "Une autre demande d'appairage est en attente pour cet appareil.\nRefusez-la dans Central Desktop ou attendez 10 minutes.".into()
        }
        "too_many_pending" | "too_many_pending_for_source" => {
            "Trop de demandes d'appairage en attente. Réessayez plus tard.".into()
        }
        "commitment_mismatch" => "Vérification de l'appairage échouée : recommencez.".into(),
        _ => format!("Appairage refusé par le COG Host (HTTP {http})"),
    }
}

/// Lit un corps de réponse borné (C-13).
async fn read_bounded(resp: reqwest::Response) -> Result<(u16, Vec<u8>), String> {
    let status = resp.status().as_u16();
    if resp
        .content_length()
        .is_some_and(|n| n > MAX_PAIR_REPLY_LEN as u64)
    {
        return Err("Réponse d'appairage trop grande".into());
    }
    let body = resp
        .bytes()
        .await
        .map_err(|e| format!("Erreur de connexion: {e}"))?;
    if body.len() > MAX_PAIR_REPLY_LEN {
        return Err("Réponse d'appairage trop grande".into());
    }
    Ok((status, body.to_vec()))
}

/// Code et empreinte du COG à afficher pendant l'attente d'approbation.
#[derive(Debug, Clone, PartialEq, Eq)]
struct DisplayedCode {
    code: String,
    cog_fingerprint: String,
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

/// Déroule l'appairage v2 (engagement, révélation, attente d'approbation).
/// Retourne la connexion à persister une fois l'appareil approuvé.
async fn run_pairing(
    host: &str,
    port: u16,
    mut displayed: Signal<Option<DisplayedCode>>,
) -> Result<SavedConnection, String> {
    // Identité E2E de l'appareil (générée une fois, stockée 0600).
    let identity: E2eIdentity = platform::load_or_create_e2e_identity()
        .map_err(|e| format!("Identité E2E indisponible : {e}"))?;
    let device_key = identity.public_key_bytes();
    drop(identity);
    let device_key_hex = hex::encode(device_key);

    let device_id = platform::get_device_id();
    let short_id: String = device_id.chars().take(8).collect();
    let device_name = format!("Android-{short_id}");
    let base = format!("http://{host}:{port}");
    let client = reqwest::Client::new();

    // 1. Engagement : Nd reste secret.
    let pairing = DevicePairing::start(device_key);
    let req = PairCommitRequest {
        pairing_version: PAIRING_PROTOCOL_VERSION,
        device_id: device_id.clone(),
        device_name,
        mode: "whitelist".into(),
        e2e_identity_key: device_key_hex.clone(),
        commitment: pairing.commitment_hex(),
    };
    let resp = client
        .post(format!("{base}{PAIR_ENDPOINT}"))
        .json(&req)
        .timeout(Duration::from_secs(10))
        .send()
        .await
        .map_err(|e| format!("Erreur de connexion: {e}"))?;
    let (http, body) = read_bounded(resp).await?;
    if http != 200 {
        return Err(refusal_message(http, &body));
    }
    let challenge = parse_commit_reply(&body, &device_key)?;

    // 2. Aléa du COG reçu : calcul du code, puis révélation de Nd.
    let revealed = pairing
        .receive_cog_nonce(challenge.cog_key, challenge.cog_nonce)
        .map_err(|_| "Clé du COG Host invalide".to_string())?;
    let reveal = PairRevealRequest {
        pairing_version: PAIRING_PROTOCOL_VERSION,
        device_id: device_id.clone(),
        device_nonce: revealed.device_nonce_hex().to_string(),
    };
    let resp = client
        .post(format!("{base}{PAIR_REVEAL_ENDPOINT}"))
        .json(&reveal)
        .timeout(Duration::from_secs(10))
        .send()
        .await
        .map_err(|e| format!("Erreur de connexion: {e}"))?;
    let (http, body) = read_bounded(resp).await?;
    if http != 200 {
        return Err(refusal_message(http, &body));
    }

    // 3. Affichage du code et de l'empreinte du COG.
    displayed.set(Some(DisplayedCode {
        code: format_sas_code(revealed.code()),
        cog_fingerprint: identity_fingerprint(&revealed.cog_identity()),
    }));

    // 4. Attente de l'approbation de CETTE clé sur le desktop.
    let status_req = PairStatusRequest {
        device_id: device_id.clone(),
        e2e_identity_key: device_key_hex,
    };
    for _ in 0..STATUS_POLL_MAX {
        tokio::time::sleep(STATUS_POLL_INTERVAL).await;
        let Ok(resp) = client
            .post(format!("{base}{PAIR_STATUS_ENDPOINT}"))
            .json(&status_req)
            .timeout(Duration::from_secs(5))
            .send()
            .await
        else {
            continue;
        };
        let Ok((200, body)) = read_bounded(resp).await else {
            continue;
        };
        let Ok(status) = serde_json::from_slice::<PairStatusResponse>(&body) else {
            continue;
        };
        match status.status {
            PairingStatus::Approved => {
                return Ok(SavedConnection {
                    mode: "whitelist".to_string(),
                    host: host.to_string(),
                    port,
                    credential: device_id,
                    cog_identity_key: Some(hex::encode(revealed.cog_identity())),
                });
            }
            PairingStatus::Committed | PairingStatus::Revealed => {}
            PairingStatus::Rejected => {
                return Err("Appairage refusé sur Central Desktop.".into());
            }
            PairingStatus::Expired | PairingStatus::Unknown => {
                return Err("Demande d'appairage expirée. Recommencez.".into());
            }
        }
    }
    Err("Délai dépassé. Approuvez le device sur Central Desktop.".to_string())
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

    // Code SAS et empreinte du COG affichés pendant l'attente d'approbation.
    let displayed = use_signal(|| None::<DisplayedCode>);

    use_effect(move || {
        let host_clone = host.clone();
        spawn(async move {
            match run_pairing(&host_clone, port, displayed).await {
                Ok(saved) => {
                    tracing::info!("Device approuvé sur le COG Host");
                    if let Err(e) = platform::save_connection(&saved) {
                        tracing::warn!("Persistance connection échouée: {e}");
                    }
                    state.write().saved_connection = Some(saved);
                    state.write().screen = AppScreen::Connexion;
                }
                Err(e) => step.set(PairingStep::Error(e)),
            }
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

            if let Some(d) = displayed.read().clone() {
                p {
                    style: "color: #e0e0e0; font-size: 14px; text-align: center; margin: 0;",
                    "Code de confirmation — n'approuvez que s'il est identique sur Central Desktop :"
                }
                p {
                    style: "color: #22c55e; font-size: 28px; font-weight: bold; font-family: monospace; letter-spacing: 4px; margin: 0;",
                    "{d.code}"
                }
                p {
                    style: "color: #888; font-size: 11px; text-align: center; font-family: monospace; margin: 0;",
                    "Empreinte du COG : {d.cog_fingerprint}"
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
    use miyukini_cog_bridge::{pairing_sas_code, verify_pairing_commitment, parse_commitment_hex};

    fn keys() -> ([u8; 32], [u8; 32]) {
        (
            E2eIdentity::generate().public_key_bytes(),
            E2eIdentity::generate().public_key_bytes(),
        )
    }

    fn reply(cog_key: &str, nonce: &str, version: u8, status: &str) -> Vec<u8> {
        serde_json::json!({
            "pairing_version": version,
            "status": status,
            "cog_identity_key": cog_key,
            "cog_nonce": nonce,
            "message": "",
            // Un code fourni par le réseau serait ignoré.
            "confirmation_code": "00000000",
        })
        .to_string()
        .into_bytes()
    }

    #[test]
    fn honest_flow_device_code_matches_cog_code() {
        let (device, cog) = keys();
        let pairing = DevicePairing::start(device);
        let commitment = parse_commitment_hex(&pairing.commitment_hex()).unwrap();
        // Côté COG : aléa Nc.
        let nc = PairingNonce::generate();
        let body = reply(&hex::encode(cog), &nc.to_hex(), PAIRING_PROTOCOL_VERSION, "committed");
        let challenge = parse_commit_reply(&body, &device).unwrap();
        assert_eq!(challenge.cog_key, cog);
        let revealed = pairing
            .receive_cog_nonce(challenge.cog_key, challenge.cog_nonce)
            .unwrap();
        // Côté COG : vérification de l'engagement puis code.
        let nd = PairingNonce::from_hex(&revealed.device_nonce_hex()).unwrap();
        assert!(verify_pairing_commitment(&commitment, &device, &nd));
        assert_eq!(revealed.code(), pairing_sas_code(&device, &cog, &nd, &nc));
        assert_ne!(revealed.code(), "00000000");
    }

    #[test]
    fn commit_reply_invalid_refused() {
        let (device, cog) = keys();
        let nonce = "11".repeat(32);
        // COG ancien (appairage v1) : pas de version ni d'aléa.
        let old = br#"{"status":"pending","cog_identity_key":"aa","confirmation_code":"123456"}"#;
        assert!(parse_commit_reply(old, &device).is_err());
        // Version différente.
        assert!(parse_commit_reply(&reply(&hex::encode(cog), &nonce, 1, "committed"), &device).is_err());
        // Statut inattendu.
        assert!(parse_commit_reply(&reply(&hex::encode(cog), &nonce, 2, "approved"), &device).is_err());
        // Clé faible, mal formée, ou égale à celle de l'appareil.
        let mut weak = [0u8; 32];
        weak[0] = 1;
        for key in [hex::encode(weak), "zz".to_string(), hex::encode(device)] {
            assert!(parse_commit_reply(&reply(&key, &nonce, 2, "committed"), &device).is_err());
        }
        // Aléa mal formé.
        assert!(parse_commit_reply(&reply(&hex::encode(cog), "abcd", 2, "committed"), &device).is_err());
        assert!(parse_commit_reply(&vec![b' '; MAX_PAIR_REPLY_LEN + 1], &device).is_err());
    }

    #[test]
    fn refusal_messages_explain_cause() {
        assert!(refusal_message(409, br#"{"status":"already_paired"}"#).contains("Révoquez"));
        assert!(refusal_message(409, br#"{"status":"pending_conflict"}"#).contains("en attente"));
        assert!(refusal_message(400, br#"{"status":"unsupported_pairing_version"}"#).contains("incompatible"));
        assert!(refusal_message(503, b"").contains("503"));
    }
}
