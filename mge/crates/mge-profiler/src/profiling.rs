//! Profiling — métriques par tick, phase, système.
//!
//! @id mge.kernel.profiler.module
//! @role simulation
//! @layer kernel
//! @human Détection overflow budget
//! @do measure_tick_phase_system_duration

use std::time::{Duration, Instant};

/// Identifiant de phase opaque. Les plugins définissent leurs constantes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhaseId(pub u32);

/// Métriques d'un tick complet.
#[derive(Debug, Clone)]
pub struct TickMetrics {
    /// Durée totale du tick.
    pub total: Duration,
    /// Métriques par phase, triées par PhaseId croissant.
    pub phases: Vec<PhaseMetrics>,
    /// true si total > tick_budget (quand configuré).
    pub budget_exceeded: bool,
}

/// Métriques d'une phase dans un tick.
#[derive(Debug, Clone)]
pub struct PhaseMetrics {
    /// Identifiant de phase.
    pub phase: PhaseId,
    /// Durée totale de la phase.
    pub duration: Duration,
    /// Métriques par système dans cette phase (ordre d'enregistrement).
    pub systems: Vec<SystemMetrics>,
}

/// Métriques d'un système dans un tick.
#[derive(Debug, Clone)]
pub struct SystemMetrics {
    /// Nom du système (si fourni lors de l'enregistrement).
    pub name: Option<String>,
    /// Durée d'exécution.
    pub duration: Duration,
}

/// Enregistre les mesures pendant un tick. Utilisé par le Scheduler.
pub struct TickProfiler {
    tick_start: Instant,
    phases: Vec<PhaseMetrics>,
    current_phase: Option<(PhaseId, Instant, Vec<SystemMetrics>)>,
}

impl TickProfiler {
    pub fn begin() -> Self {
        Self {
            tick_start: Instant::now(),
            phases: Vec::new(),
            current_phase: None,
        }
    }

    pub fn begin_phase(&mut self, phase: PhaseId) {
        self.current_phase = Some((phase, Instant::now(), Vec::new()));
    }

    pub fn begin_system(&self) -> Instant {
        Instant::now()
    }

    pub fn end_system(&mut self, start: Instant, name: Option<String>) {
        if let Some((_, _, ref mut systems)) = self.current_phase {
            systems.push(SystemMetrics {
                name,
                duration: start.elapsed(),
            });
        }
    }

    pub fn end_phase(&mut self) {
        if let Some((phase, start, systems)) = self.current_phase.take() {
            self.phases.push(PhaseMetrics {
                phase,
                duration: start.elapsed(),
                systems,
            });
        }
    }

    pub fn finish(self, tick_budget_ms: Option<u32>) -> TickMetrics {
        let total = self.tick_start.elapsed();
        let budget_exceeded = tick_budget_ms
            .map(|ms| total > Duration::from_millis(u64::from(ms)))
            .unwrap_or(false);
        TickMetrics {
            total,
            phases: self.phases,
            budget_exceeded,
        }
    }
}
