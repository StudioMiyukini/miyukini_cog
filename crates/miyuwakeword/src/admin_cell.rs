/// @id toolkit.alicia.wakeword.admin_cell
/// @do define_admin_cell_metadata_for_wake_word_toolkit
/// @role governance_metadata
/// @layer 6
/// @human Metadonnees de gouvernance AdminCell pour le toolkit alicia-wakeword
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

/// Identifiant unique du toolkit dans la pyramide COG.
pub const TOOLKIT_ID: &str = "toolkit.alicia.wakeword";

/// Singleton AdminCell (pattern OnceLock Miyukini).
static ADMIN_CELL: OnceLock<AliciaWakewordAdminCell> = OnceLock::new();

/// Identification du toolkit dans le registre COG.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AliciaWakewordIdentification {
    /// Identifiant MIP du toolkit.
    pub id: String,
    /// Version semantique.
    pub version: String,
    /// Type de module (toolkit).
    pub module_type: String,
    /// Origine du module.
    pub module_origin: String,
}

/// Critere de reussite d'un test embarque.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TestCriteria {
    /// Condition de succes.
    pub pass: String,
}

/// Definition d'un test embarque dans l'AdminCell.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddedTestDef {
    /// Identifiant du test.
    pub id: String,
    /// Nom descriptif.
    pub name: String,
    /// Protocole d'execution.
    pub protocol: String,
    /// Critere de reussite.
    pub criteria: TestCriteria,
}

/// Manifeste des tests embarques.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AliciaWakewordTestManifest {
    /// Liste des tests embarques.
    pub tests: Vec<EmbeddedTestDef>,
    /// Format de resultat (defaut: "json").
    #[serde(default = "default_result_format")]
    pub result_format: String,
}

fn default_result_format() -> String {
    "json".to_string()
}

/// Informations d'integrite du toolkit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AliciaWakewordIntegrity {
    /// Empreinte du module.
    pub fingerprint: String,
    /// Contrats de gouvernance.
    pub contracts: Vec<String>,
    /// Versions des Cores utilises.
    #[serde(default)]
    pub core_versions: std::collections::HashMap<String, String>,
}

/// AdminCell complete du toolkit alicia-wakeword.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AliciaWakewordAdminCell {
    /// Identification du toolkit.
    pub identification: AliciaWakewordIdentification,
    /// Manifeste des tests.
    pub test_manifest: AliciaWakewordTestManifest,
    /// Integrite du module.
    pub integrity: AliciaWakewordIntegrity,
}

/// Construit l'AdminCell du toolkit alicia-wakeword.
#[must_use]
pub fn alicia_wakeword_admin_cell(version: &str, fingerprint: &str) -> AliciaWakewordAdminCell {
    AliciaWakewordAdminCell {
        identification: AliciaWakewordIdentification {
            id: TOOLKIT_ID.to_string(),
            version: version.to_string(),
            module_type: "toolkit".to_string(),
            module_origin: "AliciaWakeword".to_string(),
        },
        test_manifest: AliciaWakewordTestManifest {
            tests: vec![EmbeddedTestDef {
                id: "AliciaWakeWord".to_string(),
                name: "Test chemin complet AliciaWakeWord".to_string(),
                protocol: "invoke".to_string(),
                criteria: TestCriteria {
                    pass: "all_steps_ok".to_string(),
                },
            }],
            result_format: "json".to_string(),
        },
        integrity: AliciaWakewordIntegrity {
            fingerprint: fingerprint.to_string(),
            contracts: vec!["AliciaWakeWord-Tool-Governance-Compliance".to_string()],
            core_versions: std::collections::HashMap::new(),
        },
    }
}

/// Initialise et retourne le singleton AdminCell.
///
/// Appeler une seule fois au demarrage ; les appels suivants retournent
/// la meme reference.
pub fn init_admin_cell(version: &str, fingerprint: &str) -> &'static AliciaWakewordAdminCell {
    ADMIN_CELL.get_or_init(|| alicia_wakeword_admin_cell(version, fingerprint))
}

/// Retourne le singleton AdminCell s'il a ete initialise.
pub fn get_admin_cell() -> Option<&'static AliciaWakewordAdminCell> {
    ADMIN_CELL.get()
}
