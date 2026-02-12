//! # Unités civilisées
//!
//! Représente les entités intelligentes appartenant à une race civilisée.
//!
//! @id: lifegame.entities.unit
//! @role: simulation
//! @layer: domain
//! @do: define_civilized_unit_structures_and_behaviors
//! @human: Structures et comportements des unités civilisées (Humains, Orcs, Elfes, Nains)

use serde::{Deserialize, Serialize};
use crate::{EntityId, KingdomId, Vec2};

/// Race d'une unité civilisée
///
/// @id: lifegame.entities.race
/// @do: enumerate_playable_races
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum Race {
    /// Humains — équilibrés, adaptables, expansion rapide
    #[default]
    Human,
    /// Orcs — guerriers, agressifs, technologie lente
    Orc,
    /// Elfes — longue vie, magiques, fragiles
    Elf,
    /// Nains — artisans, défensifs, lents
    Dwarf,
}

impl Race {
    /// Durée de vie maximale selon la race
    #[must_use]
    pub const fn max_lifespan(&self) -> u32 {
        match self {
            Self::Human => 80,
            Self::Orc => 60,
            Self::Elf => 400,
            Self::Dwarf => 180,
        }
    }

    /// Bonus d'attaque racial
    #[must_use]
    pub const fn attack_bonus(&self) -> f32 {
        match self {
            Self::Human => 1.0,
            Self::Orc => 1.5,    // Guerriers nés
            Self::Elf => 0.8,    // Fragiles
            Self::Dwarf => 1.2,  // Solides
        }
    }

    /// Vitesse de déplacement
    #[must_use]
    pub const fn speed_modifier(&self) -> f32 {
        match self {
            Self::Human => 1.0,
            Self::Orc => 1.1,
            Self::Elf => 1.2,    // Agiles
            Self::Dwarf => 0.8,  // Lents
        }
    }
}

/// Rôle d'une unité dans la société
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum UnitRole {
    /// Travailleur — ferme, construit, récolte
    #[default]
    Worker,
    /// Soldat — défend, attaque
    Soldier,
    /// Noble — gouverne, commerce
    Noble,
    /// Roi — dirige le royaume
    King,
    /// Prêtre — religion, guérison
    Priest,
}

/// Trait de personnalité (génétique ou acquis)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Trait {
    /// Fort — +20% attaque
    Strong,
    /// Faible — -20% attaque
    Weak,
    /// Rapide — +20% vitesse
    Fast,
    /// Lent — -20% vitesse
    Slow,
    /// Intelligent — +50% recherche
    Intelligent,
    /// Charismatique — +20% diplomatie
    Charismatic,
    /// Agressif — plus susceptible de se battre
    Aggressive,
    /// Pacifique — évite les conflits
    Peaceful,
    /// Immortel — ne vieillit pas (bénédiction divine)
    Immortal,
}

/// Arbre généalogique simplifié
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FamilyTree {
    /// ID du père (si connu)
    pub father: Option<EntityId>,
    /// ID de la mère (si connue)
    pub mother: Option<EntityId>,
    /// IDs des enfants
    pub children: Vec<EntityId>,
    /// ID du conjoint (si marié)
    pub spouse: Option<EntityId>,
}

/// Équipement d'arme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Weapon {
    pub name: String,
    pub damage: u32,
    pub quality: u8,
}

/// Équipement d'armure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Armor {
    pub name: String,
    pub defense: u32,
    pub quality: u8,
}

/// Unité civilisée complète
///
/// @id: lifegame.entities.unit_struct
/// @do: represent_civilized_entity_with_needs_and_stats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Unit {
    /// Identifiant unique
    pub id: EntityId,
    /// Nom de l'unité
    pub name: String,
    /// Race
    pub race: Race,
    /// Royaume d'appartenance
    pub kingdom: Option<KingdomId>,

    // === Position ===
    /// Position actuelle dans le monde
    pub position: Vec2,
    /// Vélocité de déplacement
    pub velocity: Vec2,
    /// Position cible (si en déplacement)
    pub target_position: Option<Vec2>,

    // === Stats de base ===
    /// Points de vie actuels
    pub health: u32,
    /// Points de vie maximum
    pub max_health: u32,
    /// Puissance d'attaque
    pub attack: u32,
    /// Défense
    pub defense: u32,
    /// Vitesse de déplacement
    pub speed: f32,

    // === Besoins (0-100) ===
    /// Faim — meurt si 0 pendant 7 jours
    pub hunger: u8,
    /// Soif — meurt si 0 pendant 3 jours
    pub thirst: u8,
    /// Énergie — dort si trop bas
    pub energy: u8,
    /// Bonheur (-100 à +100)
    pub happiness: i8,

    // === Âge et cycle de vie ===
    /// Âge en jours de jeu
    pub age: u32,
    /// Âge maximum (selon race et traits)
    pub max_age: u32,

    // === Rôle social ===
    /// Rôle dans la société
    pub role: UnitRole,

    // === Équipement ===
    /// Arme équipée
    pub weapon: Option<Weapon>,
    /// Armure équipée
    pub armor: Option<Armor>,

    // === Traits ===
    /// Traits de personnalité
    pub traits: Vec<Trait>,

    // === Relations ===
    /// Arbre généalogique
    pub family: FamilyTree,
    /// IDs des amis
    pub friends: Vec<EntityId>,
    /// IDs des ennemis
    pub enemies: Vec<EntityId>,

    // === Expérience ===
    /// Niveau
    pub level: u8,
    /// Points d'expérience
    pub experience: u32,
    /// Nombre de kills
    pub kills: u32,

    // === État interne ===
    /// Jours sans nourriture (pour calcul de mort par famine)
    pub days_without_food: u8,
    /// Jours sans eau
    pub days_without_water: u8,
}

