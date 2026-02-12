//! Composant ServiceSidebar generique pour les services Miyukini.
//!
//! @id: central.components.service_sidebar
//! @do: render_generic_service_sidebar_with_roles_and_sections
//! @role: ui
//! @layer: ui
//! @human: Sidebar generique reutilisable pour tous les services Jay*
//!
//! Note: API publique réutilisable — certains builders/variants ne sont
//! pas encore consommés mais font partie du contrat du composant.
#![allow(dead_code)]

use dioxus::prelude::*;
use crate::state::use_app_state;

/// Definition d'une section dans la sidebar.
#[derive(Clone, PartialEq)]
pub struct SidebarSection {
    /// Identifiant unique de la section.
    pub id: String,
    /// Icone emoji de la section.
    pub icon: &'static str,
    /// Label affiche.
    pub label: &'static str,
    /// Badge optionnel (compteur).
    pub badge: Option<String>,
}

impl SidebarSection {
    /// Cree une nouvelle section.
    pub fn new(id: impl Into<String>, icon: &'static str, label: &'static str) -> Self {
        Self {
            id: id.into(),
            icon,
            label,
            badge: None,
        }
    }

    /// Ajoute un badge a la section.
    #[must_use]
    pub fn with_badge(mut self, badge: impl Into<String>) -> Self {
        self.badge = Some(badge.into());
        self
    }

    /// Ajoute un badge numerique si > 0.
    #[must_use]
    pub fn with_count(mut self, count: usize) -> Self {
        if count > 0 {
            self.badge = Some(count.to_string());
        }
        self
    }
}

/// Definition d'un role/onglet dans la sidebar.
#[derive(Clone, PartialEq)]
pub struct SidebarRole {
    /// Identifiant unique du role.
    pub id: String,
    /// Label affiche.
    pub label: &'static str,
    /// Icone optionnelle.
    pub icon: Option<&'static str>,
}

impl SidebarRole {
    /// Cree un nouveau role.
    pub fn new(id: impl Into<String>, label: &'static str) -> Self {
        Self {
            id: id.into(),
            label,
            icon: None,
        }
    }

    /// Ajoute une icone au role.
    #[must_use]
    pub fn with_icon(mut self, icon: &'static str) -> Self {
        self.icon = Some(icon);
        self
    }
}

/// Element de footer personnalise.
#[derive(Clone, PartialEq)]
pub enum SidebarFooter {
    /// Bouton de deconnexion.
    Logout,
    /// Texte informatif.
    Info(&'static str),
    /// Bouton personnalise.
    Button {
        icon: &'static str,
        label: &'static str,
    },
    /// Aucun footer.
    None,
}

/// Props pour le composant ServiceSidebar.
#[derive(Props, Clone, PartialEq)]
pub struct ServiceSidebarProps {
    /// Titre du service (ex: "JayFestival", "JayKonta").
    pub title: &'static str,
    /// Sous-titre optionnel.
    #[props(default)]
    pub subtitle: Option<&'static str>,
    /// Icone du service.
    #[props(default)]
    pub icon: Option<&'static str>,
    /// Liste des roles/onglets (optionnel).
    #[props(default)]
    pub roles: Option<Vec<SidebarRole>>,
    /// Role actuellement selectionne.
    #[props(default)]
    pub current_role: Option<String>,
    /// Callback quand un role change.
    #[props(default)]
    pub on_role_change: Option<EventHandler<String>>,
    /// Liste des sections a afficher.
    pub sections: Vec<SidebarSection>,
    /// ID de la section active.
    pub active_section: String,
    /// Callback quand une section est selectionnee.
    pub on_section_change: EventHandler<String>,
    /// Largeur de la sidebar (defaut: 220px).
    #[props(default = 220)]
    pub width: u32,
    /// Type de footer.
    #[props(default = SidebarFooter::None)]
    pub footer: SidebarFooter,
    /// Callback pour le footer (si applicable).
    #[props(default)]
    pub on_footer_action: Option<EventHandler<()>>,
    /// Sections additionnelles apres un separateur.
    #[props(default)]
    pub secondary_sections: Option<Vec<SidebarSection>>,
}

/// Sidebar generique pour les services Miyukini.
#[component]
pub fn ServiceSidebar(props: ServiceSidebarProps) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let width = props.width;

