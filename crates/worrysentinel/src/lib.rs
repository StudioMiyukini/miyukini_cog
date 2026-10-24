//! @id     worrysentinel_lib
//! @do     declarer_module_degradation
//! @role   orchestration
//! @layer  core
//! @human  worrysentinel — module « lib ».

//! # WorrySentinel
//!
//! Détecteur de menaces et dégradation du Miyukini Core System.
//!
//! WorrySentinel a pour rôle de détecter les menaces, d'évaluer les niveaux de
//! sécurité et de gérer la dégradation progressive.
//!
//! ## État réel (K-17)
//!
//! Ce crate est un **modèle** : types (niveaux de sécurité, menaces, états de
//! dégradation) et gestionnaires en mémoire qui renvoient des valeurs fixées par
//! l'appelant. Ce n'est **pas** une gate active : aucune détection réelle n'a lieu
//! (`DefaultThreatDetector` ne fait que restituer les signaux qu'on lui fournit)
//! et le niveau courant ne pilote aucune frontière.
//!
//! Prévu, non implémenté : signaux des Vigies via CaringNanny (lot 3), lien
//! niveau WorrySentinel → contraintes StrongFather / BorderGuard, et mécanismes
//! d'ESCALATION / OVERRIDE (constat K-17, à mettre au plan).

pub mod degradation;
pub mod security_level;
pub mod threat_detector;

pub use degradation::{
    DefaultDegradationManager, Degradation, DegradationManager, DegradationState,
};
pub use security_level::{DefaultSecurityLevelManager, SecurityLevel, SecurityLevelManager};
pub use threat_detector::{DefaultThreatDetector, ThreatDetector, ThreatLevel};
