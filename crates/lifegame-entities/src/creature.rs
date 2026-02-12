//! # Créatures
//!
//! Représente les animaux, monstres et créatures magiques.
//!
//! @id: lifegame.entities.creature
//! @role: simulation
//! @layer: domain
//! @do: define_creature_structures_and_types
//! @human: Structures et types de créatures (animaux, monstres, boss)

use serde::{Deserialize, Serialize};
use crate::{EntityId, Vec2};

/// Type de créature
///
/// @id: lifegame.entities.creature_type
/// @do: enumerate_creature_types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum CreatureType {
    // === Animaux neutres ===
    /// Cerf — herbivore, fuit les prédateurs
    #[default]
    Deer,
    /// Mouton — domesticable
    Sheep,
    /// Vache — domesticable, nourriture
    Cow,
    /// Cheval — domesticable, transport
    Horse,
    /// Loup — chasseur en meute
    Wolf,
    /// Ours — solitaire, dangereux
    Bear,
    /// Poisson — dans l'eau
    Fish,

    // === Créatures hostiles ===
    /// Zombie — infecte les morts
    Zombie,
    /// Squelette — archer
    Skeleton,
    /// Démon — très puissant
    Demon,
    /// Dragon — vole, crache feu
    Dragon,
    /// Cold One — créature de glace
    ColdOne,

    // === Boss ===
    /// Crabzilla — crabe géant boss
    Crabzilla,

    // === Créatures magiques ===
    /// Licorne — guérit le terrain
    Unicorn,
    /// Treant — gardien de forêt
    Treant,
    /// Fée — accélère croissance
    Fairy,
    /// Golem — gardien de pierre
    Golem,
}

impl CreatureType {
    /// Points de vie par défaut
    #[must_use]
    pub const fn base_health(&self) -> u32 {
        match self {
            Self::Deer => 30,
            Self::Sheep => 20,
            Self::Cow => 40,
            Self::Horse => 50,
            Self::Wolf => 60,
            Self::Bear => 100,
            Self::Fish => 10,
            Self::Zombie => 50,
            Self::Skeleton => 40,
            Self::Demon => 200,
            Self::Dragon => 1000,
            Self::ColdOne => 150,
            Self::Crabzilla => 5000,
            Self::Unicorn => 80,
            Self::Treant => 200,
            Self::Fairy => 20,
            Self::Golem => 300,
        }
    }

    /// Dégâts de base
    #[must_use]
    pub const fn base_damage(&self) -> u32 {
        match self {
            Self::Deer => 5,
            Self::Sheep => 0,
            Self::Cow => 5,
            Self::Horse => 10,
            Self::Wolf => 20,
            Self::Bear => 40,
            Self::Fish => 0,
            Self::Zombie => 15,
            Self::Skeleton => 20,
            Self::Demon => 50,
            Self::Dragon => 100,
            Self::ColdOne => 30,
            Self::Crabzilla => 500,
            Self::Unicorn => 0,
            Self::Treant => 30,
            Self::Fairy => 0,
            Self::Golem => 40,
        }
    }

    /// Vitesse de déplacement
    #[must_use]
    pub const fn speed(&self) -> f32 {
        match self {
            Self::Deer => 2.0,
            Self::Sheep => 0.8,
            Self::Cow => 0.6,
            Self::Horse => 2.5,
            Self::Wolf => 1.8,
            Self::Bear => 1.2,
            Self::Fish => 1.0,
            Self::Zombie => 0.5,
            Self::Skeleton => 1.0,
            Self::Demon => 1.5,
            Self::Dragon => 3.0,
            Self::ColdOne => 1.0,
            Self::Crabzilla => 0.8,
            Self::Unicorn => 2.0,
            Self::Treant => 0.3,
            Self::Fairy => 2.5,
            Self::Golem => 0.5,
        }
    }

    /// Est-ce que cette créature est hostile ?
    #[must_use]
    pub const fn is_hostile(&self) -> bool {
        matches!(
            self,
            Self::Zombie
                | Self::Skeleton
                | Self::Demon
                | Self::Dragon
                | Self::ColdOne
                | Self::Crabzilla
        )
    }

    /// Est-ce que cette créature peut voler ?
    #[must_use]
    pub const fn can_fly(&self) -> bool {
        matches!(self, Self::Dragon | Self::Fairy)
    }

    /// Est-ce que cette créature est un boss ?
    #[must_use]
    pub const fn is_boss(&self) -> bool {
        matches!(self, Self::Crabzilla)
    }

    /// Est-ce que cette créature peut être contrôlée par le joueur ?
    #[must_use]
    pub const fn is_controllable(&self) -> bool {
        matches!(self, Self::Dragon | Self::Crabzilla)
    }

