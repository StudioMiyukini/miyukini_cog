use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MiyuwebwayTrackerIdentification {
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
pub struct MiyuwebwayTrackerTestManifest {
    pub tests: Vec<EmbeddedTestDef>,
    #[serde(default = "default_result_format")]
    pub result_format: String,
}

fn default_result_format() -> String { "json".to_string() }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiyuwebwayTrackerIntegrity {
    pub fingerprint: String,
    pub contracts: Vec<String>,
    #[serde(default)]
    pub core_versions: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiyuwebwayTrackerAdminCell {
    pub identification: MiyuwebwayTrackerIdentification,
    pub test_manifest: MiyuwebwayTrackerTestManifest,
    pub integrity: MiyuwebwayTrackerIntegrity,
}

pub const TOOLKIT_ID: &str = "toolkit.webway.tracker";

#[must_use] 
pub fn miyuwebway_tracker_admin_cell(version: &str, fingerprint: &str) -> MiyuwebwayTrackerAdminCell {
    MiyuwebwayTrackerAdminCell {
        identification: MiyuwebwayTrackerIdentification {
            id: TOOLKIT_ID.to_string(),
            version: version.to_string(),
            module_type: "toolkit".to_string(),
            module_origin: "MiyuwebwayTracker".to_string(),
        },
        test_manifest: MiyuwebwayTrackerTestManifest {
            tests: vec![EmbeddedTestDef {
                id: "MiyuwebwayTracker".to_string(),
                name: "Test chemin complet MiyuwebwayTracker".to_string(),
                protocol: "invoke".to_string(),
                criteria: TestCriteria { pass: "all_steps_ok".to_string() },
            }],
            result_format: "json".to_string(),
        },
        integrity: MiyuwebwayTrackerIntegrity {
            fingerprint: fingerprint.to_string(),
            contracts: vec!["MiyuwebwayTracker-Tool-Governance-Compliance".to_string()],
            core_versions: std::collections::HashMap::new(),
        },
    }
}
