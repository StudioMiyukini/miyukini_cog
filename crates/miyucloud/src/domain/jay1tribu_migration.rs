//! Migration des pieces jointes Jay1Tribu vers MiyuCloud.
//!
//! @id: miyucloud_domain_jay1tribu_migration
//! @do: migrate_jay1tribu_attachments_to_miyucloud_non_destructive
//! @role: domain
//! @layer: domain
//!
//! Migration non-destructive et idempotente : les fichiers originaux dans
//! Jay1Tribu ne sont PAS supprimes. La correspondance est enregistree dans
//! `cloud_jay1tribu_migrations` pour eviter les doublons.
//!
//! INVARIANT : Les fichiers migres sont chiffres at-rest dans MiyuCloud.
//! INVARIANT : La migration est idempotente (relancer ne cree pas de doublons).

use crate::crypto::KeyManager;
use crate::data::types::FileEntry;
use crate::domain::FileOps;
use crate::errors::MiyucloudError;
use crate::storage::StorageBackend;

#[cfg(feature = "legacy-sqlite")]
use crate::data::MiyucloudDb;

/// Rapport de migration.
#[derive(Debug, Clone)]
pub struct MigrationReport {
    /// Nombre de fichiers migres avec succes.
    pub migrated: u32,
    /// Nombre de fichiers ignores (deja migres).
    pub skipped: u32,
    /// Erreurs rencontrees (message_id, erreur).
    pub errors: Vec<(String, String)>,
}

/// Element a migrer depuis Jay1Tribu.
#[derive(Debug, Clone)]
pub struct Jay1TribuAttachment {
    /// ID du message original dans Jay1Tribu.
    pub message_id: String,
    /// Nom du fichier.
    pub file_name: String,
    /// Type MIME.
    pub mime_type: String,
    /// Donnees du fichier.
    pub data: Vec<u8>,
}

/// Migre une liste de pieces jointes Jay1Tribu vers MiyuCloud.
///
/// - `attachments` : liste des fichiers a migrer.
/// - `owner_id` : proprietaire MiyuCloud cible.
/// - `parent_folder_id` : dossier cible dans MiyuCloud (None = racine).
///
/// Retourne un rapport detaillant les fichiers migres, ignores et en erreur.
#[cfg(feature = "legacy-sqlite")]
pub fn migrate_jay1tribu_attachments(
    db: &MiyucloudDb,
    storage: &dyn StorageBackend,
    key_manager: &KeyManager,
    owner_id: &str,
    parent_folder_id: Option<&str>,
    attachments: &[Jay1TribuAttachment],
) -> Result<MigrationReport, MiyucloudError> {
    let mut report = MigrationReport {
        migrated: 0,
        skipped: 0,
        errors: Vec::new(),
    };

    for attachment in attachments {
        // Check if already migrated
        if is_already_migrated(db, &attachment.message_id)? {
            report.skipped += 1;
            continue;
        }

        // Attempt migration
        match migrate_single_attachment(
            db,
            storage,
            key_manager,
            owner_id,
            parent_folder_id,
            attachment,
        ) {
            Ok(file_entry) => {
                // Record the migration
                record_migration(db, &attachment.message_id, &file_entry.id)?;
                report.migrated += 1;
            }
            Err(e) => {
                report
                    .errors
                    .push((attachment.message_id.clone(), e.to_string()));
            }
        }
    }

    Ok(report)
}

/// Verifie si un message Jay1Tribu a deja ete migre.
#[cfg(feature = "legacy-sqlite")]
fn is_already_migrated(db: &MiyucloudDb, message_id: &str) -> Result<bool, MiyucloudError> {
    let conn = db.lock_conn()?;
    let count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM cloud_jay1tribu_migrations WHERE jay1tribu_msg_id = ?1",
            rusqlite::params![message_id],
            |row| row.get(0),
        )
        .map_err(|e| MiyucloudError::Db(e.to_string()))?;
    Ok(count > 0)
}

/// Migre un seul fichier : upload dans MiyuCloud (chunking + chiffrement).
#[cfg(feature = "legacy-sqlite")]
fn migrate_single_attachment(
    db: &MiyucloudDb,
    storage: &dyn StorageBackend,
    key_manager: &KeyManager,
    owner_id: &str,
    parent_folder_id: Option<&str>,
    attachment: &Jay1TribuAttachment,
) -> Result<FileEntry, MiyucloudError> {
    FileOps::upload_file(
        db,
        storage,
        key_manager,
        owner_id,
        parent_folder_id,
        &attachment.file_name,
        &attachment.mime_type,
        &attachment.data,
    )
}

