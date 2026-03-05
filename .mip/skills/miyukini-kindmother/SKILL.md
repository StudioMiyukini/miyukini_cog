---
name: miyukini-kindmother
description: Ecosysteme complet KindMother - le systeme de persistance exclusif de Miyukini COG. Couvre les 5 crates (kindmother, kindmother-client, kindmother-service, kindmother-db-adapter, kindmother-db-key), l'architecture Mother-Daughter, le protocole TCP/JSON, SQLCipher, l'audit, les feature flags (legacy-sqlite, kindmother-only, db-encryption), et les patterns d'acces aux donnees. Utiliser quand on travaille sur la persistance, les bases de donnees, les migrations, l'encryption at rest, le protocole IPC KindMother, ou quand on ajoute l'acces donnees a un service.
---

# Ecosysteme KindMother — Persistance souveraine

KindMother est l'autorite exclusive sur toute persistance de donnees dans Miyukini COG. Aucun autre composant ne peut acceder directement a une base de donnees.

## Architecture globale

```
Service (Operateur)
    |
    v
kindmother-client (TCP/JSON)
    |
    v
kindmother-service (serveur isole, acces exclusif)
    |
    v
SQLite/SQLCipher (base chiffree)
    |
    kindmother-db-key (derivation AES-256 via Argon2id)
    kindmother-db-adapter (utilitaires partages)
```

---

## 1. kindmother (crates/kindmother) — Core de persistance

### Architecture Mother-Daughter

```rust
pub enum InstanceType { Mother, Daughter }

pub struct InstanceIdentity {
    pub id: Id,              // UUID genere par le Kernel
    pub instance_type: InstanceType,
}

pub struct InstanceState {
    pub identity: InstanceIdentity,
    pub created_at: SystemTime,
}

impl InstanceState {
    pub fn is_mother(&self) -> bool
    pub fn is_daughter(&self) -> bool
}
```

- **Mother** : Instance source de verite (ecriture primaire)
- **Daughter** : Instance repliquee (synchronisation delta)

### Abstraction Storage (storage.rs)

```rust
pub trait Storage {
    fn read(&self, instance: &InstanceIdentity, entity_id: &str) -> Option<Vec<u8>>;
    fn write(&mut self, instance: &InstanceIdentity, entity_id: &str, data: Vec<u8>) -> Result<(), StorageError>;
    fn delete(&mut self, instance: &InstanceIdentity, entity_id: &str) -> Result<(), StorageError>;
}

pub enum StorageError { NotFound, Corruption, Io(String) }

// Implementation de test in-memory
pub struct MemoryStorage(HashMap<String, HashMap<String, Vec<u8>>>);
// Structure: instance_id -> entity_id -> data binaire
```

### Synchronisation delta (sync.rs)

```rust
pub struct SyncDelta {
    pub entity_id: String,
    pub operation: DeltaOperation,
    pub data: Option<Vec<u8>>,
}

pub enum DeltaOperation { Create, Update, Delete }

pub trait Sync {
    fn compute_delta(&self, source: &InstanceIdentity, target: &InstanceIdentity)
        -> Result<Vec<SyncDelta>, SyncError>;
    fn apply_delta(&mut self, target: &InstanceIdentity, delta: Vec<SyncDelta>)
        -> Result<(), SyncError>;
}

pub enum SyncError { Conflict(String), InvalidDelta(String), Io(String) }
```

### API CoreData (api.rs)

```rust
pub struct WriteIntent {
    pub id: Id,
    pub entity_id: String,
    pub operation: WriteOperation,
    pub data: Option<Vec<u8>>,
}

pub enum WriteOperation { Create, Update, Delete }

pub trait CoreDataAPI {
    fn read(&self, instance: &InstanceIdentity, entity_id: &str)
        -> Result<Option<Vec<u8>>, APIError>;
    fn submit_write_intent(&mut self, instance: &InstanceIdentity, intent: WriteIntent)
        -> Result<(), APIError>;
}

pub enum APIError { NotFound, PermissionDenied(String), InvalidIntent(String) }
```

