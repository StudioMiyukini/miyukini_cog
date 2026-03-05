//! Service Market — Catalogue des services Miyukini.
//!
//! Interface de découverte, recherche, installation et désinstallation
//! des services officiels et communautaires.

mod catalog;
mod service_detail;
mod sidebar;

use crate::market_client::MarketClient;
use crate::state::{use_app_state, use_service_manager};
use dioxus::prelude::*;

use catalog::MarketCatalog;
use service_detail::ServiceDetail;
use sidebar::MarketSidebar;

// ── State ──────────────────────────────────────────────────────────────

/// Section active du Market.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MarketSection {
    /// Tous les services disponibles.
    Decouvrir,
    /// Services officiels Miyukini.
    Officiels,
    /// Services communautaires tiers.
    Communaute,
    /// Services déjà installés.
    #[default]
    Installes,
}

/// Statut d'installation d'un service via le Market.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstallStatus {
    /// Téléchargement en cours.
    Downloading,
    /// Installation du package.
    Installing,
    /// Terminé avec succès.
    Done,
    /// Échec avec message.
    Error(String),
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
    /// Statut d'installation en cours (service_id → statut).
    pub install_status: Option<(String, InstallStatus)>,
    /// Mises à jour disponibles : (service_id, version_installée, version_disponible).
    pub available_updates: Vec<(String, String, String)>,
}

// ── Composant racine ───────────────────────────────────────────────────

#[component]
pub fn MarketView() -> Element {
    let _app_state = use_app_state();
    let manager = use_service_manager();
    let state = use_signal(MarketState::default);

    // Au montage : récupérer le catalogue Origin et vérifier les mises à jour
    {
        let manager = manager.clone();
        let mut state = state;
        use_effect(move || {
            let manager = manager.clone();
            spawn(async move {
                let origin_url = std::env::var("MIYUKINI_ORIGIN_URL")
                    .unwrap_or_else(|_| "http://127.0.0.1:8080".to_string());
                let client = MarketClient::new(&origin_url);

                match client.fetch_catalog().await {
                    Ok(catalog) => {
                        // Construire la liste (service_id, version_disponible)
                        let catalog_versions: Vec<(String, String)> = catalog
                            .official
                            .iter()
                            .chain(catalog.community.iter())
                            .map(|entry| {
                                (entry.manifest.id.clone(), entry.manifest.version.clone())
                            })
                            .collect();

                        let updates = manager.check_updates(&catalog_versions);
                        if !updates.is_empty() {
                            tracing::info!(
                                "Market: {} mise(s) à jour disponible(s)",
                                updates.len()
                            );
                            state.write().available_updates = updates;
                        }
                    }
                    Err(e) => {
                        tracing::debug!("Vérification des mises à jour échouée: {e}");
                    }
                }
            });
        });
    }

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
