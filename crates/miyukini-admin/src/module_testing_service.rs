//! Service de tests des modules (Module Testing and Lifecycle Contract).
//!
//! Seul MiyukiniAdmin peut exécuter les tests embarqués et interpréter le manifeste.
//! Découverte des modules via BondingBrother -> Master Butler ; lecture de la cellule Admin ;
//! exécution des tests ; vérification d'intégrité en collaboration avec TAMR.

use crate::admin_cell::{
    AdminCell, EmbeddedTestDef, EmbeddedTestResult, EmbeddedTestResults, IntegrityMetadata,
    IntegrityVerificationResult, ModuleInfo, TestVerdict,
};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Erreur du service de tests des modules.
#[derive(Debug, Clone)]
pub enum ModuleTestingError {
    /// Module non trouvé.
    ModuleNotFound(String),
    /// Cellule Admin non disponible (lecture impossible).
    AdminCellUnavailable(String),
    /// Erreur lors de l'exécution des tests.
    TestExecutionFailed(String),
    /// Erreur lors de la vérification d'intégrité.
    IntegrityCheckFailed(String),
}

impl std::fmt::Display for ModuleTestingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModuleTestingError::ModuleNotFound(id) => write!(f, "Module not found: {id}"),
            ModuleTestingError::AdminCellUnavailable(msg) => {
                write!(f, "Admin cell unavailable: {msg}")
            }
            ModuleTestingError::TestExecutionFailed(msg) => {
                write!(f, "Test execution failed: {msg}")
            }
            ModuleTestingError::IntegrityCheckFailed(msg) => {
                write!(f, "Integrity check failed: {msg}")
            }
        }
    }
}

impl std::error::Error for ModuleTestingError {}

/// Trait de découverte des modules (via BondingBrother -> Master Butler).
/// L'implémentation peut être un stub ou un vrai appel au bridge.
pub trait ModuleDiscovery: Send + Sync {
    /// Liste les modules présents (Kits d'outils, Opérateurs, Équipes, Services).
    fn discover_modules(&self) -> Vec<ModuleInfo>;
}

/// Trait de lecture de la cellule Admin d'un module.
pub trait AdminCellReader: Send + Sync {
    /// Lit la cellule Admin du module (identification, manifeste, intégrité).
    fn read_admin_cell(&self, module_id: &str) -> Option<AdminCell>;
}

/// Trait de vérification d'intégrité (collaboration TAMR).
pub trait IntegrityVerifier: Send + Sync {
    /// Vérifie l'intégrité du module (contexte TAMR).
    fn verify_integrity(&self, module_id: &str, integrity: &IntegrityMetadata) -> IntegrityVerificationResult;
}

/// Service de tests des modules : découverte, lecture cellule Admin, exécution tests, intégrité.
pub struct ModuleTestingService<D, R, V>
where
    D: ModuleDiscovery,
    R: AdminCellReader,
    V: IntegrityVerifier,
{
    discovery: Arc<D>,
    reader: Arc<R>,
    integrity_verifier: Arc<V>,
}

impl<D, R, V> ModuleTestingService<D, R, V>
where
    D: ModuleDiscovery,
    R: AdminCellReader,
    V: IntegrityVerifier,
{
    /// Crée un nouveau service de tests des modules.
    pub fn new(discovery: Arc<D>, reader: Arc<R>, integrity_verifier: Arc<V>) -> Self {
        Self {
            discovery,
            reader,
            integrity_verifier,
        }
    }

    /// Liste les modules présents (via Master Butler via BondingBrother).
    #[must_use] 
    pub fn discover_modules(&self) -> Vec<ModuleInfo> {
        self.discovery.discover_modules()
    }

    /// Lit la cellule Admin du module (identification, manifeste, métadonnées d'intégrité).
    pub fn read_admin_cell(&self, module_id: &str) -> Result<AdminCell, ModuleTestingError> {
        self.reader
            .read_admin_cell(module_id)
            .ok_or_else(|| ModuleTestingError::AdminCellUnavailable(module_id.to_string()))
    }

    /// Exécute les tests embarqués du module (interprétation du manifeste par MiyukiniAdmin).
    /// Environnement de diagnostic ; pas de modification des données métier.
    pub fn run_embedded_tests(&self, module_id: &str) -> Result<EmbeddedTestResults, ModuleTestingError> {
        let cell = self.read_admin_cell(module_id)?;
        let now = chrono::Utc::now();
        let timestamp = now.to_rfc3339();
        let mut results = Vec::new();
        let mut passed = 0u32;
        let mut warnings = 0u32;
        let mut failed = 0u32;
        let mut errors = 0u32;

        for test in &cell.test_manifest.tests {
            let (verdict, duration_ms) = execute_single_embedded_test(test);
            match verdict {
                TestVerdict::Pass => passed += 1,
                TestVerdict::Warn => warnings += 1,
                TestVerdict::Fail => failed += 1,
                TestVerdict::Error => errors += 1,
            }
            results.push(EmbeddedTestResult {
                test_id: test.id.clone(),
                name: test.name.clone(),
                verdict,
                details: None,
                duration_ms: Some(duration_ms),
            });
        }

        let overall_verdict = if failed > 0 || errors > 0 {
            TestVerdict::Fail
        } else if warnings > 0 {
            TestVerdict::Warn
        } else {
            TestVerdict::Pass
        };

        Ok(EmbeddedTestResults {
            module_id: module_id.to_string(),
            timestamp,
            results,
            overall_verdict,
            passed,
            warnings,
            failed,
            errors,
        })
    }

    /// Vérifie l'intégrité du module en collaboration avec TAMR (via BondingBrother).
    pub fn verify_integrity(&self, module_id: &str) -> Result<IntegrityVerificationResult, ModuleTestingError> {
        let cell = self.read_admin_cell(module_id)?;
        Ok(self
            .integrity_verifier
            .verify_integrity(module_id, &cell.integrity))
    }
}

