//! Écran de jeu principal (orchestration).
//!
//! @id: lord_of_the_castle.ui.game_screen
//! @do: orchestrate_game_screen_components
//! @role: ui
//! @layer: ui
//! @human: Écran de jeu complet assemblant header, footer, sidebar, game_area, overlays

use crate::app::Screen;
use crate::game_state::{GamePhase, GameState};
use crate::game_loop::{move_player, move_secondary_player};
use crate::loot::{InventoryEntry, ItemSlot};
use crate::player::{Dir8, Player};
use crate::ui::game_area::GameArea;
use crate::ui::game_footer::GameFooter;
use crate::ui::game_header::GameHeader;
use crate::ui::overlays::{
    panel_button, GameOverOverlay, InventoryPanel, Panel, SkillsPanel, StatsPanel, WaveWonOverlay,
};
use crate::ui::sidebar::{BattleSidebar, PreparationSidebar};
use dioxus::prelude::*;
use std::path::PathBuf;
use std::time::{Duration, Instant};

/// Touches de direction maintenues
#[derive(Default, Clone, Copy)]
struct HeldKeys {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    up2: bool,
    down2: bool,
    left2: bool,
    right2: bool,
}

/// Écran de jeu principal.
#[component]
pub fn GameScreen(
    screen: Signal<Screen>,
    game_state: Signal<Option<GameState>>,
    active_slot: Signal<Option<u8>>,
) -> Element {
    let mut active_panel = use_signal(|| Option::<Panel>::None);
    let mut dev_mode = use_signal(|| false);
    let build_mode = use_signal(|| false);
    let cursor_angle = use_signal(|| 0.0f32);
    let mut held_keys = use_signal(HeldKeys::default);
    let mut show_click_hint = use_signal(|| true);
    let hint_start_time = use_signal(|| Option::<Instant>::None);
    let base_path = use_context::<Signal<PathBuf>>();

    // Boucle de mouvement fluide
    use_effect(move || {
        let keys_signal = held_keys;
        let mut state_signal = game_state;
        spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_millis(16));
            loop {
                interval.tick().await;
                let keys = *keys_signal.read();
                if let Some(gs) = state_signal.write().as_mut() {
                    let delta = 0.016;
                    if keys.up && !keys.down { move_player(gs, Dir8::N, delta); }
                    else if keys.down && !keys.up { move_player(gs, Dir8::S, delta); }
                    if keys.left && !keys.right { move_player(gs, Dir8::W, delta); }
                    else if keys.right && !keys.left { move_player(gs, Dir8::E, delta); }
                    if keys.up2 && !keys.down2 { move_secondary_player(gs, Dir8::N, delta); }
                    else if keys.down2 && !keys.up2 { move_secondary_player(gs, Dir8::S, delta); }
                    if keys.left2 && !keys.right2 { move_secondary_player(gs, Dir8::W, delta); }
                    else if keys.right2 && !keys.left2 { move_secondary_player(gs, Dir8::E, delta); }
                }
            }
        });
    });

    // Timer pour notification
    use_effect(move || {
        let mut hint_signal = show_click_hint;
        let mut start_signal = hint_start_time;
        spawn(async move {
            if start_signal.read().is_none() {
                start_signal.set(Some(Instant::now()));
            }
            tokio::time::sleep(Duration::from_secs(3)).await;
            hint_signal.set(false);
        });
    });

    let gs = game_state.read().clone();
    let Some(ref gs) = gs else {
        return rsx! { div { "Aucune partie." } };
    };

    // Extraire les données du GameState
    let phase = gs.phase;
    let wave = gs.wave_number;
    let gold = gs.gold;
    let level = gs.level;
    let xp = gs.xp;
    let xp_needed = gs.xp_required_for_next_level();
    let xp_pct = if xp_needed > 0 { (xp as f32 / xp_needed as f32 * 100.0).min(100.0) } else { 0.0 };
    let player_hp = gs.player.hp;
    let player_hp_max = gs.player.hp_max;
    let player_dead = gs.player.dead;
    let is_game_over = gs.is_game_over();
    let is_wave_won = gs.is_wave_won();
    let enemies_alive = gs.enemies.len() as u32;
    let spawn_qty = gs.spawn_quantity;
    let castle_hp = gs.castle.hp;
    let castle_hp_max = gs.castle.hp_max;
    let castle_x = gs.castle.x;
    let castle_y = gs.castle.y;
    let skill_pts = gs.skill_points_available;
    let stat_pts = gs.stat_points_available;
    let towers_count = gs.towers.len() as u32;
    let troops_count = gs.troops.iter().filter(|t| t.is_active_in_squad()).count() as u32;
    let max_troops = gs.max_troops() as u32;
    let player_x = gs.player.x;
    let player_y = gs.player.y;
    let player_attack_range = Player::auto_attack_range();
    let armor = gs.player_total_armor();
    let atk_dmg = gs.player_auto_attack_damage();
    let inv_count = gs.inventory.len();
    let enemies_killed = gs.enemies_killed;
    let bosses_killed = gs.bosses_killed;
    let gold_total = gs.gold_total;
    let max_wave = gs.max_wave_reached;
    let stats = gs.effective_stats();
    let base_stats = gs.player.stats.clone();
    let skill_ranks = gs.warrior_skill_ranks.clone();
    let notifications: Vec<String> = gs.pending_level_up_notifications.clone();

    // Inventaire
    let inventory: Vec<(usize, String)> = gs
        .inventory
        .iter()
        .enumerate()
        .map(|(i, e)| {
            let label = match e {
                InventoryEntry::Unidentified(slot) => format!("? {} (non identifié)", slot.label()),
                InventoryEntry::Identified(item) => format!("{} [{}]", item.display_name, item.rarity.label()),
            };
            (i, label)
        })
        .collect();

    let equipped: Vec<(ItemSlot, String)> = ItemSlot::equipment_slots()
        .iter()
        .map(|&slot| {
            let label = gs
                .get_equipped(slot)
                .map(|i| format!("{} ({})", i.display_name, i.rarity.label()))
                .unwrap_or_else(|| "—".to_string());
            (slot, label)
        })
        .collect();

    rsx! {
        div {
            style: "display:flex;flex-direction:column;width:100%;height:100%;overflow:hidden;outline:none;",
            tabindex: "0",
            autofocus: true,
            onmounted: move |evt| {
                spawn(async move {
                    tokio::time::sleep(Duration::from_millis(50)).await;
                    let _ = evt.set_focus(true);
                });
            },
            onkeydown: move |evt| {
                show_click_hint.set(false);
                let key = evt.key().to_string();
                let mut keys = held_keys.write();
                match key.as_str() {
                    "w" | "W" | "ArrowUp" | "z" | "Z" => keys.up = true,
                    "a" | "A" | "ArrowLeft" | "q" | "Q" => keys.left = true,
                    "s" | "S" | "ArrowDown" => keys.down = true,
                    "d" | "D" | "ArrowRight" => keys.right = true,
                    "i" | "I" => keys.up2 = true,
                    "j" | "J" => keys.left2 = true,
                    "k" | "K" => keys.down2 = true,
                    "l" | "L" => keys.right2 = true,
                    _ => {}
                }
            },
            onkeyup: move |evt| {
                let key = evt.key().to_string();
                let mut keys = held_keys.write();
                match key.as_str() {
                    "w" | "W" | "ArrowUp" | "z" | "Z" => keys.up = false,
                    "a" | "A" | "ArrowLeft" | "q" | "Q" => keys.left = false,
                    "s" | "S" | "ArrowDown" => keys.down = false,
                    "d" | "D" | "ArrowRight" => keys.right = false,
                    "i" | "I" => keys.up2 = false,
                    "j" | "J" => keys.left2 = false,
                    "k" | "K" => keys.down2 = false,
                    "l" | "L" => keys.right2 = false,
                    _ => {}
                }
            },

            // Header
            GameHeader {
                screen, game_state, phase, wave, enemies_alive, spawn_qty,
                gold, level, towers_count, troops_count, max_troops
            }

            // Barre boutons + XP
            div {
                style: "display:flex;align-items:center;justify-content:space-between;padding:4px 16px;background:#1e2a3a;border-bottom:1px solid #2a3f5f;min-height:30px;flex-shrink:0;gap:8px;",
                div {
                    style: "display:flex;gap:4px;",
                    { panel_button("Stats", Panel::Stats, active_panel, skill_pts == 0 && stat_pts == 0) }
                    { panel_button("Skills", Panel::Skills, active_panel, skill_pts == 0) }
                    { panel_button("Inventaire", Panel::Inventory, active_panel, inv_count == 0) }
                    {
                        let dev_bg = if dev_mode() { "#3a2f1e" } else { "#232f3e" };
                        rsx! {
                            button {
                                style: "padding:3px 8px;background:{dev_bg};color:#8f98a0;border:1px solid #2a3f5f;border-radius:3px;cursor:pointer;font-size:11px;",
                                onclick: move |_| dev_mode.set(!dev_mode()),
                                "DEV"
                            }
                        }
                    }
                }
                div {
                    style: "flex:1;display:flex;align-items:center;gap:8px;max-width:400px;",
                    div {
                        style: "flex:1;height:10px;background:#232f3e;border-radius:5px;overflow:hidden;",
                        div { style: "width:{xp_pct:.1}%;height:100%;background:linear-gradient(90deg,#1a6fff,#1a9fff);transition:width 0.2s;" }
                    }
                    span { style: "font-size:11px;color:#8f98a0;white-space:nowrap;", "XP {xp}/{xp_needed}" }
                    if skill_pts > 0 {
                        span { style: "font-size:11px;color:#ffcc00;", "🔸{skill_pts} pts comp." }
                    }
                    if stat_pts > 0 {
                        span { style: "font-size:11px;color:#44ccff;", "🔹{stat_pts} pts stat" }
                    }
                }
            }

            // Corps : zone de jeu + sidebar
            div {
                style: "flex:1;display:flex;overflow:hidden;min-height:0;align-items:stretch;",

                GameArea {
                    game_state, cursor_angle, build_mode, show_click_hint,
                    wave, phase, castle_x, castle_y, castle_hp, castle_hp_max,
                    player_x, player_y, player_dead, player_attack_range
                }

                if phase == GamePhase::Preparation {
                    PreparationSidebar { game_state, gold, max_troops, troops_count, dev_mode, build_mode }
                }
                if phase == GamePhase::Battle {
                    BattleSidebar { wave, enemies_alive, spawn_qty, castle_hp, castle_hp_max, towers_count, troops_count, is_wave_won, is_game_over }
                }
            }

            // Footer
            GameFooter {
                game_state, active_slot, base_path, phase, wave, is_wave_won,
                player_hp, player_hp_max, player_dead, armor, atk_dmg, castle_hp, castle_hp_max
            }

            // Panneau overlay
            if let Some(panel) = active_panel() {
                div {
                    style: "position:absolute;top:68px;left:50%;transform:translateX(-50%);width:520px;max-height:calc(100vh - 150px);background:#1b2838;border:1px solid #2a3f5f;border-radius:6px;box-shadow:0 8px 32px rgba(0,0,0,0.6);z-index:50;display:flex;flex-direction:column;overflow:hidden;",
                    div {
                        style: "display:flex;align-items:center;justify-content:space-between;padding:8px 12px;background:#232f3e;border-bottom:1px solid #2a3f5f;flex-shrink:0;",
                        span {
                            style: "font-size:14px;font-weight:600;color:#1a9fff;",
                            { match panel { Panel::Stats => "Statistiques", Panel::Skills => "Compétences Guerrier", Panel::Inventory => "Inventaire" } }
                        }
                        button {
                            style: "padding:2px 8px;background:#3a2222;color:#cc6666;border:1px solid #4a3333;border-radius:3px;cursor:pointer;font-size:12px;",
                            onclick: move |_| active_panel.set(None),
                            "✕"
                        }
                    }
                    div {
                        style: "flex:1;overflow-y:auto;padding:10px;",
                        { match panel {
                            Panel::Stats => rsx! {
                                StatsPanel {
                                    game_state, base_stats: base_stats.clone(), eff_stats: stats.clone(), stat_pts,
                                    player_hp, player_hp_max, armor, atk_dmg, castle_hp, castle_hp_max,
                                    enemies_killed, bosses_killed, gold_total, max_wave
                                }
                            },
                            Panel::Skills => rsx! {
                                SkillsPanel { game_state, skill_ranks: skill_ranks.clone(), skill_pts }
                            },
                            Panel::Inventory => rsx! {
                                InventoryPanel { game_state, inventory: inventory.clone(), equipped: equipped.clone(), gold }
                            },
                        } }
                    }
                }
            }

            // Overlay Game Over
            if is_game_over {
                GameOverOverlay { screen, game_state, wave, enemies_killed, bosses_killed, gold_total }
            }

            // Overlay Vague gagnée
            if is_wave_won && !is_game_over {
                WaveWonOverlay { wave, enemies_killed }
            }

            // Notifications
            if !notifications.is_empty() {
                div {
                    style: "position:absolute;bottom:50px;right:16px;display:flex;flex-direction:column;gap:4px;z-index:60;pointer-events:none;",
                    for notif in notifications.iter() {
                        div {
                            style: "padding:6px 12px;background:#2a3f1e;border:1px solid #4a5f3e;border-radius:4px;font-size:12px;color:#88cc44;",
                            "{notif}"
                        }
                    }
                }
            }
        }
    }
}
