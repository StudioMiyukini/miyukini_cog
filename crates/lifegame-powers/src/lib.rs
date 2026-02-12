//! # LifeGame Powers — Opérateur Pouvoirs
//!
//! Gestion des pouvoirs divins du joueur.
//!
//! @id: lifegame.operator.powers
//! @role: gameplay
//! @layer: domain
//! @do: manage_divine_powers_and_god_actions
//! @human: Opérateur gérant tous les pouvoirs divins (création, destruction, créatures, magie)

use serde::{Deserialize, Serialize};
use lifegame_entities::{EntityManager, Unit, Creature, Vec2, Race, CreatureType};
use lifegame_world::{TerrainGrid, TerrainType};

pub mod creation;
pub mod destruction;

pub use creation::*;
pub use destruction::*;

/// Catégorie de pouvoir
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PowerCategory {
    /// Pouvoirs de création (terrain, ressources, vie)
    Creation,
    /// Pouvoirs de destruction (catastrophes, armes)
    Destruction,
    /// Pouvoirs d'invocation de créatures
    Creatures,
    /// Pouvoirs magiques (bénédictions, malédictions)
    Magic,
    /// Contrôle du temps
    TimeControl,
    /// Pouvoirs spéciaux (aimant, inspection)
    Special,
    /// Effets esthétiques
    Effects,
}

/// Définition d'un pouvoir divin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerDefinition {
    /// Identifiant unique
    pub id: String,
    /// Nom affiché
    pub name: String,
    /// Description
    pub description: String,
    /// Catégorie
    pub category: PowerCategory,
    /// Cooldown en secondes
    pub cooldown: f32,
    /// Rayon d'effet
    pub radius: u32,
}

/// Type de pouvoir utilisable
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PowerType {
    // === Création ===
    /// Place du terrain (terre)
    PlaceLand,
    /// Place de l'eau
    PlaceWater,
    /// Place une forêt
    PlaceForest,
    /// Place une montagne
    PlaceMountain,
    /// Place des Humains
    SpawnHumans,
    /// Place des Orcs
    SpawnOrcs,
    /// Place des Elfes
    SpawnElves,
    /// Place des Nains
    SpawnDwarves,
    /// Donne de la nourriture
    GiveFood,

    // === Destruction ===
    /// Éclair
    Lightning,
    /// Bombe
    Bomb,
    /// Tremblement de terre
    Earthquake,
    /// Incendie
    Fire,
    /// Inondation
    Flood,

    // === Créatures ===
    /// Invoque des zombies
    SpawnZombies,
    /// Invoque un dragon
    SpawnDragon,

    // === Temps ===
    /// Pause
    Pause,
    /// Vitesse normale
    SpeedNormal,
    /// Vitesse x2
    Speed2x,
    /// Vitesse x5
    Speed5x,
}

impl PowerType {
    /// Retourne la catégorie du pouvoir
    #[must_use]
    pub const fn category(&self) -> PowerCategory {
        match self {
            Self::PlaceLand | Self::PlaceWater | Self::PlaceForest 
            | Self::PlaceMountain | Self::SpawnHumans | Self::SpawnOrcs 
            | Self::SpawnElves | Self::SpawnDwarves | Self::GiveFood => PowerCategory::Creation,
            
            Self::Lightning | Self::Bomb | Self::Earthquake 
            | Self::Fire | Self::Flood => PowerCategory::Destruction,
            
            Self::SpawnZombies | Self::SpawnDragon => PowerCategory::Creatures,
            
            Self::Pause | Self::SpeedNormal | Self::Speed2x | Self::Speed5x => PowerCategory::TimeControl,
        }
    }

    /// Retourne le cooldown en secondes
    #[must_use]
    pub const fn cooldown(&self) -> f32 {
        match self {
            // Création - pas de cooldown
            Self::PlaceLand | Self::PlaceWater | Self::PlaceForest 
            | Self::PlaceMountain | Self::SpawnHumans | Self::SpawnOrcs 
            | Self::SpawnElves | Self::SpawnDwarves | Self::GiveFood => 0.0,
            
            // Destruction - cooldowns courts
            Self::Lightning => 0.5,
            Self::Bomb => 2.0,
            Self::Earthquake => 5.0,
            Self::Fire => 1.0,
            Self::Flood => 3.0,
            
            // Créatures - cooldowns moyens
            Self::SpawnZombies => 1.0,
            Self::SpawnDragon => 10.0,
            
            // Temps - pas de cooldown
            Self::Pause | Self::SpeedNormal | Self::Speed2x | Self::Speed5x => 0.0,
        }
    }

