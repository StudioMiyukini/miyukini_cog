//! Application Dioxus Lord of the Castle (Miyukini Survivor).
//! Point d'entrée UI : menu principal (slots KM), écran de jeu complet avec
//! rendu des entités, barres d'info, sidebar préparation, panneaux overlay.
//!
//! Layout sans défilement : tout rentre dans 100vh.
//! ┌─────────────────────────────────────────────────────────────┐
//! │  Header (vague, ennemis, or, niveau)                        │
//! ├─────────────────────────────────────────────────────────────┤
//! │  Boutons (Stats, Skills, Inventaire)  │  XP bar             │
//! ├───────────────────────────────┬─────────────────────────────┤
//! │                               │  Sidebar (préparation)      │
//! │   Zone de jeu (entités)       │  - Construire tour          │
//! │                               │  - Recruter troupe          │
//! │                               │  - Marchand                 │
//! ├───────────────────────────────┴─────────────────────────────┤
//! │  Footer (PV joueur, sauvegarde, actions)                    │
//! └─────────────────────────────────────────────────────────────┘

use crate::character_creation::{
    all_phrases, apply_phrase_effects, pick_three_phrases, CharacterStats, PhraseDef,
};
use crate::constants::{COMBAT_SURFACE_SIZE, TOWER_BASE_COST_GOLD};
use crate::enemies::EnemyKind;
use crate::game_loop::{move_player, move_secondary_player, tick_battle};
use crate::game_state::{rand_simple, GamePhase, GameState};
use crate::loot::{InventoryEntry, ItemSlot, LootKind};
use crate::player::{Dir8, Player};
use crate::save::{LordOfTheCastleDb, SlotMetadata};
use crate::embedded_sprites::{
    animation_frame_index, player, enemy_normal, enemy_miniboss, enemy_boss,
};
use crate::troops::TroopKind;
use crate::warrior_skills::{warrior_skill_def, WarriorSkillId};
use dioxus::prelude::*;
use std::path::PathBuf;
use std::time::{Duration, Instant};

// ─── Types ────────────────────────────────────────────────────────────

/// Écran actif de l'app (menu, création, jeu).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Screen {
    #[default]
    MainMenu,
    CharacterCreation,
    Game,
}

/// Panneaux overlay (fenêtres type UO / Mortal Online).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Panel {
    Stats,
    Skills,
    Inventory,
}

/// Surface de rendu (800×800 unités de jeu).
const SURFACE: f32 = COMBAT_SURFACE_SIZE;

// ─── Helpers ──────────────────────────────────────────────────────────

fn get_base_path() -> PathBuf {
    std::env::var("MIYUKINI_DATA_DIR")
        .ok()
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_default())
}

/// Position en pixels (zone 800×800 : 1 unité monde = 1 px).
fn px(v: f32) -> f32 {
    v
}

/// Style CSS pour une entité (div positionnée absolument, centrée sur son point).
/// Utilise des tailles en pixels pour cohérence avec background-size des sprites.
fn entity_style(x: f32, y: f32, size: f32, color: &str) -> String {
    format!(
        "position:absolute;left:{xp:.0}px;top:{yp:.0}px;width:{sp:.0}px;height:{sp:.0}px;background:{c};transform:translate(-50%,-50%);pointer-events:none;border-radius:1px;",
        xp = px(x),
        yp = px(y),
        sp = px(size),
        c = color,
    )
}

/// Style CSS pour une barre de PV au-dessus d'une entité.
fn hp_bar_outer(x: f32, y: f32, entity_size: f32) -> String {
    let bar_height = 3.0f32;
    let top_y = y - entity_size / 2.0 - bar_height;
    let bar_width = entity_size.max(12.0);
    format!(
        "position:absolute;left:{xp:.0}px;top:{yp:.0}px;width:{wp:.0}px;height:{hp:.0}px;transform:translateX(-50%);background:#333;pointer-events:none;",
        xp = px(x),
        yp = px(top_y),
        wp = px(bar_width),
        hp = bar_height,
    )
}

fn hp_bar_fill_style(hp: i32, hp_max: i32) -> String {
    let ratio = if hp_max > 0 {
        (hp as f32 / hp_max as f32).clamp(0.0, 1.0)
    } else {
        0.0
    };
    let color = if ratio > 0.6 {
        "#44cc44"
    } else if ratio > 0.3 {
        "#ccaa44"
    } else {
        "#cc4444"
    };
    format!(
        "width:{w:.1}%;height:100%;background:{c};",
        w = ratio * 100.0,
        c = color,
    )
}

/// Cherche la première cellule libre pour construire une tour (spirale autour du château).
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

// ─── Entrées standalone / embarquée ───────────────────────────────────

/// Point d'entrée standalone : fournit le chemin de données puis affiche SurvivorApp.
#[component]
pub fn SurvivorAppStandalone() -> Element {
    use_context_provider(|| Signal::new(get_base_path()));
    rsx! { SurvivorApp {} }
}

/// Position du curseur en coordonnées monde (partagée entre composants).
static CURSOR_WORLD: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

fn set_cursor_world(x: f32, y: f32) {
    let packed = ((x.to_bits() as u64) << 32) | (y.to_bits() as u64);
    CURSOR_WORLD.store(packed, std::sync::atomic::Ordering::Relaxed);
}

fn get_cursor_world() -> Option<(f32, f32)> {
    let packed = CURSOR_WORLD.load(std::sync::atomic::Ordering::Relaxed);
    if packed == 0 {
        return None;
    }
    let x = f32::from_bits((packed >> 32) as u32);
    let y = f32::from_bits(packed as u32);
    Some((x, y))
}

/// Application Dioxus pour Lord of the Castle (Miyukini Survivor).
/// Le contexte `Signal<PathBuf>` doit être fourni par le parent (Central ou SurvivorAppStandalone).
#[component]
pub fn SurvivorApp() -> Element {
    let screen = use_signal(|| Screen::MainMenu);
    let game_state = use_signal(|| Option::<GameState>::None);
    let active_slot = use_signal(|| Option::<u8>::None);

    let _ = use_context::<Signal<PathBuf>>();

    // Boucle de jeu : tick Battle ~60 FPS quand une partie est en cours
    use_effect(move || {
        let mut state_signal = game_state;
        spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_millis(16));
            let mut last = Instant::now();
            loop {
                interval.tick().await;
                let delta = last.elapsed().as_secs_f32();
                last = Instant::now();
                if let Some(gs) = state_signal.write().as_mut() {
                    if gs.phase == GamePhase::Battle {
                        tick_battle(gs, delta.min(0.1), get_cursor_world());
                    }
                }
            }
        });
    });

    let content = match screen() {
        Screen::MainMenu => rsx! { MainMenu { screen, game_state, active_slot } },
        Screen::CharacterCreation => rsx! { CharacterCreationScreen { screen, game_state, active_slot } },
        Screen::Game => rsx! { GameScreen { screen, game_state, active_slot } },
    };

    rsx! {
        style { {GLOBAL_CSS} }
        div {
            style: "width:100%;height:100%;background:#171a21;color:#c6d4df;font-family:'Segoe UI',Arial,sans-serif;display:flex;flex-direction:column;overflow:hidden;",
            {content}
        }
    }
}

