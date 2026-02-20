//! Event + EventQueue — file d'événements typés, buffer double.
//!
//! @id mge.kernel.event
//! @role simulation
//! @layer kernel
//! @human Marqueur événements et file typée
//! @do define_event_marker_and_queue

mod event;
mod event_queue;

pub use event::Event;
pub use event_queue::{EventIter, EventQueue};