### Modules supplementaires

- **threat.rs** : `ThreatDetector` trait pour detection de corruption
- **observability.rs** : Metriques et sante
- **miyusql_bridge.rs** : `MiyuSQLExecutionBridge` trait pour delegation SQL

---

## 2. kindmother-client (crates/kindmother-client) — Client TCP/JSON

### Protocole de connexion

```rust
pub struct KindMotherClient { /* connexion async TCP */ }

impl KindMotherClient {
    pub async fn connect(addr: &str, operator_id: &str, password: &str)
        -> Result<Self, ClientError>;

    pub async fn query(&self, sql: &str, params: Vec<Value>)
        -> Result<Vec<Row>, ClientError>;

    pub async fn execute(&self, sql: &str, params: Vec<Value>, operation_id: &str)
        -> Result<(), ClientError>;
}
```

### Format du protocole

```
[4 bytes BE: longueur du message JSON]
[JSON payload]
```

### Types de requete

```rust
pub struct Request {
    pub operator_id: String,
    pub operation: Operation,
}

pub enum Operation {
    Query { sql: String, params: Vec<Value> },
    Execute { sql: String, params: Vec<Value>, operation_id: String },
}

pub struct Response {
    pub success: bool,
    pub data: Option<Vec<Row>>,
    pub error: Option<String>,
}
```

### Erreurs client

```rust
pub enum ClientError {
    ConnectionFailed(String),
    Timeout,
    PermissionDenied(String),
    QueryFailed(String),
    ProtocolError(String),
}
```

**Timeout connexion** : 10 secondes

---

## 3. kindmother-service (crates/kindmother-service) — Serveur isole

### Composants

```rust
// Base de donnees chiffree
pub struct EncryptedDatabase { /* SQLCipher via rusqlite */ }

// Moteur d'arbitrage
pub struct ArbitrationEngine { /* controle d'acces par operateur */ }

// Serveur TCP/JSON
pub struct KindMotherServer { /* ecoute sur localhost */ }

// Protocole
pub struct Protocol { /* serialisation Request/Response */ }

// Erreurs
pub enum ServiceError { /* erreurs du service */ }
```

### Configuration SQLite

```sql
PRAGMA foreign_keys = ON;
PRAGMA journal_mode = WAL;
PRAGMA synchronous = NORMAL;
```

### Table d'audit

```sql
CREATE TABLE _kindmother_audit (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp TEXT NOT NULL,
    operator_id TEXT NOT NULL,
    operation TEXT NOT NULL,       -- 'query' ou 'execute'
    intent TEXT,
    sql_hash TEXT NOT NULL,        -- Hash SHA-256 du SQL (pas le SQL brut)
    rows_affected INTEGER,
    created_at TEXT DEFAULT (datetime('now'))
);
```

Le hash protege les donnees sensibles dans la piste d'audit.

### Permissions fichiers

```
Dossier base : 0o700 (rwx------)
Fichier .db  : 0o600 (rw-------)
```

---

## 4. kindmother-db-adapter (crates/kindmother-db-adapter) — Utilitaires

Crate utilitaire pure, utilise par tous les services.

### Erreur generique

```rust
pub struct DbError {
    pub service_name: String,
    pub message: String,
}

// Macro pour erreurs specifiques a un service
macro_rules! define_db_error {
    ($name:ident, $service:expr) => { /* ... */ }
}
```

### Conversions Bool/Int (SQLite n'a pas de bool)

```rust
pub fn int_to_bool(val: i64) -> bool           // 0 = false, autre = true
pub fn bool_to_int(val: bool) -> i64           // true = 1, false = 0
pub fn optional_int_to_bool(val: Option<i64>) -> Option<bool>
pub fn optional_bool_to_int(val: Option<bool>) -> Option<i64>
```

