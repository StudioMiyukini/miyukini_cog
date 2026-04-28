//! Vue Miou Chat — chat IA.

use dioxus::prelude::*;

/// Vue chat Miou.
#[component]
pub fn MiouChatView() -> Element {
    rsx! {
        div {
            style: "padding: 20px; display: flex; flex-direction: column; gap: 16px;",

            h2 {
                style: "font-size: 22px; font-weight: bold;",
                "Miou"
            }

            // TODO Phase 4 : llm_client + chat UI
            p {
                style: "color: #888; font-size: 14px;",
                "Chat avec Miou (bientot disponible)"
            }
        }
    }
}
