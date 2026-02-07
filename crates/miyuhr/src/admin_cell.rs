use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MiyuhrIdentification {
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
pub struct MiyuhrTestManifest {
    pub tests: Vec<EmbeddedTestDef>,
    #[serde(default = "default_result_format")]
    pub result_format: String,
}

fn default_result_format() -> String { "json".to_string() }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiyuhrIntegrity {
    pub fingerprint: String,
    pub contracts: Vec<String>,
    #[serde(default)]
    pub core_versions: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiyuhrAdminCell {
    pub identification: MiyuhrIdentification,
    pub test_manifest: MiyuhrTestManifest,
    pub integrity: MiyuhrIntegrity,
}

pub const TOOLKIT_ID: &str = "toolkit.hr.miyuhr";

#[must_use] 
pub fn miyuhr_admin_cell(version: &str, fingerprint: &str) -> MiyuhrAdminCell {
    MiyuhrAdminCell {
        identification: MiyuhrIdentification {
            id: TOOLKIT_ID.to_string(),
            version: version.to_string(),
            module_type: "toolkit".to_string(),
            module_origin: "Miyuhr".to_string(),
        },
        test_manifest: MiyuhrTestManifest {
            tests: vec![EmbeddedTestDef {
                id: "Miyuhr".to_string(),
                name: "Test chemin complet Miyuhr".to_string(),
                protocol: "invoke".to_string(),
                criteria: TestCriteria { pass: "all_steps_ok".to_string() },
            }],
            result_format: "json".to_string(),
        },
        integrity: MiyuhrIntegrity {
            fingerprint: fingerprint.to_string(),
            contracts: vec!["Miyuhr-Tool-Governance-Compliance".to_string()],
            core_versions: std::collections::HashMap::new(),
        },
    }
}
