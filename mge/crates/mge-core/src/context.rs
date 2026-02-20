//! Context — passé aux systèmes : time, rng, events, emit.
//!
//! @id mge.core.context
//! @role simulation
//! @layer core
//! @human Contexte d'exécution des systèmes
//! @do provide_system_execution_context

use mge_event::Event;
use mge_event::EventQueue;
use mge_rng::Rng;
use mge_time::Time;

/// Contexte passé aux systèmes.
pub struct Context<'a> {
    pub time: &'a Time,
    pub rng: &'a mut Rng,
    pub events: &'a mut EventQueue,
}

impl<'a> Context<'a> {
    pub fn new(time: &'a Time, rng: &'a mut Rng, events: &'a mut EventQueue) -> Self {
        Self { time, rng, events }
    }

    pub fn emit<E: Event>(&mut self, event: E) {
        self.events.emit(event);
    }

    pub fn delta_secs(&self) -> f32 {
        self.time.delta_secs
    }

    pub fn tick_count(&self) -> u64 {
        self.time.tick_count
    }
}
