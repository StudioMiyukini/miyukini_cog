//! Service de sauvegarde et restauration (Backup Restore Contract).
//!
//! Toute opération passe par KindMother ; validation StrongFather pour restauration.

use serde::{Deserialize, Serialize};

/// @id miyukiniadmin_backup_service_backup_record
/// @role data
/// @layer operator
/// @do represent_backup_entry
//@human / @human Enregistrement d'une sauvegarde disponible.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupRecord {
    /// @id miyukiniadmin_backup_record_id
    /// @role data
    /// @layer operator
    /// @do store_backup_identity
    //@human / @human Identifiant unique du backup.
    /// @depends miyukiniadmin_backup_service_backup_record
    pub id: String,
    /// @id miyukiniadmin_backup_record_timestamp
    /// @role data
    /// @layer operator
    /// @do store_timestamp
    //@human / @human Horodatage de création.
    /// @depends miyukiniadmin_backup_service_backup_record
    pub timestamp: String,
    /// @id miyukiniadmin_backup_record_type
    /// @role data
    /// @layer operator
    /// @do store_backup_type
    //@human / @human Type (full / incremental).
    /// @depends miyukiniadmin_backup_service_backup_record
    pub backup_type: String,
    /// @id miyukiniadmin_backup_record_size_bytes
    /// @role data
    /// @layer operator
    /// @do store_size
    //@human / @human Taille en octets (optionnel).
    /// @depends miyukiniadmin_backup_service_backup_record
    pub size_bytes: Option<u64>,
    /// @id miyukiniadmin_backup_record_status
    /// @role data
    /// @layer operator
    /// @do store_status
    //@human / @human Statut (SUCCESS / FAIL / IN_PROGRESS).
    /// @depends miyukiniadmin_backup_service_backup_record
    pub status: String,
}

/// @id miyukiniadmin_backup_service_trait
/// @role infrastructure
/// @layer operator
/// @do define_backup_service_contract
//@human / @human Contrat du service backup/restore (KindMother / StrongFather en production).
pub trait BackupService {
    /// @id miyukiniadmin_backup_service_list
    /// @role accessor
    /// @layer operator
    /// @do list_backups
    //@human / @human Liste des sauvegardes disponibles.
    /// @depends miyukiniadmin_backup_service_trait
    fn list(&self) -> Vec<BackupRecord>;

    /// @id miyukiniadmin_backup_service_create
    /// @role mutator
    /// @layer operator
    /// @do create_backup
    //@human / @human Crée une sauvegarde (validation StrongFather si manuel).
    /// @depends miyukiniadmin_backup_service_trait
    fn create(&self) -> CreateBackupResult;

    /// @id miyukiniadmin_backup_service_restore
    /// @role mutator
    /// @layer operator
    /// @do restore_from_backup
    //@human / @human Restaure depuis un backup (validation StrongFather obligatoire).
    /// @depends miyukiniadmin_backup_service_trait
    fn restore(&self, backup_id: &str, justification: Option<&str>) -> RestoreBackupResult;
}

/// @id miyukiniadmin_backup_service_create_result
/// @role data
/// @layer operator
/// @do represent_create_backup_result
//@human / @human Résultat de la création d'un backup.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBackupResult {
    /// @id miyukiniadmin_create_result_success
    /// @role data
    /// @layer operator
    /// @do store_success
    //@human / @human Succès ou échec.
    /// @depends miyukiniadmin_backup_service_create_result
    pub success: bool,
    /// @id miyukiniadmin_create_result_backup_id
    /// @role data
    /// @layer operator
    /// @do store_backup_id
    //@human / @human Identifiant du backup créé.
    /// @depends miyukiniadmin_backup_service_create_result
    pub backup_id: Option<String>,
    /// @id miyukiniadmin_create_result_message
    /// @role data
    /// @layer operator
    /// @do store_message
    //@human / @human Message ou erreur.
    /// @depends miyukiniadmin_backup_service_create_result
    pub message: Option<String>,
}