const GLOBAL_CSS: &str = r#"
* { margin: 0; padding: 0; box-sizing: border-box; }
body { overflow: hidden; }
::-webkit-scrollbar { width: 5px; }
::-webkit-scrollbar-track { background: #1b2838; }
::-webkit-scrollbar-thumb { background: #2a3f5f; border-radius: 3px; }
button { font-family: inherit; }
button:hover { filter: brightness(1.15); }
"#;

// ─── Menu principal ───────────────────────────────────────────────────

#[component]
fn MainMenu(screen: Signal<Screen>, game_state: Signal<Option<GameState>>, active_slot: Signal<Option<u8>>) -> Element {
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

// ─── Écran de création de personnage ──────────────────────────────────

/// État interne de la création de personnage.
#[derive(Debug, Clone)]
struct CreationState {
    /// Étape courante (0..3, puis 4 = récap final).
    step: usize,
    /// Stats en cours de construction.
    stats: CharacterStats,
    /// Pool d'indices de phrases encore disponibles.
    available: Vec<usize>,
    /// Les 3 phrases proposées à l'étape courante.
    choices: Vec<PhraseDef>,
    /// Historique : phrases choisies (texte + effets).
    history: Vec<String>,
    /// Nom du personnage (saisi à l'étape finale).
    name: String,
    /// Flag « Que Nawak décide » a été choisi → reroll total en fin.
    reroll_at_end: bool,
    /// Slot de sauvegarde choisi (1, 2 ou 3).
    save_slot: u8,
}

impl CreationState {
    fn new() -> Self {
        let mut available: Vec<usize> = (0..all_phrases().len()).collect();
        let mut roll = || rand_simple();
        let choices = pick_three_phrases(&mut available, &mut roll);
        Self {
            step: 0,
            stats: CharacterStats::default(),
            available,
            choices,
            history: Vec::new(),
            name: String::new(),
            reroll_at_end: false,
            save_slot: 1,
        }
    }
}

#[component]
fn CharacterCreationScreen(screen: Signal<Screen>, game_state: Signal<Option<GameState>>, active_slot: Signal<Option<u8>>) -> Element {
    let mut state = use_signal(CreationState::new);
    let base_path = use_context::<Signal<PathBuf>>();

    let st = state.read().clone();
    let step = st.step;
    let stats = st.stats.clone();
    let history = st.history.clone();
    let choices = st.choices.clone();
    let name = st.name.clone();
    let save_slot = st.save_slot;

    // Titres des étapes
    let step_title = match step {
        0 => "Etape 1/4 — Qui es-tu ?",
        1 => "Etape 2/4 — D'où viens-tu ?",
        2 => "Etape 3/4 — Que sais-tu faire ?",
        3 => "Etape 4/4 — Un dernier mot ?",
        _ => "Récapitulatif",
    };

    rsx! {
        div {
            style: "flex:1;display:flex;flex-direction:column;align-items:center;justify-content:center;gap:16px;padding:24px;",

            h1 {
                style: "font-size:28px;font-weight:700;color:#1a9fff;margin-bottom:4px;",
                "Création de personnage"
            }
            h2 {
                style: "font-size:16px;color:#c6d4df;margin-bottom:12px;",
                "{step_title}"
            }

            if step < 4 {
                // ── Choix de phrase ──
                div {
                    style: "display:flex;flex-direction:column;gap:8px;width:100%;max-width:520px;",
                    for phrase in choices.iter() {
                        {
                            let phrase_text = phrase.text.to_string();
                            let effects = phrase.effects.clone();
                            rsx! {
                                button {
                                    style: "padding:14px 20px;background:#232f3e;color:#c6d4df;border:1px solid #2a3f5f;border-radius:6px;font-size:14px;cursor:pointer;text-align:left;transition:background 0.15s;",
                                    onclick: move |_| {
                                        let mut s = state.write();
                                        let mut roll = || rand_simple();
                                        let reroll = apply_phrase_effects(&mut s.stats, &effects, &mut roll);
                                        if reroll { s.reroll_at_end = true; }
                                        s.history.push(phrase_text.clone());
                                        s.step += 1;
                                        if s.step < 4 {
                                            // Proposer 3 nouvelles phrases
                                            s.choices = pick_three_phrases(&mut s.available, &mut roll);
                                        } else {
                                            // Fin : appliquer reroll si nécessaire
                                            if s.reroll_at_end {
                                                s.stats.reroll_all(&mut roll);
                                            }
                                            s.choices.clear();
                                        }
                                    },
                                    "« {phrase.text} »"
                                }
                            }
                        }
                    }
                }
            } else {
                // ── Récapitulatif final ──
                div {
                    style: "display:flex;gap:24px;width:100%;max-width:600px;",

                    // Colonne gauche : phrases choisies
                    div {
                        style: "flex:1;display:flex;flex-direction:column;gap:8px;",
                        h3 { style: "font-size:14px;color:#1a9fff;margin-bottom:4px;", "Tes choix" }
                        for (i, text) in history.iter().enumerate() {
                            div {
                                style: "padding:8px 12px;background:#232f3e;border-radius:4px;font-size:12px;color:#c6d4df;",
                                span { style: "color:#8f98a0;margin-right:6px;", "{i+1}." }
                                "« {text} »"
                            }
                        }
                    }

                    // Colonne droite : stats finales
                    div {
                        style: "width:200px;display:flex;flex-direction:column;gap:4px;",
                        h3 { style: "font-size:14px;color:#1a9fff;margin-bottom:4px;", "Stats" }
                        { stat_row("For", "Force", stats.for_) }
                        { stat_row("Con", "Constitution", stats.con) }
                        { stat_row("Agi", "Agilité", stats.agi) }
                        { stat_row("Dex", "Dextérité", stats.dex) }
                        { stat_row("Int", "Intelligence", stats.int) }
                        { stat_row("Sag", "Sagesse", stats.sag) }
                        { stat_row("Cha", "Charisme", stats.cha) }
                        { stat_row("Luk", "Chance", stats.luk) }
                    }
                }

                // Nom du personnage + Slot de sauvegarde
                div {
                    style: "display:flex;gap:16px;width:100%;max-width:600px;margin-top:16px;",

                    // Nom
                    div {
                        style: "flex:1;display:flex;flex-direction:column;gap:6px;",
                        label { style: "font-size:13px;color:#8f98a0;", "Nom du personnage :" }
                        input {
                            style: "padding:10px 14px;background:#232f3e;color:#c6d4df;border:1px solid #2a3f5f;border-radius:6px;font-size:15px;outline:none;",
                            r#type: "text",
                            placeholder: "Héros sans nom",
                            value: "{name}",
                            oninput: move |evt: Event<FormData>| {
                                state.write().name = evt.value();
                            },
                        }
                    }

                    // Slot de sauvegarde
                    div {
                        style: "display:flex;flex-direction:column;gap:6px;",
                        label { style: "font-size:13px;color:#8f98a0;", "Emplacement de sauvegarde :" }
                        div {
                            style: "display:flex;gap:6px;",
                            for sid in [1u8, 2, 3] {
                                {
                                    let is_selected = save_slot == sid;
                                    let bg = if is_selected { "#2a5a3a" } else { "#232f3e" };
                                    let border = if is_selected { "#4a8a5a" } else { "#2a3f5f" };
                                    let color = if is_selected { "#88cc88" } else { "#8f98a0" };
                                    rsx! {
                                        button {
                                            style: "padding:8px 16px;background:{bg};color:{color};border:2px solid {border};border-radius:6px;font-size:14px;font-weight:600;cursor:pointer;min-width:48px;",
                                            onclick: move |_| {
                                                state.write().save_slot = sid;
                                            },
                                            "{sid}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Boutons
                div {
                    style: "display:flex;gap:12px;margin-top:16px;",
                    button {
                        style: "padding:10px 28px;background:#232f3e;color:#8f98a0;border:1px solid #2a3f5f;border-radius:6px;font-size:14px;cursor:pointer;",
                        onclick: move |_| {
                            // Recommencer la création
                            state.set(CreationState::new());
                        },
                        "↺ Recommencer"
                    }
                    button {
                        style: "padding:10px 36px;background:linear-gradient(135deg,#5ba32b 0%,#3d8c40 100%);color:white;border:none;border-radius:6px;font-size:16px;font-weight:600;cursor:pointer;",
                        onclick: move |_| {
                            let s = state.read();
                            let char_name = if s.name.trim().is_empty() {
                                "Héros sans nom".to_string()
                            } else {
                                s.name.trim().to_string()
                            };
                            let slot = s.save_slot;
                            let player = Player::from_creation(
                                char_name.clone(),
                                char_name,
                                s.stats.clone(),
                                400.0,
                                400.0,
                            );
                            let gs = GameState::new_with_player(400.0, 400.0, player);
                            let gs_for_save = gs.clone();
                            game_state.set(Some(gs));
                            active_slot.set(Some(slot));
                            // Sauvegarde immédiate dans le slot choisi
                            let path = base_path.read().clone();
                            spawn(async move {
                                let _ = tokio::task::spawn_blocking(move || {
                                    let db = LordOfTheCastleDb::open(path.join("lord_of_the_castle.db"))?;
                                    db.slot_write(slot, &gs_for_save)
                                }).await;
                            });
                            screen.set(Screen::Game);
                        },
                        "⚔ Commencer l'aventure (slot {save_slot})"
                    }
                }
            }

            // Bouton retour
            button {
                style: "margin-top:12px;padding:6px 20px;background:transparent;color:#556677;border:1px solid #2a3f5f;border-radius:4px;font-size:12px;cursor:pointer;",
                onclick: move |_| {
                    screen.set(Screen::MainMenu);
                },
                "← Retour au menu"
            }
        }
    }
}

/// Ligne de stat pour le récapitulatif création.
fn stat_row(short: &str, long: &str, value: i32) -> Element {
    let val_color = if value > 0 { "#88cc88" } else if value < 0 { "#cc6666" } else { "#c6d4df" };
    let display = if value < 0 {
        format!("1({})", value)
    } else {
        value.to_string()
    };
    rsx! {
        div {
            style: "display:flex;justify-content:space-between;padding:3px 8px;background:#232f3e;border-radius:3px;font-size:12px;",
            span { style: "color:#8f98a0;",
                span { style: "color:#c6d4df;font-weight:600;margin-right:4px;", "{short}" }
                "{long}"
            }
            span { style: "color:{val_color};font-weight:600;", "{display}" }
        }
    }
}

// ─── Écran de jeu ─────────────────────────────────────────────────────

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

#[component]
fn GameScreen(screen: Signal<Screen>, game_state: Signal<Option<GameState>>, active_slot: Signal<Option<u8>>) -> Element {
    let mut active_panel = use_signal(|| Option::<Panel>::None);
    let mut dev_mode = use_signal(|| false);
    let mut build_mode = use_signal(|| false); // Mode construction RTS
    let mut cursor_angle = use_signal(|| 0.0f32); // Angle du curseur par rapport au joueur (en degrés)
    let mut held_keys = use_signal(HeldKeys::default);
    let mut show_click_hint = use_signal(|| false);
    let mut last_wave_hint = use_signal(|| 0u32);

    // Boucle de mouvement fluide basée sur les touches maintenues
    use_effect(move || {
        let keys_signal = held_keys;
        spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_millis(16));
            loop {
                interval.tick().await;
                let keys = *keys_signal.read();
                if let Some(gs) = game_state.write().as_mut() {
                    // Joueur 1
                    if keys.up && !keys.down {
                        move_player(gs, Dir8::N, 0.016);
                    } else if keys.down && !keys.up {
                        move_player(gs, Dir8::S, 0.016);
                    }
                    if keys.left && !keys.right {
                        move_player(gs, Dir8::W, 0.016);
                    } else if keys.right && !keys.left {
                        move_player(gs, Dir8::E, 0.016);
                    }
                    // Joueur 2
                    if keys.up2 && !keys.down2 {
                        move_secondary_player(gs, Dir8::N, 0.016);
                    } else if keys.down2 && !keys.up2 {
                        move_secondary_player(gs, Dir8::S, 0.016);
                    }
                    if keys.left2 && !keys.right2 {
                        move_secondary_player(gs, Dir8::W, 0.016);
                    } else if keys.right2 && !keys.left2 {
                        move_secondary_player(gs, Dir8::E, 0.016);
                    }
                }
            }
        });
    });

    let gs = game_state.read().clone();
    let Some(ref gs) = gs else {
        return rsx! { div { "Aucune partie." } };
    };

    let base_path = use_context::<Signal<PathBuf>>();
    let phase = gs.phase;
    let wave = gs.wave_number;

    // Afficher notification "cliquer sur la zone" pour vagues 1, 2, 3 en phase Battle
    if phase == GamePhase::Battle && wave <= 3 && last_wave_hint() != wave {
        last_wave_hint.set(wave);
        show_click_hint.set(true);
        spawn(async move {
            tokio::time::sleep(Duration::from_secs(3)).await;
            show_click_hint.set(false);
        });
    }
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
    let enemies_alive = gs.enemies.len();
    let _enemies_spawned = gs.enemies_spawned_this_wave;
    let spawn_qty = gs.spawn_quantity;
    let castle_hp = gs.castle.hp;
    let castle_hp_max = gs.castle.hp_max;
    let skill_pts = gs.skill_points_available;
    let stat_pts = gs.stat_points_available;
    let towers_count = gs.towers.len();
    let troops_count = gs.troops.iter().filter(|t| t.is_active_in_squad()).count();
    let max_troops = gs.max_troops();
    let armor = gs.player_total_armor();
    let atk_dmg = gs.player_auto_attack_damage();
    let inv_count = gs.inventory.len();
    let phase_label = match phase {
        GamePhase::Preparation => "Préparation",
        GamePhase::Battle => "Bataille",
    };

    // ─── Données de rendu des entités ───
    let castle_x = gs.castle.x;
    let castle_y = gs.castle.y;

    // Ennemis (x, y, size, color, hp, hp_max, kind, is_flashing)
    let enemy_renders: Vec<(f32, f32, f32, String, i32, i32, EnemyKind, bool)> = gs
        .enemies
        .iter()
        .map(|e| {
            let color = match e.kind {
                EnemyKind::Normal => {
                    if e.is_flashing() { "#ffffff".to_string() } else { "#ff4444".to_string() }
                }
                EnemyKind::MiniBoss => {
                    if e.is_flashing() { "#ffffff".to_string() } else { "#ff8800".to_string() }
                }
                EnemyKind::Boss => {
                    if e.is_flashing() { "#ffffff".to_string() } else { "#cc44ff".to_string() }
                }
            };
            (e.x, e.y, e.kind.size(), color, e.hp, e.hp_max, e.kind, e.is_flashing())
        })
        .collect();

    // Tours
    let tower_renders: Vec<(f32, f32, i32, i32)> = gs
        .towers
        .iter()
        .filter(|t| t.hp > 0)
        .map(|t| (t.x, t.y, t.hp, t.hp_max))
        .collect();

    // Projectiles tours
    let proj_renders: Vec<(f32, f32)> = gs
        .tower_projectiles
        .iter()
        .map(|p| (p.x, p.y))
        .collect();

    // Loot au sol
    let loot_renders: Vec<(f32, f32, String)> = gs
        .loot_drops
        .iter()
        .map(|d| {
            let c = match d.kind {
                LootKind::Gold(_) => "#ffcc00",
                LootKind::Xp(_) => "#4488ff",
                LootKind::Item(_) => "#aa6633",
            };
            (d.x, d.y, c.to_string())
        })
        .collect();

    // Troupes
    let troop_renders: Vec<(f32, f32, bool)> = gs
        .troops
        .iter()
        .filter(|t| t.is_alive())
        .map(|t| (t.x, t.y, t.follow_secondary_id.is_some()))
        .collect();

    // Joueur secondaire
    let sec_render = gs.secondary_player.as_ref().filter(|s| !s.is_dead()).map(|s| (s.x, s.y));

    // Projectiles secondaires
    let sec_proj_renders: Vec<(f32, f32)> = gs
        .secondary_projectiles
        .iter()
        .map(|p| (p.x, p.y))
        .collect();

    // Joueur
    let player_x = gs.player.x;
    let player_y = gs.player.y;
    let _player_dir = gs.player.dir;
    let player_attack_range = crate::player::Player::auto_attack_range();

    // Notifications level up
    let notifications: Vec<String> = gs.pending_level_up_notifications.clone();

    // ─── Pré-calcul des styles de rendu des entités ───
    let _tower_cost = TOWER_BASE_COST_GOLD;

    let loot_styles: Vec<String> = loot_renders.iter()
        .map(|(x, y, c)| entity_style(*x, *y, 5.0, c))
        .collect();

    let castle_ent_style = format!(
        "{}border:2px solid #8899aa;border-radius:3px;",
        entity_style(castle_x, castle_y, 40.0, "#667788")
    );
    let castle_hp_outer_s = hp_bar_outer(castle_x, castle_y, 40.0);
    let castle_hp_fill_s = hp_bar_fill_style(castle_hp, castle_hp_max);

    let tower_styles: Vec<(String, String, String)> = tower_renders.iter()
        .map(|(x, y, hp, hm)| (
            format!("{}border:1px solid #aa8833;border-radius:2px;", entity_style(*x, *y, 40.0, "#8b6914")),
            hp_bar_outer(*x, *y, 40.0),
            hp_bar_fill_style(*hp, *hm),
        ))
        .collect();

    let troop_styles: Vec<String> = troop_renders.iter()
        .map(|(x, y, is_sec)| {
            let c = if *is_sec { "#22aa88" } else { "#44cc44" };
            format!("{}border-radius:50%;", entity_style(*x, *y, 10.0, c))
        })
        .collect();

    // ─ Temps pour les animations de sprites (basé sur l'instant système) ─
    static START: std::sync::OnceLock<Instant> = std::sync::OnceLock::new();
    let anim_elapsed = START.get_or_init(Instant::now).elapsed().as_secs_f32();

    // ─ Rendu des ennemis (sprites animés) ─
    let enemy_styles: Vec<(String, Option<(String, String)>)> = enemy_renders.iter()
        .map(|(x, y, sz, _c, hp, hm, kind, is_flashing)| {
            // Choisir le sprite selon le type d'ennemi
            let sprite = match kind {
                EnemyKind::Normal => &enemy_normal::IDLE,
                EnemyKind::MiniBoss => &enemy_miniboss::IDLE,
                EnemyKind::Boss => &enemy_boss::IDLE,
            };
            let frame_idx = animation_frame_index(anim_elapsed, sprite.frame_count, sprite.frame_duration_s);
            let sprite_css = sprite.css_style(frame_idx, false);
            
            // Taille du sprite en px (40, 56, 72 pour cohérence avec background-size)
            let sprite_size_px = match kind {
                EnemyKind::Normal => 40.0,
                EnemyKind::MiniBoss => 56.0,
                EnemyKind::Boss => 72.0,
            };
            let left_px = px(*x);
            let top_px = px(*y);
            
            let ent = if *is_flashing {
                format!(
                    "position:absolute;left:{left_px:.0}px;top:{top_px:.0}px;width:{sprite_size_px:.0}px;height:{sprite_size_px:.0}px;transform:translate(-50%,-50%);{sprite_css}filter:brightness(3);"
                )
            } else {
                format!(
                    "position:absolute;left:{left_px:.0}px;top:{top_px:.0}px;width:{sprite_size_px:.0}px;height:{sprite_size_px:.0}px;transform:translate(-50%,-50%);{sprite_css}"
                )
            };
            let hp_bar = if *hp < *hm {
                Some((hp_bar_outer(*x, *y, *sz), hp_bar_fill_style(*hp, *hm)))
            } else {
                None
            };
            (ent, hp_bar)
        })
        .collect();

    let sec_style = sec_render.map(|(x, y)|
        format!("{}border:1px solid #66ffff;border-radius:50%;", entity_style(x, y, 10.0, "#00cccc"))
    );

    // ─ Rendu du joueur (sprite animé) ─
    let player_sprite = if player_dead { &player::DEATH } else { &player::IDLE };
    let player_frame = animation_frame_index(anim_elapsed, player_sprite.frame_count, player_sprite.frame_duration_s);
    // Flip horizontal si le joueur regarde vers la gauche
    let player_facing_left = matches!(_player_dir, Dir8::W | Dir8::NW | Dir8::SW);
    let player_sprite_css = player_sprite.css_style(player_frame, player_facing_left);
    let player_left_px = px(player_x);
    let player_top_px = px(player_y);
    let player_size_px = 56.0; // Taille du sprite joueur (7% de 800)
    let player_style = if !player_dead {
        format!(
            "position:absolute;left:{player_left_px:.0}px;top:{player_top_px:.0}px;width:{player_size_px:.0}px;height:{player_size_px:.0}px;transform:translate(-50%,-50%);z-index:5;{player_sprite_css}"
        )
    } else {
        format!(
            "position:absolute;left:{player_left_px:.0}px;top:{player_top_px:.0}px;width:{player_size_px:.0}px;height:{player_size_px:.0}px;transform:translate(-50%,-50%);z-index:5;opacity:0.5;{player_sprite_css}"
        )
    };
    
    // ─ Cône d'attaque du joueur (suit le curseur) ─
    let current_cursor_angle = cursor_angle();
    let attack_cone_style = if !player_dead && phase == GamePhase::Battle {
        // Cône de 40° (demi-angle 20°), portée = player_attack_range
        let cone_size = player_attack_range * 2.0;
        Some(format!(
            "position:absolute;left:{xp:.0}px;top:{yp:.0}px;width:{sz:.0}px;height:{sz:.0}px;transform:translate(-50%,-50%) rotate({angle}deg);background:conic-gradient(from -20deg, rgba(68,136,255,0.3) 0deg, rgba(68,136,255,0.3) 40deg, transparent 40deg);border-radius:50%;pointer-events:none;z-index:4;",
            xp = px(player_x),
            yp = px(player_y),
            sz = cone_size,
            angle = current_cursor_angle,
        ))
    } else {
        None
    };

    let proj_styles: Vec<String> = proj_renders.iter()
        .map(|(x, y)| format!("{}border-radius:50%;z-index:6;", entity_style(*x, *y, 3.0, "#cccccc")))
        .collect();

    let sec_proj_styles: Vec<String> = sec_proj_renders.iter()
        .map(|(x, y)| format!("{}border-radius:50%;z-index:6;", entity_style(*x, *y, 4.0, "#00ffff")))
        .collect();

    // ─── Grille de construction RTS ───
    // Génère les cellules de la grille autour du château (rayon de 10 cellules)
    use crate::constants::size::CONSTRUCTION_CELL_SIZE;
    let grid_radius = 10i32; // Nombre de cellules autour du château
    let cell_size_world = CONSTRUCTION_CELL_SIZE; // 20.0 px
    
    let grid_cells: Vec<(i32, i32, bool, f32, f32)> = if phase == GamePhase::Preparation && build_mode() {
        let mut cells = Vec::new();
        for ci in -grid_radius..=grid_radius {
            for cj in -grid_radius..=grid_radius {
                // Convertir coordonnées de cellule en coordonnées monde
                let world_x = castle_x + (ci as f32) * cell_size_world;
                let world_y = castle_y + (cj as f32) * cell_size_world;
                let can_build = gs.can_build_at_cell(ci, cj);
                cells.push((ci, cj, can_build, world_x, world_y));
            }
        }
        cells
    } else {
        Vec::new()
    };

    // ─── Données pour les panneaux ───
    let stats = gs.effective_stats();
    let base_stats = gs.player.stats.clone();
    let enemies_killed = gs.enemies_killed;
    let bosses_killed = gs.bosses_killed;
    let gold_total = gs.gold_total;
    let max_wave = gs.max_wave_reached;

    // Skills
    let skill_ranks = gs.warrior_skill_ranks.clone();

    // Inventaire
    let inventory: Vec<(usize, String)> = gs
        .inventory
        .iter()
        .enumerate()
        .map(|(i, e)| {
            let label = match e {
                InventoryEntry::Unidentified(slot) => format!("? {} (non identifié)", slot.label()),
                InventoryEntry::Identified(item) => {
                    let (_r, _g, _b) = item.rarity.color_rgb();
                    format!("{} [{}]", item.display_name, item.rarity.label())
                }
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
                // Focus automatique pour capturer les touches clavier
                spawn(async move {
                    tokio::time::sleep(Duration::from_millis(50)).await;
                    let _ = evt.set_focus(true).await;
                });
            },
            onkeydown: move |evt| {
                let key = evt.key().to_string();
                let mut keys = held_keys.write();
                // Joueur 1 : ZQSD + WASD + flèches
                match key.as_str() {
                    "w" | "W" | "ArrowUp" | "z" | "Z" => keys.up = true,
                    "a" | "A" | "ArrowLeft" | "q" | "Q" => keys.left = true,
                    "s" | "S" | "ArrowDown" => keys.down = true,
                    "d" | "D" | "ArrowRight" => keys.right = true,
                    // Joueur 2 : IJKL
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

            // ═══ HEADER : infos de run ═══
            header {
                style: "display:flex;align-items:center;justify-content:space-between;padding:6px 16px;background:#1b2838;border-bottom:1px solid #2a3f5f;min-height:36px;flex-shrink:0;",
                div {
                    style: "display:flex;align-items:center;gap:12px;",
                    button {
                        style: "padding:4px 12px;background:#232f3e;color:#8f98a0;border:1px solid #2a3f5f;border-radius:3px;cursor:pointer;font-size:12px;",
                        onclick: move |_| {
                            game_state.set(None);
                            screen.set(Screen::MainMenu);
                        },
                        "← Menu"
                    }
                    span { style: "font-size:15px;color:#1a9fff;font-weight:600;", "Lord of the Castle" }
                }
                div {
                    style: "display:flex;gap:16px;font-size:12px;align-items:center;",
                    span { style: "color:#8f98a0;",
                        "Phase : "
                        span { style: "color:#c6d4df;font-weight:600;", "{phase_label}" }
                    }
                    span { style: "color:#8f98a0;",
                        "Vague "
                        span { style: "color:#c6d4df;", "{wave}" }
                    }
                    if phase == GamePhase::Battle {
                        span { style: "color:#8f98a0;",
                            "Ennemis "
                            span { style: "color:#ff6644;", "{enemies_alive}" }
                            span { style: "color:#555;", "/{spawn_qty}" }
                        }
                    }
                    span { style: "color:#8f98a0;",
                        "Or "
                        span { style: "color:#ffcc00;font-weight:600;", "{gold}" }
                    }
                    span { style: "color:#8f98a0;",
                        "Niv "
                        span { style: "color:#c6d4df;", "{level}" }
                    }
                    span { style: "color:#8f98a0;",
                        "Tours "
                        span { style: "color:#c6d4df;", "{towers_count}" }
                    }
                    span { style: "color:#8f98a0;",
                        "Troupes "
                        span { style: "color:#c6d4df;", "{troops_count}/{max_troops}" }
                    }
                }
            }

            // ═══ BARRE DE BOUTONS + XP ═══
            div {
                style: "display:flex;align-items:center;justify-content:space-between;padding:4px 16px;background:#1e2a3a;border-bottom:1px solid #2a3f5f;min-height:30px;flex-shrink:0;gap:8px;",
                // Boutons panneaux
                div {
                    style: "display:flex;gap:4px;",
                    { panel_button("Stats", Panel::Stats, active_panel, skill_pts == 0 && stat_pts == 0) }
                    { panel_button("Skills", Panel::Skills, active_panel, skill_pts == 0) }
                    { panel_button("Inventaire", Panel::Inventory, active_panel, inv_count == 0) }
                    // Bouton dev mode
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
                // Barre XP
                div {
                    style: "flex:1;display:flex;align-items:center;gap:8px;max-width:400px;",
                    div {
                        style: "flex:1;height:10px;background:#232f3e;border-radius:5px;overflow:hidden;",
                        div {
                            style: "width:{xp_pct:.1}%;height:100%;background:linear-gradient(90deg,#1a6fff,#1a9fff);transition:width 0.2s;",
                        }
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

            // ═══ CORPS : zone de jeu + sidebar ═══
            div {
                style: "flex:1;display:flex;overflow:hidden;min-height:0;align-items:stretch;",

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
                            
                            // Stocker pour la boucle de jeu (auto-attaque)
                            set_cursor_world(cursor_world_x, cursor_world_y);
                            
                            // Calculer l'angle pour le cône visuel
                            if let Some(gs) = game_state.read().as_ref() {
                                let px = gs.player.x;
                                let py = gs.player.y;
                                
                                let dx = cursor_world_x - px;
                                let dy = cursor_world_y - py;
                                // atan2: 0°=droite, 90°=bas (Y vers le bas sur écran)
                                // CSS rotate: 0°=haut, 90°=droite
                                // Pour pointer vers droite (atan2=0), on veut rotate(90)
                                // Donc: css_angle = atan2_angle + 90
                                let angle_rad = dy.atan2(dx);
                                let angle_deg = angle_rad.to_degrees() + 90.0;
                                
                                cursor_angle.set(angle_deg);
                            }
                        },

                        // ─ Grille de construction RTS ─
                        if build_mode() && phase == GamePhase::Preparation {
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
                                                    if let Some(gs) = game_state.write().as_mut() {
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

                        // ─ Loot au sol ─
                        for s in loot_styles.iter() {
                            div { style: "{s}" }
                        }

                        // ─ Château ─
                        div { style: "{castle_ent_style}" }
                        div {
                            style: "{castle_hp_outer_s}",
                            div { style: "{castle_hp_fill_s}" }
                        }

                        // ─ Tours ─
                        for (te, to, tf) in tower_styles.iter() {
                            div { style: "{te}" }
                            div {
                                style: "{to}",
                                div { style: "{tf}" }
                            }
                        }

                        // ─ Troupes ─
                        for s in troop_styles.iter() {
                            div { style: "{s}" }
                        }

                        // ─ Ennemis ─
                        for (ent_s, hp_opt) in enemy_styles.iter() {
                            div { style: "{ent_s}" }
                            if let Some((ho, hf)) = hp_opt {
                                div {
                                    style: "{ho}",
                                    div { style: "{hf}" }
                                }
                            }
                        }

                        // ─ Joueur secondaire ─
                        if let Some(ref ss) = sec_style {
                            div { style: "{ss}" }
                        }

                        // ─ Cône d'attaque joueur ─
                        if let Some(ref cone_style) = attack_cone_style {
                            div { style: "{cone_style}" }
                        }

                        // ─ Joueur ─
                        div { style: "{player_style}" }

                        // ─ Projectiles tours ─
                        for s in proj_styles.iter() {
                            div { style: "{s}" }
                        }

                        // ─ Projectiles secondaires ─
                        for s in sec_proj_styles.iter() {
                            div { style: "{s}" }
                        }

                        // Légende (coin bas gauche)
                        div {
                            style: "position:absolute;bottom:4px;left:4px;font-size:9px;color:#3a4555;pointer-events:none;",
                            "WASD : déplacement • IJKL : joueur 2"
                        }

                        // Notification "cliquer sur la zone" (vagues 1,2,3)
                        if show_click_hint() {
                            div {
                                style: "position:absolute;top:50%;left:50%;transform:translate(-50%,-50%);
                                        background:rgba(0,0,0,0.8);color:#ffd700;padding:16px 32px;
                                        border-radius:12px;font-size:18px;font-weight:bold;
                                        text-align:center;pointer-events:none;z-index:1000;
                                        animation:fadeInOut 3s ease-in-out;",
                                "Cliquez sur la zone de combat pour activer les contrôles !"
                            }
                        }
                    }
                }

                // ═══ SIDEBAR (Préparation seulement) ═══
                if phase == GamePhase::Preparation {
                    div {
                        style: "width:200px;min-height:600px;background:#1b2838;border-left:1px solid #2a3f5f;display:flex;flex-direction:column;overflow-y:auto;flex-shrink:0;padding:8px;gap:6px;",

                        h3 { style: "font-size:13px;color:#1a9fff;margin-bottom:4px;text-align:center;", "Préparation" }

                        // Construction tour - Mode RTS
                        div {
                            style: "background:#232f3e;border-radius:4px;padding:8px;",
                            p { style: "font-size:12px;color:#c6d4df;font-weight:600;margin-bottom:4px;", "🏗 Tour de base" }
                            p { style: "font-size:10px;color:#8f98a0;margin-bottom:6px;", "PV 100 • Portée 300px • Dmg 4" }
                            p { style: "font-size:10px;color:#ffcc00;margin-bottom:6px;", "Coût : {TOWER_BASE_COST_GOLD} or" }
                            
                            // Bouton pour activer/désactiver le mode construction
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
                            
                            // Bouton construction rapide (trouve automatiquement une place)
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
                }

                // ═══ SIDEBAR (Battle - infos de combat) ═══
                if phase == GamePhase::Battle {
                    div {
                        style: "width:200px;min-height:600px;background:#1b2838;border-left:1px solid #2a3f5f;display:flex;flex-direction:column;overflow-y:auto;flex-shrink:0;padding:8px;gap:6px;",

                        h3 { style: "font-size:13px;color:#ff6644;margin-bottom:4px;text-align:center;", "⚔ Combat" }

                        // Infos vague
                        div {
                            style: "background:#232f3e;border-radius:4px;padding:8px;",
                            p { style: "font-size:12px;color:#c6d4df;font-weight:600;margin-bottom:4px;", "Vague {wave}" }
                            p { style: "font-size:11px;color:#8f98a0;margin-bottom:4px;", "Ennemis restants : {enemies_alive}" }
                            p { style: "font-size:11px;color:#8f98a0;", "Total à spawner : {spawn_qty}" }
                        }

                        // Château
                        div {
                            style: "background:#232f3e;border-radius:4px;padding:8px;",
                            p { style: "font-size:12px;color:#c6d4df;font-weight:600;margin-bottom:4px;", "🏰 Château" }
                            {
                                let castle_pct_sidebar = if castle_hp_max > 0 { castle_hp as f32 / castle_hp_max as f32 * 100.0 } else { 0.0 };
                                rsx! {
                                    div {
                                        style: "width:100%;height:8px;background:#332222;border-radius:4px;overflow:hidden;margin-bottom:4px;",
                                        div {
                                            style: "width:{castle_pct_sidebar:.1}%;height:100%;background:#667788;",
                                        }
                                    }
                                    p { style: "font-size:10px;color:#8f98a0;text-align:center;", "{castle_hp}/{castle_hp_max} PV" }
                                }
                            }
                        }

                        // Victoire / Défaite
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

                        // Tours actives
                        div {
                            style: "background:#232f3e;border-radius:4px;padding:8px;",
                            p { style: "font-size:12px;color:#c6d4df;font-weight:600;margin-bottom:4px;", "🗼 Tours : {towers_count}" }
                            p { style: "font-size:10px;color:#8f98a0;", "Défense automatique active" }
                        }

                        // Troupes
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

            // ═══ FOOTER ═══
            footer {
                style: "display:flex;align-items:center;justify-content:space-between;padding:5px 16px;background:#1b2838;border-top:1px solid #2a3f5f;min-height:40px;flex-shrink:0;gap:8px;",

                // Gauche : PV / Mana / Armure / Att
                div {
                    style: "display:flex;align-items:center;gap:10px;flex-wrap:nowrap;",
                    // Barre PV
                    {
                        let footer_hp_pct = if player_hp_max > 0 { player_hp as f32 / player_hp_max as f32 * 100.0 } else { 0.0 };
                        let footer_hp_color = if player_dead { "#666" } else if player_hp as f32 / player_hp_max.max(1) as f32 > 0.5 { "#cc4444" } else { "#ff2222" };
                        rsx! {
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
                        }
                    }
                    span { style: "font-size:11px;color:#8f98a0;", "🛡{armor}" }
                    span { style: "font-size:11px;color:#8f98a0;", "⚔{atk_dmg}" }
                    if player_dead {
                        span { style: "font-size:11px;color:#ff4444;font-weight:600;", "💀 MORT" }
                    }
                }

                // Centre : actions (lancer vague, fin de vague)
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
                                // Auto-save après fin de combat
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

                // Droite-centre : sauvegarde (3 slots, slot actif en surbrillance)
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

                // Droite : infos château
                {
                    let castle_pct = if castle_hp_max > 0 { castle_hp as f32 / castle_hp_max as f32 * 100.0 } else { 0.0 };
                    rsx! {
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

            // ═══ PANNEAU OVERLAY ═══
            if let Some(panel) = active_panel() {
                div {
                    style: "position:absolute;top:68px;left:50%;transform:translateX(-50%);width:520px;max-height:calc(100vh - 150px);background:#1b2838;border:1px solid #2a3f5f;border-radius:6px;box-shadow:0 8px 32px rgba(0,0,0,0.6);z-index:50;display:flex;flex-direction:column;overflow:hidden;",

                    // Barre titre
                    div {
                        style: "display:flex;align-items:center;justify-content:space-between;padding:8px 12px;background:#232f3e;border-bottom:1px solid #2a3f5f;flex-shrink:0;",
                        span {
                            style: "font-size:14px;font-weight:600;color:#1a9fff;",
                            { match panel {
                                Panel::Stats => "Statistiques",
                                Panel::Skills => "Compétences Guerrier",
                                Panel::Inventory => "Inventaire",
                            } }
                        }
                        button {
                            style: "padding:2px 8px;background:#3a2222;color:#cc6666;border:1px solid #4a3333;border-radius:3px;cursor:pointer;font-size:12px;",
                            onclick: move |_| active_panel.set(None),
                            "✕"
                        }
                    }

                    // Contenu du panneau
                    div {
                        style: "flex:1;overflow-y:auto;padding:10px;",
                        { match panel {
                            Panel::Stats => rsx! {
                                StatsContent {
                                    game_state,
                                    base_stats: base_stats.clone(),
                                    eff_stats: stats.clone(),
                                    stat_pts,
                                    player_hp, player_hp_max, armor, atk_dmg,
                                    castle_hp, castle_hp_max,
                                    enemies_killed, bosses_killed, gold_total, max_wave,
                                }
                            },
                            Panel::Skills => rsx! {
                                SkillsContent {
                                    game_state,
                                    skill_ranks: skill_ranks.clone(),
                                    skill_pts,
                                }
                            },
                            Panel::Inventory => rsx! {
                                InventoryContent {
                                    game_state,
                                    inventory: inventory.clone(),
                                    equipped: equipped.clone(),
                                    gold,
                                }
                            },
                        } }
                    }
                }
            }

            // ═══ OVERLAY GAME OVER ═══
            if is_game_over {
                div {
                    style: "position:absolute;inset:0;background:rgba(0,0,0,0.8);display:flex;flex-direction:column;align-items:center;justify-content:center;z-index:100;",
                    h1 { style: "font-size:36px;color:#cc2222;margin-bottom:12px;", "GAME OVER" }
                    p { style: "font-size:16px;color:#c6d4df;margin-bottom:8px;",
                        "Le château a été détruit à la vague {wave}."
                    }
                    p { style: "font-size:14px;color:#8f98a0;margin-bottom:24px;",
                        "Ennemis tués : {enemies_killed} • Boss : {bosses_killed} • Or total : {gold_total}"
                    }
                    button {
                        style: "padding:12px 36px;background:#1a9fff;color:white;border:none;border-radius:6px;font-size:16px;font-weight:600;cursor:pointer;",
                        onclick: move |_| {
                            game_state.set(None);
                            screen.set(Screen::MainMenu);
                        },
                        "Retour au menu"
                    }
                }
            }

            // ═══ OVERLAY VAGUE GAGNÉE ═══
            if is_wave_won && !is_game_over {
                div {
                    style: "position:absolute;top:50%;left:50%;transform:translate(-50%,-50%);background:#1b3828;border:2px solid #2a5a3f;border-radius:8px;padding:20px 32px;z-index:80;text-align:center;",
                    h2 { style: "font-size:22px;color:#5ba32b;margin-bottom:8px;", "Vague {wave} terminée !" }
                    p { style: "font-size:13px;color:#c6d4df;margin-bottom:4px;",
                        "Ennemis tués : {enemies_killed}"
                    }
                    p { style: "font-size:12px;color:#8f98a0;",
                        "Passez en préparation pour construire et recruter."
                    }
                }
            }

            // ═══ NOTIFICATIONS ═══
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

/// Bouton pour ouvrir/fermer un panneau.
fn panel_button(label: &str, panel: Panel, mut active: Signal<Option<Panel>>, dim: bool) -> Element {
    let is_active = active() == Some(panel);
    let bg = if is_active { "#2a4a6a" } else { "#232f3e" };
    let color = if dim && !is_active { "#556677" } else if is_active { "#1a9fff" } else { "#c6d4df" };
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

// ─── Contenu panneau Stats ────────────────────────────────────────────

#[component]
fn StatsContent(
    game_state: Signal<Option<GameState>>,
    base_stats: crate::character_creation::CharacterStats,
    eff_stats: crate::character_creation::CharacterStats,
    stat_pts: u32,
    player_hp: i32,
    player_hp_max: i32,
    armor: i32,
    atk_dmg: i32,
    castle_hp: i32,
    castle_hp_max: i32,
    enemies_killed: u32,
    bosses_killed: u32,
    gold_total: u32,
    max_wave: u32,
) -> Element {
    use crate::character_creation::Stat;
    let stat_list: Vec<(&str, &str, i32, i32, Stat)> = vec![
        ("For", "Force", base_stats.for_, eff_stats.for_, Stat::For),
        ("Con", "Constitution", base_stats.con, eff_stats.con, Stat::Con),
        ("Agi", "Agilité", base_stats.agi, eff_stats.agi, Stat::Agi),
        ("Dex", "Dextérité", base_stats.dex, eff_stats.dex, Stat::Dex),
        ("Int", "Intelligence", base_stats.int, eff_stats.int, Stat::Int),
        ("Sag", "Sagesse", base_stats.sag, eff_stats.sag, Stat::Sag),
        ("Cha", "Charisme", base_stats.cha, eff_stats.cha, Stat::Cha),
        ("Luk", "Chance", base_stats.luk, eff_stats.luk, Stat::Luk),
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
                if stat_pts > 0 {
                    button {
                        style: "padding:1px 4px;background:#2a4a3a;color:#88cc88;border:1px solid #3a5a4a;border-radius:2px;cursor:pointer;font-size:10px;",
                        onclick: move |_| {
                            if let Some(gs) = game_state.write().as_mut() {
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
            span { style: "color:#c6d4df;", "{player_hp} / {player_hp_max}" }
            span { style: "color:#8f98a0;", "Armure" }
            span { style: "color:#c6d4df;", "{armor}" }
            span { style: "color:#8f98a0;", "Dégâts auto" }
            span { style: "color:#c6d4df;", "{atk_dmg}" }
        }

        // Château
        h3 { style: "font-size:13px;color:#1a9fff;margin-bottom:6px;", "Château" }
        div {
            style: "font-size:11px;display:grid;grid-template-columns:1fr 1fr;gap:2px 16px;margin-bottom:12px;",
            span { style: "color:#8f98a0;", "PV" }
            span { style: "color:#c6d4df;", "{castle_hp} / {castle_hp_max}" }
        }

        // Run
        h3 { style: "font-size:13px;color:#1a9fff;margin-bottom:6px;", "Run" }
        div {
            style: "font-size:11px;display:grid;grid-template-columns:1fr 1fr;gap:2px 16px;",
            span { style: "color:#8f98a0;", "Ennemis tués" }
            span { style: "color:#c6d4df;", "{enemies_killed}" }
            span { style: "color:#8f98a0;", "Boss tués" }
            span { style: "color:#c6d4df;", "{bosses_killed}" }
            span { style: "color:#8f98a0;", "Or total" }
            span { style: "color:#ffcc00;", "{gold_total}" }
            span { style: "color:#8f98a0;", "Vague max" }
            span { style: "color:#c6d4df;", "{max_wave}" }
        }
    }
}

// ─── Contenu panneau Skills ───────────────────────────────────────────

#[component]
fn SkillsContent(
    game_state: Signal<Option<GameState>>,
    skill_ranks: std::collections::HashMap<WarriorSkillId, u32>,
    skill_pts: u32,
) -> Element {
    let all_skills: Vec<(WarriorSkillId, &str, &str, u32, u32, bool)> = WarriorSkillId::all()
        .iter()
        .map(|&id| {
            let def = warrior_skill_def(id);
            let current = skill_ranks.get(&id).copied().unwrap_or(0);
            let can_learn = skill_pts > 0
                && current < def.max_rank
                && crate::warrior_skills::prerequisites_met(&skill_ranks, &def);
            (id, def.name, def.effect_description, current, def.max_rank, can_learn)
        })
        .collect();

    rsx! {
        if skill_pts > 0 {
            p { style: "font-size:12px;color:#ffcc00;margin-bottom:8px;", "Points disponibles : {skill_pts}" }
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
                                        if let Some(gs) = game_state.write().as_mut() {
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

// ─── Contenu panneau Inventaire ───────────────────────────────────────

#[component]
fn InventoryContent(
    game_state: Signal<Option<GameState>>,
    inventory: Vec<(usize, String)>,
    equipped: Vec<(ItemSlot, String)>,
    gold: u32,
) -> Element {
    rsx! {
        // Équipement
        h3 { style: "font-size:13px;color:#1a9fff;margin-bottom:6px;", "Équipement" }
        div {
            style: "display:grid;grid-template-columns:90px 1fr;gap:2px 8px;font-size:11px;margin-bottom:12px;",
            for (slot, label) in equipped.iter() {
                span { style: "color:#8f98a0;", "{slot.label()}" }
                span { style: "color:#c6d4df;", "{label}" }
            }
        }

        // Inventaire (sac)
        {
            let inv_count = inventory.len();
            rsx! {
                h3 { style: "font-size:13px;color:#1a9fff;margin-bottom:6px;",
                    "Sac ({inv_count}/20)"
                }
            }
        }
        if inventory.is_empty() {
            p { style: "font-size:11px;color:#556677;", "Inventaire vide." }
        }
        div {
            style: "display:flex;flex-direction:column;gap:3px;",
            for (idx, label) in inventory.iter() {
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
                                        if let Some(gs) = game_state.write().as_mut() {
                                            gs.identify_self(idx_copy);
                                        }
                                    },
                                    "ID soi"
                                }
                                button {
                                    style: "padding:2px 6px;background:#2a3a4a;color:#ffcc00;border:1px solid #3a4a5a;border-radius:2px;cursor:pointer;font-size:9px;",
                                    onclick: move |_| {
                                        if let Some(gs) = game_state.write().as_mut() {
                                            gs.identify_expert(idx_copy);
                                        }
                                    },
                                    "ID 20g"
                                }
                            } else {
                                button {
                                    style: "padding:2px 6px;background:#2a4a3a;color:#88cc88;border:1px solid #3a5a4a;border-radius:2px;cursor:pointer;font-size:9px;",
                                    onclick: move |_| {
                                        if let Some(gs) = game_state.write().as_mut() {
                                            gs.equip_item(idx_copy);
                                        }
                                    },
                                    "Équiper"
                                }
                                button {
                                    style: "padding:2px 6px;background:#4a3a2a;color:#ccaa88;border:1px solid #5a4a3a;border-radius:2px;cursor:pointer;font-size:9px;",
                                    onclick: move |_| {
                                        if let Some(gs) = game_state.write().as_mut() {
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
