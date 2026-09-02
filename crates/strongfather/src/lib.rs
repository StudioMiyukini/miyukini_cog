//! # StrongFather
//!
//! Moteur de décision stratégique et politique du Miyukini Core System.
//!
//! StrongFather est le moteur de décision stratégique et politique qui évalue des intentions,
//! applique des politiques, établit des priorités, et produit des décisions sans jamais
//! posséder d'autorité sur l'exécution ou la persistance.
//!
//! ## Principes fondamentaux
//!
//! - **Aucune autorité sur l'exécution** : StrongFather produit des décisions mais ne les exécute jamais
//! - **Aucune autorité sur la persistance** : StrongFather ne modifie jamais de données persistées
//! - **Décisions pures** : Toutes les décisions sont non ambiguës et justifiables
//! - **Zero-trust** : Toute intention est évaluée selon les politiques, sans présupposer la validité de l'appelant
//! - **Pureté fonctionnelle** : Aucun effet de bord, déterminisme garanti
//!
//! ## Modules
//!
//! - **intent** : Modèle d'intention (demande d'évaluation)
//! - **policy** : Modèle de politique (règles déclaratives)
//! - **decision** : Modèle de décision (résultat d'évaluation)
//! - **policy_engine** : Moteur d'application des politiques
//! - **priority** : Gestion des priorités
//! - **validator** : Validation structurelle des intentions
//!
//! ## Références
//!
//! - [Documentation Fondatrice](../../docs/core/StrongFather/foundation/StrongFather%20-%20Documentation%20Fondatrice.md)
//! - [Core Decision Contract](../../docs/core/StrongFather/contracts/decision/StrongFather%20-%20Core%20Decision%20Contract.md)
//! - [Intent Model Contract](../../docs/core/StrongFather/contracts/intent/StrongFather%20-%20Intent%20Model%20Contract.md)
//! - [Policy Engine Contract](../../docs/core/StrongFather/contracts/policy/StrongFather%20-%20Policy%20Engine%20Contract.md)

/// @id strongfather_module_intent
/// @role infrastructure
/// @layer core
/// @do expose_intent_module
//@human / @human Module de modèle d'intention. Définit la structure des intentions soumises à StrongFather.
/// Module de modèle d'intention
pub mod intent;

/// @id strongfather_module_policy
/// @role infrastructure
/// @layer core
/// @do expose_policy_module
//@human / @human Module de modèle de politique. Définit la structure des politiques appliquées par StrongFather.
/// Module de modèle de politique
pub mod policy;

/// @id strongfather_module_decision
/// @role infrastructure
/// @layer core
/// @do expose_decision_module
//@human / @human Module de modèle de décision. Définit la structure des décisions produites par StrongFather.
/// Module de modèle de décision
pub mod decision;

/// @id strongfather_module_policy_engine
/// @role infrastructure
/// @layer core
/// @do expose_policy_engine_module
//@human / @human Module de moteur de politiques. Applique les politiques pour évaluer des intentions.
/// Module de moteur de politiques
pub mod policy_engine;

/// @id strongfather_module_priority
/// @role infrastructure
/// @layer core
/// @do expose_priority_module
//@human / @human Module de gestion des priorités. Calcule les priorités relatives des intentions.
/// Module de gestion des priorités
pub mod priority;

/// @id strongfather_module_validator
/// @role infrastructure
/// @layer core
/// @do expose_validator_module
//@human / @human Module de validation. Valide la structure des intentions avant évaluation.
/// Module de validation
pub mod validator;

// Ré-exports des types principaux pour faciliter l'usage

/// @id strongfather_reexport_intent
/// @role infrastructure
/// @layer core
/// @do reexport_intent_types
//@human / @human Ré-export des types d'intention pour faciliter l'usage.
/// @depends strongfather_module_intent
pub use intent::{ActionType, CallContext, Constraint, Intent, IntentData, IntentState};

/// @id strongfather_reexport_policy
/// @role infrastructure
/// @layer core
/// @do reexport_policy_types
//@human / @human Ré-export des types de politique pour faciliter l'usage.
/// @depends strongfather_module_policy
pub use policy::{
    Policy, PolicyCondition, PolicyEffect, PolicyId, PolicyPriorityLevel, PolicyResult, PolicyRule,
    PolicySet, PolicyType,
};

/// @id strongfather_reexport_decision
/// @role infrastructure
/// @layer core
/// @do reexport_decision_types
//@human / @human Ré-export des types de décision pour faciliter l'usage.
/// @depends strongfather_module_decision
pub use decision::{Decision, DecisionMetadata, DecisionType, EvaluationContext, Justification};

/// @id strongfather_reexport_policy_engine
/// @role infrastructure
/// @layer core
/// @do reexport_policy_engine
//@human / @human Ré-export du moteur de politiques pour faciliter l'usage.
/// @depends strongfather_module_policy_engine
pub use policy_engine::{PolicyEngine, PolicyEngineError};

/// @id strongfather_reexport_priority
/// @role infrastructure
/// @layer core
/// @do reexport_priority_types
//@human / @human Ré-export des types de priorité pour faciliter l'usage.
/// @depends strongfather_module_priority
pub use priority::{Priority, PriorityCalculator};

/// @id strongfather_reexport_validator
/// @role infrastructure
/// @layer core
/// @do reexport_validator
//@human / @human Ré-export du validateur pour faciliter l'usage.
/// @depends strongfather_module_validator
pub use validator::{IntentValidator, ValidationError};