    /// Est-ce que cette créature peut infecter d'autres entités ?
    #[must_use]
    pub const fn can_infect(&self) -> bool {
        matches!(self, Self::Zombie)
    }

    /// Est-ce que cette créature peut nager ?
    #[must_use]
    pub const fn can_swim(&self) -> bool {
        matches!(self, Self::Fish | Self::Crabzilla)
    }
}

/// Comportement d'une créature
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum CreatureBehavior {
    /// Erre aléatoirement
    #[default]
    Wandering,
    /// Fuit un danger
    Fleeing,
    /// Chasse une proie
    Hunting,
    /// Attaque une cible
    Attacking,
    /// Se repose
    Resting,
    /// Contrôlée par le joueur
    PlayerControlled,
}

/// Créature dans le monde
///
/// @id: lifegame.entities.creature_struct
/// @do: represent_creature_entity_with_behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Creature {
    /// Identifiant unique
    pub id: EntityId,
    /// Type de créature
    pub creature_type: CreatureType,

    // === Position ===
    /// Position actuelle
    pub position: Vec2,
    /// Vélocité
    pub velocity: Vec2,
    /// Position cible
    pub target_position: Option<Vec2>,

    // === Stats ===
    /// Points de vie
    pub health: u32,
    /// Points de vie max
    pub max_health: u32,
    /// Dégâts
    pub damage: u32,
    /// Vitesse
    pub speed: f32,

    // === Comportement ===
    /// Comportement actuel
    pub behavior: CreatureBehavior,
    /// Cible actuelle (entité à chasser/fuir)
    pub target: Option<EntityId>,

    // === État ===
    /// Est-ce que la créature est hostile actuellement ?
    pub is_aggro: bool,
    /// Âge en jours
    pub age: u32,
}

impl Default for Creature {
    fn default() -> Self {
        Self {
            id: 0,
            creature_type: CreatureType::Deer,
            position: Vec2::default(),
            velocity: Vec2::default(),
            target_position: None,
            health: 30,
            max_health: 30,
            damage: 5,
            speed: 2.0,
            behavior: CreatureBehavior::Wandering,
            target: None,
            is_aggro: false,
            age: 0,
        }
    }
}

impl Creature {
    /// Crée une nouvelle créature d'un type donné
    #[must_use]
    pub fn new(creature_type: CreatureType, position: Vec2) -> Self {
        Self {
            creature_type,
            position,
            health: creature_type.base_health(),
            max_health: creature_type.base_health(),
            damage: creature_type.base_damage(),
            speed: creature_type.speed(),
            is_aggro: creature_type.is_hostile(),
            ..Default::default()
        }
    }

    /// Vérifie si la créature est vivante
    #[must_use]
    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    /// Applique des dégâts
    pub fn take_damage(&mut self, damage: u32) {
        self.health = self.health.saturating_sub(damage);
    }

    /// Soigne la créature
    pub fn heal(&mut self, amount: u32) {
        self.health = (self.health + amount).min(self.max_health);
    }

    /// Définit une cible à attaquer/fuir
    pub fn set_target(&mut self, target_id: EntityId, behavior: CreatureBehavior) {
        self.target = Some(target_id);
        self.behavior = behavior;
    }

    /// Efface la cible
    pub fn clear_target(&mut self) {
        self.target = None;
        self.behavior = CreatureBehavior::Wandering;
    }

    /// Passe en mode contrôle joueur
    pub fn enable_player_control(&mut self) {
        if self.creature_type.is_controllable() {
            self.behavior = CreatureBehavior::PlayerControlled;
        }
    }

    /// Quitte le mode contrôle joueur
    pub fn disable_player_control(&mut self) {
        self.behavior = CreatureBehavior::Wandering;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creature_creation() {
        let creature = Creature::new(CreatureType::Dragon, Vec2::new(100.0, 100.0));
        assert_eq!(creature.creature_type, CreatureType::Dragon);
        assert_eq!(creature.health, 1000);
        assert!(creature.is_aggro);
    }

    #[test]
    fn test_creature_damage() {
        let mut creature = Creature::new(CreatureType::Wolf, Vec2::default());
        let initial_health = creature.health;
        creature.take_damage(20);
        assert_eq!(creature.health, initial_health - 20);
    }

    #[test]
    fn test_creature_types() {
        assert!(CreatureType::Dragon.is_hostile());
        assert!(CreatureType::Dragon.can_fly());
        assert!(CreatureType::Dragon.is_controllable());
        assert!(!CreatureType::Deer.is_hostile());
        assert!(CreatureType::Crabzilla.is_boss());
    }
}
