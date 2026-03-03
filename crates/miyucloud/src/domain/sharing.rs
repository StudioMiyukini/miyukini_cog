//! Partage interne Tribu — permissions lecture/ecriture.
//!
//! @id: miyucloud_domain_sharing
//! @do: share_unshare_files_folders_with_profiles_and_tribes
//! @role: domain
//! @layer: domain
//!
//! Le partage interne permet de donner acces a des fichiers ou dossiers
//! a des profils ou des tribus entieres, avec un niveau de permission
//! (lecture seule ou lecture/ecriture).
//!
//! Le partage d'un dossier donne acces a tout son contenu recursivement.

use crate::data::types::{GranteeType, PermissionLevel, SharePermission};
use crate::errors::MiyucloudError;

#[cfg(feature = "legacy-sqlite")]
use crate::data::MiyucloudDb;

/// Operations de partage interne (Tribu).
pub struct SharingOps;

/// Element partage : fichier ou dossier avec sa permission.
#[derive(Debug, Clone)]
pub struct SharedItem {
    /// La permission de partage.
    pub permission: SharePermission,
    /// Nom de l'element (fichier ou dossier).
    pub name: String,
    /// `true` si c'est un dossier, `false` si fichier.
    pub is_folder: bool,
}

impl SharingOps {
    /// Partage un fichier ou dossier avec un profil ou une tribu.
    ///
    /// - Exactement un de `file_id` ou `folder_id` doit etre fourni.
    /// - Retourne la permission creee.
    #[cfg(feature = "legacy-sqlite")]
    pub fn share(
        db: &MiyucloudDb,
        file_id: Option<&str>,
        folder_id: Option<&str>,
        grantee_id: &str,
        grantee_type: GranteeType,
        permission: PermissionLevel,
    ) -> Result<SharePermission, MiyucloudError> {
        // Validate exactly one of file_id or folder_id
        match (file_id, folder_id) {
            (Some(_), None) | (None, Some(_)) => {}
            _ => {
                return Err(MiyucloudError::InvalidInput(
                    "Exactly one of file_id or folder_id must be provided".to_string(),
                ));
            }
        }

        // Verify the target exists
        if let Some(fid) = file_id {
            if db.file_by_id(fid)?.is_none() {
                return Err(MiyucloudError::NotFound(format!(
                    "File {fid} not found"
                )));
            }
        }
        if let Some(foid) = folder_id {
            if db.folder_by_id(foid)?.is_none() {
                return Err(MiyucloudError::NotFound(format!(
                    "Folder {foid} not found"
                )));
            }
        }

        let now = chrono::Utc::now().to_rfc3339();
        let perm = SharePermission {
            id: uuid::Uuid::new_v4().to_string(),
            file_id: file_id.map(String::from),
            folder_id: folder_id.map(String::from),
            grantee_id: grantee_id.to_string(),
            grantee_type,
            permission,
            created_at: now,
        };

        db.share_permission_create(&perm)
    }

    /// Supprime une permission de partage.
    #[cfg(feature = "legacy-sqlite")]
    pub fn unshare(db: &MiyucloudDb, permission_id: &str) -> Result<(), MiyucloudError> {
        db.share_permission_delete(permission_id)
    }

