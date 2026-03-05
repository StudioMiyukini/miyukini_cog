# Kernel â€” Architecture & Components

## 1. Introduction

### Objet du document

Ce document definit l'**architecture et les composants du Kernel** : une documentation normative etablissant la structure interne du Kernel, ses modules, leurs relations, et les flux de donnees dans le Miyukini Core System.

Ce document precise l'organisation architecturale, les composants internes (modules), leurs responsabilites, les interactions entre modules, et les points d'extension.

### Portee

Ce document s'applique a **toute l'architecture du Kernel** et definit :

- l'architecture conceptuelle du Kernel,
- les modules internes et leurs responsabilites,
- les relations et dependances entre modules,
- les flux de donnees,
- les points d'extension pour les produits,
- les invariants architecturaux.

### Statut documentaire

Ce document est **normatif et de statut ARCHITECTURE**. Il etablit la structure de reference du Kernel et doit etre respecte par toute implementation.

### Relation avec les autres documents

Ce document **synthetise et illustre** l'architecture definie dans :

- **Definition Kernel** : Perimetre, responsabilites, exclusions
- **Structure du Kernel** : Crates, dependances, visibilite, conventions
- **Revue Traits API v0.1** : Specification de l'API publique gelee
- **Invariants & Guarantees** : Contrats et invariants normatifs

Ce document ne contredit aucun autre document et constitue une vue architecturale consolidee.

---

## 2. Architecture conceptuelle

### 2.1. Vue d'ensemble

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                           ECOSYSTEME MIYUKINI                           â”‚
â”‚                                                                         â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”‚
â”‚  â”‚                           PRODUITS                                â”‚  â”‚
â”‚  â”‚              (SaaS, Web, Mobile, Jeu, Workers)                    â”‚  â”‚
â”‚  â”‚                                                                   â”‚  â”‚
â”‚  â”‚   â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”    â”‚  â”‚
â”‚  â”‚   â”‚                 ADAPTATEURS PRODUIT                      â”‚    â”‚  â”‚
â”‚  â”‚   â”‚      (Integration specifique a chaque produit)           â”‚    â”‚  â”‚
â”‚  â”‚   â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜    â”‚  â”‚
â”‚  â”‚                              â”‚                                    â”‚  â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â”‚
â”‚                                 â”‚                                        â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”‚
â”‚  â”‚                           CORES                                    â”‚  â”‚
â”‚  â”‚   StrongFather | KindMother | BondingBrother | CaringNanny | ...  â”‚  â”‚
â”‚  â”‚                              â”‚                                    â”‚  â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â”‚
â”‚                                 â”‚                                        â”‚
â”‚                                 â–¼                                        â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”    â”‚
â”‚  â”‚                          KERNEL                                  â”‚    â”‚
â”‚  â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”    â”‚    â”‚
â”‚  â”‚  â”‚ config  â”‚   id    â”‚  time   â”‚   log   â”‚    lifecycle    â”‚    â”‚    â”‚
â”‚  â”‚  â”‚         â”‚         â”‚         â”‚         â”‚                 â”‚    â”‚    â”‚
â”‚  â”‚  â”‚ Config  â”‚ IdGen   â”‚  Clock  â”‚ Logger  â”‚   Lifecycle     â”‚    â”‚    â”‚
â”‚  â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”´â”€â”€â”€â”€â”€â”€â”€â”€â”€â”´â”€â”€â”€â”€â”€â”€â”€â”€â”€â”´â”€â”€â”€â”€â”€â”€â”€â”€â”€â”´â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜    â”‚    â”‚
â”‚  â”‚                                                                  â”‚    â”‚
â”‚  â”‚              Crate unique : miyukini-kernel                      â”‚    â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜    â”‚
â”‚                                                                         â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### 2.2. Positionnement du Kernel

**Le Kernel est la fondation technique minimale** :

- Il n'est pas un framework applicatif
- Il n'est pas un serveur HTTP, un ORM, ou un client externe
- Il n'est pas le lieu de la logique metier
- Il est la couche d'execution et de coordination utilisee par tous les produits

**Dependances :**

- Le Kernel ne depend d'aucun composant externe critique (conformite a **LOI-1**)
- Les Cores et les produits dependent du Kernel
- **La dependance est strictement unidirectionnelle : Produits â†’ Cores â†’ Kernel**

