//! Catalogue Market — grille de services avec filtres et recherche.

use dioxus::prelude::*;
use crate::state::{use_app_state, ServiceInfo, ServiceSource, ServiceType, ServiceRegistry};
use super::{MarketSection, MarketState};

#[component]
pub fn MarketCatalog(state: Signal<MarketState>) -> Element {
    let app = use_app_state();
    let c = app.read().current_theme.palette();

    // Construire la liste complète : installés + disponibles (fallback local)
    let installed = app.read().services.clone();
    let available = ServiceRegistry::local_available();
    let mut all_services: Vec<ServiceInfo> = installed;
    // Ajouter les services disponibles non déjà présents
    for svc in available {
        if !all_services.iter().any(|s| s.id == svc.id) {
            all_services.push(svc);
        }
    }

    // Appliquer les filtres
    let section = state.read().section;
    let query = state.read().search_query.clone();
    let type_filter = state.read().type_filter;

    let filtered: Vec<ServiceInfo> = all_services
        .into_iter()
        .filter(|s| match section {
            MarketSection::Decouvrir => true,
            MarketSection::Officiels => s.source == ServiceSource::Officiel,
            MarketSection::Communaute => s.source == ServiceSource::Tiers,
            MarketSection::Installes => s.is_installed,
        })
        .filter(|s| {
            if query.is_empty() { return true; }
            let q = query.to_lowercase();
            s.name.to_lowercase().contains(&q)
                || s.description.to_lowercase().contains(&q)
                || s.id.to_lowercase().contains(&q)
                || s.developer.to_lowercase().contains(&q)
        })
        .filter(|s| {
            match type_filter {
                Some(t) => s.service_type == t,
                None => true,
            }
        })
        .collect();

    let section_title = match section {
        MarketSection::Decouvrir => "D\u{00e9}couvrir",
        MarketSection::Officiels => "Services Officiels Miyukini",
        MarketSection::Communaute => "Communaut\u{00e9}",
        MarketSection::Installes => "Services Install\u{00e9}s",
    };
    let total = filtered.len();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px;",

            // Header avec titre et compteur
            div {
                style: "display: flex; align-items: center; justify-content: space-between;",
                div {
                    h2 { style: "font-size: 22px; color: {c.text_white}; font-weight: 600;", "{section_title}" }
                    p { style: "font-size: 13px; color: {c.text_muted}; margin-top: 4px;", "{total} service(s)" }
                }
            }

            // Filtres par type
            TypeFilterBar { state: state }

            // Grille de services
            if filtered.is_empty() {
                EmptyState { section: section, has_query: !query.is_empty() }
            } else {
                div {
                    style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(280px, 1fr)); gap: 16px;",

                    for svc in filtered {
                        MarketCard {
                            key: "{svc.id}",
                            service: svc.clone(),
                            state: state,
                        }
                    }
                }
            }
        }
    }
}

// ── Barre de filtres par type ──────────────────────────────────────────

#[component]
fn TypeFilterBar(state: Signal<MarketState>) -> Element {
    let current = state.read().type_filter;

    rsx! {
        div {
            style: "display: flex; gap: 8px; flex-wrap: wrap;",

            FilterChip {
                label: "Tous",
                is_active: current.is_none(),
                onclick: move |_| { state.write().type_filter = None; },
            }
            FilterChip {
                label: "Interne COG",
                color: "#3b82f6",
                is_active: current == Some(ServiceType::InterneCog),
                onclick: move |_| {
                    let new_val = if current == Some(ServiceType::InterneCog) { None } else { Some(ServiceType::InterneCog) };
                    state.write().type_filter = new_val;
                },
            }
            FilterChip {
                label: "Surface Web",
                color: "#10b981",
                is_active: current == Some(ServiceType::SurfaceWeb),
                onclick: move |_| {
                    let new_val = if current == Some(ServiceType::SurfaceWeb) { None } else { Some(ServiceType::SurfaceWeb) };
                    state.write().type_filter = new_val;
                },
            }
            FilterChip {
                label: "Inter-COG",
                color: "#8b5cf6",
                is_active: current == Some(ServiceType::InterCog),
                onclick: move |_| {
                    let new_val = if current == Some(ServiceType::InterCog) { None } else { Some(ServiceType::InterCog) };
                    state.write().type_filter = new_val;
                },
            }
        }
    }
}

