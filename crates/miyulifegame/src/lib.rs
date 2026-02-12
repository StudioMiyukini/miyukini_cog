//! # MiyuLifeGame — God Simulator Sandbox
//!
//! Simulateur de vie inspiré de WorldBox pour l'écosystème Miyukini COG.
//!
//! @id: service.game.miyulifegame
//! @role: game
//! @layer: service
//! @do: orchestrate_life_game_simulation_and_ui
//! @human: Jeu de simulation de vie (god simulator) où le joueur crée et observe des civilisations
//!
//! ## Architecture
//!
//! ### Opérateurs
//! - `lifegame-simulation` — Boucle de jeu et AI
//! - `lifegame-entities` — Unités, bâtiments, créatures
//! - `lifegame-world` — Terrain et génération
//! - `lifegame-powers` — Pouvoirs divins
//!
//! ## Gameplay
//!
//! 1. Créer un monde (procédural ou manuel)
//! 2. Placer des races civilisées (Humains, Orcs, Elfes, Nains)
//! 3. Observer leur évolution autonome
//! 4. Intervenir avec des pouvoirs divins

use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

pub use lifegame_simulation::{SimulationState, TimeSpeed, WorldStats, UnitAction};
pub use lifegame_entities::{EntityManager, Unit, Building, Creature, Vec2, Race, BuildingType, CreatureType, EntityId, KingdomId};
pub use lifegame_world::{TerrainGrid, TerrainType, WorldSize, WorldMetadata, WorldGenerator, Biome, BiomeType};
pub use lifegame_powers::{PowerManager, PowerType, PowerCategory, PowerResult};

/// État complet du jeu
///
/// @id: miyulifegame.game_state
/// @do: aggregate_all_game_state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    /// Métadonnées du monde
    pub metadata: WorldMetadata,
    /// Grille de terrain
    pub terrain: TerrainGrid,
    /// Gestionnaire d'entités
    pub entities: EntityManager,
    /// État de la simulation
    pub simulation: SimulationState,
    /// Statistiques du monde
    #[serde(skip)]
    pub stats: WorldStats,
}

impl GameState {
    /// Crée un nouveau jeu avec un monde vide
    #[must_use]
    pub fn new(name: impl Into<String>, size: WorldSize) -> Self {
        let seed = rand::random();
        let metadata = WorldMetadata::new(name, size, seed);
        let (width, height) = size.dimensions();
        let terrain = TerrainGrid::new(width, height);
        
        Self {
            metadata,
            terrain,
            entities: EntityManager::new(),
            simulation: SimulationState::new(),
            stats: WorldStats::default(),
        }
    }

    /// Crée un nouveau jeu avec un monde généré procéduralement
    #[must_use]
    pub fn new_generated(name: impl Into<String>, size: WorldSize, seed: u64) -> Self {
        let metadata = WorldMetadata::new(name.into(), size, seed);
        let generator = WorldGenerator::new(seed);
        let terrain = generator.generate(size);
        
        Self {
            metadata,
            terrain,
            entities: EntityManager::new(),
            simulation: SimulationState::new(),
            stats: WorldStats::default(),
        }
    }

    /// Met à jour les statistiques
    pub fn update_stats(&mut self) {
        self.stats = WorldStats::from_entities(&self.entities);
    }

    /// Retourne la population totale
    #[must_use]
    pub fn population(&self) -> usize {
        self.entities.units.len()
    }

    /// Retourne le nombre de bâtiments
    #[must_use]
    pub fn building_count(&self) -> usize {
        self.entities.buildings.len()
    }

    /// Retourne le tick actuel
    #[must_use]
    pub fn current_tick(&self) -> u64 {
        self.simulation.tick
    }

    /// Retourne le jour actuel
    #[must_use]
    pub fn current_day(&self) -> u32 {
        self.simulation.day
    }
}

/// Moteur de jeu principal
///
/// @id: miyulifegame.game_engine
/// @do: run_game_loop_and_orchestrate_systems
pub struct GameEngine {
    /// État du jeu
    pub state: GameState,
    /// Gestionnaire de pouvoirs
    pub powers: PowerManager,
    /// Dernier tick exécuté
    last_tick: Instant,
    /// Temps écoulé depuis le dernier tick
    accumulated_time: Duration,
    /// Intervalle entre ticks (100ms = 10 ticks/sec)
    tick_interval: Duration,
}

impl GameEngine {
    /// Crée un nouveau moteur de jeu
    #[must_use]
    pub fn new(state: GameState) -> Self {
        Self {
            state,
            powers: PowerManager::new(),
            last_tick: Instant::now(),
            accumulated_time: Duration::ZERO,
            tick_interval: Duration::from_millis(100),
        }
    }

    /// Crée un nouveau jeu avec génération procédurale
    #[must_use]
    pub fn new_game(name: impl Into<String>, size: WorldSize) -> Self {
        let seed = rand::random();
        let state = GameState::new_generated(name, size, seed);
        Self::new(state)
    }

