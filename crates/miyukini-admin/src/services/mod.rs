//! Services MiyukiniAdmin (EnvironmentState, Auth, Permission, Recovery).
//!
//! Référence : MiyukiniAdmin - Implementation Security and Controls §3.2

pub mod auth;
pub mod auto_recovery;
pub mod destruction_reinit;
/// @id miyukiniadmin_services_mod
/// @role infrastructure
/// @layer operator
/// @do expose_services
//@human / @human Module services (environment_state, auth, permission, recovery).
pub mod environment_state;
pub mod permission;
pub mod pre_destruction_backup;
pub mod setup_guard;

pub use auth::AuthService;
pub use auto_recovery::AutoRecoveryService;
pub use destruction_reinit::DestructionAndReinitService;
pub use environment_state::EnvironmentStateService;
pub use permission::PermissionService;
pub use pre_destruction_backup::PreDestructionBackupService;
pub use setup_guard::SetupGuard;
