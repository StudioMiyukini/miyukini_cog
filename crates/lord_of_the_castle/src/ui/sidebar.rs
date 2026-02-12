//! Sidebars pour les phases Préparation et Combat.
//!
//! @id: lord_of_the_castle.ui.sidebar
//! @do: render_preparation_and_battle_sidebars
//! @role: ui
//! @layer: ui
//! @human: Panneaux latéraux pour construction, recrutement et infos de combat

use crate::constants::TOWER_BASE_COST_GOLD;
use crate::game_state::GameState;
use crate::troops::TroopKind;
use dioxus::prelude::*;

/// Sidebar pour la phase de préparation.
#[component]
pub fn PreparationSidebar(
    mut game_state: Signal<Option<GameState>>,
    gold: u32,
    max_troops: u32,
    troops_count: u32,
    dev_mode: Signal<bool>,
    mut build_mode: Signal<bool>,
) -> Element {
    rsx! {
        div {
            style: "width:200px;min-height:600px;background:#1b2838;border-left:1px solid #2a3f5f;display:flex;flex-direction:column;overflow-y:auto;flex-shrink:0;padding:8px;gap:6px;",

            h3 { style: "font-size:13px;color:#1a9fff;margin-bottom:4px;text-align:center;", "Préparation" }

            // Construction tour - Mode RTS
            div {
                style: "background:#232f3e;border-radius:4px;padding:8px;",
                p { style: "font-size:12px;color:#c6d4df;font-weight:600;margin-bottom:4px;", "🏗 Tour de base" }
                p { style: "font-size:10px;color:#8f98a0;margin-bottom:6px;", "PV 100 • Portée 300px • Dmg 4" }
                p { style: "font-size:10px;color:#ffcc00;margin-bottom:6px;", "Coût : {TOWER_BASE_COST_GOLD} or" }
                
                {
                    let is_build_mode = build_mode();
                    let btn_bg = if is_build_mode { "#3a5a2a" } else { "#2a4a2a" };
                    let btn_border = if is_build_mode { "#5a8a3a" } else { "#3a5a3a" };
                    let btn_text = if is_build_mode { "🔨 Mode construction ACTIF" } else { "🏗 Placer une tour" };
                    rsx! {
                        button {
                            style: "width:100%;padding:5px;background:{btn_bg};color:#88cc88;border:1px solid {btn_border};border-radius:3px;cursor:pointer;font-size:11px;margin-bottom:4px;",
                            disabled: gold < TOWER_BASE_COST_GOLD,
                            onclick: move |_| {
                                build_mode.set(!build_mode());
                            },
                            "{btn_text}"
                        }
                    }
                }
                
                button {
                    style: "width:100%;padding:4px;background:#1a3a1a;color:#668866;border:1px solid #2a4a2a;border-radius:3px;cursor:pointer;font-size:10px;",
                    disabled: gold < TOWER_BASE_COST_GOLD,
                    onclick: move |_| {
                        if let Some(gs) = game_state.write().as_mut() {
                            if let Some((ci, cj)) = find_build_cell(gs) {
                                gs.build_tower_at_cell(ci, cj);
                            }
                        }
                    },
                    "Auto-placer"
                }
                
                if build_mode() {
                    p { 
                        style: "font-size:9px;color:#88cc88;margin-top:6px;text-align:center;",
                        "Cliquez sur une case verte pour construire"
                    }
                }
            }

            // Recrutement troupe
            div {
                style: "background:#232f3e;border-radius:4px;padding:8px;",
                p { style: "font-size:12px;color:#c6d4df;font-weight:600;margin-bottom:4px;", "⚔ Milicien" }
                p { style: "font-size:10px;color:#8f98a0;margin-bottom:6px;",
                    "PV 10 • Att 2 • Portée 15px"
                }
                p { style: "font-size:10px;color:#8f98a0;margin-bottom:6px;",
                    "Limite : Charisme ({max_troops} max)"
                }
                button {
                    style: "width:100%;padding:5px;background:#2a3a4a;color:#88aacc;border:1px solid #3a4a5a;border-radius:3px;cursor:pointer;font-size:11px;",
                    disabled: troops_count >= max_troops,
                    onclick: move |_| {
                        if let Some(gs) = game_state.write().as_mut() {
                            gs.recruit_troop(TroopKind::Milicien);
                        }
                    },
                    "Recruter"
                }
            }

            // Joueur 2
            div {
                style: "background:#232f3e;border-radius:4px;padding:8px;",
                p { style: "font-size:12px;color:#c6d4df;font-weight:600;margin-bottom:4px;", "👥 Joueur 2" }
                button {
                    style: "width:100%;padding:4px;background:#2a3a4a;color:#88cccc;border:1px solid #3a4a5a;border-radius:3px;cursor:pointer;font-size:10px;margin-bottom:3px;",
                    onclick: move |_| {
                        if let Some(gs) = game_state.write().as_mut() {
                            gs.spawn_secondary_tombilol();
                        }
                    },
                    "Tombilol (CàC 360°)"
                }
                button {
                    style: "width:100%;padding:4px;background:#2a3a4a;color:#88cccc;border:1px solid #3a4a5a;border-radius:3px;cursor:pointer;font-size:10px;margin-bottom:3px;",
                    onclick: move |_| {
                        if let Some(gs) = game_state.write().as_mut() {
                            gs.spawn_secondary_tal_ratchou();
                        }
                    },
                    "Tal Ratchou (proj. homing)"
                }
                button {
                    style: "width:100%;padding:4px;background:#2a3a4a;color:#88cccc;border:1px solid #3a4a5a;border-radius:3px;cursor:pointer;font-size:10px;",
                    onclick: move |_| {
                        if let Some(gs) = game_state.write().as_mut() {
                            gs.spawn_secondary_sergent_garcia();
                        }
                    },
                    "Sgt Garcia (miliciens)"
                }
            }

            // Dev mode actions
            if dev_mode() {
                { dev_panel(game_state) }
            }
        }
    }
}

