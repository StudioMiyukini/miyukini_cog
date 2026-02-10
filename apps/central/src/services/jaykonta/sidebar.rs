//! Sidebar JayKonta — navigation Purse / Account.

use dioxus::prelude::*;
use crate::state::use_app_state;
use super::{AccountSection, JayKontaSpace, JayKontaState, PurseSection};

#[component]
pub fn JayKontaSidebar(state: Signal<JayKontaState>) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let current_space = state.read().space;
    let is_purse = current_space == JayKontaSpace::Purse;

    rsx! {
        aside {
            style: "width: 220px; background: {c.bg_secondary}; border-right: 1px solid {c.border}; padding: 16px 0; display: flex; flex-direction: column; gap: 0;",

            // En-tete service
            div {
                style: "padding: 0 16px 16px 16px; border-bottom: 1px solid {c.border};",

                h3 {
                    style: "font-size: 16px; color: {c.text_white}; margin-bottom: 8px;",
                    if is_purse { "JayBudget" } else { "JayKonta" }
                }

                // Selecteur Purse / Account
                div {
                    style: "display: flex; gap: 4px;",

                    SpaceTab {
                        label: "Purse",
                        icon: "💰",
                        is_active: is_purse,
                        onclick: move |_| {
                            state.write().space = JayKontaSpace::Purse;
                        },
                    }
                    SpaceTab {
                        label: "Account",
                        icon: "📊",
                        is_active: !is_purse,
                        onclick: move |_| {
                            state.write().space = JayKontaSpace::Account;
                        },
                    }
                }
            }

            // Navigation sections
            nav {
                style: "display: flex; flex-direction: column; gap: 4px; padding-top: 12px; flex: 1;",

                if is_purse {
                    SidebarItem {
                        icon: "📊",
                        label: "Tableau de bord",
                        is_active: state.read().purse_section == PurseSection::Dashboard,
                        onclick: move |_| { state.write().purse_section = PurseSection::Dashboard; },
                    }
                    SidebarItem {
                        icon: "📝",
                        label: "Mouvements",
                        is_active: state.read().purse_section == PurseSection::Mouvements,
                        onclick: move |_| { state.write().purse_section = PurseSection::Mouvements; },
                    }
                    SidebarItem {
                        icon: "🔄",
                        label: "Recurrents",
                        is_active: state.read().purse_section == PurseSection::Recurrents,
                        onclick: move |_| { state.write().purse_section = PurseSection::Recurrents; },
                    }
                    SidebarItem {
                        icon: "📅",
                        label: "Previsionnel",
                        is_active: state.read().purse_section == PurseSection::Previsionnel,
                        onclick: move |_| { state.write().purse_section = PurseSection::Previsionnel; },
                    }
                    SidebarItem {
                        icon: "🎯",
                        label: "Budgets occas.",
                        is_active: state.read().purse_section == PurseSection::Budgets,
                        onclick: move |_| { state.write().purse_section = PurseSection::Budgets; },
                    }
                    SidebarItem {
                        icon: "🏆",
                        label: "Objectifs",
                        is_active: state.read().purse_section == PurseSection::Objectifs,
                        onclick: move |_| { state.write().purse_section = PurseSection::Objectifs; },
                    }
                    SidebarItem {
                        icon: "📈",
                        label: "Rapports",
                        is_active: state.read().purse_section == PurseSection::Rapports,
                        onclick: move |_| { state.write().purse_section = PurseSection::Rapports; },
                    }
                    SidebarItem {
                        icon: "🔔",
                        label: "Alertes",
                        is_active: state.read().purse_section == PurseSection::Alertes,
                        onclick: move |_| { state.write().purse_section = PurseSection::Alertes; },
                    }

                    // Separateur
                    div { style: "height: 1px; background: {c.border}; margin: 8px 16px;" }

                    SidebarItem {
                        icon: "⚙️",
                        label: "Parametres",
                        is_active: state.read().purse_section == PurseSection::Parametres,
                        onclick: move |_| { state.write().purse_section = PurseSection::Parametres; },
                    }
                } else {
                    SidebarItem {
                        icon: "📊",
                        label: "Tableau de bord",
                        is_active: state.read().account_section == AccountSection::Dashboard,
                        onclick: move |_| { state.write().account_section = AccountSection::Dashboard; },
                    }
                    SidebarItem {
                        icon: "📒",
                        label: "Journal / GL",
                        is_active: state.read().account_section == AccountSection::Journal,
                        onclick: move |_| { state.write().account_section = AccountSection::Journal; },
                    }
                    SidebarItem {
                        icon: "📋",
                        label: "Devis",
                        is_active: state.read().account_section == AccountSection::Devis,
                        onclick: move |_| { state.write().account_section = AccountSection::Devis; },
                    }
                    SidebarItem {
                        icon: "🧾",
                        label: "Factures",
                        is_active: state.read().account_section == AccountSection::Factures,
                        onclick: move |_| { state.write().account_section = AccountSection::Factures; },
                    }
                    SidebarItem {
                        icon: "💳",
                        label: "Paiements",
                        is_active: state.read().account_section == AccountSection::Paiements,
                        onclick: move |_| { state.write().account_section = AccountSection::Paiements; },
                    }
                    SidebarItem {
                        icon: "📈",
                        label: "Rapports",
                        is_active: state.read().account_section == AccountSection::Rapports,
                        onclick: move |_| { state.write().account_section = AccountSection::Rapports; },
                    }
                    SidebarItem {
                        icon: "🔗",
                        label: "Integrations",
                        is_active: state.read().account_section == AccountSection::Integrations,
                        onclick: move |_| { state.write().account_section = AccountSection::Integrations; },
                    }

                    // Separateur
                    div { style: "height: 1px; background: {c.border}; margin: 8px 16px;" }

                    SidebarItem {
                        icon: "⚙️",
                        label: "Parametres",
                        is_active: state.read().account_section == AccountSection::Parametres,
                        onclick: move |_| { state.write().account_section = AccountSection::Parametres; },
                    }
                }
            }

            // Bouton retour au selecteur
            div {
                style: "padding: 12px 16px; border-top: 1px solid {c.border};",

                button {
                    style: "display: flex; align-items: center; gap: 8px; width: 100%; padding: 8px 12px; background: transparent; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_secondary}; cursor: pointer; font-size: 12px;",
                    onclick: move |_| {
                        state.write().space = JayKontaSpace::Selector;
                    },
                    span { "↔" }
                    span { if is_purse { "Basculer vers Account" } else { "Basculer vers Purse" } }
                }
            }
        }
    }
}

