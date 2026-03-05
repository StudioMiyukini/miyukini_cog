# Kernel - Reference Implementation Guidelines

## Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document est **informatif, non normatif, et non contractuel**. Il guide un dÃ©veloppeur pour implÃ©menter le Kernel correctement, sans violer les contrats FONDATION.

**Objectif pÃ©dagogique :** Ce document vise Ã  aider les dÃ©veloppeurs Ã  comprendre comment traduire les contrats FONDATION en implÃ©mentation Rust, en respectant strictement les invariants, garanties, et interdictions du Kernel.

**Avertissement :** Ce document ne doit pas Ãªtre interprÃ©tÃ© abusivement. Il ne crÃ©e aucune nouvelle rÃ¨gle contractuelle et ne modifie aucun contrat existant. Les contrats FONDATION priment toujours sur ce guide.

**Relation avec les contrats FONDATION :** Ce document fait rÃ©fÃ©rence aux contrats FONDATION existants mais ne les Ã©tend pas, ne les modifie pas, et ne crÃ©e aucune nouvelle obligation contractuelle.

---

## 1. Introduction

### 1.1 Objectif

Ce document fournit des lignes directrices pour implÃ©menter le Kernel de maniÃ¨re conforme aux contrats FONDATION. Il explique comment traduire les concepts contractuels en logique d'implÃ©mentation Rust sans interprÃ©tation abusive.

**Rappel :** Le Kernel est le **noyau technique minimal** de la fondation Miyukini, et non un kernel systÃ¨me au sens OS.

### 1.2 Nature informative

Ce document est **purement informatif**. Il ne dÃ©finit pas de nouvelles rÃ¨gles, n'impose pas de technologies, et ne prescrit pas de solutions techniques. Il guide la comprÃ©hension et l'application des contrats FONDATION.

### 1.3 Sources contractuelles

Ce document se base sur tous les contrats FONDATION du Kernel :

