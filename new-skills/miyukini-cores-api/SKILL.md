---
name: miyukini-cores-api
description: API complete des 8 Cores systeme de Miyukini COG (Strate 4). Traits, structs, enums, methodes publiques, patterns d'utilisation, etat d'implementation de chaque Core (StrongFather, KindMother, BorderGuard, CaringNanny, MasterButler, BondingBrother, EverBuddy, WorrySentinel) + Kernel, TAMR, LogisticsSteward. Utiliser quand on appelle un Core depuis un service/toolkit, quand on implemente une interaction inter-Core, quand on veut comprendre l'API reelle d'un Core, ou quand on cree un nouveau Core.
---

# API des 8 Cores Systeme

Ce skill documente l'API reelle (traits, structs, enums) de chaque Core tel qu'implemente dans le codebase. Chaque Core depend uniquement de `miyukini-kernel` et n'a aucune dependance croisee avec les autres Cores.

## Principe fondamental

Les Cores **gouvernent** mais n'executent jamais. Chaque Core expose des traits definissant son contrat, avec des implementations par defaut pour le testing.

---

## Kernel (crates/miyukini-kernel) — Substrat technique

Le Kernel fournit les primitives partagees par tous les Cores. **Aucune logique metier.**

### Traits et types

```rust
// Identite
pub trait IdGenerator { fn generate(&self) -> Id; }
pub struct UuidIdGenerator;
pub struct Id(String); // Opaque, expose via .value()

// Configuration
pub trait Config { fn get(&self, key: &str) -> Option<String>; }
pub struct EnvConfig; // Lit les variables d'environnement

// Horloge
pub trait Clock { fn now(&self) -> SystemTime; }
pub struct DefaultClock;

// Journalisation
pub trait Logger { fn log(&self, level: Level, message: &str); }
pub enum Level { Debug, Info, Warn, Error }
pub struct DefaultLogger; // Ecrit sur stdout

// Cycle de vie
pub trait Lifecycle { fn shutdown(&self); }
```

**Dependance unique** : `uuid`

---

## StrongFather (crates/strongfather) — Decisions strategiques

**Role** : Evalue les politiques et produit des decisions. Ne persiste rien, ne execute rien.

### Modele d'intention (intent.rs)

```rust
pub enum ActionType { Creation, Modification, Deletion, Read, Evaluation }

pub struct Intent {
    pub intent_id: Id,
    pub action_type: ActionType,
    pub subject: String,
    pub call_context: CallContext,
    pub data: Option<String>,
    pub requested_priority: Option<u32>,
    pub constraints: Vec<String>,
    pub metadata: HashMap<String, String>,
}

pub struct CallContext {
    pub caller_identity: String,
    pub origin: String,
    pub instance: String,
}

// Builder fluent
impl Intent {
    pub fn new(action_type, subject, call_context) -> Self
    pub fn with_priority(mut self, priority: u32) -> Self
    pub fn with_constraints(mut self, constraints: Vec<String>) -> Self
    pub fn with_metadata(mut self, key: &str, value: &str) -> Self
}
```

### Modele de politique (policy.rs)

```rust
pub enum PolicyType { Permission, Constraint, Priority, Validation, Composite }
pub enum PolicyEffect { Authorize, Deny, Constrain, Prioritize, Validate }
pub enum PolicyPriorityLevel { Critical=4, Important=3, Normal=2, Optional=1 }

pub struct Policy {
    pub id: Id,
    pub policy_type: PolicyType,
    pub condition: PolicyCondition, // Expression declarative
    pub rule: PolicyRule,           // Expression declarative
    pub effect: PolicyEffect,
    pub priority: PolicyPriorityLevel,
    pub metadata: HashMap<String, String>,
    pub composite_policies: Vec<Policy>,
}

pub struct PolicySet(HashMap<String, Policy>);
// Methodes: add(), get(), iter(), len(), is_empty()

pub enum PolicyResult {
    Satisfied { policy_id: String },
    Violated { policy_id: String, reason: String },
    Priority { policy_id: String, value: u32 },
}
```

