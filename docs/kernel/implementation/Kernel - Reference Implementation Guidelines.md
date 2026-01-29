# Kernel - Reference Implementation Guidelines

## Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document est **informatif, non normatif, et non contractuel**. Il guide un développeur pour implémenter le Kernel correctement, sans violer les contrats FONDATION.

**Objectif pédagogique :** Ce document vise à aider les développeurs à comprendre comment traduire les contrats FONDATION en implémentation Rust, en respectant strictement les invariants, garanties, et interdictions du Kernel.

**Avertissement :** Ce document ne doit pas être interprété abusivement. Il ne crée aucune nouvelle règle contractuelle et ne modifie aucun contrat existant. Les contrats FONDATION priment toujours sur ce guide.

**Relation avec les contrats FONDATION :** Ce document fait référence aux contrats FONDATION existants mais ne les étend pas, ne les modifie pas, et ne crée aucune nouvelle obligation contractuelle.

---

## 1. Introduction

### 1.1 Objectif

Ce document fournit des lignes directrices pour implémenter le Kernel de manière conforme aux contrats FONDATION. Il explique comment traduire les concepts contractuels en logique d'implémentation Rust sans interprétation abusive.

**Rappel :** Le Kernel est le **noyau technique minimal** de la fondation Miyukini, et non un kernel système au sens OS.

### 1.2 Nature informative

Ce document est **purement informatif**. Il ne définit pas de nouvelles règles, n'impose pas de technologies, et ne prescrit pas de solutions techniques. Il guide la compréhension et l'application des contrats FONDATION.

### 1.3 Sources contractuelles

Ce document se base sur tous les contrats FONDATION du Kernel :

- **[Definition Kernel](../Miyukini%20Core%20System%20-%20Definition%20Kernel.md)** : Périmètre, responsabilités, exclusions, frontières
- **[Structure du Kernel](../Miyukini%20Core%20System%20-%20Structure%20du%20Kernel.md)** : Crates, dépendances, visibilité, conventions
- **[Revue Traits API v0.1](../Miyukini%20Core%20System%20-%20Revue%20Traits%20API%20v0.1.md)** : Gel des traits publics
- **[Invariants & Guarantees](../contracts/Kernel%20-%20Invariants%20&%20Guarantees.md)** : Catalogue des invariants INV-K-*
- **[Kernel Maintenance Observability Contract](../../reference/Miyukini%20Conceptual%20References%20-%20Kernel%20Maintenance%20Observability%20Contract.md)** : Capacités d'observation
- **[Lois Autonomie Système](../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)** : Contraintes d'autonomie LOI-1 à LOI-6

