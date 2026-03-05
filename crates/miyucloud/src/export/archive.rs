//! Export ZIP et import d'archives MiyuCloud.
//!
//! @id: miyucloud_export_archive
//! @do: implement_zip_export_import_with_manifest_json
//! @role: export
//! @layer: domain
//!
//! L'export cree un ZIP contenant les fichiers dechiffres et un manifest.json
//! avec les metadonnees (LOI-8 portabilite : les donnees sont toujours recuperables).
//! L'import lit un ZIP et cree les fichiers en base avec chiffrement at-rest.

use crate::crypto::KeyManager;
use crate::data::types::FileEntry;
use crate::errors::MiyucloudError;
use crate::storage::StorageBackend;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

/// Manifest JSON inclus dans l'archive ZIP.
///
/// Contient les metadonnees de tous les fichiers exportes
/// pour permettre la reconstitution de l'arborescence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchiveManifest {
    /// Version du format.
    pub version: String,
    /// Date d'export (ISO 8601).
    pub exported_at: String,
    /// Nombre total de fichiers.
    pub total_files: u32,
    /// Taille totale en octets (fichiers dechiffres).
    pub total_size_bytes: u64,
    /// Entrees du manifest.
    pub entries: Vec<ArchiveEntry>,
}

/// Entree dans le manifest d'archive.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchiveEntry {
    /// Nom du fichier.
    pub name: String,
    /// Chemin relatif dans l'archive.
    pub path: String,
    /// Taille en octets.
    pub size_bytes: u64,
    /// SHA-256 du contenu original.
    pub checksum_sha256: String,
    /// Type MIME.
    pub mime_type: String,
}

/// Exporte un dossier en archive ZIP.
///
/// Les fichiers sont dechiffres avant d'etre mis dans le ZIP (LOI-8 portabilite).
/// Un fichier `manifest.json` est inclus avec les metadonnees.
///
/// Retourne le chemin du fichier ZIP cree.
#[cfg(feature = "legacy-sqlite")]
pub fn export_folder(
    db: &crate::data::MiyucloudDb,
    storage: &dyn StorageBackend,
    key_manager: &KeyManager,
    folder_id: Option<&str>,
    owner_id: &str,
    output_path: &Path,
) -> Result<PathBuf, MiyucloudError> {
    use crate::crypto;

    // Get files in folder
    let files = db.file_list(folder_id, owner_id)?;

    if files.is_empty() {
        return Err(MiyucloudError::NotFound(
            "No files to export in this folder".into(),
        ));
    }

    // Create ZIP file
    let zip_path = output_path.to_path_buf();
    let zip_file = std::fs::File::create(&zip_path).map_err(MiyucloudError::Io)?;
    let mut zip_writer = zip::ZipWriter::new(zip_file);
    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);

    let mut manifest_entries = Vec::new();
    let mut total_size: u64 = 0;

    for file in &files {
        // Derive file key
        let file_key = key_manager.file_key(&file.id)?;

        // Read and decrypt all chunks
        let mut plaintext = Vec::new();
        for chunk_idx in 0..file.chunk_count {
            let stored = storage.read_chunk(&file.id, chunk_idx)?;
            if stored.len() < 12 {
                return Err(MiyucloudError::Crypto(
                    "Stored chunk too short for nonce".into(),
                ));
            }
            let mut nonce = [0u8; 12];
            nonce.copy_from_slice(&stored[..12]);
            let encrypted = crypto::EncryptedChunk {
                nonce,
                ciphertext: stored[12..].to_vec(),
            };
            let chunk_data = crypto::decrypt_chunk(&file_key, &encrypted)?;
            plaintext.extend_from_slice(&chunk_data);
        }

        // Verify checksum
        let checksum = crate::storage::compute_sha256(&plaintext);
        if checksum != file.checksum_sha256 {
            return Err(MiyucloudError::Integrity(format!(
                "Checksum mismatch for file {}: expected {}, got {checksum}",
                file.name, file.checksum_sha256
            )));
        }

        // Write to ZIP
        zip_writer
            .start_file(&file.name, options)
            .map_err(|e| MiyucloudError::Io(std::io::Error::other(e)))?;
        zip_writer
            .write_all(&plaintext)
            .map_err(MiyucloudError::Io)?;

        total_size += plaintext.len() as u64;

        manifest_entries.push(ArchiveEntry {
            name: file.name.clone(),
            path: file.name.clone(),
            size_bytes: plaintext.len() as u64,
            checksum_sha256: file.checksum_sha256.clone(),
            mime_type: file.mime_type.clone(),
        });
    }

    // Write manifest.json
    let manifest = ArchiveManifest {
        version: "1.0".into(),
        exported_at: chrono::Utc::now().to_rfc3339(),
        total_files: manifest_entries.len() as u32,
        total_size_bytes: total_size,
        entries: manifest_entries,
    };
    let manifest_json = serde_json::to_string_pretty(&manifest)
        .map_err(|e| MiyucloudError::Serialization(e.to_string()))?;

    zip_writer
        .start_file("manifest.json", options)
        .map_err(|e| MiyucloudError::Io(std::io::Error::other(e)))?;
    zip_writer
        .write_all(manifest_json.as_bytes())
        .map_err(MiyucloudError::Io)?;

    zip_writer
        .finish()
        .map_err(|e| MiyucloudError::Io(std::io::Error::other(e)))?;

    Ok(zip_path)
}

