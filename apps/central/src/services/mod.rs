//! Vues des services intégrés dans Miyukini Central.
//!
//! Chaque service peut avoir sa propre vue qui s'affiche dans un onglet.

mod home;
mod service_view;
mod jayxpose;
mod jayfestival;
mod jaykoa;
mod jaykonta;
mod game_view;
mod miyuclicker;
mod survivor_embed;
mod mws_view;
mod miyukiniwatch;
mod jay1tribu;
mod jaymanga;

pub use home::HomeView;
pub use service_view::ServiceView;
pub use jayxpose::JayXposeView;
pub use jayfestival::JayFestivalView;
pub use jaykoa::JayKoaView;
pub use jaykonta::JayKontaView;
pub use miyuclicker::MiyuClickerView;
pub use survivor_embed::SurvivorEmbed;
pub use mws_view::{MwsNetworkView, MwsViewState};
pub use miyukiniwatch::MiyukiniWatchView;
pub use jay1tribu::Jay1TribuView;
pub use jaymanga::JayMangaView;

use dioxus::prelude::*;
use crate::state::use_app_state;

/// Rendu de la vue correspondant à l'onglet actif.
#[component]
pub fn ActiveServiceView() -> Element {
    let state = use_app_state();
    let open_tabs = state.read().open_tabs.clone();
    let active_index = state.read().active_tab_index;

    if let Some(tab) = open_tabs.get(active_index) {
        match tab.service_id.as_deref() {
            None => rsx! { HomeView {} },
            Some("jayxpose") => rsx! { JayXposeView {} },
            Some("jayfestival") => rsx! { JayFestivalView {} },
            Some("jaykoa") => rsx! { JayKoaView {} },
            Some("jaykonta") => rsx! { JayKontaView {} },
            Some("lord_of_the_castle") => rsx! { SurvivorEmbed {} },
            Some("miyuclicker") => rsx! { MiyuClickerView {} },
            Some("miyukiniwatch") => rsx! { MiyukiniWatchView {} },
            Some("jay1tribu") => rsx! { Jay1TribuView {} },
            Some("jaymanga") => rsx! { JayMangaView {} },
            Some(id) => rsx! { 
                ServiceView { service_id: id.to_string() } 
            },
        }
    } else {
        rsx! { HomeView {} }
    }
}
