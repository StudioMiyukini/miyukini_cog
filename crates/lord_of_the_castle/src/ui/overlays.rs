//! Panneaux overlay et overlays de fin de partie.
//!
//! Ce module contient :
//! - `Panel` : enum des types de panneaux
//! - `StatsPanel` : panneau de statistiques du joueur
//! - `SkillsPanel` : panneau de compétences guerrier
//! - `InventoryPanel` : panneau d'inventaire
//! - `GameOverOverlay` : overlay de game over
//! - `WaveWonOverlay` : overlay de vague gagnée
//!
//! @id: lord_of_the_castle.ui.overlays
//! @do: render_overlay_panels
//! @role: ui
//! @layer: ui
//! @human: Panneaux overlay (Stats, Skills, Inventaire) et écrans de fin.

use crate::app::Screen;
use crate::character_creation::{CharacterStats, Stat};
use crate::game_state::GameState;
use crate::loot::ItemSlot;
use crate::warrior_skills::{warrior_skill_def, WarriorSkillId};
use dioxus::prelude::*;
use std::collections::HashMap;

// ─── Types de panneaux ────────────────────────────────────────────────

/// Panneaux overlay (fenêtres type UO / Mortal Online).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Panel {
    Stats,
    Skills,
    Inventory,
}

/// Bouton pour ouvrir/fermer un panneau.
pub fn panel_button(
    label: &str,
    panel: Panel,
    mut active: Signal<Option<Panel>>,
    dim: bool,
) -> Element {
    let is_active = active() == Some(panel);
    let bg = if is_active { "#2a4a6a" } else { "#232f3e" };
    let color = if dim && !is_active {
        "#556677"
    } else if is_active {
        "#1a9fff"
    } else {
        "#c6d4df"
    };
    let fw = if is_active { "600" } else { "400" };
    rsx! {
        button {
            style: "padding:3px 10px;background:{bg};color:{color};border:1px solid #2a3f5f;border-radius:3px;cursor:pointer;font-size:11px;font-weight:{fw};",
            onclick: move |_| {
                if active() == Some(panel) {
                    active.set(None);
                } else {
                    active.set(Some(panel));
                }
            },
            "{label}"
        }
    }
}

// ─── Panneau Overlay conteneur ────────────────────────────────────────

/// Props pour le conteneur de panneau overlay.
#[derive(Props, Clone, PartialEq)]
pub struct OverlayPanelProps {
    pub active_panel: Signal<Option<Panel>>,
    pub panel: Panel,
    pub title: &'static str,
    pub children: Element,
}

/// Conteneur générique pour les panneaux overlay.
#[component]
pub fn OverlayPanel(mut props: OverlayPanelProps) -> Element {
    rsx! {
        div {
            style: "position:absolute;top:68px;left:50%;transform:translateX(-50%);width:520px;max-height:calc(100vh - 150px);background:#1b2838;border:1px solid #2a3f5f;border-radius:6px;box-shadow:0 8px 32px rgba(0,0,0,0.6);z-index:50;display:flex;flex-direction:column;overflow:hidden;",

            // Barre titre
            div {
                style: "display:flex;align-items:center;justify-content:space-between;padding:8px 12px;background:#232f3e;border-bottom:1px solid #2a3f5f;flex-shrink:0;",
                span {
                    style: "font-size:14px;font-weight:600;color:#1a9fff;",
                    "{props.title}"
                }
                button {
                    style: "padding:2px 8px;background:#3a2222;color:#cc6666;border:1px solid #4a3333;border-radius:3px;cursor:pointer;font-size:12px;",
                    onclick: move |_| props.active_panel.set(None),
                    "✕"
                }
            }

            // Contenu du panneau
            div {
                style: "flex:1;overflow-y:auto;padding:10px;",
                {props.children}
            }
        }
    }
}

// ─── Panneau Stats ────────────────────────────────────────────────────

/// Props pour StatsPanel.
#[derive(Props, Clone, PartialEq)]
pub struct StatsPanelProps {
    pub game_state: Signal<Option<GameState>>,
    pub base_stats: CharacterStats,
    pub eff_stats: CharacterStats,
    pub stat_pts: u32,
    pub player_hp: i32,
    pub player_hp_max: i32,
    pub armor: i32,
    pub atk_dmg: i32,
    pub castle_hp: i32,
    pub castle_hp_max: i32,
    pub enemies_killed: u32,
    pub bosses_killed: u32,
    pub gold_total: u32,
    pub max_wave: u32,
}

