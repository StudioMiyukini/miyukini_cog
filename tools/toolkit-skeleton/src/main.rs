//! Génère les squelettes de crates toolkit à partir de (name, toolkit_id).
//! Usage: cargo run --bin toolkit-skeleton (depuis tools/toolkit-skeleton ou workspace root)

fn pascal(s: &str) -> String {
    s.split('_')
        .map(|w| {
            let mut c = w.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().chain(c).collect(),
            }
        })
        .collect()
}

fn main() {
    let kits: &[(&str, &str)] = &[
        ("miyauth", "toolkit.identity.miyauth"),
        ("miyuweb", "toolkit.web.miyuweb"),
        ("miyuclock", "toolkit.kernel.miyuclock"),
        ("miyuforum", "toolkit.community.forum"),
        ("miyupm", "toolkit.communication.pm"),
        ("miyunotify", "toolkit.notify.miyunotify"),
        ("miyusearch", "toolkit.search.miyusearch"),
        ("miyuwebway_participant", "toolkit.webway.participant"),
        ("miyuwebway_tracker", "toolkit.webway.tracker"),
        ("miyubilling", "toolkit.billing.saas"),
        ("miyubooking", "toolkit.booking.reservations"),
        ("miyucms", "toolkit.content.cms"),
        ("miyumedia", "toolkit.content.media"),
        ("miyushipping", "toolkit.commerce.shipping"),
        ("miyustore", "toolkit.commerce.store"),
        ("miyuwidgets", "toolkit.web.widgets"),
        ("miyuinvoice", "toolkit.invoice.standalone"),
        ("miyucptaledger", "toolkit.compta.ledger"),
        ("miyuexpense", "toolkit.expense.claims"),
        ("miyutreasury", "toolkit.treasury.forecast"),
        ("miyuantispam", "toolkit.security.antispam"),
        ("miyubookmarks", "toolkit.content.bookmarks"),
        ("miyucomptareports", "toolkit.compta.reports"),
        ("miyucontacts", "toolkit.communication.contacts"),
        ("miyudeclarations", "toolkit.compta.declarations"),
        ("miyudiscovery", "toolkit.social.discovery"),
        ("miyufeeds", "toolkit.content.feeds"),
        ("miyuhr", "toolkit.hr.miyuhr"),
        ("miyumoderationforum", "toolkit.moderation.forum"),
        ("miyupolls", "toolkit.content.polls"),
        ("miyuposanalytics", "toolkit.pos.miyuposanalytics"),
        ("miyuposinventory", "toolkit.pos.miyuposinventory"),
        ("miyuposkitchen", "toolkit.pos.miyuposkitchen"),
        ("miyuposloyalty", "toolkit.pos.miyuposloyalty"),
        ("miyupospayment", "toolkit.pos.miyupospayment"),
        ("miyupossales", "toolkit.pos.miyupossales"),
        ("miyuprofile", "toolkit.identity.profile"),
        ("miyusocialfeed", "toolkit.social.feed"),
        ("miyusocialmessaging", "toolkit.social.messaging"),
        ("miyusocialmoderation", "toolkit.social.moderation"),
        ("miyusocialprofile", "toolkit.social.profile"),
        ("miyustory", "toolkit.social.story"),
    ];

    let root = std::env::current_dir().unwrap();
    let base = root.join("crates");
    for (name, toolkit_id) in kits {
        let p = base.join(name);
        if p.exists() {
            continue;
        }
        let pascal_name = pascal(name);
        let _ = std::fs::create_dir_all(p.join("src"));

        let desc = format!("Kit d'outils {} — squelette Phase 1", pascal_name);
        let origin = format!("miyukini-{}", name);
        let test_id = format!("Miyukini{}test", pascal_name);
        let contract = format!("{}-Tool-Governance-Compliance", pascal_name);

        let cargo = format!(
            r#"[package]
name = "{}"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true
repository.workspace = true
homepage.workspace = true
documentation.workspace = true
keywords.workspace = true
categories.workspace = true
description = "{} sous gouvernance"

[dependencies]
miyukini-kernel = {{ path = "../miyukini-kernel" }}
serde = {{ version = "1.0", features = ["derive"] }}

[dev-dependencies]

[lints.rust]
unsafe_code = "forbid"
missing_docs = "warn"

[lints.clippy]
all = "warn"
pedantic = "warn"
"#,
            name, desc
        );
        std::fs::write(p.join("Cargo.toml"), cargo).expect("write Cargo.toml");

        let lib = format!(
            r#"//! # {} — {}

pub mod admin_cell;
pub mod context;
pub mod errors;
pub mod tools;

pub use admin_cell::{{{}_admin_cell, {}AdminCell, {}Identification, {}Integrity, {}TestManifest, TOOLKIT_ID}};
pub use context::GovernedContext;
pub use errors::{}Error;
pub use tools::execute;
"#,
            pascal_name,
            toolkit_id,
            name,
            pascal_name,
            pascal_name,
            pascal_name,
            pascal_name,
            pascal_name
        );
        std::fs::write(p.join("src/lib.rs"), lib).expect("write lib.rs");

        let admin = format!(
            r#"use serde::{{Deserialize, Serialize}};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct {}Identification {{
    pub id: String,
    pub version: String,
    pub module_type: String,
    pub module_origin: String,
}}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TestCriteria {{
    pub pass: String,
}}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddedTestDef {{
    pub id: String,
    pub name: String,
    pub protocol: String,
    pub criteria: TestCriteria,
}}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {}TestManifest {{
    pub tests: Vec<EmbeddedTestDef>,
    #[serde(default = "default_result_format")]
    pub result_format: String,
}}

fn default_result_format() -> String {{ "json".to_string() }}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {}Integrity {{
    pub fingerprint: String,
    pub contracts: Vec<String>,
    #[serde(default)]
    pub core_versions: std::collections::HashMap<String, String>,
}}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {}AdminCell {{
    pub identification: {}Identification,
    pub test_manifest: {}TestManifest,
    pub integrity: {}Integrity,
}}

pub const TOOLKIT_ID: &str = "{}";

pub fn {}_admin_cell(version: &str, fingerprint: &str) -> {}AdminCell {{
    {}AdminCell {{
        identification: {}Identification {{
            id: TOOLKIT_ID.to_string(),
            version: version.to_string(),
            module_type: "toolkit".to_string(),
            module_origin: "{}".to_string(),
        }},
        test_manifest: {}TestManifest {{
            tests: vec![EmbeddedTestDef {{
                id: "{}".to_string(),
                name: "Test chemin complet {}".to_string(),
                protocol: "invoke".to_string(),
                criteria: TestCriteria {{ pass: "all_steps_ok".to_string() }},
            }}],
            result_format: "json".to_string(),
        }},
        integrity: {}Integrity {{
            fingerprint: fingerprint.to_string(),
            contracts: vec!["{}".to_string()],
            core_versions: std::collections::HashMap::new(),
        }},
    }}
}}
"#,
            pascal_name,
            pascal_name,
            pascal_name,
            pascal_name,
            pascal_name,
            pascal_name,
            pascal_name,
            toolkit_id,
            name,
            pascal_name,
            origin,
            pascal_name,
            pascal_name,
            test_id,
            pascal_name,
            pascal_name,
            pascal_name,
            contract
        );
        std::fs::write(p.join("src/admin_cell.rs"), admin).expect("write admin_cell.rs");

        let context = r#"#[derive(Debug, Clone)]