**Terminologie :** Voir [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Principes généraux d'implémentation

### 2.1 Pureté et déterminisme (INV-K-3, INV-K-6)

**Principe contractuel :**

Les invariants INV-K-3 (Primitives locales sûres uniquement) et INV-K-6 (Déterminisme) établissent que le Kernel utilise uniquement des opérations déterministes et sans effets de bord cachés.

**Traduction en logique d'implémentation Rust :**

```rust
// ✅ CORRECT : Fonction pure, déterministe
pub fn generate_id(&self) -> Id {
    // Même entrée → même sortie
    // Pas d'effet de bord caché
    Id::from(Uuid::new_v4())
}

// ✅ CORRECT : Résultat explicite pour les erreurs
pub fn load_config(&self) -> Result<Config, ConfigError> {
    // Pas de panic silencieux
    // Erreur explicite si échec
}

// ❌ INCORRECT : Effet de bord caché
pub fn generate_id(&mut self) -> Id {
    self.counter += 1; // ❌ État mutable caché
    Id::from(self.counter)
}

// ❌ INCORRECT : Panic implicite
pub fn load_config(&self) -> Config {
    std::fs::read_to_string("config.toml").unwrap() // ❌ Panic possible
}
```

**Règles clés :**

- Privilégier `Result<T, E>` plutôt que panic
- Éviter les effets de bord cachés
- Structures immutables ou contrôlées
- Opérations déterministes (même entrée → même sortie)

**Référence contrat :** Invariants & Guarantees (INV-K-3, INV-K-6)

---

### 2.2 Zéro logique métier (INV-K-1)

**Principe contractuel :**

L'invariant INV-K-1 établit que le Kernel ne contient jamais de logique métier. Il ne connaît ni les entités domaine, ni les règles de gestion.

**Traduction en logique d'implémentation Rust :**

```rust
// ✅ CORRECT : Identifiant générique
pub struct Id(Uuid);

impl IdGenerator for UuidIdGenerator {
    fn generate(&self) -> Id {
        Id(Uuid::new_v4())
    }
}

// ❌ INCORRECT : Identifiant avec sémantique métier
pub struct UserId(Uuid);  // ❌ "User" = concept métier
pub struct OrderId(Uuid); // ❌ "Order" = concept métier

// ✅ CORRECT : Configuration générique
pub trait Config {
    fn get(&self, key: &str) -> Option<&str>;
}

// ❌ INCORRECT : Configuration avec clés métier prédéfinies
pub trait Config {
    fn get_stripe_key(&self) -> Option<&str>;  // ❌ "stripe" = service métier
    fn get_user_ttl(&self) -> Duration;        // ❌ "user" = concept métier
}
```

**Ce que cela signifie concrètement :**

| Autorisé | Interdit |
|----------|----------|
| ✅ Générer un identifiant unique | ❌ Générer un "user_id" formaté |
| ✅ Fournir l'heure courante | ❌ Calculer une date d'expiration produit |
| ✅ Logger un message structuré | ❌ Logger "commande validée" |
| ✅ Charger une configuration | ❌ Définir des politiques de tarification |

**Référence contrat :** Definition Kernel (Section 1, 3), Invariants & Guarantees (INV-K-1)

---

### 2.3 Zéro dépendance externe critique (INV-K-2)

**Principe contractuel :**

L'invariant INV-K-2 établit que le Kernel ne dépend jamais d'un service externe pour fonctionner. Il doit pouvoir démarrer, tourner, et s'arrêter sans appel réseau obligatoire.

**Traduction en logique d'implémentation Rust :**

```rust
// ✅ CORRECT : Configuration depuis sources locales
pub struct EnvConfig {
    values: HashMap<String, String>,
}

impl EnvConfig {
    pub fn from_env() -> Self {
        // Utilise std::env::vars() — source locale
        Self {
            values: std::env::vars().collect(),
        }
    }
}

// ❌ INCORRECT : Configuration obligatoirement distante
pub struct RemoteConfig;

impl RemoteConfig {
    pub async fn from_server(url: &str) -> Result<Self, Error> {
        // ❌ Appel réseau obligatoire au démarrage
        let response = reqwest::get(url).await?;
        // ...
    }
}

// ✅ CORRECT : Horloge locale
pub struct DefaultClock;

impl Clock for DefaultClock {
    fn now(&self) -> SystemTime {
        SystemTime::now() // Source locale
    }
}

// ❌ INCORRECT : Synchronisation NTP obligatoire
pub struct NtpClock;

impl NtpClock {
    pub async fn new() -> Result<Self, Error> {
        // ❌ Dépendance réseau obligatoire
        sync_with_ntp_server().await?;
    }
}
```

**Dépendances autorisées (v0.1) :**

| Crate | Usage | Justification |
|-------|-------|---------------|
| **std** | Base du langage | Toujours autorisé |
| **log** | Façade de logging | Interface standard, pas un backend |
| **uuid** | Génération d'identifiants | Minimal, infra, local |
| **ulid** | Identifiants ULID (optionnel) | Ajout quand 2+ produits en ont besoin |

**Dépendances interdites :**

| Famille | Exemples | Raison |
|---------|----------|--------|
| Runtime async | tokio, async-std | Le produit choisit le runtime |
| Serveurs HTTP | axum, actix, rocket | Hors périmètre (INV-K-4) |
| Base de données | sqlx, diesel | Couche données = produit |
| Sérialisation | serde, serde_json | Le produit choisit ses formats |

**Référence contrat :** Structure du Kernel (Section 2), Invariants & Guarantees (INV-K-2), LOI-1

---

### 2.4 Pas de protocole applicatif (INV-K-4)

**Principe contractuel :**

L'invariant INV-K-4 établit que le Kernel n'implémente jamais de protocole applicatif. HTTP, WebSocket, gRPC restent du ressort des produits.

**Traduction en logique d'implémentation Rust :**

```rust
// ✅ CORRECT : Trait abstrait sans protocole
pub trait Logger {
    fn log(&self, level: Level, message: &str);
}

// Le produit implémente la sortie vers son choix de backend
pub struct ProductLogger;

impl Logger for ProductLogger {
    fn log(&self, level: Level, message: &str) {
        // Le produit décide : stdout, fichier, service distant, etc.
    }
}

// ❌ INCORRECT : Le Kernel intègre un protocole
pub struct HttpLogger {
    endpoint: String,
}

impl Logger for HttpLogger {
    fn log(&self, level: Level, message: &str) {
        // ❌ Le Kernel ne doit pas connaître HTTP
        reqwest::blocking::post(&self.endpoint).json(&message);
    }
}
```

**Ce que cela signifie concrètement :**

| Autorisé | Interdit |
|----------|----------|
| ✅ Définir des traits abstraits | ❌ Implémenter un serveur HTTP |
| ✅ Fournir des primitives de config | ❌ Gérer des routes REST |
| ✅ Logger vers une interface abstraite | ❌ Envoyer des métriques vers Prometheus |
| ✅ Fournir un lifecycle générique | ❌ Intégrer un middleware web |

**Référence contrat :** Definition Kernel (Section 1), Invariants & Guarantees (INV-K-4)

---

## 3. Implémentation des modules du Kernel

### 3.1 Module config

**Contrat (Revue API v0.1) :**

```rust
trait Config {
    fn get(&self, key: &str) -> Option<&str>;
}
```

**Implémentation recommandée :**

```rust
/// Configuration chargée depuis les variables d'environnement.
/// Le produit choisit ses clés et ses valeurs.
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

**Règles d'implémentation :**

| Règle | Justification |
|-------|---------------|
| Pas de typage (int, bool) | Le produit parse (INV-K-1) |
| Pas de validation de clés | Le produit définit ses clés (INV-K-1) |
| Source locale uniquement | Pas de dépendance réseau (INV-K-2) |
| Pas de format imposé | Le produit choisit (JSON, TOML, etc.) |

**Ce que le module config NE fait PAS :**

- Valider les valeurs de configuration
- Définir des clés obligatoires
- Parser en types complexes
- Charger depuis un service distant

**Invariants concernés :** INV-K-1, INV-K-2, INV-K-3

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

**Implémentation recommandée :**

```rust
/// Identifiant opaque. Le format interne n'est pas garanti.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id(Uuid);

impl Id {
    /// Parse un identifiant depuis sa représentation textuelle.
    /// Le format supporté peut changer ; utiliser Display pour la sérialisation.
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

/// Générateur d'identifiants basé sur UUID v4.
pub struct UuidIdGenerator;

impl IdGenerator for UuidIdGenerator {
    fn generate(&self) -> Id {
        Id(Uuid::new_v4())
    }
}
```

**Règles d'implémentation :**

| Règle | Justification |
|-------|---------------|
| Type `Id` opaque | Format interne non garanti (Revue API) |
| Pas de ré-export `Uuid` | Le kernel ne ré-exporte pas les dépendances |
| `Debug` = usage dev | Ne pas utiliser pour persistance |
| `Display` = sérialisation | Round-trip avec `Id::parse` |

**Ce que le module id NE fait PAS :**

- Générer des identifiants avec sémantique métier (user_id, order_id)
- Fournir des générateurs déterministes (pour le jeu : le produit implémente)
- Exposer le type sous-jacent (`Uuid`)

**Invariants concernés :** INV-K-1, INV-K-3, INV-K-6

---

### 3.3 Module time

**Contrat (Revue API v0.1) :**

```rust
trait Clock {
    fn now(&self) -> SystemTime;  // std::time::SystemTime
}
```

**Implémentation recommandée :**

```rust
/// Horloge système par défaut.
pub struct DefaultClock;

impl Clock for DefaultClock {
    fn now(&self) -> SystemTime {
        SystemTime::now()
    }
}

// Le produit peut implémenter un FakeClock pour les tests
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

**Règles d'implémentation :**

| Règle | Justification |
|-------|---------------|
| Retourne `SystemTime` (std) | Pas de dépendance à chrono |
| Timezone = produit | Le produit convertit si besoin |
| Injectable pour tests | `&dyn Clock` permet le mock |
| Pas de méthode `timestamp` | Le produit fait la conversion |

**Ce que le module time NE fait PAS :**

- Gérer les fuseaux horaires
- Fournir des méthodes de formatage
- Synchroniser avec un serveur NTP
- Calculer des dates métier (expiration, échéance)

**Invariants concernés :** INV-K-3, INV-K-6, INV-K-8

---

### 3.4 Module log

**Contrat (Revue API v0.1) :**

```rust
enum Level { Error, Warn, Info, Debug, Trace }

trait Logger {
    fn log(&self, level: Level, message: &str);
}
```

**Implémentation recommandée :**

```rust
/// Niveaux de log alignés sur la façade standard.
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

/// Logger par défaut : écrit sur stdout.
/// Minimal et remplaçable.
pub struct DefaultLogger;

impl Logger for DefaultLogger {
    fn log(&self, level: Level, message: &str) {
        // Format minimal : le produit peut utiliser un Logger custom
        eprintln!("[{:?}] {}", level, message);
    }
}
```

**Règles d'implémentation :**

| Règle | Justification |
|-------|---------------|
| Une seule méthode `log` | Minimal, évite `info()`, `warn()`, etc. |
| `message: &str` | Le produit formate avant l'appel |
| Pas de `log_with_fields` | Format structuré = choix du produit |
| `Level` propre au kernel | Pas de ré-export de `log::Level` |

**Ce que le module log NE fait PAS :**

- Imposer un format (JSON, key-value)
- Configurer les niveaux actifs
- Router vers des backends spécifiques
- Fournir des macros `info!`, `warn!`, etc.

**Invariants concernés :** INV-K-3, INV-K-7

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

**Implémentation recommandée :**

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
        // Exécution LIFO (dernier enregistré = premier exécuté)
        while let Some(mut hook) = self.hooks.pop() {
            hook();
        }
    }
}
```

**Règles d'implémentation :**

| Règle | Justification |
|-------|---------------|
| Shutdown uniquement | L'init reste au produit (pas d'orchestration) |
| `FnMut() + 'static` | Contrainte Rust pour `Box<dyn FnMut>` |
| Exécution LIFO | Dernier enregistré = premier fermé |
| Pas de `Result` | Panic dans un hook se propage |

