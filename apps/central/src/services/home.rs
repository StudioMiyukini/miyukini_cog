//! Vue d'accueil (onglet Home).

use dioxus::prelude::*;
use crate::state::{use_app_state, MainTab};
use crate::components::{ServiceGrid, ServiceFilter};

#[component]
pub fn HomeView() -> Element {
    let state = use_app_state();
    let theme = state.read().current_theme;
    let c = theme.palette();
    let main_tab = state.read().main_tab;

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 32px;",

            div {
                style: "background: linear-gradient(135deg, {c.accent_blue} 0%, #6366f1 50%, #ec4899 100%); border-radius: 8px; padding: 32px; position: relative; overflow: hidden;",

                div {
                    style: "position: absolute; top: -50%; right: -20%; width: 300px; height: 300px; background: radial-gradient(circle, rgba(255,255,255,0.1) 0%, transparent 70%); border-radius: 50%;",
                }

                div {
                    style: "position: relative; z-index: 1;",
                    h1 {
                        style: "font-size: 28px; font-weight: 600; color: white; margin-bottom: 8px;",
                        "Bienvenue dans Miyukini Central"
                    }
                    p {
                        style: "font-size: 14px; color: rgba(255,255,255,0.8); max-width: 500px;",
                        "Hub de gestion de votre environnement COG souverain. Interagissez avec vos Opérateurs gouvernés."
                    }
                }
            }

            // Contenu selon l'onglet principal
            match main_tab {
                MainTab::Magasin => rsx! {
                    // Services populaires
                    ServiceGrid {
                        title: "Services populaires",
                        show_filters: true,
                    }
                },
                MainTab::Bibliotheque => rsx! {
                    ServiceGrid {
                        title: "Ma bibliothèque",
                        filter: ServiceFilter::Installed,
                        show_filters: true,
                    }
                },
                MainTab::Communaute => rsx! {
                    CommunitySection {}
                },
                MainTab::Miyukini => rsx! {
                    SettingsSection {}
                },
            }
        }
    }
}

#[component]
fn CommunitySection() -> Element {
    let theme = use_app_state().read().current_theme;
    let c = theme.palette();
    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px;",
            h2 { style: "font-size: 20px; color: {c.text_white};", "Webway" }
            div {
                style: "display: grid; grid-template-columns: repeat(3, 1fr); gap: 16px;",
                CommunityCard { icon: "💬", title: "Forum", description: "Discussions et entraide" }
                CommunityCard { icon: "📖", title: "Guides", description: "Tutoriels et documentation" }
                CommunityCard { icon: "🎨", title: "Workshop", description: "Créations communautaires" }
            }
        }
    }
}

#[component]
fn CommunityCard(icon: &'static str, title: &'static str, description: &'static str) -> Element {
    let c = use_app_state().read().current_theme.palette();
    rsx! {
        div {
            style: "background: {c.bg_secondary}; border-radius: 8px; padding: 24px; text-align: center; cursor: pointer; transition: transform 0.2s;",
            span { style: "font-size: 32px; display: block; margin-bottom: 12px;", "{icon}" }
            h3 { style: "font-size: 16px; color: {c.text_white}; margin-bottom: 4px;", "{title}" }
            p { style: "font-size: 12px; color: {c.text_secondary};", "{description}" }
        }
    }
}

#[component]
fn SettingsSection() -> Element {
    let c = use_app_state().read().current_theme.palette();
    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px;",
            h2 { style: "font-size: 20px; color: {c.text_white};", "Paramètres Miyukini" }
            div {
                style: "display: flex; flex-direction: column; gap: 16px;",
                SettingsCard { icon: "⚙️", title: "Général", description: "Langue, thème, notifications" }
                SettingsCard { icon: "🔐", title: "Sécurité", description: "Authentification, permissions" }
                SettingsCard { icon: "🌐", title: "COG & Cores", description: "Configuration de l'environnement souverain" }
                SettingsCard { icon: "📊", title: "Stockage", description: "KindMother, données locales" }
            }
        }
    }
}

#[component]
fn SettingsCard(icon: &'static str, title: &'static str, description: &'static str) -> Element {
    let c = use_app_state().read().current_theme.palette();
    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 16px; background: {c.bg_secondary}; border-radius: 8px; padding: 16px; cursor: pointer; transition: background 0.2s;",
            span { style: "font-size: 24px;", "{icon}" }
            div {
                h3 { style: "font-size: 14px; color: {c.text_white}; margin-bottom: 2px;", "{title}" }
                p { style: "font-size: 12px; color: {c.text_secondary};", "{description}" }
            }
            span { style: "margin-left: auto; color: {c.text_secondary};", "›" }
        }
    }
}
