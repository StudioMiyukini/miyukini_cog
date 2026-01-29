//! Sauvegarde pre-destruction si aucune sauvegarde locale antérieure (Auth and First-Boot §3.5.4.3).
//!
//! Référence : MiyukiniAdmin - Implementation Security and Controls §9

use std::path::PathBuf;
use std::sync::Arc;

use crate::backup_service::{BackupService, BackupServiceImpl, CreateBackupResult};

/// @id: miyukiniadmin_pre_destruction_backup_service
/// @role: infrastructure
/// @layer: operator
/// @human: Sauvegarde comprimée avant destruction si pas de sauvegarde locale antérieure.
/// @do: run_pre_destruction_backup_if_needed
/// @depends: miyukiniadmin_backup_service_trait
#[derive(Clone)]
pub struct PreDestructionBackupService {
    /// Service backup (liste + create).
    backup_service: Arc<BackupServiceImpl>,
    /// Répertoire des backups (pre_destruction_<timestamp>) — réservé pour stockage fichier.
    #[allow(dead_code)]
    backups_dir: PathBuf,
}

impl PreDestructionBackupService {
    /// Crée le service.
    /// @id: miyukiniadmin_pre_destruction_backup_new
    /// @role: constructor
    /// @layer: operator
    /// @human: Construit le service pre-destruction backup.
    /// @do: create_pre_destruction_backup_service
    pub fn new(backup_service: Arc<BackupServiceImpl>, backups_dir: PathBuf) -> Self {
        Self {
            backup_service,
            backups_dir,
        }
    }

    /// Vérifie s'il existe au moins une sauvegarde locale antérieure (avant ce cycle).
    /// @id: miyukiniadmin_pre_destruction_has_prior
    fn has_prior_local_backup(&self) -> bool {
        !self.backup_service.list().is_empty()
    }

    /// Exécute la sauvegarde pre-destruction si aucune sauvegarde locale antérieure (Implementation §9.2).
    /// @id: miyukiniadmin_pre_destruction_run_if_needed
    /// @role: mutator
    /// @layer: operator
    /// @human: Crée un backup avant destruction si aucun backup existant.
    /// @do: run_pre_destruction_backup_if_needed
    pub fn run_if_needed(&self) -> PreDestructionBackupResult {
        if self.has_prior_local_backup() {
            return PreDestructionBackupResult {
                performed: false,
                backup_id: None,
                message: Some("Sauvegarde locale antérieure présente ; pas de nouveau backup.".to_string()),
            };
        }
        let result: CreateBackupResult = self.backup_service.create();
        if result.success {
            PreDestructionBackupResult {
                performed: true,
                backup_id: result.backup_id,
                message: result.message,
            }
        } else {
            PreDestructionBackupResult {
                performed: true,
                backup_id: None,
                message: result.message.or_else(|| Some("Échec création backup.".to_string())),
            }
        }
    }
}

/// Résultat de la sauvegarde pre-destruction.
/// @id: miyukiniadmin_pre_destruction_result
#[derive(Debug, Clone)]
pub struct PreDestructionBackupResult {
    /// True si un backup a été tenté/créé.
    pub performed: bool,
    /// ID du backup créé (si succès).
    pub backup_id: Option<String>,
    /// Message (audit).
    pub message: Option<String>,
}