/// Panneau dev mode.
fn dev_panel(mut game_state: Signal<Option<GameState>>) -> Element {
    rsx! {
        div {
            style: "background:#3a2f1e;border-radius:4px;padding:8px;",
            p { style: "font-size:12px;color:#ffaa44;font-weight:600;margin-bottom:4px;", "🔧 Dev" }
            button {
                style: "width:100%;padding:4px;background:#4a3a2a;color:#ccaa88;border:1px solid #5a4a3a;border-radius:3px;cursor:pointer;font-size:10px;margin-bottom:3px;",
                onclick: move |_| {
                    if let Some(gs) = game_state.write().as_mut() {
                        gs.dev_spawn_50_normal_enemies_random();
                    }
                },
                "+50 ennemis"
            }
            button {
                style: "width:100%;padding:4px;background:#4a3a2a;color:#ccaa88;border:1px solid #5a4a3a;border-radius:3px;cursor:pointer;font-size:10px;margin-bottom:3px;",
                onclick: move |_| {
                    if let Some(gs) = game_state.write().as_mut() {
                        gs.dev_give_level_up();
                    }
                },
                "+1 Niveau"
            }
            button {
                style: "width:100%;padding:4px;background:#4a3a2a;color:#ccaa88;border:1px solid #5a4a3a;border-radius:3px;cursor:pointer;font-size:10px;margin-bottom:3px;",
                onclick: move |_| {
                    if let Some(gs) = game_state.write().as_mut() {
                        gs.gold += 500;
                    }
                },
                "+500 or"
            }
            button {
                style: "width:100%;padding:4px;background:#4a3a2a;color:#ccaa88;border:1px solid #5a4a3a;border-radius:3px;cursor:pointer;font-size:10px;margin-bottom:3px;",
                onclick: move |_| {
                    if let Some(gs) = game_state.write().as_mut() {
                        gs.secondary_give_level_up();
                    }
                },
                "J2 +1 Niveau"
            }
            button {
                style: "width:100%;padding:4px;background:#4a3a2a;color:#ccaa88;border:1px solid #5a4a3a;border-radius:3px;cursor:pointer;font-size:10px;",
                onclick: move |_| {
                    if let Some(gs) = game_state.write().as_mut() {
                        gs.refresh_merchant_pools();
                    }
                },
                "Reroll marchand"
            }
        }
    }
}

