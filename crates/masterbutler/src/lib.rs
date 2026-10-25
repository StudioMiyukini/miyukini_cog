//! @id     masterbutler_lib
//! @do     declarer_module_orchestrator
//! @role   orchestration
//! @layer  core
//! @human  masterbutler — module « lib ».

//! # MasterButler
//!
//! Orchestrateur de workflows du Miyukini Core System.
//!
//! MasterButler décrit des workflows et leurs étapes. Selon D1 / C-31, il n'exécute
//! rien lui-même : l'exécution revient aux opérateurs et au Kernel. L'orchestrateur
//! par défaut refuse donc explicitement toute exécution (K-15) tant que cette
//! délégation n'est pas câblée.

// C-10 : aucun unwrap / expect / panic! / todo! / unimplemented! / unreachable! hors tests.
#![cfg_attr(
    not(test),
    deny(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::todo,
        clippy::unimplemented,
        clippy::unreachable
    )
)]

pub mod orchestrator;
pub mod step;
pub mod workflow;

pub use orchestrator::{DefaultOrchestrator, OrchestrationError, Orchestrator};
pub use step::{Step, StepResult};
pub use workflow::{Workflow, WorkflowId};
