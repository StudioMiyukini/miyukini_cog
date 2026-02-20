//! Scheduler — ordre déterministe des systèmes, PhaseId.
//!
//! @id mge.core.scheduler
//! @role simulation
//! @layer core
//! @human Ordonnancement déterministe des systèmes
//! @do execute_systems_in_deterministic_order

use std::collections::BTreeMap;

use mge_ecs::World;
use mge_profiler::{PhaseId, TickProfiler};

use crate::context::Context;

/// Type d'un système : (World, Context) -> ()
pub type SystemFn = Box<dyn FnMut(&mut World, &mut Context) + Send>;

struct SystemEntry {
    func: SystemFn,
    name: Option<String>,
}

/// Scheduler : exécution ordonnée par PhaseId croissant.
pub struct Scheduler {
    systems: BTreeMap<PhaseId, Vec<SystemEntry>>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            systems: BTreeMap::new(),
        }
    }

    pub fn add_system<F>(&mut self, phase: PhaseId, system: F)
    where
        F: FnMut(&mut World, &mut Context) + Send + 'static,
    {
        self.systems
            .entry(phase)
            .or_default()
            .push(SystemEntry {
                func: Box::new(system),
                name: None,
            });
    }

    pub fn add_named_system<F>(&mut self, phase: PhaseId, name: impl Into<String>, system: F)
    where
        F: FnMut(&mut World, &mut Context) + Send + 'static,
    {
        self.systems
            .entry(phase)
            .or_default()
            .push(SystemEntry {
                func: Box::new(system),
                name: Some(name.into()),
            });
    }

    pub(crate) fn run(&mut self, world: &mut World, ctx: &mut Context, profiler: &mut TickProfiler) {
        for (&phase, systems) in &mut self.systems {
            profiler.begin_phase(phase);
            for entry in systems.iter_mut() {
                let sys_start = profiler.begin_system();
                (entry.func)(world, ctx);
                profiler.end_system(sys_start, entry.name.clone());
            }
            profiler.end_phase();
        }
    }

    pub fn system_count(&self) -> usize {
        self.systems.values().map(Vec::len).sum()
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}
