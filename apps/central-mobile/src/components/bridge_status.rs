//! Indicateur de statut du bridge (barre en haut de l'écran).

use dioxus::prelude::*;
use miyukini_cog_bridge::BridgeStatus;

use crate::state::AppState;

/// Barre d'état de la connexion bridge.
#[component]
pub fn BridgeStatusBar() -> Element {
    let state = use_context::<Signal<AppState>>();
    let status = &state.read().bridge_status;

    let (color, text) = match status {
        BridgeStatus::Connected { mode } => ("#22c55e", format!("Connecte ({mode})")),
        BridgeStatus::Connecting => ("#eab308", "Connexion...".to_string()),
        BridgeStatus::Reconnecting { attempt } => ("#f97316", format!("Reconnexion #{attempt}")),
        BridgeStatus::Disconnected => ("#ef4444", "Deconnecte".to_string()),
        BridgeStatus::Error { message } => ("#ef4444", format!("Erreur: {message}")),
    };

    rsx! {
        div {
            style: "padding: 6px 16px; background: #16162a; border-bottom: 1px solid #2d2d44; display: flex; align-items: center; gap: 8px; flex-shrink: 0;",

            // Point coloré
            div {
                style: "width: 8px; height: 8px; border-radius: 50%; background: {color}; flex-shrink: 0;",
            }
            // Texte statut
            span {
                style: "font-size: 12px; color: #888;",
                "{text}"
            }
        }
    }
}
