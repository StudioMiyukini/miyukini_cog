//! Intégration de Miyukini Survivor (Lord of the Castle) dans Central.
//! Fournit le chemin de données et affiche SurvivorApp dans le body.

use dioxus::prelude::*;
use std::path::PathBuf;

use crate::data::use_service_connections;
use lord_of_the_castle::SurvivorApp;

/// Wrapper qui fournit le chemin de données (KindMother / Central) et affiche le jeu.
#[component]
pub fn SurvivorEmbed() -> Element {
    let conns = use_service_connections();
    let base_path: PathBuf = conns.read().miyuclicker_data_dir.clone();
    use_context_provider(|| Signal::new(base_path));

    rsx! {
        div {
            style: "height: 100%; min-height: 400px; display: flex; flex-direction: column;",
            SurvivorApp {}
        }
    }
}
