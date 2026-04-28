//! Vue Market — catalogue des services (lecture seule).

use dioxus::prelude::*;

/// Vue catalogue Market.
#[component]
pub fn MarketView() -> Element {
    rsx! {
        div {
            style: "padding: 20px; display: flex; flex-direction: column; gap: 16px;",

            h2 {
                style: "font-size: 22px; font-weight: bold;",
                "Market"
            }

            p {
                style: "color: #888; font-size: 14px;",
                "Parcourez le catalogue des services Miyukini."
            }

            // TODO Phase 2 : charger le catalogue via market_client → bridge → Origin
        }
    }
}
