//! Cellule Admin Alicia Automations (Module Testing and Lifecycle Contract).
//!
//! Identification toolkit.alicia.automations, manifeste de test, integrite.
//! Exposee uniquement a MiyukiniAdmin.

// @id: toolkit.alicia.automations.admin
// @role: governance_cell
// @layer: 6
// @human: Cellule d'administration du crate miyualicia-automations.
// @do: identify_and_self_describe_miyualicia_automations

use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

/// Identifiant canonique du toolkit.
pub const TOOLKIT_ID: &str = "toolkit.alicia.automations";

/// Singleton AdminCell (pattern OnceLock Miyukini).
static ADMIN_CELL: OnceLock<AliciaAutomationsAdminCell> = OnceLock::new();

/// Identification du module.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AliciaAutomationsIdentification {
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
pub struct AliciaAutomationsTestManifest {
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
pub struct AliciaAutomationsIntegrity {
    /// Empreinte du module (hash).
    pub fingerprint: String,
    /// Contrats references.
    pub contracts: Vec<String>,
    /// Versions des cores attendues.
    #[serde(default)]
    pub core_versions: std::collections::HashMap<String, String>,
}

/// Cellule Admin complete AliciaAutomations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AliciaAutomationsAdminCell {
    /// Identification du module.
    pub identification: AliciaAutomationsIdentification,
    /// Manifeste de test.
    pub test_manifest: AliciaAutomationsTestManifest,
    /// Metadonnees d'integrite.
    pub integrity: AliciaAutomationsIntegrity,
}

/// Construit la Cellule Admin AliciaAutomations.
#[must_use]
pub fn alicia_automations_admin_cell(
    version: &str,
    fingerprint: &str,
) -> AliciaAutomationsAdminCell {
    AliciaAutomationsAdminCell {
        identification: AliciaAutomationsIdentification {
            id: TOOLKIT_ID.to_string(),
            version: version.to_string(),
            module_type: "toolkit".to_string(),
            module_origin: "miyualicia-automations".to_string(),
        },
        test_manifest: AliciaAutomationsTestManifest {
            tests: vec![
                EmbeddedTestDef {
                    id: "AliciaAutomationsEvaluatorTest".to_string(),
                    name: "Test evaluateur de conditions (types, operators, snapshot)"
                        .to_string(),
                    protocol: "invoke".to_string(),
                    criteria: TestCriteria {
                        pass: "all_steps_ok".to_string(),
                    },
                },
                EmbeddedTestDef {
                    id: "AliciaAutomationsEngineTest".to_string(),
                    name: "Test moteur automations (CRUD, triggers, execution)".to_string(),
                    protocol: "invoke".to_string(),
                    criteria: TestCriteria {
                        pass: "all_steps_ok".to_string(),
                    },
                },
                EmbeddedTestDef {
                    id: "AliciaAutomationsParserTest".to_string(),
                    name: "Test parseur TOML/JSON automations".to_string(),
                    protocol: "invoke".to_string(),
                    criteria: TestCriteria {
                        pass: "all_steps_ok".to_string(),
                    },
                },
            ],
            result_format: "json".to_string(),
        },
        integrity: AliciaAutomationsIntegrity {
            fingerprint: fingerprint.to_string(),
            contracts: vec![
                "AliciaAutomations-Tool-Governance-Compliance".to_string(),
            ],
            core_versions: std::collections::HashMap::new(),
        },
    }
}

/// Initialise et retourne le singleton AdminCell.
///
/// Appeler une seule fois au demarrage ; les appels suivants retournent
/// la meme reference.
pub fn init_admin_cell(
    version: &str,
    fingerprint: &str,
) -> &'static AliciaAutomationsAdminCell {
    ADMIN_CELL.get_or_init(|| alicia_automations_admin_cell(version, fingerprint))
}

/// Retourne le singleton AdminCell s'il a ete initialise.
pub fn get_admin_cell() -> Option<&'static AliciaAutomationsAdminCell> {
    ADMIN_CELL.get()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_admin_cell_build() {
        let cell = alicia_automations_admin_cell("0.1.0", "abc123");
        assert_eq!(cell.identification.id, TOOLKIT_ID);
        assert_eq!(cell.identification.version, "0.1.0");
        assert_eq!(cell.identification.module_type, "toolkit");
        assert_eq!(cell.integrity.fingerprint, "abc123");
        assert_eq!(cell.test_manifest.tests.len(), 3);
        assert_eq!(cell.test_manifest.result_format, "json");
    }

    #[test]
    fn test_toolkit_id_constant() {
        assert_eq!(TOOLKIT_ID, "toolkit.alicia.automations");
    }
}
