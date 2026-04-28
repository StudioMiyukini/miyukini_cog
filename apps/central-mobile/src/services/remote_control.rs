//! Vue Remote Control — pilotage du Central Desktop.

use dioxus::prelude::*;

/// Vue pilotage à distance du Central Desktop.
#[component]
pub fn RemoteControlView() -> Element {
    rsx! {
        div {
            style: "padding: 20px; display: flex; flex-direction: column; gap: 16px;",

            h2 {
                style: "font-size: 22px; font-weight: bold;",
                "Remote Control"
            }

            p {
                style: "color: #888; font-size: 14px;",
                "Pilotez votre Central Desktop depuis votre telephone."
            }

            // TODO Phase 3 : WebSocket client → CentralRemote
            // Afficher le snapshot (onglets, services, état)
            // Envoyer des commandes (NavigateTab, OpenService, etc.)
        }
    }
}