    /// Retourne le rayon d'effet par défaut
    #[must_use]
    pub const fn default_radius(&self) -> u32 {
        match self {
            Self::PlaceLand | Self::PlaceWater | Self::PlaceForest 
            | Self::PlaceMountain => 5,
            Self::Lightning => 3,
            Self::Bomb => 8,
            Self::Earthquake => 20,
            Self::Fire => 5,
            Self::Flood => 15,
            _ => 1,
        }
    }
}

/// Gestionnaire des pouvoirs divins
///
/// @id: lifegame.powers.manager
/// @do: manage_power_usage_and_cooldowns
#[derive(Debug, Clone, Default)]
pub struct PowerManager {
    /// Dernière utilisation de chaque pouvoir (timestamp)
    pub last_used: std::collections::HashMap<PowerType, f64>,
    /// Pouvoir actuellement sélectionné
    pub selected_power: Option<PowerType>,
    /// Taille du pinceau (rayon)
    pub brush_size: u32,
}

impl PowerManager {
    /// Crée un nouveau gestionnaire de pouvoirs
    #[must_use]
    pub fn new() -> Self {
        Self {
            brush_size: 5,
            ..Default::default()
        }
    }

    /// Sélectionne un pouvoir
    pub fn select(&mut self, power: PowerType) {
        self.selected_power = Some(power);
    }

    /// Désélectionne le pouvoir actuel
    pub fn deselect(&mut self) {
        self.selected_power = None;
    }

    /// Vérifie si un pouvoir est prêt (cooldown terminé)
    #[must_use]
    pub fn is_ready(&self, power: PowerType, current_time: f64) -> bool {
        if let Some(&last_used) = self.last_used.get(&power) {
            let elapsed = current_time - last_used;
            elapsed >= f64::from(power.cooldown())
        } else {
            true
        }
    }

    /// Marque un pouvoir comme utilisé
    pub fn mark_used(&mut self, power: PowerType, current_time: f64) {
        self.last_used.insert(power, current_time);
    }

    /// Change la taille du pinceau
    pub fn set_brush_size(&mut self, size: u32) {
        self.brush_size = size.clamp(1, 20);
    }

    /// Utilise le pouvoir sélectionné à une position
    pub fn use_power(
        &mut self,
        position: Vec2,
        current_time: f64,
        entities: &mut EntityManager,
        terrain: &mut TerrainGrid,
    ) -> Option<PowerResult> {
        let power = self.selected_power?;
        
        if !self.is_ready(power, current_time) {
            return None;
        }

        let result = execute_power(power, position, self.brush_size, entities, terrain);
        self.mark_used(power, current_time);
        
        Some(result)
    }
}

/// Résultat de l'utilisation d'un pouvoir
#[derive(Debug, Clone)]
pub struct PowerResult {
    /// Pouvoir utilisé
    pub power: PowerType,
    /// Position d'utilisation
    pub position: Vec2,
    /// Entités affectées
    pub affected_entities: Vec<u64>,
    /// Tuiles modifiées
    pub tiles_modified: u32,
    /// Message de résultat
    pub message: String,
}

