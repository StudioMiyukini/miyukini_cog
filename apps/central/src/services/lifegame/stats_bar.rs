//! Barre de statistiques pour Life Game.
//!
//! @id: central.services.lifegame.stats_bar
//! @role: ui
//! @layer: domain
//! @do: render_game_statistics_bar_with_population_and_time
//! @human: Barre inférieure avec stats détaillées.

use dioxus::prelude::*;
use super::{LifeGameLocalState, BG_PANEL, BORDER, TEXT_LIGHT, TEXT_MUTED};

const GOLD: &str = "#ffd700";

#[allow(dead_code)]

/// Barre de statistiques détaillées.
#[component]
pub fn StatsBar(state: Signal<LifeGameLocalState>) -> Element {
    let s = state.read();
    let day = s.current_day;
    let elapsed = s.elapsed_time;
    let population = s.entities.units.len();
    let buildings = s.entities.buildings.len();
    let creatures = s.entities.creatures.len();

    // Statistiques par race
    let humans = s.entities.units.values()
        .filter(|u| matches!(u.race, lifegame_entities::Race::Human))
        .count();
    let orcs = s.entities.units.values()
        .filter(|u| matches!(u.race, lifegame_entities::Race::Orc))
        .count();
    let elves = s.entities.units.values()
        .filter(|u| matches!(u.race, lifegame_entities::Race::Elf))
        .count();
    let dwarves = s.entities.units.values()
        .filter(|u| matches!(u.race, lifegame_entities::Race::Dwarf))
        .count();

    // Format du temps
    let hours = ((elapsed / 3600.0) % 24.0) as u32;
    let minutes = ((elapsed / 60.0) % 60.0) as u32;
    let time_str = format!("Jour {day} - {hours:02}:{minutes:02}");

    rsx! {
        div {
            style: "display: flex; align-items: center; justify-content: space-between; padding: 8px 16px; background: {BG_PANEL}; border-top: 1px solid {BORDER};",

            // Temps
            div {
                style: "display: flex; align-items: center; gap: 8px;",
                span { style: "font-size: 14px;", "🕐" }
                span { style: "font-size: 12px; color: {GOLD}; font-weight: 600;", "{time_str}" }
            }

            // Population par race
            div {
                style: "display: flex; gap: 12px;",
                RaceStat { icon: "👤", name: "Humains", count: humans }
                RaceStat { icon: "👹", name: "Orcs", count: orcs }
                RaceStat { icon: "🧝", name: "Elfes", count: elves }
                RaceStat { icon: "🧔", name: "Nains", count: dwarves }
            }

            // Total
            div {
                style: "display: flex; gap: 12px;",
                StatItem { icon: "👥", label: "Total", value: format!("{population}") }
                StatItem { icon: "🏠", label: "Bâtiments", value: format!("{buildings}") }
                StatItem { icon: "🐾", label: "Créatures", value: format!("{creatures}") }
            }
        }
    }
}

/// Statistique de race.
#[component]
fn RaceStat(icon: &'static str, name: &'static str, count: usize) -> Element {
    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 4px;",
            span { style: "font-size: 12px;", "{icon}" }
            span { style: "font-size: 10px; color: {TEXT_MUTED};", "{name}:" }
            span { style: "font-size: 11px; color: {TEXT_LIGHT}; font-weight: 600;", "{count}" }
        }
    }
}

/// Item de statistique.
#[component]
fn StatItem(icon: &'static str, label: &'static str, value: String) -> Element {
    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 4px; padding: 4px 8px; background: rgba(255,255,255,0.05); border-radius: 4px;",
            span { style: "font-size: 12px;", "{icon}" }
            span { style: "font-size: 9px; color: {TEXT_MUTED};", "{label}" }
            span { style: "font-size: 11px; color: {TEXT_LIGHT}; font-weight: 600;", "{value}" }
        }
    }
}
