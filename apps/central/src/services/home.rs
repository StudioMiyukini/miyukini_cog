//! Vue d'accueil (onglet Home).

use dioxus::prelude::*;
use crate::state::{use_app_state, MainTab, AppContext};
use crate::components::{ServiceGrid, ServiceFilter};
use crate::miou::state::FrequenceBulles;

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
                MainTab::Salon => rsx! {
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
                MainTab::MesAmis => rsx! {
                    // Contenu Mes Amis affiché via l'onglet principal (Jay1TribuView dans app.rs)
                    div {}
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
    let mut show_miou_settings = use_signal(|| false);
    
    let arrow_rotation = if show_miou_settings() { "90deg" } else { "0deg" };
    let miou_card_style = format!(
        "display: flex; align-items: center; gap: 16px; background: {}; border-radius: 8px; padding: 16px; cursor: pointer; transition: background 0.2s; border: 1px solid #ffd4e5;",
        c.bg_secondary
    );
    let arrow_style = format!(
        "margin-left: auto; color: {}; transform: rotate({}); transition: transform 0.2s;",
        c.text_secondary, arrow_rotation
    );
    
    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px;",
            h2 { style: "font-size: 20px; color: {c.text_white};", "Paramètres Miyukini" }
            div {
                style: "display: flex; flex-direction: column; gap: 16px;",
                
                // Carte Miou (cliquable)
                div {
                    onclick: move |_| show_miou_settings.set(!show_miou_settings()),
                    style: "{miou_card_style}",
                    span { style: "font-size: 24px;", "🌸" }
                    div {
                        h3 { style: "font-size: 14px; color: #ffd4e5; margin-bottom: 2px;", "Miou" }
                        p { style: "font-size: 12px; color: {c.text_secondary};", "Bulles, voix, TTS, comportement" }
                    }
                    span { 
                        style: "{arrow_style}", 
                        "›" 
                    }
                }
                
                // Panneau des paramètres Miou (dépliant)
                if show_miou_settings() {
                    MiouSettingsPanel {}
                }
                
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

/// Panneau des paramètres Miou.
#[component]
fn MiouSettingsPanel() -> Element {
    let mut ctx = use_context::<AppContext>();
    let c = use_app_state().read().current_theme.palette();
    let prefs = ctx.state.read().miou_prefs.clone();
    
    // État local pour les toggles (synchronisé avec les prefs)
    let mut bulles_actives = use_signal(|| prefs.bulles_actives);
    let mut frequence = use_signal(|| prefs.frequence);
    let mut seuil_pause = use_signal(|| prefs.seuil_pause_minutes);
    let mut rappels_pause = use_signal(|| prefs.rappels_pause_actives);
    let mut voix_enabled = use_signal(|| prefs.voix_enabled);
    let mut tts_enabled = use_signal(|| prefs.tts_enabled);
    
    // Pré-calculer tous les styles
    let panel_style = format!(
        "background: {}; border-radius: 8px; padding: 20px; margin-left: 40px; border-left: 2px solid #ffd4e5;",
        c.bg_secondary
    );
    let row_style = format!(
        "display: flex; align-items: center; justify-content: space-between; padding: 12px 0; border-bottom: 1px solid {};",
        c.border
    );
    let row_style_last = "display: flex; align-items: center; justify-content: space-between; padding: 12px 0;";
    let label_style = format!("font-size: 14px; color: {};", c.text_white);
    let desc_style = format!("font-size: 11px; color: {}; margin-top: 2px;", c.text_secondary);
    
    // Toggle styles
    let toggle_on = "width: 48px; height: 24px; background: #e91e63; border-radius: 12px; position: relative; cursor: pointer; transition: background 0.2s;";
    let toggle_off = "width: 48px; height: 24px; background: #4a5568; border-radius: 12px; position: relative; cursor: pointer; transition: background 0.2s;";
    let thumb_on = "width: 20px; height: 20px; background: white; border-radius: 50%; position: absolute; top: 2px; right: 2px; transition: all 0.2s;";
    let thumb_off = "width: 20px; height: 20px; background: white; border-radius: 50%; position: absolute; top: 2px; left: 2px; transition: all 0.2s;";
    
    // Button base style
    let btn_base = format!("padding: 6px 12px; border-radius: 4px; color: {}; font-size: 12px; cursor: pointer;", c.text_white);
    let btn_active = format!("{} border: 1px solid #e91e63; background: rgba(233,30,99,0.2);", btn_base);
    let btn_inactive = format!("{} border: 1px solid {}; background: transparent;", btn_base, c.border);
    
    // Valeurs actuelles pour les styles conditionnels
    let bulles_toggle = if bulles_actives() { toggle_on } else { toggle_off };
    let bulles_thumb = if bulles_actives() { thumb_on } else { thumb_off };
    let rappels_toggle = if rappels_pause() { toggle_on } else { toggle_off };
    let rappels_thumb = if rappels_pause() { thumb_on } else { thumb_off };
    let voix_toggle = if voix_enabled() { toggle_on } else { toggle_off };
    let voix_thumb = if voix_enabled() { thumb_on } else { thumb_off };
    let tts_toggle = if tts_enabled() { toggle_on } else { toggle_off };
    let tts_thumb = if tts_enabled() { thumb_on } else { thumb_off };
    
    // Fréquence buttons
    let btn_discret = if matches!(frequence(), FrequenceBulles::Discret) { &btn_active } else { &btn_inactive };
    let btn_normal = if matches!(frequence(), FrequenceBulles::Normal) { &btn_active } else { &btn_inactive };
    let btn_bavard = if matches!(frequence(), FrequenceBulles::Bavard) { &btn_active } else { &btn_inactive };
    
    // Seuil buttons
    let btn_1h = if seuil_pause() == 60 { &btn_active } else { &btn_inactive };
    let btn_2h = if seuil_pause() == 120 { &btn_active } else { &btn_inactive };
    let btn_3h = if seuil_pause() == 180 { &btn_active } else { &btn_inactive };
    
    rsx! {
        div {
            style: "{panel_style}",
            
            // Titre section
            div {
                style: "margin-bottom: 20px;",
                h3 { style: "font-size: 16px; color: #ffd4e5; margin-bottom: 4px;", "🌸 Paramètres Miou" }
                p { style: "{desc_style}", "Configure le comportement de Miou, ton compagnon COG." }
            }
            
            // Bulles activées
            div {
                style: "{row_style}",
                div {
                    span { style: "{label_style}", "Bulles Miou" }
                    p { style: "{desc_style}", "Afficher les messages de Miou" }
                }
                div {
                    style: "{bulles_toggle}",
                    onclick: move |_| {
                        let new_val = !bulles_actives();
                        bulles_actives.set(new_val);
                        ctx.state.write().miou_prefs.bulles_actives = new_val;
                    },
                    div { style: "{bulles_thumb}" }
                }
            }
            
            // Fréquence
            div {
                style: "{row_style}",
                div {
                    span { style: "{label_style}", "Fréquence" }
                    p { style: "{desc_style}", "Nombre de bulles par session" }
                }
                div {
                    style: "display: flex; gap: 8px;",
                    button {
                        style: "{btn_discret}",
                        onclick: move |_| {
                            frequence.set(FrequenceBulles::Discret);
                            ctx.state.write().miou_prefs.frequence = FrequenceBulles::Discret;
                        },
                        "Discret"
                    }
                    button {
                        style: "{btn_normal}",
                        onclick: move |_| {
                            frequence.set(FrequenceBulles::Normal);
                            ctx.state.write().miou_prefs.frequence = FrequenceBulles::Normal;
                        },
                        "Normal"
                    }
                    button {
                        style: "{btn_bavard}",
                        onclick: move |_| {
                            frequence.set(FrequenceBulles::Bavard);
                            ctx.state.write().miou_prefs.frequence = FrequenceBulles::Bavard;
                        },
                        "Bavard"
                    }
                }
            }
            
            // Rappels de pause
            div {
                style: "{row_style}",
                div {
                    span { style: "{label_style}", "Rappels de pause" }
                    p { style: "{desc_style}", "Miou te rappelle de faire des pauses" }
                }
                div {
                    style: "{rappels_toggle}",
                    onclick: move |_| {
                        let new_val = !rappels_pause();
                        rappels_pause.set(new_val);
                        ctx.state.write().miou_prefs.rappels_pause_actives = new_val;
                    },
                    div { style: "{rappels_thumb}" }
                }
            }
            
            // Seuil de pause (si rappels actifs)
            if rappels_pause() {
                div {
                    style: "{row_style}",
                    div {
                        span { style: "{label_style}", "Seuil de pause" }
                        p { style: "{desc_style}", "Temps avant rappel" }
                    }
                    div {
                        style: "display: flex; gap: 8px;",
                        button {
                            style: "{btn_1h}",
                            onclick: move |_| {
                                seuil_pause.set(60);
                                ctx.state.write().miou_prefs.seuil_pause_minutes = 60;
                            },
                            "1h"
                        }
                        button {
                            style: "{btn_2h}",
                            onclick: move |_| {
                                seuil_pause.set(120);
                                ctx.state.write().miou_prefs.seuil_pause_minutes = 120;
                            },
                            "2h"
                        }
                        button {
                            style: "{btn_3h}",
                            onclick: move |_| {
                                seuil_pause.set(180);
                                ctx.state.write().miou_prefs.seuil_pause_minutes = 180;
                            },
                            "3h"
                        }
                    }
                }
            }
            
            // Voix Miou
            div {
                style: "{row_style}",
                div {
                    span { style: "{label_style}", "Voix Miou" }
                    p { style: "{desc_style}", "Sons et voix de Miou (MP3)" }
                }
                div {
                    style: "{voix_toggle}",
                    onclick: move |_| {
                        let new_val = !voix_enabled();
                        voix_enabled.set(new_val);
                        ctx.state.write().miou_prefs.voix_enabled = new_val;
                    },
                    div { style: "{voix_thumb}" }
                }
            }
            
            // TTS eSpeak
            div {
                style: "{row_style_last}",
                div {
                    span { style: "{label_style}", "TTS eSpeak" }
                    p { style: "{desc_style}", "Synthèse vocale (textes dynamiques)" }
                }
                div {
                    style: "{tts_toggle}",
                    onclick: move |_| {
                        let new_val = !tts_enabled();
                        tts_enabled.set(new_val);
                        ctx.state.write().miou_prefs.tts_enabled = new_val;
                    },
                    div { style: "{tts_thumb}" }
                }
            }
            
            // Mode LLM (grisé en 0.1.x)
            div {
                style: "display: flex; align-items: center; justify-content: space-between; padding: 12px 0; opacity: 0.5;",
                div {
                    span { style: "{label_style}", "Mode LLM" }
                    p { style: "{desc_style}", "Intelligence avancée (bientôt disponible)" }
                }
                div {
                    style: "width: 48px; height: 24px; background: #4a5568; border-radius: 12px; position: relative; cursor: not-allowed;",
                    title: "Non disponible en version 0.1.x",
                    div { style: "width: 20px; height: 20px; background: white; border-radius: 50%; position: absolute; top: 2px; left: 2px;" }
                }
            }
        }
    }
}