    /// Liste les elements partages avec un profil.
    ///
    /// Retourne les permissions avec les noms des ressources associees.
    #[cfg(feature = "legacy-sqlite")]
    pub fn list_shared_with_me(
        db: &MiyucloudDb,
        profile_id: &str,
    ) -> Result<Vec<SharedItem>, MiyucloudError> {
        let conn = db.lock_conn()?;
        let mut stmt = conn.prepare(
            "SELECT sp.id, sp.file_id, sp.folder_id, sp.grantee_id, sp.grantee_type,
                    sp.permission, sp.created_at
             FROM cloud_share_permissions sp
             WHERE sp.grantee_id = ?1
             ORDER BY sp.created_at DESC",
        )?;

        let perms: Vec<SharePermission> = stmt
            .query_map(rusqlite::params![profile_id], |row| {
                Ok(SharePermission {
                    id: row.get(0)?,
                    file_id: row.get(1)?,
                    folder_id: row.get(2)?,
                    grantee_id: row.get(3)?,
                    grantee_type: GranteeType::from_str_value(
                        &row.get::<_, String>(4).unwrap_or_default(),
                    ),
                    permission: PermissionLevel::from_str_value(
                        &row.get::<_, String>(5).unwrap_or_default(),
                    ),
                    created_at: row.get(6)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| MiyucloudError::Db(e.to_string()))?;

        drop(stmt);
        drop(conn);

        let mut items = Vec::new();
        for perm in perms {
            if let Some(fid) = &perm.file_id {
                if let Some(file) = db.file_by_id(fid)? {
                    items.push(SharedItem {
                        permission: perm,
                        name: file.name,
                        is_folder: false,
                    });
                }
            } else if let Some(foid) = &perm.folder_id {
                if let Some(folder) = db.folder_by_id(foid)? {
                    items.push(SharedItem {
                        permission: perm,
                        name: folder.name,
                        is_folder: true,
                    });
                }
            }
        }

        Ok(items)
    }

    /// Liste les permissions pour une ressource specifique (fichier ou dossier).
    #[cfg(feature = "legacy-sqlite")]
    pub fn list_shares_for_resource(
        db: &MiyucloudDb,
        resource_id: &str,
    ) -> Result<Vec<SharePermission>, MiyucloudError> {
        db.share_permission_list(resource_id)
    }
}

#[cfg(all(test, feature = "legacy-sqlite"))]
mod tests {
    use super::*;
    use crate::crypto::KeyManager;
    use crate::data::MiyucloudDb;
    use crate::domain::FileOps;
    use crate::storage::local_fs::LocalFsStorage;

    fn setup() -> (MiyucloudDb, LocalFsStorage, KeyManager) {
        let db = MiyucloudDb::open(":memory:").unwrap();
        let dir = tempfile::tempdir().unwrap();
        let storage = LocalFsStorage::new(dir.into_path()).unwrap();
        let km = KeyManager::from_master_key([1u8; 32]);
        (db, storage, km)
    }

    #[test]
    fn test_share_file_with_profile() {
        let (db, storage, km) = setup();
        let file = FileOps::upload_file(
            &db, &storage, &km, "owner1", None, "test.txt", "text/plain", b"data",
        )
        .unwrap();

        let perm = SharingOps::share(
            &db,
            Some(&file.id),
            None,
            "user-2",
            GranteeType::Profile,
            PermissionLevel::Read,
        )
        .unwrap();

        assert_eq!(perm.file_id.as_deref(), Some(file.id.as_str()));
        assert_eq!(perm.grantee_id, "user-2");
        assert_eq!(perm.permission, PermissionLevel::Read);
    }

    #[test]
    fn test_share_folder_with_tribe() {
        let (db, _storage, _km) = setup();
        let folder = FileOps::create_folder(&db, "owner1", None, "SharedDocs").unwrap();

        let perm = SharingOps::share(
            &db,
            None,
            Some(&folder.id),
            "tribe-alpha",
            GranteeType::Tribe,
            PermissionLevel::ReadWrite,
        )
        .unwrap();

        assert_eq!(perm.folder_id.as_deref(), Some(folder.id.as_str()));
        assert_eq!(perm.grantee_type, GranteeType::Tribe);
        assert_eq!(perm.permission, PermissionLevel::ReadWrite);
    }

    #[test]
    fn test_shared_user_can_read() {
        let (db, storage, km) = setup();
        let file = FileOps::upload_file(
            &db, &storage, &km, "owner1", None, "test.txt", "text/plain", b"data",
        )
        .unwrap();

        let perm = SharingOps::share(
            &db,
            Some(&file.id),
            None,
            "user-2",
            GranteeType::Profile,
            PermissionLevel::Read,
        )
        .unwrap();

        // Verify through auth module
        let perms = vec![perm];
        let result = crate::auth::verify_access(
            "owner1",
            "user-2",
            &perms,
            crate::data::types::AccessAction::Download,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_shared_user_cannot_write_if_read_only() {
        let (db, storage, km) = setup();
        let file = FileOps::upload_file(
            &db, &storage, &km, "owner1", None, "test.txt", "text/plain", b"data",
        )
        .unwrap();

        let perm = SharingOps::share(
            &db,
            Some(&file.id),
            None,
            "user-2",
            GranteeType::Profile,
            PermissionLevel::Read,
        )
        .unwrap();

        // Verify write access is denied
        let perms = vec![perm];
        let result =
            crate::auth::permissions::verify_write_access("owner1", "user-2", &perms);
        assert!(result.is_err());
    }

    #[test]
    fn test_unshare_removes_permission() {
        let (db, storage, km) = setup();
        let file = FileOps::upload_file(
            &db, &storage, &km, "owner1", None, "test.txt", "text/plain", b"data",
        )
        .unwrap();

        let perm = SharingOps::share(
            &db,
            Some(&file.id),
            None,
            "user-2",
            GranteeType::Profile,
            PermissionLevel::Read,
        )
        .unwrap();

        // Verify it exists
        let list_before = SharingOps::list_shares_for_resource(&db, &file.id).unwrap();
        assert_eq!(list_before.len(), 1);

        // Unshare
        SharingOps::unshare(&db, &perm.id).unwrap();

        // Verify it's gone
        let list_after = SharingOps::list_shares_for_resource(&db, &file.id).unwrap();
        assert_eq!(list_after.len(), 0);
    }

    #[test]
    fn test_list_shared_with_me() {
        let (db, storage, km) = setup();
        let file = FileOps::upload_file(
            &db, &storage, &km, "owner1", None, "shared.txt", "text/plain", b"data",
        )
        .unwrap();

        SharingOps::share(
            &db,
            Some(&file.id),
            None,
            "user-2",
            GranteeType::Profile,
            PermissionLevel::Read,
        )
        .unwrap();

        let items = SharingOps::list_shared_with_me(&db, "user-2").unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "shared.txt");
        assert!(!items[0].is_folder);
    }
}
