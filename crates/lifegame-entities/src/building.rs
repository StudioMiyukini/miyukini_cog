//! # Bâtiments
//!
//! Représente les structures construites par les civilisations.
//!
//! @id: lifegame.entities.building
//! @role: simulation
//! @layer: domain
//! @do: define_building_structures_and_types
//! @human: Structures et types de bâtiments constructibles

use serde::{Deserialize, Serialize};
use crate::{EntityId, KingdomId, Vec2};

/// Type de bâtiment
///
/// @id: lifegame.entities.building_type
/// @do: enumerate_building_types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum BuildingType {
    /// Maison — logement pour 4 unités
    #[default]
    House,
    /// Ferme — produit de la nourriture
    Farm,
    /// Puits — source d'eau
    Well,
    /// Mine — extrait des minerais
    Mine,
    /// Caserne — entraîne des soldats
    Barracks,
    /// Muraille — défense
    Wall,
    /// Tour de garde — vision et défense
    WatchTower,
    /// Marché — commerce
    Market,
    /// Bibliothèque — recherche
    Library,
    /// Temple — religion et bonheur
    Temple,
    /// Château — capitale du royaume
    Castle,
}

impl BuildingType {
    /// Coût en bois pour construire
    #[must_use]
    pub const fn wood_cost(&self) -> u32 {
        match self {
            Self::House => 20,
            Self::Farm => 10,
            Self::Well => 5,
            Self::Mine => 15,
            Self::Barracks => 50,
            Self::Wall => 10,
            Self::WatchTower => 30,
            Self::Market => 50,
            Self::Library => 80,
            Self::Temple => 40,
            Self::Castle => 200,
        }
    }

    /// Coût en pierre pour construire
    #[must_use]
    pub const fn stone_cost(&self) -> u32 {
        match self {
            Self::House => 0,
            Self::Farm => 0,
            Self::Well => 10,
            Self::Mine => 30,
            Self::Barracks => 20,
            Self::Wall => 100,
            Self::WatchTower => 10,
            Self::Market => 20,
            Self::Library => 40,
            Self::Temple => 100,
            Self::Castle => 500,
        }
    }

    /// Temps de construction en jours de jeu
    #[must_use]
    pub const fn build_time(&self) -> u32 {
        match self {
            Self::House => 3,
            Self::Farm => 2,
            Self::Well => 1,
            Self::Mine => 5,
            Self::Barracks => 7,
            Self::Wall => 2,
            Self::WatchTower => 4,
            Self::Market => 5,
            Self::Library => 10,
            Self::Temple => 8,
            Self::Castle => 30,
        }
    }

    /// Taille visuelle du bâtiment (en tuiles)
    #[must_use]
    pub const fn size(&self) -> (u32, u32) {
        match self {
            Self::House => (2, 2),
            Self::Farm => (3, 3),
            Self::Well => (1, 1),
            Self::Mine => (2, 2),
            Self::Barracks => (3, 2),
            Self::Wall => (1, 1),
            Self::WatchTower => (2, 2),
            Self::Market => (3, 3),
            Self::Library => (3, 2),
            Self::Temple => (3, 3),
            Self::Castle => (5, 5),
        }
    }

    /// Points de vie maximum
    #[must_use]
    pub const fn max_health(&self) -> u32 {
        match self {
            Self::House => 100,
            Self::Farm => 50,
            Self::Well => 30,
            Self::Mine => 80,
            Self::Barracks => 150,
            Self::Wall => 200,
            Self::WatchTower => 100,
            Self::Market => 80,
            Self::Library => 60,
            Self::Temple => 120,
            Self::Castle => 500,
        }
    }

    /// Production quotidienne (nourriture pour ferme, etc.)
    #[must_use]
    pub const fn daily_production(&self) -> u32 {
        match self {
            Self::Farm => 5,  // 5 nourriture/jour
            Self::Mine => 2,  // 2 minerai/jour
            Self::Well => 10, // 10 eau/jour
            _ => 0,
        }
    }

    /// Capacité (logement pour maison, soldats pour caserne)
    #[must_use]
    pub const fn capacity(&self) -> u32 {
        match self {
            Self::House => 4,     // 4 unités
            Self::Barracks => 10, // 10 soldats
            Self::Castle => 20,   // 20 unités
            _ => 0,
        }
    }
}

