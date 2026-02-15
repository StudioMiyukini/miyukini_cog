//! Vue Jay1Tribu — Tribus, amis et discussions.
//! Chat et tribu ne fonctionnent pleinement que si le COG est connecté au Webway.

use dioxus::prelude::*;
use jay1tribu::is_webway_connected;

use crate::data::use_service_connections;
use crate::state::use_app_state;

/// Vue principale du service Jay1Tribu.
#[component]
pub fn Jay1TribuView() -> Element {
    let conns = use_service_connections();
    let state = use_app_state();
    let profile_id = state.read().current_user.as_ref().map(|u| u.id.clone()).unwrap_or_default();
    let cog_id = profile_id.clone();

    let db = conns.read().jay1tribu.clone();
    let mut friends = use_signal(|| Vec::<jay1tribu::Friend>::new());
    let mut tribes = use_signal(|| Vec::<jay1tribu::Tribe>::new());
    let webway_connected = use_signal(|| is_webway_connected());

    let profile_id_eff = profile_id.clone();
    let cog_id_eff = cog_id.clone();
    use_effect(move || {
        let db = db.clone();
        let friends_list = db.friend_list(&profile_id_eff).unwrap_or_default();
        let tribes_list = db.tribe_list(&cog_id_eff).unwrap_or_default();
        friends.set(friends_list);
        tribes.set(tribes_list);
    });

    use_future(move || {
        let mut webway_connected = webway_connected;
        async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                webway_connected.set(is_webway_connected());
            }
        }
    });

    let connected = *webway_connected.read();
    let friends_list = friends.read().clone();
    let tribes_list = tribes.read().clone();
    let status_style = if connected {
        "display: flex; align-items: center; gap: 8px; margin-bottom: 20px; padding: 12px; border-radius: 8px; background: #065f46;"
    } else {
        "display: flex; align-items: center; gap: 8px; margin-bottom: 20px; padding: 12px; border-radius: 8px; background: #444;"
    };
    let friends_title = format!("Amis ({})", friends_list.len());
    let tribes_title = format!("Tribus ({})", tribes_list.len());

    rsx! {
        div { style: "padding: 24px; height: 100%; overflow-y: auto; background: #1a1a2e; color: #e5e7eb;",
            h2 { style: "margin: 0 0 16px 0; font-size: 22px;", "💬 Jay1Tribu" }
            p { style: "margin: 0 0 20px 0; color: #9ca3af;", "Tribus, amis et discussions — tout reste chez toi." }
            div { style: status_style,
                if connected {
                    span { "🟢 Connecté au Webway — Chat et sync tribu actifs." }
                } else {
                    span { "⚫ Hors ligne — Connectez-vous au Webway (onglet Communauté) pour le chat en temps réel et la sync des tribus." }
                }
            }
            div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 24px;",
                div { style: "background: #252540; border-radius: 12px; padding: 16px;",
                    h3 { style: "margin: 0 0 12px 0; font-size: 16px;", "{friends_title}" }
                    if friends_list.is_empty() {
                        p { style: "color: #9ca3af; margin: 0;", "Aucun ami." }
                    } else {
                        ul { style: "list-style: none; padding: 0; margin: 0;",
                            for f in friends_list.iter() {
                                li { style: "padding: 8px 0; border-bottom: 1px solid #333;",
                                    "{f.friend_pseudo.as_deref().unwrap_or(f.friend_cog_id.as_str())}"
                                }
                            }
                        }
                    }
                }
                div { style: "background: #252540; border-radius: 12px; padding: 16px;",
                    h3 { style: "margin: 0 0 12px 0; font-size: 16px;", "{tribes_title}" }
                    if tribes_list.is_empty() {
                        p { style: "color: #9ca3af; margin: 0;", "Aucune tribu." }
                    } else {
                        ul { style: "list-style: none; padding: 0; margin: 0;",
                            for t in tribes_list.iter() {
                                li { style: "padding: 8px 0; border-bottom: 1px solid #333;",
                                    "{t.name}"
                                    if t.creator_cog_id == cog_id {
                                        span { style: "margin-left: 8px; color: #9ca3af; font-size: 12px;", "(Chef)" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
