---
name: miyukini-services
description: Pattern standard des services Miyukini COG (Jay*, Miyukini*). Structure modules (data, auth, services, export, domain), feature flags (legacy-sqlite, kindmother-only, db-encryption), types de donnees, adaptateurs inter-services. Utiliser quand on cree ou modifie un service, quand on ajoute un module data/ ou auth/, quand on travaille sur les adaptateurs inter-services ou les exports.
---

# Services Miyukini — Patterns standards

## Structure standard d'un service

```
crates/{service_name}/
├── Cargo.toml
└── src/
    ├── lib.rs            # Point d'entree, expose les modules
    ├── data/
    │   ├── mod.rs        # Feature flags, re-exports
    │   ├── types.rs      # Types de domaine
    │   ├── kindmother_db.rs        # SQLite direct (legacy-sqlite)
    │   └── kindmother_client_db.rs # Client KindMother (kindmother-only)
    ├── auth/             # Optionnel
    │   ├── mod.rs        # sign_in, sign_up, sign_out
    │   └── permissions.rs # RLS, UserType
    ├── services/         # Optionnel — adaptateurs inter-services
    │   ├── mod.rs
    │   └── {autre_service}/
    │       ├── adapter.rs  # Synchronisation (lecture reflechie)
    │       ├── client.rs   # Appels directs
    │       └── contract.rs # Types de contrat (payloads, filtres)
    ├── export/           # Optionnel
    │   ├── mod.rs
    │   └── ical.rs       # Export iCalendar RFC 5545
    └── domain/           # Optionnel — logique metier
        └── mod.rs
```

## Module `data/mod.rs` (feature flags)

```rust
mod types;

#[cfg(feature = "legacy-sqlite")]
mod kindmother_db;

#[cfg(feature = "kindmother-only")]
mod kindmother_client_db;

pub use types::*;
#[cfg(feature = "legacy-sqlite")]
pub use kindmother_db::{DbError, ServiceDb};
#[cfg(feature = "kindmother-only")]
pub use kindmother_client_db::{DbError, ServiceDb};
```

## Module `auth/`

Types standard :

```rust
pub type AuthResult<T> = Result<T, AuthError>;

pub struct AuthError(pub String);
pub struct AuthSession {
    pub user_id: String,
    pub email: String,
    pub access_token: String,
    pub profile: Profile,
}
```

Fonctions publiques :

```rust
pub fn auth_sign_in(db: &ServiceDb, email: &str, password: &str) -> AuthResult<AuthSession>
pub fn auth_sign_up(db: &ServiceDb, email: &str, password: &str, ...) -> AuthResult<AuthSession>
pub fn auth_sign_out() -> AuthResult<()>
```

Permissions (RLS) :

```rust
pub enum UserType { Admin, Manager, Member, Visitor, Unknown }
pub fn auth_user_type_from_profile(profile: &Profile) -> UserType
pub fn auth_can_access(profile: &Profile, resource_id: &str) -> bool
```

## Adaptateurs inter-services

Pattern de **lecture reflechie** : synchronise les donnees d'un service externe en creant des "reflets" locaux (lecture seule).

```rust
pub struct ServiceAdapter {
    db: Arc<ServiceDb>,
}

impl ServiceAdapter {
    pub fn sync_entries(&self, source_db: &SourceDb) -> Result<usize, Error> {
        let entries = source_db.list_entries()?;
        for entry in entries {
            self.db.upsert_reflection(&entry)?;
        }
        Ok(entries.len())
    }
}
```

## Methodes CRUD standard

Convention de nommage dans `ServiceDb` :

| Methode | Pattern |
|---------|---------|
| `{entity}_list()` | Liste toutes les entites |
| `{entity}_by_id(id)` | Recherche par ID |
| `{entity}_create(data)` | Creation |
| `{entity}_update(data)` | Mise a jour |
| `{entity}_delete(id)` | Suppression |

## Types de donnees (`types.rs`)

Convention :

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub id: String,              // UUID v4
    pub name: String,
    pub optional_field: Option<String>,
    pub created_at: String,      // ISO 8601
    pub updated_at: String,
}
```

## lib.rs standard

```rust
//! Service {Nom} — Description courte.

pub mod data;
pub mod auth;        // Si necessaire
pub mod services;    // Si necessaire
pub mod export;      // Si necessaire

pub use data::ServiceDb;
```

## Regles

1. **Feature flags** : `legacy-sqlite` / `kindmother-only` / `db-encryption`
2. **Identite** : `InstanceType::Daughter` pour les DB locales
3. **Erreurs** : `DbError(String)` par service, `AuthError(String)` pour auth
4. **Timestamps** : ISO 8601 (`chrono::Utc::now().to_rfc3339()`)
5. **UUIDs** : `uuid::Uuid::new_v4().to_string()` pour IDs primaires
6. **Thread-safety** : `Mutex<Connection>` pour acces synchrone
7. **Adaptateurs** : lecture seule stricte (pas de modification des donnees externes)

## Services existants

| Service | Type | Crate |
|---------|------|-------|
| JayFestival | Gestion d'evenements | `crates/jayfestival` |
| JayKoa | Agenda/calendrier | `crates/jaykoa` |
| JayXpose | Portfolio/vitrine | `crates/jayxpose` |
| JayKonta | Comptabilite | `crates/jaykonta` |
| JayShop | Boutique | `crates/jayshop` |
| MiyukiniWatch | Metriques silencieuses | `crates/miyukiniwatch` |
| Miyukini Central | Hub de gestion | `crates/miyukini-central` |

## References

- **Crates services** : `crates/jay*`, `crates/miyukini-central/`, `crates/miyukiniwatch/`
- **Documentation** : `docs/services/Jay*/`, `docs/services/MiyukiniCentral/`
