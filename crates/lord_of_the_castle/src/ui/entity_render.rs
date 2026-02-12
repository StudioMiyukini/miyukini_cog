//! Fonctions de rendu des entités de jeu.
//!
//! Ce module fournit des composants réutilisables pour le rendu
//! des différentes entités : loot, château, tours, troupes, ennemis, joueur, projectiles.
//!
//! @id: lord_of_the_castle.ui.entity_render
//! @do: render_game_entities
//! @role: ui
//! @layer: ui
//! @human: Composants de rendu des entités de jeu (ennemis, tours, joueur, etc.).

use crate::enemies::EnemyKind;
use crate::loot::LootKind;
use crate::styles::{entity_style, hp_bar_fill_style, hp_bar_outer};
use dioxus::prelude::*;

// ─── Types de données de rendu ────────────────────────────────────────

/// Données de rendu pour un loot au sol.
#[derive(Clone, PartialEq)]
pub struct LootRenderData {
    pub x: f32,
    pub y: f32,
    pub kind: LootKind,
}

/// Données de rendu pour le château.
#[derive(Clone, PartialEq)]
pub struct CastleRenderData {
    pub x: f32,
    pub y: f32,
    pub hp: i32,
    pub hp_max: i32,
}

/// Données de rendu pour une tour.
#[derive(Clone, PartialEq)]
pub struct TowerRenderData {
    pub x: f32,
    pub y: f32,
    pub hp: i32,
    pub hp_max: i32,
}

/// Données de rendu pour une troupe.
#[derive(Clone, PartialEq)]
pub struct TroopRenderData {
    pub x: f32,
    pub y: f32,
    pub follows_secondary: bool,
}

/// Données de rendu pour un ennemi.
#[derive(Clone, PartialEq)]
pub struct EnemyRenderData {
    pub x: f32,
    pub y: f32,
    pub hp: i32,
    pub hp_max: i32,
    pub kind: EnemyKind,
    pub is_flashing: bool,
}

/// Données de rendu pour le joueur.
#[derive(Clone, PartialEq)]
pub struct PlayerRenderData {
    pub x: f32,
    pub y: f32,
    pub is_dead: bool,
}

/// Données de rendu pour un projectile.
#[derive(Clone, PartialEq)]
pub struct ProjectileRenderData {
    pub x: f32,
    pub y: f32,
}

// ─── Composants de rendu ──────────────────────────────────────────────

/// Rendu d'un loot au sol.
#[component]
pub fn render_loot(data: LootRenderData) -> Element {
    let color = match data.kind {
        LootKind::Gold(_) => "#ffcc00",
        LootKind::Xp(_) => "#4488ff",
        LootKind::Item(_) => "#aa6633",
    };
    let style = entity_style(data.x, data.y, 5.0, color);
    rsx! {
        div { style: "{style}" }
    }
}

/// Rendu du château.
#[component]
pub fn render_castle(data: CastleRenderData) -> Element {
    let castle_style = format!(
        "{}border:2px solid #8899aa;border-radius:3px;",
        entity_style(data.x, data.y, 40.0, "#667788")
    );
    let hp_outer = hp_bar_outer(data.x, data.y, 40.0);
    let hp_fill = hp_bar_fill_style(data.hp, data.hp_max);

    rsx! {
        div { style: "{castle_style}" }
        div {
            style: "{hp_outer}",
            div { style: "{hp_fill}" }
        }
    }
}

/// Rendu d'une tour.
#[component]
pub fn render_towers(towers: Vec<TowerRenderData>) -> Element {
    rsx! {
        for tower in towers.iter() {
            {
                let tower_style = format!(
                    "{}border:1px solid #aa8833;border-radius:2px;",
                    entity_style(tower.x, tower.y, 40.0, "#8b6914")
                );
                let hp_outer = hp_bar_outer(tower.x, tower.y, 40.0);
                let hp_fill = hp_bar_fill_style(tower.hp, tower.hp_max);
                rsx! {
                    div { style: "{tower_style}" }
                    div {
                        style: "{hp_outer}",
                        div { style: "{hp_fill}" }
                    }
                }
            }
        }
    }
}

/// Rendu des troupes.
#[component]
pub fn render_troops(troops: Vec<TroopRenderData>) -> Element {
    rsx! {
        for troop in troops.iter() {
            {
                let color = if troop.follows_secondary { "#22aa88" } else { "#44cc44" };
                let style = format!("{}border-radius:50%;", entity_style(troop.x, troop.y, 10.0, color));
                rsx! {
                    div { style: "{style}" }
                }
            }
        }
    }
}

