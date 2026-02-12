//! Écran de création de personnage.
//!
//! Système de création par choix de phrases (4 étapes) puis récapitulatif.
//!
//! @id: lord_of_the_castle.ui.character_creation
//! @do: render_character_creation_flow
//! @role: ui
//! @layer: ui
//! @human: Écran de création de personnage avec phrases et stats

use crate::app::Screen;
use crate::character_creation::{all_phrases, apply_phrase_effects, pick_three_phrases, CharacterStats, PhraseDef};
use crate::game_state::{rand_simple, GameState};
use crate::player::Player;
use crate::save::LordOfTheCastleDb;
use dioxus::prelude::*;
use std::path::PathBuf;

/// État interne de la création de personnage.
#[derive(Debug, Clone)]
struct CreationState {
    step: usize,
    stats: CharacterStats,
    available: Vec<usize>,
    choices: Vec<PhraseDef>,
    history: Vec<String>,
    name: String,
    reroll_at_end: bool,
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

/// Écran de création de personnage.
#[component]
pub fn CharacterCreationScreen(
    mut screen: Signal<Screen>,
    mut game_state: Signal<Option<GameState>>,
    mut active_slot: Signal<Option<u8>>,
) -> Element {
    let state = use_signal(CreationState::new);
    let base_path = use_context::<Signal<PathBuf>>();

    let st = state.read().clone();
    let step = st.step;
    let stats = st.stats.clone();
    let history = st.history.clone();
    let choices = st.choices.clone();
    let name = st.name.clone();
    let save_slot = st.save_slot;

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
                { phrase_selection(state, choices) }
            } else {
                { summary_screen(state, stats, history, name, save_slot, screen, game_state, active_slot, base_path) }
            }

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

fn phrase_selection(mut state: Signal<CreationState>, choices: Vec<PhraseDef>) -> Element {
    rsx! {
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
                                    s.choices = pick_three_phrases(&mut s.available, &mut roll);
                                } else {
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
    }
}

#[allow(clippy::too_many_arguments)]
fn summary_screen(
    mut state: Signal<CreationState>,
    stats: CharacterStats,
    history: Vec<String>,
    name: String,
    save_slot: u8,
    mut screen: Signal<Screen>,
    mut game_state: Signal<Option<GameState>>,
    mut active_slot: Signal<Option<u8>>,
    base_path: Signal<PathBuf>,
) -> Element {
    rsx! {
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
