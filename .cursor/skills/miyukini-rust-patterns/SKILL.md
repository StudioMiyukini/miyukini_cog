---
name: miyukini-rust-patterns
description: Patterns Rust recurrents du codebase Miyukini COG. Structure standard des crates, pattern admin_cell.rs, context.rs, errors.rs, lib.rs, Cargo.toml. Utiliser quand on cree un nouveau crate/toolkit/module, quand on modifie un module existant, ou quand on veut comprendre la structure standard du code.
---

# Patterns Rust — Miyukini COG

## Structure standard d'un crate Toolkit

Chaque toolkit (Strate 6) suit cette structure :

```
crates/miyu{nom}/
├── Cargo.toml
└── src/
    ├── lib.rs          # Racine du module + API publique
    ├── admin_cell.rs   # Metadonnees d'administration
    ├── context.rs      # Contexte gouverne
    ├── errors.rs       # Types d'erreur
    └── {metier}.rs     # Modules metier specifiques
```

## 1. Pattern `admin_cell.rs`

Fournit les metadonnees du module pour la gouvernance.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct {Module}Identification {
    pub id: String,
    pub version: String,
    pub module_type: String,
    pub module_origin: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
pub struct {Module}TestManifest {
    pub tests: Vec<EmbeddedTestDef>,
    #[serde(default = "default_result_format")]
    pub result_format: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {Module}Integrity {
    pub fingerprint: String,
    pub contracts: Vec<String>,
    #[serde(default)]
    pub core_versions: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {Module}AdminCell {
    pub identification: {Module}Identification,
    pub test_manifest: {Module}TestManifest,
    pub integrity: {Module}Integrity,
}

pub const TOOLKIT_ID: &str = "toolkit.{domain}.{module_name}";

#[must_use]
pub fn {module}_admin_cell(version: &str, fingerprint: &str) -> {Module}AdminCell {
    // Construction de l'AdminCell avec tous les champs
}
```

**Convention de nommage :**
- Struct : `{Module}AdminCell` (ex: `MiyauthAdminCell`)
- Fonction : `{module}_admin_cell` (ex: `miyauth_admin_cell`)
- Constante : `TOOLKIT_ID` = `"toolkit.{domain}.{name}"`

## 2. Pattern `context.rs`

Identique dans tous les modules :

```rust
#[derive(Debug, Clone)]
pub struct GovernedContext {
    pub mandate_id: String,
    pub security_level: u8,
}

impl GovernedContext {
    #[must_use]
    pub fn new(mandate_id: String, security_level: u8) -> Self {
        Self { mandate_id, security_level }
    }

    #[must_use]
    pub fn has_mandate(&self) -> bool {
        !self.mandate_id.is_empty()
    }
}
```

## 3. Pattern `errors.rs`

```rust
#[derive(Debug, Clone)]
pub enum {Module}Error {
    NoMandate,
    Unimplemented,
    // Variantes specifiques au module
}

impl std::fmt::Display for {Module}Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoMandate => write!(f, "Execution refused: no governed mandate"),
            Self::Unimplemented => write!(f, "Tool not yet implemented"),
        }
    }
}

impl std::error::Error for {Module}Error {}
```

## 4. Pattern `lib.rs`

```rust
#![allow(missing_docs)]
//! # {ModuleName} — {TOOLKIT_ID}
//!
//! {Description du module}

pub mod admin_cell;
pub mod context;
pub mod errors;
// pub mod {modules_metier};

pub use admin_cell::{
    {module}_admin_cell, {Module}AdminCell, {Module}Identification,
    {Module}Integrity, {Module}TestManifest, TOOLKIT_ID,
};
pub use context::GovernedContext;
pub use errors::{Module}Error;
```

## 5. Pattern `Cargo.toml`

```toml
[package]
name = "{crate_name}"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true
repository.workspace = true
homepage.workspace = true
documentation.workspace = true
keywords.workspace = true
categories.workspace = true
description = "Kit d'outils {ModuleName} — squelette Phase 1 sous gouvernance"

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
```

**Regles :**
- Toute metadata heritee du workspace (sauf `description`)
- Dependance obligatoire : `miyukini-kernel` + `serde`
- `unsafe_code = "forbid"` toujours
- Lints clippy pedantic actives

## 6. Conventions de nommage

| Element | Convention | Exemple |
|---------|-----------|---------|
| Struct | PascalCase avec prefixe module | `MiyauthAdminCell` |
| Fonction | snake_case avec prefixe module | `miyauth_admin_cell` |
| Constante | SCREAMING_SNAKE_CASE | `TOOLKIT_ID` |
| Crate toolkit | `miyu{nom}` | `miyauth` |
| Crate core | nom du core | `strongfather` |
| Crate service | nom du service | `jayfestival` |

## 7. Annotations MSCM dans le code

Certains modules incluent des annotations semantiques :

```rust
//! @id toolkit.auth.miyauth
//! @role security
//! @layer domain
//! @human Kit d'outils d'authentification
//! @do manage_authentication_and_identity
```

## References

- Exemples concrets : [references/examples.md](references/examples.md)
- Voir n'importe quel crate `crates/miyu*/` pour reference
