use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MiyusearchIdentification {
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
pub struct MiyusearchTestManifest {
    pub tests: Vec<EmbeddedTestDef>,
    #[serde(default = "default_result_format")]
    pub result_format: String,
}

fn default_result_format() -> String { "json".to_string() }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiyusearchIntegrity {
    pub fingerprint: String,
    pub contracts: Vec<String>,
    #[serde(default)]
    pub core_versions: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiyusearchAdminCell {
    pub identification: MiyusearchIdentification,
    pub test_manifest: MiyusearchTestManifest,
    pub integrity: MiyusearchIntegrity,
}

pub const TOOLKIT_ID: &str = "toolkit.search.miyusearch";

#[must_use] 
pub fn miyusearch_admin_cell(version: &str, fingerprint: &str) -> MiyusearchAdminCell {
    MiyusearchAdminCell {
        identification: MiyusearchIdentification {
            id: TOOLKIT_ID.to_string(),
            version: version.to_string(),
            module_type: "toolkit".to_string(),
            module_origin: "Miyusearch".to_string(),
        },
        test_manifest: MiyusearchTestManifest {
            tests: vec![EmbeddedTestDef {
                id: "Miyusearch".to_string(),
                name: "Test chemin complet Miyusearch".to_string(),
                protocol: "invoke".to_string(),
                criteria: TestCriteria { pass: "all_steps_ok".to_string() },
            }],
            result_format: "json".to_string(),
        },
        integrity: MiyusearchIntegrity {
            fingerprint: fingerprint.to_string(),
            contracts: vec!["Miyusearch-Tool-Governance-Compliance".to_string()],
            core_versions: std::collections::HashMap::new(),
        },
    }
}
