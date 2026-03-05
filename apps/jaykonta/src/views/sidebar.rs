//! Sidebar JayKonta — navigation Purse / Account.

use super::{AccountSection, JayKontaSpace, JayKontaState, PurseSection};
use dioxus::prelude::*;
use miyukini_service_ui::use_palette;

#[component]
pub fn JayKontaSidebar(state: Signal<JayKontaState>) -> Element {
    let c = use_palette();
    let current_space = state.read().space;
    let is_purse = current_space == JayKontaSpace::Purse;

    rsx! {
        aside {
            style: "width: 220px; background: {c.bg_secondary}; border-right: 1px solid {c.border}; padding: 16px 0; display: flex; flex-direction: column; gap: 0;",

            div {
                style: "padding: 0 16px 16px 16px; border-bottom: 1px solid {c.border};",
                h3 {
                    style: "font-size: 16px; color: {c.text_white}; margin-bottom: 8px;",
                    if is_purse { "JayBudget" } else { "JayKonta" }
                }
                div {
                    style: "display: flex; gap: 4px;",
                    SpaceTab {
                        label: "Purse",
                        icon: "\u{1F4B0}",
                        is_active: is_purse,
                        onclick: move |_| { state.write().space = JayKontaSpace::Purse; },
                    }
                    SpaceTab {
                        label: "Account",
                        icon: "\u{1F4CA}",
                        is_active: !is_purse,
                        onclick: move |_| { state.write().space = JayKontaSpace::Account; },
                    }
                }
            }

            nav {
                style: "display: flex; flex-direction: column; gap: 4px; padding-top: 12px; flex: 1;",

                if is_purse {
                    SidebarItem { icon: "\u{1F4CA}", label: "Tableau de bord", is_active: state.read().purse_section == PurseSection::Dashboard, onclick: move |_| { state.write().purse_section = PurseSection::Dashboard; } }
                    SidebarItem { icon: "\u{1F4DD}", label: "Mouvements", is_active: state.read().purse_section == PurseSection::Mouvements, onclick: move |_| { state.write().purse_section = PurseSection::Mouvements; } }
                    SidebarItem { icon: "\u{1F504}", label: "Recurrents", is_active: state.read().purse_section == PurseSection::Recurrents, onclick: move |_| { state.write().purse_section = PurseSection::Recurrents; } }
                    SidebarItem { icon: "\u{1F4C5}", label: "Previsionnel", is_active: state.read().purse_section == PurseSection::Previsionnel, onclick: move |_| { state.write().purse_section = PurseSection::Previsionnel; } }
                    SidebarItem { icon: "\u{1F3AF}", label: "Budgets occas.", is_active: state.read().purse_section == PurseSection::Budgets, onclick: move |_| { state.write().purse_section = PurseSection::Budgets; } }
                    SidebarItem { icon: "\u{1F3C6}", label: "Objectifs", is_active: state.read().purse_section == PurseSection::Objectifs, onclick: move |_| { state.write().purse_section = PurseSection::Objectifs; } }
                    SidebarItem { icon: "\u{1F4C8}", label: "Rapports", is_active: state.read().purse_section == PurseSection::Rapports, onclick: move |_| { state.write().purse_section = PurseSection::Rapports; } }
                    SidebarItem { icon: "\u{1F514}", label: "Alertes", is_active: state.read().purse_section == PurseSection::Alertes, onclick: move |_| { state.write().purse_section = PurseSection::Alertes; } }
                    div { style: "height: 1px; background: {c.border}; margin: 8px 16px;" }
                    SidebarItem { icon: "\u{2699}\u{FE0F}", label: "Parametres", is_active: state.read().purse_section == PurseSection::Parametres, onclick: move |_| { state.write().purse_section = PurseSection::Parametres; } }
                } else {
                    SidebarItem { icon: "\u{1F4CA}", label: "Tableau de bord", is_active: state.read().account_section == AccountSection::Dashboard, onclick: move |_| { state.write().account_section = AccountSection::Dashboard; } }
                    SidebarItem { icon: "\u{1F4D2}", label: "Journal / GL", is_active: state.read().account_section == AccountSection::Journal, onclick: move |_| { state.write().account_section = AccountSection::Journal; } }
                    SidebarItem { icon: "\u{1F4CB}", label: "Devis", is_active: state.read().account_section == AccountSection::Devis, onclick: move |_| { state.write().account_section = AccountSection::Devis; } }
                    SidebarItem { icon: "\u{1F9FE}", label: "Factures", is_active: state.read().account_section == AccountSection::Factures, onclick: move |_| { state.write().account_section = AccountSection::Factures; } }
                    SidebarItem { icon: "\u{1F4B3}", label: "Paiements", is_active: state.read().account_section == AccountSection::Paiements, onclick: move |_| { state.write().account_section = AccountSection::Paiements; } }
                    SidebarItem { icon: "\u{1F4C8}", label: "Rapports", is_active: state.read().account_section == AccountSection::Rapports, onclick: move |_| { state.write().account_section = AccountSection::Rapports; } }
                    SidebarItem { icon: "\u{1F517}", label: "Integrations", is_active: state.read().account_section == AccountSection::Integrations, onclick: move |_| { state.write().account_section = AccountSection::Integrations; } }
                    div { style: "height: 1px; background: {c.border}; margin: 8px 16px;" }
                    SidebarItem { icon: "\u{2699}\u{FE0F}", label: "Parametres", is_active: state.read().account_section == AccountSection::Parametres, onclick: move |_| { state.write().account_section = AccountSection::Parametres; } }
                }
            }

            div {
                style: "padding: 12px 16px; border-top: 1px solid {c.border};",
                button {
                    style: "display: flex; align-items: center; gap: 8px; width: 100%; padding: 8px 12px; background: transparent; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_secondary}; cursor: pointer; font-size: 12px;",
                    onclick: move |_| { state.write().space = JayKontaSpace::Selector; },
                    span { "\u{2194}" }
                    span { if is_purse { "Basculer vers Account" } else { "Basculer vers Purse" } }
                }
            }
        }
    }
}

#[component]
fn SpaceTab(
    label: &'static str,
    icon: &'static str,
    is_active: bool,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let c = use_palette();
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

#[component]
fn SidebarItem(
    icon: &'static str,
    label: &'static str,
    is_active: bool,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let c = use_palette();
    let bg = if is_active { c.bg_hover } else { "transparent" };
    let color = if is_active {
        c.text_white
    } else {
        c.text_secondary
    };
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
