//! Engine — cycle de vie, tick(), orchestration.
//!
//! @id mge.core.engine
//! @role simulation
//! @layer core
//! @human Point d'entrée du moteur
//! @do orchestrate_lifecycle_and_tick

use mge_ecs::World;
use mge_event::Event;
use mge_event::EventQueue;
use mge_profiler::{PhaseId, TickMetrics, TickProfiler};
use mge_rng::Rng;
use mge_time::Time;

use crate::config::EngineConfig;
use crate::context::Context;
use crate::plugin::Plugin;
use crate::scheduler::Scheduler;

/// Engine : cycle de vie, tick, accès aux sous-systèmes.
pub struct Engine {
    world: World,
    scheduler: Scheduler,
    events: EventQueue,
    time: Time,
    rng: Rng,
    config: EngineConfig,
    plugins: Vec<Box<dyn Plugin>>,
    built: bool,
    last_tick_metrics: Option<TickMetrics>,
}

impl Engine {
    /// Crée un Engine avec la configuration donnée.
    pub fn new(config: EngineConfig) -> Self {
        let seed = config.seed;
        Self {
            world: World::new(),
            scheduler: Scheduler::new(),
            events: EventQueue::new(),
            time: Time::new(),
            rng: Rng::new(seed),
            config,
            plugins: Vec::new(),
            built: false,
            last_tick_metrics: None,
        }
    }

    /// Ajoute un plugin. Doit être appelé avant build().
    pub fn add_plugin<P: Plugin + 'static>(&mut self, plugin: P) {
        assert!(!self.built, "cannot add plugin after build");
        self.plugins.push(Box::new(plugin));
    }

    /// Appelle build() sur tous les plugins enregistrés. Ne peut être appelé qu'une fois.
    pub fn build(&mut self) {
        assert!(!self.built, "build already called");
        let plugins = std::mem::take(&mut self.plugins);
        for plugin in plugins {
            plugin.build(self);
        }
        self.built = true;
    }

    /// Enregistre un type de composant dans le World.
    pub fn register_component<T: mge_ecs::Component>(&mut self) {
        self.world.register_component::<T>();
    }

    /// Enregistre un système anonyme dans une phase.
    pub fn add_system<F>(&mut self, phase: PhaseId, system: F)
    where
        F: FnMut(&mut World, &mut Context) + Send + 'static,
    {
        self.scheduler.add_system(phase, system);
    }

    /// Enregistre un système nommé dans une phase.
    pub fn add_named_system<F>(
        &mut self,
        phase: PhaseId,
        name: impl Into<String>,
        system: F,
    )
    where
        F: FnMut(&mut World, &mut Context) + Send + 'static,
    {
        self.scheduler.add_named_system(phase, name, system);
    }

    /// Avance la simulation d'un tick.
    pub fn tick(&mut self, delta_secs: f32) {
        let mut profiler = TickProfiler::begin();
        self.time.advance(delta_secs, self.config.fixed_timestep_ms);
        self.events.swap();
        let mut ctx = Context::new(&self.time, &mut self.rng, &mut self.events);
        self.scheduler.run(&mut self.world, &mut ctx, &mut profiler);
        self.last_tick_metrics = Some(profiler.finish(self.config.tick_budget_ms));
    }

    /// Métriques du dernier tick (None avant le premier tick).
    pub fn last_tick_metrics(&self) -> Option<&TickMetrics> {
        self.last_tick_metrics.as_ref()
    }

    /// Accès en lecture au World.
    pub fn world(&self) -> &World {
        &self.world
    }

    /// Accès en écriture au World.
    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    /// Accès en lecture à l'EventQueue.
    pub fn events(&self) -> &EventQueue {
        &self.events
    }

    /// Émet un événement (écrit dans le buffer, lu au prochain tick).
    pub fn emit<E: Event>(&mut self, event: E) {
        self.events.emit(event);
    }

    /// Réinitialise le RNG avec une nouvelle seed.
    pub fn set_seed(&mut self, seed: u64) {
        self.rng.seed(seed);
    }

    /// Accès en lecture au Time.
    pub fn time(&self) -> &Time {
        &self.time
    }

    /// Configuration (lecture seule).
    pub fn config(&self) -> &EngineConfig {
        &self.config
    }
}
