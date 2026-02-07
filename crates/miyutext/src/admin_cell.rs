//! Cellule Admin MiyuText. Exposée uniquement à MiyukiniAdmin.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MiyuTextIdentification {
    pub id: String,
    pub version: String,
    pub module_type: String,
    pub module_origin: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TestCriteria {
    pub pass: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddedTestDef {
    pub id: String,
    pub name: String,
    pub protocol: String,
    pub criteria: TestCriteria,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiyuTextTestManifest {
    pub tests: Vec<EmbeddedTestDef>,
    #[serde(default = "default_result_format")]
    pub result_format: String,
}

fn default_result_format() -> String {
    "json".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiyuTextIntegrity {
    pub fingerprint: String,
    pub contracts: Vec<String>,
    #[serde(default)]
    pub core_versions: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiyuTextAdminCell {
    pub identification: MiyuTextIdentification,
    pub test_manifest: MiyuTextTestManifest,
    pub integrity: MiyuTextIntegrity,
}

pub const TOOLKIT_ID: &str = "toolkit.text.miyutext";

#[must_use] 
pub fn miyutext_admin_cell(version: &str, fingerprint: &str) -> MiyuTextAdminCell {
    MiyuTextAdminCell {
        identification: MiyuTextIdentification {
            id: TOOLKIT_ID.to_string(),
            version: version.to_string(),
            module_type: "toolkit".to_string(),
            module_origin: "miyukini-miyutext".to_string(),
        },
        test_manifest: MiyuTextTestManifest {
            tests: vec![EmbeddedTestDef {
                id: "MiyukiniTexttest".to_string(),
                name: "Test chemin complet MiyuText".to_string(),
                protocol: "invoke".to_string(),
                criteria: TestCriteria { pass: "all_steps_ok".to_string() },
            }],
            result_format: "json".to_string(),
        },
        integrity: MiyuTextIntegrity {
            fingerprint: fingerprint.to_string(),
            contracts: vec!["MiyuText-Tool-Governance-Compliance".to_string()],
            core_versions: std::collections::HashMap::new(),
        },
    }
}
