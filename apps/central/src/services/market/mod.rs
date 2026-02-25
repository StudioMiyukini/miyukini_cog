//! Service Market — Catalogue des services Miyukini.
//!
//! Interface de découverte, recherche, installation et désinstallation
//! des services officiels et communautaires.

mod sidebar;
mod catalog;
mod service_detail;

use dioxus::prelude::*;
use crate::state::use_app_state;

use sidebar::MarketSidebar;
use catalog::MarketCatalog;
use service_detail::ServiceDetail;

// ── State ──────────────────────────────────────────────────────────────

/// Section active du Market.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MarketSection {
    /// Tous les services disponibles.
    #[default]
    Decouvrir,
    /// Services officiels Miyukini.
    Officiels,
    /// Services communautaires tiers.
    Communaute,
    /// Services déjà installés.
    Installes,
}

/// État local du Market.
#[derive(Debug, Clone, Default)]
pub struct MarketState {
    /// Section active.
    pub section: MarketSection,
    /// Recherche en cours.
    pub search_query: String,
    /// Service sélectionné pour la vue détail.
    pub selected_service_id: Option<String>,
    /// Filtre par type de service.
    pub type_filter: Option<crate::state::ServiceType>,
}

// ── Composant racine ───────────────────────────────────────────────────

#[component]
pub fn MarketView() -> Element {
    let _app_state = use_app_state();
    let state = use_signal(MarketState::default);

    rsx! {
        div {
            style: "display: flex; flex-grow: 1; flex-shrink: 1; flex-basis: 0; min-height: 0; overflow: hidden;",

            MarketSidebar { state: state }

            div {
                style: "flex: 1; min-height: 0; display: flex; flex-direction: column; overflow: hidden;",

                div {
                    style: "flex: 1; min-height: 0; padding: 24px; overflow-y: auto; overflow-x: hidden;",

                    if state.read().selected_service_id.is_some() {
                        ServiceDetail { state: state }
                    } else {
                        MarketCatalog { state: state }
                    }
                }
            }
        }
    }
}