pub struct GovernedContext {
    pub mandate_id: String,
    pub security_level: u8,
}

impl GovernedContext {
    pub fn new(mandate_id: String, security_level: u8) -> Self {
        Self { mandate_id, security_level }
    }
    pub fn has_mandate(&self) -> bool {
        !self.mandate_id.is_empty()
    }
}
"#;
        std::fs::write(p.join("src/context.rs"), context).expect("write context.rs");

        let errors = format!(
            r#"#[derive(Debug, Clone)]
pub enum {}Error {{
    NoMandate,
    Unimplemented,
}}

impl std::fmt::Display for {}Error {{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
        match self {{
            {}Error::NoMandate => write!(f, "Execution refused: no governed mandate"),
            {}Error::Unimplemented => write!(f, "Tool not yet implemented"),
        }}
    }}
}}
impl std::error::Error for {}Error {{}}
"#,
            pascal_name, pascal_name, pascal_name, pascal_name, pascal_name
        );
        std::fs::write(p.join("src/errors.rs"), errors).expect("write errors.rs");

        let tools = format!(
            r#"use crate::context::GovernedContext;
use crate::errors::{}Error;

pub fn execute(ctx: &GovernedContext, _tool_id: &str, _params: &[String]) -> Result<String, {}Error> {{
    if !ctx.has_mandate() {{
        return Err({}Error::NoMandate);
    }}
    Err({}Error::Unimplemented)
}}
"#,
            pascal_name, pascal_name, pascal_name, pascal_name
        );
        std::fs::write(p.join("src/tools.rs"), tools).expect("write tools.rs");

        println!("Created crate {}", name);
    }
}
