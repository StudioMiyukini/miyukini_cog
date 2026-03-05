# Exemples concrets — Patterns Rust Miyukini

## Exemple complet : Creer un nouveau Toolkit

### 1. Creer le dossier

```
crates/miyu{nom}/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── admin_cell.rs
    ├── context.rs
    └── errors.rs
```

### 2. Cargo.toml

```toml
[package]
name = "miyu{nom}"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true
repository.workspace = true
homepage.workspace = true
documentation.workspace = true
keywords.workspace = true
categories.workspace = true
description = "Kit d'outils Miyu{Nom} — squelette Phase 1 sous gouvernance"

[dependencies]
miyukini-kernel = { path = "../miyukini-kernel" }
serde = { version = "1.0", features = ["derive"] }

[dev-dependencies]

[lints.rust]
unsafe_code = "forbid"
missing_docs = "warn"

[lints.clippy]
all = { level = "warn", priority = -1 }
pedantic = { level = "warn", priority = -1 }
cast_lossless = "allow"
cast_possible_truncation = "allow"
cast_possible_wrap = "allow"
cast_precision_loss = "allow"
cast_sign_loss = "allow"
doc_markdown = "allow"
missing_errors_doc = "allow"
missing_panics_doc = "allow"
module_name_repetitions = "allow"
must_use_candidate = "allow"
needless_pass_by_value = "allow"
return_self_not_must_use = "allow"
similar_names = "allow"
struct_excessive_bools = "allow"
too_many_lines = "allow"
unreadable_literal = "allow"
wildcard_imports = "allow"
```

### 3. Ajouter au workspace

Dans le `Cargo.toml` racine, ajouter le crate a la liste `members` :

```toml
[workspace]
members = [
    # ... autres crates ...
    "crates/miyu{nom}",
]
```

### 4. admin_cell.rs concret (exemple : miyuexample)

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MiyuexampleIdentification {
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

fn default_result_format() -> String {
    "structured".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiyuexampleTestManifest {
    pub tests: Vec<EmbeddedTestDef>,
    #[serde(default = "default_result_format")]
    pub result_format: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiyuexampleIntegrity {
    pub fingerprint: String,
    pub contracts: Vec<String>,
    #[serde(default)]
    pub core_versions: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiyuexampleAdminCell {
    pub identification: MiyuexampleIdentification,
    pub test_manifest: MiyuexampleTestManifest,
    pub integrity: MiyuexampleIntegrity,
}

pub const TOOLKIT_ID: &str = "toolkit.example.miyuexample";

#[must_use]
pub fn miyuexample_admin_cell(version: &str, fingerprint: &str) -> MiyuexampleAdminCell {
    MiyuexampleAdminCell {
        identification: MiyuexampleIdentification {
            id: TOOLKIT_ID.to_string(),
            version: version.to_string(),
            module_type: "toolkit".to_string(),
            module_origin: "miyukini".to_string(),
        },
        test_manifest: MiyuexampleTestManifest {
            tests: vec![EmbeddedTestDef {
                id: "test_example_basic".to_string(),
                name: "Basic example validation".to_string(),
                protocol: "unit".to_string(),
                criteria: TestCriteria {
                    pass: "all_assertions_pass".to_string(),
                },
            }],
            result_format: default_result_format(),
        },
        integrity: MiyuexampleIntegrity {
            fingerprint: fingerprint.to_string(),
            contracts: vec!["governance.compliance.miyuexample".to_string()],
            core_versions: std::collections::HashMap::new(),
        },
    }
}
```

### 5. context.rs (identique dans tous les modules)

```rust
#[derive(Debug, Clone)]
pub struct GovernedContext {
    pub mandate_id: String,
    pub security_level: u8,
}

impl GovernedContext {
    #[must_use]
    pub fn new(mandate_id: String, security_level: u8) -> Self {
        Self {
            mandate_id,
            security_level,
        }
    }

    #[must_use]
    pub fn has_mandate(&self) -> bool {
        !self.mandate_id.is_empty()
    }
}
```

### 6. errors.rs

```rust
#[derive(Debug, Clone)]
pub enum MiyuexampleError {
    NoMandate,
    Unimplemented,
}

impl std::fmt::Display for MiyuexampleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoMandate => write!(f, "Execution refused: no governed mandate"),
            Self::Unimplemented => write!(f, "Tool not yet implemented"),
        }
    }
}

impl std::error::Error for MiyuexampleError {}
```

### 7. lib.rs

```rust
#![allow(missing_docs)]
//! # MiyuExample — toolkit.example.miyuexample
//!
//! Kit d'outils Example sous gouvernance Miyukini.

pub mod admin_cell;
pub mod context;
pub mod errors;

pub use admin_cell::{
    miyuexample_admin_cell, MiyuexampleAdminCell, MiyuexampleIdentification,
    MiyuexampleIntegrity, MiyuexampleTestManifest, TOOLKIT_ID,
};
pub use context::GovernedContext;
pub use errors::MiyuexampleError;
```

## Crates de reference pour chaque type

| Type | Exemple de reference | Chemin |
|------|---------------------|--------|
| Toolkit simple | miyucalc | `crates/miyucalc/` |
| Toolkit avec modules metier | miyujobs | `crates/miyujobs/` |
| Core | strongfather | `crates/strongfather/` |
| Service Jay | jayfestival | `crates/jayfestival/` |
| Application | miyukini-central | `crates/miyukini-central/` |
| Operateur admin | miyukini-admin | `crates/miyukini-admin/` |