/// Exécution d'un seul test embarqué (stub : simule succès selon critères ; en production, invoque le protocole du module).
fn execute_single_embedded_test(test: &EmbeddedTestDef) -> (TestVerdict, u64) {
    let start = std::time::Instant::now();
    // Stub : selon le critère "pass", on simule un verdict (en production : appel au module selon protocol).
    let verdict = if test.criteria.pass == "zero_violations" || test.criteria.pass == "all_checks_ok" {
        TestVerdict::Pass
    } else {
        TestVerdict::Pass
    };
    let duration_ms = start.elapsed().as_millis() as u64;
    (verdict, duration_ms)
}

// --- Implémentations stub pour découverte, lecture et intégrité ---

/// Découverte stub : retourne une liste vide ou des modules de démo (jusqu'à intégration Master Butler).
#[derive(Debug, Default)]
pub struct StubModuleDiscovery {
    /// Modules enregistrés (pour démo : chargés depuis un registre en mémoire).
    modules: RwLock<Vec<ModuleInfo>>,
}

impl StubModuleDiscovery {
    /// Crée un discovery stub avec une liste vide.
    #[must_use] 
    pub fn new() -> Self {
        Self {
            modules: RwLock::new(Vec::new()),
        }
    }

    /// Enregistre un module pour la découverte (démo / tests).
    pub fn register_module(&self, info: ModuleInfo) {
        let mut m = self.modules.write().expect("RwLock poisoned");
        if let Some(pos) = m.iter().position(|x| x.id == info.id) {
            m[pos] = info;
        } else {
            m.push(info);
        }
    }

    /// Retourne les modules enregistrés.
    pub fn list(&self) -> Vec<ModuleInfo> {
        self.modules.read().expect("RwLock poisoned").clone()
    }
}

impl ModuleDiscovery for StubModuleDiscovery {
    fn discover_modules(&self) -> Vec<ModuleInfo> {
        self.list()
    }
}

/// Reader stub : lit la cellule Admin depuis un registre en mémoire (jusqu'à résolution par ref_path).
#[derive(Debug, Default)]
pub struct StubAdminCellReader {
    cells: RwLock<HashMap<String, AdminCell>>,
}

impl StubAdminCellReader {
    /// Crée un reader stub vide.
    #[must_use] 
    pub fn new() -> Self {
        Self {
            cells: RwLock::new(HashMap::new()),
        }
    }

    /// Enregistre la cellule Admin d'un module (démo / tests).
    pub fn register_cell(&self, module_id: String, cell: AdminCell) {
        self.cells.write().expect("RwLock poisoned").insert(module_id, cell);
    }
}

impl AdminCellReader for StubAdminCellReader {
    fn read_admin_cell(&self, module_id: &str) -> Option<AdminCell> {
        self.cells.read().expect("RwLock poisoned").get(module_id).cloned()
    }
}

/// Vérificateur d'intégrité stub (collaboration TAMR : en production, appel via BondingBrother).
#[derive(Debug, Default)]
pub struct StubIntegrityVerifier;

impl IntegrityVerifier for StubIntegrityVerifier {
    fn verify_integrity(&self, module_id: &str, _integrity: &IntegrityMetadata) -> IntegrityVerificationResult {
        IntegrityVerificationResult {
            module_id: module_id.to_string(),
            conformant: true,
            message: Some("Stub: intégrité considérée conforme (TAMR non branché)".to_string()),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}
