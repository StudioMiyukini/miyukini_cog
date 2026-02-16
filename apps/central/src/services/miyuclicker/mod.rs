//! Vue Lord of the Click (MiyukiniClicker) — UI Dioxus complète.
//!
//! Ressources, allocations, bâtiments, conversions, clics manuels, vitesse, sauvegarde.

use dioxus::prelude::*;
use miyuclicker::idlesim::{
    apply_click, apply_allocation, apply_allocation_batisseurs, construction_progress_pct,
    convert_batisseur_to_ouvrier, convert_ouvrier_to_batisseur, convert_ouvrier_to_soldat,
    convert_soldat_to_ouvrier, production_rates, start_construction, tick,
    GAME_SPEED_1, GAME_SPEED_2, GAME_SPEED_3, GAME_SPEED_4, GAME_SPEED_5, GAME_SPEED_PAUSE,
};
use miyuclicker::state::{
    Allocation, AllocationBatisseurs, BuildingType, ClickTarget, GameState,
};
use std::path::Path;

use crate::data::use_service_connections;
use crate::state::use_app_state;

/// Vitesses de jeu (index 0 = pause, 1..5 = vitesses).
const SPEEDS: [(f64, &str); 6] = [
    (GAME_SPEED_PAUSE, "⏸ Pause"),
    (GAME_SPEED_1, "1h/s"),
    (GAME_SPEED_2, "3h/s"),
    (GAME_SPEED_3, "6h/s"),
    (GAME_SPEED_4, "1j/s"),
    (GAME_SPEED_5, "2j/s"),
];

/// Vue principale Lord of the Click.
#[component]
pub fn MiyuClickerView() -> Element {
    let app = use_app_state();
    let conns = use_service_connections();
    let data_dir = conns.read().miyuclicker_data_dir.clone();
    let c = app.read().current_theme.palette();

    let game_state = use_signal(|| load_or_new(&data_dir));
    let speed_index = use_signal(|| 0usize);

    let g = game_state.read().clone();
    let speed = SPEEDS[speed_index.read().min(&5)].0;
    let rates = production_rates(&g, speed);

    if g.game_over {
        return rsx! {
            div {
                style: "padding: 48px; text-align: center;",
                h2 { style: "color: #e74c3c;", "Game Over" }
                p { "Nourriture à zéro trop longtemps. Le seigneur est mort." }
                button {
                    style: "margin-top: 16px; padding: 12px 24px; background: {c.accent_blue}; color: white; border: none; border-radius: 8px; cursor: pointer;",
                    onclick: move |_| *game_state.write() = GameState::new_game(1),
                    "Nouvelle partie"
                }
            }
        };
    }

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 16px; height: 100%; overflow: auto; padding: 16px;",

            // Header : ressources + horloge + vitesse
            div {
                style: "display: flex; flex-wrap: wrap; align-items: center; gap: 16px; padding: 12px 16px; background: {c.bg_secondary}; border-radius: 8px;",
                span { style: "font-weight: 600; color: {c.text_white};", "🕐 {GameState::format_clock(g.temps_simule_s)}" }
                ResourceChip { label: "🍞 Nourriture", value: format!("{:.0}", g.nourriture), cap: format!("{:.0}", g.cap_nourriture()) }
                ResourceChip { label: "🪵 Bois", value: format!("{:.0}", g.bois), cap: format!("{:.0}", g.cap_bois()) }
                ResourceChip { label: "🪨 Pierre", value: format!("{:.0}", g.pierre), cap: format!("{:.0}", g.cap_pierre()) }
                ResourceChip { label: "⚙️ Métal", value: format!("{:.0}", g.metal), cap: format!("{:.0}", g.cap_metal()) }
                ResourceChip { label: "🔧 Outils", value: format!("{:.0}", g.outils), cap: format!("{:.0}", g.cap_outils()) }
                ResourceChip { label: "⚔️ Armes", value: format!("{:.0}", g.armes), cap: format!("{:.0}", g.cap_armes()) }
                span { style: "margin-left: auto; color: {c.text_secondary};", "Pop: {g.pop_totale()} / {g.pop_max()} | 😊 {g.bonheur_pourcent()}%" }
                div {
                    style: "display: flex; gap: 4px; align-items: center;",
                    SpeedButton { speed_index, index: 0, label: "Pause" }
                    SpeedButton { speed_index, index: 1, label: "1h/s" }
                    SpeedButton { speed_index, index: 2, label: "3h/s" }
                    SpeedButton { speed_index, index: 3, label: "6h/s" }
                    SpeedButton { speed_index, index: 4, label: "1j/s" }
                    SpeedButton { speed_index, index: 5, label: "2j/s" }
                    button {
                        style: "padding: 6px 10px; font-size: 12px; margin-left: 8px; background: {c.accent_blue}; color: white; border: none; border-radius: 4px; cursor: pointer;",
                        onclick: move |_| {
                            let mut g = game_state.read().clone();
                            let idx = (speed_index.read() as usize).min(5);
                            let speed = SPEEDS[idx].0;
                            tick(&mut g, 0.1, speed);
                            *game_state.write() = g;
                        },
                        "Avancer (0.1s)"
                    }
                }
            }

            // Clics manuels (+1 ressource)
            div {
                style: "display: flex; flex-wrap: wrap; gap: 8px;",
                h3 { style: "width: 100%; font-size: 14px; color: {c.text_secondary}; margin-bottom: 4px;", "Clics (+1)" }
                ClickButton { game_state, target: ClickTarget::Ferme, label: "🍞 Nourriture" }
                ClickButton { game_state, target: ClickTarget::Scierie, label: "🪵 Bois" }
                ClickButton { game_state, target: ClickTarget::Carriere, label: "🪨 Pierre" }
                ClickButton { game_state, target: ClickTarget::Mine, label: "⚙️ Métal" }
                ClickButton { game_state, target: ClickTarget::Atelier, label: "🔧 Outils" }
                ClickButton { game_state, target: ClickTarget::Forge, label: "⚔️ Armes" }
            }

            // 2 colonnes : Production (alloc) | Bâtiments + Conversions
            div {
                style: "display: grid; grid-template-columns: 1fr 1fr; gap: 24px; flex: 1;",

                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    h3 { style: "font-size: 16px; color: {c.text_white};", "Production (ouvriers)" }
                    AllocationProduction { game_state, rates }
                    h3 { style: "font-size: 16px; color: {c.text_white}; margin-top: 8px;", "Chantiers (bâtisseurs)" }
                    AllocationBuilders { game_state }
                }

                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    h3 { style: "font-size: 16px; color: {c.text_white};", "Conversions" }
                    ConversionPanel { game_state }
                    h3 { style: "font-size: 16px; color: {c.text_white}; margin-top: 8px;", "Bâtiments" }
                    BuildingPanel { game_state }
                    h3 { style: "font-size: 16px; color: {c.text_white}; margin-top: 8px;", "Sauvegarde" }
                    SaveLoadPanel { game_state, data_dir: data_dir.clone() }
                }
            }
        }
    }
}

