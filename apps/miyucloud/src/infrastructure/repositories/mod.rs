//! @id     miyucloud_infrastructure_repositories_mod
//! @do     declarer_module_pg
//! @role   donnee
//! @layer  app
//! @human  miyucloud — module « infrastructure repositories mod ».

// Repositorios PostgreSQL (blob-storage model)
pub mod pg;

// Re-exportar para facilitar acceso
pub use pg::{
    AppPasswordPgRepository, DeviceCodePgRepository, FileBlobReadRepository,
    FileBlobWriteRepository, FolderDbRepository, SessionPgRepository, TrashDbRepository,
    UserPgRepository,
};
