//! # BorderGuard
//!
//! Autorité de définition des frontières du Miyukini Core System.
//!
//! BorderGuard définit les frontières, classifie les niveaux de confiance, et établit
//! les règles de franchissement sans jamais appliquer ces règles.

/// @id: borderguard_module_boundary
/// @role: infrastructure
/// @layer: core
/// @human: Module de définition de frontière. Définit les frontières du système.
/// @do: expose_boundary_module
pub mod boundary;

/// @id: borderguard_module_trust_level
/// @role: infrastructure
/// @layer: core
/// @human: Module de niveaux de confiance. Classifie les niveaux de confiance.
/// @do: expose_trust_level_module
pub mod trust_level;

/// @id: borderguard_module_crossing
/// @role: infrastructure
/// @layer: core
/// @human: Module de règles de franchissement. Établit les règles de franchissement des frontières.
/// @do: expose_crossing_module
pub mod crossing;

pub use boundary::{Boundary, BoundaryType};
pub use crossing::{CrossingRule, CrossingRules};
pub use trust_level::{TrustLevel, TrustLevelClassifier};
