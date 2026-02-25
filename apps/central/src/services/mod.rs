//! Vues des services intégrés dans Miyukini Central.
//!
//! Chaque service peut avoir sa propre vue qui s'affiche dans un onglet.
//! Les modules sont conditionnellement compilés selon les features actives.

mod home;
mod market;
mod service_view;

#[cfg(feature = "service-jayxpose")]
mod jayxpose;
#[cfg(feature = "service-jayfestival")]
mod jayfestival;
#[cfg(feature = "service-jaykoa")]
mod jaykoa;
#[cfg(feature = "service-jaykonta")]
mod jaykonta;
#[cfg(feature = "service-miyuclicker")]
mod game_view;
#[cfg(feature = "service-miyuclicker")]
mod miyuclicker;
#[cfg(feature = "service-lord-of-the-castle")]
mod survivor_embed;
mod mws_view;
#[cfg(feature = "service-miyukiniwatch")]
mod miyukiniwatch;
#[cfg(feature = "service-jay1tribu")]
mod jay1tribu;
#[cfg(feature = "service-jaymanga")]
mod jaymanga;

pub use home::HomeView;
pub use market::MarketView;
pub use service_view::ServiceView;
#[cfg(feature = "service-jayxpose")]
pub use jayxpose::JayXposeView;
#[cfg(feature = "service-jayfestival")]
pub use jayfestival::JayFestivalView;
#[cfg(feature = "service-jaykoa")]
pub use jaykoa::JayKoaView;
#[cfg(feature = "service-jaykonta")]
pub use jaykonta::JayKontaView;
#[cfg(feature = "service-miyuclicker")]
pub use miyuclicker::MiyuClickerView;
#[cfg(feature = "service-lord-of-the-castle")]
pub use survivor_embed::SurvivorEmbed;
pub use mws_view::{MwsNetworkView, MwsViewState};
#[cfg(feature = "service-miyukiniwatch")]
pub use miyukiniwatch::MiyukiniWatchView;
#[cfg(feature = "service-jay1tribu")]
pub use jay1tribu::Jay1TribuView;
#[cfg(feature = "service-jaymanga")]
pub use jaymanga::JayMangaView;

use dioxus::prelude::*;
use crate::state::use_app_state;

/// Rendu de la vue correspondant à l'onglet actif.
/// Le dispatch est conditionnel aux features compilées.
#[component]
pub fn ActiveServiceView() -> Element {
    let state = use_app_state();
    let open_tabs = state.read().open_tabs.clone();
    let active_index = state.read().active_tab_index;

    if let Some(tab) = open_tabs.get(active_index) {
        match tab.service_id.as_deref() {
            None => rsx! { HomeView {} },
            #[cfg(feature = "service-jayxpose")]
            Some("jayxpose") => rsx! { JayXposeView {} },
            #[cfg(feature = "service-jayfestival")]
            Some("jayfestival") => rsx! { JayFestivalView {} },
            #[cfg(feature = "service-jaykoa")]
            Some("jaykoa") => rsx! { JayKoaView {} },
            #[cfg(feature = "service-jaykonta")]
            Some("jaykonta") => rsx! { JayKontaView {} },
            #[cfg(feature = "service-lord-of-the-castle")]
            Some("lord_of_the_castle") => rsx! { SurvivorEmbed {} },
            #[cfg(feature = "service-miyuclicker")]
            Some("miyuclicker") => rsx! { MiyuClickerView {} },
            #[cfg(feature = "service-miyukiniwatch")]
            Some("miyukiniwatch") => rsx! { MiyukiniWatchView {} },
            #[cfg(feature = "service-jay1tribu")]
            Some("jay1tribu") => rsx! { Jay1TribuView {} },
            #[cfg(feature = "service-jaymanga")]
            Some("jaymanga") => rsx! { JayMangaView {} },
            Some("market") => rsx! { MarketView {} },
            Some(id) => rsx! {
                ServiceView { service_id: id.to_string() }
            },
        }
    } else {
        rsx! { HomeView {} }
    }
}
