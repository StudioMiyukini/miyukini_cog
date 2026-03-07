//! Sidebar JayFestival — navigation par rôle et sections.
//!
//! @id: jf_sidebar @do: render_sidebar_navigation
//! @role: ui @layer: service
//! @human: Sidebar JayFestival: navigation par role (ORG/EXP/VIS) et sections.

use dioxus::prelude::*;
use crate::state::use_app_state;
use super::{
    ExpSection, JayFestivalRole, JayFestivalState, OrgSection, VisSection,
};

#[component]
pub fn JayFestivalSidebar(state: Signal<JayFestivalState>) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let current_role = state.read().role;

    rsx! {
        aside {
            style: "width: 220px; background: {c.bg_secondary}; border-right: 1px solid {c.border}; padding: 16px 0; display: flex; flex-direction: column;",

            // En-tête service
            div {
                style: "padding: 0 16px 16px 16px; border-bottom: 1px solid {c.border};",

                h3 {
                    style: "font-size: 16px; color: {c.text_white}; margin-bottom: 8px;",
                    "JayFestival"
                }

                // Sélecteur de rôle
                div {
                    style: "display: flex; gap: 4px;",

                    RoleTab {
                        label: "Org",
                        is_active: current_role == JayFestivalRole::Organisateur,
                        onclick: move |_| {
                            state.write().role = JayFestivalRole::Organisateur;
                        },
                    }
                    RoleTab {
                        label: "Exp",
                        is_active: current_role == JayFestivalRole::Exposant,
                        onclick: move |_| {
                            state.write().role = JayFestivalRole::Exposant;
                        },
                    }
                    RoleTab {
                        label: "Vis",
                        is_active: current_role == JayFestivalRole::Visiteur,
                        onclick: move |_| {
                            state.write().role = JayFestivalRole::Visiteur;
                        },
                    }
                }
            }

            // Navigation sections selon le rôle
            nav {
                style: "display: flex; flex-direction: column; gap: 4px; padding-top: 12px;",

                match current_role {
                    JayFestivalRole::Organisateur => rsx! {
                        SidebarItem {
                            icon: "📊",
                            label: "Dashboard",
                            is_active: state.read().org_section == OrgSection::Dashboard,
                            onclick: move |_| {
                                state.write().org_section = OrgSection::Dashboard;
                            },
                        }
                        SidebarItem {
                            icon: "📅",
                            label: "Editions",
                            is_active: state.read().org_section == OrgSection::Editions,
                            onclick: move |_| {
                                state.write().org_section = OrgSection::Editions;
                            },
                        }
                        if state.read().selected_edition_id.is_some() {
                            SidebarItem {
                                icon: "🎪",
                                label: "Edition en cours",
                                is_active: state.read().org_section == OrgSection::EditionHub,
                                onclick: move |_| {
                                    state.write().org_section = OrgSection::EditionHub;
                                },
                            }
                        }
                        SidebarItem {
                            icon: "👥",
                            label: "Equipe",
                            is_active: state.read().org_section == OrgSection::Equipe,
                            onclick: move |_| {
                                state.write().org_section = OrgSection::Equipe;
                            },
                        }
                        SidebarItem {
                            icon: "👤",
                            label: "Mon compte",
                            is_active: state.read().org_section == OrgSection::Compte,
                            onclick: move |_| {
                                state.write().org_section = OrgSection::Compte;
                            },
                        }
                    },
                    JayFestivalRole::Exposant => rsx! {
                        SidebarItem {
                            icon: "📊",
                            label: "Dashboard",
                            is_active: state.read().exp_section == ExpSection::Dashboard,
                            onclick: move |_| {
                                state.write().exp_section = ExpSection::Dashboard;
                            },
                        }
                        SidebarItem {
                            icon: "📝",
                            label: "Candidatures",
                            is_active: state.read().exp_section == ExpSection::Candidatures,
                            onclick: move |_| {
                                state.write().exp_section = ExpSection::Candidatures;
                            },
                        }
                        SidebarItem {
                            icon: "🎪",
                            label: "Participations",
                            is_active: state.read().exp_section == ExpSection::Participations,
                            onclick: move |_| {
                                state.write().exp_section = ExpSection::Participations;
                            },
                        }
                        SidebarItem {
                            icon: "📅",
                            label: "Agenda",
                            is_active: state.read().exp_section == ExpSection::Agenda,
                            onclick: move |_| {
                                state.write().exp_section = ExpSection::Agenda;
                            },
                        }
                        SidebarItem {
                            icon: "📁",
                            label: "Documents",
                            is_active: state.read().exp_section == ExpSection::Documents,
                            onclick: move |_| {
                                state.write().exp_section = ExpSection::Documents;
                            },
                        }
                        SidebarItem {
                            icon: "💰",
                            label: "Factures",
                            is_active: state.read().exp_section == ExpSection::Factures,
                            onclick: move |_| {
                                state.write().exp_section = ExpSection::Factures;
                            },
                        }
                        SidebarItem {
                            icon: "🏪",
                            label: "Ma fiche",
                            is_active: state.read().exp_section == ExpSection::FichePublique,
                            onclick: move |_| {
                                state.write().exp_section = ExpSection::FichePublique;
                            },
                        }
                        SidebarItem {
                            icon: "🔔",
                            label: "Notifications",
                            is_active: state.read().exp_section == ExpSection::Notifications,
                            onclick: move |_| {
                                state.write().exp_section = ExpSection::Notifications;
                            },
                        }
                        SidebarItem {
                            icon: "👤",
                            label: "Mon compte",
                            is_active: state.read().exp_section == ExpSection::Compte,
                            onclick: move |_| {
                                state.write().exp_section = ExpSection::Compte;
                            },
                        }
                    },
                    JayFestivalRole::Visiteur => rsx! {
                        SidebarItem {
                            icon: "📊",
                            label: "Dashboard",
                            is_active: state.read().vis_section == VisSection::Dashboard,
                            onclick: move |_| {
                                state.write().vis_section = VisSection::Dashboard;
                            },
                        }
                        SidebarItem {
                            icon: "📋",
                            label: "Catalogue",
                            is_active: state.read().vis_section == VisSection::Catalogue,
                            onclick: move |_| {
                                state.write().vis_section = VisSection::Catalogue;
                            },
                        }
                        SidebarItem {
                            icon: "📅",
                            label: "Agenda",
                            is_active: state.read().vis_section == VisSection::Agenda,
                            onclick: move |_| {
                                state.write().vis_section = VisSection::Agenda;
                            },
                        }
                        SidebarItem {
                            icon: "🎫",
                            label: "Mes billets",
                            is_active: state.read().vis_section == VisSection::Billets,
                            onclick: move |_| {
                                state.write().vis_section = VisSection::Billets;
                            },
                        }
                        SidebarItem {
                            icon: "📆",
                            label: "Reservations",
                            is_active: state.read().vis_section == VisSection::Reservations,
                            onclick: move |_| {
                                state.write().vis_section = VisSection::Reservations;
                            },
                        }
                        SidebarItem {
                            icon: "🎮",
                            label: "Activites",
                            is_active: state.read().vis_section == VisSection::Activites,
                            onclick: move |_| {
                                state.write().vis_section = VisSection::Activites;
                            },
                        }
                        SidebarItem {
                            icon: "👤",
                            label: "Mon compte",
                            is_active: state.read().vis_section == VisSection::Compte,
                            onclick: move |_| {
                                state.write().vis_section = VisSection::Compte;
                            },
                        }
                    },
                }
            }

            // Spacer
            div {
                style: "flex: 1;",
            }

            // Déconnexion
            div {
                style: "padding: 16px; border-top: 1px solid {c.border};",

                button {
                    style: "display: flex; align-items: center; gap: 8px; width: 100%; padding: 10px 12px; background: transparent; border: 1px solid {c.border}; border-radius: 6px; color: {c.text_secondary}; cursor: pointer; font-size: 13px;",
                    onclick: move |_| {
                        // Reset state pour déconnexion
                        state.write().is_connected = false;
                        state.write().current_user_id = None;
                        state.write().role = JayFestivalRole::Organisateur;
                        state.write().org_section = OrgSection::Dashboard;
                        state.write().exp_section = ExpSection::Dashboard;
                        state.write().vis_section = VisSection::Dashboard;
                    },

                    span { "🚪" }
                    span { "Deconnexion" }
                }
            }
        }
    }
}

/// Onglet de rôle compact.
#[component]
fn RoleTab(label: &'static str, is_active: bool, onclick: EventHandler<MouseEvent>) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let bg = if is_active { c.accent_blue } else { c.bg_hover };
    let color = if is_active { "white" } else { c.text_secondary };

    rsx! {
        button {
            style: "flex: 1; padding: 6px 8px; background: {bg}; color: {color}; border: none; border-radius: 4px; cursor: pointer; font-size: 11px; font-weight: 500;",
            onclick: move |evt| onclick.call(evt),
            "{label}"
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
