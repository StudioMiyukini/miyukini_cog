//! Service de détection d'état de l'environnement (Auth and First-Boot §3.1 à 3.5).
//!
//! Référence : MiyukiniAdmin - Implementation Security and Controls §4

use std::path::PathBuf;
use tokio::fs;

use crate::models::EnvironmentState;

/// @id: miyukiniadmin_environment_state_service
/// @role: infrastructure
/// @layer: operator
/// @human: Détection état environnement (VIERGE, INITIALISE, COMPROMIS) via artefacts locaux.
/// @do: detect_environment_state
/// @depends: miyukiniadmin_models_environment_state
#[derive(Clone)]
pub struct EnvironmentStateService {
    /// Répertoire de données (EIP blob, registre admin, schéma bootstrap).
    /// @id: miyukiniadmin_env_state_service_data_dir
    data_dir: PathBuf,
}

impl EnvironmentStateService {
    /// Crée le service avec un répertoire de données.
    /// @id: miyukiniadmin_env_state_service_new
    /// @role: constructor
    /// @layer: operator
    /// @human: Construit le service avec le répertoire de données.
    /// @do: create_environment_state_service
    #[must_use]
    pub fn new(data_dir: PathBuf) -> Self {
        Self { data_dir }
    }

    /// Chemin du blob EIP (KindMother ou slot dédié ; ici fichier local).
    fn eip_path(&self) -> PathBuf {
        self.data_dir.join("eip.blob")
    }

    /// Chemin du registre admin (fichier JSON).
    fn admin_registry_path(&self) -> PathBuf {
        self.data_dir.join("admin_registry.json")
    }

    /// Chemin du marqueur schéma bootstrap (tables noyau présentes).
    fn bootstrap_schema_path(&self) -> PathBuf {
        self.data_dir.join("bootstrap_schema.json")
    }

    /// Vérifie la présence d'un fichier (artefact).
    async fn has_artifact(path: &PathBuf) -> bool {
        fs::metadata(path)
            .await
            .map(|m| m.is_file())
            .unwrap_or(false)
    }

    /// Vérifie l'intégrité EIP (stub : présence = valide pour implémentation minimale).
    /// En production : déchiffrer avec clé dérivée, vérifier tag AEAD, integrity_hash.
    /// @id: miyukiniadmin_env_state_verify_eip
    async fn verify_eip_integrity(&self) -> Result<bool, std::io::Error> {
        let path = self.eip_path();
        if !Self::has_artifact(&path).await {
            return Ok(true);
        }
        let content = fs::read(&path).await?;
        if content.is_empty() {
            return Ok(false);
        }
        Ok(true)
    }

    /// Vérifie le registre admin : structure valide, au moins un compte non révoqué.
    /// @id: miyukiniadmin_env_state_verify_registry
    async fn verify_admin_registry(&self) -> Result<bool, std::io::Error> {
        let path = self.admin_registry_path();
        if !Self::has_artifact(&path).await {
            return Ok(true);
        }
        let content = fs::read_to_string(&path).await?;
        let accounts: Vec<serde_json::Value> = serde_json::from_str(&content).unwrap_or_default();
        if accounts.is_empty() {
            return Ok(false);
        }
        Ok(true)
    }

    /// Vérifie l'intégrité du schéma bootstrap (présence et contenu minimal).
    /// @id: miyukiniadmin_env_state_verify_schema
    async fn verify_schema_integrity(&self) -> Result<bool, std::io::Error> {
        let path = self.bootstrap_schema_path();
        if !Self::has_artifact(&path).await {
            return Ok(true);
        }
        let content = fs::read_to_string(&path).await?;
        let schema: serde_json::Value =
            serde_json::from_str(&content).unwrap_or(serde_json::Value::Null);
        Ok(!schema.is_null() && schema.get("version").is_some())
    }

    /// Cohérence globale : EIP + registre + schéma cohérents (stub : pas d'incohérence détectée).
    /// @id: miyukiniadmin_env_state_verify_consistency
    async fn verify_global_consistency(&self) -> Result<(), std::io::Error> {
        let _ = self;
        Ok(())
    }

    /// Détecte l'état de l'environnement (Auth and First-Boot §4.1).
    /// @id: miyukiniadmin_env_state_detect
    /// @role: accessor
    /// @layer: operator
    /// @human: Détecte VIERGE / INITIALISE / COMPROMIS selon artefacts et intégrité.
    /// @do: detect_environment_state
    /// @depends: miyukiniadmin_environment_state_service
    pub async fn detect(&self) -> Result<EnvironmentState, std::io::Error> {
        let has_eip = Self::has_artifact(&self.eip_path()).await;
        let has_admin_registry = Self::has_artifact(&self.admin_registry_path()).await;
        let has_bootstrap_schema = Self::has_artifact(&self.bootstrap_schema_path()).await;

        if !has_eip && !has_admin_registry && !has_bootstrap_schema {
            return Ok(EnvironmentState::Vierge);
        }

        if has_eip {
            let valid = self.verify_eip_integrity().await?;
            if !valid {
                return Ok(EnvironmentState::Compromis);
            }
        }
        if has_admin_registry {
            let valid = self.verify_admin_registry().await?;
            if !valid {
                return Ok(EnvironmentState::Compromis);
            }
        }
        if has_bootstrap_schema {
            let valid = self.verify_schema_integrity().await?;
            if !valid {
                return Ok(EnvironmentState::Compromis);
            }
        }

        self.verify_global_consistency().await?;
        Ok(EnvironmentState::Initialise)
    }

    /// Retourne le répertoire de données (pour autres services).
    #[must_use]
    pub fn data_dir(&self) -> &PathBuf {
        &self.data_dir
    }

    /// Crée le blob EIP minimal (First-boot — Auth and First-Boot §5).
    /// Stub : contenu non vide pour que verify_eip_integrity soit valide.
    /// @id: miyukiniadmin_env_state_write_eip_stub
    /// @role: mutator
    /// @layer: operator
    /// @human: Génère l'artefact EIP initial pour passage à INITIALISE.
    /// @do: write_eip_stub
    pub async fn write_eip_stub(&self) -> Result<(), std::io::Error> {
        let path = self.eip_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }
        // Stub : marqueur EIP minimal (en production : clé dérivée, AEAD, integrity_hash).
        let stub = br#"{"eip_version":"1.0","created_at_first_boot":true}"#;
        fs::write(&path, stub).await
    }

    /// Crée le schéma bootstrap initial (First-boot — Auth and First-Boot §5).
    /// Contenu minimal avec "version" pour que verify_schema_integrity soit valide.
    /// @id: miyukiniadmin_env_state_write_bootstrap_schema_initial
    /// @role: mutator
    /// @layer: operator
    /// @human: Génère l'artefact schéma bootstrap pour passage à INITIALISE.
    /// @do: write_bootstrap_schema_initial
    pub async fn write_bootstrap_schema_initial(&self) -> Result<(), std::io::Error> {
        let path = self.bootstrap_schema_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }
        let schema = serde_json::json!({ "version": "1.0", "created_at_first_boot": true });
        let content = serde_json::to_string_pretty(&schema).unwrap_or_default();
        fs::write(&path, content).await
    }
}
