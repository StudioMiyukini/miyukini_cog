//! # Tick du monde
//!
//! Logique de simulation pour chaque tick.
//!
//! @id: lifegame.simulation.world_tick
//! @role: simulation
//! @layer: domain
//! @do: execute_world_simulation_tick
//! @human: Exécute toutes les mises à jour pour un tick de simulation

use lifegame_entities::{EntityManager, BuildingType};
use lifegame_world::TerrainGrid;

/// Résultat d'un tick de simulation
#[derive(Debug, Clone, Default)]
pub struct TickResult {
    /// Unités mortes ce tick
    pub units_died: Vec<u64>,
    /// Unités nées ce tick
    pub units_born: Vec<u64>,
    /// Bâtiments détruits ce tick
    pub buildings_destroyed: Vec<u64>,
    /// Bâtiments complétés ce tick
    pub buildings_completed: Vec<u64>,
    /// Ressources produites (type, quantité)
    pub resources_produced: Vec<(String, u32)>,
}

/// Exécute un tick de simulation sur le monde
///
/// @id: lifegame.simulation.tick_world_fn
/// @do: process_single_simulation_tick
pub fn tick_world(
    entities: &mut EntityManager,
    terrain: &TerrainGrid,
    is_new_day: bool,
) -> TickResult {
    let mut result = TickResult::default();

    // 1. Mise à jour des besoins des unités
    tick_unit_needs(entities, &mut result);

    // 2. Production des bâtiments (une fois par jour)
    if is_new_day {
        tick_building_production(entities, &mut result);
    }

    // 3. Avancement de la construction
    tick_construction(entities, &mut result);

    // 4. IA des unités (décisions)
    tick_unit_ai(entities, terrain);

    // 5. Mouvement des unités
    tick_unit_movement(entities, terrain);

    // 6. Suppression des entités mortes
    cleanup_dead_entities(entities, &mut result);

    result
}

/// Met à jour les besoins des unités (faim, soif, énergie)
fn tick_unit_needs(entities: &mut EntityManager, result: &mut TickResult) {
    let ids: Vec<u64> = entities.units.keys().copied().collect();
    
    for id in ids {
        if let Some(unit) = entities.units.get_mut(&id) {
            // Diminution des besoins (par minute de jeu)
            // Une journée = 1440 ticks, donc les besoins diminuent lentement
            if rand::random::<u8>() < 10 {
                unit.hunger = unit.hunger.saturating_sub(1);
            }
            if rand::random::<u8>() < 15 {
                unit.thirst = unit.thirst.saturating_sub(1);
            }
            if rand::random::<u8>() < 5 {
                unit.energy = unit.energy.saturating_sub(1);
            }

            // Dégâts de famine
            if unit.hunger == 0 {
                unit.days_without_food = unit.days_without_food.saturating_add(1);
                if unit.days_without_food > 0 && rand::random::<u8>() < 10 {
                    unit.health = unit.health.saturating_sub(5);
                }
            } else {
                unit.days_without_food = 0;
            }

            // Dégâts de déshydratation
            if unit.thirst == 0 {
                unit.days_without_water = unit.days_without_water.saturating_add(1);
                if unit.days_without_water > 0 && rand::random::<u8>() < 20 {
                    unit.health = unit.health.saturating_sub(10);
                }
            } else {
                unit.days_without_water = 0;
            }

            // Vérification de la mort
            if unit.should_die() {
                result.units_died.push(id);
            }
        }
    }
}

/// Fait produire les bâtiments (fermes, mines)
fn tick_building_production(entities: &mut EntityManager, result: &mut TickResult) {
    for building in entities.buildings.values_mut() {
        if building.can_produce() {
            let produced = building.produce();
            if produced > 0 {
                let resource_name = match building.building_type {
                    BuildingType::Farm => "food",
                    BuildingType::Mine => "ore",
                    BuildingType::Well => "water",
                    _ => "unknown",
                };
                result.resources_produced.push((resource_name.to_string(), produced));
            }
        }
    }
}

