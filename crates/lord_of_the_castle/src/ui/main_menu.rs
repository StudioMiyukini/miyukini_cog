//! Menu principal du jeu.
//!
//! Ce module contient l'écran de sélection de slot / nouvelle partie.
//!
//! @id: lord_of_the_castle.ui.main_menu
//! @do: render_main_menu_screen
//! @role: ui
//! @layer: ui
//! @human: Écran de menu principal avec sélection de slots de sauvegarde.

use crate::app::Screen;
use crate::game_state::GameState;
use crate::save::{LordOfTheCastleDb, SlotMetadata};
use dioxus::prelude::*;
use std::path::PathBuf;

/// Menu principal avec sélection de slots.
#[component]
pub fn MainMenu(
    screen: Signal<Screen>,
    game_state: Signal<Option<GameState>>,
    active_slot: Signal<Option<u8>>,
) -> Element {
    let base_path = use_context::<Signal<PathBuf>>();
    let slots_resource = use_resource(move || {
        let path = base_path.read().clone();
        async move {
            tokio::task::spawn_blocking(move || {
                LordOfTheCastleDb::open(path.join("lord_of_the_castle.db")).map(|db| db.slot_list())
            })
            .await
            .map_err(|_| "spawn_blocking failed".to_string())?
            .map_err(|e| e.to_string())
        }
    });

    let slots: Vec<SlotMetadata> = slots_resource
        .read()
        .as_ref()
        .and_then(|r| r.as_ref().ok())
        .cloned()
        .unwrap_or_default();

    let slot_entries: Vec<(u8, bool, Option<String>)> = slots
        .iter()
        .map(|s| (s.slot_id, s.occupied, s.summary.clone()))
        .collect();

    rsx! {
        div {
            style: "flex:1;display:flex;flex-direction:column;align-items:center;justify-content:center;gap:20px;",
            h1 {
                style: "font-size:32px;font-weight:700;color:#1a9fff;",
                "Lord of the Castle"
            }
            p {
                style: "font-size:14px;color:#8f98a0;margin-bottom:24px;",
                "Miyukini Survivor — Survivor + Tower Defense"
            }
            button {
                style: "padding:14px 44px;background:linear-gradient(135deg,#5ba32b 0%,#3d8c40 100%);color:white;border:none;border-radius:6px;font-size:16px;font-weight:600;cursor:pointer;",
                onclick: move |_| {
                    screen.set(Screen::CharacterCreation);
                },
                "▶ Nouvelle partie"
            }
            div {
                style: "display:flex;flex-direction:column;gap:6px;width:100%;max-width:300px;",
                for (slot_id, occupied, summary) in slot_entries.into_iter() {
                    button {
                        style: "padding:10px 20px;background:#2a3f5f;color:#c6d4df;border:1px solid #3d4f5f;border-radius:6px;font-size:13px;cursor:pointer;text-align:left;",
                        disabled: !occupied,
                        onclick: move |_| {
                            if !occupied { return; }
                            let path = base_path.read().clone();
                            spawn(async move {
                                let r = tokio::task::spawn_blocking(move || {
                                    let db = LordOfTheCastleDb::open(path.join("lord_of_the_castle.db"))?;
                                    db.slot_read(slot_id)
                                }).await.ok().and_then(|r| r.ok());
                                if let Some(gs) = r {
                                    game_state.set(Some(gs));
                                    active_slot.set(Some(slot_id));
                                    screen.set(Screen::Game);
                                }
                            });
                        },
                        { if occupied {
                            format!("Slot {} : {}", slot_id, summary.as_deref().unwrap_or("Sauvegardé"))
                        } else {
                            format!("Slot {} : Vide", slot_id)
                        } }
                    }
                }
            }
        }
    }
}