/// Bâtiment construit
///
/// @id: lifegame.entities.building_struct
/// @do: represent_constructed_building_with_state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Building {
    /// Identifiant unique
    pub id: EntityId,
    /// Type de bâtiment
    pub building_type: BuildingType,
    /// Position dans le monde
    pub position: Vec2,
    /// Royaume propriétaire
    pub kingdom: Option<KingdomId>,

    // === État ===
    /// Points de vie actuels
    pub health: u32,
    /// Points de vie maximum
    pub max_health: u32,
    /// Progrès de construction (0-100)
    pub construction_progress: u8,
    /// Est-ce que la construction est terminée ?
    pub is_completed: bool,

    // === Production ===
    /// Ressources stockées
    pub stored_resources: u32,
    /// Capacité de stockage
    pub storage_capacity: u32,

    // === Occupation ===
    /// IDs des unités occupant ce bâtiment
    pub occupants: Vec<EntityId>,
    /// Capacité maximale d'occupation
    pub max_occupants: u32,
}

impl Default for Building {
    fn default() -> Self {
        Self {
            id: 0,
            building_type: BuildingType::House,
            position: Vec2::default(),
            kingdom: None,
            health: 100,
            max_health: 100,
            construction_progress: 0,
            is_completed: false,
            stored_resources: 0,
            storage_capacity: 100,
            occupants: Vec::new(),
            max_occupants: 4,
        }
    }
}

impl Building {
    /// Crée un nouveau bâtiment à une position donnée
    #[must_use]
    pub fn new(building_type: BuildingType, position: Vec2) -> Self {
        Self {
            building_type,
            position,
            max_health: building_type.max_health(),
            health: building_type.max_health(),
            max_occupants: building_type.capacity(),
            ..Default::default()
        }
    }

    /// Vérifie si le bâtiment est détruit
    #[must_use]
    pub fn is_destroyed(&self) -> bool {
        self.health == 0
    }

    /// Vérifie si la construction est en cours
    #[must_use]
    pub fn is_under_construction(&self) -> bool {
        !self.is_completed && self.construction_progress < 100
    }

    /// Avance la construction
    pub fn advance_construction(&mut self, progress: u8) {
        self.construction_progress = self.construction_progress.saturating_add(progress).min(100);
        if self.construction_progress >= 100 {
            self.is_completed = true;
        }
    }

    /// Applique des dégâts au bâtiment
    pub fn take_damage(&mut self, damage: u32) {
        self.health = self.health.saturating_sub(damage);
    }

    /// Répare le bâtiment
    pub fn repair(&mut self, amount: u32) {
        self.health = (self.health + amount).min(self.max_health);
    }

    /// Ajoute un occupant si possible
    pub fn add_occupant(&mut self, unit_id: EntityId) -> bool {
        if self.occupants.len() < self.max_occupants as usize {
            self.occupants.push(unit_id);
            true
        } else {
            false
        }
    }

    /// Retire un occupant
    pub fn remove_occupant(&mut self, unit_id: EntityId) {
        self.occupants.retain(|&id| id != unit_id);
    }

    /// Vérifie si le bâtiment peut produire
    #[must_use]
    pub fn can_produce(&self) -> bool {
        self.is_completed && !self.is_destroyed() && self.building_type.daily_production() > 0
    }

    /// Produit des ressources quotidiennes
    pub fn produce(&mut self) -> u32 {
        if self.can_produce() {
            let production = self.building_type.daily_production();
            let space = self.storage_capacity.saturating_sub(self.stored_resources);
            let produced = production.min(space);
            self.stored_resources += produced;
            produced
        } else {
            0
        }
    }

    /// Collecte les ressources stockées
    pub fn collect_resources(&mut self) -> u32 {
        let collected = self.stored_resources;
        self.stored_resources = 0;
        collected
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_building_creation() {
        let building = Building::new(BuildingType::House, Vec2::new(10.0, 20.0));
        assert_eq!(building.building_type, BuildingType::House);
        assert!(!building.is_completed);
    }

    #[test]
    fn test_building_construction() {
        let mut building = Building::new(BuildingType::Farm, Vec2::default());
        assert!(building.is_under_construction());
        building.advance_construction(50);
        assert!(building.is_under_construction());
        building.advance_construction(50);
        assert!(!building.is_under_construction());
        assert!(building.is_completed);
    }

    #[test]
    fn test_building_production() {
        let mut building = Building::new(BuildingType::Farm, Vec2::default());
        building.is_completed = true;
        building.construction_progress = 100;
        let produced = building.produce();
        assert_eq!(produced, 5); // Ferme produit 5/jour
    }
}
