//! Service Market — Catalogue des services Miyukini.
//!
//! Interface de découverte, recherche, installation et désinstallation
//! des services officiels et communautaires.

mod catalog;
mod service_detail;
mod sidebar;

use crate::market_client::MarketClient;
use crate::state::{
    market_entry_to_service_info, use_app_state, use_service_manager, ServiceInfo, ServiceRegistry,
};
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
    /// Catalogue réellement chargé depuis Origin.
    pub catalog_services: Vec<ServiceInfo>,
    /// True quand une tentative de chargement distant a déjà été faite.
    pub catalog_loaded: bool,
}

// ── Composant racine ───────────────────────────────────────────────────

#[component]
pub fn MarketView() -> Element {
    let app_state = use_app_state();
    let manager = use_service_manager();
    let state = use_signal(MarketState::default);

    // Au montage : récupérer le catalogue Origin et vérifier les mises à jour
    {
        let manager = manager.clone();
        let mut state = state;
        use_effect(move || {
            let manager = manager.clone();
            let mut app_state = app_state.clone();
            spawn(async move {
                let origin_url = market_origin_url();
                let client = MarketClient::new(&origin_url);

                match client.fetch_catalog().await {
                    Ok(catalog) => {
                        let remote_services = catalog
                            .official
                            .into_iter()
                            .chain(catalog.community.into_iter())
                            .map(|entry| market_entry_to_service_info(entry, &manager))
                            .collect::<Vec<_>>();

                        // Construire la liste (service_id, version_disponible)
                        let catalog_versions: Vec<(String, String)> = remote_services
                            .iter()
                            .map(|entry| (entry.id.clone(), entry.version.clone()))
                            .collect();

                        let updates = manager.check_updates(&catalog_versions);
                        if !updates.is_empty() {
                            tracing::info!(
                                "Market: {} mise(s) à jour disponible(s)",
                                updates.len()
                            );
                        }

                        {
                            let mut s = state.write();
                            s.catalog_services = remote_services;
                            s.catalog_loaded = true;
                            s.available_updates = updates;
                        }
                        app_state.write().refresh_services(&manager);
                    }
                    Err(e) => {
                        tracing::warn!("Chargement du Market Origin échoué: {e}");
                        state.write().catalog_loaded = true;
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

pub(super) fn market_origin_url() -> String {
    std::env::var("MIYUKINI_ORIGIN_URL").unwrap_or_else(|_| "https://miyukini.com".to_string())
}

pub(super) fn visible_market_services(
    manager: &crate::service_manager::ServiceManager,
    state: &MarketState,
) -> Vec<ServiceInfo> {
    if state.catalog_services.is_empty() {
        return ServiceRegistry::installed_services(manager);
    }

    let mut services = state.catalog_services.clone();
    for svc in &mut services {
        svc.is_installed = manager.is_installed(&svc.id);
    }

    for installed in ServiceRegistry::installed_only(manager) {
        if installed.id != "market" && !services.iter().any(|svc| svc.id == installed.id) {
            services.push(installed);
        }
    }

    services
}
