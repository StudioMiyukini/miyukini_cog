//! Services MiyukiniAdmin (EnvironmentState, Auth, Permission, Recovery).
//!
//! Référence : MiyukiniAdmin - Implementation Security and Controls §3.2

/// @id: miyukiniadmin_services_mod
/// @role: infrastructure
/// @layer: operator
/// @human: Module services (environment_state, auth, permission, recovery).
/// @do: expose_services

pub mod environment_state;
pub mod auth;
pub mod permission;
pub mod pre_destruction_backup;
pub mod destruction_reinit;
pub mod auto_recovery;

pub use environment_state::EnvironmentStateService;
pub use auth::AuthService;
pub use permission::PermissionService;
pub use pre_destruction_backup::PreDestructionBackupService;
pub use destruction_reinit::DestructionAndReinitService;
pub use auto_recovery::AutoRecoveryService;
