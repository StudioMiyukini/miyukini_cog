//! Vues des services dans Miyukini Central.
//!
//! Central est un launcher standalone : les services sont des binaires indépendants.
//! Ce module fournit :
//! - HomeView (Salon)
//! - MarketView (catalogue de services)
//! - ExternalServiceView (panneau de contrôle pour lancer un service externe)

mod home;
mod market;
mod external_service_view;
pub mod alicia;

pub use home::HomeView;
pub use market::MarketView;
pub use external_service_view::ExternalServiceView;

use dioxus::prelude::*;
use crate::state::use_app_state;

/// Rendu de la vue correspondant à l'onglet actif.
/// Les services sont désormais externes — Central affiche un panneau de lancement.
/// Les services embarqués (alicia, miyukiniwatch…) ont leur propre vue.
#[component]
pub fn ActiveServiceView() -> Element {
    let state = use_app_state();
    let open_tabs = state.read().open_tabs.clone();
    let active_index = state.read().active_tab_index;

    if let Some(tab) = open_tabs.get(active_index) {
        match tab.service_id.as_deref() {
            None => rsx! { HomeView {} },
            Some("market") => rsx! { MarketView {} },
            Some("alicia") => rsx! { alicia::AliciaView {} },
            Some(id) => rsx! {
                ExternalServiceView { service_id: id.to_string() }
            },
        }
    } else {
        rsx! { HomeView {} }
    }
}
