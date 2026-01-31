use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MiyumediaIdentification {
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
pub struct MiyumediaTestManifest {
    pub tests: Vec<EmbeddedTestDef>,
    #[serde(default = "default_result_format")]
    pub result_format: String,
}

fn default_result_format() -> String { "json".to_string() }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiyumediaIntegrity {
    pub fingerprint: String,
    pub contracts: Vec<String>,
    #[serde(default)]
    pub core_versions: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiyumediaAdminCell {
    pub identification: MiyumediaIdentification,
    pub test_manifest: MiyumediaTestManifest,
    pub integrity: MiyumediaIntegrity,
}

pub const TOOLKIT_ID: &str = "toolkit.content.media";

pub fn miyumedia_admin_cell(version: &str, fingerprint: &str) -> MiyumediaAdminCell {
    MiyumediaAdminCell {
        identification: MiyumediaIdentification {
            id: TOOLKIT_ID.to_string(),
            version: version.to_string(),
            module_type: "toolkit".to_string(),
            module_origin: "Miyumedia".to_string(),
        },
        test_manifest: MiyumediaTestManifest {
            tests: vec![EmbeddedTestDef {
                id: "Miyumedia".to_string(),
                name: "Test chemin complet Miyumedia".to_string(),
                protocol: "invoke".to_string(),
                criteria: TestCriteria { pass: "all_steps_ok".to_string() },
            }],
            result_format: "json".to_string(),
        },
        integrity: MiyumediaIntegrity {
            fingerprint: fingerprint.to_string(),
            contracts: vec!["Miyumedia-Tool-Governance-Compliance".to_string()],
            core_versions: std::collections::HashMap::new(),
        },
    }
}
