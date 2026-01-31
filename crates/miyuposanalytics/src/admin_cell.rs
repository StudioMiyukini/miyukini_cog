use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MiyuposanalyticsIdentification {
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
pub struct MiyuposanalyticsTestManifest {
    pub tests: Vec<EmbeddedTestDef>,
    #[serde(default = "default_result_format")]
    pub result_format: String,
}

fn default_result_format() -> String { "json".to_string() }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiyuposanalyticsIntegrity {
    pub fingerprint: String,
    pub contracts: Vec<String>,
    #[serde(default)]
    pub core_versions: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiyuposanalyticsAdminCell {
    pub identification: MiyuposanalyticsIdentification,
    pub test_manifest: MiyuposanalyticsTestManifest,
    pub integrity: MiyuposanalyticsIntegrity,
}

pub const TOOLKIT_ID: &str = "toolkit.pos.miyuposanalytics";

pub fn miyuposanalytics_admin_cell(version: &str, fingerprint: &str) -> MiyuposanalyticsAdminCell {
    MiyuposanalyticsAdminCell {
        identification: MiyuposanalyticsIdentification {
            id: TOOLKIT_ID.to_string(),
            version: version.to_string(),
            module_type: "toolkit".to_string(),
            module_origin: "Miyuposanalytics".to_string(),
        },
        test_manifest: MiyuposanalyticsTestManifest {
            tests: vec![EmbeddedTestDef {
                id: "Miyuposanalytics".to_string(),
                name: "Test chemin complet Miyuposanalytics".to_string(),
                protocol: "invoke".to_string(),
                criteria: TestCriteria { pass: "all_steps_ok".to_string() },
            }],
            result_format: "json".to_string(),
        },
        integrity: MiyuposanalyticsIntegrity {
            fingerprint: fingerprint.to_string(),
            contracts: vec!["Miyuposanalytics-Tool-Governance-Compliance".to_string()],
            core_versions: std::collections::HashMap::new(),
        },
    }
}