### Timestamps

```rust
pub fn now_rfc3339() -> String         // UTC RFC 3339
pub fn now_local_iso() -> String       // ISO local
pub fn current_month_key() -> String   // "2026-02" pour cles mensuelles
```

### UUIDs

```rust
pub fn new_uuid() -> String            // UUID v4
pub fn ensure_uuid(id: &str) -> String // Retourne l'id ou genere un nouveau
```

### Mots de passe

```rust
pub fn hash_password(password: &str) -> String
// Format: "argon2id$base64(salt)$base64(hash)"
// Algorithme: Argon2id (OWASP 2024)

pub fn verify_password(stored: &str, password: &str) -> bool
// Supporte: Argon2id (moderne) + SHA-256 (legacy)
// Comparaison en temps constant
```

---

## 5. kindmother-db-key (crates/kindmother-db-key) — Derivation de cle

### Encryption at rest

```rust
pub struct KeyDerivation { /* secret d'installation */ }
pub enum DbKeyError { /* erreurs de derivation */ }
```

- **Algorithme** : Argon2id (OWASP 2024)
- **Cle** : AES-256 pour SQLCipher
- **Secret** : Stocke localement sur l'installation

---

## Feature Flags — Pattern d'acces donnees

Chaque service utilise ces feature flags dans son Cargo.toml :

```toml
[features]
default = ["legacy-sqlite"]
legacy-sqlite = ["rusqlite"]
kindmother-only = ["dep:kindmother-client", "dep:tokio"]
db-encryption = ["dep:kindmother-db-key", "rusqlite/bundled-sqlcipher"]
```

### Pattern dans data/mod.rs

```rust
#[cfg(feature = "legacy-sqlite")]
mod sqlite_impl;

#[cfg(feature = "kindmother-only")]
mod kindmother_impl;

#[cfg(feature = "legacy-sqlite")]
pub use sqlite_impl::*;

#[cfg(feature = "kindmother-only")]
pub use kindmother_impl::*;
```

### Quand utiliser quel flag

| Flag | Usage | Base |
|------|-------|------|
| `legacy-sqlite` (defaut) | Dev local, prototypage rapide | SQLite direct via rusqlite |
| `kindmother-only` | Production, securite stricte | Via kindmother-client TCP |
| `db-encryption` | Donnees sensibles | SQLCipher (AES-256) |

---

## Conventions CRUD

Toutes les methodes d'acces suivent cette nomenclature :

```rust
// Lecture
fn get_{entity}(id) -> Result<Option<Entity>>
fn list_{entities}(filters) -> Result<Vec<Entity>>
fn count_{entities}(filters) -> Result<i64>
fn search_{entities}(query) -> Result<Vec<Entity>>
fn exists_{entity}(id) -> Result<bool>

// Ecriture
fn create_{entity}(data) -> Result<Entity>
fn update_{entity}(id, data) -> Result<Entity>
fn delete_{entity}(id) -> Result<()>
fn upsert_{entity}(data) -> Result<Entity>
fn archive_{entity}(id) -> Result<()>

// Relations
fn get_{entity}_by_{relation}(relation_id) -> Result<Vec<Entity>>
fn add_{entity}_to_{parent}(entity_id, parent_id) -> Result<()>
fn remove_{entity}_from_{parent}(entity_id, parent_id) -> Result<()>
```

---

## Flux typique d'acces aux donnees

```
1. Service recoit une requete utilisateur
2. Service verifie l'authentification (auth/)
3. Service construit un WriteIntent ou une query
4. [legacy-sqlite] -> rusqlite directe
   [kindmother-only] -> KindMotherClient.query/execute()
5. KindMotherService verifie les permissions (ArbitrationEngine)
6. KindMotherService execute sur SQLite/SQLCipher
7. Audit enregistre dans _kindmother_audit
8. Reponse retournee au service
```