#[component]
fn SpeedButton(speed_index: Signal<usize>, index: usize, label: &'static str) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let is_active = speed_index.read() == index;
    let bg = if is_active { c.accent_blue.clone() } else { c.bg_hover.clone() };
    rsx! {
        button {
            style: "padding: 6px 10px; font-size: 12px; background: {bg}; color: white; border: none; border-radius: 4px; cursor: pointer;",
            onclick: move |_| *speed_index.write() = index,
            "{label}"
        }
    }
}

#[component]
fn ResourceChip(label: &'static str, value: String, cap: String) -> Element {
    let c = use_app_state().read().current_theme.palette();
    rsx! {
        span {
            style: "font-size: 13px; color: {c.text_primary};",
            "{label} {value} / {cap}"
        }
    }
}

#[component]
fn ClickButton(game_state: Signal<GameState>, target: ClickTarget, label: &'static str) -> Element {
    let c = use_app_state().read().current_theme.palette();
    rsx! {
        button {
            style: "padding: 8px 14px; background: {c.bg_hover}; border: 1px solid {c.border}; border-radius: 6px; color: {c.text_primary}; cursor: pointer; font-size: 13px;",
            onclick: move |_| {
                let mut g = game_state.read().clone();
                apply_click(&mut g, target);
                *game_state.write() = g;
            },
            "{label} +1"
        }
    }
}

#[component]
fn AllocationProduction(game_state: Signal<GameState>, rates: miyuclicker::state::ProductionRates) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let g = game_state.read();
    let alloc = g.allocation.clone();
    let ouvriers_libres = g.ouvriers_libres();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 6px; font-size: 13px;",
            RowAlloc { game_state, field: "ferme", label: "Ferme", value: alloc.ferme, rate: format!("{:.2}/s", rates.nourriture) }
            RowAlloc { game_state, field: "scierie", label: "Scierie", value: alloc.scierie, rate: format!("{:.2}/s", rates.bois) }
            RowAlloc { game_state, field: "carriere", label: "Carrière", value: alloc.carriere, rate: format!("{:.2}/s", rates.pierre) }
            RowAlloc { game_state, field: "mine", label: "Mine", value: alloc.mine, rate: format!("{:.2}/s", rates.metal) }
            RowAlloc { game_state, field: "atelier", label: "Atelier", value: alloc.atelier, rate: format!("{:.2}/s", rates.outils) }
            RowAlloc { game_state, field: "forge", label: "Forge", value: alloc.forge, rate: format!("{:.2}/s", rates.armes) }
            p { style: "color: {c.text_muted}; margin-top: 4px;", "Ouvriers libres: {ouvriers_libres}" }
        }
    }
}

