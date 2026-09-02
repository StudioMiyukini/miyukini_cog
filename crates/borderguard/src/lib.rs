//! # BorderGuard
//!
//! Autorité de définition des frontières du Miyukini Core System.
//!
//! BorderGuard définit les frontières, classifie les niveaux de confiance, et établit
//! les règles de franchissement sans jamais appliquer ces règles.

/// @id borderguard_module_boundary
/// @role infrastructure
/// @layer core
/// @do expose_boundary_module
//@human / @human Module de définition de frontière. Définit les frontières du système.
pub mod boundary;

/// @id borderguard_module_trust_level
/// @role infrastructure
/// @layer core
/// @do expose_trust_level_module
//@human / @human Module de niveaux de confiance. Classifie les niveaux de confiance.
pub mod trust_level;

/// @id borderguard_module_crossing
/// @role infrastructure
/// @layer core
/// @do expose_crossing_module
//@human / @human Module de règles de franchissement. Établit les règles de franchissement des frontières.
pub mod crossing;

/// @id borderguard_module_metadata
/// @role infrastructure
/// @layer core
/// @do declarer_module_metadata
//@human / @human Métadonnées de traçabilité (INV-BG-8).
pub mod metadata;

/// @id borderguard_module_registry
/// @role infrastructure
/// @layer core
/// @do declarer_module_registry
//@human / @human Registre en mémoire des frontières (consultation uniquement).
pub mod registry;

pub use boundary::{Boundary, BoundaryType};
pub use crossing::{CrossingRule, CrossingRules};
pub use metadata::BoundaryMetadata;
pub use registry::BoundaryRegistry;
pub use trust_level::{DefaultTrustLevelClassifier, TrustLevel, TrustLevelClassifier};
