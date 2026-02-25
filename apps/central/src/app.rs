//! Application principale Miyukini Central.

use std::sync::Arc;

use dioxus::prelude::*;
use crate::data::ServiceConnections;
use crate::state::{AppContext, AppState, MainTab};
use crate::components::{Header, TabBar};
use crate::services::{ActiveServiceView, MwsNetworkView, MwsViewState};
#[cfg(feature = "service-jay1tribu")]
use crate::services::Jay1TribuView;
use crate::theme::styles;
use crate::screens::{RiteEntree, Connexion, ProfileWindow};
use crate::miou::{
    MiouBubbleOverlay, BulleOutput, BulleAction, ActionType,
    BotContext, decide, select_variante, templates::generate_bulle,
};
use crate::miou::bubble::MIOU_CSS;
#[cfg(feature = "service-jay1tribu")]
use jay1tribu::get_online_friends;

/// Point d'entrée de l'application.
#[component]
pub fn App() -> Element {
    // Un seul provider : connexions + état dérivé (évite hook-in-hook).
    use_context_provider(|| {
        let base_path = std::env::current_dir().unwrap_or_default();
        let connections = ServiceConnections::open(&base_path)
            .expect("Impossible d'ouvrir les bases de donnees service");
        let connections = Arc::new(connections);
        let auth_db = &*connections.auth_db;
        let is_cog_virgin = auth_db.is_cog_virgin().unwrap_or(true);
        // Charge le dernier profil connu uniquement pour pré-remplir l'écran de connexion (pas d'auto-login).
        let last_profile = auth_db
            .get_current_profile_id()
            .ok()
            .flatten()
            .and_then(|id| auth_db.get_profile(&id).ok().flatten());
        let (last_login_email, last_login_pseudo) = last_profile
            .as_ref()
            .map(|p| (p.email.clone(), p.pseudonyme.clone().unwrap_or_default()))
            .unwrap_or_default();
        let state = AppState {
            is_cog_virgin,
            last_login_email,
            last_login_pseudo,
            ..AppState::default()
        };
        // current_user reste None au démarrage : l'écran de connexion s'affiche toujours.
        AppContext {
            connections: Signal::new(connections),
            state: Signal::new(state),
        }
    });

    // État MWS partagé : persiste au changement d'onglet (connexion maintenue sauf déconnexion manuelle ou fermeture).
    use_context_provider(|| Signal::new(MwsViewState::default()));

    let ctx = use_context::<AppContext>();
    let state = ctx.state;
    let is_cog_virgin = state.read().is_cog_virgin;
    let has_user = state.read().current_user.is_some();
    let theme = state.read().current_theme;
    let c = theme.palette();

    // Signal pour la bulle Miou actuelle
    let mut current_bubble: Signal<Option<BulleOutput>> = use_signal(|| None);

    // Trigger de la première bulle Miou après connexion (délai 2-3s)
    let ctx_for_effect = ctx.clone();
    let mws_state = use_context::<Signal<MwsViewState>>();
    use_effect(move || {
        let mut ctx = ctx_for_effect.clone();
        let state_read = ctx.state.read();
        
        // Déclencher seulement si : utilisateur connecté, pas virgin, trigger pas déjà fait
        if state_read.current_user.is_some() 
            && !state_read.is_cog_virgin 
            && !state_read.rite_infos_pending
            && !state_read.miou_first_trigger_done 
        {
            let pseudo = state_read.current_user.as_ref()
                .and_then(|u| u.pseudonyme.clone())
                .unwrap_or_else(|| "habitant".to_string());
            let prefs = state_read.miou_prefs.clone();
            let miou_state = state_read.miou_state.clone();
            let profile_id = state_read.current_user.as_ref().map(|u| u.id.clone()).unwrap_or_default();

            // Données Jay1Tribu pour Miou (contrat Integration Central et Miou)
            #[cfg(feature = "service-jay1tribu")]
            let ami_connecte_recemment = {
                let connections = ctx.connections.read();
                let jay1tribu_db = connections.jay1tribu.clone();
                let online_cog_ids: Vec<String> = mws_state
                    .read()
                    .discovered_cogs
                    .iter()
                    .map(|c| c.cog_id.clone())
                    .collect();
                drop(connections);
                // Dégradation gracieuse : si Jay1Tribu erre, listes vides (INT-05)
                get_online_friends(&jay1tribu_db, &profile_id, &online_cog_ids)
                    .ok()
                    .and_then(|friends| friends.first().cloned())
                    .map(|f| f.friend_pseudo.unwrap_or(f.friend_cog_id))
            };
            #[cfg(not(feature = "service-jay1tribu"))]
            let ami_connecte_recemment: Option<String> = None;
            drop(state_read);
            let ami_plus_delaisse: Option<(String, u32)> = None; // TODO: implémenter via MiyukiniWatch ou last_message_to_friend

            spawn(async move {
                // Délai 2-3 secondes
                tokio::time::sleep(std::time::Duration::from_millis(2500)).await;
                
                // Construire le contexte (get_online_friends / get_friends_list pour Miou)
                let context = BotContext {
                    pseudo,
                    is_first_connection_of_session: true,
                    session_duration_minutes: 0,
                    bulles_actives: prefs.bulles_actives,
                    dnd_actif: prefs.dnd_actif,
                    max_bulles_par_session: prefs.frequence.max_bulles(),
                    bulles_deja_affichees: miou_state.bulles_count_this_session,
                    seuil_pause_minutes: prefs.seuil_pause_minutes,
                    rappels_pause_actives: prefs.rappels_pause_actives,
                    ami_connecte_recemment,
                    ami_plus_delaisse,
                    ..BotContext::default()
                };
                
                // Décision
                let decision = decide(&context, &miou_state);
                
                if let Some(categorie) = decision.categorie {
                    // Sélectionner une variante
                    if let Some(template) = select_variante(categorie, &miou_state.variantes_used) {
                        let bulle = generate_bulle(&template, &context);
                        current_bubble.set(Some(bulle.clone()));
                        
                        // Marquer le trigger comme fait et enregistrer
                        ctx.state.write().miou_first_trigger_done = true;
                        ctx.state.write().miou_state.record_bulle_shown(categorie, &template.id);
                    }
                }
            });
        }
    });

    // Handler pour dismiss de la bulle
    let mut ctx_for_dismiss = ctx.clone();
    let on_dismiss = move |_| {
        // Vérifier si la bulle est une pause et enregistrer le dismiss
        {
            let bubble = current_bubble.read();
            if let Some(b) = bubble.as_ref() {
                if b.categorie.contains("pause") {
                    ctx_for_dismiss.state.write().miou_state.record_pause_dismissed();
                }
            }
        }
        current_bubble.set(None);
    };

    // Handler pour actions de la bulle
    let mut ctx_for_action = ctx.clone();
    let on_action = move |action: BulleAction| {
        match action.action_type {
            ActionType::Dismiss => {
                current_bubble.set(None);
            }
            ActionType::Pause => {
                ctx_for_action.state.write().miou_state.record_pause_dismissed();
                current_bubble.set(None);
                // TODO: Déclencher une action de pause (ex: notification système)
            }
            ActionType::OuvrirService => {
                if let Some(service_id) = action.payload {
                    // Ouvrir le service
                    let services = ctx_for_action.state.read().services.clone();
                    if let Some(service) = services.iter().find(|s| s.id == service_id) {
                        ctx_for_action.state.write().open_service(service);
                    }
                }
                current_bubble.set(None);
            }
            ActionType::Custom => {
                current_bubble.set(None);
            }
        }
    };

    rsx! {
        div {
            style: "{styles::main_container(theme)}",

            // CSS global pour les scrollbars et fonts
            style { {GLOBAL_CSS} }
            // CSS pour les animations Miou
            style { {MIOU_CSS} }

            if is_cog_virgin || state.read().rite_infos_pending {
                RiteEntree {}
            } else if !has_user {
                Connexion {}
            } else {
                Header {}
                main {
                    style: "{styles::content_area(theme)}",
                    role: "main",

                    // Afficher TabBar uniquement pour Salon et Bibliothèque
                    if matches!(state.read().main_tab, MainTab::Salon | MainTab::Bibliotheque) {
                        TabBar {}
                    }

                    div {
                        style: "{styles::content_panel(theme)}",

                        // Wrapper flex : overflow-y auto pour vues simples (Home), min-height: 0 pour JayXpose
                        div {
                            style: "flex: 1; min-height: 0; display: flex; flex-direction: column; overflow-y: auto;",

                            // Contenu selon l'onglet principal
                            {
                                let main_tab = state.read().main_tab;
                                match main_tab {
                                    MainTab::Salon | MainTab::Bibliotheque => rsx! { ActiveServiceView {} },
                                    MainTab::Communaute => rsx! { MwsNetworkView {} },
                                    #[cfg(feature = "service-jay1tribu")]
                                    MainTab::MesAmis => rsx! { Jay1TribuView {} },
                                    #[cfg(not(feature = "service-jay1tribu"))]
                                    MainTab::MesAmis => rsx! {
                                        div { style: "padding: 32px; color: #8f98a0;",
                                            "Service Jay1Tribu non disponible."
                                        }
                                    },
                                }
                            }
                        }
                    }
                }
                footer {
                    style: "display: flex; align-items: center; justify-content: space-between; height: 24px; background: {c.bg_header}; padding: 0 16px; font-size: 11px; color: {c.text_muted}; border-top: 1px solid {c.border};",
                    span { "Miyukini Central v0.1.0" }
                    span { "COG: Actif • KindMother: 4 DB connectees" }
                }
                if state.read().show_profile_window {
                    ProfileWindow {}
                }
                
                // Overlay Miou (bulle)
                MiouBubbleOverlay {
                    bubble: current_bubble,
                    on_dismiss: on_dismiss,
                    on_action: on_action,
                }
            }
        }
    }
}