#[component]
fn RowAlloc(
    game_state: Signal<GameState>,
    field: &'static str,
    label: &'static str,
    value: i64,
    rate: String,
) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let apply_one = move |inc: i64| {
        let mut g = game_state.read().clone();
        let mut a = g.allocation.clone();
        match field {
            "ferme" => a.ferme = (a.ferme + inc).max(0),
            "scierie" => a.scierie = (a.scierie + inc).max(0),
            "carriere" => a.carriere = (a.carriere + inc).max(0),
            "mine" => a.mine = (a.mine + inc).max(0),
            "atelier" => a.atelier = (a.atelier + inc).max(0),
            "forge" => a.forge = (a.forge + inc).max(0),
            _ => {}
        }
        if apply_allocation(&mut g, &a).is_ok() {
            *game_state.write() = g;
        }
    };

    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 8px;",
            span { style: "width: 80px; color: {c.text_primary};", "{label}" }
            button {
                style: "width: 28px; height: 28px; padding: 0; background: {c.bg_hover}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_primary}; cursor: pointer; font-size: 16px;",
                onclick: move |_| apply_one(-1),
                "−"
            }
            span { style: "min-width: 32px; text-align: center; color: {c.accent_blue};", "{value}" }
            button {
                style: "width: 28px; height: 28px; padding: 0; background: {c.bg_hover}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_primary}; cursor: pointer; font-size: 16px;",
                onclick: move |_| apply_one(1),
                "+"
            }
            span { style: "color: {c.text_muted}; font-size: 12px;", "{rate}" }
        }
    }
}

#[component]
fn AllocationBuilders(game_state: Signal<GameState>) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let g = game_state.read();
    let (m, ca, gu) = (g.allocation_batisseurs.maison, g.allocation_batisseurs.caserne, g.allocation_batisseurs.guilde_des_macons);

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 6px; font-size: 13px;",
            div {
                style: "display: flex; align-items: center; gap: 8px;",
                span { style: "width: 80px; color: {c.text_primary};", "Maisons" }
                button {
                    style: "width: 28px; height: 28px; padding: 0; background: {c.bg_hover}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_primary}; cursor: pointer;",
                    onclick: move |_| {
                        let mut g = game_state.read().clone();
                        let a = AllocationBatisseurs { maison: (m - 1).max(0), caserne: ca, guilde_des_macons: gu };
                        let _ = apply_allocation_batisseurs(&mut g, &a).map(|_| *game_state.write() = g);
                    },
                    "−"
                }
                span { style: "min-width: 32px; text-align: center; color: {c.accent_blue};", "{m}" }
                button {
                    style: "width: 28px; height: 28px; padding: 0; background: {c.bg_hover}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_primary}; cursor: pointer;",
                    onclick: move |_| {
                        let mut g = game_state.read().clone();
                        let a = AllocationBatisseurs { maison: m + 1, caserne: ca, guilde_des_macons: gu };
                        let _ = apply_allocation_batisseurs(&mut g, &a).map(|_| *game_state.write() = g);
                    },
                    "+"
                }
            }
            div {
                style: "display: flex; align-items: center; gap: 8px;",
                span { style: "width: 80px; color: {c.text_primary};", "Casernes" }
                button {
                    style: "width: 28px; height: 28px; padding: 0; background: {c.bg_hover}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_primary}; cursor: pointer;",
                    onclick: move |_| {
                        let mut g = game_state.read().clone();
                        let a = AllocationBatisseurs { maison: m, caserne: (ca - 1).max(0), guilde_des_macons: gu };
                        let _ = apply_allocation_batisseurs(&mut g, &a).map(|_| *game_state.write() = g);
                    },
                    "−"
                }
                span { style: "min-width: 32px; text-align: center; color: {c.accent_blue};", "{ca}" }
                button {
                    style: "width: 28px; height: 28px; padding: 0; background: {c.bg_hover}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_primary}; cursor: pointer;",
                    onclick: move |_| {
                        let mut g = game_state.read().clone();
                        let a = AllocationBatisseurs { maison: m, caserne: ca + 1, guilde_des_macons: gu };
                        let _ = apply_allocation_batisseurs(&mut g, &a).map(|_| *game_state.write() = g);
                    },
                    "+"
                }
            }
            div {
                style: "display: flex; align-items: center; gap: 8px;",
                span { style: "width: 80px; color: {c.text_primary};", "Guilde" }
                button {
                    style: "width: 28px; height: 28px; padding: 0; background: {c.bg_hover}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_primary}; cursor: pointer;",
                    onclick: move |_| {
                        let mut g = game_state.read().clone();
                        let a = AllocationBatisseurs { maison: m, caserne: ca, guilde_des_macons: (gu - 1).max(0) };
                        let _ = apply_allocation_batisseurs(&mut g, &a).map(|_| *game_state.write() = g);
                    },
                    "−"
                }
                span { style: "min-width: 32px; text-align: center; color: {c.accent_blue};", "{gu}" }
                button {
                    style: "width: 28px; height: 28px; padding: 0; background: {c.bg_hover}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_primary}; cursor: pointer;",
                    onclick: move |_| {
                        let mut g = game_state.read().clone();
                        let a = AllocationBatisseurs { maison: m, caserne: ca, guilde_des_macons: gu + 1 };
                        let _ = apply_allocation_batisseurs(&mut g, &a).map(|_| *game_state.write() = g);
                    },
                    "+"
                }
            }
            p { style: "color: {c.text_muted}; margin-top: 4px;", "Bâtisseurs libres: {g.batisseurs_libres()}" }
        }
    }
}

