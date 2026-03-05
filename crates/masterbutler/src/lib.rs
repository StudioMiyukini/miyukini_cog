//! # MasterButler
//!
//! Orchestrateur de workflows du Miyukini Core System.
//!
//! MasterButler orchestre l'exécution de workflows en respectant les décisions de StrongFather
//! et en utilisant l'API de KindMother pour l'exécution.

pub mod orchestrator;
pub mod step;
pub mod workflow;

pub use orchestrator::{DefaultOrchestrator, Orchestrator};
pub use step::{Step, StepResult};
pub use workflow::{Workflow, WorkflowId};
