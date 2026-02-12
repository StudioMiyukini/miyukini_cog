//! # LifeGame Simulation — Opérateur Simulation
//!
//! Gestion de la boucle de jeu, ticks, et AI des entités.
//!
//! @id: lifegame.operator.simulation
//! @role: simulation
//! @layer: domain
//! @do: manage_game_loop_ticks_and_entity_ai
//! @human: Opérateur orchestrant la simulation du monde et l'intelligence des entités

use serde::{Deserialize, Serialize};
use lifegame_entities::{EntityManager, Vec2};
use lifegame_world::TerrainGrid;

pub mod world_tick;
pub mod ai;

pub use world_tick::tick_world;

/// Vitesse de simulation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum TimeSpeed {
    /// Pause — simulation arrêtée
    Paused,
    /// Normal — 1 tick par 100ms
    #[default]
    Normal,
    /// Rapide × 2
    Fast2x,
    /// Rapide × 5
    Fast5x,
    /// Rapide × 10
    Fast10x,
}

impl TimeSpeed {
    /// Retourne le multiplicateur de vitesse
    #[must_use]
    pub const fn multiplier(&self) -> u32 {
        match self {
            Self::Paused => 0,
            Self::Normal => 1,
            Self::Fast2x => 2,
            Self::Fast5x => 5,
            Self::Fast10x => 10,
        }
    }
}

/// État de la simulation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SimulationState {
    /// Tick actuel (nombre de minutes de jeu écoulées)
    pub tick: u64,
    /// Vitesse actuelle
    pub speed: TimeSpeed,
    /// Est en pause ?
    pub paused: bool,
    /// Jour actuel (1 jour = 1440 minutes)
    pub day: u32,
}

impl SimulationState {
    /// Crée un nouvel état de simulation
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Avance la simulation d'un tick
    pub fn advance_tick(&mut self) {
        if !self.paused {
            self.tick += 1;
            // Un jour = 1440 ticks (minutes)
            self.day = (self.tick / 1440) as u32;
        }
    }

    /// Met en pause la simulation
    pub fn pause(&mut self) {
        self.paused = true;
        self.speed = TimeSpeed::Paused;
    }

    /// Reprend la simulation
    pub fn resume(&mut self) {
        self.paused = false;
        if self.speed == TimeSpeed::Paused {
            self.speed = TimeSpeed::Normal;
        }
    }

    /// Change la vitesse
    pub fn set_speed(&mut self, speed: TimeSpeed) {
        self.speed = speed;
        self.paused = speed == TimeSpeed::Paused;
    }

    /// Retourne l'heure de la journée (0-23)
    #[must_use]
    pub fn hour_of_day(&self) -> u32 {
        ((self.tick % 1440) / 60) as u32
    }

    /// Est-ce la nuit ? (20h - 6h)
    #[must_use]
    pub fn is_night(&self) -> bool {
        let hour = self.hour_of_day();
        hour >= 20 || hour < 6
    }
}

/// Action qu'une unité peut effectuer
#[derive(Debug, Clone)]
pub enum UnitAction {
    /// Ne rien faire
    Idle,
    /// Se déplacer vers une position
    MoveTo(Vec2),
    /// Chercher de la nourriture
    FindFood,
    /// Chercher de l'eau
    FindWater,
    /// Construire un bâtiment
    Build(u64), // Building ID
    /// Travailler (ferme, mine)
    Work(u64),  // Building ID
    /// Attaquer une cible
    Attack(u64), // Entity ID
    /// Fuir une menace
    Flee(Vec2),
    /// Se reposer
    Rest,
    /// Errer aléatoirement
    Wander,
}

/// Statistiques globales du monde
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorldStats {
    /// Population totale
    pub total_population: u32,
    /// Nombre de bâtiments
    pub total_buildings: u32,
    /// Nombre de créatures
    pub total_creatures: u32,
    /// Nourriture totale disponible
    pub total_food: u32,
    /// Nombre de royaumes
    pub total_kingdoms: u32,
    /// Naissances ce jour
    pub births_today: u32,
    /// Morts ce jour
    pub deaths_today: u32,
}

impl WorldStats {
    /// Calcule les stats depuis l'EntityManager
    #[must_use]
    pub fn from_entities(entities: &EntityManager) -> Self {
        Self {
            total_population: entities.units.len() as u32,
            total_buildings: entities.buildings.len() as u32,
            total_creatures: entities.creatures.len() as u32,
            total_food: 0, // TODO: calculer depuis les bâtiments
            total_kingdoms: 0,
            births_today: 0,
            deaths_today: 0,
        }
    }
}

/// Vérifie si une position est traversable
#[must_use]
pub fn is_position_walkable(terrain: &TerrainGrid, pos: Vec2) -> bool {
    let x = pos.x as u32;
    let y = pos.y as u32;
    
    terrain.get(x, y)
        .map(|tile| tile.is_walkable())
        .unwrap_or(false)
}

/// Trouve une position valide dans un rayon
#[must_use]
pub fn find_valid_position(terrain: &TerrainGrid, center: Vec2, radius: f32) -> Option<Vec2> {
    let steps = 8;
    for r in 1..=(radius as i32) {
        for i in 0..steps {
            let angle = (i as f32 / steps as f32) * std::f32::consts::TAU;
            let test_pos = Vec2::new(
                center.x + (r as f32) * angle.cos(),
                center.y + (r as f32) * angle.sin(),
            );
            if is_position_walkable(terrain, test_pos) {
                return Some(test_pos);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulation_state() {
        let mut state = SimulationState::new();
        assert_eq!(state.tick, 0);
        
        state.advance_tick();
        assert_eq!(state.tick, 1);
        
        state.pause();
        state.advance_tick();
        assert_eq!(state.tick, 1); // Ne devrait pas avancer
    }

    #[test]
    fn test_time_speed() {
        assert_eq!(TimeSpeed::Normal.multiplier(), 1);
        assert_eq!(TimeSpeed::Fast5x.multiplier(), 5);
        assert_eq!(TimeSpeed::Paused.multiplier(), 0);
    }

    #[test]
    fn test_day_night_cycle() {
        let mut state = SimulationState::new();
        
        // Minuit (0h)
        state.tick = 0;
        assert!(state.is_night());
        
        // Midi (12h)
        state.tick = 12 * 60;
        assert!(!state.is_night());
        
        // 22h
        state.tick = 22 * 60;
        assert!(state.is_night());
    }
}