/// @id miyukiniadmin_backup_service_restore_result
/// @role data
/// @layer operator
/// @do represent_restore_result
//@human / @human Résultat de la restauration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreBackupResult {
    /// @id miyukiniadmin_restore_result_success
    /// @role data
    /// @layer operator
    /// @do store_success
    //@human / @human Succès ou échec.
    /// @depends miyukiniadmin_backup_service_restore_result
    pub success: bool,
    /// @id miyukiniadmin_restore_result_message
    /// @role data
    /// @layer operator
    /// @do store_message
    //@human / @human Message ou erreur.
    /// @depends miyukiniadmin_backup_service_restore_result
    pub message: Option<String>,
}

/// @id miyukiniadmin_backup_service_impl
/// @role infrastructure
/// @layer operator
/// @do implement_backup_service_stub
//@human / @human Implémentation stub du service backup/restore.
/// @depends miyukiniadmin_backup_service_trait
#[derive(Debug, Default)]
pub struct BackupServiceImpl {
    /// @id miyukiniadmin_backup_impl_backups
    /// @role data
    /// @layer operator
    /// @do store_backups
    //@human / @human Liste des backups (stub).
    /// @depends miyukiniadmin_backup_service_impl
    backups: Vec<BackupRecord>,
}

impl BackupServiceImpl {
    /// @id miyukiniadmin_backup_impl_new
    /// @role infrastructure
    /// @layer operator
    /// @do create_backup_service
    //@human / @human Crée le service avec état stub.
    /// @depends miyukiniadmin_backup_service_impl
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// @id miyukiniadmin_backup_impl_with_seed
    /// @role mutator
    /// @layer operator
    /// @do seed_demo_backups
    //@human / @human Initialise avec des backups de démo.
    /// @depends miyukiniadmin_backup_service_impl
    #[must_use]
    pub fn with_seed(mut self) -> Self {
        self.backups = vec![
            BackupRecord {
                id: "backup-20260129-001".to_string(),
                timestamp: "2026-01-29T08:00:00Z".to_string(),
                backup_type: "full".to_string(),
                size_bytes: Some(1_024 * 1_024 * 50),
                status: "SUCCESS".to_string(),
            },
            BackupRecord {
                id: "backup-20260128-001".to_string(),
                timestamp: "2026-01-28T08:00:00Z".to_string(),
                backup_type: "full".to_string(),
                size_bytes: Some(1_024 * 1_024 * 48),
                status: "SUCCESS".to_string(),
            },
        ];
        self
    }
}

impl BackupService for BackupServiceImpl {
    fn list(&self) -> Vec<BackupRecord> {
        self.backups.clone()
    }

    fn create(&self) -> CreateBackupResult {
        // Stub : en production, KindMother + StrongFather
        CreateBackupResult {
            success: true,
            backup_id: Some(format!(
                "backup-stub-{}",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs()
            )),
            message: Some("Stub: backup non exécuté (voir KindMother en production).".to_string()),
        }
    }

    fn restore(&self, backup_id: &str, _justification: Option<&str>) -> RestoreBackupResult {
        // Stub : en production, validation StrongFather + WorrySentinel obligatoire
        let found = self.backups.iter().any(|b| b.id == backup_id);
        RestoreBackupResult {
            success: found,
            message: if found {
                Some(
                    "Stub: restore non exécuté (voir StrongFather + KindMother en production)."
                        .to_string(),
                )
            } else {
                Some(format!("Backup non trouvé: {backup_id}"))
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @id miyukiniadmin_backup_test_list
    /// @role test
    /// @layer operator
    /// @do verify_backup_list
    //@human / @human Test liste backups.
    /// @depends miyukiniadmin_backup_impl_with_seed
    #[test]
    fn test_backup_list() {
        let svc = BackupServiceImpl::new().with_seed();
        assert_eq!(svc.list().len(), 2);
    }

    /// @id miyukiniadmin_backup_test_restore
    /// @role test
    /// @layer operator
    /// @do verify_restore_stub
    //@human / @human Test restore stub.
    /// @depends miyukiniadmin_backup_service_restore
    #[test]
    fn test_restore() {
        let svc = BackupServiceImpl::new().with_seed();
        let r = svc.restore("backup-20260129-001", Some("Test"));
        assert!(r.success);
        let r2 = svc.restore("unknown", None);
        assert!(!r2.success);
    }
}
