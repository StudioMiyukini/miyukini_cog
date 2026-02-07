//! Cellule Admin MiyuSQL (Module Testing and Lifecycle Contract).
//!
//! Identification toolkit.data.miyusql, manifeste de test (MiyukiniSQLtest), intégrité.
//! Exposée uniquement à MiyukiniAdmin.

use serde::{Deserialize, Serialize};

/// @id: miyusql_admin_cell
/// @role: data
/// @layer: toolkit
/// @human: Cellule Admin MiyuSQL : identification, manifeste (MiyukiniSQLtest), intégrité.
/// @do: expose_admin_cell
/// Format déclaratif lisible par MiyukiniAdmin.

/// @id: miyusql_admin_identification
/// @role: data
/// @layer: toolkit
/// @human: Identification du module (toolkit.data.miyusql).
/// @do: store_identification
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MiyuSQLIdentification {
    /// Identifiant unique du toolkit (toolkit.data.miyusql).
    pub id: String,
    /// Version du module.
    pub version: String,
    /// Type : toolkit.
    pub module_type: String,
    /// Module d'origine (miyukini-miyusql).
    pub module_origin: String,
}

/// @id: miyusql_admin_test_criteria
/// @role: data
/// @layer: toolkit
/// @human: Critères de succès/échec pour un test embarqué.
/// @do: store_test_criteria
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TestCriteria {
    /// Condition de passage (ex. all_steps_ok).
    pub pass: String,
}

/// @id: miyusql_admin_embedded_test_def
/// @role: data
/// @layer: toolkit
/// @human: Définition d'un test dans le manifeste (MiyukiniSQLtest).
/// @do: store_embedded_test_def
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddedTestDef {
    /// Identifiant du test (MiyukiniSQLtest).
    pub id: String,
    /// Nom lisible.
    pub name: String,
    /// Protocole d'exécution (invoke).
    pub protocol: String,
    /// Critères de succès/échec.
    pub criteria: TestCriteria,
}

/// @id: miyusql_admin_test_manifest
/// @role: data
/// @layer: toolkit
/// @human: Manifeste de test embarqué (MiyukiniSQLtest E2E).
/// @do: store_test_manifest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiyuSQLTestManifest {
    /// Liste des tests déclarés (MiyukiniSQLtest).
    pub tests: Vec<EmbeddedTestDef>,
    /// Format des résultats (json).
    #[serde(default = "default_result_format")]
    pub result_format: String,
}

fn default_result_format() -> String {
    "json".to_string()
}

/// @id: miyusql_admin_integrity
/// @role: data
/// @layer: toolkit
/// @human: Métadonnées d'intégrité pour vérification TAMR.
/// @do: store_integrity_metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiyuSQLIntegrity {
    /// Empreinte du module (hash).
    pub fingerprint: String,
    /// Contrats référencés (KindMother-Integration, Runtime-Boundary, etc.).
    pub contracts: Vec<String>,
    /// Versions des cores attendues.
    #[serde(default)]
    pub core_versions: std::collections::HashMap<String, String>,
}

/// @id: miyusql_admin_cell_struct
/// @role: data
/// @layer: toolkit
/// @human: Cellule Admin complète MiyuSQL.
/// @do: represent_admin_cell
/// @depends: miyusql_admin_cell
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiyuSQLAdminCell {
    /// Identification du module (toolkit.data.miyusql).
    pub identification: MiyuSQLIdentification,
    /// Manifeste de test (MiyukiniSQLtest).
    pub test_manifest: MiyuSQLTestManifest,
    /// Métadonnées d'intégrité.
    pub integrity: MiyuSQLIntegrity,
}

/// ToolkitId canonique (Documentation Fondatrice).
pub const TOOLKIT_ID: &str = "toolkit.data.miyusql";

/// Nom de la table dédiée au test cycle MiyukiniSQLtest (Cycle Tests Contract).
pub const MIYUKINI_SQLTEST_TABLE: &str = "MiyukiniSQLtest";

/// @id: miyusql_admin_cell_build
/// @role: mutator
/// @layer: toolkit
/// @human: Construit la Cellule Admin MiyuSQL (identification, manifeste MiyukiniSQLtest, intégrité).
/// @do: build_miyusql_admin_cell
/// @depends: miyusql_admin_cell_struct
#[must_use] 
pub fn miyusql_admin_cell(version: &str, fingerprint: &str) -> MiyuSQLAdminCell {
    MiyuSQLAdminCell {
        identification: MiyuSQLIdentification {
            id: TOOLKIT_ID.to_string(),
            version: version.to_string(),
            module_type: "toolkit".to_string(),
            module_origin: "miyukini-miyusql".to_string(),
        },
        test_manifest: MiyuSQLTestManifest {
            tests: vec![EmbeddedTestDef {
                id: "MiyukiniSQLtest".to_string(),
                name: "Test chemin complet MiyuSQL (WriteIntent → table → colonne → insert → read → delete)".to_string(),
                protocol: "invoke".to_string(),
                criteria: TestCriteria {
                    pass: "all_steps_ok".to_string(),
                },
            }],
            result_format: "json".to_string(),
        },
        integrity: MiyuSQLIntegrity {
            fingerprint: fingerprint.to_string(),
            contracts: vec![
                "MiyuSQL-KindMother-Integration".to_string(),
                "MiyuSQL-Runtime-Boundary".to_string(),
                "MiyuSQL-Dependencies".to_string(),
                "MiyuSQL-Cycle-Tests".to_string(),
            ],
            core_versions: std::collections::HashMap::new(),
        },
    }
}
