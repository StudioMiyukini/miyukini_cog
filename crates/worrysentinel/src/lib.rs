//! # WorrySentinel
//!
//! Détecteur de menaces et dégradation du Miyukini Core System.
//!
//! WorrySentinel détecte les menaces, évalue les niveaux de sécurité, et gère la dégradation progressive.

pub mod threat_detector;
pub mod security_level;
pub mod degradation;

pub use degradation::{Degradation, DegradationManager};
pub use security_level::{SecurityLevel, SecurityLevelManager};
pub use threat_detector::{ThreatDetector, ThreatLevel};
