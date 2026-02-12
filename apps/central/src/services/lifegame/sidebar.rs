//! Sidebar des pouvoirs divins pour Life Game.
//!
//! @id: central.services.lifegame.sidebar
//! @role: ui
//! @layer: domain
//! @do: render_divine_powers_palette_with_terrain_brushes
//! @human: Palette des pouvoirs divins et pinceaux de terrain.

use dioxus::prelude::*;
use lifegame_world::TerrainType;
use super::{LifeGameLocalState, SelectedTool, BG_PANEL, BORDER, ACCENT, TEXT_LIGHT, TEXT_MUTED};

/// Catégorie d'outils.
#[derive(Clone, Debug, PartialEq)]
enum ToolCategory {
    Terrain,
    Creation,
    Destruction,
    Creatures,
}

/// Sidebar avec les pouvoirs et pinceaux.
#[component]
pub fn PowersSidebar(state: Signal<LifeGameLocalState>) -> Element {
    let mut active_category = use_signal(|| ToolCategory::Terrain);
    let s = state.read();
    let selected_tool = s.selected_tool.clone();
    let brush_size = s.brush_size;

    rsx! {
        div {
            style: "width: 200px; background: {BG_PANEL}; border-right: 1px solid {BORDER}; display: flex; flex-direction: column; flex-shrink: 0;",

            // Onglets catégories
            div {
                style: "display: flex; border-bottom: 1px solid {BORDER};",
                CategoryTab {
                    label: "🌍",
                    active: *active_category.read() == ToolCategory::Terrain,
                    onclick: move |_| active_category.set(ToolCategory::Terrain),
                }
                CategoryTab {
                    label: "✨",
                    active: *active_category.read() == ToolCategory::Creation,
                    onclick: move |_| active_category.set(ToolCategory::Creation),
                }
                CategoryTab {
                    label: "💥",
                    active: *active_category.read() == ToolCategory::Destruction,
                    onclick: move |_| active_category.set(ToolCategory::Destruction),
                }
                CategoryTab {
                    label: "🐾",
                    active: *active_category.read() == ToolCategory::Creatures,
                    onclick: move |_| active_category.set(ToolCategory::Creatures),
                }
            }

            // Titre catégorie
            div {
                style: "padding: 12px; border-bottom: 1px solid {BORDER};",
                span {
                    style: "font-size: 12px; font-weight: 600; color: {TEXT_LIGHT}; text-transform: uppercase; letter-spacing: 1px;",
                    match *active_category.read() {
                        ToolCategory::Terrain => "Terrain",
                        ToolCategory::Creation => "Création",
                        ToolCategory::Destruction => "Destruction",
                        ToolCategory::Creatures => "Créatures",
                    }
                }
            }

            // Liste des outils
            div {
                style: "flex: 1; overflow-y: auto; padding: 8px;",

                match *active_category.read() {
                    ToolCategory::Terrain => rsx! {
                        TerrainTools { state: state, selected_tool: selected_tool.clone() }
                    },
                    ToolCategory::Creation => rsx! {
                        CreationPowers { state: state }
                    },
                    ToolCategory::Destruction => rsx! {
                        DestructionPowers { state: state }
                    },
                    ToolCategory::Creatures => rsx! {
                        CreatureSpawners { state: state }
                    },
                }
            }

            // Taille du pinceau
            div {
                style: "padding: 12px; border-top: 1px solid {BORDER};",
                span { style: "font-size: 10px; color: {TEXT_MUTED}; display: block; margin-bottom: 6px;", "Taille pinceau" }
                div {
                    style: "display: flex; gap: 4px;",
                    for size in [1u32, 2, 3, 5, 10] {
                        {
                            let is_active = brush_size == size;
                            let bg = if is_active { ACCENT } else { "rgba(255,255,255,0.1)" };
                            rsx! {
                                button {
                                    style: "flex: 1; padding: 6px 0; background: {bg}; border: none; border-radius: 4px; cursor: pointer; font-size: 11px; color: {TEXT_LIGHT};",
                                    onclick: move |_| {
                                        state.write().brush_size = size;
                                    },
                                    "{size}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Onglet de catégorie.
#[component]
fn CategoryTab(label: &'static str, active: bool, onclick: EventHandler<MouseEvent>) -> Element {
    let bg = if active { ACCENT } else { "transparent" };
    let border_color = if active { ACCENT } else { "transparent" };
    
    rsx! {
        button {
            style: "flex: 1; padding: 10px 0; background: {bg}; border: none; border-bottom: 2px solid {border_color}; cursor: pointer; font-size: 16px; transition: all 0.15s;",
            onclick: move |e| onclick.call(e),
            "{label}"
        }
    }
}

/// Outils de terrain.
#[component]
fn TerrainTools(state: Signal<LifeGameLocalState>, selected_tool: SelectedTool) -> Element {
    let terrains = [
        (TerrainType::DeepWater, "🌊", "Eau profonde"),
        (TerrainType::ShallowWater, "💧", "Eau peu profonde"),
        (TerrainType::Beach, "🏖️", "Plage"),
        (TerrainType::Grass, "🌿", "Plaines"),
        (TerrainType::Forest, "🌲", "Forêt"),
        (TerrainType::Hill, "⛰️", "Collines"),
        (TerrainType::Mountain, "🏔️", "Montagnes"),
        (TerrainType::Snow, "❄️", "Neige"),
        (TerrainType::Desert, "🏜️", "Désert"),
        (TerrainType::Swamp, "🌾", "Marais"),
        (TerrainType::Lava, "🔥", "Lave"),
    ];

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 4px;",
            for (terrain, icon, name) in terrains {
                {
                    let is_selected = matches!(&selected_tool, SelectedTool::Terrain(t) if *t == terrain);
                    let bg = if is_selected { "rgba(233,69,96,0.3)" } else { "rgba(255,255,255,0.05)" };
                    let border_color = if is_selected { ACCENT } else { BORDER };
                    rsx! {
                        button {
                            style: "display: flex; align-items: center; gap: 8px; padding: 8px; background: {bg}; border: 1px solid {border_color}; border-radius: 4px; cursor: pointer; text-align: left;",
                            onclick: move |_| {
                                state.write().selected_tool = SelectedTool::Terrain(terrain);
                            },
                            span { style: "font-size: 14px;", "{icon}" }
                            span { style: "font-size: 11px; color: {TEXT_LIGHT};", "{name}" }
                        }
                    }
                }
            }
        }
    }
}

/// Pouvoirs de création.
#[component]
fn CreationPowers(state: Signal<LifeGameLocalState>) -> Element {
    let powers = [
        ("spawn_human", "👤", "Spawn Humain", "Crée un humain"),
        ("spawn_orc", "👹", "Spawn Orc", "Crée un orc"),
        ("spawn_elf", "🧝", "Spawn Elfe", "Crée un elfe"),
        ("spawn_dwarf", "🧔", "Spawn Nain", "Crée un nain"),
        ("spawn_village", "🏘️", "Village", "Crée un village"),
        ("spawn_tree", "🌳", "Arbres", "Plante des arbres"),
        ("spawn_gold", "💎", "Ressources", "Ajoute des ressources"),
    ];

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 4px;",
            for (id, icon, name, desc) in powers {
                PowerButton {
                    state: state,
                    power_id: id,
                    icon: icon,
                    name: name,
                    description: desc,
                }
            }
        }
    }
}

/// Pouvoirs de destruction.
#[component]
fn DestructionPowers(state: Signal<LifeGameLocalState>) -> Element {
    let powers = [
        ("earthquake", "🌍", "Séisme", "Secoue le terrain"),
        ("meteor", "☄️", "Météorite", "Impact dévastateur"),
        ("flood", "🌊", "Inondation", "Noie la région"),
        ("fire", "🔥", "Incendie", "Brûle tout"),
        ("plague", "☠️", "Peste", "Maladie mortelle"),
        ("lightning", "⚡", "Foudre", "Frappe éclair"),
        ("volcano", "🌋", "Éruption", "Lave et cendres"),
    ];

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 4px;",
            for (id, icon, name, desc) in powers {
                PowerButton {
                    state: state,
                    power_id: id,
                    icon: icon,
                    name: name,
                    description: desc,
                }
            }
        }
    }
}

/// Spawners de créatures.
#[component]
fn CreatureSpawners(state: Signal<LifeGameLocalState>) -> Element {
    // Note: Les types doivent correspondre à lifegame_entities::CreatureType
    let creatures = [
        ("spawn_wolf", "🐺", "Loup", "Prédateur rapide"),
        ("spawn_bear", "🐻", "Ours", "Puissant et territorial"),
        ("spawn_dragon", "🐉", "Dragon", "Terreur légendaire"),
        ("spawn_demon", "👿", "Démon", "Créature infernale"),
        ("spawn_unicorn", "🦄", "Licorne", "Guérit le terrain"),
        ("spawn_zombie", "🧟", "Zombie", "Mort-vivant"),
        ("spawn_golem", "🗿", "Golem", "Gardien de pierre"),
        ("spawn_fairy", "🧚", "Fée", "Accélère croissance"),
        ("spawn_treant", "🌳", "Treant", "Gardien de forêt"),
    ];

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 4px;",
            for (id, icon, name, desc) in creatures {
                PowerButton {
                    state: state,
                    power_id: id,
                    icon: icon,
                    name: name,
                    description: desc,
                }
            }
        }
    }
}

/// Bouton de pouvoir divin.
#[component]
fn PowerButton(
    state: Signal<LifeGameLocalState>,
    power_id: &'static str,
    icon: &'static str,
    name: &'static str,
    description: &'static str,
) -> Element {
    let s = state.read();
    let is_selected = matches!(&s.selected_tool, SelectedTool::Power(id) if id == power_id);
    let bg = if is_selected { "rgba(233,69,96,0.3)" } else { "rgba(255,255,255,0.05)" };
    let border_color = if is_selected { ACCENT } else { BORDER };

    rsx! {
        button {
            style: "display: flex; align-items: center; gap: 8px; padding: 8px; background: {bg}; border: 1px solid {border_color}; border-radius: 4px; cursor: pointer; text-align: left;",
            onclick: move |_| {
                state.write().selected_tool = SelectedTool::Power(power_id.to_string());
            },
            span { style: "font-size: 16px;", "{icon}" }
            div {
                style: "display: flex; flex-direction: column;",
                span { style: "font-size: 11px; color: {TEXT_LIGHT}; font-weight: 500;", "{name}" }
                span { style: "font-size: 9px; color: {TEXT_MUTED};", "{description}" }
            }
        }
    }
}