    /// Met à jour le jeu (appelée chaque frame)
    pub fn update(&mut self) {
        let now = Instant::now();
        let delta = now.duration_since(self.last_tick);
        self.last_tick = now;

        if self.state.simulation.paused {
            return;
        }

        self.accumulated_time += delta;

        // Applique le multiplicateur de vitesse
        let multiplier = self.state.simulation.speed.multiplier();
        let effective_interval = self.tick_interval / multiplier.max(1);

        // Exécute les ticks nécessaires
        while self.accumulated_time >= effective_interval {
            self.accumulated_time -= effective_interval;
            self.tick();
        }
    }

    /// Exécute un tick de simulation
    fn tick(&mut self) {
        let previous_day = self.state.simulation.day;
        self.state.simulation.advance_tick();
        let is_new_day = self.state.simulation.day != previous_day;

        // Exécute le tick du monde
        let _result = lifegame_simulation::tick_world(
            &mut self.state.entities,
            &self.state.terrain,
            is_new_day,
        );

        // Met à jour les stats périodiquement
        if self.state.simulation.tick % 60 == 0 {
            self.state.update_stats();
        }
    }

    /// Met en pause ou reprend le jeu
    pub fn toggle_pause(&mut self) {
        if self.state.simulation.paused {
            self.state.simulation.resume();
        } else {
            self.state.simulation.pause();
        }
    }

    /// Change la vitesse de simulation
    pub fn set_speed(&mut self, speed: TimeSpeed) {
        self.state.simulation.set_speed(speed);
    }

    /// Sélectionne un pouvoir
    pub fn select_power(&mut self, power: PowerType) {
        self.powers.select(power);
    }

    /// Utilise le pouvoir sélectionné à une position
    pub fn use_power(&mut self, position: Vec2) -> Option<PowerResult> {
        let current_time = self.state.simulation.tick as f64 / 10.0; // Temps en secondes
        self.powers.use_power(
            position,
            current_time,
            &mut self.state.entities,
            &mut self.state.terrain,
        )
    }

    /// Place des unités d'une race à une position
    pub fn spawn_units(&mut self, race: Race, position: Vec2, count: u32) {
        for i in 0..count {
            let offset = Vec2::new(
                (i as f32 - (count as f32 / 2.0)) * 2.0,
                (rand::random::<f32>() - 0.5) * 4.0,
            );
            let spawn_pos = position + offset;
            
            let name = format!("{:?}_{}", race, self.state.entities.total_count());
            let mut unit = Unit::new(name, race);
            unit.position = spawn_pos;
            
            self.state.entities.spawn_unit(unit);
        }
    }

    /// Change le type de terrain à une position
    pub fn paint_terrain(&mut self, position: Vec2, terrain_type: TerrainType, radius: u32) {
        let x = position.x as u32;
        let y = position.y as u32;
        self.state.terrain.paint_disk(x, y, radius, terrain_type);
    }
}

/// Sauvegarde du jeu
///
/// @id: miyulifegame.save
/// @do: serialize_and_save_game_state
pub mod save {
    use super::*;
    use std::path::Path;
    use std::fs;

    /// Extension des fichiers de sauvegarde
    pub const SAVE_EXTENSION: &str = "lifegame";

    /// Sauvegarde l'état du jeu dans un fichier
    pub fn save_game(state: &GameState, path: &Path) -> Result<(), std::io::Error> {
        let json = serde_json::to_string_pretty(state)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        fs::write(path, json)
    }

    /// Charge l'état du jeu depuis un fichier
    pub fn load_game(path: &Path) -> Result<GameState, std::io::Error> {
        let json = fs::read_to_string(path)?;
        serde_json::from_str(&json)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_state_creation() {
        let state = GameState::new("Test World", WorldSize::Small);
        assert_eq!(state.metadata.name, "Test World");
        assert_eq!(state.terrain.width, 256);
        assert_eq!(state.terrain.height, 256);
    }

    #[test]
    fn test_game_state_generated() {
        let state = GameState::new_generated("Procedural World", WorldSize::Small, 12345);
        assert_eq!(state.metadata.seed, 12345);
        // Vérifie qu'il y a de la variété dans le terrain
        assert!(state.terrain.count_terrain_type(TerrainType::Grass) > 0);
    }

    #[test]
    fn test_game_engine() {
        let state = GameState::new("Test", WorldSize::Small);
        let mut engine = GameEngine::new(state);
        
        // Spawn des unités
        engine.spawn_units(Race::Human, Vec2::new(128.0, 128.0), 5);
        assert_eq!(engine.state.population(), 5);
        
        // Toggle pause
        engine.toggle_pause();
        assert!(engine.state.simulation.paused);
        engine.toggle_pause();
        assert!(!engine.state.simulation.paused);
    }

    #[test]
    fn test_power_usage() {
        let state = GameState::new("Test", WorldSize::Small);
        let mut engine = GameEngine::new(state);
        
        engine.select_power(PowerType::PlaceLand);
        let result = engine.use_power(Vec2::new(100.0, 100.0));
        assert!(result.is_some());
    }
}
