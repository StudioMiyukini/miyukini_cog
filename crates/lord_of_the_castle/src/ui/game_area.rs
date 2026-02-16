//! Zone de jeu centrale avec rendu des entités.
//!
//! Ce module gère la zone de combat où sont affichées toutes les entités
//! (joueur, ennemis, tours, troupes, château, projectiles, loot).
//!
//! @id: lord_of_the_castle.ui.game_area
//! @do: render_game_combat_area
//! @role: ui
//! @layer: ui
//! @human: Zone de jeu centrale avec rendu des entités de combat.

use crate::constants::size::CONSTRUCTION_CELL_SIZE;
use crate::constants::{COMBAT_SURFACE_SIZE, TOWER_BASE_COST_GOLD};
use crate::game_state::{GamePhase, GameState};
use crate::styles::{entity_style, hp_bar_fill_style, hp_bar_outer, px};
use dioxus::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};

/// Surface de rendu.
const SURFACE: f32 = COMBAT_SURFACE_SIZE;

/// Position du curseur en coordonnées monde (partagée entre composants).
static CURSOR_WORLD: AtomicU64 = AtomicU64::new(0);

/// Définit la position du curseur en coordonnées monde.
pub fn set_cursor_world(x: f32, y: f32) {
    let packed = ((x.to_bits() as u64) << 32) | (y.to_bits() as u64);
    CURSOR_WORLD.store(packed, Ordering::Relaxed);
}

/// Récupère la position du curseur en coordonnées monde.
pub fn get_cursor_world() -> Option<(f32, f32)> {
    let packed = CURSOR_WORLD.load(Ordering::Relaxed);
    if packed == 0 {
        return None;
    }
    let x = f32::from_bits((packed >> 32) as u32);
    let y = f32::from_bits(packed as u32);
    Some((x, y))
}

/// Props pour le composant GameArea.
#[derive(Props, Clone, PartialEq)]
pub struct GameAreaProps {
    pub game_state: Signal<Option<GameState>>,
    pub build_mode: Signal<bool>,
    pub cursor_angle: Signal<f32>,
    pub show_click_hint: Signal<bool>,
    pub phase: GamePhase,
    pub wave: u32,
    // Données de rendu pré-calculées
    pub castle_x: f32,
    pub castle_y: f32,
    pub castle_hp: i32,
    pub castle_hp_max: i32,
    pub player_x: f32,
    pub player_y: f32,
    pub player_dead: bool,
    pub player_attack_range: f32,
}