/// Sidebar pour la phase de combat.
#[component]
pub fn BattleSidebar(
    wave: u32,
    enemies_alive: u32,
    spawn_qty: u32,
    castle_hp: i32,
    castle_hp_max: i32,
    towers_count: u32,
    troops_count: u32,
    is_wave_won: bool,
    is_game_over: bool,
) -> Element {
    let castle_pct = if castle_hp_max > 0 { castle_hp as f32 / castle_hp_max as f32 * 100.0 } else { 0.0 };

    rsx! {
        div {
            style: "width:200px;min-height:600px;background:#1b2838;border-left:1px solid #2a3f5f;display:flex;flex-direction:column;overflow-y:auto;flex-shrink:0;padding:8px;gap:6px;",

            h3 { style: "font-size:13px;color:#ff6644;margin-bottom:4px;text-align:center;", "⚔ Combat" }

            div {
                style: "background:#232f3e;border-radius:4px;padding:8px;",
                p { style: "font-size:12px;color:#c6d4df;font-weight:600;margin-bottom:4px;", "Vague {wave}" }
                p { style: "font-size:11px;color:#8f98a0;margin-bottom:4px;", "Ennemis restants : {enemies_alive}" }
                p { style: "font-size:11px;color:#8f98a0;", "Total à spawner : {spawn_qty}" }
            }

            div {
                style: "background:#232f3e;border-radius:4px;padding:8px;",
                p { style: "font-size:12px;color:#c6d4df;font-weight:600;margin-bottom:4px;", "🏰 Château" }
                div {
                    style: "width:100%;height:8px;background:#332222;border-radius:4px;overflow:hidden;margin-bottom:4px;",
                    div {
                        style: "width:{castle_pct:.1}%;height:100%;background:#667788;",
                    }
                }
                p { style: "font-size:10px;color:#8f98a0;text-align:center;", "{castle_hp}/{castle_hp_max} PV" }
            }

            if is_wave_won {
                div {
                    style: "background:#2a4a2a;border:1px solid #4a8a4a;border-radius:4px;padding:12px;text-align:center;",
                    p { style: "font-size:14px;color:#88ff88;font-weight:700;", "✓ VICTOIRE !" }
                    p { style: "font-size:11px;color:#aaffaa;margin-top:4px;", "Vague {wave} terminée" }
                }
            }
            if is_game_over {
                div {
                    style: "background:#4a2a2a;border:1px solid #8a4a4a;border-radius:4px;padding:12px;text-align:center;",
                    p { style: "font-size:14px;color:#ff8888;font-weight:700;", "✗ DÉFAITE" }
                    p { style: "font-size:11px;color:#ffaaaa;margin-top:4px;", "Le château est détruit" }
                }
            }

            div {
                style: "background:#232f3e;border-radius:4px;padding:8px;",
                p { style: "font-size:12px;color:#c6d4df;font-weight:600;margin-bottom:4px;", "🗼 Tours : {towers_count}" }
                p { style: "font-size:10px;color:#8f98a0;", "Défense automatique active" }
            }

            if troops_count > 0 {
                div {
                    style: "background:#232f3e;border-radius:4px;padding:8px;",
                    p { style: "font-size:12px;color:#c6d4df;font-weight:600;margin-bottom:4px;", "⚔ Troupes : {troops_count}" }
                    p { style: "font-size:10px;color:#8f98a0;", "Combat en cours..." }
                }
            }
        }
    }
}

/// Cherche la première cellule libre pour construire une tour.
fn find_build_cell(gs: &GameState) -> Option<(i32, i32)> {
    for ring in 2i32..15 {
        for i in -ring..=ring {
            for j in -ring..=ring {
                if (i.abs() == ring || j.abs() == ring) && gs.can_build_at_cell(i, j) {
                    return Some((i, j));
                }
            }
        }
    }
    None
}
