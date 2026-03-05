//! Cellule Admin Alicia Capture (Module Testing and Lifecycle Contract).
//!
//! Identification toolkit.alicia.capture, manifeste de test, integrite.
//! Exposee uniquement a MiyukiniAdmin.

use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

// @id: alicia_capture_admin_cell
// @role: data
// @layer: toolkit
// @human: Cellule Admin Alicia Capture : identification, manifeste de test, integrite.
// @do: expose_admin_cell

/// Identifiant canonique du toolkit.
pub const TOOLKIT_ID: &str = "toolkit.alicia.capture";

/// Singleton AdminCell (pattern OnceLock Miyukini).
static ADMIN_CELL: OnceLock<AliciaCaptureAdminCell> = OnceLock::new();

/// @id: alicia_capture_admin_identification
/// @role: data
/// @layer: toolkit
/// @human: Identification du module (toolkit.alicia.capture).
/// @do: store_identification
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AliciaCaptureIdentification {
    /// Identifiant unique du toolkit.
    pub id: String,
    /// Version du module.
    pub version: String,
    /// Type : toolkit.
    pub module_type: String,
    /// Module d'origine.
    pub module_origin: String,
}

/// @id: alicia_capture_admin_test_criteria
/// @role: data
/// @layer: toolkit
/// @human: Criteres de succes/echec pour un test embarque.
/// @do: store_test_criteria
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TestCriteria {
    /// Condition de passage (ex. all_steps_ok).
    pub pass: String,
}

/// @id: alicia_capture_admin_embedded_test_def
/// @role: data
/// @layer: toolkit
/// @human: Definition d'un test dans le manifeste.
/// @do: store_embedded_test_def
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

/// @id: alicia_capture_admin_test_manifest
/// @role: data
/// @layer: toolkit
/// @human: Manifeste de test embarque.
/// @do: store_test_manifest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AliciaCaptureTestManifest {
    /// Liste des tests declares.
    pub tests: Vec<EmbeddedTestDef>,
    /// Format des resultats (json).
    #[serde(default = "default_result_format")]
    pub result_format: String,
}

fn default_result_format() -> String {
    "json".to_string()
}

/// @id: alicia_capture_admin_integrity
/// @role: data
/// @layer: toolkit
/// @human: Metadonnees d'integrite pour verification TAMR.
/// @do: store_integrity_metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AliciaCaptureIntegrity {
    /// Empreinte du module (hash).
    pub fingerprint: String,
    /// Contrats references.
    pub contracts: Vec<String>,
    /// Versions des cores attendues.
    #[serde(default)]
    pub core_versions: std::collections::HashMap<String, String>,
}

/// @id: alicia_capture_admin_cell_struct
/// @role: data
/// @layer: toolkit
/// @human: Cellule Admin complete AliciaCapture.
/// @do: represent_admin_cell
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AliciaCaptureAdminCell {
    /// Identification du module.
    pub identification: AliciaCaptureIdentification,
    /// Manifeste de test.
    pub test_manifest: AliciaCaptureTestManifest,
    /// Metadonnees d'integrite.
    pub integrity: AliciaCaptureIntegrity,
}

/// @id: alicia_capture_admin_cell_build
/// @role: mutator
/// @layer: toolkit
/// @human: Construit la Cellule Admin AliciaCapture.
/// @do: build_alicia_capture_admin_cell
#[must_use]
pub fn alicia_capture_admin_cell(version: &str, fingerprint: &str) -> AliciaCaptureAdminCell {
    AliciaCaptureAdminCell {
        identification: AliciaCaptureIdentification {
            id: TOOLKIT_ID.to_string(),
            version: version.to_string(),
            module_type: "toolkit".to_string(),
            module_origin: "miyukini-alicia-capture".to_string(),
        },
        test_manifest: AliciaCaptureTestManifest {
            tests: vec![EmbeddedTestDef {
                id: "AliciaCaptureTest".to_string(),
                name: "Test chemin complet AliciaCapture (devices, capture, VAD, buffer)"
                    .to_string(),
                protocol: "invoke".to_string(),
                criteria: TestCriteria {
                    pass: "all_steps_ok".to_string(),
                },
            }],
            result_format: "json".to_string(),
        },
        integrity: AliciaCaptureIntegrity {
            fingerprint: fingerprint.to_string(),
            contracts: vec!["AliciaCapture-Tool-Governance-Compliance".to_string()],
            core_versions: std::collections::HashMap::new(),
        },
    }
}

/// Initialise et retourne le singleton AdminCell.
///
/// Appeler une seule fois au demarrage ; les appels suivants retournent
/// la meme reference.
pub fn init_admin_cell(version: &str, fingerprint: &str) -> &'static AliciaCaptureAdminCell {
    ADMIN_CELL.get_or_init(|| alicia_capture_admin_cell(version, fingerprint))
}

/// Retourne le singleton AdminCell s'il a ete initialise.
pub fn get_admin_cell() -> Option<&'static AliciaCaptureAdminCell> {
    ADMIN_CELL.get()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_admin_cell_build() {
        let cell = alicia_capture_admin_cell("0.1.0", "abc123");
        assert_eq!(cell.identification.id, TOOLKIT_ID);
        assert_eq!(cell.identification.version, "0.1.0");
        assert_eq!(cell.identification.module_type, "toolkit");
        assert_eq!(cell.integrity.fingerprint, "abc123");
        assert_eq!(cell.test_manifest.tests.len(), 1);
        assert_eq!(cell.test_manifest.result_format, "json");
    }

    #[test]
    fn test_toolkit_id_constant() {
        assert_eq!(TOOLKIT_ID, "toolkit.alicia.capture");
    }
}
