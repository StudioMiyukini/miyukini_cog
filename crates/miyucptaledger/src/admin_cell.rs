use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MiyucptaledgerIdentification {
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
pub struct MiyucptaledgerTestManifest {
    pub tests: Vec<EmbeddedTestDef>,
    #[serde(default = "default_result_format")]
    pub result_format: String,
}

fn default_result_format() -> String { "json".to_string() }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiyucptaledgerIntegrity {
    pub fingerprint: String,
    pub contracts: Vec<String>,
    #[serde(default)]
    pub core_versions: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiyucptaledgerAdminCell {
    pub identification: MiyucptaledgerIdentification,
    pub test_manifest: MiyucptaledgerTestManifest,
    pub integrity: MiyucptaledgerIntegrity,
}

pub const TOOLKIT_ID: &str = "toolkit.compta.ledger";

#[must_use] 
pub fn miyucptaledger_admin_cell(version: &str, fingerprint: &str) -> MiyucptaledgerAdminCell {
    MiyucptaledgerAdminCell {
        identification: MiyucptaledgerIdentification {
            id: TOOLKIT_ID.to_string(),
            version: version.to_string(),
            module_type: "toolkit".to_string(),
            module_origin: "Miyucptaledger".to_string(),
        },
        test_manifest: MiyucptaledgerTestManifest {
            tests: vec![EmbeddedTestDef {
                id: "Miyucptaledger".to_string(),
                name: "Test chemin complet Miyucptaledger".to_string(),
                protocol: "invoke".to_string(),
                criteria: TestCriteria { pass: "all_steps_ok".to_string() },
            }],
            result_format: "json".to_string(),
        },
        integrity: MiyucptaledgerIntegrity {
            fingerprint: fingerprint.to_string(),
            contracts: vec!["Miyucptaledger-Tool-Governance-Compliance".to_string()],
            core_versions: std::collections::HashMap::new(),
        },
    }
}
