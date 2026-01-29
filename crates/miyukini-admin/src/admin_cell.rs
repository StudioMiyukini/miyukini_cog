//! Types pour la Cellule Admin et le manifeste de test embarqué (Module Testing and Lifecycle Contract).
//!
//! La Cellule Admin est la surface qu'un module expose uniquement à MiyukiniAdmin :
//! identification, manifeste de test, métadonnées d'intégrité.

use serde::{Deserialize, Serialize};

/// Type de module (Kit d'outils, Opérateur, Équipe d'opérateurs, Service).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModuleType {
    /// Kit d'outils (Strate 6).
    Toolkit,
    /// Opérateur (Strate 7).
    Operator,
    /// Équipe d'opérateurs.
    Team,
    /// Service (capacité perçue par l'utilisateur).
    Service,
}

/// Identification du module dans la cellule Admin.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleIdentification {
    /// Identifiant unique du module.
    pub id: String,
    /// Version du module.
    pub version: String,
    /// Type de module.
    pub module_type: ModuleType,
    /// Module d'origine (ex. miyukini-spm-cms-content).
    pub module_origin: String,
}

/// Critères de succès/échec pour un test embarqué.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TestCriteria {
    /// Condition de passage (ex. zero_violations, all_checks_ok).
    pub pass: String,
}

/// Définition d'un test dans le manifeste embarqué.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddedTestDef {
    /// Identifiant du test (ex. T-INT-001).
    pub id: String,
    /// Nom lisible du test.
    pub name: String,
    /// Protocole d'exécution (invoke, callback, script).
    pub protocol: String,
    /// Critères de succès/échec.
    pub criteria: TestCriteria,
}

/// Manifeste de test embarqué : liste des tests, format des résultats.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestManifest {
    /// Liste des tests déclarés par le module.
    pub tests: Vec<EmbeddedTestDef>,
    /// Format des résultats (json, yaml).
    #[serde(default = "default_result_format")]
    pub result_format: String,
}

fn default_result_format() -> String {
    "json".to_string()
}

/// Verdict d'un test embarqué (Module Testing and Lifecycle Contract §6.3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TestVerdict {
    /// Tous les critères respectés.
    Pass,
    /// Critères respectés avec alertes.
    Warn,
    /// Un ou plusieurs critères non respectés.
    Fail,
    /// Erreur technique pendant l'exécution.
    Error,
}

/// Métadonnées d'intégrité pour vérification avec les cores (TAMR).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrityMetadata {
    /// Empreinte du module (hash).
    pub fingerprint: String,
    /// Contrats référencés (ex. KindMother-Adapter, MasterButler-Declaration).
    pub contracts: Vec<String>,
    /// Versions des cores attendues.
    #[serde(default)]
    pub core_versions: std::collections::HashMap<String, String>,
}

/// Cellule Admin : surface exposée uniquement à MiyukiniAdmin (identification, manifeste, intégrité).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminCell {
    /// Identification du module.
    pub identification: ModuleIdentification,
    /// Manifeste de test embarqué.
    pub test_manifest: TestManifest,
    /// Métadonnées d'intégrité pour vérification avec TAMR.
    pub integrity: IntegrityMetadata,
}

/// Référence vers la cellule Admin d'un module (retournée par la découverte).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminCellRef {
    /// Identifiant du module.
    pub module_id: String,
    /// Chemin ou endpoint pour lire la cellule Admin (convention selon environnement).
    pub ref_path: String,
}

/// Résumé d'un module retourné par la découverte (Master Butler via BondingBrother).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleInfo {
    /// Identifiant du module.
    pub id: String,
    /// Version.
    pub version: String,
    /// Type de module.
    pub module_type: ModuleType,
    /// Origine du module.
    pub module_origin: String,
    /// Référence vers la cellule Admin (pour lecture par MiyukiniAdmin).
    pub admin_cell_ref: AdminCellRef,
    /// Statut lifecycle : actif, verrouillé.
    #[serde(default)]
    pub locked: bool,
}

/// Résultat d'un test embarqué individuel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddedTestResult {
    /// Identifiant du test.
    pub test_id: String,
    /// Nom du test.
    pub name: String,
    /// Verdict (PASS, WARN, FAIL, ERROR).
    pub verdict: TestVerdict,
    /// Détails optionnels (message, métriques).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
    /// Durée en millisecondes (optionnel).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<u64>,
}

/// Résultats de l'exécution des tests embarqués d'un module.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddedTestResults {
    /// Identifiant du module.
    pub module_id: String,
    /// Horodatage de l'exécution (RFC3339).
    pub timestamp: String,
    /// Résultats par test.
    pub results: Vec<EmbeddedTestResult>,
    /// Verdict global (dérivé : FAIL si au moins un FAIL ou ERROR, sinon WARN si au moins un WARN, sinon PASS).
    pub overall_verdict: TestVerdict,
    /// Nombre de tests passés.
    pub passed: u32,
    /// Nombre en avertissement.
    pub warnings: u32,
    /// Nombre échoués.
    pub failed: u32,
    /// Nombre en erreur technique.
    pub errors: u32,
}

/// Résultat de la vérification d'intégrité (collaboration TAMR).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrityVerificationResult {
    /// Identifiant du module.
    pub module_id: String,
    /// Conforme (true) ou non conforme / intervention requise (false).
    pub conformant: bool,
    /// Message ou détail optionnel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Horodatage de la vérification (RFC3339).
    pub timestamp: String,
}

/// Action de cycle de vie sur un module (add, lock, unlock, delete).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModuleLifecycleAction {
    /// Ajout d'un module (enregistrement Master Butler, validation StrongFather/Ever Buddy).
    Add,
    /// Verrouillage (blocage d'usage sans suppression).
    Lock,
    /// Déverrouillage.
    Unlock,
    /// Suppression (retrait du registre, nettoyage contrôlé).
    Delete,
}

/// Résultat d'une action de cycle de vie.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleActionResult {
    /// Action effectuée.
    pub action: ModuleLifecycleAction,
    /// Identifiant du module concerné.
    pub module_id: String,
    /// Succès ou échec.
    pub success: bool,
    /// Message optionnel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
