//! Time — delta, tick_count, time_scale, pause.
//!
//! Concept tick-based, pas frame-based.
//!
//! @id mge.kernel.time
//! @role simulation
//! @layer kernel
//! @human Gestion du temps de simulation
//! @do provide_tick_based_time

mod time;

pub use time::Time;
