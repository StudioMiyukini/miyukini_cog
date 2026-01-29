//! Modèles de données MiyukiniAdmin (Auth and First-Boot, Authentication, Permission).
//!
//! Référence : MiyukiniAdmin - Implementation Security and Controls §3.2

/// @id: miyukiniadmin_models_mod
/// @role: infrastructure
/// @layer: operator
/// @human: Module modèles (environment_state, admin_account, session, corruption_memory).
/// @do: expose_models

pub mod environment_state;
pub mod admin_account;
pub mod session;
pub mod corruption_memory;

pub use environment_state::EnvironmentState;
pub use admin_account::{AdminAccount, AdminRole};
pub use session::AdminSession;
pub use corruption_memory::CorruptionMemory;
