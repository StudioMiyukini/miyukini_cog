//! Barre de pied de page de l'écran de jeu.
//!
//! Affiche : PV joueur, armure, dégâts, boutons d'action, slots de sauvegarde, château.
//!
//! @id: lord_of_the_castle.ui.game_footer
//! @do: render_game_footer_bar
//! @role: ui
//! @layer: ui
//! @human: Footer avec PV, actions (lancer vague) et sauvegarde

use crate::game_state::{GamePhase, GameState};
use crate::save::LordOfTheCastleDb;
use dioxus::prelude::*;
use std::path::PathBuf;

/// Barre de pied de page du jeu.
#[component]
pub fn GameFooter(
    game_state: Signal<Option<GameState>>,
    active_slot: Signal<Option<u8>>,
    base_path: Signal<PathBuf>,
    phase: GamePhase,
    wave: u32,
    is_wave_won: bool,
    player_hp: i32,
    player_hp_max: i32,
    player_dead: bool,
    armor: i32,
    atk_dmg: i32,
    castle_hp: i32,
    castle_hp_max: i32,
) -> Element {
    let footer_hp_pct = if player_hp_max > 0 { player_hp as f32 / player_hp_max as f32 * 100.0 } else { 0.0 };
    let footer_hp_color = if player_dead { "#666" } else if player_hp as f32 / player_hp_max.max(1) as f32 > 0.5 { "#cc4444" } else { "#ff2222" };
    let castle_pct = if castle_hp_max > 0 { castle_hp as f32 / castle_hp_max as f32 * 100.0 } else { 0.0 };

    rsx! {
        footer {
            style: "display:flex;align-items:center;justify-content:space-between;padding:5px 16px;background:#1b2838;border-top:1px solid #2a3f5f;min-height:40px;flex-shrink:0;gap:8px;",

            // Gauche : PV / Armure / Att
            div {
                style: "display:flex;align-items:center;gap:10px;flex-wrap:nowrap;",
                div {
                    style: "display:flex;align-items:center;gap:4px;",
                    span { style: "font-size:11px;color:#cc4444;", "♥" }
                    div {
                        style: "width:100px;height:8px;background:#332222;border-radius:4px;overflow:hidden;",
                        div {
                            style: "width:{footer_hp_pct:.1}%;height:100%;background:{footer_hp_color};transition:width 0.1s;",
                        }
                    }
                    span { style: "font-size:11px;color:#c6d4df;", "{player_hp}/{player_hp_max}" }
                }
                span { style: "font-size:11px;color:#8f98a0;", "🛡{armor}" }
                span { style: "font-size:11px;color:#8f98a0;", "⚔{atk_dmg}" }
                if player_dead {
                    span { style: "font-size:11px;color:#ff4444;font-weight:600;", "💀 MORT" }
                }
            }

            // Centre : actions
            div {
                style: "display:flex;align-items:center;gap:6px;",
                if phase == GamePhase::Preparation {
                    button {
                        style: "padding:6px 20px;background:linear-gradient(135deg,#1a9fff 0%,#1477cc 100%);color:white;border:none;border-radius:4px;cursor:pointer;font-weight:700;font-size:13px;box-shadow:0 2px 8px rgba(26,159,255,0.3);",
                        onclick: move |_| {
                            if let Some(g) = game_state.write().as_mut() {
                                g.start_battle_phase();
                            }
                        },
                        "⚔ Lancer la vague {wave}"
                    }
                }
                if is_wave_won {
                    button {
                        style: "padding:6px 20px;background:linear-gradient(135deg,#5ba32b 0%,#3d8c40 100%);color:white;border:none;border-radius:4px;cursor:pointer;font-weight:700;font-size:13px;box-shadow:0 2px 8px rgba(91,163,43,0.3);",
                        onclick: move |_| {
                            if let Some(g) = game_state.write().as_mut() {
                                g.start_preparation_phase();
                            }
                            if let Some(slot) = active_slot() {
                                let path = base_path.read().clone();
                                let state = game_state.read().clone();
                                if let Some(st) = state {
                                    spawn(async move {
                                        let _ = tokio::task::spawn_blocking(move || {
                                            let db = LordOfTheCastleDb::open(path.join("lord_of_the_castle.db"))?;
                                            db.slot_write(slot, &st)
                                        }).await;
                                    });
                                }
                            }
                        },
                        "✓ Fin de vague → Préparation"
                    }
                }
            }

            // Droite-centre : sauvegarde
            div {
                style: "display:flex;align-items:center;gap:3px;",
                span { style: "font-size:10px;color:#556677;margin-right:2px;", "💾" }
                for slot_id in [1u8, 2, 3] {
                    {
                        let path_clone = base_path.read().clone();
                        let is_active = active_slot() == Some(slot_id);
                        let bg = if is_active { "#2a4a3a" } else { "#232f3e" };
                        let border = if is_active { "#4a8a5a" } else { "#2a3f5f" };
                        let color = if is_active { "#88cc88" } else { "#8f98a0" };
                        rsx! {
                            button {
                                style: "padding:3px 8px;background:{bg};color:{color};border:1px solid {border};border-radius:3px;cursor:pointer;font-size:10px;",
                                onclick: move |_| {
                                    let path = path_clone.clone();
                                    let state = game_state.read().clone();
                                    if let Some(st) = state {
                                        active_slot.set(Some(slot_id));
                                        spawn(async move {
                                            let _ = tokio::task::spawn_blocking(move || {
                                                let db = LordOfTheCastleDb::open(path.join("lord_of_the_castle.db"))?;
                                                db.slot_write(slot_id, &st)
                                            }).await;
                                        });
                                    }
                                },
                                "{slot_id}"
                            }
                        }
                    }
                }
            }

            // Droite : château
            div {
                style: "display:flex;align-items:center;gap:6px;",
                span { style: "font-size:11px;color:#8f98a0;", "🏰" }
                div {
                    style: "width:60px;height:8px;background:#333;border-radius:4px;overflow:hidden;",
                    div {
                        style: "width:{castle_pct:.1}%;height:100%;background:#667788;",
                    }
                }
                span { style: "font-size:11px;color:#c6d4df;", "{castle_hp}/{castle_hp_max}" }
            }
        }
    }
}