/// Zone de jeu centrale.
#[component]
pub fn GameArea(mut props: GameAreaProps) -> Element {
    // Grille de construction RTS
    let grid_radius = 10i32;
    let cell_size_world = CONSTRUCTION_CELL_SIZE;

    // Lecture du game_state pour les données de rendu
    let gs = props.game_state.read();
    let gs_ref = gs.as_ref();

    // Calcul des cellules de grille si en mode construction
    let grid_cells: Vec<(i32, i32, bool, f32, f32)> =
        if props.phase == GamePhase::Preparation && *props.build_mode.read() {
            if let Some(gs) = gs_ref {
                let mut cells = Vec::new();
                for ci in -grid_radius..=grid_radius {
                    for cj in -grid_radius..=grid_radius {
                        let world_x = props.castle_x + (ci as f32) * cell_size_world;
                        let world_y = props.castle_y + (cj as f32) * cell_size_world;
                        let can_build = gs.can_build_at_cell(ci, cj);
                        cells.push((ci, cj, can_build, world_x, world_y));
                    }
                }
                cells
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };

    // Pré-calcul des styles
    let castle_ent_style = format!(
        "{}border:2px solid #8899aa;border-radius:3px;",
        entity_style(props.castle_x, props.castle_y, 40.0, "#667788")
    );
    let castle_hp_outer_s = hp_bar_outer(props.castle_x, props.castle_y, 40.0);
    let castle_hp_fill_s = hp_bar_fill_style(props.castle_hp, props.castle_hp_max);

    // Style du joueur
    let player_style = if !props.player_dead {
        format!(
            "{}border:2px solid #88bbff;border-radius:50%;z-index:5;",
            entity_style(props.player_x, props.player_y, 15.0, "#4488ff")
        )
    } else {
        format!(
            "{}border:2px solid #666688;border-radius:50%;z-index:5;opacity:0.5;",
            entity_style(props.player_x, props.player_y, 15.0, "#444466")
        )
    };

    // Cône d'attaque du joueur
    let current_cursor_angle = *props.cursor_angle.read();
    let attack_cone_style = if !props.player_dead && props.phase == GamePhase::Battle {
        let cone_size = props.player_attack_range * 2.0;
        Some(format!(
            "position:absolute;left:{xp:.0}px;top:{yp:.0}px;width:{sz:.0}px;height:{sz:.0}px;transform:translate(-50%,-50%) rotate({angle}deg);background:conic-gradient(from -20deg, rgba(68,136,255,0.3) 0deg, rgba(68,136,255,0.3) 40deg, transparent 40deg);border-radius:50%;pointer-events:none;z-index:4;",
            xp = px(props.player_x),
            yp = px(props.player_y),
            sz = cone_size,
            angle = current_cursor_angle,
        ))
    } else {
        None
    };

    // Données de rendu des entités (à partir du game_state)
    let (
        loot_styles,
        tower_styles,
        troop_styles,
        enemy_styles,
        sec_style,
        proj_styles,
        sec_proj_styles,
    ) = if let Some(gs) = gs_ref {
        compute_entity_styles(gs)
    } else {
        (
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            None,
            Vec::new(),
            Vec::new(),
        )
    };

    rsx! {
        // Zone de jeu (centre)
        div {
            style: "flex:1;display:flex;align-items:center;justify-content:center;background:#0a0e14;position:relative;overflow:hidden;",

            // Surface de jeu (800×800 px fixe, zone de combat de référence)
            div {
                style: "position:relative;width:800px;height:800px;background:#0d1117;border:1px solid #1a2233;overflow:hidden;",
                onmousemove: move |evt| {
                    let coords = evt.element_coordinates();
                    let cursor_px_x = coords.x as f32;
                    let cursor_px_y = coords.y as f32;

                    let elem_size = SURFACE;
                    let cursor_world_x = (cursor_px_x / elem_size) * SURFACE;
                    let cursor_world_y = (cursor_px_y / elem_size) * SURFACE;

                    set_cursor_world(cursor_world_x, cursor_world_y);

                    if let Some(gs) = props.game_state.read().as_ref() {
                        let px = gs.player.x;
                        let py = gs.player.y;
                        let dx = cursor_world_x - px;
                        let dy = cursor_world_y - py;
                        let angle_rad = dy.atan2(dx);
                        let angle_deg = angle_rad.to_degrees() + 90.0;
                        props.cursor_angle.set(angle_deg);
                    }
                },

                // Notification "Cliquez sur la zone de combat"
                if *props.show_click_hint.read() && props.wave <= 3 {
                    div {
                        style: "position:absolute;top:50%;left:50%;transform:translate(-50%,-50%);
                                background:rgba(0,0,0,0.85);padding:16px 24px;border-radius:8px;
                                border:2px solid #4a90d9;color:#fff;font-size:16px;font-weight:bold;
                                z-index:100;text-align:center;pointer-events:none;
                                animation:fadeInOut 3s ease-in-out forwards;",
                        "Cliquez sur la zone de combat pour activer les contrôles"
                    }
                }

                // Grille de construction RTS
                if *props.build_mode.read() && props.phase == GamePhase::Preparation {
                    for (ci, cj, can_build, wx, wy) in grid_cells.iter() {
                        {
                            let ci_copy = *ci;
                            let cj_copy = *cj;
                            let can_build_copy = *can_build;
                            let bg_color = if can_build_copy {
                                "rgba(68, 204, 68, 0.3)"
                            } else {
                                "rgba(204, 68, 68, 0.2)"
                            };
                            let border_color = if can_build_copy {
                                "rgba(68, 204, 68, 0.6)"
                            } else {
                                "rgba(204, 68, 68, 0.4)"
                            };
                            let hover_style = if can_build_copy {
                                "cursor:pointer;"
                            } else {
                                "cursor:not-allowed;"
                            };
                            rsx! {
                                div {
                                    style: "position:absolute;left:{px(*wx):.0}px;top:{px(*wy):.0}px;width:{px(cell_size_world):.0}px;height:{px(cell_size_world):.0}px;background:{bg_color};border:1px solid {border_color};transform:translate(-50%,-50%);{hover_style}z-index:1;",
                                    onclick: move |_| {
                                        if can_build_copy {
                                            if let Some(gs) = props.game_state.write().as_mut() {
                                                if gs.gold >= TOWER_BASE_COST_GOLD {
                                                    gs.build_tower_at_cell(ci_copy, cj_copy);
                                                }
                                            }
                                        }
                                    },
                                }
                            }
                        }
                    }
                }

                // Loot au sol
                for s in loot_styles.iter() {
                    div { style: "{s}" }
                }

                // Château
                div { style: "{castle_ent_style}" }
                div {
                    style: "{castle_hp_outer_s}",
                    div { style: "{castle_hp_fill_s}" }
                }

                // Tours
                for (te, to, tf) in tower_styles.iter() {
                    div { style: "{te}" }
                    div {
                        style: "{to}",
                        div { style: "{tf}" }
                    }
                }

                // Troupes
                for s in troop_styles.iter() {
                    div { style: "{s}" }
                }

                // Ennemis
                for (ent_s, hp_opt) in enemy_styles.iter() {
                    div { style: "{ent_s}" }
                    if let Some((ho, hf)) = hp_opt {
                        div {
                            style: "{ho}",
                            div { style: "{hf}" }
                        }
                    }
                }

                // Joueur secondaire
                if let Some(ref ss) = sec_style {
                    div { style: "{ss}" }
                }

                // Cône d'attaque joueur
                if let Some(ref cone_style) = attack_cone_style {
                    div { style: "{cone_style}" }
                }

                // Joueur
                div { style: "{player_style}" }

                // Projectiles tours
                for s in proj_styles.iter() {
                    div { style: "{s}" }
                }

                // Projectiles secondaires
                for s in sec_proj_styles.iter() {
                    div { style: "{s}" }
                }

                // Légende (coin bas gauche)
                div {
                    style: "position:absolute;bottom:4px;left:4px;font-size:9px;color:#3a4555;pointer-events:none;",
                    "WASD : déplacement • IJKL : joueur 2"
                }
            }
        }
    }
}

/// Calcule les styles de rendu pour toutes les entités.
fn compute_entity_styles(
    gs: &GameState,
) -> (
    Vec<String>,                           // loot_styles
    Vec<(String, String, String)>,         // tower_styles
    Vec<String>,                           // troop_styles
    Vec<(String, Option<(String, String)>)>, // enemy_styles
    Option<String>,                        // sec_style
    Vec<String>,                           // proj_styles
    Vec<String>,                           // sec_proj_styles
) {
    use crate::enemies::EnemyKind;
    use crate::loot::LootKind;

    // Loot
    let loot_styles: Vec<String> = gs
        .loot_drops
        .iter()
        .map(|d| {
            let c = match d.kind {
                LootKind::Gold(_) => "#ffcc00",
                LootKind::Xp(_) => "#4488ff",
                LootKind::Item(_) => "#aa6633",
            };
            entity_style(d.x, d.y, 5.0, c)
        })
        .collect();

    // Tours
    let tower_styles: Vec<(String, String, String)> = gs
        .towers
        .iter()
        .filter(|t| t.hp > 0)
        .map(|t| {
            (
                format!(
                    "{}border:1px solid #aa8833;border-radius:2px;",
                    entity_style(t.x, t.y, 40.0, "#8b6914")
                ),
                hp_bar_outer(t.x, t.y, 40.0),
                hp_bar_fill_style(t.hp, t.hp_max),
            )
        })
        .collect();

    // Troupes
    let troop_styles: Vec<String> = gs
        .troops
        .iter()
        .filter(|t| t.is_alive())
        .map(|t| {
            let c = if t.follow_secondary_id.is_some() {
                "#22aa88"
            } else {
                "#44cc44"
            };
            format!("{}border-radius:50%;", entity_style(t.x, t.y, 10.0, c))
        })
        .collect();

    // Ennemis
    let enemy_styles: Vec<(String, Option<(String, String)>)> = gs
        .enemies
        .iter()
        .map(|e| {
            let color = match e.kind {
                EnemyKind::Normal => {
                    if e.is_flashing() {
                        "#ffffff"
                    } else {
                        "#ff4444"
                    }
                }
                EnemyKind::MiniBoss => {
                    if e.is_flashing() {
                        "#ffffff"
                    } else {
                        "#ff8800"
                    }
                }
                EnemyKind::Boss => {
                    if e.is_flashing() {
                        "#ffffff"
                    } else {
                        "#cc44ff"
                    }
                }
            };
            let sz = e.kind.size();
            let br = if sz > 20.0 { 4 } else { 2 };
            let ent = if e.is_flashing() {
                format!(
                    "{}border-radius:{br}px;filter:brightness(3);",
                    entity_style(e.x, e.y, sz, "#ffffff")
                )
            } else {
                format!(
                    "{}border-radius:{br}px;",
                    entity_style(e.x, e.y, sz, color)
                )
            };
            let hp_bar = if e.hp < e.hp_max {
                Some((hp_bar_outer(e.x, e.y, sz), hp_bar_fill_style(e.hp, e.hp_max)))
            } else {
                None
            };
            (ent, hp_bar)
        })
        .collect();

    // Joueur secondaire
    let sec_style = gs
        .secondary_player
        .as_ref()
        .filter(|s| !s.is_dead())
        .map(|s| {
            format!(
                "{}border:1px solid #66ffff;border-radius:50%;",
                entity_style(s.x, s.y, 10.0, "#00cccc")
            )
        });

    // Projectiles tours
    let proj_styles: Vec<String> = gs
        .tower_projectiles
        .iter()
        .map(|p| {
            format!(
                "{}border-radius:50%;z-index:6;",
                entity_style(p.x, p.y, 3.0, "#cccccc")
            )
        })
        .collect();

    // Projectiles secondaires
    let sec_proj_styles: Vec<String> = gs
        .secondary_projectiles
        .iter()
        .map(|p| {
            format!(
                "{}border-radius:50%;z-index:6;",
                entity_style(p.x, p.y, 4.0, "#00ffff")
            )
        })
        .collect();

    (
        loot_styles,
        tower_styles,
        troop_styles,
        enemy_styles,
        sec_style,
        proj_styles,
        sec_proj_styles,
    )
}
