//! @id     games_allumina_app
//! @do     definir_game_state
//! @role   orchestration
//! @layer  core
//! @human  games allumina — module « app ».

//! The application shell: plugins register systems into phases, and the
//! scheduler runs them in ascending `PhaseId` order each tick.

use std::collections::BTreeMap;

use crate::ecs::World;

/// High-level state of the desktop client. The renderer routes input/render
/// to a different surface for each state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    /// Title menu (new game / load / multiplayer / quit).
    TitleScreen,
    /// Class selection + character name.
    ClassSelect,
    /// Brief loading screen while a save is being applied.
    Loading,
    /// Multiplayer lobby — list/create/join games before [`Self::Playing`].
    /// Only reachable when the `netcode` cargo feature is enabled.
    Lobby,
    /// In-game.
    Playing,
    /// Paused (Esc menu) — sim frozen.
    Paused,
    /// Player died — respawn / load prompt.
    Dead,
}

/// A system is a plain function over the world.
pub type System = fn(&mut World);

/// Execution phases, run in ascending order every tick.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PhaseId {
    Input,
    Ai,
    Movement,
    Skills,
    Combat,
    Death,
    Loot,
    Progression,
    Camera,
    Render,
    Network,
}

/// A bundle of systems registered against the app.
pub trait Plugin {
    fn build(&self, app: &mut App);
}

/// Owns the world and the schedule.
pub struct App {
    pub world: World,
    startup: Vec<System>,
    schedule: BTreeMap<PhaseId, Vec<System>>,
}

impl App {
    pub fn new(world: World) -> Self {
        Self { world, startup: Vec::new(), schedule: BTreeMap::new() }
    }

    pub fn add_plugin<P: Plugin>(&mut self, plugin: P) -> &mut Self {
        plugin.build(self);
        self
    }

    pub fn add_startup_system(&mut self, system: System) -> &mut Self {
        self.startup.push(system);
        self
    }

    pub fn add_system(&mut self, phase: PhaseId, system: System) -> &mut Self {
        self.schedule.entry(phase).or_default().push(system);
        self
    }

    /// Run all registered startup systems once, in registration order.
    pub fn run_startup(&mut self) {
        for system in &self.startup {
            system(&mut self.world);
        }
    }

    /// Advance the simulation by `dt` seconds: run every phase in order.
    pub fn tick(&mut self, dt: f32) {
        self.world.dt = dt;
        self.world.tick += 1;
        for systems in self.schedule.values() {
            for system in systems {
                system(&mut self.world);
            }
        }
    }
}
