//! Rng — génération déterministe, seed global, dérivation par bits.
//!
//! @id mge.kernel.rng
//! @role simulation
//! @layer kernel
//! @human RNG déterministe seedable
//! @do provide_deterministic_rng

mod rng;

pub use rng::Rng;