/// Enregistre la correspondance dans `cloud_jay1tribu_migrations`.
#[cfg(feature = "legacy-sqlite")]
fn record_migration(
    db: &MiyucloudDb,
    message_id: &str,
    file_id: &str,
) -> Result<(), MiyucloudError> {
    let conn = db.lock_conn()?;
    let now = chrono::Utc::now().to_rfc3339();
    let id = uuid::Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO cloud_jay1tribu_migrations (id, jay1tribu_msg_id, file_id, migrated_at)
         VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![id, message_id, file_id, now],
    )?;
    Ok(())
}

#[cfg(all(test, feature = "legacy-sqlite"))]
mod tests {
    use super::*;
    use crate::data::MiyucloudDb;
    use crate::storage::local_fs::LocalFsStorage;

    fn setup() -> (MiyucloudDb, LocalFsStorage, KeyManager) {
        let db = MiyucloudDb::open(":memory:").unwrap();
        let dir = tempfile::tempdir().unwrap();
        let storage = LocalFsStorage::new(dir.into_path()).unwrap();
        let km = KeyManager::from_master_key([1u8; 32]);
        (db, storage, km)
    }

    #[test]
    fn test_migrate_attachment_creates_file_entry() {
        let (db, storage, km) = setup();
        let attachments = vec![Jay1TribuAttachment {
            message_id: "msg-001".to_string(),
            file_name: "photo.jpg".to_string(),
            mime_type: "image/jpeg".to_string(),
            data: b"fake jpeg data".to_vec(),
        }];

        let report =
            migrate_jay1tribu_attachments(&db, &storage, &km, "owner1", None, &attachments)
                .unwrap();

        assert_eq!(report.migrated, 1);
        assert_eq!(report.skipped, 0);
        assert!(report.errors.is_empty());

        // Verify file exists in MiyuCloud
        let files = db.file_list(None, "owner1").unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].name, "photo.jpg");
    }

    #[test]
    fn test_migrate_already_migrated_skips() {
        let (db, storage, km) = setup();
        let attachments = vec![Jay1TribuAttachment {
            message_id: "msg-002".to_string(),
            file_name: "doc.pdf".to_string(),
            mime_type: "application/pdf".to_string(),
            data: b"fake pdf data".to_vec(),
        }];

        // First migration
        let report1 =
            migrate_jay1tribu_attachments(&db, &storage, &km, "owner1", None, &attachments)
                .unwrap();
        assert_eq!(report1.migrated, 1);

        // Second migration (same data)
        let report2 =
            migrate_jay1tribu_attachments(&db, &storage, &km, "owner1", None, &attachments)
                .unwrap();
        assert_eq!(report2.migrated, 0);
        assert_eq!(report2.skipped, 1);
    }

    #[test]
    fn test_migration_report_counts() {
        let (db, storage, km) = setup();
        let attachments = vec![
            Jay1TribuAttachment {
                message_id: "msg-a".to_string(),
                file_name: "a.txt".to_string(),
                mime_type: "text/plain".to_string(),
                data: b"data a".to_vec(),
            },
            Jay1TribuAttachment {
                message_id: "msg-b".to_string(),
                file_name: "b.txt".to_string(),
                mime_type: "text/plain".to_string(),
                data: b"data b".to_vec(),
            },
            Jay1TribuAttachment {
                message_id: "msg-c".to_string(),
                file_name: "c.txt".to_string(),
                mime_type: "text/plain".to_string(),
                data: b"data c".to_vec(),
            },
        ];

        // Migrate first two
        let report1 =
            migrate_jay1tribu_attachments(&db, &storage, &km, "owner1", None, &attachments[..2])
                .unwrap();
        assert_eq!(report1.migrated, 2);

        // Migrate all three (first two should be skipped)
        let report2 =
            migrate_jay1tribu_attachments(&db, &storage, &km, "owner1", None, &attachments)
                .unwrap();
        assert_eq!(report2.migrated, 1);
        assert_eq!(report2.skipped, 2);
        assert!(report2.errors.is_empty());
    }
}
