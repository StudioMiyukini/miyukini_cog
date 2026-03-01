//! Cellule Admin Alicia Orchestrator (Module Testing and Lifecycle Contract).
//!
//! Identification service.alicia.orchestrator, manifeste de test, integrite.
//! Exposee uniquement a MiyukiniAdmin.

use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

// @id: service.alicia.orchestrator.admin
// @role: governance_cell
// @layer: 7
// @human: Cellule d'administration du crate miyualicia (orchestrateur).
// @do: identify_and_self_describe_miyualicia

/// Identifiant canonique du service.
pub const SERVICE_ID: &str = "service.alicia.orchestrator";

/// Singleton AdminCell.
static ADMIN_CELL: OnceLock<AliciaAdminCell> = OnceLock::new();

/// Identification du module.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AliciaIdentification {
    /// Identifiant unique du service.
    pub id: String,
    /// Version du module.
    pub version: String,
    /// Type : service.
    pub module_type: String,
    /// Module d'origine.
    pub module_origin: String,
}

/// Criteres de succes/echec pour un test embarque.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TestCriteria {
    /// Condition de passage.
    pub pass: String,
}

/// Definition d'un test dans le manifeste.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddedTestDef {
    /// Identifiant du test.
    pub id: String,
    /// Nom lisible.
    pub name: String,
    /// Protocole d'execution.
    pub protocol: String,
    /// Criteres de succes/echec.
    pub criteria: TestCriteria,
}

/// Manifeste de test embarque.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AliciaTestManifest {
    /// Liste des tests declares.
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
pub struct AliciaIntegrity {
    /// Empreinte du module.
    pub fingerprint: String,
    /// Contrats references.
    pub contracts: Vec<String>,
}

/// Cellule Admin complete Alicia.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AliciaAdminCell {
    /// Identification du module.
    pub identification: AliciaIdentification,
    /// Manifeste de test.
    pub test_manifest: AliciaTestManifest,
    /// Metadonnees d'integrite.
    pub integrity: AliciaIntegrity,
}

/// Construit la Cellule Admin Alicia.
#[must_use]
pub fn alicia_admin_cell(version: &str, fingerprint: &str) -> AliciaAdminCell {
    AliciaAdminCell {
        identification: AliciaIdentification {
            id: SERVICE_ID.to_string(),
            version: version.to_string(),
            module_type: "service".to_string(),
            module_origin: "miyualicia".to_string(),
        },
        test_manifest: AliciaTestManifest {
            tests: vec![
                EmbeddedTestDef {
                    id: "AliciaOrchestratorTest".to_string(),
                    name: "Test orchestrateur complet (registre, NLU, commandes)".to_string(),
                    protocol: "invoke".to_string(),
                    criteria: TestCriteria {
                        pass: "all_steps_ok".to_string(),
                    },
                },
                EmbeddedTestDef {
                    id: "AliciaNluFallbackTest".to_string(),
                    name: "Test NLU fallback regex (10+ patterns)".to_string(),
                    protocol: "invoke".to_string(),
                    criteria: TestCriteria {
                        pass: "all_steps_ok".to_string(),
                    },
                },
            ],
            result_format: "json".to_string(),
        },
        integrity: AliciaIntegrity {
            fingerprint: fingerprint.to_string(),
            contracts: vec!["Alicia-Orchestrator-Governance-Compliance".to_string()],
        },
    }
}

/// Initialise et retourne le singleton AdminCell.
pub fn init_admin_cell(version: &str, fingerprint: &str) -> &'static AliciaAdminCell {
    ADMIN_CELL.get_or_init(|| alicia_admin_cell(version, fingerprint))
}

/// Retourne le singleton AdminCell s'il a ete initialise.
pub fn get_admin_cell() -> Option<&'static AliciaAdminCell> {
    ADMIN_CELL.get()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_admin_cell_build() {
        let cell = alicia_admin_cell("0.1.0", "abc123");
        assert_eq!(cell.identification.id, SERVICE_ID);
        assert_eq!(cell.identification.version, "0.1.0");
        assert_eq!(cell.identification.module_type, "service");
        assert_eq!(cell.integrity.fingerprint, "abc123");
        assert_eq!(cell.test_manifest.tests.len(), 2);
    }

    #[test]
    fn test_service_id_constant() {
        assert_eq!(SERVICE_ID, "service.alicia.orchestrator");
    }
}