/// Panneau de statistiques du joueur.
#[component]
pub fn StatsPanel(mut props: StatsPanelProps) -> Element {
    let stat_list: Vec<(&str, &str, i32, i32, Stat)> = vec![
        ("For", "Force", props.base_stats.for_, props.eff_stats.for_, Stat::For),
        ("Con", "Constitution", props.base_stats.con, props.eff_stats.con, Stat::Con),
        ("Agi", "Agilité", props.base_stats.agi, props.eff_stats.agi, Stat::Agi),
        ("Dex", "Dextérité", props.base_stats.dex, props.eff_stats.dex, Stat::Dex),
        ("Int", "Intelligence", props.base_stats.int, props.eff_stats.int, Stat::Int),
        ("Sag", "Sagesse", props.base_stats.sag, props.eff_stats.sag, Stat::Sag),
        ("Cha", "Charisme", props.base_stats.cha, props.eff_stats.cha, Stat::Cha),
        ("Luk", "Chance", props.base_stats.luk, props.eff_stats.luk, Stat::Luk),
    ];

    rsx! {
        // Statistiques joueur
        h3 { style: "font-size:13px;color:#1a9fff;margin-bottom:6px;", "Joueur" }
        div {
            style: "display:grid;grid-template-columns:60px 1fr 40px 40px 32px;gap:2px 8px;font-size:11px;margin-bottom:12px;",
            span { style: "color:#8f98a0;font-weight:600;", "Stat" }
            span { style: "color:#8f98a0;", "" }
            span { style: "color:#8f98a0;text-align:right;", "Base" }
            span { style: "color:#8f98a0;text-align:right;", "Eff." }
            span { }
            for (short, long, base, eff, stat) in stat_list.into_iter() {
                span { style: "color:#c6d4df;font-weight:600;", "{short}" }
                span { style: "color:#8f98a0;", "{long}" }
                span { style: "color:#c6d4df;text-align:right;", "{base}" }
                span { style: "color:#88ccff;text-align:right;font-weight:600;", "{eff}" }
                if props.stat_pts > 0 {
                    button {
                        style: "padding:1px 4px;background:#2a4a3a;color:#88cc88;border:1px solid #3a5a4a;border-radius:2px;cursor:pointer;font-size:10px;",
                        onclick: move |_| {
                            if let Some(gs) = props.game_state.write().as_mut() {
                                gs.spend_stat_point(stat);
                            }
                        },
                        "+"
                    }
                } else {
                    span { }
                }
            }
        }

        // Combat
        h3 { style: "font-size:13px;color:#1a9fff;margin-bottom:6px;", "Combat" }
        div {
            style: "font-size:11px;display:grid;grid-template-columns:1fr 1fr;gap:2px 16px;margin-bottom:12px;",
            span { style: "color:#8f98a0;", "PV" }
            span { style: "color:#c6d4df;", "{props.player_hp} / {props.player_hp_max}" }
            span { style: "color:#8f98a0;", "Armure" }
            span { style: "color:#c6d4df;", "{props.armor}" }
            span { style: "color:#8f98a0;", "Dégâts auto" }
            span { style: "color:#c6d4df;", "{props.atk_dmg}" }
        }

        // Château
        h3 { style: "font-size:13px;color:#1a9fff;margin-bottom:6px;", "Château" }
        div {
            style: "font-size:11px;display:grid;grid-template-columns:1fr 1fr;gap:2px 16px;margin-bottom:12px;",
            span { style: "color:#8f98a0;", "PV" }
            span { style: "color:#c6d4df;", "{props.castle_hp} / {props.castle_hp_max}" }
        }

        // Run
        h3 { style: "font-size:13px;color:#1a9fff;margin-bottom:6px;", "Run" }
        div {
            style: "font-size:11px;display:grid;grid-template-columns:1fr 1fr;gap:2px 16px;",
            span { style: "color:#8f98a0;", "Ennemis tués" }
            span { style: "color:#c6d4df;", "{props.enemies_killed}" }
            span { style: "color:#8f98a0;", "Boss tués" }
            span { style: "color:#c6d4df;", "{props.bosses_killed}" }
            span { style: "color:#8f98a0;", "Or total" }
            span { style: "color:#ffcc00;", "{props.gold_total}" }
            span { style: "color:#8f98a0;", "Vague max" }
            span { style: "color:#c6d4df;", "{props.max_wave}" }
        }
    }
}

// ─── Panneau Skills ───────────────────────────────────────────────────

/// Props pour SkillsPanel.
#[derive(Props, Clone, PartialEq)]
pub struct SkillsPanelProps {
    pub game_state: Signal<Option<GameState>>,
    pub skill_ranks: HashMap<WarriorSkillId, u32>,
    pub skill_pts: u32,
}

