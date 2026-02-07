//! Service de cycle de vie des modules (Module Testing and Lifecycle Contract).
//!
//! Actions exclusives à MiyukiniAdmin : ajout, verrouillage/déverrouillage, suppression d'un module.
//! Toute action passe par BondingBrother ; validation StrongFather ; enregistrement Master Butler / Ever Buddy selon le cas.

use crate::admin_cell::{
    AdminCellRef, LifecycleActionResult, ModuleInfo, ModuleLifecycleAction, ModuleType,
};
use std::collections::HashMap;
use std::sync::RwLock;

/// Erreur du service de cycle de vie des modules.
#[derive(Debug, Clone)]
pub enum ModuleLifecycleError {
    /// Module non trouvé.
    ModuleNotFound(String),
    /// Module déjà verrouillé (pour lock) ou déjà déverrouillé (pour unlock).
    InvalidState(String),
    /// Validation StrongFather refusée (stub : non implémenté).
    StrongFatherRejected(String),
}

impl std::fmt::Display for ModuleLifecycleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModuleLifecycleError::ModuleNotFound(id) => write!(f, "Module not found: {id}"),
            ModuleLifecycleError::InvalidState(msg) => write!(f, "Invalid state: {msg}"),
            ModuleLifecycleError::StrongFatherRejected(msg) => {
                write!(f, "StrongFather rejected: {msg}")
            }
        }
    }
}

impl std::error::Error for ModuleLifecycleError {}

/// Paramètres pour l'ajout d'un module (déclaration à Master Butler, validation StrongFather/Ever Buddy).
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct AddModuleParams {
    /// Identifiant du module.
    pub id: String,
    /// Version.
    pub version: String,
    /// Type de module (toolkit, operator, team, service).
    pub module_type: ModuleType,
    /// Origine du module (ex. miyukini-spm-cms-content).
    pub module_origin: String,
    /// Référence vers la cellule Admin (chemin ou endpoint).
    pub admin_cell_ref_path: String,
}

/// Service de cycle de vie des modules : add, lock, unlock, delete.
/// En production : délègue à BondingBrother -> StrongFather / Master Butler / Ever Buddy.
pub struct ModuleLifecycleService {
    /// Registre des modules (stub : en production, Master Butler est la source de vérité).
    modules: RwLock<HashMap<String, ModuleInfo>>,
}

impl ModuleLifecycleService {
    /// Crée un nouveau service de cycle de vie (stub : registre en mémoire).
    #[must_use] 
    pub fn new() -> Self {
        Self {
            modules: RwLock::new(HashMap::new()),
        }
    }

    /// Ajoute un module (validation StrongFather, enregistrement Master Butler, traçabilité).
    /// Stub : enregistre en mémoire ; en production, passe par BondingBrother.
    pub fn add_module(&self, params: AddModuleParams) -> Result<LifecycleActionResult, ModuleLifecycleError> {
        let module_id = params.id.clone();
        let info = ModuleInfo {
            id: params.id.clone(),
            version: params.version,
            module_type: params.module_type,
            module_origin: params.module_origin,
            admin_cell_ref: AdminCellRef {
                module_id: params.id.clone(),
                ref_path: params.admin_cell_ref_path,
            },
            locked: false,
        };
        self.modules
            .write()
            .expect("RwLock poisoned")
            .insert(module_id.clone(), info);
        Ok(LifecycleActionResult {
            action: ModuleLifecycleAction::Add,
            module_id: module_id.clone(),
            success: true,
            message: Some("Module enregistré (stub ; StrongFather/Master Butler en production).".to_string()),
        })
    }

    /// Verrouille un module (blocage d'usage sans suppression ; validation StrongFather).
    pub fn lock_module(&self, module_id: &str) -> Result<LifecycleActionResult, ModuleLifecycleError> {
        let mut modules = self.modules.write().expect("RwLock poisoned");
        let info = modules
            .get_mut(module_id)
            .ok_or_else(|| ModuleLifecycleError::ModuleNotFound(module_id.to_string()))?;
        if info.locked {
            return Err(ModuleLifecycleError::InvalidState(
                "Module déjà verrouillé.".to_string(),
            ));
        }
        info.locked = true;
        Ok(LifecycleActionResult {
            action: ModuleLifecycleAction::Lock,
            module_id: module_id.to_string(),
            success: true,
            message: Some("Module verrouillé (stub ; StrongFather/CaringNanny en production).".to_string()),
        })
    }

    /// Déverrouille un module.
    pub fn unlock_module(&self, module_id: &str) -> Result<LifecycleActionResult, ModuleLifecycleError> {
        let mut modules = self.modules.write().expect("RwLock poisoned");
        let info = modules
            .get_mut(module_id)
            .ok_or_else(|| ModuleLifecycleError::ModuleNotFound(module_id.to_string()))?;
        if !info.locked {
            return Err(ModuleLifecycleError::InvalidState(
                "Module déjà déverrouillé.".to_string(),
            ));
        }
        info.locked = false;
        Ok(LifecycleActionResult {
            action: ModuleLifecycleAction::Unlock,
            module_id: module_id.to_string(),
            success: true,
            message: Some("Module déverrouillé.".to_string()),
        })
    }

    /// Supprime un module (validation StrongFather, retrait du registre Master Butler, nettoyage contrôlé).
    pub fn delete_module(&self, module_id: &str) -> Result<LifecycleActionResult, ModuleLifecycleError> {
        let mut modules = self.modules.write().expect("RwLock poisoned");
        if modules.remove(module_id).is_none() {
            return Err(ModuleLifecycleError::ModuleNotFound(module_id.to_string()));
        }
        Ok(LifecycleActionResult {
            action: ModuleLifecycleAction::Delete,
            module_id: module_id.to_string(),
            success: true,
            message: Some("Module supprimé du registre (stub ; StrongFather/Master Butler en production).".to_string()),
        })
    }

    /// Retourne la liste des modules (pour synchronisation avec le stub de découverte).
    pub fn list_modules(&self) -> Vec<ModuleInfo> {
        self.modules.read().expect("RwLock poisoned").values().cloned().collect()
    }

    /// Retourne les infos d'un module (pour enregistrement dans la découverte après add).
    pub fn get_module(&self, module_id: &str) -> Option<ModuleInfo> {
        self.modules.read().expect("RwLock poisoned").get(module_id).cloned()
    }
}

impl Default for ModuleLifecycleService {
    fn default() -> Self {
        Self::new()
    }
}
