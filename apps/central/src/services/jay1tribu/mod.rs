//! Vue Jay1Tribu — Tribus, amis et discussions.
//! Layout 3 panneaux type Steam/Discord : amis | salons | chat.

use dioxus::prelude::*;
use jay1tribu::{
    get_friends_with_presence, is_webway_connected, process_pending_deliveries,
};

use crate::data::use_service_connections;
use crate::services::MwsViewState;
use crate::state::use_app_state;

/// Style de base pour un panneau (évite expressions complexes dans rsx!).
const PANEL_BASE: &str = "background: #252540; border-radius: 8px; padding: 12px; overflow-y: auto;";

/// Vue principale du service Jay1Tribu (3 panneaux : amis | salons | chat).
#[component]
pub fn Jay1TribuView() -> Element {
    let conns = use_service_connections();
    let state = use_app_state();
    let mws_state = use_context::<Signal<MwsViewState>>();
    let profile_id = state.read().current_user.as_ref().map(|u| u.id.clone()).unwrap_or_default();
    let cog_id = profile_id.clone();

    let db = conns.read().jay1tribu.clone();
    let mut friends = use_signal(|| Vec::<jay1tribu::FriendWithPresence>::new());
    let mut tribes = use_signal(|| Vec::<jay1tribu::Tribe>::new());
    let mut salons = use_signal(|| Vec::<jay1tribu::Salon>::new());
    let mut selected_salon_id = use_signal(|| Option::<String>::None);
    let mut messages = use_signal(|| Vec::<jay1tribu::Message>::new());
    let webway_connected = use_signal(|| is_webway_connected());

    let profile_id_eff = profile_id.clone();
    let cog_id_eff = cog_id.clone();
    use_effect(move || {
        let online_ids: Vec<String> =
            mws_state.read().discovered_cogs.iter().map(|c| c.cog_id.clone()).collect();
        let _ = process_pending_deliveries(&db, &cog_id_eff, &online_ids);
        let friends_list =
            get_friends_with_presence(&db, &profile_id_eff, &online_ids).unwrap_or_default();
        friends.set(friends_list);
        tribes.set(db.tribe_list(&cog_id_eff).unwrap_or_default());
        salons.set(db.salon_list_for_cog(&cog_id_eff).unwrap_or_default());
    });

    let sid = *selected_salon_id.read();
    use_effect(move || {
        if let Some(ref salon_id) = sid {
            let list = db.message_list(salon_id, 50).unwrap_or_default();
            messages.set(list);
        } else {
            messages.set(Vec::new());
        }
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
    let salons_list = salons.read().clone();
    let messages_list = messages.read().clone();

    let status_style = if connected {
        "display: flex; align-items: center; gap: 8px; margin-bottom: 12px; padding: 8px; border-radius: 8px; background: #065f46; font-size: 13px;"
    } else {
        "display: flex; align-items: center; gap: 8px; margin-bottom: 12px; padding: 8px; border-radius: 8px; background: #444; font-size: 13px;"
    };

    let layout_style = "display: grid; grid-template-columns: 200px 220px 1fr; gap: 12px; height: 100%; min-height: 0; padding: 16px; background: #1a1a2e; color: #e5e7eb;";

    let left_style = format!("{} width: 200px; min-width: 0;", PANEL_BASE);
    let center_style = format!("{} width: 220px; min-width: 0;", PANEL_BASE);
    let right_style = format!("{} flex: 1; min-width: 0;", PANEL_BASE);

    let friends_title = format!("Amis ({})", friends_list.len());
    let tribes_title = format!("Tribus ({})", tribes_list.len());
    let salons_title = format!("Conversations ({})", salons_list.len());

    struct FriendRow {
        name: String,
        dot_color: String,
    }
    let friend_rows: Vec<FriendRow> = friends_list
        .iter()
        .map(|fwp| FriendRow {
            name: fwp
                .friend
                .friend_pseudo
                .clone()
                .unwrap_or_else(|| fwp.friend.friend_cog_id.clone()),
            dot_color: if fwp.online {
                "#22c55e".to_string()
            } else {
                "#6b7280".to_string()
            },
        })
        .collect();

    struct TribeRow {
        name: String,
        is_chef: bool,
    }
    let tribe_rows: Vec<TribeRow> = tribes_list
        .iter()
        .map(|t| TribeRow {
            name: t.name.clone(),
            is_chef: t.creator_cog_id == cog_id,
        })
        .collect();

    struct SalonRow {
        id: String,
        name: String,
        is_selected: bool,
    }
    let selected_id = selected_salon_id.read().clone();
    let salon_rows: Vec<SalonRow> = salons_list
        .iter()
        .map(|s| SalonRow {
            id: s.id.clone(),
            name: s.name.clone(),
            is_selected: selected_id.as_ref().map_or(false, |id| id == &s.id),
        })
        .collect();

    struct MessageRow {
        sender: String,
        content: String,
    }
    let message_rows: Vec<MessageRow> = messages_list
        .iter()
        .map(|m| MessageRow {
            sender: m.sender_cog_id.clone(),
            content: m.content.clone(),
        })
        .collect();

    let no_salon_msg = "Sélectionnez une conversation.";
    let chat_title = selected_id
        .as_ref()
        .and_then(|id| salons_list.iter().find(|s| s.id == *id).map(|s| s.name.clone()))
        .unwrap_or_else(|| "Chat".to_string());

    rsx! {
        div { style: "height: 100%; overflow: hidden;",
            div { style: "margin-bottom: 8px; padding: 0 16px;",
                h2 { style: "margin: 0; font-size: 20px;", "💬 Jay1Tribu" }
                p { style: "margin: 4px 0 0; color: #9ca3af; font-size: 13px;", "Tribus, amis et discussions — tout reste chez toi." }
                div { style: status_style,
                    if connected {
                        span { "🟢 Connecté au Webway" }
                    } else {
                        span { "⚫ Hors ligne" }
                    }
                }
            }
            div { style: layout_style,
                div { style: left_style,
                    h3 { style: "margin: 0 0 8px 0; font-size: 14px;", "{friends_title}" }
                    ul { style: "list-style: none; padding: 0; margin: 0;",
                        for row in friend_rows.iter() {
                            li { style: "padding: 6px 0; border-bottom: 1px solid #333; display: flex; align-items: center; gap: 6px;",
                                span { style: "width: 6px; height: 6px; border-radius: 50%; flex-shrink: 0; background: {row.dot_color};" }
                                "{row.name}"
                            }
                        }
                    }
                    h3 { style: "margin: 12px 0 8px 0; font-size: 14px;", "{tribes_title}" }
                    ul { style: "list-style: none; padding: 0; margin: 0;",
                        for row in tribe_rows.iter() {
                            li { style: "padding: 6px 0; border-bottom: 1px solid #333;",
                                "{row.name}"
                                if row.is_chef {
                                    span { style: "margin-left: 6px; color: #9ca3af; font-size: 11px;", "(Chef)" }
                                }
                            }
                        }
                    }
                }
                div { style: center_style,
                    h3 { style: "margin: 0 0 8px 0; font-size: 14px;", "{salons_title}" }
                    ul { style: "list-style: none; padding: 0; margin: 0;",
                        for row in salon_rows.iter() {
                            let salon_id = row.id.clone();
                            let li_style = if row.is_selected {
                                "padding: 8px; margin: 2px 0; border-radius: 6px; background: #3d3d5c; cursor: pointer;"
                            } else {
                                "padding: 8px; margin: 2px 0; border-radius: 6px; cursor: pointer;"
                            };
                            li {
                                style: li_style,
                                onclick: move |_| {
                                    selected_salon_id.set(Some(salon_id.clone()));
                                },
                                "{row.name}"
                            }
                        }
                    }
                }
                div { style: right_style,
                    h3 { style: "margin: 0 0 8px 0; font-size: 14px;", "{chat_title}" }
                    if selected_id.is_some() {
                        div { style: "flex: 1; overflow-y: auto; min-height: 120px;",
                            ul { style: "list-style: none; padding: 0; margin: 0;",
                                for row in message_rows.iter() {
                                    li { style: "padding: 6px 0; border-bottom: 1px solid #333;",
                                        span { style: "color: #9ca3af; font-size: 12px;", "{row.sender}: " }
                                        "{row.content}"
                                    }
                                }
                            }
                        }
                        p { style: "margin-top: 12px; color: #6b7280; font-size: 12px;", "Barre de saisie à brancher (send_message)." }
                    } else {
                        p { style: "color: #9ca3af; padding: 24px 0;", "{no_salon_msg}" }
                    }
                }
            }
        }
    }
}
