//! Barre d'en-tête de l'écran de jeu.
//!
//! Affiche : bouton menu, titre, phase, vague, ennemis, or, niveau, tours, troupes.
//!
//! @id: lord_of_the_castle.ui.game_header
//! @do: render_game_info_header
//! @role: ui
//! @layer: ui
//! @human: Header avec informations de run (vague, or, niveau, etc.)

use crate::app::Screen;
use crate::game_state::{GamePhase, GameState};
use dioxus::prelude::*;

/// Barre d'en-tête du jeu.
#[component]
pub fn GameHeader(
    mut screen: Signal<Screen>,
    mut game_state: Signal<Option<GameState>>,
    phase: GamePhase,
    wave: u32,
    enemies_alive: u32,
    spawn_qty: u32,
    gold: u32,
    level: u32,
    towers_count: u32,
    troops_count: u32,
    max_troops: u32,
) -> Element {
    let phase_label = match phase {
        GamePhase::Preparation => "Préparation",
        GamePhase::Battle => "Bataille",
    };

    rsx! {
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
    }
}
