//! @id     central_mobile_services_mws_status
//! @do     mws_status_view
//! @role   orchestration
//! @layer  app
//! @human  central-mobile — module « services mws status ».

//! Vue MWS Status — état du réseau Webway.

use dioxus::prelude::*;

/// Vue statut MWS (lecture seule).
#[component]
pub fn MwsStatusView() -> Element {
    rsx! {
        div {
            style: "padding: 20px; display: flex; flex-direction: column; gap: 16px;",

            h2 {
                style: "font-size: 22px; font-weight: bold;",
                "Webway"
            }

            // TODO Phase 2 : afficher l'état MWS via bridge
            p {
                style: "color: #888; font-size: 14px;",
                "Etat du reseau Miyukini Webway System."
            }
        }
    }
}
