//! # Intelligence Artificielle
//!
//! Logique de décision des unités.
//!
//! @id: lifegame.simulation.ai
//! @role: simulation
//! @layer: domain
//! @do: implement_unit_decision_making
//! @human: Système de décision pour les unités civilisées

use lifegame_entities::{Unit, EntityManager, Vec2};
use lifegame_world::TerrainGrid;
use crate::UnitAction;

/// Décide l'action d'une unité basée sur ses besoins et son environnement
///
/// @id: lifegame.ai.decide_action
/// @do: determine_best_action_for_unit
#[must_use]
pub fn decide_unit_action(unit: &Unit, entities: &EntityManager, terrain: &TerrainGrid) -> UnitAction {
    // Priorité 1 : Survie
    if unit.health < 20 {
        // Cherche à fuir si en danger
        return UnitAction::Flee(find_safe_position(unit, terrain));
    }

    // Priorité 2 : Besoins vitaux
    if unit.hunger < 20 {
        if let Some(food_pos) = find_nearest_food(unit, entities) {
            return UnitAction::MoveTo(food_pos);
        }
        return UnitAction::FindFood;
    }

    if unit.thirst < 20 {
        if let Some(water_pos) = find_nearest_water(unit, terrain) {
            return UnitAction::MoveTo(water_pos);
        }
        return UnitAction::FindWater;
    }

    if unit.energy < 20 {
        return UnitAction::Rest;
    }

    // Priorité 3 : Travail
    // TODO: Implémenter la logique de travail

    // Par défaut : errer
    UnitAction::Wander
}

/// Trouve une position sûre pour fuir
fn find_safe_position(unit: &Unit, terrain: &TerrainGrid) -> Vec2 {
    // Fuit dans une direction aléatoire mais valide
    let flee_distance = 20.0;
    for _ in 0..8 {
        let angle = rand::random::<f32>() * std::f32::consts::TAU;
        let target = Vec2::new(
            unit.position.x + flee_distance * angle.cos(),
            unit.position.y + flee_distance * angle.sin(),
        );

        if is_valid_position(target, terrain) {
            return target;
        }
    }
    
    // Si aucune position valide, reste sur place
    unit.position
}

/// Trouve la source de nourriture la plus proche
fn find_nearest_food(unit: &Unit, entities: &EntityManager) -> Option<Vec2> {
    // Cherche les fermes complétées
    let farms: Vec<&lifegame_entities::Building> = entities.buildings
        .values()
        .filter(|b| b.building_type == lifegame_entities::BuildingType::Farm && b.is_completed)
        .collect();

    // Trouve la plus proche avec des ressources
    farms
        .iter()
        .filter(|f| f.stored_resources > 0)
        .min_by(|a, b| {
            let dist_a = unit.position.distance(&a.position);
            let dist_b = unit.position.distance(&b.position);
            dist_a.partial_cmp(&dist_b).unwrap()
        })
        .map(|f| f.position)
}

/// Trouve la source d'eau la plus proche
fn find_nearest_water(unit: &Unit, terrain: &TerrainGrid) -> Option<Vec2> {
    let search_radius = 30;
    let ux = unit.position.x as i32;
    let uy = unit.position.y as i32;

    let mut best_pos = None;
    let mut best_dist = f32::INFINITY;

    for dy in -search_radius..=search_radius {
        for dx in -search_radius..=search_radius {
            let x = (ux + dx) as u32;
            let y = (uy + dy) as u32;

            if let Some(tile) = terrain.get(x, y) {
                if matches!(tile.terrain_type, lifegame_world::TerrainType::ShallowWater) {
                    let pos = Vec2::new(x as f32, y as f32);
                    let dist = unit.position.distance(&pos);
                    if dist < best_dist {
                        best_dist = dist;
                        best_pos = Some(pos);
                    }
                }
            }
        }
    }

    best_pos
}

/// Vérifie si une position est valide
fn is_valid_position(pos: Vec2, terrain: &TerrainGrid) -> bool {
    if pos.x < 0.0 || pos.y < 0.0 {
        return false;
    }
    
    let x = pos.x as u32;
    let y = pos.y as u32;
    
    if x >= terrain.width || y >= terrain.height {
        return false;
    }

    terrain.get(x, y)
        .map(|t| t.is_walkable())
        .unwrap_or(false)
}

/// Calcule la distance entre deux positions
#[must_use]
pub fn distance(a: Vec2, b: Vec2) -> f32 {
    a.distance(&b)
}

/// Calcule la direction normalisée entre deux positions
#[must_use]
pub fn direction(from: Vec2, to: Vec2) -> Vec2 {
    let dx = to.x - from.x;
    let dy = to.y - from.y;
    let dist = (dx * dx + dy * dy).sqrt();
    
    if dist < 0.001 {
        Vec2::default()
    } else {
        Vec2::new(dx / dist, dy / dist)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction() {
        let from = Vec2::new(0.0, 0.0);
        let to = Vec2::new(1.0, 0.0);
        let dir = direction(from, to);
        assert!((dir.x - 1.0).abs() < 0.001);
        assert!(dir.y.abs() < 0.001);
    }

    #[test]
    fn test_distance() {
        let a = Vec2::new(0.0, 0.0);
        let b = Vec2::new(3.0, 4.0);
        assert!((distance(a, b) - 5.0).abs() < 0.001);
    }
}