/// Importe une archive ZIP dans MiyuCloud.
///
/// Les fichiers sont lus depuis le ZIP et chiffres at-rest avant stockage.
/// Si un `manifest.json` est present, il est utilise pour les metadonnees.
///
/// Retourne la liste des fichiers importes.
#[cfg(feature = "legacy-sqlite")]
pub fn import_archive(
    db: &crate::data::MiyucloudDb,
    storage: &dyn StorageBackend,
    key_manager: &KeyManager,
    zip_path: &Path,
    parent_id: Option<&str>,
    owner_id: &str,
) -> Result<Vec<FileEntry>, MiyucloudError> {
    use crate::domain::file_ops::FileOps;

    let zip_file = std::fs::File::open(zip_path).map_err(MiyucloudError::Io)?;
    let mut archive =
        zip::ZipArchive::new(zip_file).map_err(|e| MiyucloudError::Io(std::io::Error::other(e)))?;

    let mut imported_files = Vec::new();

    for i in 0..archive.len() {
        let mut entry = archive
            .by_index(i)
            .map_err(|e| MiyucloudError::Io(std::io::Error::other(e)))?;

        let name = entry.name().to_string();

        // Skip manifest.json and directories
        if name == "manifest.json" || entry.is_dir() {
            continue;
        }

        // Read file content
        let mut data = Vec::new();
        entry.read_to_end(&mut data).map_err(MiyucloudError::Io)?;

        // Guess MIME type
        let mime_type = guess_mime_type(&name);

        // Upload via FileOps (handles chunking, encryption, DB insertion)
        let file_entry = FileOps::upload_file(
            db,
            storage,
            key_manager,
            owner_id,
            parent_id,
            &name,
            &mime_type,
            &data,
        )?;

        imported_files.push(file_entry);
    }

    Ok(imported_files)
}