Cette architecture respecte les lois d'autonomie systeme, notamment **LOI-1** (aucune dependance externe critique) : le Kernel peut demarrer, fonctionner, et etre audite sans aucun appel externe obligatoire.

### 2.3. Diagramme de dependances

```
                    â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
                    â”‚           Produits                  â”‚
                    â”‚  (SaaS, Web, Mobile, Jeu, Workers)  â”‚
                    â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                                    â”‚ depend de
                    â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
                    â”‚             Cores                   â”‚
                    â”‚  StrongFather, KindMother, etc.     â”‚
                    â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                                    â”‚ depend de
                    â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
                    â”‚            KERNEL                   â”‚
                    â”‚  config | id | time | log | lifecycleâ”‚
                    â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜

    Flux de dependance :
    Produits â†’ Cores â†’ Kernel
    
    JAMAIS l'inverse :
    Kernel âœ—â†’ Cores âœ—â†’ Produits
```

---

## 3. Modules internes du Kernel

### 3.1. Module config

**Definition :**

Le module **config** fournit les mecanismes de chargement et d'acces a la configuration.

**Responsabilites :**

- Fournir une interface d'acces aux valeurs de configuration
- Permettre le chargement depuis differentes sources (env, fichiers)
- Ne pas imposer de politique de nommage ou de structure

**Trait principal :**

```rust
trait Config {
    fn get(&self, key: &str) -> Option<&str>;
}
```

**Types exposes :**