    rsx! {
        aside {
            style: "width: {width}px; background: {c.bg_secondary}; border-right: 1px solid {c.border}; padding: 16px 0; display: flex; flex-direction: column;",

            // En-tete du service
            div {
                style: "padding: 0 16px 16px 16px; border-bottom: 1px solid {c.border};",

                // Titre avec icone optionnelle
                div {
                    style: "display: flex; align-items: center; gap: 8px; margin-bottom: 4px;",

                    if let Some(icon) = props.icon {
                        span { style: "font-size: 20px;", "{icon}" }
                    }

                    h3 {
                        style: "font-size: 16px; color: {c.text_white};",
                        "{props.title}"
                    }
                }

                // Sous-titre optionnel
                if let Some(subtitle) = props.subtitle {
                    p {
                        style: "font-size: 11px; color: {c.text_muted};",
                        "{subtitle}"
                    }
                }

                // Selecteur de roles
                if let Some(ref roles) = props.roles {
                    div {
                        style: "display: flex; gap: 4px; margin-top: 8px;",

                        for role in roles.iter() {
                            {
                                let role_id = role.id.clone();
                                let role_id_for_click = role_id.clone();
                                let is_active = props.current_role.as_ref() == Some(&role.id);

                                rsx! {
                                    RoleTab {
                                        key: "{role_id}",
                                        label: role.label,
                                        icon: role.icon,
                                        is_active: is_active,
                                        onclick: move |_| {
                                            if let Some(ref handler) = props.on_role_change {
                                                handler.call(role_id_for_click.clone());
                                            }
                                        },
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Navigation principale
            nav {
                style: "display: flex; flex-direction: column; gap: 4px; padding-top: 12px; flex: 1;",

                for section in props.sections.iter() {
                    {
                        let section_id = section.id.clone();
                        let section_id_for_click = section_id.clone();
                        let is_active = props.active_section == section.id;

                        rsx! {
                            SidebarItem {
                                key: "{section_id}",
                                icon: section.icon,
                                label: section.label,
                                badge: section.badge.clone(),
                                is_active: is_active,
                                onclick: move |_| {
                                    props.on_section_change.call(section_id_for_click.clone());
                                },
                            }
                        }
                    }
                }

                // Sections secondaires avec separateur
                if let Some(ref secondary) = props.secondary_sections {
                    div { style: "height: 1px; background: {c.border}; margin: 8px 16px;" }

                    for section in secondary.iter() {
                        {
                            let section_id = section.id.clone();
                            let section_id_for_click = section_id.clone();
                            let is_active = props.active_section == section.id;

                            rsx! {
                                SidebarItem {
                                    key: "sec-{section_id}",
                                    icon: section.icon,
                                    label: section.label,
                                    badge: section.badge.clone(),
                                    is_active: is_active,
                                    onclick: move |_| {
                                        props.on_section_change.call(section_id_for_click.clone());
                                    },
                                }
                            }
                        }
                    }
                }
            }

            // Footer
            match &props.footer {
                SidebarFooter::Logout => rsx! {
                    div {
                        style: "padding: 16px; border-top: 1px solid {c.border};",

                        button {
                            style: "display: flex; align-items: center; gap: 8px; width: 100%; padding: 10px 12px; background: transparent; border: 1px solid {c.border}; border-radius: 6px; color: {c.text_secondary}; cursor: pointer; font-size: 13px;",
                            onclick: move |_| {
                                if let Some(ref handler) = props.on_footer_action {
                                    handler.call(());
                                }
                            },

                            span { "🚪" }
                            span { "Deconnexion" }
                        }
                    }
                },
                SidebarFooter::Info(text) => rsx! {
                    div {
                        style: "padding: 16px; border-top: 1px solid {c.border};",

                        p {
                            style: "font-size: 10px; color: {c.text_muted}; text-align: center;",
                            "{text}"
                        }
                    }
                },
                SidebarFooter::Button { icon, label } => rsx! {
                    div {
                        style: "padding: 12px 16px; border-top: 1px solid {c.border};",

                        button {
                            style: "display: flex; align-items: center; gap: 8px; width: 100%; padding: 8px 12px; background: transparent; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_secondary}; cursor: pointer; font-size: 12px;",
                            onclick: move |_| {
                                if let Some(ref handler) = props.on_footer_action {
                                    handler.call(());
                                }
                            },

                            span { "{icon}" }
                            span { "{label}" }
                        }
                    }
                },
                SidebarFooter::None => rsx! {},
            }
        }
    }
}

/// Onglet de role compact.
#[component]
fn RoleTab(
    label: &'static str,
    #[props(default)] icon: Option<&'static str>,
    is_active: bool,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let bg = if is_active { c.accent_blue } else { c.bg_hover };
    let color = if is_active { "white" } else { c.text_secondary };

    rsx! {
        button {
            style: "flex: 1; padding: 6px 8px; background: {bg}; color: {color}; border: none; border-radius: 4px; cursor: pointer; font-size: 11px; font-weight: 500; display: flex; align-items: center; justify-content: center; gap: 4px;",
            onclick: move |evt| onclick.call(evt),

            if let Some(icon) = icon {
                span { "{icon}" }
            }
            span { "{label}" }
        }
    }
}

/// Item de navigation sidebar.
#[component]
fn SidebarItem(
    icon: &'static str,
    label: &'static str,
    #[props(default)] badge: Option<String>,
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
            span { style: "flex: 1;", "{label}" }

            if let Some(ref count) = badge {
                span {
                    style: "padding: 2px 6px; background: {c.bg_hover}; border-radius: 10px; font-size: 10px; color: {c.text_muted};",
                    "{count}"
                }
            }
        }
    }
}