/// Devine le type MIME d'un fichier a partir de son extension.
fn guess_mime_type(filename: &str) -> String {
    let ext = filename.rsplit('.').next().unwrap_or("").to_lowercase();

    match ext.as_str() {
        "txt" => "text/plain",
        "html" | "htm" => "text/html",
        "css" => "text/css",
        "js" => "application/javascript",
        "json" => "application/json",
        "xml" => "application/xml",
        "pdf" => "application/pdf",
        "zip" => "application/zip",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "webp" => "image/webp",
        "mp3" => "audio/mpeg",
        "wav" => "audio/wav",
        "ogg" => "audio/ogg",
        "mp4" => "video/mp4",
        "webm" => "video/webm",
        "rs" => "text/x-rust",
        "toml" => "application/toml",
        "md" => "text/markdown",
        _ => "application/octet-stream",
    }
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::KeyManager;
    use crate::domain::file_ops::FileOps;
    use crate::storage::local_fs::LocalFsStorage;

    fn setup_test_env() -> (
        crate::data::MiyucloudDb,
        LocalFsStorage,
        KeyManager,
        tempfile::TempDir,
    ) {
        let db = crate::data::MiyucloudDb::open(":memory:").unwrap();
        let tmp = tempfile::TempDir::new().unwrap();
        let storage_dir = tmp.path().join("storage");
        let storage = LocalFsStorage::new(storage_dir).unwrap();
        let km = KeyManager::from_master_key([42u8; 32]);
        (db, storage, km, tmp)
    }

    #[test]
    fn test_export_folder_creates_zip() {
        let (db, storage, km, tmp) = setup_test_env();

        // Upload a test file
        let data = b"Hello, export test!";
        FileOps::upload_file(
            &db,
            &storage,
            &km,
            "owner-1",
            None,
            "test.txt",
            "text/plain",
            data,
        )
        .unwrap();

        // Export
        let zip_path = tmp.path().join("export.zip");
        let result = export_folder(&db, &storage, &km, None, "owner-1", &zip_path);
        assert!(result.is_ok());

        let path = result.unwrap();
        assert!(path.exists());
        assert!(path.metadata().unwrap().len() > 0);
    }

    #[test]
    fn test_export_includes_manifest_json() {
        let (db, storage, km, tmp) = setup_test_env();

        // Upload a test file
        FileOps::upload_file(
            &db,
            &storage,
            &km,
            "owner-1",
            None,
            "doc.pdf",
            "application/pdf",
            b"PDF content",
        )
        .unwrap();

        // Export
        let zip_path = tmp.path().join("export_manifest.zip");
        export_folder(&db, &storage, &km, None, "owner-1", &zip_path).unwrap();

        // Read the ZIP and check for manifest.json
        let zip_file = std::fs::File::open(&zip_path).unwrap();
        let mut archive = zip::ZipArchive::new(zip_file).unwrap();

        let mut has_manifest = false;
        let mut has_file = false;
        for i in 0..archive.len() {
            let entry = archive.by_index(i).unwrap();
            match entry.name() {
                "manifest.json" => {
                    has_manifest = true;
                }
                "doc.pdf" => {
                    has_file = true;
                }
                _ => {}
            }
        }
        assert!(has_manifest, "ZIP should contain manifest.json");
        assert!(has_file, "ZIP should contain doc.pdf");

        // Parse manifest
        let mut manifest_entry = archive.by_name("manifest.json").unwrap();
        let mut manifest_json = String::new();
        manifest_entry.read_to_string(&mut manifest_json).unwrap();
        let manifest: ArchiveManifest = serde_json::from_str(&manifest_json).unwrap();
        assert_eq!(manifest.version, "1.0");
        assert_eq!(manifest.total_files, 1);
        assert_eq!(manifest.entries[0].name, "doc.pdf");
    }

    #[test]
    fn test_import_zip_creates_files() {
        let (db, storage, km, tmp) = setup_test_env();

        // Upload a test file first
        let original_data = b"Content to export and re-import";
        FileOps::upload_file(
            &db,
            &storage,
            &km,
            "owner-1",
            None,
            "round-trip.txt",
            "text/plain",
            original_data,
        )
        .unwrap();

        // Export
        let zip_path = tmp.path().join("import_test.zip");
        export_folder(&db, &storage, &km, None, "owner-1", &zip_path).unwrap();

        // Create a fresh DB and storage for import
        let db2 = crate::data::MiyucloudDb::open(":memory:").unwrap();
        let storage_dir2 = tmp.path().join("storage2");
        let storage2 = LocalFsStorage::new(storage_dir2).unwrap();
        let km2 = KeyManager::from_master_key([99u8; 32]); // Different key!

        // Import
        let imported = import_archive(&db2, &storage2, &km2, &zip_path, None, "owner-2").unwrap();

        assert_eq!(imported.len(), 1);
        assert_eq!(imported[0].name, "round-trip.txt");
        assert_eq!(imported[0].owner_id, "owner-2");

        // Verify the imported file can be downloaded with the new key
        let downloaded = FileOps::download_file(&db2, &storage2, &km2, &imported[0].id).unwrap();
        assert_eq!(downloaded, original_data);
    }

    #[test]
    fn test_export_empty_folder_returns_error() {
        let (db, storage, km, tmp) = setup_test_env();
        let zip_path = tmp.path().join("empty.zip");
        let result = export_folder(&db, &storage, &km, None, "owner-1", &zip_path);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("No files to export"));
    }

    #[test]
    fn test_guess_mime_type() {
        assert_eq!(guess_mime_type("photo.jpg"), "image/jpeg");
        assert_eq!(guess_mime_type("document.pdf"), "application/pdf");
        assert_eq!(guess_mime_type("script.rs"), "text/x-rust");
        assert_eq!(guess_mime_type("unknown.xyz"), "application/octet-stream");
        assert_eq!(guess_mime_type("noext"), "application/octet-stream");
    }

    #[test]
    fn test_archive_manifest_serialize_roundtrip() {
        let manifest = ArchiveManifest {
            version: "1.0".into(),
            exported_at: "2026-03-01T00:00:00Z".into(),
            total_files: 2,
            total_size_bytes: 1024,
            entries: vec![ArchiveEntry {
                name: "test.txt".into(),
                path: "test.txt".into(),
                size_bytes: 512,
                checksum_sha256: "abc".into(),
                mime_type: "text/plain".into(),
            }],
        };
        let json = serde_json::to_string(&manifest).unwrap();
        let restored: ArchiveManifest = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.version, "1.0");
        assert_eq!(restored.total_files, 2);
    }
}
