//! # LifeGame Entities — Opérateur Entités
//!
//! Gestion des entités du monde : unités civilisées, bâtiments, créatures.
//!
//! @id: lifegame.operator.entities
//! @role: simulation
//! @layer: domain
//! @do: manage_world_entities_units_buildings_creatures
//! @human: Opérateur gérant toutes les entités vivantes et construites du monde

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod unit;
pub mod building;
pub mod creature;

pub use unit::{Unit, UnitRole, Race, Trait, FamilyTree};
pub use building::{Building, BuildingType};
pub use creature::{Creature, CreatureType};

/// Identifiant unique d'une entité
pub type EntityId = u64;

/// Identifiant d'un royaume
pub type KingdomId = u64;

/// Position 2D dans le monde
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    #[must_use]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    #[must_use]
    pub fn distance(&self, other: &Self) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    #[must_use]
    pub fn splat(value: f32) -> Self {
        Self { x: value, y: value }
    }
}

impl std::ops::Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

/// Gestionnaire centralisé de toutes les entités
///
/// @id: lifegame.entities.manager
/// @do: centralize_entity_storage_and_queries
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntityManager {
    /// Toutes les unités civilisées
    pub units: HashMap<EntityId, Unit>,
    /// Tous les bâtiments
    pub buildings: HashMap<EntityId, Building>,
    /// Toutes les créatures (animaux, monstres)
    pub creatures: HashMap<EntityId, Creature>,
    /// Prochain ID à attribuer
    next_id: EntityId,
}

impl EntityManager {
    /// Crée un nouveau gestionnaire d'entités vide
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Génère un nouvel ID unique
    fn next_entity_id(&mut self) -> EntityId {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    /// Ajoute une unité et retourne son ID
    pub fn spawn_unit(&mut self, mut unit: Unit) -> EntityId {
        let id = self.next_entity_id();
        unit.id = id;
        self.units.insert(id, unit);
        id
    }

    /// Supprime une unité (mort)
    pub fn kill_unit(&mut self, id: EntityId) {
        self.units.remove(&id);
    }

    /// Ajoute un bâtiment et retourne son ID
    pub fn spawn_building(&mut self, mut building: Building) -> EntityId {
        let id = self.next_entity_id();
        building.id = id;
        self.buildings.insert(id, building);
        id
    }

    /// Supprime un bâtiment (détruit)
    pub fn destroy_building(&mut self, id: EntityId) {
        self.buildings.remove(&id);
    }

    /// Ajoute une créature et retourne son ID
    pub fn spawn_creature(&mut self, mut creature: Creature) -> EntityId {
        let id = self.next_entity_id();
        creature.id = id;
        self.creatures.insert(id, creature);
        id
    }

    /// Supprime une créature (morte)
    pub fn kill_creature(&mut self, id: EntityId) {
        self.creatures.remove(&id);
    }

    /// Récupère les unités dans un rayon donné
    #[must_use]
    pub fn units_in_radius(&self, center: Vec2, radius: f32) -> Vec<&Unit> {
        self.units
            .values()
            .filter(|u| u.position.distance(&center) <= radius)
            .collect()
    }

    /// Récupère les bâtiments dans un rayon donné
    #[must_use]
    pub fn buildings_in_radius(&self, center: Vec2, radius: f32) -> Vec<&Building> {
        self.buildings
            .values()
            .filter(|b| b.position.distance(&center) <= radius)
            .collect()
    }

    /// Compte le nombre total d'entités
    #[must_use]
    pub fn total_count(&self) -> usize {
        self.units.len() + self.buildings.len() + self.creatures.len()
    }

    /// Compte le nombre de créatures d'un type donné
    #[must_use]
    pub fn count_creatures_of_type(&self, creature_type: CreatureType) -> usize {
        self.creatures
            .values()
            .filter(|c| c.creature_type == creature_type)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec2_distance() {
        let a = Vec2::new(0.0, 0.0);
        let b = Vec2::new(3.0, 4.0);
        assert!((a.distance(&b) - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_entity_manager_spawn() {
        let mut manager = EntityManager::new();
        let unit = Unit::default();
        let id = manager.spawn_unit(unit);
        assert_eq!(id, 0);
        assert_eq!(manager.units.len(), 1);
    }
}