/// Onglet de selection Purse/Account.
#[component]
fn SpaceTab(label: &'static str, icon: &'static str, is_active: bool, onclick: EventHandler<MouseEvent>) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let bg = if is_active { c.accent_blue } else { c.bg_hover };
    let color = if is_active { "white" } else { c.text_secondary };

    rsx! {
        button {
            style: "flex: 1; padding: 6px 8px; background: {bg}; color: {color}; border: none; border-radius: 4px; cursor: pointer; font-size: 11px; font-weight: 500; display: flex; align-items: center; justify-content: center; gap: 4px;",
            onclick: move |evt| onclick.call(evt),
            span { "{icon}" }
            span { "{label}" }
        }
    }
}

/// Item de navigation sidebar.
#[component]
fn SidebarItem(
    icon: &'static str,
    label: &'static str,
    is_active: bool,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let bg = if is_active { c.bg_hover } else { "transparent" };
    let color = if is_active { c.text_white } else { c.text_secondary };
    let border = if is_active {
        format!("2px solid {}", c.accent_blue)
    } else {
        "2px solid transparent".to_string()
    };

    rsx! {
        button {
            style: "display: flex; align-items: center; gap: 12px; padding: 10px 16px; background: {bg}; color: {color}; border: none; border-left: {border}; cursor: pointer; font-size: 13px; text-align: left; width: 100%; transition: all 0.2s;",
            onclick: move |evt| onclick.call(evt),
            span { "{icon}" }
            span { "{label}" }
        }
    }
}
