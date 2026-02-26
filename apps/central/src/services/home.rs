//! Vue d'accueil (onglet Home) — Salon Genshin-like avec chat Miou.

use dioxus::prelude::*;
use crate::state::{use_app_state, MainTab, AppContext, ServiceInfo, MiouChatMsg};
use crate::components::{ServiceGrid, ServiceFilter};
use crate::miou::state::FrequenceBulles;
use crate::llm_client::{LlmClient, ChatMessage};
use chrono::{Local, Timelike};

// ═══════════════════════════════════════════════════════════════════════════
// HomeView — dispatch selon l'onglet principal
// ═══════════════════════════════════════════════════════════════════════════

#[component]
pub fn HomeView() -> Element {
    let state = use_app_state();
    let main_tab = state.read().main_tab;

    match main_tab {
        MainTab::Salon => rsx! { SalonView {} },
        MainTab::Bibliotheque => rsx! {
            div {
                style: "display: flex; flex-direction: column; gap: 32px;",
                ServiceGrid {
                    title: "Ma bibliothèque",
                    filter: ServiceFilter::Installed,
                    show_filters: true,
                }
            }
        },
        MainTab::Communaute => rsx! { CommunitySection {} },
        MainTab::MesAmis => rsx! { div {} },
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// SalonView — layout 3 colonnes inspiré Genshin Impact
// ═══════════════════════════════════════════════════════════════════════════

#[component]
fn SalonView() -> Element {
    rsx! {
        div {
            style: "display: flex; flex: 1; min-height: 0; height: 100%; overflow: hidden; background: radial-gradient(ellipse at 50% 80%, #1a3a4a 0%, #0d1b2a 50%, #0a0f18 100%);",

            // ── Colonne gauche : services récents ──
            SalonServiceMenu {}

            // ── Centre : Miou ──
            SalonMiouCenter {}

            // ── Droite : chat Miou ──
            MiouChatPanel {}
        }
    }
}

// ── Colonne gauche — suggestions intelligentes ───────────────────────

/// Suggestion de service avec raison.
struct ServiceSuggestion {
    service: ServiceInfo,
    reason: String,
}

/// Construit les suggestions basées sur le profil et l'heure.
fn build_suggestions(services: &[ServiceInfo], hour: u32) -> Vec<ServiceSuggestion> {
    let mut suggestions = Vec::new();

    // Suggestion contextuelle par heure du jour
    let time_suggestion_id = match hour {
        6..=9 => Some(("jaykoa", "Consulte ton agenda du matin")),
        10..=12 => Some(("jayxpose", "Mets à jour ta vitrine")),
        13..=14 => Some(("jaymanga", "Pause lecture manga")),
        15..=18 => Some(("jaykonta", "Vérifie tes comptes")),
        19..=22 => Some(("miyuclicker", "Détends-toi avec un jeu")),
        23..=23 | 0..=5 => Some(("miyukiniwatch", "Bilan de ta journée")),
        _ => None,
    };

    if let Some((id, reason)) = time_suggestion_id {
        if let Some(svc) = services.iter().find(|s| s.id == id && s.is_installed) {
            suggestions.push(ServiceSuggestion {
                service: svc.clone(),
                reason: reason.to_string(),
            });
        }
    }

    // Favoris de l'utilisateur
    for svc in services.iter().filter(|s| s.is_installed && s.is_favorite && s.id != "market") {
        if suggestions.iter().any(|su| su.service.id == svc.id) { continue; }
        suggestions.push(ServiceSuggestion {
            service: svc.clone(),
            reason: "Favori".to_string(),
        });
    }

    // Compléter avec les autres services installés
    for svc in services.iter().filter(|s| s.is_installed && !s.is_favorite && s.id != "market") {
        if suggestions.iter().any(|su| su.service.id == svc.id) { continue; }
        suggestions.push(ServiceSuggestion {
            service: svc.clone(),
            reason: "Installé".to_string(),
        });
    }

    suggestions.truncate(8);
    suggestions
}

#[component]
fn SalonServiceMenu() -> Element {
    let mut state = use_app_state();
    let hour = Local::now().hour();

    let suggestions: Vec<ServiceSuggestion> = {
        let s = state.read();
        build_suggestions(&s.services, hour)
    };



    rsx! {
        div {
            style: "width: 260px; display: flex; flex-direction: column; padding: 32px 0 32px 20px; gap: 6px; z-index: 2;",

            // Titre
            div {
                style: "padding: 0 16px 12px 16px;",
                h3 { style: "font-size: 13px; color: rgba(255,255,255,0.4); text-transform: uppercase; letter-spacing: 1px; font-weight: 600;", "Suggestions" }
            }

            for (i, sg) in suggestions.iter().enumerate() {
                { let svc_clone = sg.service.clone();
                  let is_miou_pick = i == 0;
                  let border_color = if is_miou_pick { "rgba(255,212,229,0.5)" } else { "rgba(255,255,255,0.1)" };
                  let bg = if is_miou_pick { "rgba(255,212,229,0.08)" } else { "rgba(255,255,255,0.03)" };
                  let reason = sg.reason.clone();
                rsx! {
                    button {
                        key: "{sg.service.id}",
                        style: "display: flex; align-items: center; gap: 10px; padding: 10px 14px; background: {bg}; border: none; border-left: 3px solid {border_color}; color: #e2e8f0; cursor: pointer; font-size: 13px; text-align: left; transition: all 0.2s; border-radius: 0 6px 6px 0;",
                        onclick: move |_| {
                            state.write().open_service(&svc_clone);
                        },

                        span { style: "font-size: 16px;", "{sg.service.icon}" }
                        div {
                            style: "display: flex; flex-direction: column; gap: 1px;",
                            span { style: "font-weight: 500; font-size: 13px;", "{sg.service.name}" }
                            span { style: "font-size: 10px; color: rgba(255,255,255,0.35);",
                                if is_miou_pick {
                                    "\u{1F338} {reason}"
                                } else {
                                    "{reason}"
                                }
                            }
                        }
                    }
                }}
            }
        }
    }
}

// ── Centre : Miou ──────────────────────────────────────────────────────

#[component]
fn SalonMiouCenter() -> Element {
    let state = use_app_state();
    let pseudo = state.read().current_user.as_ref()
        .and_then(|u| u.pseudonyme.clone())
        .unwrap_or_else(|| "Voyageur".to_string());

    rsx! {
        div {
            style: "flex: 1; display: flex; flex-direction: column; align-items: center; justify-content: center; position: relative; min-height: 0;",

            // Particules décoratives (fond)
            div {
                style: "position: absolute; inset: 0; pointer-events: none; overflow: hidden;",
                // Cercle lumineux derrière Miou
                div { style: "position: absolute; top: 30%; left: 50%; transform: translate(-50%, -50%); width: 300px; height: 300px; background: radial-gradient(circle, rgba(255,212,229,0.12) 0%, transparent 70%); border-radius: 50%;" }
                // Petites étoiles
                div { style: "position: absolute; top: 20%; left: 30%; width: 4px; height: 4px; background: rgba(255,255,255,0.4); border-radius: 50%;" }
                div { style: "position: absolute; top: 40%; left: 70%; width: 3px; height: 3px; background: rgba(255,212,229,0.5); border-radius: 50%;" }
                div { style: "position: absolute; top: 60%; left: 25%; width: 3px; height: 3px; background: rgba(147,197,253,0.4); border-radius: 50%;" }
                div { style: "position: absolute; top: 15%; left: 65%; width: 5px; height: 5px; background: rgba(255,255,255,0.3); border-radius: 50%;" }
                div { style: "position: absolute; top: 75%; left: 55%; width: 3px; height: 3px; background: rgba(255,212,229,0.3); border-radius: 50%;" }
            }

            // Avatar Miou
            div {
                style: "font-size: 120px; filter: drop-shadow(0 0 30px rgba(255,212,229,0.3)); z-index: 1; margin-bottom: 16px;",
                "\u{1F338}"
            }

            // Nom + niveau
            div {
                style: "text-align: center; z-index: 1;",
                h2 { style: "font-size: 28px; color: #ffd4e5; font-weight: 600; margin-bottom: 4px; text-shadow: 0 0 20px rgba(255,212,229,0.3);", "Miou" }
                p { style: "font-size: 13px; color: rgba(255,255,255,0.5);", "Compagnon COG de {pseudo}" }
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// MiouChatPanel — panneau de chat à droite (LM Studio)
// État persisté dans AppState pour survivre aux changements d'onglets.
// ═══════════════════════════════════════════════════════════════════════════

/// État du chat (local au composant, seuls les messages sont persistés).
#[derive(Debug, Clone, PartialEq)]
enum ChatStatus {
    Init,
    Connecting,
    Ready(String),
    Error(String),
    Sending,
}

const MIOU_SYSTEM_PROMPT: &str = "Tu es Miou, un compagnon numérique bienveillant et espiègle de l'univers Miyukini COG. \
Tu parles français avec un ton doux, parfois taquin, toujours encourageant. \
Tu utilises des \u{1F338} de temps en temps. Tu connais les services du COG (JayXpose, JayFestival, JayKoa, JayKonta, JayManga, Jay1Tribu, MiyukiniWatch, Lord of the Click). \
Tu gardes tes réponses courtes (2-3 phrases max) sauf si on te demande plus de détail.";

/// Génère le message d'accueil contextuel de Miou.
fn miou_greeting(pseudo: &str, hour: u32, services: &[ServiceInfo]) -> String {
    let time_greet = match hour {
        6..=11 => format!("Bonjour {pseudo} !"),
        12..=17 => format!("Bon après-midi {pseudo} !"),
        18..=22 => format!("Bonsoir {pseudo} !"),
        _ => format!("Coucou {pseudo} ! Tu es encore debout ?"),
    };

    // Suggestion de service contextuelle
    let suggestion = match hour {
        6..=9 => services.iter().find(|s| s.id == "jaykoa" && s.is_installed)
            .map(|_| "Si tu veux, consulte JayKoa pour voir ton programme du jour."),
        10..=12 => services.iter().find(|s| s.id == "jayxpose" && s.is_installed)
            .map(|_| "C'est le bon moment pour mettre à jour JayXpose."),
        13..=14 => services.iter().find(|s| s.id == "jaymanga" && s.is_installed)
            .map(|_| "Petite pause manga sur JayManga ?"),
        15..=18 => services.iter().find(|s| s.id == "jaykonta" && s.is_installed)
            .map(|_| "JayKonta t'attend pour un point sur tes comptes."),
        19..=22 => services.iter().find(|s| s.id == "miyuclicker" && s.is_installed)
            .map(|_| "Un petit Lord of the Click pour se détendre ?"),
        _ => None,
    };

    match suggestion {
        Some(s) => format!("{time_greet} \u{1F338} {s}"),
        None => format!("{time_greet} \u{1F338} Que veux-tu faire aujourd'hui ?"),
    }
}

#[component]
fn MiouChatPanel() -> Element {
    let mut app = use_app_state();
    let mut input_text = use_signal(String::new);
    let status: Signal<ChatStatus> = use_signal(|| ChatStatus::Init);

    // Initialiser le message d'accueil si le chat est vide
    {
        let s = app.read();
        if s.miou_chat_messages.is_empty() {
            let pseudo = s.current_user.as_ref()
                .and_then(|u| u.pseudonyme.clone())
                .unwrap_or_else(|| "Voyageur".to_string());
            let hour = Local::now().hour();
            let greeting = miou_greeting(&pseudo, hour, &s.services);
            drop(s);
            app.write().miou_chat_messages.push(MiouChatMsg {
                role: "assistant".into(),
                content: greeting,
            });
        }
    }

    // Init : vérifier le modèle LLM au montage (une seule fois)
    let status_for_init = status;
    use_effect(move || {
        let mut status = status_for_init;
        // Si on a déjà un modèle dans l'AppState, le restaurer
        let existing = app.read().miou_chat_model.clone();
        if let Some(m) = existing {
            status.set(ChatStatus::Ready(m));
        } else {
            spawn(async move {
                status.set(ChatStatus::Connecting);
                let client = LlmClient::new();
                match client.ensure_model().await {
                    Ok(model) => {
                        app.write().miou_chat_model = Some(model.clone());
                        status.set(ChatStatus::Ready(model));
                    }
                    Err(e) => status.set(ChatStatus::Error(e.to_string())),
                }
            });
        }
    });

    let is_sending = matches!(*status.read(), ChatStatus::Sending);
    let status_text = match &*status.read() {
        ChatStatus::Init => "Initialisation...".to_string(),
        ChatStatus::Connecting => "Connexion au LLM...".to_string(),
        ChatStatus::Ready(m) => format!("Modèle : {m}"),
        ChatStatus::Error(e) => {
            let short = if e.len() > 30 { &e[..30] } else { e.as_str() };
            format!("Erreur : {short}")
        }
        ChatStatus::Sending => "Miou réfléchit...".to_string(),
    };
    let status_color = match &*status.read() {
        ChatStatus::Ready(_) => "#10b981",
        ChatStatus::Error(_) => "#ef4444",
        ChatStatus::Sending => "#f59e0b",
        _ => "#6b7280",
    };

    // Lire les messages depuis AppState
    let messages = app.read().miou_chat_messages.clone();

    rsx! {
        div {
            style: "width: 380px; display: flex; flex-direction: column; background: rgba(13,27,42,0.85); border-left: 1px solid rgba(255,255,255,0.08); z-index: 2;",

            // Header chat
            div {
                style: "padding: 16px 20px; border-bottom: 1px solid rgba(255,255,255,0.08); display: flex; align-items: center; justify-content: space-between;",
                div {
                    style: "display: flex; align-items: center; gap: 8px;",
                    span { style: "font-size: 18px;", "\u{1F338}" }
                    span { style: "font-size: 15px; color: #ffd4e5; font-weight: 600;", "Chat Miou" }
                }
                div {
                    style: "display: flex; align-items: center; gap: 6px;",
                    span { style: "width: 6px; height: 6px; border-radius: 50%; background: {status_color};" }
                    span { style: "font-size: 10px; color: #8f98a0; max-width: 160px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;", "{status_text}" }
                }
            }

            // Messages
            div {
                style: "flex: 1; overflow-y: auto; padding: 16px; display: flex; flex-direction: column; gap: 12px;",

                for (i, msg) in messages.iter().enumerate() {
                    {
                        let is_user = msg.role == "user";
                        let align = if is_user { "flex-end" } else { "flex-start" };
                        let bg = if is_user { "rgba(99,102,241,0.25)" } else { "rgba(255,212,229,0.1)" };
                        let border_col = if is_user { "rgba(99,102,241,0.4)" } else { "rgba(255,212,229,0.2)" };
                        let text_col = if is_user { "#e2e8f0" } else { "#ffd4e5" };
                        let content = msg.content.clone();
                        rsx! {
                            div {
                                key: "{i}",
                                style: "align-self: {align}; max-width: 90%;",
                                div {
                                    style: "padding: 10px 14px; border-radius: 12px; background: {bg}; border: 1px solid {border_col}; font-size: 13px; color: {text_col}; line-height: 1.5;",
                                    "{content}"
                                }
                            }
                        }
                    }
                }

                if is_sending {
                    div {
                        style: "align-self: flex-start; max-width: 90%;",
                        div {
                            style: "padding: 10px 14px; border-radius: 12px; background: rgba(255,212,229,0.1); border: 1px solid rgba(255,212,229,0.2); font-size: 13px; color: #ffd4e5; line-height: 1.5;",
                            "\u{1F338} ..."
                        }
                    }
                }
            }

            // Input
            div {
                style: "padding: 12px 16px; border-top: 1px solid rgba(255,255,255,0.08); display: flex; gap: 8px;",

                input {
                    style: "flex: 1; padding: 10px 14px; background: rgba(255,255,255,0.05); border: 1px solid rgba(255,255,255,0.12); border-radius: 8px; color: #e2e8f0; font-size: 13px; outline: none;",
                    r#type: "text",
                    placeholder: "Parle à Miou...",
                    value: "{input_text}",
                    disabled: is_sending,
                    oninput: move |evt| { input_text.set(evt.value()); },
                    onkeypress: move |evt: KeyboardEvent| {
                        if evt.key() == Key::Enter && !is_sending {
                            let text = input_text.read().trim().to_string();
                            if text.is_empty() { return; }
                            send_chat_message(text, app, input_text, status);
                        }
                    },
                }

                button {
                    style: "padding: 10px 16px; background: linear-gradient(135deg, #e91e63, #ec4899); border: none; border-radius: 8px; color: white; cursor: pointer; font-size: 13px; font-weight: 500;",
                    disabled: is_sending,
                    onclick: move |_| {
                        let text = input_text.read().trim().to_string();
                        if text.is_empty() { return; }
                        send_chat_message(text, app, input_text, status);
                    },
                    "Envoyer"
                }
            }
        }
    }
}

/// Envoie un message, appelle le LLM, ajoute la réponse.
/// Les messages sont persistés dans AppState.
fn send_chat_message(
    text: String,
    mut app: Signal<crate::state::AppState>,
    mut input_text: Signal<String>,
    mut status: Signal<ChatStatus>,
) {
    let model = match &*status.read() {
        ChatStatus::Ready(m) => m.clone(),
        _ => return,
    };

    // Ajouter le message user dans AppState
    app.write().miou_chat_messages.push(MiouChatMsg {
        role: "user".into(),
        content: text.clone(),
    });
    input_text.set(String::new());
    status.set(ChatStatus::Sending);

    // Construire l'historique pour l'API
    let mut api_messages = vec![
        ChatMessage { role: "system".into(), content: MIOU_SYSTEM_PROMPT.into() },
    ];
    for msg in app.read().miou_chat_messages.iter() {
        api_messages.push(ChatMessage {
            role: msg.role.clone(),
            content: msg.content.clone(),
        });
    }

    spawn(async move {
        let client = LlmClient::new();
        match client.chat(&model, &api_messages).await {
            Ok(reply) => {
                app.write().miou_chat_messages.push(MiouChatMsg {
                    role: "assistant".into(),
                    content: reply,
                });
                status.set(ChatStatus::Ready(model));
            }
            Err(e) => {
                app.write().miou_chat_messages.push(MiouChatMsg {
                    role: "assistant".into(),
                    content: format!("Oups, je n'arrive pas à répondre... ({e})"),
                });
                status.set(ChatStatus::Error(e.to_string()));
            }
        }
    });
}

// ═══════════════════════════════════════════════════════════════════════════
// Sections utilitaires (inchangées)
// ═══════════════════════════════════════════════════════════════════════════

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
                CommunityCard { icon: "\u{1F4AC}", title: "Forum", description: "Discussions et entraide" }
                CommunityCard { icon: "\u{1F4D6}", title: "Guides", description: "Tutoriels et documentation" }
                CommunityCard { icon: "\u{1F3A8}", title: "Workshop", description: "Créations communautaires" }
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

                div {
                    onclick: move |_| show_miou_settings.set(!show_miou_settings()),
                    style: "{miou_card_style}",
                    span { style: "font-size: 24px;", "\u{1F338}" }
                    div {
                        h3 { style: "font-size: 14px; color: #ffd4e5; margin-bottom: 2px;", "Miou" }
                        p { style: "font-size: 12px; color: {c.text_secondary};", "Bulles, voix, TTS, comportement" }
                    }
                    span {
                        style: "{arrow_style}",
                        "\u{203A}"
                    }
                }

                if show_miou_settings() {
                    MiouSettingsPanel {}
                }

                SettingsCard { icon: "\u{2699}\u{FE0F}", title: "Général", description: "Langue, thème, notifications" }
                SettingsCard { icon: "\u{1F510}", title: "Sécurité", description: "Authentification, permissions" }
                SettingsCard { icon: "\u{1F310}", title: "COG & Cores", description: "Configuration de l'environnement souverain" }
                SettingsCard { icon: "\u{1F4CA}", title: "Stockage", description: "KindMother, données locales" }
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
            span { style: "margin-left: auto; color: {c.text_secondary};", "\u{203A}" }
        }
    }
}

/// Panneau des paramètres Miou.
#[component]
fn MiouSettingsPanel() -> Element {
    let mut ctx = use_context::<AppContext>();
    let c = use_app_state().read().current_theme.palette();
    let prefs = ctx.state.read().miou_prefs.clone();

    let mut bulles_actives = use_signal(|| prefs.bulles_actives);
    let mut frequence = use_signal(|| prefs.frequence);
    let mut seuil_pause = use_signal(|| prefs.seuil_pause_minutes);
    let mut rappels_pause = use_signal(|| prefs.rappels_pause_actives);
    let mut voix_enabled = use_signal(|| prefs.voix_enabled);
    let mut tts_enabled = use_signal(|| prefs.tts_enabled);

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

    let toggle_on = "width: 48px; height: 24px; background: #e91e63; border-radius: 12px; position: relative; cursor: pointer; transition: background 0.2s;";
    let toggle_off = "width: 48px; height: 24px; background: #4a5568; border-radius: 12px; position: relative; cursor: pointer; transition: background 0.2s;";
    let thumb_on = "width: 20px; height: 20px; background: white; border-radius: 50%; position: absolute; top: 2px; right: 2px; transition: all 0.2s;";
    let thumb_off = "width: 20px; height: 20px; background: white; border-radius: 50%; position: absolute; top: 2px; left: 2px; transition: all 0.2s;";

    let btn_base = format!("padding: 6px 12px; border-radius: 4px; color: {}; font-size: 12px; cursor: pointer;", c.text_white);
    let btn_active = format!("{} border: 1px solid #e91e63; background: rgba(233,30,99,0.2);", btn_base);
    let btn_inactive = format!("{} border: 1px solid {}; background: transparent;", btn_base, c.border);

    let bulles_toggle = if bulles_actives() { toggle_on } else { toggle_off };
    let bulles_thumb = if bulles_actives() { thumb_on } else { thumb_off };
    let rappels_toggle = if rappels_pause() { toggle_on } else { toggle_off };
    let rappels_thumb = if rappels_pause() { thumb_on } else { thumb_off };
    let voix_toggle = if voix_enabled() { toggle_on } else { toggle_off };
    let voix_thumb = if voix_enabled() { thumb_on } else { thumb_off };
    let tts_toggle = if tts_enabled() { toggle_on } else { toggle_off };
    let tts_thumb = if tts_enabled() { thumb_on } else { thumb_off };

    let btn_discret = if matches!(frequence(), FrequenceBulles::Discret) { &btn_active } else { &btn_inactive };
    let btn_normal = if matches!(frequence(), FrequenceBulles::Normal) { &btn_active } else { &btn_inactive };
    let btn_bavard = if matches!(frequence(), FrequenceBulles::Bavard) { &btn_active } else { &btn_inactive };

    let btn_1h = if seuil_pause() == 60 { &btn_active } else { &btn_inactive };
    let btn_2h = if seuil_pause() == 120 { &btn_active } else { &btn_inactive };
    let btn_3h = if seuil_pause() == 180 { &btn_active } else { &btn_inactive };

    rsx! {
        div {
            style: "{panel_style}",

            div {
                style: "margin-bottom: 20px;",
                h3 { style: "font-size: 16px; color: #ffd4e5; margin-bottom: 4px;", "\u{1F338} Paramètres Miou" }
                p { style: "{desc_style}", "Configure le comportement de Miou, ton compagnon COG." }
            }

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
