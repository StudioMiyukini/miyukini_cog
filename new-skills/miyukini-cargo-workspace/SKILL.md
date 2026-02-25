---
name: miyukini-cargo-workspace
description: Configuration workspace Cargo de Miyukini COG. 106 membres, resolution de dependances, feature flags (legacy-sqlite, kindmother-only, db-encryption), lints Clippy, profils de build, dependances partagees (serde, tokio, rusqlite, uuid, chrono), metadata MSCM dans Cargo.toml, structure de crate standard, pnpm/Turbo pour frontend. Utiliser quand on ajoute un nouveau crate au workspace, quand on modifie les dependances, quand on configure les feature flags, quand on travaille sur le build system, ou quand on a des erreurs de compilation.
---

# Workspace Cargo — Miyukini COG

## Vue d'ensemble

2 workspaces Rust independants + 1 workspace Node.js :

| Workspace | Chemin | Membres | Usage |
|-----------|--------|---------|-------|
| Miyukini COG | `/Cargo.toml` | 106 crates | Ecosysteme principal |
| MGE | `/mge/Cargo.toml` | 113 crates | Game Engine (independant) |
| Node.js | `/package.json` | apps/central, apps/origin | Frontend Tauri/Dioxus |

---

## Cargo.toml racine

```toml
[workspace]
resolver = "2"
members = [
    # Kernel (Strate K)
    "crates/miyukini-kernel",

    # Cores (Strate 4) — 10 crates
    "crates/strongfather",
    "crates/kindmother",
    "crates/kindmother-client",
    "crates/kindmother-service",
    "crates/kindmother-db-adapter",
    "crates/kindmother-db-key",
    "crates/borderguard",
    "crates/caringnanny",
    "crates/masterbutler",
    "crates/bondingbrother",
    "crates/everbuddy",
    "crates/worrysentinel",
    "crates/tamr",
    "crates/logisticssteward",

    # Toolkits (Strate 6) — 49 crates
    "crates/miyu*",

    # Services/Operateurs (Strate 7)
    "crates/jayfestival", "crates/jayxpose", "crates/jaykoa",
    "crates/jayrdv", "crates/jayfaim", "crates/jay1tribu",
    "crates/jaykonta", "crates/miyukinisales",
    "crates/miyukinibb", "crates/miyukiniwatch",
    "crates/lord_of_the_castle",

    # Admin (Strate 9)
    "crates/miyukini-admin",

    # Apps
    "apps/central", "apps/origin", "apps/ui-builder", "apps/miyuclicker",

    # Tools
    "tools/mip-generator", "tools/toolkit-skeleton", "tools/toolkit-registry-export",
]

# Binaire par defaut
[[bin]]
name = "miyukini-central"
path = "apps/central/src/main.rs"
```

---

## Lints globaux

```toml
[workspace.lints.rust]
unsafe_code = "forbid"  # ABSOLU — aucun unsafe dans aucun crate

[workspace.lints.clippy]
# Actives : all + pedantic
all = "warn"
pedantic = "warn"

# 21 exceptions pragmatiques (pas des erreurs)
cast_possible_truncation = "allow"
cast_precision_loss = "allow"
cast_sign_loss = "allow"
cast_lossless = "allow"
module_name_repetitions = "allow"
must_use_candidate = "allow"
return_self_not_must_use = "allow"
missing_errors_doc = "allow"
missing_panics_doc = "allow"
struct_excessive_bools = "allow"
too_many_lines = "allow"
wildcard_imports = "allow"
needless_pass_by_value = "allow"
similar_names = "allow"
unreadable_literal = "allow"
used_underscore_binding = "allow"
items_after_statements = "allow"
match_wildcard_for_single_variants = "allow"
single_match_else = "allow"
struct_field_names = "allow"
manual_string_new = "allow"
```

---

## Feature flags — Pattern standard

### Dans chaque service Cargo.toml

```toml
[features]
default = ["legacy-sqlite"]
legacy-sqlite = ["rusqlite"]
kindmother-only = ["dep:kindmother-client", "dep:tokio"]
db-encryption = ["dep:kindmother-db-key", "rusqlite/bundled-sqlcipher"]
```

### Quand utiliser

| Flag | Contexte | Dependances activees |
|------|----------|---------------------|
| `legacy-sqlite` (defaut) | Dev rapide, prototypage | rusqlite direct |
| `kindmother-only` | Production securisee | kindmother-client + tokio (async) |
| `db-encryption` | Donnees sensibles | kindmother-db-key + SQLCipher |

### Pattern d'implementation dans data/mod.rs

```rust
#[cfg(feature = "legacy-sqlite")]
mod sqlite_impl;
#[cfg(feature = "kindmother-only")]
mod kindmother_impl;

// Re-export conditionnel
#[cfg(feature = "legacy-sqlite")]
pub use sqlite_impl::*;
#[cfg(feature = "kindmother-only")]
pub use kindmother_impl::*;
```

### Compiler avec un flag specifique