/// Panneau de compétences guerrier.
#[component]
pub fn SkillsPanel(mut props: SkillsPanelProps) -> Element {
    let all_skills: Vec<(WarriorSkillId, &str, &str, u32, u32, bool)> = WarriorSkillId::all()
        .iter()
        .map(|&id| {
            let def = warrior_skill_def(id);
            let current = props.skill_ranks.get(&id).copied().unwrap_or(0);
            let can_learn = props.skill_pts > 0
                && current < def.max_rank
                && crate::warrior_skills::prerequisites_met(&props.skill_ranks, &def);
            (id, def.name, def.effect_description, current, def.max_rank, can_learn)
        })
        .collect();

    rsx! {
        if props.skill_pts > 0 {
            p { style: "font-size:12px;color:#ffcc00;margin-bottom:8px;", "Points disponibles : {props.skill_pts}" }
        }
        div {
            style: "display:flex;flex-direction:column;gap:4px;",
            for (id, name, effect, current, max_rank, can_learn) in all_skills.into_iter() {
                {
                    let name_color = if current > 0 { "#c6d4df" } else { "#667788" };
                    let rank_color = if current >= max_rank { "#5ba32b" } else if current > 0 { "#88aacc" } else { "#556677" };
                    rsx! {
                        div {
                            style: "display:flex;align-items:center;gap:8px;padding:4px 6px;background:#232f3e;border-radius:3px;",
                            div {
                                style: "flex:1;",
                                div {
                                    style: "display:flex;align-items:center;gap:6px;",
                                    span {
                                        style: "font-size:12px;color:{name_color};font-weight:600;",
                                        "{name}"
                                    }
                                    span {
                                        style: "font-size:10px;color:{rank_color};",
                                        "{current}/{max_rank}"
                                    }
                                }
                                p {
                                    style: "font-size:10px;color:#8f98a0;margin-top:1px;",
                                    "{effect}"
                                }
                            }
                            if can_learn {
                                button {
                                    style: "padding:3px 8px;background:#2a4a3a;color:#88cc88;border:1px solid #3a5a4a;border-radius:3px;cursor:pointer;font-size:10px;flex-shrink:0;",
                                    onclick: move |_| {
                                        if let Some(gs) = props.game_state.write().as_mut() {
                                            gs.learn_warrior_skill(id);
                                        }
                                    },
                                    "+1"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ─── Panneau Inventaire ───────────────────────────────────────────────

/// Props pour InventoryPanel.
#[derive(Props, Clone, PartialEq)]
pub struct InventoryPanelProps {
    pub game_state: Signal<Option<GameState>>,
    pub inventory: Vec<(usize, String)>,
    pub equipped: Vec<(ItemSlot, String)>,
    pub gold: u32,
}

/// Panneau d'inventaire.
#[component]
pub fn InventoryPanel(mut props: InventoryPanelProps) -> Element {
    rsx! {
        // Équipement
        h3 { style: "font-size:13px;color:#1a9fff;margin-bottom:6px;", "Équipement" }
        div {
            style: "display:grid;grid-template-columns:90px 1fr;gap:2px 8px;font-size:11px;margin-bottom:12px;",
            for (slot, label) in props.equipped.iter() {
                span { style: "color:#8f98a0;", "{slot.label()}" }
                span { style: "color:#c6d4df;", "{label}" }
            }
        }

        // Inventaire (sac)
        {
            let inv_count = props.inventory.len();
            rsx! {
                h3 { style: "font-size:13px;color:#1a9fff;margin-bottom:6px;",
                    "Sac ({inv_count}/20)"
                }
            }
        }
        if props.inventory.is_empty() {
            p { style: "font-size:11px;color:#556677;", "Inventaire vide." }
        }
        div {
            style: "display:flex;flex-direction:column;gap:3px;",
            for (idx, label) in props.inventory.iter() {
                div {
                    style: "display:flex;align-items:center;gap:6px;padding:3px 6px;background:#232f3e;border-radius:3px;",
                    span { style: "flex:1;font-size:11px;color:#c6d4df;", "{label}" }
                    // Boutons d'action
                    {
                        let idx_copy = *idx;
                        let is_unidentified = label.starts_with("?");
                        rsx! {
                            if is_unidentified {
                                button {
                                    style: "padding:2px 6px;background:#2a3a4a;color:#88aacc;border:1px solid #3a4a5a;border-radius:2px;cursor:pointer;font-size:9px;",
                                    onclick: move |_| {
                                        if let Some(gs) = props.game_state.write().as_mut() {
                                            gs.identify_self(idx_copy);
                                        }
                                    },
                                    "ID soi"
                                }
                                button {
                                    style: "padding:2px 6px;background:#2a3a4a;color:#ffcc00;border:1px solid #3a4a5a;border-radius:2px;cursor:pointer;font-size:9px;",
                                    onclick: move |_| {
                                        if let Some(gs) = props.game_state.write().as_mut() {
                                            gs.identify_expert(idx_copy);
                                        }
                                    },
                                    "ID 20g"
                                }
                            } else {
                                button {
                                    style: "padding:2px 6px;background:#2a4a3a;color:#88cc88;border:1px solid #3a5a4a;border-radius:2px;cursor:pointer;font-size:9px;",
                                    onclick: move |_| {
                                        if let Some(gs) = props.game_state.write().as_mut() {
                                            gs.equip_item(idx_copy);
                                        }
                                    },
                                    "Équiper"
                                }
                                button {
                                    style: "padding:2px 6px;background:#4a3a2a;color:#ccaa88;border:1px solid #5a4a3a;border-radius:2px;cursor:pointer;font-size:9px;",
                                    onclick: move |_| {
                                        if let Some(gs) = props.game_state.write().as_mut() {
                                            gs.sell_item(idx_copy);
                                        }
                                    },
                                    "Vendre"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ─── Overlays de fin de partie ────────────────────────────────────────

/// Props pour WaveWonOverlay (alias VictoryOverlay).
#[derive(Props, Clone, PartialEq)]
pub struct WaveWonOverlayProps {
    pub wave: u32,
    pub enemies_killed: u32,
}

/// Overlay de vague gagnée.
#[component]
pub fn WaveWonOverlay(props: WaveWonOverlayProps) -> Element {
    rsx! {
        div {
            style: "position:absolute;top:50%;left:50%;transform:translate(-50%,-50%);background:#1b3828;border:2px solid #2a5a3f;border-radius:8px;padding:20px 32px;z-index:80;text-align:center;",
            h2 { style: "font-size:22px;color:#5ba32b;margin-bottom:8px;", "Vague {props.wave} terminée !" }
            p { style: "font-size:13px;color:#c6d4df;margin-bottom:4px;",
                "Ennemis tués : {props.enemies_killed}"
            }
            p { style: "font-size:12px;color:#8f98a0;",
                "Passez en préparation pour construire et recruter."
            }
        }
    }
}

/// Props pour GameOverOverlay (alias DefeatOverlay).
#[derive(Props, Clone, PartialEq)]
pub struct GameOverOverlayProps {
    pub screen: Signal<Screen>,
    pub game_state: Signal<Option<GameState>>,
    pub wave: u32,
    pub enemies_killed: u32,
    pub bosses_killed: u32,
    pub gold_total: u32,
}

/// Overlay de game over.
#[component]
pub fn GameOverOverlay(mut props: GameOverOverlayProps) -> Element {
    rsx! {
        div {
            style: "position:absolute;inset:0;background:rgba(0,0,0,0.8);display:flex;flex-direction:column;align-items:center;justify-content:center;z-index:100;",
            h1 { style: "font-size:36px;color:#cc2222;margin-bottom:12px;", "GAME OVER" }
            p { style: "font-size:16px;color:#c6d4df;margin-bottom:8px;",
                "Le château a été détruit à la vague {props.wave}."
            }
            p { style: "font-size:14px;color:#8f98a0;margin-bottom:24px;",
                "Ennemis tués : {props.enemies_killed} • Boss : {props.bosses_killed} • Or total : {props.gold_total}"
            }
            button {
                style: "padding:12px 36px;background:#1a9fff;color:white;border:none;border-radius:6px;font-size:16px;font-weight:600;cursor:pointer;",
                onclick: move |_| {
                    props.game_state.set(None);
                    props.screen.set(Screen::MainMenu);
                },
                "Retour au menu"
            }
        }
    }
}

// ─── Notifications ────────────────────────────────────────────────────

/// Props pour NotificationsOverlay.
#[derive(Props, Clone, PartialEq)]
pub struct NotificationsOverlayProps {
    pub notifications: Vec<String>,
}

/// Overlay de notifications (level up, etc.).
#[component]
pub fn NotificationsOverlay(props: NotificationsOverlayProps) -> Element {
    if props.notifications.is_empty() {
        return rsx! {};
    }

    rsx! {
        div {
            style: "position:absolute;bottom:50px;right:16px;display:flex;flex-direction:column;gap:4px;z-index:60;pointer-events:none;",
            for notif in props.notifications.iter() {
                div {
                    style: "padding:6px 12px;background:#2a3f1e;border:1px solid #4a5f3e;border-radius:4px;font-size:12px;color:#88cc44;",
                    "{notif}"
                }
            }
        }
    }
}
