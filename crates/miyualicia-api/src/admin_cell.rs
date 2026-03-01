//! Cellule Admin Alicia API (Module Testing and Lifecycle Contract).
//!
//! Identification service.alicia.rest-api, manifeste de test, integrite.

use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

// @id: service.alicia.api.admin
// @role: governance_cell
// @layer: 7
// @human: Cellule d'administration du crate miyualicia-api.
// @do: identify_and_self_describe_miyualicia_api

/// Identifiant canonique du service.
pub const SERVICE_ID: &str = "service.alicia.rest-api";

/// Singleton AdminCell.
static ADMIN_CELL: OnceLock<AliciaApiAdminCell> = OnceLock::new();

/// Identification du module.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AliciaApiIdentification {
    /// Identifiant unique du service.
    pub id: String,
    /// Version du module.
    pub version: String,
    /// Type : service.
    pub module_type: String,
    /// Module d'origine.
    pub module_origin: String,
}

/// Criteres de succes/echec.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TestCriteria {
    /// Condition de passage.
    pub pass: String,
}

/// Definition d'un test.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddedTestDef {
    /// Identifiant du test.
    pub id: String,
    /// Nom lisible.
    pub name: String,
    /// Protocole d'execution.
    pub protocol: String,
    /// Criteres.
    pub criteria: TestCriteria,
}

/// Manifeste de test.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AliciaApiTestManifest {
    /// Tests declares.
    pub tests: Vec<EmbeddedTestDef>,
    /// Format des resultats.
    #[serde(default = "default_result_format")]
    pub result_format: String,
}

fn default_result_format() -> String {
    "json".to_string()
}

/// Metadonnees d'integrite.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AliciaApiIntegrity {
    /// Empreinte du module.
    pub fingerprint: String,
    /// Contrats references.
    pub contracts: Vec<String>,
}

/// Cellule Admin complete.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AliciaApiAdminCell {
    /// Identification.
    pub identification: AliciaApiIdentification,
    /// Manifeste de test.
    pub test_manifest: AliciaApiTestManifest,
    /// Integrite.
    pub integrity: AliciaApiIntegrity,
}

/// Construit la Cellule Admin.
#[must_use]
pub fn alicia_api_admin_cell(version: &str, fingerprint: &str) -> AliciaApiAdminCell {
    AliciaApiAdminCell {
        identification: AliciaApiIdentification {
            id: SERVICE_ID.to_string(),
            version: version.to_string(),
            module_type: "service".to_string(),
            module_origin: "miyualicia-api".to_string(),
        },
        test_manifest: AliciaApiTestManifest {
            tests: vec![
                EmbeddedTestDef {
                    id: "AliciaApiJwtTest".to_string(),
                    name: "Test JWT generate/verify/expiration".to_string(),
                    protocol: "invoke".to_string(),
                    criteria: TestCriteria {
                        pass: "all_steps_ok".to_string(),
                    },
                },
                EmbeddedTestDef {
                    id: "AliciaApiHealthTest".to_string(),
                    name: "Test endpoint health sans auth".to_string(),
                    protocol: "invoke".to_string(),
                    criteria: TestCriteria {
                        pass: "all_steps_ok".to_string(),
                    },
                },
            ],
            result_format: "json".to_string(),
        },
        integrity: AliciaApiIntegrity {
            fingerprint: fingerprint.to_string(),
            contracts: vec!["Alicia-API-Governance-Compliance".to_string()],
        },
    }
}

/// Initialise le singleton AdminCell.
pub fn init_admin_cell(version: &str, fingerprint: &str) -> &'static AliciaApiAdminCell {
    ADMIN_CELL.get_or_init(|| alicia_api_admin_cell(version, fingerprint))
}

/// Retourne le singleton si initialise.
pub fn get_admin_cell() -> Option<&'static AliciaApiAdminCell> {
    ADMIN_CELL.get()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_admin_cell_build() {
        let cell = alicia_api_admin_cell("0.1.0", "xyz789");
        assert_eq!(cell.identification.id, SERVICE_ID);
        assert_eq!(cell.identification.version, "0.1.0");
        assert_eq!(cell.identification.module_type, "service");
        assert_eq!(cell.integrity.fingerprint, "xyz789");
        assert_eq!(cell.test_manifest.tests.len(), 2);
    }

    #[test]
    fn test_service_id_constant() {
        assert_eq!(SERVICE_ID, "service.alicia.rest-api");
    }
}