- **[Definition Kernel](../Miyukini%20Core%20System%20-%20Definition%20Kernel.md)** : PÃ©rimÃ¨tre, responsabilitÃ©s, exclusions, frontiÃ¨res
- **[Structure du Kernel](../Miyukini%20Core%20System%20-%20Structure%20du%20Kernel.md)** : Crates, dÃ©pendances, visibilitÃ©, conventions
- **[Revue Traits API v0.1](../Miyukini%20Core%20System%20-%20Revue%20Traits%20API%20v0.1.md)** : Gel des traits publics
- **[Invariants & Guarantees](../contracts/Kernel%20-%20Invariants%20&%20Guarantees.md)** : Catalogue des invariants INV-K-*
- **[Kernel Maintenance Observability Contract](..//..//miyukini-webway-system//reference//_index.md)** : CapacitÃ©s d'observation
- **[Lois Autonomie SystÃ¨me](..//..//miyukini-webway-system//reference//_index.md)** : Contraintes d'autonomie LOI-1 Ã  LOI-6

**Terminologie :** Voir [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md)

---

## 2. Principes gÃ©nÃ©raux d'implÃ©mentation

### 2.1 PuretÃ© et dÃ©terminisme (INV-K-3, INV-K-6)

**Principe contractuel :**

Les invariants INV-K-3 (Primitives locales sÃ»res uniquement) et INV-K-6 (DÃ©terminisme) Ã©tablissent que le Kernel utilise uniquement des opÃ©rations dÃ©terministes et sans effets de bord cachÃ©s.

**Traduction en logique d'implÃ©mentation Rust :**

```rust
// âœ… CORRECT : Fonction pure, dÃ©terministe
pub fn generate_id(&self) -> Id {
    // MÃªme entrÃ©e â†’ mÃªme sortie
    // Pas d'effet de bord cachÃ©
    Id::from(Uuid::new_v4())
}

// âœ… CORRECT : RÃ©sultat explicite pour les erreurs
pub fn load_config(&self) -> Result<Config, ConfigError> {
    // Pas de panic silencieux
    // Erreur explicite si Ã©chec
}

// âŒ INCORRECT : Effet de bord cachÃ©
pub fn generate_id(&mut self) -> Id {
    self.counter += 1; // âŒ Ã‰tat mutable cachÃ©
    Id::from(self.counter)
}

// âŒ INCORRECT : Panic implicite
pub fn load_config(&self) -> Config {
    std::fs::read_to_string("config.toml").unwrap() // âŒ Panic possible
}
```

**RÃ¨gles clÃ©s :**

- PrivilÃ©gier `Result<T, E>` plutÃ´t que panic
- Ã‰viter les effets de bord cachÃ©s
- Structures immutables ou contrÃ´lÃ©es
- OpÃ©rations dÃ©terministes (mÃªme entrÃ©e â†’ mÃªme sortie)

**RÃ©fÃ©rence contrat :** Invariants & Guarantees (INV-K-3, INV-K-6)

---

### 2.2 ZÃ©ro logique mÃ©tier (INV-K-1)

**Principe contractuel :**

L'invariant INV-K-1 Ã©tablit que le Kernel ne contient jamais de logique mÃ©tier. Il ne connaÃ®t ni les entitÃ©s domaine, ni les rÃ¨gles de gestion.

**Traduction en logique d'implÃ©mentation Rust :**

```rust
// âœ… CORRECT : Identifiant gÃ©nÃ©rique
pub struct Id(Uuid);

impl IdGenerator for UuidIdGenerator {
    fn generate(&self) -> Id {
        Id(Uuid::new_v4())
    }
}

// âŒ INCORRECT : Identifiant avec sÃ©mantique mÃ©tier
pub struct UserId(Uuid);  // âŒ "User" = concept mÃ©tier
pub struct OrderId(Uuid); // âŒ "Order" = concept mÃ©tier

// âœ… CORRECT : Configuration gÃ©nÃ©rique
pub trait Config {
    fn get(&self, key: &str) -> Option<&str>;
}

// âŒ INCORRECT : Configuration avec clÃ©s mÃ©tier prÃ©dÃ©finies
pub trait Config {
    fn get_stripe_key(&self) -> Option<&str>;  // âŒ "stripe" = service mÃ©tier
    fn get_user_ttl(&self) -> Duration;        // âŒ "user" = concept mÃ©tier
}
```

**Ce que cela signifie concrÃ¨tement :**

| AutorisÃ© | Interdit |
|----------|----------|
| âœ… GÃ©nÃ©rer un identifiant unique | âŒ GÃ©nÃ©rer un "user_id" formatÃ© |
| âœ… Fournir l'heure courante | âŒ Calculer une date d'expiration produit |
| âœ… Logger un message structurÃ© | âŒ Logger "commande validÃ©e" |
| âœ… Charger une configuration | âŒ DÃ©finir des politiques de tarification |

**RÃ©fÃ©rence contrat :** Definition Kernel (Section 1, 3), Invariants & Guarantees (INV-K-1)

---

### 2.3 ZÃ©ro dÃ©pendance externe critique (INV-K-2)

**Principe contractuel :**

L'invariant INV-K-2 Ã©tablit que le Kernel ne dÃ©pend jamais d'un service externe pour fonctionner. Il doit pouvoir dÃ©marrer, tourner, et s'arrÃªter sans appel rÃ©seau obligatoire.

**Traduction en logique d'implÃ©mentation Rust :**

```rust
// âœ… CORRECT : Configuration depuis sources locales
pub struct EnvConfig {
    values: HashMap<String, String>,
}

impl EnvConfig {
    pub fn from_env() -> Self {
        // Utilise std::env::vars() â€” source locale
        Self {
            values: std::env::vars().collect(),
        }
    }
}

// âŒ INCORRECT : Configuration obligatoirement distante
pub struct RemoteConfig;

impl RemoteConfig {
    pub async fn from_server(url: &str) -> Result<Self, Error> {
        // âŒ Appel rÃ©seau obligatoire au dÃ©marrage
        let response = reqwest::get(url).await?;
        // ...
    }
}

// âœ… CORRECT : Horloge locale
pub struct DefaultClock;

impl Clock for DefaultClock {
    fn now(&self) -> SystemTime {
        SystemTime::now() // Source locale
    }
}

// âŒ INCORRECT : Synchronisation NTP obligatoire
pub struct NtpClock;

impl NtpClock {
    pub async fn new() -> Result<Self, Error> {
        // âŒ DÃ©pendance rÃ©seau obligatoire
        sync_with_ntp_server().await?;
    }
}
```

**DÃ©pendances autorisÃ©es (v0.1) :**

| Crate | Usage | Justification |
|-------|-------|---------------|
| **std** | Base du langage | Toujours autorisÃ© |
| **log** | FaÃ§ade de logging | Interface standard, pas un backend |
| **uuid** | GÃ©nÃ©ration d'identifiants | Minimal, infra, local |
| **ulid** | Identifiants ULID (optionnel) | Ajout quand 2+ produits en ont besoin |

**DÃ©pendances interdites :**

| Famille | Exemples | Raison |
|---------|----------|--------|
| Runtime async | tokio, async-std | Le produit choisit le runtime |
| Serveurs HTTP | axum, actix, rocket | Hors pÃ©rimÃ¨tre (INV-K-4) |
| Base de donnÃ©es | sqlx, diesel | Couche donnÃ©es = produit |
| SÃ©rialisation | serde, serde_json | Le produit choisit ses formats |

**RÃ©fÃ©rence contrat :** Structure du Kernel (Section 2), Invariants & Guarantees (INV-K-2), LOI-1

---

### 2.4 Pas de protocole applicatif (INV-K-4)

**Principe contractuel :**

L'invariant INV-K-4 Ã©tablit que le Kernel n'implÃ©mente jamais de protocole applicatif. HTTP, WebSocket, gRPC restent du ressort des produits.

**Traduction en logique d'implÃ©mentation Rust :**

```rust
// âœ… CORRECT : Trait abstrait sans protocole
pub trait Logger {
    fn log(&self, level: Level, message: &str);
}

// Le produit implÃ©mente la sortie vers son choix de backend
pub struct ProductLogger;

impl Logger for ProductLogger {
    fn log(&self, level: Level, message: &str) {
        // Le produit dÃ©cide : stdout, fichier, service distant, etc.
    }
}

// âŒ INCORRECT : Le Kernel intÃ¨gre un protocole
pub struct HttpLogger {
    endpoint: String,
}

impl Logger for HttpLogger {
    fn log(&self, level: Level, message: &str) {
        // âŒ Le Kernel ne doit pas connaÃ®tre HTTP
        reqwest::blocking::post(&self.endpoint).json(&message);
    }
}
```

**Ce que cela signifie concrÃ¨tement :**

| AutorisÃ© | Interdit |
|----------|----------|
| âœ… DÃ©finir des traits abstraits | âŒ ImplÃ©menter un serveur HTTP |
| âœ… Fournir des primitives de config | âŒ GÃ©rer des routes REST |
| âœ… Logger vers une interface abstraite | âŒ Envoyer des mÃ©triques vers Prometheus |
| âœ… Fournir un lifecycle gÃ©nÃ©rique | âŒ IntÃ©grer un middleware web |

**RÃ©fÃ©rence contrat :** Definition Kernel (Section 1), Invariants & Guarantees (INV-K-4)

---

## 3. ImplÃ©mentation des modules du Kernel

### 3.1 Module config

**Contrat (Revue API v0.1) :**

```rust
trait Config {
    fn get(&self, key: &str) -> Option<&str>;
}
```

**ImplÃ©mentation recommandÃ©e :**

```rust
/// Configuration chargÃ©e depuis les variables d'environnement.
/// Le produit choisit ses clÃ©s et ses valeurs.
pub struct EnvConfig {
    values: HashMap<String, String>,
}

impl EnvConfig {
    /// Charge la configuration depuis les variables d'environnement.
    /// Ne retourne pas d'erreur : std::env::vars() est infaillible.
    pub fn from_env() -> Self {
        Self {
            values: std::env::vars().collect(),
        }
    }
}

impl Config for EnvConfig {
    fn get(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(|s| s.as_str())
    }
}
```

**RÃ¨gles d'implÃ©mentation :**

| RÃ¨gle | Justification |
|-------|---------------|
| Pas de typage (int, bool) | Le produit parse (INV-K-1) |
| Pas de validation de clÃ©s | Le produit dÃ©finit ses clÃ©s (INV-K-1) |
| Source locale uniquement | Pas de dÃ©pendance rÃ©seau (INV-K-2) |
| Pas de format imposÃ© | Le produit choisit (JSON, TOML, etc.) |

**Ce que le module config NE fait PAS :**

- Valider les valeurs de configuration
- DÃ©finir des clÃ©s obligatoires
- Parser en types complexes
- Charger depuis un service distant

**Invariants concernÃ©s :** INV-K-1, INV-K-2, INV-K-3

---

### 3.2 Module id

**Contrat (Revue API v0.1) :**

```rust
trait IdGenerator {
    fn generate(&self) -> Id;
}

struct Id { /* opaque */ }

impl Id {
    pub fn parse(s: &str) -> Result<Id, IdParseError>;
}
// Display, Debug, Clone, Copy, Eq, Hash, PartialEq pour Id
```

**ImplÃ©mentation recommandÃ©e :**

```rust
/// Identifiant opaque. Le format interne n'est pas garanti.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id(Uuid);

impl Id {
    /// Parse un identifiant depuis sa reprÃ©sentation textuelle.
    /// Le format supportÃ© peut changer ; utiliser Display pour la sÃ©rialisation.
    pub fn parse(s: &str) -> Result<Id, IdParseError> {
        Uuid::parse_str(s)
            .map(Id)
            .map_err(|_| IdParseError::InvalidFormat)
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// GÃ©nÃ©rateur d'identifiants basÃ© sur UUID v4.
pub struct UuidIdGenerator;

impl IdGenerator for UuidIdGenerator {
    fn generate(&self) -> Id {
        Id(Uuid::new_v4())
    }
}
```

**RÃ¨gles d'implÃ©mentation :**

| RÃ¨gle | Justification |
|-------|---------------|
| Type `Id` opaque | Format interne non garanti (Revue API) |
| Pas de rÃ©-export `Uuid` | Le kernel ne rÃ©-exporte pas les dÃ©pendances |
| `Debug` = usage dev | Ne pas utiliser pour persistance |
| `Display` = sÃ©rialisation | Round-trip avec `Id::parse` |

**Ce que le module id NE fait PAS :**

- GÃ©nÃ©rer des identifiants avec sÃ©mantique mÃ©tier (user_id, order_id)
- Fournir des gÃ©nÃ©rateurs dÃ©terministes (pour le jeu : le produit implÃ©mente)
- Exposer le type sous-jacent (`Uuid`)

**Invariants concernÃ©s :** INV-K-1, INV-K-3, INV-K-6

---

### 3.3 Module time

**Contrat (Revue API v0.1) :**

```rust
trait Clock {
    fn now(&self) -> SystemTime;  // std::time::SystemTime
}
```

**ImplÃ©mentation recommandÃ©e :**

```rust
/// Horloge systÃ¨me par dÃ©faut.
pub struct DefaultClock;

impl Clock for DefaultClock {
    fn now(&self) -> SystemTime {
        SystemTime::now()
    }
}

// Le produit peut implÃ©menter un FakeClock pour les tests
#[cfg(test)]
pub struct FakeClock {
    fixed_time: SystemTime,
}

#[cfg(test)]
impl FakeClock {
    pub fn new(fixed_time: SystemTime) -> Self {
        Self { fixed_time }
    }
}

#[cfg(test)]
impl Clock for FakeClock {
    fn now(&self) -> SystemTime {
        self.fixed_time
    }
}
```

**RÃ¨gles d'implÃ©mentation :**

| RÃ¨gle | Justification |
|-------|---------------|
| Retourne `SystemTime` (std) | Pas de dÃ©pendance Ã  chrono |
| Timezone = produit | Le produit convertit si besoin |
| Injectable pour tests | `&dyn Clock` permet le mock |
| Pas de mÃ©thode `timestamp` | Le produit fait la conversion |

**Ce que le module time NE fait PAS :**

- GÃ©rer les fuseaux horaires
- Fournir des mÃ©thodes de formatage
- Synchroniser avec un serveur NTP
- Calculer des dates mÃ©tier (expiration, Ã©chÃ©ance)

**Invariants concernÃ©s :** INV-K-3, INV-K-6, INV-K-8

---

### 3.4 Module log

**Contrat (Revue API v0.1) :**

```rust
enum Level { Error, Warn, Info, Debug, Trace }

trait Logger {
    fn log(&self, level: Level, message: &str);
}
```

**ImplÃ©mentation recommandÃ©e :**

```rust
/// Niveaux de log alignÃ©s sur la faÃ§ade standard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

/// Trait de logging. Le produit choisit le backend.
pub trait Logger {
    fn log(&self, level: Level, message: &str);
}

/// Logger par dÃ©faut : Ã©crit sur stdout.
/// Minimal et remplaÃ§able.
pub struct DefaultLogger;

impl Logger for DefaultLogger {
    fn log(&self, level: Level, message: &str) {
        // Format minimal : le produit peut utiliser un Logger custom
        eprintln!("[{:?}] {}", level, message);
    }
}
```

**RÃ¨gles d'implÃ©mentation :**

| RÃ¨gle | Justification |
|-------|---------------|
| Une seule mÃ©thode `log` | Minimal, Ã©vite `info()`, `warn()`, etc. |
| `message: &str` | Le produit formate avant l'appel |
| Pas de `log_with_fields` | Format structurÃ© = choix du produit |
| `Level` propre au kernel | Pas de rÃ©-export de `log::Level` |

**Ce que le module log NE fait PAS :**

- Imposer un format (JSON, key-value)
- Configurer les niveaux actifs
- Router vers des backends spÃ©cifiques
- Fournir des macros `info!`, `warn!`, etc.

**Invariants concernÃ©s :** INV-K-3, INV-K-7

---

### 3.5 Module lifecycle

**Contrat (Revue API v0.1) :**

```rust
trait Lifecycle {
    fn register_shutdown_hook<F>(&mut self, f: F)
    where
        F: FnMut() + 'static;

    fn shutdown(&mut self);
}
```

**ImplÃ©mentation recommandÃ©e :**

```rust
/// Gestion du cycle de vie : shutdown hooks uniquement.
/// L'initialisation reste au produit.
pub struct DefaultLifecycle {
    hooks: Vec<Box<dyn FnMut()>>,
}

impl DefaultLifecycle {
    pub fn new() -> Self {
        Self { hooks: Vec::new() }
    }
}

impl Lifecycle for DefaultLifecycle {
    fn register_shutdown_hook<F>(&mut self, f: F)
    where
        F: FnMut() + 'static,
    {
        self.hooks.push(Box::new(f));
    }

    fn shutdown(&mut self) {
        // ExÃ©cution LIFO (dernier enregistrÃ© = premier exÃ©cutÃ©)
        while let Some(mut hook) = self.hooks.pop() {
            hook();
        }
    }
}
```

**RÃ¨gles d'implÃ©mentation :**

| RÃ¨gle | Justification |
|-------|---------------|
| Shutdown uniquement | L'init reste au produit (pas d'orchestration) |
| `FnMut() + 'static` | Contrainte Rust pour `Box<dyn FnMut>` |
| ExÃ©cution LIFO | Dernier enregistrÃ© = premier fermÃ© |
| Pas de `Result` | Panic dans un hook se propage |

**Ce que le module lifecycle NE fait PAS :**

- GÃ©rer l'ordre d'initialisation (le produit enchaÃ®ne config, log, etc.)
- Orchestrer des workflows mÃ©tier
- Fournir des hooks d'init
- Planifier des jobs ou des tÃ¢ches

**Documentation importante :**

- Un second appel Ã  `shutdown()` est implÃ©mentation-dÃ©pendant (no-op pour `DefaultLifecycle`)
- Les hooks sont appelÃ©s une seule fois

**Invariants concernÃ©s :** INV-K-1, INV-K-2, INV-K-10

---

## 4. Patterns d'implÃ©mentation

### 4.1 Pattern : Injection de dÃ©pendances via traits

**ProblÃ¨me :** Comment rendre le Kernel testable et extensible ?

**Solution :** Utiliser des traits pour permettre l'injection de dÃ©pendances.

```rust
/// Contexte d'exÃ©cution injectable
pub struct KernelContext<C: Config, L: Logger, I: IdGenerator, T: Clock> {
    pub config: C,
    pub logger: L,
    pub id_generator: I,
    pub clock: T,
}

// En production
let ctx = KernelContext {
    config: EnvConfig::from_env(),
    logger: DefaultLogger,
    id_generator: UuidIdGenerator,
    clock: DefaultClock,
};

// En test
let ctx = KernelContext {
    config: TestConfig::with_values(&[("KEY", "value")]),
    logger: NullLogger,
    id_generator: SequentialIdGenerator::new(),
    clock: FakeClock::fixed(some_time),
};
```

**Avantages :**

- TestabilitÃ© complÃ¨te
- Pas de singleton mutable
- Le produit choisit les implÃ©mentations

---

### 4.2 Pattern : Types opaques pour l'encapsulation

**ProblÃ¨me :** Comment exposer un type sans rÃ©vÃ©ler son implÃ©mentation ?

**Solution :** Utiliser un newtype pattern avec champ privÃ©.

```rust
/// Type opaque â€” l'implÃ©mentation peut changer
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id(Uuid);  // Champ privÃ©

// Le consommateur ne peut pas accÃ©der au Uuid interne
// Il utilise Display/parse pour la sÃ©rialisation

impl Id {
    // MÃ©thodes publiques limitÃ©es
    pub fn parse(s: &str) -> Result<Id, IdParseError> { ... }
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
```

**Avantages :**

- L'implÃ©mentation peut changer (UUID â†’ ULID) sans casser les consommateurs
- Le kernel ne rÃ©-exporte pas `Uuid`

---

### 4.3 Pattern : Erreurs explicites sans panic

**ProblÃ¨me :** Comment gÃ©rer les erreurs sans crash silencieux ?

**Solution :** Utiliser `Result<T, E>` avec des types d'erreur explicites.

```rust
/// Erreur de parsing d'identifiant
#[derive(Debug)]
pub struct IdParseError {
    pub reason: String,
}

impl std::fmt::Display for IdParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid ID format: {}", self.reason)
    }
}

impl std::error::Error for IdParseError {}

// Utilisation
pub fn parse(s: &str) -> Result<Id, IdParseError> {
    Uuid::parse_str(s)
        .map(Id)
        .map_err(|e| IdParseError { reason: e.to_string() })
}
```

**RÃ¨gles :**

- ImplÃ©menter `Debug`, `Display`, `Error` pour les types d'erreur
- Pas de panic pour les erreurs rÃ©cupÃ©rables
- Messages d'erreur explicables (INV-K-7)

---

## 5. Ce qu'un dÃ©veloppeur ne doit jamais faire

### 5.1 Ajouter de la logique mÃ©tier

**Interdit :**

```rust
// âŒ INCORRECT : Le kernel connaÃ®t le mÃ©tier
impl IdGenerator for UserIdGenerator {
    fn generate(&self) -> UserId {
        UserId::new(format!("USR-{}", Uuid::new_v4()))
    }
}

// âŒ INCORRECT : Configuration avec clÃ©s mÃ©tier
impl Config for ProductConfig {
    fn get_stripe_api_key(&self) -> Option<&str> { ... }
}
```

**ConsÃ©quence :** Violation de INV-K-1, couplage avec les produits.

### 5.2 DÃ©pendre d'un service externe obligatoire

**Interdit :**

```rust
// âŒ INCORRECT : Le kernel exige un service distant
impl Config for VaultConfig {
    fn new() -> Result<Self, Error> {
        // Appel rÃ©seau obligatoire
        let secrets = fetch_from_vault(VAULT_URL)?;
    }
}
```

**ConsÃ©quence :** Violation de INV-K-2, perte d'autonomie.

### 5.3 IntÃ©grer un protocole applicatif

**Interdit :**

```rust
// âŒ INCORRECT : Le kernel connaÃ®t HTTP
use axum::Router;

impl KernelServer {
    pub fn routes() -> Router {
        Router::new().route("/health", get(health_check))
    }
}
```

**ConsÃ©quence :** Violation de INV-K-4, couplage technologique.

### 5.4 Muter l'Ã©tat global

**Interdit :**

```rust
// âŒ INCORRECT : Ã‰tat global mutable
static mut COUNTER: u64 = 0;

impl IdGenerator for GlobalIdGenerator {
    fn generate(&self) -> Id {
        unsafe { COUNTER += 1 };  // âŒ Mutation globale
        Id::from(unsafe { COUNTER })
    }
}
```

**ConsÃ©quence :** Violation de INV-K-3, comportement non dÃ©terministe.

### 5.5 RÃ©-exporter les types de dÃ©pendances

**Interdit :**

```rust
// âŒ INCORRECT : Fuite des types internes
pub use uuid::Uuid;  // Le consommateur ne doit pas dÃ©pendre de uuid
pub use log::Level;  // Le kernel dÃ©finit son propre Level
```

**ConsÃ©quence :** Couplage aux dÃ©pendances, impossibilitÃ© de changer l'implÃ©mentation.

---

## 6. RÃ¨gles de tests

### 6.1 TestabilitÃ© des modules

Chaque module du Kernel DOIT Ãªtre testable de maniÃ¨re isolÃ©e grÃ¢ce Ã  l'injection de dÃ©pendances.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // Test du module id
    #[test]
    fn id_round_trip() {
        let gen = UuidIdGenerator;
        let id = gen.generate();
        let s = id.to_string();
        let parsed = Id::parse(&s).unwrap();
        assert_eq!(id, parsed);
    }

    // Test du module time avec horloge fixe
    #[test]
    fn clock_injectable() {
        let fixed = SystemTime::UNIX_EPOCH;
        let clock = FakeClock::new(fixed);
        assert_eq!(clock.now(), fixed);
    }

    // Test du module lifecycle
    #[test]
    fn shutdown_hooks_lifo() {
        let mut lifecycle = DefaultLifecycle::new();
        let order = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        
        let o1 = order.clone();
        lifecycle.register_shutdown_hook(move || o1.borrow_mut().push(1));
        
        let o2 = order.clone();
        lifecycle.register_shutdown_hook(move || o2.borrow_mut().push(2));
        
        lifecycle.shutdown();
        
        assert_eq!(*order.borrow(), vec![2, 1]); // LIFO
    }
}
```

### 6.2 PropriÃ©tÃ©s Ã  vÃ©rifier

| Module | PropriÃ©tÃ© | Comment vÃ©rifier |
|--------|-----------|------------------|
| **config** | Retourne `None` pour clÃ© absente | Test avec clÃ© inexistante |
| **id** | Round-trip `generate` â†’ `to_string` â†’ `parse` | Test d'aller-retour |
| **id** | UnicitÃ© des IDs gÃ©nÃ©rÃ©s | GÃ©nÃ©rer N IDs, vÃ©rifier absence de doublons |
| **time** | InjectabilitÃ© | Utiliser `FakeClock` |
| **log** | Accepte tous les niveaux | Logger avec chaque `Level` |
| **lifecycle** | ExÃ©cution LIFO | VÃ©rifier l'ordre des hooks |

### 6.3 Ce qui NE doit PAS Ãªtre testÃ©

- **Tests d'intÃ©gration avec services externes** â€” Le kernel n'en a pas
- **Tests de performance HTTP** â€” Le kernel ne fait pas de HTTP
- **Tests de base de donnÃ©es** â€” Hors pÃ©rimÃ¨tre

---

## 7. Contraintes de compilation

### 7.1 Features flags

Le Kernel peut utiliser des feature flags pour les modules expÃ©rimentaux (Phase 2) :

```toml
[features]
default = []
unstable = ["connection", "error"]
connection = []
error = []
```

**RÃ¨gle :** Les modules stables (config, id, time, log, lifecycle) sont toujours disponibles. Les modules expÃ©rimentaux sont derriÃ¨re un feature flag.

### 7.2 CompatibilitÃ© Rust

- **MSRV (Minimum Supported Rust Version)** : Ã€ dÃ©finir par le projet
- **Edition** : Rust 2021 recommandÃ©
- **Target** : Toutes les plateformes supportÃ©es par `std`

### 7.3 Linting

```toml
# Cargo.toml
[lints.rust]
unsafe_code = "forbid"      # Pas d'unsafe dans le kernel
missing_docs = "warn"       # Documentation requise

