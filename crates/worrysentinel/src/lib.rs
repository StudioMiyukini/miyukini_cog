//! @id     worrysentinel_lib
//! @do     declarer_module_degradation
//! @role   orchestration
//! @layer  core
//! @human  worrysentinel — module « lib ».

//! # WorrySentinel
//!
//! Détecteur de menaces et dégradation du Miyukini Core System.
//!
//! WorrySentinel détecte les menaces, évalue les niveaux de sécurité, et gère la dégradation progressive.

pub mod degradation;
pub mod security_level;
pub mod threat_detector;

pub use degradation::{
    DefaultDegradationManager, Degradation, DegradationManager, DegradationState,
};
pub use security_level::{DefaultSecurityLevelManager, SecurityLevel, SecurityLevelManager};
pub use threat_detector::{DefaultThreatDetector, ThreatDetector, ThreatLevel};
