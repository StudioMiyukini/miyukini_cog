---
name: miyukini-error-security
description: Patterns d'erreurs et securite dans Miyukini COG. Couvre les 6 patterns d'erreurs (manual Display, thiserror #[from], domain Result alias, mandate-based, service errors, save errors), WorrySentinel (ThreatLevel, SecurityLevel, DegradationState), BorderGuard (TrustLevel, Boundary, CrossingRule, BoundaryRegistry), patterns d'authentification (AuthSession, AuthResult, sign_in/sign_up/sign_out), hash de mots de passe (Argon2id OWASP), et les 8 moteurs de securite. Utiliser quand on cree des types d'erreur, quand on implemente l'authentification dans un service, quand on configure les niveaux de securite, quand on travaille sur la gestion des menaces, ou quand on ajoute de la validation.
---

# Erreurs & Securite — Miyukini COG

## Patterns d'erreurs

6 patterns utilises selon le contexte. Choisir le bon selon la sensibilite des donnees et le niveau du crate.

---

### Pattern 1 : Manual (pas thiserror) — Errors sensibles

Pour les crates qui ne doivent **jamais fuiter de donnees metier** dans les erreurs.

```rust
// crates/miyusql/src/errors.rs
#[derive(Debug, Clone)]
pub enum MiyuSQLError {
    NoMandate,           // Execution refusee : pas de mandat
    Timeout,
    Connection(String),  // Message technique, pas de contenu SQL
    Syntax(String),
    Execution(String),   // Pas de donnees business
}

impl std::fmt::Display for MiyuSQLError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoMandate => write!(f, "Execution refused: no governed mandate"),
            Self::Timeout => write!(f, "Execution timeout"),
            Self::Connection(msg) => write!(f, "Connection error: {msg}"),
            Self::Syntax(msg) => write!(f, "Syntax error: {msg}"),
            Self::Execution(msg) => write!(f, "Execution error: {msg}"),
        }
    }
}
impl std::error::Error for MiyuSQLError {}
```

**Utiliser quand** : Toolkits (Strate 6), crates proches de la donnee brute.

---

### Pattern 2 : thiserror avec #[from] — Services

Pour les services qui aggregent plusieurs sources d'erreurs.

```rust
// crates/kindmother-service/src/errors.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Database error: {0}")]
    Database(String),
    #[error("Encryption error: {0}")]
    Encryption(String),
    #[error("Authentication failed: {0}")]
    Authentication(String),
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Quota exceeded: {0}")]
    QuotaExceeded(String),
    #[error("Transport error: {0}")]
    Transport(String),
    #[error("Internal error: {0}")]
    Internal(String),
}

// Conversions automatiques
impl From<rusqlite::Error> for ServiceError {
    fn from(err: rusqlite::Error) -> Self { Self::Database(err.to_string()) }
}
impl From<std::io::Error> for ServiceError {
    fn from(err: std::io::Error) -> Self { Self::Transport(err.to_string()) }
}
impl From<serde_json::Error> for ServiceError {
    fn from(err: serde_json::Error) -> Self { Self::Validation(err.to_string()) }
}
```

**Utiliser quand** : Services (Strate 7), crates KindMother.

---

### Pattern 3 : thiserror avec sources directes

Pour les crates avec peu d'erreurs specifiques.

```rust
// crates/miyukinibb/src/errors.rs
#[derive(Debug, Error)]
pub enum MiyukiniBbError {
    #[error("URL Origin invalide ou manquante")]
    InvalidOriginUrl,
    #[error("Appel HTTP: {0}")]
    Http(#[from] ureq::Error),
    #[error("API Origin: {status} — {body}")]
    Api { status: u16, body: String },
    #[error("Reponse JSON invalide: {0}")]
    Json(String),
}
```

---

### Pattern 4 : Alias Result domaine

Standard dans chaque service :

```rust
// crates/jay1tribu/src/domain.rs
pub type Jay1TribuResult<T> = Result<T, Jay1TribuDomainError>;

#[derive(Debug)]
pub enum Jay1TribuDomainError {
    Db(DbError),
    WebwayRequired,
    TransfertFichierReserveAmi,
}

impl From<DbError> for Jay1TribuDomainError {
    fn from(e: DbError) -> Self { Self::Db(e) }
}
```

**Pattern** : `type {Service}Result<T> = Result<T, {Service}Error>;`

---

### Pattern 5 : Erreurs minimales (NoMandate)

Pour les toolkits non encore implementes :

```rust
#[derive(Debug, Clone)]
pub enum MonToolError {
    NoMandate,
    Unimplemented,
}
```

Suffisant pour la phase de scaffolding.

---

### Pattern 6 : Save/Load specifique

```rust
#[derive(Debug, Clone)]
pub enum SaveError {
    Db(String),
    Serialization(String),
    InvalidSlot(u8),
}
```

---

## WorrySentinel — Detection des menaces

### ThreatLevel (niveaux de menace)

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ThreatLevel { None, Low, Medium, High, Critical }

pub trait ThreatDetector {
    fn detect(&self) -> ThreatLevel;
}

pub struct DefaultThreatDetector {
    levels: VecDeque<ThreatLevel>,
}

impl DefaultThreatDetector {
    pub fn new() -> Self
    pub fn with_signals(levels: impl IntoIterator<Item = ThreatLevel>) -> Self
    pub fn max_signal(&self) -> ThreatLevel  // Agregation par max
}
```

**Invariant INV-WS-4** : Le detecteur est immutable une fois cree.

### SecurityLevel (niveaux operationnels)

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SecurityLevel {
    Public,     // 0 — Site vitrine
    Standard,   // 1 — CMS, backoffice (DEFAUT)
    Sensitive,  // 2 — Donnees personnelles
    Critical,   // 3 — Auth, paiement
    Hardened,   // 4 — Environnement hostile
}

pub trait SecurityLevelManager {
    fn get_current(&self) -> SecurityLevel;
}
// Default: Standard
```

### DegradationState (degradation progressive)

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DegradationState {
    Normal,      // T0 — Toutes capacites
    Unstable,    // T1 — Log renforce
    Degraded,    // T2 — Capacites desactivees
    Restricted,  // T3 — Gel produits non essentiels
    Blocked,     // T4 — Diagnostics uniquement
}

pub trait DegradationManager {
    fn get_current(&self) -> Degradation;
}
```

---

## BorderGuard — Frontieres et confiance

### TrustLevel (classification entites)

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TrustLevel {
    Trusted,   // Verifiee et approuvee
    Verified,  // Authentifiee mais pas approuvee
    Unknown,   // Pas encore verifiee (DEFAUT)
    Hostile,   // Identifiee comme menacante
}
// Ordre : Trusted > Verified > Unknown > Hostile

pub trait TrustLevelClassifier {
    fn classify(&self, entity_id: &str) -> TrustLevel;
}

pub struct DefaultTrustLevelClassifier {
    levels: HashMap<String, TrustLevel>,
}
// Default = Unknown (posture fail-secure)
```

### Boundary (frontieres systeme)

```rust
pub enum BoundaryType { External, Internal, Integration }

pub struct Boundary {
    pub id: String,
    pub boundary_type: BoundaryType,
    pub name: String,
}

pub struct BoundaryMetadata {
    pub created_at: String,
    pub created_by: String,
    pub justification: String,
    pub version: u32,
}
```

### CrossingRule (regles de traversee)

```rust
pub struct CrossingRule {
    pub boundary_id: String,
    pub min_trust_level: TrustLevel,
    pub allowed: bool,
}

pub struct CrossingRules { pub rules: Vec<CrossingRule> }
```

### BoundaryRegistry (registre central)

```rust
pub struct BoundaryRegistry {
    boundaries: HashMap<String, (Boundary, BoundaryMetadata)>,
    crossing_rules: HashMap<String, CrossingRules>,
}

impl BoundaryRegistry {
    pub fn register_boundary(&mut self, boundary: Boundary, meta: BoundaryMetadata)
    pub fn register_crossing_rules(&mut self, boundary_id: String, rules: CrossingRules)
    pub fn get_boundary(&self, id: &str) -> Option<(&Boundary, &BoundaryMetadata)>
    pub fn list_boundaries(&self) -> Vec<(&Boundary, &BoundaryMetadata)>
    pub fn get_crossing_rules(&self, boundary_id: &str) -> Option<&CrossingRules>
}
```

---

## Authentification — Pattern standard

### Structure commune a tous les services

```rust
pub type AuthResult<T> = Result<T, AuthError>;

#[derive(Debug, Clone)]
pub struct AuthError { pub message: String }

#[derive(Debug, Clone)]
pub struct AuthSession {
    pub user_id: String,
    pub email: Option<String>,
    pub access_token: String,
    pub profile: Option<Profile>,  // Type specifique au service
}

// API standard
pub fn auth_sign_in(db: &Db, email: &str, password: &str) -> AuthResult<AuthSession>
pub fn auth_sign_up(db: &Db, email: &str, password: &str, user_type: &str) -> AuthResult<AuthSession>
pub fn auth_sign_out() -> AuthResult<()>
```

### Hash de mots de passe

```rust
// crates/kindmother-db-adapter
pub fn hash_password(password: &str) -> String
// Format: "argon2id$base64(salt)$base64(hash)"
// Algorithme: Argon2id (recommandation OWASP 2024)

pub fn verify_password(stored: &str, password: &str) -> bool
// Supporte: Argon2id (moderne) + SHA-256 (legacy)
// Comparaison en temps constant (protection timing attack)
```

### Feature-gated auth

```rust
// miyukini-central/src/auth/mod.rs
#[cfg(feature = "legacy-sqlite")]
mod db;           // Acces direct rusqlite

#[cfg(feature = "kindmother-only")]
mod db_client;    // Via KindMother TCP/JSON
```

---

## 8 Moteurs de securite

| Moteur | Role | Crate principal |
|--------|------|-----------------|
| **Integrity Engine** | Verification permanente d'integrite | worrysentinel |
| **Validation Engine** | Filtrage systematique des entrees | caringnanny |
| **Policy Engine** | Regles operationnelles et acces | strongfather |
| **Consensus Engine** | Eviter decision unique (multi-agent) | — |
| **Audit Engine** | Tracabilite active | kindmother-service |
| **Sandbox Engine** | Isolation et execution securisee | borderguard |
| **Cognitive Guard** | Securite IA, derive, anti-biais | — |
| **Recovery Engine** | Resilience, rollback, restauration | — |

---

## Choisir le bon pattern d'erreur

```
Nouveau crate ?
├── Toolkit (Strate 6) sans donnees sensibles
│   └── Pattern 5 (NoMandate) ou Pattern 1 (manual)
├── Toolkit avec SQL/donnees
│   └── Pattern 1 (manual, pas de fuite)
├── Service (Strate 7) simple
│   └── Pattern 3 (thiserror + #[from])
├── Service complexe (multi-sources)
│   └── Pattern 2 (thiserror complet) + Pattern 4 (Result alias)
└── Jeu / Save-Load
    └── Pattern 6 (SaveError minimal)
```
