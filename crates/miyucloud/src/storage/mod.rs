//! Module de stockage MiyuCloud.
//!
//! @id: miyucloud_storage_module
//! @do: expose_storage_backend_and_utilities
//! @role: storage
//! @layer: infra
//!
//! Fournit le trait `StorageBackend`, l'implementation filesystem local,
//! le module de chunking et la verification d'integrite SHA-256.

pub mod chunking;
pub mod integrity;
pub mod local_fs;

pub use chunking::Chunker;
pub use integrity::{compute_sha256, verify_sha256};
pub use local_fs::LocalFsStorage;

use crate::errors::MiyucloudError;

/// Trait abstrait pour le stockage de chunks de fichiers.
///
/// Les implementations peuvent stocker sur le filesystem local, un object store,
/// ou un backend de test en memoire.
pub trait StorageBackend {
    /// Ecrit un chunk pour un fichier donne.
    fn write_chunk(
        &self,
        file_id: &str,
        chunk_index: u32,
        data: &[u8],
    ) -> Result<(), MiyucloudError>;

    /// Lit un chunk pour un fichier donne.
    fn read_chunk(&self, file_id: &str, chunk_index: u32) -> Result<Vec<u8>, MiyucloudError>;

    /// Supprime tous les chunks d'un fichier.
    fn delete_file(&self, file_id: &str) -> Result<(), MiyucloudError>;

    /// Verifie si un fichier (au moins un chunk) existe dans le stockage.
    fn exists(&self, file_id: &str) -> Result<bool, MiyucloudError>;

    /// Liste les indices de chunks presents pour un fichier.
    fn list_chunks(&self, file_id: &str) -> Result<Vec<u32>, MiyucloudError>;

    /// Ecrase un chunk avec des zeros avant suppression (securite RGPD).
    fn overwrite_chunk_with_zeros(
        &self,
        file_id: &str,
        chunk_index: u32,
    ) -> Result<(), MiyucloudError>;
}
