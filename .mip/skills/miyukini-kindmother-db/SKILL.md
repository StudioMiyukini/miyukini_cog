---
name: miyukini-kindmother-db
description: Pattern KindMother pour la persistance des donnees (Core Strate 4). Couvre kindmother (core), kindmother-service (serveur libSQL isole), kindmother-client (IPC), kindmother-db-key (chiffrement SQLCipher), kindmother-db-adapter (utilitaires). Utiliser quand on cree ou modifie un module data/, quand on ajoute un service avec persistance, quand on travaille sur le chiffrement DB, ou quand on manipule InstanceIdentity/InstanceType.
---

# KindMother DB — Persistance gouvernee

## Principe

KindMother est le Core de donnees et persistance (Strate 4). Toute persistance dans un service passe par KindMother. Deux patterns existent :

1. **SQLite direct** (feature `legacy-sqlite`) — acces `rusqlite` avec chiffrement SQLCipher
2. **KindMother Client** (feature `kindmother-only`) — delegation IPC via `kindmother-service`

## Crates

| Crate | Role | Strate |
|-------|------|--------|
| `kindmother` | Core : types fondamentaux (InstanceIdentity, InstanceType) | 4 |
| `kindmother-service` | Serveur libSQL isole (processus separe) | 4 |
| `kindmother-client` | Client delegation IPC (TCP/JSON) | 4 |
| `kindmother-db-key` | Derivation cle SQLCipher (Argon2id) | 4 |
| `kindmother-db-adapter` | Utilitaires communs (timestamps, UUID, hash, macro) | 6 |

## Types cles

```rust
pub enum InstanceType {
    Mother,   // DB mere — source de verite unique
    Daughter, // DB fille — instance locale offline-first
}

pub struct InstanceIdentity {
    pub id: Id,
    pub instance_type: InstanceType,
}
```

**Regle :** Les services utilisent toujours `InstanceType::Daughter`.

## Pattern SQLite direct (kindmother_db.rs)

Structure standard dans un service :

```rust
pub struct ServiceDb {
    conn: Mutex<Connection>,
    pub instance: InstanceIdentity,
}

impl ServiceDb {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, DbError> {
        let conn = Connection::open(path)?;

        #[cfg(feature = "db-encryption")]
        {
            let kd = KeyDerivation::new(data_dir)?;
            let pragma_key = kd.pragma_key_hex(db_name)?;
            conn.pragma_update(None, "key", &pragma_key)?;
        }

        let instance = InstanceIdentity::new(InstanceType::Daughter);
        let db = Self { conn: Mutex::new(conn), instance };
        db.init_schema()?;
        Ok(db)
    }

    fn init_schema(&self) -> Result<(), DbError> {
        let conn = self.conn.lock()?;
        conn.execute_batch("CREATE TABLE IF NOT EXISTS ...")?;
        Ok(())
    }
}
```

## Pattern KindMother Client (kindmother_client_db.rs)

```rust
pub struct ServiceDb {
    client: Arc<KindMotherClient>,
}

impl ServiceDb {
    pub fn open(_path: impl AsRef<Path>) -> Result<Self, DbError> {
        if CLIENT.get().is_none() {
            Self::init_global_sync(None)?;
        }
        let db = Self::new()?;
        db.init_schema_sync()?;
        Ok(db)
    }
}
```

## Chiffrement (kindmother-db-key)

Derivation Argon2id : `secret_installation + machine_id + nom_base` → cle AES-256 → pragma SQLCipher.

```rust
let kd = KeyDerivation::new(data_dir)?;       // Lit .kindmother_secret
let pragma_key = kd.pragma_key_hex("service")?; // x'<hex>'
conn.pragma_update(None, "key", &pragma_key)?;
```

## Utilitaires (kindmother-db-adapter)

| Fonction | Usage |
|----------|-------|
| `int_to_bool(val)` / `bool_to_int(val)` | Conversion SQLite |
| `now_rfc3339()` / `now_local_iso()` | Timestamps |
| `new_uuid()` / `ensure_uuid(opt)` | Generation UUID v4 |
| `hash_password(pwd)` | Argon2id hash |
| `verify_password(stored, pwd)` | Verification (legacy SHA-256 + Argon2id) |
| `define_db_error!(Name)` | Macro erreur DB par service |

## Type d'erreur standard

```rust
pub struct DbError(pub String);
impl From<rusqlite::Error> for DbError { ... }
```

Ou via macro : `define_db_error!(JayFestival);`

## Flux de creation DB

```
ServiceDb::open(path)
  → Connection::open(path)
  → [Si db-encryption] KeyDerivation → pragma_key → SQLCipher
  → InstanceIdentity::new(Daughter)
  → init_schema() → CREATE TABLE IF NOT EXISTS
  → Ok(ServiceDb)
```

## Feature flags

| Feature | Description |
|---------|-------------|
| `legacy-sqlite` | Acces SQLite direct (rusqlite) |
| `kindmother-only` | Delegation via KindMother Client |
| `db-encryption` | Chiffrement SQLCipher |

## Regles

1. **Isolation** : seul KindMother Service accede aux fichiers DB
2. **Chiffrement** : SQLCipher avec derivation par base (Argon2id)
3. **Identite** : chaque DB a une `InstanceIdentity` (Daughter)
4. **Audit** : operations tracees dans `_kindmother_audit`
5. **Thread-safety** : `Mutex<Connection>` pour acces synchrone
6. **Migrations** : `CREATE TABLE IF NOT EXISTS` + `ALTER TABLE ADD COLUMN` tolerant

## References

- **Crates** : `crates/kindmother/`, `crates/kindmother-service/`, `crates/kindmother-client/`, `crates/kindmother-db-key/`, `crates/kindmother-db-adapter/`
- **Documentation** : `docs/core/KindMother/`, `docs/reference/DB - Chiffrement SQLCipher Setup.md`
