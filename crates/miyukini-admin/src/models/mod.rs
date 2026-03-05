//! Modèles de données MiyukiniAdmin (Auth and First-Boot, Authentication, Permission).
//!
//! Référence : MiyukiniAdmin - Implementation Security and Controls §3.2

pub mod admin_account;
pub mod corruption_memory;
/// @id: miyukiniadmin_models_mod
/// @role: infrastructure
/// @layer: operator
/// @human: Module modèles (environment_state, admin_account, session, corruption_memory).
/// @do: expose_models
pub mod environment_state;
pub mod session;

pub use admin_account::{AdminAccount, AdminRole};
pub use corruption_memory::CorruptionMemory;
pub use environment_state::EnvironmentState;
pub use session::AdminSession;
