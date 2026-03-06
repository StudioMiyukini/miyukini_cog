//! Vues des services dans Miyukini Central.
//!
//! Central est un launcher standalone : les services sont des binaires indépendants.
//! Ce module fournit :
//! - HomeView (Salon)
//! - MarketView (catalogue de services)
//! - ExternalServiceView (panneau de contrôle pour lancer un service externe)

pub mod alicia;
mod external_service_view;
mod home;
mod market;
pub mod miyucloud;
pub(crate) mod miyucloud_settings;
pub mod miyukini_whisper;
mod mws_settings;
mod mws_view;

pub use external_service_view::ExternalServiceView;
pub use home::HomeView;
pub use market::MarketView;
pub(crate) use mws_view::auto_connect_after_login;
pub use mws_view::{MwsNetworkView, MwsViewState};

use crate::state::use_app_state;
use dioxus::prelude::*;

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
            Some("miyukini-whisper") => rsx! { miyukini_whisper::MiyukiniWhisperView {} },
            Some("miyucloud") => rsx! { miyucloud::MiyuCloudView {} },
            Some(id) => rsx! {
                ExternalServiceView { service_id: id.to_string() }
            },
        }
    } else {
        rsx! { HomeView {} }
    }
}