/// Exécute un pouvoir divin
pub fn execute_power(
    power: PowerType,
    position: Vec2,
    brush_size: u32,
    entities: &mut EntityManager,
    terrain: &mut TerrainGrid,
) -> PowerResult {
    let x = position.x as u32;
    let y = position.y as u32;

    match power {
        // === Création de terrain ===
        PowerType::PlaceLand => {
            terrain.paint_disk(x, y, brush_size, TerrainType::Grass);
            PowerResult {
                power,
                position,
                affected_entities: vec![],
                tiles_modified: brush_size * brush_size,
                message: "Terre créée".to_string(),
            }
        }
        PowerType::PlaceWater => {
            terrain.paint_disk(x, y, brush_size, TerrainType::ShallowWater);
            PowerResult {
                power,
                position,
                affected_entities: vec![],
                tiles_modified: brush_size * brush_size,
                message: "Eau créée".to_string(),
            }
        }
        PowerType::PlaceForest => {
            terrain.paint_disk(x, y, brush_size, TerrainType::Forest);
            // Ajoute des arbres
            for dy in -(brush_size as i32)..=(brush_size as i32) {
                for dx in -(brush_size as i32)..=(brush_size as i32) {
                    if dx * dx + dy * dy <= (brush_size * brush_size) as i32 {
                        let tx = (x as i32 + dx) as u32;
                        let ty = (y as i32 + dy) as u32;
                        if rand::random::<f32>() < 0.6 {
                            terrain.add_tree(tx, ty);
                        }
                    }
                }
            }
            PowerResult {
                power,
                position,
                affected_entities: vec![],
                tiles_modified: brush_size * brush_size,
                message: "Forêt créée".to_string(),
            }
        }
        PowerType::PlaceMountain => {
            terrain.paint_disk(x, y, brush_size, TerrainType::Mountain);
            PowerResult {
                power,
                position,
                affected_entities: vec![],
                tiles_modified: brush_size * brush_size,
                message: "Montagne créée".to_string(),
            }
        }

        // === Spawn d'unités ===
        PowerType::SpawnHumans => spawn_units(power, position, entities, Race::Human, 3),
        PowerType::SpawnOrcs => spawn_units(power, position, entities, Race::Orc, 3),
        PowerType::SpawnElves => spawn_units(power, position, entities, Race::Elf, 2),
        PowerType::SpawnDwarves => spawn_units(power, position, entities, Race::Dwarf, 2),

        // === Destruction ===
        PowerType::Lightning => apply_damage_radius(power, position, 3, 50, entities, terrain),
        PowerType::Bomb => apply_damage_radius(power, position, 8, 150, entities, terrain),
        PowerType::Earthquake => {
            // Détruit les bâtiments dans le rayon
            let radius = 20.0;
            let affected: Vec<u64> = entities.buildings_in_radius(position, radius)
                .iter()
                .map(|b| b.id)
                .collect();
            
            for id in &affected {
                if let Some(building) = entities.buildings.get_mut(id) {
                    building.take_damage(100);
                }
            }

            PowerResult {
                power,
                position,
                affected_entities: affected,
                tiles_modified: 0,
                message: "Tremblement de terre !".to_string(),
            }
        }
        PowerType::Fire => {
            use lifegame_world::terrain::TileEffect;
            terrain.apply_effect_radius(x, y, 5, TileEffect::Fire);
            PowerResult {
                power,
                position,
                affected_entities: vec![],
                tiles_modified: 25,
                message: "Feu déclenché !".to_string(),
            }
        }
        PowerType::Flood => {
            use lifegame_world::terrain::TileEffect;
            terrain.apply_effect_radius(x, y, 15, TileEffect::Flooded);
            PowerResult {
                power,
                position,
                affected_entities: vec![],
                tiles_modified: 225,
                message: "Zone inondée !".to_string(),
            }
        }

        // === Créatures ===
        PowerType::SpawnZombies => {
            let mut spawned = vec![];
            for i in 0..5 {
                let offset = Vec2::new(
                    (i as f32 - 2.0) * 2.0,
                    (rand::random::<f32>() - 0.5) * 4.0,
                );
                let spawn_pos = position + offset;
                let creature = Creature::new(CreatureType::Zombie, spawn_pos);
                let id = entities.spawn_creature(creature);
                spawned.push(id);
            }
            PowerResult {
                power,
                position,
                affected_entities: spawned,
                tiles_modified: 0,
                message: "5 zombies invoqués !".to_string(),
            }
        }
        PowerType::SpawnDragon => {
            let creature = Creature::new(CreatureType::Dragon, position);
            let id = entities.spawn_creature(creature);
            PowerResult {
                power,
                position,
                affected_entities: vec![id],
                tiles_modified: 0,
                message: "Dragon invoqué !".to_string(),
            }
        }

        // === Autres ===
        PowerType::GiveFood => {
            // TODO: Implémenter la distribution de nourriture
            PowerResult {
                power,
                position,
                affected_entities: vec![],
                tiles_modified: 0,
                message: "+100 nourriture".to_string(),
            }
        }

        // === Temps (géré ailleurs) ===
        PowerType::Pause | PowerType::SpeedNormal | PowerType::Speed2x | PowerType::Speed5x => {
            PowerResult {
                power,
                position,
                affected_entities: vec![],
                tiles_modified: 0,
                message: "Vitesse modifiée".to_string(),
            }
        }
    }
}