[lints.clippy]
all = "warn"
pedantic = "warn"
```

---

## 8. ConformitÃ© MSCM/MIP

### 8.1 Obligation de balisage MSCM

Tout code implÃ©mentÃ© pour le Kernel DOIT Ãªtre balisÃ© selon le protocole MSCM v1.

**RÃ©fÃ©rence :** [Miyukini Prompt Protocol - MIP v1 MSCM Index Protocol](..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md)

**Obligations minimales :**

- Chaque bloc fonctionnel DOIT avoir un identifiant unique (`@id`)
- Le rÃ´le sÃ©mantique DOIT Ãªtre explicite (`@role`)
- La couche architecturale DOIT Ãªtre dÃ©clarÃ©e (`@layer`)
- Une description humaine DOIT accompagner chaque bloc (`@human`)

### 8.2 IntÃ©gration MIP

AprÃ¨s implÃ©mentation, l'index MIP DOIT Ãªtre rÃ©gÃ©nÃ©rÃ© pour :

- Valider l'intÃ©gritÃ© des blocs MSCM
- Mettre Ã  jour le graphe de dÃ©pendances
- VÃ©rifier la cohÃ©rence hiÃ©rarchique

### 8.3 Check-list MSCM

Avant toute livraison, vÃ©rifier :

- [ ] Tous les blocs critiques sont balisÃ©s MSCM
- [ ] Les identifiants sont uniques globalement
- [ ] Les couches (layer) sont cohÃ©rentes avec l'architecture
- [ ] L'index MIP peut Ãªtre rÃ©gÃ©nÃ©rÃ© sans erreur

---

## 9. Check-list avant implÃ©mentation

Avant d'implÃ©menter ou de modifier un module du Kernel, vÃ©rifier :

### 9.1 Invariants

- [ ] **INV-K-1** : Pas de logique mÃ©tier ?
- [ ] **INV-K-2** : Pas de dÃ©pendance externe obligatoire ?
- [ ] **INV-K-3** : Primitives locales et sÃ»res uniquement ?
- [ ] **INV-K-4** : Pas de protocole applicatif ?
- [ ] **INV-K-5** : Observation sans mutation ?
- [ ] **INV-K-6** : Comportement dÃ©terministe ?
- [ ] **INV-K-7** : Messages explicables ?
- [ ] **INV-K-8** : Fonctionne offline ?
- [ ] **INV-K-9** : Ressources maÃ®trisÃ©es (Raspberry Pi compatible) ?
- [ ] **INV-K-10** : Gouvernance respectÃ©e (pas de dÃ©cision autonome) ?

### 9.2 API

- [ ] Les traits gelÃ©s sont-ils respectÃ©s ?
- [ ] Les types exposÃ©s sont-ils opaques quand nÃ©cessaire ?
- [ ] Les erreurs sont-elles explicites (`Result<T, E>`) ?
- [ ] Pas de rÃ©-export de types de dÃ©pendances ?

### 9.3 Tests

- [ ] Module testable de maniÃ¨re isolÃ©e ?
- [ ] Injection de dÃ©pendances fonctionnelle ?
- [ ] PropriÃ©tÃ©s clÃ©s vÃ©rifiÃ©es ?

---

## 10. Conclusion

Ce document fournit des lignes directrices pour implÃ©menter le Kernel de maniÃ¨re conforme aux contrats FONDATION.

**Points clÃ©s :**

- Le Kernel est **minimal** : 5 modules (config, id, time, log, lifecycle)
- Les **invariants INV-K-*** sont absolus et non nÃ©gociables
- Les **traits sont gelÃ©s** (Revue API v0.1) et ne doivent pas Ãªtre modifiÃ©s sans versioning
- L'implÃ©mentation privilÃ©gie la **puretÃ©**, le **dÃ©terminisme**, et l'**explicabilitÃ©**
- Aucune **logique mÃ©tier**, aucune **dÃ©pendance externe critique**, aucun **protocole applicatif**

**Nature informative :**

Ce document est purement informatif et ne crÃ©e aucune nouvelle obligation contractuelle. Il sert uniquement Ã  guider la comprÃ©hension et l'application des contrats FONDATION.

**Rappel :** Les contrats FONDATION priment toujours sur ce guide. En cas de doute, se rÃ©fÃ©rer aux contrats FONDATION.

---

**Document crÃ©Ã© le :** 2026-01-28  
**Version :** 1.0  
**Statut :** POST-FONDATION / NON NORMATIF / INFORMATIF  
**RÃ©fÃ©rence :** Definition Kernel v0.1, Structure du Kernel v0.1, Revue Traits API v0.1, Invariants & Guarantees v1.0  
**Type :** Guide d'implÃ©mentation non contractuel

