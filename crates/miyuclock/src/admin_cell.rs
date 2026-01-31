use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MiyuclockIdentification {
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
pub struct MiyuclockTestManifest {
    pub tests: Vec<EmbeddedTestDef>,
    #[serde(default = "default_result_format")]
    pub result_format: String,
}

fn default_result_format() -> String { "json".to_string() }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiyuclockIntegrity {
    pub fingerprint: String,
    pub contracts: Vec<String>,
    #[serde(default)]
    pub core_versions: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiyuclockAdminCell {
    pub identification: MiyuclockIdentification,
    pub test_manifest: MiyuclockTestManifest,
    pub integrity: MiyuclockIntegrity,
}

pub const TOOLKIT_ID: &str = "toolkit.kernel.miyuclock";

pub fn miyuclock_admin_cell(version: &str, fingerprint: &str) -> MiyuclockAdminCell {
    MiyuclockAdminCell {
        identification: MiyuclockIdentification {
            id: TOOLKIT_ID.to_string(),
            version: version.to_string(),
            module_type: "toolkit".to_string(),
            module_origin: "Miyuclock".to_string(),
        },
        test_manifest: MiyuclockTestManifest {
            tests: vec![EmbeddedTestDef {
                id: "Miyuclock".to_string(),
                name: "Test chemin complet Miyuclock".to_string(),
                protocol: "invoke".to_string(),
                criteria: TestCriteria { pass: "all_steps_ok".to_string() },
            }],
            result_format: "json".to_string(),
        },
        integrity: MiyuclockIntegrity {
            fingerprint: fingerprint.to_string(),
            contracts: vec!["Miyuclock-Tool-Governance-Compliance".to_string()],
            core_versions: std::collections::HashMap::new(),
        },
    }
}