/// Avance la construction des bâtiments
fn tick_construction(entities: &mut EntityManager, result: &mut TickResult) {
    for building in entities.buildings.values_mut() {
        if building.is_under_construction() {
            // Construction avance de 1% par tick si quelqu'un travaille dessus
            // (simplifiée pour le MVP — avance automatiquement)
            building.advance_construction(1);
            
            if building.is_completed {
                result.buildings_completed.push(building.id);
            }
        }
    }
}

/// IA basique des unités
fn tick_unit_ai(entities: &mut EntityManager, terrain: &TerrainGrid) {
    let ids: Vec<u64> = entities.units.keys().copied().collect();
    
    for id in ids {
        if let Some(unit) = entities.units.get_mut(&id) {
            // Si l'unité n'a pas de cible, elle erre
            if unit.target_position.is_none() {
                // Décide d'une nouvelle cible aléatoire
                let current = unit.position;
                let range = 20.0;
                let new_target = lifegame_entities::Vec2::new(
                    current.x + (rand::random::<f32>() - 0.5) * range * 2.0,
                    current.y + (rand::random::<f32>() - 0.5) * range * 2.0,
                );
                
                // Vérifie que la cible est dans les limites
                if new_target.x >= 0.0 
                    && new_target.y >= 0.0 
                    && (new_target.x as u32) < terrain.width 
                    && (new_target.y as u32) < terrain.height 
                {
                    // Vérifie que la cible est marchable
                    if let Some(tile) = terrain.get(new_target.x as u32, new_target.y as u32) {
                        if tile.is_walkable() {
                            unit.target_position = Some(new_target);
                        }
                    }
                }
            }
        }
    }
}

/// Déplace les unités vers leur cible
fn tick_unit_movement(entities: &mut EntityManager, _terrain: &TerrainGrid) {
    for unit in entities.units.values_mut() {
        if let Some(target) = unit.target_position {
            let dx = target.x - unit.position.x;
            let dy = target.y - unit.position.y;
            let distance = (dx * dx + dy * dy).sqrt();

            if distance < 1.0 {
                // Arrivé à destination
                unit.position = target;
                unit.target_position = None;
                unit.velocity = lifegame_entities::Vec2::default();
            } else {
                // Se déplace vers la cible
                let speed = unit.speed * 0.1; // Ajuste la vitesse
                let vx = (dx / distance) * speed;
                let vy = (dy / distance) * speed;
                
                unit.velocity = lifegame_entities::Vec2::new(vx, vy);
                unit.position.x += vx;
                unit.position.y += vy;
            }
        }
    }
}

/// Supprime les entités mortes
fn cleanup_dead_entities(entities: &mut EntityManager, result: &mut TickResult) {
    // Supprime les unités mortes
    for &id in &result.units_died {
        entities.kill_unit(id);
    }

    // Supprime les bâtiments détruits
    for &id in &result.buildings_destroyed {
        entities.destroy_building(id);
    }

    // Supprime les créatures mortes
    let dead_creatures: Vec<u64> = entities.creatures
        .iter()
        .filter(|(_, c)| !c.is_alive())
        .map(|(id, _)| *id)
        .collect();
    
    for id in dead_creatures {
        entities.kill_creature(id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lifegame_entities::Unit;
    use lifegame_world::WorldSize;

    #[test]
    fn test_tick_world() {
        let mut entities = EntityManager::new();
        let terrain = lifegame_world::generation::generate_simple_world(WorldSize::Small);
        
        // Ajoute une unité
        let mut unit = Unit::new("Test", lifegame_entities::Race::Human);
        unit.position = lifegame_entities::Vec2::new(128.0, 128.0);
        entities.spawn_unit(unit);

        // Exécute un tick
        let result = tick_world(&mut entities, &terrain, false);
        
        // L'unité devrait être toujours vivante
        assert!(result.units_died.is_empty());
        assert_eq!(entities.units.len(), 1);
    }
}
