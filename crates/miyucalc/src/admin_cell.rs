//! Cellule Admin MiyuCalc (Module Testing and Lifecycle Contract).
//!
//! Identification toolkit.calc.miyucalc, manifeste de test, intégrité.
//! Exposée uniquement à MiyukiniAdmin.

use serde::{Deserialize, Serialize};

/// @id miyucalc_admin_cell
/// @role data
/// @layer toolkit
/// @do expose_admin_cell
//@human / @human Cellule Admin MiyuCalc : identification, manifeste de test, intégrité.

/// @id miyucalc_admin_identification
/// @role data
/// @layer toolkit
/// @do store_identification
//@human / @human Identification du module (toolkit.calc.miyucalc).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MiyuCalcIdentification {
    /// Identifiant unique du toolkit (toolkit.calc.miyucalc).
    pub id: String,
    /// Version du module.
    pub version: String,
    /// Type : toolkit.
    pub module_type: String,
    /// Module d'origine (miyukini-miyucalc).
    pub module_origin: String,
}

/// @id miyucalc_admin_test_criteria
/// @role data
/// @layer toolkit
/// @do store_test_criteria
//@human / @human Critères de succès/échec pour un test embarqué.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TestCriteria {
    /// Condition de passage (ex. all_steps_ok).
    pub pass: String,
}

/// @id miyucalc_admin_embedded_test_def
/// @role data
/// @layer toolkit
/// @do store_embedded_test_def
//@human / @human Définition d'un test dans le manifeste.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddedTestDef {
    /// Identifiant du test.
    pub id: String,
    /// Nom lisible.
    pub name: String,
    /// Protocole d'exécution (invoke).
    pub protocol: String,
    /// Critères de succès/échec.
    pub criteria: TestCriteria,
}

/// @id miyucalc_admin_test_manifest
/// @role data
/// @layer toolkit
/// @do store_test_manifest
//@human / @human Manifeste de test embarqué.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiyuCalcTestManifest {
    /// Liste des tests déclarés.
    pub tests: Vec<EmbeddedTestDef>,
    /// Format des résultats (json).
    #[serde(default = "default_result_format")]
    pub result_format: String,
}

fn default_result_format() -> String {
    "json".to_string()
}

/// @id miyucalc_admin_integrity
/// @role data
/// @layer toolkit
/// @do store_integrity_metadata
//@human / @human Métadonnées d'intégrité pour vérification TAMR.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiyuCalcIntegrity {
    /// Empreinte du module (hash).
    pub fingerprint: String,
    /// Contrats référencés.
    pub contracts: Vec<String>,
    /// Versions des cores attendues.
    #[serde(default)]
    pub core_versions: std::collections::HashMap<String, String>,
}

/// @id miyucalc_admin_cell_struct
/// @role data
/// @layer toolkit
/// @do represent_admin_cell
//@human / @human Cellule Admin complète MiyuCalc.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiyuCalcAdminCell {
    /// Identification du module (toolkit.calc.miyucalc).
    pub identification: MiyuCalcIdentification,
    /// Manifeste de test.
    pub test_manifest: MiyuCalcTestManifest,
    /// Métadonnées d'intégrité.
    pub integrity: MiyuCalcIntegrity,
}

/// ToolkitId canonique (Documentation Fondatrice).
pub const TOOLKIT_ID: &str = "toolkit.calc.miyucalc";

/// @id miyucalc_admin_cell_build
/// @role mutator
/// @layer toolkit
/// @do build_miyucalc_admin_cell
//@human / @human Construit la Cellule Admin MiyuCalc.
#[must_use]
pub fn miyucalc_admin_cell(version: &str, fingerprint: &str) -> MiyuCalcAdminCell {
    MiyuCalcAdminCell {
        identification: MiyuCalcIdentification {
            id: TOOLKIT_ID.to_string(),
            version: version.to_string(),
            module_type: "toolkit".to_string(),
            module_origin: "miyukini-miyucalc".to_string(),
        },
        test_manifest: MiyuCalcTestManifest {
            tests: vec![EmbeddedTestDef {
                id: "MiyukiniCalctest".to_string(),
                name: "Test chemin complet MiyuCalc (expression, format, unit, round)".to_string(),
                protocol: "invoke".to_string(),
                criteria: TestCriteria {
                    pass: "all_steps_ok".to_string(),
                },
            }],
            result_format: "json".to_string(),
        },
        integrity: MiyuCalcIntegrity {
            fingerprint: fingerprint.to_string(),
            contracts: vec!["MiyuCalc-Tool-Governance-Compliance".to_string()],
            core_versions: std::collections::HashMap::new(),
        },
    }
}
