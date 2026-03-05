//! Module JayKonta — comptabilite unifiee Purse + Account.
//!
//! Architecture : sidebar + contenu dynamique par espace (Purse/Account).

pub mod components;
mod entry_point;
mod purse_dashboard;
mod purse_forecast;
mod purse_movement_form;
mod purse_movements;
mod purse_recurring;
mod sidebar;

use dioxus::prelude::*;

use entry_point::EntryPointSelector;
use purse_dashboard::PurseDashboard;
use purse_forecast::PurseForecast;
use purse_movement_form::PurseMovementForm;
use purse_movements::PurseMovements;
use purse_recurring::PurseRecurring;
use sidebar::JayKontaSidebar;

// ── State ──────────────────────────────────────────────────────────────

/// Espace actif dans JayKonta.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum JayKontaSpace {
    #[default]
    Selector,
    Purse,
    Account,
}

/// Section active cote Purse.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PurseSection {
    #[default]
    Dashboard,
    Mouvements,
    NouveauMouvement,
    Recurrents,
    Previsionnel,
    Budgets,
    Objectifs,
    Rapports,
    Alertes,
    Parametres,
}

/// Section active cote Account.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AccountSection {
    #[default]
    Dashboard,
    Journal,
    Devis,
    Factures,
    Paiements,
    Rapports,
    Integrations,
    Parametres,
}

/// Etat local complet de la vue JayKonta.
#[derive(Debug, Clone, Default)]
pub struct JayKontaState {
    pub space: JayKontaSpace,
    pub purse_section: PurseSection,
    pub account_section: AccountSection,
}

// ── Composant racine ───────────────────────────────────────────────────

#[component]
pub fn JayKontaView() -> Element {
    let state = use_signal(JayKontaState::default);
    let current_space = state.read().space;

    match current_space {
        JayKontaSpace::Selector => rsx! {
            EntryPointSelector { state: state }
        },
        JayKontaSpace::Purse | JayKontaSpace::Account => rsx! {
            div {
                style: "display: flex; height: 100%;",

                JayKontaSidebar { state: state }

                main {
                    style: "flex: 1; padding: 24px; overflow-y: auto;",

                    {
                        let purse_section = state.read().purse_section;
                        let account_section = state.read().account_section;
                        match current_space {
                        JayKontaSpace::Purse => {
                            match purse_section {
                                PurseSection::Dashboard => rsx! { PurseDashboard { state: state } },
                                PurseSection::Mouvements => rsx! { PurseMovements { state: state } },
                                PurseSection::NouveauMouvement => rsx! { PurseMovementForm { state: state } },
                                PurseSection::Recurrents => rsx! { PurseRecurring { state: state } },
                                PurseSection::Previsionnel => rsx! { PurseForecast { state: state } },
                                PurseSection::Budgets => rsx! { components::PlaceholderSection { title: "Budgets occasionnels", icon: "\u{1F3AF}" } },
                                PurseSection::Objectifs => rsx! { components::PlaceholderSection { title: "Objectifs", icon: "\u{1F3C6}" } },
                                PurseSection::Rapports => rsx! { components::PlaceholderSection { title: "Rapports", icon: "\u{1F4C8}" } },
                                PurseSection::Alertes => rsx! { components::PlaceholderSection { title: "Alertes et rappels", icon: "\u{1F514}" } },
                                PurseSection::Parametres => rsx! { components::PlaceholderSection { title: "Parametres Purse", icon: "\u{2699}\u{FE0F}" } },
                            }
                        },
                        JayKontaSpace::Account => {
                            match account_section {
                                AccountSection::Dashboard => rsx! { components::PlaceholderSection { title: "Dashboard Account", icon: "\u{1F4CA}" } },
                                AccountSection::Journal => rsx! { components::PlaceholderSection { title: "Journal / Grand Livre", icon: "\u{1F4D2}" } },
                                AccountSection::Devis => rsx! { components::PlaceholderSection { title: "Devis", icon: "\u{1F4CB}" } },
                                AccountSection::Factures => rsx! { components::PlaceholderSection { title: "Factures", icon: "\u{1F9FE}" } },
                                AccountSection::Paiements => rsx! { components::PlaceholderSection { title: "Paiements", icon: "\u{1F4B3}" } },
                                AccountSection::Rapports => rsx! { components::PlaceholderSection { title: "Rapports legaux", icon: "\u{1F4C8}" } },
                                AccountSection::Integrations => rsx! { components::PlaceholderSection { title: "Integrations", icon: "\u{1F517}" } },
                                AccountSection::Parametres => rsx! { components::PlaceholderSection { title: "Parametres Account", icon: "\u{2699}\u{FE0F}" } },
                            }
                        },
                        JayKontaSpace::Selector => rsx! {},
                    }
                    }
                }
            }
        },
    }
}