```bash
cargo build -p jayfestival --features kindmother-only --no-default-features
cargo build -p jayfestival --features db-encryption
```

---

## Dependances communes

### Ecosystem Rust

| Crate | Version | Usage |
|-------|---------|-------|
| `serde` | 1 (+derive) | Serialisation JSON/TOML |
| `serde_json` | 1 | JSON |
| `rusqlite` | 0.31+ (+bundled) | SQLite |
| `uuid` | 1 (+v4) | Identifiants |
| `chrono` | 0.4 | Dates/heures |
| `tokio` | 1 (+full) | Async runtime |
| `thiserror` | 1-2 | Derive Error |
| `anyhow` | 1 | Error catch-all (apps) |

### Crypto/Security

| Crate | Usage |
|-------|-------|
| `sha2` | Hashing SHA-256 |
| `hmac` | HMAC pour tokens |
| `argon2` | Hash mots de passe (OWASP 2024) |
| `ed25519-dalek` | Signatures |
| `rand` | RNG cryptographique |
| `rustls` + `tokio-rustls` | TLS 1.3 |
| `webpki-roots` | Certificats racine |

### UI/Desktop

| Crate | Usage |
|-------|-------|
| `dioxus` | 0.6 — Framework UI |
| `rodio` | Audio (Miou sounds) |

---

## Metadata MSCM par crate

Chaque Cargo.toml inclut des metadonnees semantiques :

```toml
[package.metadata]
"@id" = "miyukini.tools.miyusql.v1"
"@role" = "outil"            # outil | core | operateur | app
"@layer" = "toolkit"         # kernel | core | toolkit | service | app
"@domain" = "sql"            # Domaine metier
"@human" = "Execution SQL gouvernee"
"@do" = "Execute des requetes SQL sous mandat"
```

**Roles valides** : `kernel`, `core`, `outil`, `toolkit`, `operateur`, `service`, `app`

---

## Structure standard d'un crate

```
mon-crate/
├── Cargo.toml
├── src/
│   ├── lib.rs           # Point d'entree, re-exports publics
│   ├── admin_cell.rs    # AdminCell (init unique)
│   ├── context.rs       # Contexte d'execution
│   ├── errors.rs        # Types d'erreur
│   ├── data/            # Couche persistance
│   │   ├── mod.rs       # Feature-gate sqlite/kindmother
│   │   ├── sqlite_impl.rs
│   │   └── kindmother_impl.rs
│   ├── auth/            # Authentification
│   │   ├── mod.rs
│   │   └── password.rs
│   ├── services/        # Logique metier
│   └── domain.rs        # Types domaine + Result aliases
```

---

## Ajouter un nouveau crate

1. Creer le dossier `crates/mon-crate/`
2. Ajouter a `Cargo.toml` workspace members
3. Configurer le Cargo.toml du crate :

```toml
[package]
name = "mon-crate"
version = "0.1.0"
edition = "2021"

[package.metadata]
"@id" = "miyukini.outils.mon-crate.v1"
"@role" = "outil"
"@layer" = "toolkit"
"@domain" = "mon-domaine"
"@human" = "Description en francais"
"@do" = "Fait quelque chose de specifique"

[dependencies]
miyukini-kernel = { path = "../miyukini-kernel" }

[lints]
workspace = true
```

4. Creer `src/lib.rs` avec modules standard
5. Si service: ajouter feature flags dans `[features]`

---

## Node.js / pnpm workspace

```json
// package.json
{
  "name": "miyukini-cog",
  "private": true,
  "devDependencies": { "@anthropic-ai/turbo": "^2.0.0" },
  "scripts": {
    "dev": "turbo dev",
    "build": "turbo build",
    "tauri:dev": "turbo tauri:dev",
    "tauri:build": "turbo tauri:build"
  }
}
```

```yaml
# pnpm-workspace.yaml
packages:
  - 'apps/central'
  - 'apps/origin'
```

**pnpm** >= 9.0.0 requis.

---

## Commandes de build

```bash
# Build complet
cargo build

# Build un service specifique
cargo build -p jayfestival

# Build avec feature flag
cargo build -p jayxpose --features kindmother-only --no-default-features

# Clippy
cargo clippy --workspace

# Check rapide
cargo check -p miyukini-central

# App desktop (Dioxus)
cd apps/central && dx serve

# Origin (serveur MWS)
cargo run -p miyukini-origin
```

---

## Erreurs de build frequentes

| Erreur | Cause | Solution |
|--------|-------|----------|
| `unresolved import` | Crate pas dans workspace members | Ajouter dans root Cargo.toml |
| `feature flag conflict` | legacy-sqlite + kindmother-only | Un seul a la fois (--no-default-features) |
| `rusqlite link error` | SQLCipher pas compile | Utiliser `bundled-sqlcipher` feature |
| `tokio not found` | kindmother-only sans tokio | Verifier dep:tokio dans features |
| `dioxus version mismatch` | Deps incompatibles | Aligner sur 0.6.x |
