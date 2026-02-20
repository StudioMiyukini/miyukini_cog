//! Trait Event — marqueur pour les événements.
//!
//! @id mge.kernel.event.trait
//! @role simulation
//! @layer kernel
//! @human Marqueur des événements typés
//! @do define_event_marker_trait

/// Marqueur requis pour tout événement. Send + Sync pour le buffer double.
pub trait Event: Send + Sync + 'static {}
