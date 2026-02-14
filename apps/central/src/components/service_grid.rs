//! Grille d'affichage des services.

use dioxus::prelude::*;
use crate::state::{use_app_state, ServiceInfo, ServiceType};
use crate::theme::{Theme, styles};
use super::ServiceCard;

/// Filtre pour la grille de services.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[allow(dead_code)]
pub enum ServiceFilter {
    #[default]
    All,
    Installed,
    Favorites,
    Type(ServiceType),
}

/// Propriétés de la grille de services.
#[derive(Props, Clone, PartialEq)]
pub struct ServiceGridProps {
    /// Titre de la section.
    #[props(default = "Services".to_string())]
    pub title: String,
    /// Filtre à appliquer.
    #[props(default)]
    pub filter: ServiceFilter,
    /// Afficher les filtres.
    #[props(default = true)]
    pub show_filters: bool,
}

/// Grille de services avec filtres.
#[component]
pub fn ServiceGrid(props: ServiceGridProps) -> Element {
    let state = use_app_state();
    let theme = state.read().current_theme;
    let c = theme.palette();
    let mut current_filter = use_signal(|| props.filter);

    // Filtrer les services
    let services: Vec<ServiceInfo> = state.read().services.iter()
        .filter(|s| {
            // Filtre de recherche
            let query = state.read().search_query.to_lowercase();
            if !query.is_empty()
                && !s.name.to_lowercase().contains(&query)
                && !s.description.to_lowercase().contains(&query)
            {
                return false;
            }

            // Filtre de catégorie
            match *current_filter.read() {
                ServiceFilter::All => true,
                ServiceFilter::Installed => s.is_installed,
                ServiceFilter::Favorites => s.is_favorite,
                ServiceFilter::Type(t) => s.service_type == t,
            }
        })
        .cloned()
        .collect();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 16px;",

            // En-tête avec titre et filtres
            div {
                style: "display: flex; align-items: center; justify-content: space-between;",

                h2 {
                    style: "{styles::section_title(theme)}",
                    "{props.title}"
                }

                if props.show_filters {
                    div {
                        style: "display: flex; gap: 8px;",
                        FilterButton { theme, label: "Tous", is_active: *current_filter.read() == ServiceFilter::All, onclick: move |_| current_filter.set(ServiceFilter::All) }
                        FilterButton { theme, label: "Installés", is_active: *current_filter.read() == ServiceFilter::Installed, onclick: move |_| current_filter.set(ServiceFilter::Installed) }
                        FilterButton { theme, label: "Favoris", is_active: *current_filter.read() == ServiceFilter::Favorites, onclick: move |_| current_filter.set(ServiceFilter::Favorites) }
                    }
                }
            }

            if services.is_empty() {
                div {
                    style: "text-align: center; padding: 48px; color: {c.text_secondary};",
                    p { "Aucun service trouvé" }
                }
            } else {
                div {
                    style: "{styles::services_grid()}",

                    for service in services {
                        ServiceCard {
                            key: "{service.id}",
                            service: service.clone(),
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn FilterButton(
    theme: Theme,
    label: &'static str,
    is_active: bool,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let c = theme.palette();
    let bg = if is_active { c.accent_blue } else { c.bg_hover };
    let color = if is_active { c.text_white } else { c.text_secondary };

    rsx! {
        button {
            style: "padding: 6px 12px; background: {bg}; color: {color}; border: none; border-radius: 4px; cursor: pointer; font-size: 12px; transition: all 0.2s;",
            onclick: move |evt| onclick.call(evt),
            "{label}"
        }
    }
}
