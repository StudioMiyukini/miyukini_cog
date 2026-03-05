//! Module d'export et import MiyuCloud.
//!
//! @id: miyucloud_export_module
//! @do: expose_zip_export_import_with_manifest
//! @role: export
//! @layer: domain
//!
//! Export de dossiers en archives ZIP avec manifest JSON (LOI-8 portabilite).
//! Import d'archives ZIP avec chiffrement at-rest des fichiers importes.

pub mod archive;

pub use archive::{export_folder, import_archive, ArchiveEntry, ArchiveManifest};