### Modele de decision (decision.rs)

```rust
pub enum DecisionType {
    Accepted { priority: u32 },
    Refused { reason: String, violated_policies: Vec<String> },
    Ambiguous { missing_information: Vec<String>, clarifications_required: Vec<String> },
    Deferred { reason: String, context_required: Vec<String> },
}

pub struct Decision {
    pub intent_id: Id,
    pub decision_type: DecisionType,
    pub justification: Justification,
    pub policies_applied: Vec<String>,
    pub evaluation_context: HashMap<String, String>,
    pub metadata: HashMap<String, String>,
}

// Queries: is_accepted(), is_refused(), is_ambiguous(), is_deferred()

pub struct Justification {
    pub explanation: String,
    pub policy_references: Vec<String>,
    pub reasoning_steps: Vec<String>,
}
```

### Moteur de politiques (policy_engine.rs)

```rust
pub struct PolicyEngine { policy_set: PolicySet }

impl PolicyEngine {
    pub fn new(policy_set: PolicySet) -> Self
    pub fn apply(&self, intent: &Intent) -> Result<Decision, PolicyEngineError>
    // Tri par priorite descendante, arret immediat si violation critique (REGLE-PRIO-2)
}

pub enum PolicyEngineError {
    ConsistencyError { violated_invariant: String, reason: String },
}
```

**Invariants** :
- R-TYPE-1 : Un seul ActionType par intention
- G-JUST-1 : Toute decision doit etre justifiee
- REGLE-PRIO-2 : Violation critique = arret immediat

---

## BorderGuard (crates/borderguard) — Frontieres et confiance

**Role** : Definit les frontieres, classifie la confiance, etablit les regles. **Ne les applique jamais.**

### Niveaux de confiance (trust_level.rs)

```rust
pub enum TrustLevel { Trusted, Verified, Unknown, Hostile }
// Impl Ord : Trusted > Verified > Unknown > Hostile

pub trait TrustLevelClassifier {
    fn classify(&self, entity_id: &str) -> TrustLevel;
}

pub struct DefaultTrustLevelClassifier(HashMap<String, TrustLevel>);
// Default = Unknown (posture fail-secure)
// Methodes: new(), register(entity_id, level), classify_entity()
```

### Frontieres (boundary.rs)

```rust
pub enum BoundaryType { External, Internal, Integration }
pub struct Boundary { pub id: Id, pub boundary_type: BoundaryType, pub name: String }
```

### Regles de traversee (crossing.rs)

```rust
pub struct CrossingRule {
    pub boundary_id: String,
    pub min_trust_level: TrustLevel,
    pub allowed: bool,
}
pub struct CrossingRules(Vec<CrossingRule>);
// Methodes: new(), add(rule)
```

---

## CaringNanny (crates/caringnanny) — Observation et surveillance

**Role** : Observe, detecte, classifie. Ne modifie jamais, ne decide jamais.

```rust
pub enum EventType { StateChange, Anomaly, HealthChange }

pub struct SystemEvent {
    pub event_type: EventType,
    pub component: String,
    pub message: String,
}

pub trait Observer { fn observe(&mut self, event: SystemEvent); }
pub struct DefaultObserver(Vec<SystemEvent>);
// Methodes: events() -> &[SystemEvent]

pub trait HealthChecker { fn check(&self) -> HealthStatus; }
pub enum HealthStatus { /* Healthy, Degraded, Unhealthy */ }
pub struct DefaultHealthChecker;

pub trait MetricsCollector { /* collect metrics */ }
```

---

## MasterButler (crates/masterbutler) — Orchestration des workflows

**Role** : Orchestre l'execution en respectant les decisions de StrongFather, utilise l'API KindMother pour les donnees.