#[component]
fn ConversionPanel(game_state: Signal<GameState>) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let g = game_state.read();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 8px; font-size: 13px;",
            div {
                style: "display: flex; gap: 8px; flex-wrap: wrap;",
                button {
                    style: "padding: 6px 12px; background: {if g.can_convert_to_batisseur() { c.accent_blue } else { c.bg_hover }}; border: none; border-radius: 4px; color: white; cursor: pointer;",
                    disabled: !g.can_convert_to_batisseur(),
                    onclick: move |_| {
                        let mut g = game_state.read().clone();
                        if convert_ouvrier_to_batisseur(&mut g) { *game_state.write() = g; }
                    },
                    "Ouvrier → Bâtisseur"
                }
                button {
                    style: "padding: 6px 12px; background: {if g.can_convert_to_soldat() { c.accent_blue } else { c.bg_hover }}; border: none; border-radius: 4px; color: white; cursor: pointer;",
                    disabled: !g.can_convert_to_soldat(),
                    onclick: move |_| {
                        let mut g = game_state.read().clone();
                        if convert_ouvrier_to_soldat(&mut g) { *game_state.write() = g; }
                    },
                    "Ouvrier → Soldat"
                }
                button {
                    style: "padding: 6px 12px; background: {if g.batisseurs_libres() > 0 { c.bg_hover } else { c.bg_secondary }}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_primary}; cursor: pointer;",
                    disabled: g.batisseurs_libres() <= 0,
                    onclick: move |_| {
                        let mut g = game_state.read().clone();
                        if convert_batisseur_to_ouvrier(&mut g) { *game_state.write() = g; }
                    },
                    "Bâtisseur → Ouvrier"
                }
                button {
                    style: "padding: 6px 12px; background: {if g.soldats > 0 { c.bg_hover } else { c.bg_secondary }}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_primary}; cursor: pointer;",
                    disabled: g.soldats <= 0,
                    onclick: move |_| {
                        let mut g = game_state.read().clone();
                        if convert_soldat_to_ouvrier(&mut g) { *game_state.write() = g; }
                    },
                    "Soldat → Ouvrier"
                }
            }
            p { style: "color: {c.text_muted};", "Ouvriers: {g.ouvriers} | Bâtisseurs: {g.batisseurs} / {g.cap_batisseurs()} | Soldats: {g.soldats} / {g.cap_soldats()}" }
        }
    }
}

#[component]
fn BuildingPanel(game_state: Signal<GameState>) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let g = game_state.read();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 10px; font-size: 13px;",
            BuildingRow {
                game_state,
                bt: BuildingType::Maison,
                name: "Maisons",
                lvl: g.maisons_lvl,
                progress_pct: construction_progress_pct(g, BuildingType::Maison),
                can_start: g.can_start_construction(BuildingType::Maison),
                cost: g.cout_construction(BuildingType::Maison),
                paid: g.construction_maison_paid,
            }
            BuildingRow {
                game_state,
                bt: BuildingType::Caserne,
                name: "Casernes",
                lvl: g.caserne_lvl,
                progress_pct: construction_progress_pct(g, BuildingType::Caserne),
                can_start: g.can_start_construction(BuildingType::Caserne),
                cost: g.cout_construction(BuildingType::Caserne),
                paid: g.construction_caserne_paid,
            }
            BuildingRow {
                game_state,
                bt: BuildingType::GuildeDesMacons,
                name: "Guilde",
                lvl: g.guilde_lvl,
                progress_pct: construction_progress_pct(g, BuildingType::GuildeDesMacons),
                can_start: g.can_start_construction(BuildingType::GuildeDesMacons),
                cost: g.cout_construction(BuildingType::GuildeDesMacons),
                paid: g.construction_guilde_paid,
            }
        }
    }
}

