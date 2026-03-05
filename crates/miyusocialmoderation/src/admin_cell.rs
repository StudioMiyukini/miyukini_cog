use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MiyusocialmoderationIdentification {
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
pub struct MiyusocialmoderationTestManifest {
    pub tests: Vec<EmbeddedTestDef>,
    #[serde(default = "default_result_format")]
    pub result_format: String,
}

fn default_result_format() -> String {
    "json".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiyusocialmoderationIntegrity {
    pub fingerprint: String,
    pub contracts: Vec<String>,
    #[serde(default)]
    pub core_versions: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiyusocialmoderationAdminCell {
    pub identification: MiyusocialmoderationIdentification,
    pub test_manifest: MiyusocialmoderationTestManifest,
    pub integrity: MiyusocialmoderationIntegrity,
}

pub const TOOLKIT_ID: &str = "toolkit.social.moderation";

#[must_use]
pub fn miyusocialmoderation_admin_cell(
    version: &str,
    fingerprint: &str,
) -> MiyusocialmoderationAdminCell {
    MiyusocialmoderationAdminCell {
        identification: MiyusocialmoderationIdentification {
            id: TOOLKIT_ID.to_string(),
            version: version.to_string(),
            module_type: "toolkit".to_string(),
            module_origin: "Miyusocialmoderation".to_string(),
        },
        test_manifest: MiyusocialmoderationTestManifest {
            tests: vec![EmbeddedTestDef {
                id: "Miyusocialmoderation".to_string(),
                name: "Test chemin complet Miyusocialmoderation".to_string(),
                protocol: "invoke".to_string(),
                criteria: TestCriteria {
                    pass: "all_steps_ok".to_string(),
                },
            }],
            result_format: "json".to_string(),
        },
        integrity: MiyusocialmoderationIntegrity {
            fingerprint: fingerprint.to_string(),
            contracts: vec!["Miyusocialmoderation-Tool-Governance-Compliance".to_string()],
            core_versions: std::collections::HashMap::new(),
        },
    }
}