```rust
pub type WorkflowId = String;
pub struct Workflow { pub id: WorkflowId, pub name: String }

pub struct Step { pub id: String, pub name: String }
pub enum StepResult { Success, Failure, Skipped }

pub trait Orchestrator {
    fn execute(&mut self, workflow: &Workflow, steps: &[Step]) -> Vec<StepResult>;
}
pub struct DefaultOrchestrator; // Stub: retourne Success pour tout
```

**Etat** : Implementation stub, le vrai orchestrateur deleguera aux services.

---

## BondingBrother (crates/bondingbrother) — Liaison Operateurs

**Role** : Mediatise, traduit, filtre les interactions entre Operateurs et l'ecosysteme.

```rust
// Modules: connection, sync, translation

pub trait ConnectionManager { /* gestion des connexions inter-operateurs */ }
pub trait SyncManager { /* orchestration de la synchronisation */ }
pub trait Translator { /* traduction entre formats/protocoles */ }
```

---

## EverBuddy (crates/everbuddy) — Compatibilite et migration

**Role** : Gouverne l'evolution des structures, contrats et entites dans le temps.

```rust
// Modules: compatibility, migration, version

pub trait CompatibilityChecker { /* verifie la compatibilite des versions */ }
pub trait MigrationExecutor { /* execute les migrations */ }
pub trait VersionManager { /* gere le cycle de vie des versions */ }
```

---

## WorrySentinel (crates/worrysentinel) — Detection des menaces

**Role** : Detecte les menaces, evalue les niveaux de securite, gere la degradation progressive.

```rust
// Modules: threat_detector, security_level, degradation

pub trait ThreatDetector { fn detect(&self) -> ThreatLevel; }
pub enum ThreatLevel { None, Low, Medium, High, Critical }
// Agregation via max signal

pub trait SecurityLevelManager { fn evaluate(&self) -> SecurityLevel; }
// Niveaux 0-4: Public, Standard (defaut), Sensitive, Critical, Hardened

pub trait DegradationManager { fn degrade(&self) -> DegradationState; }
```

**Invariant INV-WS-4** : Le detecteur de menaces est immutable.

---

## TAMR (crates/tamr) — Taxonomie et metadata

```rust
pub trait TaxonomyManager { /* gestion des taxonomies */ }
pub trait MetadataManager { /* gestion des metadonnees */ }
pub trait Classifier { /* classification des entites */ }
```

## LogisticsSteward (crates/logisticssteward) — Ressources

```rust
pub trait ResourceManager { /* gestion des ressources */ }
pub trait AllocationManager { /* allocation */ }
pub trait Optimizer { /* optimisation */ }
```

---

## Etat d'implementation

| Core | Etat | Commentaire |
|------|------|-------------|
| StrongFather | **Complet** | Intent/Policy/Decision/Engine fonctionnels |
| KindMother | **Complet** | Abstractions Storage/Sync/API + 4 crates satellites |
| BorderGuard | **Complet** | Trust levels, Boundaries, CrossingRules |
| CaringNanny | **Partiel** | Observer in-memory, HealthChecker basique |
| MasterButler | **Stub** | Orchestrateur sequentiel stub |
| BondingBrother | **Traits** | Trait definitions, implementations minimales |
| EverBuddy | **Traits** | Trait definitions seulement |
| WorrySentinel | **Partiel** | Detection + niveaux de securite |
| TAMR | **Minimal** | Interfaces definies |
| LogisticsSteward | **Minimal** | Interfaces definies |

## Graphe de dependances

```
miyukini-kernel (uuid seulement)
    |
    +-- strongfather
    +-- kindmother (+miyusql)
    +-- borderguard
    +-- caringnanny
    +-- masterbutler
    +-- bondingbrother
    +-- everbuddy
    +-- worrysentinel
    +-- tamr
    +-- logisticssteward
```

Aucune dependance croisee entre Cores. Tous dependent uniquement du Kernel.
