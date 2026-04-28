//! Barre de présence affichant les participants actifs.

use dioxus::prelude::*;

#[component]
pub fn PresenceBar() -> Element {
    // MVP : utilisateur local uniquement (placeholder pour les autres participants).
    rsx! {
        div {
            class: "doc-presence",
            span { class: "presence-label", "En ligne:" }
            div {
                class: "presence-avatar",
                style: "background: #7c3aed;",
                title: "Vous",
                "V"
            }
            // Placeholder : avatars des autres participants (récupérés via presence manager)
        }
    }
}