#[component]
fn BuildingRow(
    game_state: Signal<GameState>,
    bt: BuildingType,
    name: &'static str,
    lvl: i64,
    progress_pct: f64,
    can_start: bool,
    cost: miyuclicker::state::BuildingCost,
    paid: bool,
) -> Element {
    let c = use_app_state().read().current_theme.palette();
    rsx! {
        div {
            style: "padding: 8px; background: {c.bg_hover}; border-radius: 6px;",
            div {
                style: "display: flex; justify-content: space-between; align-items: center;",
                span { style: "color: {c.text_primary};", "{name} Niv.{lvl}" }
                if paid {
                    span { style: "color: {c.accent_blue}; font-size: 12px;", "Construction {progress_pct:.0}%" }
                } else if can_start {
                    button {
                        style: "padding: 4px 10px; background: {c.accent_blue}; border: none; border-radius: 4px; color: white; cursor: pointer; font-size: 12px;",
                        onclick: move |_| {
                            let mut g = game_state.read().clone();
                            if start_construction(&mut g, bt) { *game_state.write() = g; }
                        },
                        "Construire B:{cost.bois:.0} P:{cost.pierre:.0} M:{cost.metal:.0}"
                    }
                }
            }
        }
    }
}

#[component]
fn SaveLoadPanel(game_state: Signal<GameState>, data_dir: std::path::PathBuf) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let slots = miyuclicker::save::slot_list(Path::new(&data_dir));

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 8px; font-size: 13px;",
            button {
                style: "padding: 8px 12px; background: {c.accent_blue}; border: none; border-radius: 6px; color: white; cursor: pointer;",
                onclick: move |_| {
                    let g = game_state.read().clone();
                    let _ = miyuclicker::save::slot_write(Path::new(&data_dir), g.slot_id, &g);
                },
                "Sauvegarder"
            }
            SlotRow { game_state, data_dir: data_dir.clone(), slot_id: 1, occupied: slots.get(0).map(|s| s.occupied).unwrap_or(false) }
            SlotRow { game_state, data_dir: data_dir.clone(), slot_id: 2, occupied: slots.get(1).map(|s| s.occupied).unwrap_or(false) }
            SlotRow { game_state, data_dir: data_dir.clone(), slot_id: 3, occupied: slots.get(2).map(|s| s.occupied).unwrap_or(false) }
        }
    }
}

#[component]
fn SlotRow(
    game_state: Signal<GameState>,
    data_dir: std::path::PathBuf,
    slot_id: u8,
    occupied: bool,
) -> Element {
    let c = use_app_state().read().current_theme.palette();
    rsx! {
        div {
            style: "display: flex; justify-content: space-between; align-items: center; padding: 8px; background: {c.bg_hover}; border-radius: 4px;",
            span { "Slot {slot_id}" }
            { match occupied {
                true => rsx! {
                    button {
                        style: "padding: 4px 8px; font-size: 12px; background: {c.bg_secondary}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_primary}; cursor: pointer;",
                        onclick: move |_| {
                            if let Ok(g) = miyuclicker::save::slot_read(Path::new(&data_dir), slot_id) {
                                *game_state.write() = g;
                            }
                        },
                        "Charger"
                    }
                },
                false => rsx! {
                    button {
                        style: "padding: 4px 8px; font-size: 12px; background: {c.accent_blue}; border: none; border-radius: 4px; color: white; cursor: pointer;",
                        onclick: move |_| *game_state.write() = GameState::new_game(slot_id),
                        "Nouvelle partie"
                    }
                },
            } }
        }
    }
}

fn load_or_new(data_dir: &Path) -> GameState {
    let slots = miyuclicker::save::slot_list(data_dir);
    if let Some(slot) = slots.iter().find(|s| s.occupied) {
        miyuclicker::save::slot_read(data_dir, slot.slot_id).unwrap_or_else(|_| GameState::new_game(1))
    } else {
        GameState::new_game(1)
    }
}
