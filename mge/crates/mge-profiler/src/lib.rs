//! Profiling — métriques par tick, phase, système.
//!
//! @id mge.kernel.profiler
//! @role simulation
//! @layer kernel
//! @human Métriques de profiling par tick/phase/système
//! @do measure_tick_phase_system_duration

mod profiling;

pub use profiling::{PhaseId, PhaseMetrics, SystemMetrics, TickMetrics, TickProfiler};
