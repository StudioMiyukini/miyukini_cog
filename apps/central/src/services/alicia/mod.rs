//! Alicia Home Assistante — Assistant vocal local pour la domotique.
//!
//! Interface : tableau de bord, configuration des pieces, dispositifs,
//! automatisations et parametres globaux.
//! Couple aux crates backend miyualicia-capture et miyualicia-wakeword.

use std::sync::{Arc, Mutex};
use std::time::Duration;

use dioxus::prelude::*;
use miyualicia_wakeword::WakeWordConfig;

use crate::state::use_app_state;

pub mod state;
mod automations;
mod dashboard;
mod devices;
mod rooms;
mod settings;

use automations::AutomationsScreen;
use dashboard::DashboardScreen;
use devices::DevicesScreen;
use rooms::RoomsScreen;
use settings::SettingsScreen;
use state::{SharedAliciaService, AliciaService, AliciaSnapshot};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum AliciaTab {
    #[default]
    Dashboard,
    Rooms,
    Devices,
    Automations,
    Settings,
}

#[component]
pub fn AliciaView() -> Element {
    let app_state = use_app_state();
    let mut active_tab = use_signal(|| AliciaTab::Dashboard);

    // Service Alicia partage (initialise une seule fois)
    let mut voice_svc: Signal<Option<SharedAliciaService>> = use_signal(|| None);

    // Snapshot lisible par les sous-composants (mis a jour par le polling)
    let mut snapshot: Signal<Option<AliciaSnapshot>> = use_signal(|| None);

    // Fournir le snapshot et le service aux sous-composants via contexte
    use_context_provider(|| snapshot);
    use_context_provider(|| voice_svc);

    // Initialisation lazy du AliciaService au premier rendu
    let mut init_error: Signal<Option<String>> = use_signal(|| None);
    use_effect(move || {
        let current = voice_svc.read();
        if current.is_some() {
            return;
        }
        drop(current);

        let config = WakeWordConfig::default();
        match AliciaService::new(config) {
            Ok(service) => {
                let shared = Arc::new(Mutex::new(service));
                voice_svc.set(Some(shared));
                tracing::info!("AliciaService initialise");
            }
            Err(e) => {
                tracing::error!("Echec initialisation AliciaService: {e}");
                init_error.set(Some(e));
            }
        }
    });

    // Polling : lire le snapshot toutes les 200ms
    use_future(move || async move {
        loop {
            tokio::time::sleep(Duration::from_millis(200)).await;
            // Clone l'Arc pour relacher le read guard avant de lock
            let svc_clone = {
                let r = voice_svc.read();
                r.as_ref().map(Arc::clone)
            };
            if let Some(svc) = svc_clone {
                if let Ok(guard) = svc.lock() {
                    let snap = guard.snapshot();
                    drop(guard);
                    snapshot.set(Some(snap));
                }
            }
        }
    });

    let theme = app_state.read().current_theme;
    let c = theme.palette();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; height: 100%; overflow: hidden;",

            // En-tete
            div {
                style: "padding: 24px 24px 16px; border-bottom: 1px solid {c.border};",
                div {
                    style: "display: flex; align-items: center; gap: 16px; margin-bottom: 8px;",
                    h1 {
                        style: "font-size: 24px; font-weight: 600; color: {c.text_white};",
                        "Alicia Home Assistante"
                    }
                    StatusBadge {}
                }
                p {
                    style: "font-size: 14px; color: {c.text_secondary}; max-width: 600px;",
                    "Assistant vocal local Alicia \u{2014} ecoute, detection de mot-cle et commandes vocales "
                    "pour ta domotique. 100% hors-ligne, aucune donnee ne quitte ton COG."
                }
                if let Some(ref err) = *init_error.read() {
                    div {
                        style: "margin-top: 12px; padding: 12px; background: {c.bg_secondary}; border-radius: 6px; color: {c.accent_red};",
                        "Erreur d'initialisation : {err}"
                    }
                }
            }

            // Onglets
            div {
                style: "display: flex; gap: 4px; padding: 0 24px; background: {c.bg_header}; border-bottom: 1px solid {c.border};",
                TabButton {
                    label: "Tableau de bord",
                    is_active: *active_tab.read() == AliciaTab::Dashboard,
                    onclick: move |_| active_tab.set(AliciaTab::Dashboard),
                }
                TabButton {
                    label: "Pieces",
                    is_active: *active_tab.read() == AliciaTab::Rooms,
                    onclick: move |_| active_tab.set(AliciaTab::Rooms),
                }
                TabButton {
                    label: "Dispositifs",
                    is_active: *active_tab.read() == AliciaTab::Devices,
                    onclick: move |_| active_tab.set(AliciaTab::Devices),
                }
                TabButton {
                    label: "Automatisations",
                    is_active: *active_tab.read() == AliciaTab::Automations,
                    onclick: move |_| active_tab.set(AliciaTab::Automations),
                }
                TabButton {
                    label: "Parametres",
                    is_active: *active_tab.read() == AliciaTab::Settings,
                    onclick: move |_| active_tab.set(AliciaTab::Settings),
                }
            }

            // Contenu
            div {
                style: "flex: 1; overflow-y: auto; padding: 24px;",
                match *active_tab.read() {
                    AliciaTab::Dashboard => rsx! { DashboardScreen {} },
                    AliciaTab::Rooms => rsx! { RoomsScreen {} },
                    AliciaTab::Devices => rsx! { DevicesScreen {} },
                    AliciaTab::Automations => rsx! { AutomationsScreen {} },
                    AliciaTab::Settings => rsx! { SettingsScreen {} },
                }
            }
        }
    }
}

/// Badge de statut global "Alicia ecoute" / "En veille".
#[component]
fn StatusBadge() -> Element {
    let app_state = use_app_state();
    let c = app_state.read().current_theme.palette();
    let snapshot = use_context::<Signal<Option<AliciaSnapshot>>>();

    let is_listening = snapshot
        .read()
        .as_ref()
        .is_some_and(|s| s.any_listening);

    let badge_bg = if is_listening { c.accent_green } else { c.bg_hover };
    let badge_label = if is_listening {
        "Alicia ecoute"
    } else {
        "En veille"
    };
    let dot_color = if is_listening { "#4ade80" } else { c.text_muted };

    rsx! {
        div {
            style: "display: inline-flex; align-items: center; gap: 6px; padding: 4px 12px; background: {badge_bg}20; border: 1px solid {badge_bg}40; border-radius: 16px;",
            div {
                style: "width: 8px; height: 8px; border-radius: 50%; background: {dot_color};",
            }
            span {
                style: "font-size: 12px; font-weight: 500; color: {badge_bg};",
                "{badge_label}"
            }
        }
    }
}

/// Bouton d'onglet local (meme pattern que MiyukiniWatch).
#[component]
fn TabButton(
    label: &'static str,
    is_active: bool,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let app_state = use_app_state();
    let c = app_state.read().current_theme.palette();
    let bg = if is_active { c.accent_blue } else { "transparent" };
    let color = if is_active { c.text_white } else { c.text_secondary };
    let border_color = if is_active { c.accent_blue } else { "transparent" };

    rsx! {
        button {
            style: "padding: 12px 20px; background: {bg}; color: {color}; border: none; cursor: pointer; font-size: 14px; border-bottom: 2px solid {border_color};",
            onclick: move |evt| onclick.call(evt),
            "{label}"
        }
    }
}
