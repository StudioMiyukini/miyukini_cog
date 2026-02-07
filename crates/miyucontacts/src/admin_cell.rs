use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MiyucontactsIdentification {
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
pub struct MiyucontactsTestManifest {
    pub tests: Vec<EmbeddedTestDef>,
    #[serde(default = "default_result_format")]
    pub result_format: String,
}

fn default_result_format() -> String { "json".to_string() }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiyucontactsIntegrity {
    pub fingerprint: String,
    pub contracts: Vec<String>,
    #[serde(default)]
    pub core_versions: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiyucontactsAdminCell {
    pub identification: MiyucontactsIdentification,
    pub test_manifest: MiyucontactsTestManifest,
    pub integrity: MiyucontactsIntegrity,
}

pub const TOOLKIT_ID: &str = "toolkit.communication.contacts";

#[must_use] 
pub fn miyucontacts_admin_cell(version: &str, fingerprint: &str) -> MiyucontactsAdminCell {
    MiyucontactsAdminCell {
        identification: MiyucontactsIdentification {
            id: TOOLKIT_ID.to_string(),
            version: version.to_string(),
            module_type: "toolkit".to_string(),
            module_origin: "Miyucontacts".to_string(),
        },
        test_manifest: MiyucontactsTestManifest {
            tests: vec![EmbeddedTestDef {
                id: "Miyucontacts".to_string(),
                name: "Test chemin complet Miyucontacts".to_string(),
                protocol: "invoke".to_string(),
                criteria: TestCriteria { pass: "all_steps_ok".to_string() },
            }],
            result_format: "json".to_string(),
        },
        integrity: MiyucontactsIntegrity {
            fingerprint: fingerprint.to_string(),
            contracts: vec!["Miyucontacts-Tool-Governance-Compliance".to_string()],
            core_versions: std::collections::HashMap::new(),
        },
    }
}