/// CSS global injecté dans la page.
#[allow(clippy::needless_raw_string_hashes)]
const GLOBAL_CSS: &str = r#"
* {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
}

body {
    font-family: 'Segoe UI', -apple-system, BlinkMacSystemFont, 'Roboto', sans-serif;
    -webkit-font-smoothing: antialiased;
    overflow: hidden;
}

/* Scrollbars style Steam */
::-webkit-scrollbar {
    width: 8px;
    height: 8px;
}

::-webkit-scrollbar-track {
    background: #1b2838;
}

::-webkit-scrollbar-thumb {
    background: #3d4f5f;
    border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
    background: #4d6070;
}

/* Focus outline */
*:focus {
    outline: none;
}

*:focus-visible {
    outline: 2px solid #1a9fff;
    outline-offset: 2px;
}

/* Button reset */
button {
    font-family: inherit;
}

/* Smooth transitions */
button, a, div {
    transition: background-color 0.15s ease, color 0.15s ease, border-color 0.15s ease, transform 0.15s ease;
}

/* Hover effects */
button:hover:not(:disabled) {
    filter: brightness(1.1);
}

/* Active state */
button:active:not(:disabled) {
    transform: scale(0.98);
}

/* Selection color */
::selection {
    background: #1a9fff;
    color: white;
}
"#;
