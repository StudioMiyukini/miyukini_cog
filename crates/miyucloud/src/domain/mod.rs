//! Logique metier MiyuCloud.
//!
//! @id: miyucloud_domain_module
//! @do: expose_file_operations_trash_sharing_quota_migration
//! @role: domain
//! @layer: domain
//!
//! Operations CRUD sur les fichiers et dossiers, corbeille avec suppression securisee,
//! partage interne Tribu, liens de partage externe, quotas et migration Jay1Tribu.

pub mod file_ops;
pub mod trash;
pub mod external_share;
pub mod sharing;
pub mod quota;
pub mod jay1tribu_migration;
pub mod onboarding;

pub use file_ops::FileOps;
pub use trash::TrashOps;
pub use external_share::ExternalShareOps;
pub use sharing::SharingOps;
pub use quota::QuotaOps;
