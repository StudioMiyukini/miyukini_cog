use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MiyuwebwayParticipantIdentification {
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
pub struct MiyuwebwayParticipantTestManifest {
    pub tests: Vec<EmbeddedTestDef>,
    #[serde(default = "default_result_format")]
    pub result_format: String,
}

fn default_result_format() -> String { "json".to_string() }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiyuwebwayParticipantIntegrity {
    pub fingerprint: String,
    pub contracts: Vec<String>,
    #[serde(default)]
    pub core_versions: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiyuwebwayParticipantAdminCell {
    pub identification: MiyuwebwayParticipantIdentification,
    pub test_manifest: MiyuwebwayParticipantTestManifest,
    pub integrity: MiyuwebwayParticipantIntegrity,
}

pub const TOOLKIT_ID: &str = "toolkit.webway.participant";

#[must_use] 
pub fn miyuwebway_participant_admin_cell(version: &str, fingerprint: &str) -> MiyuwebwayParticipantAdminCell {
    MiyuwebwayParticipantAdminCell {
        identification: MiyuwebwayParticipantIdentification {
            id: TOOLKIT_ID.to_string(),
            version: version.to_string(),
            module_type: "toolkit".to_string(),
            module_origin: "MiyuwebwayParticipant".to_string(),
        },
        test_manifest: MiyuwebwayParticipantTestManifest {
            tests: vec![EmbeddedTestDef {
                id: "MiyuwebwayParticipant".to_string(),
                name: "Test chemin complet MiyuwebwayParticipant".to_string(),
                protocol: "invoke".to_string(),
                criteria: TestCriteria { pass: "all_steps_ok".to_string() },
            }],
            result_format: "json".to_string(),
        },
        integrity: MiyuwebwayParticipantIntegrity {
            fingerprint: fingerprint.to_string(),
            contracts: vec!["MiyuwebwayParticipant-Tool-Governance-Compliance".to_string()],
            core_versions: std::collections::HashMap::new(),
        },
    }
}
