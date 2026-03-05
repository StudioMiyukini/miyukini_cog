//! Cellule Admin Alicia HTTP (Module Testing and Lifecycle Contract).
//!
//! Identification toolkit.alicia.http, manifeste de test, integrite.
//! Exposee uniquement a MiyukiniAdmin.

use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

// @id: toolkit.alicia.http.admin
// @role: governance_cell
// @layer: 6
// @human: Cellule d'administration du crate miyualicia-http.
// @do: identify_and_self_describe_miyualicia_http

/// Identifiant canonique du toolkit.
pub const TOOLKIT_ID: &str = "toolkit.alicia.http";

/// Singleton AdminCell (pattern OnceLock Miyukini).
static ADMIN_CELL: OnceLock<AliciaHttpAdminCell> = OnceLock::new();

/// Identification du module.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AliciaHttpIdentification {
    /// Identifiant unique du toolkit.
    pub id: String,
    /// Version du module.
    pub version: String,
    /// Type : toolkit.
    pub module_type: String,
    /// Module d'origine.
    pub module_origin: String,
}

/// Criteres de succes/echec pour un test embarque.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TestCriteria {
    /// Condition de passage (ex. all_steps_ok).
    pub pass: String,
}

/// Definition d'un test dans le manifeste.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddedTestDef {
    /// Identifiant du test.
    pub id: String,
    /// Nom lisible.
    pub name: String,
    /// Protocole d'execution (invoke).
    pub protocol: String,
    /// Criteres de succes/echec.
    pub criteria: TestCriteria,
}

/// Manifeste de test embarque.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AliciaHttpTestManifest {
    /// Liste des tests declares.
    pub tests: Vec<EmbeddedTestDef>,
    /// Format des resultats (json).
    #[serde(default = "default_result_format")]
    pub result_format: String,
}

fn default_result_format() -> String {
    "json".to_string()
}

/// Metadonnees d'integrite pour verification TAMR.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AliciaHttpIntegrity {
    /// Empreinte du module (hash).
    pub fingerprint: String,
    /// Contrats references.
    pub contracts: Vec<String>,
    /// Versions des cores attendues.
    #[serde(default)]
    pub core_versions: std::collections::HashMap<String, String>,
}

/// Cellule Admin complete AliciaHttp.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AliciaHttpAdminCell {
    /// Identification du module.
    pub identification: AliciaHttpIdentification,
    /// Manifeste de test.
    pub test_manifest: AliciaHttpTestManifest,
    /// Metadonnees d'integrite.
    pub integrity: AliciaHttpIntegrity,
}

/// Construit la Cellule Admin AliciaHttp.
#[must_use]
pub fn alicia_http_admin_cell(version: &str, fingerprint: &str) -> AliciaHttpAdminCell {
    AliciaHttpAdminCell {
        identification: AliciaHttpIdentification {
            id: TOOLKIT_ID.to_string(),
            version: version.to_string(),
            module_type: "toolkit".to_string(),
            module_origin: "miyualicia-http".to_string(),
        },
        test_manifest: AliciaHttpTestManifest {
            tests: vec![EmbeddedTestDef {
                id: "AliciaHttpAdaptersTest".to_string(),
                name: "Test adaptateurs HTTP Shelly et generique JSON".to_string(),
                protocol: "invoke".to_string(),
                criteria: TestCriteria {
                    pass: "all_steps_ok".to_string(),
                },
            }],
            result_format: "json".to_string(),
        },
        integrity: AliciaHttpIntegrity {
            fingerprint: fingerprint.to_string(),
            contracts: vec!["AliciaHttp-Tool-Governance-Compliance".to_string()],
            core_versions: std::collections::HashMap::new(),
        },
    }
}

/// Initialise et retourne le singleton AdminCell.
pub fn init_admin_cell(version: &str, fingerprint: &str) -> &'static AliciaHttpAdminCell {
    ADMIN_CELL.get_or_init(|| alicia_http_admin_cell(version, fingerprint))
}

/// Retourne le singleton AdminCell s'il a ete initialise.
pub fn get_admin_cell() -> Option<&'static AliciaHttpAdminCell> {
    ADMIN_CELL.get()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_admin_cell_build() {
        let cell = alicia_http_admin_cell("0.1.0", "ghi789");
        assert_eq!(cell.identification.id, TOOLKIT_ID);
        assert_eq!(cell.identification.version, "0.1.0");
        assert_eq!(cell.identification.module_type, "toolkit");
        assert_eq!(cell.integrity.fingerprint, "ghi789");
        assert_eq!(cell.test_manifest.tests.len(), 1);
    }

    #[test]
    fn test_toolkit_id_constant() {
        assert_eq!(TOOLKIT_ID, "toolkit.alicia.http");
    }
}
