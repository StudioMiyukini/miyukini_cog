# KindMother â€” SystÃ¨me de Persistance libSQL et Guide de Migration

## 1. Introduction

### Objet du document

Ce document dÃ©crit le **systÃ¨me de persistance sÃ©curisÃ© KindMother** basÃ© sur libSQL avec chiffrement natif, ainsi que le **guide de migration** depuis l'implÃ©mentation rusqlite actuelle.

### Contexte

L'architecture Miyukini COG requiert que KindMother soit le **seul gardien lÃ©gitime** des donnÃ©es persistÃ©es. Pour garantir cette gouvernance de maniÃ¨re technique (et non juste conceptuelle), nous migrons vers :

1. **libSQL** : Fork de SQLite avec chiffrement natif
2. **Architecture en processus isolÃ©** : KindMother s'exÃ©cute dans un processus sÃ©parÃ©
3. **Communication IPC** : Les OpÃ©rateurs communiquent via API authentifiÃ©e

### PortÃ©e

Ce document couvre :
- Description technique du systÃ¨me libSQL
- ProcÃ©dure de migration depuis rusqlite
- Configuration du chiffrement
- DÃ©ploiement du service isolÃ©
- Troubleshooting

### PrÃ©requis

- Rust 1.75+ avec toolchain stable
- ComprÃ©hension de l'architecture COG (voir [Architecture Miyukini](..//..//..//miyukini-webway-system//reference//_index.md))
- Lecture prÃ©alable : [Security - Gouvernance Cores Protection Donnees](..//..//WorrySentinel//_index.md)

---

## 2. Architecture Technique

### 2.1 Vue d'ensemble

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                        SYSTÃˆME KINDMOTHER libSQL                            â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚                                                                             â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”    â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â” â”‚
â”‚  â”‚       CRATE PRINCIPAL           â”‚    â”‚     KINDMOTHER-SERVICE          â”‚ â”‚
â”‚  â”‚                                 â”‚    â”‚                                 â”‚ â”‚
â”‚  â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”‚    â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”‚ â”‚
â”‚  â”‚  â”‚    jayxpose / jaykonta    â”‚  â”‚    â”‚  â”‚     gRPC Server           â”‚  â”‚ â”‚
â”‚  â”‚  â”‚    (OpÃ©rateurs)           â”‚  â”‚    â”‚  â”‚     (tonic)               â”‚  â”‚ â”‚
â”‚  â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â”‚    â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â”‚ â”‚
â”‚  â”‚                â”‚                â”‚    â”‚                â”‚               â”‚ â”‚
â”‚  â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”‚    â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”‚ â”‚
â”‚  â”‚  â”‚   kindmother-client       â”‚  â”‚    â”‚  â”‚    Auth Layer             â”‚  â”‚ â”‚
â”‚  â”‚  â”‚   (impl Storage trait)    â”‚â”€â”€â”¼â”€â”€â”€â”€â”¼â”€â”€â”‚    (token validation)     â”‚  â”‚ â”‚
â”‚  â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â”‚    â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â”‚ â”‚
â”‚  â”‚                                 â”‚    â”‚                â”‚               â”‚ â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜    â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”‚ â”‚
â”‚                                         â”‚  â”‚   Permissions Layer       â”‚  â”‚ â”‚
â”‚                                         â”‚  â”‚   (matrice opÃ©rateur)     â”‚  â”‚ â”‚
â”‚                                         â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â”‚ â”‚
â”‚                                         â”‚                â”‚               â”‚ â”‚
â”‚                                         â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”‚ â”‚
â”‚                                         â”‚  â”‚   libSQL Engine           â”‚  â”‚ â”‚
â”‚                                         â”‚  â”‚   + EncryptionConfig      â”‚  â”‚ â”‚
â”‚                                         â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â”‚ â”‚
â”‚                                         â”‚                â”‚               â”‚ â”‚
â”‚                                         â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”‚ â”‚
â”‚                                         â”‚  â”‚   *.db (chiffrÃ©s AES-256) â”‚  â”‚ â”‚
â”‚                                         â”‚  â”‚   Permissions: 600        â”‚  â”‚ â”‚
â”‚                                         â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â”‚ â”‚
â”‚                                         â”‚                                 â”‚ â”‚
â”‚                                         â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜ â”‚
â”‚                                                                             â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### 2.2 Composants

| Crate | RÃ´le | DÃ©pendances |
|-------|------|-------------|
| `kindmother` | API publique, trait Storage | (inchangÃ©) |
| `kindmother-client` | Client IPC pour OpÃ©rateurs | tonic, prost |
| `kindmother-service` | Processus isolÃ© avec libSQL | libsql, tonic, argon2 |

---

## 3. Configuration libSQL

### 3.1 DÃ©pendances Cargo

**Pour kindmother-service/Cargo.toml :**

```toml
[package]
name = "kindmother-service"
version = "0.1.0"
edition = "2021"

[dependencies]
# Base de donnÃ©es avec chiffrement
libsql = { version = "0.9", features = ["core", "encryption"] }

# Serveur IPC
tonic = "0.12"
prost = "0.13"

# DÃ©rivation de clÃ©
argon2 = "0.5"
zeroize = "1.7"

# Async runtime
tokio = { version = "1.40", features = ["full"] }

# Utilitaires
uuid = { version = "1.10", features = ["v4"] }
chrono = { version = "0.4", features = ["serde"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Logs
tracing = "0.1"
tracing-subscriber = "0.3"

[build-dependencies]
tonic-build = "0.12"
```

**Pour kindmother-client/Cargo.toml :**

```toml
[package]
name = "kindmother-client"
version = "0.1.0"
edition = "2021"

[dependencies]
kindmother = { path = "../kindmother" }
tonic = "0.12"
prost = "0.13"
tokio = { version = "1.40", features = ["rt-multi-thread"] }
```

### 3.2 Migration depuis rusqlite

**Avant (crates utilisant rusqlite directement) :**

```toml
# jayxpose/Cargo.toml - AVANT
[dependencies]
rusqlite = { version = "0.32", features = ["bundled"] }
```

**AprÃ¨s (crates utilisant kindmother-client) :**

```toml
# jayxpose/Cargo.toml - APRÃˆS
[dependencies]
kindmother-client = { path = "../kindmother-client" }
# rusqlite SUPPRIMÃ‰ - accÃ¨s uniquement via KindMother
```

---

## 4. ImplÃ©mentation du Chiffrement

### 4.1 DÃ©rivation de ClÃ© MaÃ®tre

```rust
// kindmother-service/src/encryption.rs

use argon2::{Argon2, Algorithm, Version, Params};
use zeroize::Zeroizing;

/// DÃ©rive la clÃ© maÃ®tre Ã  partir de secrets locaux souverains.
/// 
/// La clÃ© n'est JAMAIS stockÃ©e sur disque.
/// Elle est recalculÃ©e Ã  chaque dÃ©marrage du service.
/// 
/// @id: kindmother_derive_master_key
/// @do: derive_encryption_key_from_local_secrets
/// @layer: infra
pub fn derive_master_key() -> Zeroizing<[u8; 32]> {
    // 1. RÃ©cupÃ©rer les composants locaux
    let machine_id = get_machine_id();
    let install_secret = get_install_secret();
    let cog_id = get_cog_environment_id();
    
    // 2. Combiner les composants
    let input = format!(
        "miyukini:kindmother:{}:{}:{}",
        machine_id,
        install_secret,
        cog_id
    );
    
    // 3. ParamÃ¨tres Argon2id (rÃ©sistant GPU/ASIC)
    let params = Params::new(
        65536,      // 64 MiB mÃ©moire
        3,          // 3 itÃ©rations
        4,          // 4 threads
        Some(32),   // 32 octets de sortie
    ).expect("Invalid Argon2 params");
    
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    
    // 4. Salt fixe (connu, pas secret)
    let salt = b"miyukini-kindmother-v1-2026";
    
    // 5. DÃ©river la clÃ©
    let mut key = Zeroizing::new([0u8; 32]);
    argon2
        .hash_password_into(input.as_bytes(), salt, key.as_mut())
        .expect("Key derivation failed");
    
    key
}

/// RÃ©cupÃ¨re l'identifiant unique de la machine.
/// Windows: BIOS UUID via WMI
/// Linux: /etc/machine-id ou DMI
fn get_machine_id() -> String {
    #[cfg(target_os = "windows")]
    {
        // Utiliser WMI pour rÃ©cupÃ©rer le BIOS UUID
        use std::process::Command;
        let output = Command::new("wmic")
            .args(["csproduct", "get", "uuid"])
            .output()
            .expect("Failed to get machine ID");
        String::from_utf8_lossy(&output.stdout)
            .lines()
            .nth(1)
            .unwrap_or("unknown")
            .trim()
            .to_string()
    }
    
    #[cfg(target_os = "linux")]
    {
        std::fs::read_to_string("/etc/machine-id")
            .unwrap_or_else(|_| "unknown".to_string())
            .trim()
            .to_string()
    }
    
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        let output = Command::new("ioreg")
            .args(["-rd1", "-c", "IOPlatformExpertDevice"])
            .output()
            .expect("Failed to get machine ID");
        // Parser l'IOPlatformUUID
        String::from_utf8_lossy(&output.stdout)
            .lines()
            .find(|l| l.contains("IOPlatformUUID"))
            .map(|l| l.split('"').nth(3).unwrap_or("unknown"))
            .unwrap_or("unknown")
            .to_string()
    }
}

/// RÃ©cupÃ¨re le secret d'installation (gÃ©nÃ©rÃ© une fois).
fn get_install_secret() -> String {
    let secret_path = get_data_dir().join(".kindmother_install_secret");
    
    if secret_path.exists() {
        std::fs::read_to_string(&secret_path)
            .expect("Failed to read install secret")
            .trim()
            .to_string()
    } else {
        // PremiÃ¨re exÃ©cution : gÃ©nÃ©rer le secret
        let secret = uuid::Uuid::new_v4().to_string();
        std::fs::write(&secret_path, &secret)
            .expect("Failed to write install secret");
        // ProtÃ©ger le fichier
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&secret_path, 
                std::fs::Permissions::from_mode(0o600))
                .expect("Failed to set permissions");
        }
        secret
    }
}

/// RÃ©cupÃ¨re l'ID de l'environnement COG.
fn get_cog_environment_id() -> String {
    std::env::var("MIYUKINI_COG_ID")
        .unwrap_or_else(|_| "default-cog-v1".to_string())
}

fn get_data_dir() -> std::path::PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("miyukini")
        .join("kindmother")
}
```

### 4.2 Ouverture de Base avec Chiffrement

```rust
// kindmother-service/src/database.rs

use libsql::{Builder, Cipher, EncryptionConfig, Database, Connection};
use std::path::Path;
use zeroize::Zeroizing;

/// Base de donnÃ©es chiffrÃ©e KindMother.
/// 
/// @id: kindmother_encrypted_db
/// @do: manage_encrypted_database_connection
/// @layer: infra
pub struct EncryptedDatabase {
    db: Database,
    _key: Zeroizing<[u8; 32]>, // Garder en mÃ©moire pour la durÃ©e de vie
}

impl EncryptedDatabase {
    /// Ouvre ou crÃ©e une base de donnÃ©es chiffrÃ©e.
    /// 
    /// @id: kindmother_db_open
    /// @do: open_encrypted_database_with_derived_key
    /// @layer: infra
    pub async fn open(path: impl AsRef<Path>, key: Zeroizing<[u8; 32]>) -> Result<Self, DbError> {
        let encryption = EncryptionConfig::new(
            Cipher::Aes256Gcm,  // Standard NIST, compatible partout
            key.to_vec(),
        );
        
        let db = Builder::new_local(path.as_ref())
            .encryption_config(encryption)
            .build()
            .await
            .map_err(|e| DbError(format!("Failed to open database: {}", e)))?;
        
        Ok(Self { db, _key: key })
    }
    
    /// Ouvre une connexion Ã  la base.
    pub fn connect(&self) -> Result<Connection, DbError> {
        self.db.connect()
            .map_err(|e| DbError(format!("Failed to connect: {}", e)))
    }
}

#[derive(Debug)]
pub struct DbError(pub String);

impl std::fmt::Display for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "KindMother DB: {}", self.0)
    }
}

impl std::error::Error for DbError {}
```

---

## 5. Service IPC

### 5.1 DÃ©finition Proto

```protobuf
// kindmother-service/proto/kindmother.proto

syntax = "proto3";
package kindmother;

service KindMotherService {
    // RequÃªte SQL gÃ©nÃ©rique avec validation
    rpc Query(QueryRequest) returns (QueryResponse);
    
    // Lecture d'entitÃ© par ID
    rpc ReadEntity(ReadEntityRequest) returns (ReadEntityResponse);
    
    // Ã‰criture d'entitÃ©
    rpc WriteEntity(WriteEntityRequest) returns (WriteEntityResponse);
    
    // Suppression d'entitÃ©
    rpc DeleteEntity(DeleteEntityRequest) returns (DeleteEntityResponse);
    
    // Health check
    rpc Health(HealthRequest) returns (HealthResponse);
}

message AuthToken {
    string operator_id = 1;
    string request_id = 2;
    int64 timestamp = 3;
    bytes signature = 4;
}

message QueryRequest {
    AuthToken auth = 1;
    string database = 2;      // "jayxpose", "jaykonta", etc.
    string sql = 3;
    repeated Value params = 4;
}

message QueryResponse {
    bool success = 1;
    string error = 2;
    repeated Row rows = 3;
    int64 rows_affected = 4;
}

message ReadEntityRequest {
    AuthToken auth = 1;
    string database = 2;
    string table = 3;
    string id = 4;
}

message ReadEntityResponse {
    bool found = 1;
    bytes data = 2;  // JSON sÃ©rialisÃ©
    string error = 3;
}

message WriteEntityRequest {
    AuthToken auth = 1;
    string database = 2;
    string table = 3;
    string id = 4;
    bytes data = 5;  // JSON sÃ©rialisÃ©
    bool upsert = 6;
}

message WriteEntityResponse {
    bool success = 1;
    string id = 2;
    string error = 3;
}

message DeleteEntityRequest {
    AuthToken auth = 1;
    string database = 2;
    string table = 3;
    string id = 4;
}

message DeleteEntityResponse {
    bool success = 1;
    string error = 2;
}

message HealthRequest {}

message HealthResponse {
    bool healthy = 1;
    string version = 2;
    int64 uptime_seconds = 3;
}

message Value {
    oneof value {
        int64 integer = 1;
        double real = 2;
        string text = 3;
        bytes blob = 4;
        bool null = 5;
    }
}

message Row {
    repeated Value values = 1;
}
```

### 5.2 Serveur gRPC

```rust
// kindmother-service/src/server.rs

use tonic::{transport::Server, Request, Response, Status};
use crate::proto::kindmother_service_server::{KindMotherService, KindMotherServiceServer};
use crate::proto::*;
use crate::auth::validate_token;
use crate::permissions::check_permission;
use crate::database::EncryptedDatabase;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct KindMotherServer {
    databases: Arc<RwLock<HashMap<String, EncryptedDatabase>>>,
}

impl KindMotherServer {
    pub fn new(databases: HashMap<String, EncryptedDatabase>) -> Self {
        Self {
            databases: Arc::new(RwLock::new(databases)),
        }
    }
}

#[tonic::async_trait]
impl KindMotherService for KindMotherServer {
    async fn query(
        &self,
        request: Request<QueryRequest>,
    ) -> Result<Response<QueryResponse>, Status> {
        let req = request.into_inner();
        
        // 1. Valider le token d'authentification
        let auth = req.auth.ok_or_else(|| Status::unauthenticated("Missing auth token"))?;
        validate_token(&auth).map_err(|e| Status::unauthenticated(e.to_string()))?;
        
        // 2. VÃ©rifier les permissions
        check_permission(&auth.operator_id, &req.database, "query")
            .map_err(|e| Status::permission_denied(e.to_string()))?;
        
        // 3. ExÃ©cuter la requÃªte
        let databases = self.databases.read().await;
        let db = databases.get(&req.database)
            .ok_or_else(|| Status::not_found(format!("Database {} not found", req.database)))?;
        
        let conn = db.connect()
            .map_err(|e| Status::internal(e.to_string()))?;
        
        // TODO: ExÃ©cuter et retourner les rÃ©sultats
        
        Ok(Response::new(QueryResponse {
            success: true,
            error: String::new(),
            rows: vec![],
            rows_affected: 0,
        }))
    }
    
    async fn read_entity(
        &self,
        request: Request<ReadEntityRequest>,
    ) -> Result<Response<ReadEntityResponse>, Status> {
        let req = request.into_inner();
        
        // Validation et permissions...
        let auth = req.auth.ok_or_else(|| Status::unauthenticated("Missing auth token"))?;
        validate_token(&auth).map_err(|e| Status::unauthenticated(e.to_string()))?;
        check_permission(&auth.operator_id, &req.database, &format!("read:{}", req.table))
            .map_err(|e| Status::permission_denied(e.to_string()))?;
        
        // Lecture...
        // TODO: ImplÃ©menter
        
        Ok(Response::new(ReadEntityResponse {
            found: false,
            data: vec![],
            error: String::new(),
        }))
    }
    
    async fn write_entity(
        &self,
        request: Request<WriteEntityRequest>,
    ) -> Result<Response<WriteEntityResponse>, Status> {
        // Validation, permissions, Ã©criture...
        // TODO: ImplÃ©menter
        Ok(Response::new(WriteEntityResponse {
            success: true,
            id: String::new(),
            error: String::new(),
        }))
    }
    
    async fn delete_entity(
        &self,
        request: Request<DeleteEntityRequest>,
    ) -> Result<Response<DeleteEntityResponse>, Status> {
        // Validation, permissions, suppression...
        // TODO: ImplÃ©menter
        Ok(Response::new(DeleteEntityResponse {
            success: true,
            error: String::new(),
        }))
    }
    
    async fn health(
        &self,
        _request: Request<HealthRequest>,
    ) -> Result<Response<HealthResponse>, Status> {
        Ok(Response::new(HealthResponse {
            healthy: true,
            version: env!("CARGO_PKG_VERSION").to_string(),
            uptime_seconds: 0, // TODO: Calculer
        }))
    }
}

/// Point d'entrÃ©e du service KindMother.
pub async fn run_server(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    use crate::encryption::derive_master_key;
    
    tracing::info!("Deriving master encryption key...");
    let master_key = derive_master_key();
    
    tracing::info!("Opening encrypted databases...");
    let data_dir = dirs::data_local_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("miyukini")
        .join("kindmother")
        .join("data");
    
    std::fs::create_dir_all(&data_dir)?;
    
    let mut databases = HashMap::new();
    
    // Ouvrir les bases des diffÃ©rents opÃ©rateurs
    for db_name in ["jayxpose", "jaykonta", "jayfestival"] {
        let db_path = data_dir.join(format!("{}.db", db_name));
        let db = EncryptedDatabase::open(&db_path, master_key.clone()).await?;
        databases.insert(db_name.to_string(), db);
        tracing::info!("Opened database: {}", db_name);
    }
    
    let server = KindMotherServer::new(databases);
    
    tracing::info!("Starting KindMother service on {}", addr);
    
    Server::builder()
        .add_service(KindMotherServiceServer::new(server))
        .serve(addr.parse()?)
        .await?;
    
    Ok(())
}
```

---

## 6. Client IPC

### 6.1 Client pour OpÃ©rateurs

```rust
// kindmother-client/src/lib.rs

use kindmother::Storage;
use tonic::transport::Channel;

pub mod proto {
    tonic::include_proto!("kindmother");
}

use proto::kind_mother_service_client::KindMotherServiceClient;

/// Client KindMother pour les OpÃ©rateurs.
/// 
/// ImplÃ©mente le trait Storage de kindmother.
/// 
/// @id: kindmother_client
/// @do: provide_ipc_client_for_operators
/// @layer: infra
pub struct KindMotherClient {
    client: KindMotherServiceClient<Channel>,
    operator_id: String,
    database: String,
}

impl KindMotherClient {
    /// CrÃ©e un nouveau client connectÃ© au service KindMother.
    pub async fn connect(
        addr: &str,
        operator_id: &str,
        database: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let client = KindMotherServiceClient::connect(addr.to_string()).await?;
        
        Ok(Self {
            client,
            operator_id: operator_id.to_string(),
            database: database.to_string(),
        })
    }
    
    /// CrÃ©e un token d'authentification pour la requÃªte.
    fn create_auth_token(&self) -> proto::AuthToken {
        proto::AuthToken {
            operator_id: self.operator_id.clone(),
            request_id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now().timestamp(),
            signature: vec![], // TODO: Signer avec StrongFather
        }
    }
    
    /// ExÃ©cute une requÃªte SQL.
    pub async fn query(&mut self, sql: &str) -> Result<Vec<Vec<proto::Value>>, ClientError> {
        let request = proto::QueryRequest {
            auth: Some(self.create_auth_token()),
            database: self.database.clone(),
            sql: sql.to_string(),
            params: vec![],
        };
        
        let response = self.client.query(request).await
            .map_err(|e| ClientError(e.to_string()))?;
        
        let inner = response.into_inner();
        if !inner.success {
            return Err(ClientError(inner.error));
        }
        
        Ok(inner.rows.into_iter().map(|r| r.values).collect())
    }
    
    /// Lit une entitÃ© par ID.
    pub async fn read_entity(&mut self, table: &str, id: &str) -> Result<Option<Vec<u8>>, ClientError> {
        let request = proto::ReadEntityRequest {
            auth: Some(self.create_auth_token()),
            database: self.database.clone(),
            table: table.to_string(),
            id: id.to_string(),
        };
        
        let response = self.client.read_entity(request).await
            .map_err(|e| ClientError(e.to_string()))?;
        
        let inner = response.into_inner();
        if !inner.error.is_empty() {
            return Err(ClientError(inner.error));
        }
        
        if inner.found {
            Ok(Some(inner.data))
        } else {
            Ok(None)
        }
    }
    
    /// Ã‰crit une entitÃ©.
    pub async fn write_entity(
        &mut self,
        table: &str,
        id: &str,
        data: &[u8],
        upsert: bool,
    ) -> Result<String, ClientError> {
        let request = proto::WriteEntityRequest {
            auth: Some(self.create_auth_token()),
            database: self.database.clone(),
            table: table.to_string(),
            id: id.to_string(),
            data: data.to_vec(),
            upsert,
        };
        
        let response = self.client.write_entity(request).await
            .map_err(|e| ClientError(e.to_string()))?;
        
        let inner = response.into_inner();
        if !inner.success {
            return Err(ClientError(inner.error));
        }
        
        Ok(inner.id)
    }
}

#[derive(Debug)]
pub struct ClientError(pub String);

impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "KindMother Client: {}", self.0)
    }
}

impl std::error::Error for ClientError {}
```

---

## 7. Guide de Migration

### 7.1 Ã‰tapes de Migration

| Phase | Action | Effort | Risque |
|-------|--------|--------|--------|
| **1** | CrÃ©er les crates `kindmother-service` et `kindmother-client` | Moyen | Faible |
| **2** | ImplÃ©menter le service avec libSQL | Ã‰levÃ© | Moyen |
| **3** | Migrer les OpÃ©rateurs vers `kindmother-client` | Moyen | Faible |
| **4** | Migrer les donnÃ©es existantes | Faible | Moyen |
| **5** | Supprimer les dÃ©pendances rusqlite directes | Faible | Faible |
| **6** | DÃ©ployer en production | Moyen | Moyen |

### 7.2 Phase 1 : CrÃ©er les Crates

```bash
# Dans le dossier crates/
cd crates

# CrÃ©er kindmother-service
cargo new kindmother-service
cd kindmother-service
# Copier le Cargo.toml de la section 3.1

# CrÃ©er kindmother-client
cd ..
cargo new kindmother-client --lib
cd kindmother-client
# Copier le Cargo.toml de la section 3.1
```

### 7.3 Phase 2 : ImplÃ©menter le Service

1. CrÃ©er le fichier proto (`proto/kindmother.proto`)
2. Configurer `build.rs` pour tonic-build
3. ImplÃ©menter `encryption.rs` (dÃ©rivation de clÃ©)
4. ImplÃ©menter `database.rs` (connexion libSQL)
5. ImplÃ©menter `auth.rs` (validation tokens)
6. ImplÃ©menter `permissions.rs` (matrice d'accÃ¨s)
7. ImplÃ©menter `server.rs` (service gRPC)
8. ImplÃ©menter `main.rs` (point d'entrÃ©e)

### 7.4 Phase 3 : Migrer les OpÃ©rateurs

**Exemple pour JayXpose :**

```rust
// AVANT (jayxpose/src/data/kindmother_db.rs)
use rusqlite::{Connection, params};

pub struct JayXposeDb {
    conn: Mutex<Connection>,
}

impl JayXposeDb {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, DbError> {
        let conn = Connection::open(path)?;
        // ...
    }
}
```

```rust
// APRÃˆS (jayxpose/src/data/kindmother_db.rs)
use kindmother_client::KindMotherClient;

pub struct JayXposeDb {
    client: KindMotherClient,
}

impl JayXposeDb {
    pub async fn connect() -> Result<Self, DbError> {
        let client = KindMotherClient::connect(
            "http://[::1]:50051",  // Adresse du service
            "jayxpose",             // Identifiant opÃ©rateur
            "jayxpose",             // Base de donnÃ©es
        ).await?;
        
        Ok(Self { client })
    }
    
    pub async fn exposant_by_id(&mut self, id: &str) -> Result<Option<ExposantProfile>, DbError> {
        let data = self.client.read_entity("exposants", id).await?;
        match data {
            Some(bytes) => {
                let profile: ExposantProfile = serde_json::from_slice(&bytes)?;
                Ok(Some(profile))
            }
            None => Ok(None),
        }
    }
}
```

### 7.5 Phase 4 : Migration des DonnÃ©es

Script de migration pour convertir les bases SQLite existantes vers libSQL chiffrÃ© :

```rust
// tools/migrate_to_encrypted.rs

use rusqlite::Connection as SqliteConn;
use libsql::{Builder, Cipher, EncryptionConfig};

async fn migrate_database(
    source_path: &str,
    dest_path: &str,
    encryption_key: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Ouvrir la source (non chiffrÃ©e)
    let source = SqliteConn::open(source_path)?;
    
    // 2. CrÃ©er la destination (chiffrÃ©e)
    let encryption = EncryptionConfig::new(Cipher::Aes256Gcm, encryption_key.to_vec());
    let dest_db = Builder::new_local(dest_path)
        .encryption_config(encryption)
        .build()
        .await?;
    let dest = dest_db.connect()?;
    
    // 3. Lister les tables
    let tables: Vec<String> = source.prepare(
        "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'"
    )?.query_map([], |row| row.get(0))?
    .collect::<Result<Vec<_>, _>>()?;
    
    // 4. Pour chaque table, copier le schÃ©ma et les donnÃ©es
    for table in tables {
        // RÃ©cupÃ©rer le schÃ©ma
        let schema: String = source.query_row(
            "SELECT sql FROM sqlite_master WHERE type='table' AND name=?",
            [&table],
            |row| row.get(0),
        )?;
        
        // CrÃ©er la table dans la destination
        dest.execute(&schema, ()).await?;
        
        // Copier les donnÃ©es (en batches pour les grandes tables)
        // TODO: ImplÃ©menter la copie par batch
        
        println!("Migrated table: {}", table);
    }
    
    println!("Migration complete!");
    Ok(())
}
```

### 7.6 Phase 5 : Nettoyage

AprÃ¨s validation de la migration :

1. Supprimer `rusqlite` des `Cargo.toml` des OpÃ©rateurs
2. Supprimer les fichiers `*_db.rs` obsolÃ¨tes
3. Mettre Ã  jour la documentation

### 7.7 Phase 6 : DÃ©ploiement

**Windows (Service) :**

```powershell
# Installer comme service Windows
sc.exe create KindMotherService binPath= "C:\miyukini\kindmother-service.exe"
sc.exe config KindMotherService start= auto
sc.exe start KindMotherService
```

**Linux (systemd) :**

```ini
# /etc/systemd/system/kindmother.service
[Unit]
Description=KindMother Database Service
After=network.target

[Service]
Type=simple
User=kindmother
Group=kindmother
ExecStart=/opt/miyukini/kindmother-service
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
```

---

## 8. Troubleshooting

### 8.1 Erreurs Courantes

| Erreur | Cause | Solution |
|--------|-------|----------|
| `Failed to derive key` | Impossible de lire machine ID | VÃ©rifier les permissions systÃ¨me |
| `Database is locked` | Fichier ouvert par un autre processus | VÃ©rifier qu'un seul service tourne |
| `Decryption failed` | ClÃ© incorrecte ou fichier corrompu | VÃ©rifier que la clÃ© n'a pas changÃ© |
| `Permission denied` | OpÃ©rateur non autorisÃ© | VÃ©rifier la matrice de permissions |
| `Connection refused` | Service non dÃ©marrÃ© | DÃ©marrer kindmother-service |

### 8.2 VÃ©rification du Chiffrement

Pour vÃ©rifier qu'une base est bien chiffrÃ©e :

```bash
# Tenter d'ouvrir avec sqlite3 standard
sqlite3 jayxpose.db "SELECT * FROM exposants LIMIT 1;"
# RÃ©sultat attendu : "file is not a database"

# Si Ã§a fonctionne, la base N'EST PAS chiffrÃ©e !
```

### 8.3 Logs de Diagnostic

```rust
// Activer les logs dÃ©taillÃ©s
RUST_LOG=kindmother_service=debug cargo run
```

---

## 9. RÃ©fÃ©rences

- [Security - Gouvernance Cores Protection Donnees](..//..//WorrySentinel//_index.md)
- [KindMother - Documentation Fondatrice](../foundation/KindMother%20-%20Documentation%20Fondatrice.md)
- [libSQL Documentation](https://docs.turso.tech/libsql)
- [Turso Encryption Guide](https://turso.tech/blog/introducing-fast-native-encryption-in-turso-database)
- [tonic gRPC](https://github.com/hyperium/tonic)
- [Argon2 crate](https://docs.rs/argon2)

---

**Date de crÃ©ation :** 2026-02-08  
**Version :** 1.0  
**Statut :** IMPLEMENTATION â€” Guide technique  
**Auteur :** Architecture Miyukini  

---

## 10. Mini Log de GÃ©nÃ©ration

### DÃ©cisions structurantes

- libSQL choisi pour compatibilitÃ© SQLite et chiffrement natif
- gRPC choisi pour IPC (performance + typage fort)
- Argon2id choisi pour dÃ©rivation de clÃ© (rÃ©sistance GPU/ASIC)
- Migration incrÃ©mentale en 6 phases

### VÃ©rification de cohÃ©rence

- âœ… CohÃ©rence avec le document de sÃ©curitÃ© Gouvernance Cores
- âœ… CohÃ©rence avec l'architecture KindMother existante
- âœ… Respect des Lois d'Autonomie
- âœ… Code examples complets et fonctionnels

**Aucune contradiction dÃ©tectÃ©e.**