/// Rendu des ennemis.
#[component]
pub fn render_enemies(enemies: Vec<EnemyRenderData>) -> Element {
    rsx! {
        for enemy in enemies.iter() {
            {
                let size = enemy.kind.size();
                let color = match enemy.kind {
                    EnemyKind::Normal => if enemy.is_flashing { "#ffffff" } else { "#ff4444" },
                    EnemyKind::MiniBoss => if enemy.is_flashing { "#ffffff" } else { "#ff8800" },
                    EnemyKind::Boss => if enemy.is_flashing { "#ffffff" } else { "#cc44ff" },
                };
                let border_radius = if size > 20.0 { 4 } else { 2 };
                let ent_style = if enemy.is_flashing {
                    format!(
                        "{}border-radius:{border_radius}px;filter:brightness(3);",
                        entity_style(enemy.x, enemy.y, size, "#ffffff")
                    )
                } else {
                    format!(
                        "{}border-radius:{border_radius}px;",
                        entity_style(enemy.x, enemy.y, size, color)
                    )
                };
                let show_hp_bar = enemy.hp < enemy.hp_max;
                rsx! {
                    div { style: "{ent_style}" }
                    if show_hp_bar {
                        {
                            let hp_outer = hp_bar_outer(enemy.x, enemy.y, size);
                            let hp_fill = hp_bar_fill_style(enemy.hp, enemy.hp_max);
                            rsx! {
                                div {
                                    style: "{hp_outer}",
                                    div { style: "{hp_fill}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Rendu du joueur.
#[component]
pub fn render_player(data: PlayerRenderData) -> Element {
    let style = if !data.is_dead {
        format!(
            "{}border:2px solid #88bbff;border-radius:50%;z-index:5;",
            entity_style(data.x, data.y, 15.0, "#4488ff")
        )
    } else {
        format!(
            "{}border:2px solid #666688;border-radius:50%;z-index:5;opacity:0.5;",
            entity_style(data.x, data.y, 15.0, "#444466")
        )
    };

    rsx! {
        div { style: "{style}" }
    }
}

/// Rendu du joueur secondaire.
#[component]
pub fn render_secondary_player(x: f32, y: f32) -> Element {
    let style = format!(
        "{}border:1px solid #66ffff;border-radius:50%;",
        entity_style(x, y, 10.0, "#00cccc")
    );

    rsx! {
        div { style: "{style}" }
    }
}

/// Rendu des projectiles de tours.
#[component]
pub fn render_projectiles(projectiles: Vec<ProjectileRenderData>) -> Element {
    rsx! {
        for proj in projectiles.iter() {
            {
                let style = format!(
                    "{}border-radius:50%;z-index:6;",
                    entity_style(proj.x, proj.y, 3.0, "#cccccc")
                );
                rsx! {
                    div { style: "{style}" }
                }
            }
        }
    }
}

/// Rendu des projectiles secondaires (homing).
#[component]
pub fn render_secondary_projectiles(projectiles: Vec<ProjectileRenderData>) -> Element {
    rsx! {
        for proj in projectiles.iter() {
            {
                let style = format!(
                    "{}border-radius:50%;z-index:6;",
                    entity_style(proj.x, proj.y, 4.0, "#00ffff")
                );
                rsx! {
                    div { style: "{style}" }
                }
            }
        }
    }
}

// ─── Fonctions utilitaires ────────────────────────────────────────────

/// Convertit les données de GameState en données de rendu pour le loot.
pub fn extract_loot_render_data(
    loot_drops: &[crate::loot::LootDrop],
) -> Vec<LootRenderData> {
    loot_drops
        .iter()
        .map(|d| LootRenderData {
            x: d.x,
            y: d.y,
            kind: d.kind.clone(),
        })
        .collect()
}

/// Convertit les données de GameState en données de rendu pour les tours.
pub fn extract_tower_render_data(
    towers: &[crate::towers::Tower],
) -> Vec<TowerRenderData> {
    towers
        .iter()
        .filter(|t| t.hp > 0)
        .map(|t| TowerRenderData {
            x: t.x,
            y: t.y,
            hp: t.hp,
            hp_max: t.hp_max,
        })
        .collect()
}

/// Convertit les données de GameState en données de rendu pour les troupes.
pub fn extract_troop_render_data(
    troops: &[crate::troops::Troop],
) -> Vec<TroopRenderData> {
    troops
        .iter()
        .filter(|t| t.is_alive())
        .map(|t| TroopRenderData {
            x: t.x,
            y: t.y,
            follows_secondary: t.follow_secondary_id.is_some(),
        })
        .collect()
}

/// Convertit les données de GameState en données de rendu pour les ennemis.
pub fn extract_enemy_render_data(
    enemies: &[crate::enemies::Enemy],
) -> Vec<EnemyRenderData> {
    enemies
        .iter()
        .map(|e| EnemyRenderData {
            x: e.x,
            y: e.y,
            hp: e.hp,
            hp_max: e.hp_max,
            kind: e.kind,
            is_flashing: e.is_flashing(),
        })
        .collect()
}

/// Convertit les données de GameState en données de rendu pour les projectiles.
pub fn extract_projectile_render_data(
    projectiles: &[crate::towers::TowerProjectile],
) -> Vec<ProjectileRenderData> {
    projectiles
        .iter()
        .map(|p| ProjectileRenderData { x: p.x, y: p.y })
        .collect()
}

/// Convertit les données de GameState en données de rendu pour les projectiles secondaires.
pub fn extract_secondary_projectile_render_data(
    projectiles: &[crate::secondary_player::SecondaryProjectile],
) -> Vec<ProjectileRenderData> {
    projectiles
        .iter()
        .map(|p| ProjectileRenderData { x: p.x, y: p.y })
        .collect()
}