**Ce que le module lifecycle NE fait PAS :**

- Gérer l'ordre d'initialisation (le produit enchaîne config, log, etc.)
- Orchestrer des workflows métier
- Fournir des hooks d'init
- Planifier des jobs ou des tâches

**Documentation importante :**

- Un second appel à `shutdown()` est implémentation-dépendant (no-op pour `DefaultLifecycle`)
- Les hooks sont appelés une seule fois

**Invariants concernés :** INV-K-1, INV-K-2, INV-K-10

---

## 4. Patterns d'implémentation

### 4.1 Pattern : Injection de dépendances via traits

**Problème :** Comment rendre le Kernel testable et extensible ?

**Solution :** Utiliser des traits pour permettre l'injection de dépendances.

```rust
/// Contexte d'exécution injectable
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

- Testabilité complète
- Pas de singleton mutable
- Le produit choisit les implémentations

---

### 4.2 Pattern : Types opaques pour l'encapsulation

**Problème :** Comment exposer un type sans révéler son implémentation ?

**Solution :** Utiliser un newtype pattern avec champ privé.

```rust
/// Type opaque — l'implémentation peut changer
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id(Uuid);  // Champ privé

// Le consommateur ne peut pas accéder au Uuid interne
// Il utilise Display/parse pour la sérialisation

