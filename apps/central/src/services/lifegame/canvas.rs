//! Canvas pixel art pour afficher le monde Life Game.
//!
//! @id: central.services.lifegame.canvas
//! @role: ui
//! @layer: domain
//! @do: render_terrain_grid_as_pixel_art_with_mouse_interaction
//! @human: Rendu du terrain en grille colorée avec zoom, pan et pinceaux.

use dioxus::prelude::*;
use lifegame_world::TerrainType;
use super::{LifeGameLocalState, SelectedTool, TILE_SIZE, BG_DARK, BORDER, TEXT_MUTED};

/// Canvas principal affichant le monde.
///
/// # Performance Note
/// Cette implémentation crée un `<div>` par tile, ce qui est acceptable pour le MVP
/// avec des mondes de 256×256 tiles. Pour des mondes plus grands, il faudra :
/// - Utiliser un `<canvas>` HTML5 avec rendu direct
/// - Ou virtualiser le viewport (ne rendre que les tiles visibles)
/// - Ou passer à WebGL/WGPU pour le rendu GPU
#[component]
pub fn WorldCanvas(state: Signal<LifeGameLocalState>) -> Element {
    let s = state.read();
    let terrain = &s.terrain;
    let zoom = s.zoom;
    let selected_tool = s.selected_tool.clone();
    let brush_size = s.brush_size;

    // Calculer la taille visible (protection overflow avec saturating_mul)
    let tile_px = (TILE_SIZE as f32 * zoom) as u32;
    let world_width = terrain.width.saturating_mul(tile_px);
    let world_height = terrain.height.saturating_mul(tile_px);

    // Générer les couleurs des tiles
    let tiles_data: Vec<(u32, u32, &str)> = (0..terrain.height)
        .flat_map(|y| {
            (0..terrain.width).filter_map(move |x| {
                terrain.get(x, y).map(|tile| {
                    let color = terrain_color(tile.terrain_type);
                    (x, y, color)
                })
            })
        })
        .collect();

    rsx! {
        div {
            style: "flex: 1; overflow: hidden; position: relative; background: {BG_DARK};",

            // Conteneur scrollable
            div {
                style: "width: 100%; height: 100%; overflow: auto; position: relative;",
                
                // Zone de dessin
                div {
                    style: "position: relative; width: {world_width}px; height: {world_height}px; image-rendering: pixelated;",
                    
                    // Tiles
                    for (x, y, color) in tiles_data {
                        {
                            let left = x * tile_px;
                            let top = y * tile_px;
                            rsx! {
                                div {
                                    key: "tile_{x}_{y}",
                                    style: "position: absolute; left: {left}px; top: {top}px; width: {tile_px}px; height: {tile_px}px; background: {color};",
                                    onclick: {
                                        let selected_tool = selected_tool.clone();
                                        move |_| {
                                            apply_brush(state, x, y, &selected_tool, brush_size);
                                        }
                                    },
                                    onmousemove: {
                                        let selected_tool = selected_tool.clone();
                                        move |e: MouseEvent| {
                                            // Dessiner en maintenant le clic
                                            if e.held_buttons().contains(dioxus::html::input_data::MouseButton::Primary) {
                                                apply_brush(state, x, y, &selected_tool, brush_size);
                                            }
                                        }
                                    },
                                }
                            }
                        }
                    }

                    // Entités (unités, bâtiments, créatures)
                    {
                        let s = state.read();
                        // Conversion sécurisée : évite les valeurs négatives (overflow u32)
                        let safe_pos = |pos: f32, tile: u32| -> u32 {
                            (pos.max(0.0) * tile as f32) as u32
                        };
                        
                        let entities_elements = s.entities.units.iter().map(|(id, unit)| {
                            let left = safe_pos(unit.position.x, tile_px);
                            let top = safe_pos(unit.position.y, tile_px);
                            let color = unit_color(&unit.race);
                            rsx! {
                                div {
                                    key: "unit_{id}",
                                    style: "position: absolute; left: {left}px; top: {top}px; width: {tile_px}px; height: {tile_px}px; background: {color}; border-radius: 50%; border: 1px solid rgba(255,255,255,0.5);",
                                    title: "{unit.name} - {unit.race:?}",
                                }
                            }
                        }).collect::<Vec<_>>();

                        let buildings_elements = s.entities.buildings.iter().map(|(id, building)| {
                            let left = safe_pos(building.position.x, tile_px);
                            let top = safe_pos(building.position.y, tile_px);
                            rsx! {
                                div {
                                    key: "building_{id}",
                                    style: "position: absolute; left: {left}px; top: {top}px; width: {tile_px}px; height: {tile_px}px; background: #8B4513; border: 1px solid #D2691E;",
                                    title: "{building.building_type:?}",
                                }
                            }
                        }).collect::<Vec<_>>();

                        let creatures_elements = s.entities.creatures.iter().map(|(id, creature)| {
                            let left = safe_pos(creature.position.x, tile_px);
                            let top = safe_pos(creature.position.y, tile_px);
                            let color = creature_color(&creature.creature_type);
                            rsx! {
                                div {
                                    key: "creature_{id}",
                                    style: "position: absolute; left: {left}px; top: {top}px; width: {tile_px}px; height: {tile_px}px; background: {color}; border-radius: 25%;",
                                    title: "{creature.creature_type:?}",
                                }
                            }
                        }).collect::<Vec<_>>();

                        rsx! {
                            {entities_elements.into_iter()}
                            {buildings_elements.into_iter()}
                            {creatures_elements.into_iter()}
                        }
                    }
                }
            }

            // Contrôles de zoom
            div {
                style: "position: absolute; bottom: 16px; right: 16px; display: flex; gap: 4px;",
                button {
                    style: "width: 32px; height: 32px; background: rgba(0,0,0,0.7); border: 1px solid {BORDER}; border-radius: 4px; color: {TEXT_MUTED}; cursor: pointer; font-size: 16px;",
                    onclick: move |_| {
                        let mut s = state.write();
                        s.zoom = (s.zoom - 0.25).max(0.25);
                    },
                    "−"
                }
                span {
                    style: "display: flex; align-items: center; padding: 0 8px; background: rgba(0,0,0,0.7); border: 1px solid {BORDER}; border-radius: 4px; color: {TEXT_MUTED}; font-size: 11px;",
                    "{(zoom * 100.0) as u32}%"
                }
                button {
                    style: "width: 32px; height: 32px; background: rgba(0,0,0,0.7); border: 1px solid {BORDER}; border-radius: 4px; color: {TEXT_MUTED}; cursor: pointer; font-size: 16px;",
                    onclick: move |_| {
                        let mut s = state.write();
                        s.zoom = (s.zoom + 0.25).min(4.0);
                    },
                    "+"
                }
            }

            // Indicateur de position
            div {
                style: "position: absolute; bottom: 16px; left: 16px; padding: 4px 8px; background: rgba(0,0,0,0.7); border: 1px solid {BORDER}; border-radius: 4px; font-size: 10px; color: {TEXT_MUTED};",
                "Monde: {terrain.width}x{terrain.height}"
            }
        }
    }
}

