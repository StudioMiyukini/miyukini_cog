//! Intégration MiyuClock (horloge, attestation date IRL).
//!
//! JayFestival utilise MiyuClock pour l'instant présent et l'attestation de date/heure.
//! P1 : MiyuClock atteste l'horaire et la date IRL ; JayKoa organise les données et l'interface.

pub mod adapter;

pub use adapter::{miyuclock_attest_date, miyuclock_now, MiyuclockAdapterError};