impl Default for Unit {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::from("Unnamed"),
            race: Race::Human,
            kingdom: None,
            position: Vec2::default(),
            velocity: Vec2::default(),
            target_position: None,
            health: 100,
            max_health: 100,
            attack: 10,
            defense: 5,
            speed: 1.0,
            hunger: 100,
            thirst: 100,
            energy: 100,
            happiness: 50,
            age: 18 * 365, // 18 ans en jours
            max_age: 80 * 365,
            role: UnitRole::Worker,
            weapon: None,
            armor: None,
            traits: Vec::new(),
            family: FamilyTree::default(),
            friends: Vec::new(),
            enemies: Vec::new(),
            level: 1,
            experience: 0,
            kills: 0,
            days_without_food: 0,
            days_without_water: 0,
        }
    }
}

impl Unit {
    /// Crée une nouvelle unité avec un nom et une race
    #[must_use]
    pub fn new(name: impl Into<String>, race: Race) -> Self {
        let mut unit = Self {
            name: name.into(),
            race,
            ..Default::default()
        };
        // Ajuste les stats selon la race
        unit.max_age = race.max_lifespan() * 365;
        unit.speed *= race.speed_modifier();
        unit.attack = ((unit.attack as f32) * race.attack_bonus()) as u32;
        unit
    }

    /// Vérifie si l'unité est vivante
    #[must_use]
    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    /// Vérifie si l'unité meurt de faim (7 jours sans nourriture)
    #[must_use]
    pub fn is_starving(&self) -> bool {
        self.days_without_food >= 7
    }

    /// Vérifie si l'unité meurt de soif (3 jours sans eau)
    #[must_use]
    pub fn is_dehydrated(&self) -> bool {
        self.days_without_water >= 3
    }

    /// Vérifie si l'unité a un trait spécifique
    #[must_use]
    pub fn has_trait(&self, t: Trait) -> bool {
        self.traits.contains(&t)
    }

    /// Vérifie si l'unité est immortelle
    #[must_use]
    pub fn is_immortal(&self) -> bool {
        self.has_trait(Trait::Immortal)
    }

    /// Applique des dégâts à l'unité
    pub fn take_damage(&mut self, damage: u32) {
        let effective_damage = damage.saturating_sub(self.defense / 2);
        self.health = self.health.saturating_sub(effective_damage);
    }

    /// Soigne l'unité
    pub fn heal(&mut self, amount: u32) {
        self.health = (self.health + amount).min(self.max_health);
    }

    /// Nourrit l'unité
    pub fn feed(&mut self, amount: u8) {
        self.hunger = self.hunger.saturating_add(amount).min(100);
        self.days_without_food = 0;
    }

    /// Abreuve l'unité
    pub fn drink(&mut self, amount: u8) {
        self.thirst = self.thirst.saturating_add(amount).min(100);
        self.days_without_water = 0;
    }

    /// Fait vieillir l'unité d'un jour
    pub fn age_one_day(&mut self) {
        if !self.is_immortal() {
            self.age += 1;
        }

        // Diminue les besoins quotidiens
        self.hunger = self.hunger.saturating_sub(10);
        self.thirst = self.thirst.saturating_sub(15);
        self.energy = self.energy.saturating_sub(5);

        // Compte les jours sans nourriture/eau
        if self.hunger == 0 {
            self.days_without_food += 1;
        } else {
            self.days_without_food = 0;
        }

        if self.thirst == 0 {
            self.days_without_water += 1;
        } else {
            self.days_without_water = 0;
        }
    }

    /// Vérifie si l'unité doit mourir (vieillesse, famine, soif)
    #[must_use]
    pub fn should_die(&self) -> bool {
        if self.health == 0 {
            return true;
        }
        if !self.is_immortal() && self.age >= self.max_age {
            return true; // Vieillesse
        }
        if self.is_starving() {
            return true; // Famine
        }
        if self.is_dehydrated() {
            return true; // Soif
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_creation() {
        let unit = Unit::new("Test", Race::Human);
        assert_eq!(unit.name, "Test");
        assert_eq!(unit.race, Race::Human);
        assert!(unit.is_alive());
    }

    #[test]
    fn test_unit_damage() {
        let mut unit = Unit::default();
        unit.take_damage(30);
        assert!(unit.health < 100);
        assert!(unit.is_alive());
    }

    #[test]
    fn test_unit_aging() {
        let mut unit = Unit::default();
        let initial_hunger = unit.hunger;
        unit.age_one_day();
        assert!(unit.hunger < initial_hunger);
    }
}
