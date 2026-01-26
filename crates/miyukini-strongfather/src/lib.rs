//! StrongFather - Moteur d'évaluation et de décision du Miyukini Core System
//!
//! Ce crate implémente StrongFather, le moteur d'évaluation et de décision
//! qui produit des décisions non ambiguës basées sur des politiques explicites.

pub mod error;
pub mod intent;
pub mod types;
pub mod policy;
pub mod decision;
pub mod validator;
pub mod policy_source;
pub mod policy_engine;
pub mod result_composer;
pub mod priority;
pub mod producer;
pub mod tracer;
pub mod surface;

// Ré-exports publics des types
pub use error::SFError;
pub use intent::{CallContext, Constraint, Intent, IntentData, IntentState};
pub use types::{
    ActionType, Clarification, DecisionType, DeferralReason, Justification, LogicalOperator,
    PolicyEffect, PolicyId, PolicyPriority, PolicyType, Priority, ReasoningStep, RefusalReason,
};
pub use policy::{Policy, PolicyCondition, PolicyRule, PolicySet};
pub use decision::{Decision, DecisionMetadata, EvaluationContext};
pub use validator::IntentValidator;
pub use policy_source::{MemoryPolicySource, PolicySourceTrait};
pub use policy_engine::{PolicyEngine, PolicyResult, PolicyResultType};
pub use result_composer::{ComposedResult, ResultComposer};
pub use priority::PriorityCalculator;
pub use producer::DecisionProducer;
pub use tracer::Tracer;
pub use surface::StrongFather;
