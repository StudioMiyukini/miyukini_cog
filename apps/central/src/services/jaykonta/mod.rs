//! Module JayKonta — comptabilite unifiee Purse + Account.
//!
//! Architecture : sidebar + contenu dynamique par espace (Purse/Account).
//! Données réelles via JayKontaDb (conns.read().jaykonta).

mod sidebar;
mod components;
mod entry_point;
mod purse_dashboard;
mod purse_movement_form;
mod purse_movements;
mod purse_recurring;
mod purse_forecast;

use dioxus::prelude::*;
use crate::state::use_app_state;

use sidebar::JayKontaSidebar;
use entry_point::EntryPointSelector;
use purse_dashboard::PurseDashboard;
use purse_movement_form::PurseMovementForm;
use purse_movements::PurseMovements;
use purse_recurring::PurseRecurring;
use purse_forecast::PurseForecast;

// ── State ──────────────────────────────────────────────────────────────

/// Espace actif dans JayKonta.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JayKontaSpace {
    /// Choix initial.
    Selector,
    /// Budget personnel (JayBudget).
    Purse,
    /// Comptabilite entreprise (JayKonta).
    Account,
}

impl Default for JayKontaSpace {
    fn default() -> Self {
        Self::Selector
    }
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
    /// Espace actif (Purse/Account/Selector).
    pub space: JayKontaSpace,
    /// Section Purse active.
    pub purse_section: PurseSection,
    /// Section Account active.
    pub account_section: AccountSection,
}

// ── Composant racine ───────────────────────────────────────────────────

#[component]
pub fn JayKontaView() -> Element {
    let _app_state = use_app_state();
    let state = use_signal(JayKontaState::default);
    let current_space = state.read().space;

    match current_space {
        JayKontaSpace::Selector => rsx! {
            EntryPointSelector { state: state }
        },
        JayKontaSpace::Purse | JayKontaSpace::Account => rsx! {
            div {
                style: "display: flex; height: 100%;",

                // Sidebar
                JayKontaSidebar { state: state }

                // Contenu principal
                main {
                    style: "flex: 1; padding: 24px; overflow-y: auto;",

                    {
                        let purse_section = state.read().purse_section;
                        let account_section = state.read().account_section;
                        match current_space {
                        JayKontaSpace::Purse => {
                            match purse_section {
                                PurseSection::Dashboard => rsx! {
                                    PurseDashboard { state: state }
                                },
                                PurseSection::Mouvements => rsx! {
                                    PurseMovements { state: state }
                                },
                                PurseSection::NouveauMouvement => rsx! {
                                    PurseMovementForm { state: state }
                                },
                                PurseSection::Recurrents => rsx! {
                                    PurseRecurring { state: state }
                                },
                                PurseSection::Previsionnel => rsx! {
                                    PurseForecast { state: state }
                                },
                                PurseSection::Budgets => rsx! {
                                    components::PlaceholderSection { title: "Budgets occasionnels", icon: "🎯" }
                                },
                                PurseSection::Objectifs => rsx! {
                                    components::PlaceholderSection { title: "Objectifs", icon: "🏆" }
                                },
                                PurseSection::Rapports => rsx! {
                                    components::PlaceholderSection { title: "Rapports", icon: "📈" }
                                },
                                PurseSection::Alertes => rsx! {
                                    components::PlaceholderSection { title: "Alertes et rappels", icon: "🔔" }
                                },
                                PurseSection::Parametres => rsx! {
                                    components::PlaceholderSection { title: "Parametres Purse", icon: "⚙️" }
                                },
                            }
                        },
                        JayKontaSpace::Account => {
                            match account_section {
                                AccountSection::Dashboard => rsx! {
                                    components::PlaceholderSection { title: "Dashboard Account", icon: "📊" }
                                },
                                AccountSection::Journal => rsx! {
                                    components::PlaceholderSection { title: "Journal / Grand Livre", icon: "📒" }
                                },
                                AccountSection::Devis => rsx! {
                                    components::PlaceholderSection { title: "Devis", icon: "📋" }
                                },
                                AccountSection::Factures => rsx! {
                                    components::PlaceholderSection { title: "Factures", icon: "🧾" }
                                },
                                AccountSection::Paiements => rsx! {
                                    components::PlaceholderSection { title: "Paiements", icon: "💳" }
                                },
                                AccountSection::Rapports => rsx! {
                                    components::PlaceholderSection { title: "Rapports legaux", icon: "📈" }
                                },
                                AccountSection::Integrations => rsx! {
                                    components::PlaceholderSection { title: "Integrations", icon: "🔗" }
                                },
                                AccountSection::Parametres => rsx! {
                                    components::PlaceholderSection { title: "Parametres Account", icon: "⚙️" }
                                },
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