#[component]
fn FilterChip(
    label: &'static str,
    #[props(default = "#6b7280")] color: &'static str,
    is_active: bool,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let bg = if is_active { format!("{}20", color) } else { "transparent".to_string() };
    let border_color = if is_active { color } else { c.border };
    let text_color = if is_active { c.text_white } else { c.text_secondary };

    rsx! {
        button {
            style: "padding: 6px 14px; border-radius: 20px; border: 1px solid {border_color}; background: {bg}; color: {text_color}; font-size: 12px; cursor: pointer; transition: all 0.2s; display: flex; align-items: center; gap: 6px;",
            onclick: move |evt| onclick.call(evt),

            if is_active && color != "#6b7280" {
                span { style: "width: 6px; height: 6px; border-radius: 50%; background: {color};" }
            }
            "{label}"
        }
    }
}

// ── Carte d'un service dans le Market ──────────────────────────────────

#[component]
fn MarketCard(service: ServiceInfo, state: Signal<MarketState>) -> Element {
    let c = use_app_state().read().current_theme.palette();

    let svc_id = service.id.clone();
    let svc_for_detail = svc_id.clone();
    let is_installed = service.is_installed;
    let type_color = service.service_type.color();
    let type_label = service.service_type.label();
    let source_color = service.source.badge_color();
    let source_label = service.source.label();

    rsx! {
        div {
            style: "background: {c.bg_card}; border: 1px solid {c.border}; border-radius: 10px; overflow: hidden; cursor: pointer; transition: all 0.2s; display: flex; flex-direction: column;",
            onclick: move |_| {
                state.write().selected_service_id = Some(svc_for_detail.clone());
            },

            // Icone header avec gradient
            div {
                style: "height: 80px; background: linear-gradient(135deg, {type_color}30, {type_color}10); display: flex; align-items: center; justify-content: center; position: relative;",
                span { style: "font-size: 36px; filter: drop-shadow(0 2px 4px rgba(0,0,0,0.3));", "{service.icon}" }

                // Badge source
                span {
                    style: "position: absolute; top: 8px; right: 8px; padding: 2px 8px; background: {source_color}20; color: {source_color}; border: 1px solid {source_color}40; border-radius: 10px; font-size: 10px; font-weight: 500;",
                    "{source_label}"
                }
            }

            // Contenu
            div {
                style: "padding: 14px; flex: 1; display: flex; flex-direction: column;",

                h3 { style: "font-size: 15px; color: {c.text_white}; font-weight: 600; margin-bottom: 6px;", "{service.name}" }
                p {
                    style: "font-size: 12px; color: {c.text_secondary}; line-height: 1.4; flex: 1; display: -webkit-box; -webkit-line-clamp: 2; -webkit-box-orient: vertical; overflow: hidden;",
                    "{service.description}"
                }

                // Footer : type + version + statut
                div {
                    style: "display: flex; align-items: center; justify-content: space-between; margin-top: 12px;",

                    // Type badge
                    span {
                        style: "display: flex; align-items: center; gap: 4px; font-size: 11px; color: {c.text_muted};",
                        span { style: "width: 6px; height: 6px; border-radius: 50%; background: {type_color};" }
                        "{type_label}"
                    }

                    // Version + statut
                    div {
                        style: "display: flex; align-items: center; gap: 6px;",
                        span { style: "font-size: 10px; color: {c.text_muted};", "v{service.version}" }
                        if is_installed {
                            span {
                                style: "padding: 3px 10px; background: {c.accent_green}20; color: {c.accent_green}; border-radius: 10px; font-size: 11px; font-weight: 500;",
                                "Install\u{00e9}"
                            }
                        } else {
                            span {
                                style: "padding: 3px 10px; background: {c.accent_blue}20; color: {c.accent_blue}; border-radius: 10px; font-size: 11px; font-weight: 500;",
                                "Installer"
                            }
                        }
                    }
                }
            }
        }
    }
}

// ── État vide ──────────────────────────────────────────────────────────

#[component]
fn EmptyState(section: MarketSection, has_query: bool) -> Element {
    let c = use_app_state().read().current_theme.palette();

    let (icon, title, description) = if has_query {
        ("\u{1F50D}", "Aucun r\u{00e9}sultat", "Essayez avec d'autres mots-cl\u{00e9}s.")
    } else {
        match section {
            MarketSection::Communaute => ("\u{1F30D}", "Aucun service communautaire", "Les services tiers seront disponibles prochainement."),
            MarketSection::Installes => ("\u{1F4E6}", "Aucun service install\u{00e9}", "Parcourez le catalogue pour d\u{00e9}couvrir les services."),
            _ => ("\u{2728}", "Catalogue vide", "Aucun service disponible pour le moment."),
        }
    };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 80px 32px; text-align: center;",
            span { style: "font-size: 48px; margin-bottom: 16px;", "{icon}" }
            h3 { style: "font-size: 18px; color: {c.text_white}; margin-bottom: 8px;", "{title}" }
            p { style: "font-size: 13px; color: {c.text_muted}; max-width: 320px;", "{description}" }
        }
    }
}