/// Fait apparaître des unités d'une race
fn spawn_units(
    power: PowerType,
    position: Vec2,
    entities: &mut EntityManager,
    race: Race,
    count: u32,
) -> PowerResult {
    let mut spawned = vec![];
    
    for i in 0..count {
        let offset = Vec2::new(
            (i as f32 - (count as f32 / 2.0)) * 2.0,
            (rand::random::<f32>() - 0.5) * 4.0,
        );
        let spawn_pos = position + offset;
        
        let name = generate_name(race);
        let mut unit = Unit::new(name, race);
        unit.position = spawn_pos;
        
        let id = entities.spawn_unit(unit);
        spawned.push(id);
    }

    let race_name = match race {
        Race::Human => "Humains",
        Race::Orc => "Orcs",
        Race::Elf => "Elfes",
        Race::Dwarf => "Nains",
    };

    PowerResult {
        power,
        position,
        affected_entities: spawned,
        tiles_modified: 0,
        message: format!("{count} {race_name} créés"),
    }
}

/// Applique des dégâts dans un rayon
fn apply_damage_radius(
    power: PowerType,
    position: Vec2,
    radius: u32,
    damage: u32,
    entities: &mut EntityManager,
    _terrain: &mut TerrainGrid,
) -> PowerResult {
    let affected_units: Vec<u64> = entities.units_in_radius(position, radius as f32)
        .iter()
        .map(|u| u.id)
        .collect();

    let affected_creatures: Vec<u64> = entities.creatures
        .values()
        .filter(|c| c.position.distance(&position) <= radius as f32)
        .map(|c| c.id)
        .collect();

    // Applique les dégâts aux unités
    for id in &affected_units {
        if let Some(unit) = entities.units.get_mut(id) {
            unit.take_damage(damage);
        }
    }

    // Applique les dégâts aux créatures
    for id in &affected_creatures {
        if let Some(creature) = entities.creatures.get_mut(id) {
            creature.take_damage(damage);
        }
    }

    let total_affected: Vec<u64> = affected_units.iter().chain(affected_creatures.iter()).copied().collect();

    PowerResult {
        power,
        position,
        affected_entities: total_affected,
        tiles_modified: 0,
        message: format!("{damage} dégâts infligés !"),
    }
}

/// Génère un nom pour une unité
fn generate_name(race: Race) -> String {
    let first_names = match race {
        Race::Human => ["Alaric", "Baldwin", "Conrad", "Elara", "Beatrix", "Isolde"],
        Race::Orc => ["Thrall", "Grom", "Durotan", "Garona", "Draka", "Aggra"],
        Race::Elf => ["Elrohir", "Fëanor", "Celeborn", "Galadriel", "Arwen", "Elenia"],
        Race::Dwarf => ["Thorin", "Gimli", "Dwalin", "Disa", "Brunhilde", "Freya"],
    };
    
    let idx = rand::random::<usize>() % first_names.len();
    first_names[idx].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_manager() {
        let mut manager = PowerManager::new();
        manager.select(PowerType::Lightning);
        assert_eq!(manager.selected_power, Some(PowerType::Lightning));
        
        assert!(manager.is_ready(PowerType::Lightning, 0.0));
        manager.mark_used(PowerType::Lightning, 0.0);
        assert!(!manager.is_ready(PowerType::Lightning, 0.1));
        assert!(manager.is_ready(PowerType::Lightning, 1.0));
    }

    #[test]
    fn test_power_category() {
        assert_eq!(PowerType::PlaceLand.category(), PowerCategory::Creation);
        assert_eq!(PowerType::Lightning.category(), PowerCategory::Destruction);
        assert_eq!(PowerType::SpawnDragon.category(), PowerCategory::Creatures);
    }
}
