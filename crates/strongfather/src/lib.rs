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
//! - **Aucune autorité sur l'exécution** : StrongFather produit des décisions mais ne les exécute jamais (INV-SF-1)
//! - **Aucune autorité sur la persistance** : StrongFather ne modifie jamais de données persistées (INV-SF-2)
//! - **Décisions pures** : Toutes les décisions sont non ambiguës et justifiables
//! - **Zero-trust** : Toute intention est évaluée selon les politiques, sans présupposer la validité de l'appelant
//! - **Pureté fonctionnelle** : Aucun effet de bord, déterminisme garanti
//!
//! ## API publique de décision
//!
//! - `PolicyEngine::decide(&self, &Intent) -> Decision` : pure, déterministe, synchrone ; **refus par défaut**
//! - `PolicyEngine::try_new(PolicySet)` : moteur validé (expressions, cohérence, composites) ;
//!   `PolicyEngine::new` accepte tout mais refuse toute intention visée par une politique invalide
//! - `PolicyEngine::load_file(path)` / `PolicySet::from_toml_str` / `from_json_str` : chargeur
//!   fail-closed (illisible, invalide ou vide ⇒ erreur, jamais « vide ⇒ autoriser »)
//! - `PolicyEngine` se clone à coût constant (`Arc`) : un moteur par processus, partagé entre appels
//! - `Decision::verdict()` → `Verdict::Allow` / `Verdict::Deny` ; `deciding_rule` (id de la
//!   politique décisive ou `"défaut: refus"`) ; `reason()` ; `intent_digest` (SHA-256 canonique)
//!
//! ## Ordre de décision
//!
//! - (1) Intention structurellement invalide ⇒ Deny (`validation-structurelle`)
//! - (2) Sélection des politiques par cible exacte (`PolicyTarget` : action, sujet, appelant,
//!   origine, zone) — jamais par sous-chaîne ; ordre stable priorité ↓ puis id ↑
//! - (3) Erreur d'évaluation (expression mal formée, attribut inconnu, type, composite
//!   absente ou cyclique, type/effet incohérent) ⇒ Deny immédiat (C-12)
//! - (4) `Deny` applicable et règle vraie ⇒ Deny (l'emporte toujours sur Authorize)
//! - (5) `Constrain` / `Validate` applicable et règle fausse ⇒ Deny
//! - (6) `Authorize` applicable et règle vraie ⇒ Allow
//! - (7) Sinon ⇒ Deny « défaut: refus »
//!
//! ## Joindre une décision à une action
//!
//! - L'appelant ne branche pas StrongFather directement : il passe par la porte de
//!   BondingBrother (`bondingbrother::Gate`), qui seule délivre un jeton d'autorisation
//! - La `Decision` porte `intent_id` et `intent_digest` : on la journalise avec l'action ;
//!   un auditeur vérifie `decision.matches_intent(&intent)` (même id, même empreinte)
//!
//! ## Modules
//!
//! - **intent** : Modèle d'intention (demande d'évaluation) et empreinte canonique
//! - **policy** : Modèle de politique (règles déclaratives, cible structurée)
//! - **decision** : Modèle de décision (résultat d'évaluation, verdict)
//! - **expr** : Évaluateur d'expressions borné (conditions et règles)
//! - **policy_engine** : Moteur d'application des politiques et `decide`
//! - **loader** : Chargement TOML / JSON fail-closed
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

/// @id strongfather_module_expr
/// @role infrastructure
/// @layer core
/// @do expose_expr_module
//@human / @human Module d'évaluation d'expressions (conditions et règles), borné et sans `eval`.
/// Module d'évaluation d'expressions
pub mod expr;

/// @id strongfather_module_loader
/// @role infrastructure
/// @layer core
/// @do expose_loader_module
//@human / @human Module de chargement des politiques (TOML / JSON), fail-closed.
/// Module de chargement des politiques
pub mod loader;

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
    PolicySet, PolicyTarget, PolicyType, ZONE_METADATA_KEY,
};

/// @id strongfather_reexport_decision
/// @role infrastructure
/// @layer core
/// @do reexport_decision_types
//@human / @human Ré-export des types de décision pour faciliter l'usage.
/// @depends strongfather_module_decision
pub use decision::{
    Decision, DecisionMetadata, DecisionType, EvaluationContext, Justification, Verdict,
    DEFAULT_DENY_RULE,
};

/// @id strongfather_reexport_policy_engine
/// @role infrastructure
/// @layer core
/// @do reexport_policy_engine
//@human / @human Ré-export du moteur de politiques pour faciliter l'usage.
/// @depends strongfather_module_policy_engine
pub use policy_engine::{PolicyEngine, PolicyEngineError, STRUCTURAL_VALIDATION_RULE};

/// @id strongfather_reexport_expr_loader
/// @role infrastructure
/// @layer core
/// @do reexport_expr_and_loader
//@human / @human Ré-export de l'évaluateur d'expressions et des erreurs de chargement.
pub use expr::{Expr, ExprError};
pub use loader::PolicyLoadError;

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
