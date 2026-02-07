//! Destruction DB et réinitialisation vers vierge avec mémoire de corruption (Auth and First-Boot §3.5.4.4).
//!
//! Référence : MiyukiniAdmin - Implementation Security and Controls §10

use chrono::Utc;
use std::path::PathBuf;
use tokio::fs;

use crate::models::{CorruptionMemory, EnvironmentState};
use crate::services::PreDestructionBackupService;

/// @id: miyukiniadmin_destruction_reinit_service
/// @role: infrastructure
/// @layer: operator
/// @human: Destruction des artefacts et réinit vers VIERGE avec mémoire de corruption.
/// @do: run_destruction_and_reinit
/// @depends: miyukiniadmin_models_corruption_memory, miyukiniadmin_services_environment_state
#[derive(Clone)]
pub struct DestructionAndReinitService {
    /// Répertoire de données (EIP, registre admin, schéma, corruption_memory).
    data_dir: PathBuf,
    /// Service pre-destruction backup (appelé avant wipe).
    pre_destruction_backup: PreDestructionBackupService,
}

impl DestructionAndReinitService {
    /// Crée le service.
    /// @id: miyukiniadmin_destruction_reinit_new
    /// @role: constructor
    /// @layer: operator
    /// @human: Construit le service destruction/réinit.
    /// @do: create_destruction_reinit_service
    #[must_use] 
    pub fn new(data_dir: PathBuf, pre_destruction_backup: PreDestructionBackupService) -> Self {
        Self {
            data_dir,
            pre_destruction_backup,
        }
    }

    /// Chemin du fichier mémoire de corruption (hors DB, jamais effacé par wipe).
    fn corruption_memory_path(&self) -> PathBuf {
        self.data_dir.join("corruption_memory.json")
    }

    /// Écrit la mémoire de corruption (Implementation §10.1 point 2).
    /// @id: miyukiniadmin_destruction_write_corruption_memory
    async fn write_corruption_memory(&self, memory: &CorruptionMemory) -> Result<(), std::io::Error> {
        let path = self.corruption_memory_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }
        let content = serde_json::to_string_pretty(memory).unwrap_or_default();
        fs::write(&path, content).await
    }

    /// Supprime le blob EIP (fichier local).
    async fn remove_eip_blob(&self) -> Result<(), std::io::Error> {
        let path = self.data_dir.join("eip.blob");
        if fs::metadata(&path).await.is_ok() {
            fs::remove_file(&path).await?;
        }
        Ok(())
    }

    /// Supprime le registre admin.
    async fn remove_admin_registry(&self) -> Result<(), std::io::Error> {
        let path = self.data_dir.join("admin_registry.json");
        if fs::metadata(&path).await.is_ok() {
            fs::remove_file(&path).await?;
        }
        Ok(())
    }

    /// Réinitialise le schéma bootstrap (suppression ou fichier vide pour état Vierge).
    async fn recreate_bootstrap_schema_empty(&self) -> Result<(), std::io::Error> {
        let path = self.data_dir.join("bootstrap_schema.json");
        if fs::metadata(&path).await.is_ok() {
            fs::remove_file(&path).await?;
        }
        Ok(())
    }

    /// Exécute la destruction et la réinitialisation (Implementation §10.1).
    /// @id: miyukiniadmin_destruction_reinit_run
    /// @role: mutator
    /// @layer: operator
    /// @human: Sauvegarde si besoin, wipe artefacts, écrit mémoire corruption, réinit.
    /// @do: run_destruction_and_reinit
    pub async fn run(&self, reason: &str) -> Result<(), std::io::Error> {
        self.pre_destruction_backup.run_if_needed();

        self.remove_eip_blob().await?;
        self.remove_admin_registry().await?;
        self.recreate_bootstrap_schema_empty().await?;

        let memory = CorruptionMemory {
            reinitialised_at: Utc::now(),
            reason: reason.to_string(),
            previous_state: EnvironmentState::Compromis.to_string(),
        };
        self.write_corruption_memory(&memory).await?;

        Ok(())
    }
}