/// Applique un pinceau sur le terrain.
fn apply_brush(mut state: Signal<LifeGameLocalState>, x: u32, y: u32, tool: &SelectedTool, brush_size: u32) {
    let mut s = state.write();
    
    match tool {
        SelectedTool::Terrain(terrain_type) => {
            // Dessiner sur la zone du pinceau (brush_size=1 → 1 tile, brush_size=3 → 3x3 tiles)
            // Calcul du rayon: pour brush_size=1 → radius=0, brush_size=3 → radius=1
            let radius = (brush_size.saturating_sub(1) / 2) as i32;
            for dy in -radius..=radius {
                for dx in -radius..=radius {
                    let tx = (x as i32 + dx).max(0) as u32;
                    let ty = (y as i32 + dy).max(0) as u32;
                    if tx < s.terrain.width && ty < s.terrain.height {
                        s.terrain.set_terrain(tx, ty, *terrain_type);
                    }
                }
            }
        }
        SelectedTool::Power(_power_id) => {
            // TODO: Appliquer le pouvoir divin
        }
        SelectedTool::Select => {
            // TODO: Sélectionner l'entité à cette position
        }
    }
}

/// Retourne la couleur CSS pour un type de terrain.
fn terrain_color(terrain: TerrainType) -> &'static str {
    match terrain {
        TerrainType::DeepWater => "#1e3c96",
        TerrainType::ShallowWater => "#4682b4",
        TerrainType::Beach => "#eed6af",
        TerrainType::Grass => "#56b04c",
        TerrainType::Forest => "#228b22",
        TerrainType::Hill => "#8b7765",
        TerrainType::Mountain => "#808080",
        TerrainType::Snow => "#fffafa",
        TerrainType::Desert => "#d2b48c",
        TerrainType::Swamp => "#556b2f",
        TerrainType::Lava => "#ff4500",
    }
}

/// Retourne la couleur pour une race d'unité.
fn unit_color(race: &lifegame_entities::Race) -> &'static str {
    match race {
        lifegame_entities::Race::Human => "#f5deb3",
        lifegame_entities::Race::Orc => "#228b22",
        lifegame_entities::Race::Elf => "#87ceeb",
        lifegame_entities::Race::Dwarf => "#cd853f",
    }
}

/// Retourne la couleur pour un type de créature.
fn creature_color(creature_type: &lifegame_entities::CreatureType) -> &'static str {
    use lifegame_entities::CreatureType;
    match creature_type {
        CreatureType::Deer => "#c19a6b",
        CreatureType::Sheep => "#f5f5dc",
        CreatureType::Cow => "#8b4513",
        CreatureType::Horse => "#654321",
        CreatureType::Wolf => "#808080",
        CreatureType::Bear => "#8b4513",
        CreatureType::Fish => "#4682b4",
        CreatureType::Zombie => "#556b2f",
        CreatureType::Skeleton => "#f5f5dc",
        CreatureType::Demon => "#8b0000",
        CreatureType::Dragon => "#ff4500",
        CreatureType::ColdOne => "#87ceeb",
        CreatureType::Crabzilla => "#ff6347",
        CreatureType::Unicorn => "#fffacd",
        CreatureType::Treant => "#228b22",
        CreatureType::Fairy => "#ff69b4",
        CreatureType::Golem => "#696969",
    }
}