| Type | Description |
|------|-------------|
| `Config` | Trait d'acces a la configuration |
| `EnvConfig` | Implementation par defaut (variables d'environnement) |

**Ce que le module NE fait PAS :**

- Pas de parsing de types (int, bool, etc.) â€” le produit parse
- Pas de politique de nommage des cles â€” le produit decide
- Pas d'imposition de format (JSON, YAML, TOML) â€” le produit choisit
- Pas de gestion de secrets avancee â€” Phase 2

### 3.2. Module id

**Definition :**

Le module **id** fournit la generation d'identifiants uniques.

**Responsabilites :**

- Generer des identifiants uniques (UUID/ULID)
- Fournir un type opaque `Id` pour les identifiants
- Permettre la serialisation/deserialisation des identifiants

**Trait principal :**

```rust
trait IdGenerator {
    fn generate(&self) -> Id;
}
```

**Types exposes :**

| Type | Description |
|------|-------------|
| `Id` | Type opaque representant un identifiant |
| `IdParseError` | Erreur de parsing d'un identifiant |
| `IdGenerator` | Trait de generation d'identifiants |
| `UuidIdGenerator` | Implementation par defaut (UUID v4) |

**Caracteristiques du type Id :**

- **Opaque** : le format interne n'est pas garanti
- **Display** : conversion en chaine pour persistance produit
- **Parse** : reconstruction depuis une chaine
- **Clone, Copy, Eq, Hash, PartialEq** : utilisable en collections

**Ce que le module NE fait PAS :**

- Pas de generation deterministe (seeds) â€” Phase 2 / produit
- Pas d'exposition du type `uuid::Uuid` â€” encapsulation stricte

### 3.3. Module time

**Definition :**

Le module **time** fournit l'abstraction du temps systeme.

**Responsabilites :**

- Fournir l'instant present (`now()`)
- Permettre l'injection en test (fake clock)
- Ne pas imposer de timezone

**Trait principal :**

```rust
trait Clock {
    fn now(&self) -> SystemTime;  // std::time::SystemTime
}
```

**Types exposes :**

| Type | Description |
|------|-------------|
| `Clock` | Trait d'abstraction du temps |
| `DefaultClock` | Implementation par defaut (horloge systeme) |

**Ce que le module NE fait PAS :**

- Pas de gestion de timezone â€” le produit choisit (chrono, etc.)
- Pas de type `Timestamp` â€” le produit fait la conversion
- Pas de dependance a `chrono` â€” `std::time` suffit

### 3.4. Module log

**Definition :**

Le module **log** fournit le logging structure par niveau.

**Responsabilites :**

- Permettre le logging par niveau (Error, Warn, Info, Debug, Trace)
- Fournir une implementation par defaut minimale
- Ne pas imposer de backend ou de format

**Trait principal :**

```rust
trait Logger {
    fn log(&self, level: Level, message: &str);
}
```

**Types exposes :**

| Type | Description |
|------|-------------|
| `Level` | Enum des niveaux de log (Error, Warn, Info, Debug, Trace) |
| `Logger` | Trait de logging |
| `DefaultLogger` | Implementation par defaut (stdout) |

**Ce que le module NE fait PAS :**

- Pas de methodes raccourcies (`info()`, `warn()`, etc.) â€” une seule methode `log`
- Pas de format impose (JSON, etc.) â€” le produit formate `message`
- Pas de backend impose â€” le produit configure son Logger
- Pas de tracing/OpenTelemetry â€” observabilite avancee hors Kernel

### 3.5. Module lifecycle

**Definition :**

Le module **lifecycle** fournit la gestion du cycle de vie des briques techniques.

**Responsabilites :**

- Permettre l'enregistrement de hooks d'arret
- Executer les hooks dans l'ordre LIFO a l'arret
- Ne pas gerer l'orchestration metier

**Trait principal :**

```rust
trait Lifecycle {
    fn register_shutdown_hook<F>(&mut self, f: F)
    where
        F: FnMut() + 'static;

    fn shutdown(&mut self);
}
```

**Types exposes :**

| Type | Description |
|------|-------------|
| `Lifecycle` | Trait de gestion du cycle de vie |
| `DefaultLifecycle` | Implementation par defaut |

**Ce que le module NE fait PAS :**

- Pas de hooks d'initialisation â€” le produit enchaine config, log, etc.
- Pas d'orchestration de workflows metier â€” technique uniquement
- Pas de jobs metier ou hooks applicatifs

---

## 4. Relations entre modules

### 4.1. Independance des modules

**Principe fondamental :** Les modules du Kernel sont **independants** les uns des autres au niveau du contrat (traits).

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                           KERNEL                                     â”‚
â”‚                                                                     â”‚
â”‚   â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”   â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”   â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”   â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”   â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚   â”‚ config  â”‚   â”‚   id    â”‚   â”‚  time   â”‚   â”‚   log   â”‚   â”‚lifecycleâ”‚
â”‚   â”‚         â”‚   â”‚         â”‚   â”‚         â”‚   â”‚         â”‚   â”‚         â”‚
â”‚   â”‚ Config  â”‚   â”‚ IdGen   â”‚   â”‚  Clock  â”‚   â”‚ Logger  â”‚   â”‚Lifecycleâ”‚
â”‚   â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜   â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜   â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜   â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜   â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
â”‚        â”‚             â”‚             â”‚             â”‚             â”‚
â”‚        â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”´â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”´â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”´â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
â”‚                                    â”‚
â”‚                    Aucune dependance croisee
â”‚                    entre les traits des modules
â”‚                                    â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

**Regles d'independance :**

| Regle | Description |
|-------|-------------|
| **R-IND-1** | Aucun trait ne reference un autre trait du Kernel |
| **R-IND-2** | Aucun type public ne depend d'un autre module |
| **R-IND-3** | Chaque module peut etre utilise independamment |

### 4.2. Usage combine par le produit

Le **produit** combine les modules selon ses besoins :

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                        PRODUIT (exemple)                             â”‚
â”‚                                                                     â”‚
â”‚   fn main() {                                                       â”‚
â”‚       // 1. Charger la config                                       â”‚
â”‚       let config = EnvConfig::from_env();                           â”‚
â”‚                                                                     â”‚
â”‚       // 2. Initialiser le logger                                   â”‚
â”‚       let logger = DefaultLogger::new();                            â”‚
â”‚                                                                     â”‚
â”‚       // 3. Initialiser le generateur d'ID                          â”‚
â”‚       let id_gen = UuidIdGenerator::new();                          â”‚
â”‚                                                                     â”‚
â”‚       // 4. Initialiser l'horloge                                   â”‚
â”‚       let clock = DefaultClock;                                     â”‚
â”‚                                                                     â”‚
â”‚       // 5. Configurer le lifecycle                                 â”‚
â”‚       let mut lifecycle = DefaultLifecycle::new();                  â”‚
â”‚       lifecycle.register_shutdown_hook(|| {                         â”‚
â”‚           // Nettoyage technique                                    â”‚
â”‚       });                                                           â”‚
â”‚                                                                     â”‚
â”‚       // ... logique produit ...                                    â”‚
â”‚                                                                     â”‚
â”‚       lifecycle.shutdown();                                         â”‚
â”‚   }                                                                 â”‚
â”‚                                                                     â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

**L'ordre d'initialisation est decide par le produit, pas par le Kernel.**

---

## 5. Flux de donnees

### 5.1. Flux de configuration

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                     FLUX DE CONFIGURATION                            â”‚
â”‚                                                                     â”‚
â”‚   [Source]                                                          â”‚
â”‚      â”‚                                                              â”‚
â”‚      â”‚ Variables d'env / Fichiers / Secrets                         â”‚
â”‚      â–¼                                                              â”‚
â”‚   â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                                               â”‚
â”‚   â”‚   EnvConfig     â”‚ â—€â”€â”€ Constructeur (from_env, etc.)             â”‚
â”‚   â”‚  (ou custom)    â”‚                                               â”‚
â”‚   â””â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”˜                                               â”‚
â”‚            â”‚                                                        â”‚
â”‚            â”‚ impl Config                                            â”‚
â”‚            â–¼                                                        â”‚
â”‚   â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                                               â”‚
â”‚   â”‚ config.get(key) â”‚ â—€â”€â”€ Acces par cle                             â”‚
â”‚   â””â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”˜                                               â”‚
â”‚            â”‚                                                        â”‚
â”‚            â”‚ Option<&str>                                           â”‚
â”‚            â–¼                                                        â”‚
â”‚   [Produit]                                                         â”‚
â”‚      â”‚                                                              â”‚
â”‚      â”‚ Parse et utilise les valeurs                                 â”‚
â”‚      â–¼                                                              â”‚
â”‚   [Logique metier]                                                  â”‚
â”‚                                                                     â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### 5.2. Flux de generation d'ID

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                     FLUX DE GENERATION D'ID                          â”‚
â”‚                                                                     â”‚
â”‚   [Produit]                                                         â”‚
â”‚      â”‚                                                              â”‚
â”‚      â”‚ Besoin d'un nouvel identifiant                               â”‚
â”‚      â–¼                                                              â”‚
â”‚   â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                                           â”‚
â”‚   â”‚ id_gen.generate()   â”‚                                           â”‚
â”‚   â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                                           â”‚
â”‚              â”‚                                                      â”‚
â”‚              â”‚ Id (opaque)                                          â”‚
â”‚              â–¼                                                      â”‚
â”‚   [Produit]                                                         â”‚
â”‚      â”‚                                                              â”‚
â”‚      â”œâ”€â”€â–¶ id.to_string() â”€â”€â–¶ Persistance (BDD, etc.)               â”‚
â”‚      â”‚                                                              â”‚
â”‚      â””â”€â”€â–¶ Utilisation en memoire (cles, collections)               â”‚
â”‚                                                                     â”‚
â”‚   â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€  â”‚
â”‚                                                                     â”‚
â”‚   [Lecture]                                                         â”‚
â”‚      â”‚                                                              â”‚
â”‚      â”‚ Chaine depuis BDD                                            â”‚
â”‚      â–¼                                                              â”‚
â”‚   â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                                           â”‚
â”‚   â”‚   Id::parse(&str)   â”‚                                           â”‚
â”‚   â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                                           â”‚
â”‚              â”‚                                                      â”‚
â”‚              â”‚ Result<Id, IdParseError>                             â”‚
â”‚              â–¼                                                      â”‚
â”‚   [Produit] â”€â”€â–¶ Utilisation                                         â”‚
â”‚                                                                     â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### 5.3. Flux de logging

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                        FLUX DE LOGGING                               â”‚
â”‚                                                                     â”‚
â”‚   [Produit]                                                         â”‚
â”‚      â”‚                                                              â”‚
â”‚      â”‚ Evenement a logger                                           â”‚
â”‚      â–¼                                                              â”‚
â”‚   â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                               â”‚
â”‚   â”‚ logger.log(Level::Info, "msg")  â”‚                               â”‚
â”‚   â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                               â”‚
â”‚                    â”‚                                                â”‚
â”‚                    â”‚ Delegation au Logger                           â”‚
â”‚                    â–¼                                                â”‚
â”‚   â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                               â”‚
â”‚   â”‚      Implementation Logger      â”‚                               â”‚
â”‚   â”‚  (DefaultLogger ou custom)      â”‚                               â”‚
â”‚   â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                               â”‚
â”‚                    â”‚                                                â”‚
â”‚                    â”‚ Sortie (stdout, fichier, backend, etc.)        â”‚
â”‚                    â–¼                                                â”‚
â”‚   [Destination] â—€â”€â”€ Choix du produit                                â”‚
â”‚                                                                     â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### 5.4. Flux de lifecycle

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                       FLUX DE LIFECYCLE                              â”‚
â”‚                                                                     â”‚
â”‚   [Initialisation]                                                  â”‚
â”‚      â”‚                                                              â”‚
â”‚      â”‚ Produit cree le Lifecycle                                    â”‚
â”‚      â–¼                                                              â”‚
â”‚   â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                               â”‚
â”‚   â”‚   DefaultLifecycle::new()       â”‚                               â”‚
â”‚   â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                               â”‚
â”‚                    â”‚                                                â”‚
â”‚                    â–¼                                                â”‚
â”‚   [Enregistrement des hooks]                                        â”‚
â”‚      â”‚                                                              â”‚
â”‚      â”‚ Hook 1 (ex: fermer connexion DB)                             â”‚
â”‚      â–¼                                                              â”‚
â”‚   â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                               â”‚
â”‚   â”‚ lifecycle.register_shutdown_hookâ”‚                               â”‚
â”‚   â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                               â”‚
â”‚                    â”‚                                                â”‚
â”‚      â”‚ Hook 2 (ex: flush logs)                                      â”‚
â”‚      â–¼                                                              â”‚
â”‚   â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                               â”‚
â”‚   â”‚ lifecycle.register_shutdown_hookâ”‚                               â”‚
â”‚   â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                               â”‚
â”‚                    â”‚                                                â”‚
â”‚                    â–¼                                                â”‚
â”‚   [Runtime - logique produit]                                       â”‚
â”‚                    â”‚                                                â”‚
â”‚                    â”‚ Signal d'arret                                 â”‚
â”‚                    â–¼                                                â”‚
â”‚   â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                               â”‚
â”‚   â”‚    lifecycle.shutdown()         â”‚                               â”‚
â”‚   â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                               â”‚
â”‚                    â”‚                                                â”‚
â”‚                    â”‚ Execution LIFO                                 â”‚
â”‚                    â–¼                                                â”‚
â”‚   [Hook 2] â”€â”€â–¶ [Hook 1] â”€â”€â–¶ [Fin]                                   â”‚
â”‚                                                                     â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

---

## 6. Points d'extension

### 6.1. Principe d'extensibilite

Le Kernel fournit des **traits** que le produit peut implementer pour personnaliser le comportement.

**Tous les traits sont extensibles par le produit :**

| Trait | Extension possible |
|-------|-------------------|
| `Config` | Implementation custom (fichiers, vault, etc.) |
| `IdGenerator` | Generateur deterministe, ULID, etc. |
| `Clock` | Fake clock pour tests, horloge synchronisee |
| `Logger` | Backend custom (fichier, reseau, structuree) |
| `Lifecycle` | Gestion avancee du shutdown |

### 6.2. Exemple : Config custom

```rust
// Produit : implementation d'un Config depuis fichier TOML
struct TomlConfig {
    values: HashMap<String, String>,
}

impl Config for TomlConfig {
    fn get(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(|s| s.as_str())
    }
}

impl TomlConfig {
    fn from_file(path: &str) -> Result<Self, Error> {
        // Le produit parse le TOML
        // Le Kernel ne connait pas TOML
    }
}
```

### 6.3. Exemple : Clock pour tests

```rust
// Produit : fake clock pour tests deterministes
struct FakeClock {
    fixed_time: SystemTime,
}

impl Clock for FakeClock {
    fn now(&self) -> SystemTime {
        self.fixed_time
    }
}

#[test]
fn test_with_fixed_time() {
    let clock = FakeClock {
        fixed_time: SystemTime::UNIX_EPOCH,
    };
    assert_eq!(clock.now(), SystemTime::UNIX_EPOCH);
}
```

### 6.4. Exemple : Logger structure

```rust
// Produit : logger JSON structure
struct JsonLogger;

impl Logger for JsonLogger {
    fn log(&self, level: Level, message: &str) {
        let json = format!(
            r#"{{"level":"{:?}","message":"{}","timestamp":{}}}"#,
            level, message, timestamp_now()
        );
        println!("{}", json);
    }
}
```

---

## 7. Invariants architecturaux

### 7.1. Invariants de structure

| Invariant | Description |
|-----------|-------------|
| **INV-ARCH-K-1** | Une seule crate `miyukini-kernel` en v0.1 |
| **INV-ARCH-K-2** | Cinq modules : config, id, time, log, lifecycle |
| **INV-ARCH-K-3** | Aucune dependance croisee entre les traits des modules |
| **INV-ARCH-K-4** | Dependance unidirectionnelle : Produits â†’ Cores â†’ Kernel |

### 7.2. Invariants de comportement

| Invariant | Description |
|-----------|-------------|
| **INV-ARCH-K-5** | Aucune logique metier dans le Kernel |
| **INV-ARCH-K-6** | Aucune dependance externe critique |
| **INV-ARCH-K-7** | Fonctionnement complet en isolation (offline) |
| **INV-ARCH-K-8** | Traits stables et geles en v0.1 |

### 7.3. Invariants de visibilite

| Invariant | Description |
|-----------|-------------|
| **INV-ARCH-K-9** | Seuls les traits et types du contrat sont `pub` |
| **INV-ARCH-K-10** | Les details d'implementation sont `pub(crate)` ou prives |
| **INV-ARCH-K-11** | Pas de re-export de types de dependances (`uuid::Uuid`, etc.) |

---

## 8. Dependances autorisees et interdites

### 8.1. Dependances autorisees (v0.1)

| Crate | Usage | Justification |
|-------|-------|---------------|
| `std` | Base du langage | Toujours autorisee |
| `log` | Facade de logging | Interface standard Rust |
| `uuid` | Generation d'UUID | Minimal, infra, sans dependance applicative |

### 8.2. Dependances interdites

| Famille | Exemples | Raison |
|---------|----------|--------|
| Runtime async | tokio, async-std | Choix du produit |
| Serveurs HTTP | axum, actix-web, rocket | Hors perimetre |
| Clients HTTP | reqwest, hyper | Hors perimetre |
| ORM / DB | sqlx, diesel, redis | Couche donnees = produit |
| Observabilite | tracing, opentelemetry | Hooks seulement |
| Serialisation | serde, serde_json | Choix du produit |
| Auth / metier | crates JWT, OAuth | Metier = produit |

---

## 9. Evolution future (Phase 2+)

### 9.1. Modules potentiels

| Module | Description | Condition d'ajout |
|--------|-------------|-------------------|
| `connection` / `pool` | Abstraction de connexions DB/Redis | Quand 2+ produits en ont besoin |
| `error` | Types d'erreur partages | Quand besoin transverse |

### 9.2. Regles d'evolution

Toutes les conditions suivantes doivent etre vraies pour ajouter un module :

1. Au moins **2 produits ou 2 surfaces** en ont besoin
2. La responsabilite est **clairement infra** (pas de metier)
3. Le module reste **petit** et sans dependance business
4. Aucun produit existant ne peut raisonnablement le fournir sans duplication inutile

---

## 10. Conclusion documentaire

Ce document etablit de maniere normative l'architecture et les composants du Kernel.

Il garantit que :

- l'architecture est explicitement definie,
- les modules internes sont identifies et documentes,
- les relations entre modules sont formalisees,
- les flux de donnees sont illustres,
- les points d'extension sont documentes,
- les invariants architecturaux sont maintenus.

Ce document est de statut **ARCHITECTURE**. Toute modification doit etre documentee et versionnee.

---

## 11. References croisees

- [Definition Kernel](../Miyukini%20Core%20System%20-%20Definition%20Kernel.md) â€” Perimetre et responsabilites
- [Structure du Kernel](../Miyukini%20Core%20System%20-%20Structure%20du%20Kernel.md) â€” Crates, dependances, conventions
- [Revue Traits API v0.1](../Miyukini%20Core%20System%20-%20Revue%20Traits%20API%20v0.1.md) â€” Specification de l'API gelee
- [Invariants & Guarantees](../contracts/Kernel%20-%20Invariants%20&%20Guarantees.md) â€” Contrats normatifs
- [Lois d'Autonomie Systeme](..//..//miyukini-webway-system//reference//_index.md) â€” Conformite aux lois

---

**Date de creation :** 2026-01-28  
**Version :** 0.1.0  
**Statut :** VÃ©rifiÃ© Phase 3 â€” RÃ‰FÃ‰RENCE