impl Id {
    // Méthodes publiques limitées
    pub fn parse(s: &str) -> Result<Id, IdParseError> { ... }
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
```

**Avantages :**

- L'implémentation peut changer (UUID → ULID) sans casser les consommateurs
- Le kernel ne ré-exporte pas `Uuid`

---

### 4.3 Pattern : Erreurs explicites sans panic

**Problème :** Comment gérer les erreurs sans crash silencieux ?

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

**Règles :**

- Implémenter `Debug`, `Display`, `Error` pour les types d'erreur
- Pas de panic pour les erreurs récupérables
- Messages d'erreur explicables (INV-K-7)

---

## 5. Ce qu'un développeur ne doit jamais faire

### 5.1 Ajouter de la logique métier

**Interdit :**

```rust
// ❌ INCORRECT : Le kernel connaît le métier
impl IdGenerator for UserIdGenerator {
    fn generate(&self) -> UserId {
        UserId::new(format!("USR-{}", Uuid::new_v4()))
    }
}

// ❌ INCORRECT : Configuration avec clés métier
impl Config for ProductConfig {
    fn get_stripe_api_key(&self) -> Option<&str> { ... }
}
```

**Conséquence :** Violation de INV-K-1, couplage avec les produits.

### 5.2 Dépendre d'un service externe obligatoire

**Interdit :**

```rust
// ❌ INCORRECT : Le kernel exige un service distant
impl Config for VaultConfig {
    fn new() -> Result<Self, Error> {
        // Appel réseau obligatoire
        let secrets = fetch_from_vault(VAULT_URL)?;
    }
}
```

**Conséquence :** Violation de INV-K-2, perte d'autonomie.

### 5.3 Intégrer un protocole applicatif

**Interdit :**

```rust
// ❌ INCORRECT : Le kernel connaît HTTP
use axum::Router;

impl KernelServer {
    pub fn routes() -> Router {
        Router::new().route("/health", get(health_check))
    }
}
```

**Conséquence :** Violation de INV-K-4, couplage technologique.

### 5.4 Muter l'état global

**Interdit :**

```rust
// ❌ INCORRECT : État global mutable
static mut COUNTER: u64 = 0;

impl IdGenerator for GlobalIdGenerator {
    fn generate(&self) -> Id {
        unsafe { COUNTER += 1 };  // ❌ Mutation globale
        Id::from(unsafe { COUNTER })
    }
}
```

**Conséquence :** Violation de INV-K-3, comportement non déterministe.

### 5.5 Ré-exporter les types de dépendances

**Interdit :**

```rust
// ❌ INCORRECT : Fuite des types internes
pub use uuid::Uuid;  // Le consommateur ne doit pas dépendre de uuid
pub use log::Level;  // Le kernel définit son propre Level
```

**Conséquence :** Couplage aux dépendances, impossibilité de changer l'implémentation.

---

## 6. Règles de tests

### 6.1 Testabilité des modules

Chaque module du Kernel DOIT être testable de manière isolée grâce à l'injection de dépendances.

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

### 6.2 Propriétés à vérifier

| Module | Propriété | Comment vérifier |
|--------|-----------|------------------|
| **config** | Retourne `None` pour clé absente | Test avec clé inexistante |
| **id** | Round-trip `generate` → `to_string` → `parse` | Test d'aller-retour |
| **id** | Unicité des IDs générés | Générer N IDs, vérifier absence de doublons |
| **time** | Injectabilité | Utiliser `FakeClock` |
| **log** | Accepte tous les niveaux | Logger avec chaque `Level` |
| **lifecycle** | Exécution LIFO | Vérifier l'ordre des hooks |

### 6.3 Ce qui NE doit PAS être testé

- **Tests d'intégration avec services externes** — Le kernel n'en a pas
- **Tests de performance HTTP** — Le kernel ne fait pas de HTTP
- **Tests de base de données** — Hors périmètre

---

## 7. Contraintes de compilation

### 7.1 Features flags

Le Kernel peut utiliser des feature flags pour les modules expérimentaux (Phase 2) :

```toml
[features]
default = []
unstable = ["connection", "error"]
connection = []
error = []
```

**Règle :** Les modules stables (config, id, time, log, lifecycle) sont toujours disponibles. Les modules expérimentaux sont derrière un feature flag.

### 7.2 Compatibilité Rust

- **MSRV (Minimum Supported Rust Version)** : À définir par le projet
- **Edition** : Rust 2021 recommandé
- **Target** : Toutes les plateformes supportées par `std`

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

## 8. Conformité MSCM/MIP

### 8.1 Obligation de balisage MSCM

Tout code implémenté pour le Kernel DOIT être balisé selon le protocole MSCM v1.

**Référence :** [Miyukini Prompt Protocol - MIP v1 MSCM Index Protocol](../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md)

**Obligations minimales :**

- Chaque bloc fonctionnel DOIT avoir un identifiant unique (`@id`)
- Le rôle sémantique DOIT être explicite (`@role`)
- La couche architecturale DOIT être déclarée (`@layer`)
- Une description humaine DOIT accompagner chaque bloc (`@human`)

### 8.2 Intégration MIP

Après implémentation, l'index MIP DOIT être régénéré pour :

- Valider l'intégrité des blocs MSCM
- Mettre à jour le graphe de dépendances
- Vérifier la cohérence hiérarchique

### 8.3 Check-list MSCM

Avant toute livraison, vérifier :

- [ ] Tous les blocs critiques sont balisés MSCM
- [ ] Les identifiants sont uniques globalement
- [ ] Les couches (layer) sont cohérentes avec l'architecture
- [ ] L'index MIP peut être régénéré sans erreur

---

## 9. Check-list avant implémentation

Avant d'implémenter ou de modifier un module du Kernel, vérifier :

### 9.1 Invariants

- [ ] **INV-K-1** : Pas de logique métier ?
- [ ] **INV-K-2** : Pas de dépendance externe obligatoire ?
- [ ] **INV-K-3** : Primitives locales et sûres uniquement ?
- [ ] **INV-K-4** : Pas de protocole applicatif ?
- [ ] **INV-K-5** : Observation sans mutation ?
- [ ] **INV-K-6** : Comportement déterministe ?
- [ ] **INV-K-7** : Messages explicables ?
- [ ] **INV-K-8** : Fonctionne offline ?
- [ ] **INV-K-9** : Ressources maîtrisées (Raspberry Pi compatible) ?
- [ ] **INV-K-10** : Gouvernance respectée (pas de décision autonome) ?

### 9.2 API

- [ ] Les traits gelés sont-ils respectés ?
- [ ] Les types exposés sont-ils opaques quand nécessaire ?
- [ ] Les erreurs sont-elles explicites (`Result<T, E>`) ?
- [ ] Pas de ré-export de types de dépendances ?

### 9.3 Tests

- [ ] Module testable de manière isolée ?
- [ ] Injection de dépendances fonctionnelle ?
- [ ] Propriétés clés vérifiées ?

---

## 10. Conclusion

Ce document fournit des lignes directrices pour implémenter le Kernel de manière conforme aux contrats FONDATION.

**Points clés :**

- Le Kernel est **minimal** : 5 modules (config, id, time, log, lifecycle)
- Les **invariants INV-K-*** sont absolus et non négociables
- Les **traits sont gelés** (Revue API v0.1) et ne doivent pas être modifiés sans versioning
- L'implémentation privilégie la **pureté**, le **déterminisme**, et l'**explicabilité**
- Aucune **logique métier**, aucune **dépendance externe critique**, aucun **protocole applicatif**

**Nature informative :**

Ce document est purement informatif et ne crée aucune nouvelle obligation contractuelle. Il sert uniquement à guider la compréhension et l'application des contrats FONDATION.

**Rappel :** Les contrats FONDATION priment toujours sur ce guide. En cas de doute, se référer aux contrats FONDATION.

---

**Document créé le :** 2026-01-28  
**Version :** 1.0  
**Statut :** POST-FONDATION / NON NORMATIF / INFORMATIF  
**Référence :** Definition Kernel v0.1, Structure du Kernel v0.1, Revue Traits API v0.1, Invariants & Guarantees v1.0  
**Type :** Guide d'implémentation non contractuel
